//! Display driver for the SSD1306 OLED display.
//!
//! The display is used to show the weight and battery level. It also shows the raw loadcell readings and the ADC value
//! of the battery voltage when in calibration mode.
//!
//! The display is driven by the `ssd1306` crate, which provides a high-level API for the SSD1306 display driver. The
//! `embedded-graphics` crate is used to draw text and images on the display.
//!
//! The display is a 128x32 monochrome OLED display, which is connected to the ESP32 over I2C.
use embedded_graphics::{
    geometry::{Point, Size},
    image::{Image, ImageRaw},
    mono_font::{
        ascii::FONT_7X13, mapping::StrGlyphMapping, DecorationDimensions, MonoFont, MonoTextStyle,
    },
    pixelcolor::BinaryColor,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
    Drawable,
};
use esp_idf_svc::hal::i2c::I2cDriver;
use ssd1306::{
    mode::{BufferedGraphicsMode, DisplayConfig},
    prelude::I2CInterface,
    rotation::DisplayRotation,
    size::DisplaySize128x32,
    Ssd1306,
};

type Display<'a> = Ssd1306<
    I2CInterface<I2cDriver<'a>>,
    DisplaySize128x32,
    BufferedGraphicsMode<DisplaySize128x32>,
>;

/// Custom font for the display (digits and symbols in normal operation mode)
const CUSTOM_FONT: MonoFont = MonoFont {
    image: ImageRaw::new(include_bytes!("../assets/font.raw"), 266),
    glyph_mapping: &StrGlyphMapping::new("0123456789.-gb", 10),
    character_size: Size::new(19, 30),
    character_spacing: 2,
    baseline: 30,
    underline: DecorationDimensions::default_underline(30),
    strikethrough: DecorationDimensions::default_strikethrough(30),
};

/// Screen struct
pub struct Screen<'a> {
    pub display: Display<'a>,
    pub battery: u8,
}

impl<'a> Screen<'a> {
    /// Create a new Screen instance, initializing the battery to 100% for now
    pub fn new(interface: I2CInterface<I2cDriver<'a>>) -> Self {
        let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        display.init().expect("display init");
        let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("../assets/hex.raw"), 26);
        let im = Image::new(&raw, Point::new(51, 1));
        im.draw(&mut display).expect("draw splash image");
        display.flush().expect("flush display buffer");
        Screen {
            display,
            battery: 100,
        }
    }

    /// Set the battery level (displays a battery icon if the battery level is below 20%)
    pub fn set_battery(&mut self, battery: u8) {
        self.battery = battery;
    }

    /// Print the weight and symbols on the display
    pub fn print(&mut self, number: i32) {
        // use custom font
        let character_style = MonoTextStyle::new(&CUSTOM_FONT, BinaryColor::On);
        let text_style = TextStyleBuilder::new()
            .baseline(Baseline::Bottom)
            .alignment(Alignment::Right)
            .build();
        // clear the display buffer
        self.display.clear_buffer();

        // draw the first decimal place value
        // to draw two decimal places: &format!("{:02}", number.abs() % 100)
        Text::with_text_style(
            &format!("{}", number.abs() % 100 / 10),
            Point::new(116, 31),
            character_style,
            text_style,
        )
        .draw(&mut self.display)
        .expect("draw decimals");

        // Draw the digits left of the decimal separator
        if number > -100 && number < 0 {
            // -0.xx would not show the minus sign due to the divison by 100 being 0.
            Text::with_text_style(
                &format!("-{}.", number / 100),
                Point::new(110, 31), // x coordinate if drawing two decimal places: 89
                character_style,
                text_style,
            )
            .draw(&mut self.display)
            .expect("draw digits");
        } else {
            Text::with_text_style(
                &format!("{}.", number / 100),
                Point::new(110, 31), // x coordinate if drawing two decimal places: 89
                character_style,
                text_style,
            )
            .draw(&mut self.display)
            .expect("draw digits");
        }

        // draw the gram symbol
        Text::with_text_style("g", Point::new(136, 31), character_style, text_style)
            .draw(&mut self.display)
            .expect("draw gram symbol");

        // draw the battery symbol if the battery level is below 20%
        if self.battery < 20 {
            Text::with_text_style("b", Point::new(136, 8), character_style, text_style)
                .draw(&mut self.display)
                .expect("draw battery symbol");
        }

        self.display.flush().expect("flush display buffer");
    }

    /// Print the raw loadcell reading and the ADC value of the battery voltage (calibration mode)
    pub fn print_calibration(&mut self, number: i32, adc: u16) {
        let character_style = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);
        let text_style = TextStyleBuilder::new()
            .baseline(Baseline::Bottom)
            .alignment(Alignment::Left)
            .build();
        self.display.clear_buffer();
        Text::with_text_style(
            &format!("calib: {number:>7}"),
            Point::new(2, 15),
            character_style,
            text_style,
        )
        .draw(&mut self.display)
        .expect("draw loadcell");
        Text::with_text_style(
            &format!("adc: {adc}"),
            Point::new(2, 30),
            character_style,
            text_style,
        )
        .draw(&mut self.display)
        .expect("draw adc");
        self.display.flush().expect("flush display buffer");
    }
}
