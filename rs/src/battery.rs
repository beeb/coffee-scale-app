//! Battery voltage reader
//!
//! This module provides a way to read the battery voltage using the ADC peripheral.
use anyhow::Result;
use esp_idf_svc::hal::{
    adc::{
        Adc, AdcChannel,
        attenuation::DB_12,
        oneshot::{
            AdcChannelDriver, AdcDriver,
            config::{AdcChannelConfig, Calibration},
        },
    },
    gpio::ADCPin,
};

/// Battery voltage reader
pub struct BatteryReader<'d, C: AdcChannel> {
    channel: AdcChannelDriver<'d, C, AdcDriver<'d, C::AdcUnit>>,
}

impl<'d, C: AdcChannel> BatteryReader<'d, C> {
    /// Create a new battery reader
    pub fn new(
        vsense_pin: impl ADCPin<AdcChannel = C> + 'd,
        adc: impl Adc<AdcUnit = C::AdcUnit> + 'd,
    ) -> Result<Self> {
        let config = AdcChannelConfig {
            attenuation: DB_12,
            calibration: Calibration::Line,
            ..Default::default()
        };
        let channel = AdcChannelDriver::new(AdcDriver::new(adc)?, vsense_pin, &config)?;
        Ok(BatteryReader { channel })
    }

    /// Read the battery voltage and return the percentage and the raw ADC value
    ///
    /// The ADC value is the average of 10 readings, in mV.
    pub fn read_battery_percent(&mut self) -> Result<(u8, u16)> {
        let mut value = self.channel.read()?;
        for _ in 0..9 {
            value += self.channel.read()?;
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
    let voltage = 0.001_942_26f32.mul_add(f32::from(adc), 0.112_202);
    // Horner's method for readability
    (-141.608f32)
        .mul_add(voltage, 1574.53)
        .mul_add(voltage, -5694.03)
        .mul_add(voltage, 6731.1)
        .clamp(0., 100.) as u8
}
