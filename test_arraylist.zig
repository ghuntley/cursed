const std = @import("std");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var list: std.ArrayList([]const u8) = .{};
    list.allocator = allocator;
    defer list.deinit();
    
    try list.append("test");
    std.debug.print("Array list test passed\n", .{});
}
