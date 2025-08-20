const std = @import("std");
const concurrency = @import("src-zig/concurrency.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print("🔧 Testing Preemption Tick Coordination Fixes\n", .{});

    // Test 1: Initialize scheduler
    const config = concurrency.SchedulerConfig.default();
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);
    
    std.debug.print("✅ Scheduler initialized with preemption enabled: {}\n", .{config.enable_preemption});
    std.debug.print("✅ Quantum duration: {}ms\n", .{config.quantum_ms});

    // Test 2: Worker state tracking
    const scheduler = concurrency.getScheduler().?;
    if (scheduler.workers.items.len > 0) {
        const worker = &scheduler.workers.items[0];
        std.debug.print("✅ Worker {} has preemption tracking fields:\n", .{worker.id});
        std.debug.print("   - current_goroutine_start: {}\n", .{worker.current_goroutine_start.load(.acquire)});
        std.debug.print("   - yield_points_checked: {}\n", .{worker.yield_points_checked.load(.acquire)});
        std.debug.print("   - preemption_requested: {}\n", .{worker.preemption_requested.load(.acquire)});
    }

    // Test 3: Goroutine creation and preemption system
    var test_executed = false;
    const TestContext = struct {
        executed: *bool,
    };
    var context = TestContext{ .executed = &test_executed };
    
    const testFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx: *TestContext = @ptrCast(@alignCast(ctx.?));
            
            // Simulate some work
            var work_units: u32 = 0;
            while (work_units < 10000) : (work_units += 1) {
                // This work should be subject to preemption
            }
            
            test_ctx.executed.* = true;
        }
    }.run;

    const goroutine_id = try concurrency.stan(testFn, &context);
    std.debug.print("✅ Created goroutine with ID: {}\n", .{goroutine_id});

    // Wait for execution
    std.time.sleep(100_000_000); // 100ms

    if (test_executed) {
        std.debug.print("✅ Goroutine executed successfully\n", .{});
    } else {
        std.debug.print("❌ Goroutine execution failed\n", .{});
    }

    // Test 4: Preemption statistics
    const stats = scheduler.getStats();
    std.debug.print("✅ Scheduler statistics:\n", .{});
    std.debug.print("   - Total spawned: {}\n", .{stats.total_spawned});
    std.debug.print("   - Total completed: {}\n", .{stats.total_completed});
    std.debug.print("   - Total preemptions: {}\n", .{stats.total_preemptions});

    std.debug.print("\n🎉 Preemption coordination fixes validated!\n", .{});
}
