# Watiqa-Link

![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)
![Tauri](https://img.shields.io/badge/tauri-v2-blue.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

A Tauri-based desktop application for managing administrative documents in Moroccan municipal governments.

## Overview

Watiqa-Link is a bilingual (Arabic/French) document management system designed for municipal administrative police. It provides:

- **TOTP Authentication**: Secure two-factor authentication with QR code setup
- **Document Generation**: Automated creation of Arabic `.docx` documents from templates
- **Document Tracking**: Full state machine with audit trail (Registered → Dispatched → Annotated → Signed → Archived)
- **Offline-First**: All features work without network connectivity

### Supported Documents

- **Procès-Verbal** (Inspection report / محضر معاينة)
- **Avertissement** (Warning / إنذار)
- **Mise en Demeure** (Formal notice / إعذار)
- **Décision de Fermeture** (Closure decision / قرار الإغلاق)
- **Amende** (Fine / غرامة)

## Technology Stack

- **Backend**: Rust with Tauri v2
- **Frontend**: React + TypeScript + Tailwind CSS
- **Database**: SQLite with SQLx
- **Build**: Vite

## Getting Started

### Prerequisites

- Rust toolchain (v1.70+)
- Node.js (v18+)
- pnpm

### Installation

```bash
# Clone the repository
git clone https://github.com/your-repo/moroccan_docs.git
cd moroccan_docs

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev
```

### Build for Production

```bash
pnpm tauri build
```

## Make Commands

```bash
make install   # Install pnpm dependencies
make dev       # Run in development mode
make build     # Build for production
make test      # Run Rust tests
make lint      # Run lints (Rust + TypeScript)
make help      # Show all commands
```

## Documentation

Extended documentation is available in the [`docs/`](./docs) directory:

- [`docs/architecture/`](./docs/architecture/) - System design and module structure
- [`docs/api/`](./docs/api/) - API reference
- [`docs/user-guide/`](./docs/user-guide/) - End-user guides

## Contributing

Please see [`CONTRIBUTING.md`](./CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [`LICENSE`](./LICENSE) file for details.
