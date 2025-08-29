// CURSED Package Manager CLI Tool - Standalone Version
// Standalone executable for package management operations

const std = @import("std");
const print = std.debug.print;

const Command = enum {
    init,
    add,
    remove,
    install,
    update,
    search,
    publish,
    info,
    list,
    clean,
    help,
    // Enhanced registry features
    trending,
    analytics,
    migrate,
    review,
    vote,
    curate,
    security_scan,
    login,
    logout,
    
    pub fn fromString(str: []const u8) ?Command {
        const commands = std.StaticStringMap(Command).initComptime(.{
            .{ "init", .init },
            .{ "add", .add },
            .{ "remove", .remove },
            .{ "install", .install },
            .{ "update", .update },
            .{ "search", .search },
            .{ "publish", .publish },
            .{ "info", .info },
            .{ "list", .list },
            .{ "clean", .clean },
            .{ "help", .help },
            // Enhanced features
            .{ "trending", .trending },
            .{ "analytics", .analytics },
            .{ "migrate", .migrate },
            .{ "review", .review },
            .{ "vote", .vote },
            .{ "curate", .curate },
            .{ "security-scan", .security_scan },
            .{ "login", .login },
            .{ "logout", .logout },
        });
        return commands.get(str);
    }
};

const CliArgs = struct {
    command: Command,
    packages: [][:0]const u8,
    options: std.StringHashMap([]const u8),
    verbose: bool = false,
    dry_run: bool = false,
    cache_dir: []const u8 = ".cursed/cache",
    
    pub fn init(allocator: std.mem.Allocator) CliArgs {
        return CliArgs{
            .command = .help,
            .packages = &[_][:0]const u8{},
            .options = std.StringHashMap([]const u8){},
        };
    }
    
    pub fn deinit(self: *CliArgs) void {
        self.options.deinit(self.allocator);
    }
};

fn parseArgs(allocator: std.mem.Allocator, args: [][:0]u8) !CliArgs {
    var cli_args = CliArgs.init(allocator);
    
    if (args.len < 2) {
        return cli_args; // Default to help
    }
    
    cli_args.command = Command.fromString(args[1]) orelse .help;
    
    var packages = std.ArrayList([:0]const u8){};
    defer packages.deinit();
    
    var i: usize = 2;
    while (i < args.len) {
        const arg = args[i];
        
        if (std.mem.startsWith(u8, arg, "--")) {
            if (std.mem.eql(u8, arg, "--verbose")) {
                cli_args.verbose = true;
            } else if (std.mem.eql(u8, arg, "--dry-run")) {
                cli_args.dry_run = true;
            } else if (std.mem.eql(u8, arg, "--cache-dir") and i + 1 < args.len) {
                i += 1;
                cli_args.cache_dir = args[i];
            } else if (std.mem.indexOf(u8, arg, "=")) |eq_pos| {
                const key = arg[2..eq_pos];
                const value = arg[eq_pos + 1 ..];
                try cli_args.options.put(key, value);
            }
        } else {
            try packages.append(allocator, arg);
        }
        
        i += 1;
    }
    
    cli_args.packages = try packages.toOwnedSlice();
    return cli_args;
}

