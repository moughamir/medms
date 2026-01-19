use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;

pub fn get_app_data_dir(app: &AppHandle) -> PathBuf {
    let exe_dir = app
        .path()
        .executable_dir()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()));
    let is_portable = exe_dir
        .as_ref()
        .map(|p| p.join("PORTABLE").exists())
        .unwrap_or(false);

    if is_portable {
        exe_dir.unwrap().join("data")
    } else {
        app.path()
            .app_data_dir()
            .unwrap_or_else(|_| PathBuf::from("data"))
    }
}

pub fn get_documents_dir(app: &AppHandle) -> PathBuf {
    let exe_dir = app
        .path()
        .executable_dir()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()));
    let is_portable = exe_dir
        .as_ref()
        .map(|p| p.join("PORTABLE").exists())
        .unwrap_or(false);

    if is_portable {
        exe_dir.unwrap().join("WatiqaLink")
    } else {
        dirs::document_dir()
            .unwrap_or_else(|| get_app_data_dir(app))
            .join("WatiqaLink")
    }
}
