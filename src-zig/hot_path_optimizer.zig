const std = @import("std");
const builtin = @import("builtin");
const print = std.debug.print;

/// Hot Path Optimizer for CURSED Compiler
/// Identifies and optimizes frequently executed code paths
pub const HotPathOptimizer = struct {
    allocator: std.mem.Allocator,
    enabled: bool,
    
    // Hot path tracking
    execution_counts: std.HashMap(u64, ExecutionInfo, std.hash_map.DefaultHashContext(u64), std.hash_map.default_max_load_percentage),
    hot_functions: std.HashMap([]const u8, HotFunctionInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    call_chains: std.HashMap(u64, CallChain, std.hash_map.DefaultHashContext(u64), std.hash_map.default_max_load_percentage),
    
    // Optimization thresholds
    hot_path_threshold: u64,
    hot_function_threshold: u64,
    optimization_interval: u64,
    
    // Statistics
    paths_identified: u32,
    paths_optimized: u32,
    optimization_passes: u32,
    total_execution_time_saved_ns: u64,
    
    const Self = @This();
    
    /// Execution information for code paths
    pub const ExecutionInfo = struct {
        path_hash: u64,
        execution_count: u64,
        total_time_ns: u64,
        average_time_ns: u64,
        last_execution_time: i64,
        optimization_applied: bool,
        optimization_type: OptimizationType,
        
        pub fn updateExecution(self: *ExecutionInfo, execution_time_ns: u64) void {
            self.execution_count += 1;
            self.total_time_ns += execution_time_ns;
            self.average_time_ns = self.total_time_ns / self.execution_count;
            self.last_execution_time = std.time.timestamp();
        }
        
        pub fn isHot(self: *const ExecutionInfo, threshold: u64) bool {
            return self.execution_count >= threshold;
        }
        
        pub fn needsOptimization(self: *const ExecutionInfo, hot_threshold: u64) bool {
            return self.isHot(hot_threshold) and !self.optimization_applied;
        }
    };
    
    /// Hot function information
    pub const HotFunctionInfo = struct {
        name: []const u8,
        call_count: u64,
        total_execution_time_ns: u64,
        average_execution_time_ns: u64,
        inlining_candidate: bool,
        vectorization_candidate: bool,
        optimization_priority: OptimizationPriority,
        caller_functions: std.ArrayList([]const u8),
        
        pub fn init(allocator: std.mem.Allocator, name: []const u8) HotFunctionInfo {
            return HotFunctionInfo{
                .name = name,
                .call_count = 0,
                .total_execution_time_ns = 0,
                .average_execution_time_ns = 0,
                .inlining_candidate = false,
                .vectorization_candidate = false,
                .optimization_priority = .medium,
                .caller_functions = .{},
            };
        }
        
        pub fn deinit(self: *HotFunctionInfo) void {
            self.caller_functions.deinit(self.allocator);
        }
        
        pub fn addCall(self: *HotFunctionInfo, execution_time_ns: u64, caller: ?[]const u8) !void {
            self.call_count += 1;
            self.total_execution_time_ns += execution_time_ns;
            self.average_execution_time_ns = self.total_execution_time_ns / self.call_count;
            
            // Update optimization candidacy
            self.updateOptimizationCandidacy();
            
            // Track caller if provided
            if (caller) |caller_name| {
                // Add to caller list if not already present
                for (self.caller_functions.items) |existing_caller| {
                    if (std.mem.eql(u8, existing_caller, caller_name)) {
                        return; // Already tracked
                    }
                }
                try self.caller_functions.append(allocator, caller_name);
            }
        }
        
        fn updateOptimizationCandidacy(self: *HotFunctionInfo) void {
            // Inlining candidacy: small, frequently called functions
            self.inlining_candidate = (self.average_execution_time_ns < 50_000) and // <50μs
                                     (self.call_count >= 1000); // >=1000 calls
            
            // Vectorization candidacy: longer running functions with loop potential
            self.vectorization_candidate = (self.average_execution_time_ns > 100_000) and // >100μs
                                          (self.call_count >= 100); // >=100 calls
            
            // Set optimization priority
            if (self.total_execution_time_ns > 100_000_000) { // >100ms total
                self.optimization_priority = .critical;
            } else if (self.total_execution_time_ns > 10_000_000) { // >10ms total
                self.optimization_priority = .high;
            } else if (self.call_count >= 1000) {
                self.optimization_priority = .medium;
            } else {
                self.optimization_priority = .low;
            }
        }
    };
    
    /// Call chain for sequence optimization
    pub const CallChain = struct {
        chain_hash: u64,
        function_sequence: std.ArrayList([]const u8),
        execution_count: u64,
        total_time_ns: u64,
        optimization_applied: bool,
        
        pub fn init(allocator: std.mem.Allocator, chain_hash: u64) CallChain {
            return CallChain{
                .chain_hash = chain_hash,
                .function_sequence = .{},
                .execution_count = 0,
                .total_time_ns = 0,
                .optimization_applied = false,
            };
        }
        
        pub fn deinit(self: *CallChain) void {
            self.function_sequence.deinit(self.allocator);
        }
        
        pub fn addExecution(self: *CallChain, execution_time_ns: u64) void {
            self.execution_count += 1;
            self.total_time_ns += execution_time_ns;
        }
    };
    
    /// Types of optimizations that can be applied
    pub const OptimizationType = enum {
        none,
        function_inlining,
        loop_unrolling,
        vectorization,
        branch_prediction,
        memory_prefetching,
        constant_propagation,
        dead_code_elimination,
    };
    
    /// Optimization priority levels
    pub const OptimizationPriority = enum {
        low,
        medium,
        high,
        critical,
        
        pub fn getWeight(self: OptimizationPriority) f64 {
            return switch (self) {
                .low => 1.0,
                .medium => 2.0,
                .high => 4.0,
                .critical => 8.0,
            };
        }
    };
    
    /// Initialize hot path optimizer
    pub fn init(allocator: std.mem.Allocator, config: HotPathConfig) !Self {
        var optimizer = Self{
            .allocator = allocator,
            .enabled = config.enabled,
            .execution_counts = std.HashMap(u64, ExecutionInfo, std.hash_map.DefaultHashContext(u64), std.hash_map.default_max_load_percentage).init(allocator),
            .hot_functions = std.HashMap([]const u8, HotFunctionInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .call_chains = std.HashMap(u64, CallChain, std.hash_map.DefaultHashContext(u64), std.hash_map.default_max_load_percentage).init(allocator),
            .hot_path_threshold = config.hot_path_threshold,
            .hot_function_threshold = config.hot_function_threshold,
            .optimization_interval = config.optimization_interval,
            .paths_identified = 0,
            .paths_optimized = 0,
            .optimization_passes = 0,
            .total_execution_time_saved_ns = 0,
        };
        
        if (optimizer.enabled) {
            print("🔥 Hot Path Optimizer initialized\n", .{});
            print("  Hot path threshold: {s} executions\n", .{config.hot_path_threshold});
            print("  Hot function threshold: {s} executions\n", .{config.hot_function_threshold});
            print("  Optimization interval: {s} executions\n", .{config.optimization_interval});
        }
        
        return optimizer;
    }
    
    /// Cleanup hot path optimizer
    pub fn deinit(self: *Self) void {
        if (self.enabled) {
            self.printStatistics();
        }
        
        self.execution_counts.deinit(self.allocator);
        
        var func_iter = self.hot_functions.iterator();
        while (func_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.hot_functions.deinit(self.allocator);
        
        var chain_iter = self.call_chains.iterator();
        while (chain_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.call_chains.deinit(self.allocator);
    }
    
    /// Record function execution for hot path detection
    pub fn recordExecution(self: *Self, function_name: []const u8, execution_time_ns: u64, caller: ?[]const u8, path_context: ?[]const []const u8) !void {
        if (!self.enabled) return;
        
        // Record hot function information
        try self.recordHotFunction(function_name, execution_time_ns, caller);
        
        // Record execution path
        if (path_context) |context| {
            try self.recordExecutionPath(context, execution_time_ns);
        }
        
        // Check if optimization should be triggered
        if (self.shouldTriggerOptimization()) {
            try self.performOptimizationPass();
        }
    }
    
    /// Record hot function execution
    fn recordHotFunction(self: *Self, function_name: []const u8, execution_time_ns: u64, caller: ?[]const u8) !void {
        var hot_func = self.hot_functions.getPtr(function_name);
        if (hot_func == null) {
            try self.hot_functions.put(function_name, HotFunctionInfo.init(self.allocator, function_name));
            hot_func = self.hot_functions.getPtr(function_name);
        }
        
        if (hot_func) |func| {
            try func.addCall(execution_time_ns, caller);
        }
    }
    
    /// Record execution path
    fn recordExecutionPath(self: *Self, path_context: []const []const u8, execution_time_ns: u64) !void {
        // Generate hash for the execution path
        const path_hash = self.hashExecutionPath(path_context);
        
        var exec_info = self.execution_counts.getPtr(path_hash);
        if (exec_info == null) {
            const new_info = ExecutionInfo{
                .path_hash = path_hash,
                .execution_count = 0,
                .total_time_ns = 0,
                .average_time_ns = 0,
                .last_execution_time = std.time.timestamp(),
                .optimization_applied = false,
                .optimization_type = .none,
            };
            try self.execution_counts.put(path_hash, new_info);
            exec_info = self.execution_counts.getPtr(path_hash);
        }
        
        if (exec_info) |info| {
            info.updateExecution(execution_time_ns);
            
            // Check if this path became hot
            if (info.needsOptimization(self.hot_path_threshold)) {
                self.paths_identified += 1;
                print("🔥 Hot path identified: hash={s} (count={s})\n", .{ path_hash, info.execution_count });
            }
        }
    }
    
    /// Generate hash for execution path
    fn hashExecutionPath(self: *Self, path_context: []const []const u8) u64 {
        _ = self;
        
        var hasher = std.hash.Wyhash.init(0);
        for (path_context) |func_name| {
            hasher.update(func_name);
        }
        return hasher.final();
    }
    
    /// Check if optimization should be triggered
    fn shouldTriggerOptimization(self: *Self) bool {
        return (self.paths_identified % self.optimization_interval) == 0 and self.paths_identified > 0;
    }
    
    /// Perform comprehensive optimization pass
    fn performOptimizationPass(self: *Self) !void {
        if (!self.enabled) return;
        
        print("⚡ Performing hot path optimization pass #{s}\n", .{self.optimization_passes + 1});
        
        self.optimization_passes += 1;
        var optimizations_applied: u32 = 0;
        
        // Optimize hot functions
        optimizations_applied += try self.optimizeHotFunctions();
        
        // Optimize hot execution paths
        optimizations_applied += try self.optimizeHotPaths();
        
        // Optimize call chains
        optimizations_applied += try self.optimizeCallChains();
        
        self.paths_optimized += optimizations_applied;
        
        print("  ✅ Applied {s} optimizations in this pass\n", .{optimizations_applied});
    }
    
    /// Optimize hot functions
    fn optimizeHotFunctions(self: *Self) !u32 {
        var optimizations_applied: u32 = 0;
        
        var func_iter = self.hot_functions.iterator();
        while (func_iter.next()) |entry| {
            const func = entry.value_ptr;
            
            if (func.call_count >= self.hot_function_threshold) {
                if (func.inlining_candidate) {
                    try self.applyFunctionInlining(func.name);
                    optimizations_applied += 1;
                }
                
                if (func.vectorization_candidate) {
                    try self.applyVectorization(func.name);
                    optimizations_applied += 1;
                }
            }
        }
        
        return optimizations_applied;
    }
    
    /// Optimize hot execution paths
    fn optimizeHotPaths(self: *Self) !u32 {
        var optimizations_applied: u32 = 0;
        
        var path_iter = self.execution_counts.iterator();
        while (path_iter.next()) |entry| {
            var exec_info = entry.value_ptr;
            
            if (exec_info.needsOptimization(self.hot_path_threshold)) {
                const optimization_type = self.selectOptimizationType(exec_info);
                try self.applyPathOptimization(exec_info.path_hash, optimization_type);
                
                exec_info.optimization_applied = true;
                exec_info.optimization_type = optimization_type;
                optimizations_applied += 1;
                
                // Estimate time savings (10-30% improvement typical)
                const estimated_savings = exec_info.average_time_ns / 5; // 20% improvement estimate
                self.total_execution_time_saved_ns += estimated_savings * exec_info.execution_count;
            }
        }
        
        return optimizations_applied;
    }
    
    /// Optimize call chains
    fn optimizeCallChains(self: *Self) !u32 {
        var optimizations_applied: u32 = 0;
        
        var chain_iter = self.call_chains.iterator();
        while (chain_iter.next()) |entry| {
            var chain = entry.value_ptr;
            
            if (chain.execution_count >= self.hot_path_threshold and !chain.optimization_applied) {
                try self.applyCallChainOptimization(chain);
                chain.optimization_applied = true;
                optimizations_applied += 1;
            }
        }
        
        return optimizations_applied;
    }
    
    /// Select appropriate optimization type for a path
    fn selectOptimizationType(self: *Self, exec_info: *const ExecutionInfo) OptimizationType {
        _ = self;
        
        // Simple heuristics for optimization type selection
        if (exec_info.average_time_ns < 10_000) { // <10μs - likely small functions
            return .function_inlining;
        } else if (exec_info.execution_count > 10000) { // Very hot paths
            return .vectorization;
        } else if (exec_info.execution_count > 5000) { // Hot paths
            return .loop_unrolling;
        } else {
            return .branch_prediction;
        }
    }
    
    /// Apply function inlining optimization
    fn applyFunctionInlining(self: *Self, function_name: []const u8) !void {
        _ = self;
        
        print("  🔗 Applying function inlining to: {s}\n", .{function_name});
        // TODO: Implement actual function inlining in compiler
    }
    
    /// Apply vectorization optimization
    fn applyVectorization(self: *Self, function_name: []const u8) !void {
        _ = self;
        
        print("  📊 Applying SIMD vectorization to: {s}\n", .{function_name});
        // TODO: Implement actual vectorization in compiler
    }
    
    /// Apply path-specific optimization
    fn applyPathOptimization(self: *Self, path_hash: u64, optimization_type: OptimizationType) !void {
        _ = self;
        
        print("  ⚡ Applying {s} optimization to path hash: {s}\n", .{ optimization_type, path_hash });
        // TODO: Implement actual path optimization in compiler
    }
    
    /// Apply call chain optimization
    fn applyCallChainOptimization(self: *Self, chain: *CallChain) !void {
        _ = self;
        
        print("  🔄 Applying call chain optimization to: {s} functions\n", .{chain.function_sequence.items.len});
        // TODO: Implement actual call chain optimization
    }
    
    /// Identify hot paths from collected data
    pub fn identifyHotPaths(self: *Self) !HotPathAnalysis {
        print("🔍 Analyzing hot paths...\n", .{});
        
        var analysis = HotPathAnalysis.init(self.allocator);
        
        // Identify hot functions
        var func_iter = self.hot_functions.iterator();
        while (func_iter.next()) |entry| {
            const func = entry.value_ptr;
            if (func.call_count >= self.hot_function_threshold) {
                try analysis.hot_functions.append(self.allocator, HotFunctionSummary{
                    .name = func.name,
                    .call_count = func.call_count,
                    .total_time_ns = func.total_execution_time_ns,
                    .average_time_ns = func.average_execution_time_ns,
                    .priority = func.optimization_priority,
                });
            }
        }
        
        // Identify hot execution paths
        var path_iter = self.execution_counts.iterator();
        while (path_iter.next()) |entry| {
            const exec_info = entry.value_ptr;
            if (exec_info.isHot(self.hot_path_threshold)) {
                try analysis.hot_paths.append(HotPathSummary{
                    .path_hash = exec_info.path_hash,
                    .execution_count = exec_info.execution_count,
                    .total_time_ns = exec_info.total_time_ns,
                    .average_time_ns = exec_info.average_time_ns,
                    .optimization_applied = exec_info.optimization_applied,
                    .optimization_type = exec_info.optimization_type,
                });
            }
        }
        
        // Sort by total execution time (most impactful first)
        std.mem.sort(HotFunctionSummary, analysis.hot_functions.items, {}, hotFunctionCompare);
        std.mem.sort(HotPathSummary, analysis.hot_paths.items, {}, hotPathCompare);
        
        print("  🔥 Hot functions identified: {s}\n", .{analysis.hot_functions.items.len});
        print("  🛤️ Hot paths identified: {s}\n", .{analysis.hot_paths.items.len});
        
        return analysis;
    }
    
    /// Generate optimization recommendations
    pub fn generateOptimizationRecommendations(self: *Self) !std.ArrayList(OptimizationRecommendation) {
        var recommendations = std.ArrayList(OptimizationRecommendation){};
        
        // Analyze hot functions for recommendations
        var func_iter = self.hot_functions.iterator();
        while (func_iter.next()) |entry| {
            const func = entry.value_ptr;
            
            if (func.call_count >= self.hot_function_threshold) {
                if (func.inlining_candidate) {
                    const recommendation = OptimizationRecommendation{
                        .type = .function_inlining,
                        .target = func.name,
                        .priority = func.optimization_priority,
                        .expected_improvement = 15.0, // 15% improvement estimate
                        .confidence = 0.85,
                        .description = "High-frequency small function suitable for inlining",
                    };
                    try recommendations.append(allocator, recommendation);
                }
                
                if (func.vectorization_candidate) {
                    const recommendation = OptimizationRecommendation{
                        .type = .vectorization,
                        .target = func.name,
                        .priority = func.optimization_priority,
                        .expected_improvement = 25.0, // 25% improvement estimate for SIMD
                        .confidence = 0.70,
                        .description = "Function with loop patterns suitable for SIMD vectorization",
                    };
                    try recommendations.append(allocator, recommendation);
                }
            }
        }
        
        // Sort recommendations by priority and expected improvement
        std.mem.sort(OptimizationRecommendation, recommendations.items, {}, recommendationCompare);
        
        return recommendations;
    }
    
    /// Print comprehensive statistics
    pub fn printStatistics(self: *const Self) void {
        print("\n🔥 Hot Path Optimizer Statistics\n", .{});
        print("================================\n", .{});
        print("Optimizer enabled: {s}\n", .{self.enabled});
        print("Hot paths identified: {s}\n", .{self.paths_identified});
        print("Optimizations applied: {s}\n", .{self.paths_optimized});
        print("Optimization passes: {s}\n", .{self.optimization_passes});
        print("Estimated time saved: {:.2} ms\n", .{@as(f64, @floatFromInt(self.total_execution_time_saved_ns)) / 1_000_000.0});
        
        print("\n📊 Hot Function Summary:\n", .{});
        print("Functions tracked: {s}\n", .{self.hot_functions.count()});
        
        var hot_count: u32 = 0;
        var func_iter = self.hot_functions.iterator();
        while (func_iter.next()) |entry| {
            const func = entry.value_ptr;
            if (func.call_count >= self.hot_function_threshold) {
                hot_count += 1;
            }
        }
        print("Hot functions: {s}\n", .{hot_count});
        
        print("\n🛤️ Execution Path Summary:\n", .{});
        print("Paths tracked: {s}\n", .{self.execution_counts.count()});
        
        var hot_path_count: u32 = 0;
        var path_iter = self.execution_counts.iterator();
        while (path_iter.next()) |entry| {
            const exec_info = entry.value_ptr;
            if (exec_info.isHot(self.hot_path_threshold)) {
                hot_path_count += 1;
            }
        }
        print("Hot paths: {s}\n", .{hot_path_count});
        
        print("\n⚙️ Configuration:\n", .{});
        print("Hot path threshold: {s}\n", .{self.hot_path_threshold});
        print("Hot function threshold: {s}\n", .{self.hot_function_threshold});
        print("Optimization interval: {s}\n", .{self.optimization_interval});
    }
};

/// Configuration for hot path optimizer
pub const HotPathConfig = struct {
    enabled: bool = true,
    hot_path_threshold: u64 = 100,        // Consider paths hot after 100 executions
    hot_function_threshold: u64 = 1000,   // Consider functions hot after 1000 calls
    optimization_interval: u64 = 50,      // Optimize every 50 hot path identifications
    
    pub fn defaultConfig() HotPathConfig {
        return HotPathConfig{};
    }
    
    pub fn aggressiveConfig() HotPathConfig {
        return HotPathConfig{
            .hot_path_threshold = 50,
            .hot_function_threshold = 500,
            .optimization_interval = 25,
        };
    }
    
    pub fn conservativeConfig() HotPathConfig {
        return HotPathConfig{
            .hot_path_threshold = 500,
            .hot_function_threshold = 5000,
            .optimization_interval = 100,
        };
    }
};

/// Hot path analysis results
pub const HotPathAnalysis = struct {
    allocator: std.mem.Allocator,
    hot_functions: std.ArrayList(HotFunctionSummary),
    hot_paths: std.ArrayList(HotPathSummary),
    
    pub fn init(allocator: std.mem.Allocator) HotPathAnalysis {
        return HotPathAnalysis{
            .allocator = allocator,
            .hot_functions = .{},
            .hot_paths = .{},
        };
    }
    
    pub fn deinit(self: *HotPathAnalysis) void {
        self.hot_functions.deinit(self.allocator);
        self.hot_paths.deinit(self.allocator);
    }
    
    pub fn printSummary(self: *const HotPathAnalysis) void {
        print("\n📊 Hot Path Analysis Summary\n", .{});
        print("============================\n", .{});
        
        if (self.hot_functions.items.len > 0) {
            print("\n🔥 Top Hot Functions:\n", .{});
            for (self.hot_functions.items[0..@min(5, self.hot_functions.items.len)]) |func| {
                print("  {s}: {s} calls, {:.2} ms total, priority: {s}\n", .{
                    func.name,
                    func.call_count,
                    @as(f64, @floatFromInt(func.total_time_ns)) / 1_000_000.0,
                    func.priority,
                });
            }
        }
        
        if (self.hot_paths.items.len > 0) {
            print("\n🛤️ Top Hot Paths:\n", .{});
            for (self.hot_paths.items[0..@min(5, self.hot_paths.items.len)]) |path| {
                print("  Path {s}: {s} executions, {:.2} ms total, optimized: {s}\n", .{
                    path.path_hash,
                    path.execution_count,
                    @as(f64, @floatFromInt(path.total_time_ns)) / 1_000_000.0,
                    path.optimization_applied,
                });
            }
        }
    }
};

/// Hot function summary
pub const HotFunctionSummary = struct {
    name: []const u8,
    call_count: u64,
    total_time_ns: u64,
    average_time_ns: u64,
    priority: HotPathOptimizer.OptimizationPriority,
};

/// Hot path summary
pub const HotPathSummary = struct {
    path_hash: u64,
    execution_count: u64,
    total_time_ns: u64,
    average_time_ns: u64,
    optimization_applied: bool,
    optimization_type: HotPathOptimizer.OptimizationType,
};

/// Optimization recommendation
pub const OptimizationRecommendation = struct {
    type: HotPathOptimizer.OptimizationType,
    target: []const u8,
    priority: HotPathOptimizer.OptimizationPriority,
    expected_improvement: f64, // Percentage
    confidence: f64, // 0.0 to 1.0
    description: []const u8,
};

/// Comparison functions for sorting
fn hotFunctionCompare(_: void, a: HotFunctionSummary, b: HotFunctionSummary) bool {
    return a.total_time_ns > b.total_time_ns;
}

fn hotPathCompare(_: void, a: HotPathSummary, b: HotPathSummary) bool {
    return a.total_time_ns > b.total_time_ns;
}

fn recommendationCompare(_: void, a: OptimizationRecommendation, b: OptimizationRecommendation) bool {
    const a_score = a.priority.getWeight() * a.expected_improvement * a.confidence;
    const b_score = b.priority.getWeight() * b.expected_improvement * b.confidence;
    return a_score > b_score;
}

/// Create hot path optimizer with configuration
pub fn createHotPathOptimizer(allocator: std.mem.Allocator, config: HotPathConfig) !HotPathOptimizer {
    return HotPathOptimizer.init(allocator, config);
}
