use anyhow::{Context, Result};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tokio::process::Command;
use tokio::sync::Semaphore;
use tracing;

pub struct DocumentConverter {
    libreoffice_path: PathBuf,
    semaphore: Arc<Semaphore>,
}

impl DocumentConverter {
    /// Creates a new DocumentConverter.
    /// `max_concurrent` limits the number of simultaneous LibreOffice processes.
    pub fn new(libreoffice_path: PathBuf, max_concurrent: usize) -> Self {
        // Allow overriding the path via environment variable
        let path = env::var("LIBREOFFICE_PATH")
            .map(PathBuf::from)
            .unwrap_or(libreoffice_path);

        Self {
            libreoffice_path: path,
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    pub async fn convert_to_pdf(&self, input_path: &Path, output_path: &Path) -> Result<()> {
        // Validate input path
        if !input_path.exists() {
            return Err(anyhow::anyhow!(
                "Input file does not exist: {:?}",
                input_path
            ));
        }

        // Acquire a permit before spawning the process
        let _permit = self
            .semaphore
            .acquire()
            .await
            .context("Failed to acquire semaphore permit")?;

        let output_dir = output_path
            .parent()
            .context("Output path must have a parent directory")?;

        let output = Command::new(&self.libreoffice_path)
            .args(&[
                "--headless",
                "--convert-to",
                "pdf:writer_pdf_Export",
                "--outdir",
                output_dir
                    .to_str()
                    .context("Output directory path is not valid UTF-8")?,
                input_path
                    .to_str()
                    .context("Input file path is not valid UTF-8")?,
            ])
            .output()
            .await
            .context("Failed to execute LibreOffice")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Conversion failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        // LibreOffice --convert-to uses the same filename as input but with .pdf extension.
        // If the user specified a different filename in output_path, we must rename it.
        let default_output_name = input_path
            .file_stem()
            .context("Input path has no file stem")?
            .to_str()
            .context("File stem is not valid UTF-8")?
            .to_owned()
            + ".pdf";

        let actual_generated_file = output_dir.join(default_output_name);

        if actual_generated_file != output_path {
            fs::rename(&actual_generated_file, output_path)
                .await
                .with_context(|| {
                    format!(
                        "Failed to rename generated PDF from {:?} to {:?}",
                        actual_generated_file, output_path
                    )
                })?;
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
                Err(e) if attempts < max_retries => {
                    attempts += 1;
                    tracing::warn!("Conversion attempt {} failed: {}. Retrying...", attempts, e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(2_u64.pow(attempts))).await;
                }
                Err(e) => return Err(e),
            }
        }
    }
}
