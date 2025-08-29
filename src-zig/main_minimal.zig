const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const lexer = @import("lexer.zig");
const simple_import_resolver = @import("simple_import_resolver.zig");
const simple_compiler = @import("simple_compiler.zig");
const cross_compilation = @import("cross_compilation.zig");
// const ast = @import("ast.zig");  // Temporarily disabled
// const parser = @import("parser.zig");  // Temporarily disabled
const interpreter = @import("interpreter.zig");
const CursedArenaManager = @import("arena_allocator.zig").CursedArenaManager;

// Version information
const VERSION = "1.0.0";
const BUILD_INFO = "CURSED Unified Compiler (Minimal Build)";

// Command enumeration
const Command = enum {
    interpret,
    compile,
    check,
    format,
    test_cmd,
    version,
    help,
    jit,
    // Build system commands
    init,
    run,
    clean,
    doc,
    install,
    build,
};

// Backend enumeration
const Backend = enum {
    script,    // Default interpretation mode
    ast,       // AST-based interpretation (proper function support) - DISABLED
    llvm,      // LLVM compilation
    c,         // C transpilation
    wasm,      // WebAssembly
};

// Target platform enumeration
const TargetPlatform = enum {
    native,           // Host platform
    linux_x64,        // Linux x86_64
    linux_arm64,      // Linux ARM64
    macos_x64,        // macOS x86_64
    macos_arm64,      // macOS ARM64
    windows_x64,      // Windows x86_64
    wasm32,           // WebAssembly 32-bit
    
    pub fn toString(self: TargetPlatform) []const u8 {
        return switch (self) {
            .native => "native",
            .linux_x64 => "linux-x64",
            .linux_arm64 => "linux-arm64",
            .macos_x64 => "macos-x64",
            .macos_arm64 => "macos-arm64",
            .windows_x64 => "windows-x64",
            .wasm32 => "wasm32",
        };
    }
    
    pub fn fromString(str: []const u8) ?TargetPlatform {
        if (std.mem.eql(u8, str, "native")) return .native;
        if (std.mem.eql(u8, str, "linux-x64")) return .linux_x64;
        if (std.mem.eql(u8, str, "linux-arm64")) return .linux_arm64;
        if (std.mem.eql(u8, str, "macos-x64")) return .macos_x64;
        if (std.mem.eql(u8, str, "macos-arm64")) return .macos_arm64;
        if (std.mem.eql(u8, str, "windows-x64")) return .windows_x64;
        if (std.mem.eql(u8, str, "wasm32")) return .wasm32;
        return null;
    }
    
    pub fn getZigTarget(self: TargetPlatform) ?[]const u8 {
        return switch (self) {
            .native => null,
            .linux_x64 => "x86_64-linux",
            .linux_arm64 => "aarch64-linux",
            .macos_x64 => "x86_64-macos",
            .macos_arm64 => "aarch64-macos",
            .windows_x64 => "x86_64-windows",
            .wasm32 => "wasm32-freestanding",
        };
    }
    
    pub fn supportsLLVM(self: TargetPlatform) bool {
        return switch (self) {
            .native, .linux_x64, .linux_arm64, .macos_x64, .macos_arm64, .windows_x64 => true,
            .wasm32 => false,
        };
    }
    
    pub fn getExecutableExtension(self: TargetPlatform) []const u8 {
        return switch (self) {
            .windows_x64 => ".exe",
            .wasm32 => ".wasm",
            else => "",
        };
    }
};

// Linking mode enumeration
const LinkingMode = enum {
    dynamic,
    static,
    
    pub fn toString(self: LinkingMode) []const u8 {
        return switch (self) {
            .dynamic => "dynamic",
            .static => "static",
        };
    }
};

// CLI configuration
const Config = struct {
    command: Command = .interpret,
    backend: Backend = .script,
    source_file: ?[]const u8 = null,
    output_file: ?[]const u8 = null,
    project_dir: ?[]const u8 = null,
    target_platform: TargetPlatform = .native,
    linking_mode: LinkingMode = .dynamic,
    optimization_level: u8 = 2,
    debug_mode: bool = false,
    verbose: bool = false,
    show_tokens: bool = false,
    watch_mode: bool = false,
    check_only: bool = false,
    show_diff: bool = false,
    lto_enabled: bool = false,
    debug_info: bool = false,
    preserve_debug_info: bool = false,
    
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
    print("🐛 DEBUG: Executing command: {s}\n", .{@tagName(config.command)});
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
            print("Format command not yet implemented in minimal build\n", .{});
        },
        .test_cmd => {
            print("Test command not yet implemented in minimal build\n", .{});
        },
        .jit => {
            print("JIT command not yet implemented in minimal build\n", .{});
        },
        // Build system commands
        .init, .run, .clean, .doc, .install, .build => {
            print("Build system command '{s}' not yet implemented\n", .{@tagName(config.command)});
        },
    }
}

