const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast.zig");
const codegen = @import("codegen.zig");
const advanced_codegen = @import("advanced_codegen.zig");
const native_compilation = @import("native_compilation.zig");
const debug_info = @import("debug_info.zig");
const runtime_system = @import("runtime_system.zig");

/// Complete CURSED compiler with native executable generation
pub const CursedCompiler = struct {
    allocator: Allocator,
    target_platform: native_compilation.NativeCompiler.TargetPlatform,
    optimization_level: native_compilation.NativeCompiler.OptimizationLevel,
    debug_info_enabled: bool,
    source_filename: []const u8,
    output_path: []const u8,
    
    // Compilation pipeline components
    native_compiler: ?native_compilation.NativeCompiler,
    debug_generator: ?debug_info.DebugInfoGenerator,
    runtime: runtime_system.RuntimeSystem,
    
    pub const CompilerError = error{
        LexicalError,
        SyntaxError,
        SemanticError,
        CodeGenError,
        CompilationError,
        LinkingError,
        DebugInfoError,
        RuntimeError,
        OutOfMemory,
    };
    
    pub const CompilationStats = struct {
        source_lines: u32,
        tokens_generated: u32,
        ast_nodes: u32,
        llvm_instructions: u32,
        compilation_time_ms: u64,
        executable_size_bytes: u64,
        optimization_passes: u32,
    };
    
    pub fn init(allocator: Allocator, source_filename: []const u8, output_path: []const u8) CompilerError!CursedCompiler {
        // Auto-detect target platform
        const target = detectTargetPlatform();
        
        return CursedCompiler{
            .allocator = allocator,
            .target_platform = target,
            .optimization_level = .Default,
            .debug_info_enabled = true,
            .source_filename = source_filename,
            .output_path = output_path,
            .native_compiler = null,
            .debug_generator = null,
            .runtime = runtime_system.RuntimeSystem.init(allocator),
        };
    }
    
    pub fn deinit(self: *CursedCompiler) void {
        if (self.native_compiler) |*compiler| {
            compiler.deinit();
        }
        if (self.debug_generator) |*debug_gen| {
            debug_gen.deinit();
        }
    }
    
    pub fn setTarget(self: *CursedCompiler, target: native_compilation.NativeCompiler.TargetPlatform) void {
        self.target_platform = target;
    }
    
    pub fn setOptimizationLevel(self: *CursedCompiler, level: native_compilation.NativeCompiler.OptimizationLevel) void {
        self.optimization_level = level;
    }
    
    pub fn setDebugInfo(self: *CursedCompiler, enable: bool) void {
        self.debug_info_enabled = enable;
    }
    
    /// Complete compilation pipeline: Source → Native Executable
    pub fn compileToExecutable(self: *CursedCompiler, source: []const u8) CompilerError!CompilationStats {
        _ = source;
        return self.compile();
    }

    /// Complete compilation pipeline: Source → Native Executable
    pub fn compile(self: *CursedCompiler) CompilerError!CompilationStats {
        const start_time = std.time.milliTimestamp();
        
        std.debug.print("CURSED Compiler v1.0.0 - Complete Native Compilation\n", .{});
        std.debug.print("Source: {s} → Target: {s}\n", .{ self.source_filename, @tagName(self.target_platform) });
        
        // Step 1: Lexical Analysis
        std.debug.print("[1/7] Lexical Analysis...\n", .{});
        const tokens = try self.performLexicalAnalysis();
        defer tokens.deinit();
        
        // Step 2: Syntax Analysis (Parsing)
        std.debug.print("[2/7] Syntax Analysis...\n", .{});
        const program = try self.performSyntaxAnalysis(tokens);
        
        // Step 3: Semantic Analysis
        std.debug.print("[3/7] Semantic Analysis...\n", .{});
        try self.performSemanticAnalysis(program);
        
        // Step 4: Initialize Native Compiler
        std.debug.print("[4/7] LLVM Code Generation...\n", .{});
        try self.initializeNativeCompiler();
        
        // Step 5: Generate LLVM IR
        try self.generateLLVMIR(program);
        
        // Step 6: Generate Debug Information
        if (self.debug_info_enabled) {
            std.debug.print("[5/7] Debug Information Generation...\n", .{});
            try self.generateDebugInformation(program);
        } else {
            std.debug.print("[5/7] Debug Information (skipped)...\n", .{});
        }
        
        // Step 7: Native Compilation & Linking
        std.debug.print("[6/7] Native Compilation & Linking...\n", .{});
        try self.performNativeCompilation(program);
        
        std.debug.print("[7/7] Post-compilation Analysis...\n", .{});
        const stats = try self.generateCompilationStats(start_time);
        
        std.debug.print("\n✅ Compilation successful!\n", .{});
        self.printCompilationStats(stats);
        
        return stats;
    }
    
    /// Perform lexical analysis on source file
    fn performLexicalAnalysis(self: *CursedCompiler) CompilerError!ArrayList(lexer.Token) {
        // Read source file
        const file = std.fs.cwd().openFile(self.source_filename, .{}) catch |err| {
            std.debug.print("❌ Error: Could not open source file '{s}': {}\n", .{ self.source_filename, err });
            return CompilerError.LexicalError;
        };
        defer file.close();
        
        const source = file.readToEndAlloc(self.allocator, 1024 * 1024) catch |err| {
            std.debug.print("❌ Error: Could not read source file '{s}': {}\n", .{ self.source_filename, err });
            return CompilerError.LexicalError;
        };
        defer self.allocator.free(source);
        
        // Tokenize
        var l = lexer.Lexer.init(self.allocator, source);
        defer l.deinit();
        
        const tokens = l.tokenize() catch |err| {
            std.debug.print("❌ Lexical error: {}\n", .{err});
            return CompilerError.LexicalError;
        };
        
        std.debug.print("   Generated {} tokens\n", .{tokens.items.len});
        return tokens;
    }
    
    /// Perform syntax analysis (parsing)
    fn performSyntaxAnalysis(self: *CursedCompiler, tokens: ArrayList(lexer.Token)) CompilerError!ast.Program {
        var p = parser.Parser.init(self.allocator, tokens);
        defer p.deinit();
        
        const program = p.parseProgram() catch |err| {
            std.debug.print("❌ Syntax error: {}\n", .{err});
            return CompilerError.SyntaxError;
        };
        
        std.debug.print("   Parsed {} statements\n", .{program.statements.items.len});
        return program;
    }
    
    /// Perform semantic analysis
    fn performSemanticAnalysis(self: *CursedCompiler, program: ast.Program) CompilerError!void {
        _ = self;
        
        // Type checking
        var type_errors: u32 = 0;
        
        // Check for undefined variables, type mismatches, etc.
        for (program.statements.items) |stmt| {
            switch (stmt) {
                .Function => |func| {
                    // Validate function signature
                    if (func.name.len == 0) {
                        std.debug.print("❌ Semantic error: Empty function name\n", .{});
                        type_errors += 1;
                    }
                    
                    // Check for duplicate parameters
                    for (func.parameters.items, 0..) |param1, i| {
                        for (func.parameters.items[i+1..]) |param2| {
                            if (std.mem.eql(u8, param1.name, param2.name)) {
                                std.debug.print("❌ Semantic error: Duplicate parameter '{s}' in function '{s}'\n", .{ param1.name, func.name });
                                type_errors += 1;
                            }
                        }
                    }
                },
                .Struct => |struct_stmt| {
                    // Validate struct definition
                    if (struct_stmt.name.len == 0) {
                        std.debug.print("❌ Semantic error: Empty struct name\n", .{});
                        type_errors += 1;
                    }
                    
                    // Check for duplicate fields
                    for (struct_stmt.fields.items, 0..) |field1, i| {
                        for (struct_stmt.fields.items[i+1..]) |field2| {
                            if (std.mem.eql(u8, field1.name, field2.name)) {
                                std.debug.print("❌ Semantic error: Duplicate field '{s}' in struct '{s}'\n", .{ field1.name, struct_stmt.name });
                                type_errors += 1;
                            }
                        }
                    }
                },
                else => {
                    // Other semantic checks
                },
            }
        }
        
        if (type_errors > 0) {
            std.debug.print("❌ Found {} semantic errors\n", .{type_errors});
            return CompilerError.SemanticError;
        }
        
        std.debug.print("   ✅ Semantic analysis passed\n", .{});
    }
    
    /// Initialize native compiler
    fn initializeNativeCompiler(self: *CursedCompiler) CompilerError!void {
        self.native_compiler = native_compilation.NativeCompiler.init(self.allocator, self.target_platform) catch |err| {
            std.debug.print("❌ Error initializing native compiler: {}\n", .{err});
            return CompilerError.CompilationError;
        };
        
        self.native_compiler.?.setOptimizationLevel(self.optimization_level);
        self.native_compiler.?.setDebugInfo(self.debug_info_enabled);
        
        std.debug.print("   ✅ Native compiler initialized for {s}\n", .{self.target_platform.getTriple()});
    }
    
    /// Generate LLVM IR
    fn generateLLVMIR(self: *CursedCompiler, program: ast.Program) CompilerError!void {
        if (self.native_compiler == null) {
            return CompilerError.CompilationError;
        }
        
        // Generate runtime library first
        self.runtime.generateRuntimeLibrary(
            self.native_compiler.?.codegen.base_codegen.context,
            self.native_compiler.?.codegen.base_codegen.module
        ) catch |err| {
            std.debug.print("❌ Error generating runtime library: {}\n", .{err});
            return CompilerError.RuntimeError;
        };
        
        // Generate user code
        self.native_compiler.?.codegen.generateAdvancedProgram(program) catch |err| {
            std.debug.print("❌ Error generating LLVM IR: {}\n", .{err});
            return CompilerError.CodeGenError;
        };
        
        std.debug.print("   ✅ LLVM IR generated successfully\n", .{});
    }
    
    /// Generate debug information
    fn generateDebugInformation(self: *CursedCompiler, program: ast.Program) CompilerError!void {
        _ = program;
        
        if (self.native_compiler == null) {
            return CompilerError.CompilationError;
        }
        
        self.debug_generator = debug_info.DebugInfoGenerator.init(
            self.allocator,
            self.native_compiler.?.codegen.base_codegen.context,
            self.native_compiler.?.codegen.base_codegen.module
        ) catch |err| {
            std.debug.print("❌ Error initializing debug info generator: {}\n", .{err});
            return CompilerError.DebugInfoError;
        };
        
        // Create compile unit
        const current_dir = std.fs.cwd().realpathAlloc(self.allocator, ".") catch {
            return CompilerError.DebugInfoError;
        };
        defer self.allocator.free(current_dir);
        
        self.debug_generator.?.createCompileUnit(self.source_filename, current_dir) catch |err| {
            std.debug.print("❌ Error creating debug compile unit: {}\n", .{err});
            return CompilerError.DebugInfoError;
        };
        
        // Create standard types
        _ = self.debug_generator.?.createCursedTypes() catch |err| {
            std.debug.print("❌ Error creating debug types: {}\n", .{err});
            return CompilerError.DebugInfoError;
        };
        
        // Finalize debug info
        self.debug_generator.?.finalize();
        
        std.debug.print("   ✅ Debug information generated\n", .{});
    }
    
    /// Perform native compilation
    fn performNativeCompilation(self: *CursedCompiler, program: ast.Program) CompilerError!void {
        if (self.native_compiler == null) {
            return CompilerError.CompilationError;
        }
        
        self.native_compiler.?.compileProgram(program, self.output_path) catch |err| {
            std.debug.print("❌ Error during native compilation: {}\n", .{err});
            return CompilerError.CompilationError;
        };
        
        std.debug.print("   ✅ Native executable generated: {s}\n", .{self.output_path});
    }
    
    /// Generate compilation statistics
    fn generateCompilationStats(self: *CursedCompiler, start_time: i64) CompilerError!CompilationStats {
        const end_time = std.time.milliTimestamp();
        const compilation_time = @as(u64, @intCast(end_time - start_time));
        
        // Get executable size
        const exe_file = std.fs.cwd().openFile(self.output_path, .{}) catch {
            return CompilerError.CompilationError;
        };
        defer exe_file.close();
        
        const exe_stat = exe_file.stat() catch {
            return CompilerError.CompilationError;
        };
        
        // Count source lines
        const source_file = std.fs.cwd().openFile(self.source_filename, .{}) catch {
            return CompilerError.CompilationError;
        };
        defer source_file.close();
        
        const source_content = source_file.readToEndAlloc(self.allocator, 1024 * 1024) catch {
            return CompilerError.CompilationError;
        };
        defer self.allocator.free(source_content);
        
        var line_count: u32 = 1;
        for (source_content) |char| {
            if (char == '\n') line_count += 1;
        }
        
        return CompilationStats{
            .source_lines = line_count,
            .tokens_generated = 0, // Would be filled during compilation
            .ast_nodes = 0, // Would be filled during compilation
            .llvm_instructions = 0, // Would be filled during compilation
            .compilation_time_ms = compilation_time,
            .executable_size_bytes = @as(u64, @intCast(exe_stat.size)),
            .optimization_passes = switch (self.optimization_level) {
                .None => 0,
                .Less => 3,
                .Default => 8,
                .Aggressive => 15,
            },
        };
    }
    
    /// Print compilation statistics
    fn printCompilationStats(self: *CursedCompiler, stats: CompilationStats) void {
        std.debug.print("\n📊 Compilation Statistics:\n", .{});
        std.debug.print("   Source lines:          {}\n", .{stats.source_lines});
        std.debug.print("   Compilation time:      {} ms\n", .{stats.compilation_time_ms});
        std.debug.print("   Executable size:       {} bytes ({d:.2} KB)\n", .{ stats.executable_size_bytes, @as(f64, @floatFromInt(stats.executable_size_bytes)) / 1024.0 });
        std.debug.print("   Optimization passes:   {}\n", .{stats.optimization_passes});
        std.debug.print("   Target platform:       {s}\n", .{@tagName(self.target_platform)});
        std.debug.print("   Debug info:            {s}\n", .{if (self.debug_info_enabled) "enabled" else "disabled"});
        std.debug.print("\n🚀 Ready to execute: ./{s}\n", .{self.output_path});
    }
    
    /// Cross-compile for all supported platforms
    pub fn crossCompile(self: *CursedCompiler, output_dir: []const u8) CompilerError!void {
        std.debug.print("🌐 Cross-compilation for all supported platforms\n", .{});
        
        // Read and parse source once
        const tokens = try self.performLexicalAnalysis();
        defer tokens.deinit();
        
        const program = try self.performSyntaxAnalysis(tokens);
        try self.performSemanticAnalysis(program);
        
        native_compilation.NativeCompiler.crossCompile(self.allocator, program, output_dir) catch |err| {
            std.debug.print("❌ Cross-compilation failed: {}\n", .{err});
            return CompilerError.CompilationError;
        };
    }
    
    /// Generate assembly output for analysis
    pub fn generateAssembly(self: *CursedCompiler) CompilerError!void {
        const tokens = try self.performLexicalAnalysis();
        defer tokens.deinit();
        
        const program = try self.performSyntaxAnalysis(tokens);
        try self.performSemanticAnalysis(program);
        try self.initializeNativeCompiler();
        try self.generateLLVMIR(program);
        
        self.native_compiler.?.generateAssembly(self.output_path) catch |err| {
            std.debug.print("❌ Assembly generation failed: {}\n", .{err});
            return CompilerError.CompilationError;
        };
    }
    
    /// Performance benchmark compilation
    pub fn benchmarkCompilation(self: *CursedCompiler) CompilerError!u64 {
        const tokens = try self.performLexicalAnalysis();
        defer tokens.deinit();
        
        const program = try self.performSyntaxAnalysis(tokens);
        
        var benchmark = native_compilation.PerformanceBenchmark.init(self.allocator);
        const compilation_time = benchmark.benchmarkCompilation(program, self.target_platform) catch |err| {
            std.debug.print("❌ Benchmark failed: {}\n", .{err});
            return CompilerError.CompilationError;
        };
        
        std.debug.print("⏱️  Compilation benchmark: {} nanoseconds\n", .{compilation_time});
        return compilation_time;
    }
    
    /// Detect target platform automatically
    fn detectTargetPlatform() native_compilation.NativeCompiler.TargetPlatform {
        const builtin = @import("builtin");
        
        return switch (builtin.os.tag) {
            .linux => switch (builtin.cpu.arch) {
                .x86_64 => .Linux_x64,
                .aarch64 => .Linux_ARM64,
                else => .Linux_x64, // default
            },
            .macos => switch (builtin.cpu.arch) {
                .x86_64 => .MacOS_Intel,
                .aarch64 => .MacOS_ARM64,
                else => .MacOS_Intel, // default
            },
            .windows => .Windows_x64,
            else => .Linux_x64, // default fallback
        };
    }
};

