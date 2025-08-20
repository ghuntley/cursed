const std = @import("std");
const error_execution_fixed = @import("src-zig/error_execution_fixed.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    try error_execution_fixed.testErrorHandling(allocator);
}
