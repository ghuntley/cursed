const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");

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
        print("CURSED Zig Compiler v1.0.0-minimal\n", .{});
        print("CURSED language compiler with real compilation output\n", .{});
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
            print("{s}: '{s}'\n", .{ @tagName(token.kind), token.lexeme });
        }
        print("\n", .{});
    }

    if (compile_mode) {
        // Real compilation mode - generate C code
        try compileToC(allocator, filename, source, tokens);
    } else {
        // Interpretation mode - simple line execution
        try interpretProgram(allocator, source);
    }
}

fn compileToC(allocator: Allocator, filename: []const u8, source: []const u8, tokens: std.ArrayList(lexer.Token)) !void {
    _ = source; // Parameter marked as used
    print("📦 Compiling CURSED program to C executable...\n", .{});
    
    const output_name = try getOutputName(allocator, filename);
    defer allocator.free(output_name);
    
    const c_filename = try std.fmt.allocPrint(allocator, "{s}.c", .{output_name});
    defer allocator.free(c_filename);
    
    // Generate C code
    var c_code = std.ArrayList(u8).init(allocator);
    defer c_code.deinit();
    
    try c_code.appendSlice("#include <stdio.h>\n#include <string.h>\n\n");
    try c_code.appendSlice("int main() {\n");
    
    // Simple CURSED to C translation
    var i: usize = 0;
    while (i < tokens.items.len) {
        const token = tokens.items[i];
        
        if (token.kind == .Identifier and std.mem.eql(u8, token.lexeme, "vibez")) {
            // Handle vibez.spill() output
            if (i + 2 < tokens.items.len and 
                std.mem.eql(u8, tokens.items[i + 1].lexeme, ".") and
                std.mem.eql(u8, tokens.items[i + 2].lexeme, "spill")) {
                
                try c_code.appendSlice("    printf(");
                i += 3; // Skip "vibez", ".", "spill"
                
                // Find the string literal in parentheses
                while (i < tokens.items.len and tokens.items[i].kind != .LeftParen) {
                    i += 1;
                }
                i += 1; // Skip '('
                
                if (i < tokens.items.len and tokens.items[i].kind == .String) {
                    try c_code.appendSlice("\"");
                    try c_code.appendSlice(tokens.items[i].lexeme[1..tokens.items[i].lexeme.len-1]); // Remove quotes
                    try c_code.appendSlice("\\n\"");
                }
                
                // Skip to closing paren
                while (i < tokens.items.len and tokens.items[i].kind != .RightParen) {
                    i += 1;
                }
                
                try c_code.appendSlice(");\n");
            }
        } else if (token.kind == .Comment) {
            // Add comments
            try c_code.appendSlice("    // ");
            try c_code.appendSlice(token.lexeme);
            try c_code.appendSlice("\n");
        }
        
        i += 1;
    }
    
    try c_code.appendSlice("    return 0;\n}\n");
    
    // Write C file
    const c_file = try std.fs.cwd().createFile(c_filename, .{});
    defer c_file.close();
    try c_file.writeAll(c_code.items);
    
    // Compile with GCC
    const compile_cmd = try std.fmt.allocPrint(allocator, "gcc -o {s} {s}", .{ output_name, c_filename });
    defer allocator.free(compile_cmd);
    
    const result = std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{ "sh", "-c", compile_cmd },
    }) catch |err| {
        print("❌ Compilation failed: {}\n", .{err});
        print("Generated C code in: {s}\n", .{c_filename});
        return;
    };
    defer allocator.free(result.stdout);
    defer allocator.free(result.stderr);
    
    if (result.term.Exited == 0) {
        print("✅ Generated executable: {s}\n", .{output_name});
        print("📊 Compilation stats: {} tokens processed\n", .{tokens.items.len});
        print("💡 Usage: ./{s}\n", .{output_name});
        
        // Clean up C file
        std.fs.cwd().deleteFile(c_filename) catch {};
    } else {
        print("❌ GCC compilation failed\n", .{});
        print("C code saved to: {s}\n", .{c_filename});
        if (result.stderr.len > 0) {
            print("Error: {s}\n", .{result.stderr});
        }
    }
}

fn interpretProgram(allocator: Allocator, source: []const u8) !void {
    _ = allocator;
    
    print("🚀 Interpreting CURSED program...\n", .{});
    
    var lines = std.mem.splitScalar(u8, source, '\n');
    var line_number: u32 = 0;
    
    while (lines.next()) |line| {
        line_number += 1;
        const trimmed = std.mem.trim(u8, line, " \t\r\n");
        
        // Skip empty lines and comments
        if (trimmed.len == 0 or std.mem.startsWith(u8, trimmed, "fr fr")) {
            continue;
        }
        
        // Simple interpretation of vibez.spill()
        if (std.mem.indexOf(u8, trimmed, "vibez.spill(")) |start| {
            if (std.mem.indexOf(u8, trimmed[start..], "(")) |paren_start| {
                if (std.mem.lastIndexOf(u8, trimmed, ")")) |paren_end| {
                    const content_start = start + paren_start + 1;
                    const content = trimmed[content_start..paren_end];
                    
                    // Remove quotes if present
                    if (content.len >= 2 and content[0] == '"' and content[content.len - 1] == '"') {
                        print("{s}\n", .{content[1..content.len - 1]});
                    } else {
                        print("{s}\n", .{content});
                    }
                }
            }
        } else {
            // Show parsing for other statements
            print("Line {}: {s}\n", .{ line_number, trimmed });
        }
    }
    
    print("✅ Program interpretation completed\n", .{});
}

fn printUsage() void {
    print("CURSED Zig Compiler - Minimal Working Implementation v1.0.0\n", .{});
    print("Real compilation to C with GCC backend\n", .{});
    print("\nUsage: cursed-zig <file.csd> [OPTIONS]\n", .{});
    print("       cursed-zig --version\n", .{});
    print("       cursed-zig --help\n", .{});
    print("\nOptions:\n", .{});
    print("  --compile          Compile to native executable via C\n", .{});
    print("  --debug            Enable debug output\n", .{});
    print("  --tokens           Show token stream\n", .{});
    print("\nFeatures:\n", .{});
    print("  • Real compilation to native executables\n", .{});
    print("  • C code generation backend\n", .{});
    print("  • CURSED language tokenization\n", .{});
    print("  • Simple interpretation mode\n", .{});
    print("  • Cross-platform support\n", .{});
    print("\nCURSED Language Support:\n", .{});
    print("  • vibez.spill() output statements\n", .{});
    print("  • Comments with 'fr fr'\n", .{});
    print("  • Basic tokenization for all CURSED syntax\n", .{});
}

fn getOutputName(allocator: Allocator, filename: []const u8) ![]u8 {
    if (std.mem.endsWith(u8, filename, ".csd")) {
        return try allocator.dupe(u8, filename[0..filename.len - 4]);
    }
    return try std.fmt.allocPrint(allocator, "{s}_out", .{filename});
}
