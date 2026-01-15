//! Excel database operations with proper data preservation

use crate::models::{
    CommerceInfo, DocumentMetadata, DocumentType, EnforcementAction, InspectionRecord,
};
use anyhow::{Context, Result};
use calamine::{open_workbook, DataType, Reader, Xlsx};
use chrono::{Datelike, Local};
use rust_xlsxwriter::{Color, Format, Workbook, Worksheet};
use std::path::Path;

/// Sheet names (Arabic - English)
const SHEET_DOCUMENTS: &str = "الوثائق - Documents";
const SHEET_COMMERCE: &str = "سجل المحلات - Commerce";
const SHEET_INSPECTIONS: &str = "المعاينات - Inspections";
const SHEET_ACTIONS: &str = "الإجراءات - Actions";

/// Excel database manager for the administrative police system
#[derive(Debug, Clone)]
pub struct ExcelDatabase {
    file_path: String,
}

impl ExcelDatabase {
    /// Creates a new ExcelDatabase instance
    pub fn new(file_path: &str) -> Self {
        ExcelDatabase {
            file_path: file_path.to_string(),
        }
    }

    /// Returns the file path
    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    /// Initializes the database file if it doesn't exist
    pub fn initialize(&self) -> Result<()> {
        if !Path::new(&self.file_path).exists() {
            let mut workbook = Workbook::new();

            let docs_sheet = workbook.add_worksheet();
            Self::create_documents_sheet(docs_sheet)?;
            
            let commerce_sheet = workbook.add_worksheet();
            Self::create_commerce_sheet(commerce_sheet)?;
            
            let inspections_sheet = workbook.add_worksheet();
            Self::create_inspections_sheet(inspections_sheet)?;
            
            let actions_sheet = workbook.add_worksheet();
            Self::create_actions_sheet(actions_sheet)?;

            workbook
                .save(&self.file_path)
                .context("Failed to save initial workbook")?;
        }
        Ok(())
    }

    // ==================== Sheet Creation ====================

    fn create_documents_sheet(worksheet: &mut Worksheet) -> Result<()> {
        worksheet.set_name(SHEET_DOCUMENTS)?;

        let header_format = Format::new()
            .set_bold()
            .set_background_color(Color::RGB(0xD9E1F2));

        let headers = [
            "ID",
            "رقم الوثيقة",
            "نوع الوثيقة",
            "التاريخ",
            "الوقت",
            "الجماعة",
            "المقاطعة",
            "المصلحة",
            "رقم المحل",
            "رقم المعاينة",
            "رقم الإجراء",
            "العون",
            "الرتبة",
        ];

        for (i, header) in headers.iter().enumerate() {
            worksheet.write_string_with_format(0, i as u16, *header, &header_format)?;
        }

        Ok(())
    }

    fn create_commerce_sheet(worksheet: &mut Worksheet) -> Result<()> {
        worksheet.set_name(SHEET_COMMERCE)?;

        let header_format = Format::new()
            .set_bold()
            .set_background_color(Color::RGB(0xE2EFDA));

        let headers = [
            "رقم المحل",
            "التسمية التجارية",
            "اسم المالك",
            "رقم ب.و",
            "الهاتف",
            "عنوان المالك",
            "عنوان المحل",
            "الحي",
            "نوع النشاط",
            "رقم ICE",
            "رقم الباطونطا",
            "رخصة؟",
            "رقم الرخصة",
            "تاريخ الرخصة",
        ];

        for (i, header) in headers.iter().enumerate() {
            worksheet.write_string_with_format(0, i as u16, *header, &header_format)?;
        }

        Ok(())
    }

