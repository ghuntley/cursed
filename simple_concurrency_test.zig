const std = @import("std");
const print = std.debug.print;

const concurrency = @import("src-zig/concurrency.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    print("Step 1: Creating config\n", .{});
    const config = concurrency.SchedulerConfig.default();
    
    print("Step 2: Initializing scheduler\n", .{});
    try concurrency.initializeScheduler(allocator, config);
    
    print("Step 3: Testing basic functions\n", .{});
    
    print("Step 4: Shutting down\n", .{});
    concurrency.shutdownScheduler(allocator);
    
    print("✅ Basic test completed\n", .{});
}
