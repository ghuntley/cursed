const std = @import("std"); pub fn main() !void { var list = std.ArrayList(u8).init(std.heap.page_allocator); defer list.deinit(); try list.append(42); }
