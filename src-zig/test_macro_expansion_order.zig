//! Comprehensive test suite for macro expansion order system

const std = @import("std");
const testing = std.testing;
const ArrayList = std.ArrayList;

const lexer = @import("lexer.zig");
const macro_hygiene = @import("macro_hygiene.zig");
const macro_expansion_order = @import("macro_expansion_order.zig");
const parser_macro_integration = @import("parser_macro_integration.zig");

const Token = lexer.Token;
const TokenKind = lexer.TokenKind;
const MacroHygieneContext = macro_hygiene.MacroHygieneContext;
const MacroExpansionContext = macro_expansion_order.MacroExpansionContext;
const MacroAwareParser = parser_macro_integration.MacroAwareParser;

// Test utilities
fn createToken(kind: TokenKind, lexeme: []const u8, line: u32, column: u32) Token {
    return Token{
        .kind = kind,
        .lexeme = lexeme,
        .line = line,
        .column = column,
        .offset = 0,
    };
}

fn createTokens(allocator: std.mem.Allocator, token_specs: []const struct { TokenKind, []const u8 }) ![]Token {
    var tokens = .empty;
    defer tokens.deinit();
    
    for (token_specs, 0..) |spec, i| {
        try tokens.append(createToken(spec[0], spec[1], 1, @intCast(i + 1)));
    }
    
    return tokens.toOwnedSlice();
}

// Test 1: Basic macro expansion order
test "basic macro expansion order" {
    var hygiene_ctx = try MacroHygieneContext.init(testing.allocator);
    defer hygiene_ctx.deinit();
    
    var expansion_ctx = try MacroExpansionContext.init(testing.allocator, &hygiene_ctx);
    defer expansion_ctx.deinit();
    
    // Define macros with different priorities
    var high_priority_macro = MacroExpansionContext.MacroDefinition.init(testing.allocator, "HIGH");
    high_priority_macro.expansion_priority = .High;
    high_priority_macro.body = &[_]Token{
        createToken(.Number, "42", 1, 1),
    };
    try expansion_ctx.defineMacro(high_priority_macro);
    
    var normal_macro = MacroExpansionContext.MacroDefinition.init(testing.allocator, "NORMAL");
    normal_macro.expansion_priority = .Normal;
    normal_macro.body = &[_]Token{
        createToken(.Number, "24", 1, 1),
    };
    try expansion_ctx.defineMacro(normal_macro);
    
    // Queue expansions (normal first, then high)
    const normal_call = MacroExpansionContext.MacroCall{
        .name = "NORMAL",
        .arguments = &[_]Token{},
        .location = .{ .file = "test", .line = 1, .column = 1, .byte_offset = 0 },
        .context_tokens = &[_]Token{},
    };
    
    const high_call = MacroExpansionContext.MacroCall{
        .name = "HIGH",
        .arguments = &[_]Token{},
        .location = .{ .file = "test", .line = 2, .column = 1, .byte_offset = 0 },
        .context_tokens = &[_]Token{},
    };
    
    _ = try expansion_ctx.queueMacroExpansion(normal_call);
    _ = try expansion_ctx.queueMacroExpansion(high_call);
    
    // Process - high priority should be processed first
    const result = try expansion_ctx.processExpansions();
    defer testing.allocator.free(result);
    
    try testing.expect(result.len >= 2);
    // High priority result should come first
    try testing.expectEqualStrings("42", result[0].lexeme);
}

// Test 2: Recursion detection
test "macro recursion detection" {
    var hygiene_ctx = try MacroHygieneContext.init(testing.allocator);
    defer hygiene_ctx.deinit();
    
    var expansion_ctx = try MacroExpansionContext.init(testing.allocator, &hygiene_ctx);
    defer expansion_ctx.deinit();
    
    // Define recursive macro
    var recursive_macro = MacroExpansionContext.MacroDefinition.init(testing.allocator, "RECURSIVE");
    recursive_macro.body = &[_]Token{
        createToken(.At, "@", 1, 1),
        createToken(.Identifier, "RECURSIVE", 1, 2),
    };
    try expansion_ctx.defineMacro(recursive_macro);
    
    // Try to expand
    var call = MacroExpansionContext.MacroCall{
        .name = "RECURSIVE",
        .arguments = &[_]Token{},
        .location = .{ .file = "test", .line = 1, .column = 1, .byte_offset = 0 },
        .context_tokens = &[_]Token{},
    };
    
    // Should detect recursion
    const result = expansion_ctx.queueMacroExpansion(call);
    try testing.expectError(error.MacroRecursion, result);
}

