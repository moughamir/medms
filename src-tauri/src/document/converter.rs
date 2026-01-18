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

        let output_future = Command::new(&self.libreoffice_path)
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
            .output();

        let output = tokio::time::timeout(std::time::Duration::from_secs(30), output_future)
            .await
            .context("LibreOffice conversion timed out")??;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[tokio::test]
    async fn test_converter_init() {
        let converter = DocumentConverter::new(PathBuf::from("/usr/bin/libreoffice"), 2);
        assert_eq!(converter.semaphore.available_permits(), 2);
    }

    #[tokio::test]
    async fn test_convert_invalid_input() {
        let converter = DocumentConverter::new(PathBuf::from("/usr/bin/libreoffice"), 1);
        let input = Path::new("non_existent.docx");
        let output = Path::new("output.pdf");

        let result = converter.convert_to_pdf(input, output).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Input file does not exist"));
    }

    #[tokio::test]
    async fn test_retry_mechanism_failure() {
        let converter = DocumentConverter::new(PathBuf::from("/invalid/path"), 1);
        let temp_dir = tempfile::tempdir().unwrap();
        let input_path = temp_dir.path().join("input.docx");
        fs::write(&input_path, "dummy").unwrap();

        let output_path = temp_dir.path().join("output.pdf");

        // This should fail after 2 retries (3 total attempts)
        let start = std::time::Instant::now();
        let result = converter
            .convert_with_retry(&input_path, &output_path, 1)
            .await;
        let duration = start.elapsed();

        assert!(result.is_err());
        // Retry delay is 2^attempts. 1 retry = 2 seconds.
        assert!(duration.as_secs() >= 2);
    }
}
