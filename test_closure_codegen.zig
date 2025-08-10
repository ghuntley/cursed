const std = @import("std");
const print = std.debug.print;
const ast = @import("src-zig/ast.zig");
const advanced_codegen = @import("src-zig/advanced_codegen.zig");
const c = @cImport({
    @cInclude("llvm-c/Core.h");
});

test "closure IR generation" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Create a simple lambda expression AST
    var lambda_params = std.ArrayList([]const u8).init(allocator);
    defer lambda_params.deinit();
    try lambda_params.append("x");
    
    // Create a simple binary operation: x + captured_var  
    const binary_op = ast.BinaryOpExpression{
        .left = &ast.Expression{ .Variable = ast.VariableExpression{ .name = "x" } },
        .right = &ast.Expression{ .Variable = ast.VariableExpression{ .name = "captured_var" } },
        .operator = ast.BinaryOperator.Add,
    };
    
    const lambda_expr = ast.LambdaExpression{
        .parameters = lambda_params,
        .body = &ast.Expression{ .BinaryOp = binary_op },
    };
    
    // Initialize advanced codegen
    var codegen = try advanced_codegen.AdvancedCodeGen.init(allocator);
    defer codegen.deinit();
    
    // Add a captured variable
    const captured_value = c.LLVMConstInt(
        c.LLVMInt32TypeInContext(codegen.base_codegen.context), 
        42, 
        0
    );
    try codegen.base_codegen.variables.put("captured_var", captured_value);
    
    // Test closure generation
    const result = try codegen.generateAdvancedLambda(lambda_expr);
    
    print("✅ Closure generation completed successfully\n");
    print("📊 Generated closure value: {any}\n", .{result});
    
    // Verify lambda counter was incremented
    try std.testing.expect(codegen.lambda_counter == 1);
}
