// Based on https://github.com/electro-smith/libDaisy/blob/ae9b45e2927aafba5f261f2ff36e3f41ae74d019/src/daisy_seed.cpp#L299.

use crate::hal;
use hal::gpio;

type DeemphasisPin = gpio::gpiob::PB11<gpio::Output<gpio::PushPull>>; // DEMP

pub type Pins = (DeemphasisPin,);

pub struct Codec {
    deemphasis: DeemphasisPin,
}

impl Codec {
    pub fn init(pins: Pins) -> Self {
        Self {
            deemphasis: pins.0.into_push_pull_output(),
        }
    }

    pub fn start(&mut self) {
        self.deemphasis.set_low();
    }
}
