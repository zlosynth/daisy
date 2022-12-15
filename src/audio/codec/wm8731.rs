// Based on https://github.com/backtail/libdaisy-rust

use cortex_m::asm;
use cortex_m::prelude::_embedded_hal_blocking_i2c_Write;
use hal::i2c;
use hal::pac;
use hal::time;
use stm32h7xx_hal as hal; // to make the i2c2.write() work

use num_enum::IntoPrimitive;

pub const I2C_FS: time::Hertz = time::Hertz::from_raw(100_000);

pub type Pins = (
    hal::gpio::gpioh::PH4<hal::gpio::Alternate<4, hal::gpio::OpenDrain>>, // I2C2 SCL (WM8731)
    hal::gpio::gpiob::PB11<hal::gpio::Alternate<4, hal::gpio::OpenDrain>>, // I2C2 SDA (WM8731)
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

        let codec_i2c_address: u8 = 0x1a; // or 0x1b if CSB is high

        // Go through configuration setup
        for (register, value) in REGISTER_CONFIG {
            let register: u8 = (*register).into();
            let value: u8 = *value;
            let byte1: u8 = ((register << 1) & 0b1111_1110) | ((value >> 7) & 0b0000_0001u8);
            let byte2: u8 = value;
            let bytes = [byte1, byte2];

            i2c2.write(codec_i2c_address, &bytes).unwrap_or_default();

            // wait ~10us
            asm::delay(5_000);
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, IntoPrimitive)]
#[repr(u8)]
enum Register {
    LINVOL = 0x00,
    RINVOL = 0x01,
    LOUT1V = 0x02,
    ROUT1V = 0x03,
    APANA = 0x04,
    APDIGI = 0x05, // 0000_0101
    PWR = 0x06,
    IFACE = 0x07,  // 0000_0111
    SRATE = 0x08,  // 0000_1000
    ACTIVE = 0x09, // 0000_1001
    RESET = 0x0F,
}

const REGISTER_CONFIG: &[(Register, u8)] = &[
    // reset Codec
    (Register::RESET, 0x00),
    // set line inputs 0dB
    (Register::LINVOL, 0x17),
    (Register::RINVOL, 0x17),
    // set headphone to mute
    (Register::LOUT1V, 0x00),
    (Register::ROUT1V, 0x00),
    // set analog and digital routing
    (Register::APANA, 0x12),
    (Register::APDIGI, 0x01),
    // configure power management
    (Register::PWR, 0x42),
    // configure digital format
    (Register::IFACE, 0x0A),
    // set samplerate
    (Register::SRATE, 0x00),
    (Register::ACTIVE, 0x00),
    (Register::ACTIVE, 0x01),
];
