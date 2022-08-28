use stm32h7xx_hal as hal;

// - types --------------------------------------------------------------------

// https://github.com/electro-smith/DaisyWiki/wiki/2.-Daisy-Seed-Pinout
#[cfg(any(feature = "seed", feature = "seed_1_1"))]
#[allow(non_snake_case)]
pub struct Gpio {
    pub PIN_0: hal::gpio::gpiob::PB12<hal::gpio::Analog>, // STM PIN 01, USB OTG ID, I2C3 SCL
    pub PIN_1: hal::gpio::gpioc::PC11<hal::gpio::Analog>, // STM PIN 02, SD Data3, USART3 Rx
    pub PIN_2: hal::gpio::gpioc::PC10<hal::gpio::Analog>, // STM PIN 03, SD Data2, USART3 Tx
    pub PIN_3: hal::gpio::gpioc::PC9<hal::gpio::Analog>,  // STM PIN 04, SD Data1, I2C3 SDA
    pub PIN_4: hal::gpio::gpioc::PC8<hal::gpio::Analog>,  // STM PIN 05, SD Data0
    pub PIN_5: hal::gpio::gpiod::PD2<hal::gpio::Analog>,  // STM PIN 06, SD CMD, UART5 Rx
    pub PIN_6: hal::gpio::gpioc::PC12<hal::gpio::Analog>, // STM PIN 07, SD CLK, UART5 Tx
    pub PIN_7: hal::gpio::gpiog::PG10<hal::gpio::Analog>, // STM PIN 08, SPI1 CS
    pub PIN_8: hal::gpio::gpiog::PG11<hal::gpio::Analog>, // STM PIN 09, SPI1 SCK, SPDIFRX1
    pub PIN_9: hal::gpio::gpiob::PB4<hal::gpio::Alternate<0>>, // STM PIN 10, SPI1 MOSI
    pub PIN_10: hal::gpio::gpiob::PB5<hal::gpio::Analog>, // STM PIN 11, SPI1 MISO
    pub PIN_11: hal::gpio::gpiob::PB8<hal::gpio::Analog>, // STM PIN 12, I2C1 SCL, UART4 Rx
    pub PIN_12: hal::gpio::gpiob::PB9<hal::gpio::Analog>, // STM PIN 13, I2C1 SDA, UART4 Tx
    pub PIN_13: hal::gpio::gpiob::PB6<hal::gpio::Analog>, // STM PIN 14, USART1 Tx, I2C4 SCL
    pub PIN_14: hal::gpio::gpiob::PB7<hal::gpio::Analog>, // STM PIN 15, USART1 Rx, I2C4 SDA
    pub PIN_15: hal::gpio::gpioc::PC0<hal::gpio::Analog>, // STM PIN 22, ADC 0
    pub PIN_16: hal::gpio::gpioa::PA3<hal::gpio::Analog>, // STM PIN 23, ADC 1
    pub PIN_17: hal::gpio::gpiob::PB1<hal::gpio::Analog>, // STM PIN 24, ADC 2
    pub PIN_18: hal::gpio::gpioa::PA7<hal::gpio::Analog>, // STM PIN 25, ADC 3
    pub PIN_19: hal::gpio::gpioa::PA6<hal::gpio::Analog>, // STM PIN 26, ADC 4
    pub PIN_20: hal::gpio::gpioc::PC1<hal::gpio::Analog>, // STM PIN 27, ADC 5
    pub PIN_21: hal::gpio::gpioc::PC4<hal::gpio::Analog>, // STM PIN 28, ADC 6
    pub PIN_22: hal::gpio::gpioa::PA5<hal::gpio::Analog>, // STM PIN 29, DAC OUT 2, ADC 7
    pub PIN_23: hal::gpio::gpioa::PA4<hal::gpio::Analog>, // STM PIN 30, DAC OUT 1, ADC 8
    pub PIN_24: hal::gpio::gpioa::PA1<hal::gpio::Analog>, // STM PIN 31, SAI2 MCLK, ADC 9
    pub PIN_25: hal::gpio::gpioa::PA0<hal::gpio::Analog>, // STM PIN 32, SAI2 SD B, ADC 10
    pub PIN_26: hal::gpio::gpiod::PD11<hal::gpio::Analog>, // STM PIN 33, SAI2 SD A
    pub PIN_27: hal::gpio::gpiog::PG9<hal::gpio::Analog>, // STM PIN 34, SAI2 SD FS
    pub PIN_28: hal::gpio::gpioa::PA2<hal::gpio::Analog>, // STM PIN 35, SAI2 SCK, ADC 11
    pub PIN_29: hal::gpio::gpiob::PB14<hal::gpio::Analog>, // STM PIN 36, USB1 D-, USART1 Tx
    pub PIN_30: hal::gpio::gpiob::PB15<hal::gpio::Analog>, // STM PIN 37, USB1 D+, USART1 Rx
}

