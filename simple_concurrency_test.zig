const std = @import("std");
const concurrency = @import("src-zig/concurrency_race_condition_fixes.zig");

test "simple channel operations" {
    const allocator = std.testing.allocator;
    
    var channel = try concurrency.makeChannel(i32, allocator, 3);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }
    
    // Test basic operations
    const result1 = try channel.trySend(42);
    try std.testing.expect(result1 == concurrency.SendResult.sent);
    
    const result2 = try channel.tryReceive();
    try std.testing.expect(result2.? == 42);
    
    // Test channel full
    for (0..3) |i| {
        const result = try channel.trySend(@intCast(i));
        try std.testing.expect(result == concurrency.SendResult.sent);
    }
    
    // Should be full now
    const result3 = try channel.trySend(999);
    try std.testing.expect(result3 == concurrency.SendResult.would_block);
    
    std.debug.print("Simple channel test passed\n", .{});
}

test "channel close behavior" {
    const allocator = std.testing.allocator;
    
    var channel = try concurrency.makeChannel(i32, allocator, 1);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }
    
    // Send and close
    _ = try channel.trySend(100);
    channel.close();
    
    // Should not be able to send after close
    const result = try channel.trySend(200);
    try std.testing.expect(result == concurrency.SendResult.closed);
    
    // Should still be able to receive buffered data
    const received = try channel.tryReceive();
    try std.testing.expect(received.? == 100);
    
    std.debug.print("Channel close test passed\n", .{});
}

test "goroutine basic state" {
    const allocator = std.testing.allocator;
    
    const testFn = struct {
        fn run(_: ?*anyopaque) void {}
    }.run;
    
    var goroutine = concurrency.Goroutine.init(allocator, 1, testFn, null);
    
    // Test state transitions
    try std.testing.expect(goroutine.getState() == .ready);
    try std.testing.expect(goroutine.transitionState(.ready, .running));
    try std.testing.expect(goroutine.getState() == .running);
    try std.testing.expect(goroutine.transitionState(.running, .completed));
    try std.testing.expect(goroutine.getState() == .completed);
    
    std.debug.print("Goroutine state test passed\n", .{});
}
