use docx_rs::*;
use rust_xlsxwriter::{Workbook, Worksheet, Format, Color};
use calamine::{Reader, open_workbook, Xlsx, DataType};
use uuid::Uuid;
use chrono::{Local, Datelike};
use std::path::Path;
use std::io::{self, Write};

// Document types for Municipal Administrative Police
#[derive(Debug, Clone)]
enum DocumentType {
    ProcesVerbal,           // محضر معاينة
    Avertissement,          // إنذار - Warning
    MiseEnDemeure,          // إعذار - Formal Notice
    DecisionFermeture,      // قرار غلق - Closure Decision
    Amende,                 // غرامة - Fine
    RapportControle,        // تقرير مراقبة - Inspection Report
    Convocation,            // استدعاء - Summons
}

impl DocumentType {
    fn to_arabic(&self) -> &str {
        match self {
            DocumentType::ProcesVerbal => "محضر معاينة",
            DocumentType::Avertissement => "إنذار",
            DocumentType::MiseEnDemeure => "إعذار",
            DocumentType::DecisionFermeture => "قرار غلق",
            DocumentType::Amende => "غرامة إدارية",
            DocumentType::RapportControle => "تقرير مراقبة",
            DocumentType::Convocation => "استدعاء",
        }
    }
}

// Violation types
#[derive(Debug, Clone)]
enum ViolationType {
    SansAutorisation,           // بدون رخصة - No permit
    OccupationVoiePublique,     // احتلال الطريق العام - Public space occupation
    NuisancesSonores,           // إزعاج صوتي - Noise nuisance
    Insalubrite,                // عدم النظافة - Unsanitary conditions
    NonRespectHoraires,         // عدم احترام أوقات العمل - Schedule violations
    VenteAlcoolSansLicence,     // بيع الخمور بدون رخصة - Alcohol without license
    ConstructionIllegale,       // بناء غير قانوني - Illegal construction
    PubliciteNonAutorisee,      // دعاية غير مرخصة - Unauthorized advertising
    Autre,                      // أخرى - Other
}

impl ViolationType {
    fn to_arabic(&self) -> &str {
        match self {
            ViolationType::SansAutorisation => "بدون رخصة",
            ViolationType::OccupationVoiePublique => "احتلال الطريق العام",
            ViolationType::NuisancesSonores => "إزعاج صوتي",
            ViolationType::Insalubrite => "عدم النظافة",
            ViolationType::NonRespectHoraires => "عدم احترام أوقات العمل",
            ViolationType::VenteAlcoolSansLicence => "بيع الخمور بدون رخصة",
            ViolationType::ConstructionIllegale => "بناء غير قانوني",
            ViolationType::PubliciteNonAutorisee => "دعاية غير مرخصة",
            ViolationType::Autre => "أخرى",
        }
    }

    fn get_legal_reference(&self) -> &str {
        match self {
            ViolationType::SansAutorisation => "المادة 39 من القانون رقم 78.00",
            ViolationType::OccupationVoiePublique => "المادة 7 من قانون الملك العمومي",
            ViolationType::NuisancesSonores => "المادة 77 من قانون البيئة 11.03",
            ViolationType::Insalubrite => "المادة 84 من القانون رقم 28.00",
            ViolationType::NonRespectHoraires => "المادة 2 من قرار وزير الداخلية",
            ViolationType::VenteAlcoolSansLicence => "ظهير 1967",
            ViolationType::ConstructionIllegale => "المادة 59 من قانون التعمير 12.90",
            ViolationType::PubliciteNonAutorisee => "القانون رقم 78.00",
            ViolationType::Autre => "حسب القانون المعمول به",
        }
    }
}

// Enforcement action status
#[derive(Debug, Clone)]
enum ActionStatus {
    EnCours,        // جاري - In progress
    Conforme,       // متوافق - Compliant
    NonConforme,    // غير متوافق - Non-compliant
    Ferme,          // مغلق - Closed
    AmendePaye,     // الغرامة مدفوعة - Fine paid
    ContentieuxJudiciaire, // نزاع قضائي - Legal dispute
}

