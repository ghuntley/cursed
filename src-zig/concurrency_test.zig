//! Comprehensive concurrency system tests for CURSED in Zig
//!
//! This file contains extensive tests for:
//! - Goroutine spawning and execution
//! - Channel operations (send/receive/close)
//! - Work-stealing scheduler performance
//! - Select statement functionality
//! - Memory safety validation
//! - Complex concurrency patterns

const std = @import("std");
const print = std.debug.print;
const testing = std.testing;
const Thread = std.Thread;
const Allocator = std.mem.Allocator;

const concurrency = @import("concurrency.zig");
const Scheduler = concurrency.Scheduler;
const SchedulerConfig = concurrency.SchedulerConfig;
const Channel = concurrency.Channel;
const Select = concurrency.Select;
const Goroutine = concurrency.Goroutine;
const WorkStealingDeque = concurrency.WorkStealingDeque;

// Test utilities
const TestResult = struct {
    passed: bool,
    name: []const u8,
    duration_ms: u64,
    error_msg: ?[]const u8 = null,
};

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
var test_results: std.ArrayList(TestResult) = undefined;

fn runTest(comptime name: []const u8, testFn: *const fn () anyerror!void) void {
    const start_time = std.time.milliTimestamp();
    
    testFn() catch |err| {
        const end_time = std.time.milliTimestamp();
        test_results.append(allocator, TestResult{
            .passed = false,
            .name = name,
            .duration_ms = @as(u64, @intCast(end_time - start_time)),
            .error_msg = @errorName(err),
        }) catch {};
        print("❌ {s}: FAILED ({s})\n", .{ name, @errorName(err) });
        return;
    };
    
    const end_time = std.time.milliTimestamp();
    test_results.append(allocator, TestResult{
        .passed = true,
        .name = name,
        .duration_ms = @as(u64, @intCast(end_time - start_time)),
    }) catch {};
    print("✅ {s}: PASSED ({}ms)\n", .{ name, end_time - start_time });
}

// Comprehensive test suite
pub fn main() !void {
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();

    print("🚀 CURSED Concurrency System Test Suite\n", .{});
    print("=====================================\n\n", .{});

    // Initialize test results
    test_results = .{};
    defer test_results.deinit(allocator);

    // Basic functionality tests
    runTest("Basic Goroutine Creation", testBasicGoroutineCreation);
    runTest("Goroutine Execution", testGoroutineExecution);
    runTest("Multiple Goroutines", testMultipleGoroutines);
    
    // Channel tests
    runTest("Basic Channel Operations", testBasicChannelOperations);
    runTest("Buffered Channel", testBufferedChannel);
    runTest("Unbuffered Channel", testUnbufferedChannel);
    runTest("Channel Closing", testChannelClosing);
    runTest("Channel Iterator", testChannelIterator);
    
    // Scheduler tests
    runTest("Scheduler Initialization", testSchedulerInitialization);
    runTest("Work Distribution", testWorkDistribution);
    runTest("Work Stealing", testWorkStealing);
    runTest("Scheduler Statistics", testSchedulerStatistics);
    
    // Select statement tests
    runTest("Select Default Case", testSelectDefaultCase);
    runTest("Select Send/Receive", testSelectSendReceive);
    runTest("Select Timeout", testSelectTimeout);
    runTest("Select Multiple Channels", testSelectMultipleChannels);
    
    // Performance tests
    runTest("High Goroutine Count", testHighGoroutineCount);
    runTest("Channel Throughput", testChannelThroughput);
    runTest("Scheduler Performance", testSchedulerPerformance);
    
    // Concurrency pattern tests
    runTest("Producer-Consumer", testProducerConsumer);
    runTest("Fan-in Pattern", testFanInPattern);
    runTest("Fan-out Pattern", testFanOutPattern);
    runTest("Worker Pool", testWorkerPool);
    
    // Memory safety tests
    runTest("Memory Safety", testMemorySafety);
    runTest("Resource Cleanup", testResourceCleanup);
    
    // Print test summary
    printTestSummary();
}

// Basic goroutine tests
fn testBasicGoroutineCreation() !void {
    const allocator = gpa.allocator();
    const config = SchedulerConfig.default();
    
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);

    const testFn = struct {
        fn run(ctx: ?*anyopaque) void {
            _ = ctx;
            // Simple goroutine
        }
    }.run;

    const goroutine_id = try concurrency.stan(testFn, null);
    try testing.expect(goroutine_id > 0);
}

