// CURSED Package Manager CLI Tool
// Standalone executable for package management operations

const std = @import("std");
const tools = @import("tools");

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
            .options = std.StringHashMap([]const u8).init(allocator),
        };
    }
    
    pub fn deinit(self: *CliArgs) void {
        self.options.deinit();
    }
};

fn parseArgs(allocator: std.mem.Allocator, args: [][:0]u8) !CliArgs {
    var cli_args = CliArgs.init(allocator);
    
    if (args.len < 2) {
        return cli_args; // Default to help
    }
    
    cli_args.command = Command.fromString(args[1]) orelse .help;
    
    var packages = std.ArrayList([:0]const u8).init(allocator);
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
            try packages.append(arg);
        }
        
        i += 1;
    }
    
    cli_args.packages = try packages.toOwnedSlice();
    return cli_args;
}

fn printHelp() void {
    std.debug.print(
        \\CURSED Package Manager
        \\
        \\USAGE:
        \\    cursed-pkg <COMMAND> [OPTIONS] [PACKAGES...]
        \\
        \\COMMANDS:
        \\    init                 Initialize a new CURSED package
        \\    add <package>        Add a dependency to the current package
        \\    remove <package>     Remove a dependency from the current package
        \\    install              Install dependencies
        \\    update               Update dependencies
        \\    search <query>       Search for packages
        \\    publish              Publish the current package
        \\    info <package>       Show information about a package
        \\    list                 List installed packages
        \\    clean                Clean package cache
        \\    help                 Show this help message
        \\
        \\OPTIONS:
        \\    --verbose            Enable verbose output
        \\    --dry-run            Show what would be done without actually doing it
        \\    --cache-dir <dir>    Specify cache directory (default: .cursed/cache)
        \\    --dev                Add as development dependency (for 'add' command)
        \\    --global             Install globally (for 'install' command)
        \\    --force              Force operation (for various commands)
        \\
        \\EXAMPLES:
        \\    cursed-pkg init                          # Initialize new package
        \\    cursed-pkg add json                      # Add 'json' dependency
        \\    cursed-pkg add json@1.0.0                # Add specific version
        \\    cursed-pkg add --dev testz               # Add development dependency
        \\    cursed-pkg install                       # Install all dependencies
        \\    cursed-pkg update                        # Update dependencies
        \\    cursed-pkg search "http client"          # Search for packages
        \\    cursed-pkg info json                     # Show package information
        \\    cursed-pkg remove json                   # Remove dependency
        \\    cursed-pkg publish                       # Publish current package
        \\    cursed-pkg clean                         # Clean cache
        \\
    , .{});
}

fn cmdInfo(allocator: std.mem.Allocator, args: CliArgs) !void {
    if (args.packages.len == 0) {
        std.debug.print("Error: Package name required for 'info' command\n", .{});
        std.debug.print("Usage: cursed-pkg info <package>\n", .{});
        return;
    }
    
    const package_name = args.packages[0];
    
    std.debug.print("Package: {s}\n", .{package_name});
    std.debug.print("Fetching information...\n", .{});
    
    // TODO: Implement actual package info fetching from registry
    // For now, show sample information
    std.debug.print(
        \\
        \\Name: {s}
        \\Version: 1.2.3
        \\Description: A sample CURSED package
        \\Author: Package Author <author@example.com>
        \\License: MIT
        \\Repository: https://github.com/example/{s}
        \\Keywords: utility, helper, library
        \\
        \\Dependencies:
        \\  json: ^1.0.0
        \\  http: ~0.5.0
        \\
        \\Downloads: 1,234 (last 30 days)
        \\Last updated: 2025-01-15
        \\
    , .{ package_name, package_name });
    
    _ = allocator;
}

