//! Command-line interface for the application

use crate::database::ExcelDatabase;
use crate::generator::DocumentGenerator;
use crate::models::{
    ActionStatus, CommerceInfo, DocumentMetadata, DocumentType, EnforcementAction,
    InspectionRecord, ViolationType,
};
use anyhow::Result;
use chrono::Local;
use std::io::{self, Write};

/// Runs the CLI mode
pub fn run_cli() -> Result<()> {
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
            inspection_id: uuid::Uuid::new_v4().to_string(),
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
        db.save_inspection(&inspection)?;

        let fine_amount = if matches!(doc_type, DocumentType::Amende) {
            get_user_input("مبلغ الغرامة / Fine Amount (DH): ")
                .parse::<f64>()
                .unwrap_or(0.0)
        } else {
            0.0
        };

        let action = EnforcementAction {
            action_id: uuid::Uuid::new_v4().to_string(),
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
        db.save_action(&action)?;

        let metadata = DocumentMetadata {
            id: uuid::Uuid::new_v4().to_string(),
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
        db.save_metadata(&metadata)?;

        let generator = DocumentGenerator::new(
            metadata.clone(),
            Some(commerce),
            Some(inspection),
            Some(action),
        );
        let output_filename = generator.generate_filename();
        generator.create_document(&output_filename)?;

        println!(
            "✓ تم إنشاء الوثيقة: {} / Document created: {}",
            output_filename, output_filename
        );

        let continue_choice = get_user_input("\nإجراء آخر؟ / Another action? (y/n): ");
        if continue_choice.to_lowercase() != "y" {
            break;
        }
    }

    println!("\nشكراً / Thank you!");
    Ok(())
}

/// Gets input from the user
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// Menu for selecting document type
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

/// Menu for selecting violation type
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

/// Collects commerce information from user
fn collect_commerce_info() -> CommerceInfo {
    println!("\n=== معلومات المحل / Commerce Information ===");

    CommerceInfo {
        commerce_id: uuid::Uuid::new_v4().to_string(),
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
        has_autorisation: get_user_input("هل يتوفر على رخصة؟ / Has Authorization? (y/n): ")
            .to_lowercase()
            == "y",
        autorisation_number: get_user_input("رقم الرخصة / Authorization Number: "),
        autorisation_date: get_user_input("تاريخ الرخصة / Authorization Date: "),
    }
}
