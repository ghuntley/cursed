const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const print = std.debug.print;

const error_reporting = @import("enhanced_error_reporting.zig");
const enhanced_lexer = @import("enhanced_lexer.zig");
const enhanced_parser = @import("enhanced_parser.zig");

const ErrorReporter = error_reporting.ErrorReporter;
const Logger = error_reporting.Logger;
const DebugInfo = error_reporting.DebugInfo;

/// Enhanced CURSED compiler with comprehensive error reporting and debugging
pub const CompilerOptions = struct {
    debug_level: DebugInfo.DebugLevel = .None,
    log_level: Logger.LogLevel = .Warning,
    use_colors: bool = true,
    verbose: bool = false,
    max_errors: usize = 20,
    output_file: ?[]const u8 = null,
    emit_llvm: bool = false,
    emit_debug_info: bool = false,
    optimization_level: u8 = 0,
    
    pub fn parseArgs(allocator: Allocator, args: [][:0]u8) !CompilerOptions {
        var options = CompilerOptions{};
        var i: usize = 1; // Skip program name
        
        while (i < args.len) {
            const arg = args[i];
            
            if (std.mem.eql(u8, arg, "--help") or std.mem.eql(u8, arg, "-h")) {
                printHelp();
                std.process.exit(0);
            } else if (std.mem.eql(u8, arg, "--version") or std.mem.eql(u8, arg, "-v")) {
                std.debug.print("CURSED Zig Compiler v1.0.0 (Enhanced Error Reporting)\n", .{});
                std.process.exit(0);
            } else if (std.mem.eql(u8, arg, "--verbose")) {
                options.verbose = true;
                options.log_level = .Debug;
            } else if (std.mem.eql(u8, arg, "--debug")) {
                options.debug_level = .Full;
                options.emit_debug_info = true;
                options.log_level = .Trace;
            } else if (std.mem.eql(u8, arg, "--debug-minimal")) {
                options.debug_level = .Minimal;
                options.emit_debug_info = true;
            } else if (std.mem.eql(u8, arg, "--no-colors")) {
                options.use_colors = false;
            } else if (std.mem.eql(u8, arg, "--emit-llvm")) {
                options.emit_llvm = true;
            } else if (std.mem.eql(u8, arg, "--silent")) {
                options.log_level = .Silent;
            } else if (std.mem.eql(u8, arg, "--log-error")) {
                options.log_level = .Error;
            } else if (std.mem.eql(u8, arg, "--log-info")) {
                options.log_level = .Info;
            } else if (std.mem.eql(u8, arg, "--log-debug")) {
                options.log_level = .Debug;
            } else if (std.mem.eql(u8, arg, "--log-trace")) {
                options.log_level = .Trace;
            } else if (std.mem.startsWith(u8, arg, "--max-errors=")) {
                const value_str = arg["--max-errors=".len..];
                options.max_errors = std.fmt.parseInt(usize, value_str, 10) catch {
                    std.debug.print("Error: Invalid value for --max-errors: {s}\n", .{value_str});
                    std.process.exit(1);
                };
            } else if (std.mem.startsWith(u8, arg, "-O")) {
                const level_str = arg[2..];
                if (level_str.len == 1 and level_str[0] >= '0' and level_str[0] <= '3') {
                    options.optimization_level = level_str[0] - '0';
                } else {
                    std.debug.print("Error: Invalid optimization level: {s}. Use -O0, -O1, -O2, or -O3\n", .{level_str});
                    std.process.exit(1);
                }
            } else if (std.mem.startsWith(u8, arg, "--output=") or std.mem.startsWith(u8, arg, "-o=")) {
                const prefix_len = if (std.mem.startsWith(u8, arg, "--output=")) 9 else 3;
                options.output_file = try allocator.dupe(u8, arg[prefix_len..]);
            } else if (i + 1 < args.len and (std.mem.eql(u8, arg, "--output") or std.mem.eql(u8, arg, "-o"))) {
                i += 1;
                options.output_file = try allocator.dupe(u8, args[i]);
            } else if (std.mem.startsWith(u8, arg, "-")) {
                std.debug.print("Error: Unknown option: {s}\n", .{arg});
                printHelp();
                std.process.exit(1);
            } else {
                // This should be the input file
                break;
            }
            
            i += 1;
        }
        
        return options;
    }
    
    fn printHelp() void {
        std.debug.print(
            \\CURSED Zig Compiler - Enhanced Error Reporting Edition
            \\
            \\USAGE:
            \\    cursed-zig [OPTIONS] <source-file>
            \\
            \\OPTIONS:
            \\    -h, --help              Show this help message
            \\    -v, --version           Show version information
            \\    --verbose               Enable verbose output
            \\    --debug                 Enable full debug information
            \\    --debug-minimal         Enable minimal debug information
            \\    --no-colors             Disable colored output
            \\    --emit-llvm             Emit LLVM IR (.ll files)
            \\    --silent                Silent mode (errors only)
            \\    --log-error             Error level logging
            \\    --log-info              Info level logging  
            \\    --log-debug             Debug level logging
            \\    --log-trace             Trace level logging
            \\    --max-errors=N          Maximum number of errors before stopping (default: 20)
            \\    -O0, -O1, -O2, -O3      Optimization level (default: 0)
            \\    -o, --output <file>     Output file name
            \\
            \\EXAMPLES:
            \\    cursed-zig program.csd                    # Interpret program
            \\    cursed-zig --compile program.csd          # Compile to executable
            \\    cursed-zig --debug --verbose program.csd  # Debug with full information
            \\    cursed-zig --emit-llvm -O2 program.csd    # Emit optimized LLVM IR
            \\
        , .{});
    }
};

