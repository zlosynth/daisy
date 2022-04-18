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
use hal::rcc::rec::AdcClkSel;

#[entry]
fn main() -> ! {
    // - board setup ----------------------------------------------------------

    let board = daisy::Board::take().unwrap();
    let dp = daisy::pac::Peripherals::take().unwrap();

    let mut ccdr = daisy::board_freeze_clocks!(board, dp);

    // switch adc_ker_ck_input multiplexer to per_ck
    ccdr.peripheral.kernel_adc_clk_mux(AdcClkSel::PER);

    let pins = daisy::board_split_gpios!(board, ccdr, dp);

    // - adc ------------------------------------------------------------------

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, ccdr.clocks);

    let mut adc1 =
        adc::Adc::adc1(dp.ADC1, &mut delay, ccdr.peripheral.ADC12, &ccdr.clocks).enable();
    adc1.set_resolution(adc::Resolution::SIXTEENBIT);

    let mut adc1_channel_4 = pins.SEED_PIN_21.into_analog(); // Daisy Pod: POT_1
    let mut adc1_channel_0 = pins.SEED_PIN_15.into_analog(); // Daisy Pod: POT_2

    // - led ------------------------------------------------------------------

    let mut led_user = pins.LED_USER.into_push_pull_output();

    // - main loop ------------------------------------------------------------

    let scale_factor = ccdr.clocks.sys_ck().to_Hz() as f32 / 65_535.;

    loop {
        let pot_1: u32 = adc1.read(&mut adc1_channel_4).unwrap();
        let _pot_2: u32 = adc1.read(&mut adc1_channel_0).unwrap();

        let ticks = (pot_1 as f32 * scale_factor) as u32;

        led_user.set_high();
        asm::delay(ticks);

        led_user.set_low();
        asm::delay(ticks);
    }
}
