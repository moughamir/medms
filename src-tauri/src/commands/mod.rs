pub mod auth;
pub mod commerce;
pub mod documents;
pub mod inspection;

use crate::error::AppResult;
use crate::excel::ExcelLedger;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn export_ledger(app: tauri::AppHandle, db: State<'_, SqlitePool>) -> AppResult<()> {
    ExcelLedger::export_all(&app, &db)
        .await
        .map_err(|e| e.into())
}
