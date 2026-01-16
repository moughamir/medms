# Watiqa-Link MVP: Technical Architecture

## I. System Overview

**Objective**: Offline-first Document Management System for Commune de Bouskoura  
**Tech Stack**: Tauri (Rust + React) + Embedded SQLite + LibreOffice Headless  
**Deployment**: 
- **Desktop**: Windows/Linux (Tauri v2)
- **Mobile**: Android (Tauri Mobile + Capacitor)
- **Authentication**: TOTP 2FA (RFC 6238)

---

## II. Why Tauri Over Electron?

| Criterion | Tauri | Electron |
|-----------|-------|----------|
| **Binary Size** | ~3-5 MB | ~50-120 MB |
| **Memory Usage** | ~50-100 MB | ~200-400 MB |
| **Security** | Rust backend isolated from frontend | Node.js exposes native APIs |
| **Green IT** | ✅ Low power consumption | ❌ High overhead |
| **Offline** | ✅ Native FS access | ⚠️ Requires polyfills |

**Decision**: Use Tauri for edge deployment in municipal offices with limited hardware.

---

## III. MVP Architecture

```
┌─────────────────────────────────────────────────┐
│           TAURI FRONTEND (React)                │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐     │
│  │Dashboard │  │ Forms    │  │ Viewer   │     │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘     │
│       │             │              │            │
│       └─────────────┼──────────────┘            │
│                     │                           │
│              IPC (invoke/emit)                  │
└─────────────────────┼───────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────┐
│         TAURI BACKEND (Rust Core)               │
│                                                  │
│  ┌──────────────┐    ┌───────────────────┐     │
│  │ FSM Engine   │◄───┤  Command Handler  │     │
│  └──────────────┘    └───────────────────┘     │
│         │                      │                │
│         ▼                      ▼                │
│  ┌──────────────┐    ┌───────────────────┐     │
│  │   SQLite     │    │ Template Engine   │     │
│  │  (metadata)  │    │  (.docx inject)   │     │
│  └──────────────┘    └─────────┬─────────┘     │
│                                 │                │
│                                 ▼                │
│                      ┌───────────────────┐      │
│                      │ LibreOffice CLI   │      │
│                      │  (PDF Converter)  │      │
│                      └───────────────────┘      │
└──────────────────────────────────────────────────┘
                      │
                      ▼
              ┌────────────────┐
              │  File System   │
              │  /docs/        │
              │  /templates/   │
              └────────────────┘
```

---

## IV. Core Components

### 1. Frontend (React + Tailwind)

**Key Screens**:
- **Bureau d'Ordre**: Register incoming mail (N° d'Ordre, Expéditeur, Objet)
- **Service Dashboard**: View assigned documents, transition states
- **Document Viewer**: Embedded PDF preview (using `react-pdf`)
- **Template Manager**: Upload/Edit .docx templates

**State Management**: Zustand (lightweight, ~1KB)

### 2. Backend (Rust Tauri Commands)

**Commands** (exposed to frontend):

```rust
#[tauri::command]
async fn register_document(
    metadata: DocumentMetadata,
) -> Result<String, String> {
    // Insert into SQLite, return UUID
}

#[tauri::command]
async fn transition_state(
    doc_id: String,
    to_state: DocumentState,
    agent_id: String,
) -> Result<(), String> {
    // FSM validation + audit log
}

#[tauri::command]
async fn generate_police_doc(
    template_id: String,
    data: PoliceDocumentData,
) -> Result<String, String> {
    // Template injection + PDF conversion
    // Returns path to generated PDF
}

#[tauri::command]
async fn search_documents(
    query: String,
    filters: SearchFilters,
) -> Result<Vec<DocumentMetadata>, String> {
    // SQLite FTS5 full-text search
}
```

### 3. Database (SQLite with FTS5)

**Schema**:

```sql
CREATE TABLE documents (
    uuid TEXT PRIMARY KEY,
    numero_ordre TEXT UNIQUE NOT NULL,
    date_arrivee DATE NOT NULL,
    expediteur TEXT NOT NULL,
    cin_expediteur TEXT,
    objet TEXT NOT NULL,
    service_concerne TEXT NOT NULL,
    state TEXT NOT NULL, -- Registered/Dispatched/etc.
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE VIRTUAL TABLE documents_fts USING fts5(
    numero_ordre, expediteur, objet, service_concerne,
    content='documents',
    content_rowid='rowid'
);

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
```

