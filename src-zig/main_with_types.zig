/// Enhanced CURSED compiler main with comprehensive type system integration
const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast.zig");
const simple_import_resolver = @import("simple_import_resolver.zig");
const simple_compiler = @import("simple_compiler.zig");
const comprehensive_type_system = @import("comprehensive_type_system.zig");
const type_checker_integration = @import("type_checker_integration.zig");

// Version information
const VERSION = "1.0.0-types";
const BUILD_INFO = "CURSED Unified Compiler with Advanced Type System";

// Command enumeration
const Command = enum {
    interpret,
    compile,
    check,
    format,
    test_cmd,
    version,
    help,
    // Type system commands
    typecheck,
    infer,
    constraints,
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
    show_ast: bool = false,
    show_types: bool = false,
    strict_types: bool = true,
    enable_generics: bool = true,
    enable_inference: bool = true,
    stdlib_path: ?[]const u8 = null,
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    const config = try parseArgs(allocator, args);

    switch (config.command) {
        .help => {
            printHelp();
            return;
        },
        .version => {
            print("{s} version {s}\n", .{ BUILD_INFO, VERSION });
            return;
        },
        .typecheck => {
            if (config.source_file) |source_file| {
                try runTypeCheck(allocator, source_file, config);
            } else {
                print("Error: typecheck command requires a source file\n", .{});
                std.process.exit(1);
            }
        },
        .infer => {
            if (config.source_file) |source_file| {
                try runTypeInference(allocator, source_file, config);
            } else {
                print("Error: infer command requires a source file\n", .{});
                std.process.exit(1);
            }
        },
        .constraints => {
            if (config.source_file) |source_file| {
                try runConstraintResolution(allocator, source_file, config);
            } else {
                print("Error: constraints command requires a source file\n", .{});
                std.process.exit(1);
            }
        },
        .check => {
            if (config.source_file) |source_file| {
                try runSyntaxCheck(allocator, source_file, config);
            } else {
                print("Error: check command requires a source file\n", .{});
                std.process.exit(1);
            }
        },
        .interpret => {
            if (config.source_file) |source_file| {
                try runInterpret(allocator, source_file, config);
            } else {
                print("Error: interpret command requires a source file\n", .{});
                std.process.exit(1);
            }
        },
        .compile => {
            if (config.source_file) |source_file| {
                try runCompile(allocator, source_file, config);
            } else {
                print("Error: compile command requires a source file\n", .{});
                std.process.exit(1);
            }
        },
        else => {
            print("Command not implemented yet: {s}\n", .{@tagName(config.command)});
            std.process.exit(1);
        },
    }
}

fn parseArgs(allocator: Allocator, args: [][]const u8) !Config {
    var config = Config{};
    
    if (args.len < 2) {
        config.command = .help;
        return config;
    }

    var i: usize = 1;
    while (i < args.len) {
        const arg = args[i];
        
        if (std.mem.eql(u8, arg, "--help") or std.mem.eql(u8, arg, "-h")) {
            config.command = .help;
            return config;
        } else if (std.mem.eql(u8, arg, "--version") or std.mem.eql(u8, arg, "-v")) {
            config.command = .version;
            return config;
        } else if (std.mem.eql(u8, arg, "typecheck")) {
            config.command = .typecheck;
        } else if (std.mem.eql(u8, arg, "infer")) {
            config.command = .infer;
        } else if (std.mem.eql(u8, arg, "constraints")) {
            config.command = .constraints;
        } else if (std.mem.eql(u8, arg, "check")) {
            config.command = .check;
        } else if (std.mem.eql(u8, arg, "interpret")) {
            config.command = .interpret;
        } else if (std.mem.eql(u8, arg, "compile")) {
            config.command = .compile;
        } else if (std.mem.eql(u8, arg, "--verbose")) {
            config.verbose = true;
        } else if (std.mem.eql(u8, arg, "--debug")) {
            config.debug_mode = true;
        } else if (std.mem.eql(u8, arg, "--tokens")) {
            config.show_tokens = true;
        } else if (std.mem.eql(u8, arg, "--ast")) {
            config.show_ast = true;
        } else if (std.mem.eql(u8, arg, "--types")) {
            config.show_types = true;
        } else if (std.mem.eql(u8, arg, "--no-strict-types")) {
            config.strict_types = false;
        } else if (std.mem.eql(u8, arg, "--no-generics")) {
            config.enable_generics = false;
        } else if (std.mem.eql(u8, arg, "--no-inference")) {
            config.enable_inference = false;
        } else if (std.mem.eql(u8, arg, "--backend")) {
            i += 1;
            if (i >= args.len) {
                print("Error: --backend requires a value\n", .{});
                std.process.exit(1);
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
                print("Error: unknown backend '{s}'\n", .{backend_str});
                std.process.exit(1);
            }
        } else if (std.mem.eql(u8, arg, "--output") or std.mem.eql(u8, arg, "-o")) {
            i += 1;
            if (i >= args.len) {
                print("Error: --output requires a value\n", .{});
                std.process.exit(1);
            }
            config.output_file = args[i];
        } else if (std.mem.eql(u8, arg, "--stdlib-path")) {
            i += 1;
            if (i >= args.len) {
                print("Error: --stdlib-path requires a value\n", .{});
                std.process.exit(1);
            }
            config.stdlib_path = args[i];
        } else if (std.mem.startsWith(u8, arg, "-")) {
            print("Error: unknown option '{s}'\n", .{arg});
            std.process.exit(1);
        } else {
            // This should be the source file
            if (config.source_file == null) {
                config.source_file = arg;
            } else {
                print("Error: multiple source files not supported\n", .{});
                std.process.exit(1);
            }
        }
        
        i += 1;
    }

    // If no command specified but source file given, default to interpret
    if (config.command == .interpret and config.source_file == null and args.len >= 2) {
        config.source_file = args[1];
    }

    return config;
}

