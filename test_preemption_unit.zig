const std = @import("std");
const concurrency = @import("src-zig/concurrency.zig");
const testing = std.testing;

test "goroutine preemption logic" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Test goroutine creation with preemption features
    const test_fn = struct {
        fn run(_: ?*anyopaque) void {
            // Simple test function
        }
    }.run;

    var goroutine = concurrency.Goroutine.init(allocator, 1, test_fn, null);
    
    // Test initial state
    try testing.expect(goroutine.getState() == concurrency.GoroutineState.ready);
    try testing.expect(!goroutine.shouldPreempt());
    
    // Test quantum timing
    goroutine.startQuantum();
    try testing.expect(goroutine.quantum_start.load(.acquire) > 0);
    
    // Test preemption signals
    goroutine.signalPreemption(.time_slice_expired);
    try testing.expect(goroutine.shouldPreempt());
    try testing.expect(goroutine.preemption_stats.preemptions_received == 1);
    
    // Test cooperative yield
    const initial_yield_count = goroutine.yield_count.load(.acquire);
    goroutine.cooperativeYield();
    try testing.expect(goroutine.yield_count.load(.acquire) == initial_yield_count + 1);
    
    std.log.info("✅ Preemption logic tests passed");
}

test "scheduler configuration" {
    const config = concurrency.SchedulerConfig.default();
    
    // Verify preemption is enabled by default
    try testing.expect(config.enable_preemption);
    try testing.expect(config.quantum_ms == 10);
    
    std.log.info("✅ Scheduler configuration tests passed");
}

test "preemption statistics" {
    var stats = concurrency.PreemptionStats.init();
    
    // Test initial values
    try testing.expect(stats.preemptions_performed == 0);
    try testing.expect(stats.preemptions_received == 0);
    try testing.expect(stats.quantum_violations == 0);
    try testing.expect(stats.priority_escalations == 0);
    try testing.expect(stats.context_switches == 0);
    try testing.expect(stats.cooperative_yields == 0);
    
    std.log.info("✅ Preemption statistics tests passed");
}

test "worker statistics with preemption" {
    var worker_stats = concurrency.WorkerStats.init();
    
    // Test preemption-related fields
    try testing.expect(worker_stats.preemptions_handled == 0);
    try testing.expect(worker_stats.cooperative_yields == 0);
    try testing.expect(worker_stats.quantum_violations == 0);
    
    std.log.info("✅ Worker statistics tests passed");
}
