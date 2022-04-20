# Changelog

All notable changes to this project will be documented in this file. See
[VERSIONING.md](VERSIONING.md) for more information about versioning and
backwards compatibility.

## Unreleased

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
