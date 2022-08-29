#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_semihosting as _;

use daisy::led::Led;
use daisy::pac;
use daisy::sdram::Size;

// - entry point --------------------------------------------------------------

#[entry]
fn main() -> ! {
    // - board setup ----------------------------------------------------------

    let board = daisy::Board::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let ccdr = daisy::board_freeze_clocks!(board, dp);
    let pins = daisy::board_split_gpios!(board, ccdr, dp);
    let mut led_user = daisy::board_split_leds!(pins).USER;
    let sdram = daisy::board_split_sdram!(Size::SixtyFour, cp, dp, ccdr, pins);

    // - RAM handling ---------------------------------------------------------

    // Initialize a slice
    let ram_slice = unsafe {
        let ram_items = sdram.size() / core::mem::size_of::<u16>();
        let ram_ptr = sdram.base_address as *mut u16;
        core::slice::from_raw_parts_mut(ram_ptr, ram_items)
    };

    // Use memory in SDRAM
    ram_slice[0] = 1u16;
    ram_slice[3] = 2;
    ram_slice[ram_slice.len() - 1] = 3;
    assert_eq!(ram_slice[0], 1);
    assert_eq!(ram_slice[3], 2);
    assert_eq!(ram_slice[ram_slice.len() - 1], 3);

    // - main loop ------------------------------------------------------------

    let one_second = ccdr.clocks.sys_ck().to_Hz();

    loop {
        led_user.on();
        asm::delay(one_second);
        led_user.off();
        asm::delay(one_second);
    }
}
