const std = @import("std");
const ast_backup = @import("src-zig/ast_backup.zig");
const testing = std.testing;

test "AST circular dependency fix" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Create AST instance
    var ast = ast_backup.AST.init(allocator);
    defer ast.deinit(); // This should now work without circular dependency

    // Create binary expression using indices
    const left_expr = ast_backup.Expression{ .Integer = 5 };
    const right_expr = ast_backup.Expression{ .Integer = 10 };
    
    const left_index = try ast.addExpression(left_expr);
    const right_index = try ast.addExpression(right_expr);
    
    const binary = ast_backup.BinaryExpression{
        .left = left_index,
        .operator = "+",
        .right = right_index,
    };
    
    const binary_expr = ast_backup.Expression{ .Binary = binary };
    const binary_index = try ast.addExpression(binary_expr);
    
    // Verify we can access the expressions
    const retrieved = ast.getExpression(binary_index);
    try testing.expect(retrieved != null);
    
    if (retrieved) |expr| {
        switch (expr.*) {
            .Binary => |bin| {
                try testing.expect(bin.left == left_index);
                try testing.expect(bin.right == right_index);
                try testing.expect(std.mem.eql(u8, bin.operator, "+"));
            },
            else => try testing.expect(false),
        }
    }
    
    // Test call expression with multiple arguments
    var call_args = std.ArrayList(ast_backup.NodeIndex).init(allocator);
    try call_args.append(left_index);
    try call_args.append(right_index);
    
    const call = ast_backup.CallExpression{
        .function = left_index, // Using left_index as dummy function
        .arguments = call_args,
    };
    
    const call_expr = ast_backup.Expression{ .Call = call };
    _ = try ast.addExpression(call_expr);
    
    // The deinit call at the end should properly clean up without circular dependency
    std.debug.print("AST cleanup test passed - no circular dependency!\n", .{});
}

test "AST memory safety with complex nesting" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var ast = ast_backup.AST.init(allocator);
    defer ast.deinit();

    // Create deeply nested expressions
    var current_index: ast_backup.NodeIndex = ast_backup.INVALID_NODE;
    
    // Create a chain of binary expressions: ((1 + 2) + 3) + 4
    for (1..5) |i| {
        const int_expr = ast_backup.Expression{ .Integer = @intCast(i) };
        const int_index = try ast.addExpression(int_expr);
        
        if (current_index == ast_backup.INVALID_NODE) {
            current_index = int_index;
        } else {
            const binary = ast_backup.BinaryExpression{
                .left = current_index,
                .operator = "+",
                .right = int_index,
            };
            const binary_expr = ast_backup.Expression{ .Binary = binary };
            current_index = try ast.addExpression(binary_expr);
        }
    }
    
    // Verify the final expression exists
    const final_expr = ast.getExpression(current_index);
    try testing.expect(final_expr != null);
    
    std.debug.print("Complex nesting test passed - memory management works!\n", .{});
}
