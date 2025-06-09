//! Example of basic interaction with the board.

#![no_main]
#![no_std]

use cortex_m_rt::entry;

use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    // Get device peripherals and the board abstraction.
    let dp = daisy::pac::Peripherals::take().unwrap();
    let board = daisy::Board::take().unwrap();

    // Configure board's peripherals.
    let ccdr = daisy::board_freeze_clocks!(board, dp);
    let pins = daisy::board_split_gpios!(board, ccdr, dp);
    let mut led_user = daisy::board_split_leds!(pins).USER;

    // Blink every second.
    let one_second = ccdr.clocks.sys_ck().to_Hz();
    loop {
        led_user.toggle();
        cortex_m::asm::delay(one_second * 1);
        defmt::info!("Tick");
    }
}
