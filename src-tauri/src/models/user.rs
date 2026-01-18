use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(skip)]
    pub password_hash: Option<String>,
    #[serde(skip)]
    pub totp_secret: Option<String>,
    pub totp_enabled: bool,
    #[serde(skip)]
    pub backup_codes: Option<String>,
    pub role: String,
    pub created_at: Option<NaiveDateTime>,
    pub last_login: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub role: String,
}
