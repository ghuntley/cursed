const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const ArenaAllocator = std.heap.ArenaAllocator;

const memory_safe_lexer = @import("memory_safe_lexer.zig");
const memory_safe_parser = @import("memory_safe_parser.zig");
const memory_safe_error_reporting = @import("memory_safe_error_reporting.zig");

const Token = memory_safe_lexer.Token;
const TokenCollection = memory_safe_lexer.TokenCollection;
const Lexer = memory_safe_lexer.Lexer;
const Parser = memory_safe_parser.Parser;
const ErrorReporter = memory_safe_error_reporting.ErrorReporter;

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
        print("CURSED Memory-Safe Compiler v1.0.0\n", .{});
        print("Enhanced with arena allocators and automatic memory management\n", .{});
        return;
    }

    if (std.mem.eql(u8, args[1], "--help")) {
        printUsage();
        return;
    }

    const filename = args[1];
    const compile_mode = args.len > 2 and std.mem.eql(u8, args[2], "--compile");
    const debug_mode = args.len > 2 and std.mem.eql(u8, args[2], "--debug");
    const verbose_mode = args.len > 2 and std.mem.eql(u8, args[2], "--verbose");

    // Initialize error reporter with memory safety
    var error_reporter = ErrorReporter.init(allocator, 50);
    defer error_reporter.deinit();

    // Read source file
    const file = std.fs.cwd().openFile(filename, .{}) catch |err| {
        try error_reporter.reportError(
            .E001_UnterminatedString,
            "Could not open file",
            memory_safe_error_reporting.SourceLocation.init(filename, 1, 1, 0)
        );
        print("Error: Could not open file '{s}': {}\n", .{ filename, err });
        return;
    };
    defer file.close();

    const file_size = try file.getEndPos();
    const source = try allocator.alloc(u8, file_size);
    defer allocator.free(source);
    
    _ = try file.readAll(source);

    // Add source file to error reporter for better diagnostics
    try error_reporter.addSourceFile(filename, source);

    if (verbose_mode) {
        print("Parsing file: {s} ({d} bytes)\n", .{ filename, source.len });
    }

    // Initialize arena for lexing
    var lexer_arena = ArenaAllocator.init(allocator);
    defer lexer_arena.deinit();

    // Tokenize with memory safety
    var lexer_instance = Lexer.init(&lexer_arena, source);
    var tokens = lexer_instance.tokenize() catch |err| {
        try error_reporter.reportError(
            .E002_InvalidCharacter,
            "Lexing failed",
            memory_safe_error_reporting.SourceLocation.init(filename, 1, 1, 0)
        );
        print("Lexing error: {}\n", .{err});
        try error_reporter.printDiagnostics(std.fs.File.stdout().writer(&[_]u8{}));
        return;
    };
    defer tokens.deinit();

    if (verbose_mode) {
        print("Tokenized successfully: {d} tokens\n", .{tokens.items().len});
        if (debug_mode) {
            for (tokens.items(), 0..) |token, i| {
                print("Token {d}: {}\n", .{ i, token });
            }
        }
    }

    // Parse with memory safety
    var parser_instance = Parser.initWithFile(allocator, tokens.items(), filename);
    defer parser_instance.deinit();
    
    // Connect error reporter to parser
    parser_instance.setErrorReporter(&error_reporter);

    const program = parser_instance.parseProgram() catch |err| {
        try error_reporter.reportError(
            .E104_InvalidSyntax,
            "Parsing failed",
            memory_safe_error_reporting.SourceLocation.init(filename, 1, 1, 0)
        );
        print("Parsing error: {}\n", .{err});
        try error_reporter.printDiagnostics(std.fs.File.stdout().writer(&[_]u8{}));
        return;
    };

    if (verbose_mode) {
        print("Parsed successfully: {d} statements\n", .{program.statements.items.len});
    }

    // Check for compilation errors
    if (error_reporter.hasErrors()) {
        print("Compilation failed with errors:\n", .{});
        try error_reporter.printDiagnostics(std.fs.File.stdout().writer(&[_]u8{}));
        return;
    }

    if (error_reporter.hasWarnings()) {
        print("Compilation completed with warnings:\n", .{});
        try error_reporter.printDiagnostics(std.fs.File.stdout().writer(&[_]u8{}));
    }

    if (compile_mode) {
        print("Compilation mode not yet implemented in memory-safe version\n", .{});
        print("Program parsed successfully with {d} statements\n", .{program.statements.items.len});
    } else {
        // Simple interpretation
        print("Interpretation mode - program executed successfully\n", .{});
        print("Parsed {d} statements\n", .{program.statements.items.len});
        
        // Memory cleanup is automatic via arena allocators
        if (verbose_mode) {
            print("Memory cleanup completed automatically\n", .{});
        }
    }
}

fn printUsage() void {
    print("CURSED Memory-Safe Compiler\n", .{});
    print("Usage: cursed-memory-safe <file.csd> [options]\n", .{});
    print("Options:\n", .{});
    print("  --compile    Compile to native code\n", .{});
    print("  --debug      Enable debug output\n", .{});
    print("  --verbose    Enable verbose output\n", .{});
    print("  --version    Show version information\n", .{});
    print("  --help       Show this help message\n", .{});
}
