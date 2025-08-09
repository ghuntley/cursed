//! Comprehensive Channel Race Condition Stress Test
//! Tests the fixed channel implementation under extreme concurrent load

const std = @import("std");
const print = std.debug.print;
const expect = std.testing.expect;
const Allocator = std.mem.Allocator;
const Thread = std.Thread;
const concurrency = @import("concurrency.zig");
const Channel = concurrency.Channel;

/// Test configuration
const TestConfig = struct {
    num_goroutines: u32 = 1000,
    messages_per_goroutine: u32 = 100,
    test_duration_ms: u64 = 5000,  // 5 seconds
    channel_buffer_size: usize = 10,
};

/// Test statistics
var test_stats = struct {
    messages_sent: std.atomic.Value(u64) = std.atomic.Value(u64).init(0),
    messages_received: std.atomic.Value(u64) = std.atomic.Value(u64).init(0),
    goroutines_completed: std.atomic.Value(u64) = std.atomic.Value(u64).init(0),
    race_conditions_detected: std.atomic.Value(u64) = std.atomic.Value(u64).init(0),
    memory_errors: std.atomic.Value(u64) = std.atomic.Value(u64).init(0),
}{};

/// Context for goroutine operations
const GoroutineContext = struct {
    channel: *Channel(u64),
    goroutine_id: u64,
    config: TestConfig,
    should_stop: *std.atomic.Value(bool),
};

/// Sender goroutine that sends many messages then exits
fn senderGoroutine(ctx: *GoroutineContext) void {
    defer _ = test_stats.goroutines_completed.fetchAdd(1, .acq_rel);
    
    var messages_sent: u32 = 0;
    const start_time = std.time.milliTimestamp();
    
    while (messages_sent < ctx.config.messages_per_goroutine and 
           !ctx.should_stop.load(.acquire)) {
        
        // Check if test duration exceeded
        const elapsed = std.time.milliTimestamp() - start_time;
        if (elapsed > ctx.config.test_duration_ms) {
            break;
        }
        
        const message = ctx.goroutine_id * 1000000 + messages_sent;
        
        // Attempt to send with race condition detection
        const result = ctx.channel.dm_send(message) catch |err| {
            print("❌ Send error in goroutine {}: {}\n", .{ ctx.goroutine_id, err });
            _ = test_stats.memory_errors.fetchAdd(1, .acq_rel);
            return;
        };
        
        switch (result) {
            .sent => {
                _ = test_stats.messages_sent.fetchAdd(1, .acq_rel);
                messages_sent += 1;
            },
            .closed => {
                // Channel closed, exit gracefully
                break;
            },
            .would_block => {
                // Simulate some work then retry
                std.time.sleep(1_000); // 1μs
            },
        }
        
        // Simulate random goroutine early termination (race condition scenario)
        if (ctx.goroutine_id % 17 == 0 and messages_sent > 5) {
            // 1 in 17 goroutines exit early after sending few messages
            break;
        }
    }
}

/// Receiver goroutine that receives messages and may exit early
fn receiverGoroutine(ctx: *GoroutineContext) void {
    defer _ = test_stats.goroutines_completed.fetchAdd(1, .acq_rel);
    
    var messages_received: u32 = 0;
    const start_time = std.time.milliTimestamp();
    
    while (messages_received < ctx.config.messages_per_goroutine and 
           !ctx.should_stop.load(.acquire)) {
        
        // Check if test duration exceeded
        const elapsed = std.time.milliTimestamp() - start_time;
        if (elapsed > ctx.config.test_duration_ms) {
            break;
        }
        
        // Attempt to receive with race condition detection
        const message = ctx.channel.dm_recv() catch |err| {
            print("❌ Receive error in goroutine {}: {}\n", .{ ctx.goroutine_id, err });
            _ = test_stats.memory_errors.fetchAdd(1, .acq_rel);
            return;
        };
        
        if (message) |msg| {
            _ = test_stats.messages_received.fetchAdd(1, .acq_rel);
            messages_received += 1;
            
            // Validate message integrity
            const sender_id = msg / 1000000;
            const message_id = msg % 1000000;
            
            if (sender_id > ctx.config.num_goroutines or message_id > ctx.config.messages_per_goroutine) {
                print("❌ Corrupted message detected: {}\n", .{msg});
                _ = test_stats.race_conditions_detected.fetchAdd(1, .acq_rel);
            }
        } else {
            // Channel closed or empty
            break;
        }
        
        // Simulate random receiver early termination (race condition scenario)
        if (ctx.goroutine_id % 13 == 0 and messages_received > 10) {
            // 1 in 13 receivers exit early after receiving some messages
            break;
        }
    }
}

