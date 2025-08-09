const std = @import("std");
const target_mapping = @import("src-zig/target_mapping.zig");

pub fn main() !void {
    const result = target_mapping.targetToLLVMTriple("linux-x64");
    if (result) |r| {
        std.debug.print("Result: {s}\n", .{r});
    } else {
        std.debug.print("Result: null\n", .{});
    }
}
