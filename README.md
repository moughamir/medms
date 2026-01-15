# Moroccan Administrative Documents Generator

![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

A Rust-based application designed to automate the generation and management of administrative documents for municipal administrative police in Morocco.

## Overview

This system facilitates the creation of various official documents such as:
- **Procès-Verbal** (Inspection report)
- **Avertissement** (Warning)
- **Mise en Demeure** (Formal notice)
- **Décision de Fermeture** (Closure decision)
- **Amende** (Fine)

The application provides both a Command-Line Interface (CLI) and a Graphical User Interface (GUI) to collect information about commercial establishments, inspections, and enforcement actions. Data is stored in an Excel database (`police_administrative.xlsx`) and documents are generated in `.docx` format.

## Features

- **Document Generation**: Automated creation of Arabic `.docx` documents from templates.
- **Data Management**: centralized Excel-based record-keeping using `rust_xlsxwriter` and `calamine`.
- **Bilingual Interface**: Focus on Arabic for document content and administrative requirements.
- **Modular Architecture**: Clean separation of models, document generation logic, and UI.
- **Cross-platform**: Supports Linux and Windows targets.

## Technology Stack

- **Language**: Rust (Edition 2021)
- **UI**: `egui`/`eframe`
- **Excel**: `rust_xlsxwriter`, `calamine`
- **Word**: `docx-rs`
- **Utilities**: `chrono`, `uuid`, `serde`, `tracing`

## Getting Started

### Prerequisites

- Rust toolchain (v1.70+)
- Build tools (`gcc`, `pkg-config`, etc., depending on your OS)

### Installation

```bash
git clone https://github.com/your-repo/moroccan_docs.git
cd moroccan_docs
cargo build --release
```

### Usage

To run the application:
```bash
cargo run --release
```

## Documentation

Extended documentation can be found in the [`docs/`](./docs) directory.

## Contributing

Please see [`CONTRIBUTING.md`](./CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [`LICENSE`](./LICENSE) file for details.
