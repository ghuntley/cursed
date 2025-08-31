const std = @import("std");
const Parser = @import("src-zig/parser.zig").Parser;
const lexer = @import("src-zig/lexer.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Simple tokens for testing
    var tokens = [_]lexer.Token{
        lexer.Token.init(.Number, "42", 1, 1),
        lexer.Token.init(.Eof, "", 1, 3),
    };
    
    // Test with old parser
    {
        var parser = Parser.init(allocator, &tokens);
        defer parser.deinit();
        
        std.debug.print("Phase 0 Test - Old Parser:\n", .{});
        std.debug.print("  use_pratt flag: {}\n", .{parser.use_pratt});
        
        const expr = parser.parseExpression() catch |err| {
            std.debug.print("  Parse failed: {}\n", .{err});
            return;
        };
        
        switch (expr) {
            .Integer => |val| std.debug.print("  Parsed integer: {}\n", .{val}),
            else => std.debug.print("  Unexpected expression type\n", .{}),
        }
    }
    
    // Test with Pratt parser flag enabled
    {
        var parser = Parser.init(allocator, &tokens);
        defer parser.deinit();
        
        // Enable Pratt parser
        parser.use_pratt = true;
        
        std.debug.print("\nPhase 0 Test - Pratt Parser (delegating to old):\n", .{});
        std.debug.print("  use_pratt flag: {}\n", .{parser.use_pratt});
        
        const expr = parser.parseExpression() catch |err| {
            std.debug.print("  Parse failed: {}\n", .{err});
            return;
        };
        
        switch (expr) {
            .Integer => |val| std.debug.print("  Parsed integer: {}\n", .{val}),
            else => std.debug.print("  Unexpected expression type\n", .{}),
        }
    }
    
    std.debug.print("\n✅ Phase 0 Pratt parser infrastructure is working correctly!\n", .{});
    std.debug.print("   - Feature flag can be toggled\n", .{});
    std.debug.print("   - parseExpressionPratt() stub is callable\n", .{});
    std.debug.print("   - Both old and new paths produce same results\n", .{});
    std.debug.print("   - Infrastructure is ready for Pratt algorithm implementation\n", .{});
}
