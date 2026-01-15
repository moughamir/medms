//! Database queries for statistics and reporting

use crate::database::ExcelDatabase;
use crate::models::{DashboardStats, SummaryCard};
use anyhow::Result;
use std::collections::HashMap;

impl ExcelDatabase {
    /// Computes dashboard statistics from all data
    pub fn get_dashboard_stats(&self) -> Result<DashboardStats> {
        let commerces = self.get_all_commerces()?;
        let inspections = self.get_all_inspections()?;
        let documents = self.get_all_documents()?;
        let actions = self.get_all_actions()?;

        let mut stats = DashboardStats::new();

        // Basic counts
        stats.total_commerces = commerces.len();
        stats.total_inspections = inspections.len();
        stats.total_documents = documents.len();

        // Commerces without authorization
        stats.commerces_sans_autorisation = commerces.iter().filter(|c| !c.has_autorisation).count();

        // Violations by type (from inspections)
        for inspection in &inspections {
            for violation in &inspection.violation_types {
                *stats
                    .violations_by_type
                    .entry(violation.to_arabic().to_string())
                    .or_insert(0) += 1;
            }
        }

        // Actions by status
        for action in &actions {
            *stats
                .actions_by_status
                .entry(action.status.to_arabic().to_string())
                .or_insert(0) += 1;
        }

        // Documents by type
        for doc in &documents {
            *stats
                .documents_by_type
                .entry(doc.doc_type.to_arabic().to_string())
                .or_insert(0) += 1;
        }

        // Monthly activity (documents per month)
        let mut monthly_counts: HashMap<String, usize> = HashMap::new();
        for doc in &documents {
            if doc.creation_date.len() >= 7 {
                let month = &doc.creation_date[..7]; // YYYY-MM
                *monthly_counts.entry(month.to_string()).or_insert(0) += 1;
            }
        }
        let mut monthly_vec: Vec<_> = monthly_counts.into_iter().collect();
        monthly_vec.sort_by(|a, b| a.0.cmp(&b.0));
        stats.monthly_activity = monthly_vec;

        // Fine statistics
        stats.total_fines_issued = actions.iter().map(|a| a.fine_amount).sum();
        stats.total_fines_collected = actions
            .iter()
            .filter(|a| a.status.to_arabic() == "الغرامة مدفوعة")
            .map(|a| a.fine_amount)
            .sum();
        stats.pending_fines = stats.total_fines_issued - stats.total_fines_collected;

        // Overdue actions
        stats.overdue_actions = actions.iter().filter(|a| a.is_overdue()).count();

        Ok(stats)
    }

    /// Gets summary cards for the dashboard
    pub fn get_summary_cards(&self) -> Result<Vec<SummaryCard>> {
        let stats = self.get_dashboard_stats()?;

        let cards = vec![
            SummaryCard::new(
                "المحلات التجارية",
                "Commerces",
                stats.total_commerces.to_string(),
                '🏪',
            ),
            SummaryCard::new(
                "المعاينات",
                "Inspections",
                stats.total_inspections.to_string(),
                '🔍',
            ),
            SummaryCard::new(
                "الوثائق",
                "Documents",
                stats.total_documents.to_string(),
                '📄',
            ),
            SummaryCard::new(
                "بدون رخصة",
                "Sans Autorisation",
                stats.commerces_sans_autorisation.to_string(),
                '⚠',
            ),
            SummaryCard::new(
                "الغرامات المستحقة",
                "Amendes dues",
                format!("{:.2} DH", stats.pending_fines),
                '💰',
            ),
            SummaryCard::new(
                "إجراءات متأخرة",
                "Actions en retard",
                stats.overdue_actions.to_string(),
                '⏰',
            ),
        ];

        Ok(cards)
    }

    /// Searches commerces by denomination or owner name
    pub fn search_commerces(&self, query: &str) -> Result<Vec<crate::models::CommerceInfo>> {
        let all = self.get_all_commerces()?;
        let query_lower = query.to_lowercase();

        Ok(all
            .into_iter()
            .filter(|c| {
                c.denomination.to_lowercase().contains(&query_lower)
                    || c.owner_name.to_lowercase().contains(&query_lower)
                    || c.quartier.to_lowercase().contains(&query_lower)
            })
            .collect())
    }

    /// Gets recent documents (last N)
    pub fn get_recent_documents(&self, limit: usize) -> Result<Vec<crate::models::DocumentMetadata>> {
        let mut docs = self.get_all_documents()?;
        docs.sort_by(|a, b| {
            let a_dt = format!("{} {}", a.creation_date, a.creation_time);
            let b_dt = format!("{} {}", b.creation_date, b.creation_time);
            b_dt.cmp(&a_dt)
        });
        Ok(docs.into_iter().take(limit).collect())
    }

    /// Gets documents by commerce ID
    pub fn get_documents_by_commerce(&self, commerce_id: &str) -> Result<Vec<crate::models::DocumentMetadata>> {
        let docs = self.get_all_documents()?;
        Ok(docs.into_iter().filter(|d| d.commerce_id == commerce_id).collect())
    }

    /// Gets inspections by commerce ID
    pub fn get_inspections_by_commerce(&self, commerce_id: &str) -> Result<Vec<crate::models::InspectionRecord>> {
        let inspections = self.get_all_inspections()?;
        Ok(inspections.into_iter().filter(|i| i.commerce_id == commerce_id).collect())
    }

    /// Gets actions by commerce ID
    pub fn get_actions_by_commerce(&self, commerce_id: &str) -> Result<Vec<crate::models::EnforcementAction>> {
        let actions = self.get_all_actions()?;
        Ok(actions.into_iter().filter(|a| a.commerce_id == commerce_id).collect())
    }
}
