// Based on https://github.com/electro-smith/libDaisy/blob/master/src/dev/codec_pcm3060.cpp

use cortex_m::asm;
use hal::i2c;
use hal::pac;
use hal::prelude::*;
use hal::time;
use stm32h7xx_hal as hal;

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
        for (register, mask) in REGISTER_CONFIG {
            let mut buffer = [0];
            i2c2.write_read(codec_i2c_address, &[*register], &mut buffer)
                .unwrap();

            let value = buffer[0] & !mask;

            i2c2.write(codec_i2c_address, &[*register, value]).unwrap();

            // wait ~10us
            asm::delay(5_000);
        }
    }
}

#[allow(non_camel_case_types)]
const SYS_CTRL_REGISTER: u8 = 0x40;

#[allow(non_camel_case_types)]
const MRST_MASK: u8 = 0x80;
#[allow(non_camel_case_types)]
const SRST_MASK: u8 = 0x40;
#[allow(non_camel_case_types)]
const ADC_PSV_MASK: u8 = 0x20;
#[allow(non_camel_case_types)]
const DAC_PSV_MASK: u8 = 0x10;

const REGISTER_CONFIG: &[(u8, u8)] = &[
    // reset Codec
    (SYS_CTRL_REGISTER, MRST_MASK),
    (SYS_CTRL_REGISTER, SRST_MASK),
    // disable power saving
    (SYS_CTRL_REGISTER, ADC_PSV_MASK),
    (SYS_CTRL_REGISTER, DAC_PSV_MASK),
];
