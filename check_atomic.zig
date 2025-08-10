const std = @import("std");

pub fn main() !void {
    std.debug.print("Atomic orders:\n", .{});
    inline for (@typeInfo(std.builtin.AtomicOrder).Enum.fields) |field| {
        std.debug.print("- {s}\n", .{field.name});
    }
}
