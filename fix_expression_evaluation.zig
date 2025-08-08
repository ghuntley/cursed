// This is a simplified test to verify the expression evaluation logic
// The fix involves ensuring that binary operators are properly handled before
// falling back to single value evaluation

const std = @import("std");

// Simplified test version of the key function
fn testExpressionParsing() void {
    const test_expr = "x + y";
    
    // The issue: Check for + and - operators (should work)
    const low_ops = [_][]const u8{ "+", "-" };
    for (low_ops) |op| {
        if (std.mem.indexOf(u8, test_expr, op)) |pos| {
            const left_str = std.mem.trim(u8, test_expr[0..pos], " \t");
            const right_str = std.mem.trim(u8, test_expr[pos + op.len..], " \t");
            
            std.debug.print("Found operator '{s}': left='{s}', right='{s}'\n", .{ op, left_str, right_str });
            // This should output: Found operator '+': left='x', right='y'
        }
    }
}

pub fn main() void {
    testExpressionParsing();
}
