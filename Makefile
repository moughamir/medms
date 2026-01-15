# Moroccan Administrative Documents Generator - Build Tools

.PHONY: all build build-release build-linux build-windows clean test lint run run-cli help

# Default target
all: build

# Development build
build:
	cargo build

# Release build
build-release:
	cargo build --release

# Linux release build
build-linux:
	cargo build --release --target x86_64-unknown-linux-gnu
	@echo "Linux binary: target/x86_64-unknown-linux-gnu/release/moroccan_docs"

# Windows cross-compilation (requires mingw-w64)
# Install: sudo apt install mingw-w64 (Debian/Ubuntu)
#          sudo pacman -S mingw-w64-gcc (Arch)
build-windows:
	@echo "Checking Windows target..."
	rustup target add x86_64-pc-windows-gnu 2>/dev/null || true
	cargo build --release --target x86_64-pc-windows-gnu
	@echo "Windows binary: target/x86_64-pc-windows-gnu/release/moroccan_docs.exe"

# Build all platforms
release: build-linux build-windows
	@mkdir -p dist/linux dist/windows
	@cp target/x86_64-unknown-linux-gnu/release/moroccan_docs dist/linux/ 2>/dev/null || \
		cp target/release/moroccan_docs dist/linux/
	@cp target/x86_64-pc-windows-gnu/release/moroccan_docs.exe dist/windows/ 2>/dev/null || \
		echo "Windows build not available"
	@echo "Release binaries in dist/"

# Clean build artifacts
clean:
	cargo clean
	rm -rf dist/

# Run tests
test:
	cargo test

# Run lints
lint:
	cargo clippy -- -D warnings
	cargo fmt --check

# Format code
fmt:
	cargo fmt

# Run the GUI application
run:
	cargo run --release

# Run in CLI mode
run-cli:
	cargo run --release -- --cli

# Check for compilation errors
check:
	cargo check

# Generate documentation
docs:
	cargo doc --no-deps --open

# Install Windows cross-compilation dependencies (Arch Linux)
setup-windows-cross:
	@echo "Installing MinGW-w64 for Windows cross-compilation..."
	sudo pacman -S mingw-w64-gcc
	rustup target add x86_64-pc-windows-gnu
	@echo "Done! You can now run 'make build-windows'"

# Help
help:
	@echo "Moroccan Administrative Documents Generator - Build Commands"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  build          - Development build"
	@echo "  build-release  - Release build"
	@echo "  build-linux    - Linux release build"
	@echo "  build-windows  - Windows cross-compilation"
	@echo "  release        - Build all platforms"
	@echo "  clean          - Clean build artifacts"
	@echo "  test           - Run tests"
	@echo "  lint           - Run clippy and format check"
	@echo "  fmt            - Format code"
	@echo "  run            - Run GUI application"
	@echo "  run-cli        - Run CLI mode"
	@echo "  check          - Check for errors"
	@echo "  docs           - Generate documentation"
	@echo "  setup-windows-cross - Install Windows cross-compile deps"
	@echo "  help           - Show this help"
