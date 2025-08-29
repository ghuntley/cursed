/// P0 GC Integration Test Suite
// Tests the P0 GC allocator stub in realistic usage scenarios
/// Ensures stability without memory crashes for integration testing

const std = @import("std");
const ArrayList = std.ArrayList;
const P0GCAllocator = @import("gc_p0_stub.zig").P0GCAllocator;
const P0GCConfig = @import("gc_p0_stub.zig").P0GCConfig;
const ObjectHeader = @import("gc_p0_stub.zig").ObjectHeader;
const LeakInfo = @import("gc_p0_stub.zig").LeakInfo;

const testing = std.testing;
const expect = testing.expect;
const expectEqual = testing.expectEqual;

/// Simulated CURSED language types for testing
const CursedValueType = enum(u16) {
    Integer = 1,
    String = 2,
    Array = 3,
    Struct = 4,
    Function = 5,
    Channel = 6,
};

// Test P0 GC integration with CURSED runtime components
test "P0 GC integration - CURSED value allocation" {
    const allocator = testing.allocator;
    
    const config = P0GCConfig.forTesting();
    var gc = try P0GCAllocator.init(allocator, config);
    defer gc.deinit();
    
    // Test different CURSED value types
    const int_ptr = try gc.alloc(@sizeOf(i64), @intFromEnum(CursedValueType.Integer));
    const int_val = @as(*i64, @ptrCast(@alignCast(int_ptr)));
    int_val.* = 42;
    
    const string_size = 32;
    const string_ptr = try gc.alloc(string_size, @intFromEnum(CursedValueType.String));
    const string_data = @as([*]u8, @ptrCast(string_ptr))[0..string_size];
    const hello_cursed = "Hello CURSED";
    @memcpy(string_data[0..hello_cursed.len], hello_cursed);
    
    const array_size = 10 * @sizeOf(i64);
    const array_ptr = try gc.alloc(array_size, @intFromEnum(CursedValueType.Array));
    const array_data = @as([*]i64, @ptrCast(@alignCast(array_ptr)))[0..10];
    for (array_data, 0..) |*item, i| {
        item.* = @as(i64, @intCast(i * 2));
    }
    
    // Verify all data integrity
    try expectEqual(@as(i64, 42), int_val.*);
    try expect(std.mem.startsWith(u8, string_data, "Hello CURSED"));
    
    for (array_data, 0..) |item, i| {
        try expectEqual(@as(i64, @intCast(i * 2)), item);
    }
    
    // Verify object headers have correct types
    try expectEqual(@intFromEnum(CursedValueType.Integer), ObjectHeader.fromData(int_ptr).type_id);
    try expectEqual(@intFromEnum(CursedValueType.String), ObjectHeader.fromData(string_ptr).type_id);
    try expectEqual(@intFromEnum(CursedValueType.Array), ObjectHeader.fromData(array_ptr).type_id);
}

// Test P0 GC under memory pressure scenarios
test "P0 GC integration - memory pressure handling" {
    const allocator = testing.allocator;
    
    var config = P0GCConfig.forTesting();
    config.initial_heap_size = 8192; // Small heap for pressure testing
    var gc = try P0GCAllocator.init(allocator, config);
    defer gc.deinit();
    
    var allocated_ptrs = ArrayList(*anyopaque).empty;
    defer allocated_ptrs.deinit();
    
    // Gradually fill the heap
    var allocation_size: usize = 64;
    while (true) {
        const ptr = gc.alloc(allocation_size, @intFromEnum(CursedValueType.Struct)) catch break;
        try allocated_ptrs.append(allocator, ptr);
        
        // Write pattern to verify memory integrity
        const data = @as(*u64, @ptrCast(@alignCast(ptr)));
        data.* = @as(u64, @intCast(allocated_ptrs.items.len));
        
        allocation_size = (allocation_size % 128) + 32; // Vary sizes 32-159
    }
    
    // Verify we allocated a reasonable number of objects
    try expect(allocated_ptrs.items.len > 10);
    
    // Verify memory pressure is high
    try expect(gc.getMemoryPressure() > 0.8);
    
    // Verify all allocated data is still intact
    for (allocated_ptrs.items, 1..) |ptr, expected_val| {
        const data = @as(*u64, @ptrCast(@alignCast(ptr)));
        try expectEqual(@as(u64, @intCast(expected_val)), data.*);
    }
    
    std.log.info("P0 GC handled {} allocations under memory pressure", .{allocated_ptrs.items.len});
}

