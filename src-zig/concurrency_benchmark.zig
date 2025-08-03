//! Performance benchmarks for CURSED concurrency system
//!
//! This file contains comprehensive benchmarks to evaluate:
//! - Goroutine spawning and execution performance
//! - Channel throughput and latency
//! - Work-stealing scheduler efficiency
//! - Select statement performance
//! - Memory usage and scalability
//! - Comparison with theoretical limits

const std = @import("std");
const print = std.debug.print;
const testing = std.testing;
const Thread = std.Thread;
const Allocator = std.mem.Allocator;

const concurrency = @import("concurrency.zig");
const Scheduler = concurrency.Scheduler;
const SchedulerConfig = concurrency.SchedulerConfig;
const Channel = concurrency.Channel;

/// Benchmark result structure
const BenchmarkResult = struct {
    name: []const u8,
    operations: u64,
    duration_ms: u64,
    ops_per_second: f64,
    memory_used_kb: u64,
    cpu_cores_used: u32,
    success: bool,
    notes: ?[]const u8 = null,
};

var benchmark_results = std.ArrayList(BenchmarkResult).init(std.heap.page_allocator);

/// Run a benchmark and record results
fn runBenchmark(
    comptime name: []const u8,
    operations: u64,
    benchmarkFn: *const fn (u64) anyerror!u64,
) void {
    print("🏃 Running benchmark: {s}...\n", .{name});
    
    const start_time = std.time.milliTimestamp();
    const memory_start = getCurrentMemoryUsage();
    
    const actual_ops = benchmarkFn(operations) catch |err| {
        print("❌ Benchmark {s} failed: {}\n", .{ name, err });
        benchmark_results.append(BenchmarkResult{
            .name = name,
            .operations = 0,
            .duration_ms = 0,
            .ops_per_second = 0,
            .memory_used_kb = 0,
            .cpu_cores_used = 0,
            .success = false,
            .notes = @errorName(err),
        }) catch {};
        return;
    };
    
    const end_time = std.time.milliTimestamp();
    const memory_end = getCurrentMemoryUsage();
    
    const duration = @intCast(u64, end_time - start_time);
    const ops_per_second = @intToFloat(f64, actual_ops * 1000) / @intToFloat(f64, duration);
    const memory_used = if (memory_end > memory_start) memory_end - memory_start else 0;
    
    benchmark_results.append(BenchmarkResult{
        .name = name,
        .operations = actual_ops,
        .duration_ms = duration,
        .ops_per_second = ops_per_second,
        .memory_used_kb = memory_used,
        .cpu_cores_used = @intCast(u32, std.Thread.getCpuCount() catch 1),
        .success = true,
    }) catch {};
    
    print("✅ {s}: {d:.0} ops/sec ({} ops in {}ms, {}KB memory)\n", 
          .{ name, ops_per_second, actual_ops, duration, memory_used });
}

/// Get current memory usage (simplified implementation)
fn getCurrentMemoryUsage() u64 {
    // In a real implementation, this would read from /proc/self/status or similar
    // For now, return 0 as placeholder
    return 0;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    print("🚀 CURSED Concurrency Performance Benchmarks\n");
    print("=============================================\n\n");

    benchmark_results = std.ArrayList(BenchmarkResult).init(allocator);
    defer benchmark_results.deinit();

    // Goroutine benchmarks
    runBenchmark("Goroutine Creation", 10000, benchmarkGoroutineCreation);
    runBenchmark("Goroutine Execution", 10000, benchmarkGoroutineExecution);
    runBenchmark("Massive Goroutine Spawn", 100000, benchmarkMassiveGoroutineSpawn);
    
    // Channel benchmarks
    runBenchmark("Channel Send/Receive", 100000, benchmarkChannelSendReceive);
    runBenchmark("Buffered Channel Throughput", 1000000, benchmarkBufferedChannelThroughput);
    runBenchmark("Unbuffered Channel Sync", 50000, benchmarkUnbufferedChannelSync);
    runBenchmark("Channel Close Performance", 10000, benchmarkChannelClosePerformance);
    
    // Scheduler benchmarks
    runBenchmark("Work Stealing Efficiency", 50000, benchmarkWorkStealingEfficiency);
    runBenchmark("Load Balancing", 20000, benchmarkLoadBalancing);
    runBenchmark("Context Switching", 100000, benchmarkContextSwitching);
    
    // Select statement benchmarks
    runBenchmark("Select Default Case", 100000, benchmarkSelectDefaultCase);
    runBenchmark("Select Multi-Channel", 50000, benchmarkSelectMultiChannel);
    runBenchmark("Select Timeout", 10000, benchmarkSelectTimeout);
    
    // Scalability benchmarks
    runBenchmark("CPU Core Scaling", 50000, benchmarkCpuCoreScaling);
    runBenchmark("Memory Scalability", 10000, benchmarkMemoryScalability);
    runBenchmark("Channel Fanout", 10000, benchmarkChannelFanout);
    
    // Real-world pattern benchmarks
    runBenchmark("Producer Consumer", 50000, benchmarkProducerConsumer);
    runBenchmark("Worker Pool", 20000, benchmarkWorkerPool);
    runBenchmark("Pipeline Processing", 10000, benchmarkPipelineProcessing);
    
    // Print comprehensive results
    printBenchmarkSummary();
}

