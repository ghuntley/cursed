// Simple CURSED Package Manager
// Basic package management functionality

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Package manifest structure
const PackageManifest = struct {
    name: []const u8,
    version: []const u8,
    description: []const u8,
    main: []const u8,
    dependencies: std.StringHashMap([]const u8),
    
    pub fn init() PackageManifest {
        return PackageManifest{
            .name = "",
            .version = "0.1.0",
            .description = "",
            .main = "src/main.💀",
            .dependencies = std.StringHashMap([]const u8){},
        };
    }
    
    pub fn deinit(self: *PackageManifest) void {
        self.dependencies.deinit(self.allocator);
    }
};

// Package manager commands
pub fn cmdInit(allocator: Allocator, project_name: []const u8) !void {
    // Create package.json
    var manifest = PackageManifest.init(allocator);
    defer manifest.deinit();
    
    const json_content = try std.fmt.allocPrint(allocator,
        \\{{
        \\  "name": "{s}",
        \\  "version": "0.1.0",
        \\  "description": "A CURSED project",
        \\  "main": "src/main.💀",
        \\  "dependencies": {{}}
        \\}}
    , .{project_name});
    defer allocator.free(json_content);
    
    const package_file = try std.fs.cwd().createFile("package.json", .{});
    defer package_file.close();
    try package_file.writer().writeAll(json_content);
    
    // Create basic project structure
    std.fs.cwd().makePath("src") catch {};
    
    const main_file = try std.fs.cwd().createFile("src/main.💀", .{});
    defer main_file.close();
    try main_file.writer().writeAll(
        \\fr fr Main entry point for CURSED project
        \\
        \\slay main() {
        \\    vibez.spill("Hello from CURSED!")
        \\}
        \\
        \\main()
    );
    
    std.log.info("Initialized CURSED project: {s}", .{project_name});
}

pub fn cmdAdd(allocator: Allocator, package_name: []const u8, version: []const u8) !void {
        std.log.info("Adding dependency: {s}@{s}", .{ package_name, version });
    std.log.info("Note: Actual package installation not implemented in demo", .{});
}

pub fn cmdRemove(allocator: Allocator, package_name: []const u8) !void {
        std.log.info("Removing dependency: {s}", .{package_name});
}

pub fn cmdInstall(allocator: Allocator) !void {
        _ = allocator;
        // Create node_modules directory
    std.fs.cwd().makePath("node_modules") catch {};
    std.log.info("Installing dependencies...", .{});
    std.log.info("Dependencies installed (demo mode)", .{});
}

pub fn cmdSearch(allocator: Allocator, query: []const u8) !void {
        std.log.info("Searching for packages matching: {s}", .{query});
    
    // Mock search results
    const mock_results = [_]struct { name: []const u8, version: []const u8, description: []const u8 }{
        .{ .name = "http_client", .version = "1.0.0", .description = "HTTP client library for CURSED" },
        .{ .name = "json_parser", .version = "2.1.0", .description = "JSON parsing utilities" },
        .{ .name = "crypto_utils", .version = "1.5.2", .description = "Cryptographic utilities" },
    };
    
    for (mock_results) |result| {
        if (std.mem.indexOf(u8, result.name, query) != null or 
            std.mem.indexOf(u8, result.description, query) != null) {
            std.log.info("  {s}@{s} - {s}", .{ result.name, result.version, result.description });
        }
    }
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        std.log.err("Usage: cursed-pkg <command> [args...]", .{});
        std.log.err("Commands: init, add, remove, install, search", .{});
        return;
    }
    
    const command = args[1];
    
    if (std.mem.eql(u8, command, "init")) {
        const project_name = if (args.len > 2) args[2] else "my-cursed-project";
        try cmdInit(allocator, project_name);
    } else if (std.mem.eql(u8, command, "add")) {
        if (args.len < 3) {
            std.log.err("Usage: cursed-pkg add <package-name> [version]", .{});
            return;
        }
        const package_name = args[2];
        const version = if (args.len > 3) args[3] else "latest";
        try cmdAdd(allocator, package_name, version);
    } else if (std.mem.eql(u8, command, "remove")) {
        if (args.len < 3) {
            std.log.err("Usage: cursed-pkg remove <package-name>", .{});
            return;
        }
        const package_name = args[2];
        try cmdRemove(allocator, package_name);
    } else if (std.mem.eql(u8, command, "install")) {
        try cmdInstall(allocator);
    } else if (std.mem.eql(u8, command, "search")) {
        if (args.len < 3) {
            std.log.err("Usage: cursed-pkg search <query>", .{});
            return;
        }
        const query = args[2];
        try cmdSearch(allocator, query);
    } else {
        std.log.err("Unknown command: {s}", .{command});
    }
}
