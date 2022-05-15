#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_semihosting as _;

use daisy::hal::prelude::_stm32h7xx_hal_spi_SpiExt;
use daisy::hal::prelude::_stm32h7xx_hal_timer_TimerExt;
use daisy::hal::{delay::DelayFromCountDownTimer, spi};
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
};
use fugit::RateExtU32;
use ssd1306::{prelude::*, Ssd1306};

#[entry]
fn main() -> ! {
    // - board setup ----------------------------------------------------------

    let board = daisy::Board::take().unwrap();
    let dp = daisy::pac::Peripherals::take().unwrap();

    let ccdr = daisy::board_freeze_clocks!(board, dp);
    let pins = daisy::board_split_gpios!(board, ccdr, dp);

    // - display setup --------------------------------------------------------
    // This example uses 128x64 display connected via 4-wire SPI. The pin
    // mapping:
    //
    // | Interface role | Display label | Seed pin | Patch SM pin |
    // |----------------|---------------|----------|--------------|
    // | SCK            | CLK           | 9        | D10          |
    // | CS             | CS            | 8        | D1           |
    // | MOSI           | DIN           | 11       | D9           |
    // | RST            | RES           | 37       | A9           |
    // | DC             | D/C           | 10       | D8           |

    let mut display = {
        #[cfg(any(feature = "seed", feature = "seed_1_1"))]
        let pins = (
            pins.GPIO.PIN_8.into_alternate(),
            pins.GPIO.PIN_7.into_push_pull_output(),
            pins.GPIO.PIN_10.into_alternate(),
            pins.GPIO.PIN_30.into_push_pull_output(),
            pins.GPIO.PIN_9.into_push_pull_output(),
        );
        #[cfg(feature = "patch_sm")]
        let pins = (
            pins.GPIO.PIN_D10.into_alternate(),
            pins.GPIO.PIN_D1.into_push_pull_output(),
            pins.GPIO.PIN_D9.into_alternate(),
            pins.GPIO.PIN_A9.into_push_pull_output(),
            pins.GPIO.PIN_D8.into_push_pull_output(),
        );

        let (sck, cs, mosi, mut rst, dc) = pins;

        #[cfg(any(feature = "seed", feature = "seed_1_1"))]
        let spi = dp.SPI1.spi(
            (sck, spi::NoMiso, mosi),
            spi::MODE_0,
            3.MHz(),
            ccdr.peripheral.SPI1,
            &ccdr.clocks,
        );
        #[cfg(feature = "patch_sm")]
        let spi = dp.SPI2.spi(
            (sck, spi::NoMiso, mosi),
            spi::MODE_0,
            3.MHz(),
            ccdr.peripheral.SPI2,
            &ccdr.clocks,
        );

        let interface = display_interface_spi::SPIInterface::new(spi, dc, cs);
        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        let mut delay = DelayFromCountDownTimer::new(dp.TIM2.timer(
            100.Hz(),
            ccdr.peripheral.TIM2,
            &ccdr.clocks,
        ));

        display.reset(&mut rst, &mut delay).unwrap();
        display.init().unwrap();

        display
    };

    // - main loop ------------------------------------------------------------

    let one_second = ccdr.clocks.sys_ck().to_Hz();

    loop {
        Rectangle::new(Point::new(0, 0), Size::new(127, 63))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
            .draw(&mut display)
            .unwrap();
        Circle::new(Point::new(39, 7), 51)
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)
            .unwrap();
        display.flush().unwrap();
        cortex_m::asm::delay(2 * one_second);

        Rectangle::new(Point::new(0, 0), Size::new(127, 63))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)
            .unwrap();
        Circle::new(Point::new(39, 7), 51)
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
            .draw(&mut display)
            .unwrap();
        display.flush().unwrap();
        cortex_m::asm::delay(2 * one_second);
    }
}
