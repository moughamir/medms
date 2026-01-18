use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Commerce {
    pub id: String,
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub commune: Option<String>,
    pub arrondissement: Option<String>,
    pub owner_name: Option<String>,
    pub cin: Option<String>,
    pub phone: Option<String>,
    pub patente: Option<String>,
    pub activity_type: Option<String>,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommerce {
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub commune: Option<String>,
    pub arrondissement: Option<String>,
    pub owner_name: Option<String>,
    pub cin: Option<String>,
    pub phone: Option<String>,
    pub patente: Option<String>,
    pub activity_type: Option<String>,
}
