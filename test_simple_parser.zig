const std = @import("std");
const print = std.debug.print;
const lexer = @import("src-zig/lexer.zig");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const input = "facts(\"test\")";
    
    print("Testing simple facts() tokenization:\n", .{});
    print("Input: {s}\n", .{input});

    var lex = lexer.Lexer.init(allocator, input);
    const tokens = try lex.tokenize();

    print("Tokens found: {d}\n", .{tokens.items.len});
    for (tokens.items, 0..) |token, i| {
        print("Token {d}: kind={}, lexeme='{s}'\n", .{i, token.kind, token.lexeme});
    }

    print("\nChecking for proper tokenization:\n", .{});
    var found_facts = false;
    var found_paren = false;
    
    for (tokens.items) |token| {
        if (std.mem.eql(u8, token.lexeme, "facts")) {
            found_facts = true;
            print("✓ Found 'facts' token with kind: {}\n", .{token.kind});
        }
        if (std.mem.eql(u8, token.lexeme, "(")) {
            found_paren = true;
            print("✓ Found '(' token\n", .{});
        }
    }
    
    if (found_facts and found_paren) {
        print("\n✅ SUCCESS: facts() properly tokenized for function call parsing!\n", .{});
    } else {
        print("\n❌ FAILURE: Tokenization incomplete\n", .{});
    }
}
