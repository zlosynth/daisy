use cortex_m::asm;

use crate::hal;
use hal::gpio;

type ResetPin = gpio::gpiob::PB11<gpio::Output<gpio::PushPull>>; // PDN

pub type Pins = (ResetPin,);

pub struct Codec {
    reset: ResetPin,
}

impl Codec {
    pub fn init(pins: Pins) -> Self {
        Self { reset: pins.0 }
    }

    pub fn start(&mut self) {
        self.reset.set_low();
        asm::delay(480_000); // ~ 1ms (datasheet specifies minimum 150ns)
        self.reset.set_high();
    }
}