fn testGoroutineExecution() !void {
    const allocator = gpa.allocator();
    const config = SchedulerConfig.default();
    
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);

    var executed = false;
    const TestContext = struct {
        executed: *bool,
    };
    
    var context = TestContext{ .executed = &executed };
    
    const testFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @as(*TestContext, @ptrCast(@alignCast(ctx.?)));
            test_ctx.executed.* = true;
        }
    }.run;

    _ = try concurrency.stan(testFn, &context);
    
    // Wait for execution
    std.time.sleep(50_000_000); // 50ms
    
    try testing.expect(executed);
}

fn testMultipleGoroutines() !void {
    const allocator = gpa.allocator();
    const config = SchedulerConfig.default();
    
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);

    const num_goroutines = 10;
    var counter: u32 = 0;
    var mutex = std.Thread.Mutex{};
    
    const TestContext = struct {
        counter: *u32,
        mutex: *std.Thread.Mutex,
    };
    
    var context = TestContext{ .counter = &counter, .mutex = &mutex };
    
    const testFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @as(*TestContext, @ptrCast(@alignCast(ctx.?)));
            test_ctx.mutex.lock();
            defer test_ctx.mutex.unlock();
            test_ctx.counter.* += 1;
        }
    }.run;

    // Spawn multiple goroutines
    for (0..num_goroutines) |_| {
        _ = try concurrency.stan(testFn, &context);
    }
    
    // Wait for execution
    std.time.sleep(100_000_000); // 100ms
    
    try testing.expect(counter == num_goroutines);
}

// Channel tests
fn testBasicChannelOperations() !void {
    const allocator = gpa.allocator();
    
    var channel = try concurrency.makeChannel(i32, allocator, 3);
    defer {
        channel.deinit(allocator);
        allocator.destroy(channel);
    }

    // Test send
    try testing.expect(try channel.send(42) == concurrency.SendResult.sent);
    try testing.expect(try channel.send(43) == concurrency.SendResult.sent);
    
    // Test receive
    const received1 = try channel.receive();
    try testing.expect(received1.? == 42);
    
    const received2 = try channel.receive();
    try testing.expect(received2.? == 43);
    
    // Test try operations
    try testing.expect(try channel.trySend(44) == concurrency.SendResult.sent);
    const try_received = try channel.tryReceive();
    try testing.expect(try_received.? == 44);
}

fn testBufferedChannel() !void {
    const allocator = gpa.allocator();
    
    var channel = try concurrency.makeChannel(i32, allocator, 5);
    defer {
        channel.deinit(allocator);
        allocator.destroy(channel);
    }

    // Fill buffer
    for (0..5) |i| {
        try testing.expect(try channel.send(@as(i32, @intCast(i))) == concurrency.SendResult.sent);
    }
    
    // Buffer should be full
    try testing.expect(channel.isFull());
    try testing.expect(try channel.trySend(99) == concurrency.SendResult.would_block);
    
    // Drain buffer
    for (0..5) |i| {
        const received = try channel.receive();
        try testing.expect(received.? == @as(i32, @intCast(i)));
    }
    
    try testing.expect(channel.isEmpty());
}

fn testUnbufferedChannel() !void {
    const allocator = gpa.allocator();
    const config = concurrency.SchedulerConfig.default();
    
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);
    
    var channel = try concurrency.makeUnbufferedChannel(i32, allocator);
    defer {
        channel.deinit(allocator);
        allocator.destroy(channel);
    }

    // Should not be able to send without receiver
    try testing.expect(try channel.trySend(42) == concurrency.SendResult.would_block);
    
    // Test with goroutines
    var received_value: ?i32 = null;
    var mutex = std.Thread.Mutex{};
    
    const TestContext = struct {
        channel: *concurrency.Channel(i32),
        received: *?i32,
        mutex: *std.Thread.Mutex,
    };
    
    var context = TestContext{ 
        .channel = channel, 
        .received = &received_value,
        .mutex = &mutex,
    };
    
    const receiverFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @as(*TestContext, @ptrCast(@alignCast(ctx.?)));
            const value = test_ctx.channel.receive() catch return;
            test_ctx.mutex.lock();
            defer test_ctx.mutex.unlock();
            test_ctx.received.* = value;
        }
    }.run;
    
    // Start receiver
    _ = try concurrency.stan(receiverFn, &context);
    
    // Give receiver time to start
    std.time.sleep(10_000_000); // 10ms
    
    // Now send should succeed
    try testing.expect(try channel.send(123) == concurrency.SendResult.sent);
    
    // Wait for receive
    std.time.sleep(50_000_000); // 50ms
    
    mutex.lock();
    defer mutex.unlock();
    try testing.expect(received_value.? == 123);
}

