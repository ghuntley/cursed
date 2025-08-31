const std = @import("std");
const print = std.debug.print;

pub fn main() !void {
    // Simple direct test by creating a minimal binary like existing ones
    const source =
        \\yeet "vibez"
        \\slay main_character() {
        \\    vibez.spill("Time module test")
        \\    vibez.spill("02")
        \\}
    ;

    // Write source to temp file
    const file = try std.fs.cwd().createFile("temp_time_test.csd", .{});
    defer file.close();
    try file.writeAll(source);

    print("✅ Created simple time test file\n", .{});
    print("Source: {s}\n", .{source});
}
