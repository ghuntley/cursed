const std = @import("std");
const formatter = @import("src-zig/tools/formatter.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const source = "sus x drip=42;vibez.spill(x)";
    std.debug.print("Input: {s}\n", .{source});
    
    var fmt = formatter.Formatter.init(allocator, formatter.FormatterConfig{});
    defer fmt.deinit();
    
    const result = try fmt.format(source);
    defer allocator.free(result);
    
    std.debug.print("Output: {s}\n", .{result});
}
