# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-01-15

### Added
- Graphical User Interface (GUI) implemented using `egui`/`eframe`.
- Support for Windows target builds.
- Modular architecture for better maintainability (separated models and generators).
- Logging system using `tracing`.

### Fixed
- Improved error handling with `anyhow` and `thiserror`.
- Better resource management for Excel file handles.

## [0.1.0] - 2025-12-01

### Added
- Initial release with Command-Line Interface (CLI).
- Basic document generation for Procès-Verbal, Avertissement, and Mise en Demeure.
- Excel database integration for record storage.
- Support for Arabic document generation.
