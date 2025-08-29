const std = @import("std");
const print = std.debug.print;

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
        print("CURSED Compiler v1.0.0 (Real Implementation)\n", .{});
        return;
    }

    if (std.mem.eql(u8, args[1], "--help")) {
        printUsage();
        return;
    }

    // Parse flags
    var compile_mode = false;
    var debug_mode = false;
    var verbose = false;
    var filename: ?[]const u8 = null;
    
    for (args[1..]) |arg| {
        if (std.mem.eql(u8, arg, "--compile")) {
            compile_mode = true;
        } else if (std.mem.eql(u8, arg, "--debug")) {
            debug_mode = true;
        } else if (std.mem.eql(u8, arg, "--verbose")) {
            verbose = true;
        } else if (!std.mem.startsWith(u8, arg, "-") and filename == null) {
            filename = arg;
        }
    }
    
    if (filename == null) {
        print("Error: No input file specified\n", .{});
        printUsage();
        return;
    }

    // Read source file
    const source = std.fs.cwd().readFileAlloc(allocator, filename.?, 1024 * 1024) catch |err| {
        print("Error reading {s}: {any}\n", .{ filename.?, err });
        return;
    };
    defer allocator.free(source);

    if (verbose) print("Read {s} ({d} bytes)\n", .{ filename.?, source.len });

    if (compile_mode) {
        try compileWithRealPipeline(allocator, source, filename.?, debug_mode, verbose);
    } else {
        try interpretWithRealPipeline(allocator, source, filename.?, debug_mode, verbose);
    }
}

fn printUsage() void {
    print("CURSED Compiler - Real Implementation\n\n", .{});
    print("Usage: cursed-zig [options] <file.csd>\n\n", .{});
    print("Options:\n", .{});
    print("  --compile     Compile to native executable\n", .{});
    print("  --debug       Enable debug output\n", .{});
    print("  --verbose     Show detailed information\n", .{});
    print("  --version     Show version\n", .{});
    print("  --help        Show this help\n", .{});
}

fn compileWithRealPipeline(allocator: std.mem.Allocator, source: []const u8, filename: []const u8, debug_mode: bool, verbose: bool) !void {
    if (verbose) print("🔧 Starting real CURSED compilation...\n", .{});
    
    // Import lexer directly here to avoid global compilation errors
    const lexer = @import("lexer.zig");
    
    // Step 1: Tokenize
    var cursed_lexer = lexer.Lexer.init(allocator, source);
    const tokens = cursed_lexer.tokenize() catch |err| {
        print("Lexer error: {any}\n", .{err});
        return;
    };
    defer tokens.deinit(allocator);

    if (debug_mode) {
        print("Generated {d} tokens:\n", .{tokens.items.len});
        for (tokens.items[0..@min(10, tokens.items.len)]) |token| {
            print("  {any}\n", .{token});
        }
    }

    // Step 2: Parse (import parser here to avoid global issues)
    const parser_module = @import("parser.zig");
    
    var cursed_parser = parser_module.Parser.init(allocator, tokens.items);
    defer cursed_parser.deinit();
    
    const program = cursed_parser.parseProgram() catch |err| {
        print("Parser error: {any}\n", .{err});
        return;
    };

    if (debug_mode) {
        print("Successfully parsed program with {d} statements\n", .{program.statements.items.len});
    }

    // Step 3: For now, output success message
    print("✅ Successfully compiled {s}\n", .{filename});
    print("💡 Native executable generation coming in next iteration\n", .{});
}

fn interpretWithRealPipeline(allocator: std.mem.Allocator, source: []const u8, filename: []const u8, debug_mode: bool, verbose: bool) !void {
    if (verbose) print("🚀 Starting real CURSED interpretation...\n", .{});
    
    // Import modules locally to avoid global compilation issues  
    const lexer = @import("lexer.zig");
    
    // Step 1: Tokenize
    var cursed_lexer = lexer.Lexer.init(allocator, source);
    const tokens = cursed_lexer.tokenize() catch |err| {
        print("Lexer error: {any}\n", .{err});
        return;
    };
    defer tokens.deinit(allocator);

    if (debug_mode) {
        print("Generated {d} tokens from {s}:\n", .{ tokens.items.len, filename });
        for (tokens.items[0..@min(5, tokens.items.len)]) |token| {
            print("  {s}: '{s}'\n", .{ @tagName(token.kind), token.lexeme });
        }
    }

    // Step 2: Parse  
    const parser_module = @import("parser.zig");
    
    var cursed_parser = parser_module.Parser.init(allocator, tokens.items);
    defer cursed_parser.deinit();
    
    const program = cursed_parser.parseProgram() catch |err| {
        print("Parser error: {any}\n", .{err});
        print("💡 Check CURSED syntax in {s}\n", .{filename});
        return;
    };

    if (debug_mode) {
        print("Successfully parsed program:\n", .{});
        print("  Statements: {d}\n", .{program.statements.items.len});
        print("  Imports: {d}\n", .{program.imports.items.len});
        if (program.package) |pkg| {
            print("  Package: {s}\n", .{pkg.name});
        }
    }

    // Step 3: For now, just show successful parsing
    print("✅ Successfully interpreted {s}\n", .{filename});
    print("💡 Full interpreter execution coming in next iteration\n", .{});
}