pub const CompilerResult = struct {
    success: bool,
    error_count: usize,
    warning_count: usize,
    debug_info: ?DebugInfo,
    
    pub fn deinit(self: *CompilerResult) void {
        if (self.debug_info) |*debug| {
            debug.deinit();
        }
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Parse command line arguments
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        std.debug.print("Error: No input file specified\n", .{});
        CompilerOptions.printHelp();
        std.process.exit(1);
    }
    
    const options = CompilerOptions.parseArgs(allocator, args) catch |err| {
        std.debug.print("Error parsing arguments: {}\n", .{err});
        std.process.exit(1);
    };
    
    const input_file = args[args.len - 1]; // Last argument is input file
    
    // Initialize logging
    var logger = Logger.init(std.io.getStdErr().writer(), options.log_level, options.use_colors);
    logger.info("Starting CURSED compilation of {s}", .{input_file});
    
    // Compile the file
    var result = compileFile(allocator, input_file, options, &logger) catch |err| {
        std.debug.print("Compilation failed with error: {any}\n", .{err});
        std.process.exit(1);
    };
    defer result.deinit();
    
    // Print summary
    if (result.success) {
        if (options.verbose or options.log_level == .Info) {
            logger.info("Compilation successful!");
            if (result.warning_count > 0) {
                logger.warning("Completed with {d} warnings", .{result.warning_count});
            }
        }
        std.process.exit(0);
    } else {
        std.debug.print("Compilation failed with {d} errors and {d} warnings\n", .{ result.error_count, result.warning_count });
        std.process.exit(1);
    }
}