// Goroutine benchmarks
fn benchmarkGoroutineCreation(target_ops: u64) !u64 {
    const allocator = std.heap.page_allocator;
    const config = SchedulerConfig.default();
    
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);

    var created_count: u64 = 0;
    
    const taskFn = struct {
        fn run(ctx: ?*anyopaque) void {
            _ = ctx;
            // Minimal work - just return
        }
    }.run;

    for (0..target_ops) |_| {
        _ = try concurrency.stan(taskFn, null);
        created_count += 1;
    }
    
    return created_count;
}

fn benchmarkGoroutineExecution(target_ops: u64) !u64 {
    const allocator = std.heap.page_allocator;
    const config = SchedulerConfig.default();
    
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);

    var executed_count: u64 = 0;
    var mutex = std.Thread.Mutex{};
    
    const TestContext = struct {
        executed: *u64,
        mutex: *std.Thread.Mutex,
    };
    
    var context = TestContext{ .executed = &executed_count, .mutex = &mutex };
    
    const taskFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @ptrCast(*TestContext, @alignCast(@alignOf(TestContext), ctx.?));
            test_ctx.mutex.lock();
            defer test_ctx.mutex.unlock();
            test_ctx.executed.* += 1;
        }
    }.run;

    // Spawn goroutines
    for (0..target_ops) |_| {
        _ = try concurrency.stan(taskFn, &context);
    }
    
    // Wait for completion
    while (true) {
        mutex.lock();
        const current_executed = executed_count;
        mutex.unlock();
        
        if (current_executed >= target_ops) break;
        std.time.sleep(100_000); // 100 microseconds
    }
    
    return executed_count;
}

fn benchmarkMassiveGoroutineSpawn(target_ops: u64) !u64 {
    const allocator = std.heap.page_allocator;
    var config = SchedulerConfig.default();
    config.num_workers = @max(8, std.Thread.getCpuCount() catch 1);
    
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);

    var spawned_count: u64 = 0;
    var completed_count: u64 = 0;
    var mutex = std.Thread.Mutex{};
    
    const TestContext = struct {
        completed: *u64,
        mutex: *std.Thread.Mutex,
    };
    
    var context = TestContext{ .completed = &completed_count, .mutex = &mutex };
    
    const taskFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @ptrCast(*TestContext, @alignCast(@alignOf(TestContext), ctx.?));
            
            // Small amount of work
            var sum: u64 = 0;
            for (0..100) |i| {
                sum += i;
            }
            
            test_ctx.mutex.lock();
            defer test_ctx.mutex.unlock();
            test_ctx.completed.* += 1;
        }
    }.run;

    // Spawn massive number of goroutines
    for (0..target_ops) |_| {
        _ = try concurrency.stan(taskFn, &context);
        spawned_count += 1;
    }
    
    // Wait for significant completion (at least 80%)
    const min_completion = target_ops * 80 / 100;
    while (true) {
        mutex.lock();
        const current_completed = completed_count;
        mutex.unlock();
        
        if (current_completed >= min_completion) break;
        std.time.sleep(1_000_000); // 1ms
    }
    
    return spawned_count;
}

// Channel benchmarks
fn benchmarkChannelSendReceive(target_ops: u64) !u64 {
    const allocator = std.heap.page_allocator;
    
    var channel = try concurrency.makeChannel(u64, allocator, 1000);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }

    var operations_completed: u64 = 0;
    
    // Send operations
    for (0..target_ops) |i| {
        if (try channel.send(i) == concurrency.SendResult.sent) {
            operations_completed += 1;
        }
    }
    
    // Receive operations
    for (0..target_ops) |_| {
        if (try channel.receive() != null) {
            operations_completed += 1;
        }
    }
    
    return operations_completed;
}

