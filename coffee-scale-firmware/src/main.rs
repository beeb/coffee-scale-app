#![no_std]
#![no_main]

use embedded_graphics::{
    image::Image,
    pixelcolor::{BinaryColor, Gray4},
    prelude::*,
};
use esp_backtrace as _;
use esp_wifi::{initialize, EspWifiInitFor};
use hal::{
    clock::ClockControl, gpio::IO, i2c::I2C, peripherals::Peripherals, prelude::*,
    timer::TimerGroup, Delay, Rng,
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use tinybmp::Bmp;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    let timer_group = TimerGroup::new(peripherals.TIMG1, &clocks);
    let timer = timer_group.timer0;

    let mut delay = Delay::new(&clocks);

    esp_println::logger::init_logger_from_env();
    log::info!("Logging started");

    let _init = initialize(
        EspWifiInitFor::Ble,
        timer,
        Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    )
    .unwrap();
    log::info!("BLE initialized");

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio21,
        io.pins.gpio22,
        100u32.kHz(),
        &clocks,
    );

    let mut driver = ssd1327_i2c::SSD1327I2C::new(i2c);

    driver.init();

    /* let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap(); */

    log::info!("Display initialized");

    let bmp: Bmp<Gray4> = Bmp::from_slice(include_bytes!("../assets/hex.bmp")).unwrap();
    let image = Image::new(&bmp, Point::new(32, 32));
    image.draw(&mut driver).unwrap();
    log::info!("Image should be displayed");

    loop {
        image.draw(&mut driver).unwrap();
        log::info!("Image should be displayed");
        delay.delay_ms(500u32);
    }
}
