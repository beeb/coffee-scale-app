# Firmware for the ESP32 based smart scale.

The scale uses a HX711 loadcell amplifier to read the weight and a SSD1306 OLED display to show the weight and
battery level.

The scale is also a Bluetooth Low Energy (BLE) peripheral that exposes a weight characteristic and a battery
characteristic. It also notifies subscribers of the weight characteristic approx. every 200ms.

The scale can be calibrated by pressing the button for 2 seconds. The calibration mode shows the raw loadcell
readings and the ADC value of the battery voltage. The calibration mode is exited by pressing the button again.
The values can be then used to calculate the scaling factor (`LOADCELL_SCALING`) as well as adjust the battery level
conversion function (`battery::adc_to_percent`).

At the moment, there is no interactive way to set the scaling factor, so it has to be hardcoded in the source code.

The firmware is configured to be built for a 26MHz crystal frequency (for the SparkFun ESP32 Thing). Adjust accordingly
if you target other devices.

## How to flash

Instructions below taken from [ivmarkov's demo repository](https://github.com/ivmarkov/rust-esp32-std-demo).

- Install the [Rust toolchain](https://rustup.rs/)
- Install the [Rust Espressif compiler toolchain and the Espressif LLVM Clang toolchain](https://github.com/esp-rs/rust-build)
  - This is necessary, because support for the Xtensa architecture (ESP32 / ESP32-S2 / ESP32-S3) is not upstreamed in
    LLVM yet
- Switch to the `esp` toolchain from the pre-built binaries: `rustup default esp`
  - (You can also skip this step and switch to the `esp` toolchain _for this crate only_ by executing
    `rustup override set esp` inside the `rs` directory once you have cloned the repo)
- If using the custom Espressif Clang, make sure that you DON'T have a system Clang installed as well, because even if
  you have the Espressif one first on your `$PATH`, Bindgen will still pick the system one
  - A workaround that does not require uninstalling the system Clang is to do
    `export LIBCLANG_PATH=<path to the Espressif Clang lib directory>` prior to continuing the build process
- `cargo install ldproxy`
- `cargo install cargo-espflash`
  - [documentation](https://github.com/esp-rs/espflash/tree/main/cargo-espflash)
- Clone this repo: `git clone https://github.com/beeb/coffee-scale-app.git`
- Enter the `rs` directory: `cd coffee-scale-app/rs`
- Connect the esp32 with USB
- `cargo espflash flash --release`
  - The appropriate device should be found automatically

### Note for Windows

On Windows, there is a path length limit which makes it hard to work with this project in the user directory.
Personally, I had to clone the repo directly at the root of `C:\` and rename it to a two-letter name `cs` to get it to
compile.