impl ActionStatus {
    fn to_arabic(&self) -> &str {
        match self {
            ActionStatus::EnCours => "جاري",
            ActionStatus::Conforme => "متوافق",
            ActionStatus::NonConforme => "غير متوافق",
            ActionStatus::Ferme => "مغلق",
            ActionStatus::AmendePaye => "الغرامة مدفوعة",
            ActionStatus::ContentieuxJudiciaire => "نزاع قضائي",
        }
    }
}

// Commerce/Establishment information
#[derive(Debug, Clone)]
struct CommerceInfo {
    commerce_id: String,
    denomination: String,           // التسمية التجارية
    owner_name: String,             // اسم المالك
    owner_cin: String,              // رقم البطاقة الوطنية
    owner_phone: String,            // رقم الهاتف
    owner_address: String,          // عنوان المالك
    establishment_address: String,  // عنوان المحل
    quartier: String,               // الحي
    activity_type: String,          // نوع النشاط
    ice_number: String,             // رقم السجل التجاري - ICE
    patente_number: String,         // رقم رخصة الباطونطا
    has_autorisation: bool,         // هل يتوفر على رخصة
    autorisation_number: String,    // رقم الرخصة
    autorisation_date: String,      // تاريخ الرخصة
}

// Inspection/Violation record
#[derive(Debug, Clone)]
struct InspectionRecord {
    inspection_id: String,
    commerce_id: String,
    inspection_date: String,
    inspection_time: String,
    inspector_name: String,         // اسم العون
    inspector_grade: String,        // رتبة العون
    violation_types: Vec<ViolationType>,
    description: String,            // وصف المخالفة
    legal_reference: String,        // المرجع القانوني
    photos_references: Vec<String>, // مراجع الصور
    witnesses: Vec<String>,         // الشهود
    measures_taken: String,         // الإجراءات المتخذة
}

// Enforcement action
#[derive(Debug, Clone)]
struct EnforcementAction {
    action_id: String,
    commerce_id: String,
    inspection_id: String,
    action_type: DocumentType,
    action_date: String,
    deadline_date: String,          // تاريخ الأجل
    fine_amount: f64,               // مبلغ الغرامة
    status: ActionStatus,
    follow_up_date: String,         // تاريخ المتابعة
    notes: String,
}

// Document metadata structure (enhanced)
#[derive(Debug, Clone)]
struct DocumentMetadata {
    id: String,
    doc_number: String,
    doc_type: DocumentType,
    creation_date: String,
    creation_time: String,
    commune: String,                // الجماعة
    arrondissement: String,         // المقاطعة
    service: String,                // المصلحة - Service (Police Administrative)
    commerce_id: String,
    inspection_id: String,
    action_id: String,
    content: String,
    legal_references: Vec<String>,
    inspector_name: String,
    inspector_grade: String,
    disclaimer: String,
}

// Excel Database Manager (Enhanced)
struct ExcelDatabase {
    file_path: String,
}

impl ExcelDatabase {
    fn new(file_path: &str) -> Self {
        ExcelDatabase {
            file_path: file_path.to_string(),
        }
    }

    fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !Path::new(&self.file_path).exists() {
            let mut workbook = Workbook::new();

            // Sheet 1: Documents
            self.create_documents_sheet(&mut workbook)?;

            // Sheet 2: Commerce Registry
            self.create_commerce_sheet(&mut workbook)?;

            // Sheet 3: Inspections
            self.create_inspections_sheet(&mut workbook)?;

            // Sheet 4: Enforcement Actions
            self.create_actions_sheet(&mut workbook)?;

            workbook.save(&self.file_path)?;
        }
        Ok(())
    }

    fn create_documents_sheet(&self, workbook: &mut Workbook) -> Result<(), Box<dyn std::error::Error>> {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name("الوثائق - Documents")?;

        let header_format = Format::new().set_bold().set_background_color(Color::RGB(0xD9E1F2));

        let headers = vec![
            "ID", "رقم الوثيقة", "نوع الوثيقة", "التاريخ", "الوقت",
            "الجماعة", "المقاطعة", "المصلحة", "رقم المحل",
            "رقم المعاينة", "رقم الإجراء", "العون", "الرتبة"
        ];

        for (i, header) in headers.iter().enumerate() {
            worksheet.write_string_with_format(0, i as u16, *header, &header_format)?;
        }

        Ok(())
    }

    fn create_commerce_sheet(&self, workbook: &mut Workbook) -> Result<(), Box<dyn std::error::Error>> {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name("سجل المحلات - Commerce")?;

        let header_format = Format::new().set_bold().set_background_color(Color::RGB(0xE2EFDA));

        let headers = vec![
            "رقم المحل", "التسمية التجارية", "اسم المالك", "رقم ب.و",
            "الهاتف", "عنوان المالك", "عنوان المحل", "الحي",
            "نوع النشاط", "رقم ICE", "رقم الباطونطا",
            "رخصة؟", "رقم الرخصة", "تاريخ الرخصة"
        ];

        for (i, header) in headers.iter().enumerate() {
            worksheet.write_string_with_format(0, i as u16, *header, &header_format)?;
        }

        Ok(())
    }

    fn create_inspections_sheet(&self, workbook: &mut Workbook) -> Result<(), Box<dyn std::error::Error>> {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name("المعاينات - Inspections")?;

        let header_format = Format::new().set_bold().set_background_color(Color::RGB(0xFFF2CC));

        let headers = vec![
            "رقم المعاينة", "رقم المحل", "التاريخ", "الوقت",
            "اسم العون", "رتبة العون", "نوع المخالفة",
            "الوصف", "المرجع القانوني", "الإجراءات المتخذة"
        ];

        for (i, header) in headers.iter().enumerate() {
            worksheet.write_string_with_format(0, i as u16, *header, &header_format)?;
        }

        Ok(())
    }

    fn create_actions_sheet(&self, workbook: &mut Workbook) -> Result<(), Box<dyn std::error::Error>> {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name("الإجراءات - Actions")?;

        let header_format = Format::new().set_bold().set_background_color(Color::RGB(0xF8CBAD));

        let headers = vec![
            "رقم الإجراء", "رقم المحل", "رقم المعاينة",
            "نوع الإجراء", "التاريخ", "تاريخ الأجل",
            "مبلغ الغرامة", "الحالة", "تاريخ المتابعة", "ملاحظات"
        ];

        for (i, header) in headers.iter().enumerate() {
            worksheet.write_string_with_format(0, i as u16, *header, &header_format)?;
        }

        Ok(())
    }

    fn get_next_doc_number(&self) -> Result<String, Box<dyn std::error::Error>> {
        let current_year = Local::now().year();

        if !Path::new(&self.file_path).exists() {
            return Ok(format!("{}/0001", current_year));
        }

        let mut workbook: Xlsx<_> = open_workbook(&self.file_path)?;

        if let Ok(range) = workbook.worksheet_range("الوثائق - Documents") {
            let mut max_number = 0;

            for row in range.rows().skip(1) {
                if let Some(cell) = row.get(1) {
                    if let Some(doc_num) = cell.get_string() {
                        if doc_num.starts_with(&format!("{}/", current_year)) {
                            if let Some(num_part) = doc_num.split('/').nth(1) {
                                if let Ok(num) = num_part.parse::<i32>() {
                                    max_number = max_number.max(num);
                                }
                            }
                        }
                    }
                }
            }

            Ok(format!("{}/{:04}", current_year, max_number + 1))
        } else {
            Ok(format!("{}/0001", current_year))
        }
    }

    fn save_commerce(&self, commerce: &CommerceInfo) -> Result<(), Box<dyn std::error::Error>> {
        let mut workbook: Xlsx<_> = open_workbook(&self.file_path)?;
        let mut existing_data: Vec<Vec<String>> = Vec::new();

        if let Ok(range) = workbook.worksheet_range("سجل المحلات - Commerce") {
            for (i, row) in range.rows().enumerate() {
                if i == 0 { continue; }
                let row_data: Vec<String> = row.iter().map(|cell| cell.to_string()).collect();
                existing_data.push(row_data);
            }
        }

        let mut new_workbook = Workbook::new();
        self.create_documents_sheet(&mut new_workbook)?;

        let worksheet = new_workbook.add_worksheet();
        worksheet.set_name("سجل المحلات - Commerce")?;

        let header_format = Format::new().set_bold().set_background_color(Color::RGB(0xE2EFDA));
        let headers = vec![
            "رقم المحل", "التسمية التجارية", "اسم المالك", "رقم ب.و",
            "الهاتف", "عنوان المالك", "عنوان المحل", "الحي",
            "نوع النشاط", "رقم ICE", "رقم الباطونطا",
            "رخصة؟", "رقم الرخصة", "تاريخ الرخصة"
        ];

        for (i, header) in headers.iter().enumerate() {
            worksheet.write_string_with_format(0, i as u16, *header, &header_format)?;
        }

        for (i, row) in existing_data.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                worksheet.write_string((i + 1) as u32, j as u16, cell)?;
            }
        }

        let row = (existing_data.len() + 1) as u32;
        worksheet.write_string(row, 0, &commerce.commerce_id)?;
        worksheet.write_string(row, 1, &commerce.denomination)?;
        worksheet.write_string(row, 2, &commerce.owner_name)?;
        worksheet.write_string(row, 3, &commerce.owner_cin)?;
        worksheet.write_string(row, 4, &commerce.owner_phone)?;
        worksheet.write_string(row, 5, &commerce.owner_address)?;
        worksheet.write_string(row, 6, &commerce.establishment_address)?;
        worksheet.write_string(row, 7, &commerce.quartier)?;
        worksheet.write_string(row, 8, &commerce.activity_type)?;
        worksheet.write_string(row, 9, &commerce.ice_number)?;
        worksheet.write_string(row, 10, &commerce.patente_number)?;
        worksheet.write_string(row, 11, if commerce.has_autorisation { "نعم" } else { "لا" })?;
        worksheet.write_string(row, 12, &commerce.autorisation_number)?;
        worksheet.write_string(row, 13, &commerce.autorisation_date)?;

        self.create_inspections_sheet(&mut new_workbook)?;
        self.create_actions_sheet(&mut new_workbook)?;

        new_workbook.save(&self.file_path)?;
        Ok(())
    }

    fn save_inspection(&self, inspection: &InspectionRecord) -> Result<(), Box<dyn std::error::Error>> {
        // Similar implementation for inspections sheet
        // This would append to the inspections sheet
        Ok(())
    }

    fn save_action(&self, action: &EnforcementAction) -> Result<(), Box<dyn std::error::Error>> {
        // Similar implementation for actions sheet
        Ok(())
    }

    fn save_metadata(&self, metadata: &DocumentMetadata) -> Result<(), Box<dyn std::error::Error>> {
        // Save to documents sheet
        Ok(())
    }
}

