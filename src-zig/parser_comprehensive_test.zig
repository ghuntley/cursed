const std = @import("std");
const testing = std.testing;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast.zig");

const Token = lexer.Token;
const TokenKind = lexer.TokenKind;
const Parser = parser.Parser;
const Program = ast.Program;
const Statement = ast.Statement;
const Expression = ast.Expression;

// Test 1: Simple variable declarations (sus x drip = 42)
test "parse simple variable declaration" {
    const allocator = testing.allocator;
    
    // Create tokens for "sus x drip = 42"
    const tokens = [_]Token{
        Token.init(.Sus, "sus", 1, 1),
        Token.init(.Identifier, "x", 1, 5),
        Token.init(.Drip, "drip", 1, 7),
        Token.init(.Equal, "=", 1, 12),
        Token.init(.Number, "42", 1, 14),
        Token.init(.Eof, "", 1, 16),
    };
    
    var test_parser = Parser.init(allocator, &tokens);
    const program = test_parser.parseProgram() catch |err| {
        std.debug.print("Parse error: {}\n", .{err});
        return err;
    };
    defer {
        var mut_program = program;
        mut_program.deinit(allocator);
    }
    
    try testing.expect(program.statements.items.len == 1);
    
    const stmt_ptr: *Statement = @ptrCast(@alignCast(program.statements.items[0]));
    switch (stmt_ptr.*) {
        .Let => |let_stmt| {
            try testing.expect(std.mem.eql(u8, let_stmt.name, "x"));
            try testing.expect(let_stmt.is_mutable == true);
            try testing.expect(let_stmt.initializer != null);
        },
        else => {
            std.debug.print("Expected Let statement, got: {}\n", .{stmt_ptr.*});
            try testing.expect(false);
        },
    }
}

// Test 2: Function definitions (slay test() {})
test "parse simple function definition" {
    const allocator = testing.allocator;
    
    // Create tokens for "slay test() { }"
    const tokens = [_]Token{
        Token.init(.Slay, "slay", 1, 1),
        Token.init(.Identifier, "test", 1, 6),
        Token.init(.LeftParen, "(", 1, 10),
        Token.init(.RightParen, ")", 1, 11),
        Token.init(.LeftBrace, "{", 1, 13),
        Token.init(.RightBrace, "}", 1, 15),
        Token.init(.Eof, "", 1, 16),
    };
    
    var test_parser = Parser.init(allocator, &tokens);
    const program = test_parser.parseProgram() catch |err| {
        std.debug.print("Parse error: {}\n", .{err});
        return err;
    };
    defer {
        var mut_program = program;
        mut_program.deinit(allocator);
    }
    
    try testing.expect(program.statements.items.len == 1);
    
    const stmt_ptr: *Statement = @ptrCast(@alignCast(program.statements.items[0]));
    switch (stmt_ptr.*) {
        .Function => |func| {
            try testing.expect(std.mem.eql(u8, func.name, "test"));
            try testing.expect(func.parameters.items.len == 0);
            try testing.expect(func.body.items.len == 0);
        },
        else => {
            std.debug.print("Expected Function statement, got: {}\n", .{stmt_ptr.*});
            try testing.expect(false);
        },
    }
}

