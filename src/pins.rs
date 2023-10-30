//! Mapping of both internal and external pins.

use crate::hal;
use hal::gpio;

// - types --------------------------------------------------------------------

// https://github.com/electro-smith/DaisyWiki/wiki/2.-Daisy-Seed-Pinout
#[cfg(any(feature = "seed", feature = "seed_1_1", feature = "seed_1_2"))]
#[allow(non_snake_case)]
pub struct Gpio {
    pub PIN_0: gpio::gpiob::PB12<gpio::Analog>, // STM PIN 01, USB OTG ID, I2C3 SCL
    pub PIN_1: gpio::gpioc::PC11<gpio::Analog>, // STM PIN 02, SD Data3, USART3 Rx
    pub PIN_2: gpio::gpioc::PC10<gpio::Analog>, // STM PIN 03, SD Data2, USART3 Tx
    pub PIN_3: gpio::gpioc::PC9<gpio::Analog>,  // STM PIN 04, SD Data1, I2C3 SDA
    pub PIN_4: gpio::gpioc::PC8<gpio::Analog>,  // STM PIN 05, SD Data0
    pub PIN_5: gpio::gpiod::PD2<gpio::Analog>,  // STM PIN 06, SD CMD, UART5 Rx
    pub PIN_6: gpio::gpioc::PC12<gpio::Analog>, // STM PIN 07, SD CLK, UART5 Tx
    pub PIN_7: gpio::gpiog::PG10<gpio::Analog>, // STM PIN 08, SPI1 CS
    pub PIN_8: gpio::gpiog::PG11<gpio::Analog>, // STM PIN 09, SPI1 SCK, SPDIFRX1
    pub PIN_9: gpio::gpiob::PB4<gpio::Alternate<0>>, // STM PIN 10, SPI1 MOSI
    pub PIN_10: gpio::gpiob::PB5<gpio::Analog>, // STM PIN 11, SPI1 MISO
    pub PIN_11: gpio::gpiob::PB8<gpio::Analog>, // STM PIN 12, I2C1 SCL, UART4 Rx
    pub PIN_12: gpio::gpiob::PB9<gpio::Analog>, // STM PIN 13, I2C1 SDA, UART4 Tx
    pub PIN_13: gpio::gpiob::PB6<gpio::Analog>, // STM PIN 14, USART1 Tx, I2C4 SCL
    pub PIN_14: gpio::gpiob::PB7<gpio::Analog>, // STM PIN 15, USART1 Rx, I2C4 SDA
    pub PIN_15: gpio::gpioc::PC0<gpio::Analog>, // STM PIN 22, ADC 0
    pub PIN_16: gpio::gpioa::PA3<gpio::Analog>, // STM PIN 23, ADC 1
    pub PIN_17: gpio::gpiob::PB1<gpio::Analog>, // STM PIN 24, ADC 2
    pub PIN_18: gpio::gpioa::PA7<gpio::Analog>, // STM PIN 25, ADC 3
    pub PIN_19: gpio::gpioa::PA6<gpio::Analog>, // STM PIN 26, ADC 4
    pub PIN_20: gpio::gpioc::PC1<gpio::Analog>, // STM PIN 27, ADC 5
    pub PIN_21: gpio::gpioc::PC4<gpio::Analog>, // STM PIN 28, ADC 6
    pub PIN_22: gpio::gpioa::PA5<gpio::Analog>, // STM PIN 29, DAC OUT 2, ADC 7
    pub PIN_23: gpio::gpioa::PA4<gpio::Analog>, // STM PIN 30, DAC OUT 1, ADC 8
    pub PIN_24: gpio::gpioa::PA1<gpio::Analog>, // STM PIN 31, SAI2 MCLK, ADC 9
    pub PIN_25: gpio::gpioa::PA0<gpio::Analog>, // STM PIN 32, SAI2 SD B, ADC 10
    pub PIN_26: gpio::gpiod::PD11<gpio::Analog>, // STM PIN 33, SAI2 SD A
    pub PIN_27: gpio::gpiog::PG9<gpio::Analog>, // STM PIN 34, SAI2 SD FS
    pub PIN_28: gpio::gpioa::PA2<gpio::Analog>, // STM PIN 35, SAI2 SCK, ADC 11
    pub PIN_29: gpio::gpiob::PB14<gpio::Analog>, // STM PIN 36, USB1 D-, USART1 Tx
    pub PIN_30: gpio::gpiob::PB15<gpio::Analog>, // STM PIN 37, USB1 D+, USART1 Rx
}