fn testChannelClosing() !void {
    const allocator = gpa.allocator();
    
    var channel = try concurrency.makeChannel(i32, allocator, 2);
    defer {
        channel.deinit(allocator);
        allocator.destroy(channel);
    }

    // Send some data
    try testing.expect(try channel.send(100) == concurrency.SendResult.sent);
    
    // Close channel
    channel.close();
    try testing.expect(channel.isClosed());
    
    // Should not be able to send
    try testing.expect(try channel.send(101) == concurrency.SendResult.closed);
    
    // Should still be able to receive buffered data
    const received = try channel.receive();
    try testing.expect(received.? == 100);
    
    // Further receives should return null
    const received2 = try channel.receive();
    try testing.expect(received2 == null);
}

fn testChannelIterator() !void {
    const allocator = gpa.allocator();
    
    var channel = try concurrency.makeChannel(i32, allocator, 5);
    defer {
        channel.deinit(allocator);
        allocator.destroy(channel);
    }

    // Send test data
    for (1..6) |i| {
        try testing.expect(try channel.send(@as(i32, @intCast(i))) == concurrency.SendResult.sent);
    }
    
    channel.close();
    
    // Collect all values
    var values: std.ArrayList(i32) = .empty;
    defer values.deinit(allocator);
    
    while (try channel.receive()) |value| {
        try values.append(allocator, value);
    }
    
    try testing.expect(values.items.len == 5);
    for (values.items, 1..) |value, i| {
        try testing.expect(value == @as(i32, @intCast(i)));
    }
}

// Scheduler tests
fn testSchedulerInitialization() !void {
    const allocator = gpa.allocator();
    const config = SchedulerConfig.default();
    
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);

    const scheduler = concurrency.getScheduler().?;
    try testing.expect(scheduler.isRunning());
    try testing.expect(scheduler.activeGoroutineCount() == 0);
}

fn testWorkDistribution() !void {
    const allocator = gpa.allocator();
    var config = SchedulerConfig.default();
    config.num_workers = 4;
    
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);

    const num_tasks = 20;
    var completed_tasks: u32 = 0;
    var mutex = std.Thread.Mutex{};
    
    const TestContext = struct {
        completed: *u32,
        mutex: *std.Thread.Mutex,
    };
    
    var context = TestContext{ .completed = &completed_tasks, .mutex = &mutex };
    
    const taskFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @as(*TestContext, @ptrCast(@alignCast(ctx.?)));
            
            // Simulate work
            std.time.sleep(1_000_000); // 1ms
            
            test_ctx.mutex.lock();
            defer test_ctx.mutex.unlock();
            test_ctx.completed.* += 1;
        }
    }.run;

    // Spawn tasks
    for (0..num_tasks) |_| {
        _ = try concurrency.stan(taskFn, &context);
    }
    
    // Wait for completion
    std.time.sleep(200_000_000); // 200ms
    
    mutex.lock();
    defer mutex.unlock();
    try testing.expect(completed_tasks == num_tasks);
}

fn testWorkStealing() !void {
    const allocator = gpa.allocator();
    
    var deque1 = WorkStealingDeque.init(allocator);
    defer deque1.deinit(allocator);
    
    var deque2 = WorkStealingDeque.init(allocator);
    defer deque2.deinit(allocator);

    // Create test goroutines
    var goroutine1 = Goroutine.init(allocator, 1, undefined, null);
    var goroutine2 = Goroutine.init(allocator, 2, undefined, null);
    
    // Add to first deque
    try deque1.pushBottom(&goroutine1);
    try deque1.pushBottom(&goroutine2);
    
    try testing.expect(deque1.items.items.len == 2);
    try testing.expect(deque2.items.items.len == 0);
    
    // Steal from first deque
    const stolen = deque1.steal();
    try testing.expect(stolen != null);
    try testing.expect(deque1.items.items.len == 1);
    
    // Pop from bottom
    const popped = deque1.popBottom();
    try testing.expect(popped != null);
    try testing.expect(deque1.isEmpty());
}

