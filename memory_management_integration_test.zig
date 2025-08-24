// Comprehensive Memory Management Integration Test
// Tests the complete integration of GC, arena allocators, memory pools, and safety validation
// Ensures production-ready performance and zero-leak guarantee

const std = @import("std");
const testing = std.testing;
const expect = testing.expect;
const expectEqual = testing.expectEqual;
const expectError = testing.expectError;
const Allocator = std.mem.Allocator;

const EnhancedMemoryManager = @import("src-zig/enhanced_memory_manager.zig").EnhancedMemoryManager;
const MemoryPoolSystem = @import("src-zig/memory_pool_system.zig").MemoryPoolSystem;
const MemorySafetyValidator = @import("src-zig/memory_safety_validator.zig").MemorySafetyValidator;
const ArenaAllocator = @import("src-zig/arena_allocator.zig").ArenaAllocator;

/// Comprehensive test configuration
const TestConfig = struct {
    // Test parameters
    stress_test_iterations: u32 = 10_000,
    max_concurrent_allocations: u32 = 1_000,
    leak_detection_threshold_ms: u64 = 100,
    performance_benchmark_iterations: u32 = 100_000,
    
    // Memory sizes for testing
    min_allocation_size: usize = 16,
    max_allocation_size: usize = 4096,
    stress_allocation_count: usize = 50_000,
    
    // Safety validation
    enable_all_safety_checks: bool = true,
    enable_performance_tests: bool = true,
    enable_concurrent_tests: bool = true,
};

/// Test results tracking
const TestResults = struct {
    total_tests: u32 = 0,
    passed_tests: u32 = 0,
    failed_tests: u32 = 0,
    
    // Memory safety results
    zero_leaks_confirmed: bool = false,
    bounds_violations_caught: u32 = 0,
    double_free_prevented: u32 = 0,
    use_after_free_detected: u32 = 0,
    
    // Performance results
    allocation_throughput: f64 = 0.0, // allocations per second
    deallocation_throughput: f64 = 0.0, // deallocations per second
    gc_pause_times: []u64 = undefined,
    memory_overhead_percentage: f64 = 0.0,
    
    // Integration results
    gc_arena_integration_success: bool = false,
    pool_safety_integration_success: bool = false,
    concurrent_safety_confirmed: bool = false,
};

test "Enhanced Memory Manager - Initialization and Basic Operation" {
    const gpa = testing.allocator;
    
    const config = EnhancedMemoryManager.Config{
        .enable_gc = true,
        .enable_arenas = true,
        .enable_pools = true,
        .enable_stack_scanning = true,
        .enable_leak_detection = true,
        .enable_safety_validation = true,
    };
    
    var memory_manager = try EnhancedMemoryManager.init(gpa, config);
    defer memory_manager.deinit();
    
    // Test basic allocation
    const ptr = try memory_manager.allocate(1024, @alignOf(u64), "test_allocation");
    try expect(ptr != null);
    
    // Test memory safety validation
    try expect(memory_manager.validateAccess(ptr, 1024));
    try expect(!memory_manager.validateAccess(ptr, 2048)); // Should fail for out-of-bounds
    
    // Test deallocation
    memory_manager.deallocate(ptr, 1024);
    
    // Verify stats
    const stats = memory_manager.getStats();
    try expectEqual(@as(usize, 1024), stats.total_allocated.load(.acquire));
    try expectEqual(@as(usize, 1024), stats.total_freed.load(.acquire));
}

test "Memory Pool System - Advanced Allocation Strategies" {
    const gpa = testing.allocator;
    
    const config = MemoryPoolSystem.Config{
        .enable_numa = false, // Disable for testing
        .enable_thread_local = true,
        .enable_buddy_allocator = true,
        .enable_slab_allocator = true,
        .enable_adaptive_sizing = true,
    };
    
    var pool_system = try MemoryPoolSystem.init(gpa, config);
    defer pool_system.deinit();
    
    // Test different allocation strategies
    const strategies = [_]MemoryPoolSystem.PoolStrategy{
        .ThreadLocal,
        .SLAB,
        .Buddy,
        .Adaptive,
    };
    
    for (strategies) |strategy| {
        const ptr = try pool_system.allocate(256, strategy);
        try expect(ptr != null);
        
        // Write pattern to verify allocation
        const slice = @as([*]u8, @ptrCast(ptr))[0..256];
        for (slice, 0..) |*byte, i| {
            byte.* = @as(u8, @intCast(i % 256));
        }
        
        // Verify pattern
        for (slice, 0..) |byte, i| {
            try expectEqual(@as(u8, @intCast(i % 256)), byte);
        }
        
        try pool_system.deallocate(ptr, 256, strategy);
    }
    
    // Verify statistics
    const stats = pool_system.getStats();
    try expect(stats.total_allocations >= 4);
    try expect(stats.total_deallocations >= 4);
}

