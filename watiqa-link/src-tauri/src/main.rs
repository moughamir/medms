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
use tauri::Manager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_handle = app.handle();

            // Get app data directory
            let app_data_dir = app_handle
                .path()
                .app_data_dir()
                .context("Failed to get app data directory")?;

            // Ensure app data directory exists
            if !app_data_dir.exists() {
                fs::create_dir_all(&app_data_dir).context("Failed to create app data directory")?;
            }

            let db_path = app_data_dir.join("watiqa.db");
            let db_url = format!("sqlite:{}", db_path.to_str().context("Invalid DB path")?);

            // Initialize database pool in a separate task or blockingly if needed
            // For Tauri setup, we can use tokio::spawn or just block since it's startup
            let pool = tauri::async_runtime::block_on(async {
                let pool = SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect(&db_url)
                    .await
                    .context("Failed to connect to database")?;

                sqlx::migrate!("./migrations")
                    .run(&pool)
                    .await
                    .context("Failed to run migrations")?;

                Ok::<_, anyhow::Error>(pool)
            })?;

            // Initialize optimized document converter
            let converter =
                DocumentConverter::new(std::path::PathBuf::from("/usr/bin/libreoffice"), 4);

            app.manage(pool);
            app.manage(converter);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            setup_totp,
            verify_totp,
            generate_police_document
        ])
        .run(tauri::generate_context!())
        .context("error while running tauri application")?;

    Ok(())
}
