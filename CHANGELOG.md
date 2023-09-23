# Changelog

All notable changes to this project will be documented in this file. See
[VERSIONING.md](VERSIONING.md) for more information about versioning and
backwards compatibility.

## Unreleased

## 0.8.0

* **Breaking** Remove logging abstractions from the crate.
* **Breaking** The crate no longer re-exports `embedded_hal`.
* **Breaking** Remove `Led` trait.
* Bump STM32H7 HAL to version 0.14.
* Improve style and documentation of examples.
* Introduce RTIC example of audio processing.
* Introduce example of SD card interaction.

## 0.7.3

* Fix occasional stuckage of flash initialization.

## 0.7.2

* Set correct ADC/DAC format on Seed 1.1 and Patch SM to solve conversion issue.

## 0.7.1

* Fix audio input processing for Seed 1.1 and Patch SM.

## 0.7.0

* Breaking: Remove `size` parameter from SDRAM initialization.
* Allow utilization of dcache.
* Publish documentation under [https://zlosynth.com/daisy](https://zlosynth.com/daisy).

## 0.6.0

* Introduce an interface to access SDRAM.
* Breaking: Audio processing callback is now passed to
  `handle_interrupt_dma1_str1` instead of `spawn`.
* Breaking: Rename `FMCPins` to `FlashPins`.

## 0.5.0

* Add an example of OLED display configuration.
* Configure clock to support displays connected via SPI.

## 0.4.5

* Bump cortex-m-rt dependency to 0.7.

## 0.4.4

* Implicitly use peripheral clock for ADC.
* Fix ITM module.

## 0.4.3

* Fix mapping of pins B5, B6 and D10.

## 0.4.2

* Fix ADC pins C6-C9 mapping.

## 0.4.1

* Fix noise in Patch SM audio interface by switching to I2S.

## 0.4.0

* Fix macros so they respect the feature set on the depedency.
* Introduce a macro to instantiate flash abstraction.
* Update to stm32h7xx HAL 0.12.1.

## 0.3.0

* Introduce support for Daisy Patch SM, with PCM3060 codec.
* Move general purpose pins under a sub-structure. e.g. `SEED_PIN_1` to
  `GPIO.PIN_1`.
* Rename `seed_1_0` feature to `seed`.
* Remove unneeded `cty` dependency.

## 0.2.0

* Update to stm32h7xx HAL 0.12.
* Introduce support for Daisy Seed 1.1, with WM8731 codec.

## 0.1.0

* Initial fork from [antoinevg/daisy_bsp](https://github.com/antoinevg/daisy_bsp).
* Added flash memory access interface.
* Define razor guiding what goes into the library and what not.
* Introduce CI.
* Add SRAM to memory layout.
