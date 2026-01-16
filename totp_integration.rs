// src-tauri/src/security/totp.rs
//! TOTP Implementation for Watiqa-Link

use anyhow::{Context, Result};
use base32::Alphabet;
use ring::hmac;
use serde::{Deserialize, Serialize};

const DEFAULT_DIGITS: u32 = 6;
const DEFAULT_PERIOD: u64 = 30;
const VALIDATION_WINDOW: i64 = 1; // ±30s tolerance

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpSecret {
    pub secret: String,        // Base32 encoded
    pub issuer: String,        // "Watiqa - Commune Bouskoura"
    pub account: String,       // username or email
    pub algorithm: Algorithm,
    pub digits: u32,
    pub period: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Algorithm {
    SHA1,
    SHA256,
    SHA512,
}

impl TotpSecret {
    pub fn generate(issuer: &str, account: &str) -> Result<Self> {
        let secret = Self::generate_secret(20)?; // 160-bit secret
        
        Ok(Self {
            secret,
            issuer: issuer.to_string(),
            account: account.to_string(),
            algorithm: Algorithm::SHA1,
            digits: DEFAULT_DIGITS,
            period: DEFAULT_PERIOD,
        })
    }

    fn generate_secret(length: usize) -> Result<String> {
        use ring::rand::{SecureRandom, SystemRandom};
        
        let rng = SystemRandom::new();
        let mut bytes = vec![0u8; length];
        rng.fill(&mut bytes)
            .map_err(|_| anyhow::anyhow!("Failed to generate random bytes"))?;
        
        Ok(base32::encode(Alphabet::RFC4648 { padding: false }, &bytes))
    }

    /// Generate provisioning URI for QR code
    pub fn provisioning_uri(&self) -> String {
        format!(
            "otpauth://totp/{}:{}?secret={}&issuer={}&algorithm={}&digits={}&period={}",
            urlencoding::encode(&self.issuer),
            urlencoding::encode(&self.account),
            self.secret,
            urlencoding::encode(&self.issuer),
            match self.algorithm {
                Algorithm::SHA1 => "SHA1",
                Algorithm::SHA256 => "SHA256",
                Algorithm::SHA512 => "SHA512",
            },
            self.digits,
            self.period
        )
    }

    /// Generate current TOTP code
    pub fn generate_code(&self, time: Option<u64>) -> Result<String> {
        let timestamp = time.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });

        let counter = timestamp / self.period;
        self.generate_hotp(counter)
    }

    /// Validate TOTP code with window tolerance
    pub fn verify_code(&self, code: &str, time: Option<u64>) -> Result<bool> {
        let timestamp = time.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });

        let current_counter = (timestamp / self.period) as i64;

        // Check window: current ± VALIDATION_WINDOW
        for offset in -VALIDATION_WINDOW..=VALIDATION_WINDOW {
            let counter = (current_counter + offset) as u64;
            let expected = self.generate_hotp(counter)?;
            
            if constant_time_eq(code.as_bytes(), expected.as_bytes()) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn generate_hotp(&self, counter: u64) -> Result<String> {
        let secret_bytes = base32::decode(Alphabet::RFC4648 { padding: false }, &self.secret)
            .context("Invalid Base32 secret")?;

        // Convert counter to big-endian 8-byte array
        let counter_bytes = counter.to_be_bytes();

        // Compute HMAC
        let key = match self.algorithm {
            Algorithm::SHA1 => hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, &secret_bytes),
            Algorithm::SHA256 => hmac::Key::new(hmac::HMAC_SHA256, &secret_bytes),
            Algorithm::SHA512 => hmac::Key::new(hmac::HMAC_SHA512, &secret_bytes),
        };

        let tag = hmac::sign(&key, &counter_bytes);
        let hmac_result = tag.as_ref();

        // Dynamic truncation
        let offset = (hmac_result[hmac_result.len() - 1] & 0x0F) as usize;
        let binary = u32::from_be_bytes([
            hmac_result[offset] & 0x7F,
            hmac_result[offset + 1],
            hmac_result[offset + 2],
            hmac_result[offset + 3],
        ]);

        let otp = binary % 10_u32.pow(self.digits);
        Ok(format!("{:0width$}", otp, width = self.digits as usize))
    }
}

/// Constant-time string comparison (prevent timing attacks)
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    a.iter()
        .zip(b.iter())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y))
        == 0
}

// src-tauri/src/security/qr.rs
//! QR Code generation for TOTP provisioning

use qrcode::{QrCode, render::svg};
use anyhow::Result;

pub fn generate_qr_svg(uri: &str) -> Result<String> {
    let code = QrCode::new(uri.as_bytes())?;
    let svg = code
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#FFFFFF"))
        .build();
    
    Ok(svg)
}

