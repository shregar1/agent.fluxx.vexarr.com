.PHONY: all
all: help

.PHONY: help
help:
	@echo "Fluxx Agent - Available Commands:"
	@echo ""
	@echo "  make install         - Full installation (Rust + build + test)"
	@echo "  make install-quick   - Quick install (skip Rust install, just build)"
	@echo "  make build           - Build debug version"
	@echo "  make build-release   - Build optimized release version"
	@echo "  make build-dist      - Build distribution (smallest binary, ~3MB)"
	@echo "  make test            - Run all tests"
	@echo "  make test-fast       - Run tests in release mode (faster)"
	@echo "  make run             - Run the agent (debug mode)"
	@echo "  make run-release     - Run the agent (release mode)"
	@echo "  make benchmark       - Run performance benchmark"
	@echo "  make check           - Check code without building"
	@echo "  make fmt             - Format code"
	@echo "  make clippy          - Run linter"
	@echo "  make clean           - Clean build artifacts"
	@echo "  make guide           - Show beginner's guide"
	@echo ""

.PHONY: install
install: build-dist
	@echo "Installing Fluxx binary to /usr/local/bin..."
	@sudo cp target/dist/fluxx /usr/local/bin/
	@sudo chmod +x /usr/local/bin/fluxx
	@echo "✅ Fluxx installed successfully!"
	@fluxx --version

.PHONY: install-quick
install-quick: build-release test-fast
	@echo ""
	@echo "✅ Quick install complete!"
	@echo "Run 'make benchmark' to test performance"

.PHONY: build
build:
	cargo build

.PHONY: build-release
build-release:
	cargo build --release

.PHONY: build-dist
build-dist:
	cargo build --profile dist
	@echo ""
	@echo "✅ Distribution binary built: target/dist/fluxx"
	@ls -lh target/dist/fluxx | awk '{print "   Size: " $$5}'

.PHONY: run
run:
	cargo run

.PHONY: run-release
run-release:
	cargo run --release

.PHONY: run-dist
run-dist:
	cargo run --profile dist

.PHONY: check
check:
	cargo check

.PHONY: test
test:
	cargo test

.PHONY: test-fast
test-fast:
	cargo test --release

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

.PHONY: benchmark
benchmark:
	cargo run --release --example ultra_optimized_load_test

.PHONY: guide
guide:
	@cat BEGINNERS_GUIDE.md

.PHONY: quick-test
quick-test:
	@echo "Running quick smoke test..."
	@cargo test --release test_event_new_http -- --nocapture
	@echo ""
	@echo "✅ Basic test passed! Run 'make test-fast' for full test suite"
