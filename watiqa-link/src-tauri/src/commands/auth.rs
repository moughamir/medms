use crate::security::totp::TotpSecret;
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
) -> Result<TotpSetupResponse, String> {
    let totp = TotpSecret::generate("Watiqa - Bouskoura", &username).map_err(|e| e.to_string())?;

    let backup_codes: Vec<String> = (0..8)
        .map(|_| format!("{:08}", rand::random::<u32>() % 100_000_000))
        .collect();

    let backup_codes_json = serde_json::to_string(&backup_codes).unwrap();
    let secret = totp.secret.clone();
    let qr_uri = totp.provisioning_uri();

    sqlx::query!(
        "UPDATE users SET totp_secret = ?, backup_codes = ?, totp_enabled = 1 WHERE username = ?",
        totp.secret,
        backup_codes_json,
        username
    )
    .execute(&*db)
    .await
    .map_err(|e| e.to_string())?;

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
) -> Result<bool, String> {
    let row = sqlx::query!(
        "SELECT totp_secret FROM users WHERE username = ? AND totp_enabled = 1",
        username
    )
    .fetch_one(&*db)
    .await
    .map_err(|e| e.to_string())?;

    let totp = TotpSecret {
        secret: row.totp_secret.unwrap_or_default(),
        issuer: "Watiqa".to_string(),
        account: username,
        digits: 6,
        period: 30,
    };

    totp.verify_code(&code, None).map_err(|e| e.to_string())
}