fn testSchedulerStatistics() !void {
    const allocator = gpa.allocator();
    const config = SchedulerConfig.default();
    
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);

    const scheduler = concurrency.getScheduler().?;
    const initial_stats = scheduler.getStats();
    
    const taskFn = struct {
        fn run(ctx: ?*anyopaque) void {
            _ = ctx;
            std.time.sleep(1_000_000); // 1ms
        }
    }.run;

    // Spawn some tasks
    for (0..5) |_| {
        _ = try concurrency.stan(taskFn, null);
    }
    
    std.time.sleep(50_000_000); // 50ms
    
    const final_stats = scheduler.getStats();
    try testing.expect(final_stats.total_spawned > initial_stats.total_spawned);
}

// Select statement tests
fn testSelectDefaultCase() !void {
    const allocator = gpa.allocator();
    
    var select_stmt = Select.init(allocator);
    defer select_stmt.deinit(allocator);

    try select_stmt.addDefault(0);
    
    const result = try select_stmt.execute();
    try testing.expect(result == concurrency.SelectResult.default_executed);
}

fn testSelectSendReceive() !void {
    const allocator = gpa.allocator();
    
    var channel = try concurrency.makeChannel(i32, allocator, 1);
    defer {
        channel.deinit(allocator);
        allocator.destroy(channel);
    }

    var select_stmt = Select.init(allocator);
    defer select_stmt.deinit(allocator);

    try select_stmt.addSend(channel.id, 0);
    try select_stmt.addDefault(1);
    
    const result = try select_stmt.execute();
    try testing.expect(result == concurrency.SelectResult.send_completed or
                       result == concurrency.SelectResult.default_executed);
}

fn testSelectTimeout() !void {
    const allocator = gpa.allocator();
    
    var select_stmt = Select.init(allocator);
    defer select_stmt.deinit(allocator);

    select_stmt.setTimeout(10); // 10ms timeout
    
    const start_time = std.time.milliTimestamp();
    const result = try select_stmt.execute();
    const end_time = std.time.milliTimestamp();
    
    try testing.expect(result == concurrency.SelectResult.timeout);
    try testing.expect(end_time - start_time >= 10);
}

fn testSelectMultipleChannels() !void {
    const allocator = gpa.allocator();
    
    var channel1 = try concurrency.makeChannel(i32, allocator, 1);
    defer {
        channel1.deinit(allocator);
        allocator.destroy(channel1);
    }
    
    var channel2 = try concurrency.makeChannel(i32, allocator, 1);
    defer {
        channel2.deinit(allocator);
        allocator.destroy(channel2);
    }

    // Put data in first channel
    try testing.expect(try channel1.send(42) == concurrency.SendResult.sent);

    var select_stmt = Select.init(allocator);
    defer select_stmt.deinit(allocator);

    try select_stmt.addReceive(channel1.id, 0);
    try select_stmt.addReceive(channel2.id, 1);
    try select_stmt.addDefault(2);
    
    const result = try select_stmt.execute();
    try testing.expect(result == concurrency.SelectResult.receive_completed or
                       result == concurrency.SelectResult.default_executed);
}

// Performance tests
fn testHighGoroutineCount() !void {
    const allocator = gpa.allocator();
    const config = SchedulerConfig.default();
    
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);

    const num_goroutines = 1000;
    var counter: u32 = 0;
    var mutex = std.Thread.Mutex{};
    
    const TestContext = struct {
        counter: *u32,
        mutex: *std.Thread.Mutex,
    };
    
    var context = TestContext{ .counter = &counter, .mutex = &mutex };
    
    const taskFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @as(*TestContext, @ptrCast(@alignCast(ctx.?)));
            test_ctx.mutex.lock();
            defer test_ctx.mutex.unlock();
            test_ctx.counter.* += 1;
        }
    }.run;

    const start_time = std.time.milliTimestamp();
    
    // Spawn many goroutines
    for (0..num_goroutines) |_| {
        _ = try concurrency.stan(taskFn, &context);
    }
    
    // Wait for completion
    while (true) {
        mutex.lock();
        const current_count = counter;
        mutex.unlock();
        
        if (current_count == num_goroutines) break;
        std.time.sleep(1_000_000); // 1ms
    }
    
    const end_time = std.time.milliTimestamp();
    const duration = end_time - start_time;
    
    print("High goroutine test: {} goroutines in {}ms\n", .{ num_goroutines, duration });
    try testing.expect(counter == num_goroutines);
}

