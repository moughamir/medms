---
trigger: always_on
---

project:
  name: "Watiqa-Link"
  type: "Document Management System"
  domain: "Moroccan Municipal Government"
  languages: ["Rust", "TypeScript", "Arabic", "French"]
core_principles:
  - "Security first: Never compromise on crypto/auth"
  - "Offline-first: All features work without network"
  - "Bilingual: Arabic RTL + French must coexist"
  - "Type-safe: Rust backend, TypeScript frontend"
  - "Green IT: Minimize resource usage"
security:
  authentication:
    - "ALWAYS use constant-time comparison for TOTP codes"
    - "NEVER log sensitive data (passwords, TOTP secrets, PII)"
    - "Rate limit: Max 5 failed auth attempts per 15 minutes"
    - "Session timeout: 15 minutes of inactivity"
    - "TOTP window: ±1 time step only (30 seconds)"
  cryptography:
    - "Use `ring` crate for all crypto operations"
    - "TOTP secrets: Base32 encoded, 160-bit minimum"
    - "Database encryption: SQLCipher with AES-256"
    - "PDF signatures: Ed25519 (not RSA)"
    - "NEVER use deprecated algorithms (MD5, SHA1 except TOTP)"
  data_protection:
    - "PII must be encrypted at rest (CIN, addresses)"
    - "Audit trail: Merkle tree hash chain for tamper detection"
    - "Backup codes: One-time use, invalidate after consumption"
    - "License keys: HMAC-SHA256 bound to machine ID"
  code_patterns:
    forbidden:
      - "unwrap() in production code paths"
      - "Hardcoded secrets or API keys"
      - "SQL string concatenation (use prepared statements)"
      - "eval() or dynamic code execution"
    required:
      - "anyhow::Result for all fallible operations"
      - "sqlx::query! macro (compile-time checked SQL)"
      - "Sanitize all user inputs (filenames, text fields)"
rust:
  style:
    - "Follow Rust 2021 edition idioms"
    - "Use clippy::pedantic warnings"
    - "Max function length: 50 lines (except generated code)"
    - "Error messages: Include context with .context()"
  naming:
    - "Commands: verb_noun (e.g., verify_totp, register_document)"
    - "Modules: snake_case (e.g., state_machine, totp_engine)"
    - "Structs: PascalCase with descriptive names"
    - "Constants: SCREAMING_SNAKE_CASE"
  patterns:
    - "Prefer Option<T> over null checks"
    - "Use thiserror for custom errors with context"
    - "Async: tokio runtime, avoid blocking operations"
    - "Testing: Unit tests in same file, integration tests in tests/"
  dependencies:
    allowed:
      - "serde, sqlx, tokio, ring, uuid, chrono, anyhow, thiserror"
      - "base32, qrcode, zip, tesseract (mobile only)"
    forbidden:
      - "openssl (use ring instead)"
      - "diesel (use sqlx for async)"
      - "reqwest (offline-first requirement)"
typescript:
  style:
    - "Strict mode: noImplicitAny, strictNullChecks"
    - "Functional components with hooks (no class components)"
    - "Use Zustand for state management (not Redux)"
    - "Tailwind utility classes only (no custom CSS files)"
  naming:
    - "Components: PascalCase (e.g., TotpSetupPage)"
    - "Hooks: useCamelCase (e.g., useDocumentStore)"
    - "Files: camelCase.tsx (match default export)"
    - "Tauri commands: snake_case (match Rust)"
  patterns:
    - "Prop types: Use interfaces, not type aliases"
    - "Error handling: Try-catch with user-friendly messages"
    - "Loading states: Show spinners for >200ms operations"
    - "Forms: Controlled components with validation"
database:
  schema:
    - "Always use migrations (SQLx migrate!)"
    - "Primary keys: TEXT uuid (not INTEGER autoincrement)"
    - "Timestamps: TIMESTAMP DEFAULT CURRENT_TIMESTAMP"
    - "Foreign keys: ALWAYS define with ON DELETE CASCADE/RESTRICT"
    - "Indexes: Create for all foreign keys and search fields"
  queries:
    - "Use sqlx::query! for compile-time checking"
    - "Parameterized queries only (prevent SQL injection)"
    - "Transactions: Use begin/commit for multi-step operations"
    - "FTS5: Use for Arabic/French full-text search"
  performance:
    - "Batch inserts: Use VALUES (...), (...) syntax"
    - "Pagination: LIMIT/OFFSET with indexes"
    - "Avoid SELECT * (specify columns)"
    - "Connection pool: Max 5 connections for desktop app"
i18n:
  text_rendering:
    - "Arabic: Use Noto Sans Arabic font (embed in app)"
    - "RTL layout: CSS direction: rtl + text-align: right"
    - "Reshaping: Use arabic_reshaper crate for correct letter forms"
    - "BiDi: Apply unicode-bidi for mixed content"
    
  ui_patterns:
    - "All labels: Show both languages (e.g., 'المستندات / Documents')"
    - "Forms: Right-aligned for Arabic, left-aligned for French"
    - "Error messages: Provide both translations"
    - "Document templates: Use placeholders like {{CITIZEN_NAME}}"
    
  content_rules:
    - "Legal terms: Prefer Arabic (e.g., 'محضر معاينة' not 'PV')"
    - "Technical UI: Bilingual labels (e.g., 'حفظ / Enregistrer')"
    - "Dates: Format as DD/MM/YYYY (Moroccan standard)"
    - "Numbers: Use Western Arabic numerals (0-9) not Eastern (٠-٩)"
