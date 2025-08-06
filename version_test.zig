const std = @import("std");
const package_manager = @import("src-zig/tools/package_manager_enhanced.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const version = package_manager.Version{ .major = 1, .minor = 0, .patch = 0 };
    const version_str = try version.toString(allocator);
    defer allocator.free(version_str);
    
    std.debug.print("Version: {s}\n", .{version_str});
}
