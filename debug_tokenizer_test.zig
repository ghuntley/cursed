const std = @import("std");
const print = std.debug.print;
const lexer = @import("src-zig/lexer.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const source = "slay spill(msg tea) lit {\n    damn based\n}";
    
    var lex = lexer.Lexer.init(allocator, source);
    const tokens = try lex.tokenize();
    defer tokens.deinit();
    
    print("Source: {s}\n", .{source});
    print("Tokens ({} total):\n", .{tokens.items.len});
    
    for (tokens.items, 0..) |token, i| {
        print("  [{}] {s} = '{s}' (line: {}, col: {})\n", .{ i, @tagName(token.kind), token.lexeme, token.line, token.column });
    }
}
