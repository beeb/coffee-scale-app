//#![feature(box_into_inner)]
use std::{
    ffi::{c_void, CString},
    ptr,
    sync::atomic::{AtomicI16, Ordering},
};

use anyhow::{anyhow, Result};
use esp_idf_svc::{
    hal::{
        adc::ADC1,
        delay::Delay,
        gpio::{Gpio12, Gpio14, Gpio34},
        i2c,
        peripherals::Peripherals,
        prelude::*,
    },
    sys::{vTaskDelete, xTaskCreatePinnedToCore},
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

    let mut weight: AtomicI16 = AtomicI16::new(0);

    spawn(|| {
        let delay = Delay::new_default();
        loop {
            let weight = weight.load(Ordering::Relaxed);
            ble::WEIGHT
                .get()
                .unwrap()
                .lock()
                .set_value(&weight.to_be_bytes())
                .notify();
            delay.delay_ms(200);
        }
    });

    loop {
        scales.read_weight(&mut weight);
        let weight = weight.load(Ordering::Relaxed);
        screen.print(weight);
    }
}

extern "C" fn spawn_closure<F: FnOnce()>(arg: *mut c_void) {
    let closure: Box<F> = unsafe { Box::from_raw(arg as *mut F) };
    //let closure = std::boxed::Box::<F>::into_inner(closure);
    closure();
    unsafe {
        vTaskDelete(ptr::null_mut());
    }
}

fn spawn<F: FnOnce() + Send>(closure: F) {
    let fn_name = CString::new("bt").unwrap();
    let closure_ptr = Box::leak(Box::new(closure));
    unsafe {
        xTaskCreatePinnedToCore(
            Some(spawn_closure::<F>),
            fn_name.as_ptr(),
            2048,
            closure_ptr as *mut F as *mut c_void,
            2,
            ptr::null_mut(),
            1,
        );
    }
}
