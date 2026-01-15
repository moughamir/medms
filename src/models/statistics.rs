//! Dashboard statistics for reporting

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Statistics for the dashboard view
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DashboardStats {
    /// Total number of registered commerces
    pub total_commerces: usize,
    /// Total number of inspections conducted
    pub total_inspections: usize,
    /// Total number of documents generated
    pub total_documents: usize,
    /// Count of violations by type (Arabic name -> count)
    pub violations_by_type: HashMap<String, usize>,
    /// Count of actions by status (Arabic name -> count)
    pub actions_by_status: HashMap<String, usize>,
    /// Count of documents by type (Arabic name -> count)
    pub documents_by_type: HashMap<String, usize>,
    /// Monthly activity (YYYY-MM -> count)
    pub monthly_activity: Vec<(String, usize)>,
    /// Total fines issued (in DH)
    pub total_fines_issued: f64,
    /// Total fines collected (in DH)
    pub total_fines_collected: f64,
    /// Pending fines (in DH)
    pub pending_fines: f64,
    /// Count of overdue actions
    pub overdue_actions: usize,
    /// Count of commerces without authorization
    pub commerces_sans_autorisation: usize,
}

impl DashboardStats {
    /// Creates a new empty DashboardStats
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the collection rate as a percentage
    pub fn collection_rate(&self) -> f64 {
        if self.total_fines_issued > 0.0 {
            (self.total_fines_collected / self.total_fines_issued) * 100.0
        } else {
            0.0
        }
    }

    /// Returns the compliance rate as a percentage
    pub fn compliance_rate(&self) -> f64 {
        let conforme = *self.actions_by_status.get("متوافق").unwrap_or(&0);
        let total: usize = self.actions_by_status.values().sum();
        if total > 0 {
            (conforme as f64 / total as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Returns the top N violation types by count
    pub fn top_violations(&self, n: usize) -> Vec<(&String, &usize)> {
        let mut sorted: Vec<_> = self.violations_by_type.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        sorted.into_iter().take(n).collect()
    }
}

/// Summary card data for dashboard display
#[derive(Debug, Clone)]
pub struct SummaryCard {
    pub title_ar: String,
    pub title_fr: String,
    pub value: String,
    pub change: Option<f64>,
    pub icon: char,
}

impl SummaryCard {
    pub fn new(title_ar: &str, title_fr: &str, value: String, icon: char) -> Self {
        Self {
            title_ar: title_ar.to_string(),
            title_fr: title_fr.to_string(),
            value,
            change: None,
            icon,
        }
    }

    pub fn with_change(mut self, change: f64) -> Self {
        self.change = Some(change);
        self
    }
}