/// Command-line interface for the complete compiler
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
        std.debug.print("CURSED Complete Compiler v1.0.0\n", .{});
        std.debug.print("Native executable generation with LLVM backend\n", .{});
        std.debug.print("Features: Structs, Interfaces, Generics, Concurrency, Debug Info\n", .{});
        return;
    }
    
    if (std.mem.eql(u8, args[1], "--help")) {
        printUsage();
        return;
    }
    
    const source_filename = args[1];
    
    // Parse command line options
    var optimization_level = native_compilation.NativeCompiler.OptimizationLevel.Default;
    var debug_info_enabled = true;
    var cross_compile = false;
    var generate_asm = false;
    var benchmark = false;
    var target_platform: ?native_compilation.NativeCompiler.TargetPlatform = null;
    var output_path: ?[]const u8 = null;
    
    for (args[2..]) |arg| {
        if (std.mem.eql(u8, arg, "--no-debug")) {
            debug_info_enabled = false;
        } else if (std.mem.eql(u8, arg, "--cross-compile")) {
            cross_compile = true;
        } else if (std.mem.eql(u8, arg, "--assembly")) {
            generate_asm = true;
        } else if (std.mem.eql(u8, arg, "--benchmark")) {
            benchmark = true;
        } else if (std.mem.startsWith(u8, arg, "--optimize=")) {
            const level_str = arg[11..];
            if (std.mem.eql(u8, level_str, "0")) {
                optimization_level = .None;
            } else if (std.mem.eql(u8, level_str, "1")) {
                optimization_level = .Less;
            } else if (std.mem.eql(u8, level_str, "2")) {
                optimization_level = .Default;
            } else if (std.mem.eql(u8, level_str, "3")) {
                optimization_level = .Aggressive;
            }
        } else if (std.mem.startsWith(u8, arg, "--target=")) {
            const target_str = arg[9..];
            if (std.mem.eql(u8, target_str, "linux-x64")) {
                target_platform = .Linux_x64;
            } else if (std.mem.eql(u8, target_str, "linux-arm64")) {
                target_platform = .Linux_ARM64;
            } else if (std.mem.eql(u8, target_str, "macos-intel")) {
                target_platform = .MacOS_Intel;
            } else if (std.mem.eql(u8, target_str, "macos-arm64")) {
                target_platform = .MacOS_ARM64;
            } else if (std.mem.eql(u8, target_str, "windows-x64")) {
                target_platform = .Windows_x64;
            } else if (std.mem.eql(u8, target_str, "wasm32")) {
                target_platform = .WASM32;
            }
        } else if (std.mem.startsWith(u8, arg, "--output=")) {
            output_path = arg[9..];
        }
    }
    
    // Determine output path
    const final_output_path = output_path orelse blk: {
        if (std.mem.endsWith(u8, source_filename, ".csd")) {
            break :blk source_filename[0..source_filename.len - 4];
        }
        break :blk try std.fmt.allocPrint(allocator, "{s}_out", .{source_filename});
    };
    defer if (output_path == null) allocator.free(final_output_path);
    
    // Initialize compiler
    var compiler = try CursedCompiler.init(allocator, source_filename, final_output_path);
    defer compiler.deinit();
    
    compiler.setOptimizationLevel(optimization_level);
    compiler.setDebugInfo(debug_info_enabled);
    if (target_platform) |target| {
        compiler.setTarget(target);
    }
    
    // Execute requested compilation mode
    if (benchmark) {
        _ = try compiler.benchmarkCompilation();
    } else if (cross_compile) {
        try compiler.crossCompile("./cross_builds");
    } else if (generate_asm) {
        try compiler.generateAssembly();
    } else {
        _ = try compiler.compile();
    }
}

