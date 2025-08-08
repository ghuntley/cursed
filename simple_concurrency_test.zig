const std = @import("std");
const concurrency_fixed = @import("src-zig/concurrency_fixed.zig");

test "simple channel test" {
    const allocator = std.testing.allocator;
    
    var channel = try concurrency_fixed.Channel(i32).init(allocator, 3);
    defer channel.deinit();
    
    // Test basic send/receive
    const send_result = try channel.sendTimeout(42, 1_000_000); // 1ms timeout
    try std.testing.expect(send_result == .sent);
    
    const received = try channel.receiveTimeout(1_000_000); // 1ms timeout
    try std.testing.expect(received != null);
    try std.testing.expect(received.? == 42);
    
    std.debug.print("Simple channel test passed\n", .{});
}

test "channel reference counting" {
    const allocator = std.testing.allocator;
    
    var channel = try concurrency_fixed.Channel(i32).init(allocator, 1);
    defer channel.deinit();
    
    // Test reference counting
    channel.addRef();
    try std.testing.expect(channel.ref_count.load(.acquire) == 2);
    
    channel.releaseRef();
    try std.testing.expect(channel.ref_count.load(.acquire) == 1);
    
    std.debug.print("Reference counting test passed\n", .{});
}

test "channel close behavior" {
    const allocator = std.testing.allocator;
    
    var channel = try concurrency_fixed.Channel(i32).init(allocator, 1);
    defer channel.deinit();
    
    // Send a value
    const send_result = try channel.sendTimeout(100, 1_000_000);
    try std.testing.expect(send_result == .sent);
    
    // Close channel
    channel.close();
    try std.testing.expect(channel.isClosed());
    
    // Try to send to closed channel
    const send_result2 = try channel.sendTimeout(200, 1_000_000);
    try std.testing.expect(send_result2 == .closed);
    
    // Should still be able to receive buffered value
    const received = try channel.receiveTimeout(1_000_000);
    try std.testing.expect(received != null);
    try std.testing.expect(received.? == 100);
    
    std.debug.print("Channel close test passed\n", .{});
}

test "basic scheduler test" {
    const allocator = std.testing.allocator;
    
    var scheduler = try concurrency_fixed.Scheduler.init(allocator, 2);
    defer scheduler.deinit();
    
    try scheduler.start();
    
    // Test basic goroutine spawning
    const test_context = try allocator.create(i32);
    test_context.* = 42;
    
    const goroutine_id = try scheduler.spawnGoroutine(@constCast(@ptrCast(&simpleTestFunc)), test_context);
    try std.testing.expect(goroutine_id > 0);
    
    // Give it time to execute
    std.time.sleep(10_000_000); // 10ms
    
    scheduler.shutdown();
    
    std.debug.print("Basic scheduler test passed\n", .{});
}

fn simpleTestFunc(context: ?*anyopaque) void {
    const value_ptr: *i32 = @ptrCast(@alignCast(context.?));
    const value = value_ptr.*;
    
    std.debug.print("Test function executed with value: {}\n", .{value});
    
    // Cleanup context
    std.heap.c_allocator.destroy(value_ptr);
}
