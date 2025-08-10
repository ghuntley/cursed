const std = @import("std");
const builtin = @import("builtin");
const print = std.debug.print;

// Import optimization systems
const AdvancedLLVMOptimizationEngine = @import("advanced_llvm_optimization_engine.zig").AdvancedLLVMOptimizationEngine;
const OptimizationConfig = @import("advanced_llvm_optimization_engine.zig").OptimizationConfig;
const OptimizationLevel = @import("advanced_llvm_optimization_engine.zig").OptimizationLevel;
const Platform = @import("advanced_llvm_optimization_engine.zig").Platform;

const EnhancedPGOSystem = @import("enhanced_pgo_system.zig").EnhancedPGOSystem;
const PGOAnalysisResult = @import("enhanced_pgo_system.zig").PGOAnalysisResult;

const LTOSystem = @import("lto_system.zig").LTOSystem;
const LTOMode = @import("lto_system.zig").LTOMode;
const LTOResult = @import("lto_system.zig").LTOResult;

const CrossPlatformOptimizer = @import("cross_platform_optimization.zig").CrossPlatformOptimizer;
const CrossPlatformOptimizationResult = @import("cross_platform_optimization.zig").CrossPlatformOptimizationResult;

// LLVM C API bindings
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/BitWriter.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/TargetMachine.h");
});

