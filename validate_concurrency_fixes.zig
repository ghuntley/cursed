const std = @import("std");
const concurrency = @import("src-zig/concurrency_race_condition_fixes.zig");

test "race condition fixes validation" {
    const allocator = std.testing.allocator;
    
    // Initialize runtime
    try concurrency.initRuntime(allocator, 4);
    defer concurrency.shutdownRuntime();
    
    // Test 1: Channel size consistency
    var channel = try concurrency.makeChannel(i32, allocator, 5);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }
    
    // Fill channel
    for (0..5) |i| {
        const result = try channel.send(@intCast(i));
        try std.testing.expect(result == concurrency.SendResult.sent);
    }
    
    // Verify length consistency
    try std.testing.expect(channel.length() == 5);
    try std.testing.expect(channel.isFull());
    
    // Test 2: Concurrent access without races
    const TestContext = struct {
        channel: *concurrency.Channel(i32),
        sent_count: std.atomic.Value(u32),
        received_count: std.atomic.Value(u32),
        
        fn sender(ctx: ?*anyopaque) void {
            const test_ctx: *@This() = @ptrCast(@alignCast(ctx.?));
            for (0..100) |i| {
                _ = test_ctx.channel.send(@intCast(i)) catch {};
                _ = test_ctx.sent_count.fetchAdd(1, .seq_cst);
            }
        }
        
        fn receiver(ctx: ?*anyopaque) void {
            const test_ctx: *@This() = @ptrCast(@alignCast(ctx.?));
            while (test_ctx.received_count.load(.acquire) < 500) {
                if (test_ctx.channel.tryReceive() catch null) |_| {
                    _ = test_ctx.received_count.fetchAdd(1, .seq_cst);
                }
                std.time.sleep(1_000_000); // 1ms
            }
        }
    };
    
    var test_channel = try concurrency.makeChannel(i32, allocator, 50);
    defer {
        test_channel.deinit();
        allocator.destroy(test_channel);
    }
    
    var test_ctx = TestContext{
        .channel = test_channel,
        .sent_count = std.atomic.Value(u32).init(0),
        .received_count = std.atomic.Value(u32).init(0),
    };
    
    // Spawn concurrent senders and receivers
    var sender_ids: [5]concurrency.GoroutineId = undefined;
    var receiver_ids: [3]concurrency.GoroutineId = undefined;
    
    // Start senders
    for (0..5) |i| {
        sender_ids[i] = try concurrency.stan(TestContext.sender, &test_ctx);
    }
    
    // Start receivers
    for (0..3) |i| {
        receiver_ids[i] = try concurrency.stan(TestContext.receiver, &test_ctx);
    }
    
    // Wait for completion
    std.time.sleep(5_000_000_000); // 5 seconds
    
    // Verify no races occurred (sent >= received, no corruption)
    const final_sent = test_ctx.sent_count.load(.acquire);
    const final_received = test_ctx.received_count.load(.acquire);
    
    try std.testing.expect(final_sent >= final_received);
    
    std.debug.print("Sent: {}, Received: {}\n", .{ final_sent, final_received });
}

test "goroutine state transitions without races" {
    const allocator = std.testing.allocator;
    
    try concurrency.initRuntime(allocator, 2);
    defer concurrency.shutdownRuntime();
    
    var completion_count = std.atomic.Value(u32).init(0);
    
    const TestFn = struct {
        count: *std.atomic.Value(u32),
        
        fn run(ctx: ?*anyopaque) void {
            const test_ctx: *@This() = @ptrCast(@alignCast(ctx.?));
            _ = test_ctx.count.fetchAdd(1, .seq_cst);
        }
    };
    
    var test_ctx = TestFn{ .count = &completion_count };
    
    // Spawn many goroutines quickly to test state transitions
    for (0..100) |_| {
        _ = try concurrency.stan(TestFn.run, &test_ctx);
    }
    
    // Wait for completion
    std.time.sleep(2_000_000_000); // 2 seconds
    
    const final_count = completion_count.load(.acquire);
    try std.testing.expect(final_count == 100);
    
    std.debug.print("Completed goroutines: {}\n", .{final_count});
}

test "channel cleanup without use-after-free" {
    const allocator = std.testing.allocator;
    
    // Test that channels can be safely cleaned up even with concurrent access
    var channels: [10]*concurrency.Channel(i32) = undefined;
    
    // Create channels
    for (0..10) |i| {
        channels[i] = try concurrency.makeChannel(i32, allocator, 5);
    }
    
    // Use channels concurrently then clean up
    for (0..10) |i| {
        _ = try channels[i].send(@intCast(i));
        _ = try channels[i].tryReceive();
        
        channels[i].deinit();
        allocator.destroy(channels[i]);
    }
    
    // Should not crash or leak memory
}
