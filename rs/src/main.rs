use esp32_nimble::{utilities::BleUuid, uuid128, BLEDevice, NimbleProperties};
use esp_idf_svc::hal::delay;

const AUTOMATION_IO_SERVICE: BleUuid = BleUuid::from_uuid16(0x1815);
const BATTERY_SERVICE: BleUuid = BleUuid::from_uuid16(0x180F);

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
    });

    let battery_service = server.create_service(BATTERY_SERVICE);
    let battery_characteristic = battery_service
        .lock()
        .create_characteristic(BleUuid::from_uuid16(0x2A19), NimbleProperties::READ);
    battery_characteristic.lock().set_value(&50u8.to_be_bytes());

    let weight_service = server.create_service(AUTOMATION_IO_SERVICE);
    let weight_characteristic = weight_service.lock().create_characteristic(
        BleUuid::from_uuid16(0x2A59),
        NimbleProperties::READ | NimbleProperties::NOTIFY,
    );
    weight_characteristic.lock().set_value(&0i16.to_be_bytes());

    let ble_advertising = ble_device.get_advertising();
    ble_advertising
        .name("mpy-coffee")
        .add_service_uuid(AUTOMATION_IO_SERVICE)
        .add_service_uuid(BATTERY_SERVICE);

    ble_advertising.start().unwrap();

    loop {
        weight_characteristic.lock().notify();
        delay::FreeRtos::delay_ms(500);
    }
}
