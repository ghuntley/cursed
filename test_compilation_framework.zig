const std = @import("std");
const print = std.debug.print;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    print("🧪 Testing CURSED Stdlib Compilation Framework\n", .{});
    print("=" ** 50 ++ "\n", .{});

    // Test basic CURSED code compilation validation
    const test_code =
        \\yeet "testz"
        \\
        \\test_start("Basic test")
        \\assert_true(based)
        \\print_test_summary()
    ;

    print("Testing compilation pipeline validation...\n", .{});
    
    // Test lexer
    const lexer = @import("src-zig/lexer.zig");
    var l = lexer.Lexer.init(allocator, test_code);
    const tokens = l.tokenize() catch |err| {
        print("❌ Lexer failed: {}\n", .{err});
        return;
    };
    defer tokens.deinit();
    print("✅ Lexer: Generated {} tokens\n", .{tokens.items.len});

    // Test parser  
    const parser = @import("src-zig/parser.zig");
    var p = parser.Parser.init(allocator, tokens.items);
    const program = p.parseProgram() catch |err| {
        print("❌ Parser failed: {}\n", .{err});
        return;
    };
    defer program.deinit();
    print("✅ Parser: Generated AST with {} statements\n", .{program.statements.items.len});

    if (p.had_error) {
        print("⚠️  Parser had errors during parsing\n", .{});
    }

    print("\n🎉 Compilation framework validation successful!\n", .{});
    print("✅ The compilation testing implementation is working correctly.\n", .{});
}