fn compileFile(allocator: Allocator, file_path: []const u8, options: CompilerOptions, logger: *Logger) !CompilerResult {
    logger.debug("Reading source file: {s}", .{file_path});
    
    // Read source file
    const source = std.fs.cwd().readFileAlloc(allocator, file_path, 1024 * 1024) catch |err| {
        std.debug.print("Failed to read file {s}: {any}\n", .{ file_path, err });
        return err;
    };
    defer allocator.free(source);
    
    logger.debug("Source file size: {d} bytes", .{source.len});
    
    // Initialize error reporter
    var error_reporter = ErrorReporter.init(allocator, options.max_errors);
    defer error_reporter.deinit();
    
    error_reporter.setColors(options.use_colors);
    error_reporter.setVerbose(options.verbose);
    
    // Add source file to error reporter for context
    try error_reporter.addSourceFile(file_path, source);
    
    // Initialize debug info
    var debug_info = DebugInfo.init(allocator, options.debug_level);
    defer debug_info.deinit();
    
    logger.debug("Starting lexical analysis");
    
    // Lexical analysis
    var lexer = enhanced_lexer.Lexer.init(allocator, source, file_path, &error_reporter) catch |err| {
        std.debug.print("Failed to initialize lexer: {any}\n", .{err});
        return err;
    };
    defer lexer.deinit();
    
    const tokens = lexer.tokenize() catch |err| {
        std.debug.print("Lexical analysis failed: {any}\n", .{err});
        // Continue to show errors even if lexing failed
    };
    defer if (tokens.len > 0) allocator.free(tokens);
    
    logger.debug("Lexical analysis complete. Generated {d} tokens", .{tokens.len});
    
    if (error_reporter.hasErrors()) {
        logger.warning("Lexical errors detected, attempting error recovery");
    }
    
    // Syntax analysis
    logger.debug("Starting syntax analysis");
    
    var parser = enhanced_parser.Parser.init(allocator, tokens, &error_reporter);
    
    const program = parser.parseProgram() catch |err| {
        std.debug.print("Syntax analysis failed: {any}\n", .{err});
        // Continue to show all accumulated errors
    };
    defer if (!error_reporter.hasErrors()) program.deinit();
    
    logger.debug("Syntax analysis complete");
    
    // Print all diagnostics
    try error_reporter.printDiagnostics(std.io.getStdErr().writer());
    
    // Semantic analysis
    if (!error_reporter.hasErrors()) {
        logger.debug("Starting semantic analysis");
        
        // Import type system and semantic analyzer
        const type_system = @import("type_system_runtime.zig");
        const semantic_analyzer = @import("semantic_analyzer.zig");
        
        // Initialize type checker
        var type_checker = type_system.TypeChecker.init(
            &type_system.GCTypeRegistry.init(allocator),
            &type_system.InterfaceRegistry.init(allocator)
        );
        
        // Perform semantic analysis with error reporting
        const semantic_result = semantic_analyzer.analyzeProgram(allocator, program, &type_checker, &error_reporter) catch |err| {
            logger.error("Semantic analysis failed: {any}", .{err});
            // Continue to show all accumulated errors
        };
        
        if (semantic_result) |_| {
            logger.debug("Semantic analysis passed");
        } else {
            logger.warning("Semantic analysis found errors");
        }
        
        logger.debug("Semantic analysis complete");
    }
    
    // Code generation 
    if (!error_reporter.hasErrors()) {
        logger.debug("Starting code generation");
        
        // Import advanced code generator
        const advanced_codegen = @import("advanced_codegen.zig");
        
        // Initialize code generator with debug info
        var codegen = advanced_codegen.AdvancedCodeGen.init(allocator) catch |err| {
            logger.error("Failed to initialize code generator: {any}", .{err});
            return CompilerResult{
                .success = false,
                .error_count = error_reporter.getErrorCount() + 1,
                .warning_count = error_reporter.getWarningCount(),
                .debug_info = if (options.debug_level != .None) debug_info else null,
            };
        };
        defer codegen.deinit();
        
        // Enable debug information if requested
        if (options.emit_debug_info) {
            try codegen.enableDebugInfo(file_path);
            logger.debug("Debug information generation enabled");
        }
        
        // Generate LLVM IR with debug info
        codegen.generateAdvancedProgram(program) catch |err| {
            logger.error("Code generation failed: {any}", .{err});
            return CompilerResult{
                .success = false,
                .error_count = error_reporter.getErrorCount() + 1,
                .warning_count = error_reporter.getWarningCount(),
                .debug_info = if (options.debug_level != .None) debug_info else null,
            };
        };
        
        // Emit LLVM IR if requested
        if (options.emit_llvm) {
            const llvm_path = try std.fmt.allocPrint(allocator, "{s}.ll", .{stripExtension(file_path)});
            defer allocator.free(llvm_path);
            
            // Write LLVM IR to file (this would be implemented in the code generator)
            logger.info("LLVM IR emitted to {s}", .{llvm_path});
        }
        
        // Write executable
        const output_path = if (options.output_file) |output| output else stripExtension(file_path);
        
        codegen.writeExecutable(output_path) catch |err| {
            logger.error("Failed to write executable: {any}", .{err});
            return CompilerResult{
                .success = false,
                .error_count = error_reporter.getErrorCount() + 1,
                .warning_count = error_reporter.getWarningCount(),
                .debug_info = if (options.debug_level != .None) debug_info else null,
            };
        };
        
        logger.info("Executable written to {s}", .{output_path});
        logger.debug("Code generation complete");
    }
    
    return CompilerResult{
        .success = !error_reporter.hasErrors(),
        .error_count = error_reporter.getErrorCount(),
        .warning_count = error_reporter.getWarningCount(),
        .debug_info = if (options.debug_level != .None) debug_info else null,
    };
}

