use stm32h7xx_hal as hal;

use crate::clocks;
use crate::led;
use crate::pins::*;

// - global static state ------------------------------------------------------

// `no_mangle` is used here to prevent linking different minor
// versions of this crate as that would let you `take` the core
// peripherals more than once (one per minor version)
#[no_mangle]
static DAISY_BOARD: () = ();

/// Set to `true` when `take` was called to make `Board` a singleton.
static mut TAKEN: bool = false;

// - Board --------------------------------------------------------------------

pub struct Board;

impl Board {
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { TAKEN } {
                None
            } else {
                Some(unsafe { Board::steal() })
            }
        })
    }

    #[inline]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn steal() -> Self {
        Board
    }

    pub fn freeze_clocks(
        &self,
        pwr: hal::pwr::Pwr,
        rcc: hal::rcc::Rcc,
        syscfg: &hal::device::SYSCFG,
    ) -> hal::rcc::Ccdr {
        clocks::configure(pwr, rcc, syscfg)
    }

    /// Takes the board's GPIO peripherals and split them into ZST's
    /// representing the individual GPIO pins used by the board.
    #[allow(clippy::too_many_arguments)]
    pub fn split_gpios(
        &self,
        gpioa: hal::gpio::gpioa::Parts,
        gpiob: hal::gpio::gpiob::Parts,
        gpioc: hal::gpio::gpioc::Parts,
        gpiod: hal::gpio::gpiod::Parts,
        gpioe: hal::gpio::gpioe::Parts,
        gpiof: hal::gpio::gpiof::Parts,
        gpiog: hal::gpio::gpiog::Parts,
        #[allow(unused_variables)] // GPIO H is only utilized on Daisy 1.1
        gpioh: hal::gpio::gpioh::Parts,
    ) -> Pins {
        Pins {
            #[cfg(any(feature = "seed", feature = "seed_1_1"))]
            GPIO: Gpio {
                PIN_0: gpiob.pb12,
                PIN_1: gpioc.pc11,
                PIN_2: gpioc.pc10,
                PIN_3: gpioc.pc9,
                PIN_4: gpioc.pc8,
                PIN_5: gpiod.pd2,
                PIN_6: gpioc.pc12,
                PIN_7: gpiog.pg10,
                PIN_8: gpiog.pg11,
                PIN_9: gpiob.pb4,
                PIN_10: gpiob.pb5,
                PIN_11: gpiob.pb8,
                PIN_12: gpiob.pb9,
                PIN_13: gpiob.pb6,
                PIN_14: gpiob.pb7,
                PIN_15: gpioc.pc0,
                PIN_16: gpioa.pa3,
                PIN_17: gpiob.pb1,
                PIN_18: gpioa.pa7,
                PIN_19: gpioa.pa6,
                PIN_20: gpioc.pc1,
                PIN_21: gpioc.pc4,
                PIN_22: gpioa.pa5,
                PIN_23: gpioa.pa4,
                PIN_24: gpioa.pa1,
                PIN_25: gpioa.pa0,
                PIN_26: gpiod.pd11,
                PIN_27: gpiog.pg9,
                PIN_28: gpioa.pa2,
                PIN_29: gpiob.pb14,
                PIN_30: gpiob.pb15,
            },
            #[cfg(feature = "patch_sm")]
            GPIO: Gpio {
                PIN_A2: gpioa.pa1,
                PIN_A3: gpioa.pa0,
                PIN_A8: gpiob.pb14,
                PIN_A9: gpiob.pb15,
                PIN_B5: gpioc.pc14,
                PIN_B6: gpioc.pc13,
                PIN_B7: gpiob.pb8,
                PIN_B8: gpiob.pb9,
                PIN_B9: gpiog.pg14,
                PIN_B10: gpiog.pg13,
                PIN_C1: gpioa.pa5,
                PIN_C2: gpioa.pa7,
                PIN_C3: gpioa.pa2,
                PIN_C4: gpioa.pa6,
                PIN_C5: gpioa.pa3,
                PIN_C6: gpiob.pb1,
                PIN_C7: gpioc.pc4,
                PIN_C8: gpioc.pc0,
                PIN_C9: gpioc.pc1,
                PIN_C10: gpioa.pa4,
                PIN_D1: gpiob.pb4,
                PIN_D2: gpioc.pc11,
                PIN_D3: gpioc.pc10,
                PIN_D4: gpioc.pc9,
                PIN_D5: gpioc.pc8,
                PIN_D6: gpioc.pc12,
                PIN_D7: gpiod.pd2,
                PIN_D8: gpioc.pc2,
                PIN_D9: gpioc.pc3,
                PIN_D10: gpiog.pg3,
            },
            LED_USER: gpioc.pc7,
            #[cfg(feature = "seed")]
            CODEC: CodecPins {
                PDN: gpiob.pb11, // Codec Reset (AK4556)
            },
            #[cfg(feature = "seed_1_1")]
            CODEC: CodecPins {
                SCL: gpioh.ph4,  // I2C2 SCL (WM8731)
                SDA: gpiob.pb11, // I2C2 SDA (WM8731)
            },
            #[cfg(feature = "patch_sm")]
            CODEC: CodecPins {
                SCL: gpiob.pb10, // I2C2 SCL (PCM3060)
                SDA: gpiob.pb11, // I2C2 SDA (PCM3060)
            },
            SAI: SaiPins {
                MCLK_A: gpioe.pe2, // SAI1 MCLK_A
                SCK_A: gpioe.pe5,  // SAI1 SCK_A
                FS_A: gpioe.pe4,   // SAI1 FS_A
                SD_A: gpioe.pe6,   // SAI1 SD_A
                SD_B: gpioe.pe3,   // SAI1 SD_B
            },
            FMC: FMCPins {
                IO0: gpiof.pf8,
                IO1: gpiof.pf9,
                IO2: gpiof.pf7,
                IO3: gpiof.pf6,
                SCK: gpiof.pf10,
                CS: gpiog.pg6,
            },
            SDRAM: (),
            USB2: USB2Pins {
                DN: gpioa.pa11, // USB2 D-
                DP: gpioa.pa12, // USB2 D+
            },
        }
    }

    pub fn split_led_user(&self, pin: LedUserPin) -> led::LedUser {
        led::LedUser::new(pin)
    }
}

