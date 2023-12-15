use std::{
    collections::VecDeque,
    sync::atomic::{AtomicI32, Ordering},
};

use anyhow::Result;
use esp_idf_svc::hal::{
    delay::Delay,
    gpio::{self, Input, InputPin, Output, OutputPin, Pin, PinDriver},
    peripheral::Peripheral,
};
use loadcell::{
    hx711::{self, HX711},
    LoadCell,
};

const LOADCELL_READY_DELAY_US: u32 = 1000;
const LOADCELL_LOOP_DELAY_US: u32 = 10000;
const LOADCELL_STABLE_READINGS: usize = 10;
const LOADCELL_TARE_READINGS: usize = 5;

pub type LoadSensor<'a, SckPin, DtPin> =
    HX711<PinDriver<'a, SckPin, Output>, PinDriver<'a, DtPin, Input>, Delay>;

pub struct Scales<'a, SckPin, DtPin>
where
    DtPin: Peripheral<P = DtPin> + Pin + InputPin,
    SckPin: Peripheral<P = SckPin> + Pin + OutputPin,
{
    sensor: LoadSensor<'a, SckPin, DtPin>,
    delay: Delay,
}

impl<'a, SckPin, DtPin> Scales<'a, SckPin, DtPin>
where
    DtPin: Peripheral<P = DtPin> + Pin + InputPin,
    SckPin: Peripheral<P = SckPin> + Pin + OutputPin,
{
    pub fn new(clock_pin: SckPin, data_pin: DtPin) -> Result<Self> {
        let hx711_sck = gpio::PinDriver::output(clock_pin)?;
        let hx711_dt = gpio::PinDriver::input(data_pin)?;

        let delay = Delay::new_default();
        let mut sensor = hx711::HX711::new(hx711_sck, hx711_dt, delay);
        sensor.set_scale(6.492e-4);
        while !sensor.is_ready() {
            delay.delay_ms(10);
        }
        Ok(Scales { sensor, delay })
    }

    pub fn wait_ready(&self) {
        while !self.sensor.is_ready() {
            //log::warn!("Loadcell not ready");
            self.delay.delay_us(LOADCELL_READY_DELAY_US);
        }
    }

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
            self.delay.delay_us(LOADCELL_LOOP_DELAY_US);
        }
    }

    pub fn tare(&mut self, num_samples: Option<usize>) {
        self.sensor
            .tare(num_samples.unwrap_or(LOADCELL_TARE_READINGS));
    }

    pub fn read_average(&mut self, count: usize) -> i32 {
        let mut current;
        let mut average: f32 = 0.0;
        for n in 1..=count {
            self.wait_ready();
            current = self.sensor.read().expect("read with offset") as f32;
            self.delay.delay_us(LOADCELL_LOOP_DELAY_US);
            average += (current - average) / (n as f32);
        }
        average as i32
    }

    pub fn read_weight(&mut self, weight: &AtomicI32) {
        self.wait_ready();
        let reading = self.sensor.read_scaled().expect("read scaled");
        let val = (reading * 20.).round() / 20.; // rounded to 0.05g
        weight.store((val * 100.) as i32, Ordering::Relaxed);
    }
}
