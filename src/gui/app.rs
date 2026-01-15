//! Main GUI application

use crate::database::ExcelDatabase;
use crate::gui::localization::tr;
use crate::gui::text_utils::reshape;
use crate::gui::views::{
    CommerceForm, DocumentsView, InspectionForm, OnboardingView, SettingsView,
};
use crate::models::{
    CommerceInfo, DashboardStats, DocumentType, InspectionRecord, ViolationType,
};
use eframe::egui;

/// The main application state
pub struct MoroccanDocsApp {
    /// Current view being displayed
    current_view: View,
    /// Excel database connection
    database: ExcelDatabase,
    /// Dashboard statistics
    dashboard_stats: DashboardStats,
    /// Commerce form state
    commerce_form: CommerceForm,
    /// Inspection form state
    inspection_form: InspectionForm,
    /// Documents view state
    documents_view: DocumentsView,
    /// Settings view state
    settings_view: SettingsView,
    /// Onboarding view state
    onboarding_view: OnboardingView,
    /// Status message
    status_message: Option<(String, StatusType)>,
    /// Authentication state
    is_authenticated: bool,
    /// Login password input
    login_password: String,
}

/// Type of status message
#[derive(Clone, PartialEq)]
pub enum StatusType {
    Success,
    Error,
    Info,
}

/// Available views in the application
#[derive(Clone, PartialEq)]
pub enum View {
    Dashboard,
    NewCommerce,
    NewInspection,
    NewDocument,
    DocumentList,
    Settings,
}

impl Default for MoroccanDocsApp {
    fn default() -> Self {
        let settings_view = SettingsView::default();
        let database = ExcelDatabase::new(&settings_view.database_path);
        let _ = database.initialize();
        let dashboard_stats = database.get_dashboard_stats().unwrap_or_default();

        Self {
            current_view: View::Dashboard,
            database,
            dashboard_stats,
            commerce_form: CommerceForm::default(),
            inspection_form: InspectionForm::default(),
            documents_view: DocumentsView::default(),
            settings_view,
            onboarding_view: OnboardingView::default(),
            status_message: None,
            is_authenticated: false,
            login_password: String::new(),
        }
    }
}

