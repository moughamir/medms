# Watiqa-Link Phase 1: Agentic Setup Prompt

## Context
You are an AI coding agent in Google Antigravity IDE. Your task is to scaffold a production-grade Tauri v2 desktop application with Rust backend + React frontend for document management in Moroccan government offices.

---

## Project Specification

**Name**: `watiqa-link`  
**Tech Stack**: 
- Backend: Rust + Tauri 2.0 + SQLx + Ring (crypto)
- Frontend: React 18 + TypeScript + Tailwind CSS + Zustand
- Database: SQLite with FTS5 + SQLCipher
- Document Engine: LibreOffice Headless + Zip manipulation

**Core Features**:
1. TOTP authentication (RFC 6238)
2. Document workflow FSM (5 states)
3. .docx template injection → PDF/A conversion
4. Arabic/French bilingual UI with RTL support
5. Offline-first with USB sync

---

## Step-by-Step Instructions

### 1. Initialize Project Structure

```bash
# Create Tauri project
pnpm create tauri-app@latest \
  --name watiqa-link \
  --template react-ts \
  --manager pnpm \
  --yes

cd watiqa-link

# Backend dependencies
cd src-tauri
cargo add serde --features derive
cargo add serde_json
cargo add tokio --features full
cargo add sqlx --features runtime-tokio-rustls,sqlite,migrate
cargo add uuid --features v4,serde
cargo add chrono --features serde
cargo add anyhow thiserror
cargo add ring base32 hex
cargo add zip --features deflate
cargo add qrcode --features svg,png
cargo add urlencoding

# Frontend dependencies
cd ..
pnpm add zustand react-pdf date-fns clsx
pnpm add -D @types/react-pdf
pnpm add -D tailwindcss postcss autoprefixer
npx tailwindcss init -p
```

### 2. Create Database Schema

**File**: `src-tauri/migrations/001_init.sql`

```sql
-- Users with TOTP
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    full_name TEXT NOT NULL,
    role TEXT NOT NULL CHECK(role IN ('admin','agent','viewer')),
    totp_secret TEXT,
    totp_enabled INTEGER DEFAULT 0,
    backup_codes TEXT,
    last_login TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Documents registry
CREATE TABLE documents (
    uuid TEXT PRIMARY KEY,
    numero_ordre TEXT UNIQUE NOT NULL,
    date_arrivee DATE NOT NULL,
    expediteur TEXT NOT NULL,
    cin_expediteur TEXT,
    objet TEXT NOT NULL,
    service_concerne TEXT NOT NULL,
    state TEXT NOT NULL CHECK(state IN ('Registered','Dispatched','Annotated','Signed','Archived')),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Full-text search
CREATE VIRTUAL TABLE documents_fts USING fts5(
    numero_ordre, expediteur, objet, service_concerne,
    content='documents', content_rowid='rowid'
);

-- Audit trail
CREATE TABLE state_transitions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    doc_uuid TEXT NOT NULL,
    from_state TEXT NOT NULL,
    to_state TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    notes TEXT,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (doc_uuid) REFERENCES documents(uuid)
);

-- Login attempts (rate limiting)
CREATE TABLE login_attempts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    success INTEGER NOT NULL,
    ip_address TEXT,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_login_attempts ON login_attempts(username, timestamp);

-- Default admin user (password: admin123)
INSERT INTO users (id, username, password_hash, full_name, role) VALUES
    ('admin', 'admin', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYWJ3.VQo6a', 'Administrateur', 'admin');
```

### 3. Rust Backend Structure

**Create these files** with the following content:

#### `src-tauri/src/security/totp.rs`

