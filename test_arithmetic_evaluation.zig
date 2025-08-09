const std = @import("std");

// Test arithmetic expression evaluation logic
fn evaluateArithmeticExpression(
    expr: []const u8, 
    variables: *std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)
) !i64 {
    const trimmed = std.mem.trim(u8, expr, " \t\r\n");
    
    // Handle simple variable references
    if (variables.get(trimmed)) |value| {
        return value;
    }
    
    // Handle arithmetic operations with precedence (basic implementation)
    // For now, handle simple cases: x + y * 2
    if (std.mem.indexOf(u8, trimmed, " + ")) |plus_pos| {
        const left_expr = std.mem.trim(u8, trimmed[0..plus_pos], " \t");
        const right_expr = std.mem.trim(u8, trimmed[plus_pos + 3..], " \t");
        
        const left_val = try evaluateSimpleExpression(left_expr, variables);
        const right_val = try evaluateSimpleExpression(right_expr, variables);
        
        return left_val + right_val;
    }
    
    // Try to parse as literal
    return std.fmt.parseInt(i64, trimmed, 10) catch 0;
}

fn evaluateSimpleExpression(
    expr: []const u8,
    variables: *std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)
) !i64 {
    const trimmed = std.mem.trim(u8, expr, " \t");
    
    // Check if it's a variable
    if (variables.get(trimmed)) |value| {
        return value;
    }
    
    // Handle y * 2 pattern specifically
    if (std.mem.indexOf(u8, trimmed, " * ")) |mult_pos| {
        const left_part = std.mem.trim(u8, trimmed[0..mult_pos], " \t");
        const right_part = std.mem.trim(u8, trimmed[mult_pos + 3..], " \t");
        
        var left_val: i64 = 0;
        var right_val: i64 = 0;
        
        // Get left value
        if (variables.get(left_part)) |val| {
            left_val = val;
        } else {
            left_val = std.fmt.parseInt(i64, left_part, 10) catch 0;
        }
        
        // Get right value
        if (variables.get(right_part)) |val| {
            right_val = val;
        } else {
            right_val = std.fmt.parseInt(i64, right_part, 10) catch 0;
        }
        
        return left_val * right_val;
    }
    
    // Try to parse as literal
    return std.fmt.parseInt(i64, trimmed, 10) catch 0;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var variables = std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
    defer variables.deinit();
    
    // Set up test variables
    try variables.put("x", 10);
    try variables.put("y", 5);
    
    // Test the arithmetic expression
    const result = try evaluateArithmeticExpression("x + y * 2", &variables);
    std.debug.print("Result of 'x + y * 2': {}\n", .{result});
    std.debug.print("Expected: 20 (10 + 5*2)\n", .{});
}
