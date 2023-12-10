use std::collections::VecDeque;

use anyhow::{anyhow, Result};
use esp_idf_svc::hal::delay;
use loadcell::LoadCell;

mod battery;
mod ble;
mod weight;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting up...");

    ble::init()?;
    log::info!("BLE initialized");

    let battery_percent = battery::read_battery_percent()?;
    log::info!("Battery level: {}%", battery_percent);
    ble::BATTERY
        .get()
        .ok_or_else(|| anyhow!("Battery characteristic not initialized"))?
        .lock()
        .set_value(&battery_percent.to_be_bytes());

    let mut load_sensor = weight::init_load_sensor()?;

    // take readings of the loadcell and keep iterating until the weight is stable
    let mut readings: VecDeque<f32> = VecDeque::with_capacity(10);
    loop {
        let reading = load_sensor.read_scaled();
        log::info!("Reading: {}", reading);
        if readings.len() == 10 {
            readings.pop_front();
        }
        readings.push_back(reading);
        if readings.len() == 10 && readings.iter().all(|&x| (x - reading).abs() < 0.1) {
            break;
        }
        delay::FreeRtos::delay_ms(100);
    }
    load_sensor.tare(10);

    loop {
        let weight = weight::read_weight(&mut load_sensor);
        log::info!("Weight: {:.2}g", weight as f32 / 100.);
        ble::WEIGHT
            .get()
            .ok_or_else(|| anyhow!("Weight characteristic not initialized"))?
            .lock()
            .set_value(&weight.to_be_bytes())
            .notify();
        delay::FreeRtos::delay_ms(200);
    }
}
