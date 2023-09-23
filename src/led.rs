//! Simple abstraction of the on-board status LED.

use crate::hal::gpio::{self, PinMode};

pub type LedUser = gpio::gpioc::PC7<gpio::Output<gpio::PushPull>>;

#[allow(non_snake_case)]
pub struct Leds {
    pub USER: LedUser,
}

impl Leds {
    pub fn new<MODE: PinMode>(user_pin: gpio::gpioc::PC7<MODE>) -> Self {
        Self {
            USER: user_pin.into_push_pull_output(),
        }
    }
}
