// Minimal LLVM function generation fix for CURSED compiler
// This fixes the "Basic Block in function 'main' does not have terminator!" error

const std = @import("std");

pub fn generateSimpleFunction(writer: anytype, func_name: []const u8, params: []const []const u8, return_type: []const u8, body: []const u8) !void {
    // Generate function signature
    try writer.print("define {s} @{s}(", .{ return_type, func_name });
    
    // Generate parameters
    for (params, 0..) |param, i| {
        if (i > 0) try writer.writeAll(", ");
        try writer.print("i64 %{s}", .{param});
    }
    
    try writer.writeAll(") {\n");
    try writer.writeAll("entry:\n");
    
    // Simple body generation - for now just parse "damn x + y" expressions
    if (std.mem.indexOf(u8, body, "damn ")) |damn_pos| {
        const expr = std.mem.trim(u8, body[damn_pos + 5..], " \t\n\r");
        
        // Handle simple arithmetic expressions like "x + y"
        if (std.mem.indexOf(u8, expr, " + ")) |plus_pos| {
            const left = std.mem.trim(u8, expr[0..plus_pos], " \t");
            const right = std.mem.trim(u8, expr[plus_pos + 3..], " \t");
            
            try writer.print("  %result = add i64 %{s}, %{s}\n", .{ left, right });
            try writer.writeAll("  ret i64 %result\n");
        } else {
            // Single expression return
            try writer.print("  ret i64 %{s}\n", .{expr});
        }
    } else {
        // Default return
        if (std.mem.eql(u8, return_type, "void")) {
            try writer.writeAll("  ret void\n");
        } else {
            try writer.writeAll("  ret i64 0\n");
        }
    }
    
    try writer.writeAll("}\n\n");
}

pub fn parseFunctionDefinition(line: []const u8) ?struct { name: []const u8, params: [][]const u8, return_type: []const u8, body: []const u8 } {
    // Parse "slay add(x drip, y drip) drip { damn x + y }"
    if (!std.mem.startsWith(u8, line, "slay ")) return null;
    
    const after_slay = line[5..];
    
    // Find function name (everything before '(')
    const paren_pos = std.mem.indexOf(u8, after_slay, "(") orelse return null;
    const func_name = std.mem.trim(u8, after_slay[0..paren_pos], " \t");
    
    // Find parameters (between '(' and ')')
    const close_paren = std.mem.indexOf(u8, after_slay[paren_pos..], ")") orelse return null;
    const params_str = std.mem.trim(u8, after_slay[paren_pos + 1..paren_pos + close_paren], " \t");
    
    // Find return type (after ')' and before '{')
    const after_params = after_slay[paren_pos + close_paren + 1..];
    const brace_pos = std.mem.indexOf(u8, after_params, "{") orelse return null;
    const return_type_str = std.mem.trim(u8, after_params[0..brace_pos], " \t");
    
    // Find body (between '{' and '}')
    const close_brace = std.mem.lastIndexOf(u8, after_params, "}") orelse return null;
    const body = std.mem.trim(u8, after_params[brace_pos + 1..close_brace], " \t\n\r");
    
    return .{
        .name = func_name,
        .params = &[_][]const u8{}, // TODO: Parse parameters properly
        .return_type = if (std.mem.eql(u8, return_type_str, "drip")) "i64" else "void",
        .body = body,
    };
}

// Test the fix
test "simple function generation" {
    var buffer = std.ArrayList(u8).init(std.testing.allocator);
    defer buffer.deinit();
    
    const writer = buffer.writer();
    
    try generateSimpleFunction(writer, "add", &[_][]const u8{"x", "y"}, "i64", "damn x + y");
    
    const expected = 
        \\define i64 @add(i64 %x, i64 %y) {
        \\entry:
        \\  %result = add i64 %x, %y
        \\  ret i64 %result
        \\}
        \\
    ;
    
    try std.testing.expectEqualStrings(expected, buffer.items);
}
