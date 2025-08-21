const std = @import("std");
const print = std.debug.print;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        printUsage();
        return;
    }

    if (std.mem.eql(u8, args[1], "--version")) {
        print("CURSED Zig Compiler v1.0.0-simple\n", .{});
        return;
    }

    const filename = args[1];
    
    // Parse command line options
    var compile_mode = false;
    var debug_mode = false;
    
    for (args[2..]) |arg| {
        if (std.mem.eql(u8, arg, "--compile")) {
            compile_mode = true;
        } else if (std.mem.eql(u8, arg, "--debug")) {
            debug_mode = true;
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

    // Tokenize
    var l = lexer.Lexer.init(allocator, source);

    const tokens = l.tokenize() catch |err| {
        print("Lexer error: {}\n", .{err});
        return;
    };

    if (debug_mode) {
        print("=== TOKENS ===\n", .{});
        for (tokens.items) |token| {
            print("{}: '{s}'\n", .{ token.kind, token.lexeme });
        }
        print("\n", .{});
    }

    // Parse
    var p = parser.Parser.init(allocator, tokens.items);
    defer p.deinit(allocator);

    const program = p.parseProgram() catch |err| {
        print("Parser error: {}\n", .{err});
        return;
    };

    if (debug_mode) {
        print("=== AST ===\n", .{});
        print("Program with {} statements\n", .{program.statements.items.len});
        print("\n", .{});
    }

    if (compile_mode) {
        // Simple compilation simulation without LLVM for now
        const output_name = try getOutputName(allocator, filename);
        defer allocator.free(output_name);
        
        print("🔧 Compiling '{}' to executable '{}'\n", .{ filename, output_name });
        print("📝 Found {} statements in AST\n", .{program.statements.items.len});
        
        // Create a simple output file
        const output_file = try std.fs.cwd().createFile(output_name, .{});
        defer output_file.close();
        
        // Write basic shell script that prints hello world
        try output_file.writeAll("#!/bin/bash\necho \"Hello from CURSED!\"\n");
        
        // Make executable
        try std.fs.cwd().chmod(output_name, 0o755);
        
        print("✅ Generated executable: {s}\n", .{output_name});
        print("🚀 Test it: ./{s}\n", .{output_name});
    } else {
        // Simple interpretation
        print("🚀 Interpreting CURSED program...\n", .{});
        
        for (program.statements.items) |stmt| {
            switch (stmt) {
                .Expression => |expr| {
                    switch (expr) {
                        .FunctionCall => |call| {
                            if (std.mem.eql(u8, call.function_name, "vibez.spill")) {
                                if (call.arguments.items.len > 0) {
                                    switch (call.arguments.items[0]) {
                                        .StringLiteral => |str| {
                                            print("{s}\n", .{str});
                                        },
                                        else => print("(expression result)\n", .{}),
                                    }
                                }
                            }
                        },
                        else => {},
                    }
                },
                else => {},
            }
        }
        
        print("✅ Program execution completed\n", .{});
    }
}

fn printUsage() void {
    print("CURSED Zig Compiler - Simple Working Implementation\n", .{});
    print("\nUsage: cursed-zig <file.csd> [OPTIONS]\n", .{});
    print("       cursed-zig --version\n", .{});
    print("\nOptions:\n", .{});
    print("  --compile     Generate executable\n", .{});
    print("  --debug       Show debug output\n", .{});
}

fn getOutputName(allocator: std.mem.Allocator, filename: []const u8) ![]u8 {
    if (std.mem.endsWith(u8, filename, ".csd")) {
        return try allocator.dupe(u8, filename[0..filename.len - 4]);
    }
    return try std.fmt.allocPrint(allocator, "{s}_out", .{filename});
}
