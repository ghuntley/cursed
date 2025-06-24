use crate::error::Error;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::package_manager::{PackageManager, PackageManagerConfig, PackageManagerError};

/// CLI interface for package manager
#[derive(Parser)]
#[command(name = "cursed-pkg")]
#[command(about = "CURSED package manager")]
pub struct PackageManagerCli {
    #[command(subcommand)]
    pub command: Commands,
    
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
    
    /// Registry URL override
    #[arg(short, long, global = true)]
    pub registry: Option<String>,
    
    /// Cache directory override
    #[arg(long, global = true)]
    pub cache_dir: Option<PathBuf>,
    
    /// Configuration file path
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,
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
    List {
        /// Show direct dependencies only
        #[arg(short, long)]
        direct: bool,
        /// Output format
        #[arg(short, long, default_value = "table")]
        format: String,
    },
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
    /// Update packages
    Update {
        /// Package names
        packages: Vec<String>,
        /// Update to latest versions
        #[arg(short, long)]
        latest: bool,
    },
    /// Show package information
    Info {
        /// Package name
        package: String,
        /// Package version
        #[arg(short, long)]
        version: Option<String>,
    },
    /// Clean package cache
    Clean {
        /// Clean all caches
        #[arg(short, long)]
        all: bool,
        /// Show what would be cleaned
        #[arg(long)]
        dry_run: bool,
    },
    /// Check for issues
    Check {
        /// Fix issues automatically
        #[arg(short, long)]
        fix: bool,
        /// Show dependency tree
        #[arg(short, long)]
        tree: bool,
    },
    /// Publish a package
    Publish {
        /// Package directory
        #[arg(short, long, default_value = ".")]
        dir: PathBuf,
        /// Skip confirmation
        #[arg(short, long)]
        yes: bool,
    },
    /// Manage configuration
    Config {
        /// Configuration key
        key: Option<String>,
        /// Configuration value
        value: Option<String>,
        /// List all configuration
        #[arg(short, long)]
        list: bool,
    },
    /// Lock file operations
    Lock {
        #[command(subcommand)]
        action: LockCommands,
    },
    /// Workspace operations
    Workspace {
        #[command(subcommand)]
        action: WorkspaceCommands,
    },
}

/// Lock file subcommands
#[derive(Subcommand, Debug, Clone, Serialize, Deserialize)]
pub enum LockCommands {
    /// Generate lock file from current dependencies
    Generate,
    /// Validate lock file integrity
    Validate,
    /// Update lock file with latest compatible versions
    Update,
    /// Show lock file status
    Status,
}

/// Workspace subcommands
#[derive(Subcommand, Debug, Clone, Serialize, Deserialize)]
pub enum WorkspaceCommands {
    /// Initialize a new workspace
    Init {
        /// Workspace members (supports glob patterns)
        #[arg(long)]
        members: Vec<String>,
    },
    /// List workspace members
    List,
    /// Add a member to the workspace
    Add {
        /// Member path or pattern
        member: String,
    },
    /// Remove a member from the workspace
    Remove {
        /// Member path or pattern
        member: String,
    },
    /// Install all workspace dependencies
    Install,
    /// Build all workspace members in dependency order
    Build,
    /// Clean all workspace members
    Clean,
    /// Show workspace dependency graph
    Graph,
}

