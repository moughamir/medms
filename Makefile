# Watiqa-Link - Build Tools

.PHONY: all help install dev build build-linux build-windows clean test lint fmt check docs

# Default target
all: dev

#==============================================================================
# DEVELOPMENT
#==============================================================================

# Install dependencies
install:
	pnpm install

# Run in development mode
dev:
	pnpm tauri dev

# Build for production
build:
	pnpm tauri build

# Build for Linux
build-linux:
	pnpm tauri build --target x86_64-unknown-linux-gnu
	@echo "Linux bundle: src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/"

# Build for Windows (cross-compile)
build-windows:
	@echo "Checking Windows target..."
	rustup target add x86_64-pc-windows-gnu 2>/dev/null || true
	pnpm tauri build --target x86_64-pc-windows-gnu
	@echo "Windows bundle: src-tauri/target/x86_64-pc-windows-gnu/release/bundle/"

#==============================================================================
# QUALITY
#==============================================================================

# Clean build artifacts
clean:
	cd src-tauri && cargo clean
	rm -rf dist
	rm -rf node_modules/.vite

# Run Rust tests
test:
	cd src-tauri && cargo test

# Run lints (Rust + TypeScript)
lint:
	cd src-tauri && cargo clippy -- -D warnings
	cd src-tauri && cargo fmt --check
	pnpm exec tsc --noEmit

# Format code
fmt:
	cd src-tauri && cargo fmt

# Check for compilation errors
check:
	cd src-tauri && cargo check

# Generate Rust documentation
docs:
	cd src-tauri && cargo doc --no-deps --open

#==============================================================================
# SETUP
#==============================================================================

# Install Windows cross-compilation dependencies (Arch Linux)
setup-windows-cross:
	@echo "Installing MinGW-w64 for Windows cross-compilation..."
	sudo pacman -S mingw-w64-gcc
	rustup target add x86_64-pc-windows-gnu
	@echo "Done! You can now run 'make build-windows'"

# Help
help:
	@echo "Watiqa-Link - Build Commands"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "=== Development ==="
	@echo "  install       - Install pnpm dependencies"
	@echo "  dev           - Run in development mode"
	@echo "  build         - Build for production"
	@echo "  build-linux   - Build for Linux"
	@echo "  build-windows - Build for Windows (cross-compile)"
	@echo ""
	@echo "=== Quality ==="
	@echo "  clean         - Clean build artifacts"
	@echo "  test          - Run Rust tests"
	@echo "  lint          - Run lints (Rust + TypeScript)"
	@echo "  fmt           - Format Rust code"
	@echo "  check         - Check for compilation errors"
	@echo "  docs          - Generate Rust documentation"
	@echo ""
	@echo "=== Setup ==="
	@echo "  setup-windows-cross - Install Windows cross-compile deps"
	@echo "  help          - Show this help"