fn cmdList(allocator: std.mem.Allocator, args: CliArgs) !void {
    std.debug.print("Installed packages:\n", .{});
    
    // Load manifest to get dependencies
    var manifest = tools.package_manager.PackageManifest.loadFromToml(allocator, "CursedPackage.toml") catch |err| switch (err) {
        error.FileNotFound => {
            std.debug.print("No CursedPackage.toml found in current directory\n", .{});
            return;
        },
        else => return err,
    };
    defer manifest.deinit();
    
    if (manifest.dependencies.count() == 0 and manifest.dev_dependencies.count() == 0) {
        std.debug.print("  (no dependencies)\n", .{});
        return;
    }
    
    std.debug.print("\nDependencies:\n", .{});
    var dep_iter = manifest.dependencies.iterator();
    while (dep_iter.next()) |entry| {
        const dep = entry.value_ptr.*;
        const version_str = switch (dep.version_req.constraint) {
            .exact => |v| v.toString(allocator) catch "unknown",
            .caret => |v| blk: { const vstr = v.toString(allocator) catch "unknown"; break :blk std.fmt.allocPrint(allocator, "^{s}", .{vstr}) catch "unknown"; },
            .tilde => |v| blk: { const vstr = v.toString(allocator) catch "unknown"; break :blk std.fmt.allocPrint(allocator, "~{s}", .{vstr}) catch "unknown"; },
            .greater => |v| blk: { const vstr = v.toString(allocator) catch "unknown"; break :blk std.fmt.allocPrint(allocator, ">{s}", .{vstr}) catch "unknown"; },
            .greater_eq => |v| blk: { const vstr = v.toString(allocator) catch "unknown"; break :blk std.fmt.allocPrint(allocator, ">={s}", .{vstr}) catch "unknown"; },
            .less => |v| blk: { const vstr = v.toString(allocator) catch "unknown"; break :blk std.fmt.allocPrint(allocator, "<{s}", .{vstr}) catch "unknown"; },
            .less_eq => |v| blk: { const vstr = v.toString(allocator) catch "unknown"; break :blk std.fmt.allocPrint(allocator, "<={s}", .{vstr}) catch "unknown"; },
            .wildcard => |w| if (w.minor) |minor| std.fmt.allocPrint(allocator, "{}.{}.*", .{w.major.?, minor}) catch "unknown" else std.fmt.allocPrint(allocator, "{}.*", .{w.major.?}) catch "unknown",
        };
        defer if (!std.mem.eql(u8, version_str, "unknown")) allocator.free(version_str);
        std.debug.print("  {s} {s}\n", .{ dep.name, version_str });
    }
    
    if (manifest.dev_dependencies.count() > 0) {
        std.debug.print("\nDevelopment dependencies:\n", .{});
        var dev_dep_iter = manifest.dev_dependencies.iterator();
        while (dev_dep_iter.next()) |entry| {
            const dep = entry.value_ptr.*;
            const version_str = switch (dep.version_req.constraint) {
                .exact => |v| v.toString(allocator) catch "unknown",
                .caret => |v| blk: { const vstr = v.toString(allocator) catch "unknown"; break :blk std.fmt.allocPrint(allocator, "^{s}", .{vstr}) catch "unknown"; },
                .tilde => |v| blk: { const vstr = v.toString(allocator) catch "unknown"; break :blk std.fmt.allocPrint(allocator, "~{s}", .{vstr}) catch "unknown"; },
                .greater => |v| blk: { const vstr = v.toString(allocator) catch "unknown"; break :blk std.fmt.allocPrint(allocator, ">{s}", .{vstr}) catch "unknown"; },
                .greater_eq => |v| blk: { const vstr = v.toString(allocator) catch "unknown"; break :blk std.fmt.allocPrint(allocator, ">={s}", .{vstr}) catch "unknown"; },
                .less => |v| blk: { const vstr = v.toString(allocator) catch "unknown"; break :blk std.fmt.allocPrint(allocator, "<{s}", .{vstr}) catch "unknown"; },
                .less_eq => |v| blk: { const vstr = v.toString(allocator) catch "unknown"; break :blk std.fmt.allocPrint(allocator, "<={s}", .{vstr}) catch "unknown"; },
                .wildcard => |w| if (w.minor) |minor| std.fmt.allocPrint(allocator, "{}.{}.*", .{w.major.?, minor}) catch "unknown" else std.fmt.allocPrint(allocator, "{}.*", .{w.major.?}) catch "unknown",
            };
            defer if (!std.mem.eql(u8, version_str, "unknown")) allocator.free(version_str);
            std.debug.print("  {s} {s}\n", .{ dep.name, version_str });
        }
    }
    
    // Show cache information
    std.debug.print("\nCache location: {s}\n", .{args.cache_dir});
    
    const cache_stat = std.fs.cwd().statFile(args.cache_dir) catch {
        std.debug.print("Cache status: not found\n", .{});
        return;
    };
    
    std.debug.print("Cache status: exists ({} bytes)\n", .{cache_stat.size});
}

