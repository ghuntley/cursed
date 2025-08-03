const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast_simple.zig");
const codegen = @import("codegen_clean.zig");
const stdlib_integration = @import("stdlib_integration.zig");

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
    var enable_stdlib_runtime = true;
    var stdlib_debug = false;
    
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
            optimization_level = std.fmt.parseUnsigned(u8, level_str, 10) catch 2;
        } else if (std.mem.eql(u8, arg, "--stdlib-debug")) {
            stdlib_debug = true;
        } else if (std.mem.eql(u8, arg, "--no-stdlib")) {
            enable_stdlib_runtime = false;
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

    // Initialize stdlib runtime if enabled
    var stdlib_runtime_integration: ?stdlib_integration.StdlibIntegration = null;
    if (enable_stdlib_runtime) {
        print("🚀 Initializing CURSED Stdlib Runtime...\n", .{});
        if (stdlib_integration.createStdlibIntegration(allocator, "stdlib")) |integration| {
            stdlib_runtime_integration = integration;
        } else |err| {
            print("⚠️ Failed to initialize stdlib runtime: {any}\n", .{err});
            print("Continuing without stdlib runtime integration\n", .{});
        }
        
        if (stdlib_runtime_integration) |*integration| {
            integration.setDebugMode(stdlib_debug);
            print("✅ Stdlib runtime initialized successfully\n", .{});
        }
    }
    defer if (stdlib_runtime_integration) |*integration| integration.deinit();

    // Tokenize
    var l = lexer.Lexer.init(allocator, source);
    defer l.deinit();

    const tokens = l.tokenize() catch |err| {
        print("Lexer error: {}\n", .{err});
        return;
    };

    if (debug_tokens) {
        print("=== TOKENS ===\n", .{});
        for (tokens.items) |token| {
            print("{}: '{}'\n", .{ token.type, token.literal });
        }
        print("\n", .{});
    }

    // Parse
    var p = parser.Parser.init(allocator, tokens);
    defer p.deinit();

    const program = p.parseProgram() catch |err| {
        print("Parser error: {}\n", .{err});
        return;
    };

    if (debug_ast) {
        print("=== AST ===\n", .{});
        print("Program with {} statements\n", .{program.statements.items.len});
        print("\n", .{});
    }

    if (compile_mode) {
        // Generate native executable
        const output_name = try getOutputName(allocator, filename);
        defer allocator.free(output_name);
        
        var c = codegen.CodeGen.init(allocator);
        defer c.deinit();

        c.generateProgram(program) catch |err| {
            print("Code generation error: {}\n", .{err});
            return;
        };

        c.writeExecutable(output_name) catch |err| {
            print("Executable generation error: {}\n", .{err});
            return;
        };

        print("Generated executable: {s}\n", .{output_name});
    } else {
        // Interpretation mode - basic execution
        print("Interpretation mode not yet implemented in Zig version\n", .{});
        print("Use --compile flag to generate native executable\n", .{});
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
    _ = @import("ast_simple.zig");
    _ = @import("codegen_clean.zig");
}
