const std = @import("std");
const parser = @import("src-zig/parser.zig");
const lexer = @import("src-zig/lexer.zig");

const CURSED_CODE_WITH_ERRORS =
\\# Test file demonstrating error recovery
\\slay good_function() drip {
\\    damn 42
\\}
\\
\\slay bad_function(param) {
\\    vibez.spill("Should recover after param error")
\\}
\\
\\sus valid_var drip = 100;
\\sus broken_expr drip = 1 + + 2;
\\vibez.spill("After expression error");
\\
\\slay incomplete_function(
\\
\\sus another_var drip = 42;
\\vibez.spill("Final statement");
;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print("=== CURSED Parser Error Recovery Demo ===\n\n");
    std.debug.print("Input code with intentional errors:\n");
    std.debug.print("{s}\n\n", .{CURSED_CODE_WITH_ERRORS});

    // Tokenize the code
    var cursed_lexer = lexer.Lexer.init(allocator, CURSED_CODE_WITH_ERRORS);
    defer cursed_lexer.deinit();

    const tokens = cursed_lexer.tokenize() catch |err| {
        std.debug.print("Tokenization failed: {}\n", .{err});
        return;
    };
    defer allocator.free(tokens);

    std.debug.print("Tokenization successful. Found {} tokens.\n\n", .{tokens.len});

    // Parse with error recovery
    var cursed_parser = parser.Parser.initWithFile(allocator, tokens, "error_test.csd");
    defer cursed_parser.deinit();

    std.debug.print("Starting parsing with error recovery...\n\n");

    const program = cursed_parser.parseProgram() catch |err| {
        std.debug.print("Parsing completed with errors. Error type: {}\n", .{err});
        
        // Even with errors, show what we recovered
        std.debug.print("\nParser state after errors:\n");
        std.debug.print("had_error: {}\n", .{cursed_parser.had_error});
        std.debug.print("current token position: {} / {}\n", .{cursed_parser.current, tokens.len});
        
        return;
    };
    defer program.deinit(allocator);

    std.debug.print("\nParsing completed successfully!\n");
    std.debug.print("Parsed {} statements\n", .{program.statements.items.len});
    std.debug.print("Parser had errors: {}\n", .{cursed_parser.had_error});
}
