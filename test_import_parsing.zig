const std = @import("std");
const parser = @import("src-zig/parser.zig");
const lexer = @import("src-zig/lexer.zig");
const ast = @import("src-zig/ast.zig");

test "import system - all 5 canonical forms" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    // Test form 1: Single import
    {
        const source = "yeet \"testz\"";
        var lex = lexer.Lexer.init(allocator, source);
        const tokens = try lex.tokenize();
        var parse = parser.Parser.init(allocator, tokens.items);
        
        const import_stmt = try parse.parseImportStatement();
        
        std.testing.expect(std.mem.eql(u8, import_stmt.path, "testz")) catch |err| {
            std.debug.print("Expected 'testz', got '{s}'\n", .{import_stmt.path});
            return err;
        };
        std.testing.expect(import_stmt.alias == null) catch return error.TestFailed;
        std.testing.expect(import_stmt.multiple_paths.items.len == 0) catch return error.TestFailed;
        std.testing.expect(!import_stmt.is_selective) catch return error.TestFailed;
    }

    // Test form 2: Multiple imports
    {
        const source = "yeet \"mathz\", \"stringz\", \"arrayz\"";
        var lex = lexer.Lexer.init(allocator, source);
        const tokens = try lex.tokenize();
        var parse = parser.Parser.init(allocator, tokens.items);
        
        const import_stmt = try parse.parseImportStatement();
        
        std.testing.expect(std.mem.eql(u8, import_stmt.path, "mathz")) catch return error.TestFailed;
        std.testing.expect(import_stmt.multiple_paths.items.len == 2) catch return error.TestFailed;
        std.testing.expect(std.mem.eql(u8, import_stmt.multiple_paths.items[0], "stringz")) catch return error.TestFailed;
        std.testing.expect(std.mem.eql(u8, import_stmt.multiple_paths.items[1], "arrayz")) catch return error.TestFailed;
    }

    // Test form 3: Aliased import
    {
        const source = "yeet \"mathz\" as math_ops";
        var lex = lexer.Lexer.init(allocator, source);
        const tokens = try lex.tokenize();
        var parse = parser.Parser.init(allocator, tokens.items);
        
        const import_stmt = try parse.parseImportStatement();
        
        std.testing.expect(std.mem.eql(u8, import_stmt.path, "mathz")) catch return error.TestFailed;
        std.testing.expect(import_stmt.alias != null) catch return error.TestFailed;
        std.testing.expect(std.mem.eql(u8, import_stmt.alias.?, "math_ops")) catch return error.TestFailed;
    }

    // Test form 4: Selective imports
    {
        const source = "yeet { print, println } from \"vibez\"";
        var lex = lexer.Lexer.init(allocator, source);
        const tokens = try lex.tokenize();
        var parse = parser.Parser.init(allocator, tokens.items);
        
        const import_stmt = try parse.parseImportStatement();
        
        std.testing.expect(std.mem.eql(u8, import_stmt.path, "vibez")) catch return error.TestFailed;
        std.testing.expect(import_stmt.is_selective) catch return error.TestFailed;
        std.testing.expect(import_stmt.selective_items.items.len == 2) catch return error.TestFailed;
        std.testing.expect(std.mem.eql(u8, import_stmt.selective_items.items[0].name, "print")) catch return error.TestFailed;
        std.testing.expect(std.mem.eql(u8, import_stmt.selective_items.items[1].name, "println")) catch return error.TestFailed;
    }

    // Test form 5: Selective imports with per-item aliasing
    {
        const source = "yeet { HashMap as Map, Vec as List } from \"collections\"";
        var lex = lexer.Lexer.init(allocator, source);
        const tokens = try lex.tokenize();
        var parse = parser.Parser.init(allocator, tokens.items);
        
        const import_stmt = try parse.parseImportStatement();
        
        std.testing.expect(std.mem.eql(u8, import_stmt.path, "collections")) catch return error.TestFailed;
        std.testing.expect(import_stmt.is_selective) catch return error.TestFailed;
        std.testing.expect(import_stmt.selective_items.items.len == 2) catch return error.TestFailed;
        
        const first_item = import_stmt.selective_items.items[0];
        std.testing.expect(std.mem.eql(u8, first_item.name, "HashMap")) catch return error.TestFailed;
        std.testing.expect(first_item.alias != null) catch return error.TestFailed;
        std.testing.expect(std.mem.eql(u8, first_item.alias.?, "Map")) catch return error.TestFailed;
        
        const second_item = import_stmt.selective_items.items[1];
        std.testing.expect(std.mem.eql(u8, second_item.name, "Vec")) catch return error.TestFailed;
        std.testing.expect(second_item.alias != null) catch return error.TestFailed;
        std.testing.expect(std.mem.eql(u8, second_item.alias.?, "List")) catch return error.TestFailed;
    }

    // Test versioned imports (bonus)
    {
        const source = "yeet \"json@^1.0.0\"";
        var lex = lexer.Lexer.init(allocator, source);
        const tokens = try lex.tokenize();
        var parse = parser.Parser.init(allocator, tokens.items);
        
        const import_stmt = try parse.parseImportStatement();
        
        std.testing.expect(std.mem.eql(u8, import_stmt.path, "json")) catch return error.TestFailed;
        std.testing.expect(import_stmt.version != null) catch return error.TestFailed;
        std.testing.expect(std.mem.eql(u8, import_stmt.version.?, "^1.0.0")) catch return error.TestFailed;
    }
    
    std.debug.print("✅ All 5 canonical import forms test passed!\n", .{});
}
