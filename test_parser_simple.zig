const std = @import("std");
const lexer = @import("src-zig/lexer.zig");
const parser = @import("src-zig/parser.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print("Testing Enhanced Parser Error Handling\n", .{});
    std.debug.print("=====================================\n\n", .{});

    // Test 1: Valid program
    try testValidProgram(allocator);
    
    // Test 2: Invalid syntax
    try testInvalidSyntax(allocator);

    std.debug.print("Enhanced parser tests completed!\n", .{});
}

fn testValidProgram(allocator: std.mem.Allocator) !void {
    std.debug.print("Test 1: Valid Program\n", .{});
    std.debug.print("---------------------\n", .{});
    
    const tokens = [_]lexer.Token{
        lexer.Token.init(.Slay, "slay", 1, 1),
        lexer.Token.init(.Identifier, "test_func", 1, 6),
        lexer.Token.init(.LeftParen, "(", 1, 15),
        lexer.Token.init(.RightParen, ")", 1, 16),
        lexer.Token.init(.LeftBrace, "{", 1, 18),
        lexer.Token.init(.Identifier, "damn", 1, 20),
        lexer.Token.init(.Number, "42", 1, 25),
        lexer.Token.init(.RightBrace, "}", 1, 28),
        lexer.Token.init(.Eof, "", 1, 29),
    };
    
    var p = parser.Parser.init(allocator, &tokens);
    defer p.deinit();
    
    var result = p.parseProgram();
    if (result) |*program| {
        defer program.deinit(allocator);
        std.debug.print("✓ Valid program parsed successfully\n", .{});
        std.debug.print("  Statements: {}\n", .{program.statements.items.len});
    } else |err| {
        std.debug.print("✗ Failed to parse valid program: {}\n", .{err});
    }
    
    std.debug.print("\n", .{});
}

fn testInvalidSyntax(allocator: std.mem.Allocator) !void {
    std.debug.print("Test 2: Invalid Syntax\n", .{});
    std.debug.print("----------------------\n", .{});
    
    // Test with missing closing brace
    const tokens = [_]lexer.Token{
        lexer.Token.init(.Slay, "slay", 1, 1),
        lexer.Token.init(.Identifier, "broken_func", 1, 6),
        lexer.Token.init(.LeftParen, "(", 1, 17),
        lexer.Token.init(.RightParen, ")", 1, 18),
        lexer.Token.init(.LeftBrace, "{", 1, 20),
        lexer.Token.init(.Sus, "sus", 2, 1),
        lexer.Token.init(.Identifier, "x", 2, 5),
        // Missing closing brace - should be handled gracefully
        lexer.Token.init(.Eof, "", 3, 1),
    };
    
    var p = parser.Parser.init(allocator, &tokens);
    defer p.deinit();
    
    var result = p.parseProgram();
    
    std.debug.print("Parse result: ", .{});
    if (result) |*program| {
        defer program.deinit(allocator);
        std.debug.print("Completed (with errors)\n", .{});
    } else |err| {
        std.debug.print("Failed with error: {}\n", .{err});
    }
    
    std.debug.print("✓ Parser handled invalid syntax gracefully (no crashes)\n\n", .{});
}
