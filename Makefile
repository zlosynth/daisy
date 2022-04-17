.PHONY: all
all: format clippy check

.PHONY: check-format
check-format:
	cargo fmt --all -- --check

.PHONY: format
format:
	cargo fmt --all

.PHONY: clippy
clippy:
	cargo clippy --all --examples -- -D warnings

.PHONY: check
check:
	cargo check --all

.PHONY: update
update:
	cargo update

.PHONY: flash
flash:
	cargo objcopy --release --example $(WHAT) -- -O binary target/program.bin
	dfu-util -a 0 -s 0x08000000:leave -D target/program.bin -d ,0483:df11