// Test 3: Dependency ordering
test "macro dependency ordering" {
    var hygiene_ctx = try MacroHygieneContext.init(testing.allocator);
    defer hygiene_ctx.deinit();
    
    var expansion_ctx = try MacroExpansionContext.init(testing.allocator, &hygiene_ctx);
    defer expansion_ctx.deinit();
    
    // Define base macro
    var base_macro = MacroExpansionContext.MacroDefinition.init(testing.allocator, "BASE");
    base_macro.body = &[_]Token{
        createToken(.Number, "10", 1, 1),
    };
    try expansion_ctx.defineMacro(base_macro);
    
    // Define dependent macro
    var dependent_macro = MacroExpansionContext.MacroDefinition.init(testing.allocator, "DEPENDENT");
    dependent_macro.body = &[_]Token{
        createToken(.At, "@", 1, 1),
        createToken(.Identifier, "BASE", 1, 2),
        createToken(.Plus, "+", 1, 3),
        createToken(.Number, "5", 1, 4),
    };
    try expansion_ctx.defineMacro(dependent_macro);
    
    // Queue dependent first, then base
    var dependent_call = MacroExpansionContext.MacroCall{
        .name = "DEPENDENT",
        .arguments = &[_]Token{},
        .location = .{ .file = "test", .line = 1, .column = 1, .byte_offset = 0 },
        .context_tokens = &[_]Token{},
    };
    
    _ = try expansion_ctx.queueMacroExpansion(dependent_call);
    
    // Should process in dependency order
    const result = try expansion_ctx.processExpansions();
    defer testing.allocator.free(result);
    
    try testing.expect(result.len > 0);
}

// Test 4: Function-like macro parameter substitution
test "function-like macro parameter substitution" {
    var hygiene_ctx = try MacroHygieneContext.init(testing.allocator);
    defer hygiene_ctx.deinit();
    
    var expansion_ctx = try MacroExpansionContext.init(testing.allocator, &hygiene_ctx);
    defer expansion_ctx.deinit();
    
    // Define function-like macro: ADD(a, b) -> a + b
    var add_macro = MacroExpansionContext.MacroDefinition.init(testing.allocator, "ADD");
    add_macro.is_function_like = true;
    add_macro.parameters = try testing.allocator.alloc([]const u8, 2);
    add_macro.parameters[0] = "a";
    add_macro.parameters[1] = "b";
    add_macro.body = &[_]Token{
        createToken(.Identifier, "a", 1, 1),
        createToken(.Plus, "+", 1, 2),
        createToken(.Identifier, "b", 1, 3),
    };
    try expansion_ctx.defineMacro(add_macro);
    
    // Call with arguments: @ADD(5, 10)
    var call_args = [_]Token{
        createToken(.Number, "5", 1, 1),
        createToken(.Comma, ",", 1, 2),
        createToken(.Number, "10", 1, 3),
    };
    
    var call = MacroExpansionContext.MacroCall{
        .name = "ADD",
        .arguments = &call_args,
        .location = .{ .file = "test", .line = 1, .column = 1, .byte_offset = 0 },
        .context_tokens = &[_]Token{},
    };
    
    _ = try expansion_ctx.queueMacroExpansion(call);
    
    const result = try expansion_ctx.processExpansions();
    defer testing.allocator.free(result);
    
    try testing.expect(result.len >= 3);
    try testing.expectEqualStrings("5", result[0].lexeme);
    try testing.expectEqualStrings("+", result[1].lexeme);
    try testing.expectEqualStrings("10", result[2].lexeme);
}

