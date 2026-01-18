-- Create Users table
CREATE TABLE IF NOT EXISTS users (
  id TEXT PRIMARY KEY NOT NULL,
  -- UUID
  username TEXT NOT NULL UNIQUE,
  password_hash TEXT,
  -- Nullable if using only TOTP
  totp_secret TEXT,
  totp_enabled BOOLEAN DEFAULT 0,
  backup_codes TEXT,
  -- JSON array
  role TEXT DEFAULT 'inspector',
  -- admin, inspector
  created_at TEXT DEFAULT CURRENT_TIMESTAMP,
  last_login TEXT
);
-- Create Commerces table (Businesses)
CREATE TABLE IF NOT EXISTS commerces (
  id TEXT PRIMARY KEY NOT NULL,
  -- UUID
  name TEXT NOT NULL,
  address TEXT,
  city TEXT,
  commune TEXT,
  arrondissement TEXT,
  owner_name TEXT,
  cin TEXT,
  -- National ID
  phone TEXT,
  patente TEXT,
  -- Tax ID
  activity_type TEXT,
  status TEXT NOT NULL DEFAULT 'active',
  -- active, closed, suspended
  created_at TEXT DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);
-- Create Inspections table (Procès-Verbal)
CREATE TABLE IF NOT EXISTS inspections (
  id TEXT PRIMARY KEY NOT NULL,
  -- UUID
  commerce_id TEXT NOT NULL,
  inspector_id TEXT NOT NULL,
  inspection_date TEXT DEFAULT CURRENT_TIMESTAMP,
  report_number TEXT NOT NULL UNIQUE,
  -- PV Number e.g. 2024/001
  summary TEXT,
  status TEXT NOT NULL DEFAULT 'draft',
  -- draft, submitted, archived
  FOREIGN KEY (commerce_id) REFERENCES commerces(id),
  FOREIGN KEY (inspector_id) REFERENCES users(id)
);
-- Create Violations table
CREATE TABLE IF NOT EXISTS violations (
  id TEXT PRIMARY KEY NOT NULL,
  -- UUID
  inspection_id TEXT NOT NULL,
  violation_code TEXT NOT NULL,
  description TEXT,
  severity TEXT NOT NULL DEFAULT 'medium',
  -- low, medium, high, critical
  measure_taken TEXT,
  -- warning, fine, closure
  FOREIGN KEY (inspection_id) REFERENCES inspections(id) ON DELETE CASCADE
);