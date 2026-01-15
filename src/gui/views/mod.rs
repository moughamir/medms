//! GUI views module

pub mod dashboard;
pub mod commerce;
pub mod documents;
pub mod inspection;
pub mod onboarding;
pub mod settings;

pub use dashboard::DashboardView;
pub use commerce::CommerceForm;
pub use inspection::InspectionForm;
pub use onboarding::OnboardingView;
pub use documents::DocumentsView;
pub use settings::SettingsView;