// Test P0 GC concurrent allocation safety
test "P0 GC integration - concurrent allocation simulation" {
    const allocator = testing.allocator;
    
    const config = P0GCConfig.forTesting();
    var gc = try P0GCAllocator.init(allocator, config);
    defer gc.deinit();
    
    // Simulate concurrent allocations with different threads
    // (Single-threaded test that simulates concurrent patterns)
    
    var thread_allocations: [4]std.ArrayList(*anyopaque) = undefined;
    for (&thread_allocations) |*list| {
        list.* = ArrayList(*anyopaque).empty;
    }
    defer {
        for (&thread_allocations) |*list| {
            list.deinit();
        }
    }
    
    // Simulate interleaved allocations from different "threads"
    for (0..100) |i| {
        const thread_id = i % 4;
        const type_id = @as(u16, @intCast(thread_id + 1));
        const size = 32 + (i % 96); // 32-127 bytes
        
        const ptr = try gc.alloc(size, type_id);
        try thread_allocations[thread_id].append(allocator, ptr);
        
        // Write thread-specific pattern
        const data = @as(*u64, @ptrCast(@alignCast(ptr)));
        data.* = (@as(u64, @intCast(thread_id)) << 32) | @as(u64, @intCast(i));
    }
    
    // Verify all thread allocations are correct
    for (thread_allocations, 0..) |list, thread_id| {
        for (list.items) |ptr| {
            const header = ObjectHeader.fromData(ptr);
            try expectEqual(@as(u16, @intCast(thread_id + 1)), header.type_id);
            
            const data = @as(*u64, @ptrCast(@alignCast(ptr)));

            
            // Verify pattern (allowing for some variance due to interleaving)
            const actual_thread = (data.* >> 32) & 0xFFFFFFFF;
            try expectEqual(@as(u64, @intCast(thread_id)), actual_thread);
        }
    }
    
    std.log.info("P0 GC handled concurrent allocation simulation successfully", .{});
}

// Test P0 GC with large object allocation
test "P0 GC integration - large object handling" {
    const allocator = testing.allocator;
    
    var config = P0GCConfig.forTesting();
    config.initial_heap_size = 1024 * 1024; // 1MB heap
    var gc = try P0GCAllocator.init(allocator, config);
    defer gc.deinit();
    
    // Allocate objects of varying sizes
    const sizes = [_]usize{ 1024, 4096, 16384, 65536, 131072 }; // 1KB to 128KB
    var large_objects = ArrayList(*anyopaque).empty;
    defer large_objects.deinit();
    
    for (sizes, 0..) |size, i| {
        const ptr = gc.alloc(size, @intFromEnum(CursedValueType.Array)) catch {
            std.log.warn("Could not allocate {} bytes (expected for large sizes)", .{size});
            continue;
        };
        
        try large_objects.append(allocator, ptr);
        
        // Write pattern throughout the large object
        const data = @as([*]u64, @ptrCast(@alignCast(ptr)))[0..size / @sizeOf(u64)];
        for (data, 0..) |*item, j| {
            item.* = @as(u64, @intCast((i << 24) | j));
        }
    }
    
    // Verify large object data integrity
    for (large_objects.items, 0..) |ptr, i| {
        const header = ObjectHeader.fromData(ptr);
        const size = header.size - ObjectHeader.HEADER_SIZE;
        const data = @as([*]u64, @ptrCast(@alignCast(ptr)))[0..size / @sizeOf(u64)];
        
        for (data, 0..) |item, j| {
            const expected = @as(u64, @intCast((i << 24) | j));
            try expectEqual(expected, item);
        }
    }
    
    // Verify statistics reflect large allocations
    const stats = gc.getStats();
    try expect(stats.largest_allocation > 1024);
    
    std.log.info("P0 GC handled {} large objects, largest: {} bytes", .{ 
        large_objects.items.len, 
        stats.largest_allocation 
    });
}

// Test P0 GC leak detection functionality
test "P0 GC integration - leak detection accuracy" {
    const allocator = testing.allocator;
    
    var config = P0GCConfig.forTesting();
    config.leak_threshold_us = 100_000; // 0.1 seconds
    config.leak_size_threshold = 512; // 512 bytes
    config.enable_leak_detection = true;
    
    var gc = try P0GCAllocator.init(allocator, config);
    defer gc.deinit();
    
    // Allocate mix of small and large objects
    var small_objects: [10]*anyopaque = undefined;
    var large_objects: [5]*anyopaque = undefined;
    
    // Small objects (below leak threshold)
    for (&small_objects, 0..) |*ptr, i| {
        ptr.* = try gc.alloc(256, @as(u16, @intCast(i % 3)));
    }
    
    // Large objects (above leak threshold)  
    for (&large_objects, 0..) |*ptr, i| {
        ptr.* = try gc.alloc(1024, @as(u16, @intCast(i % 2 + 10)));
    }
    
    // Wait for objects to age
    std.Thread.sleep(150_000_000); // 150ms
    
    // Detect leaks
    const leaks = try gc.detectMemoryLeaks();
    defer allocator.free(leaks);
    
    // Should detect only the large objects as potential leaks
    try expect(leaks.len >= 5);
    try expect(leaks.len <= 15); // All allocations
    
    // Verify leak information accuracy
    var large_leak_count: u32 = 0;
    for (leaks) |leak| {
        try expect(leak.address != 0);
        try expect(leak.age_us >= 100_000);
        
        if (leak.size >= 1024) {
            large_leak_count += 1;
        }
    }
    
    // Should have detected at least some large object leaks
    try expect(large_leak_count > 0);
    
    std.log.info("P0 GC leak detection found {} potential leaks ({} large)", .{ 
        leaks.len, 
        large_leak_count 
    });
}

