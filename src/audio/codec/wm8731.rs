use stm32h7xx_hal as hal;

pub type Pins = (
    hal::gpio::gpioh::PH4<hal::gpio::Alternate<4>>, // I2C2 SCL (WM8731)
    hal::gpio::gpiob::PB11<hal::gpio::Alternate<4>>, // I2C2 SDA (WM8731)
);

pub struct Codec;

impl Codec {
    pub fn init(_pins: Pins) -> Self {
        Self
    }

    pub fn start(&mut self) {}
}
