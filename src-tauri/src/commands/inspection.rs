use crate::error::AppResult;
use crate::models::inspection::{CreateInspection, CreateViolation, Inspection, Violation};
use chrono::NaiveDateTime;
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn create_inspection(
    data: CreateInspection,
    db: State<'_, SqlitePool>,
) -> AppResult<Inspection> {
    let id = Uuid::new_v4().to_string();

    let inspection = sqlx::query_as!(
        Inspection,
        r#"
        INSERT INTO inspections (
            id, commerce_id, inspector_id, report_number, summary, status
        )
        VALUES (?, ?, ?, ?, ?, 'draft')
        RETURNING 
            id, commerce_id, inspector_id, report_number, summary, status,
            inspection_date as "inspection_date: NaiveDateTime"
        "#,
        id,
        data.commerce_id,
        data.inspector_id,
        data.report_number,
        data.summary
    )
    .fetch_one(&*db)
    .await?;

    Ok(inspection)
}

#[tauri::command]
pub async fn list_recent_inspections(db: State<'_, SqlitePool>) -> AppResult<Vec<Inspection>> {
    let inspections = sqlx::query_as!(
        Inspection,
        r#"
        SELECT 
            id, commerce_id, inspector_id, report_number, summary, status,
            inspection_date as "inspection_date: NaiveDateTime"
        FROM inspections 
        ORDER BY inspection_date DESC
        LIMIT 50
        "#
    )
    .fetch_all(&*db)
    .await?;

    Ok(inspections)
}

#[tauri::command]
pub async fn list_inspections_by_commerce(
    commerce_id: String,
    db: State<'_, SqlitePool>,
) -> AppResult<Vec<Inspection>> {
    let inspections = sqlx::query_as!(
        Inspection,
        r#"
        SELECT 
            id, commerce_id, inspector_id, report_number, summary, status,
            inspection_date as "inspection_date: NaiveDateTime"
        FROM inspections 
        WHERE commerce_id = ?
        ORDER BY inspection_date DESC
        "#,
        commerce_id
    )
    .fetch_all(&*db)
    .await?;

    Ok(inspections)
}

#[tauri::command]
pub async fn add_violation(
    data: CreateViolation,
    db: State<'_, SqlitePool>,
) -> AppResult<Violation> {
    let id = Uuid::new_v4().to_string();

    let violation = sqlx::query_as!(
        Violation,
        r#"
        INSERT INTO violations (
            id, inspection_id, violation_code, description, severity, measure_taken
        )
        VALUES (?, ?, ?, ?, ?, ?)
        RETURNING *
        "#,
        id,
        data.inspection_id,
        data.violation_code,
        data.description,
        data.severity,
        data.measure_taken
    )
    .fetch_one(&*db)
    .await?;

    Ok(violation)
}

#[tauri::command]
pub async fn list_violations(
    inspection_id: String,
    db: State<'_, SqlitePool>,
) -> AppResult<Vec<Violation>> {
    let violations = sqlx::query_as!(
        Violation,
        "SELECT * FROM violations WHERE inspection_id = ?",
        inspection_id
    )
    .fetch_all(&*db)
    .await?;

    Ok(violations)
}
