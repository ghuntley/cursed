const std = @import("std");
const lexer = @import("src-zig/lexer.zig");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    
    const source = "damn 42;";
    var lex = lexer.Lexer.init(allocator, source);
    const tokens = try lex.tokenize();
    
    for (tokens.items) |token| {
        std.debug.print("Token: {s}, Kind: {any}\n", .{ token.lexeme, token.kind });
    }
}