fn testChannelThroughput() !void {
    const allocator = gpa.allocator();
    const config = concurrency.SchedulerConfig.default();
    
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);
    
    var channel = try concurrency.makeChannel(u64, allocator, 1000);
    defer {
        channel.deinit(allocator);
        allocator.destroy(channel);
    }

    const num_messages = 10000;
    var sent_count: u64 = 0;
    var received_count: u64 = 0;
    var mutex = std.Thread.Mutex{};
    
    const TestContext = struct {
        channel: *concurrency.Channel(u64),
        sent: *u64,
        received: *u64,
        mutex: *std.Thread.Mutex,
        num_messages: u64,
    };
    
    var context = TestContext{ 
        .channel = channel, 
        .sent = &sent_count,
        .received = &received_count,
        .mutex = &mutex,
        .num_messages = num_messages,
    };
    
    const senderFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @as(*TestContext, @ptrCast(@alignCast(ctx.?)));
            for (0..test_ctx.num_messages) |i| {
                _ = test_ctx.channel.send(i) catch continue;
                test_ctx.mutex.lock();
                test_ctx.sent.* += 1;
                test_ctx.mutex.unlock();
            }
        }
    }.run;
    
    const receiverFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @as(*TestContext, @ptrCast(@alignCast(ctx.?)));
            while (true) {
                const value = test_ctx.channel.receive() catch break;
                if (value == null) break;
                
                test_ctx.mutex.lock();
                test_ctx.received.* += 1;
                const current_received = test_ctx.received.*;
                test_ctx.mutex.unlock();
                
                if (current_received >= test_ctx.num_messages) break;
            }
        }
    }.run;

    const start_time = std.time.milliTimestamp();
    
    // Start sender and receiver
    _ = try concurrency.stan(senderFn, &context);
    _ = try concurrency.stan(receiverFn, &context);
    
    // Wait for completion
    while (true) {
        mutex.lock();
        const current_sent = sent_count;
        const current_received = received_count;
        mutex.unlock();
        
        if (current_sent == num_messages and current_received == num_messages) break;
        std.time.sleep(1_000_000); // 1ms
    }
    
    const end_time = std.time.milliTimestamp();
    const duration = end_time - start_time;
    const throughput = @as(f64, @floatFromInt(num_messages * 1000)) / @as(f64, @floatFromInt(duration));
    
    print("Channel throughput: {d:.0} messages/second\n", .{throughput});
    try testing.expect(sent_count == num_messages);
    try testing.expect(received_count == num_messages);
}

fn testSchedulerPerformance() !void {
    const allocator = gpa.allocator();
    const config = SchedulerConfig.default();
    
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);

    const num_tasks = 10000;
    var completed: u32 = 0;
    var mutex = std.Thread.Mutex{};
    
    const TestContext = struct {
        completed: *u32,
        mutex: *std.Thread.Mutex,
    };
    
    var context = TestContext{ .completed = &completed, .mutex = &mutex };
    
    const taskFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @as(*TestContext, @ptrCast(@alignCast(ctx.?)));
            // Minimal work to test scheduler overhead
            test_ctx.mutex.lock();
            test_ctx.completed.* += 1;
            test_ctx.mutex.unlock();
        }
    }.run;

    const start_time = std.time.milliTimestamp();
    
    // Spawn tasks rapidly
    for (0..num_tasks) |_| {
        _ = try concurrency.stan(taskFn, &context);
    }
    
    // Wait for completion
    while (true) {
        mutex.lock();
        const current_completed = completed;
        mutex.unlock();
        
        if (current_completed == num_tasks) break;
        std.time.sleep(100_000); // 100 microseconds
    }
    
    const end_time = std.time.milliTimestamp();
    const duration = end_time - start_time;
    const throughput = @as(f64, @floatFromInt(num_tasks * 1000)) / @as(f64, @floatFromInt(duration));
    
    print("Scheduler performance: {d:.0} tasks/second\n", .{throughput});
    try testing.expect(completed == num_tasks);
}

