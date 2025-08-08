const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const enhanced_pipeline = @import("enhanced_llvm_pipeline.zig");
const EnhancedLLVMPipeline = enhanced_pipeline.EnhancedLLVMPipeline;
const ast = @import("ast.zig");

/// CURSED LLVM Compilation Manager
/// Orchestrates the complete compilation pipeline from CURSED AST to native executables
pub const LLVMCompilationManager = struct {
    allocator: Allocator,
    pipeline: EnhancedLLVMPipeline,
    compilation_config: CompilationConfig,
    output_config: OutputConfig,
    cross_compilation_targets: ArrayList(CrossCompilationTarget),
    
    const CompilationConfig = struct {
        optimization_level: enhanced_pipeline.EnhancedLLVMPipeline.OptimizationLevel = .Default,
        debug_info: bool = false,
        profile_guided_optimization: bool = false,
        link_time_optimization: bool = false,
        vectorization: bool = true,
        parallel_compilation: bool = false,
        emit_llvm_ir: bool = false,
        emit_assembly: bool = false,
        strip_debug_info: bool = false,
        static_linking: bool = false,
        
        // Advanced compilation options
        inline_threshold: u32 = 225,
        code_model: CodeModel = .Default,
        relocation_model: RelocationModel = .Default,
        float_abi: FloatABI = .Default,
        
        const CodeModel = enum {
            Default,
            Small,
            Kernel,
            Medium,
            Large,
        };
        
        const RelocationModel = enum {
            Default,
            Static,
            PIC,
            DynamicNoPIC,
        };
        
        const FloatABI = enum {
            Default,
            Soft,
            Hard,
        };
    };
    
    const OutputConfig = struct {
        output_dir: []const u8 = ".",
        output_name: []const u8 = "cursed_program",
        generate_debug_symbols: bool = false,
        generate_map_file: bool = false,
        generate_disassembly: bool = false,
        compress_output: bool = false,
    };
    
    const CrossCompilationTarget = struct {
        triple: []const u8,
        cpu: []const u8 = "generic",
        features: []const u8 = "",
        output_suffix: []const u8 = "",
        enabled: bool = true,
    };
    
    const CompilationResult = struct {
        success: bool,
        output_files: ArrayList([]const u8),
        compilation_time_ms: u64,
        optimization_time_ms: u64,
        code_size_bytes: u64,
        error_message: ?[]const u8 = null,
        
        pub fn deinit(self: *CompilationResult, allocator: Allocator) void {
            for (self.output_files.items) |file| {
                allocator.free(file);
            }
            self.output_files.deinit();
            
            if (self.error_message) |msg| {
                allocator.free(msg);
            }
        }
    };
    
    pub fn init(allocator: Allocator, module_name: []const u8) !LLVMCompilationManager {
        const pipeline = try EnhancedLLVMPipeline.init(allocator, module_name);
        
        return LLVMCompilationManager{
            .allocator = allocator,
            .pipeline = pipeline,
            .compilation_config = CompilationConfig{},
            .output_config = OutputConfig{},
            .cross_compilation_targets = ArrayList(CrossCompilationTarget).init(allocator),
        };
    }
    
    pub fn deinit(self: *LLVMCompilationManager) void {
        self.pipeline.deinit();
        
        // Free cross-compilation target strings
        for (self.cross_compilation_targets.items) |target| {
            self.allocator.free(target.triple);
            self.allocator.free(target.cpu);
            self.allocator.free(target.features);
            self.allocator.free(target.output_suffix);
        }
        self.cross_compilation_targets.deinit();
    }
    
    /// Configure compilation settings
    pub fn configureCompilation(self: *LLVMCompilationManager, config: CompilationConfig) !void {
        self.compilation_config = config;
        
        // Apply configuration to pipeline
        try self.pipeline.setOptimizationLevel(config.optimization_level);
        
        if (config.debug_info) {
            try self.pipeline.enableDebugInfo("program.csd", ".");
        }
        
        if (config.profile_guided_optimization) {
            try self.pipeline.enableProfileGuidedOptimization("profile.profdata");
        }
        
        if (config.link_time_optimization) {
            self.pipeline.enableLinkTimeOptimization();
        }
        
        self.pipeline.vectorization_enabled = config.vectorization;
        self.pipeline.parallel_compilation = config.parallel_compilation;
        
        std.debug.print("✅ Compilation configuration applied\n");
    }
    
    /// Add cross-compilation target
    pub fn addCrossCompilationTarget(self: *LLVMCompilationManager, triple: []const u8, cpu: []const u8, features: []const u8, suffix: []const u8) !void {
        const target = CrossCompilationTarget{
            .triple = try self.allocator.dupe(u8, triple),
            .cpu = try self.allocator.dupe(u8, cpu),
            .features = try self.allocator.dupe(u8, features),
            .output_suffix = try self.allocator.dupe(u8, suffix),
            .enabled = true,
        };
        
        try self.cross_compilation_targets.append(target);
        std.debug.print("✅ Cross-compilation target added: {s}\n", .{triple});
    }
    
    /// Compile CURSED program to native executable(s)
    pub fn compileProgram(self: *LLVMCompilationManager, program: ast.Program) !CompilationResult {
        const start_time = std.time.milliTimestamp();
        var result = CompilationResult{
            .success = false,
            .output_files = ArrayList([]const u8).init(self.allocator),
            .compilation_time_ms = 0,
            .optimization_time_ms = 0,
            .code_size_bytes = 0,
        };
        
        // Compile for native target first
        const native_result = try self.compileForTarget(program, "native", "", "", "");
        if (!native_result.success) {
            result.error_message = if (native_result.error_message) |msg| 
                try self.allocator.dupe(u8, msg) else null;
            return result;
        }
        
        // Add native output files
        for (native_result.output_files.items) |file| {
            try result.output_files.append(try self.allocator.dupe(u8, file));
        }
        
        // Compile for cross-compilation targets
        for (self.cross_compilation_targets.items) |target| {
            if (!target.enabled) continue;
            
            const cross_result = try self.compileForTarget(program, target.triple, target.cpu, target.features, target.output_suffix);
            if (cross_result.success) {
                for (cross_result.output_files.items) |file| {
                    try result.output_files.append(try self.allocator.dupe(u8, file));
                }
            } else {
                std.debug.print("⚠️ Cross-compilation failed for target: {s}\n", .{target.triple});
            }
        }
        
        const end_time = std.time.milliTimestamp();
        result.compilation_time_ms = @as(u64, @intCast(end_time - start_time));
        result.optimization_time_ms = self.pipeline.compilation_stats.optimization_time_ms;
        result.code_size_bytes = self.pipeline.compilation_stats.code_size_bytes;
        result.success = true;
        
        std.debug.print("✅ Program compilation completed in {} ms\n", .{result.compilation_time_ms});
        return result;
    }
    
    /// Compile for specific target
    fn compileForTarget(self: *LLVMCompilationManager, program: ast.Program, target_triple: []const u8, cpu: []const u8, features: []const u8, suffix: []const u8) !CompilationResult {
        _ = cpu;
        _ = features;
        var result = CompilationResult{
            .success = false,
            .output_files = ArrayList([]const u8).init(self.allocator),
            .compilation_time_ms = 0,
            .optimization_time_ms = 0,
            .code_size_bytes = 0,
        };
        
        // Setup target if not native
        if (!std.mem.eql(u8, target_triple, "native")) {
            try self.pipeline.setupTarget(target_triple);
        }
        
        // Compile program to LLVM IR
        try self.pipeline.compileProgram(program);
        
        // Run optimizations
        try self.pipeline.runOptimizations();
        
        // Validate language features
        try self.pipeline.validateLanguageFeatures();
        
        // Generate output files
        const base_name = if (suffix.len > 0)
            try std.fmt.allocPrint(self.allocator, "{s}_{s}", .{ self.output_config.output_name, suffix })
        else
            try self.allocator.dupe(u8, self.output_config.output_name);
        defer self.allocator.free(base_name);
        
        // Generate object file
        const object_file = try std.fmt.allocPrint(self.allocator, "{s}/{s}.o", .{ self.output_config.output_dir, base_name });
        try self.pipeline.generateObjectFile(object_file);
        try result.output_files.append(object_file);
        
        // Generate LLVM IR if requested
        if (self.compilation_config.emit_llvm_ir) {
            const ir_file = try std.fmt.allocPrint(self.allocator, "{s}/{s}.ll", .{ self.output_config.output_dir, base_name });
            try self.pipeline.generateLLVMIR(ir_file);
            try result.output_files.append(ir_file);
        }
        
        // Generate assembly if requested
        if (self.compilation_config.emit_assembly) {
            const asm_file = try std.fmt.allocPrint(self.allocator, "{s}/{s}.s", .{ self.output_config.output_dir, base_name });
            try self.pipeline.generateAssemblyFile(asm_file);
            try result.output_files.append(asm_file);
        }
        
        // Link to executable (placeholder - would need actual linker integration)
        if (!std.mem.eql(u8, target_triple, "native") or !self.compilation_config.static_linking) {
            const executable = try std.fmt.allocPrint(self.allocator, "{s}/{s}", .{ self.output_config.output_dir, base_name });
            try self.linkExecutable(object_file, executable, target_triple);
            try result.output_files.append(executable);
        }
        
        result.success = true;
        result.compilation_time_ms = self.pipeline.compilation_stats.compilation_time_ms;
        result.optimization_time_ms = self.pipeline.compilation_stats.optimization_time_ms;
        result.code_size_bytes = self.pipeline.compilation_stats.code_size_bytes;
        
        return result;
    }
    
    /// Link object file to executable (placeholder implementation)
    fn linkExecutable(self: *LLVMCompilationManager, object_file: []const u8, executable: []const u8, target_triple: []const u8) !void {
        _ = self;
        _ = object_file;
        _ = executable;
        _ = target_triple;
        
        // TODO: Implement actual linking with appropriate linker (ld, lld, etc.)
        // This would involve:
        // 1. Selecting appropriate linker for target
        // 2. Adding runtime library dependencies
        // 3. Setting up proper linking flags
        // 4. Handling static vs dynamic linking
        
        std.debug.print("✅ Executable linking completed (placeholder)\n");
    }
    
    /// Generate comprehensive compilation report
    pub fn generateCompilationReport(self: *LLVMCompilationManager, result: CompilationResult) ![]const u8 {
        var report = ArrayList(u8).init(self.allocator);
        
        try report.appendSlice("=== CURSED LLVM Compilation Report ===\n\n");
        
        // Compilation overview
        const overview = try std.fmt.allocPrint(self.allocator,
            "Compilation Status: {s}\n" ++
            "Total Time: {} ms\n" ++
            "Optimization Time: {} ms\n" ++
            "Code Size: {} bytes\n" ++
            "Output Files: {}\n\n",
            .{ 
                if (result.success) "SUCCESS" else "FAILED",
                result.compilation_time_ms,
                result.optimization_time_ms,
                result.code_size_bytes,
                result.output_files.items.len 
            });
        defer self.allocator.free(overview);
        try report.appendSlice(overview);
        
        // Configuration details
        try report.appendSlice("=== Configuration ===\n");
        const config_info = try std.fmt.allocPrint(self.allocator,
            "Optimization Level: {}\n" ++
            "Debug Info: {}\n" ++
            "Profile-Guided Optimization: {}\n" ++
            "Link-Time Optimization: {}\n" ++
            "Vectorization: {}\n" ++
            "Static Linking: {}\n\n",
            .{
                self.compilation_config.optimization_level,
                self.compilation_config.debug_info,
                self.compilation_config.profile_guided_optimization,
                self.compilation_config.link_time_optimization,
                self.compilation_config.vectorization,
                self.compilation_config.static_linking,
            });
        defer self.allocator.free(config_info);
        try report.appendSlice(config_info);
        
        // Output files
        try report.appendSlice("=== Output Files ===\n");
        for (result.output_files.items) |file| {
            const file_info = try std.fmt.allocPrint(self.allocator, "  {s}\n", .{file});
            defer self.allocator.free(file_info);
            try report.appendSlice(file_info);
        }
        
        // Cross-compilation targets
        if (self.cross_compilation_targets.items.len > 0) {
            try report.appendSlice("\n=== Cross-Compilation Targets ===\n");
            for (self.cross_compilation_targets.items) |target| {
                const target_info = try std.fmt.allocPrint(self.allocator,
                    "  {s} (CPU: {s}, Features: {s}) - {s}\n",
                    .{ target.triple, target.cpu, target.features, if (target.enabled) "ENABLED" else "DISABLED" });
                defer self.allocator.free(target_info);
                try report.appendSlice(target_info);
            }
        }
        
        // Error information
        if (result.error_message) |error_msg| {
            try report.appendSlice("\n=== Error Information ===\n");
            try report.appendSlice(error_msg);
            try report.appendSlice("\n");
        }
        
        // Pipeline statistics
        try report.appendSlice("\n=== Pipeline Statistics ===\n");
        self.pipeline.printStatistics();
        
        return report.toOwnedSlice();
    }
    
    /// Test compilation with sample CURSED program
    pub fn testCompilation(self: *LLVMCompilationManager) !void {
        std.debug.print("🧪 Testing CURSED LLVM compilation pipeline...\n");
        
        // Create sample program
        var program = ast.Program{
            .statements = ArrayList(ast.Statement).init(self.allocator),
        };
        defer program.statements.deinit();
        
        // Add sample statements (placeholders)
        // TODO: Create actual AST nodes for testing
        
        // Test compilation
        const result = try self.compileProgram(program);
        defer {
            var mutable_result = result;
            mutable_result.deinit(self.allocator);
        }
        
        if (result.success) {
            std.debug.print("✅ Test compilation successful!\n");
            
            // Generate and print report
            const report = try self.generateCompilationReport(result);
            defer self.allocator.free(report);
            std.debug.print("{s}\n", .{report});
        } else {
            std.debug.print("❌ Test compilation failed\n");
            if (result.error_message) |error_msg| {
                std.debug.print("Error: {s}\n", .{error_msg});
            }
        }
    }
};

