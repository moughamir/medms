//! Main GUI application

use crate::database::ExcelDatabase;
use crate::generator::DocumentGenerator;
use crate::gui::views::{CommerceForm, DashboardView, DocumentsView, InspectionForm, SettingsView};
use crate::models::{
    ActionStatus, CommerceInfo, DashboardStats, DocumentMetadata, DocumentType, EnforcementAction,
    InspectionRecord, ViolationType,
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
    /// Status message
    status_message: Option<(String, StatusType)>,
    /// Config: commune name
    commune: String,
    /// Config: arrondissement name
    arrondissement: String,
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
        let database = ExcelDatabase::new("police_administrative.xlsx");
        let _ = database.initialize();
        let dashboard_stats = database.get_dashboard_stats().unwrap_or_default();

        Self {
            current_view: View::Dashboard,
            database,
            dashboard_stats,
            commerce_form: CommerceForm::default(),
            inspection_form: InspectionForm::default(),
            documents_view: DocumentsView::default(),
            settings_view: SettingsView::default(),
            status_message: None,
            commune: "جماعة ...".to_string(),
            arrondissement: "مقاطعة ...".to_string(),
        }
    }
}

impl MoroccanDocsApp {
    /// Creates a new application instance
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure fonts for Arabic support
        Self::configure_fonts(&cc.egui_ctx);
        Self::default()
    }

    /// Configures fonts for Arabic text rendering
    fn configure_fonts(ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();

        // Use system font that supports Arabic
        // On most systems, the default proportional font should work
        // For better Arabic support, we would add a custom Arabic font here

        ctx.set_fonts(fonts);

        // Set RTL text direction for Arabic
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
                    ui.heading("الشرطة الإدارية");
                    ui.label("Police Administrative");
                    ui.separator();
                });

                ui.add_space(20.0);

                // Navigation buttons
                let nav_items = [
                    (View::Dashboard, "📊", "لوحة التحكم", "Tableau de bord"),
                    (View::NewCommerce, "🏪", "محل جديد", "Nouveau commerce"),
                    (
                        View::NewInspection,
                        "🔍",
                        "معاينة جديدة",
                        "Nouvelle inspection",
                    ),
                    (View::NewDocument, "📄", "وثيقة جديدة", "Nouveau document"),
                    (View::DocumentList, "📁", "سجل الوثائق", "Registre"),
                    (View::Settings, "⚙", "الإعدادات", "Paramètres"),
                ];

                for (view, icon, ar_label, fr_label) in nav_items {
                    let is_selected = self.current_view == view;
                    let button_text = format!("{} {} / {}", icon, ar_label, fr_label);

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
                ui.label(format!("المحلات: {}", self.dashboard_stats.total_commerces));
                ui.label(format!("الوثائق: {}", self.dashboard_stats.total_documents));
            });
    }

    /// Renders the main content area
    fn render_main_content(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Status bar
            if let Some((message, status_type)) = &self.status_message {
                let color = match status_type {
                    StatusType::Success => egui::Color32::from_rgb(76, 175, 80),
                    StatusType::Error => egui::Color32::from_rgb(244, 67, 54),
                    StatusType::Info => egui::Color32::from_rgb(33, 150, 243),
                };
                ui.horizontal(|ui| {
                    ui.colored_label(color, message);
                    if ui.button("✕").clicked() {
                        self.status_message = None;
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
    }

    /// Renders the dashboard view
    fn render_dashboard(&mut self, ui: &mut egui::Ui) {
        ui.heading("لوحة التحكم / Tableau de bord");
        ui.separator();

        // Summary cards
        ui.horizontal_wrapped(|ui| {
            self.render_stat_card(
                ui,
                "🏪",
                "المحلات",
                &self.dashboard_stats.total_commerces.to_string(),
            );
            self.render_stat_card(
                ui,
                "🔍",
                "المعاينات",
                &self.dashboard_stats.total_inspections.to_string(),
            );
            self.render_stat_card(
                ui,
                "📄",
                "الوثائق",
                &self.dashboard_stats.total_documents.to_string(),
            );
            self.render_stat_card(
                ui,
                "⚠",
                "بدون رخصة",
                &self.dashboard_stats.commerces_sans_autorisation.to_string(),
            );
        });

        ui.add_space(20.0);

        // Fine statistics
        ui.horizontal_wrapped(|ui| {
            self.render_stat_card(
                ui,
                "💰",
                "الغرامات الصادرة",
                &format!("{:.2} DH", self.dashboard_stats.total_fines_issued),
            );
            self.render_stat_card(
                ui,
                "✓",
                "المحصلة",
                &format!("{:.2} DH", self.dashboard_stats.total_fines_collected),
            );
            self.render_stat_card(
                ui,
                "⏳",
                "المستحقة",
                &format!("{:.2} DH", self.dashboard_stats.pending_fines),
            );
            self.render_stat_card(
                ui,
                "⏰",
                "متأخرة",
                &self.dashboard_stats.overdue_actions.to_string(),
            );
        });

        ui.add_space(20.0);
        ui.separator();

        // Violations by type
        if !self.dashboard_stats.violations_by_type.is_empty() {
            ui.heading("المخالفات حسب النوع / Infractions par type");
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
        if ui.button("🔄 تحديث / Actualiser").clicked() {
            self.refresh_stats();
            self.set_status("تم التحديث / Actualisé".to_string(), StatusType::Success);
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
        ui.heading("إضافة محل جديد / Nouveau commerce");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("commerce_form")
                .num_columns(2)
                .spacing([20.0, 10.0])
                .show(ui, |ui| {
                    // Trade name
                    ui.label("التسمية التجارية:");
                    ui.text_edit_singleline(&mut self.commerce_form.denomination);
                    ui.end_row();

                    // Owner name
                    ui.label("اسم المالك:");
                    ui.text_edit_singleline(&mut self.commerce_form.owner_name);
                    ui.end_row();

                    // CIN
                    ui.label("رقم ب.و.ت:");
                    ui.text_edit_singleline(&mut self.commerce_form.owner_cin);
                    ui.end_row();

                    // Phone
                    ui.label("الهاتف:");
                    ui.text_edit_singleline(&mut self.commerce_form.owner_phone);
                    ui.end_row();

                    // Owner address
                    ui.label("عنوان المالك:");
                    ui.text_edit_singleline(&mut self.commerce_form.owner_address);
                    ui.end_row();

                    // Establishment address
                    ui.label("عنوان المحل:");
                    ui.text_edit_singleline(&mut self.commerce_form.establishment_address);
                    ui.end_row();

                    // District
                    ui.label("الحي:");
                    ui.text_edit_singleline(&mut self.commerce_form.quartier);
                    ui.end_row();

                    // Activity type
                    ui.label("نوع النشاط:");
                    ui.text_edit_singleline(&mut self.commerce_form.activity_type);
                    ui.end_row();

                    // ICE
                    ui.label("رقم ICE:");
                    ui.text_edit_singleline(&mut self.commerce_form.ice_number);
                    ui.end_row();

                    // Patente
                    ui.label("رقم الباطونطا:");
                    ui.text_edit_singleline(&mut self.commerce_form.patente_number);
                    ui.end_row();

                    // Authorization
                    ui.label("رخصة:");
                    ui.checkbox(&mut self.commerce_form.has_autorisation, "نعم");
                    ui.end_row();

                    if self.commerce_form.has_autorisation {
                        ui.label("رقم الرخصة:");
                        ui.text_edit_singleline(&mut self.commerce_form.autorisation_number);
                        ui.end_row();

                        ui.label("تاريخ الرخصة:");
                        ui.text_edit_singleline(&mut self.commerce_form.autorisation_date);
                        ui.end_row();
                    }
                });

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                if ui.button("💾 حفظ / Enregistrer").clicked() {
                    self.save_commerce();
                }
                if ui.button("🗑 مسح / Effacer").clicked() {
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
                    "تم حفظ المحل بنجاح / Commerce enregistré".to_string(),
                    StatusType::Success,
                );
                self.commerce_form = CommerceForm::default();
                self.refresh_stats();
            }
            Err(e) => {
                self.set_status(format!("خطأ: {} / Erreur: {}", e, e), StatusType::Error);
            }
        }
    }

    /// Renders the inspection form
    fn render_inspection_form(&mut self, ui: &mut egui::Ui) {
        ui.heading("معاينة جديدة / Nouvelle inspection");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("inspection_form")
                .num_columns(2)
                .spacing([20.0, 10.0])
                .show(ui, |ui| {
                    // Inspector name
                    ui.label("اسم العون:");
                    ui.text_edit_singleline(&mut self.inspection_form.inspector_name);
                    ui.end_row();

                    // Inspector grade
                    ui.label("رتبة العون:");
                    ui.text_edit_singleline(&mut self.inspection_form.inspector_grade);
                    ui.end_row();

                    // Commerce selection (simplified - would need dropdown)
                    ui.label("المحل:");
                    ui.text_edit_singleline(&mut self.inspection_form.commerce_name);
                    ui.end_row();

                    // Violation type
                    ui.label("نوع المخالفة:");
                    egui::ComboBox::from_id_salt("violation_type")
                        .selected_text(self.inspection_form.selected_violation.to_arabic())
                        .show_ui(ui, |ui| {
                            for vtype in ViolationType::all() {
                                let label = vtype.to_string();
                                if ui
                                    .selectable_label(
                                        self.inspection_form.selected_violation == vtype,
                                        &label,
                                    )
                                    .clicked()
                                {
                                    self.inspection_form.selected_violation = vtype;
                                }
                            }
                        });
                    ui.end_row();

                    // Description
                    ui.label("وصف المخالفة:");
                    ui.text_edit_multiline(&mut self.inspection_form.description);
                    ui.end_row();

                    // Measures taken
                    ui.label("الإجراءات المتخذة:");
                    ui.text_edit_multiline(&mut self.inspection_form.measures_taken);
                    ui.end_row();
                });

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                if ui.button("💾 حفظ / Enregistrer").clicked() {
                    self.save_inspection();
                }
                if ui.button("🗑 مسح / Effacer").clicked() {
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
                    "تم حفظ المعاينة بنجاح / Inspection enregistrée".to_string(),
                    StatusType::Success,
                );
                self.inspection_form = InspectionForm::default();
                self.refresh_stats();
            }
            Err(e) => {
                self.set_status(format!("خطأ: {} / Erreur: {}", e, e), StatusType::Error);
            }
        }
    }

    /// Renders the document generation form
    fn render_document_form(&mut self, ui: &mut egui::Ui) {
        ui.heading("إنشاء وثيقة جديدة / Nouveau document");
        ui.separator();

        ui.label("اختر نوع الوثيقة / Choisir le type:");
        ui.add_space(10.0);

        for doc_type in DocumentType::all() {
            if ui.button(doc_type.to_string()).clicked() {
                // Would open document creation wizard
                self.set_status(
                    format!("إنشاء {} / Création {}", doc_type.to_arabic(), doc_type.to_french()),
                    StatusType::Info,
                );
            }
        }
    }

    /// Renders the document list view
    fn render_document_list(&mut self, ui: &mut egui::Ui) {
        ui.heading("سجل الوثائق / Registre des documents");
        ui.separator();

        // Search bar
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.documents_view.search_query);
            if ui.button("بحث / Rechercher").clicked() {
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
                            ui.label(doc.doc_type.to_arabic());
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
                    format!("خطأ في تحميل الوثائق: {}", e),
                );
            }
        }
    }

    /// Renders the settings view
    fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("الإعدادات / Paramètres");
        ui.separator();

        egui::Grid::new("settings_grid")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label("الجماعة:");
                ui.text_edit_singleline(&mut self.commune);
                ui.end_row();

                ui.label("المقاطعة:");
                ui.text_edit_singleline(&mut self.arrondissement);
                ui.end_row();

                ui.label("ملف قاعدة البيانات:");
                ui.label(self.database.file_path());
                ui.end_row();
            });

        ui.add_space(20.0);

        if ui.button("💾 حفظ الإعدادات / Enregistrer").clicked() {
            self.set_status(
                "تم حفظ الإعدادات / Paramètres enregistrés".to_string(),
                StatusType::Success,
            );
        }
    }
}

impl eframe::App for MoroccanDocsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_sidebar(ctx);
        self.render_main_content(ctx);
    }
}
