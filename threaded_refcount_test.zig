const std = @import("std");
const testing = std.testing;
const Thread = std.Thread;
const Allocator = std.mem.Allocator;
const type_system_runtime = @import("src-zig/type_system_runtime.zig");

const NUM_THREADS = 8;
const OPERATIONS_PER_THREAD = 1000;

const ThreadTestData = struct {
    object: *type_system_runtime.TypedAllocator.TypedObject,
    allocator: Allocator,
    iterations: u32,
    thread_id: u32,
};

fn retainReleaseWorker(data: *ThreadTestData) void {
    var i: u32 = 0;
    while (i < data.iterations) : (i += 1) {
        // Retain the object
        data.object.retain();
        
        // Do some work (simulate processing)
        std.time.sleep(1000); // 1 microsecond
        
        // Release the object
        data.object.release(data.allocator);
    }
}

fn concurrentAccessWorker(data: *ThreadTestData) void {
    var i: u32 = 0;
    while (i < data.iterations) : (i += 1) {
        // Check reference count atomically
        const ref_count = data.object.getRefCount();
        
        // Retain if count is positive
        if (ref_count > 0) {
            data.object.retain();
            
            // Simulate some work
            std.time.sleep(500);
            
            // Release
            data.object.release(data.allocator);
        }
    }
}

test "atomic reference counting under concurrent load" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Create a dummy GC registry
    var gc_registry = type_system_runtime.GCTypeRegistry.init(allocator);
    defer gc_registry.deinit();
    
    // Register a test type - we'll use the GC registry's registerType method
    // which handles creating the RuntimeTypeInfo properly

    // Create typed allocator
    var typed_allocator = type_system_runtime.TypedAllocator.init(allocator, &gc_registry);
    defer typed_allocator.deinit();

    // Create test object
    const object = try type_system_runtime.TypedAllocator.TypedObject.init(allocator, 0, 64);
    defer object.deinit(allocator);

    // Create threads for retain/release testing
    var threads: [NUM_THREADS]Thread = undefined;
    var thread_data: [NUM_THREADS]ThreadTestData = undefined;

    // Initialize thread data
    for (0..NUM_THREADS) |i| {
        thread_data[i] = ThreadTestData{
            .object = object,
            .allocator = allocator,
            .iterations = OPERATIONS_PER_THREAD,
            .thread_id = @intCast(i),
        };
    }

    // Launch threads
    for (0..NUM_THREADS) |i| {
        threads[i] = try Thread.spawn(.{}, retainReleaseWorker, .{&thread_data[i]});
    }

    // Wait for all threads to complete
    for (0..NUM_THREADS) |i| {
        threads[i].join();
    }

    // Verify final reference count (should be 1 - initial reference)
    const final_ref_count = object.getRefCount();
    try testing.expect(final_ref_count == 1);
}

test "no double-free under concurrent access" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Create a dummy GC registry
    var gc_registry = type_system_runtime.GCTypeRegistry.init(allocator);
    defer gc_registry.deinit();

    // Create typed allocator
    var typed_allocator = type_system_runtime.TypedAllocator.init(allocator, &gc_registry);
    defer typed_allocator.deinit();

    // Run multiple iterations to catch race conditions
    for (0..100) |_| {
        const object = try type_system_runtime.TypedAllocator.TypedObject.init(allocator, 0, 32);
        
        // Create threads for concurrent access
        var threads: [4]Thread = undefined;
        var thread_data: [4]ThreadTestData = undefined;

        // Initialize thread data
        for (0..4) |i| {
            thread_data[i] = ThreadTestData{
                .object = object,
                .allocator = allocator,
                .iterations = 50,
                .thread_id = @intCast(i),
            };
        }

        // Launch threads
        for (0..4) |i| {
            threads[i] = try Thread.spawn(.{}, concurrentAccessWorker, .{&thread_data[i]});
        }

        // Wait for completion
        for (0..4) |i| {
            threads[i].join();
        }

        // Clean up - this should not cause double-free
        object.deinit(allocator);
    }
}

test "reference count consistency checks" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const object = try type_system_runtime.TypedAllocator.TypedObject.init(allocator, 0, 32);
    defer object.deinit(allocator);

    // Test normal retain/release cycle
    try testing.expect(object.getRefCount() == 1);
    
    object.retain();
    try testing.expect(object.getRefCount() == 2);
    
    object.release(allocator);
    try testing.expect(object.getRefCount() == 1);
}

test "atomic operations work correctly" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const object = try type_system_runtime.TypedAllocator.TypedObject.init(allocator, 0, 32);
    defer object.deinit(allocator);

    // Test atomic load/store
    object.ref_count.store(5, .release);
    try testing.expect(object.ref_count.load(.acquire) == 5);
    
    // Test atomic add/sub
    const old_val = object.ref_count.fetchAdd(3, .acq_rel);
    try testing.expect(old_val == 5);
    try testing.expect(object.ref_count.load(.acquire) == 8);
    
    const old_val2 = object.ref_count.fetchSub(7, .acq_rel);
    try testing.expect(old_val2 == 8);
    try testing.expect(object.ref_count.load(.acquire) == 1);
}
