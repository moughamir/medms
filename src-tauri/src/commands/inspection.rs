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
    create_inspection_inner(data, &db).await
}

pub async fn create_inspection_inner(
    data: CreateInspection,
    pool: &SqlitePool,
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
    .fetch_one(pool)
    .await?;

    Ok(inspection)
}

#[tauri::command]
pub async fn list_recent_inspections(db: State<'_, SqlitePool>) -> AppResult<Vec<Inspection>> {
    list_recent_inspections_inner(&db).await
}

pub async fn list_recent_inspections_inner(pool: &SqlitePool) -> AppResult<Vec<Inspection>> {
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
    .fetch_all(pool)
    .await?;

    Ok(inspections)
}

#[tauri::command]
pub async fn list_inspections_by_commerce(
    commerce_id: String,
    db: State<'_, SqlitePool>,
) -> AppResult<Vec<Inspection>> {
    list_inspections_by_commerce_inner(commerce_id, &db).await
}

pub async fn list_inspections_by_commerce_inner(
    commerce_id: String,
    pool: &SqlitePool,
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
    .fetch_all(pool)
    .await?;

    Ok(inspections)
}

#[tauri::command]
pub async fn add_violation(
    data: CreateViolation,
    db: State<'_, SqlitePool>,
) -> AppResult<Violation> {
    add_violation_inner(data, &db).await
}

pub async fn add_violation_inner(
    data: CreateViolation,
    pool: &SqlitePool,
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
    .fetch_one(pool)
    .await?;

    Ok(violation)
}

#[tauri::command]
pub async fn list_violations(
    inspection_id: String,
    db: State<'_, SqlitePool>,
) -> AppResult<Vec<Violation>> {
    list_violations_inner(inspection_id, &db).await
}

pub async fn list_violations_inner(
    inspection_id: String,
    pool: &SqlitePool,
) -> AppResult<Vec<Violation>> {
    let violations = sqlx::query_as!(
        Violation,
        "SELECT * FROM violations WHERE inspection_id = ?",
        inspection_id
    )
    .fetch_all(pool)
    .await?;

    Ok(violations)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use std::path::Path;

    async fn setup_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create memory db");

        // We encounter a path issue with migrate! macro in tests depending on CWD.
        // sqlx::migrate! resolves relative to CARGO_MANIFEST_DIR (src-tauri).
        // Since we are running from project root usually, or src-tauri, let's verify.
        // Using ../migrations relative to this file's compile location? No, it's relative to Cargo.toml.
        // src-tauri/Cargo.toml is the manifest. migrations is src-tauri/migrations.
        // So "./migrations" should work if it picks up src-tauri/migrations.
        
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

    #[tokio::test]
    async fn test_create_inspection_flow() {
        let db = setup_db().await;

        // 1. Dependencies
        let user_id = Uuid::new_v4().to_string();
        sqlx::query!("INSERT INTO users (id, username, role) VALUES (?, ?, ?)", user_id, "inspector1", "inspector")
            .execute(&db).await.unwrap();

        let commerce_id = Uuid::new_v4().to_string();
        sqlx::query!("INSERT INTO commerces (id, name, status) VALUES (?, ?, ?)", commerce_id, "Test Shop", "active")
            .execute(&db).await.unwrap();

        // 2. Create Inspection
        let insp_data = CreateInspection {
            commerce_id: commerce_id.clone(),
            inspector_id: user_id.clone(),
            report_number: "2024/001".to_string(),
            summary: Some("Test".to_string()),
        };

        let inspection = create_inspection_inner(insp_data, &db).await.expect("Failed to create inspection");
        assert_eq!(inspection.report_number, "2024/001");
        assert_eq!(inspection.status, "draft");

        // 3. Add Violation
        let viol_data = CreateViolation {
            inspection_id: inspection.id.clone(),
            violation_code: "V001".to_string(),
            description: Some("Dirty floor".to_string()),
            severity: "medium".to_string(),
            measure_taken: Some("warning".to_string()),
        };

        let violation = add_violation_inner(viol_data, &db).await.expect("Failed to add violation");
        assert_eq!(violation.violation_code, "V001");

        // 4. List
        let recents = list_recent_inspections_inner(&db).await.unwrap();
        assert!(!recents.is_empty());
        assert_eq!(recents[0].id, inspection.id);
        
        let violations = list_violations_inner(inspection.id, &db).await.unwrap();
        assert_eq!(violations.len(), 1);
    }
}
