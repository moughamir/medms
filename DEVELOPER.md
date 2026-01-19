# Watiqa-Link Developer Guide

Welcome to the development guide for **Watiqa-Link**, a secure, offline-first document management system for Moroccan administrative entities.

## Prerequisites

### Cross-Platform
- **Rust**: [rustup.rs](https://rustup.rs/) (Stable channel)
- **Bun**: [bun.sh](https://bun.sh/) (Primary package manager)
- **LibreOffice**: Required for document conversion (PDF generation).

### OS-Specific Dependencies

#### Arch Linux
Install dependencies via `pacman` and `bun`:
```bash
sudo pacman -S --needed base-devel curl wget openssl libreoffice-fresh webkit2gtk-4.1
curl -fsSL https://bun.sh/install | bash
```

#### Windows Specifics (10/11)
- **WebView2**: Most recent Windows versions include this. 
- **C++ Build Tools**: Install "Desktop development with C++" workload via [Visual Studio Installer](https://visualstudio.microsoft.com/downloads/).

#### Windows 7 Legacy Support
- **WebView2 Runtime**: You **MUST** manually install the fixed version or Evergreen Bootstrapper of WebView2 Runtime from [Microsoft](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) for the app to launch on Windows 7.
- **DirectX**: Ensure latest DirectX updates are installed for proper rendering.

## Getting Started

1.  **Clone the Repository**:
    ```bash
    git clone <repo-url>
    cd moroccan_docs
    ```

2.  **Install Frontend Dependencies**:
    ```bash
    bun install
    ```

3.  **Setup Database**:
    The database will be automatically initialized and migrated on the first run.

4.  **Run in Development Mode**:
    ```bash
    bun tauri dev
    ```

## Document Conversion (LibreOffice)

The application automatically discovers LibreOffice in standard locations:
- **Linux**: `/usr/bin/libreoffice`, `/usr/bin/soffice`
- **Windows**: `C:\Program Files\LibreOffice\program\soffice.exe`

## Building for Production

### Windows (MSI / NSIS)
```bash
bun tauri build
```
The output will be in `src-tauri/target/release/bundle/`.

## Project Structure

- `src/`: React frontend (TypeScript, Tailwind CSS).
- `src-tauri/src/`: Rust backend.
  - `commands/`: Tauri invoke handlers.
  - `document/`: Template engines and conversion logic.
  - `excel/`: Excel ledger synchronization.
  - `models/`: Database and domain models.
- `assets/templates/`: Official Word `.docx` templates.

## Localization (Arabic RTL)

- The application uses `Noto Sans Arabic` as its primary font.
- Ensure all UI additions use the `.font-arabic` class and maintain the `direction: rtl` body global style.
