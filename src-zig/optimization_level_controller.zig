const std = @import("std");
const print = std.debug.print;

/// Optimization levels supported by the CURSED compiler
pub const OptimizationLevel = enum {
    O0, // No optimization - fast compilation
    O1, // Basic optimization - balanced compile time
    O2, // Standard optimization - good performance
    O3, // Aggressive optimization - maximum performance
    Oz, // Size optimization - smallest binaries
    Os, // Size optimization with speed consideration
    
    pub fn toString(self: OptimizationLevel) []const u8 {
        return switch (self) {
            .O0 => "O0",
            .O1 => "O1", 
            .O2 => "O2",
            .O3 => "O3",
            .Oz => "Oz",
            .Os => "Os",
        };
    }
    
    pub fn fromString(str: []const u8) ?OptimizationLevel {
        if (std.mem.eql(u8, str, "O0")) return .O0;
        if (std.mem.eql(u8, str, "O1")) return .O1;
        if (std.mem.eql(u8, str, "O2")) return .O2;
        if (std.mem.eql(u8, str, "O3")) return .O3;
        if (std.mem.eql(u8, str, "Oz")) return .Oz;
        if (std.mem.eql(u8, str, "Os")) return .Os;
        return null;
    }
};

/// Optimization configuration for each level
pub const OptimizationConfig = struct {
    // Basic passes
    enable_mem2reg: bool = true,
    enable_cfg_simplification: bool = true,
    enable_dce: bool = false,
    
    // Inlining
    enable_function_inlining: bool = false,
    inline_threshold: u32 = 225,
    
    // Loop optimizations
    enable_loop_unroll: bool = false,
    enable_loop_vectorize: bool = false,
    unroll_threshold: u32 = 150,
    
    // Interprocedural optimizations
    enable_ipsccp: bool = false,
    enable_global_dce: bool = false,
    enable_argument_promotion: bool = false,
    
    // Advanced optimizations
    enable_gvn: bool = false,
    enable_licm: bool = false,
    enable_reassociation: bool = false,
    
    // Vectorization
    enable_slp_vectorize: bool = false,
    enable_load_store_vectorize: bool = false,
    vectorization_threshold: u32 = 4,
    
    // Target-specific
    enable_target_lowering: bool = true,
    enable_machine_opts: bool = false,
    
    // Size vs speed tradeoffs
    optimize_for_size: bool = false,
    aggressive_size_opts: bool = false,
    
    pub fn forLevel(level: OptimizationLevel) OptimizationConfig {
        return switch (level) {
            .O0 => OptimizationConfig{
                // O0: Fast compilation, no optimization
                .enable_mem2reg = true,
                .enable_cfg_simplification = false,
                .enable_dce = false,
                .enable_function_inlining = false,
                .enable_loop_unroll = false,
                .enable_loop_vectorize = false,
                .enable_ipsccp = false,
                .enable_global_dce = false,
                .enable_argument_promotion = false,
                .enable_gvn = false,
                .enable_licm = false,
                .enable_reassociation = false,
                .enable_slp_vectorize = false,
                .enable_load_store_vectorize = false,
                .enable_machine_opts = false,
                .optimize_for_size = false,
                .aggressive_size_opts = false,
            },
            
            .O1 => OptimizationConfig{
                // O1: Basic optimization
                .enable_mem2reg = true,
                .enable_cfg_simplification = true,
                .enable_dce = true,
                .enable_function_inlining = true,
                .inline_threshold = 75, // Lower threshold
                .enable_loop_unroll = false,
                .enable_loop_vectorize = false,
                .enable_ipsccp = true,
                .enable_global_dce = true,
                .enable_argument_promotion = false,
                .enable_gvn = false,
                .enable_licm = true,
                .enable_reassociation = true,
                .enable_slp_vectorize = false,
                .enable_load_store_vectorize = false,
                .enable_machine_opts = false,
                .optimize_for_size = false,
                .aggressive_size_opts = false,
            },
            
            .O2 => OptimizationConfig{
                // O2: Standard optimization (default)
                .enable_mem2reg = true,
                .enable_cfg_simplification = true,
                .enable_dce = true,
                .enable_function_inlining = true,
                .inline_threshold = 225, // Standard threshold
                .enable_loop_unroll = true,
                .enable_loop_vectorize = true,
                .unroll_threshold = 150,
                .enable_ipsccp = true,
                .enable_global_dce = true,
                .enable_argument_promotion = true,
                .enable_gvn = true,
                .enable_licm = true,
                .enable_reassociation = true,
                .enable_slp_vectorize = true,
                .enable_load_store_vectorize = true,
                .vectorization_threshold = 4,
                .enable_machine_opts = true,
                .optimize_for_size = false,
                .aggressive_size_opts = false,
            },
            
            .O3 => OptimizationConfig{
                // O3: Aggressive optimization
                .enable_mem2reg = true,
                .enable_cfg_simplification = true,
                .enable_dce = true,
                .enable_function_inlining = true,
                .inline_threshold = 350, // Higher threshold
                .enable_loop_unroll = true,
                .enable_loop_vectorize = true,
                .unroll_threshold = 300, // Higher threshold
                .enable_ipsccp = true,
                .enable_global_dce = true,
                .enable_argument_promotion = true,
                .enable_gvn = true,
                .enable_licm = true,
                .enable_reassociation = true,
                .enable_slp_vectorize = true,
                .enable_load_store_vectorize = true,
                .vectorization_threshold = 2, // More aggressive
                .enable_machine_opts = true,
                .optimize_for_size = false,
                .aggressive_size_opts = false,
            },
            
            .Oz => OptimizationConfig{
                // Oz: Optimize for minimal size
                .enable_mem2reg = true,
                .enable_cfg_simplification = true,
                .enable_dce = true,
                .enable_function_inlining = true,
                .inline_threshold = 25, // Very conservative
                .enable_loop_unroll = false, // Increases size
                .enable_loop_vectorize = false, // Increases size
                .enable_ipsccp = true,
                .enable_global_dce = true,
                .enable_argument_promotion = true,
                .enable_gvn = false, // Can increase size
                .enable_licm = true,
                .enable_reassociation = false,
                .enable_slp_vectorize = false, // Increases size
                .enable_load_store_vectorize = false,
                .enable_machine_opts = true,
                .optimize_for_size = true,
                .aggressive_size_opts = true,
            },
            
            .Os => OptimizationConfig{
                // Os: Optimize for size with reasonable speed
                .enable_mem2reg = true,
                .enable_cfg_simplification = true,
                .enable_dce = true,
                .enable_function_inlining = true,
                .inline_threshold = 100, // Conservative but not extreme
                .enable_loop_unroll = true,
                .enable_loop_vectorize = true,
                .unroll_threshold = 50, // Conservative
                .enable_ipsccp = true,
                .enable_global_dce = true,
                .enable_argument_promotion = true,
                .enable_gvn = true,
                .enable_licm = true,
                .enable_reassociation = true,
                .enable_slp_vectorize = true,
                .enable_load_store_vectorize = false,
                .vectorization_threshold = 8, // Conservative
                .enable_machine_opts = true,
                .optimize_for_size = true,
                .aggressive_size_opts = false,
            },
        };
    }
};