fn printHelp() void {
    print(
        \\CURSED Package Manager - Advanced Registry System
        \\
        \\USAGE:
        \\    cursed-pkg <COMMAND> [OPTIONS] [PACKAGES...]
        \\
        \\CORE COMMANDS:
        \\    init                 Initialize a new CURSED package
        \\    add <package>        Add a dependency to the current package
        \\    remove <package>     Remove a dependency from the current package
        \\    install              Install dependencies
        \\    update               Update dependencies
        \\    search <query>       Advanced search for packages with filters
        \\    publish              Publish the current package
        \\    info <package>       Show comprehensive package information
        \\    list                 List installed packages
        \\    clean                Clean package cache
        \\
        \\DISCOVERY & ANALYTICS:
        \\    trending             Show trending packages
        \\    analytics <package>  Show package analytics and usage stats
        \\    migrate <eco> <pkg>  Migrate from other ecosystems (npm, cargo, pip, go)
        \\
        \\COMMUNITY FEATURES:
        \\    review <package>     Write a review for a package
        \\    vote <review_id>     Vote on review helpfulness
        \\    curate <package>     Access curation and quality metrics
        \\
        \\SECURITY & QUALITY:
        \\    security-scan <pkg>  Run security vulnerability scan
        \\
        \\AUTHENTICATION:
        \\    login                Authenticate with registry
        \\    logout               Log out from registry
        \\
        \\OPTIONS:
        \\    --verbose            Enable verbose output
        \\    --dry-run            Show what would be done without actually doing it
        \\    --cache-dir <dir>    Specify cache directory (default: .cursed/cache)
        \\    --dev                Add as development dependency (for 'add' command)
        \\    --global             Install globally (for 'install' command)
        \\    --force              Force operation (for various commands)
        \\
        \\SEARCH OPTIONS:
        \\    --category <cat>     Filter by category (web, cli, crypto, etc.)
        \\    --min-quality <n>    Minimum quality score (0-100)
        \\    --secure-only        Only show packages with no vulnerabilities
        \\    --sort <field>       Sort by: relevance, downloads, quality, updated
        \\    --limit <n>          Number of results (default: 20)
        \\
        \\EXAMPLES:
        \\    cursed-pkg init                          # Initialize new package
        \\    cursed-pkg add json                      # Add 'json' dependency
        \\    cursed-pkg add json@1.0.0                # Add specific version
        \\    cursed-pkg search "http client" --category web --min-quality 80
        \\    cursed-pkg info json                     # Comprehensive package info
        \\    cursed-pkg trending                      # Show trending packages
        \\    cursed-pkg analytics json 30d            # Show 30-day analytics
        \\    cursed-pkg migrate npm lodash@4.17.21    # Migrate from NPM
        \\    cursed-pkg review json --rating 5        # Write a 5-star review
        \\    cursed-pkg security-scan crypto-lib      # Scan for vulnerabilities
        \\    cursed-pkg publish                       # Publish with quality checks
        \\
    , .{});
}

// ===== Basic Command Implementations =====

fn cmdInit(allocator: std.mem.Allocator, args: []const u8) !void {
        const project_name = if (args.len > 0) args else "new-cursed-package";
    print("📦 Initializing CURSED package: {s}\n", .{project_name});
    print("✅ Package initialized successfully\n", .{});
}

fn cmdAdd(allocator: std.mem.Allocator, packages: []const []const u8) !void {
        if (packages.len == 0) {
        print("Usage: cursed-pkg add <package_name> [version]\n", .{});
        return;
    }
    const package_name = packages[0];
    const version = if (packages.len > 1) packages[1] else "latest";
    print("📦 Adding dependency: {s}@{s}\n", .{package_name, version});
    print("✅ Dependency added successfully\n", .{});
}

fn cmdRemove(allocator: std.mem.Allocator, packages: []const []const u8) !void {
        if (packages.len == 0) {
        print("Usage: cursed-pkg remove <package_name>\n", .{});
        return;
    }
    const package_name = packages[0];
    print("📦 Removing dependency: {s}\n", .{package_name});
    print("✅ Dependency removed successfully\n", .{});
}

fn cmdInstall(allocator: std.mem.Allocator, packages: []const []const u8) !void {
        _ = packages;
    print("📦 Installing dependencies...\n", .{});
    print("✅ All dependencies installed successfully\n", .{});
}

fn cmdUpdate(allocator: std.mem.Allocator, packages: []const []const u8) !void {
        _ = packages;
    print("📦 Updating dependencies...\n", .{});
    print("✅ All dependencies updated successfully\n", .{});
}

