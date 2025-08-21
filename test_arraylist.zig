const std = @import("std");
const ArrayList = std.ArrayList;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var list: ArrayList(u32) = .empty;
    try list.append(allocator, 42);
    std.debug.print("List item: {}\n", .{list.items[0]});
    list.deinit(allocator);
}
