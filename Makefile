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
	cargo clippy --all -- -D warnings

.PHONY: check
check:
	cargo check --all

.PHONY: update
update:
	cargo update