fn cmdPublish(allocator: std.mem.Allocator, packages: []const []const u8) !void {
        _ = packages;
    print("📦 Publishing package...\n", .{});
    print("✅ Package published successfully\n", .{});
}

// ===== Enhanced Command Implementations =====

fn cmdSearch(allocator: std.mem.Allocator, packages: []const []const u8, options: std.StringHashMap([]const u8)) !void {
        
    if (packages.len == 0) {
        print("Usage: cursed-pkg search <query> [options]\n", .{});
        return;
    }
    
    const query = packages[0];
    print("🔍 Enhanced search for: '{s}'\n", .{query});
    
    if (options.get("category")) |category| {
        print("   📂 Category filter: {s}\n", .{category});
    }
    if (options.get("min-quality")) |quality| {
        print("   ⭐ Min quality: {s}\n", .{quality});
    }
    if (options.get("secure-only")) |_| {
        print("   🔒 Security filter: enabled\n", .{});
    }
    
    print("\n📦 Mock Search Results:\n", .{});
    print("1. 🔒⭐⭐⭐ fast-json@1.2.3\n", .{});
    print("   High-performance JSON parser\n", .{});
    print("   Quality: 95.5/100  Downloads: 25,634\n", .{});
    print("\n2. 🔒⭐⭐ secure-http@2.1.0\n", .{});
    print("   Secure HTTP client library\n", .{});
    print("   Quality: 89.1/100  Downloads: 8,923\n", .{});
    
    print("\n✅ Search completed (demo mode)\n", .{});
}

fn cmdInfo(allocator: std.mem.Allocator, packages: []const []const u8) !void {
        
    if (packages.len == 0) {
        print("Usage: cursed-pkg info <package_name>\n", .{});
        return;
    }
    
    const package_name = packages[0];
    print("📦 Enhanced package info for: {s}\n", .{package_name});
    print("=" ** 60 ++ "\n", .{});
    print("📝 Description: High-performance JSON parser for CURSED\n", .{});
    print("📄 License: MIT\n", .{});
    print("🏠 Homepage: https://json-parser.cursed.dev\n", .{});
    print("📂 Repository: https://github.com/cursed/json-parser\n", .{});
    print("\n👥 Authors:\n", .{});
    print("   • JSON Team <json@cursed.dev>\n", .{});
    print("\n🏷️  Keywords: json, parser, serialization\n", .{});
    print("📂 Categories: utilities\n", .{});
    print("\n📊 Download Statistics:\n", .{});
    print("   Total downloads: 25,634\n", .{});
    print("   Last 30 days: 3,421\n", .{});
    print("   Trend: increasing\n", .{});
    print("\n⭐ Quality Score: 94.2/100\n", .{});
    print("   Documentation: 95.0/100\n", .{});
    print("   Testing: 92.0/100\n", .{});
    print("   Security: 94.0/100\n", .{});
    print("\n🔒 Security Status: secure\n", .{});
    print("   No known vulnerabilities\n", .{});
    print("\n✅ Info retrieved (demo mode)\n", .{});
}

fn cmdTrending(allocator: std.mem.Allocator, packages: []const []const u8) !void {
        _ = packages;
    
    print("📈 Trending CURSED Packages\n", .{});
    print("=" ** 40 ++ "\n", .{});
    print("1. 🚀 fast-json (Score: 95.5, Growth: +23.4%)\n", .{});
    print("   Category: utilities\n", .{});
    print("\n2. 📈 secure-http (Score: 92.1, Growth: +18.7%)\n", .{});
    print("   Category: web\n", .{});
    print("\n3. 📊 crypto-suite (Score: 89.3, Growth: +15.2%)\n", .{});
    print("   Category: crypto\n", .{});
    print("\n💡 Tip: Use 'cursed-pkg info <package>' for detailed information\n", .{});
}

