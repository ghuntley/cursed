//! Simple test for concurrency runtime core functionality

const std = @import("std");

// Import only core types and functions
const concurrency = @import("src-zig/concurrency.zig");

test "basic concurrency system test" {
    const allocator = std.testing.allocator;
    
    // Test basic scheduler configuration
    const config = concurrency.SchedulerConfig{
        .num_workers = 2,
        .queue_capacity = 10,
        .default_stack_size = 1024 * 1024,
        .enable_work_stealing = true,
        .enable_preemption = false,
        .quantum_ms = 10,
    };
    
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);
    
    const scheduler = concurrency.getScheduler();
    try std.testing.expect(scheduler != null);
}

test "channel creation and basic operations" {
    const allocator = std.testing.allocator;
    
    var channel = try concurrency.makeChannel(i32, allocator, 3);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }
    
    // Test channel send
    const send_result = try channel.send(42);
    try std.testing.expect(send_result == .sent);
    
    // Test channel receive
    const received = try channel.receive();
    try std.testing.expect(received != null);
    try std.testing.expect(received.? == 42);
}

test "goroutine spawning" {
    const allocator = std.testing.allocator;
    
    const config = concurrency.SchedulerConfig.default();
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);
    
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

    const goroutine_id = try concurrency.stan(testFn, &context);
    try std.testing.expect(goroutine_id > 0);
    
    // Wait a bit for execution
    std.time.sleep(10_000_000); // 10ms
    
    try std.testing.expect(executed);
}
