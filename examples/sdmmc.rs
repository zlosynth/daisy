//! Example of interaction with an SD card connected to the module's SDMMC interface.

#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use hal::gpio::Speed;
use hal::sdmmc::{SdCard, Sdmmc};
use hal::{pac, prelude::*};
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
    let dp = pac::Peripherals::take().unwrap();

    // Configure board's peripherals.
    let ccdr = daisy::board_freeze_clocks!(board, dp);
    let pins = daisy::board_split_gpios!(board, ccdr, dp);
    let mut led_user = daisy::board_split_leds!(pins).USER;
    let one_second = ccdr.clocks.sys_ck().to_Hz();

    // Select pins connected to the SD card depending on the used board.
    #[cfg(any(feature = "seed", feature = "seed_1_1"))]
    let (clk, cmd, d0, d1, d2, d3) = (
        pins.GPIO.PIN_6,
        pins.GPIO.PIN_5,
        pins.GPIO.PIN_4,
        pins.GPIO.PIN_3,
        pins.GPIO.PIN_2,
        pins.GPIO.PIN_1,
    );
    #[cfg(feature = "patch_sm")]
    let (clk, cmd, d0, d1, d2, d3) = (
        pins.GPIO.PIN_D6,
        pins.GPIO.PIN_D7,
        pins.GPIO.PIN_D5,
        pins.GPIO.PIN_D4,
        pins.GPIO.PIN_D3,
        pins.GPIO.PIN_D2,
    );

    // Configure the pins.
    let clk = clk
        .into_alternate::<12>()
        .internal_pull_up(false)
        .speed(Speed::VeryHigh);
    let clk = clk
        .into_alternate()
        .internal_pull_up(false)
        .speed(Speed::VeryHigh);
    let cmd = cmd
        .into_alternate()
        .internal_pull_up(true)
        .speed(Speed::VeryHigh);
    let d0 = d0
        .into_alternate()
        .internal_pull_up(true)
        .speed(Speed::VeryHigh);
    let d1 = d1
        .into_alternate()
        .internal_pull_up(true)
        .speed(Speed::VeryHigh);
    let d2 = d2
        .into_alternate()
        .internal_pull_up(true)
        .speed(Speed::VeryHigh);
    let d3 = d3
        .into_alternate()
        .internal_pull_up(true)
        .speed(Speed::VeryHigh);

    // Initialize abstraction of the SDMMC interface.
    let mut sdmmc: Sdmmc<_, SdCard> = dp.SDMMC1.sdmmc(
        (clk, cmd, d0, d1, d2, d3),
        ccdr.peripheral.SDMMC1,
        &ccdr.clocks,
    );

    // This can be increased up to 50MHz. We choose a lower frequency here so
    // that it should work even with flying leads connected to a SD card breakout.
    let bus_frequency = 2.MHz();

    // Try to connect to the SD card. Blink rapidly to signal waiting.
    while sdmmc.init(bus_frequency).is_err() {
        led_user.toggle();
        asm::delay(one_second / 8);
    }

    // Write to the card, read back, test that the values match.
    let mut buffer = [0x34; 512];
    log!("Writting to the card");
    sdmmc.write_block(0, &buffer).unwrap();
    log!("Reading from the card");
    sdmmc.read_block(0, &mut buffer).unwrap();
    for byte in buffer.iter() {
        assert_eq!(*byte, 0x34);
    }
    log!("All went as expected");

    // Keep blinking to block main and shows signs of life and to show that
    // the test above passed.
    let one_second = ccdr.clocks.sys_ck().to_Hz();
    loop {
        led_user.toggle();
        asm::delay(one_second);
    }
}
