//!

use anyhow::Result;
use moroccan_docs::{gui::MoroccanDocsApp, run_cli};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"--cli".to_string()) || args.contains(&"-c".to_string()) {
        tracing::info!("Starting in CLI mode");
        run_cli()?;
    } else {
        tracing::info!("Starting in GUI mode");

        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1280.0, 800.0])
                .with_min_inner_size([800.0, 600.0])
                .with_title("نظام إدارة الشرطة الإدارية | Police Administrative"),
            ..Default::default()
        };

        eframe::run_native(
            "moroccan_docs",
            options,
            Box::new(|cc| Ok(Box::new(MoroccanDocsApp::new(cc)))),
        )
        .map_err(|e| anyhow::anyhow!("Failed to start GUI: {}", e))?;
    }

    Ok(())
}
