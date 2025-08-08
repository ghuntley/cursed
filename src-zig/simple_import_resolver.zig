const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Import the advanced resolver
const AdvancedImportResolver = @import("advanced_import_resolver.zig").AdvancedImportResolver;

// Legacy compatibility wrapper for the advanced import resolver
// Maintains backward compatibility while providing enhanced functionality

pub fn resolveStdlibImport(allocator: Allocator, module_name: []const u8) !bool {
    return resolveStdlibImportWithPath(allocator, module_name, null);
}

pub fn resolveStdlibImportWithPath(allocator: Allocator, module_name: []const u8, stdlib_path_override: ?[]const u8) !bool {
    // Use advanced resolver for better module resolution
    var resolver = AdvancedImportResolver.initWithDefaults(allocator) catch {
        // Fallback to legacy behavior if advanced resolver fails
        return resolveStdlibImportLegacy(allocator, module_name, stdlib_path_override);
    };
    defer resolver.deinit();
    
    // Add custom stdlib path if provided
    if (stdlib_path_override) |custom_path| {
        try resolver.addLocalPath(custom_path);
    }
    
    // Try to resolve the module
    const import_spec = resolver.resolveImport(module_name, "unknown") catch {
        return false;
    };
    
    // Note: import_spec is a safe copy that doesn't own memory, so no need to deinit
    
    // Check if it resolved to a stdlib module
    return import_spec.module_type == .stdlib or import_spec.resolved_path != null;
}

fn resolveStdlibImportLegacy(allocator: Allocator, module_name: []const u8, stdlib_path_override: ?[]const u8) !bool {
    const cwd = std.fs.cwd();
    var buf: [1024]u8 = undefined;
    
    var stdlib_path = std.ArrayList(u8).init(allocator);
    defer stdlib_path.deinit();
    
    if (stdlib_path_override) |custom_path| {
        // Use provided stdlib path
        try stdlib_path.appendSlice(custom_path);
    } else {
        // Find project root by looking for build.zig or other marker files
        const project_root = findProjectRoot(allocator) catch blk: {
            // Fallback to current directory
            const current_dir = try cwd.realpath(".", &buf);
            break :blk try allocator.dupe(u8, current_dir);
        };
        defer allocator.free(project_root);
        
        try stdlib_path.appendSlice(project_root);
        try stdlib_path.appendSlice("/stdlib");
    }
    
    try stdlib_path.append('/');
    try stdlib_path.appendSlice(module_name);
    try stdlib_path.appendSlice("/mod.csd");
    
    // Check if file exists
    cwd.access(stdlib_path.items, .{}) catch return false;
    return true;
}

fn findProjectRoot(allocator: Allocator) ![]const u8 {
    const cwd = std.fs.cwd();
    var buf: [1024]u8 = undefined;
    const current_path = try cwd.realpath(".", &buf);
    
    // Look for marker files that indicate project root
    const markers = [_][]const u8{
        "build.zig",
        "Cargo.toml", 
        "CursedPackage.toml",
        "AGENT.md",
        ".git"
    };
    
    var path_components = std.ArrayList([]const u8).init(allocator);
    defer path_components.deinit();
    
    // Split path into components
    var iter = std.mem.splitScalar(u8, current_path, '/');
    while (iter.next()) |component| {
        if (component.len > 0) {
            try path_components.append(component);
        }
    }
    
    // Walk up the directory tree
    while (path_components.items.len > 0) {
        // Build current test path
        var test_path = std.ArrayList(u8).init(allocator);
        defer test_path.deinit();
        
        for (path_components.items) |component| {
            try test_path.append('/');
            try test_path.appendSlice(component);
        }
        
        // Check for marker files
        for (markers) |marker| {
            var marker_path = std.ArrayList(u8).init(allocator);
            defer marker_path.deinit();
            
            try marker_path.appendSlice(test_path.items);
            try marker_path.append('/');
            try marker_path.appendSlice(marker);
            
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
    // Use advanced resolver for better import extraction
    var resolver = AdvancedImportResolver.initWithDefaults(allocator) catch {
        // Fallback to legacy extraction
        return extractImportsLegacy(allocator, source);
    };
    defer resolver.deinit();
    
    const import_specs = resolver.extractImports(source) catch {
        return extractImportsLegacy(allocator, source);
    };
    defer {
        for (import_specs.items) |*spec| {
            const import_spec = spec;
            // Only free raw_path and source_file since these are allocated in parseImportStatement
            allocator.free(import_spec.raw_path);
            allocator.free(import_spec.source_file);
            if (import_spec.alias) |alias| {
                allocator.free(alias);
            }
        }
        import_specs.deinit();
    }
    
    // Convert ImportSpec array to string array for compatibility
    var imports = ArrayList([]const u8).init(allocator);
    for (import_specs.items) |spec| {
        try imports.append(try allocator.dupe(u8, spec.raw_path));
    }
    
    return imports;
}

fn extractImportsLegacy(allocator: Allocator, source: []const u8) !ArrayList([]const u8) {
    var imports = ArrayList([]const u8).init(allocator);
    
    var lines = std.mem.splitScalar(u8, source, '\n');
    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t\r");
        
        // Look for "yeet" statements
        if (std.mem.startsWith(u8, trimmed, "yeet ")) {
            const import_part = trimmed[5..]; // Skip "yeet "
            
            // Extract module name from quotes
            if (std.mem.indexOf(u8, import_part, "\"")) |start_quote| {
                const after_start = import_part[start_quote + 1..];
                if (std.mem.indexOf(u8, after_start, "\"")) |end_quote| {
                    const module_name = after_start[0..end_quote];
                    try imports.append(try allocator.dupe(u8, module_name));
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
    // Use advanced resolver for comprehensive validation
    var resolver = AdvancedImportResolver.initWithDefaults(allocator) catch {
        // Fallback to legacy validation
        return validateImportsLegacy(allocator, imports, stdlib_path_override);
    };
    defer resolver.deinit();
    
    // Add custom stdlib path if provided
    if (stdlib_path_override) |custom_path| {
        try resolver.addLocalPath(custom_path);
    }
    
    var all_valid = true;
    
    for (imports.items) |module_name| {
        const import_spec = resolver.resolveImport(module_name, "validation") catch |err| {
            print("❌ Error resolving module '{s}': {any}\n", .{module_name, err});
            all_valid = false;
            continue;
        };
        
        // Note: import_spec is a safe copy that doesn't own memory, so no need to deinit
        
        if (import_spec.resolved_path) |path| {
            print("✅ Module '{s}' found at: {s} (type: {s})\n", .{ 
                module_name, 
                path, 
                @tagName(import_spec.module_type) 
            });
        } else {
            print("❌ Module '{s}' could not be resolved\n", .{module_name});
            all_valid = false;
        }
    }
    
    // Generate dependency report if verbose
    if (imports.items.len > 0) {
        resolver.generateDependencyReport() catch {};
    }
    
    return all_valid;
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
            print("❌ Error resolving module '{s}': {any}\n", .{module_name, err});
            all_valid = false;
        }
    }
    
    return all_valid;
}
