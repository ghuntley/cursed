const std = @import("std");
const lexer = @import("src-zig/lexer.zig");
const parser = @import("src-zig/parser.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Test method call: "vibez.spill("test")"
    const input = "vibez.spill(\"test\")";
    
    std.debug.print("Testing input: {s}\n", .{input});
    
    var lex = lexer.Lexer.init(allocator, input);
    
    var tokens = std.ArrayList(lexer.Token){};
    defer tokens.deinit(allocator);
    
    // Tokenize
    while (true) {
        const token = try lex.nextToken();
        try tokens.append(allocator, token);
        std.debug.print("Token: {any} '{s}'\n", .{token.kind, token.lexeme});
        if (token.kind == .Eof) break;
    }
    
    std.debug.print("\n--- Starting to parse expression ---\n", .{});
    
    var pars = parser.Parser.init(allocator, tokens.items);
    defer pars.deinit();
    
    // Force use Pratt parser
    pars.use_pratt = true;
    
    // Try parsing a single expression
    const expr = pars.parseExpression() catch |err| {
        std.debug.print("Error parsing expression: {any}\n", .{err});
        return err;
    };
    
    std.debug.print("Successfully parsed expression!\n", .{});
    std.debug.print("Expression type: {any}\n", .{expr});
}