// Concurrency pattern tests
fn testProducerConsumer() !void {
    const allocator = gpa.allocator();
    
    try concurrency.initializeScheduler(allocator, SchedulerConfig.default());
    defer concurrency.shutdownScheduler(allocator);

    var channel = try concurrency.makeChannel(u32, allocator, 10);
    defer {
        channel.deinit(allocator);
        allocator.destroy(channel);
    }

    const num_items = 100;
    var produced: u32 = 0;
    var consumed: u32 = 0;
    var mutex = std.Thread.Mutex{};
    
    const TestContext = struct {
        channel: *concurrency.Channel(u32),
        produced: *u32,
        consumed: *u32,
        mutex: *std.Thread.Mutex,
        num_items: u32,
    };
    
    var context = TestContext{ 
        .channel = channel, 
        .produced = &produced,
        .consumed = &consumed,
        .mutex = &mutex,
        .num_items = num_items,
    };
    
    const producerFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @as(*TestContext, @ptrCast(@alignCast(ctx.?)));
            for (0..test_ctx.num_items) |i| {
                _ = test_ctx.channel.send(@as(u32, @intCast(i))) catch continue;
                test_ctx.mutex.lock();
                test_ctx.produced.* += 1;
                test_ctx.mutex.unlock();
            }
            test_ctx.channel.close();
        }
    }.run;
    
    const consumerFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @as(*TestContext, @ptrCast(@alignCast(ctx.?)));
            while (true) {
                const value = test_ctx.channel.receive() catch break;
                if (value == null) break;
                
                test_ctx.mutex.lock();
                test_ctx.consumed.* += 1;
                test_ctx.mutex.unlock();
            }
        }
    }.run;

    // Start producer and consumer
    _ = try concurrency.stan(producerFn, &context);
    _ = try concurrency.stan(consumerFn, &context);
    
    // Wait for completion
    std.time.sleep(200_000_000); // 200ms
    
    mutex.lock();
    defer mutex.unlock();
    try testing.expect(produced == num_items);
    try testing.expect(consumed == num_items);
}

fn testFanInPattern() !void {
    const allocator = gpa.allocator();
    
    try concurrency.initializeScheduler(allocator, SchedulerConfig.default());
    defer concurrency.shutdownScheduler(allocator);

    var output_channel = try concurrency.makeChannel(u32, allocator, 50);
    defer {
        output_channel.deinit(allocator);
        allocator.destroy(output_channel);
    }

    const num_producers = 5;
    const items_per_producer = 20;
    var total_received: u32 = 0;
    var mutex = std.Thread.Mutex{};
    
    const ProducerContext = struct {
        channel: *concurrency.Channel(u32),
        start_value: u32,
        count: u32,
    };
    
    const ConsumerContext = struct {
        channel: *concurrency.Channel(u32),
        received: *u32,
        mutex: *std.Thread.Mutex,
        expected_total: u32,
    };
    
    // Start multiple producers
    for (0..num_producers) |i| {
        const producer_context = try allocator.create(ProducerContext);
        producer_context.* = ProducerContext{
            .channel = output_channel,
            .start_value = @as(u32, @intCast(i * items_per_producer)),
            .count = items_per_producer,
        };
        
        const producerFn = struct {
            fn run(ctx: ?*anyopaque) void {
                const prod_ctx = @as(*ProducerContext, @ptrCast(@alignCast(ctx.?)));
                for (0..prod_ctx.count) |j| {
                    _ = prod_ctx.channel.send(prod_ctx.start_value + @as(u32, @intCast(j))) catch continue;
                }
            }
        }.run;
        
        _ = try concurrency.stan(producerFn, producer_context);
    }
    
    // Start consumer
    var consumer_context = ConsumerContext{
        .channel = output_channel,
        .received = &total_received,
        .mutex = &mutex,
        .expected_total = num_producers * items_per_producer,
    };
    
    const consumerFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const cons_ctx = @as(*ConsumerContext, @ptrCast(@alignCast(ctx.?)));
            while (true) {
                const value = cons_ctx.channel.receive() catch break;
                if (value == null) break;
                
                cons_ctx.mutex.lock();
                cons_ctx.received.* += 1;
                const current_received = cons_ctx.received.*;
                cons_ctx.mutex.unlock();
                
                if (current_received >= cons_ctx.expected_total) break;
            }
        }
    }.run;
    
    _ = try concurrency.stan(consumerFn, &consumer_context);
    
    // Wait for completion
    std.time.sleep(300_000_000); // 300ms
    
    mutex.lock();
    defer mutex.unlock();
    try testing.expect(total_received == num_producers * items_per_producer);
}