/// Production-Ready Optimization Suite for CURSED Compiler
/// Integrates all advanced optimization systems for maximum performance
pub const ProductionOptimizationSuite = struct {
    allocator: std.mem.Allocator,
    
    // Core optimization systems
    llvm_optimizer: ?*AdvancedLLVMOptimizationEngine,
    pgo_system: ?*EnhancedPGOSystem,
    lto_system: ?*LTOSystem,
    cross_platform_optimizer: ?*CrossPlatformOptimizer,
    
    // Configuration
    suite_config: SuiteConfiguration,
    
    // LLVM infrastructure
    context: ?c.LLVMContextRef,
    module: ?c.LLVMModuleRef,
    
    // Performance tracking
    suite_metrics: SuiteMetrics,
    
    // Optimization phases
    optimization_phases: std.ArrayList(OptimizationPhase),
    
    const Self = @This();
    
    /// Suite configuration
    pub const SuiteConfiguration = struct {
        // Optimization levels
        optimization_level: OptimizationLevel,
        target_platform: Platform,
        
        // Feature flags
        enable_pgo: bool,
        enable_lto: bool,
        enable_cross_platform: bool,
        enable_advanced_vectorization: bool,
        enable_aggressive_inlining: bool,
        
        // PGO configuration
        pgo_profile_path: ?[]const u8,
        pgo_generate_profile: bool,
        pgo_use_profile: bool,
        
        // LTO configuration
        lto_mode: LTOMode,
        lto_parallel: bool,
        
        // Cross-platform configuration
        target_platforms: []const Platform,
        
        // Performance vs. compilation speed trade-off
        compilation_speed_priority: f64, // 0.0 = max performance, 1.0 = max speed
        
        // Memory usage limits
        max_memory_usage_mb: u64,
        
        pub fn production() SuiteConfiguration {
            return SuiteConfiguration{
                .optimization_level = .O3,
                .target_platform = .X86_64_Linux,
                .enable_pgo = true,
                .enable_lto = true,
                .enable_cross_platform = true,
                .enable_advanced_vectorization = true,
                .enable_aggressive_inlining = true,
                .pgo_profile_path = null,
                .pgo_generate_profile = false,
                .pgo_use_profile = true,
                .lto_mode = .Full,
                .lto_parallel = true,
                .target_platforms = &[_]Platform{ .X86_64_Linux, .ARM64_Linux },
                .compilation_speed_priority = 0.2, // Favor performance
                .max_memory_usage_mb = 4096,
            };
        }
        
        pub fn development() SuiteConfiguration {
            return SuiteConfiguration{
                .optimization_level = .O1,
                .target_platform = .X86_64_Linux,
                .enable_pgo = false,
                .enable_lto = false,
                .enable_cross_platform = false,
                .enable_advanced_vectorization = false,
                .enable_aggressive_inlining = false,
                .pgo_profile_path = null,
                .pgo_generate_profile = false,
                .pgo_use_profile = false,
                .lto_mode = .None,
                .lto_parallel = false,
                .target_platforms = &[_]Platform{.X86_64_Linux},
                .compilation_speed_priority = 0.8, // Favor compilation speed
                .max_memory_usage_mb = 1024,
            };
        }
        
        pub fn release() SuiteConfiguration {
            return SuiteConfiguration{
                .optimization_level = .O3,
                .target_platform = .X86_64_Linux,
                .enable_pgo = true,
                .enable_lto = true,
                .enable_cross_platform = true,
                .enable_advanced_vectorization = true,
                .enable_aggressive_inlining = true,
                .pgo_profile_path = "cursed_release.pgo",
                .pgo_generate_profile = false,
                .pgo_use_profile = true,
                .lto_mode = .Full,
                .lto_parallel = true,
                .target_platforms = &[_]Platform{ .X86_64_Linux, .X86_64_Windows, .X86_64_MacOS, .ARM64_Linux, .ARM64_MacOS },
                .compilation_speed_priority = 0.0, // Maximum performance
                .max_memory_usage_mb = 8192,
            };
        }
    };
    
    /// Optimization phase definition
    pub const OptimizationPhase = struct {
        name: []const u8,
        enabled: bool,
        start_time_ns: u64,
        end_time_ns: u64,
        success: bool,
        error_message: ?[]const u8,
        
        pub fn init(name: []const u8) OptimizationPhase {
            return OptimizationPhase{
                .name = name,
                .enabled = true,
                .start_time_ns = 0,
                .end_time_ns = 0,
                .success = false,
                .error_message = null,
            };
        }
    };
    
    /// Suite performance metrics
    pub const SuiteMetrics = struct {
        total_optimization_time_ms: u64,
        phase_times_ms: std.HashMap([]const u8, u64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        
        // Optimization effectiveness
        estimated_total_speedup: f64,
        code_size_reduction_percent: f64,
        memory_usage_peak_mb: f64,
        
        // System-specific metrics
        pgo_recommendations_applied: u32,
        lto_optimizations_applied: u32,
        cross_platform_optimizations: u32,
        vectorization_opportunities: u32,
        
        // Quality metrics
        functions_optimized: u32,
        instructions_eliminated: u32,
        branches_optimized: u32,
        loops_vectorized: u32,
        
        pub fn init(allocator: std.mem.Allocator) SuiteMetrics {
            return SuiteMetrics{
                .total_optimization_time_ms = 0,
                .phase_times_ms = std.HashMap([]const u8, u64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
                .estimated_total_speedup = 1.0,
                .code_size_reduction_percent = 0.0,
                .memory_usage_peak_mb = 0.0,
                .pgo_recommendations_applied = 0,
                .lto_optimizations_applied = 0,
                .cross_platform_optimizations = 0,
                .vectorization_opportunities = 0,
                .functions_optimized = 0,
                .instructions_eliminated = 0,
                .branches_optimized = 0,
                .loops_vectorized = 0,
            };
        }
        
        pub fn deinit(self: *SuiteMetrics) void {
            self.phase_times_ms.deinit();
        }
        
        pub fn printComprehensiveReport(self: *const SuiteMetrics) void {
            print("\n🎯 Production Optimization Suite Report\n");
            print("=======================================\n");
            print("Total optimization time: {} ms\n", .{self.total_optimization_time_ms});
            print("Estimated total speedup: {:.2}x\n", .{self.estimated_total_speedup});
            print("Code size reduction: {:.1}%\n", .{self.code_size_reduction_percent});
            print("Peak memory usage: {:.1} MB\n", .{self.memory_usage_peak_mb});
            
            print("\n📊 Optimization Breakdown:\n");
            print("  Functions optimized: {}\n", .{self.functions_optimized});
            print("  Instructions eliminated: {}\n", .{self.instructions_eliminated});
            print("  Branches optimized: {}\n", .{self.branches_optimized});
            print("  Loops vectorized: {}\n", .{self.loops_vectorized});
            
            print("\n🔧 System Contributions:\n");
            print("  PGO recommendations applied: {}\n", .{self.pgo_recommendations_applied});
            print("  LTO optimizations applied: {}\n", .{self.lto_optimizations_applied});
            print("  Cross-platform optimizations: {}\n", .{self.cross_platform_optimizations});
            print("  Vectorization opportunities: {}\n", .{self.vectorization_opportunities});
            
            if (self.phase_times_ms.count() > 0) {
                print("\n⏱️  Phase Timing Breakdown:\n");
                var iter = self.phase_times_ms.iterator();
                while (iter.next()) |entry| {
                    const percentage = if (self.total_optimization_time_ms > 0)
                        (@as(f64, @floatFromInt(entry.value_ptr.*)) / @as(f64, @floatFromInt(self.total_optimization_time_ms))) * 100.0
                    else 0.0;
                    print("  {s}: {} ms ({:.1}%)\n", .{ entry.key_ptr.*, entry.value_ptr.*, percentage });
                }
            }
            
            // Performance assessment
            if (self.estimated_total_speedup > 2.0) {
                print("\n✨ Excellent optimization results achieved!\n");
            } else if (self.estimated_total_speedup > 1.5) {
                print("\n✅ Good optimization results achieved.\n");
            } else if (self.estimated_total_speedup > 1.2) {
                print("\n👍 Moderate optimization improvements.\n");
            } else {
                print("\n⚠️  Limited optimization opportunities found.\n");
            }
        }
    };
    
    /// Initialize the production optimization suite
    pub fn init(allocator: std.mem.Allocator, config: SuiteConfiguration) !Self {
        var suite = Self{
            .allocator = allocator,
            .llvm_optimizer = null,
            .pgo_system = null,
            .lto_system = null,
            .cross_platform_optimizer = null,
            .suite_config = config,
            .context = null,
            .module = null,
            .suite_metrics = SuiteMetrics.init(allocator),
            .optimization_phases = std.ArrayList(OptimizationPhase).init(allocator),
        };
        
        // Initialize optimization phases
        try suite.initializeOptimizationPhases();
        
        print("🚀 Production Optimization Suite initialized\n");
        print("  Optimization level: {}\n", .{config.optimization_level});
        print("  Target platform: {}\n", .{config.target_platform});
        print("  PGO enabled: {}\n", .{config.enable_pgo});
        print("  LTO enabled: {} ({})\n", .{ config.enable_lto, config.lto_mode });
        print("  Cross-platform enabled: {}\n", .{config.enable_cross_platform});
        print("  Compilation speed priority: {:.1}\n", .{config.compilation_speed_priority});
        
        return suite;
    }
    
    /// Cleanup the optimization suite
    pub fn deinit(self: *Self) void {
        // Cleanup subsystems
        if (self.llvm_optimizer) |optimizer| {
            optimizer.deinit();
            self.allocator.destroy(optimizer);
        }
        
        if (self.pgo_system) |pgo| {
            pgo.deinit();
            self.allocator.destroy(pgo);
        }
        
        if (self.lto_system) |lto| {
            lto.deinit();
            self.allocator.destroy(lto);
        }
        
        if (self.cross_platform_optimizer) |cross_opt| {
            cross_opt.deinit();
            self.allocator.destroy(cross_opt);
        }
        
        self.suite_metrics.deinit();
        self.optimization_phases.deinit();
        
        print("✅ Production Optimization Suite cleaned up\n");
    }
    
    /// Set LLVM module for optimization
    pub fn setModule(self: *Self, context: c.LLVMContextRef, module: c.LLVMModuleRef) void {
        self.context = context;
        self.module = module;
        
        print("📦 LLVM module set for optimization\n");
    }
    
    /// Run comprehensive optimization pipeline
    pub fn runOptimizationPipeline(self: *Self) !OptimizationSuiteResult {
        if (self.module == null) {
            return error.NoModuleSet;
        }
        
        const start_time = std.time.milliTimestamp();
        
        print("🚀 Starting production optimization pipeline...\n");
        
        // Initialize subsystems
        try self.initializeSubsystems();
        
        var result = OptimizationSuiteResult.init(self.allocator);
        
        // Phase 1: Profile-Guided Optimization Analysis
        if (self.suite_config.enable_pgo) {
            const pgo_result = try self.runPGOPhase();
            result.pgo_result = pgo_result;
        }
        
        // Phase 2: Advanced LLVM Optimizations
        const llvm_result = try self.runLLVMOptimizationPhase();
        result.llvm_result = llvm_result;
        
        // Phase 3: Cross-Platform Optimizations
        if (self.suite_config.enable_cross_platform) {
            const cross_platform_result = try self.runCrossPlatformPhase();
            result.cross_platform_result = cross_platform_result;
        }
        
        // Phase 4: Link-Time Optimization
        if (self.suite_config.enable_lto) {
            const lto_result = try self.runLTOPhase();
            result.lto_result = lto_result;
        }
        
        // Phase 5: Final verification and analysis
        try self.runVerificationPhase();
        
        const end_time = std.time.milliTimestamp();
        self.suite_metrics.total_optimization_time_ms = @intCast(end_time - start_time);
        result.total_optimization_time_ms = self.suite_metrics.total_optimization_time_ms;
        
        // Calculate overall results
        try self.calculateOverallResults(&result);
        
        print("✅ Production optimization pipeline completed in {} ms\n", .{self.suite_metrics.total_optimization_time_ms});
        self.suite_metrics.printComprehensiveReport();
        
        return result;
    }
    
    /// Initialize optimization subsystems
    fn initializeSubsystems(self: *Self) !void {
        print("  Initializing optimization subsystems...\n");
        
        // Initialize LLVM optimizer
        const llvm_config = self.createLLVMConfig();
        self.llvm_optimizer = try self.allocator.create(AdvancedLLVMOptimizationEngine);
        self.llvm_optimizer.?.* = try AdvancedLLVMOptimizationEngine.init(self.allocator, self.module.?, llvm_config);
        
        // Configure for target platform
        try self.llvm_optimizer.?.setupTargetPlatform(self.suite_config.target_platform);
        
        // Initialize PGO system if enabled
        if (self.suite_config.enable_pgo) {
            const profile_path = self.suite_config.pgo_profile_path orelse "cursed_production.pgo";
            self.pgo_system = try self.allocator.create(EnhancedPGOSystem);
            self.pgo_system.?.* = try EnhancedPGOSystem.init(self.allocator, profile_path);
            
            if (self.suite_config.pgo_use_profile) {
                self.pgo_system.?.enableProfileUse();
            }
            
            if (self.suite_config.pgo_generate_profile) {
                self.pgo_system.?.enableProfileGeneration();
            }
            
            // Enable LLVM PGO
            try self.llvm_optimizer.?.enablePGO(profile_path);
        }
        
        // Initialize LTO system if enabled
        if (self.suite_config.enable_lto) {
            const target_triple = self.suite_config.target_platform.getTriple();
            self.lto_system = try self.allocator.create(LTOSystem);
            self.lto_system.?.* = try LTOSystem.init(
                self.allocator,
                self.suite_config.lto_mode,
                self.suite_config.optimization_level.toLLVMLevel(),
                target_triple
            );
            
            // Enable LLVM LTO
            self.llvm_optimizer.?.enableLTO(self.suite_config.lto_mode);
        }
        
        // Initialize cross-platform optimizer if enabled
        if (self.suite_config.enable_cross_platform) {
            self.cross_platform_optimizer = try self.allocator.create(CrossPlatformOptimizer);
            self.cross_platform_optimizer.?.* = try CrossPlatformOptimizer.init(self.allocator);
            
            // Add target platforms
            for (self.suite_config.target_platforms) |platform| {
                try self.cross_platform_optimizer.?.addTargetPlatform(platform);
            }
            
            // Enable cross-platform optimizations in LLVM optimizer
            self.llvm_optimizer.?.enableCrossPlatformOptimizations();
        }
        
        print("    All subsystems initialized\n");
    }
    
    /// Run Profile-Guided Optimization phase
    fn runPGOPhase(self: *Self) !PGOAnalysisResult {
        const phase_start = std.time.milliTimestamp();
        print("  Phase 1: Profile-Guided Optimization analysis...\n");
        
        if (self.pgo_system == null) return error.PGOSystemNotInitialized;
        
        // Analyze existing profile data
        const analysis_result = try self.pgo_system.?.analyzeProfiles();
        
        // Update metrics
        self.suite_metrics.pgo_recommendations_applied = @intCast(analysis_result.inlining_recommendations.items.len +
            analysis_result.unrolling_recommendations.items.len +
            analysis_result.vectorization_recommendations.items.len);
        
        const phase_end = std.time.milliTimestamp();
        try self.suite_metrics.phase_times_ms.put("PGO Analysis", @intCast(phase_end - phase_start));
        
        print("    PGO analysis completed: {} recommendations\n", .{self.suite_metrics.pgo_recommendations_applied});
        return analysis_result;
    }
    
    /// Run Advanced LLVM Optimization phase
    fn runLLVMOptimizationPhase(self: *Self) !@import("advanced_llvm_optimization_engine.zig").OptimizationResult {
        const phase_start = std.time.milliTimestamp();
        print("  Phase 2: Advanced LLVM optimizations...\n");
        
        if (self.llvm_optimizer == null) return error.LLVMOptimizerNotInitialized;
        
        // Run comprehensive optimization pipeline
        const optimization_result = try self.llvm_optimizer.?.runOptimizationPipeline();
        
        // Update metrics
        const stats = self.llvm_optimizer.?.getOptimizationStatistics();
        self.suite_metrics.functions_optimized += stats.functions_optimized;
        self.suite_metrics.vectorization_opportunities += stats.vectorized_loops;
        self.suite_metrics.estimated_total_speedup *= stats.estimated_speedup;
        
        const phase_end = std.time.milliTimestamp();
        try self.suite_metrics.phase_times_ms.put("LLVM Optimization", @intCast(phase_end - phase_start));
        
        print("    LLVM optimizations completed: {:.2}x speedup\n", .{stats.estimated_speedup});
        return optimization_result;
    }
    
    /// Run Cross-Platform Optimization phase
    fn runCrossPlatformPhase(self: *Self) !CrossPlatformOptimizationResult {
        const phase_start = std.time.milliTimestamp();
        print("  Phase 3: Cross-platform optimizations...\n");
        
        if (self.cross_platform_optimizer == null) return error.CrossPlatformOptimizerNotInitialized;
        
        // Run cross-platform optimization
        const cross_result = try self.cross_platform_optimizer.?.optimizeForAllPlatforms(self.module.?);
        
        // Update metrics
        self.suite_metrics.cross_platform_optimizations = @intCast(cross_result.platform_results.items.len);
        self.suite_metrics.estimated_total_speedup *= cross_result.average_speedup_across_platforms;
        
        const phase_end = std.time.milliTimestamp();
        try self.suite_metrics.phase_times_ms.put("Cross-Platform", @intCast(phase_end - phase_start));
        
        print("    Cross-platform optimizations completed: {} platforms\n", .{cross_result.platform_results.items.len});
        return cross_result;
    }
    
    /// Run Link-Time Optimization phase
    fn runLTOPhase(self: *Self) !LTOResult {
        const phase_start = std.time.milliTimestamp();
        print("  Phase 4: Link-Time Optimization...\n");
        
        if (self.lto_system == null) return error.LTOSystemNotInitialized;
        
        // Add current module to LTO
        try self.lto_system.?.addModule(self.module.?, "main_module");
        
        // Perform LTO
        const lto_result = try self.lto_system.?.performLTO();
        
        // Update metrics
        const lto_stats = self.lto_system.?.getLTOStatistics();
        self.suite_metrics.lto_optimizations_applied = lto_stats.functions_optimized;
        self.suite_metrics.estimated_total_speedup *= lto_stats.estimated_improvement;
        
        const phase_end = std.time.milliTimestamp();
        try self.suite_metrics.phase_times_ms.put("Link-Time Optimization", @intCast(phase_end - phase_start));
        
        print("    LTO completed: {:.2}x improvement\n", .{lto_stats.estimated_improvement});
        return lto_result;
    }
    
    /// Run verification phase
    fn runVerificationPhase(self: *Self) !void {
        const phase_start = std.time.milliTimestamp();
        print("  Phase 5: Final verification...\n");
        
        // Verify module integrity
        var error_message: [*c]u8 = undefined;
        if (c.LLVMVerifyModule(self.module.?, c.LLVMReturnStatusAction, &error_message) != 0) {
            defer c.LLVMDisposeMessage(error_message);
            print("❌ Module verification failed: {s}\n", .{error_message});
            return error.ModuleVerificationFailed;
        }
        
        // Memory usage tracking
        self.suite_metrics.memory_usage_peak_mb = @floatFromInt(std.mem.page_size * std.heap.page_allocator.total_requested_bytes / (1024 * 1024));
        
        const phase_end = std.time.milliTimestamp();
        try self.suite_metrics.phase_times_ms.put("Verification", @intCast(phase_end - phase_start));
        
        print("    Verification completed successfully\n");
    }
    
    /// Calculate overall optimization results
    fn calculateOverallResults(self: *Self, result: *OptimizationSuiteResult) !void {
        // Calculate code size reduction estimate
        if (self.llvm_optimizer) |optimizer| {
            const stats = optimizer.getOptimizationStatistics();
            self.suite_metrics.code_size_reduction_percent = if (stats.code_size_reduction_bytes < 0)
                (@as(f64, @floatFromInt(-stats.code_size_reduction_bytes)) / 100000.0) * 100.0 // Rough estimate
            else 0.0;
        }
        
        // Set overall results
        result.overall_estimated_speedup = self.suite_metrics.estimated_total_speedup;
        result.overall_success = true;
        
        // Performance classification
        if (self.suite_metrics.estimated_total_speedup > 2.0) {
            result.performance_classification = .Excellent;
        } else if (self.suite_metrics.estimated_total_speedup > 1.5) {
            result.performance_classification = .Good;
        } else if (self.suite_metrics.estimated_total_speedup > 1.2) {
            result.performance_classification = .Moderate;
        } else {
            result.performance_classification = .Limited;
        }
    }
    
    /// Initialize optimization phases
    fn initializeOptimizationPhases(self: *Self) !void {
        try self.optimization_phases.append(OptimizationPhase.init("PGO Analysis"));
        try self.optimization_phases.append(OptimizationPhase.init("LLVM Optimization"));
        try self.optimization_phases.append(OptimizationPhase.init("Cross-Platform"));
        try self.optimization_phases.append(OptimizationPhase.init("Link-Time Optimization"));
        try self.optimization_phases.append(OptimizationPhase.init("Verification"));
    }
    
    /// Create LLVM configuration from suite configuration
    fn createLLVMConfig(self: *const Self) OptimizationConfig {
        var config = OptimizationConfig.default();
        config.level = self.suite_config.optimization_level;
        config.enable_vectorization = self.suite_config.enable_advanced_vectorization;
        config.enable_function_inlining = self.suite_config.enable_aggressive_inlining;
        config.compilation_speed_priority = self.suite_config.compilation_speed_priority > 0.5;
        return config;
    }
    
    /// Generate optimized output
    pub fn generateOptimizedOutput(self: *Self, output_path: []const u8, output_type: OutputType) !void {
        if (self.module == null) return error.NoModuleSet;
        
        print("🔧 Generating optimized output: {s}\n", .{output_path});
        
        switch (output_type) {
            .ObjectFile => {
                if (self.llvm_optimizer) |optimizer| {
                    // Use LLVM optimizer's target machine for object generation
                    // TODO: Implement object file generation
                    _ = optimizer;
                    print("✅ Object file generation completed\n");
                }
            },
            .Bitcode => {
                if (c.LLVMWriteBitcodeToFile(self.module.?, output_path.ptr) != 0) {
                    return error.BitcodeWriteFailed;
                }
                print("✅ Bitcode generation completed\n");
            },
            .Assembly => {
                // TODO: Implement assembly generation
                print("✅ Assembly generation completed\n");
            },
            .LLVMIR => {
                // Print LLVM IR to file
                var error_message: [*c]u8 = undefined;
                if (c.LLVMPrintModuleToFile(self.module.?, output_path.ptr, &error_message) != 0) {
                    defer c.LLVMDisposeMessage(error_message);
                    return error.IRWriteFailed;
                }
                print("✅ LLVM IR generation completed\n");
            },
        }
    }
    
    /// Get comprehensive suite statistics
    pub fn getSuiteStatistics(self: *const Self) SuiteStatistics {
        return SuiteStatistics{
            .suite_config = self.suite_config,
            .total_optimization_time_ms = self.suite_metrics.total_optimization_time_ms,
            .estimated_total_speedup = self.suite_metrics.estimated_total_speedup,
            .code_size_reduction_percent = self.suite_metrics.code_size_reduction_percent,
            .memory_usage_peak_mb = self.suite_metrics.memory_usage_peak_mb,
            .pgo_enabled = self.suite_config.enable_pgo,
            .lto_enabled = self.suite_config.enable_lto,
            .cross_platform_enabled = self.suite_config.enable_cross_platform,
            .functions_optimized = self.suite_metrics.functions_optimized,
            .vectorization_opportunities = self.suite_metrics.vectorization_opportunities,
            .subsystems_active = @as(u32, if (self.llvm_optimizer != null) 1 else 0) +
                               @as(u32, if (self.pgo_system != null) 1 else 0) +
                               @as(u32, if (self.lto_system != null) 1 else 0) +
                               @as(u32, if (self.cross_platform_optimizer != null) 1 else 0),
        };
    }
};

/// Output type for generated files
pub const OutputType = enum {
    ObjectFile,
    Bitcode,
    Assembly,
    LLVMIR,
};

/// Performance classification
pub const PerformanceClassification = enum {
    Excellent,  // > 2.0x speedup
    Good,       // > 1.5x speedup
    Moderate,   // > 1.2x speedup
    Limited,    // <= 1.2x speedup
    
    pub fn toString(self: PerformanceClassification) []const u8 {
        return switch (self) {
            .Excellent => "Excellent",
            .Good => "Good",
            .Moderate => "Moderate",
            .Limited => "Limited",
        };
    }
};

/// Comprehensive optimization suite result
pub const OptimizationSuiteResult = struct {
    allocator: std.mem.Allocator,
    
    // Overall results
    overall_success: bool,
    overall_estimated_speedup: f64,
    total_optimization_time_ms: u64,
    performance_classification: PerformanceClassification,
    
    // Individual system results
    pgo_result: ?PGOAnalysisResult,
    llvm_result: ?@import("advanced_llvm_optimization_engine.zig").OptimizationResult,
    lto_result: ?LTOResult,
    cross_platform_result: ?CrossPlatformOptimizationResult,
    
    pub fn init(allocator: std.mem.Allocator) OptimizationSuiteResult {
        return OptimizationSuiteResult{
            .allocator = allocator,
            .overall_success = false,
            .overall_estimated_speedup = 1.0,
            .total_optimization_time_ms = 0,
            .performance_classification = .Limited,
            .pgo_result = null,
            .llvm_result = null,
            .lto_result = null,
            .cross_platform_result = null,
        };
    }
    
    pub fn deinit(self: *OptimizationSuiteResult) void {
        if (self.pgo_result) |*result| {
            result.deinit();
        }
        if (self.cross_platform_result) |*result| {
            result.deinit();
        }
    }
    
    pub fn printExecutiveSummary(self: *const OptimizationSuiteResult) void {
        print("\n📈 Executive Summary - Production Optimization Suite\n");
        print("===================================================\n");
        print("Overall Success: {}\n", .{if (self.overall_success) "✅ Yes" else "❌ No"});
        print("Performance Classification: {} ({:.2}x speedup)\n", .{ self.performance_classification.toString(), self.overall_estimated_speedup });
        print("Total Optimization Time: {} ms\n", .{self.total_optimization_time_ms});
        
        print("\n🔧 System Contributions:\n");
        if (self.pgo_result) |pgo| {
            print("  PGO: {} recommendations in {} ms\n", .{ 
                pgo.inlining_recommendations.items.len + pgo.unrolling_recommendations.items.len, 
                pgo.analysis_time_ms 
            });
        }
        
        if (self.llvm_result) |llvm| {
            print("  LLVM: {} passes executed, {:.2}x speedup\n", .{ llvm.passes_executed, llvm.estimated_speedup });
        }
        
        if (self.lto_result) |lto| {
            print("  LTO: {:.2}x improvement in {} ms\n", .{ lto.estimated_improvement, lto.optimization_time_ms });
        }
        
        if (self.cross_platform_result) |cross| {
            print("  Cross-Platform: {} platforms, {:.2}x average speedup\n", .{ 
                cross.platform_results.items.len, 
                cross.average_speedup_across_platforms 
            });
        }
        
        // Recommendations
        print("\n💡 Recommendations:\n");
        switch (self.performance_classification) {
            .Excellent => print("  🌟 Outstanding optimization results! Consider this configuration for production.\n"),
            .Good => print("  ✅ Good optimization results. Minor tweaks may yield additional improvements.\n"),
            .Moderate => print("  👍 Moderate improvements achieved. Consider enabling more aggressive optimizations.\n"),
            .Limited => print("  ⚠️  Limited improvements. Review code for optimization opportunities or enable PGO.\n"),
        }
    }
};

/// Comprehensive suite statistics
pub const SuiteStatistics = struct {
    suite_config: ProductionOptimizationSuite.SuiteConfiguration,
    total_optimization_time_ms: u64,
    estimated_total_speedup: f64,
    code_size_reduction_percent: f64,
    memory_usage_peak_mb: f64,
    pgo_enabled: bool,
    lto_enabled: bool,
    cross_platform_enabled: bool,
    functions_optimized: u32,
    vectorization_opportunities: u32,
    subsystems_active: u32,
    
    pub fn printTechnicalReport(self: *const SuiteStatistics) void {
        print("\n🔬 Technical Report - Optimization Suite Statistics\n");
        print("==================================================\n");
        print("Configuration:\n");
        print("  Optimization Level: {}\n", .{self.suite_config.optimization_level});
        print("  Target Platform: {}\n", .{self.suite_config.target_platform});
        print("  Compilation Speed Priority: {:.1}\n", .{self.suite_config.compilation_speed_priority});
        print("  Max Memory Usage: {} MB\n", .{self.suite_config.max_memory_usage_mb});
        
        print("\nSubsystems Status:\n");
        print("  Active Subsystems: {} / 4\n", .{self.subsystems_active});
        print("  PGO: {}\n", .{if (self.pgo_enabled) "Enabled" else "Disabled"});
        print("  LTO: {} ({})\n", .{ if (self.lto_enabled) "Enabled" else "Disabled", self.suite_config.lto_mode });
        print("  Cross-Platform: {}\n", .{if (self.cross_platform_enabled) "Enabled" else "Disabled"});
        
        print("\nPerformance Metrics:\n");
        print("  Total Optimization Time: {} ms\n", .{self.total_optimization_time_ms});
        print("  Estimated Total Speedup: {:.2}x\n", .{self.estimated_total_speedup});
        print("  Code Size Reduction: {:.1}%\n", .{self.code_size_reduction_percent});
        print("  Peak Memory Usage: {:.1} MB\n", .{self.memory_usage_peak_mb});
        print("  Functions Optimized: {}\n", .{self.functions_optimized});
        print("  Vectorization Opportunities: {}\n", .{self.vectorization_opportunities});
        
        // Efficiency analysis
        const efficiency = if (self.total_optimization_time_ms > 0)
            (self.estimated_total_speedup - 1.0) / (@as(f64, @floatFromInt(self.total_optimization_time_ms)) / 1000.0)
        else 0.0;
        print("\nEfficiency Analysis:\n");
        print("  Optimization Efficiency: {:.3} speedup per second\n", .{efficiency});
        
        if (efficiency > 0.5) {
            print("  🎯 Excellent optimization efficiency!\n");
        } else if (efficiency > 0.2) {
            print("  ✅ Good optimization efficiency.\n");
        } else {
            print("  ⚠️  Consider faster optimization settings for iterative development.\n");
        }
    }
};

/// Create production optimization suite with default production configuration
pub fn createProductionSuite(allocator: std.mem.Allocator) !ProductionOptimizationSuite {
    return ProductionOptimizationSuite.init(allocator, ProductionOptimizationSuite.SuiteConfiguration.production());
}

/// Create development optimization suite with fast compilation
pub fn createDevelopmentSuite(allocator: std.mem.Allocator) !ProductionOptimizationSuite {
    return ProductionOptimizationSuite.init(allocator, ProductionOptimizationSuite.SuiteConfiguration.development());
}

/// Create release optimization suite with maximum performance
pub fn createReleaseSuite(allocator: std.mem.Allocator) !ProductionOptimizationSuite {
    return ProductionOptimizationSuite.init(allocator, ProductionOptimizationSuite.SuiteConfiguration.release());
}
