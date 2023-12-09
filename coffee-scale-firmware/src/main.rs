#![no_std]
#![no_main]

// cargo espflash flash --release --bootloader C:\tb\target\xtensa-esp32-espidf\release\build\esp-idf-sys-b70e5b0d2fa5cc23\out\build\bootloader\bootloader.bin

use embedded_graphics::{
    image::Image,
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::{BinaryColor, Gray4},
    prelude::*,
    text::Text,
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

    let mut display = ssd1327_i2c::SSD1327I2C::with_wh(i2c, 128, 96);

    display.init();

    let style = MonoTextStyle::new(&FONT_6X10, Gray4::WHITE);

    Text::new("Hello rust!", Point::new(10, 10), style)
        .draw(&mut display)
        .unwrap();

    /* let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap(); */

    log::info!("Display initialized");

    /* let bmp: Bmp<Gray4> = Bmp::from_slice(include_bytes!("../assets/hex.bmp")).unwrap();
    let image = Image::new(&bmp, Point::new(32, 32));
    image.draw(&mut display).unwrap();
    log::info!("Image should be displayed"); */

    loop {
        /*     image.draw(&mut display).unwrap();
        log::info!("Image should be displayed"); */
        delay.delay_ms(500u32);

        /* display
            .send_cmd(ssd1327_i2c::Commands::DisplayModeAllON)
            .unwrap();
        delay.delay_ms(1000u32);
        display
            .send_cmd(ssd1327_i2c::Commands::DisplayModeAllOFF)
            .unwrap();
        delay.delay_ms(1000u32); */
    }
}
