use crate::error::AppResult;
use crate::models::commerce::{Commerce, CreateCommerce};
use chrono::NaiveDateTime;
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn create_commerce(
    data: CreateCommerce,
    db: State<'_, SqlitePool>,
) -> AppResult<Commerce> {
    let id = Uuid::new_v4().to_string();

    let commerce = sqlx::query_as!(
        Commerce,
        r#"
        INSERT INTO commerces (
            id, name, address, city, commune, arrondissement, 
            owner_name, cin, phone, patente, activity_type, status
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'active')
        RETURNING 
            id, name, address, city, commune, arrondissement, 
            owner_name, cin, phone, patente, activity_type, status,
            created_at as "created_at: NaiveDateTime", 
            updated_at as "updated_at: NaiveDateTime"
        "#,
        id,
        data.name,
        data.address,
        data.city,
        data.commune,
        data.arrondissement,
        data.owner_name,
        data.cin,
        data.phone,
        data.patente,
        data.activity_type
    )
    .fetch_one(&*db)
    .await?;

    Ok(commerce)
}

#[tauri::command]
pub async fn list_commerces(
    filter: Option<String>,
    db: State<'_, SqlitePool>,
) -> AppResult<Vec<Commerce>> {
    let commerces = if let Some(query) = filter {
        let pattern = format!("%{}%", query);
        sqlx::query_as!(
            Commerce,
            r#"
            SELECT 
                id, name, address, city, commune, arrondissement, 
                owner_name, cin, phone, patente, activity_type, status,
                created_at as "created_at: NaiveDateTime", 
                updated_at as "updated_at: NaiveDateTime"
            FROM commerces 
            WHERE name LIKE ? OR owner_name LIKE ? OR cin LIKE ?
            ORDER BY created_at DESC
            "#,
            pattern,
            pattern,
            pattern
        )
        .fetch_all(&*db)
        .await?
    } else {
        sqlx::query_as!(
            Commerce,
            r#"
            SELECT 
                id, name, address, city, commune, arrondissement, 
                owner_name, cin, phone, patente, activity_type, status,
                created_at as "created_at: NaiveDateTime", 
                updated_at as "updated_at: NaiveDateTime"
            FROM commerces 
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&*db)
        .await?
    };

    Ok(commerces)
}

#[tauri::command]
pub async fn get_commerce(id: String, db: State<'_, SqlitePool>) -> AppResult<Commerce> {
    let commerce = sqlx::query_as!(
        Commerce,
        r#"
        SELECT 
            id, name, address, city, commune, arrondissement, 
            owner_name, cin, phone, patente, activity_type, status,
            created_at as "created_at: NaiveDateTime", 
            updated_at as "updated_at: NaiveDateTime"
        FROM commerces WHERE id = ?
        "#,
        id
    )
    .fetch_one(&*db)
    .await?;

    Ok(commerce)
}

#[tauri::command]
pub async fn update_commerce(
    id: String,
    data: CreateCommerce,
    db: State<'_, SqlitePool>,
) -> AppResult<Commerce> {
    let commerce = sqlx::query_as!(
        Commerce,
        r#"
        UPDATE commerces
        SET name = ?, address = ?, city = ?, commune = ?, arrondissement = ?,
            owner_name = ?, cin = ?, phone = ?, patente = ?, activity_type = ?,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        RETURNING 
            id, name, address, city, commune, arrondissement, 
            owner_name, cin, phone, patente, activity_type, status,
            created_at as "created_at: NaiveDateTime", 
            updated_at as "updated_at: NaiveDateTime"
        "#,
        data.name,
        data.address,
        data.city,
        data.commune,
        data.arrondissement,
        data.owner_name,
        data.cin,
        data.phone,
        data.patente,
        data.activity_type,
        id
    )
    .fetch_one(&*db)
    .await?;

    Ok(commerce)
}

#[tauri::command]
pub async fn delete_commerce(id: String, db: State<'_, SqlitePool>) -> AppResult<()> {
    sqlx::query!("DELETE FROM commerces WHERE id = ?", id)
        .execute(&*db)
        .await?;

    Ok(())
}
