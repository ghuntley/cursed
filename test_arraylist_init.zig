const std = @import("std");
pub fn main() !void { 
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Try different ways to initialize ArrayList
    var list1 = std.ArrayList(u32){};  // Undefined initialization
    var list2 = std.ArrayList(u32){ .items = &.{}, .capacity = 0, .allocator = allocator };
    _ = list1; _ = list2;
}
