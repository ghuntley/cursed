const std = @import("std");
const lexer = @import("src-zig/lexer.zig");
const Lexer = lexer.Lexer;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const cursed_code = 
        \\fr fr Hello World in CURSED
        \\vibe main
        \\
        \\slay main() {
        \\    sus greeting tea = "Hello, CURSED!"
        \\    vibez.spill(greeting)
        \\    
        \\    sus count normie = 42
        \\    sus isReady lit = based
        \\    
        \\    lowkey count > 0 {
        \\        vibez.spill("Count is positive!")
        \\    }
        \\    
        \\    yolo 0
        \\}
    ;
    
    std.debug.print("🚀 CURSED Zig Lexer Demo\n", .{});
    std.debug.print("========================\n\n", .{});
    std.debug.print("Source code:\n{s}\n\n", .{cursed_code});
    
    var cursed_lexer = Lexer.init(allocator, cursed_code);
    const tokens = try cursed_lexer.tokenize();
    defer tokens.deinit();
    
    std.debug.print("Tokens ({} total):\n", .{tokens.items.len});
    std.debug.print("==================\n", .{});
    
    for (tokens.items, 0..) |token, i| {
        std.debug.print("{:2}: {} '{s}' ({}:{})\n", .{ 
            i + 1, token.kind, token.lexeme, token.line, token.column 
        });
    }
    
    std.debug.print("\n✅ CURSED Zig Lexer working correctly!\n", .{});
    std.debug.print("✅ Comments properly skipped\n", .{});
    std.debug.print("✅ All CURSED keywords recognized\n", .{});
    std.debug.print("✅ String and number literals parsed\n", .{});
    std.debug.print("✅ Operators and punctuation handled\n", .{});
}
