const std = @import("std");
const concurrency_fixed = @import("src-zig/concurrency_fixed.zig");
const print = std.debug.print;

test "channel race condition fixes" {
    const allocator = std.testing.allocator;
    
    // Test concurrent channel operations
    var channel = try concurrency_fixed.Channel(i32).init(allocator, 10);
    defer channel.deinit();
    
    const num_threads = 4;
    const messages_per_thread = 100;
    
    var threads: [num_threads]std.Thread = undefined;
    
    // Producer threads
    for (0..num_threads/2) |i| {
        threads[i] = try std.Thread.spawn(.{}, producer, .{ &channel, i, messages_per_thread });
    }
    
    // Consumer threads  
    for (num_threads/2..num_threads) |i| {
        threads[i] = try std.Thread.spawn(.{}, consumer, .{ &channel, i, messages_per_thread });
    }
    
    // Wait for all threads to complete
    for (threads) |thread| {
        thread.join();
    }
    
    print("Race condition test completed successfully\n", .{});
}

fn producer(channel: *concurrency_fixed.Channel(i32), thread_id: usize, count: usize) void {
    channel.addRef(); // Increment reference count
    defer channel.releaseRef(); // Decrement when done
    
    for (0..count) |i| {
        const value = @as(i32, @intCast(thread_id * 1000 + i));
        
        // Try to send with timeout
        const result = channel.sendTimeout(value, 100_000_000) catch { // 100ms timeout
            print("Send error for thread {}\n", .{thread_id});
            continue;
        };
        
        if (result != .sent) {
            print("Failed to send value {} from thread {}\n", .{ value, thread_id });
        }
        
        // Small delay to encourage interleaving
        std.time.sleep(1000); // 1μs
    }
}

fn consumer(channel: *concurrency_fixed.Channel(i32), thread_id: usize, count: usize) void {
    channel.addRef(); // Increment reference count
    defer channel.releaseRef(); // Decrement when done
    
    var received_count: usize = 0;
    
    while (received_count < count) {
        // Try to receive with timeout
        const result = channel.receiveTimeout(100_000_000) catch { // 100ms timeout
            print("Receive error for thread {}\n", .{thread_id});
            continue;
        };
        
        if (result) |value| {
            print("Thread {} received: {}\n", .{ thread_id, value });
            received_count += 1;
        } else {
            // Channel closed or timeout
            break;
        }
        
        // Small delay to encourage interleaving
        std.time.sleep(1000); // 1μs
    }
}

test "goroutine cleanup race conditions" {
    const allocator = std.testing.allocator;
    
    var runtime = try concurrency_fixed.ConcurrencyRuntime.init(allocator);
    defer runtime.deinit();
    
    const num_goroutines = 10;
    var goroutine_ids: [num_goroutines]concurrency_fixed.GoroutineId = undefined;
    
    // Spawn multiple goroutines
    for (0..num_goroutines) |i| {
        const context_value = try allocator.create(usize);
        context_value.* = i;
        
        goroutine_ids[i] = try runtime.spawnGoroutine(@constCast(@ptrCast(&testGoroutineFunc)), context_value);
    }
    
    // Wait for goroutines to complete
    std.time.sleep(100_000_000); // 100ms
    
    print("Goroutine cleanup test completed\n", .{});
}

fn testGoroutineFunc(context: ?*anyopaque) void {
    const value_ptr: *usize = @ptrCast(@alignCast(context.?));
    const value = value_ptr.*;
    
    print("Goroutine {} executing\n", .{value});
    
    // Simulate some work
    std.time.sleep(10_000_000); // 10ms
    
    print("Goroutine {} completing\n", .{value});
    
    // Cleanup context
    std.heap.c_allocator.destroy(value_ptr);
}

test "channel buffer race conditions" {
    const allocator = std.testing.allocator;
    
    // Test with small buffer to force race conditions
    var channel = try concurrency_fixed.Channel(i32).init(allocator, 2);
    defer channel.deinit();
    
    const num_operations = 1000;
    
    // Spawn producer and consumer in separate threads
    var producer_thread = try std.Thread.spawn(.{}, rapidProducer, .{ &channel, num_operations });
    var consumer_thread = try std.Thread.spawn(.{}, rapidConsumer, .{ &channel, num_operations });
    
    producer_thread.join();
    consumer_thread.join();
    
    print("Buffer race condition test completed\n", .{});
}

fn rapidProducer(channel: *concurrency_fixed.Channel(i32), count: usize) void {
    channel.addRef();
    defer channel.releaseRef();
    
    for (0..count) |i| {
        const value = @as(i32, @intCast(i));
        
        while (true) {
            const result = channel.sendTimeout(value, 1_000_000) catch break; // 1ms timeout
            if (result == .sent) {
                break;
            }
            // Retry on timeout or would_block
        }
    }
}

fn rapidConsumer(channel: *concurrency_fixed.Channel(i32), count: usize) void {
    channel.addRef();
    defer channel.releaseRef();
    
    var received_count: usize = 0;
    
    while (received_count < count) {
        const result = channel.receiveTimeout(1_000_000) catch break; // 1ms timeout
        if (result != null) {
            received_count += 1;
        }
    }
    
    print("Rapid consumer received {} items\n", .{received_count});
}