    fn create_inspections_sheet(worksheet: &mut Worksheet) -> Result<()> {
        worksheet.set_name(SHEET_INSPECTIONS)?;

        let header_format = Format::new()
            .set_bold()
            .set_background_color(Color::RGB(0xFFF2CC));

        let headers = [
            "رقم المعاينة",
            "رقم المحل",
            "التاريخ",
            "الوقت",
            "اسم العون",
            "رتبة العون",
            "نوع المخالفة",
            "الوصف",
            "المرجع القانوني",
            "الإجراءات المتخذة",
        ];

        for (i, header) in headers.iter().enumerate() {
            worksheet.write_string_with_format(0, i as u16, *header, &header_format)?;
        }

        Ok(())
    }

    fn create_actions_sheet(worksheet: &mut Worksheet) -> Result<()> {
        worksheet.set_name(SHEET_ACTIONS)?;

        let header_format = Format::new()
            .set_bold()
            .set_background_color(Color::RGB(0xF8CBAD));

        let headers = [
            "رقم الإجراء",
            "رقم المحل",
            "رقم المعاينة",
            "نوع الإجراء",
            "التاريخ",
            "تاريخ الأجل",
            "مبلغ الغرامة",
            "الحالة",
            "تاريخ المتابعة",
            "ملاحظات",
        ];

        for (i, header) in headers.iter().enumerate() {
            worksheet.write_string_with_format(0, i as u16, *header, &header_format)?;
        }

        Ok(())
    }

    // ==================== Data Reading ====================

    /// Reads all data from a sheet as a vector of string vectors
    fn read_sheet_data(&self, sheet_name: &str) -> Result<Vec<Vec<String>>> {
        if !Path::new(&self.file_path).exists() {
            return Ok(Vec::new());
        }

        let mut workbook: Xlsx<_> =
            open_workbook(&self.file_path).context("Failed to open workbook")?;

        let mut data = Vec::new();

        if let Ok(range) = workbook.worksheet_range(sheet_name) {
            for (i, row) in range.rows().enumerate() {
                if i == 0 {
                    continue; // Skip header
                }
                let row_data: Vec<String> = row.iter().map(|cell| cell.to_string()).collect();
                if !row_data.iter().all(|s| s.is_empty()) {
                    data.push(row_data);
                }
            }
        }

        Ok(data)
    }

    /// Gets the next document number for the current year
    pub fn get_next_doc_number(&self) -> Result<String> {
        let current_year = Local::now().year();

        if !Path::new(&self.file_path).exists() {
            return Ok(format!("{}/0001", current_year));
        }

        let mut workbook: Xlsx<_> =
            open_workbook(&self.file_path).context("Failed to open workbook")?;

        if let Ok(range) = workbook.worksheet_range(SHEET_DOCUMENTS) {
            let mut max_number = 0;

            for row in range.rows().skip(1) {
                if let Some(cell) = row.get(1) {
                    if let Some(doc_num) = cell.as_string() {
                        if let Some(num_part) = doc_num
                            .strip_prefix(&format!("{}/", current_year))
                            .and_then(|s| s.parse::<i32>().ok())
                        {
                            max_number = max_number.max(num_part);
                        }
                    }
                }
            }

            Ok(format!("{}/{:04}", current_year, max_number + 1))
        } else {
            Ok(format!("{}/0001", current_year))
        }
    }

    // ==================== Commerce Operations ====================

    /// Saves a commerce record (preserving all other data)
    pub fn save_commerce(&self, commerce: &CommerceInfo) -> Result<()> {
        let docs_data = self.read_sheet_data(SHEET_DOCUMENTS)?;
        let mut commerce_data = self.read_sheet_data(SHEET_COMMERCE)?;
        let inspections_data = self.read_sheet_data(SHEET_INSPECTIONS)?;
        let actions_data = self.read_sheet_data(SHEET_ACTIONS)?;

        commerce_data.push(vec![
            commerce.commerce_id.clone(),
            commerce.denomination.clone(),
            commerce.owner_name.clone(),
            commerce.owner_cin.clone(),
            commerce.owner_phone.clone(),
            commerce.owner_address.clone(),
            commerce.establishment_address.clone(),
            commerce.quartier.clone(),
            commerce.activity_type.clone(),
            commerce.ice_number.clone(),
            commerce.patente_number.clone(),
            if commerce.has_autorisation {
                "نعم".to_string()
            } else {
                "لا".to_string()
            },
            commerce.autorisation_number.clone(),
            commerce.autorisation_date.clone(),
        ]);

        self.write_all_sheets(&docs_data, &commerce_data, &inspections_data, &actions_data)
    }

