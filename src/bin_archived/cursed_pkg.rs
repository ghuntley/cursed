use crate::error::CursedError;
/// CURSED Package Manager Binary
/// 
/// Command-line interface for managing CURSED packages, dependencies,
/// and project configuration.

use clap::Parser;
use cursed::package_manager::{PackageManagerCli, PackageManagerError};

#[tokio::main]
async fn main() -> crate::error::Result<()> {
    // Parse command line arguments
    let cli = PackageManagerCli::parse();
    
    // Execute the command
    if let Err(e) = cli.execute().await {
        eprintln!("CursedError: {}", e);
        
        // Set appropriate exit code based on error type
        let exit_code = match e {
            PackageManagerError::PackageNotFound { .. } => 1,
            PackageManagerError::VersionConflict { .. } => 2,
            PackageManagerError::CircularDependency { .. } => 3,
            PackageManagerError::DependencyError { .. } => 4,
            PackageManagerError::InvalidVersion { .. } => 5,
            PackageManagerError::DependencyNotFound { .. } => 6,
            PackageManagerError::DependencyVersionConflict { .. } => 7,
            PackageManagerError::FileSystemError { .. } => 8,
            PackageManagerError::LockTimeout { .. } => 9,
            PackageManagerError::PackageTooLarge { .. } => 10,
            PackageManagerError::CacheCorruption { .. } => 11,
            PackageManagerError::RegistryError { .. } => 12,
            PackageManagerError::InvalidMetadata { .. } => 13,
            PackageManagerError::Io(_) => 14,
            PackageManagerError::Serialization(_) => 15,
            PackageManagerError::Toml(_) => 16,
            PackageManagerError::Http(_) => 17,
            PackageManagerError::UrlParse(_) => 18,
            PackageManagerError::LockFile(_) => 19,
            PackageManagerError::Workspace(_) => 20,
            PackageManagerError::UnsupportedVersion { .. } => 21,
        };
        
        std::process::exit(exit_code);
    }
    
    Ok(())
}
