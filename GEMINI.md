# Moroccan Administrative Documents Generator

## Project Overview

This project is a Rust application designed to automate the generation and management of administrative documents for municipal administrative police in a Moroccan context. It facilitates the creation of various official documents such as "Procès-Verbal" (inspection report), "Avertissement" (warning), "Mise en Demeure" (formal notice), "Décision de Fermeture" (closure decision), and "Amende" (fine).

The application interacts with the user via a command-line interface to collect necessary information regarding commercial establishments, inspections, and enforcement actions. This data is then used to populate `Word` documents (`.docx` format) and maintain a central `Excel` database (`police_administrative.xlsx`) for record-keeping. The system uses Arabic for document content and user prompts, catering to local administrative requirements.

**Key Features:**
*   **Document Generation:** Creates `.docx` documents with pre-defined templates for various administrative actions.
*   **Data Management:** Stores and retrieves information about businesses, inspections, and actions in an `Excel` spreadsheet.
*   **User-Friendly CLI:** Guides the user through data input for document creation.
*   **Localization:** Fully supports Arabic for document text and user interaction.

**Core Technologies:**
*   **Rust:** The primary programming language.
*   **docx-rs:** For generating `.docx` (Microsoft Word) documents.
*   **rust_xlsxwriter:** For creating and updating `Excel` (`.xlsx`) files.
*   **calamine:** For reading data from `Excel` (`.xlsx`) files.
*   **uuid:** For generating unique identifiers.
*   **chrono:** For handling dates and times.
*   **serde / serde_json:** For serialization/deserialization, although its direct usage isn't fully apparent in `main.rs`, it's a common Rust dependency for data handling.

## Building and Running

This project uses `Cargo`, the Rust package manager.

### Prerequisites

*   Rust toolchain (rustup recommended)

### Build the Project

To build the project, navigate to the root directory of the project and run:

```bash
cargo build --release
```

This will compile the project and create an executable in the `target/release/` directory.

### Run the Application

To run the application, execute the compiled binary:

```bash
./target/release/moroccan_docs
```

Or, using cargo:

```bash
cargo run --release
```

The application will then guide you through a series of prompts in the terminal to generate documents and manage records.

### Testing

*(No explicit test files were found in the initial analysis. A `TODO` for adding tests is appropriate if this were a production project.)*

To run any potential tests (if they were implemented), you would typically use:

```bash
cargo test
```

## Development Conventions

*   **Language:** Rust.
*   **Code Structure:** The `main.rs` file contains the bulk of the application logic, including data structures (enums and structs), `Excel` database management, and `Word` document generation.
*   **Naming Conventions:** Standard Rust naming conventions are followed (e.g., `CamelCase` for enums/structs, `snake_case` for functions/variables).
*   **Error Handling:** Uses `Result` for error propagation.

**Note:** The project heavily relies on the `Path` module for file system interactions and `io` for standard input/output.
