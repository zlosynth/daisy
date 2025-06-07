//! Interrupt-driven access to the audio interface.

use crate::hal;
use hal::time;

mod codec;
pub mod interface;
mod transfer;

pub use interface::{Block, Interface};

#[cfg(not(feature = "block_length_64"))]
pub const BLOCK_LENGTH: usize = 32; // 32 samples
#[cfg(feature = "block_length_64")]
pub const BLOCK_LENGTH: usize = 64; // 64 samples

pub const HALF_DMA_BUFFER_LENGTH: usize = BLOCK_LENGTH * 2; // 2 channels
pub const DMA_BUFFER_LENGTH: usize = HALF_DMA_BUFFER_LENGTH * 2; // 2 half-blocks

#[cfg(not(feature = "sampling_rate_96khz"))]
pub const FS: time::Hertz = time::Hertz::from_raw(48_000);
#[cfg(feature = "sampling_rate_96khz")]
pub const FS: time::Hertz = time::Hertz::from_raw(96_000);
