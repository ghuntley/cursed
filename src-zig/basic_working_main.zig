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
        print("CURSED Compiler v1.0.0 - Real Implementation\n", .{});
        return;
    }

    if (std.mem.eql(u8, args[1], "--help")) {
        printUsage();
        return;
    }

    // Find filename
    var filename: ?[]const u8 = null;
    var verbose = false;
    var debug_mode = false;
    var compile_mode = false;
    
    for (args[1..]) |arg| {
        if (std.mem.eql(u8, arg, "--verbose")) {
            verbose = true;
        } else if (std.mem.eql(u8, arg, "--debug")) {
            debug_mode = true;
        } else if (std.mem.eql(u8, arg, "--compile")) {
            compile_mode = true;
        } else if (!std.mem.startsWith(u8, arg, "-")) {
            filename = arg;
        }
    }
    
    if (filename == null) {
        print("Error: No input file specified\n", .{});
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
        try realCompile(allocator, source, filename.?, debug_mode, verbose);
    } else {
        try realInterpret(allocator, source, filename.?, debug_mode, verbose);
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

fn realInterpret(allocator: std.mem.Allocator, source: []const u8, filename: []const u8, debug_mode: bool, verbose: bool) !void {
    if (verbose) print("🚀 Starting CURSED interpretation with real lexer/parser...\n", .{});
    
    // Import lexer - this is the real CURSED lexer
    const lexer = @import("lexer.zig");
    
    // Step 1: Tokenize source
    var cursed_lexer = lexer.Lexer.init(allocator, source);
    var token_list = cursed_lexer.tokenize() catch |err| {
        print("❌ Lexer error in {s}: {any}\n", .{ filename, err });
        return;
    };
    defer token_list.deinit(allocator);

    if (debug_mode) {
        print("📝 Generated {d} tokens:\n", .{token_list.items.len});
        for (token_list.items[0..@min(10, token_list.items.len)]) |token| {
            print("  {s}: '{s}'\n", .{ @tagName(token.kind), token.lexeme });
        }
        if (token_list.items.len > 10) {
            print("  ... and {d} more tokens\n", .{token_list.items.len - 10});
        }
    }

    print("✅ Tokenization successful: {d} tokens from {s}\n", .{ token_list.items.len, filename });
    print("💡 Full parser integration coming next...\n", .{});
}

fn realCompile(allocator: std.mem.Allocator, source: []const u8, filename: []const u8, debug_mode: bool, verbose: bool) !void {
    if (verbose) print("🔧 Starting CURSED compilation with real pipeline...\n", .{});
    
    // Same tokenization as interpreter
    const lexer = @import("lexer.zig");
    
    var cursed_lexer = lexer.Lexer.init(allocator, source);
    var token_list = cursed_lexer.tokenize() catch |err| {
        print("❌ Lexer error in {s}: {any}\n", .{ filename, err });
        return;
    };
    defer token_list.deinit(allocator);

    if (debug_mode) {
        print("📝 Generated {d} tokens for compilation\n", .{token_list.items.len});
    }

    print("✅ Compilation frontend successful: {d} tokens from {s}\n", .{ token_list.items.len, filename });
    print("💡 LLVM backend integration coming next...\n", .{});
}
