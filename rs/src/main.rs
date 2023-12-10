use anyhow::{anyhow, Result};
use esp_idf_svc::{hal::delay, sys::esp_random};

mod battery;
mod ble;

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

    loop {
        let rand = unsafe { u16::try_from(esp_random() % 5000)? };
        log::info!("Weight: {:.2}g", rand as f32 / 100.);
        ble::WEIGHT
            .get()
            .ok_or_else(|| anyhow!("Weight characteristic not initialized"))?
            .lock()
            .set_value(&rand.to_be_bytes())
            .notify();
        delay::FreeRtos::delay_ms(200);
    }
}
