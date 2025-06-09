use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use crate::package_manager::{PackageManager, PackageManagerConfig, PackageManagerError};

/// CLI interface for package manager
#[derive(Parser)]
#[command(name = "cursed-pkg")]
#[command(about = "CURSED package manager")]
pub struct PackageManagerCli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available CLI commands
#[derive(Subcommand, Debug, Clone, Serialize, Deserialize)]
pub enum Commands {
    /// Install a package
    Install {
        /// Package name
        package: String,
        /// Package version
        #[arg(short, long)]
        version: Option<String>,
    },
    /// Search for packages
    Search {
        /// Search query
        query: String,
        /// Maximum results
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    /// List installed packages
    List,
    /// Remove a package
    Remove {
        /// Package name
        package: String,
    },
    /// Initialize new package
    Init {
        /// Package name
        name: String,
    },
}

impl PackageManagerCli {
    /// Execute the CLI command
    pub async fn execute(&self) -> Result<(), PackageManagerError> {
        let config = PackageManagerConfig::default();
        let mut manager = PackageManager::new(config)?;
        
        match &self.command {
            Commands::Install { package, version } => {
                let packages = manager.install_package(package, version.as_deref()).await?;
                println!("✅ Installed {} package(s)", packages.len());
                for pkg in packages {
                    println!("  📦 {} v{}", pkg.name, pkg.version);
                }
            },
            Commands::Search { query, limit } => {
                let packages = manager.search_packages(query, Some(*limit)).await?;
                println!("🔍 Found {} package(s) matching '{}'", packages.len(), query);
                for pkg in packages {
                    println!("  📦 {} v{} - {}", pkg.name, pkg.version, pkg.description);
                }
            },
            Commands::List => {
                let packages = manager.list_installed()?;
                println!("📋 Installed packages ({})", packages.len());
                for pkg in packages {
                    println!("  📦 {} v{}", pkg.name, pkg.version);
                }
            },
            Commands::Remove { package } => {
                manager.remove_package(package)?;
                println!("✅ Removed package '{}'", package);
            },
            Commands::Init { name } => {
                crate::package_manager::init_package(name, None, None)?;
                println!("✅ Initialized package '{}'", name);
            },
        }
        
        Ok(())
    }
}