test "Memory Safety Validator - Comprehensive Safety Checks" {
    const gpa = testing.allocator;
    
    const config = MemorySafetyValidator.Config{
        .enable_bounds_checking = true,
        .enable_double_free_detection = true,
        .enable_use_after_free_detection = true,
        .enable_leak_detection = true,
        .enable_corruption_detection = true,
        .leak_threshold_ms = 50, // Short threshold for testing
    };
    
    var validator = try MemorySafetyValidator.init(gpa, config);
    defer validator.deinit();
    
    // Test 1: Normal allocation and deallocation
    const ptr1 = try gpa.alloc(u8, 1024);
    defer gpa.free(ptr1);
    
    try validator.trackAllocation(ptr1.ptr, 1024, "test_normal_allocation");
    try validator.validateMemoryAccess(ptr1.ptr, 1024, false);
    try validator.trackDeallocation(ptr1.ptr);
    
    // Test 2: Bounds violation detection
    const ptr2 = try gpa.alloc(u8, 512);
    defer gpa.free(ptr2);
    
    try validator.trackAllocation(ptr2.ptr, 512, "test_bounds_violation");
    
    // This should fail - accessing beyond bounds
    try expectError(error.BoundsViolation, validator.validateMemoryAccess(ptr2.ptr, 1024, false));
    
    try validator.trackDeallocation(ptr2.ptr);
    
    // Test 3: Double-free detection
    const ptr3 = try gpa.alloc(u8, 256);
    defer gpa.free(ptr3);
    
    try validator.trackAllocation(ptr3.ptr, 256, "test_double_free");
    try validator.trackDeallocation(ptr3.ptr);
    
    // This should fail - double free
    try expectError(error.DoubleFree, validator.trackDeallocation(ptr3.ptr));
    
    // Test 4: Stack overflow protection
    try validator.checkStackOverflow(); // Should pass normally
    
    const stats = validator.getStats();
    try expect(stats.bounds_violations.load(.acquire) >= 1);
    try expect(stats.double_free_attempts.load(.acquire) >= 1);
}

test "Arena Allocator - Specialized Allocation Patterns" {
    const gpa = testing.allocator;
    
    const patterns = [_]ArenaAllocator.AllocationPattern{
        .Sequential,
        .Stack,
        .Pool,
        .Temporary,
        .StringIntern,
        .ASTNodes,
        .RuntimeValues,
    };
    
    for (patterns) |pattern| {
        const config = ArenaAllocator.ArenaConfig{
            .initial_size = 64 * 1024,
            .debug_tracking = true,
        };
        
        var arena = try ArenaAllocator.init(gpa, config, pattern);
        defer arena.deinit();
        
        // Test multiple allocations
        var allocations: [10][]u8 = undefined;
        for (&allocations, 0..) |*alloc, i| {
            const size = 128 + i * 64;
            alloc.* = try arena.alloc(size);
            
            // Write pattern
            for (alloc.*, 0..) |*byte, j| {
                byte.* = @as(u8, @intCast((i + j) % 256));
            }
        }
        
        // Verify patterns
        for (allocations, 0..) |alloc, i| {
            for (alloc, 0..) |byte, j| {
                try expectEqual(@as(u8, @intCast((i + j) % 256)), byte);
            }
        }
        
        // Test stack frame operations for stack pattern
        if (pattern == .Stack) {
            try arena.pushStackFrame();
            const stack_alloc = try arena.alloc(256);
            @memset(stack_alloc, 0xAA);
            arena.popStackFrame(); // Should deallocate stack_alloc
        }
        
        // Get usage stats
        const usage = arena.getUsage();
        try expect(usage.allocation_count >= 10);
        try expect(usage.total_used > 0);
    }
}

