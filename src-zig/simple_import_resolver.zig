const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Simple Import Resolution for CURSED Zig Compiler
// Handles "yeet" import statements without complex formatting

pub fn resolveStdlibImport(allocator: Allocator, module_name: []const u8) !bool {
    // Get current working directory
    const cwd = std.fs.cwd();
    var buf: [1024]u8 = undefined;
    const current_dir = try cwd.realpath(".", &buf);
    
    // Build stdlib path manually
    var stdlib_path = std.ArrayList(u8).init(allocator);
    defer stdlib_path.deinit();
    
    try stdlib_path.appendSlice(current_dir);
    try stdlib_path.appendSlice("/stdlib/");
    try stdlib_path.appendSlice(module_name);
    try stdlib_path.appendSlice("/mod.csd");
    
    // Check if file exists
    cwd.access(stdlib_path.items, .{}) catch return false;
    return true;
}

pub fn extractImports(allocator: Allocator, source: []const u8) !ArrayList([]const u8) {
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
    var all_valid = true;
    
    for (imports.items) |module_name| {
        if (resolveStdlibImport(allocator, module_name)) |valid| {
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
