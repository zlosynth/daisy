#![no_main]
#![no_std]

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_semihosting as _;

use daisy::pac;
use pac::interrupt;

use daisy::audio;
use daisy::led::Led;
use daisy::loggit;

// - static global state ------------------------------------------------------

static AUDIO_INTERFACE: Mutex<RefCell<Option<audio::Interface>>> = Mutex::new(RefCell::new(None));

// - entry point --------------------------------------------------------------

#[entry]
fn main() -> ! {
    // - board setup ----------------------------------------------------------

    // Using caches should provide a major performance boost.
    let mut cp = cortex_m::Peripherals::take().unwrap();
    cp.SCB.enable_icache();
    // NOTE: Data caching requires cache management around all use of DMA.
    // This crate already handles that for audio processing.
    cp.SCB.enable_dcache(&mut cp.CPUID);

    let board = daisy::Board::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let ccdr = daisy::board_freeze_clocks!(board, dp);
    let pins = daisy::board_split_gpios!(board, ccdr, dp);
    let mut led_user = daisy::board_split_leds!(pins).USER;
    let audio_interface = daisy::board_split_audio!(ccdr, pins);

    // - audio callback -------------------------------------------------------

    // handle callback with function pointer
    let audio_interface = match audio_interface.spawn() {
        Ok(audio_interface) => audio_interface,
        Err(e) => {
            loggit!("Failed to start audio interface: {:?}", e);
            #[allow(clippy::empty_loop)]
            loop {}
        }
    };

    cortex_m::interrupt::free(|cs| {
        AUDIO_INTERFACE.borrow(cs).replace(Some(audio_interface));
    });

    // - main loop ------------------------------------------------------------

    let one_second = ccdr.clocks.sys_ck().to_Hz();

    loop {
        led_user.on();
        asm::delay(one_second);
        led_user.off();
        asm::delay(one_second);
    }
}

// - interrupts ---------------------------------------------------------------

/// interrupt handler for: dma1, stream1
#[interrupt]
fn DMA1_STR1() {
    cortex_m::interrupt::free(|cs| {
        if let Some(audio_interface) = AUDIO_INTERFACE.borrow(cs).borrow_mut().as_mut() {
            match audio_interface.handle_interrupt_dma1_str1(|audio_buffer| {
                for frame in audio_buffer {
                    let (left, right) = *frame;
                    *frame = (right * 0.8, left * 0.8);
                }
            }) {
                Ok(()) => (),
                Err(e) => {
                    loggit!("Failed to handle interrupt: {:?}", e);
                }
            };
        }
    });
}
