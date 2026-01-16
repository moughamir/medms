#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod document;
mod security;
mod templates;
mod workflow;

use commands::auth::{setup_totp, verify_totp};
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite:watiqa.db")
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(pool)
        .invoke_handler(tauri::generate_handler![setup_totp, verify_totp,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
