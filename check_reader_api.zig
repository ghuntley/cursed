const std = @import("std");

pub fn main() void {
    const stdin = std.io.getStdIn().reader();
    
    // Check what methods are available
    std.debug.print("Available methods on Reader:\n");
    std.debug.print("- readUntilDelimiterAlloc: {}\n", .{@hasDecl(@TypeOf(stdin), "readUntilDelimiterAlloc")});
    std.debug.print("- readUntilDelimiterArrayList: {}\n", .{@hasDecl(@TypeOf(stdin), "readUntilDelimiterArrayList")});
    std.debug.print("- readUntilDelimiter: {}\n", .{@hasDecl(@TypeOf(stdin), "readUntilDelimiter")});
}
