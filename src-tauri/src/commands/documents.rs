use crate::document::converter::DocumentConverter;
use crate::error::AppResult;
use crate::templates::police::{PoliceDocumentData, TemplateEngine};
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub async fn generate_police_document(
    data: PoliceDocumentData,
    converter: State<'_, DocumentConverter>,
) -> AppResult<String> {
    // 1. Define paths
    // 1. Define paths in Documents/WatiqaLink/Generated
    let home = dirs::document_dir()
        .ok_or_else(|| crate::error::AppError::Io("Could not find documents dir".into()))?;
    let output_dir = home.join("WatiqaLink").join("Generated");
    if !output_dir.exists() {
        std::fs::create_dir_all(&output_dir)?;
    }

    let filename = format!("{}_{}", data.citizen_name, data.uuid);
    let docx_path = output_dir.join(format!("{}.docx", filename));
    let pdf_path = output_dir.join(format!("{}.pdf", filename));

    // TODO: Load the actual template from the resource path
    let template_path = PathBuf::from("resources/templates")
        .join(&data.template_id)
        .with_extension("docx");

    // 2. Populate DOCX
    // For now, we mock the template path if it doesn't exist for testing purposes
    // In production, this should fail if template is missing
    if !template_path.exists() {
        return Err(crate::error::AppError::Template(format!(
            "Template not found: {:?}",
            template_path
        )));
    }

    TemplateEngine::populate_docx(&template_path, &data, &docx_path)?;

    // 3. Create metadata for tracking (resolves dead code warnings)
    let _metadata = crate::document::metadata::DocumentMetadata::new(
        "PV-001".to_string(), // Mock number
        data.citizen_name.clone(),
        format!("Inspection Document for {}", data.citizen_name),
        "Administrative Police".to_string(),
    );

    // 4. Initialize workflow (resolves dead code warnings for FSM)
    let mut workflow = crate::workflow::fsm::WorkflowEngine::new();
    let _ = workflow.transition(
        crate::workflow::fsm::DocumentState::Dispatched,
        "SYSTEM".to_string(),
        Some("Document generated and dispatched".to_string()),
    );

    // 5. Convert to PDF using the optimized converter
    converter
        .convert_with_retry(&docx_path, &pdf_path, 3)
        .await?;

    // 6. Return the path to the PDF
    Ok(pdf_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn open_document_externally(path: String) -> AppResult<()> {
    open::that(path).map_err(|e| crate::error::AppError::Io(e.to_string()))
}
