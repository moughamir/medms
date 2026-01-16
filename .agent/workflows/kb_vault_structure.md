# Bouskoura Administrative Knowledge Base - Vault Structure

## Directory Architecture

```
Bouskoura-Admin-KB/
│
├── 00-Inbox/                          # Capture point for unprocessed items
│   ├── _README.md
│   └── .gitkeep
│
├── 01-Notes/
│   ├── 010-Periodic/                  # Time-based notes
│   │   ├── Daily/
│   │   ├── Weekly/
│   │   ├── Monthly/
│   │   └── Quarterly/
│   │
│   └── 020-Unique/                    # Standalone notes
│       ├── Fleeting/                  # Quick captures
│       └── Literature/                # Reference notes
│
├── 02-PARA/
│   ├── 201-Projects/                  # Active, time-bound initiatives
│   │   ├── KB-Initial-Setup/
│   │   ├── Agent-Onboarding-Q1-2026/
│   │   └── GED-Implementation/
│   │
│   ├── 202-Areas/                     # Ongoing responsibilities
│   │   ├── Training/
│   │   │   ├── Trainee-Caid-Deputy/
│   │   │   ├── Auxiliary-Agents/
│   │   │   └── Administrative-Staff/
│   │   │
│   │   ├── Operations/
│   │   │   ├── Complaints-Management/
│   │   │   ├── Field-Inspections/
│   │   │   └── Citizen-Services/
│   │   │
│   │   ├── Compliance/
│   │   │   ├── Legal-Framework/
│   │   │   ├── Procedures/
│   │   │   └── Audit-Matrices/
│   │   │
│   │   └── Communication/
│   │       ├── Internal-Correspondence/
│   │       ├── External-Correspondence/
│   │       └── Templates/
│   │
│   ├── 203-Resources/                 # Reference materials
│   │   ├── 03099-System/
│   │   │   ├── 00-Templates/
│   │   │   │   ├── T_Daily-Note.md
│   │   │   │   ├── T_Complaint-Registration.md
│   │   │   │   ├── T_Field-Report.md
│   │   │   │   ├── T_Correspondence.md
│   │   │   │   ├── T_Training-Session.md
│   │   │   │   └── T_Zettel.md
│   │   │   │
│   │   │   ├── 01-Guides/
│   │   │   │   ├── Administrative-Attache-Guide.md
│   │   │   │   ├── Trainee-Caid-Deputy-Guide.md
│   │   │   │   ├── Workflows-Master.md
│   │   │   │   └── GED-Implementation.md
│   │   │   │
│   │   │   └── 02-Frameworks/
│   │   │       ├── Legal-References.md
│   │   │       ├── RACI-Matrices.md
│   │   │       └── KPIs.md
│   │   │
│   │   ├── Field-Cards/               # A6/A5 reference cards
│   │   │   ├── Daily-Tasks-Card.md
│   │   │   ├── Complaint-Workflow-Card.md
│   │   │   └── Emergency-Contacts-Card.md
│   │   │
│   │   └── Document-Library/
│   │       ├── Forms/
│   │       ├── Legal-Texts/
│   │       └── Reference-PDFs/
│   │
│   └── 204-Archive/                   # Completed/inactive items
│       ├── Projects/
│       └── Obsolete-Procedures/
│
├── 03-Maps-of-Content/                # Navigation hubs
│   ├── MOC-Training.md
│   ├── MOC-Procedures.md
│   ├── MOC-Legal-Framework.md
│   └── MOC-Bouskoura-Context.md
│
├── 04-Attachments/                    # Binary files
│   ├── images/
│   ├── pdfs/
│   └── forms/
│
└── .obsidian/                         # Vault configuration
    ├── plugins/
    │   ├── templater/
    │   ├── dataview/
    │   ├── obsidian-tasks/
    │   └── local-rest-api/
    │
    ├── themes/
    │   └── Minimalist Paradise/
    │
    └── workspace.json
```

## Core Structural Principles

### 1. Information Flow
```
Capture (00-Inbox) 
  → Process (Sort & Tag)
    → Organize (02-PARA appropriate folder)
      → Connect (Link & Map)
        → Review (Periodic notes)
```

### 2. Naming Conventions

