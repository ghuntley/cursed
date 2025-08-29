const std = @import("std");
const testing = std.testing;
const gc_module = @import("gc.zig");
const GC = gc_module.GC;
const GCConfig = gc_module.GCConfig;
const WeakRef = gc_module.WeakRef;

// Test allocator for GC tests
const test_allocator = testing.allocator;

// Test object for GC testing
const TestObject = struct {
    value: i32,
    next: ?*TestObject,
    
    fn finalizer(object: *anyopaque) void {
        const test_obj = @as(*TestObject, @ptrCast(@alignCast(object)));
        std.log.info("Finalizing TestObject with value: {}", .{test_obj.value});
    }
};

// Test basic GC initialization and cleanup
test "gc basic initialization" {
    const config = GCConfig.default();
    const gc = try GC.init(test_allocator, config);
    defer gc.deinit();
    
    const stats = gc.getStats();
    try testing.expect(stats.total_allocations == 0);
    try testing.expect(stats.gc_cycles == 0);
    try testing.expect(stats.current_heap_size == 0);
}

// Test simple allocation and deallocation
test "gc simple allocation" {
    const config = GCConfig.default();
    const gc = try GC.init(test_allocator, config);
    defer gc.deinit();
    
    // Allocate a test object
    const obj_ptr = try gc.alloc(@sizeOf(TestObject), 1);
    const test_obj = @as(*TestObject, @ptrCast(@alignCast(obj_ptr)));
    test_obj.value = 42;
    test_obj.next = null;
    
    const stats = gc.getStats();
    try testing.expect(stats.total_allocations == 1);
    try testing.expect(stats.current_heap_size > 0);
    
    // Test object access
    try testing.expect(test_obj.value == 42);
}

// Test root registration and collection
test "gc root registration" {
    const config = GCConfig.default();
    const gc = try GC.init(test_allocator, config);
    defer gc.deinit();
    
    // Allocate test objects
    const obj1_ptr = try gc.alloc(@sizeOf(TestObject), 1);
    const obj1 = @as(*TestObject, @ptrCast(@alignCast(obj1_ptr)));
    obj1.value = 1;
    obj1.next = null;
    
    const obj2_ptr = try gc.alloc(@sizeOf(TestObject), 1);
    const obj2 = @as(*TestObject, @ptrCast(@alignCast(obj2_ptr)));
    obj2.value = 2;
    obj2.next = null;
    
    // Register obj1 as root, leave obj2 unrooted
    var root_ptr: ?*anyopaque = obj1_ptr;
    try gc.addRoot(&root_ptr, 1);
    
    // Force collection
    try gc.collectNow();
    
    // obj1 should survive (rooted), obj2 might be collected (unrooted)
    try testing.expect(obj1.value == 1);
    
    // Remove root
    gc.removeRoot(&root_ptr);
}

// Test linked object traversal
test "gc object traversal" {
    const config = GCConfig.default();
    const gc = try GC.init(test_allocator, config);
    defer gc.deinit();
    
    // Create linked list of objects
    const obj1_ptr = try gc.alloc(@sizeOf(TestObject), 1);
    const obj1 = @as(*TestObject, @ptrCast(@alignCast(obj1_ptr)));
    obj1.value = 1;
    
    const obj2_ptr = try gc.alloc(@sizeOf(TestObject), 1);
    const obj2 = @as(*TestObject, @ptrCast(@alignCast(obj2_ptr)));
    obj2.value = 2;
    obj2.next = null;
    
    obj1.next = obj2;
    
    // Register obj1 as root
    var root_ptr: ?*anyopaque = obj1_ptr;
    try gc.addRoot(&root_ptr, 1);
    
    // Force collection - both objects should survive due to reachability
    try gc.collectNow();
    
    try testing.expect(obj1.value == 1);
    try testing.expect(obj1.next != null);
    try testing.expect(obj1.next.?.value == 2);
    
    gc.removeRoot(&root_ptr);
}

// Test generational collection
test "gc generational collection" {
    var config = GCConfig.default();
    config.young_gen_size = 1024; // Small young generation for testing
    config.promotion_threshold = 2;
    
    const gc = try GC.init(test_allocator, config);
    defer gc.deinit();
    
    // Allocate several objects to trigger promotion
    var objects: [10]*TestObject = undefined;
    var roots: [10]?*anyopaque = undefined;
    
    for (0..10) |i| {
        const obj_ptr = try gc.alloc(@sizeOf(TestObject), 1);
        const obj = @as(*TestObject, @ptrCast(@alignCast(obj_ptr)));
        obj.value = @intCast(i);
        obj.next = null;
        
        objects[i] = obj;
        roots[i] = obj_ptr;
        try gc.addRoot(&roots[i], 1);
    }
    
    // Trigger several collections to test promotion
    for (0..3) |_| {
        try gc.collectNow();
    }
    
    const stats = gc.getStats();
    try testing.expect(stats.gc_cycles >= 3);
    try testing.expect(stats.promotions > 0);
    
    // Verify objects are still alive
    for (objects, 0..) |obj, i| {
        try testing.expect(obj.value == @as(i32, @intCast(i)));
    }
    
    // Clean up roots
    for (0..10) |i| {
        gc.removeRoot(&roots[i]);
    }
}

