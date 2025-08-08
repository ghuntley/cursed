// Simple test for the CURSED security linter

const std = @import("std");
const linter = @import("src-zig/tools/linter.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Initialize linter configuration
    var config = linter.LinterConfig.init(allocator);
    defer config.deinit();
    
    // Create linter instance
    var cursed_linter = linter.Linter.init(allocator, config);
    defer cursed_linter.deinit();
    
    // Test file path
    const test_file = "simple_security_test.csd";
    
    std.log.info("Testing CURSED security linter on: {s}", .{test_file});
    
    // Run linter on test file
    cursed_linter.lintFile(test_file) catch |err| {
        std.log.err("Failed to lint file: {}", .{err});
        return;
    };
    
    // Get and display results
    const issues = cursed_linter.getIssues();
    std.log.info("Found {} security issues:", .{issues.len});
    
    try linter.printIssues(allocator, issues, "human");
}
