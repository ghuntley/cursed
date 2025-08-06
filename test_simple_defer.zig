const std = @import("std");
const print = std.debug.print;

// Test defer functionality by creating a minimal defer test
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    _ = gpa.allocator(); // Keep allocator variable to avoid unused warning
    
    print("🚀 Testing CURSED defer implementation\n", .{});
    
    // Test basic defer functionality
    var defer_runtime = @import("src-zig/defer_runtime.zig");
    
    try defer_runtime.runTests();
    
    print("✅ All defer tests passed!\n", .{});
}
