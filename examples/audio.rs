//! Example of how to configure audio input output and implement a basic
//! passthrough swapping left and right channels.

#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m::asm;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_semihosting as _;

use hal::pac::{self, interrupt};
use stm32h7xx_hal as hal;

use daisy::audio;
use daisy::led::Led;

// Keep audio interface in a global, so it can be shared between functions.
static AUDIO_INTERFACE: Mutex<RefCell<Option<audio::Interface>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    // Get core and device peripherals.
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Using caches should provide a major performance boost.
    cp.SCB.enable_icache();
    // NOTE: Data caching requires cache management around all use of DMA.
    // This crate already handles that for audio processing.
    cp.SCB.enable_dcache(&mut cp.CPUID);

    // Initialize the board abstraction.
    let board = daisy::Board::take().unwrap();

    // Configure board's peripherals.
    let ccdr = daisy::board_freeze_clocks!(board, dp);
    let pins = daisy::board_split_gpios!(board, ccdr, dp);
    let mut led_user = daisy::board_split_leds!(pins).USER;
    let audio_interface = daisy::board_split_audio!(ccdr, pins);

    // Start audio processing and put its abstraction into a global.
    let audio_interface = audio_interface.spawn().unwrap();
    cortex_m::interrupt::free(|cs| {
        AUDIO_INTERFACE.borrow(cs).replace(Some(audio_interface));
    });

    // Keep blinking to block main and shows signs of life.
    let one_second = ccdr.clocks.sys_ck().to_Hz();
    loop {
        led_user.on();
        asm::delay(one_second);
        led_user.off();
        asm::delay(one_second);
    }
}

// Audio is tranfered from the input and to the input periodically thorugh DMA.
// Every time Daisy is done transferring data, it will ask for more by triggering
// the DMA 1 Stream 1 interrupt.
#[interrupt]
fn DMA1_STR1() {
    cortex_m::interrupt::free(|cs| {
        // Acquire the audio interface from the global.
        if let Some(audio_interface) = AUDIO_INTERFACE.borrow(cs).borrow_mut().as_mut() {
            // Read input audio from the buffer and write back desired
            // output samples.
            audio_interface
                .handle_interrupt_dma1_str1(|audio_buffer| {
                    for frame in audio_buffer {
                        let (left, right) = *frame;
                        *frame = (right * 0.8, left * 0.8);
                    }
                })
                .unwrap();
        }
    });
}
