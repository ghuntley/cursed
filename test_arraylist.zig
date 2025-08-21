const std = @import("std"); pub fn main() !void { var gpa = std.heap.GeneralPurposeAllocator(.{}){}; const allocator = gpa.allocator(); var arr = std.ArrayList(i32){}; defer arr.deinit(allocator); }