// ES_Patch_SM_datasheet_v1.0.3.pdf
// * 47K pullups are connected to this pin. This may have an effect on the behavior when used as UART
#[cfg(feature = "patch_sm")]
#[allow(non_snake_case)]
pub struct Gpio {
    pub PIN_A2: hal::gpio::gpioa::PA1<hal::gpio::Analog>, // ADC_9, GPIO, UART4_RX
    pub PIN_A3: hal::gpio::gpioa::PA0<hal::gpio::Analog>, // ADC_10, GPIO, UART4_TX
    pub PIN_A8: hal::gpio::gpiob::PB14<hal::gpio::Analog>, // USB_DM, GPIO, USART1_TX
    pub PIN_A9: hal::gpio::gpiob::PB15<hal::gpio::Analog>, // USB_DP, GPIO, USART1_RX
    pub PIN_B5: hal::gpio::gpioc::PC13<hal::gpio::Analog>, // GATE_OUT_1 Output Only
    pub PIN_B6: hal::gpio::gpioc::PC14<hal::gpio::Analog>, // GATE_OUT_2 Output Only
    pub PIN_B7: hal::gpio::gpiob::PB8<hal::gpio::Analog>, // I2C1_SCL, GPIO, UART4_RX, PWM (TIM4_CH3)
    pub PIN_B8: hal::gpio::gpiob::PB9<hal::gpio::Analog>, // I2C1_SDA, GPIO, UART4_TX, PWM (TIM4_CH4)
    pub PIN_B9: hal::gpio::gpiog::PG14<hal::gpio::Analog>, // GATE_IN_2, Input Only
    pub PIN_B10: hal::gpio::gpiog::PG13<hal::gpio::Analog>, // GATE_IN_1, Input Only
    pub PIN_C1: hal::gpio::gpioa::PA5<hal::gpio::Analog>, // CV_OUT_2, Output Only
    pub PIN_C2: hal::gpio::gpioa::PA7<hal::gpio::Analog>, // CV_4, Input Only
    pub PIN_C3: hal::gpio::gpioa::PA2<hal::gpio::Analog>, // CV_3, Input Only
    pub PIN_C4: hal::gpio::gpioa::PA6<hal::gpio::Analog>, // CV_2, Input Only
    pub PIN_C5: hal::gpio::gpioa::PA3<hal::gpio::Analog>, // CV_1, Input Only
    // FIXME: https://github.com/zlosynth/daisy/issues/1
    pub PIN_C6: hal::gpio::gpioc::PC1<hal::gpio::Analog>, // CV_8, Input Only
    pub PIN_C7: hal::gpio::gpioc::PC0<hal::gpio::Analog>, // CV_7, Input Only
    pub PIN_C8: hal::gpio::gpiob::PB1<hal::gpio::Analog>, // CV_5, Input Only
    pub PIN_C9: hal::gpio::gpioc::PC4<hal::gpio::Analog>, // CV_6, Input Only
    pub PIN_C10: hal::gpio::gpioa::PA4<hal::gpio::Analog>, // CV_OUT_1, Output Only
    pub PIN_D1: hal::gpio::gpiob::PB4<hal::gpio::Alternate<0>>, // SPI2_CS, GPIO,
    pub PIN_D2: hal::gpio::gpioc::PC11<hal::gpio::Analog>, // SDMMC1_D3, GPIO, USART3_RX*
    pub PIN_D3: hal::gpio::gpioc::PC10<hal::gpio::Analog>, // SDMMC1_D2, GPIO, USART3_TX*
    pub PIN_D4: hal::gpio::gpioc::PC9<hal::gpio::Analog>, // SDMMC1_D1, GPIO
    pub PIN_D5: hal::gpio::gpioc::PC8<hal::gpio::Analog>, // SDMMC1_D0, GPIO
    pub PIN_D6: hal::gpio::gpioc::PC12<hal::gpio::Analog>, // SDMMC1_CLK, GPIO, UART5_TX*
    pub PIN_D7: hal::gpio::gpiod::PD2<hal::gpio::Analog>, // SDMMC1_CMD, GPIO, UART5_RX*
    pub PIN_D8: hal::gpio::gpioc::PC2<hal::gpio::Analog>, // ADC_12, GPIO, SPI2_MISO
    pub PIN_D9: hal::gpio::gpioc::PC3<hal::gpio::Analog>, // ADC_11, GPIO, SPI2_MOSI
    pub PIN_D10: hal::gpio::gpiod::PD3<hal::gpio::Analog>, // SPI2_SCK, GPIO
}

