const std = @import("std");
const print = std.debug.print;

// Simple test to verify AST integration works
const ast = @import("src-zig/ast.zig");
const lexer = @import("src-zig/lexer.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        print("Usage: {s} <file.csd>\n", .{args[0]});
        return;
    }

    const filename = args[1];
    
    // Read source file
    const source = readSourceFile(allocator, filename) catch |err| {
        print("Error reading file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);
    
    print("🎯 CURSED AST Integration Test\n", .{});
    print("File: {s}\n", .{filename});
    print("Source length: {} bytes\n", .{source.len});
    
    // Test lexer (this should work)
    var lexer_instance = lexer.Lexer.init(allocator, source);
    
    var tokens = lexer_instance.tokenize() catch |err| {
        print("Tokenization failed: {any}\n", .{err});
        return;
    };
    defer tokens.deinit(allocator);
    
    print("✅ Tokenized successfully: {} tokens\n", .{tokens.items.len});
    
    // Print some tokens for verification
    for (tokens.items[0..@min(10, tokens.items.len)]) |token| {
        print("  Token: {s} = '{s}'\n", .{ @tagName(token.kind), token.lexeme });
    }
    
    // Test AST types (verify they compile)
    print("\n🎯 AST Types Test:\n", .{});
    
    // Create some basic AST nodes
    const int_literal = ast.Expression{
        .Integer = 42
    };
    print("  Integer literal created: {any}\n", .{int_literal});
    
    const string_literal = ast.Expression{
        .String = "Hello, CURSED!"
    };  
    print("  String literal created: {any}\n", .{string_literal});
    
    // Test let statement (variable declaration in AST)
    const let_stmt = ast.Statement{
        .Let = ast.LetStatement{
            .name = "test_var",
            .var_type = ast.Type{ .Basic = ast.BasicType.Drip },
            .type_annotation = null,
            .initializer = null,
            .is_mutable = false,
        }
    };
    print("  Let statement created: {any}\n", .{let_stmt});
    
    print("\n✅ AST integration test completed successfully!\n", .{});
    print("🎯 Full parser can now be safely integrated.\n", .{});
}

fn readSourceFile(allocator: std.mem.Allocator, filename: []const u8) ![]u8 {
    var file = std.fs.cwd().openFile(filename, .{}) catch |err| switch (err) {
        error.FileNotFound => {
            print("File not found: {s}\n", .{filename});
            return err;
        },
        else => return err,
    };
    defer file.close();
    
    const file_size = try file.getEndPos();
    const contents = try allocator.alloc(u8, file_size);
    _ = try file.readAll(contents);
    return contents;
}
