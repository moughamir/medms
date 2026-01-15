//! Inspection form state

use crate::models::ViolationType;

/// Form state for adding/editing an inspection
#[derive(Debug, Clone)]
pub struct InspectionForm {
    pub inspector_name: String,
    pub inspector_grade: String,
    pub commerce_id: String,
    pub commerce_name: String,
    pub selected_violation: ViolationType,
    pub additional_violations: Vec<ViolationType>,
    pub description: String,
    pub measures_taken: String,
    pub witnesses: Vec<String>,
    pub photo_paths: Vec<String>,
    /// For edit mode
    pub editing_id: Option<String>,
}

impl Default for InspectionForm {
    fn default() -> Self {
        Self {
            inspector_name: String::new(),
            inspector_grade: String::new(),
            commerce_id: String::new(),
            commerce_name: String::new(),
            selected_violation: ViolationType::SansAutorisation,
            additional_violations: Vec::new(),
            description: String::new(),
            measures_taken: String::new(),
            witnesses: Vec::new(),
            photo_paths: Vec::new(),
            editing_id: None,
        }
    }
}

impl InspectionForm {
    pub fn new() -> Self {
        Self::default()
    }

    /// Clears all form fields
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// Gets all selected violations
    pub fn all_violations(&self) -> Vec<ViolationType> {
        let mut violations = vec![self.selected_violation.clone()];
        violations.extend(self.additional_violations.clone());
        violations
    }

    /// Validates the form
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.inspector_name.trim().is_empty() {
            errors.push("اسم العون مطلوب".to_string());
        }
        if self.description.trim().is_empty() {
            errors.push("وصف المخالفة مطلوب".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
