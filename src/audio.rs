use hal::dma;
use hal::gpio;
use hal::sai::{self, I2sUsers, SaiChannel, SaiI2sExt};
use hal::time;
use stm32h7xx_hal as hal;

use embedded_hal::digital::v2::OutputPin;
use hal::hal as embedded_hal;

use hal::pac;

// - global constants ---------------------------------------------------------

pub const BLOCK_LENGTH: usize = 32; // 32 samples
pub const HALF_DMA_BUFFER_LENGTH: usize = BLOCK_LENGTH * 2; //  2 channels
pub const DMA_BUFFER_LENGTH: usize = HALF_DMA_BUFFER_LENGTH * 2; //  2 half-blocks

pub const FS: time::Hertz = time::Hertz(48_000);

// - static data --------------------------------------------------------------

#[link_section = ".sram1_bss"]
static mut TX_BUFFER: [u32; DMA_BUFFER_LENGTH] = [0; DMA_BUFFER_LENGTH];
#[link_section = ".sram1_bss"]
static mut RX_BUFFER: [u32; DMA_BUFFER_LENGTH] = [0; DMA_BUFFER_LENGTH];

// - types --------------------------------------------------------------------

pub type Frame = (f32, f32);
pub type Block = [Frame; BLOCK_LENGTH];

pub type CodecPins = (
    gpio::gpiob::PB11<gpio::Output<gpio::PushPull>>, // PDN
    gpio::gpioe::PE2<gpio::Alternate<gpio::AF6>>,    // MCLK_A
    gpio::gpioe::PE5<gpio::Alternate<gpio::AF6>>,    // SCK_A
    gpio::gpioe::PE4<gpio::Alternate<gpio::AF6>>,    // FS_A
    gpio::gpioe::PE6<gpio::Alternate<gpio::AF6>>,    // SD_A
    gpio::gpioe::PE3<gpio::Alternate<gpio::AF6>>,    // SD_B
);

pub type Sai1Pins = (
    gpio::gpioe::PE2<gpio::Alternate<gpio::AF6>>, // MCLK_A
    gpio::gpioe::PE5<gpio::Alternate<gpio::AF6>>, // SCK_A
    gpio::gpioe::PE4<gpio::Alternate<gpio::AF6>>, // FS_A
    gpio::gpioe::PE6<gpio::Alternate<gpio::AF6>>, // SD_A
    Option<gpio::gpioe::PE3<gpio::Alternate<gpio::AF6>>>, // SD_B
);

type TransferDma1Str0 = dma::Transfer<
    dma::dma::Stream0<pac::DMA1>,
    pac::SAI1,
    dma::MemoryToPeripheral,
    &'static mut [u32; DMA_BUFFER_LENGTH],
    dma::DBTransfer,
>;

type TransferDma1Str1 = dma::Transfer<
    dma::dma::Stream1<pac::DMA1>,
    pac::SAI1,
    dma::PeripheralToMemory,
    &'static mut [u32; DMA_BUFFER_LENGTH],
    dma::DBTransfer,
>;

// - Error --------------------------------------------------------------------

#[derive(Debug)]
pub enum Error {
    I2c,
    Dma,
}

// - audio::Interface ---------------------------------------------------------

pub struct Interface<'a> {
    pub fs: time::Hertz,

    function_ptr: Option<fn(f32, &mut Block)>,

    ak4556_reset: Option<gpio::gpiob::PB11<gpio::Output<gpio::PushPull>>>,
    transfer: Transfer,

    _marker: core::marker::PhantomData<&'a ()>,
}

impl<'a> Interface<'a> {
    pub fn init(
        clocks: &hal::rcc::CoreClocks,
        sai1_rec: hal::rcc::rec::Sai1, // reset and enable control
        pins: CodecPins,
        dma1_rec: hal::rcc::rec::Dma1,
    ) -> Result<Interface<'a>, Error> {
        let sai1_pins = (pins.1, pins.2, pins.3, pins.4, Some(pins.5));
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

            function_ptr: None,

            ak4556_reset: Some(pins.0),
            transfer,