fn printHelp() void {
    print("CURSED Compiler with Advanced Type System v{s}\n", .{VERSION});
    print("\n", .{});
    print("USAGE:\n", .{});
    print("    cursed [COMMAND] [OPTIONS] <source-file>\n", .{});
    print("\n", .{});
    print("COMMANDS:\n", .{});
    print("    interpret    Interpret CURSED source code (default)\n", .{});
    print("    compile      Compile to native binary\n", .{});
    print("    check        Check syntax without execution\n", .{});
    print("    typecheck    Perform comprehensive type checking\n", .{});
    print("    infer        Show type inference results\n", .{});
    print("    constraints  Resolve and display type constraints\n", .{});
    print("    version      Show version information\n", .{});
    print("    help         Show this help message\n", .{});
    print("\n", .{});
    print("OPTIONS:\n", .{});
    print("    --backend <backend>  Choose compilation backend (script, llvm, c, wasm)\n", .{});
    print("    --output, -o <file>  Specify output file\n", .{});
    print("    --verbose            Enable verbose output\n", .{});
    print("    --debug              Enable debug mode\n", .{});
    print("    --tokens             Show tokenization results\n", .{});
    print("    --ast                Show AST structure\n", .{});
    print("    --types              Show detailed type information\n", .{});
    print("    --no-strict-types    Disable strict type checking\n", .{});
    print("    --no-generics        Disable generic type support\n", .{});
    print("    --no-inference       Disable type inference\n", .{});
    print("    --stdlib-path <path> Specify custom standard library path\n", .{});
    print("\n", .{});
    print("EXAMPLES:\n", .{});
    print("    cursed hello.csd                    # Interpret file\n", .{});
    print("    cursed compile hello.csd -o hello   # Compile to native binary\n", .{});
    print("    cursed typecheck program.csd        # Check types thoroughly\n", .{});
    print("    cursed infer --types expression.csd # Show type inference\n", .{});
    print("    cursed check --verbose program.csd  # Syntax check with details\n", .{});
    print("\n", .{});
}

