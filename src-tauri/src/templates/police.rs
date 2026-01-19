use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use zip::{ZipArchive, ZipWriter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliceDocumentData {
    pub template_id: String,
    pub country: String,
    pub ministry: String,
    pub province: String,
    pub commune: String,
    pub department: String,
    pub citizen_name: String,
    pub citizen_cin: String,
    pub citizen_address: String,
    pub agent_name: String,
    pub agent_grade: String,
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

                contents = Self::replace_placeholders(&contents, data);

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

    fn replace_placeholders(xml: &str, data: &PoliceDocumentData) -> String {
        let mut result = xml.to_string();

        let placeholders = [
            ("COUNTRY", &data.country),
            ("MINISTRY", &data.ministry),
            ("PROVINCE", &data.province),
            ("COMMUNE", &data.commune),
            ("DEPARTMENT", &data.department),
            ("CITIZEN_NAME", &data.citizen_name),
            ("CITIZEN_CIN", &data.citizen_cin),
            ("CITIZEN_ADDRESS", &data.citizen_address),
            ("AGENT_NAME", &data.agent_name),
            ("AGENT_GRADE", &data.agent_grade),
            ("UUID", &data.uuid.to_string()),
            ("TIMESTAMP", &data.timestamp),
        ];

        for (key, value) in placeholders {
            // Pattern matches {{ followed by optional XML tags, then key, then optional XML tags, then }}
            let pattern = format!(r"\x7B\x7B(?:<[^>]+>)*{}(?:<[^>]+>)*\x7D\x7D", key);
            let re = Regex::new(&pattern).unwrap();
            result = re.replace_all(&result, value).to_string();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_robust_replacement() {
        let data = PoliceDocumentData {
            template_id: "test".to_string(),
            country: "المملكة المغربية".to_string(),
            ministry: "وزارة الداخلية".to_string(),
            province: "إقليم النواصر".to_string(),
            commune: "جماعة بوسكورة".to_string(),
            department: "قسم الشرطة الإدارية".to_string(),
            citizen_name: "Yassine".to_string(),
            citizen_cin: "AB123456".to_string(),
            citizen_address: "Bouskoura".to_string(),
            agent_name: "Agent X".to_string(),
            agent_grade: "Grade A".to_string(),
            uuid: Uuid::nil(),
            timestamp: "2026-01-16".to_string(),
        };

        // Case 1: Normal replacement
        let xml = "Hello {{CITIZEN_NAME}}!";
        let replaced = TemplateEngine::replace_placeholders(xml, &data);
        assert_eq!(replaced, "Hello Yassine!");

        // Case 2: Fragmented XML replacement
        let xml_fragmented = "Hello {{<w:t>CITIZEN_NAME</w:t>}}!";
        let replaced_fragmented = TemplateEngine::replace_placeholders(xml_fragmented, &data);
        assert_eq!(replaced_fragmented, "Hello Yassine!");
    }
}
