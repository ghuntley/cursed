const std = @import("std");
const print = std.debug.print;
const testing = std.testing;

const concurrency = @import("src-zig/concurrency.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    print("🔧 Debug Concurrency Test\n", .{});
    
    const config = concurrency.SchedulerConfig.default();
    print("Config: workers={}, queue_capacity={}\n", .{ config.num_workers, config.queue_capacity });
    
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);
    
    print("✅ Scheduler initialized successfully\n", .{});
    
    // Simple test function
    const testFn = struct {
        fn run(ctx: ?*anyopaque) void {
            _ = ctx;
            print("🎯 Goroutine executed!\n", .{});
        }
    }.run;
    
    print("🚀 Spawning goroutine...\n", .{});
    const goroutine_id = try concurrency.stan(testFn, null);
    print("✅ Goroutine {} spawned\n", .{goroutine_id});
    
    // Wait for execution
    std.time.sleep(100_000_000); // 100ms
    
    print("✅ Test completed successfully\n", .{});
}