    /// Gets all commerce records
    pub fn get_all_commerces(&self) -> Result<Vec<CommerceInfo>> {
        let data = self.read_sheet_data(SHEET_COMMERCE)?;
        let mut commerces = Vec::new();

        for row in data {
            if row.len() >= 14 {
                commerces.push(CommerceInfo {
                    commerce_id: row.get(0).cloned().unwrap_or_default(),
                    denomination: row.get(1).cloned().unwrap_or_default(),
                    owner_name: row.get(2).cloned().unwrap_or_default(),
                    owner_cin: row.get(3).cloned().unwrap_or_default(),
                    owner_phone: row.get(4).cloned().unwrap_or_default(),
                    owner_address: row.get(5).cloned().unwrap_or_default(),
                    establishment_address: row.get(6).cloned().unwrap_or_default(),
                    quartier: row.get(7).cloned().unwrap_or_default(),
                    activity_type: row.get(8).cloned().unwrap_or_default(),
                    ice_number: row.get(9).cloned().unwrap_or_default(),
                    patente_number: row.get(10).cloned().unwrap_or_default(),
                    has_autorisation: row.get(11).map(|s| s == "نعم").unwrap_or(false),
                    autorisation_number: row.get(12).cloned().unwrap_or_default(),
                    autorisation_date: row.get(13).cloned().unwrap_or_default(),
                });
            }
        }

        Ok(commerces)
    }

    /// Finds a commerce by ID
    pub fn get_commerce_by_id(&self, commerce_id: &str) -> Result<Option<CommerceInfo>> {
        let commerces = self.get_all_commerces()?;
        Ok(commerces.into_iter().find(|c| c.commerce_id == commerce_id))
    }

    // ==================== Inspection Operations ====================

    /// Saves an inspection record (preserving all other data)
    pub fn save_inspection(&self, inspection: &InspectionRecord) -> Result<()> {
        let docs_data = self.read_sheet_data(SHEET_DOCUMENTS)?;
        let commerce_data = self.read_sheet_data(SHEET_COMMERCE)?;
        let mut inspections_data = self.read_sheet_data(SHEET_INSPECTIONS)?;
        let actions_data = self.read_sheet_data(SHEET_ACTIONS)?;

        let violations_str = inspection.violations_summary();

        inspections_data.push(vec![
            inspection.inspection_id.clone(),
            inspection.commerce_id.clone(),
            inspection.inspection_date.clone(),
            inspection.inspection_time.clone(),
            inspection.inspector_name.clone(),
            inspection.inspector_grade.clone(),
            violations_str,
            inspection.description.clone(),
            inspection.legal_reference.clone(),
            inspection.measures_taken.clone(),
        ]);

        self.write_all_sheets(&docs_data, &commerce_data, &inspections_data, &actions_data)
    }

    /// Gets all inspection records
    pub fn get_all_inspections(&self) -> Result<Vec<InspectionRecord>> {
        let data = self.read_sheet_data(SHEET_INSPECTIONS)?;
        let mut inspections = Vec::new();

        for row in data {
            if row.len() >= 10 {
                let mut inspection = InspectionRecord::new(row.get(1).cloned().unwrap_or_default());
                inspection.inspection_id = row.get(0).cloned().unwrap_or_default();
                inspection.inspection_date = row.get(2).cloned().unwrap_or_default();
                inspection.inspection_time = row.get(3).cloned().unwrap_or_default();
                inspection.inspector_name = row.get(4).cloned().unwrap_or_default();
                inspection.inspector_grade = row.get(5).cloned().unwrap_or_default();
                // Note: violations are stored as summary string, would need parsing
                inspection.description = row.get(7).cloned().unwrap_or_default();
                inspection.legal_reference = row.get(8).cloned().unwrap_or_default();
                inspection.measures_taken = row.get(9).cloned().unwrap_or_default();
                inspections.push(inspection);
            }
        }

        Ok(inspections)
    }

