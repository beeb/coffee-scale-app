use anyhow::Result;
use esp_idf_svc::hal::{
    adc::{self, Adc},
    gpio::ADCPin,
    peripheral::Peripheral,
};

pub struct BatteryReader<'a, ADC: Adc + 'a, PIN: Peripheral<P = PIN> + ADCPin<Adc = ADC>> {
    adc: adc::AdcDriver<'a, ADC>,
    analog: adc::AdcChannelDriver<'a, { adc::attenuation::DB_11 }, PIN>,
}

impl<'a, ADC: Peripheral<P = ADC> + Adc, PIN: Peripheral<P = PIN> + ADCPin<Adc = ADC>>
    BatteryReader<'a, ADC, PIN>
{
    pub fn new(vsense_pin: PIN, adc: ADC) -> Result<Self> {
        let analog =
            adc::AdcChannelDriver::<{ adc::attenuation::DB_11 }, _>::new(vsense_pin).expect("adc");
        Ok(BatteryReader {
            adc: adc::AdcDriver::new(adc, &adc::config::Config::new().calibration(true))?,
            analog,
        })
    }

    pub fn read_battery_percent(&mut self) -> Result<(u8, u16)> {
        let mut value = self.adc.read(&mut self.analog)?;
        for _ in 0..9 {
            value += self.adc.read(&mut self.analog)?;
        }
        value /= 10;

        Ok((adc_to_percent(value), value))
    }
}

/// Convert ADC reading into voltage and then percentage
///
/// Calibration values:
///
/// 2080: 4.15V
/// 2055: 4.1V
/// 2000: 4.0V
/// 1949: 3.9V
/// 1897: 3.8V
/// 1848: 3.7V
/// 1795: 3.6V
/// 1746: 3.5V
/// 1692: 3.4V
/// 1642: 3.3V
///
/// Least-squares fit: V = 0.112202 + 0.00194226 ADC
///
/// Conversion to percentage (extracted from a chart a long time ago, can't remember the source):
///
/// 4.2V: 100%
/// 4.1V: 94%
/// 4.0V: 83%
/// 3.9V: 72%
/// 3.8V: 59%
/// 3.7V: 50%
/// 3.6V: 33%
/// 3.5V: 15%
/// 3.4V: 6%
/// 3.3V: 0%
///
/// Cubic fit: y = -141.608 x^3 + 1574.53 x^2 - 5694.03 x + 6731.1
fn adc_to_percent(adc: u16) -> u8 {
    let voltage = 0.112202 + 0.00194226 * (adc as f32);
    (-141.608 * voltage.powi(3) + 1574.53 * voltage.powi(2) - 5694.03 * voltage + 6731.1)
        .clamp(0., 100.) as u8
}