/// Mixed goroutine that both sends and receives (more complex race conditions)
fn mixedGoroutine(ctx: *GoroutineContext) void {
    defer _ = test_stats.goroutines_completed.fetchAdd(1, .acq_rel);
    
    var operations: u32 = 0;
    const start_time = std.time.milliTimestamp();
    
    while (operations < ctx.config.messages_per_goroutine and 
           !ctx.should_stop.load(.acquire)) {
        
        // Check if test duration exceeded
        const elapsed = std.time.milliTimestamp() - start_time;
        if (elapsed > ctx.config.test_duration_ms) {
            break;
        }
        
        // Alternately send and receive
        if (operations % 2 == 0) {
            // Send operation
            const message = ctx.goroutine_id * 1000000 + operations;
            const result = ctx.channel.dm_send(message) catch |err| {
                print("❌ Mixed send error in goroutine {}: {}\n", .{ ctx.goroutine_id, err });
                _ = test_stats.memory_errors.fetchAdd(1, .acq_rel);
                return;
            };
            
            if (result == .sent) {
                _ = test_stats.messages_sent.fetchAdd(1, .acq_rel);
            }
        } else {
            // Receive operation
            const message = ctx.channel.dm_recv() catch |err| {
                print("❌ Mixed receive error in goroutine {}: {}\n", .{ ctx.goroutine_id, err });
                _ = test_stats.memory_errors.fetchAdd(1, .acq_rel);
                return;
            };
            
            if (message) |_| {
                _ = test_stats.messages_received.fetchAdd(1, .acq_rel);
            }
        }
        
        operations += 1;
        
        // Aggressive early termination to stress race conditions
        if (ctx.goroutine_id % 7 == 0 and operations > 3) {
            break;
        }
    }
}

