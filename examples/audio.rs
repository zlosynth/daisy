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

mod dsp;
use dsp::osc;

// - static global state ------------------------------------------------------

static AUDIO_INTERFACE: Mutex<RefCell<Option<audio::Interface>>> = Mutex::new(RefCell::new(None));

// - entry point --------------------------------------------------------------

#[entry]
fn main() -> ! {
    // - board setup ----------------------------------------------------------

    let board = daisy::Board::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let ccdr = daisy::board_freeze_clocks!(board, dp);
    let pins = daisy::board_split_gpios!(board, ccdr, dp);
    let mut led_user = daisy::board_split_leds!(pins).USER;
    let audio_interface = daisy::board_split_audio!(ccdr, pins);

    // - audio callback -------------------------------------------------------

    // handle callback with function pointer
    let audio_interface = {
        fn callback(fs: f32, block: &mut audio::Block) {
            static mut OSC_1: osc::Wavetable = osc::Wavetable::new(osc::Shape::Sin);
            static mut OSC_2: osc::Wavetable = osc::Wavetable::new(osc::Shape::Saw);
            unsafe { OSC_1.dx = (1. / fs) * 110.00 };
            unsafe { OSC_2.dx = (1. / fs) * 110.00 };
            for frame in block {
                *frame = (unsafe { OSC_1.step() }, unsafe { OSC_2.step() });
            }
        }

        audio_interface.spawn(callback)
    };

    let audio_interface = match audio_interface {
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

    let one_second = ccdr.clocks.sys_ck().0;

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
            match audio_interface.handle_interrupt_dma1_str1() {
                Ok(()) => (),
                Err(e) => {
                    loggit!("Failed to handle interrupt: {:?}", e);
                }
            };
        }
    });
}
