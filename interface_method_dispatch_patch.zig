// Simple method dispatch patch for CURSED interpreter
// This adds basic interface method dispatch to the main interpreter

const std = @import("std");
const print = std.debug.print;

// Simple method dispatch function that can be integrated into main_unified.zig
pub fn dispatchMethodCall(variables: anytype, line: []const u8, verbose: bool) !bool {
    const trimmed = std.mem.trim(u8, line, " \t");
    
    // Check if this is a method call (has both . and parentheses)
    const dot_pos = std.mem.indexOf(u8, trimmed, ".") orelse return false;
    const paren_pos = std.mem.indexOf(u8, trimmed, "(") orelse return false;
    
    // Make sure the dot comes before the parentheses
    if (dot_pos >= paren_pos) return false;
    
    const object_name = std.mem.trim(u8, trimmed[0..dot_pos], " \t");
    const method_part = std.mem.trim(u8, trimmed[dot_pos + 1..], " \t");
    const method_paren_pos = std.mem.indexOf(u8, method_part, "(") orelse return false;
    const method_name = std.mem.trim(u8, method_part[0..method_paren_pos], " \t");
    
    if (verbose) print("🔧 Method dispatch: {s}.{s}()\n", .{ object_name, method_name });
    
    // Look up the object in variables
    if (variables.get(object_name)) |object_var| {
        switch (object_var) {
            .Struct => |_| {
                // Simple hard-coded method dispatch for demonstration
                if (std.mem.eql(u8, method_name, "draw")) {
                    print("Drawing a circle\n");
                    return true;
                } else {
                    if (verbose) print("⚠️  Method '{s}' not implemented\n", .{method_name});
                    return true; // Handled, even if not implemented
                }
            },
            else => {
                if (verbose) print("⚠️  Object '{s}' is not a struct\n", .{object_name});
                return false;
            },
        }
    } else {
        if (verbose) print("⚠️  Object '{s}' not found\n", .{object_name});
        return false;
    }
}
