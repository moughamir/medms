pub mod commands;
pub mod db;
pub mod document;
pub mod error;
pub mod excel;
pub mod models;
pub mod security;
pub mod templates;
pub mod utils; // Added utils module
pub mod workflow;

use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                let db = db::init_db(app.handle()).await.expect("failed to init db");
                app.manage(db);
            });

            // Initialize optimized document converter
            let converter = document::converter::DocumentConverter::new(4);
            app.manage(converter);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::auth::setup_totp,
            commands::auth::verify_totp,
            commands::documents::generate_police_document,
            commands::commerce::create_commerce,
            commands::commerce::list_commerces,
            commands::commerce::get_commerce,
            commands::commerce::update_commerce,
            commands::commerce::delete_commerce,
            commands::inspection::create_inspection,
            commands::inspection::list_inspections_by_commerce,
            commands::inspection::list_recent_inspections,
            commands::inspection::add_violation,
            commands::inspection::list_violations,
            commands::export_ledger
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
