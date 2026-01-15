//! Documents list view state

use crate::models::{DocumentMetadata, DocumentType};

/// State for the documents list view
#[derive(Debug, Clone, Default)]
pub struct DocumentsView {
    /// Search query
    pub search_query: String,
    /// Selected document type filter
    pub type_filter: Option<DocumentType>,
    /// Date range filter (start)
    pub date_from: String,
    /// Date range filter (end)
    pub date_to: String,
    /// Currently selected document
    pub selected_document: Option<String>,
    /// Sort order
    pub sort_order: SortOrder,
}

/// Sort order for document list
#[derive(Debug, Clone, Default, PartialEq)]
pub enum SortOrder {
    #[default]
    DateDesc,
    DateAsc,
    NumberDesc,
    NumberAsc,
}

impl SortOrder {
    pub fn to_label(&self) -> &str {
        match self {
            SortOrder::DateDesc => "التاريخ (الأحدث أولا)",
            SortOrder::DateAsc => "التاريخ (الأقدم أولا)",
            SortOrder::NumberDesc => "الرقم (تنازلي)",
            SortOrder::NumberAsc => "الرقم (تصاعدي)",
        }
    }
}

impl DocumentsView {
    pub fn new() -> Self {
        Self::default()
    }

    /// Clears all filters
    pub fn clear_filters(&mut self) {
        self.search_query.clear();
        self.type_filter = None;
        self.date_from.clear();
        self.date_to.clear();
    }
}
