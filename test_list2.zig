const std = @import("std"); pub fn main() !void { var gpa = std.heap.GeneralPurposeAllocator(.{}){}; const a = gpa.allocator(); var list = std.ArrayList(u32).init(a); _ = list; }
