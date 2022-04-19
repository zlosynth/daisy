use hal::dma;
use hal::gpio;
use hal::prelude::*;
use hal::sai::{self, I2sUsers, SaiChannel, SaiI2sExt};
use stm32h7xx_hal as hal;

use hal::pac;

use super::{DMA_BUFFER_LENGTH, FS};

pub type Sai1Pins = (
    gpio::gpioe::PE2<gpio::Alternate<6>>,         // MCLK_A
    gpio::gpioe::PE5<gpio::Alternate<6>>,         // SCK_A
    gpio::gpioe::PE4<gpio::Alternate<6>>,         // FS_A
    gpio::gpioe::PE6<gpio::Alternate<6>>,         // SD_A
    Option<gpio::gpioe::PE3<gpio::Alternate<6>>>, // SD_B
);

pub struct Transfer {
    transmitter: Transmitter,
    receiver: Receiver,
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
        let dma1_streams =
            dma::dma::StreamsTuple::new(unsafe { pac::Peripherals::steal().DMA1 }, dma1_rec);

        let transmitter = Transmitter::init_with_channel_a(dma1_streams.0, tx_buffer);
        let receiver = Receiver::init_with_channel_b(dma1_streams.1, rx_buffer);

        let sai1_master_config = sai::I2SChanConfig::new(sai::I2SDir::Tx)
            .set_frame_sync_active_high(true)
            .set_clock_strobe(sai::I2SClockStrobe::Falling);
        let sai1_slave_config = sai::I2SChanConfig::new(sai::I2SDir::Rx)
            .set_sync_type(sai::I2SSync::Internal)
            .set_frame_sync_active_high(true)
            .set_clock_strobe(sai::I2SClockStrobe::Rising);
        let sai1 = unsafe { pac::Peripherals::steal().SAI1 }.i2s_ch_a(
            sai1_pins,
            FS,
            sai::I2SDataSize::BITS_24,
            sai1_rec,
            clocks,
            I2sUsers::new(sai1_master_config).add_slave(sai1_slave_config),
        );

        Self {
            transmitter,
            receiver,
            sai1,
        }
    }

    pub fn start(&mut self) {
        let transmitter = &mut self.transmitter;
        let receiver = &mut self.receiver;
        let sai1 = &mut self.sai1;

        receiver.start(sai1);
        transmitter.start(sai1);
    }

    pub fn examine_interrupt(&mut self) -> Result<State, ()> {
        if self.receiver.get_half_transfer_flag() {
            self.receiver.clear_half_transfer_interrupt();
            Ok(State::HalfSent)
        } else if self.receiver.get_transfer_complete_flag() {
            self.receiver.clear_transfer_complete_interrupt();
            Ok(State::FullSent)
        } else {
            Err(())
        }
    }
}

pub enum State {
    HalfSent,
    FullSent,
}

type _Transmitter<C> = dma::Transfer<
    dma::dma::Stream0<pac::DMA1>,
    C,
    dma::MemoryToPeripheral,
    &'static mut [u32; DMA_BUFFER_LENGTH],
    dma::DBTransfer,
>;

enum Transmitter {
    ChannelA(_Transmitter<sai::dma::ChannelA<pac::SAI1>>),
    // ChannelB(_Transmitter<sai::dma::ChannelB<pac::SAI1>>),
}

impl Transmitter {
    fn init_with_channel_a(
        dma1_str0: dma::dma::Stream0<pac::DMA1>,
        tx_buffer: &'static mut [u32; DMA_BUFFER_LENGTH],
    ) -> Self {
        let dma_config = dma::dma::DmaConfig::default()
            .priority(dma::config::Priority::High)
            .memory_increment(true)
            .peripheral_increment(false)
            .circular_buffer(true)
            .fifo_enable(false);
        Transmitter::ChannelA(dma::Transfer::init(
            dma1_str0,
            unsafe { pac::Peripherals::steal().SAI1.dma_ch_a() },
            tx_buffer,
            None,
            dma_config,
        ))
    }

    fn start(&mut self, sai1: &mut hal::sai::Sai<pac::SAI1, hal::sai::I2S>) {
        match self {
            Transmitter::ChannelA(dma1_str0) => {
                dma1_str0.start(|sai1_rb| {
                    sai1.enable_dma(SaiChannel::ChannelA);

                    // wait until sai1's fifo starts to receive data
                    while sai1_rb.cha.sr.read().flvl().is_empty() {}

                    sai1.enable();

                    use hal::traits::i2s::FullDuplex;
                    sai1.try_send(0, 0).unwrap();
                });
            }
        }
    }
}

type _Receiver<C> = dma::Transfer<
    dma::dma::Stream1<pac::DMA1>,
    C,
    dma::PeripheralToMemory,
    &'static mut [u32; DMA_BUFFER_LENGTH],
    dma::DBTransfer,
>;

enum Receiver {
    // ChannelA(_Receiver<sai::dma::ChannelA<pac::SAI1>>),
    ChannelB(_Receiver<sai::dma::ChannelB<pac::SAI1>>),
}

impl Receiver {
    fn init_with_channel_b(
        dma1_str1: dma::dma::Stream1<pac::DMA1>,
        rx_buffer: &'static mut [u32; DMA_BUFFER_LENGTH],
    ) -> Self {
        let dma_config = dma::dma::DmaConfig::default()
            .priority(dma::config::Priority::High)
            .memory_increment(true)
            .peripheral_increment(false)
            .circular_buffer(true)
            .fifo_enable(false)
            .transfer_complete_interrupt(true)
            .half_transfer_interrupt(true);
        Receiver::ChannelB(dma::Transfer::init(
            dma1_str1,
            unsafe { pac::Peripherals::steal().SAI1.dma_ch_b() },
            rx_buffer,
            None,
            dma_config,
        ))
    }

    fn start(&mut self, sai1: &mut hal::sai::Sai<pac::SAI1, hal::sai::I2S>) {
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::DMA1_STR1);
        }

        match self {
            Receiver::ChannelB(dma1_str1) => {
                dma1_str1.start(|_sai1_rb| {
                    sai1.enable_dma(SaiChannel::ChannelB);
                });
            }
        }
    }

    fn get_half_transfer_flag(&mut self) -> bool {
        match self {
            Self::ChannelB(dma1_str1) => dma1_str1.get_half_transfer_flag(),
        }
    }

    fn clear_half_transfer_interrupt(&mut self) {
        match self {
            Self::ChannelB(dma1_str1) => dma1_str1.clear_half_transfer_interrupt(),
        }
    }

    fn get_transfer_complete_flag(&mut self) -> bool {
        match self {
            Self::ChannelB(dma1_str1) => dma1_str1.get_transfer_complete_flag(),
        }
    }

    fn clear_transfer_complete_interrupt(&mut self) {
        match self {
            Self::ChannelB(dma1_str1) => dma1_str1.clear_transfer_complete_interrupt(),
        }
    }
}