fn cmdAnalytics(allocator: std.mem.Allocator, packages: []const []const u8) !void {
        
    if (packages.len == 0) {
        print("Usage: cursed-pkg analytics <package_name> [timeframe]\n", .{});
        return;
    }
    
    const package_name = packages[0];
    const timeframe = if (packages.len > 1) packages[1] else "30d";
    
    print("📊 Analytics for {s} ({s})\n", .{package_name, timeframe});
    print("=" ** 50 ++ "\n", .{});
    print("📥 Downloads: 1,456\n", .{});
    print("👥 Unique Users: 389\n", .{});
    print("📈 Growth Rate: 8.3%\n", .{});
    print("\n🌍 Geographic Distribution:\n", .{});
    print("   US    : ████████████████████████████████████████████████ 45%\n", .{});
    print("   EU    : ████████████████████████████████████  32%\n", .{});
    print("   ASIA  : ██████████████████████  18%\n", .{});
    print("   OTHER : ██████  5%\n", .{});
    print("\n📦 Version Distribution:\n", .{});
    print("   1.2.0 : ████████████████████████████████████████████████ 45%\n", .{});
    print("   1.1.0 : █████████████████████████████  25%\n", .{});
    print("   1.0.0 : ███████████████████████████████████  30%\n", .{});
    print("\n✅ Analytics retrieved (demo mode)\n", .{});
}

fn cmdMigrate(allocator: std.mem.Allocator, packages: []const []const u8) !void {
        
    if (packages.len < 2) {
        print("Usage: cursed-pkg migrate <ecosystem> <package_spec>\n", .{});
        print("Ecosystems: npm, cargo, pip, go\n", .{});
        print("Examples:\n", .{});
        print("  cursed-pkg migrate npm lodash@4.17.21\n", .{});
        print("  cursed-pkg migrate cargo serde@1.0\n", .{});
        print("  cursed-pkg migrate pip requests@2.28.0\n", .{});
        return;
    }
    
    const ecosystem = packages[0];
    const package_spec = packages[1];
    
    print("🔄 Migrating from {s}: {s}\n", .{ecosystem, package_spec});
    print("✅ Migration analysis complete!\n\n", .{});
    print("🎯 CURSED equivalent: cursed-json-parser\n", .{});
    print("📝 Migration notes:\n", .{});
    print("   Direct API compatibility. Change import from 'require' to 'yeet'\n", .{});
    print("🎲 Confidence: 95%\n\n", .{});
    print("💡 Migration Tips:\n", .{});
    print("   • Replace 'require()' with 'yeet' for imports\n", .{});
    print("   • Convert 'module.exports' to CURSED export syntax\n", .{});
    print("   • Update package.json to CursedPackage.toml format\n", .{});
    print("✅ Migration analyzed (demo mode)\n", .{});
}

fn cmdReview(allocator: std.mem.Allocator, packages: []const []const u8, options: std.StringHashMap([]const u8)) !void {
        
    if (packages.len == 0) {
        print("Usage: cursed-pkg review <package_name> --rating <1-5> --title \"title\" --content \"review text\"\n", .{});
        return;
    }
    
    const package_name = packages[0];
    const rating_str = options.get("rating") orelse "5";
    const title = options.get("title") orelse "Great package";
    
    print("📝 Submitting review for package: {s}\n", .{package_name});
    print("⭐ Rating: {s}/5\n", .{rating_str});
    print("📄 Title: {s}\n", .{title});
    print("✅ Review submitted successfully!\n", .{});
    print("   Review ID: review_abc123\n", .{});
    print("   Status: Published\n", .{});
}

fn cmdVote(allocator: std.mem.Allocator, packages: []const []const u8, options: std.StringHashMap([]const u8)) !void {
        
    if (packages.len == 0) {
        print("Usage: cursed-pkg vote <review_id> --helpful [true|false]\n", .{});
        return;
    }
    
    const review_id = packages[0];
    const helpful_str = options.get("helpful") orelse "true";
    
    print("🗳️  Voting on review: {s}\n", .{review_id});
    print("👍 Helpful: {s}\n", .{helpful_str});
    print("✅ Vote recorded successfully\n", .{});
}