// Document Generator (Enhanced)
struct DocumentGenerator {
    metadata: DocumentMetadata,
    commerce: Option<CommerceInfo>,
    inspection: Option<InspectionRecord>,
    action: Option<EnforcementAction>,
}

impl DocumentGenerator {
    fn new(
        metadata: DocumentMetadata,
        commerce: Option<CommerceInfo>,
        inspection: Option<InspectionRecord>,
        action: Option<EnforcementAction>,
    ) -> Self {
        DocumentGenerator { metadata, commerce, inspection, action }
    }

    fn create_document(&self, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut docx = Docx::new();

        let header = self.create_header();
        let footer = self.create_footer();

        // Generate content based on document type
        let content = self.generate_content();

        let doc = docx
            .header(header)
            .footer(footer)
            .add_paragraph(content)
            .page_size(3300, 4680)
            .page_margin(PageMargin::new().top(720).right(720).bottom(720).left(720));

        let file = std::fs::File::create(output_path)?;
        doc.build().pack(file)?;

        Ok(())
    }

    fn generate_content(&self) -> Paragraph {
        let content_text = match self.metadata.doc_type {
            DocumentType::ProcesVerbal => self.generate_pv_content(),
            DocumentType::Avertissement => self.generate_warning_content(),
            DocumentType::MiseEnDemeure => self.generate_notice_content(),
            DocumentType::DecisionFermeture => self.generate_closure_content(),
            DocumentType::Amende => self.generate_fine_content(),
            _ => self.metadata.content.clone(),
        };

        Paragraph::new()
            .add_run(Run::new().add_text(&content_text).size(24))
            .align(AlignmentType::Right)
    }

