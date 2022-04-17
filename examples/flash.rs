#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_semihosting as _;

use daisy::led::Led;

#[entry]
fn main() -> ! {
    // - board setup ----------------------------------------------------------

    let board = daisy::Board::take().unwrap();
    let dp = daisy::pac::Peripherals::take().unwrap();

    let ccdr = daisy::board_freeze_clocks!(board, dp);
    let pins = daisy::board_split_gpios!(board, ccdr, dp);
    let mut led_user = daisy::board_split_leds!(pins).USER;

    let mut flash =
        daisy::flash::Flash::new(&ccdr.clocks, dp.QUADSPI, ccdr.peripheral.QSPI, pins.FMC);

    // - test that what was written can be read back --------------------------

    const ADDRESS: u32 = 0x00;
    const SIZE: usize = 8000;

    // create the array
    let mut data: [u8; SIZE] = [0; SIZE];
    for (i, x) in data.iter_mut().enumerate() {
        *x = (i % 256) as u8;
    }

    // write it to memory
    flash.write(ADDRESS, &data);

    // read it back
    let mut buffer: [u8; SIZE] = [0; SIZE];
    flash.read(ADDRESS, &mut buffer);

    // and compare!
    if data == buffer {
        led_user.on();
    } else {
        led_user.off();
    }

    loop {
        cortex_m::asm::nop();
    }
}
