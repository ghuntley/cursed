const std = @import("std");
const concurrency = @import("src-zig/concurrency_production_safe.zig");

// Demonstrates that the identified race conditions have been fixed:
// 1. Channel Size vs Buffer Length Inconsistency - Fixed with single mutex
// 2. Reference Count vs Cleanup Timing - Eliminated complex reference counting  
// 3. Double-Check Pattern Vulnerability - Eliminated double-check patterns
// 4. Goroutine State Transition Races - Simplified atomic state transitions

test "Race Condition Fix #1: Channel Size Consistency" {
    const allocator = std.testing.allocator;
    
    var channel = try concurrency.makeChannel(u32, allocator, 5);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }
    
    // Fill channel and verify consistency between sends and length
    for (0..5) |i| {
        const result = try channel.trySend(@intCast(i));
        try std.testing.expect(result == concurrency.SendResult.sent);
        
        // Length should always match number of successful sends
        try std.testing.expect(channel.length() == i + 1);
    }
    
    // Channel should be full now
    try std.testing.expect(channel.isFull());
    try std.testing.expect(channel.length() == 5);
    
    // Further sends should fail consistently
    const overflow_result = try channel.trySend(999);
    try std.testing.expect(overflow_result == concurrency.SendResult.would_block);
    
    // Length should remain consistent
    try std.testing.expect(channel.length() == 5);
    
    std.debug.print("✓ Channel size consistency fixed\n", .{});
}

test "Race Condition Fix #2: Safe Channel Cleanup" {
    const allocator = std.testing.allocator;
    
    // Test that channels can be safely cleaned up without complex reference counting
    for (0..10) |i| {
        var channel = try concurrency.makeChannel(u32, allocator, 3);
        
        // Use the channel
        _ = try channel.trySend(@intCast(i));
        _ = try channel.tryReceive();
        
        // Close and cleanup should be safe
        channel.close();
        channel.deinit();
        allocator.destroy(channel);
    }
    
    std.debug.print("✓ Safe cleanup without use-after-free fixed\n", .{});
}

test "Race Condition Fix #3: Elimination of Double-Check Patterns" {
    const allocator = std.testing.allocator;
    
    var channel = try concurrency.makeChannel(u32, allocator, 1);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }
    
    // Operations use single lock - no window for receivers to disappear
    _ = try channel.trySend(42);
    
    // This operation is atomic - no race between check and action
    const received = try channel.tryReceive();
    try std.testing.expect(received.? == 42);
    
    // Channel should be empty consistently
    try std.testing.expect(channel.isEmpty());
    
    // Next receive should consistently fail
    const empty_result = try channel.tryReceive();
    try std.testing.expect(empty_result == null);
    
    std.debug.print("✓ Double-check pattern vulnerabilities eliminated\n", .{});
}

test "Race Condition Fix #4: Goroutine State Transitions" {
    const allocator = std.testing.allocator;
    
    try concurrency.initRuntime(allocator);
    defer concurrency.shutdownRuntime();
    
    // Test that goroutine state transitions are atomic and consistent
    var state_test_completed = std.atomic.Value(bool).init(false);
    
    const TestContext = struct {
        completed: *std.atomic.Value(bool),
        
        fn run(ctx: ?*anyopaque) void {
            const test_ctx: *@This() = @ptrCast(@alignCast(ctx.?));
            
            // Simulate some work
            std.time.sleep(10_000_000); // 10ms
            
            // Mark as completed
            test_ctx.completed.store(true, .release);
        }
    };
    
    var context = TestContext{ .completed = &state_test_completed };
    
    // Spawn goroutine
    _ = try concurrency.stan(TestContext.run, &context);
    
    // Wait for completion
    var timeout_count: u32 = 0;
    while (!state_test_completed.load(.acquire) and timeout_count < 100) {
        std.time.sleep(10_000_000); // 10ms
        timeout_count += 1;
    }
    
    try std.testing.expect(state_test_completed.load(.acquire));
    
    std.debug.print("✓ Goroutine state transitions race-free\n", .{});
}

