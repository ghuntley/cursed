// Simple CURSED Code Formatter
// Basic formatting functionality without complex dependencies

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Simple formatter that works with basic CURSED syntax
pub fn formatCursedCode(allocator: Allocator, source: []const u8) ![]const u8 {
    var formatted = ArrayList(u8).init(allocator);
    defer formatted.deinit(allocator);
    
    var lines = std.mem.splitScalar(u8, source, '\n');
    var indent_level: u32 = 0;
    
    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t");
        
        // Skip empty lines
        if (trimmed.len == 0) {
            try formatted.append('\n');
            continue;
        }
        
        // Decrease indent for closing braces
        if (std.mem.startsWith(u8, trimmed, "}")) {
            if (indent_level > 0) indent_level -= 1;
        }
        
        // Add indentation
        for (0..indent_level * 4) |_| {
            try formatted.append(' ');
        }
        
        // Add the line
        try formatted.appendSlice(trimmed);
        try formatted.append('\n');
        
        // Increase indent for opening braces
        if (std.mem.endsWith(u8, trimmed, "{")) {
            indent_level += 1;
        }
    }
    
    return try formatted.toOwnedSlice();
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        std.log.err("Usage: cursed-fmt <file>", .{});
        return;
    }
    
    const file_path = args[1];
    
    // Read file
    const file = std.fs.cwd().openFile(file_path, .{}) catch |err| {
        std.log.err("Cannot open file {s}: {}", .{ file_path, err });
        return;
    };
    defer file.close();
    
    const source = file.readToEndAlloc(allocator, 1024 * 1024) catch |err| {
        std.log.err("Cannot read file {s}: {}", .{ file_path, err });
        return;
    };
    defer allocator.free(source);
    
    // Format code
    const formatted = formatCursedCode(allocator, source) catch |err| {
        std.log.err("Cannot format file {s}: {}", .{ file_path, err });
        return;
    };
    defer allocator.free(formatted);
    
    // Write back to file
    const output_file = std.fs.cwd().createFile(file_path, .{}) catch |err| {
        std.log.err("Cannot write file {s}: {}", .{ file_path, err });
        return;
    };
    defer output_file.close();
    
    output_file.writeAll(formatted) catch |err| {
        std.log.err("Cannot write formatted content to {s}: {}", .{ file_path, err });
        return;
    };
    
    std.log.info("Formatted: {s}", .{file_path});
}
