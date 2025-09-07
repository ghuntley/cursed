const std = @import("std"); pub fn main() !void { var list = std.ArrayList(u32){}; try list.ensureTotalCapacity(0); _ = list; }