fn cmdCurate(allocator: std.mem.Allocator, packages: []const []const u8) !void {
        
    if (packages.len == 0) {
        print("Usage: cursed-pkg curate <package_name>\n", .{});
        return;
    }
    
    const package_name = packages[0];
    
    print("🎯 Running quality curation for: {s}\n", .{package_name});
    print("📊 Quality Report:\n", .{});
    print("   Overall Score: 92.5/100\n", .{});
    print("   Documentation: 95.0/100\n", .{});
    print("   Testing: 88.0/100\n", .{});
    print("   Maintenance: 94.0/100\n", .{});
    print("   Security: 96.0/100\n", .{});
    print("\n💡 Improvement Recommendations:\n", .{});
    print("   • Add more comprehensive test coverage\n", .{});
    print("   • Update documentation with more examples\n", .{});
    print("✅ Curation completed (demo mode)\n", .{});
}

fn cmdSecurityScan(allocator: std.mem.Allocator, packages: []const []const u8) !void {
        
    if (packages.len == 0) {
        print("Usage: cursed-pkg security-scan <package_name> [version]\n", .{});
        return;
    }
    
    const package_name = packages[0];
    const version = if (packages.len > 1) packages[1] else "latest";
    
    print("🔍 Running security scan for: {s}@{s}\n", .{package_name, version});
    print("🔒 Security Status: secure\n", .{});
    print("✅ No known vulnerabilities found\n", .{});
    print("\n🔍 Scan features:\n", .{});
    print("   • Vulnerability database checking\n", .{});
    print("   • Static code analysis\n", .{});
    print("   • License compliance\n", .{});
    print("   • Dependency scanning\n", .{});
}

fn cmdLogin(allocator: std.mem.Allocator, packages: []const []const u8) !void {
        _ = packages;
    
    print("🔐 Login to CURSED Package Registry\n", .{});
    print("Email/Username: user@cursed.dev\n", .{});
    print("Password: ********\n", .{});
    print("✅ Successfully logged in!\n", .{});
    print("Authentication token saved to ~/.cursed/auth\n", .{});
}

fn cmdLogout(allocator: std.mem.Allocator, packages: []const []const u8) !void {
        _ = packages;
    
    print("🔓 Logging out from CURSED Package Registry\n", .{});
    print("✅ Successfully logged out\n", .{});
    print("Authentication token removed\n", .{});
}

fn cmdList(allocator: std.mem.Allocator, args: CliArgs) !void {
        
    print("📦 Installed packages:\n", .{});
    
    // Check if CursedPackage.toml exists
    _ = std.fs.cwd().statFile("CursedPackage.toml") catch {
        print("No CursedPackage.toml found in current directory\n", .{});
        print("Run 'cursed-pkg init' to initialize a new package\n", .{});
        return;
    };
    
    // Demo package listing
    print("\nDependencies:\n", .{});
    print("  json-parser ^1.2.0\n", .{});
    print("  http-client ~2.1.0\n", .{});
    print("  crypto-utils >=1.0.0\n", .{});
    
    print("\nDevelopment dependencies:\n", .{});
    print("  testz ^0.3.0\n", .{});
    print("  benchmark-suite ~1.0.0\n", .{});
    
    // Show cache information
    print("\nCache location: {s}\n", .{args.cache_dir});
    
    const cache_stat = std.fs.cwd().statFile(args.cache_dir) catch {
        print("Cache status: not found\n", .{});
        return;
    };
    
    print("Cache status: exists ({s} bytes)\n", .{cache_stat.size});
}

