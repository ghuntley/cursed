const std = @import("std");
const Lexer = @import("src-zig/lexer.zig").Lexer;
const TokenKind = @import("src-zig/lexer.zig").TokenKind;

test "CURSED lexer comprehensive test" {
    const allocator = std.testing.allocator;
    std.debug.print("\n=== CURSED Lexer Comprehensive Test ===\n", .{});

    // Test 1: Basic CURSED keywords
    {
        std.debug.print("\n1. Testing basic CURSED keywords...\n", .{});
        const input = "sus slay vibez bestie yeet damn spill based cringe nah";
        var lexer = Lexer.init(allocator, input);
        const tokens = try lexer.tokenize();
        defer tokens.deinit();

        const expected_keywords = [_]TokenKind{
            .Sus, .Slay, .Identifier, .Bestie, .Yeet, .Identifier, .Spill, .Based, .Cringe, .Nah
        };

        std.debug.print("Expected {} keywords, got {} tokens\n", .{ expected_keywords.len, tokens.items.len });
        
        for (tokens.items, 0..) |token, i| {
            if (i < expected_keywords.len) {
                std.debug.print("  Token {}: {} = '{}' (expected: {})\n", .{ i, token.kind, token.lexeme, expected_keywords[i] });
                try std.testing.expect(token.kind == expected_keywords[i]);
            }
        }
        std.debug.print("✓ Keywords test passed\n", .{});
    }

    // Test 2: String literals
    {
        std.debug.print("\n2. Testing string literals...\n", .{});
        const inputs = [_][]const u8{
            "\"hello world\"",
            "\"escaped \\\"quote\\\"\"",
            "\"multiline\nstring\"",
            "\"\"", // empty string
        };

        for (inputs) |input| {
            var lexer = Lexer.init(allocator, input);
            const tokens = try lexer.tokenize();
            defer tokens.deinit();

            try std.testing.expect(tokens.items.len >= 1);
            try std.testing.expect(tokens.items[0].kind == .StringLiteral);
            std.debug.print("  String: {} = '{}'\n", .{ tokens.items[0].kind, tokens.items[0].lexeme });
        }
        std.debug.print("✓ String literals test passed\n", .{});
    }

    // Test 3: Numbers and identifiers
    {
        std.debug.print("\n3. Testing numbers and identifiers...\n");
        const input = "42 3.14 0 variable_name _underscore camelCase 123.456";
        var lexer = Lexer.init(allocator, input);
        const tokens = try lexer.tokenize();
        defer tokens.deinit();

        const expected_types = [_]TokenKind{
            .Number, .Number, .Number, .Identifier, .Identifier, .Identifier, .Number
        };

        std.debug.print("Expected {} tokens, got {} tokens\n", .{ expected_types.len, tokens.items.len });
        
        for (tokens.items, 0..) |token, i| {
            if (i < expected_types.len) {
                std.debug.print("  Token {}: {} = '{}' (expected: {})\n", .{ i, token.kind, token.lexeme, expected_types[i] });
                try std.testing.expect(token.kind == expected_types[i]);
            }
        }
        std.debug.print("✓ Numbers and identifiers test passed\n");
    }

    // Test 4: Operators and punctuation
    {
        std.debug.print("\n4. Testing operators and punctuation...\n");
        const input = "+ - * / % == != < > <= >= && || ! = := += -= *= /= %= () {} [] , ; : . .. ... ? @ #";
        var lexer = Lexer.init(allocator, input);
        const tokens = try lexer.tokenize();
        defer tokens.deinit();

        const expected_ops = [_]TokenKind{
            .Plus, .Minus, .Star, .Slash, .Percent,
            .EqualEqual, .BangEqual, .Less, .Greater, .LessEqual, .GreaterEqual,
            .AmpAmp, .PipePipe, .Bang, .Equal, .ColonEqual,
            .PlusEqual, .MinusEqual, .StarEqual, .SlashEqual, .PercentEqual,
            .LeftParen, .RightParen, .LeftBrace, .RightBrace, .LeftBracket, .RightBracket,
            .Comma, .Semicolon, .Colon, .Dot, .DotDot, .DotDotDot, .Question, .At, .Hash
        };

        std.debug.print("Expected {} operators, got {} tokens\n", .{ expected_ops.len, tokens.items.len });
        
        for (tokens.items, 0..) |token, i| {
            if (i < expected_ops.len) {
                std.debug.print("  Op {}: {} = '{}' (expected: {})\n", .{ i, token.kind, token.lexeme, expected_ops[i] });
                try std.testing.expect(token.kind == expected_ops[i]);
            }
        }
        std.debug.print("✓ Operators and punctuation test passed\n");
    }

    // Test 5: Character literals
    {
        std.debug.print("\n5. Testing character literals...\n");
        const inputs = [_][]const u8{
            "'a'",
            "'\\n'",
            "'\\t'",
            "'\\''",
        };

        for (inputs) |input| {
            var lexer = Lexer.init(allocator, input);
            const tokens = try lexer.tokenize();
            defer tokens.deinit();

            try std.testing.expect(tokens.items.len >= 1);
            try std.testing.expect(tokens.items[0].kind == .Character);
            std.debug.print("  Char: {} = '{}'\n", .{ tokens.items[0].kind, tokens.items[0].lexeme });
        }
        std.debug.print("✓ Character literals test passed\n");
    }

    // Test 6: Type keywords
    {
        std.debug.print("\n6. Testing type keywords...\n");
        const input = "normie tea lit smol thicc meal snack byte dm";
        var lexer = Lexer.init(allocator, input);
        const tokens = try lexer.tokenize();
        defer tokens.deinit();

        const expected_types = [_]TokenKind{
            .Normie, .Tea, .Lit, .Smol, .Thicc, .Meal, .Snack, .Byte, .Dm
        };

        for (tokens.items, 0..) |token, i| {
            if (i < expected_types.len) {
                std.debug.print("  Type {}: {} = '{}'\n", .{ i, token.kind, token.lexeme });
                try std.testing.expect(token.kind == expected_types[i]);
            }
        }
        std.debug.print("✓ Type keywords test passed\n");
    }

    // Test 7: Comments handling
    {
        std.debug.print("\n7. Testing comments...\n");
        
        // Line comments
        const line_comment_inputs = [_][]const u8{
            "// traditional comment",
            "# hash comment",
            "fr fr cursed comment",
        };

        for (line_comment_inputs) |input| {
            var lexer = Lexer.init(allocator, input);
            const token = try lexer.nextToken();
            std.debug.print("  Line comment: {} = '{}'\n", .{ token.kind, token.lexeme });
            try std.testing.expect(token.kind == .LineComment);
        }

        // Test that comments are filtered in tokenize()
        var lexer = Lexer.init(allocator, "sus x normie // comment\nyeet");
        const tokens = try lexer.tokenize();
        defer tokens.deinit();
        
        // Should only get sus, x, normie, yeet (no comment token)
        try std.testing.expect(tokens.items.len == 4);
        std.debug.print("  Filtered tokens: {}\n", .{tokens.items.len});
        std.debug.print("✓ Comments test passed\n");
    }

    // Test 8: Real CURSED program fragment
    {
        std.debug.print("\n8. Testing real CURSED program fragment...\n");
        const program = 
            \\slay main_character() {
            \\  sus count normie = 42
            \\  vibez.spill("Hello, CURSED!")
            \\  damn count
            \\}
        ;

        var lexer = Lexer.init(allocator, program);
        const tokens = try lexer.tokenize();
        defer tokens.deinit();

        std.debug.print("Program tokens ({}):\n", .{tokens.items.len});
        for (tokens.items, 0..) |token, i| {
            std.debug.print("  {}: {} = '{}'\n", .{ i, token.kind, token.lexeme });
        }

        // Verify key tokens are present
        try std.testing.expect(tokens.items[0].kind == .Slay);
        try std.testing.expect(tokens.items[1].kind == .MainCharacter);
        
        // Find the sus token
        var found_sus = false;
        var found_string = false;
        var found_identifier = false;
        
        for (tokens.items) |token| {
            if (token.kind == .Sus) found_sus = true;
            if (token.kind == .StringLiteral) found_string = true;
            if (token.kind == .Identifier and std.mem.eql(u8, token.lexeme, "vibez")) found_identifier = true;
        }
        
        try std.testing.expect(found_sus);
        try std.testing.expect(found_string);
        try std.testing.expect(found_identifier);
        std.debug.print("✓ Real program fragment test passed\n");
    }

    // Test 9: Error conditions
    {
        std.debug.print("\n9. Testing error conditions...\n");
        
        // Unterminated string
        var lexer1 = Lexer.init(allocator, "\"unterminated string");
        const result1 = lexer1.nextToken();
        try std.testing.expectError(error.UnterminatedString, result1);
        std.debug.print("  ✓ Unterminated string error detected\n");

        // Unterminated character
        var lexer2 = Lexer.init(allocator, "'unterminated");
        const result2 = lexer2.nextToken();
        try std.testing.expectError(error.UnterminatedChar, result2);
        std.debug.print("  ✓ Unterminated character error detected\n");

        // Unexpected character
        var lexer3 = Lexer.init(allocator, "©");
        const result3 = lexer3.nextToken();
        try std.testing.expectError(error.UnexpectedCharacter, result3);
        std.debug.print("  ✓ Unexpected character error detected\n");
        
        std.debug.print("✓ Error conditions test passed\n");
    }

    std.debug.print("\n=== All lexer tests passed! ===\n");
}

