use stm32h7xx_hal::gpio;

// - traits -------------------------------------------------------------------

/// Generic LED
pub trait Led {
    /// Turns the LED off
    fn off(&mut self);

    /// Turns the LED on
    fn on(&mut self);
}

// - types --------------------------------------------------------------------

#[allow(non_snake_case)]
pub struct Leds {
    pub USER: LedUser,
}

pub struct LedUser(pub gpio::gpioc::PC7<gpio::Output<gpio::PushPull>>);

impl LedUser {
    // TODO: https://github.com/stm32-rs/stm32h7xx-hal/pull/354
    pub fn new(pin: gpio::gpioc::PC7<gpio::Analog>) -> Self {
        LedUser(pin.into_push_pull_output())
    }
}

impl Led for LedUser {
    fn on(&mut self) {
        self.0.set_high();
    }

    fn off(&mut self) {
        self.0.set_low();
    }
}
