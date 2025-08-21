const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");

// Minimal CURSED Zig Compiler with Concurrency Integration
// This is a simplified version to test the concurrency integration

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
        print("CURSED Zig Compiler v2.0.0-concurrency-minimal\n", .{});
        return;
    }

    const filename = args[1];
    
    // Parse command line options
    var compile_mode = false;
    var verbose = false;
    
    for (args[2..]) |arg| {
        if (std.mem.eql(u8, arg, "--compile")) {
            compile_mode = true;
        } else if (std.mem.eql(u8, arg, "--verbose")) {
            verbose = true;
        }
    }

    // Read source file
    const source = std.fs.cwd().readFileAlloc(allocator, filename, 1024 * 1024) catch |err| {
        print("❌ Error reading file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);

    if (verbose) print("📁 Read {s} ({} bytes)\n", .{ filename, source.len });

    // Tokenize
    var l = lexer.Lexer.init(allocator, source);
    const tokens = l.tokenize() catch |err| {
        print("❌ Lexer error: {}\n", .{err});
        return;
    };
    defer tokens.deinit(); // Fix memory leak

    if (verbose) print("🔍 Lexed {} tokens\n", .{tokens.items.len});

    // Detect concurrency features
    const has_concurrency = detectConcurrencyFeatures(tokens.items);
    if (verbose and has_concurrency) {
        print("🔧 Concurrency features detected\n", .{});
    }

    if (compile_mode) {
        try compileProgram(allocator, filename, tokens, verbose);
    } else {
        try interpretProgram(allocator, source, verbose);
    }
}

fn detectConcurrencyFeatures(tokens: []const lexer.Token) bool {
    for (tokens) |token| {
        switch (token.kind) {
            .Stan => return true,
            .Dm => return true,
            .Ready => return true,
            .Identifier => {
                if (std.mem.eql(u8, token.lexeme, "dm") or 
                    std.mem.eql(u8, token.lexeme, "stan") or
                    std.mem.eql(u8, token.lexeme, "ready")) {
                    return true;
                }
            },
            else => {},
        }
    }
    return false;
}

fn compileProgram(allocator: Allocator, filename: []const u8, tokens: ArrayList(lexer.Token), verbose: bool) !void {
    print("📦 Compiling CURSED program with concurrency support...\n", .{});
    
    const output_name = try getOutputName(allocator, filename);
    defer allocator.free(output_name);
    
    const c_filename = try std.fmt.allocPrint(allocator, "{s}.c", .{output_name});
    defer allocator.free(c_filename);
    
    var c_code: std.ArrayList(u8) = .empty;
    defer c_code.deinit();
    
    // Generate basic C code with concurrency stubs
    try c_code.appendSlice("#include <stdio.h>\n#include <stdlib.h>\n#include <pthread.h>\n\n");
    try c_code.appendSlice("// CURSED Runtime Stubs\n");
    try c_code.appendSlice("void cursed_runtime_init() { printf(\"[RUNTIME] Initialized\\n\"); }\n");
    try c_code.appendSlice("void cursed_runtime_shutdown() { printf(\"[RUNTIME] Shutdown\\n\"); }\n");
    try c_code.appendSlice("void cursed_spawn_goroutine() { printf(\"[RUNTIME] Goroutine spawned\\n\"); }\n");
    try c_code.appendSlice("void cursed_create_channel() { printf(\"[RUNTIME] Channel created\\n\"); }\n\n");
    
    try c_code.appendSlice("int main() {\n");
    try c_code.appendSlice("    cursed_runtime_init();\n");
    
    // Process tokens for basic output
    var i: usize = 0;
    while (i < tokens.items.len) {
        const token = tokens.items[i];
        
        if (token.kind == .Stan) {
            try c_code.appendSlice("    cursed_spawn_goroutine();\n");
        } else if (token.kind == .Dm) {
            try c_code.appendSlice("    cursed_create_channel();\n");
        } else if (token.kind == .Identifier and std.mem.eql(u8, token.lexeme, "vibez")) {
            if (i + 2 < tokens.items.len and 
                std.mem.eql(u8, tokens.items[i + 1].lexeme, ".") and
                std.mem.eql(u8, tokens.items[i + 2].lexeme, "spill")) {
                
                try c_code.appendSlice("    printf(");
                i += 3;
                
                while (i < tokens.items.len and tokens.items[i].kind != .LeftParen) {
                    i += 1;
                }
                i += 1;
                
                if (i < tokens.items.len and (tokens.items[i].kind == .String or tokens.items[i].kind == .StringLiteral)) {
                    const literal = tokens.items[i].lexeme;
                    try c_code.appendSlice("\"");
                    const content = if (literal.len >= 2 and literal[0] == '"' and literal[literal.len - 1] == '"')
                        literal[1..literal.len-1]
                    else 
                        literal;
                    try c_code.appendSlice(content);
                    try c_code.appendSlice("\\n\"");
                }
                
                while (i < tokens.items.len and tokens.items[i].kind != .RightParen) {
                    i += 1;
                }
                
                try c_code.appendSlice(");\n");
            }
        }
        
        i += 1;
    }
    
    try c_code.appendSlice("    cursed_runtime_shutdown();\n");
    try c_code.appendSlice("    return 0;\n");
    try c_code.appendSlice("}\n");
    
    // Write C file
    const c_file = try std.fs.cwd().createFile(c_filename, .{});
    defer c_file.close();
    try c_file.writeAll(c_code.items);
    
    if (verbose) print("✅ Generated C code: {s}\n", .{c_filename});
    
    // Compile with GCC
    const compile_cmd = try std.fmt.allocPrint(allocator, "gcc -o {s} {s} -lpthread", .{ output_name, c_filename });
    defer allocator.free(compile_cmd);
    
    if (verbose) print("🔨 Running: {s}\n", .{compile_cmd});
    
    const result = std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{ "sh", "-c", compile_cmd },
    }) catch |err| {
        print("❌ Compilation failed: {}\n", .{err});
        return;
    };
    defer allocator.free(result.stdout);
    defer allocator.free(result.stderr);
    
    if (result.term.Exited == 0) {
        print("✅ Generated native executable: {s}\n", .{output_name});
        print("💡 Usage: ./{s}\n", .{output_name});
        
        if (!verbose) {
            std.fs.cwd().deleteFile(c_filename) catch {};
        }
    } else {
        print("❌ GCC compilation failed\n", .{});
        if (result.stderr.len > 0) {
            print("Error: {s}\n", .{result.stderr});
        }
    }
}

