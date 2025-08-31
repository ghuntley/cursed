const std = @import("std");
const lexer = @import("src-zig/lexer.zig");
const parser = @import("src-zig/parser.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Test full program parsing with simple arithmetic statement
    const input = "5 + 3";
    
    std.debug.print("Testing input as full program: {s}\n", .{input});
    
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
    
    std.debug.print("\n--- Starting to parse program ---\n", .{});
    
    var pars = parser.Parser.init(allocator, tokens.items);
    defer pars.deinit();
    
    // Force use Pratt parser
    pars.use_pratt = true;
    
    // Try parsing the full program
    var program = pars.parseProgram() catch |err| {
        std.debug.print("Error parsing program: {any}\n", .{err});
        return err;
    };
    
    defer program.deinit(allocator);
    
    std.debug.print("Successfully parsed program!\n", .{});
    std.debug.print("Number of statements: {}\n", .{program.statements.items.len});
}