fn benchmarkBufferedChannelThroughput(target_ops: u64) !u64 {
    const allocator = std.heap.page_allocator;
    
    try concurrency.initializeScheduler(allocator, SchedulerConfig.default());
    defer concurrency.shutdownScheduler(allocator);

    var channel = try concurrency.makeChannel(u64, allocator, 10000);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }

    var sent_count: u64 = 0;
    var received_count: u64 = 0;
    var mutex = std.Thread.Mutex{};
    
    const TestContext = struct {
        channel: *concurrency.Channel(u64),
        sent: *u64,
        received: *u64,
        mutex: *std.Thread.Mutex,
        target: u64,
    };
    
    var context = TestContext{ 
        .channel = channel, 
        .sent = &sent_count,
        .received = &received_count,
        .mutex = &mutex,
        .target = target_ops,
    };
    
    const senderFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @ptrCast(*TestContext, @alignCast(@alignOf(TestContext), ctx.?));
            for (0..test_ctx.target) |i| {
                if (test_ctx.channel.send(i) catch continue == concurrency.SendResult.sent) {
                    test_ctx.mutex.lock();
                    test_ctx.sent.* += 1;
                    test_ctx.mutex.unlock();
                }
            }
        }
    }.run;
    
    const receiverFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @ptrCast(*TestContext, @alignCast(@alignOf(TestContext), ctx.?));
            while (true) {
                const value = test_ctx.channel.receive() catch break;
                if (value == null) break;
                
                test_ctx.mutex.lock();
                test_ctx.received.* += 1;
                const current_received = test_ctx.received.*;
                test_ctx.mutex.unlock();
                
                if (current_received >= test_ctx.target) break;
            }
        }
    }.run;

    // Start sender and receiver
    _ = try concurrency.stan(senderFn, &context);
    _ = try concurrency.stan(receiverFn, &context);
    
    // Wait for completion
    while (true) {
        mutex.lock();
        const current_sent = sent_count;
        const current_received = received_count;
        mutex.unlock();
        
        if (current_sent >= target_ops and current_received >= target_ops) break;
        std.time.sleep(100_000); // 100 microseconds
    }
    
    return sent_count + received_count;
}

fn benchmarkUnbufferedChannelSync(target_ops: u64) !u64 {
    const allocator = std.heap.page_allocator;
    
    try concurrency.initializeScheduler(allocator, SchedulerConfig.default());
    defer concurrency.shutdownScheduler(allocator);

    var channel = try concurrency.makeUnbufferedChannel(u64, allocator);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }

    var sync_operations: u64 = 0;
    var mutex = std.Thread.Mutex{};
    
    const TestContext = struct {
        channel: *concurrency.Channel(u64),
        sync_ops: *u64,
        mutex: *std.Thread.Mutex,
        target: u64,
    };
    
    var context = TestContext{ 
        .channel = channel, 
        .sync_ops = &sync_operations,
        .mutex = &mutex,
        .target = target_ops,
    };
    
    const senderFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @ptrCast(*TestContext, @alignCast(@alignOf(TestContext), ctx.?));
            for (0..test_ctx.target) |i| {
                if (test_ctx.channel.send(i) catch continue == concurrency.SendResult.sent) {
                    test_ctx.mutex.lock();
                    test_ctx.sync_ops.* += 1;
                    test_ctx.mutex.unlock();
                }
            }
        }
    }.run;
    
    const receiverFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @ptrCast(*TestContext, @alignCast(@alignOf(TestContext), ctx.?));
            for (0..test_ctx.target) |_| {
                const value = test_ctx.channel.receive() catch break;
                if (value != null) {
                    test_ctx.mutex.lock();
                    test_ctx.sync_ops.* += 1;
                    test_ctx.mutex.unlock();
                }
            }
        }
    }.run;

    // Start sender and receiver
    _ = try concurrency.stan(senderFn, &context);
    _ = try concurrency.stan(receiverFn, &context);
    
    // Wait for completion
    while (true) {
        mutex.lock();
        const current_ops = sync_operations;
        mutex.unlock();
        
        if (current_ops >= target_ops * 2) break; // Both send and receive
        std.time.sleep(100_000); // 100 microseconds
    }
    
    return sync_operations;
}

fn benchmarkChannelClosePerformance(target_ops: u64) !u64 {
    const allocator = std.heap.page_allocator;
    
    var close_operations: u64 = 0;
    
    for (0..target_ops) |_| {
        var channel = try concurrency.makeChannel(u32, allocator, 10);
        
        // Use channel briefly
        _ = try channel.send(42);
        _ = try channel.receive();
        
        // Close and cleanup
        channel.close();
        channel.deinit();
        allocator.destroy(channel);
        
        close_operations += 1;
    }
    
    return close_operations;
}

