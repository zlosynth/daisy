// Based on https://github.com/electro-smith/libDaisy/blob/master/src/dev/codec_pcm3060.cpp.

use cortex_m::asm;

use crate::hal;
use hal::gpio;
use hal::i2c;
use hal::pac;
use hal::prelude::*;
use hal::time;

const I2C_FS: time::Hertz = time::Hertz::from_raw(100_000);
const I2C_CODEC_ADDRESS: u8 = 0x8c >> 1;

pub type Pins = (
    gpio::gpiob::PB10<gpio::Alternate<4, gpio::OpenDrain>>, // I2C2 SCL (PCM3060)
    gpio::gpiob::PB11<gpio::Alternate<4, gpio::OpenDrain>>, // I2C2 SDA (PCM3060)
);

pub struct Codec {
    i2c2: hal::i2c::I2c<pac::I2C2>,
}

impl Codec {
    pub fn init(
        clocks: &hal::rcc::CoreClocks,
        i2c2_rec: hal::rcc::rec::I2c2,
        i2c2_pins: Pins,
    ) -> Self {
        let i2c2 = i2c::I2cExt::i2c(
            unsafe { pac::Peripherals::steal().I2C2 },
            i2c2_pins,
            I2C_FS,
            i2c2_rec,
            clocks,
        );
        Self { i2c2 }
    }

    pub fn start(&mut self) {
        let i2c2 = &mut self.i2c2;

        // Go through configuration setup.
        for (register, mask, set) in REGISTER_CONFIG {
            let mut buffer = [0];
            i2c2.write_read(I2C_CODEC_ADDRESS, &[*register], &mut buffer)
                .unwrap();

            let value = if *set {
                buffer[0] | mask
            } else {
                buffer[0] & !mask
            };

            i2c2.write(I2C_CODEC_ADDRESS, &[*register, value]).unwrap();

            // Wait ~10us.
            asm::delay(5_000);
        }
    }
}

const SYS_CTRL_REGISTER: u8 = 0x40;
const ADC_CTRL1_REGISTER: u8 = 0x48;
const DAC_CTRL1_REGISTER: u8 = 0x43;

const MRST_MASK: u8 = 0x80;
const SRST_MASK: u8 = 0x40;
const ADC_PSV_MASK: u8 = 0x20;
const DAC_PSV_MASK: u8 = 0x10;
const FMT_MASK: u8 = 0x1;

const REGISTER_CONFIG: &[(u8, u8, bool)] = &[
    // Reset Codec.
    (SYS_CTRL_REGISTER, MRST_MASK, false),
    (SYS_CTRL_REGISTER, SRST_MASK, false),
    // Set 24-bit LJ.
    (ADC_CTRL1_REGISTER, FMT_MASK, true),
    (DAC_CTRL1_REGISTER, FMT_MASK, true),
    // Disable power saving.
    (SYS_CTRL_REGISTER, ADC_PSV_MASK, false),
    (SYS_CTRL_REGISTER, DAC_PSV_MASK, false),
];
