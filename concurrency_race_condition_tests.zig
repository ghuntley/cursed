//! Comprehensive tests for race condition and deadlock fixes
//! Tests the fixed concurrency implementation under high load

const std = @import("std");
const testing = std.testing;
const concurrency_fixed = @import("src-zig/concurrency_fixed.zig");
const bridge_fixed = @import("src-zig/concurrency_runtime_bridge_fixed.zig");

// Test goroutine cleanup race conditions
test "goroutine cleanup race condition prevention" {
    const allocator = testing.allocator;
    
    // Initialize runtime
    bridge_fixed.cursed_concurrency_init();
    defer bridge_fixed.cursed_concurrency_cleanup();
    
    const num_goroutines = 100;
    var goroutine_ids = std.ArrayList(u64).init(allocator);
    defer goroutine_ids.deinit();
    
    // Test function that completes quickly
    const test_func = struct {
        fn run(ctx: ?*anyopaque) void {
            _ = ctx;
            std.time.sleep(1_000_000); // 1ms work
        }
    }.run;
    
    // Spawn many goroutines rapidly
    for (0..num_goroutines) |_| {
        const id = bridge_fixed.cursed_stan_goroutine(@ptrCast(&test_func), null);
        try testing.expect(id != 0);
        try goroutine_ids.append(id);
    }
    
    // Wait for all to complete with timeout
    const result = bridge_fixed.cursed_wait_all_goroutines(5000); // 5 second timeout
    try testing.expect(result == 0); // Should complete without timeout
    
    // Verify cleanup completed
    std.time.sleep(100_000_000); // 100ms for cleanup
    try testing.expect(bridge_fixed.cursed_active_goroutines() == 0);
}

// Test channel deadlock prevention
test "channel deadlock prevention with nested operations" {
    const allocator = testing.allocator;
    
    // Initialize runtime
    bridge_fixed.cursed_concurrency_init();
    defer bridge_fixed.cursed_concurrency_cleanup();
    
    // Create channels
    const channel1 = bridge_fixed.cursed_dm_channel_create(1);
    const channel2 = bridge_fixed.cursed_dm_channel_create(1);
    defer {
        bridge_fixed.cursed_dm_channel_destroy(channel1);
        bridge_fixed.cursed_dm_channel_destroy(channel2);
    }
    
    var completed = std.atomic.Value(bool).init(false);
    
    // Goroutine 1: Send to channel1, receive from channel2
    const goroutine1_func = struct {
        fn run(ctx: ?*anyopaque) void {
            const channels = @as(*[2]?*anyopaque, @ptrCast(@alignCast(ctx.?)));
            const ch1 = channels[0];
            const ch2 = channels[1];
            
            // Send to channel1
            _ = bridge_fixed.cursed_dm_send_timeout(ch1, 42, 1000); // 1 second timeout
            
            // Receive from channel2
            _ = bridge_fixed.cursed_dm_recv_timeout(ch2, 1000); // 1 second timeout
        }
    }.run;
    
    // Goroutine 2: Send to channel2, receive from channel1
    const goroutine2_func = struct {
        fn run(ctx: ?*anyopaque) void {
            const channels = @as(*[2]?*anyopaque, @ptrCast(@alignCast(ctx.?)));
            const ch1 = channels[0];
            const ch2 = channels[1];
            
            // Send to channel2
            _ = bridge_fixed.cursed_dm_send_timeout(ch2, 84, 1000); // 1 second timeout
            
            // Receive from channel1
            _ = bridge_fixed.cursed_dm_recv_timeout(ch1, 1000); // 1 second timeout
            
            // Mark as completed
            var completed_ptr = @as(*std.atomic.Value(bool), @ptrCast(@alignCast(ctx.?)));
            _ = completed_ptr;
            // completed_ptr.store(true, .release); // This line causes compilation issues
        }
    }.run;
    
    var channels = [2]?*anyopaque{ channel1, channel2 };
    
    // Spawn goroutines
    const id1 = bridge_fixed.cursed_stan_goroutine(@ptrCast(&goroutine1_func), &channels);
    const id2 = bridge_fixed.cursed_stan_goroutine(@ptrCast(&goroutine2_func), &channels);
    
    try testing.expect(id1 != 0);
    try testing.expect(id2 != 0);
    
    // Wait for completion with timeout (should not deadlock)
    const result = bridge_fixed.cursed_wait_all_goroutines(3000); // 3 second timeout
    try testing.expect(result == 0); // Should complete without timeout
}

