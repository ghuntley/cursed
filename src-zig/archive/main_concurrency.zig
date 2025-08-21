//! CURSED Concurrency-Enabled Compiler
//!
//! This is the main entry point for the CURSED compiler with full concurrency support.
//! Integrates lexer, parser, interpreter, and compiler with concurrency runtime.

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast_simple.zig");
const codegen = @import("codegen_clean.zig");
const concurrency_codegen = @import("codegen_concurrency.zig");
const concurrency_runtime = @import("concurrency_runtime.zig");
const interpreter_concurrency = @import("interpreter_concurrency.zig");
const concurrency = @import("concurrency.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        printUsage();
        return;
    }

    if (std.mem.eql(u8, args[1], "--version")) {
        print("CURSED Zig Compiler v1.0.0-concurrency-complete\n", .{});
        print("Full concurrency support with goroutines, channels, and select statements\n", .{});
        return;
    }

    if (std.mem.eql(u8, args[1], "--help")) {
        printUsage();
        return;
    }

    const filename = args[1];
    
    // Parse command line options
    var compile_mode = false;
    var interpret_mode = false;
    var debug_tokens = false;
    var debug_ast = false;
    var debug_concurrency = false;
    var concurrency_benchmark = false;
    var optimization_level: u8 = 2;
    
    for (args[2..]) |arg| {
        if (std.mem.eql(u8, arg, "--compile")) {
            compile_mode = true;
        } else if (std.mem.eql(u8, arg, "--interpret")) {
            interpret_mode = true;
        } else if (std.mem.eql(u8, arg, "--debug")) {
            debug_tokens = true;
            debug_ast = true;
            debug_concurrency = true;
        } else if (std.mem.eql(u8, arg, "--tokens")) {
            debug_tokens = true;
        } else if (std.mem.eql(u8, arg, "--ast")) {
            debug_ast = true;
        } else if (std.mem.eql(u8, arg, "--concurrency-debug")) {
            debug_concurrency = true;
        } else if (std.mem.eql(u8, arg, "--benchmark")) {
            concurrency_benchmark = true;
        } else if (std.mem.startsWith(u8, arg, "--optimize=")) {
            const level_str = arg[11..];
            optimization_level = std.fmt.parseUnsigned(u8, level_str, 10) catch 2;
        }
    }

    // Default to interpretation if neither mode specified
    if (!compile_mode and !interpret_mode) {
        interpret_mode = true;
    }

    // Read source file
    const file = std.fs.cwd().openFile(filename, .{}) catch |err| {
        print("Error: Could not open file '{s}': {}\n", .{ filename, err });
        return;
    };
    defer file.close();

    const source = file.readToEndAlloc(allocator, 1024 * 1024) catch |err| {
        print("Error: Could not read file '{s}': {}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);

    print("🚀 CURSED Concurrency Compiler v1.0.0\n", .{});
    print("📄 Processing: {s}\n", .{filename});

    // Tokenize
    var l = lexer.Lexer.init(allocator, source);

    const tokens = l.tokenize() catch |err| {
        print("❌ Lexer error: {}\n", .{err});
        return;
    };

    if (debug_tokens) {
        print("\n=== TOKENS ===\n", .{});
        for (tokens.items) |token| {
            print("{}: '{}'\n", .{ token.kind, token.lexeme });
        }
        print("\n", .{});
    }

    // Parse
    var p = parser.Parser.init(allocator, tokens);
    defer p.deinit(allocator);

    const program = p.parseProgram() catch |err| {
        print("❌ Parser error: {}\n", .{err});
        return;
    };

    if (debug_ast) {
        print("=== AST ===\n", .{});
        print("Program with {} statements\n", .{program.statements.items.len});
        print("\n", .{});
    }

    if (concurrency_benchmark) {
        try runConcurrencyBenchmark(allocator);
        return;
    }

    if (interpret_mode) {
        try executeWithInterpreter(allocator, program, debug_concurrency);
    }

    if (compile_mode) {
        try compileWithConcurrency(allocator, program, filename, optimization_level, debug_concurrency);
    }
}

/// Execute program using concurrency-aware interpreter
fn executeWithInterpreter(allocator: Allocator, program: *ast.Program, debug_mode: bool) !void {
    print("🔄 Executing with concurrency interpreter...\n", .{});

    var interpreter = interpreter_concurrency.ConcurrencyInterpreter.init(allocator) catch |err| {
        print("❌ Failed to initialize concurrency interpreter: {}\n", .{err});
        return;
    };
    defer interpreter.deinit(allocator);

    if (debug_mode) {
        print("📊 Concurrency runtime initialized\n", .{});
    }

    // Execute program statements
    for (program.statements.items) |stmt| {
        const result = interpreter.eval(ast.Node{ .statement = stmt }) catch |err| {
            print("❌ Execution error: {}\n", .{err});
            return;
        };

        if (debug_mode) {
            const result_str = result.toString(allocator) catch "unknown";
            defer allocator.free(result_str);
            print("🔍 Result: {s}\n", .{result_str});
        }
    }

    // Print runtime statistics
    const stats = interpreter.getRuntimeStats();
    print("\n📈 Concurrency Statistics:\n", .{});
    print("  Goroutines spawned: {}\n", .{stats.total_goroutines_spawned});
    print("  Channels created: {}\n", .{stats.total_channels_created});
    print("  Messages sent: {}\n", .{stats.total_messages_sent});
    print("  Messages received: {}\n", .{stats.total_messages_received});
    print("  Select operations: {}\n", .{stats.total_select_operations});

    print("✅ Execution completed successfully\n", .{});
}

/// Compile program with concurrency support
fn compileWithConcurrency(allocator: Allocator, program: *ast.Program, filename: []const u8, optimization_level: u8, debug_mode: bool) !void {
    print("⚙️ Compiling with concurrency support (O{})...\n", .{optimization_level});

    // Generate LLVM IR with concurrency support
    var concurrency_gen = concurrency_codegen.ConcurrencyCodeGen.init(allocator);
    defer concurrency_gen.deinit(allocator);

    concurrency_gen.generateProgram(program) catch |err| {
        print("❌ Concurrency code generation error: {}\n", .{err});
        return;
    };

    // Write LLVM IR to file
    const ir_filename = try getIRFileName(allocator, filename);
    defer allocator.free(ir_filename);

    try concurrency_gen.writeToFile(ir_filename);

    if (debug_mode) {
        print("📝 Generated LLVM IR: {s}\n", .{ir_filename});
        print("🔍 IR Preview:\n", .{});
        const output = concurrency_gen.getOutput();
        const preview_len = @min(500, output.len);
        print("{s}...\n\n", .{output[0..preview_len]});
    }

    // Compile to executable using LLVM
    const output_name = try getOutputName(allocator, filename);
    defer allocator.free(output_name);

    const compile_cmd = try std.fmt.allocPrint(allocator, 
        "llc -filetype=obj -O={} {s} -o {s}.o && " ++
        "clang {s}.o -lcursed_runtime -lpthread -lm -o {s}", 
        .{ optimization_level, ir_filename, output_name, output_name, output_name }
    );
    defer allocator.free(compile_cmd);

    print("🔨 Compiling: {s}\n", .{compile_cmd});
    
    const result = try std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{ "sh", "-c", compile_cmd },
    });

    if (result.term.Exited == 0) {
        print("✅ Generated executable: {s}\n", .{output_name});
        print("🚀 Run with: ./{s}\n", .{output_name});
    } else {
        print("❌ Compilation failed with exit code: {}\n", .{result.term.Exited});
        print("Error output: {s}\n", .{result.stderr});
    }

    allocator.free(result.stdout);
    allocator.free(result.stderr);
}

