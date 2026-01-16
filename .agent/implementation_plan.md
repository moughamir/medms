# Watiqa Implementation Roadmap

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

---

## Phase 2: Desktop UI (Weeks 5-7)

### Week 5: Authentication Screens
- [ ] Login form with TOTP input
- [ ] TOTP setup wizard (QR display)
- [ ] Backup codes download/print
- [ ] Session management (JWT in Tauri secure store)
- [ ] Auto-logout after 15min inactivity

### Week 6: Core Workflows
- [ ] Bureau d'Ordre form (register documents)
- [ ] Service dashboard (assigned docs)
- [ ] State transition buttons with confirmation
- [ ] Document metadata editor
- [ ] Arabic RTL support (Tailwind `dir="rtl"`)

### Week 7: Document Viewer
- [ ] PDF preview with `react-pdf`
- [ ] Search with highlighting
- [ ] Print dialog
- [ ] Export to USB drive
- [ ] Folder access button (Open `docs/` in file manager)

**Milestone 2 Deliverable**: Functional desktop app (Windows + Linux builds)

---

## Phase 3: Mobile Integration (Weeks 8-10)

### Week 8: Android Setup
- [ ] Configure Tauri Mobile plugin
- [ ] Setup Android Gradle project
- [ ] Integrate Camera API (for OCR)
- [ ] Add Biometric authentication (fingerprint)
- [ ] Test on physical device

### Week 9: Mobile Features
- [ ] TOTP QR scanner (parse `otpauth://` URI)
- [ ] Offline document viewer
- [ ] Camera-based document capture
- [ ] OCR with Tesseract (Arabic + French)
- [ ] Sync status indicator

### Week 10: USB Sync
- [ ] USB device detection on Android
- [ ] Export sync bundle to USB
- [ ] Import from USB with conflict resolution
- [ ] Progress notifications
- [ ] Sync history log

**Milestone 3 Deliverable**: Android APK with sync capability

---

## Phase 4: Security Hardening (Week 11)

### Security Implementation
- [ ] Implement constant-time comparison for TOTP codes
- [ ] Add rate limiting (5 failed attempts = 15min lockout)
- [ ] Digital signature for PDFs (Ed25519)
- [ ] Certificate pinning (if cloud sync added)
- [ ] ProGuard obfuscation (Android)
- [ ] Root detection (SafetyNet)

### License System
- [ ] HMAC-SHA256 license key generation
- [ ] Hardware binding (machine ID hash)
- [ ] Online activation API (optional)
- [ ] Grace period handling
- [ ] License renewal workflow

**Milestone 4 Deliverable**: Penetration test report + fixes

---

## Phase 5: Testing (Week 12)

### Unit Tests (Target: 85% coverage)
- [ ] TOTP generation/validation
- [ ] FSM state transitions
- [ ] Template injection
- [ ] SQLite CRUD operations
- [ ] Base32 encoding/decoding

### Integration Tests
- [ ] End-to-end document workflow
- [ ] Multi-user concurrent editing
- [ ] USB sync roundtrip
- [ ] Authentication flows
- [ ] PDF generation with 1000+ docs

### User Acceptance Testing
- [ ] On-site testing at Bureau d'Ordre
- [ ] Mobile device testing (3+ Android versions)
- [ ] Training materials (screenshots + videos)
- [ ] Bug fixes from UAT feedback

**Milestone 5 Deliverable**: UAT sign-off document

---

## Phase 6: Deployment (Weeks 13-14)

### Week 13: Production Build
- [ ] Create release builds (desktop: Windows .msi, Linux .deb)
- [ ] Sign Android APK with production keystore
- [ ] Bundle fonts + templates in resources
- [ ] Create installer with auto-update mechanism
- [ ] Prepare offline documentation (PDF)

### Week 14: Rollout
- [ ] Install on 3 workstations (pilot)
- [ ] Configure TOTP for 5 users
- [ ] Generate office-specific license keys
- [ ] Import existing Excel data
- [ ] On-site training (2 days)

**Final Deliverable**: Installed system + source code escrow + 6-month warranty

---

## Post-Launch (Months 2-6)

### Support Plan
- [ ] Weekly check-ins (first month)
- [ ] Bug hotfixes within 48h
- [ ] Monthly usage reports
- [ ] Feature requests triage
- [ ] Security patches

