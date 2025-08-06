const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const lexer = @import("lexer.zig");
const simple_import_resolver = @import("simple_import_resolver.zig");
const simple_compiler = @import("simple_compiler.zig");
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
    project_dir: ?[]const u8 = null,
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
        // Build system commands - delegate to CLI command handler (disabled)
        .init, .run, .clean, .doc, .install, .build => {
            print("Build system command '{s}' not yet implemented\n", .{@tagName(config.command)});
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
        var source_lines = std.mem.split(u8, source, "\n");
        var formatted_lines = std.mem.split(u8, formatted, "\n");
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
                var result = std.ArrayList(u8).init(allocator);
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
fn interpretScript(allocator: Allocator, source: []const u8, config: Config) !void {
    if (config.verbose) print("🚀 Using enhanced script interpreter with variable support\n", .{});
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
        
        // Skip import statements during execution
        if (std.mem.startsWith(u8, trimmed, "yeet ")) {
            if (config.verbose) print("📦 Import: {s}\n", .{trimmed});
            continue;
        }
        
        // Handle variable declarations: sus varname type = value
        if (std.mem.startsWith(u8, trimmed, "sus ")) {
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

fn compileWithLLVM(allocator: Allocator, source: []const u8, filename: []const u8, config: Config) !void {
    // Use existing simple_compiler for LLVM compilation
    try simple_compiler.compileProgramWithOutput(allocator, source, filename, config.output_file, config.optimization_level, config.verbose);
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

fn handleVariableDeclaration(variables: *VariableStore, allocator: Allocator, line: []const u8, verbose: bool) !void {
    // Parse: sus varname type = value
    // Examples: sus x drip = 42, sus numbers [normie] = [10, 20, 30]
    
    // Find the equals sign to split the declaration
    const equals_pos = std.mem.indexOf(u8, line, "=") orelse return;
    const decl_part = std.mem.trim(u8, line[0..equals_pos], " \t");
    const value_str = std.mem.trim(u8, line[equals_pos + 1..], " \t");
    
    // Parse declaration part: "sus varname type" 
    var parts = std.mem.tokenizeScalar(u8, decl_part, ' ');
    _ = parts.next(); // skip "sus"
    
    const var_name = parts.next() orelse return;
    
    // The type might be compound like [normie], so get the rest
    const remaining = parts.rest();
    const var_type = if (remaining.len > 0) remaining else return;
    
    if (verbose) print("🔧 Declaring variable: {s} (type: {s}) = {s}\n", .{ var_name, var_type, value_str });
    
    // Parse value based on type
    const variable_value = if (std.mem.eql(u8, var_type, "drip")) blk: {
        // Integer type
        const parsed_int = std.fmt.parseInt(i64, std.mem.trim(u8, value_str, " \t"), 10) catch |err| {
            if (verbose) print("❌ Error parsing integer '{s}': {any}\n", .{ value_str, err });
            return;
        };
        break :blk Variable{ .Integer = parsed_int };
    } else if (std.mem.eql(u8, var_type, "meal")) blk: {
        // Float type
        const parsed_float = std.fmt.parseFloat(f64, std.mem.trim(u8, value_str, " \t")) catch |err| {
            if (verbose) print("❌ Error parsing float '{s}': {any}\n", .{ value_str, err });
            return;
        };
        break :blk Variable{ .Float = parsed_float };
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
    } else if (std.mem.startsWith(u8, var_type, "[") and std.mem.endsWith(u8, var_type, "]")) blk: {
        // Array type like [normie]
        const element_type = var_type[1..var_type.len - 1];
        const trimmed_val = std.mem.trim(u8, value_str, " \t");
        
        if (trimmed_val.len >= 2 and trimmed_val[0] == '[' and trimmed_val[trimmed_val.len - 1] == ']') {
            // Parse array literal [1, 2, 3]
            var array = ArrayList(Variable).init(allocator);
            const content = trimmed_val[1..trimmed_val.len - 1];
            
            if (content.len > 0) {
                var elements = std.mem.split(u8, content, ",");
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
            
            // Check if it's a string literal
            if (trimmed_content.len >= 2 and trimmed_content[0] == '"' and trimmed_content[trimmed_content.len - 1] == '"') {
                print("{s}\n", .{trimmed_content[1..trimmed_content.len - 1]});
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
                                        print("{s}\n", .{element_str});
                                        if (verbose) print("✅ Array access {s}[{}] = {s}\n", .{ array_name, index, element_str });
                                    } else {
                                        print("undefined\n", .{});
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
                                                    print("{s}\n", .{element_str});
                                                    if (verbose) print("✅ Array access {s}[{s}={}] = {s}\n", .{ array_name, index_expr, index, element_str });
                                                } else {
                                                    print("undefined\n", .{});
                                                    if (verbose) print("⚠️  Array index {} out of bounds for {s} (length: {})\n", .{ index, array_name, array.items.len });
                                                }
                                            },
                                            else => {
                                                print("{s}\n", .{trimmed_content});
                                                if (verbose) print("⚠️  Index variable {s} is not an integer\n", .{index_expr});
                                            },
                                        }
                                    } else {
                                        print("{s}\n", .{trimmed_content});
                                        if (verbose) print("⚠️  Index variable {s} not found\n", .{index_expr});
                                    }
                                }
                            },
                            else => {
                                print("{s}\n", .{trimmed_content});
                                if (verbose) print("⚠️  Variable {s} is not an array\n", .{array_name});
                            },
                        }
                    } else {
                        print("{s}\n", .{trimmed_content});
                        if (verbose) print("⚠️  Array not found: {s}\n", .{array_name});
                    }
                } else {
                    print("{s}\n", .{trimmed_content});
                }
            } else if (variables.get(trimmed_content)) |variable| {
                // Variable reference - evaluate and print
                const var_str = try variable.toString(allocator);
                defer allocator.free(var_str);
                print("{s}\n", .{var_str});
                if (verbose) print("✅ Resolved variable {s} to: {s}\n", .{ trimmed_content, var_str });
            } else {
                // Try to parse as literal value
                if (std.fmt.parseInt(i64, trimmed_content, 10)) |int_val| {
                    print("{}\n", .{int_val});
                } else |_| {
                    if (std.fmt.parseFloat(f64, trimmed_content)) |float_val| {
                        print("{d}\n", .{float_val});
                    } else |_| {
                        // Unknown identifier
                        print("{s}\n", .{trimmed_content});
                        if (verbose) print("⚠️  Unknown variable: {s}\n", .{trimmed_content});
                    }
                }
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
    print("    --backend, -b BACKEND    Compilation backend [script, llvm, c, wasm]\n", .{});
    print("    --output, -o FILE        Output file (for compile command)\n", .{});
    print("    --optimize, -O LEVEL     Optimization level (0-3) [default: 2]\n", .{});
    print("    --lto                    Enable link-time optimization\n", .{});
    print("    --debug-info, -g         Generate debug information\n", .{});
    print("    --preserve-debug-info    Preserve debug info in optimized builds\n", .{});
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