    // ==================== Action Operations ====================

    /// Saves an enforcement action (preserving all other data)
    pub fn save_action(&self, action: &EnforcementAction) -> Result<()> {
        let docs_data = self.read_sheet_data(SHEET_DOCUMENTS)?;
        let commerce_data = self.read_sheet_data(SHEET_COMMERCE)?;
        let inspections_data = self.read_sheet_data(SHEET_INSPECTIONS)?;
        let mut actions_data = self.read_sheet_data(SHEET_ACTIONS)?;

        actions_data.push(vec![
            action.action_id.clone(),
            action.commerce_id.clone(),
            action.inspection_id.clone(),
            action.action_type.to_arabic().to_string(),
            action.action_date.clone(),
            action.deadline_date.clone(),
            action.fine_amount.to_string(),
            action.status.to_arabic().to_string(),
            action.follow_up_date.clone(),
            action.notes.clone(),
        ]);

        self.write_all_sheets(&docs_data, &commerce_data, &inspections_data, &actions_data)
    }

    /// Gets all enforcement actions
    pub fn get_all_actions(&self) -> Result<Vec<EnforcementAction>> {
        let data = self.read_sheet_data(SHEET_ACTIONS)?;
        let mut actions = Vec::new();

        for row in data {
            if row.len() >= 10 {
                let mut action = EnforcementAction::default();
                action.action_id = row.get(0).cloned().unwrap_or_default();
                action.commerce_id = row.get(1).cloned().unwrap_or_default();
                action.inspection_id = row.get(2).cloned().unwrap_or_default();
                // action_type would need parsing from Arabic
                action.action_date = row.get(4).cloned().unwrap_or_default();
                action.deadline_date = row.get(5).cloned().unwrap_or_default();
                action.fine_amount = row.get(6).and_then(|s| s.parse().ok()).unwrap_or(0.0);
                // status would need parsing from Arabic
                action.follow_up_date = row.get(8).cloned().unwrap_or_default();
                action.notes = row.get(9).cloned().unwrap_or_default();
                actions.push(action);
            }
        }

        Ok(actions)
    }

    // ==================== Document Metadata Operations ====================

    /// Saves document metadata (preserving all other data)
    pub fn save_metadata(&self, metadata: &DocumentMetadata) -> Result<()> {
        let mut docs_data = self.read_sheet_data(SHEET_DOCUMENTS)?;
        let commerce_data = self.read_sheet_data(SHEET_COMMERCE)?;
        let inspections_data = self.read_sheet_data(SHEET_INSPECTIONS)?;
        let actions_data = self.read_sheet_data(SHEET_ACTIONS)?;

        docs_data.push(vec![
            metadata.id.clone(),
            metadata.doc_number.clone(),
            metadata.doc_type.to_arabic().to_string(),
            metadata.creation_date.clone(),
            metadata.creation_time.clone(),
            metadata.commune.clone(),
            metadata.arrondissement.clone(),
            metadata.service.clone(),
            metadata.commerce_id.clone(),
            metadata.inspection_id.clone(),
            metadata.action_id.clone(),
            metadata.inspector_name.clone(),
            metadata.inspector_grade.clone(),
        ]);

        self.write_all_sheets(&docs_data, &commerce_data, &inspections_data, &actions_data)
    }

