const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const simple_interpreter = @import("simple_interpreter.zig");

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
        print("CURSED Zig Compiler v1.0.0-working\n", .{});
        print("CURSED language interpreter and basic compiler\n", .{});
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
    
    for (args[2..]) |arg| {
        if (std.mem.eql(u8, arg, "--compile")) {
            compile_mode = true;
        } else if (std.mem.eql(u8, arg, "--debug")) {
            debug_tokens = true;
        } else if (std.mem.eql(u8, arg, "--tokens")) {
            debug_tokens = true;
        }
    }

    // Read source file
    const file = std.fs.cwd().openFile(filename, .{}) catch |err| {
        print("Error: Could not open file '{s}': {}\n", .{ filename, err });
        return;
    };
    defer file.close();

    const source = file.readToEndAlloc(allocator, 1024 * 1024) catch |err| {
        print("Error: Could not read file '{s}': {}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);

    print("🚀 CURSED Compiler Processing: {s}\n", .{filename});

    // Tokenize
    var l = lexer.Lexer.init(allocator, source);

    const tokens = l.tokenize() catch |err| {
        print("Lexer error: {}\n", .{err});
        return;
    };

    if (debug_tokens) {
        print("=== TOKENS ===\n", .{});
        for (tokens.items) |token| {
            print("{}: '{s}'\n", .{ token.kind, token.lexeme });
        }
        print("\n", .{});
    }

    if (compile_mode) {
        // Simple script-based compilation
        try compileToScript(allocator, filename, source);
    } else {
        // Interpretation mode - execute CURSED program directly
        print("🚀 Executing CURSED program via interpreter...\n", .{});
        
        var simple_interp = simple_interpreter.SimpleInterpreter.init(allocator);
        defer simple_interp.deinit();
        
        simple_interp.execute(tokens.items) catch |err| {
            print("Interpreter error: {}\n", .{err});
            return;
        };
        
        print("✅ Program execution completed\n", .{});
    }
}

fn compileToScript(allocator: Allocator, filename: []const u8, source: []const u8) !void {
    print("📦 Compiling CURSED program to executable script...\n", .{});
    
    const output_name = try getOutputName(allocator, filename);
    defer allocator.free(output_name);
    
    // Count lines for stats
    var line_count: u32 = 1;
    for (source) |char| {
        if (char == '\n') line_count += 1;
    }
    
    // Generate executable script that calls the interpreter
    const script_content = try std.fmt.allocPrint(allocator,
        \\#!/usr/bin/env bash
        \\# CURSED Executable
        \\# Generated from: {s}
        \\# Source lines: {}
        \\
        \\SCRIPT_DIR="$(cd "$(dirname "${{BASH_SOURCE[0]}}")" && pwd)"
        \\SOURCE_FILE="$SCRIPT_DIR/{s}"
        \\
        \\if [ ! -f "$SOURCE_FILE" ]; then
        \\    echo "Error: Source file not found: $SOURCE_FILE"
        \\    exit 1
        \\fi
        \\
        \\echo "Running CURSED program: {s}"
        \\
        \\# Check if cursed-zig interpreter is available
        \\if command -v cursed-zig >/dev/null 2>&1; then
        \\    cursed-zig "$SOURCE_FILE"
        \\else
        \\    echo "Error: cursed-zig interpreter not found in PATH"
        \\    echo "Please build and install the CURSED compiler first:"
        \\    echo "  zig build"
        \\    echo "  export PATH=\"$PATH:./zig-out/bin\""
        \\    exit 1
        \\fi
        \\
    , .{ filename, line_count, filename, filename });
    defer allocator.free(script_content);
    
    const output_file = try std.fs.cwd().createFile(output_name, .{});
    defer output_file.close();
    
    try output_file.writeAll(script_content);
    
    // Make executable on Unix systems
    if (@import("builtin").os.tag != .windows) {
        try output_file.chmod(0o755);
    }
    
    print("✅ Generated executable: {s}\n", .{output_name});
    print("📊 Compilation stats: {} lines processed\n", .{line_count});
    print("💡 Usage: ./{s}\n", .{output_name});
}

fn printUsage() void {
    print("CURSED Zig Compiler - Working Implementation v1.0.0\n", .{});
    print("Full CURSED language interpreter with script compilation\n", .{});
    print("\nUsage: cursed-zig <file.csd> [OPTIONS]\n", .{});
    print("       cursed-zig --version\n", .{});
    print("       cursed-zig --help\n", .{});
    print("\nOptions:\n", .{});
    print("  --compile          Compile to executable script\n", .{});
    print("  --debug            Enable debug output\n", .{});
    print("  --tokens           Show token stream\n", .{});
    print("\nFeatures:\n", .{});
    print("  • Full CURSED language parsing\n", .{});
    print("  • Interpretation mode (default)\n", .{});
    print("  • Script-based compilation\n", .{});
    print("  • Cross-platform support\n", .{});
    print("  • Error handling and reporting\n", .{});
    print("\nCURSED Language Features:\n", .{});
    print("  • Variables: sus, drip, tea, lit types\n", .{});
    print("  • Functions: slay keyword\n", .{});
    print("  • Output: vibez.spill()\n", .{});
    print("  • Control flow: bestie loops, conditions\n", .{});
    print("  • And much more!\n", .{});
}

fn getOutputName(allocator: Allocator, filename: []const u8) ![]u8 {
    if (std.mem.endsWith(u8, filename, ".csd")) {
        return try allocator.dupe(u8, filename[0..filename.len - 4]);
    }
    return try std.fmt.allocPrint(allocator, "{s}_out", .{filename});
}
