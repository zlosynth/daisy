//! Example of usage of the on-board 64 MB SDRAM.

#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use hal::pac;
use stm32h7xx_hal as hal;

macro_rules! log {
    ($message:expr) => {
        #[cfg(feature = "defmt")]
        defmt::info!($message);
    };
}

#[entry]
fn main() -> ! {
    // Get core and device peripherals, and the board abstraction.
    let board = daisy::Board::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Configure board's peripherals.
    let ccdr = daisy::board_freeze_clocks!(board, dp);
    let pins = daisy::board_split_gpios!(board, ccdr, dp);
    let mut led_user = daisy::board_split_leds!(pins).USER;
    let sdram = daisy::board_split_sdram!(cp, dp, ccdr, pins);

    // Initialize a slice placed in the SDRAM. In this example, we use the whole
    // space as `u16` slice. However, it can be mapped to any type.
    let ram_slice = unsafe {
        let ram_items = sdram.size() / core::mem::size_of::<u16>();
        let ram_ptr = sdram.base_address as *mut u16;
        core::slice::from_raw_parts_mut(ram_ptr, ram_items)
    };

    // Test the SDRAM memory by writing to it and reading back.
    log!("Writting into RAM");
    ram_slice[0] = 1u16;
    ram_slice[3] = 2;
    ram_slice[ram_slice.len() - 1] = 3;
    assert_eq!(ram_slice[0], 1);
    assert_eq!(ram_slice[3], 2);
    assert_eq!(ram_slice[ram_slice.len() - 1], 3);
    log!("All went as expected");

    // Keep blinking to block main and shows signs of life and to show that
    // the test above passed.
    let one_second = ccdr.clocks.sys_ck().to_Hz();
    loop {
        led_user.toggle();
        asm::delay(one_second);
    }
}
