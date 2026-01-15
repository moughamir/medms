//! Document types and metadata for administrative documents

use serde::{Deserialize, Serialize};

/// Types of administrative documents that can be generated
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentType {
    /// محضر معاينة - Inspection Report
    ProcesVerbal,
    /// إنذار - Warning
    Avertissement,
    /// إعذار - Formal Notice
    MiseEnDemeure,
    /// قرار غلق - Closure Decision
    DecisionFermeture,
    /// غرامة - Fine
    Amende,
    /// تقرير مراقبة - Control Report
    RapportControle,
    /// استدعاء - Summons
    Convocation,
}

impl DocumentType {
    /// Returns the Arabic name for the document type
    pub fn to_arabic(&self) -> &str {
        match self {
            DocumentType::ProcesVerbal => "محضر معاينة",
            DocumentType::Avertissement => "إنذار",
            DocumentType::MiseEnDemeure => "إعذار",
            DocumentType::DecisionFermeture => "قرار غلق",
            DocumentType::Amende => "غرامة إدارية",
            DocumentType::RapportControle => "تقرير مراقبة",
            DocumentType::Convocation => "استدعاء",
        }
    }

    /// Returns the French name for the document type
    pub fn to_french(&self) -> &str {
        match self {
            DocumentType::ProcesVerbal => "Procès-Verbal",
            DocumentType::Avertissement => "Avertissement",
            DocumentType::MiseEnDemeure => "Mise en Demeure",
            DocumentType::DecisionFermeture => "Décision de Fermeture",
            DocumentType::Amende => "Amende",
            DocumentType::RapportControle => "Rapport de Contrôle",
            DocumentType::Convocation => "Convocation",
        }
    }

    /// Returns all document types for UI selection
    pub fn all() -> Vec<DocumentType> {
        vec![
            DocumentType::ProcesVerbal,
            DocumentType::Avertissement,
            DocumentType::MiseEnDemeure,
            DocumentType::DecisionFermeture,
            DocumentType::Amende,
            DocumentType::RapportControle,
            DocumentType::Convocation,
        ]
    }
}

impl std::fmt::Display for DocumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.to_arabic(), self.to_french())
    }
}

/// Metadata for a generated document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    /// Unique identifier (UUID)
    pub id: String,
    /// Document number (e.g., "2026/0001")
    pub doc_number: String,
    /// Type of document
    pub doc_type: DocumentType,
    /// Creation date (YYYY-MM-DD)
    pub creation_date: String,
    /// Creation time (HH:MM:SS)
    pub creation_time: String,
    /// الجماعة - Commune
    pub commune: String,
    /// المقاطعة - Arrondissement
    pub arrondissement: String,
    /// المصلحة - Service (Police Administrative)
    pub service: String,
    /// Associated commerce ID
    pub commerce_id: String,
    /// Associated inspection ID
    pub inspection_id: String,
    /// Associated action ID
    pub action_id: String,
    /// Document content
    pub content: String,
    /// Legal references
    pub legal_references: Vec<String>,
    /// Inspector name
    pub inspector_name: String,
    /// Inspector grade/rank
    pub inspector_grade: String,
    /// Disclaimer text
    pub disclaimer: String,
}

impl DocumentMetadata {
    /// Creates a new DocumentMetadata with default values
    pub fn new(doc_type: DocumentType, doc_number: String) -> Self {
        let now = chrono::Local::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            doc_number,
            doc_type,
            creation_date: now.format("%Y-%m-%d").to_string(),
            creation_time: now.format("%H:%M:%S").to_string(),
            commune: String::new(),
            arrondissement: String::new(),
            service: "الشرطة الإدارية".to_string(),
            commerce_id: String::new(),
            inspection_id: String::new(),
            action_id: String::new(),
            content: String::new(),
            legal_references: Vec::new(),
            inspector_name: String::new(),
            inspector_grade: String::new(),
            disclaimer: "وثيقة رسمية - لا يمكن استعمالها إلا للأغراض القانونية".to_string(),
        }
    }
}
