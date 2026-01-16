#!/bin/bash
#
# Watiqa-Link & Watiqa-Police: Tauri Project Scaffolder
#
# This script automates the setup of the Tauri-based MVP environment as
# defined in the technical architecture document. It scaffolds the project,
# sets up the required directory structure, and installs initial dependencies.
#
# Usage:
#   bash tauri_setup.sh
#

# --- Configuration ---
PROJECT_NAME="watiqa_link"
WINDOW_TITLE="Watiqa-Link"
AUTHOR="Your Name"

# Exit on any error
set -e

echo "--- Watiqa-Link Project Scaffolder ---"

# --- 1. Prerequisite Check ---
echo "[1/6] Checking for prerequisites (Node.js, Rust, Cargo)..."
command -v node >/dev/null 2>&1 || { echo >&2 "Node.js is required but not installed. Aborting."; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo >&2 "Rust/Cargo is required but not installed. Aborting."; exit 1; }
echo "✅ Prerequisites found."

# --- 2. Tauri Project Initialization ---
echo "[2/6] Scaffolding Tauri project with React + TypeScript..."

# Create project directory
if [ -d "$PROJECT_NAME" ]; then
    echo "⚠️  Directory '$PROJECT_NAME' already exists. Skipping project creation."
else
    # We use `create-tauri-app` for a robust React TS template
    npm create tauri-app@latest "$PROJECT_NAME" -- --template react-ts
    cd "$PROJECT_NAME"
    echo "✅ Project '$PROJECT_NAME' created."
fi

# Navigate into project directory if not already there
if [ "$(basename "$(pwd)")" != "$PROJECT_NAME" ]; then
    cd "$PROJECT_NAME"
fi


# --- 3. Install Frontend Dependencies ---
echo "[3/6] Installing frontend dependencies (React, Tailwind, Zustand, react-pdf)..."
npm install
npm install zustand react-pdf tailwindcss postcss autoprefixer
npx tailwindcss init -p
echo "✅ Frontend dependencies installed."

# Configure TailwindCSS
echo "module.exports = {
  content: [
    './index.html',
    './src/**/*.{js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};" > tailwind.config.js

echo "@tailwind base;
@tailwind components;
@tailwind utilities;" > src/index.css


# --- 4. Install Rust Backend Dependencies ---
echo "[4/6] Installing Rust backend dependencies (SQLite, Serde, Chrono, SQLCipher)..."
cargo add tauri@^1.0 --features "shell-open"
cargo add serde --features "derive"
cargo add serde_json
cargo add rusqlite --features "bundled,fts5"
cargo add sqlcipher --features "bundled"
cargo add chrono --features "serde"
cargo add ring
cargo add base64
cargo add machine_uid
cargo add zip
echo "✅ Rust dependencies installed."

# --- 5. Create Application Directory Structure ---
echo "[5/6] Creating application-specific directories..."
mkdir -p store/documents
mkdir -p assets/templates
mkdir -p assets/fonts
echo "✅ Directory structure created."

# --- 6. Configure Tauri ---
echo "[6/6] Configuring tauri.conf.json..."
# Use sed or another tool to update tauri.conf.json non-interactively
# For simplicity, we'll show a message here. A more robust script would parse and modify the JSON.
echo "⚠️  Action required: Please manually update 'src-tauri/tauri.conf.json' with the following:"
echo "   - Set 'productName' to '$PROJECT_NAME'"
echo "   - Set 'window.title' to '$WINDOW_TITLE'"
echo "   - Configure 'tauri.allowlist' for fs, shell, and http access as needed."
echo "   - Add bundled assets to 'tauri.bundle.resources'."

echo "---"
echo "✅ Setup complete!"
echo "Navigate to the '$PROJECT_NAME' directory and run 'npm run tauri dev' to start."
echo "---"