// ES_Patch_SM_datasheet_v1.0.3.pdf
// * 47K pullups are connected to this pin. This may have an effect on the behavior when used as UART
#[cfg(feature = "patch_sm")]
#[allow(non_snake_case)]
pub struct Gpio {
    pub PIN_A2: gpio::gpioa::PA1<gpio::Analog>, // ADC_9, GPIO, UART4_RX
    pub PIN_A3: gpio::gpioa::PA0<gpio::Analog>, // ADC_10, GPIO, UART4_TX
    pub PIN_A8: gpio::gpiob::PB14<gpio::Analog>, // USB_DM, GPIO, USART1_TX
    pub PIN_A9: gpio::gpiob::PB15<gpio::Analog>, // USB_DP, GPIO, USART1_RX
    pub PIN_B5: gpio::gpioc::PC13<gpio::Analog>, // GATE_OUT_1 Output Only
    pub PIN_B6: gpio::gpioc::PC14<gpio::Analog>, // GATE_OUT_2 Output Only
    pub PIN_B7: gpio::gpiob::PB8<gpio::Analog>, // I2C1_SCL, GPIO, UART4_RX, PWM (TIM4_CH3)
    pub PIN_B8: gpio::gpiob::PB9<gpio::Analog>, // I2C1_SDA, GPIO, UART4_TX, PWM (TIM4_CH4)
    pub PIN_B9: gpio::gpiog::PG14<gpio::Analog>, // GATE_IN_2, Input Only
    pub PIN_B10: gpio::gpiog::PG13<gpio::Analog>, // GATE_IN_1, Input Only
    pub PIN_C1: gpio::gpioa::PA5<gpio::Analog>, // CV_OUT_2, Output Only
    pub PIN_C2: gpio::gpioa::PA7<gpio::Analog>, // CV_4, Input Only
    pub PIN_C3: gpio::gpioa::PA2<gpio::Analog>, // CV_3, Input Only
    pub PIN_C4: gpio::gpioa::PA6<gpio::Analog>, // CV_2, Input Only
    pub PIN_C5: gpio::gpioa::PA3<gpio::Analog>, // CV_1, Input Only
    // FIXME: https://github.com/zlosynth/daisy/issues/1
    pub PIN_C6: gpio::gpioc::PC1<gpio::Analog>, // CV_8, Input Only
    pub PIN_C7: gpio::gpioc::PC0<gpio::Analog>, // CV_7, Input Only
    pub PIN_C8: gpio::gpiob::PB1<gpio::Analog>, // CV_5, Input Only
    pub PIN_C9: gpio::gpioc::PC4<gpio::Analog>, // CV_6, Input Only
    pub PIN_C10: gpio::gpioa::PA4<gpio::Analog>, // CV_OUT_1, Output Only
    pub PIN_D1: gpio::gpiob::PB4<gpio::Alternate<0>>, // SPI2_CS, GPIO,
    pub PIN_D2: gpio::gpioc::PC11<gpio::Analog>, // SDMMC1_D3, GPIO, USART3_RX*
    pub PIN_D3: gpio::gpioc::PC10<gpio::Analog>, // SDMMC1_D2, GPIO, USART3_TX*
    pub PIN_D4: gpio::gpioc::PC9<gpio::Analog>, // SDMMC1_D1, GPIO
    pub PIN_D5: gpio::gpioc::PC8<gpio::Analog>, // SDMMC1_D0, GPIO
    pub PIN_D6: gpio::gpioc::PC12<gpio::Analog>, // SDMMC1_CLK, GPIO, UART5_TX*
    pub PIN_D7: gpio::gpiod::PD2<gpio::Analog>, // SDMMC1_CMD, GPIO, UART5_RX*
    pub PIN_D8: gpio::gpioc::PC2<gpio::Analog>, // ADC_12, GPIO, SPI2_MISO
    pub PIN_D9: gpio::gpioc::PC3<gpio::Analog>, // ADC_11, GPIO, SPI2_MOSI
    pub PIN_D10: gpio::gpiod::PD3<gpio::Analog>, // SPI2_SCK, GPIO
}

