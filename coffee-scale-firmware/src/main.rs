#![no_std]
#![no_main]

// cargo espflash flash --release --bootloader C:\tb\target\xtensa-esp32-espidf\release\build\esp-idf-sys-b70e5b0d2fa5cc23\out\build\bootloader\bootloader.bin

use bleps::{
    ad_structure::{
        create_advertising_data, AdStructure, BR_EDR_NOT_SUPPORTED, LE_GENERAL_DISCOVERABLE,
    },
    att::Uuid,
    attribute_server::{AttributeServer, NotificationData, WorkResult},
    gatt, Ble, HciConnector,
};
use esp_backtrace as _;
use esp_wifi::{ble::controller::BleConnector, initialize, EspWifiInitFor};
use hal::{
    clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Delay, Rng,
};

// org.bluetooth.characteristic.gap.appearance.xml
const ADV_APPEARANCE_TYPE: u8 = 0x19;
const APPEARANCE_GENERIC_WEIGHT_SCALE: [u8; 2] = 0x032u16.to_le_bytes();

const AUTOMATION_IO_UUID: Uuid = Uuid::Uuid16(0x1815);
const BATTERY_UUID: Uuid = Uuid::Uuid16(0x180F);

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    let timer_group = TimerGroup::new(peripherals.TIMG1, &clocks);
    let timer = timer_group.timer0;

    let mut delay = Delay::new(&clocks);

    esp_println::logger::init_logger_from_env();
    log::info!("Logging started");

    let Ok(init) = initialize(
        EspWifiInitFor::Ble,
        timer,
        Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    ) else {
        panic!("Failed to initialize esp-wifi");
    };
    log::info!("BLE initialized");

    let mut bluetooth = peripherals.BT;

    /*let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio21,
        io.pins.gpio22,
        100u32.kHz(),
        &clocks,
    ); */

    loop {
        let connector = BleConnector::new(&init, &mut bluetooth);
        let hci = HciConnector::new(connector, esp_wifi::current_millis);
        let mut ble = Ble::new(&hci);
        match ble.init() {
            Ok(_) => log::info!("BLE initialized"),
            Err(e) => panic!("Failed to initialize BLE: {:?}", e),
        }
        match ble.cmd_set_le_advertising_parameters() {
            Ok(_) => log::info!("BLE advertising parameters set"),
            Err(e) => panic!("Failed to set advertising parameters: {:?}", e),
        }

        let Ok(adv_data) = create_advertising_data(&[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::ServiceUuids16(&[AUTOMATION_IO_UUID, BATTERY_UUID]),
            AdStructure::CompleteLocalName("mpy-coffee"),
            // appearance
            AdStructure::Unknown {
                ty: ADV_APPEARANCE_TYPE,
                data: &APPEARANCE_GENERIC_WEIGHT_SCALE,
            },
        ]) else {
            panic!("Failed to create advertising data");
        };
        match ble.cmd_set_le_advertising_data(adv_data) {
            Ok(_) => log::info!("BLE advertising data set"),
            Err(e) => panic!("Failed to set advertising data: {:?}", e),
        }
        match ble.cmd_set_le_advertise_enable(true) {
            Ok(_) => log::info!("BLE advertising enabled"),
            Err(e) => panic!("Failed to enable advertising: {:?}", e),
        }

        log::info!("Started advertising");

        let battery_percent: u8 = 50; // 50%
        let weight_value: i16 = 10 * 100; // 10g

        let mut weight = |_offset: usize, data: &mut [u8]| {
            data[..2].copy_from_slice(&weight_value.to_be_bytes());
            2
        };

        let mut bat = |_offset: usize, data: &mut [u8]| {
            data[..1].copy_from_slice(&battery_percent.to_be_bytes());
            1
        };

        gatt!([
            service {
                uuid: "0000180F-0000-1000-8000-00805F9B34FB", // Battery
                characteristics: [characteristic {
                    name: "battery",
                    uuid: "00002A19-0000-1000-8000-00805F9B34FB", // 0x2A19 = Battery Level
                    read: bat,
                },],
            },
            service {
                uuid: "00001815-0000-1000-8000-00805F9B34FB", // Automation IO
                characteristics: [characteristic {
                    name: "weight",
                    uuid: "00002A59-0000-1000-8000-00805F9B34FB", // 0x2A59 = analog IO
                    read: weight,
                    notify: true,
                },],
            }
        ]);

        let mut srv = AttributeServer::new(&mut ble, &mut gatt_attributes);
        loop {
            let mut notification = None;
            let mut buf = [0u8; 1];
            if let Some(1) = srv.get_characteristic_value(weight_notify_enable_handle, 0, &mut buf)
            {
                if buf[0] == 1 {
                    // notitications enabled
                    notification = Some(NotificationData::new(
                        weight_handle,
                        &weight_value.to_be_bytes(),
                    ));
                }
            }
            match srv.do_work_with_notification(notification) {
                Ok(res) => {
                    if let WorkResult::GotDisconnected = res {
                        break;
                    }
                }
                Err(err) => {
                    log::error!("{:?}", err);
                }
            }
        }
    }
}