    /// Gets all document metadata records
    pub fn get_all_documents(&self) -> Result<Vec<DocumentMetadata>> {
        let data = self.read_sheet_data(SHEET_DOCUMENTS)?;
        let mut documents = Vec::new();

        for row in data {
            if row.len() >= 13 {
                let mut doc = DocumentMetadata::new(DocumentType::ProcesVerbal, String::new());
                doc.id = row.get(0).cloned().unwrap_or_default();
                doc.doc_number = row.get(1).cloned().unwrap_or_default();
                // doc_type would need parsing from Arabic
                doc.creation_date = row.get(3).cloned().unwrap_or_default();
                doc.creation_time = row.get(4).cloned().unwrap_or_default();
                doc.commune = row.get(5).cloned().unwrap_or_default();
                doc.arrondissement = row.get(6).cloned().unwrap_or_default();
                doc.service = row.get(7).cloned().unwrap_or_default();
                doc.commerce_id = row.get(8).cloned().unwrap_or_default();
                doc.inspection_id = row.get(9).cloned().unwrap_or_default();
                doc.action_id = row.get(10).cloned().unwrap_or_default();
                doc.inspector_name = row.get(11).cloned().unwrap_or_default();
                doc.inspector_grade = row.get(12).cloned().unwrap_or_default();
                documents.push(doc);
            }
        }

        Ok(documents)
    }

    // ==================== Helper Methods ====================

    /// Writes all sheet data back to the workbook (fixes data loss bug)
    fn write_all_sheets(
        &self,
        docs_data: &[Vec<String>],
        commerce_data: &[Vec<String>],
        inspections_data: &[Vec<String>],
        actions_data: &[Vec<String>],
    ) -> Result<()> {
        let mut workbook = Workbook::new();

        // Documents sheet
        {
            let docs_sheet = workbook.add_worksheet();
            Self::create_documents_sheet(docs_sheet)?;
            for (i, row) in docs_data.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    docs_sheet.write_string((i + 1) as u32, j as u16, cell)?;
                }
            }
        }

        // Commerce sheet
        {
            let commerce_sheet = workbook.add_worksheet();
            Self::create_commerce_sheet(commerce_sheet)?;
            for (i, row) in commerce_data.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    commerce_sheet.write_string((i + 1) as u32, j as u16, cell)?;
                }
            }
        }

        // Inspections sheet
        {
            let inspections_sheet = workbook.add_worksheet();
            Self::create_inspections_sheet(inspections_sheet)?;
            for (i, row) in inspections_data.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    inspections_sheet.write_string((i + 1) as u32, j as u16, cell)?;
                }
            }
        }

        // Actions sheet
        {
            let actions_sheet = workbook.add_worksheet();
            Self::create_actions_sheet(actions_sheet)?;
            for (i, row) in actions_data.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    actions_sheet.write_string((i + 1) as u32, j as u16, cell)?;
                }
            }
        }

        workbook
            .save(&self.file_path)
            .context("Failed to save workbook")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_database_initialization() {
        let test_path = "/tmp/test_police_admin.xlsx";
        let _ = fs::remove_file(test_path);

        let db = ExcelDatabase::new(test_path);
        assert!(db.initialize().is_ok());
        assert!(Path::new(test_path).exists());

        let _ = fs::remove_file(test_path);
    }

    #[test]
    fn test_save_commerce_preserves_data() {
        let test_path = "/tmp/test_commerce_preserve.xlsx";
        let _ = fs::remove_file(test_path);

        let db = ExcelDatabase::new(test_path);
        db.initialize().unwrap();

        // Save first commerce
        let commerce1 = CommerceInfo {
            commerce_id: "test-1".to_string(),
            denomination: "Test Shop 1".to_string(),
            owner_name: "Owner 1".to_string(),
            ..Default::default()
        };
        db.save_commerce(&commerce1).unwrap();

        // Save second commerce
        let commerce2 = CommerceInfo {
            commerce_id: "test-2".to_string(),
            denomination: "Test Shop 2".to_string(),
            owner_name: "Owner 2".to_string(),
            ..Default::default()
        };
        db.save_commerce(&commerce2).unwrap();

        // Verify both exist
        let commerces = db.get_all_commerces().unwrap();
        assert_eq!(commerces.len(), 2);

        let _ = fs::remove_file(test_path);
    }
}
