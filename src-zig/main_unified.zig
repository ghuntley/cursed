// Import the minimal main for now during Oracle P2 migration
const minimal = @import("minimal_main.zig");

pub fn main() !void {
    return minimal.main();
}

test "unified compiler basic test" {
    // Basic test to ensure build system works
    const allocator = std.testing.allocator;
    _ = allocator;
    
    // Test that we can run tests
    try std.testing.expect(true);
}

const std = @import("std");
