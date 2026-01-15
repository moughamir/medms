//! Database module for Excel-based data storage

pub mod excel;
pub mod queries;

pub use excel::ExcelDatabase;
pub use queries::*;