pub fn main() !void {
    std.debug.print("Running CURSED lexer comprehensive test...\n");
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Run manual tests that are easier to debug
    try manual_lexer_tests(allocator);
}

fn manual_lexer_tests(allocator: std.mem.Allocator) !void {
    std.debug.print("\n=== Manual Lexer Tests ===\n");
    
    // Test basic functionality
    const test_cases = [_]struct {
        name: []const u8,
        input: []const u8,
        expected_count: usize,
    }{
        .{ .name = "Simple keywords", .input = "sus slay vibez", .expected_count = 3 },
        .{ .name = "Numbers", .input = "42 3.14", .expected_count = 2 },
        .{ .name = "String literal", .input = "\"hello\"", .expected_count = 1 },
        .{ .name = "Operators", .input = "+ - * /", .expected_count = 4 },
        .{ .name = "Mixed", .input = "sus x normie = 42", .expected_count = 5 },
    };
    
    for (test_cases) |case| {
        std.debug.print("\nTesting: {s}\n", .{case.name});
        std.debug.print("Input: '{s}'\n", .{case.input});
        
        var lexer = Lexer.init(allocator, case.input);
        const tokens = try lexer.tokenize();
        defer tokens.deinit();
        
        std.debug.print("Tokens ({}):\n", .{tokens.items.len});
        for (tokens.items, 0..) |token, i| {
            std.debug.print("  {}: {} = '{}' [line {}, col {}]\n", .{ i, token.kind, token.lexeme, token.line, token.column });
        }
        
        if (tokens.items.len == case.expected_count) {
            std.debug.print("✓ Expected token count matched\n");
        } else {
            std.debug.print("✗ Expected {} tokens, got {}\n", .{ case.expected_count, tokens.items.len });
        }
    }
}