// Test 3: Function with parameters and return type
test "parse function with parameters" {
    const allocator = testing.allocator;
    
    // Create tokens for "slay add(a drip, b drip) drip { }"
    const tokens = [_]Token{
        Token.init(.Slay, "slay", 1, 1),
        Token.init(.Identifier, "add", 1, 6),
        Token.init(.LeftParen, "(", 1, 9),
        Token.init(.Identifier, "a", 1, 10),
        Token.init(.Drip, "drip", 1, 12),
        Token.init(.Comma, ",", 1, 16),
        Token.init(.Identifier, "b", 1, 18),
        Token.init(.Drip, "drip", 1, 20),
        Token.init(.RightParen, ")", 1, 24),
        Token.init(.Drip, "drip", 1, 26),
        Token.init(.LeftBrace, "{", 1, 31),
        Token.init(.RightBrace, "}", 1, 33),
        Token.init(.Eof, "", 1, 34),
    };
    
    var test_parser = Parser.init(allocator, &tokens);
    const program = test_parser.parseProgram() catch |err| {
        std.debug.print("Parse error: {}\n", .{err});
        return err;
    };
    defer {
        var mut_program = program;
        mut_program.deinit(allocator);
    }
    
    try testing.expect(program.statements.items.len == 1);
    
    const stmt_ptr: *Statement = @ptrCast(@alignCast(program.statements.items[0]));
    switch (stmt_ptr.*) {
        .Function => |func| {
            try testing.expect(std.mem.eql(u8, func.name, "add"));
            try testing.expect(func.parameters.items.len == 2);
            try testing.expect(std.mem.eql(u8, func.parameters.items[0].name, "a"));
            try testing.expect(std.mem.eql(u8, func.parameters.items[1].name, "b"));
            try testing.expect(func.return_type != null);
        },
        else => {
            std.debug.print("Expected Function statement, got: {}\n", .{stmt_ptr.*});
            try testing.expect(false);
        },
    }
}

// Test 4: Function calls (vibez.spill("hello"))
test "parse function call expression" {
    const allocator = testing.allocator;
    
    // Create tokens for "vibez.spill("hello")"
    const tokens = [_]Token{
        Token.init(.Identifier, "vibez", 1, 1),
        Token.init(.Dot, ".", 1, 6),
        Token.init(.Identifier, "spill", 1, 7),
        Token.init(.LeftParen, "(", 1, 12),
        Token.init(.String, "\"hello\"", 1, 13),
        Token.init(.RightParen, ")", 1, 20),
        Token.init(.Eof, "", 1, 21),
    };
    
    var test_parser = Parser.init(allocator, &tokens);
    const program = test_parser.parseProgram() catch |err| {
        std.debug.print("Parse error: {}\n", .{err});
        return err;
    };
    defer {
        var mut_program = program;
        mut_program.deinit(allocator);
    }
    
    try testing.expect(program.statements.items.len == 1);
    
    const stmt_ptr: *Statement = @ptrCast(@alignCast(program.statements.items[0]));
    switch (stmt_ptr.*) {
        .Expression => |expr| {
            switch (expr) {
                .Call => |call| {
                    try testing.expect(call.arguments.items.len == 1);
                    // Verify it's a member access call (vibez.spill)
                    const func_expr: *Expression = @ptrCast(@alignCast(call.function));
                    switch (func_expr.*) {
                        .MemberAccess => |member| {
                            try testing.expect(std.mem.eql(u8, member.property, "spill"));
                        },
                        else => {
                            std.debug.print("Expected MemberAccess, got: {}\n", .{func_expr.*});
                            try testing.expect(false);
                        },
                    }
                },
                else => {
                    std.debug.print("Expected Call expression, got: {}\n", .{expr});
                    try testing.expect(false);
                },
            }
        },
        else => {
            std.debug.print("Expected Expression statement, got: {}\n", .{stmt_ptr.*});
            try testing.expect(false);
        },
    }
}

