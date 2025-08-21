const std = @import("std"); pub fn main() void { var list = std.ArrayList(u8).init(std.heap.page_allocator); _ = list; }
