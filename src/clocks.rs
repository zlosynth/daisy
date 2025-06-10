//! Configuration of the onboard clock to accomodate for typical usage focused
//! on high-performance audio processing.

use crate::hal;
use hal::pac;
use hal::prelude::*;
use hal::pwr;
use hal::rcc;
use hal::rcc::rec::AdcClkSel;
use hal::time::Hertz;
use hal::time::MegaHertz;

use crate::audio;

// SAI clock uses pll3
const PLL3_P: Hertz = Hertz::from_raw(audio::FS.to_Hz() * 256);

pub trait SeedCrystal {
    const CRYSTAL_FREQ: MegaHertz = MegaHertz::from_raw(16);

    fn use_seed_crystal(self) -> Self;
}

impl SeedCrystal for rcc::Rcc {
    fn use_seed_crystal(self) -> Self {
        self.use_hse(Self::CRYSTAL_FREQ.convert())
    }
}

/// Configures the 16 MHz crystal, a 480 MHz system clock and PLL3 for
/// SAI audio.
///
/// The Daisy Seed has a 16 MHz crystal wired to the MCU's high-speed
/// external oscillator pins. We enable that, and use it to drive the
/// full 480 MHz system clock.
///
/// Usage:
///
/// ```
/// let dp = pac::Peripherals::take().unwrap();
/// let ccdr = configure(dp.PWR.constrain(), dp.RCC.constrain(), &dp.SYSCFG);
/// let clocks = configure(rcc);
/// ```
pub fn configure(pwr: pwr::Pwr, rcc: rcc::Rcc, syscfg: &pac::SYSCFG) -> rcc::Ccdr {
    let pwrcfg = pwr.vos0(syscfg).freeze();

    let mut ccdr = rcc
        .use_seed_crystal() // high speed external crystal @ 16 MHz
        .pll1_strategy(rcc::PllConfigStrategy::Iterative) // pll1 drives system clock
        .pll1_q_ck(48.MHz()) // required for SPI display
        .pll3_strategy(rcc::PllConfigStrategy::Fractional) // ensure we get as close as possible to 12.288 MHz (audio clock)
        .sys_ck(480.MHz()) // system clock @ 480 MHz
        .pll3_p_ck(PLL3_P) // audio clock  @ 12.288 MHz
        .freeze(pwrcfg, syscfg);

    // switch adc_ker_ck_input multiplexer to per_ck
    ccdr.peripheral.kernel_adc_clk_mux(AdcClkSel::Per);

    ccdr
}
