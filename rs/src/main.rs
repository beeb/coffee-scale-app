use std::time::Duration;

use anyhow::{anyhow, Result};
use embedded_graphics::{
    geometry::Point,
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    text::{Baseline, Text},
    Drawable,
};
use esp_idf_svc::{
    hal::{
        adc::ADC1,
        gpio::{Gpio12, Gpio14, Gpio34},
        i2c,
        peripherals::Peripherals,
        prelude::*,
    },
    systime::EspSystemTime,
};
use ssd1306::{
    mode::DisplayConfig, rotation::DisplayRotation, size::DisplaySize128x32, I2CDisplayInterface,
    Ssd1306,
};

use crate::weight::Scales;

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

    let i2c = i2c::I2cDriver::new(
        peripherals.i2c0,
        pins.gpio21,
        pins.gpio22,
        &i2c::I2cConfig::new().baudrate(400.kHz().into()),
    )?;
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();
    Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();
    display.flush().unwrap();

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

    let mut weight;

    let system_time = EspSystemTime {};
    let mut last_notify = Duration::default();

    loop {
        weight = scales.read_weight();
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