fn runTypeCheck(allocator: Allocator, source_file: []const u8, config: Config) !void {
    if (config.verbose) {
        print("🔍 Running comprehensive type check on: {s}\n", .{source_file});
    }

    // Read source file
    const file_content = std.fs.cwd().readFileAlloc(allocator, source_file, std.math.maxInt(usize)) catch |err| {
        print("Error reading file '{s}': {s}\n", .{ source_file, err });
        std.process.exit(1);
    };
    defer allocator.free(file_content);

    // Tokenize
    var token_list = lexer.tokenize(allocator, file_content) catch |err| {
        print("Tokenization error: {s}\n", .{err});
        std.process.exit(1);
    };
    defer token_list.deinit();

    if (config.show_tokens) {
        print("\n📝 Tokens:\n", .{});
        for (token_list.items, 0..) |token, i| {
            print("{s}: {s} '{s}'\n", .{ i, token.type, token.value });
        }
        print("\n", .{});
    }

    // Parse
    var cursed_parser = parser.Parser.init(allocator, token_list.items);
    var program = cursed_parser.parseProgram() catch |err| {
        print("Parse error: {s}\n", .{err});
        std.process.exit(1);
    };
    defer program.deinit();

    if (config.show_ast) {
        print("\n🌳 AST Structure:\n", .{});
        try printAST(program, 0);
        print("\n", .{});
    }

    // Type check with comprehensive system
    var type_integration = type_checker_integration.TypeCheckerIntegration.init(allocator) catch |err| {
        print("Type checker initialization error: {s}\n", .{err});
        std.process.exit(1);
    };
    defer type_integration.deinit();

    const type_result = type_integration.checkProgram(&program) catch |err| {
        print("Type checking error: {s}\n", .{err});
        std.process.exit(1);
    };
    defer {
        type_result.errors.deinit();
        type_result.warnings.deinit();
    }

    // Display results
    if (type_result.success) {
        print("✅ Type checking passed successfully!\n", .{});
        
        if (config.show_types) {
            print("\n📊 Type Information:\n", .{});
            print("- Strict type checking: {s}\n", .{config.strict_types});
            print("- Generic types enabled: {s}\n", .{config.enable_generics});
            print("- Type inference enabled: {s}\n", .{config.enable_inference});
        }
        
        if (type_result.warnings.items.len > 0) {
            print("\n⚠️  Warnings:\n", .{});
            for (type_result.warnings.items) |warning| {
                print("  Line {s}, Column {s}: {s}\n", .{ warning.line, warning.column, warning.message });
            }
        }
    } else {
        print("❌ Type checking failed!\n", .{});
        
        if (type_result.errors.items.len > 0) {
            print("\n🚨 Type Errors:\n", .{});
            for (type_result.errors.items, 0..) |error_detail, i| {
                print("{s}. [{s}] Line {s}, Column {s}: {s}\n", .{
                    i + 1,
                    @tagName(error_detail.kind),
                    error_detail.line,
                    error_detail.column,
                    error_detail.message,
                });
            }
        }
        
        std.process.exit(1);
    }
}

fn runTypeInference(allocator: Allocator, source_file: []const u8, config: Config) !void {
    if (config.verbose) {
        print("🧠 Running type inference analysis on: {s}\n", .{source_file});
    }

    // Read and parse source file (similar to type check)
    const file_content = std.fs.cwd().readFileAlloc(allocator, source_file, std.math.maxInt(usize)) catch |err| {
        print("Error reading file '{s}': {s}\n", .{ source_file, err });
        std.process.exit(1);
    };
    defer allocator.free(file_content);

    var token_list = lexer.tokenize(allocator, file_content) catch |err| {
        print("Tokenization error: {s}\n", .{err});
        std.process.exit(1);
    };
    defer token_list.deinit();

    var cursed_parser = parser.Parser.init(allocator, token_list.items);
    var program = cursed_parser.parseProgram() catch |err| {
        print("Parse error: {s}\n", .{err});
        std.process.exit(1);
    };
    defer program.deinit();

    // Initialize type system for inference analysis
    var type_integration = type_checker_integration.TypeCheckerIntegration.init(allocator) catch |err| {
        print("Type system initialization error: {s}\n", .{err});
        std.process.exit(1);
    };
    defer type_integration.deinit();

    print("📋 Type Inference Results:\n", .{});
    print("========================\n", .{});

    // Analyze each statement for type inference
    for (program.statements.items, 0..) |stmt, i| {
        print("\nStatement {s}:\n", .{i + 1});
        
        switch (stmt) {
            .VariableDeclaration => |var_decl| {
                print("  Variable: {s}\n", .{var_decl.name});
                if (var_decl.init_value) |init_val| {
                    const expr_result = type_integration.checkExpression(init_val) catch |err| {
                        print("  Inference error: {s}\n", .{err});
                        continue;
                    };
                    
                    print("  Inferred type: {s}\n", .{expr_result.inferred_type});
                    print("  Inference successful: {s}\n", .{expr_result.success});
                    
                    if (expr_result.error_message) |err_msg| {
                        print("  Error: {s}\n", .{err_msg});
                    }
                }
            },
            .FunctionDeclaration => |func_decl| {
                print("  Function: {s}\n", .{func_decl.name});
                print("  Parameter count: {s}\n", .{func_decl.parameters.items.len});
                print("  Has return type: {s}\n", .{func_decl.return_type != null});
                
                // Check if function is generic
                const generic_result = type_integration.checkGenericFunction(&func_decl) catch |err| {
                    print("  Generic check error: {s}\n", .{err});
                    continue;
                };
                defer generic_result.deinit();
                
                print("  Is generic: {s}\n", .{generic_result.is_generic});
                print("  Monomorphization needed: {s}\n", .{generic_result.monomorphization_needed});
            },
            .Expression => |expr_stmt| {
                print("  Expression statement\n", .{});
                const expr_result = type_integration.checkExpression(expr_stmt.expression) catch |err| {
                    print("  Inference error: {s}\n", .{err});
                    continue;
                };
                
                print("  Inferred type: {s}\n", .{expr_result.inferred_type});
                print("  Inference successful: {s}\n", .{expr_result.success});
            },
            else => {
                print("  Type: {s}\n", .{@tagName(stmt)});
            },
        }
    }

    print("\n✨ Type inference analysis complete!\n", .{});
}

