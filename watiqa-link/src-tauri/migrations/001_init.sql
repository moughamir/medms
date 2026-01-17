-- Users with TOTP
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    full_name TEXT NOT NULL,
    role TEXT NOT NULL CHECK(role IN ('admin', 'agent', 'viewer')),
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
    state TEXT NOT NULL CHECK(
        state IN (
            'Registered',
            'Dispatched',
            'Annotated',
            'Signed',
            'Archived'
        )
    ),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
-- Full-text search
CREATE VIRTUAL TABLE documents_fts USING fts5(
    numero_ordre,
    expediteur,
    objet,
    service_concerne,
    content = 'documents',
    content_rowid = 'rowid'
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
    FOREIGN KEY (doc_uuid) REFERENCES documents(uuid) ON DELETE CASCADE
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
INSERT INTO users (id, username, password_hash, full_name, role)
VALUES (
        'admin',
        'admin',
        '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYWJ3.VQo6a',
        'Administrateur',
        'admin'
    );