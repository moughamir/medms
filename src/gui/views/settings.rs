//! Settings view state

/// State for the settings view
#[derive(Debug, Clone)]
pub struct SettingsView {
    /// Commune name
    pub commune: String,
    /// Arrondissement name
    pub arrondissement: String,
    /// Service name
    pub service: String,
    /// Default inspector name
    pub default_inspector_name: String,
    /// Default inspector grade
    pub default_inspector_grade: String,
    /// Database file path
    pub database_path: String,
    /// Output directory for documents
    pub output_directory: String,
    /// Language preference
    pub language: Language,
    /// Theme preference
    pub theme: Theme,
}

/// Language options
#[derive(Debug, Clone, Default, PartialEq)]
pub enum Language {
    #[default]
    ArabicFrench,
    Arabic,
    French,
}

impl Language {
    pub fn to_label(&self) -> &str {
        match self {
            Language::ArabicFrench => "العربية/Français",
            Language::Arabic => "العربية",
            Language::French => "Français",
        }
    }
}

/// Theme options
#[derive(Debug, Clone, Default, PartialEq)]
pub enum Theme {
    #[default]
    Dark,
    Light,
    System,
}

impl Theme {
    pub fn to_label(&self) -> &str {
        match self {
            Theme::Dark => "داكن / Sombre",
            Theme::Light => "فاتح / Clair",
            Theme::System => "حسب النظام / Système",
        }
    }
}

impl SettingsView {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for SettingsView {
    fn default() -> Self {
        Self {
            commune: "جماعة ...".to_string(),
            arrondissement: "مقاطعة ...".to_string(),
            service: "الشرطة الإدارية".to_string(),
            default_inspector_name: String::new(),
            default_inspector_grade: String::new(),
            database_path: "store/police_administrative.xlsx".to_string(),
            output_directory: "store/documents".to_string(),
            language: Language::ArabicFrench,
            theme: Theme::Dark,
        }
    }
}
