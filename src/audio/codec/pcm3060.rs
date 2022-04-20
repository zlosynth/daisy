// Based on https://github.com/electro-smith/libDaisy/blob/master/src/dev/codec_pcm3060.cpp

use cortex_m::asm;
use hal::i2c;
use hal::pac;
use hal::prelude::*;
use hal::time;
use stm32h7xx_hal as hal;

use num_enum::IntoPrimitive;

pub const I2C_FS: time::Hertz = time::Hertz::from_raw(100_000);

pub type Pins = (
    hal::gpio::gpiob::PB10<hal::gpio::Alternate<4, hal::gpio::OpenDrain>>, // I2C2 SCL (PCM3060)
    hal::gpio::gpiob::PB11<hal::gpio::Alternate<4, hal::gpio::OpenDrain>>, // I2C2 SDA (PCM3060)
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

        let codec_i2c_address: u8 = 0x8c >> 1;

        // Go through configuration setup
        for (register, mask, operation) in REGISTER_CONFIG {
            let mut buffer = [0];
            i2c2.write_read(codec_i2c_address, &[(*register).into()], &mut buffer)
                .unwrap();

            let mut value = buffer[0];
            match operation {
                Operation::On => {
                    value |= mask;
                }
                Operation::Off => {
                    value &= !mask;
                }
            }

            i2c2.write(codec_i2c_address, &[(*register).into(), value])
                .unwrap();

            // wait ~10us
            asm::delay(5_000);
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, IntoPrimitive)]
#[repr(u8)]
enum Register {
    SYS_CTRL = 0x40,
    DAC_CTRL1 = 0x43,
    ADC_CTRL1 = 0x48,
}

enum Operation {
    On,
    Off,
}

#[allow(non_camel_case_types)]
const MRST_MASK: u8 = 0x80;
#[allow(non_camel_case_types)]
const SRST_MASK: u8 = 0x40;
#[allow(non_camel_case_types)]
const ADC_PSV_MASK: u8 = 0x20;
#[allow(non_camel_case_types)]
const DAC_PSV_MASK: u8 = 0x10;
#[allow(non_camel_case_types)]
const FMT_MASK: u8 = 0x11;

const REGISTER_CONFIG: &[(Register, u8, Operation)] = &[
    // reset Codec
    (Register::SYS_CTRL, MRST_MASK, Operation::Off),
    (Register::SYS_CTRL, SRST_MASK, Operation::Off),
    // set ADC format to 24-bit LJ
    (Register::DAC_CTRL1, FMT_MASK & 1, Operation::On),
    (Register::ADC_CTRL1, FMT_MASK & 1, Operation::On),
    // disable power saving
    (Register::SYS_CTRL, ADC_PSV_MASK, Operation::Off),
    (Register::SYS_CTRL, DAC_PSV_MASK, Operation::Off),
];
