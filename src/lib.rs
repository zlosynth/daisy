#![no_std]

//! Rust `no_std`, `embedded_hal` board support package for the Electro-Smith Daisy platform.
//!
//! * [Documentation](https://zlosynth.com/daisy)
//! * [Crate (crates.io)](https://crates.io/crates/daisy)
//! * [Repository (github.com)](https://github.com/zlosynth/daisy)
//!
//! # Supported boards
//!
//! Currently this library supports following boards:
//!
//! * [Daisy Seed](https://www.electro-smith.com/daisy/daisy) (codec AK4556), `seed`
//! * [Daisy Seed 1.1](https://www.electro-smith.com/daisy/daisy) (codec WM8731), `seed_1_1`
//! * [Daisy Patch SM](https://www.electro-smith.com/daisy/patch-sm) (codec PCM3060), `patch_sm`
//!
//! Select the board by using its respective feature.
//!
//! # API stability
//!
//! I am still trying to figure out a good API for the project. Expect it to change.
//! To mitigate breakage of your code on library update, use macros defined under
//! `board.rs` to initialize resources whenever possible.
//!
//! # HAL compatibility
//!
//! This library is closely tied to [stm32h7xx-hal](https://github.com/stm32-rs/stm32h7xx-hal/).
//! Make sure to use compatible versions in your `Cargo.toml`.
//!
//! | **Daisy**   | **HAL** |
//! |-------------|---------|
//! | `0.8`       | `0.14`  |
//! | `0.2`-`0.7` | `0.12`  |
//! | `0.1`       | `0.11`  |
//!
//! # Usage
//!
//! See the [examples/](https://github.com/zlosynth/daisy/tree/main/examples)
//! directory to find usage examples:
//!
//! * [ADC](https://github.com/zlosynth/daisy/blob/main/examples/adc.rs)
//! * [Audio](https://github.com/zlosynth/daisy/blob/main/examples/audio.rs)
//! * [Audio with RTIC](https://github.com/zlosynth/daisy/blob/main/examples/audio_rtic.rs)
//! * [Blinky](https://github.com/zlosynth/daisy/blob/main/examples/blinky.rs)
//! * [Blinky with RTIC](https://github.com/zlosynth/daisy/blob/main/examples/blinky_rtic.rs)
//! * [Flash storage](https://github.com/zlosynth/daisy/blob/main/examples/flash.rs)
//! * [OLED display](https://github.com/zlosynth/daisy/blob/main/examples/oled.rs)
//! * [SDRAM memory](https://github.com/zlosynth/daisy/blob/main/examples/sdram.rs)
//! * [SD card](https://github.com/zlosynth/daisy/blob/main/examples/sdmmc.rs)
//!
//! ``` sh
//! make flash WHAT=blinky BOARD=seed_1_1
//! ```

#[cfg(all(feature = "seed_1_1", any(feature = "seed", feature = "patch_sm")))]
compile_error!("only a single target board must be selected");

#[cfg(all(feature = "seed", any(feature = "seed_1_1", feature = "patch_sm")))]
compile_error!("only a single target board must be selected");

#[cfg(all(feature = "patch_sm", any(feature = "seed_1_1", feature = "seed")))]
compile_error!("only a single target board must be selected");

#[cfg(not(any(feature = "seed_1_1", feature = "seed", feature = "patch_sm")))]
compile_error!(
    "target board must be selected using a feature: \"seed_1_1\" | \"seed\" | \"patch_sm\""
);

pub mod audio;
pub mod board;
pub mod clocks;
pub mod flash;
pub mod led;
pub mod pins;
pub mod sdram;

pub use board::Board;
pub use hal::pac;
// Re-exported so it can be used from daisy macros.
pub use stm32h7xx_hal as hal;
