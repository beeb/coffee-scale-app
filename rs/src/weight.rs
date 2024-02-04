//! Weight sensor module
//!
//! This module contains the code for the weight sensor. It uses the HX711 library to interface with the loadcell and
//! a Kalman filter to smooth out the readings.
//!
//! Ideally, this module would also use interrupts to detect when the HX711 is ready to read, but the current version
//! does polling instead.
use std::{
    collections::VecDeque,
    sync::atomic::{AtomicI32, Ordering},
};

use anyhow::Result;
use esp_idf_svc::hal::{
    delay::Ets,
    gpio::{self, Input, InputPin, Output, OutputPin, Pin, PinDriver},
    peripheral::Peripheral,
};
use loadcell::{
    hx711::{self, HX711},
    LoadCell,
};
use signalo_filters::{
    observe::kalman::{Config, Kalman},
    signalo_traits::{Filter, Reset, WithConfig},
};

/// How long to wait until retry if the hx711 is not ready
const LOADCELL_READY_DELAY_US: u32 = 1000;

/// How long to wait between readings
const LOADCELL_LOOP_DELAY_US: u32 = 10000;

/// How many readings to take to determine if the weight is stable
const LOADCELL_STABLE_READINGS: usize = 10;

/// How many readings to take to tare the loadcell
const LOADCELL_TARE_READINGS: usize = 5;

/// Type alias for the HX711 load sensor
pub type LoadSensor<'a, SckPin, DtPin> =
    HX711<PinDriver<'a, SckPin, Output>, PinDriver<'a, DtPin, Input>, Ets>;

/// Loadcell struct
pub struct Loadcell<'a, SckPin, DtPin>
where
    DtPin: Peripheral<P = DtPin> + Pin + InputPin,
    SckPin: Peripheral<P = SckPin> + Pin + OutputPin,
{
    sensor: LoadSensor<'a, SckPin, DtPin>,
    filter: Kalman<f32>,
}

impl<'a, SckPin, DtPin> Loadcell<'a, SckPin, DtPin>
where
    DtPin: Peripheral<P = DtPin> + Pin + InputPin,
    SckPin: Peripheral<P = SckPin> + Pin + OutputPin,
{
    /// Create a new Loadcell instance, taking ownership of the pins
    pub fn new(clock_pin: SckPin, data_pin: DtPin, scale: f32) -> Result<Self> {
        let filter = Kalman::with_config(Config {
            r: 0.5, // process noise covariance
            q: 0.1, // measurement noise covariance
            // parameters below are not tunable
            a: 1.0,
            b: 0.0,
            c: 1.0,
        });

        let hx711_sck = gpio::PinDriver::output(clock_pin)?;
        let hx711_dt = gpio::PinDriver::input(data_pin)?;

        let mut sensor = hx711::HX711::new(hx711_sck, hx711_dt, Ets);
        sensor.set_scale(scale);
        while !sensor.is_ready() {
            Ets::delay_us(LOADCELL_READY_DELAY_US);
        }
        Ok(Loadcell { sensor, filter })
    }

    /// Wait until the HX711 is ready to read
    pub fn wait_ready(&self) {
        while !self.sensor.is_ready() {
            Ets::delay_us(LOADCELL_READY_DELAY_US);
        }
    }

    /// Wait until the weight is stable
    ///
    /// This function takes readings of the loadcell and keeps iterating until the weight is stable (all readings are
    /// within 0.1g of each other)
    pub fn wait_stable(&mut self) {
        // take readings of the loadcell and keep iterating until the weight is stable
        let mut readings: VecDeque<f32> = VecDeque::with_capacity(LOADCELL_STABLE_READINGS);
        loop {
            self.wait_ready();
            let reading = self.sensor.read_scaled().expect("read scaled");
            log::info!("Waiting for stable weight: {:.4}", reading);
            if readings.len() == LOADCELL_STABLE_READINGS {
                readings.pop_front();
            }
            readings.push_back(reading);
            if readings.len() == LOADCELL_STABLE_READINGS
                && readings.iter().all(|&x| (x - reading).abs() < 0.1)
            {
                break;
            }
            Ets::delay_us(LOADCELL_LOOP_DELAY_US);
        }
    }

    /// Tare the loadcell
    pub fn tare(&mut self, num_samples: Option<usize>) {
        self.filter = self.filter.clone().reset();
        self.sensor
            .tare(num_samples.unwrap_or(LOADCELL_TARE_READINGS))
    }

    /// Read the loadcell and return the average of `count` readings, in raw units
    pub fn read_average(&mut self, count: usize) -> i32 {
        let mut current;
        let mut average: f32 = 0.0;
        for n in 1..=count {
            self.wait_ready();
            current = self.sensor.read().expect("read with offset") as f32;
            Ets::delay_us(LOADCELL_LOOP_DELAY_US);
            average += (current - average) / (n as f32);
        }
        average as i32
    }

    /// Read the loadcell and store the weight in grams into the `weight` atomic integer
    ///
    /// This function reads the loadcell and returns the weight in grams, after filtering.
    pub fn read_weight(&mut self, weight: &AtomicI32) {
        self.wait_ready();
        let reading = self.sensor.read_scaled().expect("read scaled");
        log::info!("Raw reading: {reading:.2}");
        let filtered = self.filter.filter(reading);
        log::info!("Filtered reading: {filtered:.2}");
        // round to 0.10g, multiply by 100 to cast as integer with 2 decimal places
        let val = (filtered / 0.1).round() * 10.;
        weight.store(val as i32, Ordering::Relaxed);
    }
}
