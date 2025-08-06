const std = @import("std");
const Lexer = @import("src-zig/lexer.zig").Lexer;
const TokenKind = @import("src-zig/lexer.zig").TokenKind;

fn run_lexer_test(allocator: std.mem.Allocator, name: []const u8, input: []const u8) !void {
    std.debug.print("\n--- Testing: {s} ---\n", .{name});
    std.debug.print("Input: '{s}'\n", .{input});
    
    var lexer = Lexer.init(allocator, input);
    const tokens = try lexer.tokenize();
    defer tokens.deinit();
    
    std.debug.print("Tokens ({}): ", .{tokens.items.len});
    for (tokens.items, 0..) |token, i| {
        if (i > 0) std.debug.print(", ", .{});
        std.debug.print("{s}('{s}')", .{ @tagName(token.kind), token.lexeme });
    }
    std.debug.print("\n", .{});
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    std.debug.print("=== CURSED Lexer Test Suite ===\n", .{});
    
    // Test 1: Basic CURSED keywords
    try run_lexer_test(allocator, "Basic Keywords", "sus slay vibez yeet damn spill");
    
    // Test 2: Boolean literals
    try run_lexer_test(allocator, "Boolean Literals", "based cringe nah");
    
    // Test 3: Type keywords
    try run_lexer_test(allocator, "Type Keywords", "normie tea lit smol thicc meal snack");
    
    // Test 4: Numbers
    try run_lexer_test(allocator, "Numbers", "42 3.14 0 123.456");
    
    // Test 5: String literals
    try run_lexer_test(allocator, "String Literals", "\"hello world\" \"escaped\\\"quote\"");
    
    // Test 6: Character literals
    try run_lexer_test(allocator, "Character Literals", "'a' '\\n' '\\t'");
    
    // Test 7: Operators
    try run_lexer_test(allocator, "Arithmetic Operators", "+ - * / %");
    
    // Test 8: Comparison operators
    try run_lexer_test(allocator, "Comparison Operators", "== != < > <= >=");
    
    // Test 9: Logical operators
    try run_lexer_test(allocator, "Logical Operators", "&& || !");
    
    // Test 10: Assignment operators
    try run_lexer_test(allocator, "Assignment Operators", "= := += -= *= /=");
    
    // Test 11: Punctuation
    try run_lexer_test(allocator, "Punctuation", "() {} [] , ; : .. ... ? @");
    
    // Test 12: Comments
    try run_lexer_test(allocator, "Comments", "// comment");
    try run_lexer_test(allocator, "Hash Comments", "# comment");
    
    // Test 13: Identifiers
    try run_lexer_test(allocator, "Identifiers", "variable_name camelCase _underscore");
    
    // Test 14: Complete function
    try run_lexer_test(allocator, "Complete Function", 
        \\slay main_character() {
        \\  sus count normie = 42
        \\  vibez.spill("Hello!")
        \\}
    );
    
    // Test 15: Control flow
    try run_lexer_test(allocator, "Control Flow", "bestie lowkey highkey periodt ghosted simp");
    
    // Test 16: Advanced features
    try run_lexer_test(allocator, "Advanced Features", "squad collab impl yikes shook fam");
    
    // Test 17: Channel operations
    try run_lexer_test(allocator, "Channel Operations", "dm select ready <- ->");
    
    std.debug.print("\n=== Running specific error tests ===\n", .{});
    
    // Error test 1: Unterminated string
    {
        std.debug.print("\nTesting unterminated string...\n", .{});
        var lexer = Lexer.init(allocator, "\"unterminated");
        const result = lexer.nextToken();
        if (result) |_| {
            std.debug.print("ERROR: Expected error for unterminated string\n", .{});
        } else |err| {
            std.debug.print("✓ Correctly caught error: {}\n", .{err});
        }
    }
    
    // Error test 2: Unterminated character
    {
        std.debug.print("\nTesting unterminated character...\n", .{});
        var lexer = Lexer.init(allocator, "'unterminated");
        const result = lexer.nextToken();
        if (result) |_| {
            std.debug.print("ERROR: Expected error for unterminated character\n", .{});
        } else |err| {
            std.debug.print("✓ Correctly caught error: {}\n", .{err});
        }
    }
    
    std.debug.print("\n=== Testing position tracking ===\n", .{});
    {
        const multiline_input = 
            \\sus x normie = 42
            \\tea message = "hello"
            \\slay test() { }
        ;
        
        var lexer = Lexer.init(allocator, multiline_input);
        const tokens = try lexer.tokenize();
        defer tokens.deinit();
        
        std.debug.print("Position tracking test:\n", .{});
        for (tokens.items[0..@min(10, tokens.items.len)]) |token| {
            std.debug.print("  {s} = '{s}' [line {}, col {}]\n", .{ 
                @tagName(token.kind), token.lexeme, token.line, token.column 
            });
        }
    }
    
    std.debug.print("\n=== All tests completed ===\n", .{});
}
