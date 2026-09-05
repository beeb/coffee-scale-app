# Firmware for the ESP32 based smart scale

The scale uses a HX711 loadcell amplifier to read the weight and a SSD1306 OLED display to show the weight and battery
level.

The scale is also a Bluetooth Low Energy (BLE) peripheral that exposes a weight characteristic and a battery
characteristic. It also notifies subscribers of the weight characteristic approx. every 200ms.

This is a `std` (ESP-IDF based) Rust project: it uses `esp-idf-svc` / `esp-idf-hal` on top of the ESP-IDF C SDK, which
is downloaded and built automatically by the `esp-idf-sys` build script.

## Target hardware

The firmware is built for the [SparkFun ESP32 Thing](https://learn.sparkfun.com/tutorials/esp32-thing-hookup-guide),
which uses a **26MHz crystal** instead of the 40MHz crystal found on most other ESP32 boards. This is configured in
[`sdkconfig.defaults`](./sdkconfig.defaults):

```
CONFIG_XTAL_FREQ_26=y
```

If you target a board with a 40MHz crystal, change this to `CONFIG_XTAL_FREQ_40=y` (or remove the line, since 40MHz is
the ESP-IDF default).

Pinout used by the firmware:

| Function                  | GPIO                                  |
| ------------------------- | ------------------------------------- |
| I2C SDA / SCL (SSD1306)   | `21` / `22`                           |
| HX711 SCK / DT            | `13` / `14`                           |
| Battery sense (ADC1)      | `34`                                  |
| Tare / calibration button | `0` (on-board `0` button, active low) |

## Prerequisites

Espressif's [Rust on ESP book](https://docs.espressif.com/projects/rust/book/getting-started/index.html) is the
reference for the toolchain setup. TL;DR:

1. Install [Rust with `rustup`](https://rustup.rs/).

2. Install [`espup`](https://github.com/esp-rs/espup) and the Xtensa toolchain:

   ```sh
   cargo install espup --locked
   espup install
   ```

   [`rust-toolchain.toml`](./rust-toolchain.toml) configures this workspace to use the `esp` toolchain.

3. Activate the environment from `espup`. **This has to be done in every new shell**:

   ```powershell
   # Windows (PowerShell)
   . $env:USERPROFILE\export-esp.ps1
   ```

   ```sh
   # Linux / macOS
   . $HOME/export-esp.sh
   ```

4. Install the linker wrapper and the flashing tool:

   ```sh
   cargo install ldproxy --locked
   cargo install espflash --locked
   ```

   `ldproxy` is required by [`.cargo/config.toml`](./.cargo/config.toml); `espflash` is used as the cargo `runner`.
   [`cargo-espflash`](https://github.com/esp-rs/espflash/tree/main/cargo-espflash) can be installed as well if you
   prefer the `cargo espflash ...` subcommand form.

5. Python 3 must be available on the `PATH`. ESP-IDF uses it to setup its tooling. See the
   [`esp-idf-template` prerequisites](https://github.com/esp-rs/esp-idf-template#prerequisites) for the full list of
   host requirements.

### ESP-IDF

The version is pinned in [`.cargo/config.toml`](./.cargo/config.toml):

```toml
[env]
ESP_IDF_VERSION = "v5.5.4"
```

On the first `cargo build`, the `esp-idf-sys` build script clones that tag into `.embuild/` and downloads the matching
ESP-IDF toolchains. This will take a few GBs of space and take a while the first time.

Since running the build does not clean up any potential leftovers from an older ESP-IDF version, and those can conflict
on the `PATH`, it's a good idea to cleanup all artifacts before the first build (the `target` directory contains some
artifacts too):

```sh
rm -rf .embuild
cargo clean
cargo build --release
```

## Build

```sh
cargo build --release
```

## Flash and monitor

Connect the ESP32 over USB, then:

```sh
cargo run --release
```

The `runner` configured in [`.cargo/config.toml`](./.cargo/config.toml) invokes `espflash flash --monitor --no-stub`,
which flashes the firmware and then opens the serial monitor.

The equivalent using `cargo-espflash` is:

```sh
cargo espflash flash --release --monitor --no-stub
```

`--no-stub` is required on this board: the flasher stub does not complete its handshake on boards with a 26MHz crystal
(see [esp-flasher-stub#63](https://github.com/esp-rs/esp-flasher-stub/issues/63)). Flashing is slower without the stub,
but it works.

### If the board is not detected

With espflash 4.x the automatic reset into download mode does not reliably work on this board, and the flash fails with
`Error while connecting to device`. Put it into download mode by hand instead:

1. Hold the `0` button.
2. Press `RST` shortly, while keeping `0` held.
3. Release `0`.

Then flash while skipping the reset sequence entirely:

```sh
cargo build --release # making sure the binary is up-to-date
espflash flash --no-stub --before no-reset --port <PORT> target/xtensa-esp32-espidf/release/firmware-rs
```

Tap `RST` afterwards to leave the bootloader if it didn't automatically reset.

## Calibration

The scale can be calibrated by pressing the button for 2 seconds. The calibration mode shows the raw loadcell readings
and the ADC value of the battery voltage (in mV). The calibration mode is exited by resetting the board. The values can
then be used to calculate the scaling factor (`LOADCELL_SCALING` in [`src/main.rs`](./src/main.rs)) as well as to adjust
the battery level conversion function (`adc_to_percent` in [`src/battery.rs`](./src/battery.rs)).

At the moment, there is no interactive way to set the scaling factor, so it has to be hardcoded in the source code.

## Note for Windows

There is a path length limit which makes it hard to work with this project in the user directory. Personally, I had to
clone the repo directly at the root of `C:\` and rename it to a two-letter name `cs` to get it to compile.

Enabling long paths helps as well, both in Windows and in git:

```sh
git config --global core.longpaths true
```
