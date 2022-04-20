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
    pub PIN_B5: hal::gpio::gpioc::PC14<hal::gpio::Analog>, // GATE_OUT_1 Output Only
    pub PIN_B6: hal::gpio::gpioc::PC13<hal::gpio::Analog>, // GATE_OUT_2 Output Only
    pub PIN_B7: hal::gpio::gpiob::PB8<hal::gpio::Analog>, // I2C1_SCL, GPIO, UART4_RX, PWM (TIM4_CH3)
    pub PIN_B8: hal::gpio::gpiob::PB9<hal::gpio::Analog>, // I2C1_SDA, GPIO, UART4_TX, PWM (TIM4_CH4)
    pub PIN_B9: hal::gpio::gpiog::PG14<hal::gpio::Analog>, // GATE_IN_2, Input Only
    pub PIN_B10: hal::gpio::gpiog::PG13<hal::gpio::Analog>, // GATE_IN_1, Input Only
    pub PIN_C1: hal::gpio::gpioa::PA5<hal::gpio::Analog>, // CV_OUT_2, Output Only
    pub PIN_C2: hal::gpio::gpioa::PA7<hal::gpio::Analog>, // CV_4, Input Only
    pub PIN_C3: hal::gpio::gpioa::PA2<hal::gpio::Analog>, // CV_3, Input Only
    pub PIN_C4: hal::gpio::gpioa::PA6<hal::gpio::Analog>, // CV_2, Input Only
    pub PIN_C5: hal::gpio::gpioa::PA3<hal::gpio::Analog>, // CV_1, Input Only
    pub PIN_C6: hal::gpio::gpiob::PB1<hal::gpio::Analog>, // CV_5, Input Only
    pub PIN_C7: hal::gpio::gpioc::PC4<hal::gpio::Analog>, // CV_6, Input Only
    pub PIN_C8: hal::gpio::gpioc::PC0<hal::gpio::Analog>, // CV_7, Input Only
    pub PIN_C9: hal::gpio::gpioc::PC1<hal::gpio::Analog>, // CV_8, Input Only
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
    pub PIN_D10: hal::gpio::gpiog::PG3<hal::gpio::Analog>, // SPI2_SCK, GPIO
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
pub struct FMCPins {
    // https://github.com/electro-smith/libDaisy/blob/3dda55e9ed55a2f8b6bc4fa6aa2c7ae134c317ab/src/per/qspi.c#L695
    pub IO0: hal::gpio::gpiof::PF8<hal::gpio::Analog>, // (SI)
    pub IO1: hal::gpio::gpiof::PF9<hal::gpio::Analog>, // (SO)
    pub IO2: hal::gpio::gpiof::PF7<hal::gpio::Analog>,
    pub IO3: hal::gpio::gpiof::PF6<hal::gpio::Analog>,
    pub SCK: hal::gpio::gpiof::PF10<hal::gpio::Analog>,
    pub CS: hal::gpio::gpiog::PG6<hal::gpio::Analog>,
}

// - Pins ---------------------------------------------------------------------

#[allow(non_snake_case)]
pub struct Pins {
    pub GPIO: Gpio,

    // board peripherals
    pub LED_USER: LedUserPin,
    pub CODEC: CodecPins,
    pub SAI: SaiPins,
    pub FMC: FMCPins,
    pub SDRAM: (), // TODO
    pub USB2: USB2Pins,
}
