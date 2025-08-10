const std = @import("std");
const testing = std.testing;
const MemoryPool = @import("src-zig/memory_pool_system.zig").MemoryPool;
const PoolConfig = @import("src-zig/memory_pool_system.zig").PoolConfig;
const NUMATopology = @import("src-zig/numa_system.zig").NUMATopology;
const MemoryPerformanceMonitor = @import("src-zig/memory_performance_monitor.zig").MemoryPerformanceMonitor;

// Integration test for P2 Item #6: Memory Pool Optimization and NUMA Awareness
// This test validates that the memory pool system integrates correctly with
// the CURSED compiler build system and provides the expected functionality.

test "Memory Pool System Integration" {
    const allocator = testing.allocator;
    
    // Test pool creation with default configuration
    const config = PoolConfig{
        .strategy = .SizeClass,
        .min_size = 1024 * 1024, // 1MB
        .max_size = 16 * 1024 * 1024, // 16MB
        .thread_local_cache = true,
        .cache_size = 64 * 1024, // 64KB
        .monitoring = true,
        .auto_tuning = false, // Disable for testing
    };
    
    var pool = MemoryPool.init(config, allocator, null) catch |err| {
        std.log.err("Failed to initialize memory pool: {}", .{err});
        return err;
    };
    defer pool.deinit();
    
    // Test basic allocation and deallocation
    const test_sizes = [_]usize{ 64, 128, 256, 512, 1024, 4096 };
    var allocations = std.ArrayList([]u8).init(allocator);
    defer allocations.deinit();
    
    for (test_sizes) |size| {
        const allocation = pool.alloc(size) catch |err| {
            std.log.err("Failed to allocate {} bytes: {}", .{ size, err });
            return err;
        };
        
        // Verify allocation size
        try testing.expect(allocation.len == size);
        
        // Write to memory to ensure it's valid
        @memset(allocation, 0xAA);
        
        // Verify written data
        for (allocation) |byte| {
            try testing.expect(byte == 0xAA);
        }
        
        try allocations.append(allocation);
    }
    
    // Free all allocations
    for (allocations.items) |allocation| {
        pool.free(allocation);
    }
    
    // Verify pool statistics
    const stats = pool.getStats();
    try testing.expect(stats.total_allocs.load(.acquire) == test_sizes.len);
    try testing.expect(stats.total_frees.load(.acquire) == test_sizes.len);
    try testing.expect(stats.active_allocs.load(.acquire) == 0);
    try testing.expect(stats.active_bytes.load(.acquire) == 0);
    
    std.log.info("✅ Memory pool integration test passed");
}

test "NUMA Topology Detection" {
    const allocator = testing.allocator;
    
    var numa = NUMATopology.init(allocator) catch |err| {
        // NUMA detection might fail in test environments, which is acceptable
        if (err == error.NoNUMASupport or err == error.PermissionDenied) {
            std.log.warn("⚠️ NUMA topology detection not available in test environment");
            return;
        }
        std.log.err("Failed to initialize NUMA topology: {}", .{err});
        return err;
    };
    defer numa.deinit(allocator);
    
    // Test basic NUMA functionality
    const current_node = numa.getCurrentNode();
    try testing.expect(current_node < numa.nodes.len);
    
    const optimal_node = numa.getOptimalNode(4096, null);
    try testing.expect(optimal_node < numa.nodes.len);
    
    // Test memory statistics
    const stats = numa.getMemoryStats();
    try testing.expect(stats.total_memory > 0);
    defer stats.deinit();
    
    std.log.info("✅ NUMA topology integration test passed");
}

test "Performance Monitoring Integration" {
    const allocator = testing.allocator;
    
    const config = MemoryPerformanceMonitor.MonitorConfig{
        .buffer_size = 1024, // Small buffer for testing
        .metrics_window_us = 1000000, // 1 second
        .hotspot_threshold = 10.0,
        .stack_traces = false, // Disable for testing
        .regression_detection = false, // Disable for testing
    };
    
    var monitor = MemoryPerformanceMonitor.init(allocator, config) catch |err| {
        std.log.err("Failed to initialize performance monitor: {}", .{err});
        return err;
    };
    defer monitor.deinit();
    
    // Record some allocation events
    const test_allocations = 10;
    for (0..test_allocations) |i| {
        const size = 1024 + i * 64;
        const address = 0x1000000 + i * size;
        const latency = 500 + @as(u32, @intCast(i)) * 10; // Simulate increasing latency
        
        monitor.recordAllocation(size, address, latency);
    }
    
    // Give the monitor thread time to process events
    std.time.sleep(100_000_000); // 100ms
    
    // Get current metrics
    const metrics = monitor.getCurrentMetrics();
    
    // Basic validation of metrics
    try testing.expect(metrics.window_duration_us > 0);
    
    std.log.info("✅ Performance monitoring integration test passed");
}

