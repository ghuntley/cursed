const std = @import("std");
const print = std.debug.print;
const testing = std.testing;

// Import concurrency module
const concurrency = @import("src-zig/concurrency.zig");

/// Test suite for CURSED concurrency runtime
pub const ConcurrencyRuntimeTest = struct {
    allocator: std.mem.Allocator,
    
    pub fn init(allocator: std.mem.Allocator) ConcurrencyRuntimeTest {
        return ConcurrencyRuntimeTest{
            .allocator = allocator,
        };
    }
    
    /// Test channel operations
    pub fn testChannelOperations(self: *ConcurrencyRuntimeTest) !void {
        print("🧪 Testing channel operations...\n", .{});
        
        // Initialize scheduler
        const config = concurrency.SchedulerConfig.default();
        try concurrency.initializeScheduler(self.allocator, config);
        defer concurrency.shutdownScheduler(self.allocator);
        
        // Create channel
        var channel = try concurrency.makeChannel(i32, self.allocator, 3);
        defer {
            channel.deinit();
            self.allocator.destroy(channel);
        }
        
        // Test send operation
        const send_result = try channel.send(42);
        try testing.expect(send_result == concurrency.SendResult.sent);
        print("✅ Channel send operation successful\n", .{});
        
        // Test receive operation
        const received = try channel.receive();
        try testing.expect(received != null);
        try testing.expect(received.? == 42);
        print("✅ Channel receive operation successful\n", .{});
        
        // Test multiple sends and receives
        _ = try channel.send(100);
        _ = try channel.send(200);
        _ = try channel.send(300);
        
        const val1 = try channel.receive();
        const val2 = try channel.receive();
        const val3 = try channel.receive();
        
        try testing.expect(val1.? == 100);
        try testing.expect(val2.? == 200);
        try testing.expect(val3.? == 300);
        print("✅ Multiple channel operations successful\n", .{});
        
        // Test channel closing
        channel.close();
        try testing.expect(channel.isClosed());
        print("✅ Channel close operation successful\n", .{});
    }
    
    /// Test goroutine spawning and execution
    pub fn testGoroutineExecution(self: *ConcurrencyRuntimeTest) !void {
        print("🧪 Testing goroutine execution...\n", .{});
        
        // Initialize scheduler
        const config = concurrency.SchedulerConfig.default();
        try concurrency.initializeScheduler(self.allocator, config);
        defer concurrency.shutdownScheduler(self.allocator);
        
        // Test context for goroutine
        var executed = false;
        const TestContext = struct {
            executed: *bool,
        };
        
        var context = TestContext{ .executed = &executed };
        
        const testFn = struct {
            fn run(ctx: ?*anyopaque) void {
                const test_ctx: *TestContext = @ptrCast(@alignCast(ctx.?));
                test_ctx.executed.* = true;
            }
        }.run;
        
        // Spawn goroutine
        const goroutine_id = try concurrency.stan(testFn, &context);
        try testing.expect(goroutine_id > 0);
        
        // Wait for execution
        std.time.sleep(100_000_000); // 100ms
        
        try testing.expect(executed);
        print("✅ Goroutine execution successful\n", .{});
    }
    
    /// Test select operations
    pub fn testSelectOperations(self: *ConcurrencyRuntimeTest) !void {
        print("🧪 Testing select operations...\n", .{});
        
        // Initialize scheduler
        const config = concurrency.SchedulerConfig.default();
        try concurrency.initializeScheduler(self.allocator, config);
        defer concurrency.shutdownScheduler(self.allocator);
        
        // Create channels
        var ch1 = try concurrency.makeChannel(i32, self.allocator, 1);
        defer {
            ch1.deinit();
            self.allocator.destroy(ch1);
        }
        
        var ch2 = try concurrency.makeChannel(i32, self.allocator, 1);
        defer {
            ch2.deinit();
            self.allocator.destroy(ch2);
        }
        
        // Send on one channel
        _ = try ch1.send(100);
        
        // Create select statement
        var select_stmt = concurrency.Select.init(self.allocator);
        defer select_stmt.deinit();
        
        try select_stmt.addReceive(ch1.id, 0);
        try select_stmt.addReceive(ch2.id, 1);
        try select_stmt.addDefault(2);
        
        // Execute select
        const result = try select_stmt.execute();
        try testing.expect(result == concurrency.SelectResult.receive_completed or 
                          result == concurrency.SelectResult.default_executed);
        print("✅ Select operation successful\n", .{});
    }
    
    /// Test goroutine coordination
    pub fn testGoroutineCoordination(self: *ConcurrencyRuntimeTest) !void {
        print("🧪 Testing goroutine coordination...\n", .{});
        
        // Initialize scheduler
        const config = concurrency.SchedulerConfig.default();
        try concurrency.initializeScheduler(self.allocator, config);
        defer concurrency.shutdownScheduler(self.allocator);
        
        // Create coordination channel
        var done_channel = try concurrency.makeChannel(bool, self.allocator, 2);
        defer {
            done_channel.deinit();
            self.allocator.destroy(done_channel);
        }
        
        // Shared counter
        var counter: i32 = 0;
        const CounterContext = struct {
            counter: *i32,
            done_ch: *concurrency.Channel(bool),
        };
        
        var context1 = CounterContext{ .counter = &counter, .done_ch = done_channel };
        var context2 = CounterContext{ .counter = &counter, .done_ch = done_channel };
        
        const worker1 = struct {
            fn run(ctx: ?*anyopaque) void {
                const worker_ctx: *CounterContext = @ptrCast(@alignCast(ctx.?));
                worker_ctx.counter.* += 10;
                _ = worker_ctx.done_ch.send(true) catch {};
            }
        }.run;
        
        const worker2 = struct {
            fn run(ctx: ?*anyopaque) void {
                const worker_ctx: *CounterContext = @ptrCast(@alignCast(ctx.?));
                worker_ctx.counter.* += 20;
                _ = worker_ctx.done_ch.send(true) catch {};
            }
        }.run;
        
        // Spawn worker goroutines
        _ = try concurrency.stan(worker1, &context1);
        _ = try concurrency.stan(worker2, &context2);
        
        // Wait for both workers to complete
        _ = try done_channel.receive(); // First worker
        _ = try done_channel.receive(); // Second worker
        
        try testing.expect(counter == 30); // 10 + 20
        print("✅ Goroutine coordination successful\n", .{});
    }
    
    /// Test scheduler statistics
    pub fn testSchedulerStats(self: *ConcurrencyRuntimeTest) !void {
        print("🧪 Testing scheduler statistics...\n", .{});
        
        // Initialize scheduler
        const config = concurrency.SchedulerConfig.default();
        try concurrency.initializeScheduler(self.allocator, config);
        defer concurrency.shutdownScheduler(self.allocator);
        
        const scheduler = concurrency.getScheduler();
        try testing.expect(scheduler != null);
        
        const stats = scheduler.?.getStats();
        try testing.expect(stats.total_spawned >= 0);
        print("✅ Scheduler statistics accessible\n", .{});
        
        // Test scheduler is running
        try testing.expect(scheduler.?.isRunning());
        print("✅ Scheduler running check successful\n", .{});
    }
    
    /// Run all concurrency runtime tests
    pub fn runAllTests(self: *ConcurrencyRuntimeTest) !void {
        print("\n🚀 Running CURSED Concurrency Runtime Tests\n", .{});
        print("============================================\n\n", .{});
        
        try self.testChannelOperations();
        print("\n", .{});
        
        try self.testGoroutineExecution();
        print("\n", .{});
        
        try self.testSelectOperations();
        print("\n", .{});
        
        try self.testGoroutineCoordination();
        print("\n", .{});
        
        try self.testSchedulerStats();
        print("\n", .{});
        
        print("🎉 All concurrency runtime tests passed!\n", .{});
        print("✅ Channels: Send/receive operations working\n", .{});
        print("✅ Goroutines: Spawning and execution working\n", .{});
        print("✅ Select: Non-blocking operations working\n", .{});
        print("✅ Coordination: Multi-goroutine synchronization working\n", .{});
        print("✅ Scheduler: Statistics and management working\n\n", .{});
    }
};

// Individual test functions for the Zig test runner
test "channel operations" {
    const allocator = testing.allocator;
    var test_suite = ConcurrencyRuntimeTest.init(allocator);
    try test_suite.testChannelOperations();
}

test "goroutine execution" {
    const allocator = testing.allocator;
    var test_suite = ConcurrencyRuntimeTest.init(allocator);
    try test_suite.testGoroutineExecution();
}

test "select operations" {
    const allocator = testing.allocator;
    var test_suite = ConcurrencyRuntimeTest.init(allocator);
    try test_suite.testSelectOperations();
}

test "goroutine coordination" {
    const allocator = testing.allocator;
    var test_suite = ConcurrencyRuntimeTest.init(allocator);
    try test_suite.testGoroutineCoordination();
}

test "scheduler statistics" {
    const allocator = testing.allocator;
    var test_suite = ConcurrencyRuntimeTest.init(allocator);
    try test_suite.testSchedulerStats();
}

// Example usage and demonstration
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var test_suite = ConcurrencyRuntimeTest.init(allocator);
    try test_suite.runAllTests();
}
