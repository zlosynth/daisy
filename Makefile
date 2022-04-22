.PHONY: all
all: format clippy

.PHONY: check-format
check-format:
	cargo fmt --all -- --check

.PHONY: format
format:
	cargo fmt --all

.PHONY: clippy
clippy:
	cargo clippy --all --examples --features seed -- -D warnings
	cargo clippy --all --examples --features seed_1_1 -- -D warnings
	cargo clippy --all --examples --features patch_sm -- -D warnings
	cargo clippy --all --examples --features seed,log-itm -- -D warnings

.PHONY: update
update:
	cargo update

.PHONY: flash
flash:
	cargo objcopy --release --example $(WHAT) --features $(BOARD) -- -O binary target/program.bin
	dfu-util -a 0 -s 0x08000000:leave -D target/program.bin -d ,0483:df11
