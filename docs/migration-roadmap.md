# Watiqa-Link Migration Roadmap

## Overview

This document outlines the systematic plan to migrate functionality from the legacy Rust/egui application to the new Tauri/React architecture.

## Phase 1: Database Layer 🗄️

**Goal**: Replace Excel persistence with a robust SQLite database.

- [x] **Schema Design**: Define tables for `users`, `commerces`, `inspections`, `violations`.
- [x] **Migrations**: Create SQLx migration files (`migrations/YYYYMMDD_create_tables.sql`).
- [x] **Rust Models**: Create structs in `src-tauri/src/models/` mirroring the database schema.
- [x] **Database Manager**: Implement `db.rs` to handle connection pooling and query execution.

## Phase 2: Commerce Module 🏪

**Goal**: Enable business administrative management.

- [x] **Backend**:
  - `create_commerce(Commerce)`
  - `get_commerce(uuid)`
  - `list_commerces(filter)`
  - `update_commerce(Commerce)`
- [x] **Frontend**:
  - `CommerceList.tsx`: Data table with search/filter.
  - `CommerceForm.tsx`: Form for adding/editing businesses (Bilingual).
  - `CommerceStore`: Zustand store for state management.

## Phase 3: Inspection Module 🔍

**Goal**: Digitize field inspection reports (Procès-Verbal).

- [ ] **Backend**:
  - `create_inspection(Inspection)`
  - `list_inspections_by_commerce(commerce_id)`
  - `add_violation(inspection_id, violation_type)`
- [ ] **Frontend**:
  - `NewInspection.tsx`: Wizard for conducting inspections.
  - `ViolationSelector`: Widget for selecting infractions from a master list.

## Phase 4: Document Generation 📄

**Goal**: Re-integrate the `.docx` generation engine.

- [ ] **Templates**: specialized templates for PV, Mise en Demeure, Avertissement.
- [ ] **Engine**: Update `src-tauri/src/document/` to fetch data from SQLite instead of passed structs.
- [ ] **Frontend**: "Generate Document" button on Inspection details page.

## Phase 5: Dashboard & Analytics 📊

**Goal**: Provide administrative oversight.

- [ ] **Backend**: Aggregation queries (Total fines, inspections per week, etc.).
- [ ] **Frontend**: Dashboard widgets and charts.

## Phase 6: Sync & Mobile 🔄

**Goal**: Support future mobile app data synchronization.

- [ ] **API**: Design JSON export/import format.
- [ ] **Sync Logic**: Conflict resolution strategy (Last-Write-Wins).

---

## Technical Guidelines

- **Git Flow**: Create feature branches (`feature/database`, `feature/commerce`) from `develop` or `feature/to-tauri`.
- **Testing**: Write unit tests for all new Rust commands.
- **Validation**: Use `zod` for frontend form validation.