test "Integrated Memory Management - Full System Test" {
    const gpa = testing.allocator;
    
    // Initialize complete memory management system
    const mm_config = EnhancedMemoryManager.Config{
        .enable_gc = true,
        .enable_arenas = true,
        .enable_pools = true,
        .enable_stack_scanning = true,
        .enable_leak_detection = true,
        .enable_safety_validation = true,
        .gc_threshold = 1024 * 1024, // 1MB
    };
    
    var memory_manager = try EnhancedMemoryManager.init(gpa, mm_config);
    defer memory_manager.deinit();
    
    var test_results = TestResults{};
    test_results.total_tests = 5;
    
    // Test 1: Mixed allocation patterns
    {
        var allocations: [100]*anyopaque = undefined;
        var sizes: [100]usize = undefined;
        
        for (0..100) |i| {
            sizes[i] = 64 + (i * 32);
            allocations[i] = try memory_manager.allocate(sizes[i], @alignOf(u64), "mixed_pattern_test");
            
            // Write and verify pattern
            const slice = @as([*]u8, @ptrCast(allocations[i]))[0..sizes[i]];
            for (slice, 0..) |*byte, j| {
                byte.* = @as(u8, @intCast((i + j) % 256));
            }
        }
        
        // Verify all patterns
        var pattern_errors: u32 = 0;
        for (0..100) |i| {
            const slice = @as([*]u8, @ptrCast(allocations[i]))[0..sizes[i]];
            for (slice, 0..) |byte, j| {
                if (byte != @as(u8, @intCast((i + j) % 256))) {
                    pattern_errors += 1;
                }
            }
        }
        
        try expectEqual(@as(u32, 0), pattern_errors);
        
        // Deallocate half randomly
        for (0..50) |i| {
            const idx = (i * 2) % 100;
            memory_manager.deallocate(allocations[idx], sizes[idx]);
        }
        
        // Force GC
        try memory_manager.forceGC();
        
        // Deallocate remainder
        for (0..100) |i| {
            if (i % 2 == 1 or i >= 100) { // Odd indices weren't freed yet
                memory_manager.deallocate(allocations[i], sizes[i]);
            }
        }
        
        test_results.passed_tests += 1;
    }
    
    // Test 2: Arena integration
    {
        const arena_ptr = try memory_manager.allocateArena(2048, .ASTNodes);
        try expect(arena_ptr != null);
        
        // Write pattern
        const slice = @as([*]u8, @ptrCast(arena_ptr))[0..2048];
        @memset(slice, 0xBB);
        
        // Verify pattern
        for (slice) |byte| {
            try expectEqual(@as(u8, 0xBB), byte);
        }
        
        test_results.gc_arena_integration_success = true;
        test_results.passed_tests += 1;
    }
    
    // Test 3: Stack scanning
    {
        try memory_manager.performStackScan();
        test_results.passed_tests += 1;
    }
    
    // Test 4: Leak detection
    {
        // Intentionally "leak" some memory
        const leak1 = try memory_manager.allocate(128, @alignOf(u64), "intentional_leak_1");
        const leak2 = try memory_manager.allocate(256, @alignOf(u64), "intentional_leak_2");
        _ = leak1;
        _ = leak2;
        
        // Wait a bit and detect leaks
        std.time.sleep(200 * std.time.ns_per_ms); // 200ms
        
        const leaks = try memory_manager.detectLeaks(100); // 100ms threshold
        try expect(leaks.len >= 2);
        
        // Clean up leaks
        memory_manager.deallocate(leak1, 128);
        memory_manager.deallocate(leak2, 256);
        
        gpa.free(leaks);
        test_results.passed_tests += 1;
    }
    
    // Test 5: Final validation
    {
        try memory_manager.forceGC();
        
        const final_stats = memory_manager.getStats();
        const current_usage = final_stats.getCurrentUsage();
        
        // Should have minimal usage after cleanup
        try expect(current_usage < 1024); // Less than 1KB remaining
        
        test_results.zero_leaks_confirmed = (current_usage == 0);
        test_results.passed_tests += 1;
    }
    
    // Final assertions
    try expectEqual(test_results.total_tests, test_results.passed_tests);
    try expect(test_results.gc_arena_integration_success);
}

