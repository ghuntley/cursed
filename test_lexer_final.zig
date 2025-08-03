const std = @import("std");
const lexer = @import("src-zig/lexer.zig");
const Lexer = lexer.Lexer;
const TokenKind = lexer.TokenKind;

test "final comprehensive CURSED lexer validation" {
    const allocator = std.testing.allocator;
    
    const cursed_program = 
        \\fr fr Complete CURSED program test
        \\vibe main
        \\
        \\yeet "vibez"
        \\
        \\slay greet(name tea) {
        \\    vibez.spill("Hello, " + name + "!")
        \\}
        \\
        \\slay getValue() normie {
        \\    yolo 42
        \\}
        \\
        \\slay main() {
        \\    fr fr Variables and types
        \\    sus count normie = 42
        \\    sus message tea = "CURSED rocks!"
        \\    sus pi meal = 3.14159
        \\    sus isAwesome lit = based
        \\    sus notReady lit = cringe
        \\    
        \\    fr fr Function calls
        \\    greet("World")
        \\    
        \\    fr fr Conditionals
        \\    lowkey count > 0 {
        \\        vibez.spill("Positive")
        \\    } highkey {
        \\        vibez.spill("Non-positive")
        \\    }
        \\    
        \\    fr fr Error handling
        \\    yikes MyError = {
        \\        code normie
        \\        msg tea
        \\    }
        \\    
        \\    fr fr Pattern matching
        \\    match count {
        \\        0 => vibez.spill("zero"),
        \\        42 => vibez.spill("answer"),
        \\        _ => vibez.spill("other")
        \\    }
        \\    
        \\    no cap
        \\    This is a block comment
        \\    that spans multiple lines
        \\    and should be skipped
        \\    on god
        \\    
        \\    vibez.spill("Program complete!")
        \\}
    ;
    
    var test_lexer = Lexer.init(allocator, cursed_program);
    const tokens = try test_lexer.tokenize();
    defer tokens.deinit();
    
    std.debug.print("\n🎯 Final CURSED lexer test with {} tokens\n", .{tokens.items.len});
    
    // Verify key tokens are present and correctly categorized
    var token_counts = std.HashMap(TokenKind, u32, TokenKindContext, 80).init(allocator);
    defer token_counts.deinit();
    
    for (tokens.items) |token| {
        const result = try token_counts.getOrPut(token.kind);
        if (!result.found_existing) {
            result.value_ptr.* = 1;
        } else {
            result.value_ptr.* += 1;
        }
    }
    
    // Check that we have the expected CURSED keywords
    const expected_keywords = [_]TokenKind{
        .Vibe, .Yeet, .Slay, .Sus, .Yolo, .Lowkey, .Highkey, 
        .Normie, .Tea, .Meal, .Lit, .Based, .Cap, .Yikes, .Match
    };
    
    var missing_keywords: u32 = 0;
    for (expected_keywords) |expected| {
        if (!token_counts.contains(expected)) {
            std.debug.print("❌ Missing token kind: {}\n", .{expected});
            missing_keywords += 1;
        } else {
            std.debug.print("✅ Found token kind: {} (count: {})\n", .{ expected, token_counts.get(expected).? });
        }
    }
    
    // Verify literals are parsed correctly
    var found_string_literal = false;
    var found_number_literal = false;
    var found_boolean_literal = false;
    
    for (tokens.items) |token| {
        if (token.kind == .StringLiteral) found_string_literal = true;
        if (token.kind == .Number) found_number_literal = true;
        if (token.kind == .Based or token.kind == .Cap) found_boolean_literal = true;
    }
    
    try std.testing.expect(found_string_literal);
    try std.testing.expect(found_number_literal);
    try std.testing.expect(found_boolean_literal);
    
    // Verify no comment content appears as regular tokens
    for (tokens.items) |token| {
        try std.testing.expect(!std.mem.eql(u8, token.lexeme, "Complete"));
        try std.testing.expect(!std.mem.eql(u8, token.lexeme, "block"));
        try std.testing.expect(!std.mem.eql(u8, token.lexeme, "comment"));
        try std.testing.expect(!std.mem.eql(u8, token.lexeme, "spans"));
    }
    
    try std.testing.expect(missing_keywords == 0);
    
    std.debug.print("🎉 Final lexer validation passed! All {} expected keywords found.\n", .{expected_keywords.len});
}

// Helper for HashMap with TokenKind keys
const TokenKindContext = struct {
    pub fn hash(self: @This(), s: TokenKind) u64 {
        _ = self;
        return std.hash_map.hashString(@tagName(s));
    }
    
    pub fn eql(self: @This(), a: TokenKind, b: TokenKind) bool {
        _ = self;
        return a == b;
    }
};

test "operator and punctuation parsing" {
    const allocator = std.testing.allocator;
    
    const operators_code = 
        \\sus result = (a + b) * c / d - e % f
        \\sus comparison = x == y && z != w || p > q && r <= s
        \\sus assignment = value := 42
        \\sus increment = count++
        \\sus decrement = index--
    ;
    
    var test_lexer = Lexer.init(allocator, operators_code);
    const tokens = try test_lexer.tokenize();
    defer tokens.deinit();
    
    std.debug.print("\n🔧 Testing operators and punctuation with {} tokens\n", .{tokens.items.len});
    
    // Check for essential operators
    var found_operators = std.HashMap(TokenKind, bool, TokenKindContext, 80).init(allocator);
    defer found_operators.deinit();
    
    const expected_operators = [_]TokenKind{
        .Plus, .Minus, .Star, .Slash, .Percent,
        .EqualEqual, .BangEqual, .AmpAmp, .PipePipe,
        .Greater, .LessEqual, .ColonEqual, .PlusPlus, .MinusMinus,
        .LeftParen, .RightParen
    };
    
    for (tokens.items) |token| {
        for (expected_operators) |op| {
            if (token.kind == op) {
                try found_operators.put(op, true);
            }
        }
    }
    
    var missing_ops: u32 = 0;
    for (expected_operators) |op| {
        if (!found_operators.contains(op)) {
            std.debug.print("❌ Missing operator: {}\n", .{op});
            missing_ops += 1;
        }
    }
    
    try std.testing.expect(missing_ops == 0);
    std.debug.print("✅ All {} operators found!\n", .{expected_operators.len});
}
