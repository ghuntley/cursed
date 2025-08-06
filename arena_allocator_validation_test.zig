const std = @import("std");
const testing = std.testing;
const ArenaAllocator = @import("arena_allocator.zig").ArenaAllocator;
const CursedArenaManager = @import("arena_allocator.zig").CursedArenaManager;

test "ArenaAllocator sequential pattern" {
    var arena = try ArenaAllocator.init(
        testing.allocator,
        .{ .initial_size = 1024, .debug_tracking = true },
        .Sequential
    );
    defer arena.deinit();
    
    // Test basic allocation
    const ptr1 = try arena.alloc(100);
    const ptr2 = try arena.alloc(200);
    const ptr3 = try arena.alloc(50);
    
    try testing.expect(ptr1.len == 100);
    try testing.expect(ptr2.len == 200);
    try testing.expect(ptr3.len == 50);
    
    // Verify pointers are different and non-null
    try testing.expect(ptr1.ptr != ptr2.ptr);
    try testing.expect(ptr2.ptr != ptr3.ptr);
    try testing.expect(ptr1.ptr != null);
    try testing.expect(ptr2.ptr != null);
    try testing.expect(ptr3.ptr != null);
    
    // Test usage statistics
    const usage = arena.getUsage();
    try testing.expect(usage.total_used >= 350); // At least the allocated size
    try testing.expect(usage.allocation_count == 3);
    
    std.log.info("Sequential pattern test passed: allocated {} bytes in {} allocations", 
                .{usage.total_used, usage.allocation_count});
}

test "ArenaAllocator stack pattern" {
    var arena = try ArenaAllocator.init(
        testing.allocator,
        .{ .initial_size = 2048 },
        .Stack
    );
    defer arena.deinit();
    
    // Push stack frame
    try arena.pushStackFrame();
    
    const ptr1 = try arena.alloc(100);
    const ptr2 = try arena.alloc(200);
    
    // Use the pointers to avoid unused warnings
    try testing.expect(ptr1.len == 100);
    try testing.expect(ptr2.len == 200);
    
    const usage_before_pop = arena.getUsage();
    
    // Pop stack frame (should deallocate ptr1 and ptr2)
    arena.popStackFrame();
    
    const usage_after_pop = arena.getUsage();
    
    // After pop, used memory should be less (stack frame deallocated)
    try testing.expect(usage_after_pop.total_used < usage_before_pop.total_used);
    
    std.log.info("Stack pattern test passed: before pop {} bytes, after pop {} bytes", 
                .{usage_before_pop.total_used, usage_after_pop.total_used});
}

test "ArenaAllocator pool pattern" {
    var arena = try ArenaAllocator.init(
        testing.allocator,
        .{ .initial_size = 4096 },
        .Pool
    );
    defer arena.deinit();
    
    // First allocation sets pool object size
    const ptr1 = try arena.alloc(64);
    try testing.expect(ptr1.len == 64);
    
    // Subsequent allocations should be from the pool
    const ptr2 = try arena.alloc(32); // Should fit in 64-byte pool object
    const ptr3 = try arena.alloc(64);
    
    try testing.expect(ptr2.len == 32);
    try testing.expect(ptr3.len == 64);
    
    // Test freeing (should work for pool pattern)
    arena.free(ptr1);
    arena.free(ptr3);
    
    // Allocate again (should reuse freed objects)
    const ptr4 = try arena.alloc(60);
    try testing.expect(ptr4.len == 60);
    
    std.log.info("Pool pattern test passed");
}

test "ArenaAllocator alignment" {
    var arena = try ArenaAllocator.init(
        testing.allocator,
        .{ .alignment = 16 },
        .Sequential
    );
    defer arena.deinit();
    
    const ptr1 = try arena.allocAligned(100, 16);
    const ptr2 = try arena.allocAligned(50, 32);
    
    // Check alignment
    try testing.expect(@intFromPtr(ptr1.ptr) % 16 == 0);
    try testing.expect(@intFromPtr(ptr2.ptr) % 32 == 0);
    
    std.log.info("Alignment test passed: ptr1 aligned to {}, ptr2 aligned to {}", 
                .{@intFromPtr(ptr1.ptr) % 16, @intFromPtr(ptr2.ptr) % 32});
}

test "ArenaAllocator reset functionality" {
    var arena = try ArenaAllocator.init(
        testing.allocator,
        .{ .initial_size = 1024 },
        .Sequential
    );
    defer arena.deinit();
    
    // Allocate some memory
    _ = try arena.alloc(100);
    _ = try arena.alloc(200);
    _ = try arena.alloc(300);
    
    const usage_before_reset = arena.getUsage();
    try testing.expect(usage_before_reset.total_used >= 600);
    
    // Reset arena
    arena.reset();
    
    const usage_after_reset = arena.getUsage();
    try testing.expect(usage_after_reset.total_used == 0);
    try testing.expect(usage_after_reset.allocation_count == 0);
    
    // Allocate again after reset
    const ptr = try arena.alloc(50);
    try testing.expect(ptr.len == 50);
    
    std.log.info("Reset functionality test passed");
}