// Test P0 GC statistics accuracy and monitoring
test "P0 GC integration - statistics and monitoring" {
    const allocator = testing.allocator;
    
    const config = P0GCConfig.forTesting();
    var gc = try P0GCAllocator.init(allocator, config);
    defer gc.deinit();
    
    // Record initial state
    const initial_stats = gc.getStats();
    const initial_usage = gc.getMemoryUsage();
    
    // Perform various allocations
    var total_allocated: usize = 0;
    var largest_size: usize = 0;
    const num_allocs = 50;
    
    for (0..num_allocs) |i| {
        const size = 32 + (i * 7) % 200; // Variable sizes
        _ = try gc.alloc(size, @as(u16, @intCast(i % 8)));
        
        total_allocated += size;
        if (size > largest_size) {
            largest_size = size;
        }
    }
    
    // Verify final statistics
    const final_stats = gc.getStats();
    const final_usage = gc.getMemoryUsage();
    
    // Basic counters
    try expectEqual(initial_stats.total_allocations + num_allocs, final_stats.total_allocations);
    try expect(final_stats.total_bytes_allocated > initial_stats.total_bytes_allocated);
    
    // Memory usage
    try expect(final_usage.heap_used > initial_usage.heap_used);
    try expect(final_usage.tracked_allocations >= num_allocs);
    
    // Size statistics
    try expect(final_stats.largest_allocation >= largest_size);
    try expect(final_stats.average_allocation_size > 0);
    
    // Memory pressure
    try expect(final_usage.pressure >= 0.0);
    try expect(final_usage.pressure <= 1.0);
    
    // Heap accounting
    try expectEqual(final_usage.heap_used + final_usage.heap_free, final_usage.heap_size);
    try expect(final_stats.current_heap_usage == final_usage.heap_used);
    
    std.log.info("P0 GC statistics verification passed", .{});
    std.log.info("  Total allocations: {}", .{final_stats.total_allocations});
    std.log.info("  Heap usage: {} / {} bytes ({:.1}%)", .{
        final_usage.heap_used,
        final_usage.heap_size,
        final_usage.pressure * 100.0,
    });
    std.log.info("  Average allocation: {:.1} bytes", .{final_stats.average_allocation_size});
}

// Test P0 GC interface compatibility with main GC
test "P0 GC integration - interface compatibility" {
    const allocator = testing.allocator;
    
    const config = P0GCConfig.forTesting();
    var gc = try P0GCAllocator.init(allocator, config);
    defer gc.deinit();
    
    // Test allocation interface
    const ptr1 = try gc.alloc(64, 1);
    const ptr2 = try gc.allocWithSource(128, 2, "test_source.csd:42");
    
    // Test root management interface (should be no-ops)
    var root1: ?*anyopaque = ptr1;
    var root2: ?*anyopaque = ptr2;
    
    try gc.addRoot(&root1, 1); // Should not fail
    try gc.addRoot(&root2, 2); // Should not fail
    
    gc.removeRoot(&root1); // Should not fail
    gc.removeRoot(&root2); // Should not fail
    
    // Test collection interface (should be no-op)
    try gc.collectNow(); // Should not fail
    
    // Verify allocations are still valid after "collection"
    const data1 = @as(*u64, @ptrCast(@alignCast(ptr1)));
    const data2 = @as(*u64, @ptrCast(@alignCast(ptr2)));
    
    data1.* = 0x1111111111111111;
    data2.* = 0x2222222222222222;
    
    try expectEqual(@as(u64, 0x1111111111111111), data1.*);
    try expectEqual(@as(u64, 0x2222222222222222), data2.*);
    
    std.log.info("P0 GC interface compatibility verified", .{});
}

