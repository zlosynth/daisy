CARGO := cargo
DEFMT_LOG := info

.PHONY: all
all: format clippy

.PHONY: check-format
check-format:
	$(CARGO) fmt --all -- --check

.PHONY: format
format:
	$(CARGO) fmt --all

.PHONY: clippy
clippy:
	$(CARGO) clippy --all --examples --features seed -- -D warnings
	$(CARGO) clippy --all --examples --features seed_1_1 -- -D warnings
	$(CARGO) clippy --all --examples --features seed_1_1,sampling_rate_96khz -- -D warnings
	$(CARGO) clippy --all --examples --features seed_1_2 -- -D warnings
	$(CARGO) clippy --all --examples --features patch_sm -- -D warnings
	$(CARGO) clippy --all --examples --features seed -- -D warnings

.PHONY: update
update:
	$(CARGO) update

.PHONY: flash
flash:
	DEFMT_LOG=$(DEFMT_LOG) $(CARGO) run --release --example $(WHAT) --features $(BOARD),defmt

.PHONY: flash-dfu
flash-dfu:
	$(CARGO) objcopy --release --example $(WHAT) --features $(BOARD) -- -O binary target/program.bin
	dfu-util -a 0 -s 0x08000000:leave -D target/program.bin -d ,0483:df11