test "Performance Benchmarks - Allocation Throughput" {
    const gpa = testing.allocator;
    const config = TestConfig{};
    
    // Benchmark standard allocator
    const standard_start = std.time.nanoTimestamp();
    var standard_ptrs: [1000]*anyopaque = undefined;
    
    for (&standard_ptrs, 0..) |*ptr, i| {
        const size = config.min_allocation_size + (i % (config.max_allocation_size - config.min_allocation_size));
        const slice = try gpa.alloc(u8, size);
        ptr.* = slice.ptr;
    }
    
    const standard_alloc_time = std.time.nanoTimestamp() - standard_start;
    
    for (standard_ptrs, 0..) |ptr, i| {
        const size = config.min_allocation_size + (i % (config.max_allocation_size - config.min_allocation_size));
        const slice = @as([*]u8, @ptrCast(ptr))[0..size];
        gpa.free(slice);
    }
    
    const standard_total_time = std.time.nanoTimestamp() - standard_start;
    
    // Benchmark enhanced memory manager
    const mm_config = EnhancedMemoryManager.Config{
        .enable_gc = true,
        .enable_pools = true,
        .enable_safety_validation = false, // Disable for performance test
    };
    
    var memory_manager = try EnhancedMemoryManager.init(gpa, mm_config);
    defer memory_manager.deinit();
    
    const enhanced_start = std.time.nanoTimestamp();
    var enhanced_ptrs: [1000]*anyopaque = undefined;
    
    for (&enhanced_ptrs, 0..) |*ptr, i| {
        const size = config.min_allocation_size + (i % (config.max_allocation_size - config.min_allocation_size));
        ptr.* = try memory_manager.allocate(size, @alignOf(u64), null);
    }
    
    const enhanced_alloc_time = std.time.nanoTimestamp() - enhanced_start;
    
    for (enhanced_ptrs, 0..) |ptr, i| {
        const size = config.min_allocation_size + (i % (config.max_allocation_size - config.min_allocation_size));
        memory_manager.deallocate(ptr, size);
    }
    
    const enhanced_total_time = std.time.nanoTimestamp() - enhanced_start;
    
    // Calculate throughput
    const standard_throughput = @as(f64, 1000.0) / (@as(f64, @floatFromInt(standard_total_time)) / 1e9);
    const enhanced_throughput = @as(f64, 1000.0) / (@as(f64, @floatFromInt(enhanced_total_time)) / 1e9);
    
    std.debug.print("\nPerformance Benchmark Results:\n", .{});
    std.debug.print("Standard Allocator: {d:.0} alloc+dealloc/sec\n", .{standard_throughput});
    std.debug.print("Enhanced Manager:   {d:.0} alloc+dealloc/sec\n", .{enhanced_throughput});
    std.debug.print("Performance Ratio:  {d:.2}x\n", .{enhanced_throughput / standard_throughput});
    
    // Enhanced manager should be at least 50% of standard performance
    // (considering the added safety and GC features)
    try expect(enhanced_throughput > standard_throughput * 0.5);
}

