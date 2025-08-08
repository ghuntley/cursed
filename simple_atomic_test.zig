const std = @import("std");
const testing = std.testing;
const Thread = std.Thread;
const Allocator = std.mem.Allocator;
const type_system_runtime = @import("src-zig/type_system_runtime.zig");

test "atomic reference counting works correctly" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Create test object directly
    const object = try type_system_runtime.TypedAllocator.TypedObject.init(allocator, 0, 64);
    defer object.deinit(allocator);

    // Test atomic operations
    try testing.expect(object.getRefCount() == 1);
    
    object.retain();
    try testing.expect(object.getRefCount() == 2);
    
    object.release(allocator);
    try testing.expect(object.getRefCount() == 1);
}

test "concurrent atomic operations are safe" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Create test object
    const object = try type_system_runtime.TypedAllocator.TypedObject.init(allocator, 0, 64);
    defer object.deinit(allocator);

    const ThreadData = struct {
        obj: *type_system_runtime.TypedAllocator.TypedObject,
        alloc: Allocator,
    };

    const worker = struct {
        fn run(data: ThreadData) void {
            for (0..100) |_| {
                data.obj.retain();
                data.obj.release(data.alloc);
            }
        }
    }.run;

    var threads: [4]Thread = undefined;
    const thread_data = ThreadData{ .obj = object, .alloc = allocator };

    // Launch threads
    for (0..4) |i| {
        threads[i] = try Thread.spawn(.{}, worker, .{thread_data});
    }

    // Wait for completion
    for (0..4) |i| {
        threads[i].join();
    }

    // Should still have reference count of 1
    try testing.expect(object.getRefCount() == 1);
}

test "atomic load and store operations" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const object = try type_system_runtime.TypedAllocator.TypedObject.init(allocator, 0, 32);
    defer object.deinit(allocator);

    // Test atomic store/load
    object.ref_count.store(5, .release);
    try testing.expect(object.ref_count.load(.acquire) == 5);
    
    // Test fetchAdd
    const old_val = object.ref_count.fetchAdd(3, .acq_rel);
    try testing.expect(old_val == 5);
    try testing.expect(object.ref_count.load(.acquire) == 8);
    
    // Test fetchSub  
    const old_val2 = object.ref_count.fetchSub(7, .acq_rel);
    try testing.expect(old_val2 == 8);
    try testing.expect(object.ref_count.load(.acquire) == 1);
}
