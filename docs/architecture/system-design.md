# System Design

## Overview

**Watiqa-Link** is a modern document management system for Moroccan municipal administration. It is built as a hybrid desktop application using **Tauri**, combining the performance and security of **Rust** with the rich user interface capabilities of **React**.

## Technology Stack

### Frontend (User Interface)

- **Framework**: React 18 + TypeScript
- **Styling**: Tailwind CSS v4 (with RTL support)
- **Build Tool**: Vite
- **Icons**: Heroicons / custom SVG assets

### Backend (Core Logic)

- **Runtime**: Tauri v2
- **Language**: Rust (2021 Edition)
- **Database**: SQLite (via `sqlx`)
- **Authentication**: TOTP (Time-based One-Time Password) using `ring`

## Architecture

The application follows the Tauri multi-process architecture:

### 1. Core Process (Rust)

Responsible for system-level operations, security, and data persistence.

- **`src-tauri/src/main.rs`**: Application entry point.
- **`lib.rs`**: Core library logic.
- **`commands/`**: Exposed functions callable from the frontend (e.g., `setup_totp`, `verify_totp`).
- **`db.rs`**: Database connection and migration management.

### 2. WebView Process (Frontend)

Responsible for the user interface and presentation logic.

- **`src/`**: React application source.
- **`src/pages/`**: Application views (e.g., `TotpSetup.tsx`).
- **`src/stores/`**: Client-side state management (Zustand).

## Data Flow

1. **User Interaction**: User performs an action in the React UI.
2. **IPC Call**: Frontend calls a Rust command via `@tauri-apps/api/core`'s `invoke()`.
3. **Core Processing**: Rust backend processes the request (validates input, queries DB).
4. **Response**: Rust returns a serialized response (JSON) to the frontend.
5. **UI Update**: Frontend updates state and re-renders.

## Security Features

- **OTP Authentication**: 2FA-only access model (no static passwords).
- **Constant-Time Comparison**: Prevents timing attacks during code verification.
- **Secure Storage**: Sensitive secrets are handled within the Rust memory space; database is local SQLite.

## Cross-Platform Support

- **Primary Targets**: Linux, Windows.
- **Localization**: Native support for Arabic (RTL) and French (LTR).
