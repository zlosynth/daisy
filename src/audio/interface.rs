use core::num::Wrapping;
use core::ptr;

use super::codec::{Codec, Pins as CodecPins};
use super::transfer::{Channel, Config as TransferConfig, Sai1Pins, State, Sync, Transfer};
use super::{BLOCK_LENGTH, DMA_BUFFER_LENGTH, FS, HALF_DMA_BUFFER_LENGTH};
use crate::hal;
use hal::pac::{CPUID, CorePeripherals};
use hal::time;

#[unsafe(link_section = ".sram1_bss")]
static mut TX_BUFFER: [u32; DMA_BUFFER_LENGTH] = [0; DMA_BUFFER_LENGTH];
#[unsafe(link_section = ".sram1_bss")]
static mut RX_BUFFER: [u32; DMA_BUFFER_LENGTH] = [0; DMA_BUFFER_LENGTH];

pub type Frame = (f32, f32);
pub type Block = [Frame; BLOCK_LENGTH];

#[derive(Debug)]
pub enum Error {
    I2c,
    Dma,
}

pub struct Interface {
    pub fs: time::Hertz,
    codec: Codec,
    transfer: Transfer,
}

impl Interface {
    pub fn init(
        clocks: &hal::rcc::CoreClocks,
        sai1_rec: hal::rcc::rec::Sai1,
        sai1_pins: Sai1Pins,
        codec_pins: CodecPins,
        #[allow(unused_variables)] // i2c2 is not used on Seed 1.0
        i2c2_rec: hal::rcc::rec::I2c2,
        dma1_rec: hal::rcc::rec::Dma1,
    ) -> Result<Interface, Error> {
        #[cfg(any(feature = "seed", feature = "seed_1_2"))]
        let codec = Codec::init(codec_pins);
        #[cfg(any(feature = "seed_1_1", feature = "patch_sm"))]
        let codec = Codec::init(clocks, i2c2_rec, codec_pins);

        #[cfg(any(feature = "seed_1_2", feature = "seed"))]
        let transfer_config = TransferConfig {
            tx_channel: Channel::A,
            rx_channel: Channel::B,
            tx_sync: Sync::Master,
            rx_sync: Sync::Slave,
        };
        #[cfg(any(feature = "seed_1_1", feature = "patch_sm"))]
        let transfer_config = TransferConfig {
            tx_channel: Channel::B,
            rx_channel: Channel::A,
            tx_sync: Sync::Slave,
            rx_sync: Sync::Master,
        };

        let transfer = Transfer::init(
            clocks,
            sai1_rec,
            sai1_pins,
            dma1_rec,
            unsafe { &mut *ptr::addr_of_mut!(TX_BUFFER) },
            unsafe { &mut *ptr::addr_of_mut!(RX_BUFFER) },
            transfer_config,
        );

        // Verifying safety requirements and recommendations of
        // invalidate_dcache_by_slice and clean_dcache_by_slice.
        validate_slice_against_cache_line(unsafe { &*ptr::addr_of!(TX_BUFFER) });
        validate_slice_against_cache_line(unsafe { &*ptr::addr_of!(RX_BUFFER) });

        Ok(Self {
            fs: FS,
            codec,
            transfer,
        })
    }

    /// Start audio streaming.
    pub fn spawn(mut self) -> Result<Self, Error> {
        self.codec.start();
        self.transfer.start();
        Ok(self)
    }

    pub fn handle_interrupt_dma1_str1(
        &mut self,
        mut callback: impl FnMut(&mut [(f32, f32); BLOCK_LENGTH]),
    ) -> Result<(), Error> {
        let skip = match self.transfer.examine_interrupt() {
            Ok(State::HalfSent) => (0, HALF_DMA_BUFFER_LENGTH),
            Ok(State::FullSent) => (HALF_DMA_BUFFER_LENGTH, 0),
            _ => return Err(Error::Dma),
        };

        let mut block: Block = [(0.0, 0.0); BLOCK_LENGTH];

        // Force dcache to get populated from memory.
        // Safety: RX buffer is accessed only through this function, without any
        // concurrency. The init function verifies that the buffer has the
        // correct size and alignment.
        unsafe {
            CorePeripherals::steal()
                .SCB
                .invalidate_dcache_by_slice(&mut *ptr::addr_of_mut!(RX_BUFFER));
        }

        // Convert and copy received audio to callback buffer.
        let mut dma_index: usize = 0;
        let mut block_index: usize = 0;
        while dma_index < HALF_DMA_BUFFER_LENGTH {
            let rx0: usize = dma_index + skip.1;
            let rx1: usize = rx0 + 1;

            let rx_y0 = unsafe { RX_BUFFER[rx0] };
            let rx_y1 = unsafe { RX_BUFFER[rx1] };

            let y0 = u24_to_f32(rx_y0);
            let y1 = u24_to_f32(rx_y1);
            block[block_index] = (y0, y1);

            dma_index += 2;
            block_index += 1;
        }

        // Invoke user-supplied callback.
        callback(&mut block);

        // Convert and copy callback buffer to output audio buffer.
        let mut dma_index: usize = 0;
        let mut block_index: usize = 0;
        while dma_index < HALF_DMA_BUFFER_LENGTH {
            let tx0: usize = dma_index + skip.0;
            let tx1: usize = tx0 + 1;

            let (y1, y0) = block[block_index];
            unsafe { TX_BUFFER[tx0] = f32_to_u24(y0) };
            unsafe { TX_BUFFER[tx1] = f32_to_u24(y1) };

            dma_index += 2;
            block_index += 1;
        }

        // Force dcache to get flushed into memory.
        // Safety: TX buffer is accessed only through this function, without any
        // concurrency. The init function verifies that the buffer has the
        // correct size and alignment.
        unsafe {
            CorePeripherals::steal()
                .SCB
                .clean_dcache_by_slice(&*ptr::addr_of!(TX_BUFFER));
        }

        Ok(())
    }
}

/// Verifying safety requirements and recommendations of
/// invalidate_dcache_by_slice and clean_dcache_by_slice. The slice must be
/// aligned with cache lines.
fn validate_slice_against_cache_line<T>(slice: &[T]) {
    let addr = slice.as_ptr() as usize;
    let size = slice.len() * core::mem::size_of_val(slice);
    let dminline = CPUID::cache_dminline();
    let line_size = (1 << dminline) * 4;
    assert!((addr & (line_size - 1)) == 0);
    assert!((size & (line_size - 1)) == 0);
}

/// Convert audio data from u24 to f32.
#[inline(always)]
fn u24_to_f32(y: u32) -> f32 {
    let y = (Wrapping(y) + Wrapping(0x0080_0000)).0 & 0x00FF_FFFF; // convert to i32
    (y as f32 / 8_388_608.0) - 1.0 // (2^24) / 2
}

/// Convert audio data from f32 to u24.
#[inline(always)]
fn f32_to_u24(x: f32) -> u32 {
    let x = x * 8_388_607.0;
    let x = x.clamp(-8_388_608.0, 8_388_607.0);
    (x as i32) as u32
}