// Test 5: Basic control flow (lowkey/highkey)
test "parse if statement" {
    const allocator = testing.allocator;
    
    // Create tokens for "lowkey x > 5 { vibez.spill("true") } highkey { vibez.spill("false") }"
    const tokens = [_]Token{
        Token.init(.Lowkey, "lowkey", 1, 1),
        Token.init(.Identifier, "x", 1, 8),
        Token.init(.Greater, ">", 1, 10),
        Token.init(.Number, "5", 1, 12),
        Token.init(.LeftBrace, "{", 1, 14),
        Token.init(.Identifier, "vibez", 1, 16),
        Token.init(.Dot, ".", 1, 21),
        Token.init(.Identifier, "spill", 1, 22),
        Token.init(.LeftParen, "(", 1, 27),
        Token.init(.String, "\"true\"", 1, 28),
        Token.init(.RightParen, ")", 1, 34),
        Token.init(.RightBrace, "}", 1, 36),
        Token.init(.Highkey, "highkey", 1, 38),
        Token.init(.LeftBrace, "{", 1, 46),
        Token.init(.Identifier, "vibez", 1, 48),
        Token.init(.Dot, ".", 1, 53),
        Token.init(.Identifier, "spill", 1, 54),
        Token.init(.LeftParen, "(", 1, 59),
        Token.init(.String, "\"false\"", 1, 60),
        Token.init(.RightParen, ")", 1, 67),
        Token.init(.RightBrace, "}", 1, 69),
        Token.init(.Eof, "", 1, 70),
    };
    
    var test_parser = Parser.init(allocator, &tokens);
    const program = test_parser.parseProgram() catch |err| {
        std.debug.print("Parse error: {}\n", .{err});
        return err;
    };
    defer {
        var mut_program = program;
        mut_program.deinit(allocator);
    }
    
    try testing.expect(program.statements.items.len == 1);
    
    const stmt_ptr: *Statement = @ptrCast(@alignCast(program.statements.items[0]));
    switch (stmt_ptr.*) {
        .If => |if_stmt| {
            try testing.expect(if_stmt.condition != null);
            try testing.expect(if_stmt.then_branch.items.len == 1);
            try testing.expect(if_stmt.else_branch != null);
        },
        else => {
            std.debug.print("Expected If statement, got: {}\n", .{stmt_ptr.*});
            try testing.expect(false);
        },
    }
}

// Test 6: Binary expressions
test "parse binary expression" {
    const allocator = testing.allocator;
    
    // Create tokens for "42 + 24"
    const tokens = [_]Token{
        Token.init(.Number, "42", 1, 1),
        Token.init(.Plus, "+", 1, 4),
        Token.init(.Number, "24", 1, 6),
        Token.init(.Eof, "", 1, 8),
    };
    
    var test_parser = Parser.init(allocator, &tokens);
    const program = test_parser.parseProgram() catch |err| {
        std.debug.print("Parse error: {}\n", .{err});
        return err;
    };
    defer {
        var mut_program = program;
        mut_program.deinit(allocator);
    }
    
    try testing.expect(program.statements.items.len == 1);
    
    const stmt_ptr: *Statement = @ptrCast(@alignCast(program.statements.items[0]));
    switch (stmt_ptr.*) {
        .Expression => |expr| {
            switch (expr) {
                .Binary => |bin| {
                    try testing.expect(std.mem.eql(u8, bin.operator, "+"));
                    
                    // Check left operand
                    const left_expr: *Expression = @ptrCast(@alignCast(bin.left));
                    switch (left_expr.*) {
                        .Integer => |val| try testing.expect(val == 42),
                        else => try testing.expect(false),
                    }
                    
                    // Check right operand  
                    const right_expr: *Expression = @ptrCast(@alignCast(bin.right));
                    switch (right_expr.*) {
                        .Integer => |val| try testing.expect(val == 24),
                        else => try testing.expect(false),
                    }
                },
                else => {
                    std.debug.print("Expected Binary expression, got: {}\n", .{expr});
                    try testing.expect(false);
                },
            }
        },
        else => {
            std.debug.print("Expected Expression statement, got: {}\n", .{stmt_ptr.*});
            try testing.expect(false);
        },
    }
}

// Test 7: String literals and identifiers
test "parse string and identifier literals" {
    const allocator = testing.allocator;
    
    // Test string literal
    const string_tokens = [_]Token{
        Token.init(.String, "\"hello world\"", 1, 1),
        Token.init(.Eof, "", 1, 14),
    };
    
    var string_parser = Parser.init(allocator, &string_tokens);
    const string_program = string_parser.parseProgram() catch |err| {
        std.debug.print("Parse error: {}\n", .{err});
        return err;
    };
    defer {
        var mut_program = string_program;
        mut_program.deinit(allocator);
    }
    
    try testing.expect(string_program.statements.items.len == 1);
    
    const stmt_ptr: *Statement = @ptrCast(@alignCast(string_program.statements.items[0]));
    switch (stmt_ptr.*) {
        .Expression => |expr| {
            switch (expr) {
                .String => |str| {
                    try testing.expect(std.mem.eql(u8, str, "\"hello world\""));
                },
                else => {
                    std.debug.print("Expected String expression, got: {}\n", .{expr});
                    try testing.expect(false);
                },
            }
        },
        else => {
            std.debug.print("Expected Expression statement, got: {}\n", .{stmt_ptr.*});
            try testing.expect(false);
        },
    }
}