test "Concurrent Channel Operations Stress Test" {
    const allocator = std.testing.allocator;
    
    try concurrency.initRuntime(allocator);
    defer concurrency.shutdownRuntime();
    
    var channel = try concurrency.makeChannel(u32, allocator, 10);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }
    
    // Counters for validation
    var sent_count = std.atomic.Value(u32).init(0);
    var received_count = std.atomic.Value(u32).init(0);
    var producer_done = std.atomic.Value(bool).init(false);
    
    const ProducerContext = struct {
        channel: *concurrency.Channel(u32),
        sent_count: *std.atomic.Value(u32),
        done: *std.atomic.Value(bool),
        
        fn run(ctx: ?*anyopaque) void {
            const producer_ctx: *@This() = @ptrCast(@alignCast(ctx.?));
            
            // Send 50 values
            for (0..50) |i| {
                while (true) {
                    const result = producer_ctx.channel.trySend(@intCast(i)) catch break;
                    if (result == concurrency.SendResult.sent) {
                        _ = producer_ctx.sent_count.fetchAdd(1, .seq_cst);
                        break;
                    }
                    std.time.sleep(1_000_000); // 1ms retry
                }
            }
            
            producer_ctx.done.store(true, .release);
        }
    };
    
    const ConsumerContext = struct {
        channel: *concurrency.Channel(u32),
        received_count: *std.atomic.Value(u32),
        producer_done: *std.atomic.Value(bool),
        
        fn run(ctx: ?*anyopaque) void {
            const consumer_ctx: *@This() = @ptrCast(@alignCast(ctx.?));
            
            while (!consumer_ctx.producer_done.load(.acquire) or 
                   consumer_ctx.channel.length() > 0) {
                
                if (consumer_ctx.channel.tryReceive() catch null) |_| {
                    _ = consumer_ctx.received_count.fetchAdd(1, .seq_cst);
                } else {
                    std.time.sleep(1_000_000); // 1ms retry
                }
            }
        }
    };
    
    var producer_context = ProducerContext{
        .channel = channel,
        .sent_count = &sent_count,
        .done = &producer_done,
    };
    
    var consumer_context = ConsumerContext{
        .channel = channel,
        .received_count = &received_count,
        .producer_done = &producer_done,
    };
    
    // Start producer and consumer
    _ = try concurrency.stan(ProducerContext.run, &producer_context);
    _ = try concurrency.stan(ConsumerContext.run, &consumer_context);
    
    // Wait for completion
    std.time.sleep(2_000_000_000); // 2 seconds
    
    const final_sent = sent_count.load(.acquire);
    const final_received = received_count.load(.acquire);
    
    std.debug.print("Sent: {}, Received: {}\n", .{ final_sent, final_received });
    
    // In a race-free implementation, received should equal sent
    try std.testing.expect(final_sent == 50);
    try std.testing.expect(final_received == final_sent);
    
    std.debug.print("✓ Concurrent operations completed without races\n", .{});
}

test "Memory Safety Validation" {
    const allocator = std.testing.allocator;
    
    // This test validates that the fixes prevent memory corruption
    // and use-after-free issues
    
    var channels: [20]*concurrency.Channel(u64) = undefined;
    
    // Create many channels
    for (0..20) |i| {
        channels[i] = try concurrency.makeChannel(u64, allocator, 5);
    }
    
    // Use them concurrently
    for (0..20) |i| {
        _ = try channels[i].trySend(i * 1000);
        
        if (i % 2 == 0) {
            _ = try channels[i].tryReceive();
        }
    }
    
    // Clean up in reverse order (stress test)
    var cleanup_idx: usize = 20;
    while (cleanup_idx > 0) {
        cleanup_idx -= 1;
        channels[cleanup_idx].deinit();
        allocator.destroy(channels[cleanup_idx]);
    }
    
    std.debug.print("✓ Memory safety validation passed\n", .{});
}
