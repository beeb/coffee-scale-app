//! Firmware for the ESP32 based smart scale.
//!
//! The scale uses a HX711 loadcell amplifier to read the weight and a SSD1306 OLED display to show the weight and
//! battery level.
//!
//! The scale is also a Bluetooth Low Energy (BLE) peripheral that exposes a weight characteristic and a battery
//! characteristic. It also notifies subscribers of the weight characteristic approx. every 200ms.
//!
//! The scale can be calibrated by pressing the button for 2 seconds. The calibration mode shows the raw loadcell
//! readings and the ADC value of the battery voltage. The calibration mode is exited by pressing the button again.
//! The values can be then used to calculate the scaling factor (`LOADCELL_SCALING`) as well as adjust the battery level
//! conversion function (`battery::adc_to_percent`).
//!
//! At the moment, there is no interactive way to set the scaling factor, so it has to be hardcoded in the source code.
use std::{
    num::NonZeroU32,
    sync::{
        atomic::{AtomicBool, AtomicI32, Ordering},
        Arc, Mutex,
    },
    thread,
};

use anyhow::{anyhow, Result};
use esp_idf_svc::{
    hal::{
        delay::{Ets, BLOCK},
        gpio::{self, InterruptType, Pull},
        i2c,
        peripherals::Peripherals,
        prelude::*,
        task::notification::Notification,
        timer::{config, TimerDriver},
    },
    systime::EspSystemTime,
};
use ssd1306::I2CDisplayInterface;

use crate::{battery::BatteryReader, screen::Screen, weight::Loadcell};

mod battery;
mod ble;
mod critical_section;
mod screen;
mod weight;

/// Scaling factor for the loadcell.
///
/// The hx711 raw value is multiplied by this to get the weight in grams.
const LOADCELL_SCALING: f32 = 6.49304e-4;

fn main() -> Result<()> {
    // Initialize the IDF stuff and logger
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    // Setup screen communication over I2C
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

    // Initialize BLE
    ble::init()?;
    log::info!("BLE initialized");

    // Read battery level
    let mut battery_reader = BatteryReader::new(pins.gpio34, peripherals.adc1)?;
    let (battery_percent, _) = battery_reader.read_battery_percent()?;
    log::info!("Battery level: {}%", battery_percent);
    screen.set_battery(battery_percent);
    ble::BATTERY
        .get()
        .ok_or_else(|| anyhow!("Battery characteristic not initialized"))?
        .lock()
        .set_value(&battery_percent.to_be_bytes());

    // Initialize the loadcell
    let scales = Arc::new(Mutex::new(Loadcell::new(
        pins.gpio13,
        pins.gpio14,
        LOADCELL_SCALING,
    )?));

    // Tare the scales after it's become stable
    {
        let mut scales = scales.lock().expect("mutex lock");
        scales.wait_stable();
        scales.tare(None);
        // unlock mutex
    }

    // Weight value to be shared between threads
    let weight: Arc<AtomicI32> = Arc::new(AtomicI32::new(0));

    // Bluetooth reporting thread
    thread::spawn({
        let weight = Arc::clone(&weight);
        move || {
            // Timer to notify subscribers of the weight characteristic value
            let notification = Notification::new();
            let timer_conf = config::Config::new().auto_reload(true);
            let mut timer = TimerDriver::new(peripherals.timer00, &timer_conf).expect("timer");
            timer
                .set_alarm(timer.tick_hz() / 5) // every 200ms = 5 times per second
                .expect("set timer alarm");
            let notifier = notification.notifier();
            unsafe {
                timer
                    .subscribe(move || {
                        notifier.notify(NonZeroU32::new(0b00000000001).expect("new bitset"));
                    })
                    .expect("subscribe to timer");
            }
            // Enable timer interrupt
            timer.enable_interrupt().expect("enable timer interrupt");
            timer.enable_alarm(true).expect("enable timer alarm");
            timer.enable(true).expect("enable timer");
            loop {
                notification.wait(BLOCK);
                log::info!("Timer fired");
                let weight = weight.load(Ordering::Relaxed);
                ble::WEIGHT
                    .get()
                    .expect("weight characteristic not initialized")
                    .lock()
                    .set_value(&weight.to_be_bytes())
                    .notify();
            }
        }
    });

    // Calibration mode flag to be shared between threads
    let calibration_mode = Arc::new(AtomicBool::new(false));

    // Tare/Calibration button thread
    thread::spawn({
        let calibration_mode = Arc::clone(&calibration_mode); // moved inside thread
        let scales = Arc::clone(&scales); // moved inside thread
        move || {
            let mut button_pin = gpio::PinDriver::input(pins.gpio0).expect("button pin");
            button_pin
                .set_pull(Pull::Up)
                .expect("set button pin to pull up");
            button_pin
                .set_interrupt_type(InterruptType::NegEdge)
                .expect("set interrupt type");

            let notification = Notification::new();
            let notifier = notification.notifier();
            unsafe {
                button_pin
                    .subscribe(move || {
                        notifier.notify(NonZeroU32::new(0b00000000001).expect("new bitset"));
                    })
                    .expect("subscribe to button press");
            }
            button_pin
                .enable_interrupt()
                .expect("enable button interrupt");
            loop {
                notification.wait(BLOCK);
                log::info!("button pressed, wait for letting go");
                let before = EspSystemTime {}.now();
                let mut calib = false;
                while button_pin.is_low() {
                    Ets::delay_ms(10);
                    let after = EspSystemTime {}.now();
                    if (after - before).as_millis() > 2000 {
                        calib = true;
                        break;
                    }
                }
                if calib {
                    log::info!("long press, enter calibration mode");
                    let mut scales = scales.lock().expect("mutex lock");
                    scales.tare(None);
                    calibration_mode.store(true, Ordering::Relaxed);
                    break;
                }
                log::info!("button released");
                log::info!("short press, tare scales");
                let mut scales = scales.lock().expect("mutex lock");
                scales.tare(Some(5));
                button_pin
                    .enable_interrupt()
                    .expect("enable button interrupt");
            }
        }
    });

    // Main loop
    loop {
        // Check if we are in calibration mode
        if calibration_mode.load(Ordering::Relaxed) {
            // Calibration mode, display the raw readings
            let average = {
                let mut scales = scales.lock().expect("mutex lock");
                scales.read_average(10)
            };
            log::info!("Weight reading: {average}");
            let (_, adc_value) = battery_reader.read_battery_percent()?;
            screen.print_calibration(average, adc_value);
            continue;
        }

        // Normal operation
        // Read weight from loadcell and display
        {
            let mut scales = scales.lock().expect("mutex lock");
            scales.read_weight(&weight);
            // unlock mutex
        }
        let weight = weight.load(Ordering::Relaxed);
        screen.print(weight);
    }
}
