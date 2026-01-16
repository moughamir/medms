# Watiqa-Police (MEDMS) Production Readiness Plan

## Executive Summary
**Current Status**: AT RISK (45/100)  
**Target**: 85/100 for production deployment  
**Timeline**: 3 weeks (120 hours total effort)

---

## Critical Issues Summary

| Priority | Issue | Impact | Effort |
|----------|-------|--------|--------|
| P0 | Arabic rendering broken | Documents unreadable | 4h |
| P0 | No template system | Core feature missing | 24h |
| P0 | Concurrency deadlocks | Production crashes | 16h |
| P1 | Input sanitization | Security vulnerability | 8h |
| P1 | Error handling | Poor UX | 12h |
| P2 | Memory leaks | Resource exhaustion | 8h |
| P2 | Green IT optimization | High costs | 16h |

---

## Milestone 1: Critical Fixes (Week 1)
**Goal**: Resolve P0 blockers → 65/100 readiness

### M1.1: Arabic Rendering Fix (Day 1) ✓ 4 hours
**Owner**: Backend Engineer  
**Status**: 🔴 BLOCKING

**Tasks**:
```rust
// File: src/generator/docx.rs
use crate::gui::text_utils::reshape;

impl DocumentGenerator {
    fn generate_content(&self) -> Paragraph {
        let content_text = match self.metadata.doc_type {
            // ... existing logic
        };
        
        // ✅ ADD: Apply reshaping before paragraph creation
        let shaped_text = reshape(&content_text);
        
        Paragraph::new()
            .add_run(Run::new().add_text(&shaped_text).size(24))
            .align(AlignmentType::Right)
    }
}
```

**Validation**:
- [ ] Test document with "محضر معاينة" renders as connected letters
- [ ] Mixed French/Arabic paragraphs maintain RTL direction
- [ ] PDF output preserves ligatures (verify with "محمد" → ﻣﺤﻤﺪ not م-ح-م-د)

---

### M1.2: Template Engine Implementation (Day 2-4) ✓ 24 hours
**Owner**: Backend Engineer  
**Status**: 🔴 BLOCKING

**Architecture**:
```rust
// File: src/generator/template_engine.rs (NEW)
use quick_xml::Reader;
use std::collections::HashMap;
use zip::{ZipArchive, ZipWriter};

pub struct TemplateEngine {
    templates: HashMap<String, Vec<u8>>, // Cached .docx bytes
}

impl TemplateEngine {
    pub async fn populate(
        &self,
        template_id: &str,
        data: HashMap<String, String>
    ) -> Result<Vec<u8>> {
        // 1. Extract document.xml from .docx zip
        let docx_bytes = self.templates.get(template_id)
            .context("Template not found")?;
        
        let mut archive = ZipArchive::new(Cursor::new(docx_bytes))?;
        let mut doc_xml = String::new();
        archive.by_name("word/document.xml")?.read_to_string(&mut doc_xml)?;
        
        // 2. Replace placeholders with XML-escaped, shaped text
        for (key, value) in data {
            let escaped = xml_escape(&value);
            let shaped = reshape(&escaped);
            doc_xml = doc_xml.replace(&format!("{{{{{}}}}}", key), &shaped);
        }
        
        // 3. Inject UUID footer
        doc_xml = inject_footer_metadata(&doc_xml, &uuid::Uuid::new_v4())?;
        
        // 4. Rezip modified archive
        rezip_docx(archive, doc_xml)
    }
}

fn xml_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
```

**Dependencies to add**:
```toml
[dependencies]
zip = "0.6"
quick-xml = "0.31"
```

**Templates to create**:
- [ ] `templates/proces_verbal_v1.docx` with placeholders: `{{CITIZEN_NAME}}`, `{{CIN}}`, `{{DATE}}`, `{{VIOLATION}}`
- [ ] `templates/avertissement_v1.docx`
- [ ] `templates/mise_en_demeure_v1.docx`

