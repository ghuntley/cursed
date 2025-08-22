const std = @import("std");
const testing = std.testing;
const Allocator = std.mem.Allocator;
const ast = @import("src-zig/ast.zig");

test "Basic Reference Counting" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const leaks = gpa.detectLeaks();
        if (leaks) {
            std.debug.print("Memory leaks detected!\n", .{});
        }
    }
    const allocator = gpa.allocator();

    // Test basic reference counting
    var ref_counted = ast.RefCounted.init(allocator);
    try testing.expect(ref_counted.getCount() == 1);

    ref_counted.retain();
    try testing.expect(ref_counted.getCount() == 2);
}

test "Type Reference Management" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Test RefPtr operations
    const test_type = try allocator.create(ast.Type);
    defer allocator.destroy(test_type);
    test_type.* = ast.Type{ .Basic = ast.BasicType.Normie };

    var array_type = try ast.ArrayType.create(allocator, test_type, 10);
    defer array_type.ref_counted.release(ast.ArrayType, ast.ArrayType.deinit);

    try testing.expect(array_type.ref_counted.getCount() == 1);
    try testing.expect(array_type.size.? == 10);
}

test "Memory Safety with Multiple References" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const element_type = try allocator.create(ast.Type);
    defer allocator.destroy(element_type);
    element_type.* = ast.Type{ .Basic = ast.BasicType.Tea };

    var shared_type = try ast.ArrayType.create(allocator, element_type, 100);
    defer shared_type.ref_counted.release(ast.ArrayType, ast.ArrayType.deinit);

    // Create multiple references to the same type
    var ref_ptrs: [5]ast.RefPtr(ast.ArrayType) = undefined;
    for (&ref_ptrs) |*ref_ptr| {
        ref_ptr.* = ast.RefPtr(ast.ArrayType).init(shared_type);
    }
    
    // Verify reference count increased correctly
    try testing.expect(shared_type.ref_counted.getCount() == 6); // 1 original + 5 refs

    // Cleanup references
    for (&ref_ptrs) |*ref_ptr| {
        ref_ptr.deinit(ast.ArrayType.deinit);
    }
    
    // Should be back to 1 reference
    try testing.expect(shared_type.ref_counted.getCount() == 1);
}