fn interpretProgram(_: Allocator, source: []const u8, verbose: bool) !void {
    if (verbose) print("🚀 Interpreting CURSED program with concurrency...\n", .{});
    
    var lines = std.mem.splitScalar(u8, source, '\n');
    var line_number: u32 = 0;
    
    while (lines.next()) |line| {
        line_number += 1;
        const trimmed = std.mem.trim(u8, line, " \t\r\n");
        
        if (trimmed.len == 0 or std.mem.startsWith(u8, trimmed, "fr fr")) {
            continue;
        }
        
        // Concurrency-aware interpretation
        if (std.mem.indexOf(u8, trimmed, "stan")) |_| {
            if (verbose) print("Line {}: Goroutine spawn detected\n", .{line_number});
            print("[Goroutine spawned]\n", .{});
        } else if (std.mem.indexOf(u8, trimmed, "dm")) |_| {
            if (verbose) print("Line {}: Channel operation detected\n", .{line_number});
            print("[Channel operation]\n", .{});
        } else if (std.mem.indexOf(u8, trimmed, "ready")) |_| {
            if (verbose) print("Line {}: Select statement detected\n", .{line_number});
            print("[Select statement executed]\n", .{});
        } else if (std.mem.indexOf(u8, trimmed, "vibez.spill(")) |start| {
            if (std.mem.indexOf(u8, trimmed[start..], "(")) |paren_start| {
                if (std.mem.lastIndexOf(u8, trimmed, ")")) |paren_end| {
                    const content_start = start + paren_start + 1;
                    const content = trimmed[content_start..paren_end];
                    
                    if (content.len >= 2 and content[0] == '"' and content[content.len - 1] == '"') {
                        print("{s}\n", .{content[1..content.len - 1]});
                    } else {
                        print("{s}\n", .{content});
                    }
                }
            }
        } else if (verbose) {
            print("Line {}: {s}\n", .{ line_number, trimmed });
        }
    }
    
    if (verbose) print("✅ Program interpretation completed\n", .{});
}

fn printUsage() void {
    print("CURSED Zig Compiler - Concurrency Minimal v2.0.0\n", .{});
    print("Minimal compiler with concurrency support for testing\n", .{});
    print("\nUsage: cursed-minimal <file.csd> [OPTIONS]\n", .{});
    print("       cursed-minimal --version\n", .{});
    print("\nOptions:\n", .{});
    print("  --compile          Compile to native executable\n", .{});
    print("  --verbose          Enable verbose output\n", .{});
    print("\nConcurrency Features:\n", .{});
    print("  • Goroutines: stan keyword detection\n", .{});
    print("  • Channels: dm<T> type detection\n", .{});
    print("  • Select: ready keyword detection\n", .{});
}

fn getOutputName(allocator: Allocator, filename: []const u8) ![]u8 {
    if (std.mem.endsWith(u8, filename, ".csd")) {
        return try allocator.dupe(u8, filename[0..filename.len - 4]);
    }
    return try std.fmt.allocPrint(allocator, "{s}_out", .{filename});
}

test "minimal concurrency integration" {
    _ = @import("lexer.zig");
}
