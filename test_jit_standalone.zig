const std = @import("std");
const jit_engine = @import("src-zig/jit_execution_engine_backup.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print("🚀 Testing JIT Execution Engine\n", .{});
    
    try jit_engine.testJITExecutionEngine(allocator);
    
    std.debug.print("✅ JIT Engine test completed successfully\n", .{});
}
