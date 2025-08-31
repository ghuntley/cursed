const std = @import("std");
const testing = std.testing;
const Parser = @import("src-zig/parser.zig").Parser;
const lexer = @import("src-zig/lexer.zig");
const Token = lexer.Token;

// Test that the Pratt parser infrastructure works correctly
test "Pratt parser Phase 0 - feature flag toggle" {
    const allocator = testing.allocator;
    
    // Test tokens for simple expression: 42 + 24
    const tokens = [_]Token{
        Token.init(.Number, "42", 1, 1),
        Token.init(.Plus, "+", 1, 4),
        Token.init(.Number, "24", 1, 6),
        Token.init(.Eof, "", 1, 8),
    };
    
    // Test with old parser (use_pratt = false)
    {
        var parser = Parser.init(allocator, &tokens);
        defer parser.deinit();
        
        // Ensure flag is false by default
        try testing.expect(parser.use_pratt == false);
        
        // Parse expression using old parser
        const expr = parser.parseExpression() catch |err| {
            std.debug.print("Old parser failed: {}\n", .{err});
            return err;
        };
        
        // Verify we get a binary expression
        switch (expr) {
            .Binary => |bin| {
                try testing.expect(std.mem.eql(u8, bin.operator, "+"));
            },
            else => {
                std.debug.print("Expected Binary expression, got: {}\n", .{expr});
                return error.UnexpectedExpressionType;
            }
        }
    }
    
    // Test with Pratt parser flag (use_pratt = true)
    {
        var parser = Parser.init(allocator, &tokens);
        defer parser.deinit();
        
        // Enable Pratt parser
        parser.use_pratt = true;
        try testing.expect(parser.use_pratt == true);
        
        // Parse expression using Pratt parser (which currently delegates to old parser)
        const expr = parser.parseExpression() catch |err| {
            std.debug.print("Pratt parser failed: {}\n", .{err});
            return err;
        };
        
        // Should get same result as old parser for now
        switch (expr) {
            .Binary => |bin| {
                try testing.expect(std.mem.eql(u8, bin.operator, "+"));
            },
            else => {
                std.debug.print("Expected Binary expression, got: {}\n", .{expr});
                return error.UnexpectedExpressionType;
            }
        }
    }
}

// Test parsing more complex expressions to ensure compatibility
test "Pratt parser Phase 0 - complex expression compatibility" {
    const allocator = testing.allocator;
    
    // Test tokens for: 1 + 2 * 3
    const tokens = [_]Token{
        Token.init(.Number, "1", 1, 1),
        Token.init(.Plus, "+", 1, 3), 
        Token.init(.Number, "2", 1, 5),
        Token.init(.Star, "*", 1, 7),
        Token.init(.Number, "3", 1, 9),
        Token.init(.Eof, "", 1, 10),
    };
    
    // Test old parser
    var old_parser = Parser.init(allocator, &tokens);
    defer old_parser.deinit();
    old_parser.use_pratt = false;
    
    const old_expr = old_parser.parseExpression() catch |err| {
        std.debug.print("Old parser failed on complex expression: {}\n", .{err});
        return err;
    };
    
    // Reset tokens for Pratt parser
    var pratt_parser = Parser.init(allocator, &tokens);
    defer pratt_parser.deinit();
    pratt_parser.use_pratt = true;
    
    const pratt_expr = pratt_parser.parseExpression() catch |err| {
        std.debug.print("Pratt parser failed on complex expression: {}\n", .{err});
        return err;
    };
    
    // Both should produce binary expressions (exact structure may differ but both should work)
    switch (old_expr) {
        .Binary => {},
        else => return error.UnexpectedExpressionType,
    }
    
    switch (pratt_expr) {
        .Binary => {},
        else => return error.UnexpectedExpressionType,
    }
}

// Test direct parsing method access  
test "Pratt parser Phase 0 - direct method calls" {
    const allocator = testing.allocator;
    
    const tokens = [_]Token{
        Token.init(.Number, "42", 1, 1),
        Token.init(.Eof, "", 1, 3),
    };
    
    var parser = Parser.init(allocator, &tokens);
    defer parser.deinit();
    
    // Test that we can call parseExpressionPratt directly
    const expr = parser.parseExpressionPratt() catch |err| {
        std.debug.print("Direct parseExpressionPratt call failed: {}\n", .{err});
        return err;
    };
    
    switch (expr) {
        .Integer => |val| try testing.expect(val == 42),
        else => return error.UnexpectedExpressionType,
    }
}
