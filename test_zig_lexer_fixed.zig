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
    
    // Print first 20 tokens for debugging
    for (tokens.items[0..@min(20, tokens.items.len)], 0..) |token, i| {
        std.debug.print("[{}] {} '{s}' ({}:{})\n", .{ i, token.kind, token.lexeme, token.line, token.column });
    }
    
    if (tokens.items.len > 20) {
        std.debug.print("... and {} more tokens\n", .{tokens.items.len - 20});
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
    
    // Report missing keywords
    if (!found_vibe) std.debug.print("❌ Missing: vibe\n", .{});
    if (!found_slay) std.debug.print("❌ Missing: slay\n", .{});
    if (!found_sus) std.debug.print("❌ Missing: sus\n", .{});
    if (!found_yolo) std.debug.print("❌ Missing: yolo\n", .{});
    if (!found_lowkey) std.debug.print("❌ Missing: lowkey\n", .{});
    if (!found_highkey) std.debug.print("❌ Missing: highkey\n", .{});
    if (!found_yeet) std.debug.print("❌ Missing: yeet\n", .{});
    if (!found_based) std.debug.print("❌ Missing: based\n", .{});
    if (!found_vibez) std.debug.print("❌ Missing: vibez\n", .{});
    if (!found_match) std.debug.print("❌ Missing: match\n", .{});
    if (!found_yikes) std.debug.print("❌ Missing: yikes\n", .{});
    
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
    
    std.debug.print("✅ CURSED lexer test completed!\n", .{});
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
            std.debug.print("String: '{s}'\n", .{token.lexeme});
        }
        if (token.kind == .Number and std.mem.eql(u8, token.lexeme, "42")) {
            found_integer = true;
            std.debug.print("Integer: '{s}'\n", .{token.lexeme});
        }
        if (token.kind == .Number and std.mem.eql(u8, token.lexeme, "3.14159")) {
            found_float = true;
            std.debug.print("Float: '{s}'\n", .{token.lexeme});
        }
        if (token.kind == .Based) {
            found_boolean = true;
            std.debug.print("Boolean: '{s}'\n", .{token.lexeme});
        }
    }
    
    try std.testing.expect(found_string);
    try std.testing.expect(found_integer);
    try std.testing.expect(found_float);
    try std.testing.expect(found_boolean);
    
    std.debug.print("✅ Literal parsing test passed!\n", .{});
}
