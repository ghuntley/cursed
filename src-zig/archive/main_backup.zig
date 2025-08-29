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
        printUsage();
        return;
    }

    if (std.mem.eql(u8, args[1], "--version")) {
        print("CURSED Zig Compiler v1.0.0-complete\n", .{});
        print("Advanced parser with structs, interfaces, generics, and more\n", .{});
        return;
    }

    if (std.mem.eql(u8, args[1], "--help")) {
        printUsage();
        return;
    }

    const filename = args[1];
    
    // Parse command line options
    var compile_mode = false;
    var debug_tokens = false;
    var debug_ast = false;
    var optimization_level: u8 = 2;
    
    for (args[2..]) |arg| {
        if (std.mem.eql(u8, arg, "--compile")) {
            compile_mode = true;
        } else if (std.mem.eql(u8, arg, "--debug")) {
            debug_tokens = true;
            debug_ast = true;
        } else if (std.mem.eql(u8, arg, "--tokens")) {
            debug_tokens = true;
        } else if (std.mem.eql(u8, arg, "--ast")) {
            debug_ast = true;
        } else if (std.mem.startsWith(u8, arg, "--optimize=")) {
            const level_str = arg[11..];
            optimization_level = std.fmt.parseInt(u8, level_str, 10) catch 2;
        }
    }

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

    if (debug_tokens) {
        print("=== TOKENS ===\n", .{});
        for (tokens.items) |token| {
            print("{any}: '{s}'\n", .{ token.kind, token.lexeme });
        }
        print("\n", .{});
    }

    // Parse into AST
    var parse = parser.Parser.init(allocator, tokens.items);
    const program = try parse.parseProgram();
    defer program.deinit();

    if (debug_ast) {
        print("=== AST ===\n", .{});
        try program.writer().print(0);
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
        print("✅ Compiled {s} to {s} (optimization level: {s})\n", .{{ filename, output_name, optimization_level });
    } else {
        // Interpret mode
        var interpreter = @import("interpreter.zig").Interpreter.init(allocator);
        defer interpreter.deinit();
        
        print("🚀 Executing {s} in interpretation mode...\n", .{filename});
        try interpreter.execute(program);
        print("✅ Execution completed successfully\n", .{});
    }
}

fn printUsage() void {
    print("CURSED Zig Compiler - Complete Implementation v1.0.0\n", .{});
    print("Advanced parser with structs, interfaces, generics, pattern matching\n", .{});
    print("\nUsage: cursed-zig <file.csd> [OPTIONS]\n", .{});
    print("       cursed-zig --version\n", .{});
    print("       cursed-zig --help\n", .{});
    print("\nOptions:\n", .{});
    print("  --compile          Compile to native executable\n", .{});
    print("  --debug            Enable all debug output\n", .{});
    print("  --ast              Show AST representation\n", .{});
    print("  --tokens           Show token stream\n", .{});
    print("  --optimize=LEVEL   Optimization level (0-3, default: 2)\n", .{});
    print("\nAdvanced Features Supported:\n", .{});
    print("  • Structs (squad keyword)\n", .{});
    print("  • Interfaces (collab keyword)\n", .{});
    print("  • Generics with type parameters\n", .{});
    print("  • Pattern matching (match statements)\n", .{});
    print("  • Error handling (shook types)\n", .{});
    print("  • For loops (bestie keyword)\n", .{});
    print("  • Tuples and member access\n", .{});
    print("  • LLVM-based compilation\n", .{});
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
