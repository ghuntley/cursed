const std = @import("std");
const print = std.debug.print;
const testing = std.testing;

const crash_handler = @import("src-zig/crash_handler.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    print("Testing CURSED crash handler...\n", .{});
    
    // Test basic telemetry
    var telemetry = crash_handler.CrashTelemetry.init(allocator, true, 5);
    defer telemetry.deinit();
    
    var context = try crash_handler.CrashContext.init(
        allocator,
        .Warning,
        "Test warning",
        "test.zig",
        42,
        10,
        "testFunction"
    );
    defer context.deinit(allocator);
    
    try telemetry.recordCrash(context);
    
    print("✅ Crash telemetry working!\n", .{});
    print("📊 Recorded {} crashes\n", .{telemetry.crash_log.items.len});
}