test "Memory Pool with NUMA Integration" {
    const allocator = testing.allocator;
    
    // Initialize NUMA topology
    var numa = NUMATopology.init(allocator) catch {
        std.log.warn("⚠️ Skipping NUMA integration test - topology detection failed");
        return;
    };
    defer numa.deinit(allocator);
    
    // Create pool with NUMA awareness
    const config = PoolConfig{
        .strategy = .NUMAAware,
        .min_size = 1024 * 1024,
        .max_size = 16 * 1024 * 1024,
        .numa_node = -1, // Auto-detect
        .monitoring = false, // Disable for simplicity
    };
    
    var pool = MemoryPool.init(config, allocator, null) catch |err| {
        std.log.err("Failed to initialize NUMA-aware memory pool: {}", .{err});
        return err;
    };
    defer pool.deinit();
    
    // Test NUMA-aware allocations
    const allocations = [_]usize{ 4096, 8192, 16384 };
    var ptrs = std.ArrayList([]u8).init(allocator);
    defer ptrs.deinit();
    
    for (allocations) |size| {
        const allocation = pool.alloc(size) catch |err| {
            std.log.err("Failed NUMA-aware allocation of {} bytes: {}", .{ size, err });
            return err;
        };
        
        try testing.expect(allocation.len == size);
        try ptrs.append(allocation);
    }
    
    // Free allocations
    for (ptrs.items) |ptr| {
        pool.free(ptr);
    }
    
    std.log.info("✅ NUMA-aware memory pool integration test passed");
}

test "Memory Pool Performance Validation" {
    const allocator = testing.allocator;
    
    const config = PoolConfig{
        .strategy = .SizeClass,
        .min_size = 1024 * 1024,
        .max_size = 16 * 1024 * 1024,
        .thread_local_cache = true,
        .monitoring = true,
    };
    
    var pool = MemoryPool.init(config, allocator, null) catch |err| {
        std.log.err("Failed to initialize pool for performance test: {}", .{err});
        return err;
    };
    defer pool.deinit();
    
    // Performance test: allocate and free many small objects
    const iterations = 1000;
    const size = 256;
    
    const start_time = std.time.nanoTimestamp();
    
    for (0..iterations) |_| {
        const allocation = pool.alloc(size) catch |err| {
            std.log.err("Performance test allocation failed: {}", .{err});
            return err;
        };
        
        // Touch the memory
        @memset(allocation, 0xBB);
        
        // Immediate free to test allocation/deallocation performance
        pool.free(allocation);
    }
    
    const end_time = std.time.nanoTimestamp();
    const duration_ns = @as(u64, @intCast(end_time - start_time));
    const ops_per_sec = (@as(f64, @floatFromInt(iterations)) * 1_000_000_000.0) / @as(f64, @floatFromInt(duration_ns));
    
    std.log.info("Performance test: {d:.0} alloc/free pairs per second", .{ops_per_sec});
    
    // Verify we achieve reasonable performance (at least 10K ops/sec)
    try testing.expect(ops_per_sec > 10000.0);
    
    // Check pool statistics
    const stats = pool.getStats();
    try testing.expect(stats.total_allocs.load(.acquire) == iterations);
    try testing.expect(stats.total_frees.load(.acquire) == iterations);
    try testing.expect(stats.active_allocs.load(.acquire) == 0);
    
    std.log.info("✅ Memory pool performance validation passed");
}

test "C API Export Validation" {
    // Test that C API functions are properly exported
    const pool_config = PoolConfig{
        .strategy = .SizeClass,
        .min_size = 1024 * 1024,
        .max_size = 16 * 1024 * 1024,
    };
    
    // Test pool creation and destruction via C API
    const pool_ptr = @import("src-zig/memory_pool_system.zig").cursed_memory_pool_create(&pool_config);
    try testing.expect(pool_ptr != null);
    
    if (pool_ptr) |pool| {
        // Test allocation via C API
        const ptr = @import("src-zig/memory_pool_system.zig").cursed_memory_pool_alloc(pool, 1024);
        try testing.expect(ptr != null);
        
        if (ptr) |allocation| {
            // Test deallocation via C API
            @import("src-zig/memory_pool_system.zig").cursed_memory_pool_free(pool, allocation, 1024);
        }
        
        // Test pool destruction via C API
        @import("src-zig/memory_pool_system.zig").cursed_memory_pool_destroy(pool);
    }
    
    std.log.info("✅ C API export validation passed");
}