            _marker: core::marker::PhantomData,
        })
    }

    /// assign function pointer for interrupt callback and start audio
    pub fn spawn(mut self, function_ptr: fn(f32, &mut Block)) -> Result<Self, Error> {
        self.function_ptr = Some(function_ptr);
        self.start()?;
        Ok(self) // TODO type state for started audio interface
    }

    fn start(&mut self) -> Result<(), Error> {
        // - AK4556 -----------------------------------------------------------

        let ak4556_reset = self.ak4556_reset.as_mut().unwrap();
        ak4556_reset.set_low().unwrap();
        use cortex_m::asm;
        asm::delay(480_000); // ~ 1ms (datasheet specifies minimum 150ns)
        ak4556_reset.set_high().unwrap();

        // - start audio ------------------------------------------------------

        self.transfer.start();

        Ok(())
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

struct Transfer {
    dma1_str0: TransferDma1Str0,
    dma1_str1: TransferDma1Str1,
    sai1: hal::sai::Sai<pac::SAI1, hal::sai::I2S>,
}

impl Transfer {
    pub fn init(
        clocks: &hal::rcc::CoreClocks,
        sai1_rec: hal::rcc::rec::Sai1,
        sai1_pins: Sai1Pins,
        dma1_rec: hal::rcc::rec::Dma1,
        tx_buffer: &'static mut [u32; DMA_BUFFER_LENGTH],
        rx_buffer: &'static mut [u32; DMA_BUFFER_LENGTH],
    ) -> Self {
        // - configure dma1 ---------------------------------------------------

        let dma1_streams =
            dma::dma::StreamsTuple::new(unsafe { pac::Peripherals::steal().DMA1 }, dma1_rec);

        // dma1 stream 0
        let dma_config = dma::dma::DmaConfig::default()
            .priority(dma::config::Priority::High)
            .memory_increment(true)
            .peripheral_increment(false)
            .circular_buffer(true)
            .fifo_enable(false);
        let dma1_str0: dma::Transfer<_, _, dma::MemoryToPeripheral, _, _> = dma::Transfer::init(
            dma1_streams.0,
            unsafe { pac::Peripherals::steal().SAI1 },
            tx_buffer,
            None,
            dma_config,
        );

        // dma1 stream 1
        let dma_config = dma_config
            .transfer_complete_interrupt(true)
            .half_transfer_interrupt(true);
        let dma1_str1: dma::Transfer<_, _, dma::PeripheralToMemory, _, _> = dma::Transfer::init(
            dma1_streams.1,
            unsafe { pac::Peripherals::steal().SAI1 },
            rx_buffer,
            None,
            dma_config,
        );

        // - configure sai1 ---------------------------------------------------

        let sai1_a_config = sai::I2SChanConfig::new(sai::I2SDir::Tx)
            .set_frame_sync_active_high(true)
            .set_clock_strobe(sai::I2SClockStrobe::Falling);

        let sai1_b_config = sai::I2SChanConfig::new(sai::I2SDir::Rx)
            .set_sync_type(sai::I2SSync::Internal)
            .set_frame_sync_active_high(true)
            .set_clock_strobe(sai::I2SClockStrobe::Rising);

        let sai1 = unsafe { pac::Peripherals::steal().SAI1 }.i2s_ch_a(
            sai1_pins,
            FS,
            sai::I2SDataSize::BITS_24,
            sai1_rec,
            clocks,
            I2sUsers::new(sai1_a_config).add_slave(sai1_b_config),
        );

        Self {
            dma1_str0,
            dma1_str1,
            sai1,
        }
    }

    pub fn start(&mut self) {
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::DMA1_STR1);
        }

        let dma1_str0 = &mut self.dma1_str0;
        let dma1_str1 = &mut self.dma1_str1;
        let sai1 = &mut self.sai1;

        dma1_str1.start(|_sai1_rb| {
            sai1.enable_dma(SaiChannel::ChannelB);
        });

        dma1_str0.start(|sai1_rb| {
            sai1.enable_dma(SaiChannel::ChannelA);

            // wait until sai1's fifo starts to receive data
            while sai1_rb.cha.sr.read().flvl().is_empty() {}

            sai1.enable();

            use stm32h7xx_hal::traits::i2s::FullDuplex;
            sai1.try_send(0, 0).unwrap();
        });
    }

    pub fn examine_interrupt(&mut self) -> Result<State, ()> {
        if self.dma1_str1.get_half_transfer_flag() {
            self.dma1_str1.clear_half_transfer_interrupt();
            Ok(State::HalfSent)
        } else if self.dma1_str1.get_transfer_complete_flag() {
            self.dma1_str1.clear_transfer_complete_interrupt();
            Ok(State::FullSent)
        } else {
            Err(())
        }
    }
}

enum State {
    HalfSent,
    FullSent,
}

// - conversion helpers -------------------------------------------------------

use core::num::Wrapping;

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
