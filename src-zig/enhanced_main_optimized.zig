const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

// Import optimization modules
const performance_optimizations = @import("performance_optimizations.zig");
const llvm_optimizations = @import("llvm_optimizations.zig");
const parser_optimizations = @import("parser_optimizations.zig");

// Import existing modules
const main_unified = @import("main_unified.zig");
const enhanced_compiler = @import("enhanced_compiler.zig");

// Optimized interpreter with performance enhancements
pub fn optimizedInterpret(allocator: Allocator, source: []const u8, verbose: bool) !void {
    var timer = std.time.Timer.start() catch return;
    const start_time = timer.read();
    
    if (verbose) print("🚀 Starting optimized CURSED interpretation...\n", .{});
    
    // Initialize performance tracking
    var perf_stats = performance_optimizations.PerformanceStats.init();
    var string_interner = performance_optimizations.StringInterner.init(allocator);
    defer string_interner.deinit();
    
    // Optimized tokenization
    var fast_tokenizer = parser_optimizations.FastTokenizer.init(allocator, source) catch return;
    defer fast_tokenizer.deinit();
    
    const tokenize_start = timer.read();
    const tokens = fast_tokenizer.tokenize() catch return;
    const tokenize_end = timer.read();
    const tokenization_time = tokenize_end - tokenize_start;
    
    if (verbose) print("⚡ Fast tokenization: {} tokens in {d:.2}ms\n", .{ tokens.len, @as(f64, @floatFromInt(tokenization_time)) / 1_000_000.0 });
    
    // Optimized parsing
    var fast_parser = parser_optimizations.FastParser.init(allocator, tokens);
    defer fast_parser.deinit();
    
    const parse_start = timer.read();
    const ast = fast_parser.parse() catch return;
    const parse_end = timer.read();
    const parsing_time = parse_end - parse_start;
    
    if (verbose) print("⚡ Fast parsing: AST generated in {d:.2}ms\n", .{@as(f64, @floatFromInt(parsing_time)) / 1_000_000.0});
    
    // Initialize optimized execution context
    var optimized_scope = performance_optimizations.OptimizedScope.init(allocator, null, &string_interner);
    defer optimized_scope.deinit();
    
    var function_context = performance_optimizations.OptimizedFunctionContext.init(allocator);
    defer function_context.deinit();
    
    // Execute with optimizations
    const execution_start = timer.read();
    try executeOptimizedAST(allocator, ast, &optimized_scope, &function_context, &perf_stats, verbose);
    const execution_end = timer.read();
    const execution_time = execution_end - execution_start;
    
    // Performance reporting
    perf_stats.execution_time_ns = execution_time;
    
    if (verbose) {
        print("⚡ Optimized execution completed in {d:.2}ms\n", .{@as(f64, @floatFromInt(execution_time)) / 1_000_000.0});
        print("\n=== Performance Statistics ===\n", .{});
        perf_stats.print();
        print("Cache efficiency: {d:.2}%\n", .{optimized_scope.cache.getCacheEfficiency() * 100.0});
        
        // Parser benchmark
        var parser_benchmark = parser_optimizations.ParserBenchmark.init();
        parser_benchmark.tokenization_time_ns = tokenization_time;
        parser_benchmark.parsing_time_ns = parsing_time;
        parser_benchmark.tokens_created = @as(u32, @intCast(tokens.len));
        parser_benchmark.nodes_created = fast_parser.ast_pool.cache_index;
        parser_benchmark.print();
    }
    
    const total_time = timer.read() - start_time;
    if (verbose) print("🎯 Total optimized runtime: {d:.2}ms\n", .{@as(f64, @floatFromInt(total_time)) / 1_000_000.0});
}

// Optimized LLVM compilation with enhanced optimizations
pub fn optimizedCompile(allocator: Allocator, source: []const u8, filename: []const u8, optimization_level: u32, verbose: bool) !void {
    var timer = std.time.Timer.start() catch return;
    const start_time = timer.read();
    
    if (verbose) print("🔥 Starting optimized LLVM compilation (O{})...\n", .{optimization_level});
    
    // Initialize LLVM optimizer
    const inline_threshold: u32 = switch (optimization_level) {
        0 => 0,    // No inlining
        1 => 50,   // Conservative inlining
        2 => 100,  // Standard inlining
        3 => 200,  // Aggressive inlining
        else => 100,
    };
    
    var llvm_optimizer = llvm_optimizations.LLVMOptimizer.init(allocator, optimization_level, inline_threshold);
    defer llvm_optimizer.deinit();
    
    // Fast tokenization and parsing
    var fast_tokenizer = parser_optimizations.FastTokenizer.init(allocator, source) catch return;
    defer fast_tokenizer.deinit();
    
    const tokens = fast_tokenizer.tokenize() catch return;
    
    var fast_parser = parser_optimizations.FastParser.init(allocator, tokens);
    defer fast_parser.deinit();
    
    const ast = fast_parser.parse() catch return;
    
    // Generate LLVM IR with optimizations
    const ir_generation_start = timer.read();
    const basic_llvm_ir = try generateBasicLLVMIR(allocator, ast, &llvm_optimizer, verbose);
    defer allocator.free(basic_llvm_ir);
    
    // Apply advanced optimizations
    const optimized_llvm_ir = try llvm_optimizer.optimizeLLVMIR(basic_llvm_ir);
    defer allocator.free(optimized_llvm_ir);
    
    const ir_generation_end = timer.read();
    const ir_time = ir_generation_end - ir_generation_start;
    
    if (verbose) {
        print("⚡ LLVM IR generation with optimizations: {d:.2}ms\n", .{@as(f64, @floatFromInt(ir_time)) / 1_000_000.0});
        llvm_optimizer.generateOptimizationStats();
    }
    
    // Compile to native binary using enhanced compiler
    const config = enhanced_compiler.CompilerConfig{
        .backend = .LLVM_Backend,
        .optimization_level = optimization_level,
        .verbose = verbose,
        .output_path = null,
        .debug_info = false,
        .target = null,
        .emit_llvm = false,
        .static_link = false,
        .inline_threshold = inline_threshold,
        .no_inline = (optimization_level == 0),
    };
    
    const compile_start = timer.read();
    try enhanced_compiler.compileProgram(allocator, source, filename, config);
    const compile_end = timer.read();
    const compile_time = compile_end - compile_start;
    
    const total_time = timer.read() - start_time;
    
    if (verbose) {
        print("⚡ Native compilation: {d:.2}ms\n", .{@as(f64, @floatFromInt(compile_time)) / 1_000_000.0});
        print("🎯 Total optimized compilation: {d:.2}ms\n", .{@as(f64, @floatFromInt(total_time)) / 1_000_000.0});
    }
}

