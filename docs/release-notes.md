# Release Notes

## Version 0.3.0 - The Tauri Migration

*Release Date: 2026-01-18*

### 🚀 Major Architecture Change

- **Moved to Tauri v2**: Replaced the legacy `egui` interface with a modern, responsive web-based UI using **React** and **Tailwind CSS**.
- **Backend Refactor**: Migrated core logic to a dedicated Tauri backend with SQLite storage.

### ✨ New Features

- **Secure Authentication**: Implemented TOTP (Time-based One-Time Password) flow with QR code setup.
- **Emergency Access**: Generated 8-digit backup codes for account recovery.
- **Modern UI**: Completely new "Premium" design aesthetic with glassmorphism and smooth animations.
- **RTL Support**: Native right-to-left layout support for Arabic users.

### 🛠️ Technical Improvements

- **Project Structure**: Consolidated `watiqa-link` into the project root for better maintainability.
- **Build System**: Unified `Makefile` for backend and frontend workflows.
- **Performance**: Optimized startup time and reduced binary size.

---

## Version 0.2.0 - UI Overhaul & Document Generation

*Release Date: 2026-01-15*

- Initial GUI implementation using `egui`.
- Added Document Generation with `docx-rs`.
- Introduced Arabic text reshaping support.