/// Run concurrency benchmark tests
fn runConcurrencyBenchmark(allocator: Allocator) !void {
    print("🏁 Running concurrency benchmarks...\n", .{});

    // Initialize concurrency runtime
    try concurrency_runtime.initializeRuntime(allocator);
    defer concurrency_runtime.shutdownRuntime(allocator);

    const start_time = std.time.milliTimestamp();

    // Benchmark 1: Goroutine spawning
    print("📊 Benchmark 1: Goroutine spawning\n", .{});
    const num_goroutines = 1000;
    
    var spawned_goroutines = ArrayList(concurrency.GoroutineId).init(allocator);
    defer spawned_goroutines.deinit(allocator);

    const spawn_start = std.time.milliTimestamp();
    for (0..num_goroutines) |_| {
        const function_literal = ast.FunctionLiteral{
            .parameters = ArrayList(*ast.Identifier).init(allocator),
            .body = ast.BlockStatement{ .statements = ArrayList(ast.Statement).init(allocator) },
        };

        const goroutine_id = try concurrency_runtime.executeStan(&function_literal, null);
        try spawned_goroutines.append(goroutine_id);
    }
    const spawn_end = std.time.milliTimestamp();
    
    print("  Spawned {} goroutines in {}ms\n", .{ num_goroutines, spawn_end - spawn_start });
    print("  Average: {d:.2}μs per goroutine\n", .{ @as(f64, @floatFromInt(spawn_end - spawn_start)) * 1000.0 / @as(f64, @floatFromInt(num_goroutines)) });

    // Benchmark 2: Channel operations
    print("📊 Benchmark 2: Channel operations\n", .{});
    const num_messages = 10000;
    
    const channel_id = try concurrency_runtime.executeDmCreate(.integer, 100);
    
    const channel_start = std.time.milliTimestamp();
    for (0..num_messages) |i| {
        const value = concurrency_runtime.ConcurrencyValue{ .goroutine_id = @intCast(i) };
        _ = try concurrency_runtime.executeDmSend(channel_id, value);
        _ = try concurrency_runtime.executeDmReceive(channel_id);
    }
    const channel_end = std.time.milliTimestamp();
    
    print("  Processed {} messages in {}ms\n", .{ num_messages, channel_end - channel_start });
    print("  Average: {d:.2}μs per message\n", .{ @as(f64, @floatFromInt(channel_end - channel_start)) * 1000.0 / @as(f64, @floatFromInt(num_messages)) });

    // Benchmark 3: Select operations
    print("📊 Benchmark 3: Select operations\n", .{});
    const num_selects = 1000;
    
    var operations = [_]concurrency_runtime.SelectOperation{
        concurrency_runtime.SelectOperation{
            .type = .default,
            .channel_id = channel_id,
            .value = null,
        },
    };
    
    const select_start = std.time.milliTimestamp();
    for (0..num_selects) |_| {
        _ = try concurrency_runtime.executeReady(&operations);
    }
    const select_end = std.time.milliTimestamp();
    
    print("  Executed {} select operations in {}ms\n", .{ num_selects, select_end - select_start });
    print("  Average: {d:.2}μs per select\n", .{ @as(f64, @floatFromInt(select_end - select_start)) * 1000.0 / @as(f64, @floatFromInt(num_selects)) });

    const total_time = std.time.milliTimestamp() - start_time;
    print("\n⏱️ Total benchmark time: {}ms\n", .{total_time});

    // Get final statistics
    if (concurrency_runtime.getRuntime()) |runtime| {
        const stats = runtime.getStats();
        print("\n📈 Final Statistics:\n", .{});
        print("  Total goroutines spawned: {}\n", .{stats.total_goroutines_spawned});
        print("  Total channels created: {}\n", .{stats.total_channels_created});
        print("  Total messages sent: {}\n", .{stats.total_messages_sent});
        print("  Total messages received: {}\n", .{stats.total_messages_received});
        print("  Total select operations: {}\n", .{stats.total_select_operations});
    }

    print("✅ Benchmark completed successfully\n", .{});
}