### Phase 2 Features (Optional)
- [ ] Cloud sync via Supabase
- [ ] Web portal (view-only for citizens)
- [ ] E-signature integration
- [ ] Bulk import from scanned PDFs
- [ ] Analytics dashboard

---

## Technical Debt Prevention

### Code Quality Gates
- All PRs require:
  - [ ] Passing CI/CD (tests + clippy)
  - [ ] Security audit (`cargo audit`)
  - [ ] Documentation updates
  - [ ] Performance benchmarks (if critical path)

### Documentation
- [ ] API documentation (`cargo doc`)
- [ ] User manual (Arabic + French)
- [ ] Admin guide (installation + backup)
- [ ] Developer guide (architecture + patterns)

---

## Risk Mitigation Strategy

| Risk | Likelihood | Mitigation |
|------|-----------|------------|
| LibreOffice rendering bugs | High | Bundle tested LO 7.6; fallback to Gotenberg |
| Arabic font issues | Medium | Include Noto fonts in APK/installer |
| USB sync conflicts | Medium | CRDT + LWW with office ID tiebreaker |
| License key leaks | Low | Hardware binding + online activation |
| Mobile performance | Medium | Lazy loading + pagination (50 docs/page) |

---

## Success Metrics

### Technical KPIs
- Document generation: < 2 seconds (95th percentile)
- App startup time: < 1 second
- Search latency: < 200ms for 10,000 docs
- Crash rate: < 0.1%
- Battery usage: < 5% per hour (mobile)

### Business KPIs
- User adoption: 80% within 3 months
- Support tickets: < 5 per month after Month 3
- Document processing time: 50% reduction vs paper
- Compliance rate: 100% (all docs digitally signed)

---

## Legal Checklist

### Contracts
- [ ] NDA signed by both parties
- [ ] Development contract with acceptance criteria
- [ ] Devis/quote approved
- [ ] Source code escrow agreement
- [ ] Maintenance SLA defined

### Compliance
- [ ] Dahir 09-08 data protection compliance
- [ ] ISO 27001 security policies documented
- [ ] WCAG 2.1 accessibility review (future web portal)
- [ ] GDPR considerations (if EU citizen data)

---

## Budget Breakdown (Detailed)

| Task | Hours | Rate/hr | Subtotal |
|------|-------|---------|----------|
| **Backend Development** | | | |
| FSM + TOTP | 40 | 500 | 20,000 |
| Database layer | 40 | 500 | 20,000 |
| Document engine | 50 | 500 | 25,000 |
| Mobile APIs | 30 | 500 | 15,000 |
| **Frontend Development** | | | |
| Desktop UI | 80 | 400 | 32,000 |
| Mobile UI | 40 | 400 | 16,000 |
| **Testing** | | | |
| Unit + Integration | 40 | 350 | 14,000 |
| UAT + Bug fixes | 40 | 350 | 14,000 |
| **Deployment** | | | |
| Packaging + Training | 40 | 400 | 16,000 |
| **Project Management** | 20 | 500 | 10,000 |
| **Subtotal** | **420h** | | **182,000** |
| Contingency (15%) | | | 27,300 |
| **Total HT** | | | **209,300** |
| TVA (20%) | | | 41,860 |
| **Total TTC** | | | **251,160** |

### Payment Schedule
1. **30% upfront** (75,348 MAD) - upon contract signature
2. **40% at Milestone 3** (100,464 MAD) - Android APK delivery
3. **30% at final delivery** (75,348 MAD) - after UAT sign-off

---

## Next Immediate Actions

1. **This Week**: Schedule kickoff meeting with Commune IT department
2. **Day 1**: Sign NDA + collect requirements (document templates)
3. **Day 3**: Setup development environment (repo + CI/CD)
4. **Day 5**: Deliver Phase 1 prototype (TOTP + template injection demo)

---

## Contact & Escalation

**Technical Lead**: [Your Name]  
**Project Manager**: [PM Name]  
**Client POC**: Commune de Bouskoura IT Director  
**Escalation Path**: Technical issues → PM → Client POC → Commune President

**Communication Channels**:
- Daily standup: 9:00 AM (Signal group)
- Weekly review: Fridays 2:00 PM (on-site or Zoom)
- Emergency hotline: [Phone] (24/7 for critical bugs)

---

*Document Version: 1.0*  
*Last Updated: 2026-01-15*  
*Status: Awaiting Client Approval*
