# Uploading firmware through Daisy Bootloader

The chip on board has quite limited flash capacity of 128 kB. To fit larger
firmware on Daisy, [the Daisy Bootloader exists](https://electro-smith.github.io/libDaisy/md_doc_2md_2__a7___getting-_started-_daisy-_bootloader.html).
Read the documentation to learn about different modes of the bootloader, and
methods of uploading firmware to it.

In this directory, you can find an example of a project that makes use of the
bootloader. The program is copied using DFU to the on-board 8 MB flash storage.
When the board starts, the bootloader would then copy the program to SDRAM for
faster execution.

This example is made for Daisy Patch SM and assumes prior knowledge of this
crate. If you want to flash to a different board, adjust the `Cargo.toml`
accordingly.

## Flashing the example

First, install the bootloader on the board. You can use <https://flash.daisy.audio/>,
go to the "Bootloader" tab, select version "v6.2", and flash it. Alternatively
you can use the [libDaisy](https://github.com/electro-smith/libDaisy/tree/master)
project and its `Makefile`.

Once the firmware is uploaded, restart the module and hit the BOOT button within
the first 2 seconds after start. The on-board LED should be now pulsing,
signaling that the bootloader is waiting.

Build the example firware:

```sh
cargo objcopy --release -- -O binary target/program.bin
```

After that, it is just a matter of using `dfu-util` to upload the program.
Note the `-s` parameter which now points at the beginning of writeable
on-board flash, instead of the internal flash:

```sh
dfu-util -a 0 -s 0x90040000:leave -D target/program.bin -d ,0483:df11
```

The program should be now uploaded and the on-board LED blinking.

## Attaching to logs

Although the bootloader does not allow the program to be flashed with
[`probe-rs`](https://probe.rs/), it is still possible to attach to the logs
using [`cargo-embed`].

Build the program including INFO-level defmt logs, flash it, and attach to
it using an ST-Link programmer:

```sh
DEFMT_LOG=info cargo objcopy --release -- -O binary target/program.bin
dfu-util -a 0 -s 0x90040000:leave -D target/program.bin -d ,0483:df11
DEFMT_LOG=info cargo-embed --release
```

You should now see the stream of logs.

## Notable files

### `Cargo.toml`

"set-vtor"

### `link.x`

### `memory.x`

### `Embed.toml`

Optional, for logs without probe-rs.

## Caveats

### SRAM

Can't use it.

### Reserved flash

Since the on-board flash is used to store the firmware, we have to be careful
where we write to with the firmware.

**If you use bootloader in the SDRAM mode**

SRAM, as the ultimate target of the
firmware, has capacity of 512 kB. The documentation explains that the last
32 kB of it are used for internal processes of the bootloader. This leaves
us with 480 kB available for the firmware.
The SRAM has 

To make this work, 

**If you use bootloader in FLASH mode**

## Credits

<https://github.com/rust-embedded/cortex-m/issues/599#issuecomment-2956003568>