**Why SQLite?**
- Zero-config embedded database
- Full-Text Search (FTS5) for Arabic text (with trigram extension)
- ~500KB footprint
- WAL mode for concurrent access

### 4. Document Conversion Pipeline

```rust
pub async fn generate_document(
    template: &str,
    data: PoliceDocumentData,
) -> Result<PathBuf> {
    // 1. Populate .docx template
    let populated_docx = TemplateEngine::populate_docx(
        &format!("templates/{}.docx", template),
        &data,
        &temp_file_path,
    )?;

    // 2. Convert to PDF/A via LibreOffice
    let converter = DocumentConverter::new(
        PathBuf::from("/usr/bin/soffice"), // Or bundled portable LO
        PathBuf::from("/tmp"),
    );
    
    let pdf_path = format!("docs/{}.pdf", data.uuid);
    converter.convert_with_retry(&populated_docx, &pdf_path, 3).await?;

    Ok(PathBuf::from(pdf_path))
}
```

**Font Configuration**:
Ensure `Noto Sans Arabic` is installed:
```bash
# Linux
apt-get install fonts-noto-arabic

# Bundled with app (add to Tauri resources)
resources/
  fonts/
    NotoSansArabic-Regular.ttf
```

Set environment variable in Tauri:
```rust
#[tauri::command]
fn init_libreoffice_fonts() {
    std::env::set_var("SAL_USE_VCLPLUGIN", "svp");
    std::env::set_var("HOME", app_config_dir()); // Point to bundled fonts
}
```

---

## V. Offline Synchronization

### Strategy: Event Sourcing + Conflict-Free Replicated Data Types (CRDTs)

**Problem**: Multiple offices may edit documents offline. When syncing, conflicts arise.

