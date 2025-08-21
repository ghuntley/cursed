const std = @import("std"); pub fn main() void { var list = std.ArrayList(u8){.allocator = std.heap.page_allocator, .items = &[_]u8{}, .capacity = 0}; _ = list; }
