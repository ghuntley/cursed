const std = @import("std");
const lexer = @import("src-zig/lexer.zig");
const Lexer = lexer.Lexer;
const TokenKind = lexer.TokenKind;

test "comprehensive CURSED lexer test" {
    const allocator = std.testing.allocator;
    
    const cursed_code = 
        \\fr fr This is a comprehensive CURSED test program
        \\vibe main
        \\
        \\yeet "vibez"  fr fr import standard library
        \\
        \\slay calculateArea(radius snack) snack {
        \\    yolo 3.14159 * radius * radius
        \\}
        \\
        \\slay main() {
        \\    fr fr Variable declarations
        \\    sus x normie = 42
        \\    sus name tea = "Hello CURSED!"
        \\    sus isAwesome lit = based  fr fr true
        \\    sus notGood lit = cringe   fr fr false
        \\    
        \\    fr fr Function call
        \\    sus area = calculateArea(5.0)
        \\    
        \\    fr fr Conditionals
        \\    lowkey x > 0 {
        \\        vibez.spill("Positive number!")
        \\    } highkey {
        \\        vibez.spill("Zero or negative")
        \\    }
        \\    
        \\    fr fr Error handling
        \\    yikes CustomError = {
        \\        message tea
        \\    }
        \\    
        \\    fr fr Pattern matching
        \\    match x {
        \\        0 => vibez.spill("zero"),
        \\        _ => vibez.spill("not zero")
        \\    }
        \\    
        \\    vibez.spill("Demo complete!")
        \\}
    ;
    
    var test_lexer = Lexer.init(allocator, cursed_code);
    const tokens = try test_lexer.tokenize();
    defer tokens.deinit();
    
    std.debug.print("\n🔍 Testing CURSED lexer with {} tokens\n", .{tokens.items.len});
    
    // Print all tokens for debugging
    for (tokens.items, 0..) |token, i| {
        std.debug.print("[{}] {} '{s}' ({}:{})\n", .{ i, token.kind, token.lexeme, token.line, token.column });
    }
    
    // Test key CURSED keywords are recognized
    var found_vibe = false;
    var found_slay = false;
    var found_sus = false;
    var found_yolo = false;
    var found_lowkey = false;
    var found_highkey = false;
    var found_yeet = false;
    var found_based = false;
    var found_cringe = false;
    var found_vibez = false;
    var found_match = false;
    var found_yikes = false;
    
    for (tokens.items) |token| {
        if (std.mem.eql(u8, token.lexeme, "vibe")) found_vibe = true;
        if (std.mem.eql(u8, token.lexeme, "slay")) found_slay = true;
        if (std.mem.eql(u8, token.lexeme, "sus")) found_sus = true;
        if (std.mem.eql(u8, token.lexeme, "yolo")) found_yolo = true;
        if (std.mem.eql(u8, token.lexeme, "lowkey")) found_lowkey = true;
        if (std.mem.eql(u8, token.lexeme, "highkey")) found_highkey = true;
        if (std.mem.eql(u8, token.lexeme, "yeet")) found_yeet = true;
        if (std.mem.eql(u8, token.lexeme, "based")) found_based = true;
        if (std.mem.eql(u8, token.lexeme, "cringe")) found_cringe = true;
        if (std.mem.eql(u8, token.lexeme, "vibez")) found_vibez = true;
        if (std.mem.eql(u8, token.lexeme, "match")) found_match = true;
        if (std.mem.eql(u8, token.lexeme, "yikes")) found_yikes = true;
    }
    
    // Verify all expected keywords were found
    try std.testing.expect(found_vibe);
    try std.testing.expect(found_slay);
    try std.testing.expect(found_sus);
    try std.testing.expect(found_yolo);
    try std.testing.expect(found_lowkey);
    try std.testing.expect(found_highkey);
    try std.testing.expect(found_yeet);
    try std.testing.expect(found_based);
    try std.testing.expect(found_vibez);
    try std.testing.expect(found_match);
    try std.testing.expect(found_yikes);
    
    // Test that "cringe" maps to the right token type
    for (tokens.items) |token| {
        if (std.mem.eql(u8, token.lexeme, "based")) {
            try std.testing.expect(token.kind == .Based);
        }
        if (std.mem.eql(u8, token.lexeme, "cringe")) {
            // Should be Cap (false/nil)
            try std.testing.expect(token.kind == .Cap);
        }
    }
    
    std.debug.print("✅ All CURSED keywords found and correctly tokenized!\n", .{});
}

