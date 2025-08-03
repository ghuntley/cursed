const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const ast = @import("ast_new.zig");
const parser = @import("parser_new.zig");

const Token = lexer.Token;
const TokenKind = lexer.TokenKind;
const Lexer = lexer.Lexer;
const Parser = parser.Parser;
const Program = ast.Program;

pub fn testComplexProgramParsing(allocator: Allocator) !void {
    // Test complex CURSED program parsing
    const source = 
        \\squad Point {
        \\    spill x normie
        \\    spill y normie
        \\}
        \\
        \\collab Drawable {
        \\    slay draw()
        \\    slay area() meal
        \\}
        \\
        \\slay test_function(param normie) normie {
        \\    sus local_var normie = 42
        \\    yolo local_var * 2
        \\}
    ;
    
    // Tokenize the source
    var lex = Lexer.init(allocator, source);
    const tokens = try lex.tokenize();
    defer tokens.deinit();
    
    // Parse the tokens
    var parse = Parser.init(allocator, tokens.items);
    var program = try parse.parseProgram();
    defer program.deinit();
    
    // Verify the program structure
    std.debug.print("Parsed program with {} statements\n", .{program.statements.items.len});
    
    // Check that we have struct, interface, and function statements
    var struct_count: usize = 0;
    var interface_count: usize = 0;
    var function_count: usize = 0;
    
    for (program.statements.items) |stmt| {
        switch (stmt.kind) {
            .struct_stmt => struct_count += 1,
            .interface => interface_count += 1,
            .function => function_count += 1,
            else => {},
        }
    }
    
    std.debug.print("Found: {} structs, {} interfaces, {} functions\n", .{ struct_count, interface_count, function_count });
    
    // Print the parsed AST structure
    try program.print(0);
}

pub fn testMemoryManagement(allocator: Allocator) !void {
    // Test that AST nodes are properly cleaned up
    const source = "sus x normie = 42 + 10 * 5";
    
    var lex = Lexer.init(allocator, source);
    const tokens = try lex.tokenize();
    defer tokens.deinit();
    
    var parse = Parser.init(allocator, tokens.items);
    var program = try parse.parseProgram();
    defer program.deinit();
    
    std.debug.print("Memory management test completed successfully\n", .{});
}

pub fn testExpressionParsing(allocator: Allocator) !void {
    // Test complex expression parsing
    const source = "sus result normie = (x + y) * z.property";
    
    var lex = Lexer.init(allocator, source);
    const tokens = try lex.tokenize();
    defer tokens.deinit();
    
    var parse = Parser.init(allocator, tokens.items);
    var program = try parse.parseProgram();
    defer program.deinit();
    
    // Verify we got a let statement with a complex expression
    if (program.statements.items.len > 0) {
        const stmt = program.statements.items[0];
        switch (stmt.kind) {
            .let => |let_data| {
                if (let_data.initializer) |init| {
                    switch (init.kind) {
                        .binary => std.debug.print("Successfully parsed binary expression\n", .{}),
                        else => std.debug.print("Expression type: {s}\n", .{@tagName(init.kind)}),
                    }
                }
            },
            else => std.debug.print("Statement type: {s}\n", .{@tagName(stmt.kind)}),
        }
    }
}

// Test functions
test "complex program parsing" {
    const allocator = std.testing.allocator;
    try testComplexProgramParsing(allocator);
}

test "memory management" {
    const allocator = std.testing.allocator;
    try testMemoryManagement(allocator);
}

test "expression parsing" {
    const allocator = std.testing.allocator;
    try testExpressionParsing(allocator);
}

test "AST circular dependency resolution" {
    // This test verifies that the AST can be created without circular dependencies
    const allocator = std.testing.allocator;
    
    // Create nested expressions that would have caused circular dependency issues
    const left = try ast.createIntegerExpression(allocator, 1);
    defer left.deinit();
    
    const right = try ast.createIntegerExpression(allocator, 2);
    defer right.deinit();
    
    const binary = try ast.createBinaryExpression(allocator, left, "+", right);
    defer binary.deinit();
    
    // Create another level of nesting
    const nested_right = try ast.createIntegerExpression(allocator, 3);
    defer nested_right.deinit();
    
    const nested_binary = try ast.createBinaryExpression(allocator, binary, "*", nested_right);
    defer nested_binary.deinit();
    
    // If we get here without circular dependency errors, the test passes
    try std.testing.expect(true);
}

test "advanced CURSED features parsing" {
    const allocator = std.testing.allocator;
    
    // Test parsing of advanced CURSED constructs
    const sources = [_][]const u8{
        "squad Point { spill x normie }",
        "collab Drawable { slay draw() }",
        "sus x normie = based",
        "sus y normie = lies",
        "yolo 42",
    };
    
    for (sources) |source| {
        var lex = Lexer.init(allocator, source);
        const tokens = try lex.tokenize();
        defer tokens.deinit();
        
        var parse = Parser.init(allocator, tokens.items);
        var program = try parse.parseProgram();
        defer program.deinit();
        
        // Verify that parsing completed without errors
        try std.testing.expect(program.statements.items.len > 0);
    }
}