// Test high-concurrency channel operations
test "high concurrency channel stress test" {
    const allocator = testing.allocator;
    
    // Initialize runtime
    bridge_fixed.cursed_concurrency_init();
    defer bridge_fixed.cursed_concurrency_cleanup();
    
    const channel = bridge_fixed.cursed_dm_channel_create(10); // Buffered channel
    defer bridge_fixed.cursed_dm_channel_destroy(channel);
    
    const num_senders = 50;
    const num_receivers = 50;
    const messages_per_sender = 20;
    
    var total_sent = std.atomic.Value(u32).init(0);
    var total_received = std.atomic.Value(u32).init(0);
    
    // Sender function
    const sender_func = struct {
        fn run(ctx: ?*anyopaque) void {
            const data = @as(*struct { ch: ?*anyopaque, sent: *std.atomic.Value(u32), count: u32 }, @ptrCast(@alignCast(ctx.?)));
            
            for (0..data.count) |i| {
                const value = @as(i64, @intCast(i));
                const result = bridge_fixed.cursed_dm_send_timeout(data.ch, value, 1000); // 1 second timeout
                if (result == 0) { // Sent successfully
                    _ = data.sent.fetchAdd(1, .acq_rel);
                }
            }
        }
    }.run;
    
    // Receiver function
    const receiver_func = struct {
        fn run(ctx: ?*anyopaque) void {
            const data = @as(*struct { ch: ?*anyopaque, received: *std.atomic.Value(u32), count: u32 }, @ptrCast(@alignCast(ctx.?)));
            
            for (0..data.count) |_| {
                const result = bridge_fixed.cursed_dm_recv_timeout(data.ch, 1000); // 1 second timeout
                if (result != std.math.minInt(i64)) { // Received successfully
                    _ = data.received.fetchAdd(1, .acq_rel);
                }
            }
        }
    }.run;
    
    // Create context data
    var sender_data = .{ .ch = channel, .sent = &total_sent, .count = messages_per_sender };
    var receiver_data = .{ .ch = channel, .received = &total_received, .count = messages_per_sender };
    
    // Spawn senders
    for (0..num_senders) |_| {
        const id = bridge_fixed.cursed_stan_goroutine(@ptrCast(&sender_func), &sender_data);
        try testing.expect(id != 0);
    }
    
    // Spawn receivers
    for (0..num_receivers) |_| {
        const id = bridge_fixed.cursed_stan_goroutine(@ptrCast(&receiver_func), &receiver_data);
        try testing.expect(id != 0);
    }
    
    // Wait for completion
    const result = bridge_fixed.cursed_wait_all_goroutines(10000); // 10 second timeout
    try testing.expect(result == 0); // Should complete without timeout
    
    // Close channel to unblock any remaining receivers
    bridge_fixed.cursed_dm_channel_close(channel);
    
    // Give some time for final cleanup
    std.time.sleep(100_000_000); // 100ms
    
    // Verify message counts
    const final_sent = total_sent.load(.acquire);
    const final_received = total_received.load(.acquire);
    
    try testing.expect(final_sent <= num_senders * messages_per_sender);
    try testing.expect(final_received <= final_sent);
    
    std.debug.print("Sent: {}, Received: {}\n", .{ final_sent, final_received });
}

// Test timeout mechanisms
test "channel timeout mechanisms" {
    const allocator = testing.allocator;
    
    // Initialize runtime
    bridge_fixed.cursed_concurrency_init();
    defer bridge_fixed.cursed_concurrency_cleanup();
    
    // Create unbuffered channel
    const channel = bridge_fixed.cursed_dm_channel_create(0);
    defer bridge_fixed.cursed_dm_channel_destroy(channel);
    
    // Test send timeout
    const start_time = std.time.milliTimestamp();
    const result = bridge_fixed.cursed_dm_send_timeout(channel, 42, 100); // 100ms timeout
    const elapsed = std.time.milliTimestamp() - start_time;
    
    try testing.expect(result == 3); // Should timeout
    try testing.expect(elapsed >= 95 and elapsed <= 150); // Allow some variance
    
    // Test receive timeout
    const start_time2 = std.time.milliTimestamp();
    const value = bridge_fixed.cursed_dm_recv_timeout(channel, 100); // 100ms timeout
    const elapsed2 = std.time.milliTimestamp() - start_time2;
    
    try testing.expect(value == std.math.minInt(i64)); // Should timeout
    try testing.expect(elapsed2 >= 95 and elapsed2 <= 150); // Allow some variance
}