pub type LedUserPin = hal::gpio::gpioc::PC7<hal::gpio::Analog>; // LED_USER

#[cfg(feature = "seed")]
#[allow(non_snake_case)]
pub struct CodecPins {
    pub PDN: hal::gpio::gpiob::PB11<hal::gpio::Analog>, // Codec Reset (AK4556)
}

#[cfg(feature = "seed_1_1")]
#[allow(non_snake_case)]
pub struct CodecPins {
    pub SCL: hal::gpio::gpioh::PH4<hal::gpio::Analog>, // I2C2 SCL (WM8731)
    pub SDA: hal::gpio::gpiob::PB11<hal::gpio::Analog>, // I2C2 SDA (WM8731)
}

#[cfg(feature = "patch_sm")]
#[allow(non_snake_case)]
pub struct CodecPins {
    pub SCL: hal::gpio::gpiob::PB10<hal::gpio::Analog>, // I2C2 SCL (PCM3060)
    pub SDA: hal::gpio::gpiob::PB11<hal::gpio::Analog>, // I2C2 SDA (PCM3060)
}

#[allow(non_snake_case)]
pub struct SaiPins {
    pub MCLK_A: hal::gpio::gpioe::PE2<hal::gpio::Analog>, // SAI1 MCLK_A
    pub SCK_A: hal::gpio::gpioe::PE5<hal::gpio::Analog>,  // SAI1 SCK_A
    pub FS_A: hal::gpio::gpioe::PE4<hal::gpio::Analog>,   // SAI1 FS_A
    pub SD_A: hal::gpio::gpioe::PE6<hal::gpio::Analog>,   // SAI1 SD_A
    pub SD_B: hal::gpio::gpioe::PE3<hal::gpio::Analog>,   // SAI1 SD_B
}

#[allow(non_snake_case)]
pub struct USB2Pins {
    pub DN: hal::gpio::gpioa::PA11<hal::gpio::Analog>, // USB2 D-
    pub DP: hal::gpio::gpioa::PA12<hal::gpio::Analog>, // USB2 D+
}

#[allow(non_snake_case)]
pub struct FlashPins {
    // https://github.com/electro-smith/libDaisy/blob/3dda55e9ed55a2f8b6bc4fa6aa2c7ae134c317ab/src/per/qspi.c#L695
    pub IO0: hal::gpio::gpiof::PF8<hal::gpio::Analog>, // (SI)
    pub IO1: hal::gpio::gpiof::PF9<hal::gpio::Analog>, // (SO)
    pub IO2: hal::gpio::gpiof::PF7<hal::gpio::Analog>,
    pub IO3: hal::gpio::gpiof::PF6<hal::gpio::Analog>,
    pub SCK: hal::gpio::gpiof::PF10<hal::gpio::Analog>,
    pub CS: hal::gpio::gpiog::PG6<hal::gpio::Analog>,
}

