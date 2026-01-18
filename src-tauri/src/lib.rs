pub mod commands;
pub mod db;
pub mod document;
pub mod error;
pub mod models;
pub mod security;
pub mod templates;
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
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                let db = db::init_db(app.handle()).await.expect("failed to init db");
                app.manage(db);
            });
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
            commands::commerce::delete_commerce
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
