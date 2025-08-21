const std = @import("std");

pub fn main() !void {
    const stdout_file = std.io.getStdOut();
    const stdout = stdout_file.writer();
    try stdout.print("test\n", .{});
}
