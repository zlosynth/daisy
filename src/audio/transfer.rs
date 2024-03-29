use crate::hal;
use hal::dma;
use hal::gpio;
use hal::prelude::*;
use hal::sai::{self, I2sUsers, SaiChannel, SaiI2sExt};

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
    audio_interface: AudioInterface,
}

pub struct Config {
    pub tx_channel: Channel,
    pub rx_channel: Channel,
    pub tx_sync: Sync,
    pub rx_sync: Sync,
}

pub enum Channel {
    A,
    B,
}

#[derive(Clone, Copy)]
pub enum Sync {
    Master,
    Slave,
}

impl Transfer {
    pub fn init(
        clocks: &hal::rcc::CoreClocks,
        sai1_rec: hal::rcc::rec::Sai1,
        sai1_pins: Sai1Pins,
        dma1_rec: hal::rcc::rec::Dma1,
        tx_buffer: &'static mut [u32; DMA_BUFFER_LENGTH],
        rx_buffer: &'static mut [u32; DMA_BUFFER_LENGTH],
        config: Config,
    ) -> Self {
        let dma1_streams =
            dma::dma::StreamsTuple::new(unsafe { pac::Peripherals::steal().DMA1 }, dma1_rec);

        let (transmitter, receiver) = match (config.tx_channel, config.rx_channel) {
            (Channel::A, Channel::B) => (
                Transmitter::init_with_channel_a(dma1_streams.0, tx_buffer),
                Receiver::init_with_channel_b(dma1_streams.1, rx_buffer),
            ),
            (Channel::B, Channel::A) => (
                Transmitter::init_with_channel_b(dma1_streams.1, tx_buffer),
                Receiver::init_with_channel_a(dma1_streams.0, rx_buffer),
            ),
            _ => panic!("Rx and Tx must use different channels"),
        };

        let (master, slave) = match (config.tx_sync, config.rx_sync) {
            (Sync::Master, Sync::Slave) => (sai::I2SDir::Tx, sai::I2SDir::Rx),
            (Sync::Slave, Sync::Master) => (sai::I2SDir::Rx, sai::I2SDir::Tx),
            _ => panic!("There must be only one master and one slave"),
        };
        let audio_interface = AudioInterface::init(clocks, sai1_rec, sai1_pins, master, slave);

        Self {
            transmitter,
            receiver,
            audio_interface,
        }
    }

    pub fn start(&mut self) {
        let transmitter = &mut self.transmitter;
        let receiver = &mut self.receiver;
        let audio_interface = &mut self.audio_interface;

        receiver.start(audio_interface);
        transmitter.start(audio_interface);
    }

    pub fn examine_interrupt(&mut self) -> Result<State, ()> {
        // TODO: Decide this during compilation
        if let Some(channel_b) = self.transmitter.channel_b() {
            if channel_b.get_half_transfer_flag() {
                channel_b.clear_half_transfer_interrupt();
                Ok(State::HalfSent)
            } else if channel_b.get_transfer_complete_flag() {
                channel_b.clear_transfer_complete_interrupt();
                Ok(State::FullSent)
            } else {
                Err(())
            }
        } else if let Some(channel_b) = self.receiver.channel_b() {
            if channel_b.get_half_transfer_flag() {
                channel_b.clear_half_transfer_interrupt();
                Ok(State::HalfSent)
            } else if channel_b.get_transfer_complete_flag() {
                channel_b.clear_transfer_complete_interrupt();
                Ok(State::FullSent)
            } else {
                Err(())
            }
        } else {
            unreachable!("There is always one channel B");
        }
    }
}

pub enum State {
    HalfSent,
    FullSent,
}

struct AudioInterface(hal::sai::Sai<pac::SAI1, hal::sai::I2S>);

impl AudioInterface {
    fn init(
        clocks: &hal::rcc::CoreClocks,
        sai1_rec: hal::rcc::rec::Sai1,
        sai1_pins: Sai1Pins,
        master: sai::I2SDir,
        slave: sai::I2SDir,
    ) -> Self {
        let mut sai1_master_config =
            sai::I2SChanConfig::new(master).set_frame_sync_active_high(true);
        let mut sai1_slave_config = sai::I2SChanConfig::new(slave)
            .set_sync_type(sai::I2SSync::Internal)
            .set_frame_sync_active_high(true);

        if master == sai::I2SDir::Tx {
            sai1_master_config = sai1_master_config.set_clock_strobe(sai::I2SClockStrobe::Falling);
            sai1_slave_config = sai1_slave_config.set_clock_strobe(sai::I2SClockStrobe::Rising);
        }

        Self(unsafe { pac::Peripherals::steal().SAI1 }.i2s_ch_a(
            sai1_pins,
            FS,
            sai::I2SDataSize::BITS_24,
            sai1_rec,
            clocks,
            I2sUsers::new(sai1_master_config).add_slave(sai1_slave_config),
        ))
    }
}

type _Transmitter<S, C> = dma::Transfer<
    S,
    C,
    dma::MemoryToPeripheral,
    &'static mut [u32; DMA_BUFFER_LENGTH],
    dma::DBTransfer,
>;

enum Transmitter {
    ChannelA(_Transmitter<dma::dma::Stream0<pac::DMA1>, sai::dma::ChannelA<pac::SAI1>>),
    ChannelB(_Transmitter<dma::dma::Stream1<pac::DMA1>, sai::dma::ChannelB<pac::SAI1>>),
}