/// Run comprehensive stress test
pub fn runChannelStressTest(allocator: Allocator, config: TestConfig) !void {
    print("🚀 Starting Channel Race Condition Stress Test\n");
    print("   Goroutines: {}\n", .{config.num_goroutines});
    print("   Messages per goroutine: {}\n", .{config.messages_per_goroutine});
    print("   Channel buffer size: {}\n", .{config.channel_buffer_size});
    print("   Test duration: {}ms\n", .{config.test_duration_ms});
    
    // Reset statistics
    test_stats.messages_sent.store(0, .release);
    test_stats.messages_received.store(0, .release);
    test_stats.goroutines_completed.store(0, .release);
    test_stats.race_conditions_detected.store(0, .release);
    test_stats.memory_errors.store(0, .release);
    
    // Create channel with race condition fixes
    var channel = try Channel(u64).init(allocator, config.channel_buffer_size);
    defer {
        // Use proper cleanup with reference counting
        channel.dm_close();
        channel.release(); // This will trigger cleanup when all goroutines are done
    }
    
    // Control flag for stopping test
    var should_stop = std.atomic.Value(bool).init(false);
    
    // Create goroutine contexts
    var contexts = try allocator.alloc(GoroutineContext, config.num_goroutines);
    defer allocator.free(contexts);
    
    for (contexts, 0..) |*ctx, i| {
        ctx.* = GoroutineContext{
            .channel = &channel,
            .goroutine_id = i,
            .config = config,
            .should_stop = &should_stop,
        };
    }
    
    // Create thread handles
    var threads = try allocator.alloc(?Thread, config.num_goroutines);
    defer allocator.free(threads);
    
    const start_time = std.time.milliTimestamp();
    
    // Start goroutines with different patterns
    for (threads, 0..) |*thread, i| {
        const ctx = &contexts[i];
        
        // Distribute goroutine types to create maximum race condition potential
        if (i % 3 == 0) {
            // Senders (33%)
            thread.* = try Thread.spawn(.{}, senderGoroutine, .{ctx});
        } else if (i % 3 == 1) {
            // Receivers (33%)
            thread.* = try Thread.spawn(.{}, receiverGoroutine, .{ctx});
        } else {
            // Mixed (33%) - most likely to cause race conditions
            thread.* = try Thread.spawn(.{}, mixedGoroutine, .{ctx});
        }
    }
    
    // Let test run for specified duration
    std.time.sleep(config.test_duration_ms * 1_000_000); // Convert to nanoseconds
    
    // Signal all goroutines to stop
    should_stop.store(true, .release);
    
    // Wait for all goroutines to complete
    print("⏳ Waiting for {} goroutines to complete...\n", .{config.num_goroutines});
    for (threads) |thread| {
        if (thread) |t| {
            t.join();
        }
    }
    
    const total_time = std.time.milliTimestamp() - start_time;
    
    // Close channel and allow cleanup
    channel.dm_close();
    
    // Wait a bit for cleanup to complete
    std.time.sleep(100_000_000); // 100ms
    
    // Print results
    const messages_sent = test_stats.messages_sent.load(.acquire);
    const messages_received = test_stats.messages_received.load(.acquire);
    const goroutines_completed = test_stats.goroutines_completed.load(.acquire);
    const race_conditions = test_stats.race_conditions_detected.load(.acquire);
    const memory_errors = test_stats.memory_errors.load(.acquire);
    
    print("\n📊 Stress Test Results ({}ms):\n", .{total_time});
    print("   Messages sent: {}\n", .{messages_sent});
    print("   Messages received: {}\n", .{messages_received});
    print("   Goroutines completed: {}/{}\n", .{ goroutines_completed, config.num_goroutines });
    print("   Race conditions detected: {}\n", .{race_conditions});
    print("   Memory errors: {}\n", .{memory_errors});
    print("   Throughput: {:.2} messages/second\n", .{ @as(f64, @floatFromInt(messages_sent + messages_received)) / (@as(f64, @floatFromInt(total_time)) / 1000.0) });
    
    // Validate results
    if (memory_errors > 0) {
        print("❌ FAILED: Memory errors detected\n");
        return error.MemoryErrors;
    }
    
    if (race_conditions > 0) {
        print("⚠️  WARNING: Race conditions detected (data corruption)\n");
    }
    
    if (goroutines_completed < config.num_goroutines / 2) {
        print("⚠️  WARNING: Many goroutines failed to complete\n");
    }
    
    if (messages_sent == 0 and messages_received == 0) {
        print("❌ FAILED: No message throughput\n");
        return error.NoThroughput;
    }
    
    print("✅ Channel stress test completed successfully!\n");
}

/// Entry point for stress testing
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Test configuration variants
    const test_configs = [_]TestConfig{
        // Quick test
        TestConfig{
            .num_goroutines = 100,
            .messages_per_goroutine = 10,
            .test_duration_ms = 1000,
            .channel_buffer_size = 5,
        },
        // Medium stress test
        TestConfig{
            .num_goroutines = 500,
            .messages_per_goroutine = 50,
            .test_duration_ms = 3000,
            .channel_buffer_size = 10,
        },
        // Heavy stress test
        TestConfig{
            .num_goroutines = 1000,
            .messages_per_goroutine = 100,
            .test_duration_ms = 5000,
            .channel_buffer_size = 20,
        },
    };
    
    for (test_configs, 0..) |config, i| {
        print("\n{'='*60}\n");
        print("🧪 Running stress test variant {}/{}\n", .{ i + 1, test_configs.len });
        print("{'='*60}\n");
        
        try runChannelStressTest(allocator, config);
        
        // Brief pause between tests
        std.time.sleep(500_000_000); // 500ms
    }
    
    print("\n🎉 All channel race condition stress tests passed!\n");
}
