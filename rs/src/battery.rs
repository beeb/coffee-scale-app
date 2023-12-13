use anyhow::Result;
use esp_idf_svc::hal::{adc, gpio::ADCPin, peripheral::Peripheral};

pub fn read_battery_percent<VPin, Adc>(vsense_pin: VPin, adc: Adc) -> Result<u8>
where
    VPin: Peripheral<P = VPin> + ADCPin<Adc = Adc>,
    Adc: Peripheral<P = Adc> + adc::Adc,
{
    let mut analog =
        adc::AdcChannelDriver::<{ adc::attenuation::DB_11 }, _>::new(vsense_pin).unwrap();
    let mut powered_adc1 = adc::AdcDriver::new(adc, &adc::config::Config::new().calibration(true))?;
    let mut value = powered_adc1.read(&mut analog)?;
    for _ in 0..9 {
        value += powered_adc1.read(&mut analog)?;
    }
    value /= 10;

    Ok(adc_to_percent(value))
}

fn adc_to_percent(adc: u16) -> u8 {
    let adc_float = adc as f32;
    match adc {
        2400.. => (0.10169492 * adc_float - 149.966).clamp(0., 100.).floor() as u8, // 4.1-4.2V = 94-100%
        2341.. => (0.18965517 * adc_float - 360.983).floor() as u8, // 4.0-4.1V = 83-94%
        2282.. => (0.18644068 * adc_float - 353.458).floor() as u8, // 3.9-4.0V = 72-83%
        2224.. => (0.22413793 * adc_float - 439.483).floor() as u8, // 3.8-3.9V = 59-72%
        2165.. => (0.15254237 * adc_float - 280.254).floor() as u8, // 3.7-3.8V = 50-59%
        2107.. => (0.29310345 * adc_float - 584.569).floor() as u8, // 3.6-3.7V = 33-50%
        2048.. => (0.30508475 * adc_float - 609.814).floor() as u8, // 3.5-3.6V = 15-33%
        1990.. => (0.15517241 * adc_float - 302.793).floor() as u8, // 3.4-3.5V = 6-15%
        1931.. => (0.10169492 * adc_float - 196.373).floor() as u8, // 3.3-3.4V = 0-6%
        _ => 0,
    }
}
