const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

const lexer = @import("lexer.zig");
const parser = @import("parser_simple.zig");
const ast = @import("ast_simple.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        std.debug.writer().print("Usage: {s} <source.csd>\n", .{args[0]});
        return;
    }

    const source_file = args[1];
    std.debug.writer().print("Parsing CURSED file: {s}\n", .{source_file});

    // Read the source file
    const file = std.fs.cwd().openFile(source_file, .{}) catch |err| {
        std.debug.writer().print("Error opening file {s}: {s}\n", .{{ source_file, err });
        return;
    };
    defer file.close();

    const file_size = try file.getEndPos();
    const source_content = try allocator.alloc(u8, file_size);
    defer allocator.free(source_content);
    _ = try file.readAll(source_content);

    std.debug.writer().print("Source content ({s} bytes):\n{s}\n\n", .{{ file_size, source_content });

    // Tokenize the source
    var lex = lexer.Lexer.init(allocator, source_content);
    var tokens = ArrayList(lexer.Token){};
    defer tokens.deinit();

    std.debug.writer().print("Tokenizing...\n", .{});
    while (true) {
        const token = lex.nextToken() catch |err| {
            std.debug.writer().print("Lexer error: {s}\n", .{{err});
            break;
        };
        
        std.debug.writer().print("Token: {any} '{s}'\n", .{ token.kind, token.lexeme });
        try tokens.append(allocator, token);
        
        if (token.kind == .Eof) break;
    }

    std.debug.writer().print("\nParsing {s} tokens...\n", .{{tokens.items.len});

    // Parse the tokens
    var parse = parser.Parser.init(allocator, tokens.items);
    var program = parse.parseProgram() catch |err| {
        std.debug.writer().print("Parser error: {s}\n", .{{err});
        return;
    };
    defer program.deinit();

    std.debug.writer().print("\nParsing completed successfully!\n", .{});
    std.debug.writer().print("Program has {s} statements\n", .{{program.statements.items.len});
    
    if (program.package) |pkg| {
        std.debug.writer().print("Package: {s}\n", .{pkg.name});
    }
    
    std.debug.writer().print("Imports: {s}\n", .{{program.imports.items.len});
    for (program.imports.items) |import| {
        std.debug.writer().print("  - {s}\n", .{import.path});
    }

    // Print AST
    std.debug.writer().print("\nAST Structure:\n", .{});
    try program.writer().print(0);
    
    std.debug.writer().print("\n✅ CURSED Parser Test Completed Successfully!\n", .{});
}
