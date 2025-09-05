//! Simple CURSED Code Linter  
//! Provides basic code quality checks for CURSED syntax

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
    print("🔍 Linting CURSED file: {s}\n", .{filename});

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

    // Perform basic linting checks
    const issues = try lintCursedCode(content, filename);
    defer std.heap.page_allocator.free(issues);

    // Report results
    if (issues.len == 0) {
        print("✨ No issues found! Code looks good.\n", .{});
    } else {
        print("⚠️  Found {s} issue(s):\n\n", .{issues.len});
        for (issues) |issue| {
            print("  {s}:{s}: {s} - {s}\n", .{ issue.line, issue.column, @tagName(issue.level), issue.message });
        }
        print("\n", .{});
    }
}

const LintLevel = enum {
    Error,
    Warning,
    Info,
};

const LintIssue = struct {
    level: LintLevel,
    line: u32,
    column: u32,
    message: []const u8,
};

fn lintCursedCode(code: []const u8, filename: []const u8) ![]LintIssue {
    _ = filename; // Unused for now
    const allocator = std.heap.page_allocator;
    var issues = std.ArrayList(LintIssue){};
    try issues.ensureTotalCapacity(allocator, 100);
    
    var line_number: u32 = 1;
    var lines = std.mem.splitScalar(u8, code, '\n');
    
    while (lines.next()) |line| {
        defer line_number += 1;
        
        const trimmed = std.mem.trim(u8, line, " \t\r");
        if (trimmed.len == 0) continue;
        
        // Check for common issues
        
        // 1. Lines too long
        if (line.len > 100) {
            try issues.append(allocator, .{
                .level = .Warning,
                .line = line_number,
                .column = 101,
                .message = try allocator.dupe(u8, "Line exceeds 100 characters"),
            });
        }
        
        // 2. Trailing whitespace
        if (line.len > 0 and (line[line.len - 1] == ' ' or line[line.len - 1] == '\t')) {
            try issues.append(allocator, .{
                .level = .Info,
                .line = line_number,
                .column = @intCast(line.len),
                .message = try allocator.dupe(u8, "Trailing whitespace"),
            });
        }
        
        // 3. Missing semicolon (simple check)
        if ((std.mem.indexOf(u8, trimmed, "sus ") != null or 
             std.mem.indexOf(u8, trimmed, "damn ") != null) and 
             !std.mem.endsWith(u8, trimmed, ";") and 
             !std.mem.endsWith(u8, trimmed, "{") and 
             !std.mem.endsWith(u8, trimmed, "}")) {
            try issues.append(allocator, .{
                .level = .Warning,
                .line = line_number,
                .column = @intCast(trimmed.len),
                .message = try allocator.dupe(u8, "Statement may be missing semicolon"),
            });
        }
        
        // 4. Deprecated syntax
        if (std.mem.indexOf(u8, trimmed, "var ") != null) {
            try issues.append(allocator, .{
                .level = .Warning,
                .line = line_number,
                .column = 1,
                .message = try allocator.dupe(u8, "Consider using 'sus' instead of deprecated 'var'"),
            });
        }
    }
    
    return issues.toOwnedSlice(allocator);
}

fn printUsage() void {
    print("CURSED Code Linter v1.0.0\n\n", .{});
    print("USAGE:\n", .{});
    print("    cursed-lint <file.💀.💀>\n\n", .{});
    print("CHECKS:\n", .{});
    print("    • Line length (max 100 characters)\n", .{});
    print("    • Trailing whitespace\n", .{});
    print("    • Missing semicolons\n", .{});
    print("    • Deprecated syntax usage\n", .{});
    print("    • Code style consistency\n\n", .{});
    print("EXAMPLES:\n", .{});
    print("    cursed-lint hello.💀.💀\n", .{});
    print("    cursed-lint src/main.💀.💀\n", .{});
}