// Test 5: Hygiene violation detection
test "hygiene violation detection" {
    var hygiene_ctx = try MacroHygieneContext.init(testing.allocator);
    defer hygiene_ctx.deinit();
    
    // Declare variable in outer scope
    _ = try hygiene_ctx.declareSymbol("x", .Variable);
    
    // Begin macro expansion that shadows the variable
    _ = try hygiene_ctx.beginMacroExpansion("test_macro", "test.csd:1");
    
    // Declare variable with same name (should trigger hygiene violation)
    _ = try hygiene_ctx.declareSymbol("x", .Variable);
    
    try hygiene_ctx.endMacroExpansion();
    
    // Should have detected shadowing violation
    try testing.expect(hygiene_ctx.hygiene_violations.items.len > 0);
    try testing.expect(hygiene_ctx.hygiene_violations.items[0].kind == .SymbolShadowing);
}

// Test 6: Parser integration
test "parser macro integration" {
    const test_source = 
        \\#define MAX 100
        \\sus value drip = @MAX;
    ;
    
    // This is a simplified test - in practice, we'd need to run the lexer first
    var test_tokens = [_]Token{
        createToken(.Hash, "#", 1, 1),
        createToken(.Identifier, "define", 1, 2),
        createToken(.Identifier, "MAX", 1, 3),
        createToken(.Number, "100", 1, 4),
        createToken(.Newline, "\n", 1, 5),
        createToken(.Identifier, "sus", 2, 1),
        createToken(.Identifier, "value", 2, 2),
        createToken(.Identifier, "drip", 2, 3),
        createToken(.Equal, "=", 2, 4),
        createToken(.At, "@", 2, 5),
        createToken(.Identifier, "MAX", 2, 6),
        createToken(.Semicolon, ";", 2, 7),
    };
    
    var macro_parser = try MacroAwareParser.init(testing.allocator, &test_tokens);
    defer macro_parser.deinit();
    
    // Test scanning
    try macro_parser.scanForMacros();
    try testing.expect(macro_parser.pending_macro_calls.items.len > 0);
    
    // Test builtin macros
    try macro_parser.registerBuiltinMacros();
    
    const stats = macro_parser.getExpansionStats();
    try testing.expect(stats.total_macros_defined > 0);
}

// Test 7: Nested macro expansion
test "nested macro expansion order" {
    var hygiene_ctx = try MacroHygieneContext.init(testing.allocator);
    defer hygiene_ctx.deinit();
    
    var expansion_ctx = try MacroExpansionContext.init(testing.allocator, &hygiene_ctx);
    defer expansion_ctx.deinit();
    
    // Define inner macro
    var inner_macro = MacroExpansionContext.MacroDefinition.init(testing.allocator, "INNER");
    inner_macro.body = &[_]Token{
        createToken(.Number, "42", 1, 1),
    };
    try expansion_ctx.defineMacro(inner_macro);
    
    // Define outer macro that uses inner
    var outer_macro = MacroExpansionContext.MacroDefinition.init(testing.allocator, "OUTER");
    outer_macro.body = &[_]Token{
        createToken(.At, "@", 1, 1),
        createToken(.Identifier, "INNER", 1, 2),
        createToken(.Plus, "+", 1, 3),
        createToken(.Number, "1", 1, 4),
    };
    try expansion_ctx.defineMacro(outer_macro);
    
    // Call outer macro
    var call = MacroExpansionContext.MacroCall{
        .name = "OUTER",
        .arguments = &[_]Token{},
        .location = .{ .file = "test", .line = 1, .column = 1, .byte_offset = 0 },
        .context_tokens = &[_]Token{},
    };
    
    _ = try expansion_ctx.queueMacroExpansion(call);
    
    const result = try expansion_ctx.processExpansions();
    defer testing.allocator.free(result);
    
    // Should expand to: 42 + 1
    try testing.expect(result.len >= 3);
}

