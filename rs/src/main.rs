//#![feature(box_into_inner)]
use std::{
    num::NonZeroU32,
    sync::{
        atomic::{AtomicI16, Ordering},
        Arc,
    },
    thread,
};

use anyhow::{anyhow, Result};
use esp_idf_svc::hal::{
    adc::ADC1,
    delay::BLOCK,
    gpio::{Gpio12, Gpio14, Gpio34},
    i2c,
    peripherals::Peripherals,
    prelude::*,
    task::notification::Notification,
    timer::{config, TimerDriver},
};
use ssd1306::I2CDisplayInterface;

use crate::{screen::Screen, weight::Scales};

mod battery;
mod ble;
mod screen;
mod weight;

const CALIBRATE_MODE: bool = false;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let pins = peripherals.pins;

    log::info!("Starting up...");

    let i2c = i2c::I2cDriver::new(
        peripherals.i2c0,
        pins.gpio21,
        pins.gpio22,
        &i2c::I2cConfig::new().baudrate(400.kHz().into()),
    )?;
    let interface = I2CDisplayInterface::new(i2c);

    let mut screen = Screen::new(interface);

    ble::init()?;
    log::info!("BLE initialized");

    let battery_percent =
        battery::read_battery_percent::<Gpio34, ADC1>(pins.gpio34, peripherals.adc1)?;
    log::info!("Battery level: {}%", battery_percent);
    screen.set_battery(battery_percent);
    ble::BATTERY
        .get()
        .ok_or_else(|| anyhow!("Battery characteristic not initialized"))?
        .lock()
        .set_value(&battery_percent.to_be_bytes());

    let mut scales = Scales::<Gpio12, Gpio14>::new(pins.gpio12, pins.gpio14)?;

    if CALIBRATE_MODE {
        // loop indefinitely
        scales.tare();
        loop {
            let average = scales.read_average(10);
            log::info!("Weight reading: {:.4}", average);
        }
    }

    scales.wait_stable();

    scales.tare();

    let weight: Arc<AtomicI16> = Arc::new(AtomicI16::new(0));
    let shared_weight = Arc::clone(&weight);

    thread::spawn(move || {
        let notification = Notification::new();
        let timer_conf = config::Config::new().auto_reload(true);
        let mut timer = TimerDriver::new(peripherals.timer00, &timer_conf).unwrap();
        timer.set_alarm(timer.tick_hz() / 5).unwrap();
        let notifier = notification.notifier();
        // Saftey: make sure the `Notification` object is not dropped while the subscription is active
        unsafe {
            timer
                .subscribe(move || {
                    let bitset = 0b00000000001;
                    notifier.notify(NonZeroU32::new(bitset).unwrap());
                })
                .unwrap();
        }
        timer.enable_interrupt().unwrap();
        timer.enable_alarm(true).unwrap();
        timer.enable(true).unwrap();
        loop {
            notification.wait(BLOCK);
            let weight = shared_weight.load(Ordering::Relaxed);
            ble::WEIGHT
                .get()
                .unwrap()
                .lock()
                .set_value(&weight.to_be_bytes())
                .notify();
        }
    });

    loop {
        scales.read_weight(&weight);
        let weight = weight.load(Ordering::Relaxed);
        screen.print(weight);
    }
}