pub fn generate_qr_png(uri: &str, size: u32) -> Result<Vec<u8>> {
    use qrcode::render::png;
    
    let code = QrCode::new(uri.as_bytes())?;
    let png_data = code
        .render::<png::Renderer>()
        .min_dimensions(size, size)
        .build();
    
    Ok(png_data)
}

// src-tauri/src/commands/auth.rs
//! Authentication commands with TOTP

use crate::security::totp::{TotpSecret, Algorithm};
use crate::security::qr;
use tauri::State;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Serialize)]
pub struct TotpSetupResponse {
    pub secret: String,
    pub qr_code_svg: String,
    pub provisioning_uri: String,
    pub backup_codes: Vec<String>,
}

#[tauri::command]
pub async fn setup_totp(
    username: String,
    db: State<'_, SqlitePool>,
) -> Result<TotpSetupResponse, String> {
    // Generate TOTP secret
    let totp = TotpSecret::generate(
        "Watiqa - Commune Bouskoura",
        &username,
    ).map_err(|e| e.to_string())?;

    let provisioning_uri = totp.provisioning_uri();
    let qr_code_svg = qr::generate_qr_svg(&provisioning_uri)
        .map_err(|e| e.to_string())?;

    // Generate backup codes
    let backup_codes = generate_backup_codes(8)?;

    // Store in database (encrypted)
    sqlx::query!(
        "UPDATE users SET totp_secret = ?, backup_codes = ?, totp_enabled = 1 
         WHERE username = ?",
        totp.secret,
        serde_json::to_string(&backup_codes).unwrap(),
        username
    )
    .execute(&*db)
    .await
    .map_err(|e| e.to_string())?;

    Ok(TotpSetupResponse {
        secret: totp.secret,
        qr_code_svg,
        provisioning_uri,
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
        "SELECT totp_secret, totp_enabled FROM users WHERE username = ?",
        username
    )
    .fetch_one(&*db)
    .await
    .map_err(|e| e.to_string())?;

    if !row.totp_enabled.unwrap_or(0) == 1 {
        return Err("TOTP not enabled".to_string());
    }

    let totp = TotpSecret {
        secret: row.totp_secret.unwrap_or_default(),
        issuer: "Watiqa - Commune Bouskoura".to_string(),
        account: username.clone(),
        algorithm: Algorithm::SHA1,
        digits: 6,
        period: 30,
    };

    let valid = totp.verify_code(&code, None)
        .map_err(|e| e.to_string())?;

    if valid {
        // Update last_login timestamp
        sqlx::query!(
            "UPDATE users SET last_login = CURRENT_TIMESTAMP WHERE username = ?",
            username
        )
        .execute(&*db)
        .await
        .map_err(|e| e.to_string())?;
    }

    Ok(valid)
}

#[tauri::command]
pub async fn verify_backup_code(
    username: String,
    code: String,
    db: State<'_, SqlitePool>,
) -> Result<bool, String> {
    let row = sqlx::query!(
        "SELECT backup_codes FROM users WHERE username = ?",
        username
    )
    .fetch_one(&*db)
    .await
    .map_err(|e| e.to_string())?;

    let mut codes: Vec<String> = serde_json::from_str(&row.backup_codes.unwrap_or_default())
        .unwrap_or_default();

    if let Some(pos) = codes.iter().position(|c| c == &code) {
        // Remove used code (one-time use)
        codes.remove(pos);
        
        sqlx::query!(
            "UPDATE users SET backup_codes = ? WHERE username = ?",
            serde_json::to_string(&codes).unwrap(),
            username
        )
        .execute(&*db)
        .await
        .map_err(|e| e.to_string())?;

        return Ok(true);
    }

    Ok(false)
}

fn generate_backup_codes(count: usize) -> Result<Vec<String>, String> {
    use ring::rand::{SecureRandom, SystemRandom};
    
    let rng = SystemRandom::new();
    let mut codes = Vec::with_capacity(count);

    for _ in 0..count {
        let mut bytes = [0u8; 4];
        rng.fill(&mut bytes).map_err(|e| e.to_string())?;
        
        let code = u32::from_be_bytes(bytes) % 100_000_000; // 8 digits
        codes.push(format!("{:08}", code));
    }

    Ok(codes)
}

// Update Cargo.toml dependencies
/*
[dependencies]
# ... existing ...
base32 = "0.4"
qrcode = { version = "0.14", features = ["svg", "png"] }
urlencoding = "2.1"
*/

// Update SQLite schema (migration)
/*
ALTER TABLE users ADD COLUMN totp_secret TEXT;
ALTER TABLE users ADD COLUMN totp_enabled INTEGER DEFAULT 0;
ALTER TABLE users ADD COLUMN backup_codes TEXT; -- JSON array
ALTER TABLE users ADD COLUMN last_login TIMESTAMP;

CREATE TABLE login_attempts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    success INTEGER NOT NULL,
    ip_address TEXT,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_login_attempts ON login_attempts(username, timestamp);
*/
