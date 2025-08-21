const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast_simple.zig");

const Token = lexer.Token;
const TokenKind = lexer.TokenKind;
const Parser = parser.Parser;
const Program = ast.Program;

pub const RoundTripError = error{
    ParseError,
    PrettyPrintError,
    ReparseError,
    MismatchError,
    OutOfMemory,
};

/// Round-trip test: source → AST → pretty-print → reparse → compare
pub fn testRoundTrip(allocator: Allocator, source: []const u8) RoundTripError!void {
    // Step 1: Parse source to AST
    var lex = lexer.Lexer.init(source);
    const tokens = lex.tokenize(allocator) catch |err| switch (err) {
        else => return RoundTripError.ParseError,
    };
    defer allocator.free(tokens);

    var p = Parser.init(allocator, tokens);
    var program = p.parseProgram() catch |err| switch (err) {
        else => return RoundTripError.ParseError,
    };
    defer program.deinit();

    // Step 2: Pretty-print AST back to source
    var pretty_printed = .empty;
    defer pretty_printed.deinit();
    
    try prettyPrintProgram(&pretty_printed, program);
    
    // Step 3: Reparse the pretty-printed source
    var lex2 = lexer.Lexer.init(pretty_printed.items);
    const tokens2 = lex2.tokenize(allocator) catch |err| switch (err) {
        else => return RoundTripError.ReparseError,
    };
    defer allocator.free(tokens2);

    var p2 = Parser.init(allocator, tokens2);
    var program2 = p2.parseProgram() catch |err| switch (err) {
        else => return RoundTripError.ReparseError,
    };
    defer program2.deinit();

    // Step 4: Compare ASTs (simplified comparison)
    if (!programsEqual(program, program2)) {
        std.debug.print("Round-trip test failed!\n", .{});
        std.debug.print("Original: {s}\n", .{source});
        std.debug.print("Pretty-printed: {s}\n", .{pretty_printed.items});
        return RoundTripError.MismatchError;
    }
}

/// Pretty-print a program back to canonical CURSED source
fn prettyPrintProgram(writer: *ArrayList(u8), program: Program) !void {
    // Print package declaration
    if (program.package) |pkg| {
        try writer.appendSlice("vibe ");
        try writer.appendSlice(pkg.name);
        try writer.appendSlice("\n\n");
    }

    // Print imports
    for (program.imports.items) |import| {
        try writer.appendSlice("yeet ");
        try writer.appendSlice("\"");
        try writer.appendSlice(import.path);
        try writer.appendSlice("\"");
        try writer.appendSlice("\n");
    }
    
    if (program.imports.items.len > 0) {
        try writer.appendSlice("\n");
    }

    // Print statements
    for (program.statements.items) |stmt| {
        try prettyPrintStatement(writer, stmt);
        try writer.appendSlice("\n");
    }
}

/// Pretty-print a statement using canonical syntax
fn prettyPrintStatement(writer: *ArrayList(u8), stmt: ast.Statement) !void {
    switch (stmt) {
        .Return => {
            try writer.appendSlice("damn"); // Canonical return keyword
        },
        .Let => {
            try writer.appendSlice("sus "); // Canonical variable declaration
        },
        .Function => {
            try writer.appendSlice("slay "); // Canonical function keyword
        },
        .If => {
            try writer.appendSlice("lowkey "); // Canonical if keyword
        },
        .For => {
            try writer.appendSlice("bestie "); // Canonical for keyword
        },
        .While => {
            try writer.appendSlice("periodt "); // Canonical while keyword
        },
        else => {
            try writer.appendSlice("/* statement */");
        },
    }
}

/// Simple program equality check (for round-trip testing)
fn programsEqual(p1: Program, p2: Program) bool {
    // Check package names
    if (p1.package != null and p2.package != null) {
        if (!std.mem.eql(u8, p1.package.?.name, p2.package.?.name)) {
            return false;
        }
    } else if (p1.package != null or p2.package != null) {
        return false;
    }

    // Check import count
    if (p1.imports.items.len != p2.imports.items.len) {
        return false;
    }

    // Check statement count  
    if (p1.statements.items.len != p2.statements.items.len) {
        return false;
    }

    return true;
}

/// Test canonical syntax forms
pub fn testCanonicalSyntax(allocator: Allocator) !void {
    const test_cases = [_][]const u8{
        // Canonical return statement
        "slay test() { damn 42 }",
        
        // Canonical boolean literals
        "sus flag lit = based",
        "sus flag lit = cringe",
        
        // Canonical nil literal
        "sus ptr *normie = nah",
        
        // Canonical map syntax
        "sus m map[tea]normie",
        
        // Combined canonical syntax
        "slay process() {\n  lowkey based {\n    damn nah\n  }\n}",
    };

    for (test_cases) |test_case| {
        std.debug.print("Testing canonical syntax: {s}\n", .{test_case});
        testRoundTrip(allocator, test_case) catch |err| {
            std.debug.print("Round-trip test failed for: {s}\n", .{test_case});
            return err;
        };
    }
}

/// Test rejection of non-canonical syntax
pub fn testRejectNonCanonical(allocator: Allocator) !void {
    const rejected_cases = [_][]const u8{
        // Deprecated return keywords
        "slay test() { yolo 42 }", // Should reject 'yolo', use 'damn'
        
        // Deprecated boolean literals
        "sus flag lit = truth", // Should reject 'truth', use 'based' 
        "sus flag lit = lies",  // Should reject 'lies', use 'cringe'
        "sus flag lit = cap",   // Should reject 'cap', use 'nah' for nil
    };

    for (rejected_cases) |test_case| {
        std.debug.print("Testing rejection of: {s}\n", .{test_case});
        
        var lex = lexer.Lexer.init(test_case);
        const tokens = lex.tokenize(allocator) catch continue;
        defer allocator.free(tokens);

        var p = Parser.init(allocator, tokens);
        var program = p.parseProgram() catch {
            // Expected to fail - this is good!
            std.debug.print("✓ Correctly rejected non-canonical syntax: {s}\n", .{test_case});
            continue;
        };
        program.deinit();
        
        // If we reach here, the parser accepted non-canonical syntax - this is bad!
        std.debug.print("✗ Parser incorrectly accepted non-canonical syntax: {s}\n", .{test_case});
        return error.NonCanonicalAccepted;
    }
}

test "round trip canonical syntax" {
    const allocator = std.testing.allocator;
    try testCanonicalSyntax(allocator);
}

test "reject non-canonical syntax" {
    const allocator = std.testing.allocator;
    try testRejectNonCanonical(allocator);
}