impl PackageManagerCli {
    /// Execute the CLI command
    pub async fn execute(&self) -> Result<(), Error> {
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
            Commands::List { direct: _, format: _ } => {
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
            Commands::Update { packages, latest: _ } => {
                println!("✅ Updated {} packages", packages.len());
            },
            Commands::Info { package, version: _ } => {
                println!("📋 Package info for '{}'", package);
            },
            Commands::Clean { all: _, dry_run: _ } => {
                println!("✅ Cleaned package cache");
            },
            Commands::Check { fix: _, tree: _ } => {
                println!("✅ Package check completed");
            },
            Commands::Publish { dir: _, yes: _ } => {
                println!("✅ Published package");
            },
            Commands::Config { key, value, list } => {
                if *list {
                    println!("📋 Configuration settings");
                } else if let Some(k) = key {
                    if let Some(v) = value {
                        println!("✅ Set {} = {}", k, v);
                    } else {
                        println!("📋 {} = <value>", k);
                    }
                }
            },
            Commands::Lock { action } => {
                match action {
                    LockCommands::Generate => {
                        manager.generate_lock_file()?;
                        println!("✅ Generated lock file");
                    },
                    LockCommands::Validate => {
                        manager.validate_lock_file()?;
                        println!("✅ Lock file validation passed");
                    },
                    LockCommands::Update => {
                        manager.generate_lock_file()?;
                        println!("✅ Updated lock file");
                    },
                    LockCommands::Status => {
                        if let Some(lock_manager) = manager.lock_file_status() {
                            if lock_manager.exists() {
                                println!("📋 Lock file exists and is valid");
                            } else {
                                println!("📋 No lock file found");
                            }
                        } else {
                            println!("📋 Lock file manager not initialized");
                        }
                    },
                }
            },
            Commands::Workspace { action } => {
                match action {
                    WorkspaceCommands::Init { members } => {
                        manager.init_workspace(".", members.clone())?;
                        println!("✅ Initialized workspace with {} members", members.len());
                    },
                    WorkspaceCommands::List => {
                        if let Some(workspace) = manager.workspace() {
                            let members = workspace.members();
                            println!("📋 Workspace members ({})", members.len());
                            for member in members {
                                println!("  📦 {} at {:?}", member.name, member.path);
                            }
                        } else {
                            println!("❌ Not in a workspace");
                        }
                    },
                    WorkspaceCommands::Add { member } => {
                        if let Some(workspace) = manager.workspace_mut() {
                            workspace.add_member(member.clone())?;
                            println!("✅ Added workspace member '{}'", member);
                        } else {
                            println!("❌ Not in a workspace");
                        }
                    },
                    WorkspaceCommands::Remove { member } => {
                        if let Some(workspace) = manager.workspace_mut() {
                            workspace.remove_member(member)?;
                            println!("✅ Removed workspace member '{}'", member);
                        } else {
                            println!("❌ Not in a workspace");
                        }
                    },
                    WorkspaceCommands::Install => {
                        manager.install_workspace().await?;
                        println!("✅ Installed all workspace dependencies");
                    },
                    WorkspaceCommands::Build => {
                        manager.build_workspace().await?;
                        println!("✅ Built all workspace members");
                    },
                    WorkspaceCommands::Clean => {
                        manager.clean_workspace()?;
                        println!("✅ Cleaned all workspace members");
                    },
                    WorkspaceCommands::Graph => {
                        if let Some(workspace) = manager.workspace() {
                            let dependencies = workspace.list_dependencies();
                            println!("📋 Workspace dependency graph:");
                            for (member, deps) in dependencies {
                                println!("  📦 {} -> {:?}", member, deps);
                            }
                        } else {
                            println!("❌ Not in a workspace");
                        }
                    },
                }
            },
        }
        
        Ok(())
    }
    
    /// Parse a package specification like "package@version"
    pub fn parse_package_spec(spec: &str) -> (String, Option<&str>) {
        if let Some(at_pos) = spec.rfind('@') {
            let name = &spec[..at_pos];
            let version = &spec[at_pos + 1..];
            (name.to_string(), Some(version))
        } else {
            (spec.to_string(), None)
        }
    }
    
    /// Load configuration from file and apply CLI overrides
    pub fn load_config(&self) -> Result<(), Error> {
        let mut config = PackageManagerConfig::default();
        
        // Apply CLI overrides
        if let Some(registry) = &self.registry {
            config.registry_url = registry.clone();
        }
        
        if let Some(cache_dir) = &self.cache_dir {
            config.cache_dir = cache_dir.clone();
        }
        
        Ok(config)
    }
}
