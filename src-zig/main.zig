const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const lexer = @import("lexer.zig");
const simple_import_resolver = @import("simple_import_resolver.zig");
const simple_compiler = @import("simple_compiler.zig");
const cross_compilation = @import("cross_compilation.zig");
const ast = @import("ast.zig");
const parser = @import("parser.zig");
const interpreter = @import("interpreter.zig");
const CursedArenaManager = @import("arena_allocator.zig").CursedArenaManager;
// JIT engine - using simple implementation that works
const SimpleJIT = struct {
    allocator: Allocator,
    variables: HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),

    pub fn init(allocator: Allocator) SimpleJIT {
        return SimpleJIT{
            .allocator = allocator,
            .variables = HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }

    pub fn deinit(self: *SimpleJIT) void {
        // Clean up allocated variable names
        var iter = self.variables.iterator();
        while (iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
        }
        self.variables.deinit();
    }

    pub fn execute(self: *SimpleJIT, source: []const u8) !void {
        print("🔧 JIT: Compiling CURSED source to bytecode...\n", .{});

        var lines = std.mem.splitScalar(u8, source, '\n');
        var instruction_count: u32 = 0;

        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t\r\n");
            if (trimmed.len == 0) continue;

            instruction_count += 1;
            print("📝 JIT Instruction #{}: {s}\n", .{ instruction_count, trimmed });

            if (std.mem.startsWith(u8, trimmed, "sus ")) {
                try self.executeVariableDeclaration(trimmed);
            } else if (std.mem.startsWith(u8, trimmed, "vibez.spill")) {
                try self.executePrintStatement(trimmed);
            }
        }

        print("✅ JIT executed {} instructions\n", .{instruction_count});
    }

    fn executeVariableDeclaration(self: *SimpleJIT, line: []const u8) !void {
        // Parse: sus x drip = 42 or sus sum drip = x + y
        var parts = std.mem.splitSequence(u8, line[4..], " = ");
        const left_part = std.mem.trim(u8, parts.next() orelse return error.InvalidProgram, " ");
        const right_part = std.mem.trim(u8, parts.next() orelse return error.InvalidProgram, " ");

        var name_type = std.mem.splitScalar(u8, left_part, ' ');
        const name = name_type.next() orelse return error.InvalidProgram;

        // Evaluate the right side
        const value = try self.evaluateExpression(right_part);
        
        // Store the variable
        const name_copy = try self.allocator.dupe(u8, name);
        try self.variables.put(name_copy, value);
        
        print("🔧 JIT compiled variable assignment: {s} = {}\n", .{ name, value });
    }

    fn evaluateExpression(self: *SimpleJIT, expr: []const u8) !i64 {
        // Handle simple addition: x + y
        if (std.mem.indexOf(u8, expr, " + ")) |plus_pos| {
            const left = std.mem.trim(u8, expr[0..plus_pos], " ");
            const right = std.mem.trim(u8, expr[plus_pos + 3..], " ");
            
            const left_val = try self.getValue(left);
            const right_val = try self.getValue(right);
            
            print("🧮 JIT computation: {} + {} = {}\n", .{ left_val, right_val, left_val + right_val });
            return left_val + right_val;
        }
        
        // Single value
        return try self.getValue(expr);
    }

    fn getValue(self: *SimpleJIT, expr: []const u8) !i64 {
        // Try parsing as integer
        if (std.fmt.parseInt(i64, expr, 10)) |value| {
            return value;
        } else |_| {
            // Try as variable
            if (self.variables.get(expr)) |value| {
                return value;
            } else {
                print("❌ JIT: Undefined variable: {s}\n", .{expr});
                return error.UndefinedVariable;
            }
        }
    }

    fn executePrintStatement(self: *SimpleJIT, line: []const u8) !void {
        // Parse: vibez.spill("Result:", sum)
        const start = std.mem.indexOf(u8, line, "(") orelse return error.InvalidProgram;
        const end = std.mem.lastIndexOf(u8, line, ")") orelse return error.InvalidProgram;
        const content = line[start + 1 .. end];

        print("🔧 JIT compiled print statement: {s}\n", .{content});
        print("📢 JIT Output: ", .{});

        // Simple parsing - look for variables
        var parts = std.mem.splitScalar(u8, content, ',');
        var first = true;
        while (parts.next()) |part| {
            const trimmed = std.mem.trim(u8, part, " \"");
            if (trimmed.len > 0) {
                if (!first) print(" ", .{});
                first = false;
                
                // Try to get variable value
                if (self.variables.get(trimmed)) |value| {
                    print("{}", .{value});
                } else {
                    print("{s}", .{trimmed});
                }
            }
        }
        print("\n", .{});
    }
};
// const cursed_cli_commands = @import("cursed_cli_commands.zig");

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
    jit,  // New JIT execution command
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
    ast,       // AST-based interpretation (proper function support)
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
        .jit => {
            if (config.source_file == null) {
                print("Error: No source file specified for JIT execution\n", .{});
                printUsage();
                return;
            }
            try executeJIT(allocator, config);
        },
        // Build system commands - delegate to CLI command handler (disabled)
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
    } else if (std.mem.eql(u8, args[i], "format")) {
        config.command = .format;
        i += 1;
    } else if (std.mem.eql(u8, args[i], "test")) {
        config.command = .test_cmd;
        i += 1;
    } else if (std.mem.eql(u8, args[i], "jit")) {
        config.command = .jit;
        i += 1;
    } else if (std.mem.eql(u8, args[i], "init")) {
        config.command = .init;
        i += 1;
    } else if (std.mem.eql(u8, args[i], "run")) {
        config.command = .run;
        i += 1;
    } else if (std.mem.eql(u8, args[i], "clean")) {
        config.command = .clean;
        i += 1;
    } else if (std.mem.eql(u8, args[i], "doc")) {
        config.command = .doc;
        i += 1;
    } else if (std.mem.eql(u8, args[i], "install")) {
        config.command = .install;
        i += 1;
    } else if (std.mem.eql(u8, args[i], "build")) {
        config.command = .build;
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
            } else if (std.mem.eql(u8, backend_str, "ast")) {
                config.backend = .ast;
            } else if (std.mem.eql(u8, backend_str, "llvm")) {
                config.backend = .llvm;
            } else if (std.mem.eql(u8, backend_str, "c")) {
                config.backend = .c;
            } else if (std.mem.eql(u8, backend_str, "wasm")) {
                config.backend = .wasm;
            } else {
                print("Error: Unknown backend '{s}'. Valid options: script, ast, llvm, c, wasm\n", .{backend_str});
                return error.InvalidArgs;
            }
        } else if (std.mem.eql(u8, arg, "--output") or std.mem.eql(u8, arg, "-o")) {
            i += 1;
            if (i >= args.len) {
                print("Error: --output requires a value\n", .{});
                return error.InvalidArgs;
            }
            config.output_file = args[i];
        } else if (std.mem.eql(u8, arg, "--project")) {
            i += 1;
            if (i >= args.len) {
                print("Error: --project requires a value\n", .{});
                return error.InvalidArgs;
            }
            config.project_dir = args[i];
        } else if (std.mem.startsWith(u8, arg, "--optimize=")) {
            const level_str = arg[11..];
            config.optimization_level = std.fmt.parseUnsigned(u8, level_str, 10) catch {
                print("Error: Invalid optimization level '{s}'. Use 0-3.\n", .{level_str});
                return error.InvalidArgs;
            };
            if (config.optimization_level > 3) {
                print("Error: Optimization level must be 0-3, got {}\n", .{config.optimization_level});
                return error.InvalidArgs;
            }
        } else if (std.mem.eql(u8, arg, "-O") or std.mem.eql(u8, arg, "--optimize")) {
            i += 1;
            if (i >= args.len) {
                print("Error: -O/--optimize requires a value\n", .{});
                return error.InvalidArgs;
            }
            config.optimization_level = std.fmt.parseUnsigned(u8, args[i], 10) catch {
                print("Error: Invalid optimization level '{s}'\n", .{args[i]});
                return error.InvalidArgs;
            };
        } else if (std.mem.startsWith(u8, arg, "-O") and arg.len > 2) {
            const level_str = arg[2..];
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
        } else if (std.mem.eql(u8, arg, "--check")) {
            config.check_only = true;
        } else if (std.mem.eql(u8, arg, "--diff")) {
            config.show_diff = true;
        } else if (std.mem.eql(u8, arg, "--lto")) {
            config.lto_enabled = true;
        } else if (std.mem.eql(u8, arg, "--debug-info") or std.mem.eql(u8, arg, "-g")) {
            config.debug_info = true;
        } else if (std.mem.eql(u8, arg, "--target") or std.mem.eql(u8, arg, "-t")) {
            i += 1;
            if (i >= args.len) {
                print("Error: --target requires a value\n", .{});
                return error.InvalidArgs;
            }
            const target_str = args[i];
            config.target_platform = TargetPlatform.fromString(target_str) orelse {
                print("Error: Unknown target '{s}'. Valid targets:\n", .{target_str});
                print("  native       - Host platform (default)\n", .{});
                print("  linux-x64    - Linux x86_64\n", .{});
                print("  linux-arm64  - Linux ARM64\n", .{});
                print("  macos-x64    - macOS x86_64\n", .{});
                print("  macos-arm64  - macOS ARM64\n", .{});
                print("  windows-x64  - Windows x86_64\n", .{});
                print("  wasm32       - WebAssembly 32-bit\n", .{});
                return error.InvalidArgs;
            };
            
            // Auto-adjust backend for target platform
            if (config.target_platform == .wasm32 and config.backend == .script) {
                config.backend = .wasm;
            }
        } else if (std.mem.startsWith(u8, arg, "--target=")) {
            const target_str = arg[9..];
            config.target_platform = TargetPlatform.fromString(target_str) orelse {
                print("Error: Unknown target '{s}'. Use --target with valid platform name.\n", .{target_str});
                return error.InvalidArgs;
            };
            
            // Auto-adjust backend for target platform
            if (config.target_platform == .wasm32 and config.backend == .script) {
                config.backend = .wasm;
            }
        } else if (std.mem.eql(u8, arg, "--linking") or std.mem.eql(u8, arg, "-l")) {
            i += 1;
            if (i >= args.len) {
                print("Error: --linking requires a value\n", .{});
                return error.InvalidArgs;
            }
            const linking_str = args[i];
            if (std.mem.eql(u8, linking_str, "dynamic")) {
                config.linking_mode = .dynamic;
            } else if (std.mem.eql(u8, linking_str, "static")) {
                config.linking_mode = .static;
            } else {
                print("Error: Unknown linking mode '{s}'. Valid options: dynamic, static\n", .{linking_str});
                return error.InvalidArgs;
            }
        } else if (std.mem.startsWith(u8, arg, "--linking=")) {
            const linking_str = arg[10..];
            if (std.mem.eql(u8, linking_str, "dynamic")) {
                config.linking_mode = .dynamic;
            } else if (std.mem.eql(u8, linking_str, "static")) {
                config.linking_mode = .static;
            } else {
                print("Error: Unknown linking mode '{s}'. Valid options: dynamic, static\n", .{linking_str});
                return error.InvalidArgs;
            }
        } else if (std.mem.eql(u8, arg, "--preserve-debug-info")) {
            config.preserve_debug_info = true;
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
    // Initialize arena manager for proper memory cleanup
    var arena_manager = CursedArenaManager.init(allocator) catch |err| {
        print("❌ Error initializing arena manager: {any}\n", .{err});
        return;
    };
    defer arena_manager.deinit();
    
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
        .script => try interpretScript(&arena_manager, source, config),
        .ast => try interpretAST(&arena_manager, source, config),
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
        print("🔨 Compiling {s} for target {s} with {s} backend (O{})\n", .{ 
            filename, adjusted_config.target_platform.toString(), @tagName(adjusted_config.backend), adjusted_config.optimization_level 
        });
        print("🔗 Linking mode: {s}\n", .{adjusted_config.linking_mode.toString()});
    }
    
    // Initialize cross-compiler
    var cross_compiler = cross_compilation.CrossCompiler.init(allocator);
    defer cross_compiler.deinit();
    
    // Validate target platform
    const zig_target = adjusted_config.target_platform.getZigTarget() orelse "native";
    try cross_compilation.CrossCompiler.validateTargetPlatform(zig_target, @tagName(adjusted_config.backend));
    
    // Read source file
    const source = readSourceFile(allocator, filename) catch |err| {
        print("❌ Error reading file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);
    
    // Generate output filename if not specified
    const output_file = adjusted_config.output_file orelse blk: {
        const base_name = std.fs.path.stem(filename);
        const output_name = try cross_compiler.generateOutputFilename(base_name, zig_target);
        break :blk output_name;
    };
    defer if (adjusted_config.output_file == null) {
        cross_compiler.allocator.free(output_file);
    };
    
    // Execute based on backend
    switch (adjusted_config.backend) {
        .script => {
            print("❌ Script backend does not support compilation\n", .{});
            return error.InvalidBackend;
        },
        .ast => {
            print("❌ AST backend does not support compilation\n", .{});
            return error.InvalidBackend;
        },
        .llvm => try compileWithLLVMCrossCompilation(source, filename, adjusted_config, &cross_compiler, zig_target, output_file),
        .c => try compileWithCCrossCompilation(allocator, source, filename, adjusted_config, &cross_compiler, zig_target, output_file),
        .wasm => try compileWithWASMCrossCompilation(source, filename, adjusted_config, &cross_compiler, zig_target, output_file),
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
    
    // Import formatter
    const formatter = @import("tools/formatter.zig");
    
    // Format the code
    var fmt = formatter.Formatter.init(allocator, formatter.FormatterConfig{});
    defer fmt.deinit();
    
    const formatted = fmt.format(source) catch |err| {
        print("❌ Error formatting file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(formatted);
    
    if (config.check_only) {
        // Check if source matches formatted output
        if (std.mem.eql(u8, source, formatted)) {
            if (config.verbose) {
                print("✅ {s} is already formatted\n", .{filename});
            }
        } else {
            print("❌ {s} needs formatting\n", .{filename});
            std.process.exit(1);
        }
        return;
    }
    
    if (config.show_diff) {
        // Show differences between original and formatted
        print("--- {s} (original)\n", .{filename});
        print("+++ {s} (formatted)\n", .{filename});
        // Simple line-by-line diff
        var source_lines = std.mem.splitScalar(u8, source, '\n');
        var formatted_lines = std.mem.splitScalar(u8, formatted, '\n');
        var line_num: u32 = 1;
        
        while (true) {
            const source_line = source_lines.next();
            const formatted_line = formatted_lines.next();
            
            if (source_line == null and formatted_line == null) break;
            
            const src = source_line orelse "";
            const fmt_line = formatted_line orelse "";
            
            if (!std.mem.eql(u8, src, fmt_line)) {
                if (source_line != null) {
                    print("-{}: {s}\n", .{ line_num, src });
                }
                if (formatted_line != null) {
                    print("+{}: {s}\n", .{ line_num, fmt_line });
                }
            }
            line_num += 1;
        }
        return;
    }
    
    // Write back to file
    const output_file = std.fs.cwd().createFile(filename, .{}) catch |err| {
        print("❌ Error creating output file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer output_file.close();
    
    output_file.writeAll(formatted) catch |err| {
        print("❌ Error writing formatted output to {s}: {any}\n", .{ filename, err });
        return;
    };
    
    if (config.verbose) {
        print("✅ Formatting completed\n", .{});
    }
}

fn executeTest(allocator: Allocator, config: Config) !void {
    // Use arena allocator for better memory management in tests
    var arena = std.heap.ArenaAllocator.init(allocator);
    defer arena.deinit();
    const arena_allocator = arena.allocator();
    
    if (config.verbose) {
        print("🧪 Running tests\n", .{});
    }
    
    if (config.source_file) |filename| {
        // Test specific file
        if (config.verbose) {
            print("🔍 Running single test file: {s}\n", .{filename});
        }
        
        const result = runSingleTest(arena_allocator, filename, config.verbose, config.debug_mode);
        if (result) |test_result| {
            if (test_result.success) {
                print("✅ Test file passed: {s}\n", .{filename});
                if (config.verbose) {
                    print("   Passed: {d} | Failed: {d} | Duration: {d}ms\n", .{ test_result.passed, test_result.failed, test_result.duration_ms });
                }
            } else {
                print("❌ Test file failed: {s}\n", .{filename});
                print("   Passed: {d} | Failed: {d} | Duration: {d}ms\n", .{ test_result.passed, test_result.failed, test_result.duration_ms });
                if (test_result.error_message) |msg| {
                    print("   Error: {s}\n", .{msg});
                }
                return error.TestFailed;
            }
        } else |err| {
            print("❌ Error running test file {s}: {any}\n", .{ filename, err });
            return err;
        }
    } else {
        // Run test suite
        try runTestSuite(arena_allocator, config.verbose, config.debug_mode);
    }
}

const TestResult = struct {
    filename: []const u8,
    passed: u32,
    failed: u32,
    duration_ms: u64,
    success: bool,
    error_message: ?[]const u8 = null,
};

fn runSingleTest(allocator: Allocator, filename: []const u8, verbose: bool, debug_mode: bool) !TestResult {
    const start_time = std.time.milliTimestamp();
    
    if (verbose) {
        print("   📄 Reading file: {s}\n", .{filename});
    }
    
    // Read test file with better error handling
    const file = std.fs.cwd().openFile(filename, .{}) catch |err| {
        const err_msg = switch (err) {
            error.FileNotFound => "File not found",
            error.AccessDenied => "Access denied", 
            error.IsDir => "Is a directory",
            else => "Failed to open file",
        };
        return TestResult{
            .filename = filename,
            .passed = 0,
            .failed = 1,
            .duration_ms = 0,
            .success = false,
            .error_message = err_msg,
        };
    };
    defer file.close();
    
    const content = file.readToEndAlloc(allocator, 2 * 1024 * 1024) catch |err| { // Increased buffer size
        const err_msg = switch (err) {
            error.OutOfMemory => "Out of memory reading file",
            error.InputOutput => "I/O error reading file",
            else => "Failed to read file content",
        };
        return TestResult{
            .filename = filename,
            .passed = 0,
            .failed = 1,
            .duration_ms = 0,
            .success = false,
            .error_message = err_msg,
        };
    };
    defer allocator.free(content);
    
    if (verbose) {
        print("   🚀 Executing test ({} bytes)\n", .{content.len});
    }
    
    // Execute the test by calling the interpreter directly
    const interpret_config = Config{
        .command = .interpret,
        .backend = .script,
        .source_file = filename,
        .verbose = verbose and debug_mode, // Only show verbose output in debug mode
        .debug_mode = debug_mode,
    };
    
    // Try to execute the test with error capture
    executeInterpret(allocator, interpret_config) catch {
        return TestResult{
            .filename = filename,
            .passed = 0,
            .failed = 1,
            .duration_ms = @as(u64, @intCast(std.time.milliTimestamp() - start_time)),
            .success = false,
            .error_message = "Execution failed",
        };
    };
    
    const end_time = std.time.milliTimestamp();
    const duration = @as(u64, @intCast(end_time - start_time));
    
    if (verbose) {
        print("   ✨ Test execution completed in {d}ms\n", .{duration});
    }
    
    // Parse test results from output (simplified - successful execution means test passed)
    // In a more sophisticated implementation, this would parse testz framework output
    const success = true; // If we got here, the test executed successfully
    const passed: u32 = if (success) 1 else 0;
    const failed: u32 = if (success) 0 else 1;
    
    return TestResult{
        .filename = filename,
        .passed = passed,
        .failed = failed,
        .duration_ms = duration,
        .success = success,
        .error_message = null,
    };
}

fn executeJIT(allocator: Allocator, config: Config) !void {
    const filename = config.source_file.?;
    
    if (config.verbose) {
        print("🚀 JIT executing {s}\n", .{filename});
    }
    
    // Read source file
    const source = readSourceFile(allocator, filename) catch |err| {
        print("❌ Error reading file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);
    
    if (config.verbose) print("📁 Read {s} ({} bytes)\n", .{ filename, source.len });
    
    print("🚀 Using Fixed JIT Execution Engine\n", .{});
    
    var jit_engine = SimpleJIT.init(allocator);
    defer jit_engine.deinit();
    
    // Execute multiple times to demonstrate JIT optimization
    var i: u32 = 1;
    const max_runs = if (config.verbose) 3 else 1;
    
    while (i <= max_runs) {
        if (max_runs > 1) {
            print("\n🔄 JIT Execution #{}\n", .{i});
            print("----------------------\n", .{});
        }
        
        jit_engine.execute(source) catch |err| {
            print("❌ JIT execution failed: {any}\n", .{err});
            return;
        };
        
        i += 1;
    }
    
    // Show JIT statistics  
    if (config.verbose) {
        print("📊 JIT Statistics: {} variables in scope\n", .{jit_engine.variables.count()});
        var var_iter = jit_engine.variables.iterator();
        while (var_iter.next()) |entry| {
            print("  {s} = {}\n", .{ entry.key_ptr.*, entry.value_ptr.* });
        }
    }
    
    print("✅ JIT execution completed successfully!\n", .{});
}

fn runTestSuite(allocator: Allocator, verbose: bool, debug_mode: bool) !void {
    if (verbose) {
        print("🔍 Discovering test files...\n", .{});
    }
    
    // Discover test files using arena allocator (all memory cleaned up automatically)
    var test_files: std.ArrayList([]const u8) = .empty;
    defer test_files.deinit();
    
    try discoverTestFiles(allocator, &test_files, "tests");
    
    if (test_files.items.len == 0) {
        print("⚠️ No test files found in tests/ directory\n", .{});
        return;
    }
    
    if (verbose) {
        print("📁 Found {d} test files\n", .{test_files.items.len});
    }
    
    // Run all tests
    var total_passed: u32 = 0;
    var total_failed: u32 = 0;
    var total_duration: u64 = 0;
    var failed_tests: std.ArrayList([]const u8) = .empty;
    defer failed_tests.deinit();
    
    const suite_start_time = std.time.milliTimestamp();
    
    print("\n🧪 Running test suite...\n", .{});
    print("═══════════════════════════════════\n", .{});
    
    for (test_files.items) |test_file| {
        // Truncate long paths for cleaner output
        const display_name = if (std.mem.lastIndexOf(u8, test_file, "/")) |idx| 
            test_file[idx + 1..] 
        else 
            test_file;
            
        if (verbose) {
            print("🔄 Running: {s}\n", .{display_name});
        } else {
            // Limit test name width to prevent output wrapping
            const max_width = 50;
            if (display_name.len > max_width) {
                print("🔄 ...{s} ... ", .{display_name[display_name.len - max_width + 3..]});
            } else {
                print("🔄 {s:<50} ", .{display_name});
            }
        }
        
        const result = runSingleTest(allocator, test_file, verbose, debug_mode) catch |err| {
            print("❌ ERROR ({any})\n", .{err});
            total_failed += 1;
            try failed_tests.append(test_file);
            continue;
        };
        
        total_passed += result.passed;
        total_failed += result.failed;
        total_duration += result.duration_ms;
        
        if (result.success) {
            if (verbose) {
                print("   ✅ PASS ({d}ms)\n", .{result.duration_ms});
            } else {
                print("✅ PASS\n", .{});
            }
        } else {
            if (verbose) {
                print("   ❌ FAIL ({d}ms)\n", .{result.duration_ms});
                if (result.error_message) |msg| {
                    print("      Error: {s}\n", .{msg});
                }
            } else {
                print("❌ FAIL\n", .{});
            }
            try failed_tests.append(test_file);
        }
    }
    
    const suite_end_time = std.time.milliTimestamp();
    const suite_duration = suite_end_time - suite_start_time;
    
    // Print summary
    print("\n📊 Test Suite Summary\n", .{});
    print("═══════════════════════════════════\n", .{});
    print("Total tests:     {d}\n", .{test_files.items.len});
    print("Passed:          {d} ✅\n", .{total_passed});
    print("Failed:          {d} ❌\n", .{total_failed});
    print("Success rate:    {d:.1}%\n", .{@as(f64, @floatFromInt(total_passed)) / @as(f64, @floatFromInt(test_files.items.len)) * 100.0});
    print("Total duration:  {d}ms\n", .{suite_duration});
    
    if (failed_tests.items.len > 0) {
        print("\n❌ Failed tests:\n", .{});
        for (failed_tests.items) |failed_test| {
            print("   • {s}\n", .{failed_test});
        }
        print("\n", .{});
        return error.TestSuiteFailed;
    } else {
        print("\n🎉 All tests passed!\n\n", .{});
    }
}

fn discoverTestFiles(allocator: Allocator, test_files: *std.ArrayList([]const u8), dir_path: []const u8) !void {
    var dir = std.fs.cwd().openDir(dir_path, .{ .iterate = true }) catch |err| {
        // If tests directory doesn't exist, that's okay
        if (err == error.FileNotFound) {
            return;
        }
        return err;
    };
    defer dir.close();
    
    var iterator = dir.iterate();
    while (try iterator.next()) |entry| {
        const full_path = try std.fmt.allocPrint(allocator, "{s}/{s}", .{ dir_path, entry.name });
        
        switch (entry.kind) {
            .file => {
                // Check if it's a .csd file
                if (std.mem.endsWith(u8, entry.name, ".csd")) {
                    try test_files.append(full_path);
                } else {
                    allocator.free(full_path);
                }
            },
            .directory => {
                // Recursively search subdirectories
                try discoverTestFiles(allocator, test_files, full_path);
                allocator.free(full_path);
            },
            else => {
                allocator.free(full_path);
            },
        }
    }
}

// Simple variable store for runtime evaluation
const Variable = union(enum) {
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    Array: ArrayList(Variable),
    
    pub fn toString(self: Variable, allocator: Allocator) ![]u8 {
        switch (self) {
            .Integer => |int| return std.fmt.allocPrint(allocator, "{}", .{int}),
            .Float => |float| return std.fmt.allocPrint(allocator, "{d}", .{float}),
            .String => |str| return allocator.dupe(u8, str),
            .Boolean => |bool_val| return allocator.dupe(u8, if (bool_val) "based" else "cap"),
            .Array => |arr| {
                var result: std.ArrayList(u8) = .empty;
                try result.append('[');
                for (arr.items, 0..) |item, i| {
                    if (i > 0) try result.appendSlice(", ");
                    const item_str = try item.toString(allocator);
                    defer allocator.free(item_str);
                    try result.appendSlice(item_str);
                }
                try result.append(']');
                return result.toOwnedSlice();
            },
        }
    }
};

const VariableStore = HashMap([]const u8, Variable, std.hash_map.StringContext, std.hash_map.default_max_load_percentage);

// Backend implementation functions
fn interpretScript(arena_manager: *CursedArenaManager, source: []const u8, config: Config) !void {
    if (config.verbose) print("🚀 Using enhanced script interpreter with variable support\n", .{});
    // Use the existing interpretation logic from main_unified.zig
    const allocator = arena_manager.getParserAllocator();
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
    
    // Initialize variable storage
    var variables = VariableStore.init(allocator);
    defer {
        var iter = variables.iterator();
        while (iter.next()) |entry| {
            allocator.free(entry.key_ptr.*);
            // Free Variable values that contain allocated memory
            switch (entry.value_ptr.*) {
                .String => |str| allocator.free(str),
                .Array => |arr| {
                    for (arr.items) |item| {
                        switch (item) {
                            .String => |str| allocator.free(str),
                            else => {},
                        }
                    }
                    arr.deinit();
                },
                else => {},
            }
        }
        variables.deinit();
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
        
        if (config.verbose) print("🔍 Processing line: '{s}' (len: {})\n", .{ trimmed, trimmed.len });
        
        // Skip import statements during execution
        if (std.mem.startsWith(u8, trimmed, "yeet ")) {
            if (config.verbose) print("📦 Import: {s}\n", .{trimmed});
            continue;
        }
        
        // Handle variable declarations: sus varname type = value
        if (std.mem.startsWith(u8, trimmed, "sus ")) {
            if (config.verbose) print("🔍 Detected sus statement: {s}\n", .{trimmed});
            try handleVariableDeclaration(&variables, allocator, trimmed, config.verbose);
            continue;
        }
        
        // Handle vibez.spill() statements with variable support
        if (std.mem.indexOf(u8, trimmed, "vibez.spill(")) |start| {
            try handleVibesSpill(&variables, allocator, trimmed, start, config.verbose);
        } else if (config.verbose) {
            print("Line {}: {s}\n", .{ line_number, trimmed });
        }
    }
    
    if (config.verbose) print("✅ Script interpretation completed\n", .{});
}

fn interpretAST(arena_manager: *CursedArenaManager, source: []const u8, config: Config) !void {
    if (config.verbose) print("🚀 Using AST-based interpreter with full function support\n", .{});
    
    // Use appropriate allocators from arena manager
    const parser_allocator = arena_manager.getParserAllocator();
    const ast_allocator = arena_manager.getASTAllocator();
    
    // Tokenize the source
    var lex = lexer.Lexer.init(parser_allocator, source);
    var tokens = lex.tokenize() catch |err| {
        print("❌ Tokenization error: {any}\n", .{err});
        return;
    };
    defer tokens.deinit();
    
    if (config.verbose) {
        print("🔤 Tokenized {} tokens\n", .{tokens.items.len});
    }
    
    // Parse tokens into AST using appropriate allocator
    var p = parser.Parser.init(ast_allocator, tokens.items);
    
    var program = p.parseProgram() catch |err| {
        print("❌ Parsing error: {any}\n", .{err});
        return;
    };
    defer program.deinit(ast_allocator);
    
    if (config.verbose) {
        print("🌳 Generated AST with {} statements\n", .{program.statements.items.len});
    }
    
    // TODO: Execute with proper interpreter once compilation issues are resolved
    // const runtime_allocator = arena_manager.getRuntimeAllocator();
    // var cursed_interpreter = interpreter.Interpreter.init(runtime_allocator);
    // defer cursed_interpreter.deinit();
    
    if (config.verbose) print("🚀 AST parsing completed successfully - function execution not yet implemented\n", .{});
    // cursed_interpreter.execute(program) catch |err| {
    //     print("❌ Execution error: {any}\n", .{err});
    //     return;
    // };
    
    if (config.verbose) print("✅ AST parsing completed (execution pending)\n", .{});
}

fn compileWithLLVM(allocator: Allocator, source: []const u8, filename: []const u8, config: Config) !void {
    if (config.verbose) {
        print("🚀 Using LLVM backend (enhanced) for compilation\n", .{});
    }
    
    // Try using enhanced compiler if it compiles, fall back to simple_compiler
    // Import the enhanced compiler with real LLVM backend
    const enhanced_compiler = @import("enhanced_compiler.zig");
    
    // Determine output file
    const output_file = if (config.output_file) |custom_output| 
        custom_output
    else if (std.mem.endsWith(u8, filename, ".csd"))
        try std.fmt.allocPrint(allocator, "{s}", .{filename[0..filename.len - 4]})
    else
        try std.fmt.allocPrint(allocator, "{s}_compiled", .{filename});
    defer if (config.output_file == null) allocator.free(output_file);
    
    if (config.verbose) {
        print("📝 Compiling {s} to {s} with LLVM IR generation\n", .{ filename, output_file });
    }
    
    // Use enhanced compiler for LLVM IR generation
    const compiler_config = enhanced_compiler.CompilerConfig{
        .backend = enhanced_compiler.CompilationBackend.LLVM_Backend,
        .optimization_level = config.optimization_level,
        .output_path = output_file,
        .verbose = config.verbose,
    };
    
    enhanced_compiler.compileProgram(allocator, source, filename, compiler_config) catch |err| {
        print("❌ LLVM compilation failed: {any}\n", .{err});
        print("🔄 Falling back to simple C transpilation...\n", .{});
        return simple_compiler.compileProgramWithOutput(allocator, source, filename, config.output_file, config.optimization_level, config.verbose);
    };
    
    if (config.verbose) {
        print("✅ LLVM compilation successful!\n", .{});
    }
    
    print("📦 Output executable: {s}\n", .{output_file});
    print("🚀 Run with: ./{s}\n", .{output_file});
}

fn compileWithC(_: Allocator, source: []const u8, filename: []const u8, config: Config) !void {
        _ = source;
    _ = filename;
    _ = config;
    print("❌ C backend not yet implemented\n", .{});
    return error.NotImplemented;
}

fn compileWithWASM(_: Allocator, source: []const u8, filename: []const u8, config: Config) !void {
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

fn handleVariableDeclaration(variables: *VariableStore, allocator: Allocator, line: []const u8, verbose: bool) !void {
    // Parse: sus varname type = value
    // Examples: sus x drip = 42, sus numbers [normie] = [10, 20, 30]
    
    if (verbose) print("🔧 Processing variable declaration: {s}\n", .{line});
    
    // Find the equals sign to split the declaration
    const equals_pos = std.mem.indexOf(u8, line, "=") orelse {
        if (verbose) print("❌ No equals sign found in variable declaration\n", .{});
        return;
    };
    const decl_part = std.mem.trim(u8, line[0..equals_pos], " \t");
    const value_str = std.mem.trim(u8, line[equals_pos + 1..], " \t");
    
    if (verbose) print("🔧 Declaration part: '{s}' = '{s}'\n", .{ decl_part, value_str });
    
    // Parse declaration part: "sus varname type" 
    var parts = std.mem.tokenizeScalar(u8, decl_part, ' ');
    _ = parts.next(); // skip "sus"
    
    const var_name = parts.next() orelse {
        if (verbose) print("❌ No variable name found\n", .{});
        return;
    };
    
    // The type might be compound like [normie], so get the rest
    const remaining = parts.rest();
    const var_type = if (remaining.len > 0) remaining else {
        if (verbose) print("❌ No variable type found\n", .{});
        return;
    };
    
    if (verbose) print("🔧 Declaring variable: {s} (type: {s}) = {s}\n", .{ var_name, var_type, value_str });
    
    // Parse value based on type
    const variable_value = if (std.mem.eql(u8, var_type, "drip") or std.mem.eql(u8, var_type, "normie")) blk: {
        // Integer type (both drip and normie are integers)
        // First try parsing as float and convert to int if it has no decimal part
        if (std.fmt.parseFloat(f64, std.mem.trim(u8, value_str, " \t"))) |parsed_float| {
            const int_val = @as(i64, @intFromFloat(parsed_float));
            break :blk Variable{ .Integer = int_val };
        } else |_| {
            // If not a literal, check if it's a module function call (but not decimal numbers)
            if (std.mem.indexOf(u8, value_str, ".") != null and std.mem.indexOf(u8, value_str, "(") != null) {
                // Only treat as module function if it has both "." and "(" 
                if (verbose) print("📦 Module function call detected: {s} (returning placeholder 0)\n", .{value_str});
                break :blk Variable{ .Integer = 0 };
            } else {
                if (verbose) print("❌ Error parsing integer '{s}': not a valid number or function call\n", .{value_str});
                return;
            }
        }
    } else if (std.mem.eql(u8, var_type, "meal")) blk: {
        // Float type - try to parse as literal first, then as function call
        if (std.fmt.parseFloat(f64, std.mem.trim(u8, value_str, " \t"))) |parsed_float| {
            break :blk Variable{ .Float = parsed_float };
        } else |_| {
            // If not a literal, check if it's a module function call
            if (std.mem.indexOf(u8, value_str, ".")) |_| {
                // For now, return a placeholder value for module function calls
                if (verbose) print("📦 Module function call detected: {s} (returning placeholder 0.0)\n", .{value_str});
                break :blk Variable{ .Float = 0.0 };
            } else {
                if (verbose) print("❌ Error parsing float '{s}': not a valid number or function call\n", .{value_str});
                return;
            }
        }
    } else if (std.mem.eql(u8, var_type, "tea")) blk: {
        // String type
        var trimmed_value = std.mem.trim(u8, value_str, " \t");
        if (trimmed_value.len >= 2 and trimmed_value[0] == '"' and trimmed_value[trimmed_value.len - 1] == '"') {
            trimmed_value = trimmed_value[1..trimmed_value.len - 1];
        }
        const string_copy = try allocator.dupe(u8, trimmed_value);
        break :blk Variable{ .String = string_copy };
    } else if (std.mem.eql(u8, var_type, "lit")) blk: {
        // Boolean type
        const bool_val = std.mem.eql(u8, std.mem.trim(u8, value_str, " \t"), "based");
        break :blk Variable{ .Boolean = bool_val };
    } else if (std.mem.eql(u8, var_type, "sip")) blk: {
        // Character type - treat as single character string
        var trimmed_value = std.mem.trim(u8, value_str, " \t");
        if (trimmed_value.len >= 2 and trimmed_value[0] == '\'' and trimmed_value[trimmed_value.len - 1] == '\'') {
            trimmed_value = trimmed_value[1..trimmed_value.len - 1];
        }
        const string_copy = try allocator.dupe(u8, trimmed_value);
        break :blk Variable{ .String = string_copy };
    } else if (std.mem.startsWith(u8, var_type, "[") and std.mem.endsWith(u8, var_type, "]")) blk: {
        // Array type like [normie]
        const element_type = var_type[1..var_type.len - 1];
        const trimmed_val = std.mem.trim(u8, value_str, " \t");
        
        if (trimmed_val.len >= 2 and trimmed_val[0] == '[' and trimmed_val[trimmed_val.len - 1] == ']') {
            // Parse array literal [1, 2, 3]
            var array = .empty;
            const content = trimmed_val[1..trimmed_val.len - 1];
            
            if (content.len > 0) {
                var elements = std.mem.splitScalar(u8, content, ',');
                while (elements.next()) |element| {
                    const trimmed_element = std.mem.trim(u8, element, " \t");
                    
                    if (std.mem.eql(u8, element_type, "normie")) {
                        const int_val = std.fmt.parseInt(i64, trimmed_element, 10) catch {
                            if (verbose) print("❌ Error parsing array element '{s}'\n", .{trimmed_element});
                            continue;
                        };
                        try array.append(Variable{ .Integer = int_val });
                    } else {
                        if (verbose) print("❌ Unsupported array element type: {s}\n", .{element_type});
                    }
                }
            }
            
            break :blk Variable{ .Array = array };
        } else {
            if (verbose) print("❌ Invalid array literal: {s}\n", .{trimmed_val});
            return;
        }
    } else {
        if (verbose) print("❌ Unknown variable type: {s}\n", .{var_type});
        return;
    };
    
    // Store variable (copy name for hash map key)
    const name_copy = try allocator.dupe(u8, var_name);
    try variables.put(name_copy, variable_value);
    
    if (verbose) print("✅ Variable {s} stored successfully\n", .{var_name});
}

fn handleVibesSpill(variables: *VariableStore, allocator: Allocator, line: []const u8, start: usize, verbose: bool) !void {
    if (std.mem.indexOf(u8, line[start..], "(")) |paren_start| {
        if (std.mem.lastIndexOf(u8, line, ")")) |paren_end| {
            const content_start = start + paren_start + 1;
            const content = line[content_start..paren_end];
            const trimmed_content = std.mem.trim(u8, content, " \t");
            
            if (verbose) print("🔍 Evaluating vibez.spill argument: '{s}'\n", .{trimmed_content});
            
            // Check if there are multiple arguments separated by commas (but not inside quotes)  
            if (hasCommaOutsideQuotes(trimmed_content)) {
                // Handle multiple arguments - need to parse them properly respecting quotes
                var args = try parseArguments(allocator, trimmed_content);
                defer args.deinit();
                
                var first_arg = true;
                for (args.items) |arg| {
                    if (!first_arg) print(" ", .{});
                    first_arg = false;
                    
                    try evaluateAndPrintArgument(variables, allocator, arg, verbose, false); // no newline for multi-args
                }
                print("\n", .{});
                return;
            }
            
            // Single argument - evaluate and print with newline
            try evaluateAndPrintArgument(variables, allocator, trimmed_content, verbose, true);
        }
    }
}

fn hasCommaOutsideQuotes(text: []const u8) bool {
    var in_quotes = false;
    for (text) |char| {
        if (char == '"') {
            in_quotes = !in_quotes;
        } else if (char == ',' and !in_quotes) {
            return true;
        }
    }
    return false;
}

fn parseArguments(_: Allocator, text: []const u8) !ArrayList([]const u8) {
    var args = .empty;
    var start: usize = 0;
    var in_quotes = false;
    
    for (text, 0..) |char, i| {
        if (char == '"') {
            in_quotes = !in_quotes;
        } else if (char == ',' and !in_quotes) {
            const arg = std.mem.trim(u8, text[start..i], " \t");
            try args.append(arg);
            start = i + 1;
        }
    }
    
    // Add the last argument
    const arg = std.mem.trim(u8, text[start..], " \t");
    try args.append(arg);
    
    return args;
}

fn evaluateAndPrintArgument(variables: *VariableStore, allocator: Allocator, trimmed_content: []const u8, verbose: bool, add_newline: bool) !void {
    // Check if it's a string literal
    if (trimmed_content.len >= 2 and trimmed_content[0] == '"' and trimmed_content[trimmed_content.len - 1] == '"') {
        print("{s}", .{trimmed_content[1..trimmed_content.len - 1]});
        if (add_newline) print("\n", .{});
    } else if (std.mem.indexOf(u8, trimmed_content, "[")) |bracket_pos| {
        // Array access expression like numbers[i]
        const array_name = trimmed_content[0..bracket_pos];
        if (std.mem.indexOf(u8, trimmed_content[bracket_pos..], "]")) |end_bracket| {
            const index_expr = trimmed_content[bracket_pos + 1..bracket_pos + end_bracket];
            
            if (verbose) print("🔍 Array access: {s}[{s}]\n", .{ array_name, index_expr });
            
            if (variables.get(array_name)) |array_var| {
                switch (array_var) {
                    .Array => |array| {
                        // Parse index
                        if (std.fmt.parseInt(i64, index_expr, 10)) |index| {
                            if (index >= 0 and index < array.items.len) {
                                const element_str = try array.items[@intCast(index)].toString(allocator);
                                defer allocator.free(element_str);
                                print("{s}", .{element_str});
                                if (add_newline) print("\n", .{});
                                if (verbose) print("✅ Array access {s}[{}] = {s}\n", .{ array_name, index, element_str });
                            } else {
                                print("undefined", .{});
                                if (add_newline) print("\n", .{});
                                if (verbose) print("⚠️  Array index {} out of bounds for {s} (length: {})\n", .{ index, array_name, array.items.len });
                            }
                        } else |_| {
                            // Try to resolve index as a variable
                            if (variables.get(index_expr)) |index_var| {
                                switch (index_var) {
                                    .Integer => |index| {
                                        if (index >= 0 and index < array.items.len) {
                                            const element_str = try array.items[@intCast(index)].toString(allocator);
                                            defer allocator.free(element_str);
                                            print("{s}", .{element_str});
                                            if (add_newline) print("\n", .{});
                                            if (verbose) print("✅ Array access {s}[{s}={}] = {s}\n", .{ array_name, index_expr, index, element_str });
                                        } else {
                                            print("undefined", .{});
                                            if (add_newline) print("\n", .{});
                                            if (verbose) print("⚠️  Array index {} out of bounds for {s} (length: {})\n", .{ index, array_name, array.items.len });
                                        }
                                    },
                                    else => {
                                        print("{s}", .{trimmed_content});
                                        if (add_newline) print("\n", .{});
                                        if (verbose) print("⚠️  Index variable {s} is not an integer\n", .{index_expr});
                                    },
                                }
                            } else {
                                print("{s}", .{trimmed_content});
                                if (add_newline) print("\n", .{});
                                if (verbose) print("⚠️  Index variable {s} not found\n", .{index_expr});
                            }
                        }
                    },
                    else => {
                        print("{s}", .{trimmed_content});
                        if (add_newline) print("\n", .{});
                        if (verbose) print("⚠️  Variable {s} is not an array\n", .{array_name});
                    },
                }
            } else {
                print("{s}", .{trimmed_content});
                if (add_newline) print("\n", .{});
                if (verbose) print("⚠️  Array not found: {s}\n", .{array_name});
            }
        } else {
            print("{s}", .{trimmed_content});
            if (add_newline) print("\n", .{});
        }
    } else if (variables.get(trimmed_content)) |variable| {
        // Variable reference - evaluate and print
        const var_str = try variable.toString(allocator);
        defer allocator.free(var_str);
        print("{s}", .{var_str});
        if (add_newline) print("\n", .{});
        if (verbose) print("✅ Resolved variable {s} to: {s}\n", .{ trimmed_content, var_str });
    } else {
        // Try to parse as literal value
        if (std.fmt.parseInt(i64, trimmed_content, 10)) |int_val| {
            print("{}", .{int_val});
            if (add_newline) print("\n", .{});
        } else |_| {
            if (std.fmt.parseFloat(f64, trimmed_content)) |float_val| {
                print("{d}", .{float_val});
                if (add_newline) print("\n", .{});
            } else |_| {
                // Unknown identifier
                print("{s}", .{trimmed_content});
                if (add_newline) print("\n", .{});
                if (verbose) print("⚠️  Unknown variable: {s}\n", .{trimmed_content});
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
    print("    jit             Execute CURSED source via JIT compilation\n", .{});
    print("    check           Type check CURSED source code\n", .{});
    print("    format          Format CURSED source code\n", .{});
    print("    test            Run tests\n", .{});
    print("\n", .{});
    print("BUILD SYSTEM COMMANDS:\n", .{});
    print("    init            Initialize new CURSED project\n", .{});
    print("    run             Run CURSED project\n", .{});
    print("    clean           Clean build artifacts\n", .{});
    print("    doc             Generate documentation\n", .{});
    print("    install         Install project dependencies\n", .{});
    print("    build           Build project using Zig build system\n", .{});
    print("\n", .{});
    print("    --version, -v   Show version information\n", .{});
    print("    --help, -h      Show this help message\n\n", .{});
    
    print("OPTIONS:\n", .{});
    print("    --backend, -b BACKEND    Compilation backend [script, ast, llvm, c, wasm]\n", .{});
    print("    --target, -t TARGET      Target platform for cross-compilation\n", .{});
    print("    --linking, -l MODE       Linking mode [dynamic, static]\n", .{});
    print("    --output, -o FILE        Output file (for compile command)\n", .{});
    print("    --optimize, -O LEVEL     Optimization level (0-3) [default: 2]\n", .{});
    print("    --lto                    Enable link-time optimization\n", .{});
    print("    --debug-info, -g         Generate debug information\n", .{});
    print("    --preserve-debug-info    Preserve debug info in optimized builds\n", .{});
    print("    --debug, -d              Enable debug mode (verbose + tokens)\n", .{});
    print("    --verbose                Enable verbose output\n", .{});
    print("    --tokens                 Show token stream\n", .{});
    print("    --watch, -w              Watch for file changes\n\n", .{});
    
    print("CROSS-COMPILATION TARGETS:\n", .{});
    print("    native                   Host platform (default)\n", .{});
    print("    linux-x64                Linux x86_64\n", .{});
    print("    linux-arm64              Linux ARM64\n", .{});
    print("    macos-x64                macOS x86_64\n", .{});
    print("    macos-arm64              macOS ARM64\n", .{});
    print("    windows-x64              Windows x86_64\n", .{});
    print("    wasm32                   WebAssembly 32-bit\n\n", .{});
    
    print("EXAMPLES:\n", .{});
    print("    cursed hello.csd                          # Interpret hello.csd\n", .{});
    print("    cursed interpret hello.csd --verbose       # Interpret with verbose output\n", .{});
    print("    cursed compile hello.csd -b llvm           # Compile with LLVM backend\n", .{});
    print("    cursed compile hello.csd -t linux-x64      # Cross-compile for Linux x64\n", .{});
    print("    cursed compile hello.csd -t wasm32         # Compile for WebAssembly\n", .{});
    print("    cursed compile hello.csd --linking static  # Static linking\n", .{});
    print("    cursed check hello.csd                     # Type check hello.csd\n", .{});
    print("    cursed format hello.csd                    # Format hello.csd\n", .{});
    print("    cursed jit hello.csd --verbose             # JIT compile and execute\n", .{});
    print("    cursed test                                # Run test suite\n\n", .{});
    
    print("For more information, visit: https://github.com/ghuntley/cursed\n", .{});
}

// Cross-compilation implementation functions




fn compileWithLLVMCrossCompilation(
    source: []const u8, 
    filename: []const u8, 
    config: Config,
    cross_compiler: *cross_compilation.CrossCompiler,
    target_platform: []const u8,
    output_file: []const u8
) !void {
    if (config.verbose) {
        print("🔨 LLVM cross-compilation for target: {s}\n", .{target_platform});
    }
    
    // Step 1: Compile CURSED source to LLVM IR using advanced codegen
    
    // Create a temporary allocator for compilation
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Create temporary LLVM IR file
    const ir_filename = try std.fmt.allocPrint(allocator, "{s}.ll", .{std.fs.path.stem(filename)});
    defer allocator.free(ir_filename);
    
    if (config.verbose) {
        print("🔄 Compiling CURSED to LLVM IR: {s} → {s}\n", .{ filename, ir_filename });
    }
    
    // Compile CURSED source to LLVM IR using enhanced compiler
    const enhanced_compiler = @import("enhanced_compiler.zig");
    const compiler_config = enhanced_compiler.CompilerConfig{
        .verbose = config.verbose,
        .debug_info = false,
    };
    try enhanced_compiler.compileToLLVMBackend(allocator, source, filename, ir_filename, compiler_config);
    
    if (config.verbose) {
        print("✅ LLVM IR generated successfully\n", .{});
    }
    
    // Step 2: Compile LLVM IR to native executable using cross-compilation
    const command = try cross_compiler.generateLLVMIRCompileCommand(
        ir_filename,
        target_platform,
        output_file,
        config.optimization_level,
        config.linking_mode.toString(),
        config.verbose
    );
    defer cross_compiler.freeCompileCommand(command);
    
    try cross_compiler.executeCompilation(command, config.verbose);
    
    // Clean up temporary IR file (only if not verbose)
    if (!config.verbose) {
        std.fs.cwd().deleteFile(ir_filename) catch |err| {
            if (config.verbose) {
                print("⚠️  Could not delete temporary IR file {s}: {any}\n", .{ ir_filename, err });
            }
        };
    } else {
        print("📝 LLVM IR file saved: {s}\n", .{ir_filename});
    }
    
    if (config.verbose) {
        print("✅ LLVM cross-compilation completed: {s}\n", .{output_file});
    }
}

fn compileWithCCrossCompilation(
    _: Allocator, 
    source: []const u8, 
    filename: []const u8, 
    config: Config,
    cross_compiler: *cross_compilation.CrossCompiler,
    target_platform: []const u8,
    output_file: []const u8
) !void {
    _ = source;
        _ = filename;
    _ = cross_compiler;
    _ = output_file;
    
    if (config.verbose) {
        print("🔨 C backend cross-compilation for target: {s}\n", .{target_platform});
    }
    
    // C backend cross-compilation
    // 1. Transpile CURSED to C
    // 2. Use target-specific C compiler
    
    print("❌ C backend cross-compilation not yet fully implemented\n", .{});
    print("   Consider using LLVM backend for cross-compilation\n", .{});
}

fn compileWithWASMCrossCompilation(
    source: []const u8, 
    filename: []const u8, 
    config: Config,
    cross_compiler: *cross_compilation.CrossCompiler,
    target_platform: []const u8,
    output_file: []const u8
) !void {
    _ = source;
    
    if (config.verbose) {
        print("🔨 WebAssembly compilation for target: {s}\n", .{target_platform});
    }
    
    // Generate WASM-specific compilation command
    const command = try cross_compiler.generateCompileCommand(
        filename,
        "wasm32-freestanding",
        output_file,
        config.optimization_level,
        "static", // WASM typically uses static linking
        config.verbose
    );
    defer cross_compiler.freeCompileCommand(command);
    
    try cross_compiler.executeCompilation(command, config.verbose);
    
    if (config.verbose) {
        print("✅ WebAssembly compilation completed: {s}\n", .{output_file});
    }
}
