const std = @import("std");
const parser = @import("src-zig/parser.zig");
const lexer = @import("src-zig/lexer.zig");

const CURSED_CODE_WITH_SYNTAX_ERRORS =
\\slay good_function() drip {
\\    damn 42
\\}
\\
\\# Missing parameter type - syntax error
\\slay bad_function(param drip {  
\\    vibez.spill("After error recovery")
\\}
\\
\\sus valid_var drip = 100;
\\
\\# Missing closing parenthesis - syntax error
\\slay another_bad(param drip 
\\
\\sus another_var drip = 200;
\\
\\# Missing operand - syntax error
\\sus broken_expr drip = 1 +;
\\
\\vibez.spill("Final statement");
;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print("=== CURSED Parser Error Recovery Test ===\n\n", .{});
    std.debug.print("Input code with intentional syntax errors:\n", .{});
    std.debug.print("{s}\n\n", .{CURSED_CODE_WITH_SYNTAX_ERRORS});

    // Tokenize the code
    var cursed_lexer = lexer.Lexer.init(allocator, CURSED_CODE_WITH_SYNTAX_ERRORS);

    var tokens = cursed_lexer.tokenize() catch |err| {
        std.debug.print("Tokenization failed: {}\n", .{err});
        return;
    };
    defer tokens.deinit();

    std.debug.print("Tokenization successful. Found {} tokens.\n\n", .{tokens.items.len});

    // Parse with error recovery
    std.debug.print("Starting parsing with error recovery enabled...\n\n", .{});
    
    var cursed_parser = parser.Parser.initWithFile(allocator, tokens.items, "error_recovery_test.csd");
    defer cursed_parser.deinit();

    var program = cursed_parser.parseProgram() catch |err| {
        std.debug.print("\nParsing failed with error: {}\n", .{err});
        std.debug.print("Parser had_error flag: {}\n", .{cursed_parser.had_error});
        std.debug.print("Final token position: {} / {}\n", .{cursed_parser.current, tokens.items.len});
        return;
    };
    defer program.deinit(allocator);

    std.debug.print("\nParsing completed!\n", .{});
    std.debug.print("Successfully parsed {} statements\n", .{program.statements.items.len});
    std.debug.print("Parser had errors but recovered: {}\n", .{cursed_parser.had_error});
    
    if (program.statements.items.len > 0) {
        std.debug.print("\nSuccessfully recovered statements demonstrate error recovery is working.\n", .{});
    }
}