pub type LedUserPin = gpio::gpioc::PC7<gpio::Analog>; // LED_USER

#[cfg(feature = "seed")]
#[allow(non_snake_case)]
pub struct CodecPins {
    pub PDN: gpio::gpiob::PB11<gpio::Analog>, // Codec Reset (AK4556)
}

#[cfg(feature = "seed_1_1")]
#[allow(non_snake_case)]
pub struct CodecPins {
    pub SCL: gpio::gpioh::PH4<gpio::Analog>,  // I2C2 SCL (WM8731)
    pub SDA: gpio::gpiob::PB11<gpio::Analog>, // I2C2 SDA (WM8731)
}

#[cfg(feature = "seed_1_2")]
#[allow(non_snake_case)]
pub struct CodecPins {
    pub DEMP: gpio::gpiob::PB11<gpio::Analog>, // DEMP (PCM3060)
}

#[cfg(feature = "patch_sm")]
#[allow(non_snake_case)]
pub struct CodecPins {
    pub SCL: gpio::gpiob::PB10<gpio::Analog>, // I2C2 SCL (PCM3060)
    pub SDA: gpio::gpiob::PB11<gpio::Analog>, // I2C2 SDA (PCM3060)
}

#[allow(non_snake_case)]
pub struct SaiPins {
    pub MCLK_A: gpio::gpioe::PE2<gpio::Analog>, // SAI1 MCLK_A
    pub SCK_A: gpio::gpioe::PE5<gpio::Analog>,  // SAI1 SCK_A
    pub FS_A: gpio::gpioe::PE4<gpio::Analog>,   // SAI1 FS_A
    pub SD_A: gpio::gpioe::PE6<gpio::Analog>,   // SAI1 SD_A
    pub SD_B: gpio::gpioe::PE3<gpio::Analog>,   // SAI1 SD_B
}

#[allow(non_snake_case)]
pub struct USB2Pins {
    pub DN: gpio::gpioa::PA11<gpio::Analog>, // USB2 D-
    pub DP: gpio::gpioa::PA12<gpio::Analog>, // USB2 D+
}

#[allow(non_snake_case)]
pub struct FlashPins {
    // https://github.com/electro-smith/libDaisy/blob/3dda55e9ed55a2f8b6bc4fa6aa2c7ae134c317ab/src/per/qspi.c#L695
    pub IO0: gpio::gpiof::PF8<gpio::Analog>, // (SI)
    pub IO1: gpio::gpiof::PF9<gpio::Analog>, // (SO)
    pub IO2: gpio::gpiof::PF7<gpio::Analog>,
    pub IO3: gpio::gpiof::PF6<gpio::Analog>,
    pub SCK: gpio::gpiof::PF10<gpio::Analog>,
    pub CS: gpio::gpiog::PG6<gpio::Analog>,
}