fsm:
  states:
    - "Registered → Dispatched → Annotated → Signed → Archived"
    - "Transitions must be validated (no skipping states)"
    - "Each transition: Record timestamp + agent ID + notes"
  validation:
    - "Cannot sign before annotation"
    - "Cannot archive unsigned documents"
    - "Allow backward transitions only for corrections (with justification)"
  audit:
    - "Log ALL state changes to state_transitions table"
    - "Include: from_state, to_state, timestamp, agent_id, notes"
    - "Hash chain: Each entry includes SHA-256 of previous entry"
templates:
  docx_injection:
    - "Unzip .docx → Modify word/document.xml → Re-zip"
    - "Placeholders: {{VARIABLE_NAME}} (uppercase, no spaces)"
    - "Preserve formatting: Don't remove XML namespaces"
    - "Footer injection: Add UUID + timestamp in w:ftr section"
  pdf_conversion:
    - "Use LibreOffice --headless --convert-to pdf"
    - "Ensure Noto Arabic fonts in environment (SAL_USE_VCLPLUGIN=svp)"
    - "Output: PDF/A-3 format for archival compliance"
    - "Retry logic: 3 attempts with exponential backoff"
mobile:
  permissions:
    request_rationale:
      - "Camera: 'لمسح الوثائق / Pour scanner les documents'"
      - "Biometric: 'للوصول الآمن / Pour un accès sécurisé'"
      - "Storage: Only request when actually needed (not on startup)"
  ui:
    - "Bottom navigation bar for main sections"
    - "Swipe gestures for state transitions"
    - "Pull-to-refresh for document list"
    - "Haptic feedback on critical actions (sign, archive)"
  performance:
    - "Lazy load document list (50 items per page)"
    - "Compress images before OCR (max 2MB)"
    - "Cache TOTP secrets in Android KeyStore"
    - "Battery optimization: Disable background sync when <15%"
testing:
  coverage:
    - "Unit tests: 85% minimum for core modules (FSM, TOTP, crypto)"
    - "Integration tests: All Tauri commands must have test coverage"
    - "Security tests: Timing attacks, SQL injection, XSS attempts"
  test_data:
    - "Use deterministic TOTP time values (not SystemTime::now())"
    - "Mock SQLite with in-memory database (:memory:)"
    - "Test with Arabic/French/mixed content"
    - "Edge cases: Empty strings, special chars, max lengths"
  ci_pipeline:
    - "cargo test --all-features"
    - "cargo clippy -- -D warnings"
    - "cargo audit (check dependencies for CVEs)"
    - "pnpm test (React component tests)"
docs:
  code_comments:
    - "Public APIs: Rustdoc with examples"
    - "Complex algorithms: Link to RFC/spec (e.g., RFC 6238 for TOTP)"
    - "Security decisions: Explain why (e.g., 'constant-time to prevent timing attacks')"
    - "TODOs: Include ticket number (e.g., '// TODO(#42): Add retry logic')"
  commit_messages:
    - "Format: type(scope): description"
    - "Types: feat, fix, sec, perf, refactor, test, docs"
    - "Security fixes: Prefix with 'sec:' and reference CVE if applicable"
    - "Example: 'sec(totp): Use constant-time comparison (fixes timing attack)'"
errors:
  user_facing:
    - "Arabic + French messages"
    - "No technical jargon (e.g., 'Failed to connect' not 'ECONNREFUSED')"
    - "Actionable: Tell user what to do next"
    - "Example: 'الرمز غير صحيح. حاول مرة أخرى / Code incorrect. Réessayez'"
  logging:
    - "Use tracing crate with levels: error!, warn!, info!, debug!"
    - "Include context: user ID (hashed), operation, timestamp"
    - "Redact sensitive data: TOTP codes, passwords, CINs"
    - "Rotate logs: Max 100MB per file, keep last 10 files"
  recovery:
    - "Database errors: Retry once, then show 'Save failed' message"
    - "TOTP errors: Show backup code option after 3 failures"
    - "Conversion errors: Queue for retry, notify user"
    - "Network errors: N/A (offline-first design)"
performance:
  desktop:
    - "App startup: <1 second"
    - "Document generation: <2 seconds (95th percentile)"
    - "Search: <200ms for 10,000 documents"
    - "State transition: <100ms"
    - "Memory usage: <150MB idle, <300MB under load"
  mobile:
    - "App startup: <2 seconds"
    - "TOTP generation: <50ms"
    - "Camera capture: <500ms from tap to capture"
    - "Battery: <5% drain per hour in active use"
    - "APK size: <15MB"
sync:
  strategy:
    - "Event sourcing: All changes are immutable events"
    - "CRDT: Last-Write-Wins with timestamp + office ID tiebreaker"
    - "Export format: JSON bundle with signature"
    - "Import: Validate signature, replay events, resolve conflicts"
  usb_transfer:
    - "Auto-detect USB drives on insertion"
    - "Export filename: watiqa_sync_YYYYMMDD_HHMMSS.bundle"
    - "Compression: gzip with level 6 (balance size/speed)"
    - "Verification: SHA-256 checksum included in bundle"
  conflict_resolution:
    - "State transitions: Merge if non-conflicting, else show dialog"
    - "Metadata: LWW by timestamp (server time if available)"
    - "Documents: Immutable (UUID prevents duplicates)"
deployment:
  build:
    - "Release profile: opt-level = 3, lto = true, strip = true"
    - "Bundle fonts: Include in resources/ directory"
    - "Templates: Copy to resources/templates/"
    - "LibreOffice: Bundle portable version (Windows) or document dependency (Linux)"
  installers:
    - "Windows: .msi with auto-update via Tauri updater"
    - "Linux: .deb and .AppImage"
    - "Android: .apk signed with production keystore"
  updates:
    - "Check for updates on startup (with user consent)"
    - "Auto-download if <50MB,