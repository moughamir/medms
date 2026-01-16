use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use zip::{ZipArchive, ZipWriter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliceDocumentData {
    pub template_id: String,
    pub citizen_name: String,
    pub citizen_cin: String,
    pub citizen_address: String,
    pub agent_name: String,
    pub agent_grade: String,
    pub commune: String,
    pub uuid: uuid::Uuid,
    pub timestamp: String,
}

pub struct TemplateEngine;

impl TemplateEngine {
    pub fn populate_docx(
        template_path: &std::path::Path,
        data: &PoliceDocumentData,
        output_path: &std::path::Path,
    ) -> Result<()> {
        let template_file = std::fs::File::open(template_path)?;
        let mut archive = ZipArchive::new(template_file)?;

        let mut output_file = std::fs::File::create(output_path)?;
        let mut output_zip = ZipWriter::new(&mut output_file);

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let name = file.name().to_owned();

            if name == "word/document.xml" {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                contents = contents
                    .replace("{{CITIZEN_NAME}}", &data.citizen_name)
                    .replace("{{CITIZEN_CIN}}", &data.citizen_cin)
                    .replace("{{CITIZEN_ADDRESS}}", &data.citizen_address)
                    .replace("{{AGENT_NAME}}", &data.agent_name)
                    .replace("{{AGENT_GRADE}}", &data.agent_grade)
                    .replace("{{COMMUNE}}", &data.commune)
                    .replace("{{UUID}}", &data.uuid.to_string())
                    .replace("{{TIMESTAMP}}", &data.timestamp);

                output_zip.start_file(&name, Default::default())?;
                output_zip.write_all(contents.as_bytes())?;
            } else {
                output_zip.start_file(&name, Default::default())?;
                std::io::copy(&mut file, &mut output_zip)?;
            }
        }

        output_zip.finish()?;
        Ok(())
    }
}
