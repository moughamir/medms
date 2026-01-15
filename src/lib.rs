//! Moroccan Administrative Documents Generator
//!
//! A document management system for municipal administrative police.
//! Generates official documents (Procès-Verbal, Avertissement, etc.) in Arabic
//! and maintains records in an Excel database.

pub mod cli;
pub mod database;
pub mod generator;
pub mod gui;
pub mod models;

pub use cli::run_cli;
pub use database::ExcelDatabase;
pub use generator::DocumentGenerator;
pub use gui::MoroccanDocsApp;