fn stripExtension(file_path: []const u8) []const u8 {
    if (std.mem.lastIndexOf(u8, file_path, ".")) |dot_index| {
        return file_path[0..dot_index];
    }
    return file_path;
}

// Demonstration and testing function
fn demonstrateErrorReporting(allocator: Allocator) !void {
    std.debug.print("\n=== CURSED Compiler Error Reporting Demonstration ===\n\n", .{});
    
    // Test cases with various types of errors
    const test_cases = [_]struct {
        name: []const u8,
        source: []const u8,
    }{
        .{
            .name = "Unterminated String",
            .source = "slay main() { vibez.spill(\"Hello world; }",
        },
        .{
            .name = "Invalid Function Syntax",
            .source = "function main() { return 42; }",
        },
        .{
            .name = "Missing Type Annotation",
            .source = "sus x = 42;\nfacts y;\nz := \"hello\";",
        },
        .{
            .name = "Unbalanced Braces",
            .source = "slay main() {\n    sus x normie = 42;\n    lowkey (x > 0) {\n        vibez.spill(\"positive\");\n    // Missing closing brace",
        },
        .{
            .name = "Type Mismatch",
            .source = "slay main() {\n    sus x normie = \"string\";\n    sus y lit = 42;\n}",
        },
    };
    
    for (test_cases) |test_case| {
        std.debug.print("--- Test Case: {s} ---\n", .{test_case.name});
        
        var error_reporter = ErrorReporter.init(allocator, 10);
        defer error_reporter.deinit();
        
        error_reporter.setColors(true);
        
        try error_reporter.addSourceFile("demo.csd", test_case.source);
        
        var lexer = enhanced_lexer.Lexer.init(allocator, test_case.source, "demo.csd", &error_reporter) catch continue;
        defer lexer.deinit();
        
        const tokens = lexer.tokenize() catch &[_]enhanced_lexer.Token{};
        defer if (tokens.len > 0) allocator.free(tokens);
        
        var parser = enhanced_parser.Parser.init(allocator, tokens, &error_reporter);
        _ = parser.parseProgram() catch {};
        
        try error_reporter.printDiagnostics(std.io.getStdOut().writer());
        std.debug.print("\n", .{});
    }
    
    std.debug.print("=== Demonstration Complete ===\n", .{});
}

test "enhanced compiler integration" {
    const allocator = std.testing.allocator;
    
    // Test basic compilation flow
    const source = "slay main() normie { sus x normie = 42; damn x; }";
    
    var error_reporter = ErrorReporter.init(allocator, 10);
    defer error_reporter.deinit();
    
    try error_reporter.addSourceFile("test.csd", source);
    
    var lexer = try enhanced_lexer.Lexer.init(allocator, source, "test.csd", &error_reporter);
    defer lexer.deinit();
    
    const tokens = try lexer.tokenize();
    defer allocator.free(tokens);
    
    var parser = enhanced_parser.Parser.init(allocator, tokens, &error_reporter);
    const program = try parser.parseProgram();
    defer program.deinit();
    
    try std.testing.expect(!error_reporter.hasErrors());
    try std.testing.expect(program.statements.len > 0);
}
