//! Arabic text input widget with RTL support

use eframe::egui;

/// An input widget optimized for Arabic (RTL) text
pub struct ArabicTextInput<'a> {
    text: &'a mut String,
    hint: Option<String>,
    multiline: bool,
}

impl<'a> ArabicTextInput<'a> {
    /// Creates a new Arabic text input
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            hint: None,
            multiline: false,
        }
    }

    /// Sets the hint text
    pub fn hint(mut self, hint: &str) -> Self {
        self.hint = Some(hint.to_string());
        self
    }

    /// Makes this a multiline input
    pub fn multiline(mut self) -> Self {
        self.multiline = true;
        self
    }

    /// Shows the input widget
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        // Apply RTL-friendly styling
        let text_style = egui::TextStyle::Body;
        let font_id = ui.style().text_styles.get(&text_style).cloned().unwrap_or_default();

        if self.multiline {
            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                let mut job = egui::text::LayoutJob::default();
                job.wrap.max_width = wrap_width;
                job.halign = egui::Align::RIGHT; // RTL alignment
                job.append(
                    string,
                    0.0,
                    egui::TextFormat {
                        font_id: font_id.clone(),
                        ..Default::default()
                    },
                );
                ui.fonts(|f| f.layout_job(job))
            };

            egui::TextEdit::multiline(self.text)
                .hint_text(self.hint.unwrap_or_default())
                .layouter(&mut layouter)
                .show(ui)
                .response
        } else {
            egui::TextEdit::singleline(self.text)
                .hint_text(self.hint.unwrap_or_default())
                .horizontal_align(egui::Align::RIGHT)
                .show(ui)
                .response
        }
    }
}

/// Helper trait for creating bilingual labels
pub trait BilingualLabel {
    fn bilingual_label(&mut self, arabic: &str, french: &str) -> egui::Response;
}

impl BilingualLabel for egui::Ui {
    fn bilingual_label(&mut self, arabic: &str, french: &str) -> egui::Response {
        self.label(format!("{} / {}", arabic, french))
    }
}
