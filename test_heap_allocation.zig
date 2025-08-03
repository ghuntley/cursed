const std = @import("std");
const ArrayList = std.ArrayList;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Test ArrayList allocation and deallocation
    var list = ArrayList(u32).init(allocator);
    defer list.deinit(); // This should prevent leaks
    
    try list.append(1);
    try list.append(2);
    try list.append(3);
    
    std.debug.print("Allocated {} items\n", .{list.items.len});
    
    // Test missing deinit (commented out to simulate leak)
    var leaked_list = ArrayList(u32).init(allocator);
    // defer leaked_list.deinit(); // INTENTIONALLY COMMENTED OUT TO CAUSE LEAK
    
    try leaked_list.append(100);
    try leaked_list.append(200);
    
    std.debug.print("Leaked list has {} items\n", .{leaked_list.items.len});
}
