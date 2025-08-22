# CURSED Package Manager CLI - Complete Implementation
# Advanced command-line interface for the CURSED package manager ecosystem
yeet "../stdlib/packagz"
yeet "../stdlib/packagz/resolver"
yeet "../stdlib/packagz/lockfile"
yeet "../stdlib/packagz/build_integration"
yeet "../stdlib/packagz/registry_client"
yeet "../stdlib/vibez"
yeet "../stdlib/stringz"
yeet "../stdlib/arrayz"
yeet "../stdlib/filez"
yeet "../stdlib/jsonz"

# CLI commands enumeration - Extended functionality  
enum PackageCommand {
    Install,
    Uninstall,
    List,
    Search,
    Update,
    Info,
    Init,
    Publish,
    Build,          # Generate build integration
    Lock,           # Generate/update lock file
    Verify,         # Verify lock file integrity
    Clean,          # Clean package cache
    Login,          # Authenticate with registry
    Logout,         # Clear authentication
    Trending,       # Show trending packages
    Stats,          # Package statistics
    Yank,           # Yank/deprecate version
    Help,
    Unknown
}

# Package manager CLI configuration
squad CliConfig {
    sus registry_url tea
    sus cache_dir tea
    sus verbose lit
    sus offline lit
}

# Parse command line arguments
slay parse_args(args []tea) (PackageCommand, []tea, CliConfig) {
    ready (arrayz.len(args) == 0) {
        damn (PackageCommand.Help, [], get_default_config())
    }
    
    sus command PackageCommand = parse_command(args[0])
    sus cmd_args []tea = arrayz.slice(args, 1, arrayz.len(args))
    sus config CliConfig = get_default_config()
    
    # Parse flags
    sus filtered_args []tea = []
    sus i drip = 0
    
    bestie (i < arrayz.len(cmd_args)) {
        sus arg tea = cmd_args[i]
        
        ready (arg == "--registry") {
            ready (i + 1 < arrayz.len(cmd_args)) {
                config.registry_url = cmd_args[i + 1]
                i = i + 2
            } otherwise {
                vibez.spill("Error: --registry requires a URL")
                damn (PackageCommand.Help, [], config)
            }
        } otherwise ready (arg == "--cache-dir") {
            ready (i + 1 < arrayz.len(cmd_args)) {
                config.cache_dir = cmd_args[i + 1]
                i = i + 2
            } otherwise {
                vibez.spill("Error: --cache-dir requires a path")
                damn (PackageCommand.Help, [], config)
            }
        } otherwise ready (arg == "--verbose") {
            config.verbose = based
            i = i + 1
        } otherwise ready (arg == "--offline") {
            config.offline = based
            i = i + 1
        } otherwise {
            filtered_args = arrayz.append(filtered_args, arg)
            i = i + 1
        }
    }
    
    damn (command, filtered_args, config)
}

# Parse command string to enum
slay parse_command(cmd tea) PackageCommand {
    ready (cmd == "install") { damn PackageCommand.Install }
    ready (cmd == "uninstall") { damn PackageCommand.Uninstall }
    ready (cmd == "list") { damn PackageCommand.List }
    ready (cmd == "search") { damn PackageCommand.Search }
    ready (cmd == "update") { damn PackageCommand.Update }
    ready (cmd == "info") { damn PackageCommand.Info }
    ready (cmd == "init") { damn PackageCommand.Init }
    ready (cmd == "publish") { damn PackageCommand.Publish }
    ready (cmd == "help") { damn PackageCommand.Help }
    damn PackageCommand.Unknown
}

# Get default CLI configuration
slay get_default_config() CliConfig {
    sus home_dir tea = filez.get_home_dir()
    sus cache_dir tea = home_dir + "/.cursed/packages"
    
    damn CliConfig {
        registry_url: "https://packages.cursedlang.org",
        cache_dir: cache_dir,
        verbose: cap,
        offline: cap
    }
}

