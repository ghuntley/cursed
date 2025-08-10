const std = @import("std");
const Lexer = @import("src-zig/lexer.zig").Lexer;
const TokenKind = @import("src-zig/lexer.zig").TokenKind;

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    
    // Test ASCII strings with escape sequences
    const ascii_input = "sus test tea = \"Hello\\nWorld\\t\\u0041\\x42\"";
    
    // Test Unicode strings and identifiers
    const unicode_input = "sus 变量 tea = \"Hello, 世界! 🌍\"";
    
    // Test Unicode escape sequences
    const escape_input = "sus test tea = \"\\u4E16\\u754C\\u{1F30D}\"";
    
    const inputs = [_][]const u8{ ascii_input, unicode_input, escape_input };
    const names = [_][]const u8{ "ASCII", "Unicode", "Escapes" };
    
    for (inputs, names) |input, name| {
        std.debug.print("\n=== Testing {s} ===\n", .{name});
        std.debug.print("Input: {s}\n", .{input});
        
        var lexer = Lexer.init(allocator, input);
        
        while (true) {
            const token = lexer.nextToken() catch |err| {
                std.debug.print("❌ Lexer error: {}\n", .{err});
                break;
            };
            
            if (token.kind == .Eof) break;
            
            std.debug.print("Token: {s} = \"{s}\" (line {}, col {})\n", 
                .{ @tagName(token.kind), token.lexeme, token.line, token.column });
        }
    }
}