test "ArenaAllocator growth" {
    var arena = try ArenaAllocator.init(
        testing.allocator,
        .{ .initial_size = 256, .growth_factor = 2.0 },
        .Sequential
    );
    defer arena.deinit();
    
    // Allocate more than initial size to trigger growth
    const large_ptr = try arena.alloc(512); // Larger than initial 256 bytes
    try testing.expect(large_ptr.len == 512);
    
    const usage = arena.getUsage();
    try testing.expect(usage.total_allocated > 256); // Should have grown
    
    std.log.info("Growth test passed: grew to {} bytes", .{usage.total_allocated});
}

test "CursedArenaManager comprehensive test" {
    var manager = try CursedArenaManager.init(testing.allocator);
    defer manager.deinit();
    
    // Test different allocator types
    const parser_allocator = manager.getParserAllocator();
    const ast_allocator = manager.getASTAllocator();
    const runtime_allocator = manager.getRuntimeAllocator();
    const string_allocator = manager.getStringAllocator();
    const temp_allocator = manager.getTemporaryAllocator();
    
    // Allocate from different arenas
    const parser_mem = try parser_allocator.alloc(u8, 100);
    const ast_mem = try ast_allocator.alloc(u8, 200);
    const runtime_mem = try runtime_allocator.alloc(u8, 300);
    const string_mem = try string_allocator.alloc(u8, 50);
    const temp_mem = try temp_allocator.alloc(u8, 150);
    
    try testing.expect(parser_mem.len == 100);
    try testing.expect(ast_mem.len == 200);
    try testing.expect(runtime_mem.len == 300);
    try testing.expect(string_mem.len == 50);
    try testing.expect(temp_mem.len == 150);
    
    // Test usage statistics
    const total_usage = manager.getTotalUsage();
    try testing.expect(total_usage.total_used >= 800); // At least sum of allocations
    
    // Test reset functionality
    manager.resetTemporary();
    const usage_after_temp_reset = manager.getTotalUsage();
    try testing.expect(usage_after_temp_reset.temporary.total_used == 0);
    
    manager.resetAll();
    const usage_after_full_reset = manager.getTotalUsage();
    try testing.expect(usage_after_full_reset.total_used == 0);
    
    std.log.info("CursedArenaManager test passed: peak usage {} bytes", .{total_usage.total_used});
}

test "ArenaAllocator memory pattern fill" {
    var arena = try ArenaAllocator.init(
        testing.allocator,
        .{ .fill_pattern = 0xAA },
        .Sequential
    );
    defer arena.deinit();
    
    const ptr = try arena.alloc(100);
    
    // Check that memory is filled with pattern
    for (ptr) |byte| {
        try testing.expect(byte == 0xAA);
    }
    
    std.log.info("Memory pattern fill test passed");
}

test "ArenaAllocator bounds checking" {
    var arena = try ArenaAllocator.init(
        testing.allocator,
        .{ .max_alloc_size = 1000, .bounds_checking = true },
        .Sequential
    );
    defer arena.deinit();
    
    // This should succeed
    const ptr1 = try arena.alloc(500);
    try testing.expect(ptr1.len == 500);
    
    // This should fail due to size limit
    const result = arena.alloc(2000);
    try testing.expectError(error.AllocationTooLarge, result);
    
    std.log.info("Bounds checking test passed");
}

test "ArenaAllocator stress test" {
    var arena = try ArenaAllocator.init(
        testing.allocator,
        .{ .initial_size = 4096, .growth_factor = 1.5 },
        .Sequential
    );
    defer arena.deinit();
    
    // Allocate many small objects
    var ptrs: [1000]*[]u8 = undefined;
    for (&ptrs, 0..) |*ptr, i| {
        const size = (i % 100) + 1; // Varying sizes 1-100
        const allocation = try arena.alloc(size);
        ptr.* = &allocation;
    }
    
    const usage = arena.getUsage();
    try testing.expect(usage.allocation_count == 1000);
    
    // Test fragmentation calculation
    try testing.expect(usage.fragmentation >= 0.0 and usage.fragmentation <= 1.0);
    
    std.log.info("Stress test passed: {} allocations, {:.2}% fragmentation", 
                .{usage.allocation_count, usage.fragmentation * 100});
}

test "ArenaAllocator concurrent access simulation" {
    var arena = try ArenaAllocator.init(
        testing.allocator,
        .{ .initial_size = 8192 },
        .Sequential
    );
    defer arena.deinit();
    
    // Simulate concurrent access by alternating allocations
    const num_threads = 4;
    const allocs_per_thread = 100;
    
    var i: usize = 0;
    while (i < num_threads * allocs_per_thread) {
        const thread_id = i % num_threads;
        const size = 50 + (thread_id * 10); // Different sizes per "thread"
        
        const ptr = try arena.alloc(size);
        try testing.expect(ptr.len == size);
        
        i += 1;
    }
    
    const usage = arena.getUsage();
    try testing.expect(usage.allocation_count == num_threads * allocs_per_thread);
    
    std.log.info("Concurrent simulation test passed: {} allocations", .{usage.allocation_count});
}