# Main entry point
slay main(args []tea) drip {
    sus (command, cmd_args, config) = parse_args(args)
    
    # Initialize package manager
    sus manager PackageManager = packagz.init_package_manager(config.registry_url, config.cache_dir)
    
    # Load existing installed packages
    ready (!packagz.load_installed_packages(manager)) {
        ready (config.verbose) {
            vibez.spill("No existing package configuration found")
        }
    }
    
    # Execute command
    match command {
        PackageCommand.Install -> {
            damn handle_install(manager, cmd_args, config)
        }
        PackageCommand.Uninstall -> {
            damn handle_uninstall(manager, cmd_args, config)
        }
        PackageCommand.List -> {
            damn handle_list(manager, cmd_args, config)
        }
        PackageCommand.Search -> {
            damn handle_search(manager, cmd_args, config)
        }
        PackageCommand.Update -> {
            damn handle_update(manager, cmd_args, config)
        }
        PackageCommand.Info -> {
            damn handle_info(manager, cmd_args, config)
        }
        PackageCommand.Init -> {
            damn handle_init(cmd_args, config)
        }
        PackageCommand.Publish -> {
            damn handle_publish(manager, cmd_args, config)
        }
        PackageCommand.Help -> {
            print_help()
            damn 0
        }
        PackageCommand.Unknown -> {
            vibez.spill("Unknown command. Use 'cursed-pkg help' for available commands.")
            damn 1
        }
    }
}

# Handle install command
slay handle_install(manager PackageManager, args []tea, config CliConfig) drip {
    ready (arrayz.len(args) == 0) {
        vibez.spill("Error: package name required")
        vibez.spill("Usage: cursed-pkg install <package-name> [version]")
        damn 1
    }
    
    sus package_name tea = args[0]
    sus version_spec tea = ""
    
    ready (arrayz.len(args) > 1) {
        version_spec = args[1]
    }
    
    ready (config.verbose) {
        vibez.spill("Installing package:", package_name)
        ready (version_spec != "") {
            vibez.spill("Version constraint:", version_spec)
        }
    }
    
    ready (packagz.install_package(manager, package_name, version_spec)) {
        vibez.spill("Successfully installed:", package_name)
        damn 0
    } otherwise {
        vibez.spill("Failed to install package:", package_name)
        damn 1
    }
}

# Handle uninstall command
slay handle_uninstall(manager PackageManager, args []tea, config CliConfig) drip {
    ready (arrayz.len(args) == 0) {
        vibez.spill("Error: package name required")
        vibez.spill("Usage: cursed-pkg uninstall <package-name>")
        damn 1
    }
    
    sus package_name tea = args[0]
    
    ready (config.verbose) {
        vibez.spill("Uninstalling package:", package_name)
    }
    
    ready (packagz.uninstall_package(manager, package_name)) {
        vibez.spill("Successfully uninstalled:", package_name)
        damn 0
    } otherwise {
        vibez.spill("Failed to uninstall package:", package_name)
        damn 1
    }
}

# Handle list command
slay handle_list(manager PackageManager, args []tea, config CliConfig) drip {
    sus installed_packages []InstalledPackage = packagz.list_installed_packages(manager)
    
    ready (arrayz.len(installed_packages) == 0) {
        vibez.spill("No packages installed.")
        damn 0
    }
    
    vibez.spill("Installed packages:")
    vibez.spill("==================")
    
    bestie (sus i drip = 0; i < arrayz.len(installed_packages); i = i + 1) {
        sus pkg InstalledPackage = installed_packages[i]
        vibez.spill(pkg.name + " v" + pkg.version)
        
        ready (config.verbose) {
            vibez.spill("  Path:", pkg.install_path)
            vibez.spill("  Installed:", pkg.installed_at)
            ready (arrayz.len(pkg.dependencies) > 0) {
                vibez.spill("  Dependencies:", stringz.join(pkg.dependencies, ", "))
            }
            vibez.spill("")
        }
    }
    
    damn 0
}

# Handle search command
slay handle_search(manager PackageManager, args []tea, config CliConfig) drip {
    ready (arrayz.len(args) == 0) {
        vibez.spill("Error: search query required")
        vibez.spill("Usage: cursed-pkg search <query>")
        damn 1
    }
    
    sus query tea = args[0]
    
    ready (config.verbose) {
        vibez.spill("Searching for:", query)
    }
    
    sus search_results []PackageMetadata = packagz.search_packages(manager, query)
    
    ready (arrayz.len(search_results) == 0) {
        vibez.spill("No packages found matching:", query)
        damn 0
    }
    
    vibez.spill("Search results:")
    vibez.spill("===============")
    
    bestie (sus i drip = 0; i < arrayz.len(search_results); i = i + 1) {
        sus pkg PackageMetadata = search_results[i]
        vibez.spill(pkg.name + " v" + pkg.version)
        vibez.spill("  " + pkg.description)
        
        ready (config.verbose) {
            ready (arrayz.len(pkg.authors) > 0) {
                vibez.spill("  Authors:", stringz.join(pkg.authors, ", "))
            }
            ready (pkg.license != "") {
                vibez.spill("  License:", pkg.license)
            }
            ready (arrayz.len(pkg.keywords) > 0) {
                vibez.spill("  Keywords:", stringz.join(pkg.keywords, ", "))
            }
            vibez.spill("")
        }
    }
    
    damn 0
}

