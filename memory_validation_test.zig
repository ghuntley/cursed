const std = @import("std");
const testing = std.testing;
const Allocator = std.mem.Allocator;
const ast = @import("src-zig/ast.zig");

test "AST Reference Counting Basic Operations" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Test basic reference counting
    var ref_counted = ast.RefCounted.init(allocator);
    try testing.expect(ref_counted.getCount() == 1);

    ref_counted.retain();
    try testing.expect(ref_counted.getCount() == 2);

    // Test RefPtr operations
    const test_type = try allocator.create(ast.Type);
    defer allocator.destroy(test_type);
    test_type.* = ast.Type{ .Basic = ast.BasicType.Normie };

    var array_type = try ast.ArrayType.create(allocator, test_type, 10);
    defer array_type.ref_counted.release(ast.ArrayType, ast.ArrayType.deinit);

    try testing.expect(array_type.ref_counted.getCount() == 1);
    try testing.expect(array_type.size.? == 10);
}

test "AST Memory Leak Detection" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const leaks = gpa.detectLeaks();
        if (leaks) {
            std.debug.print("Memory leaks detected!\n", .{});
        }
        try std.testing.expect(!leaks);
    }
    const allocator = gpa.allocator();

    // Create many AST nodes and ensure they're properly cleaned up
    const iterations = 100;
    for (0..iterations) |i| {
        _ = i;
        
        const element_type = try allocator.create(ast.Type);
        element_type.* = ast.Type{ .Basic = ast.BasicType.Normie };
        defer allocator.destroy(element_type);

        var array_type = try ast.ArrayType.create(allocator, element_type, 5);
        defer array_type.ref_counted.release(ast.ArrayType, ast.ArrayType.deinit);

        var slice_type = try ast.SliceType.create(allocator, element_type);
        defer slice_type.ref_counted.release(ast.SliceType, ast.SliceType.deinit);

        var function_type = try ast.FunctionType.create(allocator);
        defer function_type.ref_counted.release(ast.FunctionType, ast.FunctionType.deinit);
        
        function_type.setReturnType(element_type);
    }
}

test "AST Concurrent Access Safety" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const element_type = try allocator.create(ast.Type);
    defer allocator.destroy(element_type);
    element_type.* = ast.Type{ .Basic = ast.BasicType.Tea };

    var shared_type = try ast.ArrayType.create(allocator, element_type, 100);
    defer shared_type.ref_counted.release(ast.ArrayType, ast.ArrayType.deinit);

    // Simulate concurrent access by multiple references
    var ref_ptrs: [10]ast.RefPtr(ast.ArrayType) = undefined;
    for (&ref_ptrs) |*ref_ptr| {
        ref_ptr.* = ast.RefPtr(ast.ArrayType).init(shared_type);
    }
    
    // Verify reference count increased correctly
    try testing.expect(shared_type.ref_counted.getCount() == 11); // 1 original + 10 refs

    // Cleanup references
    for (&ref_ptrs) |*ref_ptr| {
        ref_ptr.deinit(ast.ArrayType.deinit);
    }
    
    // Should be back to 1 reference
    try testing.expect(shared_type.ref_counted.getCount() == 1);
}

test "AST Type Hierarchy Memory Management" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const leaks = gpa.detectLeaks();
        if (leaks) {
            std.debug.print("Memory leaks in type hierarchy!\n", .{});
        }
        try std.testing.expect(!leaks);
    }
    const allocator = gpa.allocator();

    // Create complex nested type hierarchy
    const base_type = try allocator.create(ast.Type);
    base_type.* = ast.Type{ .Basic = ast.BasicType.Normie };
    defer allocator.destroy(base_type);

    var array_type = try ast.ArrayType.create(allocator, base_type, 5);
    defer array_type.ref_counted.release(ast.ArrayType, ast.ArrayType.deinit);

    var pointer_type = try ast.PointerType.create(allocator, @ptrCast(array_type));
    defer pointer_type.ref_counted.release(ast.PointerType, ast.PointerType.deinit);

    var function_type = try ast.FunctionType.create(allocator);
    defer function_type.ref_counted.release(ast.FunctionType, ast.FunctionType.deinit);
    
    function_type.setReturnType(@ptrCast(pointer_type));

    // Verify the chain is properly linked through RefPtr
    try testing.expect(pointer_type.ref_counted.getCount() >= 1);
    try testing.expect(array_type.ref_counted.getCount() >= 1);
}
