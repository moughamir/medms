# API Overview

Since this is a primarily a GUI application, the "API" refers to the internal Rust modules and public interfaces used by the `gui` crate.

## Core Modules

### `database`
- **`ExcelDatabase::new(path: &str)`**: Initializes the connection.
- **`save_commerce(&self, info: &CommerceInfo)`**: Writes a commerce record.
- **`save_inspection(&self, record: &InspectionRecord)`**: Writes an inspection record.
- **`get_dashboard_stats(&self)`**: Aggregates data for the dashboard.

### `gui::localization`
- **`tr(key: &str, lang: &Language)`**: Returns a localized string.
- **`tr_dynamic(arabic: &str, french: &str, lang: &Language)`**: Helper for dynamic content.

### `gui::text_utils`
- **`reshape(text: &str)`**: Applies Arabic shaping and BiDi reordering.

### `generator`
- **`DocumentGenerator::new(...)`**: Creates a new generator instance with metadata.
- **`create_document(&self, path: &str)`**: Generates and saves the `.docx` file.

## Generating Docs
Run the following command to generate standard Rust documentation for all modules:

```bash
cargo doc --open --no-deps
```
