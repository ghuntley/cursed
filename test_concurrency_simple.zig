const std = @import("std");
const concurrency = @import("src-zig/concurrency_old.zig");

test "basic scheduler initialization" {
    const allocator = std.testing.allocator;
    
    const config = concurrency.SchedulerConfig.default();
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);
    
    // Test that scheduler was created
    const scheduler = concurrency.getScheduler();
    try std.testing.expect(scheduler != null);
}

test "basic channel creation" {
    const allocator = std.testing.allocator;
    
    const config = concurrency.SchedulerConfig.default();
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);
    
    var channel = try concurrency.makeChannel(i32, allocator, 2);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }
    
    try std.testing.expect(channel.id > 0);
}