**Solution**:
1. **Event Log**: Each action (state transition, metadata update) is stored as an immutable event in SQLite.
2. **Sync Protocol**: 
   - USB Transfer: Export `events.db` + `docs/` folder
   - WiFi/BT: Use [Syncthing](https://syncthing.net/) or custom TCP protocol
3. **Conflict Resolution**: Last-Write-Wins (LWW) with timestamp + office ID as tiebreaker

**Implementation**:
```rust
#[derive(Serialize, Deserialize)]
struct SyncEvent {
    event_id: Uuid,
    doc_uuid: Uuid,
    event_type: EventType, // StateTransition, MetadataUpdate
    payload: serde_json::Value,
    office_id: String,
    timestamp: DateTime<Utc>,
}

// Export for sync
#[tauri::command]
async fn export_sync_package(since: DateTime<Utc>) -> Result<PathBuf> {
    let events = db.query_events_since(since)?;
    let bundle = SyncBundle { events, documents: vec![] };
    
    let export_path = "/tmp/sync_bouskoura_2026_01_15.watiqa";
    std::fs::write(export_path, serde_json::to_vec(&bundle)?)?;
    Ok(PathBuf::from(export_path))
}

// Import sync package
#[tauri::command]
async fn import_sync_package(path: PathBuf) -> Result<()> {
    let bundle: SyncBundle = serde_json::from_slice(&std::fs::read(path)?)?;
    
    for event in bundle.events {
        db.apply_event(event)?; // CRDT merge logic
    }
    Ok(())
}
```

---

## VI. Security & Licensing

### A. Security Model

**Threat Model**:
- **Unauthorized Access**: Office PC shared by multiple users
- **Data Exfiltration**: USB/network transfer of sensitive data
- **Tampering**: Modifying archived documents

**Mitigations**:

1. **Authentication**:
   - Local SQLite user table with bcrypt hashed passwords
   - Session tokens (JWT stored in Tauri secure storage)
   - Biometric unlock (Windows Hello / Linux PAM)

2. **Encryption at Rest**:
   - Use [sqlcipher](https://www.zetetic.net/sqlcipher/) (SQLite with AES-256)
   - Encrypt `/docs/` folder using [age](https://github.com/FiloSottile/age) (Rust crate)

3. **Audit Logging**:
   - All actions logged to append-only `audit.log`
   - Tamper-evident using Merkle tree (hash chain)

4. **Digital Signatures**:
   - Sign PDFs with office certificate (using [lopdf](https://github.com/J-F-Liu/lopdf) + OpenSSL)
   - Verify on load: reject if signature invalid

**Implementation Snippet**:
```rust
use ring::signature::{Ed25519KeyPair, KeyPair};

#[tauri::command]
fn sign_document(pdf_path: &str, private_key: &[u8]) -> Result<()> {
    let key_pair = Ed25519KeyPair::from_pkcs8(private_key)?;
    let pdf_bytes = std::fs::read(pdf_path)?;
    let signature = key_pair.sign(&pdf_bytes);
    
    // Embed signature in PDF metadata (XMP or AcroForm)
    embed_signature_in_pdf(pdf_path, signature.as_ref())?;
    Ok(())
}
```

### B. Licensing Strategy

**Options**:

| Model | Pros | Cons | Fit |
|-------|------|------|-----|
| **Perpetual License** | One-time payment | No recurring revenue | ✅ Govt prefers |
| **SaaS Subscription** | Recurring revenue | Requires cloud | ❌ Privacy concerns |
| **Per-Seat + Support** | Scalable | Complex | ✅ Hybrid approach |

**Recommended**: **Per-Office License + Annual Support**

**Pricing Structure**:
- **Base License**: 50,000 MAD (unlimited users in one office)
- **Annual Support**: 10,000 MAD (updates, bug fixes, training)
- **Additional Offices**: 30,000 MAD each

**License Key System**:
```rust
// Generate license
pub fn generate_license(
    office_id: &str,
    expiry: DateTime<Utc>,
    secret: &[u8],
) -> String {
    let payload = format!("{}|{}", office_id, expiry.to_rfc3339());
    let signature = hmac_sha256(secret, payload.as_bytes());
    base64::encode(format!("{}:{}", payload, hex::encode(signature)))
}

// Validate license
#[tauri::command]
fn validate_license(license_key: &str, secret: &[u8]) -> Result<bool> {
    let parts: Vec<&str> = license_key.split(':').collect();
    let (payload, sig_hex) = (parts[0], parts[1]);
    
    let expected_sig = hmac_sha256(secret, payload.as_bytes());
    if hex::encode(expected_sig) != sig_hex {
        return Ok(false);
    }
    
    // Check expiry
    let expiry_str = payload.split('|').nth(1).unwrap();
    let expiry = DateTime::parse_from_rfc3339(expiry_str)?;
    Ok(expiry > Utc::now())
}
```

**Hardware Binding** (optional):
```rust
use machine_uid::get;

#[tauri::command]
fn bind_license_to_machine(license_key: &str) -> Result<()> {
    let machine_id = get().map_err(|e| format!("Failed to get machine ID: {}", e))?;
    // Store hash(machine_id + license_key) in config
    // On startup, verify match
    Ok(())
}
```

---

## VII. Legal & Contractual Documents

### A. Required Documents

1. **Devis (Quote)**
   - Itemized costs: Development, Testing, Deployment, Training
   - Timeline: 3 months (MVP)
   - Payment schedule: 30% upfront, 40% on beta, 30% on delivery

2. **Contrat de Développement (Development Contract)**
   - Scope: Features listed in Cahier des Charges
   - Deliverables: Source code (escrow), Binaries, Documentation
   - Warranty: 6 months bug fixes
   - SLA: 48h response for critical bugs

3. **NDA (Accord de Confidentialité)**
   - Mutual NDA between developer and Commune
   - Covers: Citizen data, internal processes, security architecture
   - Duration: 5 years post-contract

4. **License Agreement**
   - Grant of perpetual, non-transferable license
   - Restrictions: No reverse engineering, resale
   - Support terms: Email/phone support during business hours

5. **Cahier des Charges (Specifications)**
   - Functional requirements (French/Arabic)
   - Non-functional: Performance (< 2s document generation), Security (ISO 27001)
   - Acceptance criteria: 100% of core features, 95% uptime in pilot

### B. Sample Clause (NDA)

```
Article 3 - Obligations de Confidentialité

Le Développeur s'engage à :
a) Ne pas divulguer les Informations Confidentielles à des tiers sans 
   autorisation écrite préalable de la Commune de Bouskoura ;
b) Limiter l'accès aux Informations Confidentielles aux membres de son 
   équipe ayant un besoin légitime ;
c) Mettre en place des mesures techniques (chiffrement AES-256, contrôle 
   d'accès par rôle) pour protéger les données stockées dans le système.

Durée : 5 ans à compter de la signature du Contrat.
```

---

## VIII. MVP Roadmap

### Phase 1: Core Backend (4 weeks)
- [ ] FSM engine (statig)
- [ ] SQLite schema + migrations (diesel/sqlx)
- [ ] Template injection engine (zip crate)
- [ ] LibreOffice integration + font config
- [ ] Unit tests (90% coverage)

### Phase 2: Tauri Integration (3 weeks)
- [ ] Tauri commands (register, transition, search)
- [ ] Secure storage (sqlcipher)
- [ ] License validation
- [ ] Build scripts (Windows/Linux)

### Phase 3: Frontend (4 weeks)
- [ ] React components (Dashboard, Forms, Viewer)
- [ ] Arabic RTL support (via `direction: rtl` CSS)
- [ ] PDF viewer (react-pdf)
- [ ] Dark mode

### Phase 4: Testing & Pilot (3 weeks)
- [ ] Integration tests (Selenium)
- [ ] Performance testing (1000+ docs)
- [ ] Pilot deployment (Bureau d'Ordre only)
- [ ] User training (2 days on-site)

**Total**: ~14 weeks (~3.5 months)

---

## IX. Cost Estimate

| Item | Hours | Rate (MAD) | Total |
|------|-------|------------|-------|
| Backend Development | 160 | 500 | 80,000 |
| Frontend Development | 120 | 400 | 48,000 |
| Tauri Integration | 80 | 500 | 40,000 |
| Testing & QA | 60 | 350 | 21,000 |
| Deployment & Training | 40 | 400 | 16,000 |
| **Subtotal** | | | **205,000** |
| Contingency (15%) | | | 30,750 |
| **Total (HT)** | | | **235,750** |
| TVA (20%) | | | 47,150 |
| **Total TTC** | | | **282,900** |

---

## X. Risk Mitigation

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| LibreOffice rendering issues | High | Medium | Bundle tested LO version; fallback to Gotenberg API |
| Arabic font rendering fails | Medium | High | Include Noto fonts in app bundle; test on target machines |
| License key leaked | Low | High | Hardware binding + online activation (opt-in) |
| SQLite corruption | Low | Critical | WAL mode + hourly backups + integrity checks |
| User resistance | Medium | Medium | Gradual rollout; keep paper workflow in parallel for 3 months |

---

## XI. Compliance & Standards

**Moroccan Legal Requirements**:
- **Dahir 09-08** (Data Protection): Encrypt PII (CIN, addresses)
- **ISO 27001**: Implement ISMS (security policies, access logs)
- **Accessibility**: WCAG 2.1 AA (for web portal, future Phase 2)

**Archival Standards**:
- PDF/A-3 format for long-term preservation
- Metadata embedding per ISO 19005-3
- Digital signatures per PAdES (PDF Advanced Electronic Signatures)

---

## XII. Next Steps

1. **Week 1**: Client meeting to validate architecture
2. **Week 2**: Sign NDA + Devis
3. **Week 3**: Setup dev environment (Tauri + Rust toolchain)
4. **Week 4**: Begin Phase 1 (Backend core)

**Key Decision**: Cloud sync vs. USB-only?
- Recommend starting with **USB-only** for MVP (lower complexity)
- Add cloud sync (Supabase) in Phase 2 after security audit

---

## XIII. References

- [Tauri Documentation](https://tauri.app/)
- [SQLite FTS5](https://www.sqlite.org/fts5.html)
- [LibreOffice Headless](https://wiki.documentfoundation.org/Development/Headless)
- [PDF/A Standard](https://pdfa.org/resource/iso-19005-pdfa/)
- [Dahir 09-08 (Morocco)](https://www.cndp.ma/)
