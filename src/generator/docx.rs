//! Word document generator for administrative documents

use crate::generator::templates;
use crate::models::{CommerceInfo, DocumentMetadata, DocumentType, EnforcementAction, InspectionRecord};
use anyhow::{Context, Result};
use docx_rs::*;

/// Generator for Word (.docx) documents
pub struct DocumentGenerator {
    metadata: DocumentMetadata,
    commerce: Option<CommerceInfo>,
    inspection: Option<InspectionRecord>,
    action: Option<EnforcementAction>,
}

impl DocumentGenerator {
    /// Creates a new DocumentGenerator
    pub fn new(
        metadata: DocumentMetadata,
        commerce: Option<CommerceInfo>,
        inspection: Option<InspectionRecord>,
        action: Option<EnforcementAction>,
    ) -> Self {
        DocumentGenerator {
            metadata,
            commerce,
            inspection,
            action,
        }
    }

    /// Creates and saves the document to the specified path
    pub fn create_document(&self, output_path: &str) -> Result<()> {
        let docx = Docx::new();

        let header = self.create_header();
        let footer = self.create_footer();
        let content = self.generate_content();

        let doc = docx
            .header(header)
            .footer(footer)
            .add_paragraph(content)
            .page_size(11906, 16838) // A4 in twips (210mm x 297mm)
            .page_margin(
                PageMargin::new()
                    .top(1440)    // 1 inch
                    .right(1440)
                    .bottom(1440)
                    .left(1440),
            );

        let file = std::fs::File::create(output_path)
            .context(format!("Failed to create file: {}", output_path))?;
        doc.build()
            .pack(file)
            .context("Failed to write document")?;

        Ok(())
    }

    /// Generates the document filename based on metadata
    pub fn generate_filename(&self) -> String {
        format!(
            "{}_{}.docx",
            self.metadata.doc_number.replace('/', "_"),
            self.metadata.doc_type.to_arabic()
        )
    }

    /// Generates the main content paragraph based on document type
    fn generate_content(&self) -> Paragraph {
        let content_text = match self.metadata.doc_type {
            DocumentType::ProcesVerbal => self.generate_pv_content(),
            DocumentType::Avertissement => self.generate_warning_content(),
            DocumentType::MiseEnDemeure => self.generate_notice_content(),
            DocumentType::DecisionFermeture => self.generate_closure_content(),
            DocumentType::Amende => self.generate_fine_content(),
            DocumentType::RapportControle => self.generate_report_content(),
            DocumentType::Convocation => self.generate_convocation_content(),
        };

        Paragraph::new()
            .add_run(Run::new().add_text(&content_text).size(24))
            .align(AlignmentType::Right)
    }

    /// Generates Procès-Verbal (inspection report) content
    fn generate_pv_content(&self) -> String {
        let mut content = templates::pv_header();

        if let Some(ref commerce) = self.commerce {
            content.push_str(&templates::commerce_section(commerce));
        }

        if let Some(ref inspection) = self.inspection {
            content.push_str(&templates::inspection_section(inspection));
        }

        content.push_str(&templates::signature_section(
            &self.metadata.inspector_name,
            &self.metadata.inspector_grade,
        ));

        content
    }

    /// Generates Avertissement (warning) content
    fn generate_warning_content(&self) -> String {
        let mut content = templates::warning_header();

        if let Some(ref commerce) = self.commerce {
            content.push_str(&templates::addressee_section(commerce));
        }

        content.push_str(&format!(
            "\n\n{}\n\n",
            self.metadata.content
        ));

        if let Some(ref action) = self.action {
            if !action.deadline_date.is_empty() {
                content.push_str(&format!(
                    "يتعين عليكم إصلاح الوضعية قبل تاريخ: {}\n\n",
                    action.deadline_date
                ));
            }
        }

        content.push_str(&templates::warning_footer());
        content.push_str(&templates::signature_section(
            &self.metadata.inspector_name,
            &self.metadata.inspector_grade,
        ));

        content
    }

    /// Generates Mise en Demeure (formal notice) content
    fn generate_notice_content(&self) -> String {
        let mut content = templates::notice_header();

        if let Some(ref commerce) = self.commerce {
            content.push_str(&templates::addressee_section(commerce));
        }

        content.push_str(&format!(
            "\n\n{}\n\n",
            self.metadata.content
        ));

        if let Some(ref action) = self.action {
            if !action.deadline_date.is_empty() {
                content.push_str(&format!(
                    "آخر أجل للامتثال هو: {}\n\n",
                    action.deadline_date
                ));
            }
        }

        if !self.metadata.legal_references.is_empty() {
            content.push_str(&format!(
                "المراجع القانونية: {}\n\n",
                self.metadata.legal_references.join("، ")
            ));
        }

        content.push_str(&templates::notice_footer());
        content.push_str(&templates::signature_section(
            &self.metadata.inspector_name,
            &self.metadata.inspector_grade,
        ));

        content
    }