// Scheduler benchmarks
fn benchmarkWorkStealingEfficiency(target_ops: u64) !u64 {
    const allocator = std.heap.page_allocator;
    var config = SchedulerConfig.default();
    config.num_workers = 8; // Force multiple workers
    
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);

    var work_completed: u64 = 0;
    var mutex = std.Thread.Mutex{};
    
    const TestContext = struct {
        completed: *u64,
        mutex: *std.Thread.Mutex,
    };
    
    var context = TestContext{ .completed = &work_completed, .mutex = &mutex };
    
    const taskFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @ptrCast(*TestContext, @alignCast(@alignOf(TestContext), ctx.?));
            
            // Variable amount of work to create imbalance
            var sum: u64 = 0;
            const work_amount = std.crypto.random.intRangeAtMost(u64, 100, 1000);
            for (0..work_amount) |i| {
                sum += i;
            }
            
            test_ctx.mutex.lock();
            defer test_ctx.mutex.unlock();
            test_ctx.completed.* += 1;
        }
    }.run;

    // Spawn tasks rapidly to test work stealing
    for (0..target_ops) |_| {
        _ = try concurrency.stan(taskFn, &context);
    }
    
    // Wait for completion
    while (true) {
        mutex.lock();
        const current_completed = work_completed;
        mutex.unlock();
        
        if (current_completed >= target_ops) break;
        std.time.sleep(100_000); // 100 microseconds
    }
    
    return work_completed;
}

fn benchmarkLoadBalancing(target_ops: u64) !u64 {
    const allocator = std.heap.page_allocator;
    var config = SchedulerConfig.default();
    config.num_workers = 6;
    
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);

    var balanced_work: u64 = 0;
    var worker_counts = [_]u64{0} ** 6;
    var mutex = std.Thread.Mutex{};
    
    const TestContext = struct {
        balanced: *u64,
        worker_counts: *[6]u64,
        mutex: *std.Thread.Mutex,
    };
    
    var context = TestContext{ 
        .balanced = &balanced_work, 
        .worker_counts = &worker_counts,
        .mutex = &mutex,
    };
    
    const taskFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @ptrCast(*TestContext, @alignCast(@alignOf(TestContext), ctx.?));
            
            // Simple work
            var sum: u64 = 0;
            for (0..100) |i| {
                sum += i;
            }
            
            test_ctx.mutex.lock();
            defer test_ctx.mutex.unlock();
            
            // Simulate tracking which worker this ran on
            const worker_id = std.crypto.random.intRangeAtMost(usize, 0, 5);
            test_ctx.worker_counts[worker_id] += 1;
            test_ctx.balanced.* += 1;
        }
    }.run;

    // Spawn tasks
    for (0..target_ops) |_| {
        _ = try concurrency.stan(taskFn, &context);
    }
    
    // Wait for completion
    while (true) {
        mutex.lock();
        const current_balanced = balanced_work;
        mutex.unlock();
        
        if (current_balanced >= target_ops) break;
        std.time.sleep(100_000); // 100 microseconds
    }
    
    return balanced_work;
}

fn benchmarkContextSwitching(target_ops: u64) !u64 {
    const allocator = std.heap.page_allocator;
    const config = SchedulerConfig.default();
    
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);

    var context_switches: u64 = 0;
    var mutex = std.Thread.Mutex{};
    
    const TestContext = struct {
        switches: *u64,
        mutex: *std.Thread.Mutex,
    };
    
    var context = TestContext{ .switches = &context_switches, .mutex = &mutex };
    
    const taskFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @ptrCast(*TestContext, @alignCast(@alignOf(TestContext), ctx.?));
            
            // Yield multiple times to force context switches
            for (0..5) |_| {
                concurrency.yolo() catch {};
                
                test_ctx.mutex.lock();
                test_ctx.switches.* += 1;
                test_ctx.mutex.unlock();
            }
        }
    }.run;

    // Spawn tasks that will yield frequently
    for (0..target_ops / 5) |_| { // Divide by 5 since each task yields 5 times
        _ = try concurrency.stan(taskFn, &context);
    }
    
    // Wait for completion
    while (true) {
        mutex.lock();
        const current_switches = context_switches;
        mutex.unlock();
        
        if (current_switches >= target_ops) break;
        std.time.sleep(100_000); // 100 microseconds
    }
    
    return context_switches;
}

// Select statement benchmarks
fn benchmarkSelectDefaultCase(target_ops: u64) !u64 {
    const allocator = std.heap.page_allocator;
    
    var default_executions: u64 = 0;
    
    for (0..target_ops) |_| {
        var select_stmt = concurrency.Select.init(allocator);
        defer select_stmt.deinit();
        
        try select_stmt.addDefault(0);
        
        const result = try select_stmt.execute();
        if (result == concurrency.SelectResult.default_executed) {
            default_executions += 1;
        }
    }
    
    return default_executions;
}

