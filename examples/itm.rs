#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_semihosting as _;

use daisy::hal::prelude::*;
use daisy::led::Led;
use daisy::loggit;

#[entry]
fn main() -> ! {
    // - board setup ----------------------------------------------------------

    let board = daisy::Board::take().unwrap();
    let dp = daisy::pac::Peripherals::take().unwrap();

    let ccdr = daisy::board_freeze_clocks!(board, dp);
    let pins = daisy::board_split_gpios!(board, ccdr, dp);
    let mut led_user = daisy::board_split_leds!(pins).USER;

    loggit!("Hello daisy::itm !");

    // - main loop ------------------------------------------------------------

    let one_second = ccdr.clocks.sys_ck().0;
    let mut counter = 0;

    loop {
        loggit!("ping: {}", counter);
        counter += 1;

        led_user.on();
        cortex_m::asm::delay(one_second);
        led_user.off();
        cortex_m::asm::delay(one_second);
    }
}