    fn generate_pv_content(&self) -> String {
        let mut content = String::new();
        content.push_str("محضر معاينة\n\n");

        if let Some(ref commerce) = self.commerce {
            content.push_str(&format!("المحل: {}\n", commerce.denomination));
            content.push_str(&format!("المالك: {}\n", commerce.owner_name));
            content.push_str(&format!("رقم البطاقة الوطنية: {}\n", commerce.owner_cin));
            content.push_str(&format!("العنوان: {}\n", commerce.establishment_address));
            content.push_str(&format!("الحي: {}\n\n", commerce.quartier));
        }

        if let Some(ref inspection) = self.inspection {
            content.push_str(&format!("المخالفة المعاينة:\n{}\n\n", inspection.description));
            content.push_str(&format!("المرجع القانوني: {}\n\n", inspection.legal_reference));
            content.push_str(&format!("الإجراءات المتخذة:\n{}\n", inspection.measures_taken));
        }

        content
    }

    fn generate_warning_content(&self) -> String {
        format!(
            "إنذار\n\nبناء على المعاينة المحررة بتاريخ...\n\n{}\n\nيتعين عليكم إصلاح الوضعية داخل أجل...",
            self.metadata.content
        )
    }

    fn generate_notice_content(&self) -> String {
        format!(
            "إعذار\n\nنظرا لعدم الامتثال للإنذار السابق...\n\n{}\n\nآخر أجل للامتثال هو...",
            self.metadata.content
        )
    }

    fn generate_closure_content(&self) -> String {
        format!(
            "قرار غلق إداري\n\nبناء على عدم الامتثال للإعذارات...\n\n{}\n\nيقرر غلق المحل إلى حين...",
            self.metadata.content
        )
    }

    fn generate_fine_content(&self) -> String {
        let amount = self.action.as_ref().map(|a| a.fine_amount).unwrap_or(0.0);
        format!(
            "غرامة إدارية\n\n{}\n\nمبلغ الغرامة: {} درهم\n\nيجب الأداء داخل أجل...",
            self.metadata.content, amount
        )
    }

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

        let header_table = Table::new(vec![
            TableRow::new(vec![
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new()
                            .add_run(Run::new().add_text(&hierarchy_text).size(20))
                            .align(AlignmentType::Right)
                    ),
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new()
                            .add_run(Run::new().add_text(&doc_info).size(18))
                            .align(AlignmentType::Left)
                    ),
            ]),
        ])
        .width(9000, WidthType::Dxa)
        .set_grid(vec![4500, 4500]);

        Header::new().add_table(header_table)
    }

    fn create_footer(&self) -> Footer {
        let legal_refs = self.metadata.legal_references.join(" - ");
        let footer_text = format!(
            "UUID: {}\nصفحة [PAGE] من [TOTAL_PAGES]\n{}\nالمراجع القانونية: {}",
            self.metadata.id,
            self.metadata.disclaimer,
            legal_refs
        );

        Footer::new().add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text(&footer_text).size(16))
                .align(AlignmentType::Center)
        )
    }
}

