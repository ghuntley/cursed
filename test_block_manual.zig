const std = @import("std");
const interpreter = @import("src-zig/interpreter.zig");
const ast = @import("src-zig/ast.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Create interpreter
    var interp = interpreter.Interpreter.init(allocator);
    defer interp.deinit();

    std.debug.print("Interpreter initialized successfully\n", .{});

    // Create a simple block statement manually - use the same pattern as the codebase
    const ArrayList = std.ArrayList;
    var block_statements = ArrayList(*anyopaque).init(allocator);
    defer block_statements.deinit();
    
    // Create the block with our statements after we've added to the list
    // We need to do this after adding statements since the block holds the reference

    // Create a simple expression statement to put in the block
    const expr = ast.Expression{ .Integer = 42 };
    const expr_stmt = try allocator.create(ast.Statement);
    expr_stmt.* = ast.Statement{ .Expression = expr };
    try block_statements.append(@ptrCast(expr_stmt));
    
    const block = ast.BlockStatement{ .statements = block_statements };
    const block_stmt = ast.Statement{ .Block = block };

    std.debug.print("Created block statement with {} statements\n", .{block_statements.items.len});

    // Try to execute the block statement
    interp.executeStatement(block_stmt) catch |err| {
        std.debug.print("Error executing block statement: {}\n", .{err});
        return;
    };

    std.debug.print("Block statement executed successfully!\n", .{});

    // Cleanup
    allocator.destroy(expr_stmt);
}
