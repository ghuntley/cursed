const std = @import("std");

test "simple test" {
    std.debug.print("Testing Pratt parser Phase 0\n", .{});
    try std.testing.expect(true);
}
