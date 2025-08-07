const std = @import("std");
const print = std.debug.print;
const concurrency_handlers = @import("src-zig/main_concurrency_handlers.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Test the concurrency handlers directly
    print("🚀 Testing CURSED concurrency implementation...\n", .{});
    
    // Initialize concurrency
    concurrency_handlers.initGlobalConcurrency(allocator);
    defer concurrency_handlers.deinitGlobalConcurrency();
    
    // Simulate a stan statement
    var source_lines = std.ArrayList([]const u8).init(allocator);
    defer source_lines.deinit();
    
    try source_lines.append("stan {");
    try source_lines.append("    vibez.spill(\"Hello from goroutine!\")");
    try source_lines.append("}");
    
    // Call the handler
    var dummy_vars: u8 = undefined;
    var dummy_funcs: u8 = undefined;
    try concurrency_handlers.handleStanStatement(@ptrCast(&dummy_vars), @ptrCast(&dummy_funcs), allocator, source_lines, 0, true);
    
    // Test wait_all
    try concurrency_handlers.handleWaitFunction(@ptrCast(&dummy_vars), allocator, "wait_all()", true);
    
    print("✅ Concurrency test completed!\n", .{});
}