impl MoroccanDocsApp {
    /// Creates a new application instance
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure fonts for Arabic support
        Self::configure_fonts(&cc.egui_ctx);
        // Install image loaders
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self::default()
    }

    /// Configures fonts for Arabic text rendering
    /// Configures fonts for Arabic text rendering
    fn configure_fonts(ctx: &egui::Context) {
        // Set Light Mode by default
        ctx.set_visuals(egui::Visuals::light());

        let mut fonts = egui::FontDefinitions::default();

        // Install Noto Sans Arabic
        fonts.font_data.insert(
            "NotoSansArabic".to_owned(),
            egui::FontData::from_static(include_bytes!("../../assets/fonts/NotoSansArabic-Regular.ttf")),
        );

        // Put my font first (highest priority) for proportional text:
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "NotoSansArabic".to_owned());

        // Put my font as last fallback for monospace:
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("NotoSansArabic".to_owned());

        ctx.set_fonts(fonts);

        // Set styling
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        ctx.set_style(style);
    }

    /// Refreshes dashboard statistics
    fn refresh_stats(&mut self) {
        if let Ok(stats) = self.database.get_dashboard_stats() {
            self.dashboard_stats = stats;
        }
    }

    /// Shows a status message
    fn set_status(&mut self, message: String, status_type: StatusType) {
        self.status_message = Some((message, status_type));
    }

    /// Clears the status message
    fn clear_status(&mut self) {
        self.status_message = None;
    }

    /// Renders the sidebar navigation
    fn render_sidebar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("sidebar")
            .min_width(200.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(tr("app_title", &self.settings_view.language));
                    ui.label("Police Administrative");
                    ui.separator();
                });

                ui.add_space(20.0);

                // Navigation buttons
                let nav_items = [
                    (View::Dashboard, "📊", tr("dashboard", &self.settings_view.language)),
                    (View::NewCommerce, "🏪", tr("new_commerce", &self.settings_view.language)),
                    (
                        View::NewInspection,
                        "🔍",
                        tr("new_inspection", &self.settings_view.language),
                    ),
                    (View::NewDocument, "📄", tr("new_document", &self.settings_view.language)),
                    (View::DocumentList, "📁", tr("documents_registry", &self.settings_view.language)),
                    (View::Settings, "⚙", tr("settings", &self.settings_view.language)),
                ];

                for (view, icon, label) in nav_items {
                    let is_selected = self.current_view == view;
                    let button_text = format!("{} {}", icon, label);

                    if ui
                        .selectable_label(is_selected, button_text)
                        .clicked()
                    {
                        self.current_view = view;
                        self.clear_status();
                    }
                }

                ui.add_space(20.0);
                ui.separator();

                // Quick stats
                ui.add_space(10.0);
                ui.label(format!("{}: {}", tr("commerces", &self.settings_view.language), self.dashboard_stats.total_commerces));
                ui.label(format!("{}: {}", tr("documents", &self.settings_view.language), self.dashboard_stats.total_documents));
            });
    }

    /// Renders the main content area
    fn render_main_content(&mut self, ctx: &egui::Context) {
        // Clone status message to avoid borrow issues
        let status_clone = self.status_message.clone();
        let mut clear_status = false;
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // Status bar
            if let Some((message, status_type)) = &status_clone {
                let color = match status_type {
                    StatusType::Success => egui::Color32::from_rgb(76, 175, 80),
                    StatusType::Error => egui::Color32::from_rgb(244, 67, 54),
                    StatusType::Info => egui::Color32::from_rgb(33, 150, 243),
                };
                ui.horizontal(|ui| {
                    ui.colored_label(color, message);
                    if ui.button("✕").clicked() {
                        clear_status = true;
                    }
                });
                ui.separator();
            }

            // Main content based on current view
            match self.current_view.clone() {
                View::Dashboard => self.render_dashboard(ui),
                View::NewCommerce => self.render_commerce_form(ui),
                View::NewInspection => self.render_inspection_form(ui),
                View::NewDocument => self.render_document_form(ui),
                View::DocumentList => self.render_document_list(ui),
                View::Settings => self.render_settings(ui),
            }
        });
        
        if clear_status {
            self.status_message = None;
        }
    }

    /// Renders the dashboard view
    fn render_dashboard(&mut self, ui: &mut egui::Ui) {
        ui.heading(tr("dashboard", &self.settings_view.language));
        ui.separator();

        // Summary cards
        ui.horizontal_wrapped(|ui| {
            self.render_stat_card(
                ui,
                "🏪",
                &tr("commerces", &self.settings_view.language),
                &self.dashboard_stats.total_commerces.to_string(),
            );
            self.render_stat_card(
                ui,
                "🔍",
                &tr("inspections", &self.settings_view.language),
                &self.dashboard_stats.total_inspections.to_string(),
            );
            self.render_stat_card(
                ui,
                "📄",
                &tr("documents", &self.settings_view.language),
                &self.dashboard_stats.total_documents.to_string(),
            );
            self.render_stat_card(
                ui,
                "⚠",
                &tr("no_license", &self.settings_view.language),
                &self.dashboard_stats.commerces_sans_autorisation.to_string(),
            );
        });

        ui.add_space(20.0);

        // Fine statistics
        ui.horizontal_wrapped(|ui| {
            self.render_stat_card(
                ui,
                "💰",
                &tr("fines_issued", &self.settings_view.language),
                &format!("{:.2} DH", self.dashboard_stats.total_fines_issued),
            );
            self.render_stat_card(
                ui,
                "✓",
                &tr("collected", &self.settings_view.language),
                &format!("{:.2} DH", self.dashboard_stats.total_fines_collected),
            );
            self.render_stat_card(
                ui,
                "⏳",
                &tr("pending", &self.settings_view.language),
                &format!("{:.2} DH", self.dashboard_stats.pending_fines),
            );
            self.render_stat_card(
                ui,
                "⏰",
                &tr("overdue", &self.settings_view.language),
                &self.dashboard_stats.overdue_actions.to_string(),
            );
        });

        ui.add_space(20.0);
        ui.separator();

        // Violations by type
        if !self.dashboard_stats.violations_by_type.is_empty() {
            ui.heading(tr("violations_by_type", &self.settings_view.language));
            ui.add_space(10.0);

            let top_violations = self.dashboard_stats.top_violations(5);
            for (violation_type, count) in top_violations {
                ui.horizontal(|ui| {
                    ui.label(format!("{}: {}", violation_type, count));
                    let max_count = self.dashboard_stats.violations_by_type.values().max().unwrap_or(&1);
                    let bar_width = (*count as f32 / *max_count as f32) * 200.0;
                    let (rect, _) = ui.allocate_exact_size(
                        egui::vec2(bar_width, 16.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().rect_filled(
                        rect,
                        4.0,
                        egui::Color32::from_rgb(33, 150, 243),
                    );
                });
            }
        }

        ui.add_space(20.0);

        // Refresh button
        if ui.button(format!("🔄 {}", tr("refresh", &self.settings_view.language))).clicked() {
            self.refresh_stats();
            self.set_status(tr("updated", &self.settings_view.language), StatusType::Success);
        }
    }

    /// Renders a statistics card
    fn render_stat_card(&self, ui: &mut egui::Ui, icon: &str, label: &str, value: &str) {
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(45, 45, 48))
            .rounding(8.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.set_min_width(150.0);
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new(icon).size(24.0));
                    ui.label(label);
                    ui.label(egui::RichText::new(value).size(20.0).strong());
                });
            });
    }

    /// Renders the commerce form
    fn render_commerce_form(&mut self, ui: &mut egui::Ui) {
        ui.heading(tr("add_commerce", &self.settings_view.language));
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("commerce_form")
                .num_columns(2)
                .spacing([20.0, 10.0])
                .show(ui, |ui| {
                    // Trade name
                    ui.label(format!("{}:", tr("trade_name", &self.settings_view.language)));
                    ui.text_edit_singleline(&mut self.commerce_form.denomination);
                    ui.end_row();

                    // Owner name
                    ui.label(format!("{}:", tr("owner_name", &self.settings_view.language)));
                    ui.text_edit_singleline(&mut self.commerce_form.owner_name);
                    ui.end_row();

                    // CIN
                    ui.label(format!("{}:", tr("cin", &self.settings_view.language)));
                    ui.text_edit_singleline(&mut self.commerce_form.owner_cin);
                    ui.end_row();

                    // Phone
                    ui.label(format!("{}:", tr("phone", &self.settings_view.language)));
                    ui.text_edit_singleline(&mut self.commerce_form.owner_phone);
                    ui.end_row();

                    // Owner address
                    ui.label(format!("{}:", tr("owner_address", &self.settings_view.language)));
                    ui.text_edit_singleline(&mut self.commerce_form.owner_address);
                    ui.end_row();

                    // Establishment address
                    ui.label(format!("{}:", tr("shop_address", &self.settings_view.language)));
                    ui.text_edit_singleline(&mut self.commerce_form.establishment_address);
                    ui.end_row();

                    // District
                    ui.label(format!("{}:", tr("district", &self.settings_view.language)));
                    ui.text_edit_singleline(&mut self.commerce_form.quartier);
                    ui.end_row();

                    // Activity type
                    ui.label(format!("{}:", tr("activity", &self.settings_view.language)));
                    ui.text_edit_singleline(&mut self.commerce_form.activity_type);
                    ui.end_row();

                    // ICE
                    ui.label(format!("{}:", tr("ice", &self.settings_view.language)));
                    ui.text_edit_singleline(&mut self.commerce_form.ice_number);
                    ui.end_row();

                    // Patente
                    ui.label(format!("{}:", tr("patente", &self.settings_view.language)));
                    ui.text_edit_singleline(&mut self.commerce_form.patente_number);
                    ui.end_row();

                    // Authorization
                    ui.label(format!("{}:", tr("has_license", &self.settings_view.language)));
                    ui.checkbox(&mut self.commerce_form.has_autorisation, tr("yes", &self.settings_view.language));
                    ui.end_row();

                    if self.commerce_form.has_autorisation {
                        ui.label(format!("{}:", tr("license_num", &self.settings_view.language)));
                        ui.text_edit_singleline(&mut self.commerce_form.autorisation_number);
                        ui.end_row();

                        ui.label(format!("{}:", tr("license_date", &self.settings_view.language)));
                        ui.text_edit_singleline(&mut self.commerce_form.autorisation_date);
                        ui.end_row();
                    }
                });

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                if ui.button(format!("💾 {}", tr("save", &self.settings_view.language))).clicked() {
                    self.save_commerce();
                }
                if ui.button(format!("🗑 {}", tr("clear", &self.settings_view.language))).clicked() {
                    self.commerce_form = CommerceForm::default();
                }
            });
        });
    }

    /// Saves the commerce form data
    fn save_commerce(&mut self) {
        let commerce = CommerceInfo {
            commerce_id: uuid::Uuid::new_v4().to_string(),
            denomination: self.commerce_form.denomination.clone(),
            owner_name: self.commerce_form.owner_name.clone(),
            owner_cin: self.commerce_form.owner_cin.clone(),
            owner_phone: self.commerce_form.owner_phone.clone(),
            owner_address: self.commerce_form.owner_address.clone(),
            establishment_address: self.commerce_form.establishment_address.clone(),
            quartier: self.commerce_form.quartier.clone(),
            activity_type: self.commerce_form.activity_type.clone(),
            ice_number: self.commerce_form.ice_number.clone(),
            patente_number: self.commerce_form.patente_number.clone(),
            has_autorisation: self.commerce_form.has_autorisation,
            autorisation_number: self.commerce_form.autorisation_number.clone(),
            autorisation_date: self.commerce_form.autorisation_date.clone(),
        };

        match self.database.save_commerce(&commerce) {
            Ok(_) => {
                self.set_status(
                    tr("commerce_saved", &self.settings_view.language),
                    StatusType::Success,
                );
                self.commerce_form = CommerceForm::default();
                self.refresh_stats();
            }
            Err(e) => {
                self.set_status(format!("{}: {}", tr("error", &self.settings_view.language), e), StatusType::Error);
            }
        }
    }

    /// Renders the inspection form
    fn render_inspection_form(&mut self, ui: &mut egui::Ui) {
        ui.heading(tr("new_inspection", &self.settings_view.language));
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("inspection_form")
                .num_columns(2)
                .spacing([20.0, 10.0])
                .show(ui, |ui| {
                    // Inspector name
                    ui.label(format!("{}:", tr("inspector_name", &self.settings_view.language)));
                    ui.text_edit_singleline(&mut self.inspection_form.inspector_name);
                    ui.end_row();

                    // Inspector grade
                    ui.label(format!("{}:", tr("inspector_grade", &self.settings_view.language)));
                    ui.text_edit_singleline(&mut self.inspection_form.inspector_grade);
                    ui.end_row();

                    // Commerce selection (simplified - would need dropdown)
                    ui.label(format!("{}:", tr("shop", &self.settings_view.language)));
                    ui.text_edit_singleline(&mut self.inspection_form.commerce_name);
                    ui.end_row();

                    // Violation type
                    ui.label(format!("{}:", tr("violation_type", &self.settings_view.language)));
                    egui::ComboBox::from_id_salt("violation_type")
                        .selected_text(reshape(self.inspection_form.selected_violation.to_arabic()))
                        .show_ui(ui, |ui| {
                            for vtype in ViolationType::all() {
                                let label = vtype.to_string();
                                if ui
                                    .selectable_label(
                                        self.inspection_form.selected_violation == vtype,
                                        &reshape(&label), // TODO: also localize violation types if needed
                                    )
                                    .clicked()
                                {
                                    self.inspection_form.selected_violation = vtype;
                                }
                            }
                        });
                    ui.end_row();

                    // Description
                    ui.label(format!("{}:", tr("description", &self.settings_view.language)));
                    ui.text_edit_multiline(&mut self.inspection_form.description);
                    ui.end_row();

                    // Measures taken
                    ui.label(format!("{}:", tr("measures", &self.settings_view.language)));
                    ui.text_edit_multiline(&mut self.inspection_form.measures_taken);
                    ui.end_row();
                });

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                if ui.button(format!("💾 {}", tr("save", &self.settings_view.language))).clicked() {
                    self.save_inspection();
                }
                if ui.button(format!("🗑 {}", tr("clear", &self.settings_view.language))).clicked() {
                    self.inspection_form = InspectionForm::default();
                }
            });
        });
    }

    /// Saves the inspection form data
    fn save_inspection(&mut self) {
        let inspection = InspectionRecord {
            inspection_id: uuid::Uuid::new_v4().to_string(),
            commerce_id: String::new(), // Would be set from commerce selection
            inspection_date: chrono::Local::now().format("%Y-%m-%d").to_string(),
            inspection_time: chrono::Local::now().format("%H:%M:%S").to_string(),
            inspector_name: self.inspection_form.inspector_name.clone(),
            inspector_grade: self.inspection_form.inspector_grade.clone(),
            violation_types: vec![self.inspection_form.selected_violation.clone()],
            description: self.inspection_form.description.clone(),
            legal_reference: self
                .inspection_form
                .selected_violation
                .get_legal_reference()
                .to_string(),
            photos_references: Vec::new(),
            witnesses: Vec::new(),
            measures_taken: self.inspection_form.measures_taken.clone(),
        };

        match self.database.save_inspection(&inspection) {
            Ok(_) => {
                self.set_status(
                    tr("inspection_saved", &self.settings_view.language),
                    StatusType::Success,
                );
                self.inspection_form = InspectionForm::default();
                self.refresh_stats();
            }
            Err(e) => {
                self.set_status(format!("{}: {}", tr("error", &self.settings_view.language), e), StatusType::Error);
            }
        }
    }

    /// Renders the document generation form
    fn render_document_form(&mut self, ui: &mut egui::Ui) {
        ui.heading(tr("new_document", &self.settings_view.language));
        ui.separator();

        ui.label(format!("{}:", tr("choose_type", &self.settings_view.language)));
        ui.add_space(10.0);

        for doc_type in DocumentType::all() {
            if ui.button(doc_type.to_string()).clicked() {
                // Would open document creation wizard
                self.set_status(
                    format!("{} {}", tr("create", &self.settings_view.language), reshape(doc_type.to_arabic())),
                    StatusType::Info,
                );
            }
        }
    }

    /// Renders the document list view
    fn render_document_list(&mut self, ui: &mut egui::Ui) {
        ui.heading(tr("documents_registry", &self.settings_view.language));
        ui.separator();

        // Search bar
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.documents_view.search_query);
            if ui.button(tr("search", &self.settings_view.language)).clicked() {
                // Would trigger search
            }
        });

        ui.add_space(10.0);

        // Document list
        match self.database.get_recent_documents(50) {
            Ok(docs) => {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for doc in docs {
                        ui.horizontal(|ui| {
                            ui.label(&doc.doc_number);
                            ui.label(reshape(doc.doc_type.to_arabic()));
                            ui.label(&doc.creation_date);
                            ui.label(&doc.inspector_name);
                        });
                        ui.separator();
                    }
                });
            }
            Err(e) => {
                ui.colored_label(
                    egui::Color32::RED,
                    format!("{}: {}", tr("loading_error", &self.settings_view.language), e),
                );
            }
        }
    }

    /// Renders the settings view
    fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading(tr("settings", &self.settings_view.language));
        ui.separator();

        egui::Grid::new("settings_grid")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label(format!("{}:", tr("commune", &self.settings_view.language)));
                ui.text_edit_singleline(&mut self.settings_view.commune);
                ui.end_row();

                ui.label(format!("{}:", tr("arrondissement", &self.settings_view.language)));
                ui.text_edit_singleline(&mut self.settings_view.arrondissement);
                ui.end_row();

                ui.label(format!("{}:", tr("db_file", &self.settings_view.language)));
                ui.label(self.database.file_path());
                ui.end_row();
            });

        ui.add_space(20.0);

        if ui.button(format!("💾 {}", tr("save", &self.settings_view.language))).clicked() {
            self.set_status(
                tr("settings_saved", &self.settings_view.language),
                StatusType::Success,
            );
        }
    }
    /// Renders the login screen
    fn render_login(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.heading(tr("login", &self.settings_view.language));
                ui.add_space(20.0);
                
                ui.horizontal(|ui| {
                    ui.label(format!("{}:", tr("password", &self.settings_view.language)));
                    ui.add(egui::TextEdit::singleline(&mut self.login_password).password(true));
                });
                
                ui.add_space(20.0);
                
                if ui.button(tr("enter", &self.settings_view.language)).clicked() {
                    if self.login_password == "admin" {
                        self.is_authenticated = true;
                        self.login_password.clear();
                    } else {
                        // Using a simple alert for now if status can't be shown yet (or show invalid password text)
                        ui.label(tr("incorrect_password", &self.settings_view.language));
                    }
                }
            });
        });
    }
}

impl eframe::App for MoroccanDocsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.settings_view.is_onboarded {
            self.onboarding_view.update(ctx, &mut self.settings_view);
        } else if !self.is_authenticated {
            self.render_login(ctx);
        } else {
            self.render_sidebar(ctx);
            self.render_main_content(ctx);
        }
    }

}
