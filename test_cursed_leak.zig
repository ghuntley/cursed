const std = @import("std");
const ArrayList = std.ArrayList;
const lexer = @import("src-zig/lexer.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const source = "vibez.spill(\"test\")";
    
    // Test lexer with intentional memory leak
    var l = lexer.Lexer.init(allocator, source);
    const tokens = try l.tokenize();
    // defer tokens.deinit(); // INTENTIONALLY COMMENTED OUT TO SIMULATE LEAK
    
    std.debug.print("Tokenized {} tokens\n", .{tokens.items.len});
    for (tokens.items) |token| {
        std.debug.print("Token: {s}\n", .{@tagName(token.kind)});
    }
}
