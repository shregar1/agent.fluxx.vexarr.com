.PHONY: all
all: build

.PHONY: build
build:
	cargo build

.PHONY: run
run:
	cargo run

.PHONY: check
check:
	cargo check

.PHONY: test
test:
	cargo test

.PHONY: clippy
clippy:
	cargo clippy -- -D warnings

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: clean
clean:
	cargo clean

.PHONY: update
update:
	cargo update

.PHONY: build-release
build-release:
	cargo build --release
