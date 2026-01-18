use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Inspection {
    pub id: String,
    pub commerce_id: String,
    pub inspector_id: String,
    pub inspection_date: Option<NaiveDateTime>,
    pub report_number: String,
    pub summary: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Violation {
    pub id: String,
    pub inspection_id: String,
    pub violation_code: String,
    pub description: Option<String>,
    pub severity: String,
    pub measure_taken: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInspection {
    pub commerce_id: String,
    pub inspector_id: String,
    pub report_number: String,
    pub summary: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateViolation {
    pub inspection_id: String,
    pub violation_code: String,
    pub description: Option<String>,
    pub severity: String,
    pub measure_taken: Option<String>,
}
