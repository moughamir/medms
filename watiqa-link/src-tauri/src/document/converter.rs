use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct DocumentConverter {
    libreoffice_path: PathBuf,
}

impl DocumentConverter {
    pub fn new(libreoffice_path: PathBuf) -> Self {
        Self { libreoffice_path }
    }

    pub async fn convert_to_pdf(&self, input_path: &Path, output_path: &Path) -> Result<()> {
        let output = Command::new(&self.libreoffice_path)
            .args(&[
                "--headless",
                "--convert-to",
                "pdf:writer_pdf_Export",
                "--outdir",
                output_path.parent().unwrap().to_str().unwrap(),
                input_path.to_str().unwrap(),
            ])
            .output()
            .context("Failed to execute LibreOffice")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Conversion failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    pub async fn convert_with_retry(
        &self,
        input_path: &Path,
        output_path: &Path,
        max_retries: u32,
    ) -> Result<()> {
        let mut attempts = 0;

        loop {
            match self.convert_to_pdf(input_path, output_path).await {
                Ok(_) => return Ok(()),
                Err(_e) if attempts < max_retries => {
                    attempts += 1;
                    tokio::time::sleep(tokio::time::Duration::from_secs(2_u64.pow(attempts))).await;
                }
                Err(e) => return Err(e),
            }
        }
    }
}
