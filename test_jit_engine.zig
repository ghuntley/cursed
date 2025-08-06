const std = @import("std");
const jit = @import("src-zig/jit_execution_engine_backup.zig");
const ast = @import("src-zig/ast.zig");
const interpreter = @import("src-zig/interpreter.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print("🧪 Testing JIT Execution Engine\n", .{});
    
    // Test the JIT execution engine
    try jit.testJITExecutionEngine(allocator);
    
    // Test individual components
    try testStringConcatenation(allocator);
    try testTypeConversions(allocator);
    try testComplexExpressions(allocator);
    
    std.debug.print("✅ All JIT Engine tests passed!\n", .{});
}

fn testStringConcatenation(allocator: std.mem.Allocator) !void {
    std.debug.print("\n🔗 Testing String Concatenation...\n", .{});
    
    var engine = try jit.JITExecutionEngine.init(allocator);
    defer engine.deinit();
    
    const test_values = [_]interpreter.Value{
        interpreter.Value{ .String = "Hello" },
        interpreter.Value{ .String = " " },
        interpreter.Value{ .String = "JIT" },
        interpreter.Value{ .String = "!" },
    };
    
    const result = try engine.performStringConcatenation(&test_values);
    defer allocator.free(result.String);
    
    std.debug.print("Concatenated: '{s}'\n", .{result.String});
    
    if (!std.mem.eql(u8, result.String, "Hello JIT!")) {
        return error.StringConcatenationFailed;
    }
    
    std.debug.print("✅ String concatenation test passed\n", .{});
}

fn testTypeConversions(allocator: std.mem.Allocator) !void {
    std.debug.print("\n🔄 Testing Type Conversions...\n", .{});
    
    var engine = try jit.JITExecutionEngine.init(allocator);
    defer engine.deinit();
    
    // Test struct creation
    var test_struct = try interpreter.StructInstance.init(allocator, "TestStruct");
    defer test_struct.deinit();
    
    try test_struct.setField("x", interpreter.Value{ .Integer = 42 });
    try test_struct.setField("y", interpreter.Value{ .String = "test" });
    
    // Test struct to interface conversion
    const interface_result = try engine.convertStructToInterface(test_struct, "TestInterface");
    
    std.debug.print("Converted struct to interface: {s}\n", .{interface_result.Interface.vtable.interface_name});
    
    if (!std.mem.eql(u8, interface_result.Interface.vtable.interface_name, "TestInterface")) {
        return error.TypeConversionFailed;
    }
    
    std.debug.print("✅ Type conversion test passed\n", .{});
}

fn testComplexExpressions(allocator: std.mem.Allocator) !void {
    std.debug.print("\n🧮 Testing Complex Expressions...\n", .{});
    
    var engine = try jit.JITExecutionEngine.init(allocator);
    defer engine.deinit();
    
    // Test arithmetic operations
    const left = interpreter.Value{ .Integer = 10 };
    const right = interpreter.Value{ .Integer = 5 };
    
    const add_result = try engine.performAddition(left, right);
    const mult_result = try engine.performMultiplication(left, right);
    const div_result = try engine.performDivision(left, right);
    
    std.debug.print("10 + 5 = {}\n", .{add_result.Integer});
    std.debug.print("10 * 5 = {}\n", .{mult_result.Integer});
    std.debug.print("10 / 5 = {d}\n", .{div_result.Float});
    
    if (add_result.Integer != 15 or mult_result.Integer != 50 or div_result.Float != 2.0) {
        return error.ComplexExpressionFailed;
    }
    
    std.debug.print("✅ Complex expression test passed\n", .{});
}