// Test 8: Complex nested expressions
test "parse complex nested expression" {
    const allocator = testing.allocator;
    
    // Create tokens for "(42 + 24) * 2"
    const tokens = [_]Token{
        Token.init(.LeftParen, "(", 1, 1),
        Token.init(.Number, "42", 1, 2),
        Token.init(.Plus, "+", 1, 5),
        Token.init(.Number, "24", 1, 7),
        Token.init(.RightParen, ")", 1, 9),
        Token.init(.Star, "*", 1, 11),
        Token.init(.Number, "2", 1, 13),
        Token.init(.Eof, "", 1, 14),
    };
    
    var test_parser = Parser.init(allocator, &tokens);
    const program = test_parser.parseProgram() catch |err| {
        std.debug.print("Parse error: {}\n", .{err});
        return err;
    };
    defer {
        var mut_program = program;
        mut_program.deinit(allocator);
    }
    
    try testing.expect(program.statements.items.len == 1);
    
    const stmt_ptr: *Statement = @ptrCast(@alignCast(program.statements.items[0]));
    switch (stmt_ptr.*) {
        .Expression => |expr| {
            switch (expr) {
                .Binary => |bin| {
                    try testing.expect(std.mem.eql(u8, bin.operator, "*"));
                    
                    // Left side should be a binary expression (42 + 24)
                    const left_expr: *Expression = @ptrCast(@alignCast(bin.left));
                    switch (left_expr.*) {
                        .Binary => |left_bin| {
                            try testing.expect(std.mem.eql(u8, left_bin.operator, "+"));
                        },
                        else => {
                            std.debug.print("Expected nested Binary expression, got: {}\n", .{left_expr.*});
                            try testing.expect(false);
                        },
                    }
                },
                else => {
                    std.debug.print("Expected Binary expression, got: {}\n", .{expr});
                    try testing.expect(false);
                },
            }
        },
        else => {
            std.debug.print("Expected Expression statement, got: {}\n", .{stmt_ptr.*});
            try testing.expect(false);
        },
    }
}

// Test runner function
pub fn runAllTests(allocator: Allocator) !void {
    const tests = [_]fn () anyerror!void{
        test_simple_variable_declaration,
        test_simple_function_definition,
        test_function_with_parameters,
        test_function_call_expression,
        test_if_statement,
        test_binary_expression,
        test_string_and_identifier_literals,
        test_complex_nested_expression,
    };
    
    std.debug.print("Running {} parser tests...\n", .{tests.len});
    
    var passed: usize = 0;
    var failed: usize = 0;
    
    for (tests, 0..) |test_func, i| {
        std.debug.print("Test {}: ", .{i + 1});
        test_func() catch |err| {
            std.debug.print("FAILED - {}\n", .{err});
            failed += 1;
            continue;
        };
        std.debug.print("PASSED\n");
        passed += 1;
    }
    
    std.debug.print("\n=== Test Results ===\n");
    std.debug.print("Passed: {}\n", .{passed});
    std.debug.print("Failed: {}\n", .{failed});
    std.debug.print("Total:  {}\n", .{tests.len});
    
    if (failed > 0) {
        return error.TestsFailed;
    }
}

// Individual test functions wrapper
fn test_simple_variable_declaration() !void {
    const allocator = testing.allocator;
    
    // Create tokens for "sus x drip = 42"
    const tokens = [_]Token{
        Token.init(.Sus, "sus", 1, 1),
        Token.init(.Identifier, "x", 1, 5),
        Token.init(.Drip, "drip", 1, 7),
        Token.init(.Equal, "=", 1, 12),
        Token.init(.Number, "42", 1, 14),
        Token.init(.Eof, "", 1, 16),
    };
    
    var test_parser = Parser.init(allocator, &tokens);
    const program = test_parser.parseProgram() catch |err| {
        std.debug.print("Parse error: {}\n", .{err});
        return err;
    };
    defer {
        var mut_program = program;
        mut_program.deinit(allocator);
    }
    
    try testing.expect(program.statements.items.len == 1);
}
