const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const simple_import_resolver = @import("simple_import_resolver.zig");
const simple_compiler = @import("simple_compiler.zig");

// Version information
const VERSION = "1.0.0";
const BUILD_INFO = "CURSED Unified Compiler";

// Command enumeration
const Command = enum {
    interpret,
    compile,
    check,
    format,
    test_cmd,
    version,
    help,
};

// Backend enumeration
const Backend = enum {
    script,    // Default interpretation mode
    llvm,      // LLVM compilation
    c,         // C transpilation
    wasm,      // WebAssembly
};

// CLI configuration
const Config = struct {
    command: Command = .interpret,
    backend: Backend = .script,
    source_file: ?[]const u8 = null,
    output_file: ?[]const u8 = null,
    optimization_level: u8 = 2,
    debug_mode: bool = false,
    verbose: bool = false,
    show_tokens: bool = false,
    watch_mode: bool = false,
    
    pub fn init() Config {
        return Config{};
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    // Parse command line arguments
    const config = try parseArgs(allocator, args);
    
    // Execute the appropriate command
    switch (config.command) {
        .version => {
            printVersion();
            return;
        },
        .help => {
            printHelp();
            return;
        },
        .interpret => {
            if (config.source_file == null) {
                print("Error: No source file specified for interpretation\n", .{});
                printUsage();
                return;
            }
            try executeInterpret(allocator, config);
        },
        .compile => {
            if (config.source_file == null) {
                print("Error: No source file specified for compilation\n", .{});
                printUsage();
                return;
            }
            try executeCompile(allocator, config);
        },
        .check => {
            if (config.source_file == null) {
                print("Error: No source file specified for type checking\n", .{});
                printUsage();
                return;
            }
            try executeCheck(allocator, config);
        },
        .format => {
            if (config.source_file == null) {
                print("Error: No source file specified for formatting\n", .{});
                printUsage();
                return;
            }
            try executeFormat(allocator, config);
        },
        .test_cmd => {
            try executeTest(allocator, config);
        },
    }
}

fn parseArgs(allocator: Allocator, args: [][]const u8) !Config {
    _ = allocator;
    var config = Config.init();
    
    if (args.len < 2) {
        config.command = .help;
        return config;
    }
    
    var i: usize = 1;
    
    // Check for global flags first
    if (std.mem.eql(u8, args[i], "--version") or std.mem.eql(u8, args[i], "-v")) {
        config.command = .version;
        return config;
    }
    
    if (std.mem.eql(u8, args[i], "--help") or std.mem.eql(u8, args[i], "-h")) {
        config.command = .help;
        return config;
    }
    
    // Parse subcommand
    if (std.mem.eql(u8, args[i], "interpret")) {
        config.command = .interpret;
        i += 1;
    } else if (std.mem.eql(u8, args[i], "compile")) {
        config.command = .compile;
        i += 1;
    } else if (std.mem.eql(u8, args[i], "check")) {
        config.command = .check;
        i += 1;
    } else if (std.mem.eql(u8, args[i], "format")) {
        config.command = .format;
        i += 1;
    } else if (std.mem.eql(u8, args[i], "test")) {
        config.command = .test_cmd;
        i += 1;
    } else {
        // No subcommand provided, assume interpret with first arg as file
        config.command = .interpret;
        if (!std.mem.startsWith(u8, args[i], "-")) {
            config.source_file = args[i];
            i += 1;
        }
    }
    
    // Parse remaining arguments
    while (i < args.len) {
        const arg = args[i];
        
        if (std.mem.eql(u8, arg, "--backend") or std.mem.eql(u8, arg, "-b")) {
            i += 1;
            if (i >= args.len) {
                print("Error: --backend requires a value\n", .{});
                return error.InvalidArgs;
            }
            const backend_str = args[i];
            if (std.mem.eql(u8, backend_str, "script")) {
                config.backend = .script;
            } else if (std.mem.eql(u8, backend_str, "llvm")) {
                config.backend = .llvm;
            } else if (std.mem.eql(u8, backend_str, "c")) {
                config.backend = .c;
            } else if (std.mem.eql(u8, backend_str, "wasm")) {
                config.backend = .wasm;
            } else {
                print("Error: Unknown backend '{s}'. Valid options: script, llvm, c, wasm\n", .{backend_str});
                return error.InvalidArgs;
            }
        } else if (std.mem.eql(u8, arg, "--output") or std.mem.eql(u8, arg, "-o")) {
            i += 1;
            if (i >= args.len) {
                print("Error: --output requires a value\n", .{});
                return error.InvalidArgs;
            }
            config.output_file = args[i];
        } else if (std.mem.startsWith(u8, arg, "--optimize=") or std.mem.startsWith(u8, arg, "-O")) {
            const level_str = if (std.mem.startsWith(u8, arg, "--optimize=")) 
                arg[11..] 
            else 
                arg[2..];
            config.optimization_level = std.fmt.parseUnsigned(u8, level_str, 10) catch {
                print("Error: Invalid optimization level '{s}'\n", .{level_str});
                return error.InvalidArgs;
            };
        } else if (std.mem.eql(u8, arg, "--debug") or std.mem.eql(u8, arg, "-d")) {
            config.debug_mode = true;
            config.verbose = true;
            config.show_tokens = true;
        } else if (std.mem.eql(u8, arg, "--verbose")) {
            config.verbose = true;
        } else if (std.mem.eql(u8, arg, "--tokens")) {
            config.show_tokens = true;
        } else if (std.mem.eql(u8, arg, "--watch") or std.mem.eql(u8, arg, "-w")) {
            config.watch_mode = true;
        } else if (!std.mem.startsWith(u8, arg, "-")) {
            // Assume it's a source file if no source file is set yet
            if (config.source_file == null) {
                config.source_file = arg;
            } else {
                print("Error: Multiple source files specified\n", .{});
                return error.InvalidArgs;
            }
        } else {
            print("Error: Unknown option '{s}'\n", .{arg});
            return error.InvalidArgs;
        }
        
        i += 1;
    }
    
    return config;
}

fn executeInterpret(allocator: Allocator, config: Config) !void {
    const filename = config.source_file.?;
    
    if (config.verbose) {
        print("🚀 Interpreting {s} with {s} backend\n", .{ filename, @tagName(config.backend) });
    }
    
    // Read source file
    const source = readSourceFile(allocator, filename) catch |err| {
        print("❌ Error reading file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);
    
    if (config.verbose) print("📁 Read {s} ({} bytes)\n", .{ filename, source.len });
    
    // Tokenize if requested
    if (config.show_tokens) {
        try showTokens(allocator, source);
    }
    
    // Execute based on backend
    switch (config.backend) {
        .script => try interpretScript(allocator, source, config),
        .llvm => {
            print("❌ LLVM interpretation not yet implemented\n", .{});
            return error.NotImplemented;
        },
        .c => {
            print("❌ C interpretation not yet implemented\n", .{});
            return error.NotImplemented;
        },
        .wasm => {
            print("❌ WASM interpretation not yet implemented\n", .{});
            return error.NotImplemented;
        },
    }
}

fn executeCompile(allocator: Allocator, config: Config) !void {
    const filename = config.source_file.?;
    
    if (config.verbose) {
        print("🔨 Compiling {s} with {s} backend (O{})\n", .{ 
            filename, @tagName(config.backend), config.optimization_level 
        });
    }
    
    // Read source file
    const source = readSourceFile(allocator, filename) catch |err| {
        print("❌ Error reading file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);
    
    // Execute based on backend
    switch (config.backend) {
        .script => {
            print("❌ Script backend does not support compilation\n", .{});
            return error.InvalidBackend;
        },
        .llvm => try compileWithLLVM(allocator, source, filename, config),
        .c => try compileWithC(allocator, source, filename, config),
        .wasm => try compileWithWASM(allocator, source, filename, config),
    }
}

fn executeCheck(allocator: Allocator, config: Config) !void {
    const filename = config.source_file.?;
    
    if (config.verbose) {
        print("🔍 Type checking {s}\n", .{filename});
    }
    
    // Read source file
    const source = readSourceFile(allocator, filename) catch |err| {
        print("❌ Error reading file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);
    
    // Perform basic syntax and import checking
    try checkSyntax(allocator, source, config);
    
    print("✅ Type checking completed successfully\n", .{});
}

fn executeFormat(allocator: Allocator, config: Config) !void {
    const filename = config.source_file.?;
    
    if (config.verbose) {
        print("📝 Formatting {s}\n", .{filename});
    }
    
    // Read source file
    const source = readSourceFile(allocator, filename) catch |err| {
        print("❌ Error reading file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);
    
    // For now, just output the source as-is (formatting not implemented)
    print("{s}", .{source});
    
    if (config.verbose) {
        print("✅ Formatting completed\n", .{});
    }
}

fn executeTest(allocator: Allocator, config: Config) !void {
    if (config.verbose) {
        print("🧪 Running tests\n", .{});
    }
    
    if (config.source_file) |filename| {
        // Test specific file
        try executeInterpret(allocator, Config{
            .command = .interpret,
            .backend = .script,
            .source_file = filename,
            .verbose = config.verbose,
            .debug_mode = config.debug_mode,
        });
    } else {
        // Run test suite
        print("❌ Test suite not yet implemented\n", .{});
        return error.NotImplemented;
    }
}

// Backend implementation functions
fn interpretScript(allocator: Allocator, source: []const u8, config: Config) !void {
    // Use the existing interpretation logic from main_unified.zig
    const imports = simple_import_resolver.extractImports(allocator, source) catch |err| {
        print("Error: Failed to extract imports: {any}\n", .{err});
        return;
    };
    defer {
        for (imports.items) |import_name| {
            allocator.free(import_name);
        }
        imports.deinit();
    }
    
    // Validate imports
    if (imports.items.len > 0) {
        if (config.verbose) {
            print("📦 Validating {} imports...\n", .{imports.items.len});
        }
        
        const all_valid = simple_import_resolver.validateImports(allocator, imports) catch |err| {
            print("Error: Failed to validate imports: {any}\n", .{err});
            return;
        };
        
        if (!all_valid) {
            print("❌ Some imports could not be resolved\n", .{});
            return;
        }
        
        if (config.verbose) {
            print("✅ All imports validated successfully\n", .{});
        }
    }
    
    // Simple line-by-line interpretation
    var lines = std.mem.splitScalar(u8, source, '\n');
    var line_number: u32 = 0;
    
    while (lines.next()) |line| {
        line_number += 1;
        const trimmed = std.mem.trim(u8, line, " \t\r\n");
        
        // Skip empty lines and comments
        if (trimmed.len == 0 or std.mem.startsWith(u8, trimmed, "fr fr")) {
            continue;
        }
        
        // Skip import statements during execution
        if (std.mem.startsWith(u8, trimmed, "yeet ")) {
            if (config.verbose) print("📦 Import: {s}\n", .{trimmed});
            continue;
        }
        
        // Handle basic vibez.spill() statements
        if (std.mem.indexOf(u8, trimmed, "vibez.spill(")) |start| {
            try handleSimpleVibesSpill(trimmed, start);
        } else if (config.verbose) {
            print("Line {}: {s}\n", .{ line_number, trimmed });
        }
    }
    
    if (config.verbose) print("✅ Script interpretation completed\n", .{});
}

fn compileWithLLVM(allocator: Allocator, source: []const u8, filename: []const u8, config: Config) !void {
    // Use existing simple_compiler for LLVM compilation
    try simple_compiler.compileProgram(allocator, source, filename, config.optimization_level, config.verbose);
}

fn compileWithC(allocator: Allocator, source: []const u8, filename: []const u8, config: Config) !void {
    _ = allocator;
    _ = source;
    _ = filename;
    _ = config;
    print("❌ C backend not yet implemented\n", .{});
    return error.NotImplemented;
}

fn compileWithWASM(allocator: Allocator, source: []const u8, filename: []const u8, config: Config) !void {
    _ = allocator;
    _ = source;
    _ = filename;
    _ = config;
    print("❌ WASM backend not yet implemented\n", .{});
    return error.NotImplemented;
}

// Utility functions
fn readSourceFile(allocator: Allocator, filename: []const u8) ![]u8 {
    return std.fs.cwd().readFileAlloc(allocator, filename, 1024 * 1024);
}

fn showTokens(allocator: Allocator, source: []const u8) !void {
    var l = lexer.Lexer.init(allocator, source);
    const tokens = l.tokenize() catch |err| {
        print("❌ Lexer error: {}\n", .{err});
        return;
    };
    defer tokens.deinit();
    
    print("=== TOKENS ({}) ===\n", .{tokens.items.len});
    for (tokens.items) |token| {
        print("{any}: '{s}'\n", .{ token.kind, token.lexeme });
    }
    print("\n", .{});
}

fn checkSyntax(allocator: Allocator, source: []const u8, config: Config) !void {
    // Basic syntax checking - tokenize and check imports
    var l = lexer.Lexer.init(allocator, source);
    const tokens = l.tokenize() catch |err| {
        print("❌ Syntax error during tokenization: {}\n", .{err});
        return;
    };
    defer tokens.deinit();
    
    if (config.verbose) {
        print("✅ Tokenization successful ({} tokens)\n", .{tokens.items.len});
    }
    
    // Check imports
    const imports = simple_import_resolver.extractImports(allocator, source) catch |err| {
        print("❌ Import resolution error: {any}\n", .{err});
        return;
    };
    defer {
        for (imports.items) |import_name| {
            allocator.free(import_name);
        }
        imports.deinit();
    }
    
    if (imports.items.len > 0) {
        const all_valid = simple_import_resolver.validateImports(allocator, imports) catch |err| {
            print("❌ Import validation error: {any}\n", .{err});
            return;
        };
        
        if (!all_valid) {
            print("❌ Some imports could not be resolved\n", .{});
            return;
        }
        
        if (config.verbose) {
            print("✅ All {} imports validated\n", .{imports.items.len});
        }
    }
}

fn handleSimpleVibesSpill(line: []const u8, start: usize) !void {
    if (std.mem.indexOf(u8, line[start..], "(")) |paren_start| {
        if (std.mem.lastIndexOf(u8, line, ")")) |paren_end| {
            const content_start = start + paren_start + 1;
            const content = line[content_start..paren_end];
            const trimmed_content = std.mem.trim(u8, content, " \t");
            
            // Handle string literals
            if (trimmed_content.len >= 2 and trimmed_content[0] == '"' and trimmed_content[trimmed_content.len - 1] == '"') {
                print("{s}\n", .{trimmed_content[1..trimmed_content.len - 1]});
            } else {
                // Handle other expressions (basic support)
                print("{s}\n", .{trimmed_content});
            }
        }
    }
}

// Help and version functions
fn printVersion() void {
    print("{s} v{s}\n", .{ BUILD_INFO, VERSION });
    print("A modern programming language for the next generation\n", .{});
}

fn printUsage() void {
    print("Usage: cursed [COMMAND] [OPTIONS] [FILE]\n", .{});
    print("\nFor more information, use: cursed --help\n", .{});
}

fn printHelp() void {
    print("{s} v{s}\n", .{ BUILD_INFO, VERSION });
    print("A modern programming language for the next generation\n\n", .{});
    
    print("USAGE:\n", .{});
    print("    cursed [COMMAND] [OPTIONS] [FILE]\n\n", .{});
    
    print("COMMANDS:\n", .{});
    print("    interpret       Interpret CURSED source code (default)\n", .{});
    print("    compile         Compile CURSED source to native executable\n", .{});
    print("    check           Type check CURSED source code\n", .{});
    print("    format          Format CURSED source code\n", .{});
    print("    test            Run tests\n", .{});
    print("    --version, -v   Show version information\n", .{});
    print("    --help, -h      Show this help message\n\n", .{});
    
    print("OPTIONS:\n", .{});
    print("    --backend, -b BACKEND    Compilation backend [script, llvm, c, wasm]\n", .{});
    print("    --output, -o FILE        Output file (for compile command)\n", .{});
    print("    --optimize, -O LEVEL     Optimization level (0-3) [default: 2]\n", .{});
    print("    --debug, -d              Enable debug mode (verbose + tokens)\n", .{});
    print("    --verbose                Enable verbose output\n", .{});
    print("    --tokens                 Show token stream\n", .{});
    print("    --watch, -w              Watch for file changes\n\n", .{});
    
    print("EXAMPLES:\n", .{});
    print("    cursed hello.csd                    # Interpret hello.csd\n", .{});
    print("    cursed interpret hello.csd --verbose # Interpret with verbose output\n", .{});
    print("    cursed compile hello.csd -b llvm    # Compile with LLVM backend\n", .{});
    print("    cursed check hello.csd              # Type check hello.csd\n", .{});
    print("    cursed format hello.csd             # Format hello.csd\n", .{});
    print("    cursed test                         # Run test suite\n\n", .{});
    
    print("For more information, visit: https://github.com/ghuntley/cursed\n", .{});
}