// Test weak references
test "gc weak references" {
    const config = GCConfig.default();
    const gc = try GC.init(test_allocator, config);
    defer gc.deinit();
    
    // Allocate test object
    const obj_ptr = try gc.alloc(@sizeOf(TestObject), 1);
    const obj = @as(*TestObject, @ptrCast(@alignCast(obj_ptr)));
    obj.value = 42;
    obj.next = null;
    
    // Create weak reference
    const weak_ref = try gc.createWeakRef(obj_ptr);
    
    // Object should be accessible via weak reference
    const weak_obj = weak_ref.get();
    try testing.expect(weak_obj != null);
    
    // Force collection (object is not rooted, so it should be collected)
    try gc.collectNow();
    
    // Weak reference should now return null
    const after_gc = weak_ref.get();
    try testing.expect(after_gc == null);
}

// Test finalization
test "gc finalization" {
    var config = GCConfig.default();
    config.enable_finalization = true;
    
    const gc = try GC.init(test_allocator, config);
    defer gc.deinit();
    
    // Allocate test object with finalizer
    const obj_ptr = try gc.alloc(@sizeOf(TestObject), 1);
    const obj = @as(*TestObject, @ptrCast(@alignCast(obj_ptr)));
    obj.value = 42;
    obj.next = null;
    
    // Register finalizer
    try gc.addFinalizer(obj_ptr, TestObject.finalizer);
    
    // Force collection (object is not rooted)
    try gc.collectNow();
    
    // Give finalization thread time to run
    std.Thread.sleep(50_000_000); // 50ms
    
    const stats = gc.getStats();
    try testing.expect(stats.finalized_objects > 0);
}

// Test write barriers for concurrent collection
test "gc write barriers" {
    var config = GCConfig.default();
    config.enable_write_barriers = true;
    
    const gc = try GC.init(test_allocator, config);
    defer gc.deinit();
    
    // Allocate test objects
    const obj1_ptr = try gc.alloc(@sizeOf(TestObject), 1);
    const obj1 = @as(*TestObject, @ptrCast(@alignCast(obj1_ptr)));
    obj1.value = 1;
    obj1.next = null;
    
    const obj2_ptr = try gc.alloc(@sizeOf(TestObject), 1);
    const obj2 = @as(*TestObject, @ptrCast(@alignCast(obj2_ptr)));
    obj2.value = 2;
    obj2.next = null;
    
    // Simulate pointer write with write barrier
    gc.writeBarrier(obj1_ptr, obj2_ptr);
    obj1.next = obj2;
    
    // Register root and collect
    var root_ptr: ?*anyopaque = obj1_ptr;
    try gc.addRoot(&root_ptr, 1);
    
    try gc.collectNow();
    
    // Both objects should survive
    try testing.expect(obj1.value == 1);
    try testing.expect(obj1.next != null);
    try testing.expect(obj1.next.?.value == 2);
    
    gc.removeRoot(&root_ptr);
}

// Test large object allocation
test "gc large objects" {
    const config = GCConfig.default();
    const gc = try GC.init(test_allocator, config);
    defer gc.deinit();
    
    // Allocate large object (should go directly to old generation)
    const large_size = 64 * 1024; // 64KB
    const large_obj = try gc.alloc(large_size, 2);
    
    // Fill with test data
    const data = @as([*]u8, @ptrCast(large_obj));
    for (0..large_size) |i| {
        data[i] = @intCast(i % 256);
    }
    
    // Verify data integrity
    for (0..large_size) |i| {
        try testing.expect(data[i] == @as(u8, @intCast(i % 256)));
    }
    
    const stats = gc.getStats();
    try testing.expect(stats.total_allocations == 1);
    try testing.expect(stats.total_bytes_allocated >= large_size);
}