**Validation**:
- [ ] Populate template with test data: Name="محمد العربي", CIN="AB123456"
- [ ] Verify placeholders replaced correctly
- [ ] Open generated .docx in MS Word/LibreOffice (no corruption)
- [ ] Convert to PDF and check Arabic rendering

---

### M1.3: Concurrency Control (Day 5) ✓ 16 hours
**Owner**: Backend Engineer  
**Status**: 🔴 BLOCKING

**Implementation**:
```rust
// File: src/generator/docx.rs
use tokio::sync::Semaphore;
use once_cell::sync::Lazy;

static OFFICE_SEMAPHORE: Lazy<Arc<Semaphore>> = 
    Lazy::new(|| Arc::new(Semaphore::new(2))); // Max 2 concurrent conversions

impl DocumentGenerator {
    pub async fn create_document(&self, output_path: &str) -> Result<()> {
        // 1. Generate .docx from template
        let template_engine = TemplateEngine::default();
        let data = self.build_template_data();
        let docx_bytes = template_engine.populate(&self.metadata.doc_type.to_string(), data).await?;
        
        // 2. Save temporary .docx
        let temp_docx = tempfile::Builder::new()
            .suffix(".docx")
            .tempfile()?;
        tokio::fs::write(temp_docx.path(), docx_bytes).await?;
        
        // 3. Convert with semaphore (prevents process overload)
        let _permit = OFFICE_SEMAPHORE.acquire().await?;
        
        let conversion = tokio::time::timeout(
            Duration::from_secs(30),
            tokio::process::Command::new("soffice")
                .args([
                    "--headless",
                    "--convert-to", "pdf:writer_pdf_Export",
                    "--outdir", output_path,
                    temp_docx.path().to_str().unwrap()
                ])
                .env("LANG", "ar_MA.UTF-8")
                .kill_on_drop(true)
                .output()
        ).await;
        
        match conversion {
            Ok(Ok(output)) if output.status.success() => Ok(()),
            Ok(Ok(output)) => Err(anyhow!("Conversion failed: {}", 
                String::from_utf8_lossy(&output.stderr))),
            Ok(Err(e)) => Err(e.into()),
            Err(_) => Err(anyhow!("Conversion timeout (30s)")),
        }
    }
}
```

**Dependencies to add**:
```toml
[dependencies]
once_cell = "1.19"
tempfile = "3.8"
```

**Validation**:
- [ ] Stress test: 10 concurrent requests, verify only 2 LibreOffice processes active
- [ ] Monitor with `ps aux | grep soffice` during load
- [ ] Confirm no zombie processes after conversion failures

---

## Milestone 2: Security & Stability (Week 2)
**Goal**: Harden production reliability → 75/100 readiness

### M2.1: Input Validation & Sanitization (Day 6-7) ✓ 8 hours
**Owner**: Backend Engineer

**Implementation**:
```rust
// File: src/gui/views/commerce.rs
use validator::Validate;

#[derive(Debug, Clone, Validate)]
pub struct CommerceForm {
    #[validate(length(min = 1, max = 200, message = "التسمية التجارية مطلوبة"))]
    pub denomination: String,
    
    #[validate(length(min = 1, max = 100))]
    pub owner_name: String,
    
    #[validate(regex(path = "CIN_REGEX", message = "رقم ب.و.ت غير صالح"))]
    pub owner_cin: String,
    
    #[validate(phone)]
    pub owner_phone: String,
    // ...
}

lazy_static! {
    static ref CIN_REGEX: Regex = Regex::new(r"^[A-Z]{1,2}[0-9]{5,6}$").unwrap();
}

impl CommerceForm {
    pub fn validate_and_sanitize(&mut self) -> Result<(), Vec<String>> {
        // 1. Validate structure
        self.validate()
            .map_err(|e| e.field_errors()
                .into_iter()
                .flat_map(|(_, errs)| errs.iter().map(|e| e.to_string()))
                .collect())?;
        
        // 2. Sanitize for XML injection
        self.denomination = xml_escape(&self.denomination);
        self.owner_name = xml_escape(&self.owner_name);
        // ... sanitize all string fields
        
        Ok(())
    }
}
```

