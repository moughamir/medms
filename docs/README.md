# Watiqa-Link Documentation

This directory contains extended documentation for the Watiqa-Link document management system.

## Directory Structure

- `architecture/` - System design and modular structure
- `api/` - API reference (generate with `cd src-tauri && cargo doc --open`)
- `user-guide/` - Guides for end-users

## Quick Links

| Document | Description |
| ---------- | ------------- |
| [Architecture Overview](./architecture/overview.md) | System design and components |
| [Testing Guide](./testing.md) | How to run and write tests |
| [Release Notes](./release-notes.md) | Version history and changes |

## Generating Documentation

### Rust API Docs

```bash
cd src-tauri
cargo doc --open
```

### Build & Run

```bash
# Development
pnpm tauri dev

# Production
pnpm tauri build
```
