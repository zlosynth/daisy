//! Example of usage of the on-board 65 MB flash memory.

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    // Get device peripherals and the board abstraction.
    let board = daisy::Board::take().unwrap();
    let dp = daisy::pac::Peripherals::take().unwrap();

    // Configure board's peripherals.
    let ccdr = daisy::board_freeze_clocks!(board, dp);
    let pins = daisy::board_split_gpios!(board, ccdr, dp);
    let mut led_user = daisy::board_split_leds!(pins).USER;
    let mut flash = daisy::board_split_flash!(ccdr, dp, pins);

    // We will be using the first 8000 bytes of the flash.
    const ADDRESS: u32 = 0x00;
    const SIZE: usize = 8000;

    // Create an array of data to write.
    let mut data: [u8; SIZE] = [0; SIZE];
    for (i, x) in data.iter_mut().enumerate() {
        *x = (i % 256) as u8;
    }

    // Write it to the flash memory.
    flash.write(ADDRESS, &data);

    // Read it back.
    let mut buffer: [u8; SIZE] = [0; SIZE];
    flash.read(ADDRESS, &mut buffer);

    // Compare the read values with those written and lit the LED if they match.
    if data == buffer {
        led_user.set_high();
    } else {
        led_user.set_low();
    }

    // Sleep forever.
    loop {
        cortex_m::asm::nop();
    }
}
