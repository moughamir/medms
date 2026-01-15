//! Data models for the Moroccan Administrative Documents Generator

pub mod commerce;
pub mod document;
pub mod enforcement;
pub mod inspection;
pub mod statistics;

pub use commerce::CommerceInfo;
pub use document::{DocumentMetadata, DocumentType};
pub use enforcement::{ActionStatus, EnforcementAction};
pub use inspection::{InspectionRecord, ViolationType};
pub use statistics::DashboardStats;
