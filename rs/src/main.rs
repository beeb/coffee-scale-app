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
        delay::{Delay, BLOCK},
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

use crate::{screen::Screen, weight::Scales};

mod battery;
mod ble;
mod screen;
mod weight;

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

    // read battery level
    let battery_percent = battery::read_battery_percent(pins.gpio34, peripherals.adc1)?;
    log::info!("Battery level: {}%", battery_percent);
    screen.set_battery(battery_percent);
    ble::BATTERY
        .get()
        .ok_or_else(|| anyhow!("Battery characteristic not initialized"))?
        .lock()
        .set_value(&battery_percent.to_be_bytes());

    let scales = Arc::new(Mutex::new(Scales::new(pins.gpio12, pins.gpio14)?));
    // tare the scales after it's become stable
    {
        let mut scales = scales.lock().expect("mutex lock");
        scales.wait_stable();
        scales.tare(None);
    }

    let weight: Arc<AtomicI32> = Arc::new(AtomicI32::new(0));
    let shared_weight = Arc::clone(&weight);

    // Bluetooth reporting thread
    thread::spawn(move || {
        let notification = Notification::new();
        let timer_conf = config::Config::new().auto_reload(true);
        let mut timer = TimerDriver::new(peripherals.timer00, &timer_conf).expect("timer");
        timer
            .set_alarm(timer.tick_hz() / 5)
            .expect("set timer alarm");
        let notifier = notification.notifier();
        unsafe {
            timer
                .subscribe(move || {
                    let bitset = 0b00000000001;
                    notifier.notify(NonZeroU32::new(bitset).expect("new bitset"));
                })
                .expect("subscribe to timer");
        }
        timer.enable_interrupt().expect("enable timer interrupt");
        timer.enable_alarm(true).expect("enable timer alarm");
        timer.enable(true).expect("enable timer");
        loop {
            notification.wait(BLOCK);
            let weight = shared_weight.load(Ordering::Relaxed);
            ble::WEIGHT
                .get()
                .expect("weight characteristic not initialized")
                .lock()
                .set_value(&weight.to_be_bytes())
                .notify();
        }
    });

    // Tare/Calibration button thread
    let calibration_mode = Arc::new(AtomicBool::new(false));
    let shared_calibration_mode = Arc::clone(&calibration_mode);
    let shared_scales = Arc::clone(&scales);
    thread::spawn(move || {
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
                    let bitset = 0b00000000001;
                    notifier.notify(NonZeroU32::new(bitset).expect("new bitset"));
                })
                .expect("subscribe to button press");
        }
        button_pin
            .enable_interrupt()
            .expect("enable button interrupt");
        let delay = Delay::new_default();
        loop {
            notification.wait(BLOCK);
            log::info!("button pressed, wait for letting go");
            let before = EspSystemTime {}.now();
            let mut calib = false;
            while button_pin.is_low() {
                delay.delay_ms(10);
                let after = EspSystemTime {}.now();
                if (after - before).as_millis() > 2000 {
                    calib = true;
                    break;
                }
            }
            if calib {
                log::info!("long press, enter calibration mode");
                let mut scales = shared_scales.lock().expect("mutex lock");
                scales.tare(None);
                shared_calibration_mode.store(true, Ordering::Relaxed);
                break;
            }
            log::info!("button released");
            log::info!("short press, tare scales");
            let mut scales = shared_scales.lock().expect("mutex lock");
            scales.tare(Some(20));
            button_pin
                .enable_interrupt()
                .expect("enable button interrupt");
        }
    });

    // Main loop
    loop {
        if calibration_mode.load(Ordering::Relaxed) {
            let average = {
                let mut scales = scales.lock().expect("mutex lock");
                scales.read_average(10)
            };
            log::info!("Weight reading: {average}");
            screen.print_calibration(average);
            continue;
        }
        // Read weight from loadcell
        {
            let mut scales = scales.lock().expect("mutex lock");
            scales.read_weight(&weight);
        }
        let weight = weight.load(Ordering::Relaxed);
        screen.print(weight);
    }
}
