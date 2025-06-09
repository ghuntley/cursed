// CURSED Profiling CLI Tool

use clap::Parser;
use tracing::{error, info, Level};
use tracing_subscriber::{FmtSubscriber, EnvFilter};

use cursed::profiling::cli::{ProfileCli, CliExecutor, CliConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_env_filter(filter)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
    
    // Parse command line arguments
    let cli = ProfileCli::parse();
    
    if cli.verbose {
        info!("CURSED Profiling Tool - Version 1.0.0");
        info!("Verbose mode enabled");
    }
    
    // Load configuration
    let config = if let Some(config_path) = &cli.config {
        match std::fs::read_to_string(config_path) {
            Ok(content) => {
                toml::from_str(&content)
                    .unwrap_or_else(|e| {
                        error!("Failed to parse config file: {}", e);
                        CliConfig::default()
                    })
            }
            Err(e) => {
                error!("Failed to read config file: {}", e);
                CliConfig::default()
            }
        }
    } else {
        CliConfig::default()
    };
    
    // Execute command
    let executor = CliExecutor::new(config);
    
    match executor.execute(cli).await {
        Ok(()) => {
            info!("Operation completed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Operation failed: {}", e);
            std::process::exit(1);
        }
    }
}
