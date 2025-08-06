const std = @import("std");
const jit_engine = @import("src-zig/jit_execution_engine_backup.zig");
const interpreter = @import("src-zig/interpreter.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print("🧪 Testing JIT Engine with CURSED Programs\n", .{});
    
    var engine = try jit_engine.JITExecutionEngine.init(allocator);
    defer engine.deinit();
    
    // Register vibez.spill function
    var spill_params = try allocator.alloc([]const u8, 1);
    spill_params[0] = "value";
    try engine.registerFunctionWithSignature("spill", "vibez", spill_params, "null");
    
    // Test basic function calls
    const test_args = [_]interpreter.Value{
        interpreter.Value{ .String = "Hello JIT!" },
        interpreter.Value{ .Integer = 42 },
        interpreter.Value{ .Float = 3.14 },
        interpreter.Value{ .Boolean = true },
    };
    
    std.debug.print("Testing vibez.spill with different types:\n", .{});
    _ = try engine.executeFunction("vibez.spill", &test_args);
    
    // Test struct operations
    var test_struct = try interpreter.StructInstance.init(allocator, "TestStruct");
    try test_struct.setField("name", interpreter.Value{ .String = "JIT Test" });
    try test_struct.setField("value", interpreter.Value{ .Integer = 100 });
    
    const struct_result = try engine.convertStructType(test_struct, "ConvertedStruct");
    std.debug.print("Struct conversion result: {}\n", .{struct_result});
    
    const interface_result = try engine.convertStructToInterface(test_struct, "TestInterface");
    std.debug.print("Interface conversion result: {}\n", .{interface_result});
    
    std.debug.print("✅ JIT Engine basic tests completed\n", .{});
}
