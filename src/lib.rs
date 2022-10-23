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
//! # Usage
//!
//! See the [examples/](https://github.com/zlosynth/daisy/tree/main/examples)
//! directory to find usage examples:
//!
//! * [ADC](https://github.com/zlosynth/daisy/blob/main/examples/adc.rs)
//! * [Audio](https://github.com/zlosynth/daisy/blob/main/examples/audio.rs)
//! * [Blinky](https://github.com/zlosynth/daisy/blob/main/examples/blinky.rs)
//! * [Blinky with RTIC](https://github.com/zlosynth/daisy/blob/main/examples/blinky_rtic.rs)
//! * [Flash storage](https://github.com/zlosynth/daisy/blob/main/examples/flash.rs)
//! * [OLED display](https://github.com/zlosynth/daisy/blob/main/examples/oled.rs)
//! * [SDRAM memory](https://github.com/zlosynth/daisy/blob/main/examples/sdram.rs)
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

// - modules ------------------------------------------------------------------

pub mod audio;
pub mod board;
pub mod clocks;
pub mod flash;
#[cfg(any(feature = "log-itm"))]
pub mod itm;
pub mod led;
pub mod pins;
pub mod sdram;

// - log macros ---------------------------------------------------------------

#[cfg(any(feature = "log-itm"))]
#[macro_export]
macro_rules! loggit {
    ($($arg:tt)*) => (
        let itm = unsafe { &mut *cortex_m::peripheral::ITM::PTR };
        cortex_m::iprintln!(&mut itm.stim[0], $($arg)*);
    )
}

#[cfg(not(feature = "log-itm"))]
#[macro_export]
macro_rules! loggit {
    ($($arg:tt)*) => (
        cortex_m_semihosting::hprintln!($($arg)*).unwrap();
    )
}

// - exports ------------------------------------------------------------------

pub use hal::hal as embedded_hal;
pub use hal::pac;
pub use stm32h7xx_hal as hal;

pub use board::Board;