// - macros -------------------------------------------------------------------

#[macro_export]
macro_rules! board_freeze_clocks {
    ($board:expr, $dp:expr) => {{
        use daisy::hal::prelude::_stm32h7xx_hal_pwr_PwrExt;
        use daisy::hal::prelude::_stm32h7xx_hal_rcc_RccExt;
        $board.freeze_clocks($dp.PWR.constrain(), $dp.RCC.constrain(), &$dp.SYSCFG)
    }};
}

#[macro_export]
macro_rules! board_split_gpios {
    ($board:expr, $ccdr:expr, $dp:expr) => {{
        use daisy::hal::gpio::GpioExt;
        $board.split_gpios(
            $dp.GPIOA.split($ccdr.peripheral.GPIOA),
            $dp.GPIOB.split($ccdr.peripheral.GPIOB),
            $dp.GPIOC.split($ccdr.peripheral.GPIOC),
            $dp.GPIOD.split($ccdr.peripheral.GPIOD),
            $dp.GPIOE.split($ccdr.peripheral.GPIOE),
            $dp.GPIOF.split($ccdr.peripheral.GPIOF),
            $dp.GPIOG.split($ccdr.peripheral.GPIOG),
            $dp.GPIOH.split($ccdr.peripheral.GPIOH),
        )
    }};
}

#[macro_export]
macro_rules! board_split_audio {
    ($ccdr:expr, $pins:expr) => {{
        #[cfg(feature = "seed")]
        let codec_pins = ($pins.CODEC.PDN.into_push_pull_output(),);

        #[cfg(any(feature = "seed_1_1", feature = "patch_sm"))]
        let codec_pins = (
            $pins.CODEC.SCL.into_alternate::<4>().set_open_drain(),
            $pins.CODEC.SDA.into_alternate::<4>().set_open_drain(),
        );

        let sai1_pins = (
            $pins.SAI.MCLK_A.into_alternate::<6>(),
            $pins.SAI.SCK_A.into_alternate::<6>(),
            $pins.SAI.FS_A.into_alternate::<6>(),
            $pins.SAI.SD_A.into_alternate::<6>(),
            Some($pins.SAI.SD_B.into_alternate::<6>()),
        );

        let sai1_prec = $ccdr
            .peripheral
            .SAI1
            .kernel_clk_mux(daisy::hal::rcc::rec::Sai1ClkSel::PLL3_P);

        daisy::audio::Interface::init(
            &$ccdr.clocks,
            sai1_prec,
            sai1_pins,
            codec_pins,
            $ccdr.peripheral.I2C2,
            $ccdr.peripheral.DMA1,
        )
        .unwrap()
    }};
}

#[macro_export]
macro_rules! board_split_leds {
    ($pins:expr) => {{
        daisy::led::Leds {
            USER: daisy::led::LedUser::new($pins.LED_USER),
        }
    }};
}

#[macro_export]
macro_rules! board_split_flash {
    ($ccdr:expr, $dp:expr, $pins:expr) => {{
        daisy::flash::Flash::new(&$ccdr.clocks, $dp.QUADSPI, $ccdr.peripheral.QSPI, $pins.FMC)
    }};
}