**Dependencies**:
```toml
[dependencies]
validator = { version = "0.16", features = ["derive"] }
regex = "1.10"
lazy_static = "1.4"
```

**Tests**:
- [ ] Attempt SQL-like injection: `Owner Name = "'; DROP TABLE--"`
- [ ] XML injection: `Name = "<script>alert('XSS')</script>"`
- [ ] Unicode homoglyphs: `CIN = "АB123456"` (Cyrillic A instead of Latin)

---

### M2.2: Error Handling & User Feedback (Day 8-9) ✓ 12 hours
**Owner**: Frontend + Backend

**Backend Changes**:
```rust
// File: src/generator/docx.rs
#[derive(Debug, thiserror::Error)]
pub enum DocumentError {
    #[error("Template not found: {0}")]
    TemplateNotFound(String),
    
    #[error("LibreOffice conversion failed: {0}")]
    ConversionFailed(String),
    
    #[error("Font rendering error (Arabic not supported)")]
    FontError,
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] anyhow::Error),
}

impl DocumentGenerator {
    pub async fn create_document(&self, path: &str) -> Result<String, DocumentError> {
        // ... existing logic with proper error propagation
        
        // Return generated file path or detailed error
        Ok(format!("{}/output.pdf", path))
    }
}
```

**GUI Changes**:
```rust
// File: src/gui/app.rs
fn generate_document(&mut self) {
    match self.document_generator.create_document("docs/").await {
        Ok(path) => {
            self.set_status(
                format!("✅ {}: {}", tr("doc_created", &lang), path),
                StatusType::Success
            );
        }
        Err(DocumentError::ConversionFailed(msg)) => {
            self.set_status(
                format!("❌ فشل التحويل: {}\nالمرجو التحقق من LibreOffice", msg),
                StatusType::Error
            );
            tracing::error!("Conversion failed: {}", msg);
        }
        Err(e) => {
            self.set_status(
                format!("❌ خطأ: {}", e),
                StatusType::Error
            );
        }
    }
}
```

**Validation**:
- [ ] Trigger conversion failure (kill LibreOffice mid-process)
- [ ] Verify user sees actionable error message (not generic "error")
- [ ] Confirm errors logged to `watiqa.log` with timestamps

---

### M2.3: Memory & Resource Management (Day 10) ✓ 8 hours
**Owner**: Backend Engineer

**Changes**:
```rust
// File: src/database/excel.rs
use tokio::fs;

impl ExcelDatabase {
    pub async fn save_commerce(&self, commerce: &CommerceInfo) -> Result<()> {
        // ✅ CHANGE: Use async I/O to prevent blocking
        let docs_data = self.read_sheet_data_async(SHEET_DOCUMENTS).await?;
        // ...
    }
    
    async fn read_sheet_data_async(&self, sheet: &str) -> Result<Vec<Vec<String>>> {
        let bytes = tokio::fs::read(&self.file_path).await?;
        // Parse in separate task to avoid blocking
        tokio::task::spawn_blocking(move || {
            let mut workbook: Xlsx<_> = open_workbook_from_bytes(&bytes)?;
            // ... existing parsing logic
        }).await?
    }
}

// Add cleanup for temporary files
impl Drop for DocumentGenerator {
    fn drop(&mut self) {
        // Clean up any temp .docx files
        if let Some(temp_dir) = &self.temp_dir {
            let _ = std::fs::remove_dir_all(temp_dir);
        }
    }
}
```

**Profiling**:
- [ ] Run with `heaptrack` for 100 document generations
- [ ] Verify memory returns to baseline after each batch
- [ ] Check for file descriptor leaks (`lsof -p <PID>`)

---

## Milestone 3: Production Optimization (Week 3)
**Goal**: Performance tuning & deployment prep → 85/100 readiness

### M3.1: LibreOffice Configuration (Day 11-12) ✓ 16 hours
**Owner**: DevOps + Backend