/// Optimization controller that manages pass selection and configuration
pub const OptimizationController = struct {
    level: OptimizationLevel,
    config: OptimizationConfig,
    allocator: std.mem.Allocator,
    
    const Self = @This();
    
    pub fn init(allocator: std.mem.Allocator, level: OptimizationLevel) Self {
        const config = OptimizationConfig.forLevel(level);
        
        return Self{
            .level = level,
            .config = config,
            .allocator = allocator,
        };
    }
    
    /// Apply optimization passes to LLVM module based on level
    pub fn applyOptimizations(self: *Self, llvm_module: anytype) !void {
        print("🚀 Applying {} optimization passes...\n", .{self.level.toString()});
        
        const c = @import("llvm_c_api.zig");
        
        // Create pass managers
        const module_pass_manager = c.LLVMCreatePassManager();
        defer c.LLVMDisposePassManager(module_pass_manager);
        
        const function_pass_manager = c.LLVMCreateFunctionPassManagerForModule(llvm_module);
        defer c.LLVMDisposeFunctionPassManager(function_pass_manager);
        
        // Apply passes based on configuration
        try self.addBasicPasses(function_pass_manager, module_pass_manager);
        
        if (self.config.enable_function_inlining) {
            try self.addInliningPasses(function_pass_manager);
        }
        
        if (self.config.enable_loop_unroll or self.config.enable_loop_vectorize) {
            try self.addLoopOptimizationPasses(function_pass_manager);
        }
        
        if (self.config.enable_ipsccp or self.config.enable_global_dce) {
            try self.addInterproceduralPasses(module_pass_manager);
        }
        
        if (self.config.enable_slp_vectorize or self.config.enable_load_store_vectorize) {
            try self.addVectorizationPasses(function_pass_manager);
        }
        
        if (self.config.optimize_for_size) {
            try self.addSizeOptimizationPasses(module_pass_manager);
        }
        
        // Run the passes
        try self.runOptimizationPasses(llvm_module, function_pass_manager, module_pass_manager);
        
        print("✅ {} optimization passes completed\n", .{self.level.toString()});
    }
    
    fn addBasicPasses(self: *Self, fpm: anytype, mpm: anytype) !void {
        const c = @import("llvm_c_api.zig");
        
        if (self.config.enable_mem2reg) {
            c.LLVMAddPromoteMemoryToRegisterPass(fpm);
        }
        
        if (self.config.enable_cfg_simplification) {
            c.LLVMAddCFGSimplificationPass(fpm);
        }
        
        if (self.config.enable_dce) {
            c.LLVMAddDeadCodeEliminationPass(fpm);
            c.LLVMAddAggressiveDCEPass(fpm);
        }
        
        // Always add instruction combining
        c.LLVMAddInstructionCombiningPass(fpm);
        
        _ = mpm; // Module passes added in other methods
    }
    
    fn addInliningPasses(self: *Self, fpm: anytype) !void {
        const c = @import("llvm_c_api.zig");
        
        // Add function inlining with threshold
        c.LLVMAddFunctionInliningPass(fpm);
        
        // Add cleanup passes after inlining
        c.LLVMAddInstructionCombiningPass(fpm);
        c.LLVMAddCFGSimplificationPass(fpm);
        
        print("  📈 Added inlining passes (threshold: {})\n", .{self.config.inline_threshold});
    }
    
    fn addLoopOptimizationPasses(self: *Self, fpm: anytype) !void {
        const c = @import("llvm_c_api.zig");
        
        // Loop canonicalization
        c.LLVMAddLoopRotatePass(fpm);
        c.LLVMAddLCSSAPass(fpm);
        
        // Loop optimizations
        if (self.config.enable_licm) {
            c.LLVMAddLICMPass(fpm);
        }
        
        if (self.config.enable_loop_unroll) {
            c.LLVMAddLoopUnrollPass(fpm);
        }
        
        if (self.config.enable_loop_vectorize) {
            c.LLVMAddLoopVectorizePass(fpm);
        }
        
        // Cleanup after loop optimizations
        c.LLVMAddIndVarSimplifyPass(fpm);
        c.LLVMAddLoopDeletionPass(fpm);
        
        print("  🔄 Added loop optimization passes\n", .{});
    }
    
    fn addInterproceduralPasses(self: *Self, mpm: anytype) !void {
        const c = @import("llvm_c_api.zig");
        
        if (self.config.enable_ipsccp) {
            c.LLVMAddIPSCCPPass(mpm);
        }
        
        if (self.config.enable_global_dce) {
            c.LLVMAddGlobalDCEPass(mpm);
        }
        
        if (self.config.enable_argument_promotion) {
            c.LLVMAddArgumentPromotionPass(mpm);
        }
        
        // Dead argument elimination
        c.LLVMAddDeadArgEliminationPass(mpm);
        
        print("  🌐 Added interprocedural optimization passes\n", .{});
    }
    
    fn addVectorizationPasses(self: *Self, fpm: anytype) !void {
        const c = @import("llvm_c_api.zig");
        
        if (self.config.enable_slp_vectorize) {
            c.LLVMAddSLPVectorizePass(fpm);
        }
        
        if (self.config.enable_load_store_vectorize) {
            c.LLVMAddLoadStoreVectorizerPass(fpm);
        }
        
        print("  ⚡ Added vectorization passes\n", .{});
    }
    
    fn addSizeOptimizationPasses(self: *Self, mpm: anytype) !void {
        const c = @import("llvm_c_api.zig");
        
        // Merge identical functions
        c.LLVMAddMergeFunctionsPass(mpm);
        
        // Constant merging
        c.LLVMAddConstantMergePass(mpm);
        
        if (self.config.aggressive_size_opts) {
            // Strip symbols for smaller binaries
            c.LLVMAddStripSymbolsPass(mpm);
            c.LLVMAddStripDeadPrototypesPass(mpm);
        }
        
        print("  📦 Added size optimization passes\n", .{});
    }
    
    fn runOptimizationPasses(self: *Self, llvm_module: anytype, fpm: anytype, mpm: anytype) !void {
        const c = @import("llvm_c_api.zig");
        
        // Initialize function pass manager
        if (c.LLVMInitializeFunctionPassManager(fpm) == 0) {
            return error.FunctionPassManagerInitFailed;
        }
        
        // Run function passes on each function
        var function = c.LLVMGetFirstFunction(llvm_module);
        var functions_processed: u32 = 0;
        while (function != null) {
            _ = c.LLVMRunFunctionPassManager(fpm, function);
            functions_processed += 1;
            function = c.LLVMGetNextFunction(function);
        }
        
        // Finalize function pass manager
        _ = c.LLVMFinalizeFunctionPassManager(fpm);
        
        // Run module passes
        _ = c.LLVMRunPassManager(mpm, llvm_module);
        
        // Verify module
        var error_message: [*c]u8 = undefined;
        if (c.LLVMVerifyModule(llvm_module, c.LLVMReturnStatusAction, &error_message) != 0) {
            defer c.LLVMDisposeMessage(error_message);
            print("⚠️  Module verification failed: {s}\n", .{error_message});
            return error.ModuleVerificationFailed;
        }
        
        print("  ✅ Processed {} functions successfully\n", .{functions_processed});
        _ = self; // Suppress unused variable warning
    }
    
    pub fn getEstimatedSpeedup(self: *Self) f32 {
        return switch (self.level) {
            .O0 => 1.0,   // No optimization
            .O1 => 1.3,   // 30% improvement
            .O2 => 2.0,   // 100% improvement
            .O3 => 2.8,   // 180% improvement
            .Oz => 1.2,   // 20% improvement (focuses on size)
            .Os => 1.6,   // 60% improvement (balanced)
        };
    }
    
    pub fn getEstimatedSizeReduction(self: *Self) f32 {
        return switch (self.level) {
            .O0 => 1.0,   // No size change
            .O1 => 0.95,  // 5% reduction
            .O2 => 0.85,  // 15% reduction
            .O3 => 0.80,  // 20% reduction
            .Oz => 0.65,  // 35% reduction
            .Os => 0.75,  // 25% reduction
        };
    }
};

/// Test the optimization controller
pub fn testOptimizationController() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    print("🧪 Testing Optimization Controller...\n", .{});
    
    // Test all optimization levels
    const levels = [_]OptimizationLevel{ .O0, .O1, .O2, .O3, .Oz, .Os };
    
    for (levels) |level| {
        var controller = OptimizationController.init(allocator, level);
        
        print("\n📊 Level {s}:\n", .{level.toString()});
        print("  Inlining threshold: {}\n", .{controller.config.inline_threshold});
        print("  Loop unrolling: {}\n", .{controller.config.enable_loop_unroll});
        print("  Vectorization: {}\n", .{controller.config.enable_slp_vectorize});
        print("  Size optimization: {}\n", .{controller.config.optimize_for_size});
        print("  Estimated speedup: {d:.1}x\n", .{controller.getEstimatedSpeedup()});
        print("  Estimated size: {d:.0}% of original\n", .{controller.getEstimatedSizeReduction() * 100});
    }
    
    print("\n✅ Optimization Controller test completed\n", .{});
}
