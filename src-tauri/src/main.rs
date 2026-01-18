#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod document;
mod error;
mod security;
mod templates;
mod workflow;

use anyhow::Context;
use commands::auth::{setup_totp, verify_totp};
use commands::documents::generate_police_document;
use document::converter::DocumentConverter;
use sqlx::sqlite::SqlitePoolOptions;
use std::fs;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    // 1. Initialize Runtime for async tasks during setup
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("Failed to build tokio runtime")?;

    // 2. Resolve paths for the database
    // We use the identifier from tauri.conf.json to match app_data_dir
    let app_data_dir = dirs::data_dir()
        .context("Failed to get system data directory")?
        .join("ma.bouskoura.watiqa");

    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir).context("Failed to create app data directory")?;
    }

    let db_path = app_data_dir.join("watiqa.db");
    let db_url = format!("sqlite:{}", db_path.to_str().context("Invalid DB path")?);

    // 3. Initialize database pool
    let pool = rt.block_on(async {
        let options =
            sqlx::sqlite::SqliteConnectOptions::from_str(&db_url)?.create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .context("Failed to connect to database")?;

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .context("Failed to run migrations")?;

        Ok::<_, anyhow::Error>(pool)
    })?;

    // 4. Initialize optimized document converter
    let converter = DocumentConverter::new(std::path::PathBuf::from("/usr/bin/libreoffice"), 4);

    // 5. Run the Tauri application
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .manage(pool)
        .manage(converter)
        .invoke_handler(tauri::generate_handler![
            setup_totp,
            verify_totp,
            generate_police_document
        ])
        .run(tauri::generate_context!())
        .context("error while running tauri application")?;

    Ok(())
}