fn runConstraintResolution(allocator: Allocator, source_file: []const u8, config: Config) !void {
    if (config.verbose) {
        print("🔗 Running constraint resolution analysis on: {s}\n", .{source_file});
    }

    // Similar setup to other type operations
    const file_content = std.fs.cwd().readFileAlloc(allocator, source_file, std.math.maxInt(usize)) catch |err| {
        print("Error reading file '{s}': {s}\n", .{ source_file, err });
        std.process.exit(1);
    };
    defer allocator.free(file_content);

    var token_list = lexer.tokenize(allocator, file_content) catch |err| {
        print("Tokenization error: {s}\n", .{err});
        std.process.exit(1);
    };
    defer token_list.deinit();

    var cursed_parser = parser.Parser.init(allocator, token_list.items);
    var program = cursed_parser.parseProgram() catch |err| {
        print("Parse error: {s}\n", .{err});
        std.process.exit(1);
    };
    defer program.deinit();

    var type_integration = type_checker_integration.TypeCheckerIntegration.init(allocator) catch |err| {
        print("Type system initialization error: {s}\n", .{err});
        std.process.exit(1);
    };
    defer type_integration.deinit();

    // Run constraint resolution
    const constraint_result = type_integration.resolveConstraints() catch |err| {
        print("Constraint resolution error: {s}\n", .{err});
        std.process.exit(1);
    };
    defer constraint_result.deinit();

    print("🔍 Constraint Resolution Results:\n", .{});
    print("=================================\n", .{});
    print("Resolution successful: {s}\n", .{constraint_result.success});
    print("Resolved constraints: {s}\n", .{constraint_result.resolved_constraints.items.len});
    print("Remaining unknowns: {s}\n", .{constraint_result.remaining_unknowns});

    if (constraint_result.resolved_constraints.items.len > 0) {
        print("\n📋 Resolved Type Variables:\n", .{});
        for (constraint_result.resolved_constraints.items, 0..) |resolved, i| {
            print("{s}. T{s} = {s}\n", .{ i + 1, resolved.type_var_id, resolved.resolved_type });
        }
    }

    if (constraint_result.remaining_unknowns > 0) {
        print("\n⚠️  {s} type variables remain unresolved\n", .{constraint_result.remaining_unknowns});
        print("This may indicate incomplete type information or circular constraints.\n", .{});
    }

    print("\n✨ Constraint resolution analysis complete!\n", .{});
}

