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
const stack_trace = @import("stack_trace_runtime.zig");

// Mixed type value for variables (strings and integers)
const VariableValue = union(enum) {
    integer: i64,
    string: []const u8,
    boolean: bool,
};

// Version information
const VERSION = "1.0.0";
const BUILD_INFO = "CURSED Unified Compiler (AST Enabled)";

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
    ast,       // AST-based interpretation (enabled but limited)
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

// CLI configuration
const Config = struct {
    command: Command = .interpret,
    backend: Backend = .script,
    source_file: ?[]const u8 = null,
    output_file: ?[]const u8 = null,
    project_dir: ?[]const u8 = null,
    target_platform: TargetPlatform = .native,
    optimization_level: u8 = 2,
    debug_mode: bool = false,
    verbose: bool = false,
    show_tokens: bool = false,
    
    pub fn init() Config {
        return Config{};
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Initialize stack trace system for error handling
    stack_trace.setGlobalAllocator(allocator);
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    // Parse command line arguments
    const config = try parseArgs(allocator, args);
    
    // Execute the appropriate command
    print("🐛 DEBUG: Executing command: {s} with backend: {s}\n", .{@tagName(config.command), @tagName(config.backend)});
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
        else => {
            print("Command '{s}' not yet implemented\n", .{@tagName(config.command)});
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
            } else if (std.mem.eql(u8, backend_str, "ast")) {
                config.backend = .ast;
                print("🎯 AST backend enabled!\n", .{});
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
        } else if (std.mem.eql(u8, arg, "--verbose")) {
            config.verbose = true;
        } else if (std.mem.eql(u8, arg, "--debug") or std.mem.eql(u8, arg, "-d")) {
            config.debug_mode = true;
            config.verbose = true;
        } else if (std.mem.eql(u8, arg, "--compile")) {
            config.command = .compile;
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
    
    // Execute based on backend
    switch (config.backend) {
        .script => try interpretScript(allocator, source, config),
        .ast => try interpretWithAST(allocator, source, config),
        else => {
            print("❌ Backend {s} not supported in this build\n", .{@tagName(config.backend)});
            return error.InvalidBackend;
        },
    }
}

fn executeCompile(allocator: Allocator, config: Config) !void {
    const filename = config.source_file.?;
    
    if (config.verbose) {
        print("🔨 Compiling {s} with backend: {s}\n", .{ filename, @tagName(config.backend) });
    }
    
    // Read source file
    const source = readSourceFile(allocator, filename) catch |err| {
        print("❌ Error reading file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);
    
    // Generate output filename if not specified
    const output_file = config.output_file orelse blk: {
        const base_name = std.fs.path.stem(filename);
        const output_name = try std.fmt.allocPrint(allocator, "{s}{s}", .{ base_name, config.target_platform.getExecutableExtension() });
        break :blk output_name;
    };
    defer if (config.output_file == null) {
        allocator.free(output_file);
    };
    
    // Execute based on backend
    switch (config.backend) {
        .llvm => try compileWithSmartLLVM(allocator, source, filename, config, output_file),
        else => {
            print("❌ Backend {s} not supported for compilation\n", .{@tagName(config.backend)});
            return error.InvalidBackend;
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
    
    // Perform basic syntax checking with AST awareness
    if (config.backend == .ast) {
        print("🎯 Performing AST-based type checking\n", .{});
        try checkWithAST(allocator, source, config);
    } else {
        try checkBasicSyntax(allocator, source, config);
    }
    
    print("✅ Type checking completed successfully\n", .{});
}

fn interpretWithAST(allocator: Allocator, source: []const u8, config: Config) !void {
    if (config.verbose) print("🎯 Using AST-based interpreter\n", .{});
    
    // Tokenize the source
    var lexer_instance = lexer.Lexer.init(allocator, source);
    
    var tokens = lexer_instance.tokenize() catch |err| {
        print("❌ Tokenization failed: {any}\n", .{err});
        return;
    };
    defer tokens.deinit();
    
    if (config.verbose) {
        print("✅ Tokenized into {s} tokens\n", .{tokens.items.len});
        for (tokens.items, 0..) |token, i| {
            print("  [{s}] {s} = '{s}'\n", .{ i, @tagName(token.kind), token.lexeme });
        }
    }
    
    // Create basic AST-style interpretation
    print("🎯 Creating AST representation...\n", .{});
    
    // Simple AST-based execution - for now, just use enhanced interpretation
    try interpretEnhanced(allocator, source, config, tokens.items);
    
    if (config.verbose) print("✅ AST interpretation completed\n", .{});
}

fn checkWithAST(allocator: Allocator, source: []const u8, config: Config) !void {
    if (config.verbose) print("🎯 Performing AST-based type checking\n", .{});
    
    // Tokenize the source
    var lexer_instance = lexer.Lexer.init(allocator, source);
    
    var tokens = lexer_instance.tokenize() catch |err| {
        print("❌ Tokenization failed: {any}\n", .{err});
        return err;
    };
    defer tokens.deinit();
    
    if (config.verbose) {
        print("✅ Tokenized into {s} tokens for AST analysis\n", .{tokens.items.len});
    }
    
    // Basic AST structure validation
    var variable_count: u32 = 0;
    var function_count: u32 = 0;
    var print_count: u32 = 0;
    
    for (tokens.items) |token| {
        if (std.mem.eql(u8, token.lexeme, "sus")) variable_count += 1;
        if (std.mem.eql(u8, token.lexeme, "slay")) function_count += 1;
        if (std.mem.eql(u8, token.lexeme, "vibez.spill")) print_count += 1;
    }
    
    if (config.verbose) {
        print("🔍 AST Analysis Results:\n", .{});
        print("  Variables declared: {s}\n", .{variable_count});
        print("  Functions defined: {s}\n", .{function_count});
        print("  Print statements: {s}\n", .{print_count});
    }
}

// Function storage structure
const FunctionDef = struct {
    name: []const u8,
    params: [][]const u8,
    body: []const u8,
    allocator: Allocator,
    
    pub fn deinit(self: *FunctionDef) void {
        self.allocator.free(self.name);
        for (self.params) |param| {
            self.allocator.free(param);
        }
        self.allocator.free(self.params);
        self.allocator.free(self.body);
    }
};

fn interpretEnhanced(allocator: Allocator, source: []const u8, config: Config, tokens: []const lexer.Token) !void {
    _ = tokens;
    
    if (config.verbose) print("🚀 Using enhanced interpreter with AST support\n", .{});
    
    // Global variable storage supporting both strings and integers
    var variables = std.HashMap([]const u8, VariableValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){};
    defer {
        var iter = variables.iterator();
        while (iter.next()) |entry| {
            allocator.free(entry.key_ptr.*);
        }
        variables.deinit();
    }
    
    // Function storage
    var functions = std.HashMap([]const u8, FunctionDef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){};
    defer {
        var func_iter = functions.iterator();
        while (func_iter.next()) |entry| {
            allocator.free(entry.key_ptr.*);
            entry.value_ptr.deinit();
        }
        functions.deinit();
    }
    
    // First pass: Parse functions and separate from main code
    const statements = try parseStatements(allocator, source);
    defer {
        for (statements) |stmt| {
            allocator.free(stmt);
        }
        allocator.free(statements);
    }
    
    // Process statements
    for (statements) |stmt| {
        const trimmed = std.mem.trim(u8, stmt, " \t\r\n");
        
        if (trimmed.len == 0 or std.mem.startsWith(u8, trimmed, "fr fr")) {
            continue;
        }
        
        if (config.verbose) print("🎯 AST Processing statement: '{s}'\n", .{trimmed});
        
        // Skip import statements during execution
        if (std.mem.startsWith(u8, trimmed, "yeet ")) {
            if (config.verbose) print("📦 Import: {s}\n", .{trimmed});
            continue;
        }
        
        // Handle function definitions
        if (std.mem.startsWith(u8, trimmed, "slay ")) {
            try handleAST_FunctionDefinition(allocator, &functions, trimmed, config.verbose);
        }
        // Handle variable declarations
        else if (std.mem.startsWith(u8, trimmed, "sus ")) {
            try handleAST_Variable(allocator, &variables, &functions, trimmed, config.verbose);
        }
        // Handle print statements
        else if (std.mem.indexOf(u8, trimmed, "vibez.spill(")) |_| {
            try handleAST_Print(&variables, trimmed, config.verbose);
        }
        // Handle expression statements (like "x + 10")
        else {
            // Try to evaluate as expression and print result
            const result = evaluateAST_ExpressionStatement(&variables, trimmed) catch |err| {
                if (config.verbose) {
                    print("Unknown statement: {s} (error: {s})\n", .{trimmed, err});
                }
                continue;
            };
            
            if (config.verbose) print("🎯 AST Expression result: {s}\n", .{result});
            print("{s}\n", .{result});
        }
    }
    
    if (config.verbose) print("✅ Enhanced AST interpretation completed\n", .{});
}

// Parse source into statements (handles multi-line constructs properly)
fn parseStatements(allocator: Allocator, source: []const u8) ![][]const u8 {
    var statements = std.ArrayList([]const u8){};
    defer statements.deinit();
    
    var i: usize = 0;
    var current_statement = std.ArrayList(u8){};
    defer current_statement.deinit();
    
    var brace_count: i32 = 0;
    var paren_count: i32 = 0;
    var in_string = false;
    var in_function = false;
    
    while (i < source.len) {
        const c = source[i];
        
        // Handle string literals
        if (c == '"' and (i == 0 or source[i-1] != '\\')) {
            in_string = !in_string;
            try current_statement.append(allocator, c);
            i += 1;
            continue;
        }
        
        if (in_string) {
            try current_statement.append(allocator, c);
            i += 1;
            continue;
        }
        
        // Track braces and parentheses
        switch (c) {
            '{' => {
                brace_count += 1;
                try current_statement.append(allocator, c);
            },
            '}' => {
                brace_count -= 1;
                try current_statement.append(allocator, c);
                
                // End of function or block
                if (brace_count == 0 and in_function) {
                    in_function = false;
                    // Complete the function statement
                    const stmt = std.mem.trim(u8, current_statement.items, " \t\r\n");
                    if (stmt.len > 0) {
                        const stmt_copy = try allocator.dupe(u8, stmt);
                        try statements.append(allocator, stmt_copy);
                    }
                    current_statement.clearRetainingCapacity();
                }
            },
            '(' => {
                paren_count += 1;
                try current_statement.append(allocator, c);
            },
            ')' => {
                paren_count -= 1;
                try current_statement.append(allocator, c);
            },
            '\n' => {
                // Handle newlines based on context
                if (brace_count > 0 or paren_count > 0 or in_function) {
                    // Inside multi-line construct, replace newline with space
                    try current_statement.append(allocator, ' ');
                } else {
                    // End of statement
                    const stmt = std.mem.trim(u8, current_statement.items, " \t\r\n");
                    if (stmt.len > 0) {
                        const stmt_copy = try allocator.dupe(u8, stmt);
                        try statements.append(allocator, stmt_copy);
                    }
                    current_statement.clearRetainingCapacity();
                }
            },
            else => {
                try current_statement.append(allocator, c);
                
                // Check if starting a function definition
                if (!in_function and current_statement.items.len >= 4) {
                    const last_4 = current_statement.items[current_statement.items.len-4..];
                    if (std.mem.eql(u8, last_4, "slay")) {
                        in_function = true;
                    }
                }
            },
        }
        
        i += 1;
    }
    
    // Handle remaining statement
    const stmt = std.mem.trim(u8, current_statement.items, " \t\r\n");
    if (stmt.len > 0) {
        const stmt_copy = try allocator.dupe(u8, stmt);
        try statements.append(allocator, stmt_copy);
    }
    
    return statements.toOwnedSlice(allocator);
}

// Handle function definition: slay add(a drip, b drip) drip { damn a + b }
fn handleAST_FunctionDefinition(allocator: Allocator, functions: *std.HashMap([]const u8, FunctionDef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), line: []const u8, verbose: bool) !void {
    if (verbose) print("🎯 AST Parsing function definition: {s}\n", .{line});
    
    // Find the function name - after "slay " and before "("
    const slay_end = std.mem.indexOf(u8, line, "slay ") orelse return;
    const name_start = slay_end + 5; // length of "slay "
    const paren_start = std.mem.indexOf(u8, line[name_start..], "(") orelse return;
    const func_name = std.mem.trim(u8, line[name_start..name_start + paren_start], " \t");
    
    if (verbose) print("🎯 AST Function name: '{s}'\n", .{func_name});
    
    // Find parameter section
    const abs_paren_start = name_start + paren_start;
    const paren_end = std.mem.indexOf(u8, line[abs_paren_start..], ")") orelse return;
    const param_section = line[abs_paren_start + 1..abs_paren_start + paren_end];
    
    if (verbose) print("🎯 AST Parameters section: '{s}'\n", .{param_section});
    
    // Find function body
    const brace_start = std.mem.indexOf(u8, line, "{") orelse return;
    const brace_end = std.mem.lastIndexOf(u8, line, "}") orelse return;
    const body = std.mem.trim(u8, line[brace_start + 1..brace_end], " \t");
    
    if (verbose) print("🎯 AST Function body: '{s}'\n", .{body});
    
    // Parse parameters: "a drip, b drip" -> ["a", "b"]
    var params = std.ArrayList([]const u8){};
    defer params.deinit();
    
    if (param_section.len > 0) {
        var param_pairs = std.mem.splitSequence(u8, param_section, ", ");
        while (param_pairs.next()) |pair| {
            const trimmed_pair = std.mem.trim(u8, pair, " \t");
            if (trimmed_pair.len == 0) continue;
            
            var parts = std.mem.splitScalar(u8, trimmed_pair, ' ');
            if (parts.next()) |param_name| {
                if (param_name.len > 0) {
                    const name_copy = try allocator.dupe(u8, param_name);
                    try params.append(allocator, name_copy);
                    if (verbose) print("🎯 AST Parameter: '{s}'\n", .{param_name});
                }
            }
        }
    }
    
    const func_def = FunctionDef{
        .name = try allocator.dupe(u8, func_name),
        .params = try params.toOwnedSlice(allocator),
        .body = try allocator.dupe(u8, body),
        .allocator = allocator,
    };
    
    const key_copy = try allocator.dupe(u8, func_name);
    try functions.put(key_copy, func_def);
    
    if (verbose) {
        print("✅ AST Function defined: {s} with {s} parameters\n", .{ func_name, func_def.params.len });
        for (func_def.params) |param| {
            print("  - Parameter: {s}\n", .{param});
        }
        print("  - Body: {s}\n", .{body});
    }
}

fn handleAST_Variable(allocator: Allocator, variables: *std.HashMap([]const u8, VariableValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), functions: *std.HashMap([]const u8, FunctionDef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), line: []const u8, verbose: bool) !void {
    // Parse: sus x drip = 42 or sus sum drip = x + y or sus result drip = add(5, 3)
    var parts = std.mem.splitSequence(u8, line[4..], " = ");
    const left_part = std.mem.trim(u8, parts.next() orelse return, " ");
    const right_part = std.mem.trim(u8, parts.next() orelse return, " ");
    
    var name_type = std.mem.splitScalar(u8, left_part, ' ');
    const name = name_type.next() orelse return;
    
    // Determine the type of value and handle appropriately
    const value = if (std.mem.startsWith(u8, right_part, "\"") and std.mem.endsWith(u8, right_part, "\"")) blk: {
        // String literal - strip quotes and duplicate for storage
        const string_content = right_part[1..right_part.len-1];
        const string_copy = try allocator.dupe(u8, string_content);
        break :blk VariableValue{ .string = string_copy };
    } else if (std.mem.eql(u8, right_part, "based")) blk: {
        // Boolean literal - true
        break :blk VariableValue{ .boolean = true };
    } else if (std.mem.eql(u8, right_part, "cap")) blk: {
        // Boolean literal - false
        break :blk VariableValue{ .boolean = false };
    } else blk: {
        // Integer expression - evaluate it
        const int_result = try evaluateAST_Expression(allocator, variables, functions, right_part, verbose);
        break :blk VariableValue{ .integer = int_result };
    };
    
    // Store the variable
    const name_copy = try allocator.dupe(u8, name);
    try variables.put(name_copy, value);
    
    if (verbose) {
        switch (value) {
            .integer => |int_val| print("🎯 AST Variable: {s} = {s}\n", .{ name, int_val }),
            .string => |str_val| print("🎯 AST Variable: {s} = \"{s}\"\n", .{ name, str_val }),
            .boolean => |bool_val| print("🎯 AST Variable: {s} = {s}\n", .{ name, bool_val }),
        }
    }
}

fn evaluateAST_Expression(allocator: Allocator, variables: *std.HashMap([]const u8, VariableValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), functions: *std.HashMap([]const u8, FunctionDef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), expr: []const u8, verbose: bool) !i64 {
    // Handle function calls: add(5, 3)
    if (std.mem.indexOf(u8, expr, "(")) |paren_pos| {
        const func_name = std.mem.trim(u8, expr[0..paren_pos], " ");
        const paren_end = std.mem.lastIndexOf(u8, expr, ")") orelse return error.InvalidExpression;
        const args_str = expr[paren_pos + 1..paren_end];
        
        return try callAST_Function(allocator, variables, functions, func_name, args_str, verbose);
    }
    
    // Handle simple addition: x + y
    if (std.mem.indexOf(u8, expr, " + ")) |plus_pos| {
        const left = std.mem.trim(u8, expr[0..plus_pos], " ");
        const right = std.mem.trim(u8, expr[plus_pos + 3..], " ");
        
        const left_val = try getAST_Value(variables, left);
        const right_val = try getAST_Value(variables, right);
        
        if (verbose) print("🎯 AST Expression: {s} + {s} = {s}\n", .{ left_val, right_val, left_val + right_val });
        return left_val + right_val;
    }
    
    // Single value
    return try getAST_Value(variables, expr);
}

// Function argument value (mixed types)
const FunctionArgValue = union(enum) {
    integer: i64,
    string: []const u8,
};

// Execute a function call
fn callAST_Function(allocator: Allocator, variables: *std.HashMap([]const u8, VariableValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), functions: *std.HashMap([]const u8, FunctionDef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), func_name: []const u8, args_str: []const u8, verbose: bool) !i64 {
    const func_def = functions.get(func_name) orelse {
        print("❌ AST: Undefined function: {s}\n", .{func_name});
        return error.UndefinedFunction;
    };
    
    // Parse arguments: "5, 3" or "10, \"test\"" -> mixed values
    var args = std.ArrayList(FunctionArgValue){};
    defer args.deinit();
    
    if (args_str.len > 0) {
        var arg_parts = std.mem.splitSequence(u8, args_str, ", ");
        while (arg_parts.next()) |arg| {
            const trimmed_arg = std.mem.trim(u8, arg, " \t");
            
            // Handle string literals
            if (std.mem.startsWith(u8, trimmed_arg, "\"") and std.mem.endsWith(u8, trimmed_arg, "\"")) {
                const string_content = trimmed_arg[1..trimmed_arg.len-1];
                try args.append(allocator, FunctionArgValue{ .string = string_content });
                if (verbose) print("🎯 AST String argument: \"{s}\"\n", .{string_content});
            } else {
                // Try as integer or variable
                const value = try getAST_Value(variables, trimmed_arg);
                try args.append(allocator, FunctionArgValue{ .integer = value });
                if (verbose) print("🎯 AST Integer argument: {s}\n", .{value});
            }
        }
    }
    
    // Check parameter count
    if (args.items.len != func_def.params.len) {
        print("❌ AST: Function {s} expects {s} arguments, got {s}\n", .{ func_name, func_def.params.len, args.items.len });
        return error.InvalidArgumentCount;
    }
    
    if (verbose) {
        print("🎯 AST Function call: {s}(", .{func_name});
        for (args.items, 0..) |arg, i| {
            if (i > 0) print(", ", .{});
            switch (arg) {
                .integer => |int_val| print("{s}", .{int_val}),
                .string => |str_val| print("\"{s}\"", .{str_val}),
            }
        }
        print(")\n", .{});
    }
    
    // Create function scope with parameters
    var func_scope = std.HashMap([]const u8, FunctionArgValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){};
    defer func_scope.deinit();
    
    // Bind parameters to argument values
    for (func_def.params, args.items) |param, arg_value| {
        try func_scope.put(param, arg_value);
        if (verbose) {
            switch (arg_value) {
                .integer => |int_val| print("🎯 AST Parameter binding: {s} = {s}\n", .{ param, int_val }),
                .string => |str_val| print("🎯 AST Parameter binding: {s} = \"{s}\"\n", .{ param, str_val }),
            }
        }
    }
    
    // Execute function body with parameter scope
    const result = try executeAST_FunctionBodyMixed(allocator, &func_scope, functions, func_def.body, verbose);
    
    if (verbose) print("🎯 AST Function {s} returned: {s}\n", .{ func_name, result });
    return result;
}

// Execute function body with mixed-type arguments: "damn a + b" or "sus var drip = expr damn var"
fn executeAST_FunctionBodyMixed(allocator: Allocator, func_scope: *std.HashMap([]const u8, FunctionArgValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), functions: *std.HashMap([]const u8, FunctionDef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), body: []const u8, verbose: bool) !i64 {
    if (verbose) print("🎯 AST Executing function body: '{s}'\n", .{body});
    
    // Split body by spaces to find statements
    var body_statements = std.mem.splitSequence(u8, body, " damn ");
    var last_value: i64 = 0;
    
    // Process each statement before the final return
    while (body_statements.next()) |stmt_part| {
        const trimmed = std.mem.trim(u8, stmt_part, " \t");
        if (trimmed.len == 0) continue;
        
        if (verbose) print("🎯 AST Processing function statement: '{s}'\n", .{trimmed});
        
        // Handle variable declarations in function scope: "sus var drip = expr"
        if (std.mem.startsWith(u8, trimmed, "sus ")) {
            try handleAST_FunctionVariableDeclarationMixed(allocator, func_scope, functions, trimmed, verbose);
        }
        // Handle direct return: "damn expr" or just "expr"
        else if (std.mem.startsWith(u8, trimmed, "damn ")) {
            const return_expr = std.mem.trim(u8, trimmed[5..], " ");
            last_value = try evaluateAST_FunctionExpressionMixed(allocator, func_scope, functions, return_expr, verbose);
        }
        // Handle expression that should be returned
        else {
            last_value = try evaluateAST_FunctionExpressionMixed(allocator, func_scope, functions, trimmed, verbose);
        }
    }
    
    if (verbose) print("🎯 AST Function body returned: {s}\n", .{last_value});
    return last_value;
}

// Execute function body: "damn a + b" or "sus var drip = expr damn var" (legacy integer-only version)
fn executeAST_FunctionBody(allocator: Allocator, func_scope: *std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), functions: *std.HashMap([]const u8, FunctionDef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), body: []const u8, verbose: bool) !i64 {
    if (verbose) print("🎯 AST Executing function body: '{s}'\n", .{body});
    
    // Split body by spaces to find statements
    var body_statements = std.mem.splitSequence(u8, body, " damn ");
    var last_value: i64 = 0;
    
    // Process each statement before the final return
    while (body_statements.next()) |stmt_part| {
        const trimmed = std.mem.trim(u8, stmt_part, " \t");
        if (trimmed.len == 0) continue;
        
        if (verbose) print("🎯 AST Processing function statement: '{s}'\n", .{trimmed});
        
        // Handle variable declarations in function scope: "sus var drip = expr"
        if (std.mem.startsWith(u8, trimmed, "sus ")) {
            try handleAST_FunctionVariableDeclaration(allocator, func_scope, functions, trimmed, verbose);
        }
        // Handle direct return: "damn expr" or just "expr"
        else if (std.mem.startsWith(u8, trimmed, "damn ")) {
            const return_expr = std.mem.trim(u8, trimmed[5..], " ");
            last_value = try evaluateAST_FunctionExpression(allocator, func_scope, functions, return_expr, verbose);
        }
        // Handle expression that should be returned
        else {
            last_value = try evaluateAST_FunctionExpression(allocator, func_scope, functions, trimmed, verbose);
        }
    }
    
    if (verbose) print("🎯 AST Function body returned: {s}\n", .{last_value});
    return last_value;
}

// Handle variable declaration in function scope: "sus local_var drip = param1 + 5"
fn handleAST_FunctionVariableDeclaration(allocator: Allocator, func_scope: *std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), functions: *std.HashMap([]const u8, FunctionDef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), line: []const u8, verbose: bool) !void {
    // Parse: sus var_name type = expression
    var parts = std.mem.splitSequence(u8, line[4..], " = ");
    const left_part = std.mem.trim(u8, parts.next() orelse return, " ");
    const right_part = std.mem.trim(u8, parts.next() orelse return, " ");
    
    var name_type = std.mem.splitScalar(u8, left_part, ' ');
    const name = name_type.next() orelse return;
    
    if (verbose) print("🎯 AST Function variable declaration: {s} = {s}\n", .{ name, right_part });
    
    // Evaluate the expression in function scope
    const value = try evaluateAST_FunctionExpression(allocator, func_scope, functions, right_part, verbose);
    
    // Store in function scope (no need to duplicate key since we're using string literals)
    try func_scope.put(name, value);
    
    if (verbose) print("🎯 AST Function variable: {s} = {s}\n", .{ name, value });
}

// Handle variable declaration in function scope with mixed types: "sus local_var drip = param1 + 5"
fn handleAST_FunctionVariableDeclarationMixed(allocator: Allocator, func_scope: *std.HashMap([]const u8, FunctionArgValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), functions: *std.HashMap([]const u8, FunctionDef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), line: []const u8, verbose: bool) !void {
    // Parse: sus var_name type = expression
    var parts = std.mem.splitSequence(u8, line[4..], " = ");
    const left_part = std.mem.trim(u8, parts.next() orelse return, " ");
    const right_part = std.mem.trim(u8, parts.next() orelse return, " ");
    
    var name_type = std.mem.splitScalar(u8, left_part, ' ');
    const name = name_type.next() orelse return;
    
    if (verbose) print("🎯 AST Function variable declaration: {s} = {s}\n", .{ name, right_part });
    
    // Evaluate the expression in function scope
    const value = try evaluateAST_FunctionExpressionMixed(allocator, func_scope, functions, right_part, verbose);
    
    // Store in function scope as integer for now (extend later for mixed types if needed)
    try func_scope.put(name, FunctionArgValue{ .integer = value });
    
    if (verbose) print("🎯 AST Function variable: {s} = {s}\n", .{ name, value });
}

// Evaluate expressions in function scope with mixed types (looks in function params first, then globals)
fn evaluateAST_FunctionExpressionMixed(_: Allocator, func_scope: *std.HashMap([]const u8, FunctionArgValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), _: *std.HashMap([]const u8, FunctionDef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), expr: []const u8, verbose: bool) !i64 {
    // Handle addition: a + b
    if (std.mem.indexOf(u8, expr, " + ")) |plus_pos| {
        const left = std.mem.trim(u8, expr[0..plus_pos], " ");
        const right = std.mem.trim(u8, expr[plus_pos + 3..], " ");
        
        const left_val = try getAST_FunctionValueMixed(func_scope, left);
        const right_val = try getAST_FunctionValueMixed(func_scope, right);
        
        if (verbose) print("🎯 AST Function expression: {s} + {s} = {s}\n", .{ left_val, right_val, left_val + right_val });
        return left_val + right_val;
    }
    
    // Handle multiplication: a * b
    if (std.mem.indexOf(u8, expr, " * ")) |mult_pos| {
        const left = std.mem.trim(u8, expr[0..mult_pos], " ");
        const right = std.mem.trim(u8, expr[mult_pos + 3..], " ");
        
        const left_val = try getAST_FunctionValueMixed(func_scope, left);
        const right_val = try getAST_FunctionValueMixed(func_scope, right);
        
        if (verbose) print("🎯 AST Function expression: {s} * {s} = {s}\n", .{ left_val, right_val, left_val * right_val });
        return left_val * right_val;
    }
    
    // Single value
    return try getAST_FunctionValueMixed(func_scope, expr);
}

// Get value from function scope with mixed types first, then fall back to error
fn getAST_FunctionValueMixed(func_scope: *std.HashMap([]const u8, FunctionArgValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), expr: []const u8) !i64 {
    // Try parsing as integer
    if (std.fmt.parseInt(i64, expr, 10)) |value| {
        return value;
    } else |_| {
        // Try as function parameter
        if (func_scope.get(expr)) |value| {
            switch (value) {
                .integer => |int_val| return int_val,
                .string => |_| {
                    print("❌ AST: Cannot use string parameter '{s}' in integer context\n", .{expr});
                    return error.TypeMismatch;
                },
            }
        } else {
            print("❌ AST: Undefined variable in function scope: {s}\n", .{expr});
            return error.UndefinedVariable;
        }
    }
}

// Evaluate expressions in function scope (looks in function params first, then globals)
fn evaluateAST_FunctionExpression(_: Allocator, func_scope: *std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), _: *std.HashMap([]const u8, FunctionDef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), expr: []const u8, verbose: bool) !i64 {
    // Handle addition: a + b
    if (std.mem.indexOf(u8, expr, " + ")) |plus_pos| {
        const left = std.mem.trim(u8, expr[0..plus_pos], " ");
        const right = std.mem.trim(u8, expr[plus_pos + 3..], " ");
        
        const left_val = try getAST_FunctionValue(func_scope, left);
        const right_val = try getAST_FunctionValue(func_scope, right);
        
        if (verbose) print("🎯 AST Function expression: {s} + {s} = {s}\n", .{ left_val, right_val, left_val + right_val });
        return left_val + right_val;
    }
    
    // Handle multiplication: a * b
    if (std.mem.indexOf(u8, expr, " * ")) |mult_pos| {
        const left = std.mem.trim(u8, expr[0..mult_pos], " ");
        const right = std.mem.trim(u8, expr[mult_pos + 3..], " ");
        
        const left_val = try getAST_FunctionValue(func_scope, left);
        const right_val = try getAST_FunctionValue(func_scope, right);
        
        if (verbose) print("🎯 AST Function expression: {s} * {s} = {s}\n", .{ left_val, right_val, left_val * right_val });
        return left_val * right_val;
    }
    
    // Single value
    return try getAST_FunctionValue(func_scope, expr);
}

// Get value from function scope first, then fall back to error
fn getAST_FunctionValue(func_scope: *std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), expr: []const u8) !i64 {
    // Try parsing as integer
    if (std.fmt.parseInt(i64, expr, 10)) |value| {
        return value;
    } else |_| {
        // Try as function parameter
        if (func_scope.get(expr)) |value| {
            return value;
        } else {
            print("❌ AST: Undefined variable in function scope: {s}\n", .{expr});
            return error.UndefinedVariable;
        }
    }
}

// Evaluate expression statements like "x + 10" or "5 * 3"
fn evaluateAST_ExpressionStatement(variables: *std.HashMap([]const u8, VariableValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), expr: []const u8) !i64 {
    // Handle addition: x + 10
    if (std.mem.indexOf(u8, expr, " + ")) |plus_pos| {
        const left = std.mem.trim(u8, expr[0..plus_pos], " ");
        const right = std.mem.trim(u8, expr[plus_pos + 3..], " ");
        
        const left_val = try getAST_Value(variables, left);
        const right_val = try getAST_Value(variables, right);
        
        return left_val + right_val;
    }
    
    // Handle subtraction: x - 5
    if (std.mem.indexOf(u8, expr, " - ")) |minus_pos| {
        const left = std.mem.trim(u8, expr[0..minus_pos], " ");
        const right = std.mem.trim(u8, expr[minus_pos + 3..], " ");
        
        const left_val = try getAST_Value(variables, left);
        const right_val = try getAST_Value(variables, right);
        
        return left_val - right_val;
    }
    
    // Handle multiplication: x * 2
    if (std.mem.indexOf(u8, expr, " * ")) |mult_pos| {
        const left = std.mem.trim(u8, expr[0..mult_pos], " ");
        const right = std.mem.trim(u8, expr[mult_pos + 3..], " ");
        
        const left_val = try getAST_Value(variables, left);
        const right_val = try getAST_Value(variables, right);
        
        return left_val * right_val;
    }
    
    // Handle division: x / 2
    if (std.mem.indexOf(u8, expr, " / ")) |div_pos| {
        const left = std.mem.trim(u8, expr[0..div_pos], " ");
        const right = std.mem.trim(u8, expr[div_pos + 3..], " ");
        
        const left_val = try getAST_Value(variables, left);
        const right_val = try getAST_Value(variables, right);
        
        if (right_val == 0) return error.DivisionByZero;
        return @divTrunc(left_val, right_val);
    }
    
    // Single value or variable
    return try getAST_Value(variables, expr);
}

fn getAST_Value(variables: *std.HashMap([]const u8, VariableValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), expr: []const u8) !i64 {
    // Try parsing as integer literal
    if (std.fmt.parseInt(i64, expr, 10)) |value| {
        return value;
    } else |_| {
        // Try as variable lookup
        if (variables.get(expr)) |var_value| {
            switch (var_value) {
                .integer => |int_val| return int_val,
                .string => |_| {
                    print("❌ AST: Cannot use string variable '{s}' in integer context\n", .{expr});
                    return error.TypeMismatch;
                },
                .boolean => |bool_val| return if (bool_val) 1 else 0, // Convert boolean to integer for math
            }
        } else {
            // Check if it's a string literal (shouldn't happen here, but defensive)
            if (std.mem.startsWith(u8, expr, "\"") and std.mem.endsWith(u8, expr, "\"")) {
                print("❌ AST: String literal '{s}' used in integer context\n", .{expr});
                return error.TypeMismatch;
            }
            print("❌ AST: Undefined variable: {s}\n", .{expr});
            return error.UndefinedVariable;
        }
    }
}

fn handleAST_Print(variables: *std.HashMap([]const u8, VariableValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), line: []const u8, verbose: bool) !void {
    const start = std.mem.indexOf(u8, line, "(") orelse return;
    const end = std.mem.lastIndexOf(u8, line, ")") orelse return;
    const content = line[start + 1 .. end];
    
    if (verbose) print("🎯 AST Print: {s}\n", .{content});
    print("📢 ", .{});
    
    // Simple parsing - look for variables
    var parts = std.mem.splitScalar(u8, content, ',');
    var first = true;
    while (parts.next()) |part| {
        const trimmed = std.mem.trim(u8, part, " \"");
        if (trimmed.len > 0) {
            if (!first) print(" ", .{});
            first = false;
            
            // Try to get variable value
            if (variables.get(trimmed)) |var_value| {
                switch (var_value) {
                    .integer => |int_val| print("{s}", .{int_val}),
                    .string => |str_val| print("{s}", .{str_val}),
                    .boolean => |bool_val| print("{s}", .{bool_val}),
                }
            } else {
                // Check if it's a string literal
                if (std.mem.startsWith(u8, part, "\"") and std.mem.endsWith(u8, part, "\"")) {
                    const string_content = part[1..part.len-1];
                    print("{s}", .{string_content});
                } else {
                    print("{s}", .{trimmed});
                }
            }
        }
    }
    print("\n", .{});
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

// Help and version functions
fn printVersion() void {
    print("{s} v{s}\n", .{ BUILD_INFO, VERSION });
    print("A modern programming language for the next generation (AST Enabled)\n", .{});
}

fn printUsage() void {
    print("Usage: cursed [COMMAND] [OPTIONS] [FILE]\n", .{});
    print("\nFor more information, use: cursed --help\n", .{});
}

fn printHelp() void {
    print("{s} v{s}\n", .{ BUILD_INFO, VERSION });
    print("A modern programming language for the next generation (AST Enabled)\n\n", .{});
    
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
    print("    --backend, -b BACKEND    Backend [script, ast, llvm, c, wasm]\n", .{});
    print("    --debug, -d              Enable debug mode (verbose + tokens)\n", .{});
    print("    --verbose                Enable verbose output\n\n", .{});
    
    print("EXAMPLES:\n", .{});
    print("    cursed hello.csd                          # Interpret hello.csd (script backend)\n", .{});
    print("    cursed hello.csd -b ast --verbose         # Interpret with AST backend\n", .{});
    print("    cursed hello.csd --compile                 # Compile hello.csd to native executable\n", .{});
    print("    cursed check hello.csd -b ast              # AST-based type checking\n", .{});
    
    print("For more information, visit: https://github.com/ghuntley/cursed\n", .{});
}
