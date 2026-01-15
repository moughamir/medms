# Contributing to Moroccan Administrative Documents Generator

Thank you for your interest in contributing to this project! We welcome contributions of all kinds, from bug reports and documentation improvements to new features and refactors.

## How to Contribute

1.  **Report Bugs**: If you find a bug, please open an issue describing the problem, steps to reproduce, and any relevant logs or error messages.
2.  **Suggest Enhancements**: We are always looking for ways to improve. Feel free to open an issue with your ideas.
3.  **Submit Pull Requests**:
    -   Fork the repository.
    -   Create a new branch for your changes (`git checkout -b feature/your-feature`).
    -   Make your changes and ensure the code follows our style guidelines.
    -   Run `cargo fmt` and `cargo clippy`.
    -   Commit your changes (`git commit -am 'Add some feature'`).
    -   Push to the branch (`git push origin feature/your-feature`).
    -   Create a new Pull Request.

## Development Setup

-   Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
-   Build the project: `cargo build`
-   Run tests (when available): `cargo test`

## Code Style

-   Follow standard Rust naming conventions (`snake_case` for functions/variables, `PascalCase` for types).
-   Use `cargo fmt` to format your code before submitting a PR.
-   Address any warnings from `cargo clippy`.

## Translations

Since this project primarily handles documents in Arabic, contributions to the Arabic text templates and UI translations are highly appreciated.

---
By contributing, you agree that your contributions will be licensed under the MIT License.
