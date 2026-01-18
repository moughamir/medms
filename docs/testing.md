# Watiqa-Link Testing Documentation

## Testing Philosophy

Reliability and security are paramount. We use a layered testing strategy covering both the Rust backend and React frontend.

## 1. Backend Testing (Rust)

We aim for **85% code coverage** on security-critical modules.

### Running Tests

```bash
cd src-tauri
cargo test
```

### Key Modules

- **Security (`security/`)**: TOTP generation (RFC 6238), window validation, backup codes.
- **Workflow (`workflow/`)**: State machine transitions.
- **Commands**: Integration tests for Tauri commands.

## 2. Frontend Testing (React)

Unit tests for UI components and hooks.

### Running Tests

```bash
pnpm test
```

## Security Audits

We use `cargo audit` to check for vulnerabilities in dependencies.

```bash
cd src-tauri
cargo audit
```

## Continuous Integration

All PRs must pass `cargo test` and `pnpm test` before merge.