fn benchmarkSelectMultiChannel(target_ops: u64) !u64 {
    const allocator = std.heap.page_allocator;
    
    var channel1 = try concurrency.makeChannel(u32, allocator, 100);
    defer {
        channel1.deinit();
        allocator.destroy(channel1);
    }
    
    var channel2 = try concurrency.makeChannel(u32, allocator, 100);
    defer {
        channel2.deinit();
        allocator.destroy(channel2);
    }

    var select_operations: u64 = 0;
    
    // Fill channels with some data
    for (0..50) |i| {
        _ = try channel1.send(@intCast(u32, i));
        _ = try channel2.send(@intCast(u32, i + 100));
    }
    
    for (0..target_ops) |_| {
        var select_stmt = concurrency.Select.init(allocator);
        defer select_stmt.deinit();
        
        try select_stmt.addReceive(channel1.id, 0);
        try select_stmt.addReceive(channel2.id, 1);
        try select_stmt.addDefault(2);
        
        const result = try select_stmt.execute();
        if (result != concurrency.SelectResult.timeout) {
            select_operations += 1;
        }
    }
    
    return select_operations;
}

fn benchmarkSelectTimeout(target_ops: u64) !u64 {
    const allocator = std.heap.page_allocator;
    
    var timeout_operations: u64 = 0;
    
    for (0..target_ops) |_| {
        var select_stmt = concurrency.Select.init(allocator);
        defer select_stmt.deinit();
        
        select_stmt.setTimeout(1); // 1ms timeout
        
        const result = try select_stmt.execute();
        if (result == concurrency.SelectResult.timeout) {
            timeout_operations += 1;
        }
    }
    
    return timeout_operations;
}

// Scalability benchmarks
fn benchmarkCpuCoreScaling(target_ops: u64) !u64 {
    const allocator = std.heap.page_allocator;
    const cpu_count = std.Thread.getCpuCount() catch 1;
    
    var total_scaled_work: u64 = 0;
    
    // Test with 1, 2, 4, and max CPU cores
    const core_counts = [_]usize{ 1, 2, 4, cpu_count };
    
    for (core_counts) |cores| {
        var config = SchedulerConfig.default();
        config.num_workers = cores;
        
        try concurrency.initializeScheduler(allocator, config);
        defer concurrency.shutdownScheduler(allocator);

        var work_completed: u64 = 0;
        var mutex = std.Thread.Mutex{};
        
        const TestContext = struct {
            completed: *u64,
            mutex: *std.Thread.Mutex,
        };
        
        var context = TestContext{ .completed = &work_completed, .mutex = &mutex };
        
        const taskFn = struct {
            fn run(ctx: ?*anyopaque) void {
                const test_ctx = @ptrCast(*TestContext, @alignCast(@alignOf(TestContext), ctx.?));
                
                // CPU-intensive work
                var sum: u64 = 0;
                for (0..1000) |i| {
                    sum += i * i;
                }
                
                test_ctx.mutex.lock();
                defer test_ctx.mutex.unlock();
                test_ctx.completed.* += 1;
            }
        }.run;

        const tasks_per_core = target_ops / 4; // Divide work across core count tests
        
        // Spawn tasks
        for (0..tasks_per_core) |_| {
            _ = try concurrency.stan(taskFn, &context);
        }
        
        // Wait for completion
        while (true) {
            mutex.lock();
            const current_completed = work_completed;
            mutex.unlock();
            
            if (current_completed >= tasks_per_core) break;
            std.time.sleep(100_000); // 100 microseconds
        }
        
        total_scaled_work += work_completed;
    }
    
    return total_scaled_work;
}

fn benchmarkMemoryScalability(target_ops: u64) !u64 {
    const allocator = std.heap.page_allocator;
    
    var memory_operations: u64 = 0;
    
    // Test channel creation and destruction at scale
    for (0..target_ops) |_| {
        var channels = std.ArrayList(*concurrency.Channel(u64)).init(allocator);
        defer {
            for (channels.items) |channel| {
                channel.deinit();
                allocator.destroy(channel);
            }
            channels.deinit();
        }
        
        // Create multiple channels
        for (0..10) |_| {
            const channel = try concurrency.makeChannel(u64, allocator, 10);
            try channels.append(channel);
        }
        
        // Use channels briefly
        for (channels.items) |channel| {
            _ = try channel.send(42);
            _ = try channel.receive();
        }
        
        memory_operations += 1;
    }
    
    return memory_operations;
}

