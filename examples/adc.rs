//! Example of how to directly access the ADC peripheral on the Daisy.

#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_semihosting as _;

use daisy::hal;
use hal::adc;
use hal::delay::Delay;
use hal::prelude::*;

#[entry]
fn main() -> ! {
    // Get core and device peripherals, and the board abstraction.
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = daisy::pac::Peripherals::take().unwrap();
    let board = daisy::Board::take().unwrap();

    // Configure board's peripherals.
    let ccdr = daisy::board_freeze_clocks!(board, dp);
    let pins = daisy::board_split_gpios!(board, ccdr, dp);

    // Configure ADC.
    let mut delay = Delay::new(cp.SYST, ccdr.clocks);
    let mut adc1 =
        adc::Adc::adc1(dp.ADC1, &mut delay, ccdr.peripheral.ADC12, &ccdr.clocks).enable();
    adc1.set_resolution(adc::Resolution::SIXTEENBIT);

    // Select a pin that will be used for ADC, depending on the board.
    #[cfg(any(feature = "seed", feature = "seed_1_1"))]
    let mut adc1_channel = pins.GPIO.PIN_21.into_analog();
    #[cfg(feature = "patch_sm")]
    let mut adc1_channel = pins.GPIO.PIN_C2.into_analog(); // CV_4

    // Get a handle on the on-board LED to later use as an indicator.
    let mut led_user = pins.LED_USER.into_push_pull_output();

    // Divide the processor clock with 2^16 (for 16 bit ADC resolution configured above).
    // Scaling by the result will produce zero delay for the lowest ADC value and
    // 1 second for the highest.
    let scale_factor = ccdr.clocks.sys_ck().to_Hz() as f32 / 65_535.0;

    loop {
        // Read value from the ADC pin.
        let pot: u32 = adc1.read(&mut adc1_channel).unwrap();

        // Scale the value to a delay counted in CPU ticks.
        let ticks = (pot as f32 * scale_factor) as u32;

        // Blink the LED with interval depending on the read value.
        led_user.set_high();
        asm::delay(ticks);
        led_user.set_low();
        asm::delay(ticks);
    }
}
