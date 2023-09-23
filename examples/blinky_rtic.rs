//! Example of basic interaction with the board, using RTIC.
//!
//! Read https://rtic.rs to learn more about the framework.

#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = stm32h7xx_hal::pac, peripherals = true, dispatchers = [EXTI0])]
mod app {
    use systick_monotonic::*;

    use daisy::led::LedUser;

    #[monotonic(binds = SysTick, default = true)]
    type Mono = Systick<1000>; // 1 kHz / 1 ms granularity

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: LedUser,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        // Get device peripherals and the board abstraction.
        let dp = cx.device;
        let board = daisy::Board::take().unwrap();

        // Configure board's peripherals.
        let ccdr = daisy::board_freeze_clocks!(board, dp);
        let pins = daisy::board_split_gpios!(board, ccdr, dp);
        let led = daisy::board_split_leds!(pins).USER;

        // Initialize monotonic timer.
        let mono = Systick::new(cx.core.SYST, ccdr.clocks.sys_ck().to_Hz());

        // Spawn blinking task.
        toggle_led::spawn().unwrap();

        (Shared {}, Local { led }, init::Monotonics(mono))
    }

    // Blink every second.
    #[task(local = [led])]
    fn toggle_led(cx: toggle_led::Context) {
        cx.local.led.toggle();
        toggle_led::spawn_after(1.secs()).unwrap();
    }
}