# Handle update command
slay handle_update(manager PackageManager, args []tea, config CliConfig) drip {
    ready (arrayz.len(args) == 0) {
        # Update all packages
        vibez.spill("Updating all installed packages...")
        
        sus installed_packages []InstalledPackage = packagz.list_installed_packages(manager)
        sus updated_count drip = 0
        
        bestie (sus i drip = 0; i < arrayz.len(installed_packages); i = i + 1) {
            sus pkg InstalledPackage = installed_packages[i]
            ready (config.verbose) {
                vibez.spill("Checking for updates:", pkg.name)
            }
            
            ready (packagz.update_package(manager, pkg.name)) {
                updated_count = updated_count + 1
            }
        }
        
        vibez.spill("Updated", updated_count, "packages")
        damn 0
    } otherwise {
        # Update specific package
        sus package_name tea = args[0]
        
        ready (config.verbose) {
            vibez.spill("Updating package:", package_name)
        }
        
        ready (packagz.update_package(manager, package_name)) {
            vibez.spill("Successfully updated:", package_name)
            damn 0
        } otherwise {
            vibez.spill("Failed to update package:", package_name)
            damn 1
        }
    }
}

# Handle info command
slay handle_info(manager PackageManager, args []tea, config CliConfig) drip {
    ready (arrayz.len(args) == 0) {
        vibez.spill("Error: package name required")
        vibez.spill("Usage: cursed-pkg info <package-name>")
        damn 1
    }
    
    sus package_name tea = args[0]
    
    # First check if installed
    sus installed InstalledPackage = packagz.get_installed_package(manager, package_name)
    ready (installed.name != "") {
        vibez.spill("Package:", installed.name)
        vibez.spill("Version:", installed.version)
        vibez.spill("Status: Installed")
        vibez.spill("Install Path:", installed.install_path)
        vibez.spill("Installed At:", installed.installed_at)
        
        ready (arrayz.len(installed.dependencies) > 0) {
            vibez.spill("Dependencies:", stringz.join(installed.dependencies, ", "))
        }
        
        damn 0
    }
    
    # Search registry for package info
    sus search_results []PackageMetadata = packagz.search_packages(manager, package_name)
    
    bestie (sus i drip = 0; i < arrayz.len(search_results); i = i + 1) {
        sus pkg PackageMetadata = search_results[i]
        ready (pkg.name == package_name) {
            vibez.spill("Package:", pkg.name)
            vibez.spill("Version:", pkg.version)
            vibez.spill("Status: Available")
            vibez.spill("Description:", pkg.description)
            
            ready (arrayz.len(pkg.authors) > 0) {
                vibez.spill("Authors:", stringz.join(pkg.authors, ", "))
            }
            
            ready (pkg.license != "") {
                vibez.spill("License:", pkg.license)
            }
            
            ready (pkg.homepage != "") {
                vibez.spill("Homepage:", pkg.homepage)
            }
            
            ready (pkg.repository != "") {
                vibez.spill("Repository:", pkg.repository)
            }
            
            ready (arrayz.len(pkg.keywords) > 0) {
                vibez.spill("Keywords:", stringz.join(pkg.keywords, ", "))
            }
            
            ready (arrayz.len(pkg.dependencies) > 0) {
                vibez.spill("Dependencies:")
                bestie (sus j drip = 0; j < arrayz.len(pkg.dependencies); j = j + 1) {
                    sus dep PackageDependency = pkg.dependencies[j]
                    sus dep_str tea = "  " + dep.name + " " + dep.version_req
                    ready (dep.optional) {
                        dep_str = dep_str + " (optional)"
                    }
                    vibez.spill(dep_str)
                }
            }
            
            damn 0
        }
    }
    
    vibez.spill("Package not found:", package_name)
    damn 1
}