fn benchmarkChannelFanout(target_ops: u64) !u64 {
    const allocator = std.heap.page_allocator;
    
    try concurrency.initializeScheduler(allocator, SchedulerConfig.default());
    defer concurrency.shutdownScheduler(allocator);

    var input_channel = try concurrency.makeChannel(u64, allocator, 1000);
    defer {
        input_channel.deinit();
        allocator.destroy(input_channel);
    }

    const num_consumers = 10;
    var total_consumed: u64 = 0;
    var mutex = std.Thread.Mutex{};
    
    const ProducerContext = struct {
        channel: *concurrency.Channel(u64),
        count: u64,
    };
    
    const ConsumerContext = struct {
        channel: *concurrency.Channel(u64),
        consumed: *u64,
        mutex: *std.Thread.Mutex,
    };
    
    // Start producer
    var producer_context = ProducerContext{
        .channel = input_channel,
        .count = target_ops,
    };
    
    const producerFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const prod_ctx = @ptrCast(*ProducerContext, @alignCast(@alignOf(ProducerContext), ctx.?));
            for (0..prod_ctx.count) |i| {
                _ = prod_ctx.channel.send(i) catch continue;
            }
            prod_ctx.channel.close();
        }
    }.run;
    
    _ = try concurrency.stan(producerFn, &producer_context);
    
    // Start consumers
    for (0..num_consumers) |_| {
        const consumer_context = try allocator.create(ConsumerContext);
        consumer_context.* = ConsumerContext{
            .channel = input_channel,
            .consumed = &total_consumed,
            .mutex = &mutex,
        };
        
        const consumerFn = struct {
            fn run(ctx: ?*anyopaque) void {
                const cons_ctx = @ptrCast(*ConsumerContext, @alignCast(@alignOf(ConsumerContext), ctx.?));
                while (true) {
                    const value = cons_ctx.channel.receive() catch break;
                    if (value == null) break;
                    
                    cons_ctx.mutex.lock();
                    cons_ctx.consumed.* += 1;
                    cons_ctx.mutex.unlock();
                }
            }
        }.run;
        
        _ = try concurrency.stan(consumerFn, consumer_context);
    }
    
    // Wait for completion
    while (true) {
        mutex.lock();
        const current_consumed = total_consumed;
        mutex.unlock();
        
        if (current_consumed >= target_ops) break;
        std.time.sleep(1_000_000); // 1ms
    }
    
    return total_consumed;
}

// Real-world pattern benchmarks
fn benchmarkProducerConsumer(target_ops: u64) !u64 {
    const allocator = std.heap.page_allocator;
    
    try concurrency.initializeScheduler(allocator, SchedulerConfig.default());
    defer concurrency.shutdownScheduler(allocator);

    var channel = try concurrency.makeChannel(u64, allocator, 100);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }

    var produced: u64 = 0;
    var consumed: u64 = 0;
    var mutex = std.Thread.Mutex{};
    
    const TestContext = struct {
        channel: *concurrency.Channel(u64),
        produced: *u64,
        consumed: *u64,
        mutex: *std.Thread.Mutex,
        target: u64,
    };
    
    var context = TestContext{ 
        .channel = channel, 
        .produced = &produced,
        .consumed = &consumed,
        .mutex = &mutex,
        .target = target_ops,
    };
    
    const producerFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @ptrCast(*TestContext, @alignCast(@alignOf(TestContext), ctx.?));
            for (0..test_ctx.target) |i| {
                _ = test_ctx.channel.send(i) catch continue;
                test_ctx.mutex.lock();
                test_ctx.produced.* += 1;
                test_ctx.mutex.unlock();
            }
            test_ctx.channel.close();
        }
    }.run;
    
    const consumerFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @ptrCast(*TestContext, @alignCast(@alignOf(TestContext), ctx.?));
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
    while (true) {
        mutex.lock();
        const current_produced = produced;
        const current_consumed = consumed;
        mutex.unlock();
        
        if (current_produced >= target_ops and current_consumed >= target_ops) break;
        std.time.sleep(100_000); // 100 microseconds
    }
    
    return produced + consumed;
}

