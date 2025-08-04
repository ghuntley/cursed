const std = @import("std");
const concurrency = @import("src-zig/concurrency.zig");
const print = std.debug.print;

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    
    print("Testing scheduler initialization...\n", .{});
    
    // Test scheduler initialization
    const config = concurrency.SchedulerConfig.default();
    print("Config created: {} workers\n", .{config.num_workers});
    
    try concurrency.initializeScheduler(allocator, config);
    print("Scheduler initialized!\n", .{});
    
    // Get scheduler and test basic operations
    const scheduler = concurrency.getScheduler();
    if (scheduler) |s| {
        print("Scheduler found, running: {}\n", .{s.isRunning()});
        print("Active goroutines: {}\n", .{s.activeGoroutineCount()});
    } else {
        print("No scheduler found!\n", .{});
    }
    
    // Cleanup
    concurrency.shutdownScheduler(allocator);
    print("Scheduler shut down!\n", .{});
}
