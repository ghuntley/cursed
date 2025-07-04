//! CURSED Package Manager binary

use clap::{Arg, Command, ArgMatches};
use std::process;
use cursed::package_manager::{PackageManager, PackageManagerConfig};

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let matches = build_cli().get_matches();
    
    if let Err(e) = run(matches).await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn build_cli() -> Command {
    Command::new("cursed-pkg")
        .version("0.1.0")
        .about("CURSED Package Manager - Manage CURSED packages")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("install")
                .about("Install a package")
                .arg(Arg::new("package")
                    .help("Package name to install")
                    .required(true)
                    .index(1))
                .arg(Arg::new("version")
                    .help("Specific version to install")
                    .short('v')
                    .long("version")
                    .value_name("VERSION"))
        )
        .subcommand(
            Command::new("uninstall")
                .about("Uninstall a package")
                .arg(Arg::new("package")
                    .help("Package name to uninstall")
                    .required(true)
                    .index(1))
        )
        .subcommand(
            Command::new("list")
                .about("List installed packages")
        )
        .subcommand(
            Command::new("search")
                .about("Search for packages")
                .arg(Arg::new("query")
                    .help("Search query")
                    .required(true)
                    .index(1))
        )
        .subcommand(
            Command::new("update")
                .about("Update packages")
                .arg(Arg::new("package")
                    .help("Specific package to update (updates all if not specified)")
                    .index(1))
        )
        .subcommand(
            Command::new("info")
                .about("Show package information")
                .arg(Arg::new("package")
                    .help("Package name")
                    .required(true)
                    .index(1))
        )
        .subcommand(
            Command::new("init")
                .about("Initialize a new package workspace")
        )
        .subcommand(
            Command::new("build")
                .about("Build the current workspace")
        )
        .subcommand(
            Command::new("clean")
                .about("Clean the package cache")
        )
}

async fn run(matches: ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let config = PackageManagerConfig::default();
    let mut pkg_manager = PackageManager::new(config)?;
    
    match matches.subcommand() {
        Some(("install", sub_matches)) => {
            let package = sub_matches.get_one::<String>("package").unwrap();
            let version = sub_matches.get_one::<String>("version");
            
            println!("Installing package: {} {}", package, version.map_or("(latest)".to_string(), |v| format!("({})", v)));
            
            match pkg_manager.install_package(package, version.map(|s| s.as_str())).await {
                Ok(installed) => {
                    println!("Successfully installed {} version {}", installed.name, installed.version);
                }
                Err(e) => {
                    eprintln!("Failed to install package: {}", e);
                    return Err(e.into());
                }
            }
        }
        
        Some(("uninstall", sub_matches)) => {
            let package = sub_matches.get_one::<String>("package").unwrap();
            
            println!("Uninstalling package: {}", package);
            
            match pkg_manager.uninstall_package(package).await {
                Ok(_) => {
                    println!("Successfully uninstalled {}", package);
                }
                Err(e) => {
                    eprintln!("Failed to uninstall package: {}", e);
                    return Err(e.into());
                }
            }
        }
        
        Some(("list", _)) => {
            let installed = pkg_manager.list_installed();
            
            if installed.is_empty() {
                println!("No packages installed");
            } else {
                println!("Installed packages:");
                for package in installed {
                    println!("  {} ({})", package.name, package.version);
                }
            }
        }
        
        Some(("search", sub_matches)) => {
            let query = sub_matches.get_one::<String>("query").unwrap();
            
            println!("Searching for packages matching: {}", query);
            
            match pkg_manager.search_packages(query).await {
                Ok(packages) => {
                    if packages.is_empty() {
                        println!("No packages found matching '{}'", query);
                    } else {
                        println!("Found {} package(s):", packages.len());
                        for package in packages {
                            println!("  {} ({}): {}", 
                                package.name, 
                                package.version, 
                                package.description
                            );
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Search failed: {}", e);
                    return Err(e.into());
                }
            }
        }
        
        Some(("update", sub_matches)) => {
            if let Some(package) = sub_matches.get_one::<String>("package") {
                println!("Updating package: {}", package);
                
                match pkg_manager.update_package(package).await {
                    Ok(updated) => {
                        println!("Successfully updated {} to version {}", updated.name, updated.version);
                    }
                    Err(e) => {
                        eprintln!("Failed to update package: {}", e);
                        return Err(e.into());
                    }
                }
            } else {
                println!("Updating all packages...");
                
                match pkg_manager.update_all().await {
                    Ok(updated) => {
                        if updated.is_empty() {
                            println!("All packages are up to date");
                        } else {
                            println!("Updated {} package(s):", updated.len());
                            for package in updated {
                                println!("  {} -> {}", package.name, package.version);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to update packages: {}", e);
                        return Err(e.into());
                    }
                }
            }
        }
        
        Some(("info", sub_matches)) => {
            let package = sub_matches.get_one::<String>("package").unwrap();
            
            if pkg_manager.is_installed(package) {
                if let Some(installed) = pkg_manager.get_installed_package(package) {
                    println!("Package: {} (installed)", installed.name);
                    println!("Version: {}", installed.version);
                    println!("Install path: {}", installed.install_path.display());
                }
            } else {
                match pkg_manager.get_package_info(package, None).await {
                    Ok(info) => {
                        println!("Package: {} (not installed)", info.name);
                        println!("Latest version: {}", info.version);
                        println!("Description: {}", info.description);
                    }
                    Err(e) => {
                        eprintln!("Package not found: {}", e);
                        return Err(e.into());
                    }
                }
            }
        }
        
        Some(("init", _)) => {
            println!("Initializing workspace...");
            
            let current_dir = std::env::current_dir().unwrap();
            let members = vec!["main".to_string()];
            match pkg_manager.init_workspace(&current_dir, members) {
                Ok(_) => {
                    println!("Successfully initialized workspace");
                }
                Err(e) => {
                    eprintln!("Failed to initialize workspace: {}", e);
                    return Err(e.into());
                }
            }
        }
        
        Some(("build", _)) => {
            println!("Building workspace...");
            
            match pkg_manager.build_workspace().await {
                Ok(_) => {
                    println!("Build completed successfully");
                }
                Err(e) => {
                    eprintln!("Build failed: {}", e);
                    return Err(e.into());
                }
            }
        }
        
        Some(("clean", _)) => {
            println!("Cleaning package cache...");
            
            match pkg_manager.clean_workspace() {
                Ok(_) => {
                    println!("Cache cleaned successfully");
                }
                Err(e) => {
                    eprintln!("Failed to clean cache: {}", e);
                    return Err(e.into());
                }
            }
        }
        
        _ => unreachable!()
    }
    
    Ok(())
}