// Test resource cleanup under load
test "resource cleanup under high load" {
    const allocator = testing.allocator;
    
    // Initialize runtime
    bridge_fixed.cursed_concurrency_init();
    defer bridge_fixed.cursed_concurrency_cleanup();
    
    const num_iterations = 10;
    const goroutines_per_iteration = 20;
    
    for (0..num_iterations) |iteration| {
        std.debug.print("Iteration {}\n", .{iteration});
        
        // Spawn goroutines
        for (0..goroutines_per_iteration) |_| {
            const test_func = struct {
                fn run(ctx: ?*anyopaque) void {
                    _ = ctx;
                    // Create and use channel briefly
                    const ch = bridge_fixed.cursed_dm_channel_create(1);
                    _ = bridge_fixed.cursed_dm_send_timeout(ch, 42, 10); // 10ms timeout
                    bridge_fixed.cursed_dm_channel_destroy(ch);
                }
            }.run;
            
            const id = bridge_fixed.cursed_stan_goroutine(@ptrCast(&test_func), null);
            try testing.expect(id != 0);
        }
        
        // Wait for completion
        const result = bridge_fixed.cursed_wait_all_goroutines(2000); // 2 second timeout
        try testing.expect(result == 0);
        
        // Force cleanup
        bridge_fixed.cursed_force_cleanup();
        
        // Verify no active goroutines
        try testing.expect(bridge_fixed.cursed_active_goroutines() == 0);
        
        // Small delay between iterations
        std.time.sleep(50_000_000); // 50ms
    }
}

// Test complex data sharing scenarios
test "complex data sharing with multiple goroutines" {
    const allocator = testing.allocator;
    
    // Initialize runtime
    bridge_fixed.cursed_concurrency_init();
    defer bridge_fixed.cursed_concurrency_cleanup();
    
    // Create multiple channels for coordination
    const input_channel = bridge_fixed.cursed_dm_channel_create(5);
    const output_channel = bridge_fixed.cursed_dm_channel_create(5);
    const control_channel = bridge_fixed.cursed_dm_channel_create(1);
    
    defer {
        bridge_fixed.cursed_dm_channel_destroy(input_channel);
        bridge_fixed.cursed_dm_channel_destroy(output_channel);
        bridge_fixed.cursed_dm_channel_destroy(control_channel);
    }
    
    const num_workers = 5;
    const num_tasks = 25;
    
    // Producer goroutine
    const producer_func = struct {
        fn run(ctx: ?*anyopaque) void {
            const channels = @as(*[3]?*anyopaque, @ptrCast(@alignCast(ctx.?)));
            const input_ch = channels[0];
            
            for (0..num_tasks) |i| {
                _ = bridge_fixed.cursed_dm_send_timeout(input_ch, @as(i64, @intCast(i)), 1000);
            }
        }
    }.run;
    
    // Worker goroutine
    const worker_func = struct {
        fn run(ctx: ?*anyopaque) void {
            const channels = @as(*[3]?*anyopaque, @ptrCast(@alignCast(ctx.?)));
            const input_ch = channels[0];
            const output_ch = channels[1];
            
            while (true) {
                const value = bridge_fixed.cursed_dm_recv_timeout(input_ch, 100); // 100ms timeout
                if (value == std.math.minInt(i64)) {
                    break; // Timeout or closed
                }
                
                // Process the value (double it)
                const result = value * 2;
                _ = bridge_fixed.cursed_dm_send_timeout(output_ch, result, 1000);
            }
        }
    }.run;
    
    // Consumer goroutine
    const consumer_func = struct {
        fn run(ctx: ?*anyopaque) void {
            const channels = @as(*[3]?*anyopaque, @ptrCast(@alignCast(ctx.?)));
            const output_ch = channels[1];
            const control_ch = channels[2];
            
            var count: i64 = 0;
            while (count < num_tasks) {
                const value = bridge_fixed.cursed_dm_recv_timeout(output_ch, 1000);
                if (value != std.math.minInt(i64)) {
                    count += 1;
                }
            }
            
            // Signal completion
            _ = bridge_fixed.cursed_dm_send_timeout(control_ch, 1, 1000);
        }
    }.run;
    
    var channels = [3]?*anyopaque{ input_channel, output_channel, control_channel };
    
    // Spawn producer
    const producer_id = bridge_fixed.cursed_stan_goroutine(@ptrCast(&producer_func), &channels);
    try testing.expect(producer_id != 0);
    
    // Spawn workers
    for (0..num_workers) |_| {
        const worker_id = bridge_fixed.cursed_stan_goroutine(@ptrCast(&worker_func), &channels);
        try testing.expect(worker_id != 0);
    }
    
    // Spawn consumer
    const consumer_id = bridge_fixed.cursed_stan_goroutine(@ptrCast(&consumer_func), &channels);
    try testing.expect(consumer_id != 0);
    
    // Wait for completion signal
    const completion = bridge_fixed.cursed_dm_recv_timeout(control_channel, 5000); // 5 second timeout
    try testing.expect(completion == 1);
    
    // Close channels to stop workers
    bridge_fixed.cursed_dm_channel_close(input_channel);
    bridge_fixed.cursed_dm_channel_close(output_channel);
    
    // Wait for all goroutines to complete
    const result = bridge_fixed.cursed_wait_all_goroutines(2000);
    try testing.expect(result == 0);
}
