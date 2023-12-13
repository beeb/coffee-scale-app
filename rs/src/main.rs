use std::{collections::VecDeque, time::Duration};

use anyhow::{anyhow, Result};
use esp_idf_svc::{
    hal::{
        adc::ADC1,
        delay::Delay,
        gpio::{Gpio12, Gpio14, Gpio34},
        peripherals::Peripherals,
    },
    systime::EspSystemTime,
    timer::EspTaskTimerService,
};
use loadcell::LoadCell;

use crate::weight::LOADCELL_READY_DELAY;

mod battery;
mod ble;
mod weight;

const CALIBRATE_MODE: bool = false;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let pins = peripherals.pins;

    log::info!("Starting up...");

    ble::init()?;
    log::info!("BLE initialized");

    let battery_percent =
        battery::read_battery_percent::<Gpio34, ADC1>(pins.gpio34, peripherals.adc1)?;
    log::info!("Battery level: {}%", battery_percent);
    ble::BATTERY
        .get()
        .ok_or_else(|| anyhow!("Battery characteristic not initialized"))?
        .lock()
        .set_value(&battery_percent.to_be_bytes());

    let mut load_sensor = weight::init_load_sensor::<Gpio14, Gpio12>(pins.gpio14, pins.gpio12)?;
    let delay = Delay::new_default();

    if CALIBRATE_MODE {
        load_sensor.tare(1);
        loop {
            let mut current;
            let mut average: f32 = 0.0;
            for n in 1..=10 {
                while !load_sensor.is_ready() {
                    delay.delay_us(LOADCELL_READY_DELAY);
                }
                current = load_sensor.read() as f32;
                delay.delay_us(LOADCELL_READY_DELAY * 2);
                average += (current - average) / (n as f32);
            }
            log::info!("Weight reading: {:.4}", average);
        }
    }

    // take readings of the loadcell and keep iterating until the weight is stable
    let mut readings: VecDeque<f32> = VecDeque::with_capacity(10);
    loop {
        while !load_sensor.is_ready() {
            delay.delay_us(LOADCELL_READY_DELAY);
        }
        let reading = load_sensor.read_scaled();
        log::info!("Waiting for stable weight: {:.4}", reading);
        if readings.len() == 10 {
            readings.pop_front();
        }
        readings.push_back(reading);
        if readings.len() == 10 && readings.iter().all(|&x| (x - reading).abs() < 0.1) {
            break;
        }
        delay.delay_us(LOADCELL_READY_DELAY * 2);
    }
    log::info!("offset: {}", load_sensor.get_offset());
    load_sensor.tare(10);

    let mut weight;

    let system_time = EspSystemTime {};
    let mut last_notify = Duration::default();

    loop {
        weight = weight::read_weight(&mut load_sensor, &delay);
        log::info!("weight: {weight}");
        let now = system_time.now();
        if now - last_notify > Duration::from_millis(200) {
            last_notify = now;
            ble::WEIGHT
                .get()
                .unwrap()
                .lock()
                .set_value(&weight.to_be_bytes())
                .notify();
        }
    }
}
