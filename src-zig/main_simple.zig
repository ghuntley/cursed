const std = @import("std");
const print = std.debug.print;

const lexer = @import("lexer.zig");
const ast = @import("ast_simple.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        print("Usage: cursed-zig <file.csd>\n", .{});
        return;
    }

    if (std.mem.eql(u8, args[1], "--version")) {
        print("CURSED Zig Compiler v0.1.0\n", .{});
        return;
    }

    const filename = args[1];

    // Read source file
    const file_content = std.fs.cwd().readFileAlloc(allocator, filename, 1024 * 1024) catch |err| {
        print("Error reading file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(file_content);

    print("Successfully read {s} ({} bytes)\n", .{ filename, file_content.len });

    // Lexical analysis
    var lex = lexer.Lexer.init(allocator, file_content);
    const tokens = try lex.tokenize();
    defer tokens.deinit();

    print("Lexed {} tokens\n", .{tokens.items.len});
    
    // Print first few tokens for verification
    for (tokens.items[0..@min(tokens.items.len, 5)]) |token| {
        print("Token: {any} = '{s}'\n", .{ token.kind, token.lexeme });
    }

    print("✅ CURSED Zig compiler working!\n", .{});
}

test "main tests" {
    _ = lexer;
    _ = ast;
}