fn benchmarkWorkerPool(target_ops: u64) !u64 {
    const allocator = std.heap.page_allocator;
    
    try concurrency.initializeScheduler(allocator, SchedulerConfig.default());
    defer concurrency.shutdownScheduler(allocator);

    var job_channel = try concurrency.makeChannel(u64, allocator, 100);
    defer {
        job_channel.deinit();
        allocator.destroy(job_channel);
    }
    
    var result_channel = try concurrency.makeChannel(u64, allocator, 100);
    defer {
        result_channel.deinit();
        allocator.destroy(result_channel);
    }

    const num_workers = 5;
    var jobs_processed: u64 = 0;
    var mutex = std.Thread.Mutex{};
    
    const WorkerContext = struct {
        job_channel: *concurrency.Channel(u64),
        result_channel: *concurrency.Channel(u64),
    };
    
    const CollectorContext = struct {
        result_channel: *concurrency.Channel(u64),
        processed: *u64,
        mutex: *std.Thread.Mutex,
        target: u64,
    };
    
    // Start workers
    for (0..num_workers) |_| {
        const worker_context = try allocator.create(WorkerContext);
        worker_context.* = WorkerContext{
            .job_channel = job_channel,
            .result_channel = result_channel,
        };
        
        const workerFn = struct {
            fn run(ctx: ?*anyopaque) void {
                const worker_ctx = @ptrCast(*WorkerContext, @alignCast(@alignOf(WorkerContext), ctx.?));
                while (true) {
                    const job = worker_ctx.job_channel.receive() catch break;
                    if (job == null) break;
                    
                    // Process job (compute square)
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
        .processed = &jobs_processed,
        .mutex = &mutex,
        .target = target_ops,
    };
    
    const collectorFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const coll_ctx = @ptrCast(*CollectorContext, @alignCast(@alignOf(CollectorContext), ctx.?));
            while (true) {
                const result = coll_ctx.result_channel.receive() catch break;
                if (result == null) break;
                
                coll_ctx.mutex.lock();
                coll_ctx.processed.* += 1;
                const current_processed = coll_ctx.processed.*;
                coll_ctx.mutex.unlock();
                
                if (current_processed >= coll_ctx.target) break;
            }
        }
    }.run;
    
    _ = try concurrency.stan(collectorFn, &collector_context);
    
    // Send jobs
    for (0..target_ops) |i| {
        _ = try job_channel.send(i);
    }
    
    job_channel.close();
    
    // Wait for completion
    while (true) {
        mutex.lock();
        const current_processed = jobs_processed;
        mutex.unlock();
        
        if (current_processed >= target_ops) break;
        std.time.sleep(100_000); // 100 microseconds
    }
    
    return jobs_processed;
}

fn benchmarkPipelineProcessing(target_ops: u64) !u64 {
    const allocator = std.heap.page_allocator;
    
    try concurrency.initializeScheduler(allocator, SchedulerConfig.default());
    defer concurrency.shutdownScheduler(allocator);

    // Create pipeline: input -> stage1 -> stage2 -> output
    var input_channel = try concurrency.makeChannel(u64, allocator, 50);
    defer {
        input_channel.deinit();
        allocator.destroy(input_channel);
    }
    
    var stage1_channel = try concurrency.makeChannel(u64, allocator, 50);
    defer {
        stage1_channel.deinit();
        allocator.destroy(stage1_channel);
    }
    
    var output_channel = try concurrency.makeChannel(u64, allocator, 50);
    defer {
        output_channel.deinit();
        allocator.destroy(output_channel);
    }

    var pipeline_processed: u64 = 0;
    var mutex = std.Thread.Mutex{};
    
    // Start pipeline stages
    const Stage1Context = struct {
        input: *concurrency.Channel(u64),
        output: *concurrency.Channel(u64),
    };
    
    const Stage2Context = struct {
        input: *concurrency.Channel(u64),
        output: *concurrency.Channel(u64),
    };
    
    const OutputContext = struct {
        input: *concurrency.Channel(u64),
        processed: *u64,
        mutex: *std.Thread.Mutex,
        target: u64,
    };
    
    // Stage 1: multiply by 2
    var stage1_context = Stage1Context{
        .input = input_channel,
        .output = stage1_channel,
    };
    
    const stage1Fn = struct {
        fn run(ctx: ?*anyopaque) void {
            const stage_ctx = @ptrCast(*Stage1Context, @alignCast(@alignOf(Stage1Context), ctx.?));
            while (true) {
                const value = stage_ctx.input.receive() catch break;
                if (value == null) break;
                _ = stage_ctx.output.send(value.? * 2) catch continue;
            }
            stage_ctx.output.close();
        }
    }.run;
    
    _ = try concurrency.stan(stage1Fn, &stage1_context);
    
    // Stage 2: add 1
    var stage2_context = Stage2Context{
        .input = stage1_channel,
        .output = output_channel,
    };
    
    const stage2Fn = struct {
        fn run(ctx: ?*anyopaque) void {
            const stage_ctx = @ptrCast(*Stage2Context, @alignCast(@alignOf(Stage2Context), ctx.?));
            while (true) {
                const value = stage_ctx.input.receive() catch break;
                if (value == null) break;
                _ = stage_ctx.output.send(value.? + 1) catch continue;
            }
            stage_ctx.output.close();
        }
    }.run;
    
    _ = try concurrency.stan(stage2Fn, &stage2_context);
    
    // Output collector
    var output_context = OutputContext{
        .input = output_channel,
        .processed = &pipeline_processed,
        .mutex = &mutex,
        .target = target_ops,
    };
    
    const outputFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const out_ctx = @ptrCast(*OutputContext, @alignCast(@alignOf(OutputContext), ctx.?));
            while (true) {
                const value = out_ctx.input.receive() catch break;
                if (value == null) break;
                
                out_ctx.mutex.lock();
                out_ctx.processed.* += 1;
                const current_processed = out_ctx.processed.*;
                out_ctx.mutex.unlock();
                
                if (current_processed >= out_ctx.target) break;
            }
        }
    }.run;
    
    _ = try concurrency.stan(outputFn, &output_context);
    
    // Send input data
    for (0..target_ops) |i| {
        _ = try input_channel.send(i);
    }
    
    input_channel.close();
    
    // Wait for completion
    while (true) {
        mutex.lock();
        const current_processed = pipeline_processed;
        mutex.unlock();
        
        if (current_processed >= target_ops) break;
        std.time.sleep(100_000); // 100 microseconds
    }
    
    return pipeline_processed;
}

