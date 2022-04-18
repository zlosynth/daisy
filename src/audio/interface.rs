use core::num::Wrapping;

use hal::time;
use stm32h7xx_hal as hal;

use super::codec::{Codec, Pins as CodecPins};
use super::transfer::{Sai1Pins, State, Transfer};
use super::{BLOCK_LENGTH, DMA_BUFFER_LENGTH, FS, HALF_DMA_BUFFER_LENGTH};

#[link_section = ".sram1_bss"]
static mut TX_BUFFER: [u32; DMA_BUFFER_LENGTH] = [0; DMA_BUFFER_LENGTH];
#[link_section = ".sram1_bss"]
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
    function_ptr: Option<fn(f32, &mut Block)>,
}

impl Interface {
    pub fn init(
        clocks: &hal::rcc::CoreClocks,
        sai1_rec: hal::rcc::rec::Sai1, // reset and enable control
        codec_pins: CodecPins,
        sai1_pins: Sai1Pins,
        dma1_rec: hal::rcc::rec::Dma1,
    ) -> Result<Interface, Error> {
        let codec = Codec::init(codec_pins);
        let transfer = Transfer::init(
            clocks,
            sai1_rec,
            sai1_pins,
            dma1_rec,
            unsafe { &mut TX_BUFFER },
            unsafe { &mut RX_BUFFER },
        );

        Ok(Self {
            fs: FS,
            codec,
            transfer,
            function_ptr: None,
        })
    }

    /// assign function pointer for interrupt callback and start audio
    pub fn spawn(mut self, function_ptr: fn(f32, &mut Block)) -> Result<Self, Error> {
        self.function_ptr = Some(function_ptr);
        self.codec.start();
        self.transfer.start();
        Ok(self)
    }

    pub fn handle_interrupt_dma1_str1(&mut self) -> Result<(), Error> {
        let skip = match self.transfer.examine_interrupt() {
            Ok(State::HalfSent) => (0, HALF_DMA_BUFFER_LENGTH),
            Ok(State::FullSent) => (HALF_DMA_BUFFER_LENGTH, 0),
            _ => return Err(Error::Dma),
        };

        // callback buffer
        let mut block: Block = [(0., 0.); BLOCK_LENGTH];

        // convert & copy rx buffer to callback buffer
        let mut dma_index: usize = 0;
        let mut block_index: usize = 0;
        while dma_index < HALF_DMA_BUFFER_LENGTH {
            let rx0: usize = dma_index + skip.1;
            let rx1: usize = rx0 + 1;

            let rx_y0 = unsafe { RX_BUFFER[rx0] };
            let rx_y1 = unsafe { RX_BUFFER[rx1] };

            let y0 = u24_to_f32(rx_y0);
            let y1 = u24_to_f32(rx_y1);
            block[block_index] = (y1, y0);

            dma_index += 2;
            block_index += 1;
        }

        // invoke audio callback
        self.invoke_callback(&mut block);

        // convert & copy callback buffer to tx buffer
        let mut dma_index: usize = 0;
        let mut block_index: usize = 0;
        while dma_index < HALF_DMA_BUFFER_LENGTH {
            let tx0: usize = dma_index + skip.0;
            let tx1: usize = tx0 + 1;

            let (y0, y1) = block[block_index];
            unsafe { TX_BUFFER[tx0] = f32_to_u24(y0) };
            unsafe { TX_BUFFER[tx1] = f32_to_u24(y1) };

            dma_index += 2;
            block_index += 1;
        }

        Ok(())
    }

    fn invoke_callback(&mut self, block: &mut Block) {
        if let Some(function_ptr) = self.function_ptr.as_mut() {
            function_ptr(self.fs.0 as f32, block);
        }
    }
}


/// convert audio data from u24 to f32
#[inline(always)]
fn u24_to_f32(y: u32) -> f32 {
    let y = (Wrapping(y) + Wrapping(0x0080_0000)).0 & 0x00FF_FFFF; // convert to i32
    (y as f32 / 8_388_608.) - 1. // (2^24) / 2
}

/// convert audio data from f32 to u24
#[inline(always)]
fn f32_to_u24(x: f32) -> u32 {
    //return (int16_t) __SSAT((int32_t) (x * 32767.f), 16);
    let x = x * 8_388_607.;
    let x = if x > 8_388_607. {
        8_388_607.
    } else if x < -8_388_608. {
        -8_388_608.
    } else {
        x
    };
    (x as i32) as u32
}
