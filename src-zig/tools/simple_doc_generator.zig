// Simple CURSED Documentation Generator
// Basic documentation generation functionality

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Documentation item
const DocItem = struct {
    name: []const u8,
    type: []const u8,
    description: []const u8,
    signature: []const u8,
    file: []const u8,
    line: u32,
};

// Simple documentation extractor
pub fn extractDocumentation(allocator: Allocator, source: []const u8, file_path: []const u8) ![]DocItem {
    var items = ArrayList(DocItem).init(allocator);
    defer items.deinit(allocator);
    
    var lines = std.mem.splitScalar(u8, source, '\n');
    var line_number: u32 = 0;
    var current_comment: ?[]const u8 = null;
    
    while (lines.next()) |line| {
        line_number += 1;
        const trimmed = std.mem.trim(u8, line, " \t");
        
        // Check for documentation comments
        if (std.mem.startsWith(u8, trimmed, "fr fr")) {
            current_comment = trimmed[5..];
        } else if (std.mem.startsWith(u8, trimmed, "#")) {
            current_comment = trimmed[1..];
        }
        // Check for function definitions
        else if (std.mem.startsWith(u8, trimmed, "slay ")) {
            const func_start = 5; // Length of "slay "
            if (func_start < trimmed.len) {
                const rest = trimmed[func_start..];
                const paren_pos = std.mem.indexOf(u8, rest, "(") orelse rest.len;
                const func_name = std.mem.trim(u8, rest[0..paren_pos], " \t");
                
                try items.append(DocItem{
                    .name = try allocator.dupe(u8, func_name),
                    .type = "function",
                    .description = if (current_comment) |comment| try allocator.dupe(u8, std.mem.trim(u8, comment, " \t")) else "No description",
                    .signature = try allocator.dupe(u8, trimmed),
                    .file = try allocator.dupe(u8, file_path),
                    .line = line_number,
                });
            }
            current_comment = null;
        }
        // Check for struct definitions
        else if (std.mem.startsWith(u8, trimmed, "squad ")) {
            const struct_start = 6; // Length of "squad "
            if (struct_start < trimmed.len) {
                const rest = trimmed[struct_start..];
                const brace_pos = std.mem.indexOf(u8, rest, "{") orelse rest.len;
                const struct_name = std.mem.trim(u8, rest[0..brace_pos], " \t");
                
                try items.append(DocItem{
                    .name = try allocator.dupe(u8, struct_name),
                    .type = "struct",
                    .description = if (current_comment) |comment| try allocator.dupe(u8, std.mem.trim(u8, comment, " \t")) else "No description",
                    .signature = try allocator.dupe(u8, trimmed),
                    .file = try allocator.dupe(u8, file_path),
                    .line = line_number,
                });
            }
            current_comment = null;
        }
        // Check for interface definitions
        else if (std.mem.startsWith(u8, trimmed, "collab ")) {
            const interface_start = 7; // Length of "collab "
            if (interface_start < trimmed.len) {
                const rest = trimmed[interface_start..];
                const brace_pos = std.mem.indexOf(u8, rest, "{") orelse rest.len;
                const interface_name = std.mem.trim(u8, rest[0..brace_pos], " \t");
                
                try items.append(DocItem{
                    .name = try allocator.dupe(u8, interface_name),
                    .type = "interface",
                    .description = if (current_comment) |comment| try allocator.dupe(u8, std.mem.trim(u8, comment, " \t")) else "No description",
                    .signature = try allocator.dupe(u8, trimmed),
                    .file = try allocator.dupe(u8, file_path),
                    .line = line_number,
                });
            }
            current_comment = null;
        } else if (trimmed.len == 0) {
            // Keep comment for next item
        } else {
            current_comment = null;
        }
    }
    
    return try items.toOwnedSlice();
}