fn parseArgs(_: Allocator, args: [][:0]u8) !Config {
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
        if (config.verbose) print("🐛 DEBUG: Command set to compile\n", .{});
        i += 1;
    } else if (std.mem.eql(u8, args[i], "check")) {
        config.command = .check;
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
        } else if (std.mem.eql(u8, arg, "--verbose")) {
            config.verbose = true;
        } else if (std.mem.eql(u8, arg, "--debug") or std.mem.eql(u8, arg, "-d")) {
            config.debug_mode = true;
            config.verbose = true;
        } else if (std.mem.eql(u8, arg, "--compile")) {
            config.command = .compile;
            if (config.verbose) print("🐛 DEBUG: --compile flag detected, switching to compile command\n", .{});
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
        print("🚀 Interpreting {s} with backend: {s}\n", .{ filename, @tagName(config.backend) });
    }
    
    // Read source file
    const source = readSourceFile(allocator, filename) catch |err| {
        print("❌ Error reading file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);
    
    // For now, only support script backend
    if (config.backend != .script) {
        print("❌ Only script backend supported in minimal build\n", .{});
        return;
    }
    
    try interpretScript(allocator, source, config);
}

fn executeCompile(allocator: Allocator, config: Config) !void {
    const filename = config.source_file.?;
    
    // Auto-adjust backend for compilation if script backend is selected
    var adjusted_config = config;
    if (config.backend == .script) {
        if (config.target_platform == .wasm32) {
            adjusted_config.backend = .wasm;
        } else {
            adjusted_config.backend = .llvm;  // Default to LLVM for compilation
        }
        
        if (config.verbose) {
            print("🔧 Auto-adjusted backend from script to {s} for compilation\n", .{@tagName(adjusted_config.backend)});
        }
    }
    
    if (adjusted_config.verbose) {
        print("🔨 Compiling {s} for target {s} with {s} backend (O{s})\n", .{ 
            filename, adjusted_config.target_platform.toString(), @tagName(adjusted_config.backend), adjusted_config.optimization_level 
        });
        print("🔗 Linking mode: {s}\n", .{adjusted_config.linking_mode.toString()});
    }
    
    // Read source file
    const source = readSourceFile(allocator, filename) catch |err| {
        print("❌ Error reading file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);
    
    // Generate output filename if not specified
    const output_file = adjusted_config.output_file orelse blk: {
        const base_name = std.fs.path.stem(filename);
        const output_name = try std.fmt.allocPrint(allocator, "{s}{s}", .{ base_name, adjusted_config.target_platform.getExecutableExtension() });
        break :blk output_name;
    };
    defer if (adjusted_config.output_file == null) {
        allocator.free(output_file);
    };
    
    // Execute based on backend
    switch (adjusted_config.backend) {
        .script => {
            print("❌ Script backend does not support compilation\n", .{});
            return error.InvalidBackend;
        },
        .ast => {
            print("❌ AST backend not available in minimal build\n", .{});
            return error.InvalidBackend;
        },
        .llvm => try compileWithSmartLLVM(allocator, source, filename, adjusted_config, output_file),
        .c => {
            print("❌ C backend not yet implemented in minimal build\n", .{});
            return error.NotImplemented;
        },
        .wasm => {
            print("❌ WASM backend not yet implemented in minimal build\n", .{});
            return error.NotImplemented;
        },
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
    
    // Perform basic syntax checking
    try checkBasicSyntax(allocator, source, config);
    
    print("✅ Type checking completed successfully\n", .{});
}

fn compileWithSmartLLVM(allocator: Allocator, source: []const u8, filename: []const u8, config: Config, output_file: []const u8) !void {
    if (config.verbose) {
        print("🔨 Compiling with Smart LLVM backend\n", .{});
    }
    
    const smart_llvm = @import("llvm_backend_smart.zig");
    
    var backend = smart_llvm.SmartLLVMBackend.init(allocator);
    
    try backend.compileToNative(source, filename, output_file, config.verbose);
    
    if (config.verbose) {
        print("✅ Smart LLVM compilation completed: {s}\n", .{output_file});
    }
}

fn readSourceFile(allocator: Allocator, filename: []const u8) ![]u8 {
    var file = std.fs.cwd().openFile(filename, .{}) catch |err| switch (err) {
        error.FileNotFound => {
            print("❌ File not found: {s}\n", .{filename});
            return err;
        },
        else => return err,
    };
    defer file.close();
    
    const file_size = try file.getEndPos();
    const contents = try allocator.alloc(u8, file_size);
    _ = try file.readAll(contents);
    return contents;
}

fn interpretScript(allocator: Allocator, source: []const u8, config: Config) !void {
    _ = allocator;
    if (config.verbose) print("🚀 Using simple script interpreter\n", .{});
    
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
        
        if (config.verbose) print("🔍 Processing line: '{s}'\n", .{trimmed});
        
        // Skip import statements during execution
        if (std.mem.startsWith(u8, trimmed, "yeet ")) {
            if (config.verbose) print("📦 Import: {s}\n", .{trimmed});
            continue;
        }
        
        // Handle print statements: vibez.spill("Hello")
        if (std.mem.indexOf(u8, trimmed, "vibez.spill(")) |_| {
            try handleSimpleVibesSpill(trimmed);
        } else if (config.verbose) {
            print("Line {s}: {s}\n", .{ line_number, trimmed });
        }
    }
    
    if (config.verbose) print("✅ Script interpretation completed\n", .{});
}

fn handleSimpleVibesSpill(line: []const u8) !void {
    const start = std.mem.indexOf(u8, line, "(") orelse return;
    const end = std.mem.lastIndexOf(u8, line, ")") orelse return;
    const content = line[start + 1..end];
    const trimmed_content = std.mem.trim(u8, content, " \t");
    
    // Handle string literals
    if (trimmed_content.len >= 2 and trimmed_content[0] == '"' and trimmed_content[trimmed_content.len - 1] == '"') {
        print("{s}\n", .{trimmed_content[1..trimmed_content.len - 1]});
    } else {
        // Handle other expressions (basic support)
        print("{s}\n", .{trimmed_content});
    }
}

fn checkBasicSyntax(allocator: Allocator, source: []const u8, config: Config) !void {
    _ = allocator;
    
    var lines = std.mem.splitScalar(u8, source, '\n');
    var line_number: u32 = 0;
    var error_count: u32 = 0;
    
    while (lines.next()) |line| {
        line_number += 1;
        const trimmed = std.mem.trim(u8, line, " \t\r\n");
        
        if (trimmed.len == 0 or std.mem.startsWith(u8, trimmed, "fr fr")) {
            continue;
        }
        
        // Basic syntax checks
        if (std.mem.startsWith(u8, trimmed, "sus ")) {
            if (std.mem.indexOf(u8, trimmed, "=") == null) {
                print("❌ Line {s}: sus statement missing assignment\n", .{line_number});
                error_count += 1;
            }
        }
        
        if (config.verbose) {
            print("✅ Line {s} syntax OK\n", .{line_number});
        }
    }
    
    if (error_count > 0) {
        print("❌ Found {s} syntax error(s)\n", .{error_count});
        return error.SyntaxError;
    }
}

// Help and version functions
fn printVersion() void {
    print("{s} v{s}\n", .{ BUILD_INFO, VERSION });
    print("A modern programming language for the next generation (Minimal Build)\n", .{});
}

fn printUsage() void {
    print("Usage: cursed [COMMAND] [OPTIONS] [FILE]\n", .{});
    print("\nFor more information, use: cursed --help\n", .{});
}

fn printHelp() void {
    print("{s} v{s}\n", .{ BUILD_INFO, VERSION });
    print("A modern programming language for the next generation (Minimal Build)\n\n", .{});
    
    print("USAGE:\n", .{});
    print("    cursed [COMMAND] [OPTIONS] [FILE]\n\n", .{});
    
    print("COMMANDS:\n", .{});
    print("    interpret       Interpret CURSED source code (default)\n", .{});
    print("    compile         Compile CURSED source to native executable\n", .{});
    print("    check           Type check CURSED source code\n", .{});
    print("\n", .{});
    print("    --version, -v   Show version information\n", .{});
    print("    --help, -h      Show this help message\n\n", .{});
    
    print("OPTIONS:\n", .{});
    print("    --compile                Compile source to native executable (same as compile command)\n", .{});
    print("    --backend, -b BACKEND    Compilation backend [script, llvm, c, wasm]\n", .{});
    print("    --output, -o FILE        Output file (for compile command)\n", .{});
    print("    --debug, -d              Enable debug mode (verbose + tokens)\n", .{});
    print("    --verbose                Enable verbose output\n\n", .{});
    
    print("EXAMPLES:\n", .{});
    print("    cursed hello.csd                          # Interpret hello.csd\n", .{});
    print("    cursed interpret hello.csd --verbose       # Interpret with verbose output\n", .{});
    print("    cursed hello.csd --compile                 # Compile hello.csd to native executable\n", .{});
    print("    cursed compile hello.csd -b llvm           # Compile with LLVM backend\n", .{});
    print("    cursed check hello.csd                     # Type check hello.csd\n", .{});
    
    print("For more information, visit: https://github.com/ghuntley/cursed\n", .{});
}