```rust
use anyhow::{Context, Result};
use base32::Alphabet;
use ring::hmac;
use serde::{Deserialize, Serialize};

const DEFAULT_DIGITS: u32 = 6;
const DEFAULT_PERIOD: u64 = 30;
const VALIDATION_WINDOW: i64 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpSecret {
    pub secret: String,
    pub issuer: String,
    pub account: String,
    pub digits: u32,
    pub period: u64,
}

impl TotpSecret {
    pub fn generate(issuer: &str, account: &str) -> Result<Self> {
        use ring::rand::{SecureRandom, SystemRandom};
        
        let rng = SystemRandom::new();
        let mut bytes = vec![0u8; 20];
        rng.fill(&mut bytes).map_err(|_| anyhow::anyhow!("Random gen failed"))?;
        
        let secret = base32::encode(Alphabet::RFC4648 { padding: false }, &bytes);
        
        Ok(Self {
            secret,
            issuer: issuer.to_string(),
            account: account.to_string(),
            digits: DEFAULT_DIGITS,
            period: DEFAULT_PERIOD,
        })
    }

    pub fn provisioning_uri(&self) -> String {
        format!(
            "otpauth://totp/{}:{}?secret={}&issuer={}&digits={}&period={}",
            urlencoding::encode(&self.issuer),
            urlencoding::encode(&self.account),
            self.secret,
            urlencoding::encode(&self.issuer),
            self.digits,
            self.period
        )
    }

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

    pub fn verify_code(&self, code: &str, time: Option<u64>) -> Result<bool> {
        let timestamp = time.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });

        let current_counter = (timestamp / self.period) as i64;

        for offset in -VALIDATION_WINDOW..=VALIDATION_WINDOW {
            let counter = (current_counter + offset) as u64;
            if self.generate_hotp(counter)? == code {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn generate_hotp(&self, counter: u64) -> Result<String> {
        let secret_bytes = base32::decode(Alphabet::RFC4648 { padding: false }, &self.secret)
            .context("Invalid Base32 secret")?;

        let counter_bytes = counter.to_be_bytes();
        let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, &secret_bytes);
        let tag = hmac::sign(&key, &counter_bytes);
        let hmac_result = tag.as_ref();

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
```

#### `src-tauri/src/commands/auth.rs`

```rust
use crate::security::totp::TotpSecret;
use tauri::State;
use sqlx::SqlitePool;
use serde::Serialize;

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
    let totp = TotpSecret::generate("Watiqa - Bouskoura", &username)
        .map_err(|e| e.to_string())?;

    let backup_codes = (0..8).map(|_| {
        format!("{:08}", rand::random::<u32>() % 100_000_000)
    }).collect::<Vec<_>>();

    sqlx::query!(
        "UPDATE users SET totp_secret = ?, backup_codes = ?, totp_enabled = 1 WHERE username = ?",
        totp.secret,
        serde_json::to_string(&backup_codes).unwrap(),
        username
    )
    .execute(&*db)
    .await
    .map_err(|e| e.to_string())?;

    Ok(TotpSetupResponse {
        secret: totp.secret,
        qr_uri: totp.provisioning_uri(),
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
```

#### `src-tauri/src/main.rs`

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod security;

