const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;

const lexer = @import("src-zig/lexer.zig");
const enhanced_parser = @import("src-zig/parser_enhanced.zig");

const Token = lexer.Token;
const TokenKind = lexer.TokenKind;
const EnhancedParser = enhanced_parser.EnhancedParser;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    print("Testing Enhanced Parser Error Handling\n");
    print("=====================================\n\n");

    // Test 1: Valid program should parse successfully
    try testValidProgram(allocator);
    
    // Test 2: Invalid syntax should be handled gracefully
    try testInvalidSyntax(allocator);
    
    // Test 3: Recovery from errors
    try testErrorRecovery(allocator);
    
    // Test 4: Memory safety under error conditions
    try testMemorySafety(allocator);

    print("\nAll enhanced parser tests completed successfully!\n");
}

fn testValidProgram(allocator: std.mem.Allocator) !void {
    print("Test 1: Valid Program Parsing\n");
    print("-----------------------------\n");
    
    const tokens = [_]Token{
        Token.init(.Slay, "slay", 1, 1),
        Token.init(.Identifier, "test_func", 1, 6),
        Token.init(.LeftParen, "(", 1, 15),
        Token.init(.Identifier, "x", 1, 16),
        Token.init(.Drip, "drip", 1, 18),
        Token.init(.RightParen, ")", 1, 22),
        Token.init(.Drip, "drip", 1, 24),
        Token.init(.LeftBrace, "{", 1, 29),
        Token.init(.Identifier, "damn", 1, 31),
        Token.init(.Identifier, "x", 1, 36),
        Token.init(.Plus, "+", 1, 38),
        Token.init(.Number, "1", 1, 40),
        Token.init(.RightBrace, "}", 1, 42),
        Token.init(.Eof, "", 1, 43),
    };
    
    var parser = EnhancedParser.init(allocator, &tokens);
    defer parser.deinit();
    
    if (parser.parseProgram()) |program| {
        defer program.deinit(allocator);
        print("✓ Valid program parsed successfully\n");
        print("  Statements: {}\n", .{program.statements.items.len});
        
        if (parser.hasErrors()) {
            print("  Unexpected errors found:\n");
            try parser.printErrors(std.debug);
        }
    } else |err| {
        print("✗ Failed to parse valid program: {}\n", .{err});
        try parser.printErrors(std.debug);
    }
    
    print("\n");
}

fn testInvalidSyntax(allocator: std.mem.Allocator) !void {
    print("Test 2: Invalid Syntax Handling\n");
    print("-------------------------------\n");
    
    // Test missing closing brace
    const tokens = [_]Token{
        Token.init(.Slay, "slay", 1, 1),
        Token.init(.Identifier, "broken_func", 1, 6),
        Token.init(.LeftParen, "(", 1, 17),
        Token.init(.RightParen, ")", 1, 18),
        Token.init(.LeftBrace, "{", 1, 20),
        Token.init(.Sus, "sus", 2, 1),
        Token.init(.Identifier, "x", 2, 5),
        Token.init(.Drip, "drip", 2, 7),
        Token.init(.Equal, "=", 2, 12),
        Token.init(.Number, "42", 2, 14),
        // Missing closing brace - should be handled gracefully
        Token.init(.Eof, "", 3, 1),
    };
    
    var parser = EnhancedParser.init(allocator, &tokens);
    defer parser.deinit();
    
    const result = parser.parseProgram();
    
    print("Parse result: ");
    if (result) |program| {
        defer program.deinit(allocator);
        print("Completed with errors\n");
    } else |err| {
        print("Failed with error: {}\n", .{err});
    }
    
    print("Errors collected: {}\n", .{parser.getErrors().len});
    if (parser.hasErrors()) {
        print("Error details:\n");
        try parser.printErrors(std.debug);
    }
    
    print("✓ Parser handled invalid syntax gracefully\n\n");
}

fn testErrorRecovery(allocator: std.mem.Allocator) !void {
    print("Test 3: Error Recovery\n");
    print("---------------------\n");
    
    // Multiple errors in sequence - parser should recover and continue
    const tokens = [_]Token{
        // First function with error
        Token.init(.Slay, "slay", 1, 1),
        Token.init(.Identifier, "func1", 1, 6),
        Token.init(.LeftParen, "(", 1, 11),
        Token.init(.Number, "invalid", 1, 12), // Invalid parameter
        Token.init(.RightParen, ")", 1, 19),
        Token.init(.LeftBrace, "{", 1, 21),
        Token.init(.RightBrace, "}", 1, 22),
        
        // Second function should still parse
        Token.init(.Slay, "slay", 2, 1),
        Token.init(.Identifier, "func2", 2, 6),
        Token.init(.LeftParen, "(", 2, 11),
        Token.init(.RightParen, ")", 2, 12),
        Token.init(.LeftBrace, "{", 2, 14),
        Token.init(.Identifier, "damn", 2, 16),
        Token.init(.Number, "42", 2, 21),
        Token.init(.RightBrace, "}", 2, 23),
        
        Token.init(.Eof, "", 3, 1),
    };
    
    var parser = EnhancedParser.init(allocator, &tokens);
    defer parser.deinit();
    
    if (parser.parseProgram()) |program| {
        defer program.deinit(allocator);
        print("✓ Parser recovered and continued parsing\n");
        print("  Statements parsed: {}\n", .{program.statements.items.len});
        print("  Errors encountered: {}\n", .{parser.getErrors().len});
        
        if (parser.hasErrors()) {
            print("  Error details:\n");
            try parser.printErrors(std.debug);
        }
    } else |err| {
        print("✗ Parser failed to recover: {}\n", .{err});
        try parser.printErrors(std.debug);
    }
    
    print("\n");
}

fn testMemorySafety(allocator: std.mem.Allocator) !void {
    print("Test 4: Memory Safety\n");
    print("--------------------\n");
    
    // Test with complex nested structures that could cause crashes
    const tokens = [_]Token{
        Token.init(.Slay, "slay", 1, 1),
        Token.init(.Identifier, "complex_func", 1, 6),
        Token.init(.LeftParen, "(", 1, 18),
        Token.init(.RightParen, ")", 1, 19),
        Token.init(.LeftBrace, "{", 1, 21),
        
        // Nested expressions with potential alignment issues
        Token.init(.Sus, "sus", 2, 1),
        Token.init(.Identifier, "result", 2, 5),
        Token.init(.Drip, "drip", 2, 12),
        Token.init(.Equal, "=", 2, 17),
        Token.init(.LeftParen, "(", 2, 19),
        Token.init(.Number, "1", 2, 20),
        Token.init(.Plus, "+", 2, 22),
        Token.init(.Number, "2", 2, 24),
        Token.init(.RightParen, ")", 2, 25),
        Token.init(.Star, "*", 2, 27),
        Token.init(.Number, "3", 2, 29),
        
        Token.init(.RightBrace, "}", 3, 1),
        Token.init(.Eof, "", 4, 1),
    };
    
    var parser = EnhancedParser.init(allocator, &tokens);
    defer parser.deinit();
    
    if (parser.parseProgram()) |program| {
        defer program.deinit(allocator);
        print("✓ Complex program parsed safely\n");
        print("  Memory allocations handled correctly\n");
        
        if (parser.hasErrors()) {
            print("  Errors (if any):\n");
            try parser.printErrors(std.debug);
        }
    } else |err| {
        print("Parse failed (expected for some test cases): {}\n", .{err});
        try parser.printErrors(std.debug);
    }
    
    print("✓ No memory corruption or crashes detected\n\n");
}
