//! Commerce/Establishment information

use serde::{Deserialize, Serialize};

/// Information about a commercial establishment
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommerceInfo {
    /// Unique identifier
    pub commerce_id: String,
    /// التسمية التجارية - Trade name
    pub denomination: String,
    /// اسم المالك - Owner name
    pub owner_name: String,
    /// رقم البطاقة الوطنية - National ID (CIN)
    pub owner_cin: String,
    /// رقم الهاتف - Phone number
    pub owner_phone: String,
    /// عنوان المالك - Owner address
    pub owner_address: String,
    /// عنوان المحل - Establishment address
    pub establishment_address: String,
    /// الحي - District/Quarter
    pub quartier: String,
    /// نوع النشاط - Activity type
    pub activity_type: String,
    /// رقم السجل التجاري - ICE number
    pub ice_number: String,
    /// رقم رخصة الباطونطا - Patente number
    pub patente_number: String,
    /// هل يتوفر على رخصة - Has authorization
    pub has_autorisation: bool,
    /// رقم الرخصة - Authorization number
    pub autorisation_number: String,
    /// تاريخ الرخصة - Authorization date
    pub autorisation_date: String,
}

impl CommerceInfo {
    /// Creates a new CommerceInfo with a generated UUID
    pub fn new() -> Self {
        Self {
            commerce_id: uuid::Uuid::new_v4().to_string(),
            ..Default::default()
        }
    }

    /// Validates the commerce information
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.denomination.trim().is_empty() {
            errors.push("التسمية التجارية مطلوبة / Trade name is required".to_string());
        }
        if self.owner_name.trim().is_empty() {
            errors.push("اسم المالك مطلوب / Owner name is required".to_string());
        }
        if self.establishment_address.trim().is_empty() {
            errors.push("عنوان المحل مطلوب / Establishment address is required".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Returns a display-friendly summary
    pub fn summary(&self) -> String {
        format!(
            "{} - {} ({})",
            self.denomination, self.owner_name, self.quartier
        )
    }
}
