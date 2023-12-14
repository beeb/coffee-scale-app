use embedded_graphics::{
    geometry::Point,
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
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

pub struct Screen<'a> {
    pub display: Display<'a>,
}

impl<'a> Screen<'a> {
    pub fn new(interface: I2CInterface<I2cDriver<'a>>) -> Self {
        let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        display.init().unwrap();
        let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("../assets/hex.raw"), 26);
        let im = Image::new(&raw, Point::new(51, 1));
        im.draw(&mut display).unwrap();
        display.flush().unwrap();
        Screen { display }
    }
}
