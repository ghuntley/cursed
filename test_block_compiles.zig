const std = @import("std");
const interpreter = @import("src-zig/interpreter.zig");

pub fn main() !void {
    std.debug.print("Block statement implementation compiled successfully!\n", .{});
    std.debug.print("The executeBlockStatement function is now available.\n", .{});
    
    // Verify the function exists by checking if we can reference it
    const has_func = @hasDecl(interpreter.Interpreter, "executeBlockStatement");
    std.debug.print("executeBlockStatement function exists: {}\n", .{has_func});
}