use sqlx::sqlite::SqlitePoolOptions;
use commands::auth::{setup_totp, verify_totp};

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite:watiqa.db")
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    tauri::Builder::default()
        .manage(pool)
        .invoke_handler(tauri::generate_handler![
            setup_totp,
            verify_totp,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 4. Frontend Setup

#### `src/stores/authStore.ts`

```typescript
import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/tauri';

interface AuthStore {
  isAuthenticated: boolean;
  username: string | null;
  totpEnabled: boolean;
  login: (username: string, password: string) => Promise<void>;
  verifyTotp: (code: string) => Promise<boolean>;
  logout: () => void;
}

export const useAuthStore = create<AuthStore>((set, get) => ({
  isAuthenticated: false,
  username: null,
  totpEnabled: false,

  login: async (username, password) => {
    // TODO: Verify password with bcrypt
    set({ username, totpEnabled: true });
  },

  verifyTotp: async (code) => {
    const { username } = get();
    if (!username) return false;

    const valid = await invoke<boolean>('verify_totp', {
      username,
      code,
    });

    if (valid) {
      set({ isAuthenticated: true });
    }

    return valid;
  },

  logout: () => {
    set({ isAuthenticated: false, username: null });
  },
}));
```

#### `src/pages/TotpSetup.tsx`

```tsx
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { QRCodeSVG } from 'qrcode.react';

export const TotpSetupPage = () => {
  const [step, setStep] = useState<'qr' | 'verify'>('qr');
  const [qrUri, setQrUri] = useState('');
  const [backupCodes, setBackupCodes] = useState<string[]>([]);

  const handleSetup = async (username: string) => {
    const result = await invoke<{
      secret: string;
      qr_uri: string;
      backup_codes: string[];
    }>('setup_totp', { username });

    setQrUri(result.qr_uri);
    setBackupCodes(result.backup_codes);
    setStep('verify');
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-100">
      <div className="bg-white p-8 rounded-lg shadow-md max-w-md">
        {step === 'qr' && (
          <div className="text-center">
            <h2 className="text-2xl font-bold mb-4">
              تفعيل المصادقة الثنائية
            </h2>
            <QRCodeSVG value={qrUri} size={256} className="mx-auto mb-4" />
            <p className="text-sm text-gray-600">
              امسح الرمز باستخدام Google Authenticator
            </p>
          </div>
        )}
        
        {step === 'verify' && (
          <div>
            <h3 className="text-lg font-semibold mb-2">رموز الاحتياط</h3>
            <div className="grid grid-cols-2 gap-2">
              {backupCodes.map((code, i) => (
                <div key={i} className="font-mono bg-gray-100 p-2 rounded">
                  {code}
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
```

### 5. Configuration Files

#### `tailwind.config.js`

```javascript
export default {
  content: ['./src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      fontFamily: {
        arabic: ['Noto Sans Arabic', 'sans-serif'],
      },
    },
  },
  plugins: [],
};
```

#### `src-tauri/tauri.conf.json`

```json
{
  "build": {
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build",
    "devPath": "http://localhost:1420",
    "distDir": "../dist"
  },
  "package": {
    "productName": "Watiqa-Link",
    "version": "0.1.0"
  },
  "tauri": {
    "bundle": {
      "identifier": "ma.bouskoura.watiqa",
      "resources": ["resources/*"]
    },
    "allowlist": {
      "all": false,
      "fs": {
        "scope": ["$APPDATA/watiqa/*"]
      }
    }
  }
}
```

---

## Validation Steps

After setup completes:

1. **Test TOTP generation**:
```bash
cd src-tauri
cargo test --lib totp
```

2. **Run development server**:
```bash
pnpm tauri dev
```

3. **Verify database creation**:
```bash
sqlite3 src-tauri/watiqa.db ".tables"
```

Expected output: `documents`, `users`, `state_transitions`, `documents_fts`

4. **Test TOTP flow**:
   - Login as `admin`/`admin123`
   - Navigate to TOTP setup
   - Scan QR with Google Authenticator
   - Verify 6-digit code

---

## Success Criteria

✅ Project compiles without errors  
✅ Database migrations run successfully  
✅ TOTP codes validate correctly (±30s window)  
✅ QR codes render in frontend  
✅ Login flow completes end-to-end  

---

## Next Steps (Post-Phase 1)

- Implement document FSM (5 states)
- Add LibreOffice integration
- Create .docx template injection
- Setup sync mechanism

---

## Error Handling

If you encounter:

**"Failed to connect to database"**: Check SQLite file permissions  
**"Base32 decode error"**: Verify secret format (no spaces/padding)  
**"Ring crypto error"**: Ensure `ring` feature flags are correct  
**"Migration not found"**: Run `cargo sqlx prepare` in `src-tauri/`

---

## Notes for AI Agent

- Prioritize security (constant-time comparisons, rate limiting)
- Use `anyhow::Result` for all fallible operations
- Add comprehensive error messages in Arabic/French
- Test TOTP with multiple time offsets (past/future)
- Ensure QR codes work with Google Authenticator AND Authy

**DO NOT**:
- Hardcode secrets in source
- Use `unwrap()` in production code paths
- Skip input validation
- Commit `.db` files to git
