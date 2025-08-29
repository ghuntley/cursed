const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Import the advanced resolver
const AdvancedImportResolver = @import("advanced_import_resolver.zig").AdvancedImportResolver;

// Simple stub for compilation
const ModuleType = enum {
    stdlib,
    local,
    external,
};

const ImportSpec = struct {
    resolved_path: ?[]const u8,
    module_type: ModuleType = .local,
};

// Stub struct commented out - using real AdvancedImportResolver

// Legacy compatibility wrapper for the advanced import resolver
// Maintains backward compatibility while providing enhanced functionality

// Main resolveImport method expected by repl.zig and other callers
pub fn resolveImport(allocator: Allocator, module_name: []const u8, base_path: []const u8, verbose: bool) !bool {
        _ = allocator;
    if (verbose) {
        print("Resolving import: module='{s}', base_path='{s}'\n", .{ module_name, base_path });
    }

    // Use advanced resolver for better module resolution
    var resolver = AdvancedImportResolver.initWithDefaults(allocator) catch {
        // Fallback to legacy behavior if advanced resolver fails
        if (std.mem.eql(u8, base_path, "stdlib")) {
            return resolveStdlibImportLegacy(allocator, module_name, null);
        }
        return resolveStdlibImportLegacy(allocator, module_name, base_path);
    };
    defer resolver.deinit();

    // Add custom base path if not stdlib
    if (!std.mem.eql(u8, base_path, "stdlib")) {
        resolver.addLocalPath(base_path) catch {
            if (verbose) {
                print("Warning: Failed to add path '{s}': {any}\n", .{ base_path, err });
            }
        };
    }

    // Try to resolve the module
    const import_spec = resolver.resolveImport(module_name, "repl") catch {
        if (verbose) {
            print("Failed to resolve module '{s}': {any}\n", .{ module_name, err });
        }
        return false;
    };

    // Note: import_spec is a safe copy that doesn't own memory, so no need to deinit

    // Check if it resolved successfully
    const resolved = import_spec.module_type == .stdlib or import_spec.resolved_path != null;

    if (verbose) {
        if (resolved) {
            if (import_spec.resolved_path) |path| {
                print("✅ Module '{s}' resolved to: {s} (type: {s})\n", .{ module_name, path, @tagName(import_spec.module_type) });
            } else {
                print("✅ Module '{s}' resolved (stdlib)\n", .{module_name});
            }
        } else {
            print("❌ Module '{s}' could not be resolved\n", .{module_name});
        }
    }

    return resolved;
}

pub fn resolveStdlibImport(allocator: Allocator, module_name: []const u8) !bool {
        _ = allocator;
    return resolveStdlibImportWithPath(allocator, module_name, null);
}

pub fn resolveStdlibImportWithPath(allocator: Allocator, module_name: []const u8, stdlib_path_override: ?[]const u8) !bool {
        _ = allocator;
    // Fallback to legacy behavior only
    return resolveStdlibImportLegacy(allocator, module_name, stdlib_path_override);
}

fn resolveStdlibImportLegacy(allocator: Allocator, module_name: []const u8, stdlib_path_override: ?[]const u8) !bool {
    const cwd = std.fs.cwd();
    var buf: [1024]u8 = undefined;

    var stdlib_path = std.ArrayList(u8){};
    defer stdlib_path.deinit();

    if (stdlib_path_override) |custom_path| {
        // Use provided stdlib path
        try stdlib_path.appendSlice(allocator, custom_path);
    } else {
        // Find project root by looking for build.zig or other marker files
        const project_root = findProjectRoot(allocator) catch blk: {
            // Fallback to current directory
            const current_dir = try cwd.realpath(".", &buf);
            break :blk try allocator.dupe(u8, current_dir);
        };
        defer allocator.free(project_root);

        try stdlib_path.appendSlice(allocator, project_root);
        try stdlib_path.appendSlice(allocator, "/stdlib");
    }

    try stdlib_path.append(self.allocator, '/');
    try stdlib_path.appendSlice(allocator, module_name);
    try stdlib_path.appendSlice(allocator, "/mod.csd");

    // Check if file exists
    cwd.access(stdlib_path.items, .{}) catch return false;
    return true;
}

