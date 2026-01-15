//! Commerce form state

/// Form state for adding/editing a commerce
#[derive(Debug, Clone, Default)]
pub struct CommerceForm {
    pub denomination: String,
    pub owner_name: String,
    pub owner_cin: String,
    pub owner_phone: String,
    pub owner_address: String,
    pub establishment_address: String,
    pub quartier: String,
    pub activity_type: String,
    pub ice_number: String,
    pub patente_number: String,
    pub has_autorisation: bool,
    pub autorisation_number: String,
    pub autorisation_date: String,
    /// For edit mode
    pub editing_id: Option<String>,
}

impl CommerceForm {
    pub fn new() -> Self {
        Self::default()
    }

    /// Clears all form fields
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// Validates the form
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.denomination.trim().is_empty() {
            errors.push("التسمية التجارية مطلوبة".to_string());
        }
        if self.owner_name.trim().is_empty() {
            errors.push("اسم المالك مطلوب".to_string());
        }
        if self.establishment_address.trim().is_empty() {
            errors.push("عنوان المحل مطلوب".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
