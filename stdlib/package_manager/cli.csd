// Command-Line Interface for CURSED Package Manager
// Provides user-friendly CLI commands for package management operations

yeet "testz"
yeet "vibez"
yeet "package_manager"
yeet "stringz"
yeet "arrayz"

// CLI command structure
squad CliCommand {
    spill name tea
    spill description tea
    spill usage tea
    spill examples []tea
    spill flags []CliFlag
    
    slay new(name tea, description tea, usage tea) CliCommand {
        damn CliCommand{
            name: name,
            description: description,
            usage: usage,
            examples: []tea{},
            flags: []CliFlag{}
        }
    }
    
    slay addExample(self CliCommand, example tea) {
        self.examples = append_array(self.examples, example)
    }
    
    slay addFlag(self CliCommand, flag CliFlag) {
        self.flags = append_array(self.flags, flag)
    }
    
    slay printHelp(self CliCommand) {
        vibez.spill("USAGE:")
        vibez.spill("    {}", self.usage)
        vibez.spill("")
        vibez.spill("DESCRIPTION:")
        vibez.spill("    {}", self.description)
        
        ready (len(self.flags) > 0) {
            vibez.spill("")
            vibez.spill("FLAGS:")
            sus i drip = 0
            bestie (i < len(self.flags)) {
                sus flag CliFlag = self.flags[i]
                flag.printHelp()
                i = i + 1
            }
        }
        
        ready (len(self.examples) > 0) {
            vibez.spill("")
            vibez.spill("EXAMPLES:")
            sus i drip = 0
            bestie (i < len(self.examples)) {
                vibez.spill("    {}", self.examples[i])
                i = i + 1
            }
        }
    }
}

squad CliFlag {
    spill short tea      // -v
    spill long tea       // --verbose
    spill description tea
    spill has_value lit  // whether flag takes a value
    spill required lit
    
    slay new(short tea, long tea, description tea) CliFlag {
        damn CliFlag{
            short: short,
            long: long,
            description: description,
            has_value: cringe,
            required: cringe
        }
    }
    
    slay withValue(short tea, long tea, description tea) CliFlag {
        sus flag CliFlag = CliFlag.new(short, long, description)
        flag.has_value = based
        damn flag
    }
    
    slay required(short tea, long tea, description tea) CliFlag {
        sus flag CliFlag = CliFlag.new(short, long, description)
        flag.required = based
        damn flag
    }
    
    slay printHelp(self CliFlag) {
        sus flags_str tea = ""
        ready (self.short != "") {
            flags_str = format_str("-{}", self.short)
        }
        ready (self.long != "") {
            ready (flags_str != "") { flags_str = concat_str(flags_str, ", ") }
            flags_str = concat_str(flags_str, format_str("--{}", self.long))
        }
        
        ready (self.has_value) {
            flags_str = concat_str(flags_str, " <value>")
        }
        
        sus required_marker tea = ready (self.required) { damn " (required)" } otherwise { damn "" }
        vibez.spill("    {:<20} {}{}", flags_str, self.description, required_marker)
    }
}

// Parsed command line arguments
squad ParsedArgs {
    spill command tea
    spill subcommand tea
    spill packages []tea
    spill flags []ParsedFlag
    spill positional []tea
    
    slay new() ParsedArgs {
        damn ParsedArgs{
            command: "",
            subcommand: "",
            packages: []tea{},
            flags: []ParsedFlag{},
            positional: []tea{}
        }
    }
    
    slay hasFlag(self ParsedArgs, flag_name tea) lit {
        sus i drip = 0
        bestie (i < len(self.flags)) {
            sus flag ParsedFlag = self.flags[i]
            ready (flag.name == flag_name) {
                damn based
            }
            i = i + 1
        }
        damn cringe
    }
    
    slay getFlagValue(self ParsedArgs, flag_name tea) tea {
        sus i drip = 0
        bestie (i < len(self.flags)) {
            sus flag ParsedFlag = self.flags[i]
            ready (flag.name == flag_name) {
                damn flag.value
            }
            i = i + 1
        }
        damn ""
    }
    
    slay isVerbose(self ParsedArgs) lit {
        damn self.hasFlag("verbose")
    }
    
    slay isDryRun(self ParsedArgs) lit {
        damn self.hasFlag("dry-run")
    }
}