// Form input handler
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn select_document_type() -> DocumentType {
    println!("\n=== نوع الوثيقة / Document Type ===");
    println!("1. محضر معاينة (Procès-Verbal)");
    println!("2. إنذار (Avertissement)");
    println!("3. إعذار (Mise en Demeure)");
    println!("4. قرار غلق (Décision de Fermeture)");
    println!("5. غرامة إدارية (Amende)");
    println!("6. تقرير مراقبة (Rapport de Contrôle)");
    println!("7. استدعاء (Convocation)");

    loop {
        let choice = get_user_input("الاختيار / Choice (1-7): ");
        match choice.as_str() {
            "1" => return DocumentType::ProcesVerbal,
            "2" => return DocumentType::Avertissement,
            "3" => return DocumentType::MiseEnDemeure,
            "4" => return DocumentType::DecisionFermeture,
            "5" => return DocumentType::Amende,
            "6" => return DocumentType::RapportControle,
            "7" => return DocumentType::Convocation,
            _ => println!("اختيار غير صالح / Invalid choice"),
        }
    }
}

fn select_violation_type() -> ViolationType {
    println!("\n=== نوع المخالفة / Violation Type ===");
    println!("1. بدون رخصة (Sans Autorisation)");
    println!("2. احتلال الطريق العام (Occupation Voie Publique)");
    println!("3. إزعاج صوتي (Nuisances Sonores)");
    println!("4. عدم النظافة (Insalubrité)");
    println!("5. عدم احترام أوقات العمل (Non-respect Horaires)");
    println!("6. بيع الخمور بدون رخصة (Vente Alcool Sans Licence)");
    println!("7. بناء غير قانوني (Construction Illégale)");
    println!("8. دعاية غير مرخصة (Publicité Non Autorisée)");
    println!("9. أخرى (Autre)");

    loop {
        let choice = get_user_input("الاختيار / Choice (1-9): ");
        match choice.as_str() {
            "1" => return ViolationType::SansAutorisation,
            "2" => return ViolationType::OccupationVoiePublique,
            "3" => return ViolationType::NuisancesSonores,
            "4" => return ViolationType::Insalubrite,
            "5" => return ViolationType::NonRespectHoraires,
            "6" => return ViolationType::VenteAlcoolSansLicence,
            "7" => return ViolationType::ConstructionIllegale,
            "8" => return ViolationType::PubliciteNonAutorisee,
            "9" => return ViolationType::Autre,
            _ => println!("اختيار غير صالح / Invalid choice"),
        }
    }
}

