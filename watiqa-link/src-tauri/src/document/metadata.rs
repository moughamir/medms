use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub uuid: Uuid,
    pub numero_ordre: String,
    pub date_arrivee: String,
    pub expediteur: String,
    pub cin_expediteur: Option<String>,
    pub objet: String,
    pub service_concerne: String,
    pub priorite: PriorityLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriorityLevel {
    Urgent,
    Normal,
    Low,
}

impl DocumentMetadata {
    pub fn new(
        numero_ordre: String,
        expediteur: String,
        objet: String,
        service_concerne: String,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            numero_ordre,
            date_arrivee: chrono::Local::now().format("%Y-%m-%d").to_string(),
            expediteur,
            cin_expediteur: None,
            objet,
            service_concerne,
            priorite: PriorityLevel::Normal,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
