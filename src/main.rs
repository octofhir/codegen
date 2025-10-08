//! CLI entry point for octofhir-codegen

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!("OctoFHIR Codegen - Starting");

    // TODO: Implement CLI in Phase 3
    println!("OctoFHIR Codegen v{}", env!("CARGO_PKG_VERSION"));
    println!("CLI implementation coming soon...");

    Ok(())
}
