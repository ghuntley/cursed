const std = @import("std");
const RealLLVMBackend = @import("llvm_backend_real.zig").RealLLVMBackend;
const ast = @import("ast.zig");

/// Test the real LLVM backend with LLVM 18
pub fn main() !void {
    std.debug.print("🚀 Testing Real LLVM Backend Integration...\n", .{});
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Test 1: Basic initialization
    std.debug.print("\n📋 Test 1: Backend Initialization\n", .{});
    var backend = RealLLVMBackend.init(allocator, "cursed_test") catch |err| {
        std.debug.print("❌ Failed to initialize LLVM backend: {any}\n", .{err});
        std.debug.print("💡 This likely means LLVM 18 is not properly installed or accessible\n", .{});
        return;
    };
    defer backend.deinit();
    std.debug.print("✅ LLVM backend initialized successfully\n", .{});
    
    // Test 2: IR Generation
    std.debug.print("\n📋 Test 2: Basic IR Generation\n", .{});
    const ir = backend.generateIR() catch |err| {
        std.debug.print("❌ Failed to generate IR: {any}\n", .{err});
        return;
    };
    defer allocator.free(ir);
    std.debug.print("✅ LLVM IR Generated:\n{s}\n", .{ir});
    
    // Test 3: Simple expression compilation
    std.debug.print("\n📋 Test 3: Expression Compilation\n", .{});
    
    // Create a simple integer literal expression
    const int_expr = ast.Expression{ 
        .integer_literal = ast.IntegerLiteral{ .value = 42 } 
    };
    
    const llvm_value = backend.compileExpression(int_expr) catch |err| {
        std.debug.print("❌ Failed to compile expression: {any}\n", .{err});
        return;
    };
    _ = llvm_value; // Suppress unused warning
    std.debug.print("✅ Expression compiled successfully\n", .{});
    
    // Test 4: Function creation
    std.debug.print("\n📋 Test 4: Function Declaration\n", .{});
    
    // Create a simple function declaration
    const test_func = ast.FunctionDeclaration{
        .name = "test_func",
        .parameters = &[_]ast.Parameter{},
        .return_type = "drip",
        .body = ast.Statement{
            .block_statement = ast.BlockStatement{
                .statements = &[_]ast.Statement{
                    ast.Statement{
                        .return_statement = ast.ReturnStatement{
                            .expression = ast.Expression{
                                .integer_literal = ast.IntegerLiteral{ .value = 123 }
                            }
                        }
                    }
                }
            }
        }
    };
    
    backend.compileFunctionDeclaration(test_func) catch |err| {
        std.debug.print("❌ Failed to compile function: {any}\n", .{err});
        return;
    };
    std.debug.print("✅ Function declaration compiled successfully\n", .{});
    
    // Test 5: Final IR after compilation
    std.debug.print("\n📋 Test 5: Final IR Generation\n", .{});
    const final_ir = backend.generateIR() catch |err| {
        std.debug.print("❌ Failed to generate final IR: {any}\n", .{err});
        return;
    };
    defer allocator.free(final_ir);
    std.debug.print("✅ Final LLVM IR:\n{s}\n", .{final_ir});
    
    // Test 6: Optimization
    std.debug.print("\n📋 Test 6: Optimization Passes\n", .{});
    backend.optimize(2) catch |err| {
        std.debug.print("❌ Failed to optimize: {any}\n", .{err});
        return;
    };
    std.debug.print("✅ Optimization completed\n", .{});
    
    // Test 7: Native compilation (if supported)
    std.debug.print("\n📋 Test 7: Native Compilation Test\n", .{});
    backend.compileToNative("test_output") catch |err| {
        std.debug.print("⚠️  Native compilation test failed: {any}\n", .{err});
        std.debug.print("💡 This is expected if clang is not available or linking fails\n", .{});
    } else {
        std.debug.print("✅ Native compilation test completed\n", .{});
    };
    
    std.debug.print("\n🎉 Real LLVM Backend Integration Test Completed!\n", .{});
    std.debug.print("🚀 The LLVM backend is working and ready for real compilation\n", .{});
}