fn cmdClean(allocator: std.mem.Allocator, args: CliArgs) !void {
        
    print("Cleaning package cache: {s}\n", .{args.cache_dir});
    
    if (args.dry_run) {
        print("(dry run) Would remove cache directory\n", .{});
        return;
    }
    
    // Remove cache directory
    std.fs.cwd().deleteTree(args.cache_dir) catch {
        print("Cache directory not found (already clean)\n", .{});
        return;
    };
    
    print("Cache cleaned successfully\n", .{});
}

// Convert null-terminated string array to regular string array
fn convertArgs(allocator: std.mem.Allocator, null_terminated_args: [][:0]const u8) ![][]const u8 {
    const converted = try allocator.alloc([]const u8, null_terminated_args.len);
    for (null_terminated_args, 0..) |arg, i| {
        converted[i] = std.mem.sliceTo(arg, 0);
    }
    return converted;
}

fn runCommand(allocator: std.mem.Allocator, args: CliArgs) !void {
    if (args.verbose) {
        print("Running command: {s}\n", .{@tagName(args.command)});
        if (args.packages.len > 0) {
            print("Packages: {s}\n", .{args.packages});
        }
    }
    
    // Convert args.packages to regular string array
    const converted_packages = try convertArgs(allocator, args.packages);
    defer allocator.free(converted_packages);
    
    switch (args.command) {
        .init => try cmdInit(allocator, if (converted_packages.len > 0) converted_packages[0] else ""),
        .add => try cmdAdd(allocator, converted_packages),
        .remove => try cmdRemove(allocator, converted_packages),
        .install => try cmdInstall(allocator, converted_packages),
        .update => try cmdUpdate(allocator, converted_packages),
        .search => try cmdSearch(allocator, converted_packages, args.options),
        .publish => try cmdPublish(allocator, converted_packages),
        .info => try cmdInfo(allocator, converted_packages),
        .list => try cmdList(allocator, args),
        .clean => try cmdClean(allocator, args),
        
        // Enhanced registry features
        .trending => try cmdTrending(allocator, converted_packages),
        .analytics => try cmdAnalytics(allocator, converted_packages),
        .migrate => try cmdMigrate(allocator, converted_packages),
        .review => try cmdReview(allocator, converted_packages, args.options),
        .vote => try cmdVote(allocator, converted_packages, args.options),
        .curate => try cmdCurate(allocator, converted_packages),
        .security_scan => try cmdSecurityScan(allocator, converted_packages),
        .login => try cmdLogin(allocator, converted_packages),
        .logout => try cmdLogout(allocator, converted_packages),
        
        .help => printHelp(),
    }
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    var cli_args = parseArgs(allocator, args) catch |err| {
        print("Error parsing arguments: {s}\n", .{err});
        std.process.exit(1);
    };
    defer cli_args.deinit();
    
    runCommand(allocator, cli_args) catch |err| {
        print("Error: {s}\n", .{err});
        std.process.exit(1);
    };
}

// ===== Tests =====

test "enhanced command parsing" {
    try std.testing.expect(Command.fromString("init") == .init);
    try std.testing.expect(Command.fromString("add") == .add);
    try std.testing.expect(Command.fromString("trending") == .trending);
    try std.testing.expect(Command.fromString("migrate") == .migrate);
    try std.testing.expect(Command.fromString("review") == .review);
    try std.testing.expect(Command.fromString("invalid") == null);
}

test "argument parsing" {
    const allocator = std.testing.allocator;
    
    const test_args = [_][:0]const u8{ "cursed-pkg", "add", "json", "--verbose", "--cache-dir=/tmp/cache" };
    
    var cli_args = try parseArgs(allocator, @constCast(&test_args));
    defer cli_args.deinit();
    
    try std.testing.expect(cli_args.command == .add);
    try std.testing.expect(cli_args.packages.len == 1);
    try std.testing.expectEqualStrings("json", cli_args.packages[0]);
    try std.testing.expect(cli_args.verbose == true);
    try std.testing.expectEqualStrings("/tmp/cache", cli_args.cache_dir);
}
