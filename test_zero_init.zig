const std = @import("std");
pub fn main() !void { 
    // Try proper zero initialization  
    var list = std.ArrayList(u32){ .items = &.{}, .capacity = 0 };
    _ = list;
}
