const std = @import("std");
const gc = @import("src-zig/gc.zig");
const print = std.debug.print;

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    
    print("🧪 Testing production-ready GC implementation...\n");
    
    // Test 1: Basic allocation and collection
    print("\n📝 Test 1: Basic allocation and collection\n");
    
    var config = gc.GCConfig.default();
    config.initial_heap_size = 1024 * 1024; // 1MB for testing
    
    var garbage_collector = try gc.GC.init(allocator, config);
    defer garbage_collector.deinit();
    
    // Test basic allocation
    const ptr1 = try garbage_collector.alloc(64, 0);
    print("✅ Allocated 64 bytes at {*}\n", .{ptr1});
    
    const ptr2 = try garbage_collector.alloc(128, 1);
    print("✅ Allocated 128 bytes at {*}\n", .{ptr2});
    
    // Test root management
    var root1: ?*anyopaque = ptr1;
    try garbage_collector.addRoot(&root1, 0);
    print("✅ Added root reference for ptr1\n");
    
    // Force collection
    try garbage_collector.collectNow();
    print("✅ Garbage collection completed\n");
    
    // Print statistics
    print("\n📊 GC Statistics:\n");
    garbage_collector.printStats();
    
    // Test 2: Memory pools
    print("\n📝 Test 2: Memory pool allocation\n");
    
    var pool_manager = try gc.MemoryPoolManager.init(allocator);
    defer pool_manager.deinit();
    
    const pool_ptr1 = try pool_manager.getAllocation(16);
    if (pool_ptr1) |ptr| {
        print("✅ Pool allocated 16 bytes at {*}\n", .{ptr});
        try pool_manager.deallocate(ptr, 16);
        print("✅ Pool deallocated 16 bytes\n");
    }
    
    // Test 3: Write barriers
    print("\n📝 Test 3: Write barriers\n");
    
    const ptr3 = try garbage_collector.alloc(32, 0);
    const ptr4 = try garbage_collector.alloc(32, 0);
    
    garbage_collector.writeBarrier(ptr3, ptr4);
    print("✅ Write barrier recorded\n");
    
    // Test 4: Weak references
    print("\n📝 Test 4: Weak references\n");
    
    const ptr5 = try garbage_collector.alloc(64, 0);
    var weak_ref = gc.WeakRef{
        .target = ptr5,
        .header = gc.ObjectHeader.fromData(ptr5),
    };
    
    if (weak_ref.get()) |target| {
        print("✅ Weak reference valid, target at {*}\n", .{target});
    }
    
    // Test 5: Memory tracking
    print("\n📝 Test 5: Memory tracking\n");
    
    const leaks = try garbage_collector.detectMemoryLeaks();
    defer allocator.free(leaks);
    print("✅ Memory leak detection completed: {} potential leaks\n", .{leaks.len});
    
    // Test 6: Stress test
    print("\n📝 Test 6: Stress test\n");
    
    var allocated_ptrs = std.ArrayList(*anyopaque).init(allocator);
    defer allocated_ptrs.deinit();
    
    // Allocate many objects
    for (0..100) |i| {
        const size = 32 + (i % 50);
        const type_id = @as(u16, @intCast(i % 4));
        
        const ptr = try garbage_collector.alloc(size, type_id);
        try allocated_ptrs.append(ptr);
        
        // Add some as roots
        if (i % 10 == 0) {
            var root: ?*anyopaque = ptr;
            try garbage_collector.addRoot(&root, type_id);
        }
    }
    
    // Force collection
    try garbage_collector.collectNow();
    print("✅ Stress test completed\n");
    
    // Final statistics
    print("\n📊 Final GC Statistics:\n");
    garbage_collector.printStats();
    
    print("\n🎉 All GC tests completed successfully!\n");
}