test "CURSED comment handling" {
    const allocator = std.testing.allocator;
    
    const comment_code = 
        \\fr fr This is a line comment
        \\slay hello() { }
        \\no cap
        \\This is a block comment
        \\on god
        \\yolo 42
    ;
    
    var test_lexer = Lexer.init(allocator, comment_code);
    const tokens = try test_lexer.tokenize();
    defer tokens.deinit();
    
    std.debug.print("\n🔍 Testing CURSED comment handling with {} tokens\n", .{tokens.items.len});
    
    // Print all tokens
    for (tokens.items, 0..) |token, i| {
        std.debug.print("[{}] {} '{}'\n", .{ i, token.kind, token.lexeme });
    }
    
    // Should find slay, hello, yolo, 42 but not comment content
    var found_slay = false;
    var found_hello = false;
    var found_yolo = false;
    var found_42 = false;
    
    for (tokens.items) |token| {
        if (std.mem.eql(u8, token.lexeme, "slay")) found_slay = true;
        if (std.mem.eql(u8, token.lexeme, "hello")) found_hello = true;
        if (std.mem.eql(u8, token.lexeme, "yolo")) found_yolo = true;
        if (std.mem.eql(u8, token.lexeme, "42")) found_42 = true;
        
        // Should NOT find comment content as regular tokens
        try std.testing.expect(!std.mem.eql(u8, token.lexeme, "This"));
        try std.testing.expect(!std.mem.eql(u8, token.lexeme, "comment"));
        try std.testing.expect(!std.mem.eql(u8, token.lexeme, "block"));
    }
    
    try std.testing.expect(found_slay);
    try std.testing.expect(found_hello);
    try std.testing.expect(found_yolo);
    try std.testing.expect(found_42);
    
    std.debug.print("✅ Comment handling test passed!\n", .{});
}

test "string and number literals" {
    const allocator = std.testing.allocator;
    
    const literals_code = 
        \\sus name tea = "Hello, CURSED!"
        \\sus count normie = 42
        \\sus pi snack = 3.14159
        \\sus flag lit = based
    ;
    
    var test_lexer = Lexer.init(allocator, literals_code);
    const tokens = try test_lexer.tokenize();
    defer tokens.deinit();
    
    std.debug.print("\n🔍 Testing literals with {} tokens\n", .{tokens.items.len});
    
    var found_string = false;
    var found_integer = false;
    var found_float = false;
    var found_boolean = false;
    
    for (tokens.items) |token| {
        if (token.kind == .StringLiteral) {
            found_string = true;
            std.debug.print("String: '{}'\n", .{token.lexeme});
        }
        if (token.kind == .Number and std.mem.eql(u8, token.lexeme, "42")) {
            found_integer = true;
            std.debug.print("Integer: '{}'\n", .{token.lexeme});
        }
        if (token.kind == .Number and std.mem.eql(u8, token.lexeme, "3.14159")) {
            found_float = true;
            std.debug.print("Float: '{}'\n", .{token.lexeme});
        }
        if (token.kind == .Based) {
            found_boolean = true;
            std.debug.print("Boolean: '{}'\n", .{token.lexeme});
        }
    }
    
    try std.testing.expect(found_string);
    try std.testing.expect(found_integer);
    try std.testing.expect(found_float);
    try std.testing.expect(found_boolean);
    
    std.debug.print("✅ Literal parsing test passed!\n", .{});
}