    /// Generates Décision de Fermeture (closure decision) content
    fn generate_closure_content(&self) -> String {
        let mut content = templates::closure_header();

        if let Some(ref commerce) = self.commerce {
            content.push_str(&format!(
                "\nالمحل التجاري: {}\n",
                commerce.denomination
            ));
            content.push_str(&format!(
                "الكائن بـ: {}\n",
                commerce.establishment_address
            ));
            content.push_str(&format!(
                "صاحب المحل: {}\n\n",
                commerce.owner_name
            ));
        }

        content.push_str(&format!(
            "{}\n\n",
            self.metadata.content
        ));

        content.push_str(&templates::closure_footer());
        content.push_str(&templates::signature_section(
            &self.metadata.inspector_name,
            &self.metadata.inspector_grade,
        ));

        content
    }

    /// Generates Amende (fine) content
    fn generate_fine_content(&self) -> String {
        let amount = self.action.as_ref().map(|a| a.fine_amount).unwrap_or(0.0);
        let mut content = templates::fine_header();

        if let Some(ref commerce) = self.commerce {
            content.push_str(&templates::addressee_section(commerce));
        }

        content.push_str(&format!(
            "\n\n{}\n\n",
            self.metadata.content
        ));

        content.push_str(&format!(
            "مبلغ الغرامة: {:.2} درهم\n\n",
            amount
        ));

        if let Some(ref action) = self.action {
            if !action.deadline_date.is_empty() {
                content.push_str(&format!(
                    "يجب الأداء قبل تاريخ: {}\n\n",
                    action.deadline_date
                ));
            }
        }

        content.push_str(&templates::fine_footer());
        content.push_str(&templates::signature_section(
            &self.metadata.inspector_name,
            &self.metadata.inspector_grade,
        ));

        content
    }

    /// Generates Rapport de Contrôle (control report) content
    fn generate_report_content(&self) -> String {
        let mut content = templates::report_header();

        if let Some(ref commerce) = self.commerce {
            content.push_str(&templates::commerce_section(commerce));
        }

        content.push_str(&format!(
            "\nالملاحظات:\n{}\n\n",
            self.metadata.content
        ));

        content.push_str(&templates::signature_section(
            &self.metadata.inspector_name,
            &self.metadata.inspector_grade,
        ));

        content
    }

    /// Generates Convocation (summons) content
    fn generate_convocation_content(&self) -> String {
        let mut content = templates::convocation_header();

        if let Some(ref commerce) = self.commerce {
            content.push_str(&templates::addressee_section(commerce));
        }

        content.push_str(&format!(
            "\n\n{}\n\n",
            self.metadata.content
        ));

        content.push_str(&templates::convocation_footer());
        content.push_str(&templates::signature_section(
            &self.metadata.inspector_name,
            &self.metadata.inspector_grade,
        ));

        content
    }

    /// Creates the document header
    fn create_header(&self) -> Header {
        let hierarchy_text = format!(
            "المملكة المغربية\n{}\n{}\n{}",
            self.metadata.commune,
            self.metadata.arrondissement,
            self.metadata.service
        );

        let doc_info = format!(
            "رقم: {}\nالتاريخ: {}\nالساعة: {}\nالعون: {}\nالرتبة: {}",
            self.metadata.doc_number,
            self.metadata.creation_date,
            self.metadata.creation_time,
            self.metadata.inspector_name,
            self.metadata.inspector_grade
        );

        let header_table = Table::new(vec![TableRow::new(vec![
            TableCell::new().add_paragraph(
                Paragraph::new()
                    .add_run(Run::new().add_text(&hierarchy_text).size(20))
                    .align(AlignmentType::Right),
            ),
            TableCell::new().add_paragraph(
                Paragraph::new()
                    .add_run(Run::new().add_text(&doc_info).size(18))
                    .align(AlignmentType::Left),
            ),
        ])])
        .width(9000, WidthType::Dxa)
        .set_grid(vec![4500, 4500]);

        Header::new().add_table(header_table)
    }

    /// Creates the document footer
    fn create_footer(&self) -> Footer {
        let legal_refs = self.metadata.legal_references.join(" - ");
        let footer_text = format!(
            "UUID: {}\nصفحة [PAGE] من [TOTAL_PAGES]\n{}\nالمراجع القانونية: {}",
            self.metadata.id, self.metadata.disclaimer, legal_refs
        );

        Footer::new().add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text(&footer_text).size(16))
                .align(AlignmentType::Center),
        )
    }
}
