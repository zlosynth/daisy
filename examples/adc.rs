//! Example of how to directly access the ADC peripheral on the Daisy
//! Seed when using the Board Support Crate.

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_semihosting as _;

use cortex_m::asm;

use daisy::hal;
use hal::adc;
use hal::delay::Delay;
use hal::prelude::*;

#[entry]
fn main() -> ! {
    // - board setup ----------------------------------------------------------

    let board = daisy::Board::take().unwrap();
    let dp = daisy::pac::Peripherals::take().unwrap();

    let ccdr = daisy::board_freeze_clocks!(board, dp);
    let pins = daisy::board_split_gpios!(board, ccdr, dp);

    // - adc ------------------------------------------------------------------

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, ccdr.clocks);

    let mut adc1 =
        adc::Adc::adc1(dp.ADC1, &mut delay, ccdr.peripheral.ADC12, &ccdr.clocks).enable();
    adc1.set_resolution(adc::Resolution::SIXTEENBIT);

    #[cfg(any(feature = "seed", feature = "seed_1_1"))]
    let mut adc1_channel = pins.GPIO.PIN_21.into_analog();
    #[cfg(feature = "patch_sm")]
    let mut adc1_channel = pins.GPIO.PIN_C2.into_analog(); // CV_4

    // - led ------------------------------------------------------------------

    let mut led_user = pins.LED_USER.into_push_pull_output();

    // - main loop ------------------------------------------------------------

    let scale_factor = ccdr.clocks.sys_ck().to_Hz() as f32 / 65_535.;

    loop {
        let pot: u32 = adc1.read(&mut adc1_channel).unwrap();

        let ticks = (pot as f32 * scale_factor) as u32;

        led_user.set_high();
        asm::delay(ticks);

        led_user.set_low();
        asm::delay(ticks);
    }
}
