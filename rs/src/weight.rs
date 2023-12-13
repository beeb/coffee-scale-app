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

pub const LOADCELL_READY_DELAY: u32 = 5000;

pub type LoadSensor<'a, SckPin, DtPin> =
    HX711<PinDriver<'a, SckPin, Output>, PinDriver<'a, DtPin, Input>, Delay>;

pub fn init_load_sensor<'a, DtPin, SckPin>(
    data_pin: DtPin,
    clock_pin: SckPin,
) -> Result<LoadSensor<'a, SckPin, DtPin>>
where
    DtPin: Peripheral<P = DtPin> + Pin + InputPin,
    SckPin: Peripheral<P = SckPin> + Pin + OutputPin,
{
    let hx711_sck = gpio::PinDriver::output(clock_pin)?;
    let hx711_dt = gpio::PinDriver::input(data_pin)?;

    let delay = Delay::new_default();
    let mut load_sensor = hx711::HX711::new(hx711_sck, hx711_dt, delay);
    load_sensor.set_scale(1.57e-4);
    while !load_sensor.is_ready() {
        delay.delay_ms(10);
    }
    Ok(load_sensor)
}

pub fn read_weight<DtPin, SckPin>(
    load_sensor: &mut LoadSensor<'_, SckPin, DtPin>,
    delay: &Delay,
) -> i16
where
    DtPin: Pin,
    SckPin: Pin,
{
    while !load_sensor.is_ready() {
        delay.delay_us(LOADCELL_READY_DELAY);
    }
    let reading = (load_sensor.read_scaled() / 0.05).round() * 0.05; // rounded to 0.05g
    (reading * 100.) as i16
}