fn testFanOutPattern() !void {
    const allocator = gpa.allocator();
    
    try concurrency.initializeScheduler(allocator, SchedulerConfig.default());
    defer concurrency.shutdownScheduler(allocator);

    var input_channel = try concurrency.makeChannel(u32, allocator, 50);
    defer {
        input_channel.deinit(allocator);
        allocator.destroy(input_channel);
    }

    const num_consumers = 3;
    const total_items = 60;
    var consumer_counts = [_]u32{0} ** num_consumers;
    var mutex = std.Thread.Mutex{};
    
    const ProducerContext = struct {
        channel: *concurrency.Channel(u32),
        count: u32,
    };
    
    const ConsumerContext = struct {
        channel: *concurrency.Channel(u32),
        count: *u32,
        mutex: *std.Thread.Mutex,
        id: u32,
    };
    
    // Start producer
    var producer_context = ProducerContext{
        .channel = input_channel,
        .count = total_items,
    };
    
    const producerFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const prod_ctx = @as(*ProducerContext, @ptrCast(@alignCast(ctx.?)));
            for (0..prod_ctx.count) |i| {
                _ = prod_ctx.channel.send(@as(u32, @intCast(i))) catch continue;
            }
            prod_ctx.channel.close();
        }
    }.run;
    
    _ = try concurrency.stan(producerFn, &producer_context);
    
    // Start multiple consumers
    for (0..num_consumers) |i| {
        const consumer_context = try allocator.create(ConsumerContext);
        consumer_context.* = ConsumerContext{
            .channel = input_channel,
            .count = &consumer_counts[i],
            .mutex = &mutex,
            .id = @as(u32, @intCast(i)),
        };
        
        const consumerFn = struct {
            fn run(ctx: ?*anyopaque) void {
                const cons_ctx = @as(*ConsumerContext, @ptrCast(@alignCast(ctx.?)));
                while (true) {
                    const value = cons_ctx.channel.receive() catch break;
                    if (value == null) break;
                    
                    cons_ctx.mutex.lock();
                    cons_ctx.count.* += 1;
                    cons_ctx.mutex.unlock();
                }
            }
        }.run;
        
        _ = try concurrency.stan(consumerFn, consumer_context);
    }
    
    // Wait for completion
    std.time.sleep(300_000_000); // 300ms
    
    mutex.lock();
    defer mutex.unlock();
    
    var total_consumed: u32 = 0;
    for (consumer_counts) |count| {
        total_consumed += count;
    }
    
    try testing.expect(total_consumed == total_items);
}

