use cortex_m::asm;
use embedded_hal::digital::v2::OutputPin;
use hal::gpio;
use hal::hal as embedded_hal;
use stm32h7xx_hal as hal;

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
        self.reset.set_low().unwrap();
        asm::delay(480_000); // ~ 1ms (datasheet specifies minimum 150ns)
        self.reset.set_high().unwrap();
    }
}
