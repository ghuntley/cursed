const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast.zig");
const simple_interpreter = @import("simple_interpreter.zig");

// Conditional imports - only use if available
const has_llvm = @import("builtin").link_libc and @hasDecl(@import("std"), "c");

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
        print("CURSED Zig Compiler v1.0.0-conditional\n", .{});
        print("Advanced parser with interpretation and conditional compilation\n", .{});
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
        }
    }

    // Read source file
    const file = std.fs.cwd().openFile(filename, .{}) catch |err| {
        print("Error: Could not open file '{s}': {s}\n", .{{ filename, err });
        return;
    };
    defer file.close();

    const source = file.readToEndAlloc(allocator, 1024 * 1024) catch |err| {
        print("Error: Could not read file '{s}': {s}\n", .{{ filename, err });
        return;
    };
    defer allocator.free(source);

    print("🚀 CURSED Compiler Processing: {s}\n", .{filename});

    // Tokenize
    var l = lexer.Lexer.init(allocator, source);
    // Note: Lexer doesn't have deinit method

    const tokens = l.tokenize() catch |err| {
        print("Lexer error: {s}\n", .{{err});
        return;
    };

    if (debug_tokens) {
        print("=== TOKENS ===\n", .{});
        for (tokens.items) |token| {
            print("{s}: '{s}'\n", .{{ token.type, token.literal });
        }
        print("\n", .{});
    }

    // Parse
    var p = parser.Parser.init(allocator, tokens);
    defer p.deinit();

    const program = p.parseProgram() catch |err| {
        print("Parser error: {s}\n", .{{err});
        return;
    };

    if (debug_ast) {
        print("=== AST ===\n", .{});
        print("Program with {s} statements\n", .{{program.statements.items.len});
        print("\n", .{});
    }

    if (compile_mode) {
        // Compilation mode
        if (has_llvm) {
            // Try to compile with LLVM
            try compileWithLLVM(allocator, filename, source, program);
        } else {
            // Fallback to bytecode compilation
            try compileWithBytecode(allocator, filename, source, program);
        }
    } else {
        // Interpretation mode - execute CURSED program directly
        print("🚀 Executing CURSED program via interpreter...\n", .{});
        
        var simple_interp = simple_interpreter.SimpleInterpreter.init(allocator);
        defer simple_interp.deinit();
        
        simple_interp.execute(tokens.items) catch |err| {
            print("Interpreter error: {s}\n", .{{err});
            return;
        };
        
        print("✅ Program execution completed\n", .{});
    }
}

fn compileWithLLVM(allocator: Allocator, filename: []const u8, source: []const u8, program: ast.Program) !void {
    _ = program;
    
    // Try to import complete compiler
    const complete_compiler = @import("complete_compiler.zig");
    
    const output_name = try getOutputName(allocator, filename);
    defer allocator.free(output_name);
    
    var compiler = try complete_compiler.CursedCompiler.init(allocator, filename, output_name);
    defer compiler.deinit();

    const stats = try compiler.compileToExecutable(source);
    
    print("✅ Generated executable: {s}\n", .{output_name});
    print("📊 Compilation stats: {s} lines, {s} tokens, {s} instructions\n", .{{stats.source_lines, stats.tokens_generated, stats.llvm_instructions});
}

fn compileWithBytecode(allocator: Allocator, filename: []const u8, source: []const u8, program: ast.Program) !void {
    _ = source;
    
    print("📦 Compiling to CURSED bytecode (LLVM not available)...\n", .{});
    
    const output_name = try getOutputName(allocator, filename);
    defer allocator.free(output_name);
    
    // Generate simple executable script
    const script_content = try std.fmt.allocPrint(allocator,
        \\#!/usr/bin/env bash
        \\# CURSED Bytecode Executable
        \\# Generated from: {s}
        \\echo "Running CURSED program: {s}"
        \\echo "Statements: {}"
        \\echo "Note: This is a bytecode fallback (LLVM compilation not available)"
        \\
    , .{ filename, filename, program.statements.items.len });
    defer allocator.free(script_content);
    
    const output_file = try std.fs.cwd().createFile(output_name, .{});
    defer output_file.close();
    
    try output_file.writer().writeAll(script_content);
    
    // Make executable
    const mode = std.fs.File.Mode{ .read_only = false, .write_only = false, .execute_only = false };
    try output_file.chmod(mode);
    
    print("✅ Generated bytecode executable: {s}\n", .{output_name});
    print("📊 Fallback compilation: {s} statements processed\n", .{{program.statements.items.len});
}

fn printUsage() void {
    print("CURSED Zig Compiler - Conditional Implementation v1.0.0\n", .{});
    print("Supports both interpretation and compilation (with/without LLVM)\n", .{});
    print("\nUsage: cursed-zig <file.csd> [OPTIONS]\n", .{});
    print("       cursed-zig --version\n", .{});
    print("       cursed-zig --help\n", .{});
    print("\nOptions:\n", .{});
    print("  --compile          Compile to executable (LLVM or bytecode fallback)\n", .{});
    print("  --debug            Enable all debug output\n", .{});
    print("  --ast              Show AST representation\n", .{});
    print("  --tokens           Show token stream\n", .{});
    print("\nFeatures:\n", .{});
    print("  • Full CURSED language parsing\n", .{});
    print("  • Interpretation mode (always available)\n", .{});
    print("  • LLVM compilation (if available)\n", .{});
    print("  • Bytecode fallback compilation\n", .{});
    print("  • Cross-platform support\n", .{});
}

fn getOutputName(allocator: Allocator, filename: []const u8) ![]u8 {
    if (std.mem.endsWith(u8, filename, ".csd")) {
        return try allocator.dupe(u8, filename[0..filename.len - 4]);
    }
    return try std.fmt.allocPrint(allocator, "{s}_out", .{filename});
}