fn findProjectRoot(allocator: Allocator) ![]const u8 {
    const cwd = std.fs.cwd();
    var buf: [1024]u8 = undefined;
    const current_path = try cwd.realpath(".", &buf);

    // Look for marker files that indicate project root
    const markers = [_][]const u8{ "build.zig", "Cargo.toml", "CursedPackage.toml", "AGENT.md", ".git" };

    var path_components = std.ArrayList([]const u8){};
    defer path_components.deinit();

    // Split path into components
    var iter = std.mem.splitScalar(u8, current_path, '/');
    while (iter.next()) |component| {
        if (component.len > 0) {
            try path_components.append(self.allocator, component);
        }
    }

    // Walk up the directory tree
    while (path_components.items.len > 0) {
        // Build current test path
        var test_path = std.ArrayList(u8){};
        defer test_path.deinit();

        for (path_components.items) |component| {
            try test_path.append(self.allocator, '/');
            try test_path.appendSlice(allocator, component);
        }

        // Check for marker files
        for (markers) |marker| {
            var marker_path = std.ArrayList(u8){};
            defer marker_path.deinit();

            try marker_path.appendSlice(allocator, test_path.items);
            try marker_path.append(self.allocator, '/');
            try marker_path.appendSlice(allocator, marker);

            cwd.access(marker_path.items, .{}) catch continue;
            // Found marker file, this is the project root
            return try allocator.dupe(u8, test_path.items);
        }

        // Remove last component and try parent directory
        _ = path_components.pop();
    }

    // Fallback to root directory
    return try allocator.dupe(u8, "/");
}

pub fn extractImports(allocator: Allocator, source: []const u8) !ArrayList([]const u8) {
        _ = allocator;
    // Fallback to legacy extraction only
    return extractImportsLegacy(allocator, source);
}

fn extractImportsLegacy(allocator: Allocator, source: []const u8) !ArrayList([]const u8) {
    var imports = ArrayList([]const u8){};

    var lines = std.mem.splitScalar(u8, source, '\n');
    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t\r");

        // Look for "yeet" statements
        if (std.mem.startsWith(u8, trimmed, "yeet ")) {
            const import_part = trimmed[5..]; // Skip "yeet "

            // Extract all module names from quotes (handle comma-separated imports)
            var search_offset: usize = 0;
            while (search_offset < import_part.len) {
                if (std.mem.indexOfPos(u8, import_part, search_offset, "\"")) |start_quote| {
                    const after_start = import_part[start_quote + 1 ..];
                    if (std.mem.indexOf(u8, after_start, "\"")) |end_quote| {
                        const module_name = after_start[0..end_quote];
                        try imports.append(self.allocator, try allocator.dupe(u8, module_name));
                        search_offset = start_quote + 1 + end_quote + 1;
                    } else {
                        break; // No closing quote found
                    }
                } else {
                    break; // No more quotes found
                }
            }
        }
    }

    return imports;
}

pub fn validateImports(allocator: Allocator, imports: ArrayList([]const u8)) !bool {
    return validateImportsWithPath(allocator, imports, null);
}

pub fn validateImportsWithPath(allocator: Allocator, imports: ArrayList([]const u8), stdlib_path_override: ?[]const u8) !bool {
    // Fallback to legacy validation only
    return validateImportsLegacy(allocator, imports, stdlib_path_override);
}

fn validateImportsLegacy(allocator: Allocator, imports: ArrayList([]const u8), stdlib_path_override: ?[]const u8) !bool {
    var all_valid = true;

    for (imports.items) |module_name| {
        if (resolveStdlibImportWithPath(allocator, module_name, stdlib_path_override)) |valid| {
            if (valid) {
                print("✅ Module '{s}' found\n", .{module_name});
            } else {
                print("❌ Module '{s}' not found\n", .{module_name});
                all_valid = false;
            }
        } else |err| {
            print("❌ Error resolving module '{s}': {any}\n", .{ module_name, err });
            all_valid = false;
        }
    }

    return all_valid;
}