impl Transmitter {
    fn init_with_channel_a(
        dma1_str0: dma::dma::Stream0<pac::DMA1>,
        tx_buffer: &'static mut [u32; DMA_BUFFER_LENGTH],
    ) -> Self {
        Transmitter::ChannelA(dma::Transfer::init(
            dma1_str0,
            unsafe { pac::Peripherals::steal().SAI1.dma_ch_a() },
            tx_buffer,
            None,
            channel_a_dma_config(),
        ))
    }

    fn init_with_channel_b(
        dma1_str1: dma::dma::Stream1<pac::DMA1>,
        tx_buffer: &'static mut [u32; DMA_BUFFER_LENGTH],
    ) -> Self {
        Transmitter::ChannelB(dma::Transfer::init(
            dma1_str1,
            unsafe { pac::Peripherals::steal().SAI1.dma_ch_b() },
            tx_buffer,
            None,
            channel_b_dma_config(),
        ))
    }

    fn start(&mut self, audio_interface: &mut AudioInterface) {
        let sai1 = &mut audio_interface.0;
        match self {
            Transmitter::ChannelA(dma1_str0) => {
                dma1_str0.start(|sai1_rb| {
                    sai1.enable_dma(SaiChannel::ChannelA);
                    while sai1_rb.cha().sr.read().flvl().is_empty() {} // wait until sai1's fifo starts to receive data
                    sai1.enable();
                    use hal::traits::i2s::FullDuplex;
                    sai1.try_send(0, 0).unwrap();
                });
            }
            Transmitter::ChannelB(dma1_str1) => {
                unsafe {
                    pac::NVIC::unmask(pac::Interrupt::DMA1_STR1);
                }
                dma1_str1.start(|sai1_rb| {
                    sai1.enable_dma(SaiChannel::ChannelB);
                    while sai1_rb.chb().sr.read().flvl().is_empty() {} // wait until sai1's fifo starts to receive data
                    sai1.enable();
                    use hal::traits::i2s::FullDuplex;
                    sai1.try_send(0, 0).unwrap();
                });
            }
        }
    }

    fn channel_b(
        &mut self,
    ) -> Option<&mut _Transmitter<dma::dma::Stream1<pac::DMA1>, sai::dma::ChannelB<pac::SAI1>>>
    {
        if let Self::ChannelB(channel_b) = self {
            Some(channel_b)
        } else {
            None
        }
    }
}

type _Receiver<S, C> = dma::Transfer<
    S,
    C,
    dma::PeripheralToMemory,
    &'static mut [u32; DMA_BUFFER_LENGTH],
    dma::DBTransfer,
>;

enum Receiver {
    ChannelA(_Receiver<dma::dma::Stream0<pac::DMA1>, sai::dma::ChannelA<pac::SAI1>>),
    ChannelB(_Receiver<dma::dma::Stream1<pac::DMA1>, sai::dma::ChannelB<pac::SAI1>>),
}

impl Receiver {
    fn init_with_channel_a(
        dma1_str0: dma::dma::Stream0<pac::DMA1>,
        rx_buffer: &'static mut [u32; DMA_BUFFER_LENGTH],
    ) -> Self {
        Receiver::ChannelA(dma::Transfer::init(
            dma1_str0,
            unsafe { pac::Peripherals::steal().SAI1.dma_ch_a() },
            rx_buffer,
            None,
            channel_a_dma_config(),
        ))
    }

    fn init_with_channel_b(
        dma1_str1: dma::dma::Stream1<pac::DMA1>,
        rx_buffer: &'static mut [u32; DMA_BUFFER_LENGTH],
    ) -> Self {
        Receiver::ChannelB(dma::Transfer::init(
            dma1_str1,
            unsafe { pac::Peripherals::steal().SAI1.dma_ch_b() },
            rx_buffer,
            None,
            channel_b_dma_config(),
        ))
    }

    fn start(&mut self, audio_interface: &mut AudioInterface) {
        let sai1 = &mut audio_interface.0;
        match self {
            Receiver::ChannelA(dma1_str0) => {
                dma1_str0.start(|_sai1_rb| {
                    sai1.enable_dma(SaiChannel::ChannelA);
                });
            }
            Receiver::ChannelB(dma1_str1) => {
                unsafe {
                    pac::NVIC::unmask(pac::Interrupt::DMA1_STR1);
                }
                dma1_str1.start(|_sai1_rb| {
                    sai1.enable_dma(SaiChannel::ChannelB);
                });
            }
        }
    }

    fn channel_b(
        &mut self,
    ) -> Option<&mut _Receiver<dma::dma::Stream1<pac::DMA1>, sai::dma::ChannelB<pac::SAI1>>> {
        if let Self::ChannelB(channel_b) = self {
            Some(channel_b)
        } else {
            None
        }
    }
}

fn channel_a_dma_config() -> dma::dma::DmaConfig {
    dma::dma::DmaConfig::default()
        .priority(dma::config::Priority::High)
        .memory_increment(true)
        .peripheral_increment(false)
        .circular_buffer(true)
        .fifo_enable(false)
}

// Since channel B is always tied to DMA stream 1, it will be the one
// handling interrupts.
fn channel_b_dma_config() -> dma::dma::DmaConfig {
    dma::dma::DmaConfig::default()
        .priority(dma::config::Priority::High)
        .memory_increment(true)
        .peripheral_increment(false)
        .circular_buffer(true)
        .fifo_enable(false)
        .transfer_complete_interrupt(true)
        .half_transfer_interrupt(true)
}
