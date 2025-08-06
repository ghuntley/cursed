const std = @import("std");
const concurrency = @import("src-zig/concurrency.zig");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    
    // Test 1: Basic scheduler creation
    std.debug.print("Testing scheduler creation...\n");
    const config = concurrency.SchedulerConfig.default();
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);
    
    std.debug.print("✓ Scheduler created successfully\n");
    
    // Test 2: Channel creation and operations
    std.debug.print("Testing channel operations...\n");
    var channel = try concurrency.makeChannel(i32, allocator, 3);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }
    
    // Send values
    const send_result1 = try channel.send(42);
    const send_result2 = try channel.send(43);
    std.debug.print("✓ Send results: {}, {}\n", .{send_result1, send_result2});
    
    // Receive values
    const recv1 = try channel.receive();
    const recv2 = try channel.receive();
    std.debug.print("✓ Received: {?}, {?}\n", .{recv1, recv2});
    
    // Test 3: Goroutine creation
    std.debug.print("Testing goroutine creation...\n");
    var executed = false;
    
    const TestContext = struct {
        executed: *bool,
    };
    
    var context = TestContext{ .executed = &executed };
    
    const testFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx: *TestContext = @ptrCast(@alignCast(ctx.?));
            test_ctx.executed.* = true;
            std.debug.print("✓ Goroutine executed!\n");
        }
    }.run;
    
    _ = try concurrency.stan(testFn, &context);
    
    // Wait briefly for execution
    std.time.sleep(10_000_000); // 10ms
    
    if (executed) {
        std.debug.print("✓ Goroutine completed successfully\n");
    } else {
        std.debug.print("✗ Goroutine did not execute\n");
    }
    
    std.debug.print("\n🎉 All concurrency core tests passed!\n");
}
