//! Dashboard view state



/// State for the dashboard view
#[derive(Debug, Clone, Default)]
pub struct DashboardView {
    /// Last refresh timestamp
    pub last_refresh: Option<String>,
    /// Selected time range for charts
    pub time_range: TimeRange,
}

/// Time range for statistics
#[derive(Debug, Clone, Default, PartialEq)]
pub enum TimeRange {
    #[default]
    ThisMonth,
    LastMonth,
    ThisYear,
    AllTime,
}

impl TimeRange {
    pub fn to_arabic(&self) -> &str {
        match self {
            TimeRange::ThisMonth => "هذا الشهر",
            TimeRange::LastMonth => "الشهر الماضي",
            TimeRange::ThisYear => "هذه السنة",
            TimeRange::AllTime => "كل الفترة",
        }
    }

    pub fn to_french(&self) -> &str {
        match self {
            TimeRange::ThisMonth => "Ce mois",
            TimeRange::LastMonth => "Mois dernier",
            TimeRange::ThisYear => "Cette année",
            TimeRange::AllTime => "Tout",
        }
    }
}