// Stress test with many allocations
test "gc stress test" {
    var config = GCConfig.default();
    config.gc_trigger_threshold = 0.5; // Trigger GC more frequently
    
    const gc = try GC.init(test_allocator, config);
    defer gc.deinit();
    
    const num_objects = 1000;
    var live_objects = std.ArrayList(*TestObject){};
    defer live_objects.deinit();
    
    // Allocate many objects, keeping some as roots
    for (0..num_objects) |i| {
        const obj_ptr = try gc.alloc(@sizeOf(TestObject), 1);
        const obj = @as(*TestObject, @ptrCast(@alignCast(obj_ptr)));
        obj.value = @intCast(i);
        obj.next = null;
        
        // Keep every 10th object as a root
        if (i % 10 == 0) {
            try live_objects.append(allocator, obj);
            var root_ptr: ?*anyopaque = obj_ptr;
            try gc.addRoot(&root_ptr, 1);
        }
    }
    
    // Force several collections
    for (0..5) |_| {
        try gc.collectNow();
    }
    
    const stats = gc.getStats();
    try testing.expect(stats.total_allocations == num_objects);
    try testing.expect(stats.gc_cycles >= 5);
    
    // Verify live objects are still accessible
    for (live_objects.items, 0..) |obj, i| {
        try testing.expect(obj.value == @as(i32, @intCast(i * 10)));
    }
    
    // Clean up roots
    for (live_objects.items) |obj| {
        var root_ptr: ?*anyopaque = @ptrCast(obj);
        gc.removeRoot(&root_ptr);
    }
}

// Test heap growth and shrinking
test "gc heap management" {
    var config = GCConfig.default();
    config.initial_heap_size = 1024; // Very small initial heap
    
    const gc = try GC.init(test_allocator, config);
    defer gc.deinit();
    
    var objects = std.ArrayList(*TestObject){};
    defer objects.deinit();
    
    // Allocate objects to force heap growth
    for (0..100) |i| {
        const obj_ptr = gc.alloc(@sizeOf(TestObject), 1) catch |err| {
            if (err == error.OutOfMemory) {
                // Expected for small heap
                break;
            }
            return err;
        };
        
        const obj = @as(*TestObject, @ptrCast(@alignCast(obj_ptr)));
        obj.value = @intCast(i);
        obj.next = null;
        
        try objects.append(allocator, obj);
        var root_ptr: ?*anyopaque = obj_ptr;
        try gc.addRoot(&root_ptr, 1);
    }
    
    const stats_before = gc.getStats();
    
    // Force collection
    try gc.collectNow();
    
    const stats_after = gc.getStats();
    
    // Verify statistics
    try testing.expect(stats_after.gc_cycles > stats_before.gc_cycles);
    
    // Clean up roots
    for (objects.items) |obj| {
        var root_ptr: ?*anyopaque = @ptrCast(obj);
        gc.removeRoot(&root_ptr);
    }
}

// Test concurrent collection safety
test "gc concurrent safety" {
    var config = GCConfig.default();
    config.concurrent_threads = 2;
    config.enable_write_barriers = true;
    
    const gc = try GC.init(test_allocator, config);
    defer gc.deinit();
    
    const AllocatorThread = struct {
        gc: *GC,
        objects: std.ArrayList(*TestObject),
        
        fn run(self: *@This()) !void {
            for (0..100) |i| {
                const obj_ptr = try self.gc.alloc(@sizeOf(TestObject), 1);
                const obj = @as(*TestObject, @ptrCast(@alignCast(obj_ptr)));
                obj.value = @intCast(i);
                obj.next = null;
                
                try self.objects.append(allocator, obj);
                
                // Occasionally trigger collection
                if (i % 20 == 0) {
                    self.gc.triggerCollection();
                }
                
                // Small delay to allow other threads to work
                std.Thread.sleep(1_000_000); // 1ms
            }
        }
    };
    
    // Create allocator threads
    var thread1 = AllocatorThread{
        .gc = gc,
        .objects = .{},
    };
    defer thread1.objects.deinit();
    
    var thread2 = AllocatorThread{
        .gc = gc,
        .objects = .{},
    };
    defer thread2.objects.deinit();
    
    // Run threads concurrently
    const t1 = try std.Thread.spawn(.{}, AllocatorThread.run, .{&thread1});
    const t2 = try std.Thread.spawn(.{}, AllocatorThread.run, .{&thread2});
    
    t1.join();
    t2.join();
    
    // Force final collection
    try gc.collectNow();
    
    const stats = gc.getStats();
    try testing.expect(stats.total_allocations >= 200);
    try testing.expect(stats.gc_cycles > 0);
}

