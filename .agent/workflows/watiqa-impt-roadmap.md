---
description: Watiqa Implementation Roadmap
---

## Phase 1: Foundation (Weeks 1-4)
### Week 1: Project Setup
- [ ] Initialize Tauri project with React + TypeScript
- [ ] Setup Rust workspace structure
- [ ] Configure SQLite with migrations
- [ ] Install LibreOffice portable (Windows/Linux)
- [ ] Create Base32-encoded TOTP templates
### Week 2: Core Backend
- [ ] Implement FSM (Registered → Dispatched → Annotated → Signed → Archived)
- [ ] TOTP generation with `ring` crate
- [ ] TOTP validation with ±1 window tolerance
- [ ] QR code generation (SVG + PNG)
- [ ] Backup code system (8×8-digit codes)
### Week 3: Database Layer
- [ ] Create `documents`, `users`, `state_transitions` tables
- [ ] Add FTS5 full-text search for Arabic
- [ ] Implement CRUD operations with SQLx
- [ ] Add audit logging with Merkle tree hash chain
- [ ] Setup SQLCipher encryption
### Week 4: Document Engine
- [ ] Template injection (.docx XML manipulation)
- [ ] LibreOffice headless integration
- [ ] PDF/A-3 conversion with font embedding
- [ ] Footer injection (UUID + timestamp)
- [ ] Batch conversion queue with retry logic
**Milestone 1 Deliverable**: Working Rust backend with TOTP auth + document generation