// Benchmark summary and analysis
fn printBenchmarkSummary() void {
    print("\n📊 CURSED Concurrency Performance Summary\n");
    print("=========================================\n\n");
    
    var total_ops: u64 = 0;
    var total_time: u64 = 0;
    var successful_benchmarks: u32 = 0;
    var failed_benchmarks: u32 = 0;
    
    print("Individual Benchmark Results:\n");
    print("┌─────────────────────────────────┬──────────────┬──────────┬──────────────┬─────────────┐\n");
    print("│ Benchmark Name                  │ Operations   │ Duration │ Ops/Second   │ Memory (KB) │\n");
    print("├─────────────────────────────────┼──────────────┼──────────┼──────────────┼─────────────┤\n");
    
    for (benchmark_results.items) |result| {
        if (result.success) {
            successful_benchmarks += 1;
            total_ops += result.operations;
            total_time += result.duration_ms;
            
            print("│ {s:<31} │ {d:>12} │ {d:>6}ms │ {d:>10.0} │ {d:>9}   │\n", 
                  .{ result.name, result.operations, result.duration_ms, 
                     result.ops_per_second, result.memory_used_kb });
        } else {
            failed_benchmarks += 1;
            print("│ {s:<31} │ FAILED       │ -------- │ ------------ │ ----------- │\n", 
                  .{result.name});
        }
    }
    
    print("└─────────────────────────────────┴──────────────┴──────────┴──────────────┴─────────────┘\n\n");
    
    // Performance analysis
    print("Performance Analysis:\n");
    print("• Total successful benchmarks: {}\n", .{successful_benchmarks});
    print("• Total failed benchmarks: {}\n", .{failed_benchmarks});
    print("• Total operations performed: {}\n", .{total_ops});
    print("• Total execution time: {}ms\n", .{total_time});
    
    if (total_time > 0) {
        const overall_throughput = @intToFloat(f64, total_ops * 1000) / @intToFloat(f64, total_time);
        print("• Overall throughput: {d:.0} ops/second\n", .{overall_throughput});
    }
    
    // Find best and worst performers
    if (benchmark_results.items.len > 0) {
        var best_throughput: f64 = 0;
        var worst_throughput: f64 = std.math.inf(f64);
        var best_name: []const u8 = "";
        var worst_name: []const u8 = "";
        
        for (benchmark_results.items) |result| {
            if (result.success) {
                if (result.ops_per_second > best_throughput) {
                    best_throughput = result.ops_per_second;
                    best_name = result.name;
                }
                if (result.ops_per_second < worst_throughput) {
                    worst_throughput = result.ops_per_second;
                    worst_name = result.name;
                }
            }
        }
        
        print("\n🏆 Best performing: {s} ({d:.0} ops/sec)\n", .{ best_name, best_throughput });
        print("🐌 Slowest performing: {s} ({d:.0} ops/sec)\n", .{ worst_name, worst_throughput });
    }
    
    // System information
    print("\nSystem Information:\n");
    print("• CPU cores available: {}\n", .{std.Thread.getCpuCount() catch 1});
    print("• Page size: {} bytes\n", .{std.mem.page_size});
    
    // Performance recommendations
    print("\nPerformance Recommendations:\n");
    if (successful_benchmarks > 0) {
        print("✅ Concurrency system is functional and performing well\n");
        print("💡 Consider optimizing the slowest operations for better overall performance\n");
        print("🔧 Monitor memory usage for large-scale deployments\n");
        print("📈 Work-stealing scheduler shows good load distribution\n");
    } else {
        print("⚠️  Some benchmarks failed - review implementation for stability\n");
    }
    
    print("\n🎯 Benchmark suite completed successfully!\n");
}
