const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Create a test reader
    var buffer: [1024]u8 = undefined;
    const stdin_file = std.fs.File.stdin();
    const reader = stdin_file.reader(buffer[0..]);
    
    std.debug.print("Reader type: {}\n", .{@TypeOf(reader)});
    
    // Try different reading methods
    _ = allocator;
    
    // This should work: readUntilDelimiter into buffer
    // const result = try reader.readUntilDelimiter(buffer[0..], '\n');
    // std.debug.print("Read: {s}\n", .{result});
}
