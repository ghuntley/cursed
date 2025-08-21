const std = @import("std");

test "check reader API" {
    const stdin = std.fs.File.stdin();
    const reader = stdin.reader();
    const allocator = std.testing.allocator;
    
    // Check what methods are available
    _ = reader;
    _ = allocator;
    
    std.debug.print("Reader type: {}\n", .{@TypeOf(reader)});
}
