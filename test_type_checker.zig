const std = @import("std");
const type_system = @import("src-zig/type_system.zig");
const ast = @import("src-zig/ast.zig");

test "type_checker_integration" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Test TypeChecker initialization
    var type_checker = try type_system.TypeChecker.init(allocator);
    defer type_checker.deinit();

    // Test TypeExpression methods
    const int_type = type_system.TypeExpression.named(allocator, "drip");
    defer int_type.deinit();
    
    const bool_type = type_system.TypeExpression.named(allocator, "lit");
    defer bool_type.deinit();
    
    const string_type = type_system.TypeExpression.named(allocator, "tea");
    defer string_type.deinit();

    // Test type checking methods
    try std.testing.expect(int_type.isInteger());
    try std.testing.expect(bool_type.isBoolean());
    try std.testing.expect(string_type.isString());
    try std.testing.expect(string_type.isIterable());

    // Test type compatibility
    const small_int_type = type_system.TypeExpression.named(allocator, "smol");
    defer small_int_type.deinit();
    
    try std.testing.expect(small_int_type.canCoerceTo(&int_type));

    std.debug.print("✅ Type checker integration test passed!\n", .{});
}
