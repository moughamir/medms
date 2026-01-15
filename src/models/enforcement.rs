//! Enforcement actions and status tracking

use crate::models::DocumentType;
use serde::{Deserialize, Serialize};

/// Status of an enforcement action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionStatus {
    /// جاري - In progress
    EnCours,
    /// متوافق - Compliant
    Conforme,
    /// غير متوافق - Non-compliant
    NonConforme,
    /// مغلق - Closed
    Ferme,
    /// الغرامة مدفوعة - Fine paid
    AmendePaye,
    /// نزاع قضائي - Legal dispute
    ContentieuxJudiciaire,
}

impl ActionStatus {
    /// Returns the Arabic name for the status
    pub fn to_arabic(&self) -> &str {
        match self {
            ActionStatus::EnCours => "جاري",
            ActionStatus::Conforme => "متوافق",
            ActionStatus::NonConforme => "غير متوافق",
            ActionStatus::Ferme => "مغلق",
            ActionStatus::AmendePaye => "الغرامة مدفوعة",
            ActionStatus::ContentieuxJudiciaire => "نزاع قضائي",
        }
    }

    /// Returns the French name for the status
    pub fn to_french(&self) -> &str {
        match self {
            ActionStatus::EnCours => "En cours",
            ActionStatus::Conforme => "Conforme",
            ActionStatus::NonConforme => "Non conforme",
            ActionStatus::Ferme => "Fermé",
            ActionStatus::AmendePaye => "Amende payée",
            ActionStatus::ContentieuxJudiciaire => "Contentieux judiciaire",
        }
    }

    /// Returns all statuses for UI selection
    pub fn all() -> Vec<ActionStatus> {
        vec![
            ActionStatus::EnCours,
            ActionStatus::Conforme,
            ActionStatus::NonConforme,
            ActionStatus::Ferme,
            ActionStatus::AmendePaye,
            ActionStatus::ContentieuxJudiciaire,
        ]
    }

    /// Returns true if this is a final/closed status
    pub fn is_final(&self) -> bool {
        matches!(
            self,
            ActionStatus::Conforme
                | ActionStatus::Ferme
                | ActionStatus::AmendePaye
                | ActionStatus::ContentieuxJudiciaire
        )
    }
}

impl std::fmt::Display for ActionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.to_arabic(), self.to_french())
    }
}

impl Default for ActionStatus {
    fn default() -> Self {
        ActionStatus::EnCours
    }
}

/// An enforcement action taken against a commerce
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementAction {
    /// Unique action identifier
    pub action_id: String,
    /// Associated commerce ID
    pub commerce_id: String,
    /// Associated inspection ID
    pub inspection_id: String,
    /// Type of action/document
    pub action_type: DocumentType,
    /// Action date (YYYY-MM-DD)
    pub action_date: String,
    /// تاريخ الأجل - Deadline date
    pub deadline_date: String,
    /// مبلغ الغرامة - Fine amount in DH
    pub fine_amount: f64,
    /// Current status
    pub status: ActionStatus,
    /// تاريخ المتابعة - Follow-up date
    pub follow_up_date: String,
    /// Additional notes
    pub notes: String,
}

impl EnforcementAction {
    /// Creates a new EnforcementAction with generated ID and current date
    pub fn new(commerce_id: String, inspection_id: String, action_type: DocumentType) -> Self {
        let now = chrono::Local::now();
        Self {
            action_id: uuid::Uuid::new_v4().to_string(),
            commerce_id,
            inspection_id,
            action_type,
            action_date: now.format("%Y-%m-%d").to_string(),
            deadline_date: String::new(),
            fine_amount: 0.0,
            status: ActionStatus::EnCours,
            follow_up_date: String::new(),
            notes: String::new(),
        }
    }

    /// Returns true if the action has a fine amount
    pub fn has_fine(&self) -> bool {
        self.fine_amount > 0.0
    }

    /// Returns true if the deadline has passed
    pub fn is_overdue(&self) -> bool {
        if self.deadline_date.is_empty() {
            return false;
        }
        if let Ok(deadline) = chrono::NaiveDate::parse_from_str(&self.deadline_date, "%Y-%m-%d") {
            let today = chrono::Local::now().date_naive();
            return deadline < today && !self.status.is_final();
        }
        false
    }
}

impl Default for EnforcementAction {
    fn default() -> Self {
        Self::new(String::new(), String::new(), DocumentType::ProcesVerbal)
    }
}