#[allow(non_snake_case)]
pub struct SDRAMPins {
    // https://github.com/x37v/libdaisy-rust/blob/0279b7b9d9a7cc867e7fce0d07287acf8ce3f72c/src/sdram.rs
    pub A0: gpio::gpiof::PF0<gpio::Analog>,
    pub A1: gpio::gpiof::PF1<gpio::Analog>,
    pub A2: gpio::gpiof::PF2<gpio::Analog>,
    pub A3: gpio::gpiof::PF3<gpio::Analog>,
    pub A4: gpio::gpiof::PF4<gpio::Analog>,
    pub A5: gpio::gpiof::PF5<gpio::Analog>,
    pub A6: gpio::gpiof::PF12<gpio::Analog>,
    pub A7: gpio::gpiof::PF13<gpio::Analog>,
    pub A8: gpio::gpiof::PF14<gpio::Analog>,
    pub A9: gpio::gpiof::PF15<gpio::Analog>,
    pub A10: gpio::gpiog::PG0<gpio::Analog>,
    pub A11: gpio::gpiog::PG1<gpio::Analog>,
    pub A12: gpio::gpiog::PG2<gpio::Analog>,
    pub BA0: gpio::gpiog::PG4<gpio::Analog>,
    pub BA1: gpio::gpiog::PG5<gpio::Analog>,
    pub D0: gpio::gpiod::PD14<gpio::Analog>,
    pub D1: gpio::gpiod::PD15<gpio::Analog>,
    pub D2: gpio::gpiod::PD0<gpio::Analog>,
    pub D3: gpio::gpiod::PD1<gpio::Analog>,
    pub D4: gpio::gpioe::PE7<gpio::Analog>,
    pub D5: gpio::gpioe::PE8<gpio::Analog>,
    pub D6: gpio::gpioe::PE9<gpio::Analog>,
    pub D7: gpio::gpioe::PE10<gpio::Analog>,
    pub D8: gpio::gpioe::PE11<gpio::Analog>,
    pub D9: gpio::gpioe::PE12<gpio::Analog>,
    pub D10: gpio::gpioe::PE13<gpio::Analog>,
    pub D11: gpio::gpioe::PE14<gpio::Analog>,
    pub D12: gpio::gpioe::PE15<gpio::Analog>,
    pub D13: gpio::gpiod::PD8<gpio::Analog>,
    pub D14: gpio::gpiod::PD9<gpio::Analog>,
    pub D15: gpio::gpiod::PD10<gpio::Analog>,
    pub D16: gpio::gpioh::PH8<gpio::Analog>,
    pub D17: gpio::gpioh::PH9<gpio::Analog>,
    pub D18: gpio::gpioh::PH10<gpio::Analog>,
    pub D19: gpio::gpioh::PH11<gpio::Analog>,
    pub D20: gpio::gpioh::PH12<gpio::Analog>,
    pub D21: gpio::gpioh::PH13<gpio::Analog>,
    pub D22: gpio::gpioh::PH14<gpio::Analog>,
    pub D23: gpio::gpioh::PH15<gpio::Analog>,
    pub D24: gpio::gpioi::PI0<gpio::Analog>,
    pub D25: gpio::gpioi::PI1<gpio::Analog>,
    pub D26: gpio::gpioi::PI2<gpio::Analog>,
    pub D27: gpio::gpioi::PI3<gpio::Analog>,
    pub D28: gpio::gpioi::PI6<gpio::Analog>,
    pub D29: gpio::gpioi::PI7<gpio::Analog>,
    pub D30: gpio::gpioi::PI9<gpio::Analog>,
    pub D31: gpio::gpioi::PI10<gpio::Analog>,
    pub NBL0: gpio::gpioe::PE0<gpio::Analog>,
    pub NBL1: gpio::gpioe::PE1<gpio::Analog>,
    pub NBL2: gpio::gpioi::PI4<gpio::Analog>,
    pub NBL3: gpio::gpioi::PI5<gpio::Analog>,
    pub SDCKE0: gpio::gpioh::PH2<gpio::Analog>,
    pub SDCLK: gpio::gpiog::PG8<gpio::Analog>,
    pub SDNCAS: gpio::gpiog::PG15<gpio::Analog>,
    pub SDNE0: gpio::gpioh::PH3<gpio::Analog>,
    pub SDRAS: gpio::gpiof::PF11<gpio::Analog>,
    pub SDNWE: gpio::gpioh::PH5<gpio::Analog>,
}

#[allow(non_snake_case)]
pub struct Pins {
    pub GPIO: Gpio,

    // Board peripherals.
    pub LED_USER: LedUserPin,
    pub CODEC: CodecPins,
    pub SAI: SaiPins,
    pub FLASH: FlashPins,
    pub SDRAM: SDRAMPins,
    pub USB2: USB2Pins,
}