// Benchmark allocation performance
test "gc allocation benchmark" {
    const config = GCConfig.default();
    const gc = try GC.init(test_allocator, config);
    defer gc.deinit();
    
    const num_allocations = 10000;
    const start_time = std.time.nanoTimestamp();
    
    // Perform many allocations
    for (0..num_allocations) |i| {
        const obj_ptr = try gc.alloc(@sizeOf(TestObject), 1);
        const obj = @as(*TestObject, @ptrCast(@alignCast(obj_ptr)));
        obj.value = @intCast(i);
        obj.next = null;
    }
    
    const end_time = std.time.nanoTimestamp();
    const duration_ns = end_time - start_time;
    const allocations_per_second = @as(f64, @floatFromInt(num_allocations)) * 1e9 / @as(f64, @floatFromInt(duration_ns));
    
    std.log.info("GC Allocation Benchmark:", .{});
    std.log.info("  {} allocations in {} ns", .{ num_allocations, duration_ns });
    std.log.info("  {d:.0} allocations/second", .{allocations_per_second});
    
    const stats = gc.getStats();
    try testing.expect(stats.total_allocations == num_allocations);
}

// Benchmark collection performance
test "gc collection benchmark" {
    var config = GCConfig.default();
    config.young_gen_size = 1024 * 1024; // 1MB young generation
    
    const gc = try GC.init(test_allocator, config);
    defer gc.deinit();
    
    // Fill heap with objects
    const num_objects = 10000;
    var roots = std.ArrayList(?*anyopaque){};
    defer roots.deinit();
    
    for (0..num_objects) |i| {
        const obj_ptr = try gc.alloc(@sizeOf(TestObject), 1);
        const obj = @as(*TestObject, @ptrCast(@alignCast(obj_ptr)));
        obj.value = @intCast(i);
        obj.next = null;
        
        // Keep some objects alive
        if (i % 100 == 0) {
            try roots.append(allocator, obj_ptr);
            try gc.addRoot(&roots.items[roots.items.len - 1], 1);
        }
    }
    
    // Benchmark collection
    const start_time = std.time.nanoTimestamp();
    try gc.collectNow();
    const end_time = std.time.nanoTimestamp();
    
    const duration_ns = end_time - start_time;
    const duration_us = @divTrunc(duration_ns, 1000);
    
    std.log.info("GC Collection Benchmark:", .{});
    std.log.info("  Collection took {} μs", .{duration_us});
    
    const stats = gc.getStats();
    std.log.info("  Objects collected: young={}, old={}", .{ 
        stats.young_collections, stats.old_collections 
    });
    
    // Clean up roots
    for (roots.items) |root_ptr| {
        gc.removeRoot(&root_ptr);
    }
    
    // Verify collection was reasonably fast (less than 100ms for this test)
    try testing.expect(duration_us < 100_000);
}

// Test memory leak detection
test "gc memory leak detection" {
    const config = GCConfig.default();
    const gc = try GC.init(test_allocator, config);
    defer gc.deinit();
    
    _ = gc.getStats(); // Get baseline stats
    
    // Allocate many objects without keeping references
    for (0..1000) |i| {
        const obj_ptr = try gc.alloc(@sizeOf(TestObject), 1);
        const obj = @as(*TestObject, @ptrCast(@alignCast(obj_ptr)));
        obj.value = @intCast(i);
        obj.next = null;
        // No roots - these should be collectible
    }
    
    const after_alloc_stats = gc.getStats();
    try testing.expect(after_alloc_stats.total_allocations == 1000);
    
    // Force collection - all objects should be collected
    try gc.collectNow();
    
    const after_gc_stats = gc.getStats();
    
    // Heap should be mostly empty after collection
    const heap_usage = @as(f64, @floatFromInt(after_gc_stats.current_heap_size)) / 
                      @as(f64, @floatFromInt(after_alloc_stats.current_heap_size));
    
    std.log.info("Heap usage after collection: {d:.2}%", .{heap_usage * 100});
    
    // Most objects should have been collected
    try testing.expect(heap_usage < 0.1); // Less than 10% of peak usage
}

// Test GC integration with stack scanning
test "gc stack scanning" {
    const config = GCConfig.default();
    const gc = try GC.init(test_allocator, config);
    defer gc.deinit();
    
    // Allocate object and keep reference on "stack"
    const obj_ptr = try gc.alloc(@sizeOf(TestObject), 1);
    const obj = @as(*TestObject, @ptrCast(@alignCast(obj_ptr)));
    obj.value = 42;
    obj.next = null;
    
    // Simulate stack reference
    try gc.addStackRoot(obj_ptr);
    
    // Force collection - object should survive due to stack reference
    try gc.collectNow();
    
    // Object should still be accessible
    try testing.expect(obj.value == 42);
}
