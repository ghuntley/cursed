const std = @import("std");
const jit_engine = @import("src-zig/jit_execution_engine_backup.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print("🚀 Starting CURSED JIT Implementation Test\n", .{});
    std.debug.print("============================================\n", .{});

    // Test the JIT execution engine
    try jit_engine.testJITExecutionEngine(allocator);

    std.debug.print("\n🎉 All JIT tests completed successfully!\n", .{});
}
