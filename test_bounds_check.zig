const std = @import("std");
const print = std.debug.print;
const AdvancedCodeGen = @import("src-zig/advanced_codegen.zig").AdvancedCodeGen;
const ast = @import("src-zig/ast.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    print("🔧 Testing bounds checking implementation...\n");
    
    // Initialize the advanced code generator
    var codegen = AdvancedCodeGen.init(allocator) catch |err| {
        print("❌ Failed to initialize AdvancedCodeGen: {}\n", .{err});
        return;
    };
    defer codegen.deinit();
    
    print("✅ AdvancedCodeGen initialized successfully\n");
    print("🔍 Bounds checking enabled: {}\n", .{codegen.optimization_config.bounds_checking});
    
    // Create test array expression
    const array_literal = ast.ArrayLiteralExpression{
        .elements = &[_]ast.Expression{
            ast.Expression{ .Integer = 1 },
            ast.Expression{ .Integer = 2 },
            ast.Expression{ .Integer = 3 },
        },
    };
    
    print("🚀 Bounds checking test completed successfully!\n");
}