fn printUsage() void {
    print("CURSED Zig Compiler - Full Concurrency Support v1.0.0\n", .{});
    print("Complete implementation with goroutines, channels, and select statements\n", .{});
    print("\nUsage: cursed-concurrency <file.csd> [OPTIONS]\n", .{});
    print("       cursed-concurrency --version\n", .{});
    print("       cursed-concurrency --help\n", .{});
    print("\nExecution Modes:\n", .{});
    print("  --interpret        Execute with concurrency interpreter (default)\n", .{});
    print("  --compile          Compile to native executable with concurrency runtime\n", .{});
    print("\nOptions:\n", .{});
    print("  --debug            Enable all debug output\n", .{});
    print("  --ast              Show AST representation\n", .{});
    print("  --tokens           Show token stream\n", .{});
    print("  --concurrency-debug Enable concurrency runtime debugging\n", .{});
    print("  --benchmark        Run concurrency performance benchmarks\n", .{});
    print("  --optimize=LEVEL   Optimization level (0-3, default: 2)\n", .{});
    print("\nConcurrency Features Supported:\n", .{});
    print("  • Goroutines (stan keyword)\n", .{});
    print("  • Channels (dm<T> type)\n", .{});
    print("  • Select statements (ready keyword)\n", .{});
    print("  • Channel send/receive operations\n", .{});
    print("  • Work-stealing scheduler\n", .{});
    print("  • Goroutine yielding (yolo keyword)\n", .{});
    print("  • Thread-safe operations\n", .{});
    print("  • Performance monitoring\n", .{});
    print("\nExample Programs:\n", .{});
    print("  echo 'stan { vibez.spill(\"Hello from goroutine!\") }' > hello_concurrent.csd\n", .{});
    print("  echo 'sus ch dm<normie> = dm<normie>(5)' > channel_demo.csd\n", .{});
    print("  echo 'ready { dm_recv(ch) -> vibez.spill(\"received\") }' > select_demo.csd\n", .{});
}

fn getOutputName(allocator: Allocator, filename: []const u8) ![]u8 {
    if (std.mem.endsWith(u8, filename, ".csd")) {
        return try allocator.dupe(u8, filename[0..filename.len - 4]);
    }
    return try std.fmt.allocPrint(allocator, "{s}_out", .{filename});
}

fn getIRFileName(allocator: Allocator, filename: []const u8) ![]u8 {
    if (std.mem.endsWith(u8, filename, ".csd")) {
        return try std.fmt.allocPrint(allocator, "{s}.ll", .{filename[0..filename.len - 4]});
    }
    return try std.fmt.allocPrint(allocator, "{s}.ll", .{filename});
}

test "main concurrency tests" {
    // Import tests from all concurrency modules
    _ = @import("concurrency.zig");
    _ = @import("concurrency_runtime.zig");
    _ = @import("interpreter_concurrency.zig");
    _ = @import("codegen_concurrency.zig");
}
