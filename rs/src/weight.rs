use anyhow::Result;
use esp_idf_svc::hal::{
    delay::Delay,
    gpio::{self, Gpio13, Gpio14, Input, Output, PinDriver},
    peripherals::Peripherals,
};
use loadcell::{
    hx711::{self, HX711},
    LoadCell,
};

type SckPin = PinDriver<'static, Gpio13, Output>;
type DtPin = PinDriver<'static, Gpio14, Input>;

pub fn init_load_sensor() -> Result<HX711<SckPin, DtPin, Delay>> {
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let hx711_sck = gpio::PinDriver::output(pins.gpio13)?;
    let hx711_dt = gpio::PinDriver::input(pins.gpio14)?;

    let delay = Delay::new_default();
    let mut load_sensor = hx711::HX711::new(hx711_sck, hx711_dt, delay);
    load_sensor.set_scale(1544.667);
    while !load_sensor.is_ready() {
        delay.delay_ms(10);
    }
    load_sensor.tare(1);
    Ok(load_sensor)
}

pub fn read_weight(load_sensor: &mut HX711<SckPin, DtPin, Delay>) -> i16 {
    let reading = (load_sensor.read_scaled() / 0.05).round() * 0.05; // rounded to 0.05g
    (reading * 100.) as i16
}
