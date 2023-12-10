use esp_idf_svc::{hal::delay, sys::esp_random};

mod ble;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting up...");

    ble::init();
    log::info!("BLE initialized");

    loop {
        let rand = unsafe { u16::try_from(esp_random() % 5000).unwrap() };
        ble::WEIGHT
            .get()
            .unwrap()
            .lock()
            .set_value(&rand.to_be_bytes())
            .notify();
        delay::FreeRtos::delay_ms(200);
    }
}
