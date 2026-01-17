# Watiqa-Link Testing Documentation

This document describes the testing strategy and procedures for the Watiqa-Link project.

## Testing Philosophy

We follow a layered testing approach to ensure the reliability and security of the application:

1. **Unit Tests**: Focus on individual modules (Security, Workflow, Documents).
2. **Integration Tests**: Verify the interaction between different components and the Tauri backend.
3. **Security Tests**: Specific tests for sensitive operations like TOTP and Encryption.

## Running Tests

To run all tests in the project, use the following command:

```bash
cargo test
```

### Core Module Coverage Requirement

Our goal is to maintain at least **85% code coverage** for the following core modules:

- `src/security/totp.rs`
- `src/security/crypto.rs`
- `src/workflow/fsm.rs`

## Test Breakdown

### 1. Security Tests (`src/security/`)

- **TOTP**: Verification of code generation, window-based validation, and constant-time comparison.
- **Crypto**: Verification of AES-256-GCM encryption and decryption.

### 2. Workflow Tests (`src/workflow/`)

- **FSM**: Exhaustive checking of legal and illegal state transitions.

### 3. Document Tests (`src/document/`)

- **Converter**: Verification of the conversion logic, including retry mechanisms and timeout handling.

## Code Coverage

To generate a coverage report, we recommend using `cargo-tarpaulin`:

```bash
cargo tarpaulin --out Html
```

## Continuous Integration

Every pull request must pass all automated tests before being merged. Security-sensitive changes require additional peer review.