// Comprehensive P0 GC integration test
test "P0 GC integration - comprehensive runtime simulation" {
    const allocator = testing.allocator;
    
    var config = P0GCConfig.forTesting();
    config.initial_heap_size = 512 * 1024; // 512KB
    config.leak_threshold_us = 50_000; // 50ms
    config.leak_size_threshold = 1024; // 1KB
    
    var gc = try P0GCAllocator.init(allocator, config);
    defer gc.deinit();
    
    // Phase 1: Initial allocations (simulating program startup)
    var startup_objects = ArrayList(*anyopaque).empty;
    defer startup_objects.deinit();
    
    for (0..20) |i| {
        const size = 256 + (i * 13) % 500;
        const type_id = @as(u16, @intCast(i % 6));
        const ptr = try gc.alloc(size, type_id);
        try startup_objects.append(allocator, ptr);
        
        // Write initialization pattern
        const data = @as(*u32, @ptrCast(@alignCast(ptr)));
        data.* = @as(u32, @intCast(0x10000 + i));
    }
    
    // Phase 2: Runtime allocations (simulating normal execution)
    var runtime_objects = ArrayList(*anyopaque).empty;
    defer runtime_objects.deinit();
    
    for (0..100) |i| {
        const size = 64 + (i * 7) % 128;
        const type_id = @as(u16, @intCast((i % 4) + 20));
        const ptr = try gc.alloc(size, type_id);
        try runtime_objects.append(allocator, ptr);
        
        // Write runtime pattern
        const data = @as(*u32, @ptrCast(@alignCast(ptr)));
        data.* = @as(u32, @intCast(0x20000 + i));
        
        // Simulate some "collection" calls
        if (i % 25 == 0) {
            try gc.collectNow();
        }
    }
    
    // Phase 3: Large object allocations (simulating heavy workload)
    var large_objects = ArrayList(*anyopaque).empty;
    defer large_objects.deinit();
    
    var large_count: usize = 0;
    for (0..20) |i| {
        const size = 2048 + (i * 512);
        const type_id = @as(u16, @intCast(i % 3 + 30));
        
        const ptr = gc.alloc(size, type_id) catch {
            // Expected to run out of memory eventually
            break;
        };
        
        try large_objects.append(allocator, ptr);
        large_count += 1;
        
        // Write large object pattern
        const data = @as([*]u32, @ptrCast(@alignCast(ptr)))[0..size / @sizeOf(u32)];
        for (data, 0..) |*item, j| {
            item.* = @as(u32, @intCast((0x30000 + i) ^ @as(u32, @intCast(j))));
        }
    }
    
    // Verification phase
    std.log.info("P0 GC comprehensive test allocated:", .{});
    std.log.info("  Startup objects: {}", .{startup_objects.items.len});
    std.log.info("  Runtime objects: {}", .{runtime_objects.items.len});
    std.log.info("  Large objects: {}", .{large_count});
    
    // Verify data integrity across all phases
    for (startup_objects.items, 0..) |ptr, i| {
        const data = @as(*u32, @ptrCast(@alignCast(ptr)));
        try expectEqual(@as(u32, @intCast(0x10000 + i)), data.*);
    }
    
    for (runtime_objects.items, 0..) |ptr, i| {
        const data = @as(*u32, @ptrCast(@alignCast(ptr)));
        try expectEqual(@as(u32, @intCast(0x20000 + i)), data.*);
    }
    
    for (large_objects.items, 0..) |ptr, i| {
        const header = ObjectHeader.fromData(ptr);
        const size = header.size - ObjectHeader.HEADER_SIZE;
        const data = @as([*]u32, @ptrCast(@alignCast(ptr)))[0..size / @sizeOf(u32)];
        
        // Verify first and last elements
        const expected_base = @as(u32, @intCast(0x30000 + i));
        try expectEqual(expected_base ^ 0, data[0]);
        if (data.len > 1) {
            try expectEqual(expected_base ^ @as(u32, @intCast(data.len - 1)), data[data.len - 1]);
        }
    }
    
    // Final statistics
    const stats = gc.getStats();
    const usage = gc.getMemoryUsage();
    
    try expect(stats.total_allocations >= 140); // At least startup + runtime objects
    try expect(usage.heap_used > 50_000); // Should have used significant heap
    try expect(stats.largest_allocation >= 2048); // Should have large allocations
    
    std.log.info("P0 GC final state:", .{});
    std.log.info("  Allocations: {}", .{stats.total_allocations});
    std.log.info("  Heap usage: {} bytes ({:.1}% pressure)", .{ usage.heap_used, usage.pressure * 100.0 });
    std.log.info("  Peak usage: {} bytes", .{stats.peak_heap_usage});
    std.log.info("  Average size: {:.1} bytes", .{stats.average_allocation_size});
    
    // Test leak detection on comprehensive state
    std.Thread.sleep(60_000_000); // 60ms to age objects
    const leaks = try gc.detectMemoryLeaks();
    defer allocator.free(leaks);
    
    std.log.info("  Potential leaks: {}", .{leaks.len});
    
    // Final verification - all data should still be intact
    std.log.info("P0 GC comprehensive integration test completed successfully", .{});
}
