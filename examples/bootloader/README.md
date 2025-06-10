# Uploading firmware through Daisy Bootloader

The chip on board has quite limited flash capacity of 128 kB. To fit larger
firmware on Daisy, [the Daisy Bootloader exists](https://electro-smith.github.io/libDaisy/md_doc_2md_2__a7___getting-_started-_daisy-_bootloader.html).
Check the documentation to learn about the different bootloader modes and the
methods for uploading firmware through it.

In this directory, you'll find an example project that uses the bootloader.
The program is copied via DFU to the onboard 8 MB flash storage. When the board
starts, the bootloader will then copy the program to SDRAM for faster execution.

This example is made for Daisy Patch SM and assumes you're already familiar with
this crate. If you want to flash a different board, update the `Cargo.toml`
accordingly.

## Flashing the example

First, install the bootloader on the board. You can use <https://flash.daisy.audio/>,
go to the "Bootloader" tab, select version "v6.2", and flash it. Alternatively
you can use the [libDaisy](https://github.com/electro-smith/libDaisy/tree/master)
project and its `Makefile`.

Once the bootloader is installed, restart the module and press the BOOT button
within the first 2 seconds after startup. The onboard LED should start pulsing,
indicating the bootloader is active and waiting.

Now build the example firmware:

```sh
cargo objcopy --release -- -O binary target/program.bin
```

After building, use `dfu-util` to upload the program. Note the `-s` parameter,
which now points to the beginning of the writable onboard flash, not the
internal flash:

```sh
dfu-util -a 0 -s 0x90040000:leave -D target/program.bin -d ,0483:df11
```

The program should now be uploaded, and the onboard LED should be blinking.

## Attaching to logs

The bootloader doesn't allow flashing the program using
[`probe-rs`](https://probe.rs/), but you can still attach to logs using
[cargo-embed](https://probe.rs/docs/tools/cargo-embed/).

Build the program including INFO-level logs, flash it, and attach to
it using an ST-Link programmer:

```sh
DEFMT_LOG=info cargo objcopy --release -- -O binary target/program.bin
dfu-util -a 0 -s 0x90040000:leave -D target/program.bin -d ,0483:df11
DEFMT_LOG=info cargo-embed --release
```

You should now see the log output.

## What makes this work

### `Cargo.toml`

The `Cargo.toml` config is standard, except for the `set-vtor` feature flag
that must be enabled on `cortex-m-rt`.

### `memory.x`

This file is different from the standard `memory.x`. The main difference is the
region alias replacing `FLASH` of the default link file with `SRAM`.
The rest of the layout is also slightly adjusted to meet the bootloader’s
requirements.

### `Embed.toml`

This file is optional. It makes `cargo-embed` only read logs without trying to
flash the firmware.

## Caveats

### SRAM

Since the program is uploaded to SRAM and runs from there, your program can't
use that memory. Other memory regions are still available, but keep in mind
they are either smaller or slower.

### Reserved flash

Part of the onboard flash is used to store the firmware. It’s still possible
for the firmware to use this flash, but care must be taken when writing to it.
The first four 64 kB blocks are reserved, followed by the firmware itself.
When using the bootloader in SRAM mode (as in this example), the firmware can
take up to 480 kB, so the first 736 kB, or 184 sectors, are occupied and should
not be written to.

## Credits

Kudos to `eulerdisk` for [explaining](https://github.com/rust-embedded/cortex-m/issues/599#issuecomment-2956003568)
how to adjust the linker to work with the bootloader. Thanks to `Corvus Prudens`
and `mito3705` from [Daisy Discord](https://discord.com/channels/1037767234803740694/1039305128886403072)
who shared valuable input on using the Daisy Bootloader with Rust.