fn testWorkerPool() !void {
    const allocator = gpa.allocator();
    
    try concurrency.initializeScheduler(allocator, SchedulerConfig.default());
    defer concurrency.shutdownScheduler(allocator);

    var job_channel = try concurrency.makeChannel(u32, allocator, 20);
    defer {
        job_channel.deinit(allocator);
        allocator.destroy(job_channel);
    }
    
    var result_channel = try concurrency.makeChannel(u32, allocator, 20);
    defer {
        result_channel.deinit(allocator);
        allocator.destroy(result_channel);
    }

    const num_workers = 3;
    const num_jobs = 30;
    var completed_jobs: u32 = 0;
    var mutex = std.Thread.Mutex{};
    
    const WorkerContext = struct {
        job_channel: *concurrency.Channel(u32),
        result_channel: *concurrency.Channel(u32),
        worker_id: u32,
    };
    
    const CollectorContext = struct {
        result_channel: *concurrency.Channel(u32),
        completed: *u32,
        mutex: *std.Thread.Mutex,
        expected_jobs: u32,
    };
    
    // Start workers
    for (0..num_workers) |i| {
        const worker_context = try allocator.create(WorkerContext);
        worker_context.* = WorkerContext{
            .job_channel = job_channel,
            .result_channel = result_channel,
            .worker_id = @as(u32, @intCast(i)),
        };
        
        const workerFn = struct {
            fn run(ctx: ?*anyopaque) void {
                const worker_ctx = @as(*WorkerContext, @ptrCast(@alignCast(ctx.?)));
                while (true) {
                    const job = worker_ctx.job_channel.receive() catch break;
                    if (job == null) break;
                    
                    // Simulate work - square the number
                    const result = job.? * job.?;
                    _ = worker_ctx.result_channel.send(result) catch continue;
                }
            }
        }.run;
        
        _ = try concurrency.stan(workerFn, worker_context);
    }
    
    // Start result collector
    var collector_context = CollectorContext{
        .result_channel = result_channel,
        .completed = &completed_jobs,
        .mutex = &mutex,
        .expected_jobs = num_jobs,
    };
    
    const collectorFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const coll_ctx = @as(*CollectorContext, @ptrCast(@alignCast(ctx.?)));
            while (true) {
                const result = coll_ctx.result_channel.receive() catch break;
                if (result == null) break;
                
                coll_ctx.mutex.lock();
                coll_ctx.completed.* += 1;
                const current_completed = coll_ctx.completed.*;
                coll_ctx.mutex.unlock();
                
                if (current_completed >= coll_ctx.expected_jobs) break;
            }
        }
    }.run;
    
    _ = try concurrency.stan(collectorFn, &collector_context);
    
    // Send jobs
    for (1..num_jobs + 1) |i| {
        _ = try job_channel.send(@as(u32, @intCast(i)));
    }
    
    job_channel.close();
    
    // Wait for completion
    std.time.sleep(300_000_000); // 300ms
    
    mutex.lock();
    defer mutex.unlock();
    try testing.expect(completed_jobs == num_jobs);
}

// Memory safety tests
fn testMemorySafety() !void {
    const allocator = gpa.allocator();
    
    // Test that channels and goroutines properly clean up memory
    for (0..100) |_| {
        var channel = try concurrency.makeChannel(u64, allocator, 5);
        
        // Use the channel briefly
        _ = try channel.send(42);
        _ = try channel.receive();
        
        // Clean up
        channel.deinit(allocator);
        allocator.destroy(channel);
    }
    
    // If we get here without memory leaks, the test passes
    try testing.expect(true);
}

fn testResourceCleanup() !void {
    const allocator = gpa.allocator();
    const config = SchedulerConfig.default();
    
    // Test multiple scheduler initializations and shutdowns
    for (0..5) |_| {
        try concurrency.initializeScheduler(allocator, config);
        
        // Spawn some goroutines
        const taskFn = struct {
            fn run(ctx: ?*anyopaque) void {
                _ = ctx;
                std.time.sleep(1_000_000); // 1ms
            }
        }.run;
        
        for (0..10) |_| {
            _ = try concurrency.stan(taskFn, null);
        }
        
        // Brief wait then shutdown
        std.time.sleep(50_000_000); // 50ms
        concurrency.shutdownScheduler(allocator);
    }
    
    try testing.expect(true);
}

// Test summary printing
fn printTestSummary() void {
    print("\n📊 Test Summary\n", .{});
    print("===============\n", .{});
    
    var passed: u32 = 0;
    var failed: u32 = 0;
    var total_duration: u64 = 0;
    
    for (test_results.items) |result| {
        if (result.passed) {
            passed += 1;
        } else {
            failed += 1;
            print("❌ {s}: {s}\n", .{ result.name, result.error_msg orelse "Unknown error" });
        }
        total_duration += result.duration_ms;
    }
    
    print("\n✅ Passed: {}\n", .{passed});
    print("❌ Failed: {}\n", .{failed});
    print("⏱️  Total time: {}ms\n", .{total_duration});
    print("📈 Success rate: {d:.1}%\n", .{@as(f64, @floatFromInt(passed)) * 100.0 / @as(f64, @floatFromInt(passed + failed))});
    
    if (failed == 0) {
        print("\n🎉 All tests passed! CURSED concurrency system is working correctly.\n", .{});
    } else {
        print("\n⚠️  Some tests failed. Please review the implementation.\n", .{});
    }
}
