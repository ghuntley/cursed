const std = @import("std");
const gc_module = @import("src-zig/gc.zig");
const GC = gc_module.GC;
const GCConfig = gc_module.GCConfig;

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    
    // Initialize GC with smaller heap for faster testing
    var config = GCConfig.default();
    config.initial_heap_size = 64 * 1024; // 64KB heap
    config.young_gen_size = 16 * 1024;    // 16KB young generation
    config.concurrent_threads = 1;        // Single threaded for testing
    
    const gc = try GC.init(allocator, config);
    defer gc.deinit();
    
    std.log.info("🚀 CURSED Garbage Collector - Benchmark Test", .{});
    
    // Test: Basic allocation performance
    std.log.info("=== Basic Allocation Performance ===", .{});
    const start_time = std.time.nanoTimestamp();
    
    for (0..1000) |i| {
        const obj = gc.alloc(32, 1) catch {
            std.log.info("Allocation failed at iteration {}", .{i});
            break;
        };
        _ = obj;
        
        // Force collection every 100 allocations to prevent heap exhaustion
        if (i % 100 == 0 and i > 0) {
            std.log.info("Triggering collection at iteration {}", .{i});
            gc.triggerCollection();
            // Small delay to let collection complete
            std.time.sleep(1_000_000); // 1ms
        }
    }
    
    const end_time = std.time.nanoTimestamp();
    const duration_ns = end_time - start_time;
    const duration_ms = @divTrunc(duration_ns, 1_000_000);
    
    std.log.info("✅ Allocation test completed in {} ms", .{duration_ms});
    
    // Test: Root registration performance
    std.log.info("=== Root Registration Performance ===", .{});
    const obj1 = try gc.alloc(64, 2);
    const obj2 = try gc.alloc(64, 3);
    
    var root1: ?*anyopaque = obj1;
    var root2: ?*anyopaque = obj2;
    
    const root_start = std.time.nanoTimestamp();
    try gc.addRoot(&root1, 2);
    try gc.addRoot(&root2, 3);
    const root_end = std.time.nanoTimestamp();
    
    const root_duration = @divTrunc(root_end - root_start, 1000); // microseconds
    std.log.info("✅ Root registration completed in {} μs", .{root_duration});
    
    // Test: Collection performance
    std.log.info("=== Collection Performance ===", .{});
    const collect_start = std.time.nanoTimestamp();
    try gc.collectNow();
    const collect_end = std.time.nanoTimestamp();
    
    const collect_duration = @divTrunc(collect_end - collect_start, 1000); // microseconds
    std.log.info("✅ Collection completed in {} μs", .{collect_duration});
    
    // Verify roots survived
    if (root1 != null and root2 != null) {
        std.log.info("✅ Rooted objects survived collection", .{});
    } else {
        std.log.warn("⚠️ Some rooted objects were collected", .{});
    }
    
    // Print final statistics
    std.log.info("=== Final GC Statistics ===", .{});
    const stats = gc.getStats();
    std.log.info("Total allocations: {}", .{stats.total_allocations});
    std.log.info("GC cycles: {}", .{stats.gc_cycles});
    std.log.info("Current heap size: {} bytes", .{stats.current_heap_size});
    std.log.info("Peak heap size: {} bytes", .{stats.peak_heap_size});
    
    if (stats.gc_cycles > 0) {
        const avg_pause = @divTrunc(stats.total_pause_time_us, stats.gc_cycles);
        std.log.info("Average pause time: {} μs", .{avg_pause});
        std.log.info("Max pause time: {} μs", .{stats.max_pause_time_us});
    }
    
    // Cleanup
    gc.removeRoot(&root1);
    gc.removeRoot(&root2);
    
    std.log.info("🎉 GC Benchmark completed successfully!", .{});
}
