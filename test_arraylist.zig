const std = @import("std");
const ArrayList = std.ArrayList;
pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Test different ArrayList init patterns
    var list1 = ArrayList(u32){};
    var list2 = ArrayList(u32).init(allocator);
    _ = list1; _ = list2;
}
