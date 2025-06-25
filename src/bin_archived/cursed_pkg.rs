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
        
        std::process::exit(exit_code);
    Ok(())
}
