//! Simple CURSED Code Formatter
//! Provides basic code formatting for CURSED syntax

const std = @import("std");
const print = std.debug.print;

pub fn main() !void {
    // Parse command line arguments
    const args = try std.process.argsAlloc(std.heap.page_allocator);
    defer std.process.argsFree(std.heap.page_allocator, args);

    if (args.len < 2) {
        printUsage();
        return;
    }

    const filename = args[1];
    print("🎨 Formatting CURSED file: {s}\n", .{filename});

    // Read file
    const file = std.fs.cwd().openFile(filename, .{}) catch |err| {
        print("❌ Error opening file '{s}': {s}\n", .{ filename, err });
        return;
    };
    defer file.close();

    // Read content
    const content = file.readToEndAlloc(std.heap.page_allocator, 1024 * 1024) catch |err| {
        print("❌ Error reading file: {s}\n", .{err});
        return;
    };
    defer std.heap.page_allocator.free(content);

    print("✅ File read successfully ({s} bytes)\n", .{content.len});

    // Simple formatting (placeholder - just outputs formatted version)
    const formatted = try formatCursedCode(content);
    defer std.heap.page_allocator.free(formatted);

    // Write back to file
    const output_file = std.fs.cwd().createFile(filename, .{}) catch |err| {
        print("❌ Error creating output file: {s}\n", .{err});
        return;
    };
    defer output_file.close();

    output_file.writer().writeAll(formatted) catch |err| {
        print("❌ Error writing formatted code: {s}\n", .{err});
        return;
    };

    print("✨ Successfully formatted {s}\n", .{filename});
}

fn formatCursedCode(code: []const u8) ![]u8 {
    // Simple formatter: normalize whitespace and indentation
    const allocator = std.heap.page_allocator;
    var result = std.ArrayList(u8){};
    try result.ensureTotalCapacity(allocator, code.len * 2);
    
    var lines = std.mem.splitScalar(u8, code, '\n');
    var indent_level: u32 = 0;
    
    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t\r");
        
        // Adjust indentation for braces
        if (std.mem.endsWith(u8, trimmed, "{")) {
            try addIndentedLine(&result, allocator, trimmed, indent_level);
            indent_level += 1;
        } else if (std.mem.startsWith(u8, trimmed, "}")) {
            if (indent_level > 0) indent_level -= 1;
            try addIndentedLine(&result, allocator, trimmed, indent_level);
        } else if (trimmed.len > 0) {
            try addIndentedLine(&result, allocator, trimmed, indent_level);
        } else {
            try result.append(allocator, '\n');
        }
    }
    
    return result.toOwnedSlice(allocator);
}

fn addIndentedLine(result: *std.ArrayList(u8), allocator: std.mem.Allocator, line: []const u8, indent_level: u32) !void {
    // Add 4 spaces per indent level
    var i: u32 = 0;
    while (i < indent_level * 4) : (i += 1) {
        try result.append(allocator, ' ');
    }
    try result.appendSlice(allocator, line);
    try result.append(allocator, '\n');
}

fn printUsage() void {
    print("CURSED Code Formatter v1.0.0\n\n", .{});
    print("USAGE:\n", .{});
    print("    cursed-fmt <file.csd>\n\n", .{});
    print("FEATURES:\n", .{});
    print("    • Consistent indentation (4 spaces)\n", .{});
    print("    • Normalized whitespace\n", .{});
    print("    • Proper brace formatting\n\n", .{});
    print("EXAMPLES:\n", .{});
    print("    cursed-fmt hello.csd\n", .{});
    print("    cursed-fmt src/main.csd\n", .{});
}
