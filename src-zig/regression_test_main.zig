const std = @import("std");
const regression_test_runner = @import("regression_test_runner.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();
    
    // Get command line arguments
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    // Skip program name
    const test_args = if (args.len > 1) args[1..] else &[_][]const u8{};
    
    try regression_test_runner.runRegressionTests(allocator, test_args);
}
