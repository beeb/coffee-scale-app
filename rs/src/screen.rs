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

const CUSTOM_FONT: MonoFont = MonoFont {
    image: ImageRaw::new(include_bytes!("../assets/font.raw"), 266),
    glyph_mapping: &StrGlyphMapping::new("0123456789.-gb", 10),
    character_size: Size::new(19, 30),
    character_spacing: 2,
    baseline: 30,
    underline: DecorationDimensions::default_underline(30),
    strikethrough: DecorationDimensions::default_strikethrough(30),
};

pub struct Screen<'a> {
    pub display: Display<'a>,
    pub battery: u8,
}

impl<'a> Screen<'a> {
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

    pub fn set_battery(&mut self, battery: u8) {
        self.battery = battery;
    }

    pub fn print(&mut self, number: i32) {
        let character_style = MonoTextStyle::new(&CUSTOM_FONT, BinaryColor::On);
        let text_style = TextStyleBuilder::new()
            .baseline(Baseline::Bottom)
            .alignment(Alignment::Right)
            .build();
        self.display.clear_buffer();

        if number > -10_000 {
            Text::with_text_style(
                &format!("{:02}", number.abs() % 100),
                Point::new(116, 31),
                character_style,
                text_style,
            )
            .draw(&mut self.display)
            .expect("draw decimals");
            Text::with_text_style(
                &format!("{}.", number / 100),
                Point::new(89, 31),
                character_style,
                text_style,
            )
            .draw(&mut self.display)
            .expect("draw digits");
        } else {
            Text::with_text_style(
                &format!("{}", number.abs() % 100 / 10),
                Point::new(116, 31),
                character_style,
                text_style,
            )
            .draw(&mut self.display)
            .expect("draw decimals");
            Text::with_text_style(
                &format!("{}.", number / 100),
                Point::new(110, 31),
                character_style,
                text_style,
            )
            .draw(&mut self.display)
            .expect("draw digits");
        }

        Text::with_text_style("g", Point::new(136, 31), character_style, text_style)
            .draw(&mut self.display)
            .expect("draw gram symbol");
        if self.battery < 20 {
            Text::with_text_style("b", Point::new(136, 8), character_style, text_style)
                .draw(&mut self.display)
                .expect("draw battery symbol");
        }
        self.display.flush().expect("flush display buffer");
    }

    pub fn print_calibration(&mut self, number: i32) {
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
        .expect("draw decimals");
        self.display.flush().expect("flush display buffer");
    }
}