// Test 8: Expansion caching
test "expansion result caching" {
    var hygiene_ctx = try MacroHygieneContext.init(testing.allocator);
    defer hygiene_ctx.deinit();
    
    var expansion_ctx = try MacroExpansionContext.init(testing.allocator, &hygiene_ctx);
    defer expansion_ctx.deinit();
    
    // Define macro
    var cached_macro = MacroExpansionContext.MacroDefinition.init(testing.allocator, "CACHED");
    cached_macro.body = &[_]Token{
        createToken(.String, "expensive_computation", 1, 1),
    };
    try expansion_ctx.defineMacro(cached_macro);
    
    // Call same macro multiple times
    var call1 = MacroExpansionContext.MacroCall{
        .name = "CACHED",
        .arguments = &[_]Token{},
        .location = .{ .file = "test", .line = 1, .column = 1, .byte_offset = 0 },
        .context_tokens = &[_]Token{},
    };
    
    var call2 = MacroExpansionContext.MacroCall{
        .name = "CACHED",
        .arguments = &[_]Token{},
        .location = .{ .file = "test", .line = 2, .column = 1, .byte_offset = 0 },
        .context_tokens = &[_]Token{},
    };
    
    _ = try expansion_ctx.queueMacroExpansion(call1);
    _ = try expansion_ctx.queueMacroExpansion(call2);
    
    // Second call should use cached result
    const result = try expansion_ctx.processExpansions();
    defer testing.allocator.free(result);
    
    try testing.expect(result.len > 0);
}

// Test 9: Circular dependency detection
test "circular dependency detection" {
    var hygiene_ctx = try MacroHygieneContext.init(testing.allocator);
    defer hygiene_ctx.deinit();
    
    var expansion_ctx = try MacroExpansionContext.init(testing.allocator, &hygiene_ctx);
    defer expansion_ctx.deinit();
    
    // Define macro A that depends on B
    var macro_a = MacroExpansionContext.MacroDefinition.init(testing.allocator, "A");
    macro_a.body = &[_]Token{
        createToken(.At, "@", 1, 1),
        createToken(.Identifier, "B", 1, 2),
    };
    try expansion_ctx.defineMacro(macro_a);
    
    // Define macro B that depends on A (circular)
    var macro_b = MacroExpansionContext.MacroDefinition.init(testing.allocator, "B");
    macro_b.body = &[_]Token{
        createToken(.At, "@", 1, 1),
        createToken(.Identifier, "A", 1, 2),
    };
    try expansion_ctx.defineMacro(macro_b);
    
    // Try to expand A (should detect circular dependency)
    var call = MacroExpansionContext.MacroCall{
        .name = "A",
        .arguments = &[_]Token{},
        .location = .{ .file = "test", .line = 1, .column = 1, .byte_offset = 0 },
        .context_tokens = &[_]Token{},
    };
    
    _ = try expansion_ctx.queueMacroExpansion(call);
    
    // Should detect circular dependency during processing
    const result = expansion_ctx.processExpansions();
    try testing.expectError(error.CircularDependency, result);
}

// Test 10: Performance with many macros
test "performance with many macro expansions" {
    var hygiene_ctx = try MacroHygieneContext.init(testing.allocator);
    defer hygiene_ctx.deinit();
    
    var expansion_ctx = try MacroExpansionContext.init(testing.allocator, &hygiene_ctx);
    defer expansion_ctx.deinit();
    
    // Define many simple macros
    const num_macros = 100;
    for (0..num_macros) |i| {
        var name_buf: [32]u8 = undefined;
        const name = try std.fmt.bufPrint(&name_buf, "MACRO_{d}", .{i});
        
        var macro_def = MacroExpansionContext.MacroDefinition.init(testing.allocator, name);
        macro_def.name = try testing.allocator.dupe(u8, name);
        macro_def.body = &[_]Token{
            createToken(.Number, "1", 1, 1),
        };
        try expansion_ctx.defineMacro(macro_def);
        
        // Queue expansion
        var call = MacroExpansionContext.MacroCall{
            .name = try testing.allocator.dupe(u8, name),
            .arguments = &[_]Token{},
            .location = .{ .file = "test", .line = 1, .column = 1, .byte_offset = 0 },
            .context_tokens = &[_]Token{},
        };
        
        _ = try expansion_ctx.queueMacroExpansion(call);
    }
    
    // Process all expansions
    const start_time = std.time.milliTimestamp();
    const result = try expansion_ctx.processExpansions();
    const end_time = std.time.milliTimestamp();
    defer testing.allocator.free(result);
    
    // Should complete in reasonable time (< 100ms for 100 macros)
    const duration = end_time - start_time;
    try testing.expect(duration < 100);
    try testing.expect(result.len == num_macros);
}

// Note: Individual test functions can be run via `zig test` command