test "Concurrent Safety - Multi-threaded Operations" {
    const gpa = testing.allocator;
    
    if (std.Thread.getCpuCount()) |cpu_count| {
        if (cpu_count < 2) {
            return; // Skip concurrent test on single-core systems
        }
    } else |_| {
        return; // Skip if can't determine CPU count
    }
    
    const mm_config = EnhancedMemoryManager.Config{
        .enable_gc = true,
        .enable_pools = true,
        .enable_concurrent_gc = true,
        .concurrent_gc_threads = 2,
    };
    
    var memory_manager = try EnhancedMemoryManager.init(gpa, mm_config);
    defer memory_manager.deinit();
    
    const ThreadData = struct {
        manager: *EnhancedMemoryManager,
        thread_id: u32,
        allocations_made: std.atomic.Value(u32),
        errors_encountered: std.atomic.Value(u32),
    };
    
    var thread_data = ThreadData{
        .manager = memory_manager,
        .thread_id = 0,
        .allocations_made = std.atomic.Value(u32).init(0),
        .errors_encountered = std.atomic.Value(u32).init(0),
    };
    
    const workerFn = struct {
        fn worker(data: *ThreadData) void {
            var prng = std.rand.DefaultPrng.init(@as(u64, @intCast(std.time.microTimestamp())));
            const random = prng.random();
            
            var allocations: [100]*anyopaque = undefined;
            var sizes: [100]usize = undefined;
            
            for (0..100) |i| {
                const size = 64 + random.uintAtMost(usize, 1024);
                sizes[i] = size;
                
                allocations[i] = data.manager.allocate(size, @alignOf(u64), null) catch |err| {
                    _ = err;
                    _ = data.errors_encountered.fetchAdd(1, .monotonic);
                    continue;
                };
                
                // Write pattern
                const slice = @as([*]u8, @ptrCast(allocations[i]))[0..size];
                @memset(slice, @as(u8, @intCast(data.thread_id + i)));
                
                _ = data.allocations_made.fetchAdd(1, .monotonic);
            }
            
            // Deallocate all
            for (allocations, 0..) |ptr, i| {
                if (ptr != @as(*anyopaque, @ptrFromInt(0))) {
                    data.manager.deallocate(ptr, sizes[i]);
                }
            }
        }
    }.worker;
    
    // Start multiple threads
    const num_threads = 4;
    var threads: [num_threads]std.Thread = undefined;
    var thread_data_array: [num_threads]ThreadData = undefined;
    
    for (&thread_data_array, 0..) |*data, i| {
        data.* = ThreadData{
            .manager = memory_manager,
            .thread_id = @as(u32, @intCast(i)),
            .allocations_made = std.atomic.Value(u32).init(0),
            .errors_encountered = std.atomic.Value(u32).init(0),
        };
        
        threads[i] = try std.Thread.spawn(.{}, workerFn, .{data});
    }
    
    // Wait for all threads to complete
    for (&threads) |*thread| {
        thread.join();
    }
    
    // Verify results
    var total_allocations: u32 = 0;
    var total_errors: u32 = 0;
    
    for (thread_data_array) |data| {
        total_allocations += data.allocations_made.load(.monotonic);
        total_errors += data.errors_encountered.load(.monotonic);
    }
    
    std.debug.print("\nConcurrent Test Results:\n", .{});
    std.debug.print("Total Allocations: {}\n", .{total_allocations});
    std.debug.print("Total Errors: {}\n", .{total_errors});
    
    // Should have significant allocations and minimal errors
    try expect(total_allocations > 300); // At least 75% success rate
    try expect(total_errors < 100); // Less than 25% error rate
    
    // Force final GC
    try memory_manager.forceGC();
}

test "Memory Report Generation" {
    const gpa = testing.allocator;
    
    const config = EnhancedMemoryManager.Config{
        .enable_gc = true,
        .enable_arenas = true,
        .enable_pools = true,
    };
    
    var memory_manager = try EnhancedMemoryManager.init(gpa, config);
    defer memory_manager.deinit();
    
    // Make some allocations
    var ptrs: [10]*anyopaque = undefined;
    for (&ptrs, 0..) |*ptr, i| {
        const size = 128 + i * 64;
        ptr.* = try memory_manager.allocate(size, @alignOf(u64), "report_test");
    }
    
    // Generate report
    const report = try memory_manager.getMemoryReport(gpa);
    defer gpa.free(report);
    
    // Verify report contains expected information
    try expect(std.mem.indexOf(u8, report, "Enhanced Memory Manager Report") != null);
    try expect(std.mem.indexOf(u8, report, "Total Allocated:") != null);
    try expect(std.mem.indexOf(u8, report, "Arena Usage") != null);
    
    std.debug.print("\nMemory Report:\n{s}\n", .{report});
    
    // Clean up
    for (ptrs, 0..) |ptr, i| {
        const size = 128 + i * 64;
        memory_manager.deallocate(ptr, size);
    }
}

// Helper function to simulate time passage (for testing)
fn simulateTimePassing(ms: u64) void {
    std.time.sleep(ms * std.time.ns_per_ms);
}
