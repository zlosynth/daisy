//! Example of how to configure audio input output and implement a basic
//! passthrough swapping left and right channels, using RTIC.
//!
//! Read https://rtic.rs to learn more about the framework.

#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = daisy::pac, peripherals = true)]
mod app {
    use systick_monotonic::*;

    use daisy::audio::Interface;

    #[monotonic(binds = SysTick, default = true)]
    type Mono = Systick<1000>; // 1 kHz / 1 ms granularity

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        audio_interface: Interface,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        // Get device peripherals.
        let mut cp = cx.core;
        let dp = cx.device;

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
        let audio_interface = daisy::board_split_audio!(ccdr, pins);

        // Start audio processing and put its abstraction into a global.
        let audio_interface = audio_interface.spawn().unwrap();

        // Initialize monotonic timer.
        let mono = Systick::new(cp.SYST, ccdr.clocks.sys_ck().to_Hz());

        (Shared {}, Local { audio_interface }, init::Monotonics(mono))
    }

    // Audio is tranfered from the input and to the input periodically thorugh DMA.
    // Every time Daisy is done transferring data, it will ask for more by triggering
    // the DMA 1 Stream 1 interrupt.
    #[task(binds = DMA1_STR1, local = [audio_interface])]
    fn dsp(cx: dsp::Context) {
        let audio_interface = cx.local.audio_interface;

        audio_interface
            .handle_interrupt_dma1_str1(|audio_buffer| {
                for frame in audio_buffer {
                    let (left, right) = *frame;
                    *frame = (right * 0.8, left * 0.8);
                }
            })
            .unwrap();
    }
}