fn collect_commerce_info() -> CommerceInfo {
    println!("\n=== معلومات المحل / Commerce Information ===");

    CommerceInfo {
        commerce_id: Uuid::new_v4().to_string(),
        denomination: get_user_input("التسمية التجارية / Trade Name: "),
        owner_name: get_user_input("اسم المالك / Owner Name: "),
        owner_cin: get_user_input("رقم البطاقة الوطنية / CIN: "),
        owner_phone: get_user_input("رقم الهاتف / Phone: "),
        owner_address: get_user_input("عنوان المالك / Owner Address: "),
        establishment_address: get_user_input("عنوان المحل / Establishment Address: "),
        quartier: get_user_input("الحي / District: "),
        activity_type: get_user_input("نوع النشاط / Activity Type: "),
        ice_number: get_user_input("رقم ICE / ICE Number: "),
        patente_number: get_user_input("رقم الباطونطا / Patente Number: "),
        has_autorisation: get_user_input("هل يتوفر على رخصة؟ / Has Authorization? (y/n): ").to_lowercase() == "y",
        autorisation_number: get_user_input("رقم الرخصة / Authorization Number: "),
        autorisation_date: get_user_input("تاريخ الرخصة / Authorization Date: "),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== نظام إدارة الشرطة الإدارية الجماعية ===");
    println!("=== Municipal Administrative Police Management System ===\n");

    let db = ExcelDatabase::new("police_administrative.xlsx");
    db.initialize()?;

    loop {
        println!("\n--- إجراء جديد / New Action ---");

        let doc_type = select_document_type();

        let commerce = collect_commerce_info();
        db.save_commerce(&commerce)?;

        let violation_type = select_violation_type();
        let legal_ref = violation_type.get_legal_reference().to_string();

        println!("\n=== معلومات المعاينة / Inspection Details ===");
        let inspector_name = get_user_input("اسم العون / Inspector Name: ");
        let inspector_grade = get_user_input("رتبة العون / Inspector Grade: ");
        let description = get_user_input("وصف المخالفة / Violation Description: ");
        let measures = get_user_input("الإجراءات المتخذة / Measures Taken: ");

        let now = Local::now();
        let inspection = InspectionRecord {
            inspection_id: Uuid::new_v4().to_string(),
            commerce_id: commerce.commerce_id.clone(),
            inspection_date: now.format("%Y-%m-%d").to_string(),
            inspection_time: now.format("%H:%M:%S").to_string(),
            inspector_name: inspector_name.clone(),
            inspector_grade: inspector_grade.clone(),
            violation_types: vec![violation_type],
            description: description.clone(),
            legal_reference: legal_ref.clone(),
            photos_references: vec![],
            witnesses: vec![],
            measures_taken: measures,
        };

        let fine_amount = if matches!(doc_type, DocumentType::Amende) {
            get_user_input("مبلغ الغرامة / Fine Amount (DH): ")
                .parse::<f64>()
                .unwrap_or(0.0)
        } else {
            0.0
        };

        let action = EnforcementAction {
            action_id: Uuid::new_v4().to_string(),
            commerce_id: commerce.commerce_id.clone(),
            inspection_id: inspection.inspection_id.clone(),
            action_type: doc_type.clone(),
            action_date: now.format("%Y-%m-%d").to_string(),
            deadline_date: get_user_input("تاريخ الأجل / Deadline Date (YYYY-MM-DD): "),
            fine_amount,
            status: ActionStatus::EnCours,
            follow_up_date: String::new(),
            notes: String::new(),
        };

        let metadata = DocumentMetadata {
            id: Uuid::new_v4().to_string(),
            doc_number: db.get_next_doc_number()?,
            doc_type: doc_type.clone(),
            creation_date: now.format("%Y-%m-%d").to_string(),
            creation_time: now.format("%H:%M:%S").to_string(),
            commune: get_user_input("الجماعة / Commune: "),
            arrondissement: get_user_input("المقاطعة / Arrondissement: "),
            service: "الشرطة الإدارية".to_string(),
            commerce_id: commerce.commerce_id.clone(),
            inspection_id: inspection.inspection_id.clone(),
            action_id: action.action_id.clone(),
            content: description,
            legal_references: vec![legal_ref],
            inspector_name,
            inspector_grade,
            disclaimer: "وثيقة رسمية - لا يمكن استعمالها إلا للأغراض القانونية".to_string(),
        };

        let output_filename = format!("{}_{}.docx",
            metadata.doc_number.replace("/", "_"),
            doc_type.to_arabic()
        );

        let generator = DocumentGenerator::new(metadata, Some(commerce), Some(inspection), Some(action));
        generator.create_document(&output_filename)?;

        println!("✓ تم إنشاء الوثيقة: {} / Document created: {}", output_filename, output_filename);

        let continue_choice = get_user_input("\nإجراء آخر؟ / Another action? (y/n): ");
        if continue_choice.to_lowercase() != "y" {
            break;
        }
    }

    println!("\nشكراً / Thank you!");
    Ok(())
}
