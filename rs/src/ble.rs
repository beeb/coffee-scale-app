//! Bluetooth Low Energy (BLE) server implementation
//!
//! This module provides the BLE server implementation for the coffee scale. It exposes a weight characteristic and a
//! battery characteristic.
use std::sync::{Arc, OnceLock};

use anyhow::{anyhow, Result};
use esp32_nimble::{
    utilities::{mutex::Mutex, BleUuid},
    BLEAdvertisementData, BLECharacteristic, BLEDevice, NimbleProperties,
};

const WEIGHT_SCALE_SERVICE: BleUuid = BleUuid::from_uuid16(0x181D);
const WEIGHT_MEASUREMENT_CHARACTERISTIC: BleUuid = BleUuid::from_uuid16(0x2A9D);

const BATTERY_SERVICE: BleUuid = BleUuid::from_uuid16(0x180F);
const BATTERY_LEVEL_CHARACTERISTIC: BleUuid = BleUuid::from_uuid16(0x2A19);

/// Make weight characteristic available globally
pub static WEIGHT: OnceLock<Arc<Mutex<BLECharacteristic>>> = OnceLock::new();

/// Make battery characteristic available globally
pub static BATTERY: OnceLock<Arc<Mutex<BLECharacteristic>>> = OnceLock::new();

/// Initialize the BLE server
pub fn init() -> Result<()> {
    let ble_device = BLEDevice::take();
    let ble_advertising = ble_device.get_advertising();
    let server = ble_device.get_server();
    server.on_connect(|server, desc| {
        log::info!("Client connected");

        server
            .update_conn_params(desc.conn_handle(), 24, 48, 0, 60)
            .expect("ble update conn params");

        log::info!("Multi-connect support: start advertising");
        ble_advertising
            .lock()
            .start()
            .expect("ble start advertising");
    });
    server.on_disconnect(|_desc, reason| {
        log::info!("Client disconnected ({:?})", reason);
        ble_advertising
            .lock()
            .start()
            .expect("ble start advertising after disconnect");
    });

    let battery_service = server.create_service(BATTERY_SERVICE);
    let battery_characteristic = battery_service
        .lock()
        .create_characteristic(BATTERY_LEVEL_CHARACTERISTIC, NimbleProperties::READ);
    battery_characteristic.lock().set_value(&50u8.to_be_bytes());
    BATTERY
        .set(battery_characteristic)
        .map_err(|_| anyhow!("Battery characteristic already initialized"))?;

    let weight_service = server.create_service(WEIGHT_SCALE_SERVICE);
    let weight_characteristic = weight_service.lock().create_characteristic(
        WEIGHT_MEASUREMENT_CHARACTERISTIC,
        NimbleProperties::READ | NimbleProperties::NOTIFY,
    );
    weight_characteristic.lock().set_value(&0i32.to_be_bytes());
    WEIGHT
        .set(weight_characteristic)
        .map_err(|_| anyhow!("Weight characteristic already initialized"))?;

    ble_advertising
        .lock()
        .set_data(
            BLEAdvertisementData::new()
                .name("coffee-scale")
                .add_service_uuid(WEIGHT_SCALE_SERVICE)
                .add_service_uuid(BATTERY_SERVICE),
        )
        .map_err(|_| anyhow!("Set advertisement data error"))?;

    ble_advertising
        .lock()
        .start()
        .map_err(|_| anyhow!("Advertising start error"))?;
    Ok(())
}
