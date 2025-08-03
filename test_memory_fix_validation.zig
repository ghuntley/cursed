const std = @import("std");
const ArrayList = std.ArrayList;
const lexer = @import("src-zig/lexer.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const source = 
        \\fr fr Complex test program
        \\vibez.spill("Hello World")
        \\sus var1 drip = 42
        \\sus var2 drip = 84
        \\vibez.spill("Variable:", var1)
        \\vibez.spill("Another variable:", var2)
        \\fr fr More comments
        \\vibez.spill("Final test")
    ;
    
    std.debug.print("Testing CURSED lexer memory management...\n", .{});
    
    // Test with PROPER memory management (our fix)
    {
        var l = lexer.Lexer.init(allocator, source);
        const tokens = try l.tokenize();
        defer tokens.deinit(); // CRITICAL FIX: This prevents leaks
        
        std.debug.print("✅ Properly tokenized {} tokens with cleanup\n", .{tokens.items.len});
    }
    
    std.debug.print("✅ Memory test completed - no leaks should be detected\n", .{});
}