squad ParsedFlag {
    spill name tea
    spill value tea
    
    slay new(name tea, value tea) ParsedFlag {
        damn ParsedFlag{name: name, value: value}
    }
}

// Main CLI application
squad PackageManagerCLI {
    spill commands []CliCommand
    spill current_directory tea
    spill global_flags []CliFlag
    
    slay new() PackageManagerCLI {
        sus cli PackageManagerCLI = PackageManagerCLI{
            commands: []CliCommand{},
            current_directory: get_current_dir(),
            global_flags: []CliFlag{}
        }
        
        cli.setupCommands()
        cli.setupGlobalFlags()
        
        damn cli
    }
    
    slay setupCommands(self PackageManagerCLI) {
        // Initialize command
        sus init_cmd CliCommand = CliCommand.new(
            "init",
            "Initialize a new CURSED package in the current directory",
            "cursed-pkg init [package-name] [options]"
        )
        init_cmd.addExample("cursed-pkg init my-package")
        init_cmd.addExample("cursed-pkg init my-package --version 0.1.0")
        init_cmd.addFlag(CliFlag.withValue("", "version", "Initial version (default: 0.1.0)"))
        init_cmd.addFlag(CliFlag.withValue("", "description", "Package description"))
        init_cmd.addFlag(CliFlag.withValue("", "author", "Package author"))
        init_cmd.addFlag(CliFlag.withValue("", "license", "Package license (default: MIT)"))
        self.commands = append_array(self.commands, init_cmd)
        
        // Add command
        sus add_cmd CliCommand = CliCommand.new(
            "add",
            "Add a dependency to the current package",
            "cursed-pkg add <package> [version] [options]"
        )
        add_cmd.addExample("cursed-pkg add json")
        add_cmd.addExample("cursed-pkg add json@1.0.0")
        add_cmd.addExample("cursed-pkg add json ^1.0.0")
        add_cmd.addExample("cursed-pkg add testz --dev")
        add_cmd.addFlag(CliFlag.new("", "dev", "Add as development dependency"))
        add_cmd.addFlag(CliFlag.new("", "optional", "Add as optional dependency"))
        add_cmd.addFlag(CliFlag.withValue("", "git", "Add from git repository"))
        add_cmd.addFlag(CliFlag.withValue("", "path", "Add from local path"))
        self.commands = append_array(self.commands, add_cmd)
        
        // Remove command
        sus remove_cmd CliCommand = CliCommand.new(
            "remove",
            "Remove a dependency from the current package",
            "cursed-pkg remove <package> [options]"
        )
        remove_cmd.addExample("cursed-pkg remove json")
        remove_cmd.addExample("cursed-pkg remove testz --dev")
        remove_cmd.addFlag(CliFlag.new("", "dev", "Remove from development dependencies"))
        self.commands = append_array(self.commands, remove_cmd)
        
        // Install command
        sus install_cmd CliCommand = CliCommand.new(
            "install",
            "Install all dependencies for the current package",
            "cursed-pkg install [options]"
        )
        install_cmd.addExample("cursed-pkg install")
        install_cmd.addExample("cursed-pkg install --dev")
        install_cmd.addExample("cursed-pkg install --production")
        install_cmd.addFlag(CliFlag.new("", "dev", "Install development dependencies"))
        install_cmd.addFlag(CliFlag.new("", "production", "Skip development dependencies"))
        install_cmd.addFlag(CliFlag.new("", "force", "Force reinstall all packages"))
        install_cmd.addFlag(CliFlag.withValue("", "cache-dir", "Custom cache directory"))
        self.commands = append_array(self.commands, install_cmd)
        
        // Update command
        sus update_cmd CliCommand = CliCommand.new(
            "update",
            "Update dependencies to their latest compatible versions",
            "cursed-pkg update [package] [options]"
        )
        update_cmd.addExample("cursed-pkg update")
        update_cmd.addExample("cursed-pkg update json")
        update_cmd.addExample("cursed-pkg update --major")
        update_cmd.addFlag(CliFlag.new("", "major", "Allow major version updates"))
        update_cmd.addFlag(CliFlag.new("", "minor", "Allow minor version updates"))
        update_cmd.addFlag(CliFlag.new("", "patch", "Only patch version updates"))
        self.commands = append_array(self.commands, update_cmd)
        
        // Search command
        sus search_cmd CliCommand = CliCommand.new(
            "search",
            "Search for packages in the registry",
            "cursed-pkg search <query> [options]"
        )
        search_cmd.addExample("cursed-pkg search json")
        search_cmd.addExample("cursed-pkg search \"http client\"")
        search_cmd.addExample("cursed-pkg search crypto --limit 20")
        search_cmd.addFlag(CliFlag.withValue("", "limit", "Maximum number of results (default: 10)"))
        search_cmd.addFlag(CliFlag.new("", "details", "Show detailed package information"))
        self.commands = append_array(self.commands, search_cmd)
        
        // Info command
        sus info_cmd CliCommand = CliCommand.new(
            "info",
            "Show detailed information about a package",
            "cursed-pkg info <package> [version] [options]"
        )
        info_cmd.addExample("cursed-pkg info json")
        info_cmd.addExample("cursed-pkg info json@1.0.0")
        info_cmd.addFlag(CliFlag.new("", "versions", "Show all available versions"))
        info_cmd.addFlag(CliFlag.new("", "dependencies", "Show dependency tree"))
        self.commands = append_array(self.commands, info_cmd)
        
        // List command
        sus list_cmd CliCommand = CliCommand.new(
            "list",
            "List installed packages",
            "cursed-pkg list [options]"
        )
        list_cmd.addExample("cursed-pkg list")
        list_cmd.addExample("cursed-pkg list --dev")
        list_cmd.addExample("cursed-pkg list --tree")
        list_cmd.addFlag(CliFlag.new("", "dev", "Show development dependencies"))
        list_cmd.addFlag(CliFlag.new("", "tree", "Show dependency tree"))
        list_cmd.addFlag(CliFlag.new("", "outdated", "Show outdated packages"))
        self.commands = append_array(self.commands, list_cmd)
        
        // Publish command
        sus publish_cmd CliCommand = CliCommand.new(
            "publish",
            "Publish the current package to the registry",
            "cursed-pkg publish [options]"
        )
        publish_cmd.addExample("cursed-pkg publish")
        publish_cmd.addExample("cursed-pkg publish --tag beta")
        publish_cmd.addFlag(CliFlag.withValue("", "tag", "Publish with specific tag"))
        publish_cmd.addFlag(CliFlag.new("", "dry-run", "Show what would be published"))
        publish_cmd.addFlag(CliFlag.new("", "force", "Force publish even if version exists"))
        self.commands = append_array(self.commands, publish_cmd)
        
        // Login command
        sus login_cmd CliCommand = CliCommand.new(
            "login",
            "Authenticate with the package registry",
            "cursed-pkg login [options]"
        )
        login_cmd.addExample("cursed-pkg login")
        login_cmd.addExample("cursed-pkg login --registry https://custom.registry.com")
        login_cmd.addFlag(CliFlag.withValue("", "registry", "Custom registry URL"))
        login_cmd.addFlag(CliFlag.withValue("", "username", "Username for authentication"))
        self.commands = append_array(self.commands, login_cmd)
        
        // Logout command
        sus logout_cmd CliCommand = CliCommand.new(
            "logout",
            "Remove authentication credentials",
            "cursed-pkg logout"
        )
        logout_cmd.addExample("cursed-pkg logout")
        self.commands = append_array(self.commands, logout_cmd)
        
        // Clean command
        sus clean_cmd CliCommand = CliCommand.new(
            "clean",
            "Clean package cache and temporary files",
            "cursed-pkg clean [options]"
        )
        clean_cmd.addExample("cursed-pkg clean")
        clean_cmd.addExample("cursed-pkg clean --cache")
        clean_cmd.addExample("cursed-pkg clean --all")
        clean_cmd.addFlag(CliFlag.new("", "cache", "Clean package cache only"))
        clean_cmd.addFlag(CliFlag.new("", "all", "Clean all temporary files"))
        self.commands = append_array(self.commands, clean_cmd)
    }
    
    slay setupGlobalFlags(self PackageManagerCLI) {
        self.global_flags = append_array(self.global_flags, 
            CliFlag.new("v", "verbose", "Enable verbose output"))
        self.global_flags = append_array(self.global_flags, 
            CliFlag.new("h", "help", "Show help information"))
        self.global_flags = append_array(self.global_flags, 
            CliFlag.new("", "version", "Show version information"))
        self.global_flags = append_array(self.global_flags, 
            CliFlag.new("", "dry-run", "Show what would be done without executing"))
        self.global_flags = append_array(self.global_flags, 
            CliFlag.withValue("", "config", "Use custom configuration file"))
    }
    
    slay parseArgs(self PackageManagerCLI, args []tea) ParsedArgs {
        sus parsed ParsedArgs = ParsedArgs.new()
        
        ready (len(args) == 0) {
            damn parsed
        }
        
        // First argument is the command
        parsed.command = args[0]
        
        sus i drip = 1
        bestie (i < len(args)) {
            sus arg tea = args[i]
            
            ready (starts_with(arg, "--")) {
                // Long flag
                sus flag_name tea = slice_str(arg, 2)
                sus flag_value tea = ""
                
                // Check if flag has value (--flag=value)
                sus eq_pos drip = find_str(flag_name, "=")
                ready (eq_pos != -1) {
                    flag_value = slice_str(flag_name, eq_pos + 1)
                    flag_name = slice_str(flag_name, 0, eq_pos)
                } otherwise {
                    // Check if next argument is the value
                    ready (i + 1 < len(args) && !starts_with(args[i + 1], "-")) {
                        i = i + 1
                        flag_value = args[i]
                    }
                }
                
                sus flag ParsedFlag = ParsedFlag.new(flag_name, flag_value)
                parsed.flags = append_array(parsed.flags, flag)
                
            } otherwise ready (starts_with(arg, "-") && len(arg) > 1) {
                // Short flag
                sus flag_name tea = slice_str(arg, 1)
                sus flag_value tea = ""
                
                // Check if next argument is the value
                ready (i + 1 < len(args) && !starts_with(args[i + 1], "-")) {
                    i = i + 1
                    flag_value = args[i]
                }
                
                sus flag ParsedFlag = ParsedFlag.new(flag_name, flag_value)
                parsed.flags = append_array(parsed.flags, flag)
                
            } otherwise {
                // Positional argument
                ready (parsed.subcommand == "" && i == 1) {
                    parsed.subcommand = arg
                } otherwise {
                    parsed.positional = append_array(parsed.positional, arg)
                }
            }
            
            i = i + 1
        }
        
        damn parsed
    }
    
    slay run(self PackageManagerCLI, args []tea) drip {
        sus parsed ParsedArgs = self.parseArgs(args)
        
        // Handle global flags first
        ready (parsed.hasFlag("help")) {
            self.printHelp(parsed.command)
            damn 0
        }
        
        ready (parsed.hasFlag("version")) {
            self.printVersion()
            damn 0
        }
        
        // Handle empty command
        ready (parsed.command == "") {
            self.printGeneralHelp()
            damn 1
        }
        
        // Execute command
        damn self.executeCommand(parsed)
    }
    
    slay executeCommand(self PackageManagerCLI, args ParsedArgs) drip {
        ready (args.isVerbose()) {
            vibez.spill("Executing command: {}", args.command)
        }
        
        sus cmd tea = args.command
        
        ready (cmd == "init") {
            damn self.handleInit(args)
        }
        ready (cmd == "add") {
            damn self.handleAdd(args)
        }
        ready (cmd == "remove") {
            damn self.handleRemove(args)
        }
        ready (cmd == "install") {
            damn self.handleInstall(args)
        }
        ready (cmd == "update") {
            damn self.handleUpdate(args)
        }
        ready (cmd == "search") {
            damn self.handleSearch(args)
        }
        ready (cmd == "info") {
            damn self.handleInfo(args)
        }
        ready (cmd == "list") {
            damn self.handleList(args)
        }
        ready (cmd == "publish") {
            damn self.handlePublish(args)
        }
        ready (cmd == "login") {
            damn self.handleLogin(args)
        }
        ready (cmd == "logout") {
            damn self.handleLogout(args)
        }
        ready (cmd == "clean") {
            damn self.handleClean(args)
        }
        
        vibez.spill("Unknown command: {}", cmd)
        vibez.spill("Run 'cursed-pkg help' for available commands")
        damn 1
    }
    
    // Command handlers
    slay handleInit(self PackageManagerCLI, args ParsedArgs) drip {
        sus package_name tea = ready (len(args.positional) > 0) {
            damn args.positional[0]
        } otherwise {
            damn get_current_dir_name()
        }
        
        sus version tea = args.getFlagValue("version")
        ready (version == "") { version = "0.1.0" }
        
        ready (args.isVerbose()) {
            vibez.spill("Initializing package '{}' version {}", package_name, version)
        }
        
        sus success lit = cmd_init(package_name, version)
        
        ready (success) {
            vibez.spill("Initialized package {} in {}", package_name, self.current_directory)
            damn 0
        } otherwise {
            vibez.spill("Failed to initialize package")
            damn 1
        }
    }
    
    slay handleAdd(self PackageManagerCLI, args ParsedArgs) drip {
        ready (len(args.positional) == 0) {
            vibez.spill("Error: Package name required")
            vibez.spill("Usage: cursed-pkg add <package> [version]")
            damn 1
        }
        
        sus package_spec tea = args.positional[0]
        sus package_name tea = package_spec
        sus version tea = "latest"
        
        // Parse package@version syntax
        sus at_pos drip = find_str(package_spec, "@")
        ready (at_pos != -1) {
            package_name = slice_str(package_spec, 0, at_pos)
            version = slice_str(package_spec, at_pos + 1)
        } otherwise ready (len(args.positional) > 1) {
            version = args.positional[1]
        }
        
        sus is_dev lit = args.hasFlag("dev")
        
        ready (args.isVerbose()) {
            sus dep_type tea = ready (is_dev) { damn "development dependency" } otherwise { damn "dependency" }
            vibez.spill("Adding {} {} version {}", dep_type, package_name, version)
        }
        
        sus success lit = cmd_add(package_name, version, is_dev)
        
        ready (success) {
            damn 0
        } otherwise {
            damn 1
        }
    }
    
    slay handleRemove(self PackageManagerCLI, args ParsedArgs) drip {
        ready (len(args.positional) == 0) {
            vibez.spill("Error: Package name required")
            vibez.spill("Usage: cursed-pkg remove <package>")
            damn 1
        }
        
        sus package_name tea = args.positional[0]
        
        ready (args.isVerbose()) {
            vibez.spill("Removing package {}", package_name)
        }
        
        sus success lit = cmd_remove(package_name)
        
        ready (success) {
            damn 0
        } otherwise {
            damn 1
        }
    }
    
    slay handleInstall(self PackageManagerCLI, args ParsedArgs) drip {
        ready (args.isVerbose()) {
            vibez.spill("Installing dependencies...")
        }
        
        ready (args.isDryRun()) {
            vibez.spill("Dry run - would install dependencies")
            damn 0
        }
        
        sus success lit = cmd_install()
        
        ready (success) {
            damn 0
        } otherwise {
            damn 1
        }
    }
    
    slay handleSearch(self PackageManagerCLI, args ParsedArgs) drip {
        ready (len(args.positional) == 0) {
            vibez.spill("Error: Search query required")
            vibez.spill("Usage: cursed-pkg search <query>")
            damn 1
        }
        
        sus query tea = args.positional[0]
        
        ready (args.isVerbose()) {
            vibez.spill("Searching for packages matching '{}'", query)
        }
        
        cmd_search(query)
        damn 0
    }
    
    slay handleInfo(self PackageManagerCLI, args ParsedArgs) drip {
        ready (len(args.positional) == 0) {
            vibez.spill("Error: Package name required")
            vibez.spill("Usage: cursed-pkg info <package>")
            damn 1
        }
        
        sus package_name tea = args.positional[0]
        
        ready (args.isVerbose()) {
            vibez.spill("Getting information for package {}", package_name)
        }
        
        // Get package info from registry
        sus cache_dir tea = ".cursed/cache"
        sus client PackageRegistryClient = PackageRegistryClient.new(
            "https://packages.cursed.dev/api/v1", 
            cache_dir
        )
        
        sus info PackageInfo = client.getPackageInfo(package_name)
        ready (info.name != "") {
            vibez.spill("Package: {}", info.name)
            vibez.spill("Version: {}", info.version.toString())
            vibez.spill("Description: {}", info.description)
            damn 0
        } otherwise {
            vibez.spill("Package {} not found", package_name)
            damn 1
        }
    }
    
    slay handleList(self PackageManagerCLI, args ParsedArgs) drip {
        ready (args.isVerbose()) {
            vibez.spill("Listing installed packages...")
        }
        
        // Implementation would list packages from manifest and cache
        vibez.spill("Installed packages:")
        
        ready (file_exists("package.csd")) {
            sus manifest PackageManifest = PackageManifest.loadFromFile("package.csd")
            
            ready (len(manifest.dependencies) > 0) {
                vibez.spill("\nDependencies:")
                sus i drip = 0
                bestie (i < len(manifest.dependencies)) {
                    sus dep PackageDependency = manifest.dependencies[i]
                    vibez.spill("  {} {}", dep.name, dep.version_constraint)
                    i = i + 1
                }
            }
            
            ready (len(manifest.dev_dependencies) > 0 && !args.hasFlag("production")) {
                vibez.spill("\nDevelopment dependencies:")
                sus i drip = 0
                bestie (i < len(manifest.dev_dependencies)) {
                    sus dep PackageDependency = manifest.dev_dependencies[i]
                    vibez.spill("  {} {}", dep.name, dep.version_constraint)
                    i = i + 1
                }
            }
        } otherwise {
            vibez.spill("  (no package.csd found)")
        }
        
        damn 0
    }
    
    slay handlePublish(self PackageManagerCLI, args ParsedArgs) drip {
        ready (!file_exists("package.csd")) {
            vibez.spill("Error: No package.csd found in current directory")
            damn 1
        }
        
        ready (args.isVerbose()) {
            vibez.spill("Publishing package...")
        }
        
        ready (args.isDryRun()) {
            vibez.spill("Dry run - would publish package")
            damn 0
        }
        
        sus success lit = cmd_publish()
        
        ready (success) {
            damn 0
        } otherwise {
            damn 1
        }
    }
    
    slay handleLogin(self PackageManagerCLI, args ParsedArgs) drip {
        sus username tea = args.getFlagValue("username")
        ready (username == "") {
            vibez.spill("Username: ")
            username = read_line()
        }
        
        vibez.spill("Password: ")
        sus password tea = read_password()
        
        sus registry_url tea = args.getFlagValue("registry")
        ready (registry_url == "") {
            registry_url = "https://packages.cursed.dev/api/v1"
        }
        
        sus client PackageRegistryClient = PackageRegistryClient.new(registry_url, ".cursed/cache")
        sus success lit = client.authenticate(username, password)
        
        ready (success) {
            vibez.spill("Successfully logged in as {}", username)
            damn 0
        } otherwise {
            vibez.spill("Login failed")
            damn 1
        }
    }
    
    slay handleLogout(self PackageManagerCLI, args ParsedArgs) drip {
        sus auth_file tea = ".cursed/cache/auth.txt"
        ready (file_exists(auth_file)) {
            delete_file(auth_file)
            vibez.spill("Logged out successfully")
        } otherwise {
            vibez.spill("Not currently logged in")
        }
        damn 0
    }
    
    slay handleClean(self PackageManagerCLI, args ParsedArgs) drip {
        ready (args.isVerbose()) {
            vibez.spill("Cleaning package cache...")
        }
        
        sus cache_dir tea = ".cursed/cache"
        ready (dir_exists(cache_dir)) {
            delete_dir_recursive(cache_dir)
            vibez.spill("Cache cleaned successfully")
        } otherwise {
            vibez.spill("No cache to clean")
        }
        
        damn 0
    }
    
    // Help and information methods
    slay printGeneralHelp(self PackageManagerCLI) {
        vibez.spill("CURSED Package Manager")
        vibez.spill("")
        vibez.spill("A fast, reliable package manager for the CURSED programming language")
        vibez.spill("")
        vibez.spill("USAGE:")
        vibez.spill("    cursed-pkg <COMMAND> [OPTIONS]")
        vibez.spill("")
        vibez.spill("COMMANDS:")
        
        sus i drip = 0
        bestie (i < len(self.commands)) {
            sus cmd CliCommand = self.commands[i]
            vibez.spill("    {:<12} {}", cmd.name, cmd.description)
            i = i + 1
        }
        
        vibez.spill("")
        vibez.spill("GLOBAL OPTIONS:")
        i = 0
        bestie (i < len(self.global_flags)) {
            sus flag CliFlag = self.global_flags[i]
            flag.printHelp()
            i = i + 1
        }
        
        vibez.spill("")
        vibez.spill("Use 'cursed-pkg help <command>' for more information about a specific command.")
    }
    
    slay printHelp(self PackageManagerCLI, command_name tea) {
        ready (command_name == "") {
            self.printGeneralHelp()
            damn
        }
        
        sus i drip = 0
        bestie (i < len(self.commands)) {
            sus cmd CliCommand = self.commands[i]
            ready (cmd.name == command_name) {
                cmd.printHelp()
                damn
            }
            i = i + 1
        }
        
        vibez.spill("Unknown command: {}", command_name)
        vibez.spill("Use 'cursed-pkg help' to see available commands.")
    }
    
    slay printVersion(self PackageManagerCLI) {
        vibez.spill("CURSED Package Manager v1.0.0")
        vibez.spill("Built with CURSED programming language")
    }
}

// Utility functions
slay get_current_dir() tea {
    // Mock implementation - would use filez stdlib
    damn "/current/directory"
}

slay get_current_dir_name() tea {
    sus full_path tea = get_current_dir()
    sus parts []tea = split_str(full_path, "/")
    ready (len(parts) > 0) {
        damn parts[len(parts) - 1]
    }
    damn "my-package"
}

slay read_line() tea {
    // Mock implementation - would use vibez stdlib
    damn "user_input"
}

slay read_password() tea {
    // Mock implementation - would use vibez stdlib with hidden input
    damn "password123"
}

// Main entry point for CLI application
slay main() drip {
    sus cli PackageManagerCLI = PackageManagerCLI.new()
    
    // Get command line arguments (mock for now)
    sus args []tea = []tea{}
    args = append_array(args, "add")
    args = append_array(args, "json")
    args = append_array(args, "--verbose")
    
    damn cli.run(args)
}

// Run CLI if this module is executed directly
main()
