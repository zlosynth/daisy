# Daisy

Rust `no_std`, `embedded_hal` board support package for the Electro-Smith Daisy
Seed and Daisy Patch SM.

This project was forked from
[antoinevg/daisy_bsp](https://github.com/antoinevg/daisy_bsp).

# Flash an example

``` sh
make flash WHAT=blinky
```

# Razor

* This library aims to abstract all peripherals that are available on Daisy.
* This library will **not** abstract anything that is not on Daisy, e.g. pots,
  SD cards, MIDI, ...

# License

This library is distributed under the terms of the MIT license. See
[LICENSE](LICENSE) for details.

# Changelog

Read the [CHANGELOG.md](CHANGELOG.md) to learn about changes introduced in each
release.

# Versioning

See [VERSIONING.md](VERSIONING.md) to find detailed information about versioning
of the project.
