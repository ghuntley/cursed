const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast.zig");
const codegen = @import("codegen.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        print("Usage: cursed-zig <file.csd> [--compile]\n", .{});
        print("       cursed-zig --version\n", .{});
        return;
    }

    if (std.mem.eql(u8, args[1], "--version")) {
        print("CURSED Zig Compiler v0.1.0\n", .{});
        return;
    }

    const filename = args[1];
    const compile_mode = args.len > 2 and std.mem.eql(u8, args[2], "--compile");

    // Read source file
    const file_content = std.fs.cwd().readFileAlloc(allocator, filename, 1024 * 1024) catch |err| {
        print("Error reading file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(file_content);

    // Lexical analysis
    var lex = lexer.Lexer.init(allocator, file_content);
    const tokens = try lex.tokenize();
    defer tokens.deinit();

    if (std.process.hasEnvVar(allocator, "CURSED_DEBUG_TOKENS") catch false) {
        print("=== TOKENS ===\n", .{});
        for (tokens.items) |token| {
            print("{any}: '{s}'\n", .{ token.kind, token.lexeme });
        }
        print("\n", .{});
    }

    // Parse into AST
    var parse = parser.Parser.init(allocator, tokens.items);
    const program = try parse.parseProgram();
    defer program.deinit(allocator);

    if (std.process.hasEnvVar(allocator, "CURSED_DEBUG_AST") catch false) {
        print("=== AST ===\n", .{});
        try program.print(0);
        print("\n", .{});
    }

    if (compile_mode) {
        // Compile to native executable
        var codegen_ctx = codegen.CodeGen.init(allocator);
        defer codegen_ctx.deinit();
        
        try codegen_ctx.generateProgram(program);
        const output_name = try getOutputName(allocator, filename);
        defer allocator.free(output_name);
        
        try codegen_ctx.writeExecutable(output_name);
        print("Compiled {s} to {s}\n", .{ filename, output_name });
    } else {
        // Interpret mode
        var interpreter = @import("interpreter.zig").Interpreter.init(allocator);
        defer interpreter.deinit();
        
        try interpreter.execute(program);
    }
}

fn getOutputName(allocator: Allocator, filename: []const u8) ![]u8 {
    if (std.mem.endsWith(u8, filename, ".csd")) {
        return try allocator.dupe(u8, filename[0..filename.len - 4]);
    }
    return try std.fmt.allocPrint(allocator, "{s}_out", .{filename});
}

test "main tests" {
    // Import tests from submodules
    _ = @import("lexer.zig");
    _ = @import("parser.zig");
    _ = @import("ast.zig");
    _ = @import("codegen.zig");
}
