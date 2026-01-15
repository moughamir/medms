# System Design

## Overview
The **Moroccan Administrative Documents Generator** is a Rust-based desktop application designed for high performance, portability, and offline capability.

## Technology Stack
- **Language**: Rust (2021 Edition)
- **GUI Framework**: `egui` with `eframe` (Immediate Mode GUI).
- **Storage**: Excel (`.xlsx`) via `calamine` (read) and `rust_xlsxwriter` (write).
- **Document Generation**: `docx-rs` for data-driven Microsoft Word generation with embedded assets (Logos).
- **Localization**: Custom `Translator` struct with `arabic_reshaper` + `unicode-bidi` for correct text rendering.

## Architecture Patterns
The application follows a modular architecture:

### 1. Presentation Layer (`src/gui/`)
- **`app.rs`**: Main entry point and state manager.
- **`views/`**: Individual screens (Dashboard, Commerce, Inspection, Settings, Onboarding, Documents).
- **`widgets/`**: Reusable components (e.g., `CommerceSelector`).
- **`localization.rs`**: Handles string translation and shaping.
- **`text_utils.rs`**: Utilities for BiDi text processing.

### 2. Domain Layer (`src/models/`)
- Defines core data structures: `CommerceInfo`, `InspectionRecord`, `DashboardStats`.
- Enums for `ViolationType`, `DocumentType`.

### 3. Data Layer (`src/database/`)
- **`excel.rs`**: Encapsulates all Excel I/O operations.
- Implements a flat-file database pattern where the Excel file serves as the source of truth.

## Data Flow
1. **User Action**: User submits a form (e.g., New Commerce).
2. **State Update**: GUI state captures input.
3. **Database Write**: `ExcelDatabase` appends a row to the respective sheet in `police_administrative.xlsx`.
4. **Refetch**: Dashboard stats are recalculated by scanning the updated file.

## Cross-Platform Support
- Designed for **Linux** and **Windows**.
- Uses `winapi` for Windows-specific optimizations (if needed).
- Embeds fonts (`NotoSansArabic`) to ensure consistent rendering across OSs.
