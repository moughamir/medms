//! Inspection and violation types

use serde::{Deserialize, Serialize};

/// Types of violations that can be recorded
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationType {
    /// بدون رخصة - No permit
    SansAutorisation,
    /// احتلال الطريق العام - Public space occupation
    OccupationVoiePublique,
    /// إزعاج صوتي - Noise nuisance
    NuisancesSonores,
    /// عدم النظافة - Unsanitary conditions
    Insalubrite,
    /// عدم احترام أوقات العمل - Schedule violations
    NonRespectHoraires,
    /// بيع الخمور بدون رخصة - Alcohol without license
    VenteAlcoolSansLicence,
    /// بناء غير قانوني - Illegal construction
    ConstructionIllegale,
    /// دعاية غير مرخصة - Unauthorized advertising
    PubliciteNonAutorisee,
    /// أخرى - Other
    Autre,
}

impl ViolationType {
    /// Returns the Arabic name for the violation type
    pub fn to_arabic(&self) -> &str {
        match self {
            ViolationType::SansAutorisation => "بدون رخصة",
            ViolationType::OccupationVoiePublique => "احتلال الطريق العام",
            ViolationType::NuisancesSonores => "إزعاج صوتي",
            ViolationType::Insalubrite => "عدم النظافة",
            ViolationType::NonRespectHoraires => "عدم احترام أوقات العمل",
            ViolationType::VenteAlcoolSansLicence => "بيع الخمور بدون رخصة",
            ViolationType::ConstructionIllegale => "بناء غير قانوني",
            ViolationType::PubliciteNonAutorisee => "دعاية غير مرخصة",
            ViolationType::Autre => "أخرى",
        }
    }

    /// Returns the French name for the violation type
    pub fn to_french(&self) -> &str {
        match self {
            ViolationType::SansAutorisation => "Sans Autorisation",
            ViolationType::OccupationVoiePublique => "Occupation Voie Publique",
            ViolationType::NuisancesSonores => "Nuisances Sonores",
            ViolationType::Insalubrite => "Insalubrité",
            ViolationType::NonRespectHoraires => "Non-respect Horaires",
            ViolationType::VenteAlcoolSansLicence => "Vente Alcool Sans Licence",
            ViolationType::ConstructionIllegale => "Construction Illégale",
            ViolationType::PubliciteNonAutorisee => "Publicité Non Autorisée",
            ViolationType::Autre => "Autre",
        }
    }

    /// Returns the legal reference for this violation type
    pub fn get_legal_reference(&self) -> &str {
        match self {
            ViolationType::SansAutorisation => "المادة 39 من القانون رقم 78.00",
            ViolationType::OccupationVoiePublique => "المادة 7 من قانون الملك العمومي",
            ViolationType::NuisancesSonores => "المادة 77 من قانون البيئة 11.03",
            ViolationType::Insalubrite => "المادة 84 من القانون رقم 28.00",
            ViolationType::NonRespectHoraires => "المادة 2 من قرار وزير الداخلية",
            ViolationType::VenteAlcoolSansLicence => "ظهير 1967",
            ViolationType::ConstructionIllegale => "المادة 59 من قانون التعمير 12.90",
            ViolationType::PubliciteNonAutorisee => "القانون رقم 78.00",
            ViolationType::Autre => "حسب القانون المعمول به",
        }
    }

    /// Returns all violation types for UI selection
    pub fn all() -> Vec<ViolationType> {
        vec![
            ViolationType::SansAutorisation,
            ViolationType::OccupationVoiePublique,
            ViolationType::NuisancesSonores,
            ViolationType::Insalubrite,
            ViolationType::NonRespectHoraires,
            ViolationType::VenteAlcoolSansLicence,
            ViolationType::ConstructionIllegale,
            ViolationType::PubliciteNonAutorisee,
            ViolationType::Autre,
        ]
    }
}

impl std::fmt::Display for ViolationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.to_arabic(), self.to_french())
    }
}

/// Record of an inspection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionRecord {
    /// Unique inspection identifier
    pub inspection_id: String,
    /// Associated commerce ID
    pub commerce_id: String,
    /// Inspection date (YYYY-MM-DD)
    pub inspection_date: String,
    /// Inspection time (HH:MM:SS)
    pub inspection_time: String,
    /// اسم العون - Inspector name
    pub inspector_name: String,
    /// رتبة العون - Inspector grade/rank
    pub inspector_grade: String,
    /// Violation types found
    pub violation_types: Vec<ViolationType>,
    /// وصف المخالفة - Violation description
    pub description: String,
    /// المرجع القانوني - Legal reference
    pub legal_reference: String,
    /// مراجع الصور - Photo references
    pub photos_references: Vec<String>,
    /// الشهود - Witnesses
    pub witnesses: Vec<String>,
    /// الإجراءات المتخذة - Measures taken
    pub measures_taken: String,
}

impl InspectionRecord {
    /// Creates a new InspectionRecord with generated ID and current timestamp
    pub fn new(commerce_id: String) -> Self {
        let now = chrono::Local::now();
        Self {
            inspection_id: uuid::Uuid::new_v4().to_string(),
            commerce_id,
            inspection_date: now.format("%Y-%m-%d").to_string(),
            inspection_time: now.format("%H:%M:%S").to_string(),
            inspector_name: String::new(),
            inspector_grade: String::new(),
            violation_types: Vec::new(),
            description: String::new(),
            legal_reference: String::new(),
            photos_references: Vec::new(),
            witnesses: Vec::new(),
            measures_taken: String::new(),
        }
    }

    /// Returns a comma-separated list of violation names (Arabic)
    pub fn violations_summary(&self) -> String {
        self.violation_types
            .iter()
            .map(|v| v.to_arabic())
            .collect::<Vec<_>>()
            .join("، ")
    }
}

impl Default for InspectionRecord {
    fn default() -> Self {
        Self::new(String::new())
    }
}
