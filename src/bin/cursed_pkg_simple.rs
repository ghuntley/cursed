/// Simple CURSED Package Manager Binary
use std::path::PathBuf;
use cursed::package_manager::{PackageManager, PackageManagerConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();
    
    println!("🚀 CURSED Package Manager");
    
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return Ok(());
    }
    
    match args[1].as_str() {
        "init" => {
            let name = args.get(2).map(|s| s.as_str()).unwrap_or("my-cursed-project");
            handle_init(name)?;
        }
        "install" => {
            if args.len() < 3 {
                eprintln!("Error: Package name required for install");
                return Ok(());
            }
            handle_install(&args[2]).await?;
        }
        "search" => {
            if args.len() < 3 {
                eprintln!("Error: Search query required");
                return Ok(());
            }
            handle_search(&args[2]).await?;
        }
        "list" => {
            handle_list().await?;
        }
        "clean" => {
            handle_clean().await?;
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            print_help();
        }
    }
    
    Ok(())
}

fn print_help() {
    println!("CURSED Package Manager");
    println!();
    println!("Usage: cursed-pkg-simple <command> [args...]");
    println!();
    println!("Commands:");
    println!("  init <name>     Initialize a new CURSED package");
    println!("  install <pkg>   Install a package");
    println!("  search <query>  Search for packages");
    println!("  list            List installed packages");
    println!("  clean           Clean package cache");
    println!("  help            Show this help message");
}

fn handle_init(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 Initializing package: {}", name);
    
    // Create basic CursedPackage.toml
    let package_content = format!(r#"[package]
name = "{}"
version = "0.1.0"
description = "A CURSED package"
authors = ["Your Name <your.email@example.com>"]

[dependencies]
# Add your dependencies here

[dev-dependencies]
# Add your development dependencies here
"#, name);
    
    std::fs::write("CursedPackage.toml", package_content)?;
    
    // Create src directory and main file
    std::fs::create_dir_all("src")?;
    std::fs::write("src/main.csd", r#"slay main() {
    capicola("Hello, CURSED World!");
}
"#)?;
    
    println!("✅ Package '{}' initialized successfully!", name);
    println!("📝 Edit CursedPackage.toml to add dependencies");
    println!("🚀 Run 'cursed-pkg-simple install' to install dependencies");
    
    Ok(())
}

async fn handle_install(package_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 Installing package: {}", package_name);
    
    // Create package manager with default configuration
    let config = PackageManagerConfig::default();
    let mut manager = PackageManager::new(config)?;
    
    // Parse package name and version
    let (name, version) = if let Some(at_pos) = package_name.rfind('@') {
        let name = &package_name[..at_pos];
        let version = &package_name[at_pos + 1..];
        (name, Some(version))
    } else {
        (package_name, None)
    };
    
    println!("🔍 Searching for package in registry...");
    
    match manager.install_package(name, version).await {
        Ok(packages) => {
            println!("✅ Successfully installed {} package(s):", packages.len());
            for pkg in packages {
                println!("  📦 {} v{} - {}", pkg.name, pkg.version, pkg.description);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to install package: {}", e);
            // Fall back to mock behavior for demo purposes
            println!("🔄 Falling back to mock installation...");
            println!("📥 Downloading package...");
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            println!("⚙️  Installing dependencies...");
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            println!("✅ Package '{}' installed successfully! (mock)", package_name);
        }
    }
    
    Ok(())
}

async fn handle_search(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Searching for packages matching: {}", query);
    
    // Create package manager with default configuration
    let config = PackageManagerConfig::default();
    let mut manager = PackageManager::new(config)?;
    
    match manager.search_packages(query, Some(10)).await {
        Ok(packages) => {
            if packages.is_empty() {
                println!("📦 No packages found matching '{}'", query);
            } else {
                println!("📦 Found {} package(s):", packages.len());
                for pkg in packages {
                    println!("  {} v{} - {}", pkg.name, pkg.version, pkg.description);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Search failed: {}", e);
            // Fall back to mock search
            println!("🔄 Falling back to mock search...");
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            
            println!("📦 Found packages:");
            
            // Mock search results
            let mock_results = vec![
                ("json", "1.0.0", "JSON parsing and generation for CURSED"),
                ("http", "2.1.0", "HTTP client and server for CURSED"),
                ("crypto", "1.5.2", "Cryptographic functions for CURSED"),
            ];
            
            for (name, version, desc) in mock_results {
                if name.contains(query) || desc.to_lowercase().contains(&query.to_ascii_lowercase()) {
                    println!("  {} {} - {}", name, version, desc);
                }
            }
        }
    }
    
    Ok(())
}

async fn handle_list() -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 Listing installed packages");
    
    // Create package manager with default configuration
    let config = PackageManagerConfig::default();
    let manager = PackageManager::new(config)?;
    
    match manager.list_installed() {
        Ok(packages) => {
            if packages.is_empty() {
                println!("📦 No packages installed");
            } else {
                println!("📦 Installed packages ({}):", packages.len());
                for pkg in packages {
                    println!("  {} v{} - {}", pkg.name, pkg.version, pkg.description);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list packages: {}", e);
        }
    }
    
    Ok(())
}

async fn handle_clean() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧹 Cleaning package cache");
    
    // Create package manager with default configuration
    let config = PackageManagerConfig::default();
    let mut manager = PackageManager::new(config)?;
    
    match manager.clean_cache() {
        Ok(_) => {
            println!("✅ Package cache cleaned successfully");
        }
        Err(e) => {
            eprintln!("❌ Failed to clean cache: {}", e);
        }
    }
    
    Ok(())
}