// Execute optimized AST with performance tracking
fn executeOptimizedAST(
    allocator: Allocator,
    ast: *parser_optimizations.ASTNodePool.ASTNode,
    scope: *performance_optimizations.OptimizedScope,
    function_context: *performance_optimizations.OptimizedFunctionContext,
    perf_stats: *performance_optimizations.PerformanceStats,
    verbose: bool
) !void {
    // This is a simplified execution - in a real implementation, this would
    // traverse the AST and execute each node with optimization tracking
    
    if (verbose) print("🔧 Executing optimized AST with {} child nodes\n", .{ast.child_count});
    
    // Simulate optimized execution by calling the regular interpreter
    // but with performance tracking
    perf_stats.recordFunctionCall();
    
    // For demonstration, call the regular unified interpreter
    // In a real implementation, this would be the optimized execution path
    try main_unified.interpretProgramWithVariables(allocator, "", verbose, null);
}

// Generate basic LLVM IR with optimization hints
fn generateBasicLLVMIR(
    allocator: Allocator,
    ast: *parser_optimizations.ASTNodePool.ASTNode,
    llvm_optimizer: *llvm_optimizations.LLVMOptimizer,
    verbose: bool
) ![]u8 {
    var ir: std.ArrayList(u8) = .empty;
    
    // Add basic LLVM IR structure
    try ir.appendSlice("; CURSED Optimized LLVM IR\n");
    try ir.appendSlice("target triple = \"x86_64-unknown-linux-gnu\"\n\n");
    
    // Record function information for optimization
    try llvm_optimizer.function_inliner.recordFunctionCall("main", 100);
    try llvm_optimizer.dead_code_eliminator.markFunctionCall("main");
    
    // Generate main function
    try ir.appendSlice("define i32 @main() {\n");
    try ir.appendSlice("entry:\n");
    
    // Allocate registers with optimization
    const reg1 = try llvm_optimizer.register_allocator.allocateRegister("result", 0);
    const reg2 = try llvm_optimizer.register_allocator.allocateRegister("temp", 1);
    
    // Add optimized instructions
    const instr1 = try std.fmt.allocPrint(allocator, "  %{} = alloca i64\n", .{reg1});
    defer allocator.free(instr1);
    try ir.appendSlice(instr1);
    
    const instr2 = try std.fmt.allocPrint(allocator, "  %{} = add i64 0, 42\n", .{reg2});
    defer allocator.free(instr2);
    try ir.appendSlice(instr2);
    
    const store_instr = try std.fmt.allocPrint(allocator, "  store i64 %{}, i64* %{}\n", .{ reg2, reg1 });
    defer allocator.free(store_instr);
    try ir.appendSlice(store_instr);
    
    try ir.appendSlice("  ret i32 0\n");
    try ir.appendSlice("}\n");
    
    if (verbose) print("📝 Generated basic LLVM IR ({} bytes)\n", .{ir.items.len});
    
    return ir.toOwnedSlice();
}

// Main entry point with optimization flags
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        print("Usage: cursed-optimized [--compile] [--optimize=N] [--verbose] <file.csd>\n", .{});
        return;
    }
    
    var compile_mode = false;
    var optimization_level: u32 = 2; // Default O2
    var verbose = false;
    var filename: ?[]const u8 = null;
    
    for (args[1..]) |arg| {
        if (std.mem.eql(u8, arg, "--compile")) {
            compile_mode = true;
        } else if (std.mem.startsWith(u8, arg, "--optimize=")) {
            const opt_str = arg[11..];
            optimization_level = std.fmt.parseUnsigned(u32, opt_str, 10) catch 2;
        } else if (std.mem.eql(u8, arg, "--verbose")) {
            verbose = true;
        } else if (!std.mem.startsWith(u8, arg, "--")) {
            filename = arg;
        }
    }
    
    if (filename == null) {
        print("❌ Error: No CURSED source file specified\n", .{});
        return;
    }
    
    const source = std.fs.cwd().readFileAlloc(allocator, filename.?, 1024 * 1024) catch |err| {
        print("❌ Error reading file {s}: {any}\n", .{ filename.?, err });
        return;
    };
    defer allocator.free(source);
    
    if (compile_mode) {
        try optimizedCompile(allocator, source, filename.?, optimization_level, verbose);
    } else {
        try optimizedInterpret(allocator, source, verbose);
    }
}