**Dockerfile**:
```dockerfile
FROM rust:1.75-slim-bookworm

# Install LibreOffice with Arabic support
RUN apt-get update && apt-get install -y \
    libreoffice-writer-nogui \
    fonts-noto-core \
    fonts-noto-arabic \
    fonts-liberation2 \
    && rm -rf /var/lib/apt/lists/*

# Configure font paths
ENV FONTCONFIG_PATH=/etc/fonts
COPY fonts.conf /etc/fonts/local.conf

# Copy application
COPY target/release/moroccan_docs /usr/local/bin/
WORKDIR /app

CMD ["moroccan_docs"]
```

**fonts.conf**:
```xml
<?xml version="1.0"?>
<!DOCTYPE fontconfig SYSTEM "fonts.dtd">
<fontconfig>
  <!-- Prioritize Noto Sans Arabic for Arabic text -->
  <match target="pattern">
    <test name="lang" compare="contains">
      <string>ar</string>
    </test>
    <edit name="family" mode="prepend">
      <string>Noto Sans Arabic</string>
    </edit>
  </match>
  
  <!-- Font substitution for missing glyphs -->
  <alias>
    <family>serif</family>
    <prefer>
      <family>Noto Serif</family>
      <family>Noto Sans Arabic</family>
    </prefer>
  </alias>
</fontconfig>
```

**Systemd Service** (for VPS deployment):
```ini
[Unit]
Description=Watiqa-Police Document Management System
After=network.target

[Service]
Type=simple
User=watiqa
Environment="LANG=ar_MA.UTF-8"
Environment="LC_ALL=ar_MA.UTF-8"
ExecStart=/usr/local/bin/moroccan_docs
Restart=on-failure
RestartSec=10s

# Process cleanup
KillMode=mixed
KillSignal=SIGTERM
TimeoutStopSec=30s

# Resource limits
LimitNOFILE=4096
MemoryMax=2G

[Install]
WantedBy=multi-user.target
```

**Validation**:
- [ ] Build Docker image and verify LibreOffice version (`soffice --version`)
- [ ] Test PDF generation inside container
- [ ] Run `fc-list | grep Noto` to confirm fonts loaded
- [ ] Generate test document with "الجمهورية المغربية" and verify PDF rendering

---

### M3.2: Observability & Monitoring (Day 13-14) ✓ 12 hours
**Owner**: Backend Engineer

**Structured Logging**:
```rust
// File: src/main.rs
use tracing_subscriber::{fmt, EnvFilter};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("watiqa_police=debug".parse().unwrap())
                .add_directive("tower_http=info".parse().unwrap())
        )
        .json() // Structured logs for aggregation
        .init();
    
    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        "Starting Watiqa-Police"
    );
    
    // ... app startup
}
```

**Request Instrumentation**:
```rust
// File: src/gui/app.rs
use tracing::{instrument, info, error};

#[instrument(skip(self), fields(doc_type = %self.metadata.doc_type))]
async fn generate_document(&mut self) {
    let start = std::time::Instant::now();
    
    match self.document_generator.create_document("docs/").await {
        Ok(path) => {
            let duration = start.elapsed();
            tracing::info!(
                duration_ms = duration.as_millis(),
                output_path = %path,
                "Document generated successfully"
            );
        }
        Err(e) => {
            tracing::error!(
                error = %e,
                duration_ms = start.elapsed().as_millis(),
                "Document generation failed"
            );
        }
    }
}
```

**Metrics Endpoints** (if adding HTTP API):
```rust
// File: src/api/metrics.rs
use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Metrics {
    documents_generated: u64,
    conversion_success_rate: f64,
    avg_generation_time_ms: f64,
    libreoffice_queue_depth: usize,
}

async fn metrics_handler() -> Json<Metrics> {
    // Query from global stats
    Json(Metrics {
        documents_generated: TOTAL_DOCS.load(Ordering::Relaxed),
        // ... other metrics
    })
}

pub fn routes() -> Router {
    Router::new().route("/metrics", get(metrics_handler))
}
```

**Dependencies**:
```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json", "env-filter"] }
```

