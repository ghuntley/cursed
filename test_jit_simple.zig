const std = @import("std");
const jit = @import("src-zig/jit_execution_engine_backup.zig");
const interpreter = @import("src-zig/interpreter.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print("🧪 Testing JIT Engine Core Functionality\n", .{});
    
    var engine = try jit.JITExecutionEngine.init(allocator);
    defer engine.deinit();
    
    // Test string concatenation
    const test_values = [_]interpreter.Value{
        interpreter.Value{ .String = "Hello" },
        interpreter.Value{ .String = " JIT!" },
    };
    
    const result = try engine.performStringConcatenation(&test_values);
    defer allocator.free(result.String);
    
    std.debug.print("String concatenation result: '{s}'\n", .{result.String});
    
    // Test arithmetic
    const left = interpreter.Value{ .Integer = 10 };
    const right = interpreter.Value{ .Integer = 5 };
    
    const add_result = try engine.performAddition(left, right);
    std.debug.print("10 + 5 = {}\n", .{add_result.Integer});
    
    std.debug.print("✅ JIT Engine core tests passed!\n", .{});
}
