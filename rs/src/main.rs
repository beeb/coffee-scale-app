use esp32_nimble::{utilities::BleUuid, BLEDevice, NimbleProperties};
use esp_idf_svc::{hal::delay, sys::esp_random};

const WEIGHT_SCALE_SERVICE: BleUuid = BleUuid::from_uuid16(0x181D);
const WEIGHT_MEASUREMENT_CHARACTERISTIC: BleUuid = BleUuid::from_uuid16(0x2A9D);

const BATTERY_SERVICE: BleUuid = BleUuid::from_uuid16(0x180F);
const BATTERY_LEVEL_CHARACTERISTIC: BleUuid = BleUuid::from_uuid16(0x2A19);

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting up...");

    let ble_device = BLEDevice::take();
    let server = ble_device.get_server();
    server.on_connect(|server, desc| {
        log::info!("Client connected");

        server
            .update_conn_params(desc.conn_handle, 24, 48, 0, 60)
            .unwrap();

        log::info!("Multi-connect support: start advertising");
        ble_device.get_advertising().start().unwrap();
    });
    server.on_disconnect(|_desc, reason| {
        log::info!("Client disconnected ({:X})", reason);
        ble_device.get_advertising().start().unwrap();
    });

    let battery_service = server.create_service(BATTERY_SERVICE);
    let battery_characteristic = battery_service
        .lock()
        .create_characteristic(BATTERY_LEVEL_CHARACTERISTIC, NimbleProperties::READ);
    battery_characteristic.lock().set_value(&50u8.to_be_bytes());

    let weight_service = server.create_service(WEIGHT_SCALE_SERVICE);
    let weight_characteristic = weight_service.lock().create_characteristic(
        WEIGHT_MEASUREMENT_CHARACTERISTIC,
        NimbleProperties::READ | NimbleProperties::NOTIFY,
    );
    weight_characteristic.lock().set_value(&0i16.to_be_bytes());

    let ble_advertising = ble_device.get_advertising();
    ble_advertising
        .name("coffee-scale")
        .add_service_uuid(WEIGHT_SCALE_SERVICE)
        .add_service_uuid(BATTERY_SERVICE);

    ble_advertising.start().unwrap();

    loop {
        let rand = unsafe { u16::try_from(esp_random() % 5000).unwrap() };
        weight_characteristic
            .lock()
            .set_value(&rand.to_be_bytes())
            .notify();
        delay::FreeRtos::delay_ms(200);
    }
}
