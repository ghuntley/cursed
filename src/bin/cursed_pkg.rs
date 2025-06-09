/// CURSED Package Manager Binary
/// 
/// Command-line interface for managing CURSED packages, dependencies,
/// and project configuration.

use clap::Parser;
use cursed::package_manager::{PackageManagerCli, PackageManagerError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let cli = PackageManagerCli::parse();
    
    // Execute the command
    if let Err(e) = cli.execute().await {
        eprintln!("Error: {}", e);
        
        // Set appropriate exit code based on error type
        let exit_code = match e {
            PackageManagerError::PackageNotFound { .. } => 1,
            PackageManagerError::VersionConflict { .. } => 2,
            PackageManagerError::CircularDependency { .. } => 3,
            PackageManagerError::CacheCorruption { .. } => 4,
            PackageManagerError::RegistryError { .. } => 5,
            PackageManagerError::InvalidMetadata { .. } => 6,
            PackageManagerError::Io(_) => 7,
            PackageManagerError::Serialization(_) => 8,
            PackageManagerError::Toml(_) => 9,
        };
        
        std::process::exit(exit_code);
    }
    
    Ok(())
}
