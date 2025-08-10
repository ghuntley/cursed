const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("src-zig/lexer.zig");
const parser = @import("src-zig/parser.zig");
const ast = @import("src-zig/ast.zig");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const input = 
        \\// Test the working functionality
        \\yeet "vibez"
        \\
        \\sus x drip = 42
        \\sus message tea = "Hello CURSED!"
        \\
        \\facts("Testing basic output:", x)
        \\facts("Message:", message)
        \\vibez.spill("Using vibez module:", x + 8)
    ;

    print("Testing parser fix for facts() function calls...\n", .{});
    print("Input:\n{s}\n\n", .{input});

    // Tokenize
    var lex = lexer.Lexer.init(allocator, input);
    const tokens = try lex.tokenize();

    print("Tokens found: {d}\n", .{tokens.items.len});
    for (tokens.items) |token| {
        if (std.mem.eql(u8, token.lexeme, "facts")) {
            print("Found 'facts' token: kind={}, lexeme='{s}', line={d}\n", .{token.kind, token.lexeme, token.line});
        }
    }

    // Parse
    var p = parser.Parser.init(allocator, tokens.items);
    const program = p.parseProgram() catch |err| {
        print("Parse error: {}\n", .{err});
        return;
    };

    print("\nParsing completed successfully!\n", .{});
    print("Program has {d} statements\n", .{program.statements.items.len});

    var expression_count: u32 = 0;
    var let_count: u32 = 0;
    var import_count: u32 = 0;
    
    for (program.statements.items, 0..) |stmt, i| {
        print("Statement {d}: {s}\n", .{i, @tagName(stmt)});
        if (std.mem.eql(u8, @tagName(stmt), "Expression")) {
            expression_count += 1;
        } else if (std.mem.eql(u8, @tagName(stmt), "Let")) {
            let_count += 1;
        } else if (std.mem.eql(u8, @tagName(stmt), "Import")) {
            import_count += 1;
        }
    }
    
    print("\nSummary:\n", .{});
    print("- Expression statements (function calls): {d}\n", .{expression_count});
    print("- Let statements (variable declarations): {d}\n", .{let_count});
    print("- Import statements: {d}\n", .{import_count});
    
    if (expression_count == 2) {
        print("\n✅ SUCCESS: Both facts() calls were parsed as expression statements!\n", .{});
    } else {
        print("\n❌ FAILURE: Expected 2 expression statements but got {d}\n", .{expression_count});
    }
}
