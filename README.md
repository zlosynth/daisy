# Daisy

Rust `no_std`, `embedded_hal` board support package for the Electro-Smith Daisy
platform.

* [Documentation](https://zlosynth.com/daisy)
* [Crate (crates.io)](https://crates.io/crates/daisy)
* [Repository (github.com)](https://github.com/zlosynth/daisy)

# Supported boards

Currently this library supports following boards:

* [Daisy Seed](https://www.electro-smith.com/daisy/daisy) (codec AK4556), `seed`
* [Daisy Seed 1.1](https://www.electro-smith.com/daisy/daisy) (codec WM8731), `seed_1_1`
* [Daisy Patch SM](https://www.electro-smith.com/daisy/patch-sm) (codec PCM3060), `patch_sm`

Select the board by using its respective feature.

# API stability

I am still trying to figure out a good API for the project. Expect it to change.
To mitigate breakage of your code on library update, use macros defined under
`board.rs` to initialize resources whenever possible.

# Flashing an example

``` sh
make flash WHAT=blinky BOARD=seed_1_1
```

# Razor

* This library aims to abstract all peripherals that are available on Daisy.
* This library will **not** abstract anything that is not on Daisy, e.g. pots,
  SD cards, MIDI, ...

# HAL compatibility

This library is closely tied to [stm32h7xx-hal](https://github.com/stm32-rs/stm32h7xx-hal/).
Make sure to use compatible versions in your `Cargo.toml`.

| **Daisy**   | **HAL** |
|-------------|---------|
| `0.8`       | `0.14`  |
| `0.2`-`0.7` | `0.12`  |
| `0.1`       | `0.11`  |

# License

This library is distributed under the terms of the MIT license. See
[LICENSE](LICENSE) for details.

This project was forked from
[antoinevg/daisy_bsp](https://github.com/antoinevg/daisy_bsp).
Kudos to Antoine for making his library public.

# Changelog

Read the [CHANGELOG.md](CHANGELOG.md) to learn about changes introduced in each
release.

# Versioning

See [VERSIONING.md](VERSIONING.md) to find detailed information about versioning
of the project.