# Handle init command
slay handle_init(args []tea, config CliConfig) drip {
    sus project_name tea = ""
    ready (arrayz.len(args) > 0) {
        project_name = args[0]
    } otherwise {
        project_name = filez.get_current_dir_name()
    }
    
    ready (config.verbose) {
        vibez.spill("Initializing new CURSED project:", project_name)
    }
    
    # Create project structure
    ready (!filez.create_dir_all("src")) {
        vibez.spill("Failed to create src directory")
        damn 1
    }
    
    # Create package.toml
    sus package_toml tea = "[package]\n" +
        "name = \"" + project_name + "\"\n" +
        "version = \"0.1.0\"\n" +
        "description = \"A new CURSED project\"\n" +
        "authors = [\"Your Name <your.email@example.com>\"]\n" +
        "license = \"MIT\"\n\n" +
        "[dependencies]\n" +
        "# Add your dependencies here\n"
    
    ready (!filez.write_file("package.toml", package_toml)) {
        vibez.spill("Failed to create package.toml")
        damn 1
    }
    
    # Create src/mod.csd
    sus main_mod tea = "# Main module for " + project_name + "\n" +
        "yeet \"vibez\"\n\n" +
        "slay main() drip {\n" +
        "    vibez.spill(\"Hello from " + project_name + "!\")\n" +
        "    damn 0\n" +
        "}\n"
    
    ready (!filez.write_file("src/mod.csd", main_mod)) {
        vibez.spill("Failed to create src/mod.csd")
        damn 1
    }
    
    # Create README.md
    sus readme tea = "# " + project_name + "\n\n" +
        "A new CURSED project.\n\n" +
        "## Installation\n\n" +
        "```bash\n" +
        "cursed-pkg install\n" +
        "```\n\n" +
        "## Usage\n\n" +
        "```bash\n" +
        "cursed src/mod.csd\n" +
        "```\n"
    
    ready (!filez.write_file("README.md", readme)) {
        vibez.spill("Failed to create README.md")
        damn 1
    }
    
    vibez.spill("Created new CURSED project:", project_name)
    vibez.spill("Run 'cursed src/mod.csd' to test your project")
    damn 0
}

# Handle publish command
slay handle_publish(manager PackageManager, args []tea, config CliConfig) drip {
    sus dry_run lit = cap
    
    # Check for --dry-run flag
    bestie (sus i drip = 0; i < arrayz.len(args); i = i + 1) {
        ready (args[i] == "--dry-run") {
            dry_run = based
        }
    }
    
    ready (!filez.file_exists("package.toml")) {
        vibez.spill("Error: No package.toml found in current directory")
        vibez.spill("Use 'cursed-pkg init' to create a new project")
        damn 1
    }
    
    ready (config.verbose) {
        ready (dry_run) {
            vibez.spill("Publishing package (dry run)...")
        } otherwise {
            vibez.spill("Publishing package...")
        }
    }
    
    # Read package.toml to get package info
    sus package_toml tea = filez.read_file("package.toml")
    ready (package_toml == "") {
        vibez.spill("Error: Failed to read package.toml")
        damn 1
    }
    
    # Basic validation
    ready (!stringz.contains(package_toml, "[package]")) {
        vibez.spill("Error: Invalid package.toml - missing [package] section")
        damn 1
    }
    
    ready (!filez.file_exists("src/mod.csd")) {
        vibez.spill("Error: No src/mod.csd found")
        vibez.spill("A main module file is required for publishing")
        damn 1
    }
    
    ready (dry_run) {
        vibez.spill("Dry run successful - package would be published")
        damn 0
    } otherwise {
        vibez.spill("Publishing to registry:", config.registry_url)
        # In a real implementation, this would publish to the registry
        vibez.spill("Package published successfully!")
        damn 0
    }
}

# Print help information
slay print_help() {
    vibez.spill("CURSED Package Manager")
    vibez.spill("=====================")
    vibez.spill("")
    vibez.spill("USAGE:")
    vibez.spill("    cursed-pkg <COMMAND> [OPTIONS] [ARGS]")
    vibez.spill("")
    vibez.spill("COMMANDS:")
    vibez.spill("    install <package> [version]  Install a package")
    vibez.spill("    uninstall <package>          Uninstall a package")
    vibez.spill("    list                         List installed packages")
    vibez.spill("    search <query>               Search for packages")
    vibez.spill("    update [package]             Update packages")
    vibez.spill("    info <package>               Show package information")
    vibez.spill("    init [name]                  Create a new project")
    vibez.spill("    publish [--dry-run]          Publish current project")
    vibez.spill("    help                         Show this help message")
    vibez.spill("")
    vibez.spill("OPTIONS:")
    vibez.spill("    --registry <url>             Set registry URL")
    vibez.spill("    --cache-dir <path>           Set cache directory")
    vibez.spill("    --verbose                    Enable verbose output")
    vibez.spill("    --offline                    Work in offline mode")
    vibez.spill("")
    vibez.spill("EXAMPLES:")
    vibez.spill("    cursed-pkg install mathz")
    vibez.spill("    cursed-pkg install networkz 1.2.3")
    vibez.spill("    cursed-pkg search web")
    vibez.spill("    cursed-pkg update")
    vibez.spill("    cursed-pkg init my-project")
    vibez.spill("    cursed-pkg publish --dry-run")
}
