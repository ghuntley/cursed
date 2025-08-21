// Simple CURSED Code Linter
// Basic linting functionality without complex dependencies

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Lint Issue
const LintIssue = struct {
    line: u32,
    column: u32,
    message: []const u8,
    severity: []const u8,
};

// Simple linter for CURSED code
pub fn lintCursedCode(allocator: Allocator, source: []const u8, _: []const u8) ![]LintIssue {
    var issues = ArrayList(LintIssue).init(allocator);
    defer issues.deinit(allocator);
    
    var lines = std.mem.splitScalar(u8, source, '\n');
    var line_number: u32 = 0;
    
    while (lines.next()) |line| {
        line_number += 1;
        
        // Check line length
        if (line.len > 100) {
            try issues.append(LintIssue{
                .line = line_number,
                .column = 101,
                .message = "Line too long (> 100 characters)",
                .severity = "warning",
            });
        }
        
        // Check for trailing whitespace
        if (line.len > 0 and (line[line.len - 1] == ' ' or line[line.len - 1] == '\t')) {
            try issues.append(LintIssue{
                .line = line_number,
                .column = @intCast(line.len),
                .message = "Trailing whitespace",
                .severity = "info",
            });
        }
        
        // Check for deprecated keywords
        if (std.mem.indexOf(u8, line, "function") != null) {
            try issues.append(LintIssue{
                .line = line_number,
                .column = 1,
                .message = "Use 'slay' instead of 'function'",
                .severity = "warning",
            });
        }
        
        if (std.mem.indexOf(u8, line, "var ") != null) {
            try issues.append(LintIssue{
                .line = line_number,
                .column = 1,
                .message = "Use 'sus' instead of 'var'",
                .severity = "warning",
            });
        }
        
        if (std.mem.indexOf(u8, line, "return") != null) {
            try issues.append(LintIssue{
                .line = line_number,
                .column = 1,
                .message = "Use 'damn' instead of 'return'",
                .severity = "warning",
            });
        }
        
        // Check for inconsistent indentation
        var spaces: u32 = 0;
        var tabs: u32 = 0;
        for (line) |char| {
            if (char == ' ') {
                spaces += 1;
            } else if (char == '\t') {
                tabs += 1;
            } else {
                break;
            }
        }
        
        if (spaces > 0 and tabs > 0) {
            try issues.append(LintIssue{
                .line = line_number,
                .column = 1,
                .message = "Mixed spaces and tabs for indentation",
                .severity = "warning",
            });
        }
    }
    
    return try issues.toOwnedSlice();
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        std.log.err("Usage: cursed-lint <file>", .{});
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
    
    // Lint code
    const issues = lintCursedCode(allocator, source, file_path) catch |err| {
        std.log.err("Cannot lint file {s}: {}", .{ file_path, err });
        return;
    };
    defer allocator.free(issues);
    
    // Output results
    var stdout_buffer: [4096]u8 = undefined;
    const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
    
    if (issues.len == 0) {
        try stdout.print("✅ No issues found in {s}\n", .{file_path});
    } else {
        for (issues) |issue| {
            try stdout.print("{s}:{}:{}: {s}: {s}\n", .{
                file_path,
                issue.line,
                issue.column,
                issue.severity,
                issue.message,
            });
        }
        try stdout.print("\nFound {} issues in {s}\n", .{ issues.len, file_path });
    }
}
