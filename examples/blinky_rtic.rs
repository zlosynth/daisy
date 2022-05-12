#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = stm32h7xx_hal::pac, peripherals = true, dispatchers = [EXTI0])]
mod app {
    use systick_monotonic::*;

    use daisy::led::{Led, LedUser};

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
        let mono = Systick::new(cx.core.SYST, 480_000_000);

        let dp = cx.device;
        let board = daisy::Board::take().unwrap();
        let ccdr = daisy::board_freeze_clocks!(board, dp);
        let pins = daisy::board_split_gpios!(board, ccdr, dp);
        let led = daisy::board_split_leds!(pins).USER;

        set_led::spawn(true).unwrap();

        (Shared {}, Local { led }, init::Monotonics(mono))
    }

    #[task(local = [led])]
    fn set_led(cx: set_led::Context, on: bool) {
        if on {
            cx.local.led.on();
            set_led::spawn_after(1.secs(), false).unwrap();
        } else {
            cx.local.led.off();
            set_led::spawn_after(1.secs(), true).unwrap();
        }
    }
}
