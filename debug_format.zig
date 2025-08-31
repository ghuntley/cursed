const std = @import("std");

pub fn main() !void {
    const float_val: f64 = 3.5;
    std.debug.print("Zig {{d}} format: {d}\n", .{float_val});
    std.debug.print("Zig {{}} format: {}\n", .{float_val});
}
