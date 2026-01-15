use eframe::egui;
use crate::models::CommerceInfo;
use crate::gui::localization::tr;

use crate::gui::views::settings::Language;

#[derive(Debug, Clone)]
pub struct CommerceSelector {
    pub search_query: String,
    pub selected_commerce: Option<CommerceInfo>,
    pub is_open: bool,
}

impl Default for CommerceSelector {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            selected_commerce: None,
            is_open: false,
        }
    }
}

impl CommerceSelector {
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        commerces: &[CommerceInfo],
        lang: &Language,
    ) -> SelectorResult {
        let mut result = SelectorResult::None;

        ui.horizontal(|ui| {
            ui.label(format!("{}:", tr("shop", lang)));
            
            let btn_text = if let Some(c) = &self.selected_commerce {
                // Determine display based on language/content
                // For now just show name
                c.denomination.clone()
            } else {
                tr("search", lang)
            };

            if ui.button(btn_text).clicked() {
                self.is_open = !self.is_open;
            }
        });

        if self.is_open {
            let mut is_open = true;
            let mut should_close = false;
            
            egui::Window::new(tr("search", lang))
                .open(&mut is_open)
                .show(ui.ctx(), |ui| {
                    ui.text_edit_singleline(&mut self.search_query);
                    ui.separator();

                    egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                        let query = self.search_query.to_lowercase();
                        for commerce in commerces {
                            if commerce.denomination.to_lowercase().contains(&query) 
                                || commerce.owner_name.to_lowercase().contains(&query) {
                                
                                if ui.button(&commerce.denomination).clicked() {
                                    self.selected_commerce = Some(commerce.clone());
                                    result = SelectorResult::Selected(commerce.clone());
                                    should_close = true;
                                }
                                ui.separator();
                            }
                        }
                        
                        // "Add New" button if no results or always visible at bottom
                        ui.add_space(10.0);
                        if ui.button(format!("+ {}", tr("add_commerce", lang))).clicked() {
                             result = SelectorResult::AddNew;
                             should_close = true;
                        }
                    });
                });
            
            if !is_open || should_close {
                self.is_open = false;
            }
        }
        
        result
    }
}


pub enum SelectorResult {
    None,
    Selected(CommerceInfo),
    AddNew,
}
