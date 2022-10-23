//! Interrupt-driven access to the audio interface.

use hal::time;
use stm32h7xx_hal as hal;

mod codec;
pub mod interface;
mod transfer;

pub use interface::{Block, Interface};

pub const BLOCK_LENGTH: usize = 32; // 32 samples
pub const HALF_DMA_BUFFER_LENGTH: usize = BLOCK_LENGTH * 2; //  2 channels
pub const DMA_BUFFER_LENGTH: usize = HALF_DMA_BUFFER_LENGTH * 2; //  2 half-blocks
pub const FS: time::Hertz = time::Hertz::from_raw(48_000);