fn runSyntaxCheck(allocator: Allocator, source_file: []const u8, config: Config) !void {
    if (config.verbose) {
        print("📝 Running syntax check on: {s}\n", .{source_file});
    }

    const file_content = std.fs.cwd().readFileAlloc(allocator, source_file, std.math.maxInt(usize)) catch |err| {
        print("Error reading file '{s}': {s}\n", .{ source_file, err });
        std.process.exit(1);
    };
    defer allocator.free(file_content);

    var token_list = lexer.tokenize(allocator, file_content) catch |err| {
        print("Tokenization error: {s}\n", .{err});
        std.process.exit(1);
    };
    defer token_list.deinit();

    var cursed_parser = parser.Parser.init(allocator, token_list.items);
    var program = cursed_parser.parseProgram() catch |err| {
        print("Parse error: {s}\n", .{err});
        std.process.exit(1);
    };
    defer program.deinit();

    print("✅ Syntax check passed successfully!\n", .{});
    print("📊 Program statistics:\n", .{});
    print("  - Statements: {s}\n", .{program.statements.items.len});
    
    // Count different statement types
    var function_count: u32 = 0;
    var variable_count: u32 = 0;
    var struct_count: u32 = 0;
    var interface_count: u32 = 0;
    
    for (program.statements.items) |stmt| {
        switch (stmt) {
            .FunctionDeclaration => function_count += 1,
            .VariableDeclaration => variable_count += 1,
            .StructDeclaration => struct_count += 1,
            .InterfaceDeclaration => interface_count += 1,
            else => {},
        }
    }
    
    print("  - Functions: {s}\n", .{function_count});
    print("  - Variables: {s}\n", .{variable_count});
    print("  - Structs: {s}\n", .{struct_count});
    print("  - Interfaces: {s}\n", .{interface_count});
}

fn runInterpret(allocator: Allocator, source_file: []const u8, config: Config) !void {
    if (config.verbose) {
        print("🚀 Interpreting: {s}\n", .{source_file});
    }

    // Use existing simple compiler for interpretation
    try simple_compiler.compileAndRun(allocator, source_file, config.stdlib_path, config.verbose);
}

fn runCompile(allocator: Allocator, source_file: []const u8, config: Config) !void {
    if (config.verbose) {
        print("🔨 Compiling: {s} with backend: {s}\n", .{ source_file, @tagName(config.backend) });
    }

    switch (config.backend) {
        .script => {
            print("Note: Script backend is interpretation mode\n", .{});
            try runInterpret(allocator, source_file, config);
        },
        .llvm => {
            print("LLVM compilation not yet implemented in enhanced compiler\n", .{});
            print("Use 'cursed-zig' for LLVM compilation\n", .{});
            std.process.exit(1);
        },
        .c => {
            print("C transpilation not yet implemented\n", .{});
            std.process.exit(1);
        },
        .wasm => {
            print("WebAssembly compilation not yet implemented\n", .{});
            std.process.exit(1);
        },
    }
}

fn printAST(program: ast.Program, indent: u32) !void {
    const indent_str = "  " ** 10; // Max depth of 10 for display
    const actual_indent = if (indent > 10) 10 else indent;
    
    for (program.statements.items, 0..) |stmt, i| {
        print("{s}Statement {s}: {s}\n", .{ indent_str[0..(actual_indent * 2)], i, @tagName(stmt) });
        
        switch (stmt) {
            .FunctionDeclaration => |func| {
                print("{s}  Function: {s}\n", .{ indent_str[0..(actual_indent * 2)], func.name });
                print("{s}  Parameters: {s}\n", .{ indent_str[0..(actual_indent * 2)], func.parameters.items.len });
                print("{s}  Body statements: {s}\n", .{ indent_str[0..(actual_indent * 2)], func.body.items.len });
            },
            .VariableDeclaration => |var_decl| {
                print("{s}  Variable: {s}\n", .{ indent_str[0..(actual_indent * 2)], var_decl.name });
                print("{s}  Mutable: {s}\n", .{ indent_str[0..(actual_indent * 2)], var_decl.is_mutable });
                print("{s}  Has explicit type: {s}\n", .{ indent_str[0..(actual_indent * 2)], var_decl.var_type != null });
                print("{s}  Has initializer: {s}\n", .{ indent_str[0..(actual_indent * 2)], var_decl.init_value != null });
            },
            .StructDeclaration => |struct_decl| {
                print("{s}  Struct: {s}\n", .{ indent_str[0..(actual_indent * 2)], struct_decl.name });
                print("{s}  Fields: {s}\n", .{ indent_str[0..(actual_indent * 2)], struct_decl.fields.items.len });
            },
            else => {},
        }
    }
}