---

### M3.3: Final Testing & Documentation (Day 15) ✓ 8 hours
**Owner**: QA + Tech Writer

**Test Scenarios**:
1. **Stress Test**: 20 concurrent document requests
2. **Arabic Rendering**: Generate all 7 document types with complex Arabic (Quranic verses)
3. **Edge Cases**:
   - Empty optional fields (phone, ICE)
   - Maximum length strings (200 chars in trade name)
   - Special characters: `@#$%&*`
4. **Recovery Test**:
   - Kill LibreOffice mid-conversion
   - Fill disk space to 95%
   - Simulate network timeout

**Documentation to Update**:
- [ ] `README.md`: Add Docker deployment instructions
- [ ] `getting-started.md`: Update with template management
- [ ] `API.md`: Document error codes and responses
- [ ] Create `DEPLOYMENT.md` with production checklist

**Deployment Checklist**:
```markdown
## Pre-Deployment Checklist
- [ ] All P0 issues resolved
- [ ] 95%+ test coverage on critical paths
- [ ] Arabic rendering validated with native speaker
- [ ] Docker image tested on production-like hardware
- [ ] Backup strategy for `police_administrative.xlsx`
- [ ] Rollback plan documented
- [ ] Monitoring dashboards configured
- [ ] On-call rotation scheduled
```

---

## Dependencies & Crate Additions

```toml
[dependencies]
# Existing
docx-rs = "0.4"
calamine = "0.25"
rust_xlsxwriter = "0.76"
uuid = "1.10"
chrono = "0.4"
eframe = "0.29"
anyhow = "1.0"
thiserror = "2.0"

# NEW - Templating
zip = "0.6"
quick-xml = "0.31"

# NEW - Async Runtime
tokio = { version = "1", features = ["full"] }
tokio-util = "0.7"

# NEW - Concurrency
once_cell = "1.19"

# NEW - Validation
validator = { version = "0.16", features = ["derive"] }
regex = "1.10"
lazy_static = "1.4"
unicode-normalization = "0.1"

# NEW - Observability
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json", "env-filter"] }

# NEW - Testing
tempfile = "3.8"

[dev-dependencies]
criterion = "0.5" # For benchmarking
proptest = "1.4"  # Property-based testing
```

---

## Success Criteria

| Milestone | Completion Criteria | Target Date |
|-----------|---------------------|-------------|
| M1 (Week 1) | ✅ Arabic renders correctly<br>✅ Template system working<br>✅ No concurrency crashes | Day 5 |
| M2 (Week 2) | ✅ Input validation passes security scan<br>✅ Error messages user-friendly<br>✅ No memory leaks in 1000-doc test | Day 10 |
| M3 (Week 3) | ✅ Docker deployment tested<br>✅ <3s response time under load<br>✅ Documentation complete | Day 15 |

**Final Readiness Score**: 85/100  
**Production Deployment**: Week 4 (soft launch with 5 users)

---

## Risk Mitigation

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| LibreOffice version incompatibility | Medium | High | Pin exact version in Dockerfile, test on staging |
| Arabic font licensing issues | Low | High | Use SIL Open Font License (Noto), verify compliance |
| Performance degradation under load | Medium | Medium | Implement circuit breaker, add autoscaling |
| Data loss in Excel writes | Low | Critical | Add transaction log, hourly backups |

---

## Team Allocation

- **Backend Engineer** (1 FTE): 100 hours
- **DevOps Engineer** (0.3 FTE): 16 hours
- **QA Tester** (0.1 FTE): 8 hours

**Total**: 124 hours over 3 weeks

---

## Post-Production Monitoring

**Week 1 Metrics**:
- Response time P95 < 3s
- Error rate < 1%
- Arabic rendering complaints = 0

**Week 2 Review**:
- Collect user feedback
- Analyze logs for unexpected errors
- Tune LibreOffice pool size based on usage patterns

**Week 4 Go/No-Go**:
- Scale to 50 users if metrics green
- Otherwise, roll back and iterate