#[allow(non_snake_case)]
pub struct SDRAMPins {
    // https://github.com/x37v/libdaisy-rust/blob/0279b7b9d9a7cc867e7fce0d07287acf8ce3f72c/src/sdram.rs
    pub A0: hal::gpio::gpiof::PF0<hal::gpio::Analog>,
    pub A1: hal::gpio::gpiof::PF1<hal::gpio::Analog>,
    pub A2: hal::gpio::gpiof::PF2<hal::gpio::Analog>,
    pub A3: hal::gpio::gpiof::PF3<hal::gpio::Analog>,
    pub A4: hal::gpio::gpiof::PF4<hal::gpio::Analog>,
    pub A5: hal::gpio::gpiof::PF5<hal::gpio::Analog>,
    pub A6: hal::gpio::gpiof::PF12<hal::gpio::Analog>,
    pub A7: hal::gpio::gpiof::PF13<hal::gpio::Analog>,
    pub A8: hal::gpio::gpiof::PF14<hal::gpio::Analog>,
    pub A9: hal::gpio::gpiof::PF15<hal::gpio::Analog>,
    pub A10: hal::gpio::gpiog::PG0<hal::gpio::Analog>,
    pub A11: hal::gpio::gpiog::PG1<hal::gpio::Analog>,
    pub A12: hal::gpio::gpiog::PG2<hal::gpio::Analog>,
    pub BA0: hal::gpio::gpiog::PG4<hal::gpio::Analog>,
    pub BA1: hal::gpio::gpiog::PG5<hal::gpio::Analog>,
    pub D0: hal::gpio::gpiod::PD14<hal::gpio::Analog>,
    pub D1: hal::gpio::gpiod::PD15<hal::gpio::Analog>,
    pub D2: hal::gpio::gpiod::PD0<hal::gpio::Analog>,
    pub D3: hal::gpio::gpiod::PD1<hal::gpio::Analog>,
    pub D4: hal::gpio::gpioe::PE7<hal::gpio::Analog>,
    pub D5: hal::gpio::gpioe::PE8<hal::gpio::Analog>,
    pub D6: hal::gpio::gpioe::PE9<hal::gpio::Analog>,
    pub D7: hal::gpio::gpioe::PE10<hal::gpio::Analog>,
    pub D8: hal::gpio::gpioe::PE11<hal::gpio::Analog>,
    pub D9: hal::gpio::gpioe::PE12<hal::gpio::Analog>,
    pub D10: hal::gpio::gpioe::PE13<hal::gpio::Analog>,
    pub D11: hal::gpio::gpioe::PE14<hal::gpio::Analog>,
    pub D12: hal::gpio::gpioe::PE15<hal::gpio::Analog>,
    pub D13: hal::gpio::gpiod::PD8<hal::gpio::Analog>,
    pub D14: hal::gpio::gpiod::PD9<hal::gpio::Analog>,
    pub D15: hal::gpio::gpiod::PD10<hal::gpio::Analog>,
    pub D16: hal::gpio::gpioh::PH8<hal::gpio::Analog>,
    pub D17: hal::gpio::gpioh::PH9<hal::gpio::Analog>,
    pub D18: hal::gpio::gpioh::PH10<hal::gpio::Analog>,
    pub D19: hal::gpio::gpioh::PH11<hal::gpio::Analog>,
    pub D20: hal::gpio::gpioh::PH12<hal::gpio::Analog>,
    pub D21: hal::gpio::gpioh::PH13<hal::gpio::Analog>,
    pub D22: hal::gpio::gpioh::PH14<hal::gpio::Analog>,
    pub D23: hal::gpio::gpioh::PH15<hal::gpio::Analog>,
    pub D24: hal::gpio::gpioi::PI0<hal::gpio::Analog>,
    pub D25: hal::gpio::gpioi::PI1<hal::gpio::Analog>,
    pub D26: hal::gpio::gpioi::PI2<hal::gpio::Analog>,
    pub D27: hal::gpio::gpioi::PI3<hal::gpio::Analog>,
    pub D28: hal::gpio::gpioi::PI6<hal::gpio::Analog>,
    pub D29: hal::gpio::gpioi::PI7<hal::gpio::Analog>,
    pub D30: hal::gpio::gpioi::PI9<hal::gpio::Analog>,
    pub D31: hal::gpio::gpioi::PI10<hal::gpio::Analog>,
    pub NBL0: hal::gpio::gpioe::PE0<hal::gpio::Analog>,
    pub NBL1: hal::gpio::gpioe::PE1<hal::gpio::Analog>,
    pub NBL2: hal::gpio::gpioi::PI4<hal::gpio::Analog>,
    pub NBL3: hal::gpio::gpioi::PI5<hal::gpio::Analog>,
    pub SDCKE0: hal::gpio::gpioh::PH2<hal::gpio::Analog>,
    pub SDCLK: hal::gpio::gpiog::PG8<hal::gpio::Analog>,
    pub SDNCAS: hal::gpio::gpiog::PG15<hal::gpio::Analog>,
    pub SDNE0: hal::gpio::gpioh::PH3<hal::gpio::Analog>,
    pub SDRAS: hal::gpio::gpiof::PF11<hal::gpio::Analog>,
    pub SDNWE: hal::gpio::gpioh::PH5<hal::gpio::Analog>,
}

// - Pins ---------------------------------------------------------------------

#[allow(non_snake_case)]
pub struct Pins {
    pub GPIO: Gpio,

    // board peripherals
    pub LED_USER: LedUserPin,
    pub CODEC: CodecPins,
    pub SAI: SaiPins,
    pub FLASH: FlashPins,
    pub SDRAM: SDRAMPins,
    pub USB2: USB2Pins,
}
