#![no_std]

//! Board support crate for Daisy hardware
//!
//! # Usage - see examples/

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