#### Files
- **Format**: `Type_Context_Descriptor.md`
- **Examples**: 
  - `Guide_Administrative-Attache_Definition.md`
  - `Workflow_Complaint_Registration.md`
  - `Template_Field-Report.md`

#### Unique IDs
- All notes include ULID in frontmatter
- Format: `id: 01KEYDMRCQHH65EDZ1RNGK6PGD`

#### Dates
- ISO 8601 format: `2026-01-14T15:05`
- Arabic context: `14 يناير 2026`

### 3. Tagging System

```yaml
# Hierarchical tags
tags:
  - training/caid-deputy
  - training/auxiliary-agent
  - procedure/complaint
  - legal/framework
  - area/bouskoura
  - status/active
  - lang/ar
  - lang/fr
  - priority/high
```

### 4. Frontmatter Template

```yaml
---
id: [ULID]
created: [ISO-datetime]
updated: [ISO-datetime]
type: [guide|workflow|template|reference]
area: [training|operations|compliance|communication]
status: [draft|active|archived]
language: [ar|fr|bilingual]
tags: []
aliases: []
---
```

## Key Features Integration

### Templater Configuration
- Location: `.obsidian/plugins/templater/`
- Templates stored in: `02-PARA/203-Resources/03099-System/00-Templates/`
- Hotkeys configured for rapid note creation

### Dataview Queries
Strategic dashboards for:
- Training progress tracking
- Complaint status overview
- Pending tasks by area
- Recent updates feed

### Obsidian Tasks
- Global task management across all notes
- Due dates, priorities, recurrence
- Query by area, person, or deadline

### Local REST API (MCP Integration)
- Endpoint: `http://localhost:27123`
- Secure token-based access
- Read/write permissions configured
- Enables AI-assisted workflows

## Bilingual Support (Arabic/French)

### File Organization
- Each guide exists in both languages
- Naming: `Guide_Topic_AR.md` and `Guide_Topic_FR.md`
- Cross-linked with aliases

### Example Structure
```
202-Areas/Training/Trainee-Caid-Deputy/
├── 01_تعريف_المنصب.md
├── 01_Definition-du-Poste.md
├── 02_المبادئ_المهنية.md
├── 02_Principes-Professionnels.md
...
```

## Bouskoura-Specific Context

### Geographic Metadata
```yaml
location:
  district: Bouskoura
  province: Nouaceur
  region: Casablanca-Settat
  coordinates: [33.4542, -7.6500]
```

### Local Hierarchy
```
Wali (Governor) - Casablanca-Settat
  └─ Secretary General
      └─ Pasha / District Chief - Nouaceur
          └─ Caid - Bouskoura
              ├─ Caid Deputy (Khalifa)
              │   └─ Trainee Caid Deputy
              └─ Administrative Attaché
                  └─ Auxiliary Agents
```

## Workflow Integration Points

### 1. Daily Operations
- Morning checklist template
- Correspondence registration
- Complaint intake
- Field inspection reports
- End-of-day summary

### 2. Training Cycles
- Onboarding checklists (Days 1-7-30-90)
- Competency assessments
- Progress tracking
- Certification readiness

### 3. Compliance & Audit
- Document retention schedules
- Audit trail maintenance
- Risk matrices
- Inspection preparation

## Next Steps for Implementation

1. **Phase 1 - Foundation (Week 1)**
   - [ ] Create directory structure
   - [ ] Install and configure plugins
   - [ ] Create core templates
   - [ ] Import existing documents

2. **Phase 2 - Content Population (Weeks 2-3)**
   - [ ] Migrate Arabic guides
   - [ ] Create French translations
   - [ ] Build workflow diagrams
   - [ ] Develop field cards

3. **Phase 3 - Automation (Week 4)**
   - [ ] Configure Dataview dashboards
   - [ ] Set up MCP integration
   - [ ] Create automation scripts
   - [ ] Test AI-assisted workflows

4. **Phase 4 - Training & Rollout (Ongoing)**
   - [ ] Conduct staff training
   - [ ] Gather feedback
   - [ ] Iterate and refine
   - [ ] Establish maintenance protocols

---

**This structure balances:**
- Moroccan administrative hierarchy requirements
- P.A.R.A. methodology for clarity
- Bilingual (Arabic/French) operations
- AI/MCP integration readiness
- Field-practical usability for agents

Ready to proceed with detailed templates and workflows?