/// High-level compilation API for CURSED programs
pub fn compileCursedProgram(
    allocator: Allocator,
    program: ast.Program,
    optimization_level: enhanced_pipeline.EnhancedLLVMPipeline.OptimizationLevel,
    output_name: []const u8,
    debug_info: bool
) !LLVMCompilationManager.CompilationResult {
    var manager = try LLVMCompilationManager.init(allocator, "cursed_program");
    defer manager.deinit();
    
    // Configure compilation
    const config = LLVMCompilationManager.CompilationConfig{
        .optimization_level = optimization_level,
        .debug_info = debug_info,
        .emit_llvm_ir = true,
        .emit_assembly = true,
    };
    
    try manager.configureCompilation(config);
    
    // Configure output
    manager.output_config.output_name = output_name;
    manager.output_config.generate_debug_symbols = debug_info;
    
    // Add common cross-compilation targets
    try manager.addCrossCompilationTarget("x86_64-linux-gnu", "generic", "", "linux_x64");
    try manager.addCrossCompilationTarget("aarch64-linux-gnu", "generic", "", "linux_arm64");
    try manager.addCrossCompilationTarget("x86_64-apple-darwin", "generic", "", "macos_x64");
    try manager.addCrossCompilationTarget("aarch64-apple-darwin", "generic", "", "macos_arm64");
    
    // Compile program
    return manager.compileProgram(program);
}

test "llvm compilation manager initialization" {
    const allocator = std.testing.allocator;
    
    var manager = try LLVMCompilationManager.init(allocator, "test_module");
    defer manager.deinit();
    
    try std.testing.expect(manager.compilation_config.optimization_level == .Default);
    try std.testing.expect(manager.cross_compilation_targets.items.len == 0);
}

test "cross compilation target management" {
    const allocator = std.testing.allocator;
    
    var manager = try LLVMCompilationManager.init(allocator, "test_module");
    defer manager.deinit();
    
    try manager.addCrossCompilationTarget("x86_64-linux-gnu", "generic", "", "linux");
    try std.testing.expect(manager.cross_compilation_targets.items.len == 1);
    try std.testing.expectEqualStrings(manager.cross_compilation_targets.items[0].triple, "x86_64-linux-gnu");
}
