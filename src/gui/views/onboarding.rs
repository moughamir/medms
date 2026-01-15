use eframe::egui;
use crate::gui::localization::tr;
use crate::gui::text_utils::reshape;
use crate::gui::views::SettingsView;

pub struct OnboardingView {
    pub current_step: usize,
    pub settings: SettingsView,
}

impl Default for OnboardingView {
    fn default() -> Self {
        Self {
            current_step: 0,
            settings: SettingsView::default(),
        }
    }
}

impl OnboardingView {
    pub fn update(&mut self, ctx: &egui::Context, settings_from_app: &mut SettingsView) {
        // Sync local settings with app settings initially
        if self.current_step == 0 && self.settings.commune == "جماعة ..." {
            self.settings = settings_from_app.clone();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                match self.current_step {
                    0 => self.render_welcome(ui),
                    1 => self.render_location(ui),
                    2 => self.render_identity(ui),
                    3 => self.render_database(ui),
                    _ => {},
                }
            });
            
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Navigation buttons
            ui.horizontal(|ui| {
                if self.current_step > 0 {
                    if ui.button(tr("back", &self.settings.language)).clicked() {
                        self.current_step -= 1;
                    }
                }
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.current_step < 3 {
                         if ui.button(tr("next", &self.settings.language)).clicked() {
                            self.current_step += 1;
                        }
                    } else {
                        if ui.button(tr("finish", &self.settings.language)).clicked() {
                            // Save everything
                            settings_from_app.commune = self.settings.commune.clone();
                            settings_from_app.arrondissement = self.settings.arrondissement.clone();
                            settings_from_app.default_inspector_name = self.settings.default_inspector_name.clone();
                            settings_from_app.default_inspector_grade = self.settings.default_inspector_grade.clone();
                            settings_from_app.database_path = self.settings.database_path.clone();
                            settings_from_app.is_onboarded = true;
                        }
                    }
                });
            });
        });
    }

    fn render_welcome(&mut self, ui: &mut egui::Ui) {
        ui.add_space(50.0);
        
        // Logo
        ui.image(egui::include_image!("../../../assets/images/logo.png"));
        ui.add_space(20.0);
        
        ui.heading(reshape("مرحباً بكم في نظام الشرطة الإدارية"));
        ui.heading("Bienvenue dans le système de Police Administrative");
        ui.add_space(20.0);
        ui.label("This wizard will guide you through the initial setup.");
        ui.label(reshape("سيقوم هذا المعالج بتوجيهك خلال الإعداد الأولي."));
    }

    fn render_location(&mut self, ui: &mut egui::Ui) {
        ui.heading(tr("location_settings", &self.settings.language));
        ui.add_space(20.0);
        
        ui.label(format!("{}:", tr("commune", &self.settings.language)));
        ui.text_edit_singleline(&mut self.settings.commune);
        ui.add_space(10.0);
        
        ui.label(format!("{}:", tr("arrondissement", &self.settings.language)));
        ui.text_edit_singleline(&mut self.settings.arrondissement);
    }

    fn render_identity(&mut self, ui: &mut egui::Ui) {
        ui.heading(tr("identity_settings", &self.settings.language));
        ui.add_space(20.0);
        
        ui.label(format!("{}:", tr("inspector_name", &self.settings.language)));
        ui.text_edit_singleline(&mut self.settings.default_inspector_name);
        ui.add_space(10.0);
        
        ui.label(format!("{}:", tr("inspector_grade", &self.settings.language)));
        ui.text_edit_singleline(&mut self.settings.default_inspector_grade);
    }

    fn render_database(&mut self, ui: &mut egui::Ui) {
        ui.heading(tr("database_settings", &self.settings.language));
        ui.add_space(20.0);
        
        ui.label(format!("{}:", tr("db_file", &self.settings.language)));
        ui.text_edit_singleline(&mut self.settings.database_path);
    }
}
