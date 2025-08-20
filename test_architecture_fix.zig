const std = @import("std");
const print = std.debug.print;

pub fn main() !void {
    print("✅ Architecture fix test successful!\n", .{});
    print("🔧 Binary architecture: {s}\n", .{@tagName(@import("builtin").target.cpu.arch)});
    print("🔧 Operating system: {s}\n", .{@tagName(@import("builtin").target.os.tag)});
    print("🔧 ABI: {s}\n", .{@tagName(@import("builtin").target.abi)});
}
