const std = @import("std");
const performance_hooks = @import("src-zig/performance_hooks.zig");
const performance_integration = @import("src-zig/performance_integration.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print("=== CURSED Performance Hooks Test ===\n\n", .{});

    // Test the performance integration
    try performance_integration.testPerformanceIntegration(allocator);

    std.debug.print("\n=== Test completed successfully ===\n", .{});
}
