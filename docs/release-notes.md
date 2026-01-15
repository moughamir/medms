# Release Notes

## Version 0.2.0 - UI Overhaul & Document Generation
*Release Date: 2026-01-15*

### New Features
- **Modern User Interface**: Completely redesigned GUI with a sidebar navigation, dashboard, and light/dark theme support.
- **Onboarding Wizard**: A new guided setup for first-time users to configure their location and identity.
- **Commerce Selector**: New smart widget to search and select businesses during inspections.
- **Document Generation**: Generate official Word documents (`.docx`) directly from the application.
- **Localization**: Full support for Arabic, French, and Bilingual modes.
- **Output Management**: Open generated files and folders directly from the application.

### Improvements
- **Arabic Rendering**: Fixed disconnected letters and RTL layout issues using `arabic_reshaper` and `unicode-bidi`.
- **Database**: Enhanced data preservation when writing to Excel.
- **Performance**: Optimized asset loading (fonts and images).

### Fixes
- Resolved stack overflow crash in Settings view.
- Fixed oversized logo on welcome screen.
- Fixed borrow checker issues in commerce selector.