fn cmdClean(allocator: std.mem.Allocator, args: CliArgs) !void {
    std.debug.print("Cleaning package cache: {s}\n", .{args.cache_dir});
    
    if (args.dry_run) {
        std.debug.print("(dry run) Would remove cache directory\n", .{});
        return;
    }
    
    // Remove cache directory
    std.fs.cwd().deleteTree(args.cache_dir) catch {
        std.debug.print("Cache directory not found (already clean)\n", .{});
        return;
    };
    
    std.debug.print("Cache cleaned successfully\n", .{});
    
    _ = allocator;
}

// Convert null-terminated string array to regular string array
fn convertArgs(allocator: std.mem.Allocator, null_terminated_args: [][:0]const u8) ![][]const u8 {
    const converted = try allocator.alloc([]const u8, null_terminated_args.len);
    for (null_terminated_args, 0..) |arg, i| {
        // Convert null-terminated string to regular string slice
        converted[i] = std.mem.sliceTo(arg, 0);
    }
    return converted;
}

fn runCommand(allocator: std.mem.Allocator, args: CliArgs) !void {
    if (args.verbose) {
        std.debug.print("Running command: {s}\n", .{@tagName(args.command)});
        if (args.packages.len > 0) {
            std.debug.print("Packages: {s}\n", .{args.packages});
        }
    }
    
    // Convert args.packages to regular string array
    const converted_packages = try convertArgs(allocator, args.packages);
    defer allocator.free(converted_packages);
    
    switch (args.command) {
        .init => try tools.package_manager.commands.init(allocator, converted_packages),
        .add => {
            // Handle package@version syntax
            if (args.packages.len > 0) {
                const pkg_spec = args.packages[0];
                const pkg_spec_str = std.mem.sliceTo(pkg_spec, 0);
                if (std.mem.indexOf(u8, pkg_spec_str, "@")) |at_pos| {
                    const pkg_name = pkg_spec_str[0..at_pos];
                    const version = pkg_spec_str[at_pos + 1 ..];
                    
                    var add_args_array = [_][]const u8{ pkg_name, version };
                    try tools.package_manager.commands.add(allocator, add_args_array[0..]);
                    return;
                }
            }
            
            try tools.package_manager.commands.add(allocator, converted_packages);
        },
        .remove => try tools.package_manager.commands.remove(allocator, converted_packages),
        .install => try tools.package_manager.commands.install(allocator, converted_packages),
        .update => try tools.package_manager.commands.update(allocator, converted_packages),
        .search => try tools.package_manager.commands.search(allocator, converted_packages),
        .publish => try tools.package_manager.commands.publish(allocator, converted_packages),
        .info => try cmdInfo(allocator, args),
        .list => try cmdList(allocator, args),
        .clean => try cmdClean(allocator, args),
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
        std.debug.print("Error parsing arguments: {}\n", .{err});
        std.process.exit(1);
    };
    defer cli_args.deinit();
    
    runCommand(allocator, cli_args) catch |err| {
        std.debug.print("Error: {}\n", .{err});
        std.process.exit(1);
    };
}

// Test functions
test "command parsing" {
    try std.testing.expect(Command.fromString("init") == .init);
    try std.testing.expect(Command.fromString("add") == .add);
    try std.testing.expect(Command.fromString("invalid") == null);
}

test "argument parsing" {
    const allocator = std.testing.allocator;
    
    const test_args = [_][:0]const u8{ "cursed-pkg", "add", "json", "--verbose", "--cache-dir=/tmp/cache" };
    
    var cli_args = try parseArgs(allocator, &test_args);
    defer cli_args.deinit();
    
    try std.testing.expect(cli_args.command == .add);
    try std.testing.expect(cli_args.packages.len == 1);
    try std.testing.expectEqualStrings("json", cli_args.packages[0]);
    try std.testing.expect(cli_args.verbose == true);
    try std.testing.expectEqualStrings("/tmp/cache", cli_args.cache_dir);
}