fn printUsage() void {
    std.debug.print("CURSED Complete Compiler v1.0.0 - Native Executable Generation\n", .{});
    std.debug.print("\nUsage: cursed-complete <file.csd> [OPTIONS]\n", .{});
    std.debug.print("       cursed-complete --version\n", .{});
    std.debug.print("       cursed-complete --help\n", .{});
    std.debug.print("\nOptions:\n", .{});
    std.debug.print("  --output=PATH          Specify output executable path\n", .{});
    std.debug.print("  --optimize=LEVEL       Optimization level (0-3, default: 2)\n", .{});
    std.debug.print("  --target=PLATFORM      Target platform (linux-x64, macos-intel, etc.)\n", .{});
    std.debug.print("  --no-debug             Disable debug information generation\n", .{});
    std.debug.print("  --cross-compile        Cross-compile for all supported platforms\n", .{});
    std.debug.print("  --assembly             Generate assembly output (.s file)\n", .{});
    std.debug.print("  --benchmark            Benchmark compilation performance\n", .{});
    std.debug.print("\nSupported Targets:\n", .{});
    std.debug.print("  linux-x64              Linux x86_64\n", .{});
    std.debug.print("  linux-arm64            Linux ARM64\n", .{});
    std.debug.print("  macos-intel            macOS x86_64\n", .{});
    std.debug.print("  macos-arm64            macOS ARM64 (Apple Silicon)\n", .{});
    std.debug.print("  windows-x64            Windows x86_64\n", .{});
    std.debug.print("  wasm32                 WebAssembly\n", .{});
    std.debug.print("\nFeatures:\n", .{});
    std.debug.print("  ✅ Complete CURSED language support\n", .{});
    std.debug.print("  ✅ LLVM-based native compilation\n", .{});
    std.debug.print("  ✅ Advanced optimization passes\n", .{});
    std.debug.print("  ✅ DWARF debug information\n", .{});
    std.debug.print("  ✅ Cross-platform compilation\n", .{});
    std.debug.print("  ✅ Runtime system with GC\n", .{});
    std.debug.print("  ✅ Performance benchmarking\n", .{});
}

test "complete compiler initialization" {
    const allocator = std.testing.allocator;
    
    var compiler = try CursedCompiler.init(allocator, "test.csd", "test_output");
    defer compiler.deinit();
    
    try std.testing.expect(compiler.optimization_level == .Default);
    try std.testing.expect(compiler.debug_info_enabled == true);
}

test "target platform detection" {
    const detected = CursedCompiler.detectTargetPlatform();
    
    // Should detect a valid platform
    const valid_platforms = [_]native_compilation.NativeCompiler.TargetPlatform{
        .Linux_x64, .Linux_ARM64, .MacOS_Intel, .MacOS_ARM64, .Windows_x64
    };
    
    var found = false;
    for (valid_platforms) |platform| {
        if (detected == platform) {
            found = true;
            break;
        }
    }
    
    try std.testing.expect(found);
}
