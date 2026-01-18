use crate::error::AppResult;
use crate::security::crypto::CryptEngine;
use crate::security::totp::TotpSecret;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::SqlitePool;
use tauri::State;

#[derive(Serialize)]
pub struct TotpSetupResponse {
    pub secret: String,
    pub qr_uri: String,
    pub backup_codes: Vec<String>,
}

#[tauri::command]
pub async fn setup_totp(
    username: String,
    db: State<'_, SqlitePool>,
) -> AppResult<TotpSetupResponse> {
    let totp = TotpSecret::generate("Watiqa - Bouskoura", &username)?;
    let crypto = CryptEngine::new()?;

    let backup_codes: Vec<String> = (0..8)
        .map(|_| format!("{:08}", rand::random::<u32>() % 100_000_000))
        .collect();

    let backup_codes_json = serde_json::to_string(&backup_codes).unwrap();
    let secret = totp.secret.clone();
    let qr_uri = totp.provisioning_uri();

    // Encrypt the secret before storing
    let encrypted_secret = crypto.encrypt(&totp.secret)?;

    sqlx::query!(
        "UPDATE users SET totp_secret = ?, backup_codes = ?, totp_enabled = 1 WHERE username = ?",
        encrypted_secret,
        backup_codes_json,
        username
    )
    .execute(&*db)
    .await?;

    Ok(TotpSetupResponse {
        secret,
        qr_uri,
        backup_codes,
    })
}

#[tauri::command]
pub async fn verify_totp(
    username: String,
    code: String,
    db: State<'_, SqlitePool>,
) -> AppResult<Option<crate::models::user::User>> {
    let crypto = CryptEngine::new()?;

    let user = sqlx::query_as!(
        crate::models::user::User,
        r#"SELECT id, username, password_hash, totp_secret, totp_enabled as "totp_enabled: bool", backup_codes, role, created_at as "created_at: NaiveDateTime", last_login as "last_login: NaiveDateTime" FROM users WHERE username = ? AND totp_enabled = 1"#,
        username
    )
    .fetch_optional(&*db)
    .await?;

    if let Some(user) = user {
        let encrypted_secret = user.totp_secret.as_ref().cloned().unwrap_or_default();
        let decrypted_secret = crypto.decrypt(&encrypted_secret)?;

        let totp = TotpSecret {
            secret: decrypted_secret,
            issuer: "Watiqa".to_string(),
            account: username,
            digits: 6,
            period: 30,
        };

        if totp.verify_code(&code, None)? {
            // Update last login
            sqlx::query!(
                "UPDATE users SET last_login = CURRENT_TIMESTAMP WHERE id = ?",
                user.id
            )
            .execute(&*db)
            .await?;

            return Ok(Some(user));
        }
    }

    Ok(None)
}