// Generate HTML documentation
pub fn generateHTML(allocator: Allocator, items: []DocItem, output_dir: []const u8) !void {
    // Create output directory
    std.fs.cwd().makePath(output_dir) catch {};
    
    // Generate index.html
    const index_path = try std.fs.path.join(allocator, &[_][]const u8{ output_dir, "index.html" });
    defer allocator.free(index_path);
    
    const index_file = try std.fs.cwd().createFile(index_path, .{});
    defer index_file.close();
    
    const writer = index_file.writer();
    
    try writer.writeAll(
        \\<!DOCTYPE html>
        \\<html>
        \\<head>
        \\    <title>CURSED Documentation</title>
        \\    <style>
        \\        body { font-family: Arial, sans-serif; margin: 40px; }
        \\        .item { border: 1px solid #ddd; margin: 20px 0; padding: 20px; border-radius: 5px; }
        \\        .signature { background: #f5f5f5; padding: 10px; border-radius: 3px; font-family: monospace; }
        \\        .type { color: #666; font-size: 0.9em; }
        \\        .location { color: #999; font-size: 0.8em; }
        \\    </style>
        \\</head>
        \\<body>
        \\    <h1>CURSED Documentation</h1>
        \\
    );
    
    // Group items by type
    var functions = ArrayList(DocItem).init(allocator);
    var structs = ArrayList(DocItem).init(allocator);
    var interfaces = ArrayList(DocItem).init(allocator);
    defer functions.deinit(allocator);
    defer structs.deinit(allocator);
    defer interfaces.deinit(allocator);
    
    for (items) |item| {
        if (std.mem.eql(u8, item.type, "function")) {
            try functions.append(item);
        } else if (std.mem.eql(u8, item.type, "struct")) {
            try structs.append(item);
        } else if (std.mem.eql(u8, item.type, "interface")) {
            try interfaces.append(item);
        }
    }
    
    // Write functions section
    if (functions.items.len > 0) {
        try writer.writeAll("    <h2>Functions</h2>\n");
        for (functions.items) |item| {
            try writer.print(
                \\    <div class="item">
                \\        <h3>{s} <span class="type">({s})</span></h3>
                \\        <div class="signature">{s}</div>
                \\        <p>{s}</p>
                \\        <div class="location">{s}:{}</div>
                \\    </div>
                \\
            , .{ item.name, item.type, item.signature, item.description, item.file, item.line });
        }
    }
    
    // Write structs section
    if (structs.items.len > 0) {
        try writer.writeAll("    <h2>Structs</h2>\n");
        for (structs.items) |item| {
            try writer.print(
                \\    <div class="item">
                \\        <h3>{s} <span class="type">({s})</span></h3>
                \\        <div class="signature">{s}</div>
                \\        <p>{s}</p>
                \\        <div class="location">{s}:{}</div>
                \\    </div>
                \\
            , .{ item.name, item.type, item.signature, item.description, item.file, item.line });
        }
    }
    
    // Write interfaces section
    if (interfaces.items.len > 0) {
        try writer.writeAll("    <h2>Interfaces</h2>\n");
        for (interfaces.items) |item| {
            try writer.print(
                \\    <div class="item">
                \\        <h3>{s} <span class="type">({s})</span></h3>
                \\        <div class="signature">{s}</div>
                \\        <p>{s}</p>
                \\        <div class="location">{s}:{}</div>
                \\    </div>
                \\
            , .{ item.name, item.type, item.signature, item.description, item.file, item.line });
        }
    }
    
    try writer.writeAll(
        \\</body>
        \\</html>
    );
    
    std.log.info("Generated documentation in {s}/index.html", .{output_dir});
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        std.log.err("Usage: cursed-doc <source-directory> [--output <output-dir>]", .{});
        return;
    }
    
    const source_dir = args[1];
    var output_dir: []const u8 = "docs";
    
    // Parse output directory option
    var i: usize = 2;
    while (i < args.len) {
        if (std.mem.eql(u8, args[i], "--output") and i + 1 < args.len) {
            output_dir = args[i + 1];
            i += 2;
        } else {
            i += 1;
        }
    }
    
    var all_items = ArrayList(DocItem).init(allocator);
    defer all_items.deinit(allocator);
    
    // Process files in directory
    var dir = std.fs.cwd().openDir(source_dir, .{ .iterate = true }) catch |err| {
        std.log.err("Cannot open directory {s}: {}", .{ source_dir, err });
        return;
    };
    defer dir.close();
    
    var iterator = dir.iterate();
    while (try iterator.next()) |entry| {
        if (entry.kind == .file and std.mem.endsWith(u8, entry.name, ".csd")) {
            const file_path = try std.fs.path.join(allocator, &[_][]const u8{ source_dir, entry.name });
            defer allocator.free(file_path);
            
            const file = std.fs.cwd().openFile(file_path, .{}) catch continue;
            defer file.close();
            
            const source = file.readToEndAlloc(allocator, 1024 * 1024) catch continue;
            defer allocator.free(source);
            
            const items = extractDocumentation(allocator, source, file_path) catch continue;
            defer allocator.free(items);
            
            for (items) |item| {
                try all_items.append(item);
            }
        }
    }
    
    // Generate documentation
    try generateHTML(allocator, all_items.items, output_dir);
    
    std.log.info("Processed {} documentation items", .{all_items.items.len});
}
