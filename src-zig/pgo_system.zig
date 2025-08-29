const std = @import("std");
const builtin = @import("builtin");
const print = std.debug.print;

/// Profile-Guided Optimization System for CURSED Compiler
/// Collects runtime profiling data to guide compiler optimizations
pub const PGOSystem = struct {
    allocator: std.mem.Allocator,
    profile_data_path: []const u8,
    
    // Function call frequency tracking
    function_call_counts: std.HashMap([]const u8, CallProfile, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Branch prediction data
    branch_profiles: std.HashMap(u64, BranchProfile, std.hash_map.DefaultHashContext(u64), std.hash_map.default_max_load_percentage),
    
    // Loop iteration counts
    loop_profiles: std.HashMap(u64, LoopProfile, std.hash_map.DefaultHashContext(u64), std.hash_map.default_max_load_percentage),
    
    // Memory access patterns
    memory_profiles: std.HashMap(u64, MemoryProfile, std.hash_map.DefaultHashContext(u64), std.hash_map.default_max_load_percentage),
    
    // Hot/cold function classification
    hot_functions: std.ArrayList([]const u8),
    cold_functions: std.ArrayList([]const u8),
    
    // Performance counters
    profiling_overhead_ns: u64,
    profile_collection_count: u64,
    
    const Self = @This();
    
    /// Function call profile data
    pub const CallProfile = struct {
        function_name: []const u8,
        call_count: u64,
        total_execution_time_ns: u64,
        average_execution_time_ns: u64,
        min_execution_time_ns: u64,
        max_execution_time_ns: u64,
        last_called_timestamp: i64,
        call_sites: std.ArrayList(CallSite),
        
        pub fn init(allocator: std.mem.Allocator, name: []const u8) CallProfile {
            return CallProfile{
                .function_name = name,
                .call_count = 0,
                .total_execution_time_ns = 0,
                .average_execution_time_ns = 0,
                .min_execution_time_ns = std.math.maxInt(u64),
                .max_execution_time_ns = 0,
                .last_called_timestamp = std.time.timestamp(),
                .call_sites = .{},
            };
        }
        
        pub fn updateExecutionTime(self: *CallProfile, execution_time_ns: u64) void {
            self.call_count += 1;
            self.total_execution_time_ns += execution_time_ns;
            self.average_execution_time_ns = self.total_execution_time_ns / self.call_count;
            self.min_execution_time_ns = @min(self.min_execution_time_ns, execution_time_ns);
            self.max_execution_time_ns = @max(self.max_execution_time_ns, execution_time_ns);
            self.last_called_timestamp = std.time.timestamp();
        }
        
        pub fn isHotFunction(self: *const CallProfile, threshold_calls: u64, threshold_time_ns: u64) bool {
            return self.call_count >= threshold_calls or self.total_execution_time_ns >= threshold_time_ns;
        }
        
        pub fn deinit(self: *CallProfile) void {
            self.call_sites.deinit(self.allocator);
        }
    };
    
    /// Call site information for inlining decisions
    pub const CallSite = struct {
        caller_function: []const u8,
        line_number: u32,
        column_number: u32,
        call_frequency: u64,
        context_hash: u64,
    };
    
    /// Branch prediction profile
    pub const BranchProfile = struct {
        branch_id: u64,
        taken_count: u64,
        not_taken_count: u64,
        total_executions: u64,
        taken_probability: f64,
        misprediction_count: u64,
        
        pub fn init(branch_id: u64) BranchProfile {
            return BranchProfile{
                .branch_id = branch_id,
                .taken_count = 0,
                .not_taken_count = 0,
                .total_executions = 0,
                .taken_probability = 0.5,
                .misprediction_count = 0,
            };
        }
        
        pub fn recordBranch(self: *BranchProfile, taken: bool) void {
            self.total_executions += 1;
            if (taken) {
                self.taken_count += 1;
            } else {
                self.not_taken_count += 1;
            }
            self.taken_probability = @as(f64, @floatFromInt(self.taken_count)) / @as(f64, @floatFromInt(self.total_executions));
        }
        
        pub fn isPredictable(self: *const BranchProfile, threshold: f64) bool {
            return self.taken_probability >= threshold or self.taken_probability <= (1.0 - threshold);
        }
    };
    
    /// Loop execution profile
    pub const LoopProfile = struct {
        loop_id: u64,
        execution_count: u64,
        total_iterations: u64,
        average_iterations: f64,
        min_iterations: u64,
        max_iterations: u64,
        unroll_candidate: bool,
        vectorization_candidate: bool,
        
        pub fn init(loop_id: u64) LoopProfile {
            return LoopProfile{
                .loop_id = loop_id,
                .execution_count = 0,
                .total_iterations = 0,
                .average_iterations = 0.0,
                .min_iterations = std.math.maxInt(u64),
                .max_iterations = 0,
                .unroll_candidate = false,
                .vectorization_candidate = false,
            };
        }
        
        pub fn recordExecution(self: *LoopProfile, iterations: u64) void {
            self.execution_count += 1;
            self.total_iterations += iterations;
            self.average_iterations = @as(f64, @floatFromInt(self.total_iterations)) / @as(f64, @floatFromInt(self.execution_count));
            self.min_iterations = @min(self.min_iterations, iterations);
            self.max_iterations = @max(self.max_iterations, iterations);
            
            // Heuristics for optimization candidates
            self.unroll_candidate = (self.average_iterations >= 4.0 and self.average_iterations <= 16.0);
            self.vectorization_candidate = (self.average_iterations >= 8.0);
        }
    };
    
    /// Memory access pattern profile
    pub const MemoryProfile = struct {
        access_id: u64,
        load_count: u64,
        store_count: u64,
        cache_misses: u64,
        sequential_accesses: u64,
        random_accesses: u64,
        access_stride: i64,
        prefetch_candidate: bool,
        
        pub fn init(access_id: u64) MemoryProfile {
            return MemoryProfile{
                .access_id = access_id,
                .load_count = 0,
                .store_count = 0,
                .cache_misses = 0,
                .sequential_accesses = 0,
                .random_accesses = 0,
                .access_stride = 0,
                .prefetch_candidate = false,
            };
        }
        
        pub fn recordAccess(self: *MemoryProfile, is_load: bool, address: u64, previous_address: u64) void {
            if (is_load) {
                self.load_count += 1;
            } else {
                self.store_count += 1;
            }
            
            // Analyze access pattern
            const stride = @as(i64, @intCast(address)) - @as(i64, @intCast(previous_address));
            if (@abs(stride) <= 64) { // Cache line size heuristic
                self.sequential_accesses += 1;
                self.access_stride = stride;
            } else {
                self.random_accesses += 1;
            }
            
            // Prefetch candidate if sequential accesses dominate
            self.prefetch_candidate = (self.sequential_accesses > self.random_accesses * 3);
        }
    };
    
    /// Initialize PGO system
    pub fn init(allocator: std.mem.Allocator, profile_path: []const u8) !Self {
        var pgo = Self{
            .allocator = allocator,
            .profile_data_path = try allocator.dupe(u8, profile_path),
            .function_call_counts = std.HashMap([]const u8, CallProfile, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .branch_profiles = std.HashMap(u64, BranchProfile, std.hash_map.DefaultHashContext(u64), std.hash_map.default_max_load_percentage).init(allocator),
            .loop_profiles = std.HashMap(u64, LoopProfile, std.hash_map.DefaultHashContext(u64), std.hash_map.default_max_load_percentage).init(allocator),
            .memory_profiles = std.HashMap(u64, MemoryProfile, std.hash_map.DefaultHashContext(u64), std.hash_map.default_max_load_percentage).init(allocator),
            .hot_functions = .{},
            .cold_functions = .{},
            .profiling_overhead_ns = 0,
            .profile_collection_count = 0,
        };
        
        // Try to load existing profile data
        pgo.loadProfileData() catch |err| {
            print("📝 Creating new PGO profile database ({s})\n", .{err});
        };
        
        print("🎯 Profile-Guided Optimization system initialized\n", .{});
        print("  Profile data path: {s}\n", .{profile_path});
        
        return pgo;
    }
    
    /// Cleanup PGO system
    pub fn deinit(self: *Self) void {
        // Save profile data before cleanup
        self.saveProfileData() catch |err| {
            print("⚠️ Warning: Could not save PGO data: {s}\n", .{err});
        };
        
        // Cleanup hashmaps
        var function_iter = self.function_call_counts.iterator();
        while (function_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.function_call_counts.deinit(self.allocator);
        self.branch_profiles.deinit(self.allocator);
        self.loop_profiles.deinit(self.allocator);
        self.memory_profiles.deinit(self.allocator);
        self.hot_functions.deinit(self.allocator);
        self.cold_functions.deinit(self.allocator);
        
        self.allocator.free(self.profile_data_path);
        
        print("💾 PGO profile data saved and system cleaned up\n", .{});
    }
    
    /// Record function call for profiling
    pub fn recordFunctionCall(self: *Self, function_name: []const u8, execution_time_ns: u64, caller: ?[]const u8, line: u32, column: u32) !void {
        const start_time = std.time.nanoTimestamp();
        defer {
            const end_time = std.time.nanoTimestamp();
            self.profiling_overhead_ns += @intCast(end_time - start_time);
        }
        
        // Get or create function profile
        var profile = self.function_call_counts.getPtr(function_name);
        if (profile == null) {
            try self.function_call_counts.put(function_name, CallProfile.init(self.allocator, function_name));
            profile = self.function_call_counts.getPtr(function_name);
        }
        
        if (profile) |p| {
            p.updateExecutionTime(execution_time_ns);
            
            // Record call site if caller provided
            if (caller) |caller_name| {
                const call_site = CallSite{
                    .caller_function = caller_name,
                    .line_number = line,
                    .column_number = column,
                    .call_frequency = 1,
                    .context_hash = std.hash_map.hashString(caller_name) ^ line ^ column,
                };
                try p.call_sites.append(allocator, call_site);
            }
        }
        
        self.profile_collection_count += 1;
    }
    
    /// Record branch execution for prediction optimization
    pub fn recordBranch(self: *Self, branch_id: u64, taken: bool) !void {
        var profile = self.branch_profiles.getPtr(branch_id);
        if (profile == null) {
            try self.branch_profiles.put(branch_id, BranchProfile.init(branch_id));
            profile = self.branch_profiles.getPtr(branch_id);
        }
        
        if (profile) |p| {
            p.recordBranch(taken);
        }
    }
    
    /// Record loop execution for unrolling and vectorization decisions
    pub fn recordLoop(self: *Self, loop_id: u64, iterations: u64) !void {
        var profile = self.loop_profiles.getPtr(loop_id);
        if (profile == null) {
            try self.loop_profiles.put(loop_id, LoopProfile.init(loop_id));
            profile = self.loop_profiles.getPtr(loop_id);
        }
        
        if (profile) |p| {
            p.recordExecution(iterations);
        }
    }
    
    /// Record memory access for prefetching and cache optimization
    pub fn recordMemoryAccess(self: *Self, access_id: u64, is_load: bool, address: u64, previous_address: u64) !void {
        var profile = self.memory_profiles.getPtr(access_id);
        if (profile == null) {
            try self.memory_profiles.put(access_id, MemoryProfile.init(access_id));
            profile = self.memory_profiles.getPtr(access_id);
        }
        
        if (profile) |p| {
            p.recordAccess(is_load, address, previous_address);
        }
    }
    
    /// Analyze collected profile data and generate optimization recommendations
    pub fn analyzeProfiles(self: *Self) !PGOAnalysisResult {
        print("🔍 Analyzing PGO profile data...\n", .{});
        
        var result = PGOAnalysisResult.init(self.allocator);
        
        // Analyze function call patterns
        try self.analyzeFunctionProfiles(&result);
        
        // Analyze branch prediction patterns
        try self.analyzeBranchProfiles(&result);
        
        // Analyze loop patterns
        try self.analyzeLoopProfiles(&result);
        
        // Analyze memory access patterns
        try self.analyzeMemoryProfiles(&result);
        
        // Generate hot/cold function classifications
        try self.classifyFunctions(&result);
        
        print("✅ PGO analysis completed\n", .{});
        result.printSummary();
        
        return result;
    }
    
    /// Analyze function call profiles for inlining decisions
    fn analyzeFunctionProfiles(self: *Self, result: *PGOAnalysisResult) !void {
        var iter = self.function_call_counts.iterator();
        while (iter.next()) |entry| {
            const profile = entry.value_ptr;
            
            // Identify hot functions (called frequently or consume significant time)
            if (profile.isHotFunction(100, 1_000_000_000)) { // 100 calls or 1s total time
                try result.hot_functions.append(allocator, profile.function_name);
                
                // Check for inlining candidates
                if (profile.call_count > 1000 and profile.average_execution_time_ns < 10_000) {
                    const recommendation = OptimizationRecommendation{
                        .type = .function_inlining,
                        .target = profile.function_name,
                        .confidence = 0.9,
                        .expected_improvement = 15.0, // 15% improvement estimate
                        .description = "High-frequency small function, excellent inlining candidate",
                    };
                    try result.recommendations.append(allocator, recommendation);
                }
            } else if (profile.call_count < 10) {
                try result.cold_functions.append(allocator, profile.function_name);
            }
        }
    }
    
    /// Analyze branch profiles for prediction optimization
    fn analyzeBranchProfiles(self: *Self, result: *PGOAnalysisResult) !void {
        var iter = self.branch_profiles.iterator();
        while (iter.next()) |entry| {
            const profile = entry.value_ptr;
            
            // Identify highly predictable branches
            if (profile.isPredictable(0.9) and profile.total_executions > 100) {
                const recommendation = OptimizationRecommendation{
                    .type = .branch_prediction,
                    .target = try std.fmt.allocPrint(self.allocator, "branch_{}", .{profile.branch_id}),
                    .confidence = profile.taken_probability,
                    .expected_improvement = 5.0, // 5% improvement estimate
                    .description = "Highly predictable branch, optimize for common case",
                };
                try result.recommendations.append(self.allocator, recommendation);
            }
        }
    }
    
    /// Analyze loop profiles for unrolling and vectorization
    fn analyzeLoopProfiles(self: *Self, result: *PGOAnalysisResult) !void {
        var iter = self.loop_profiles.iterator();
        while (iter.next()) |entry| {
            const profile = entry.value_ptr;
            
            // Loop unrolling candidates
            if (profile.unroll_candidate and profile.execution_count > 50) {
                const recommendation = OptimizationRecommendation{
                    .type = .loop_unrolling,
                    .target = try std.fmt.allocPrint(self.allocator, "loop_{}", .{profile.loop_id}),
                    .confidence = 0.8,
                    .expected_improvement = 10.0, // 10% improvement estimate
                    .description = "Loop with consistent iteration count, good unrolling candidate",
                };
                try result.recommendations.append(self.allocator, recommendation);
            }
            
            // Vectorization candidates
            if (profile.vectorization_candidate and profile.execution_count > 20) {
                const recommendation = OptimizationRecommendation{
                    .type = .vectorization,
                    .target = try std.fmt.allocPrint(self.allocator, "loop_{}", .{profile.loop_id}),
                    .confidence = 0.7,
                    .expected_improvement = 25.0, // 25% improvement estimate for SIMD
                    .description = "Long-running loop, potential SIMD vectorization target",
                };
                try result.recommendations.append(self.allocator, recommendation);
            }
        }
    }
    
    /// Analyze memory access patterns for prefetching
    fn analyzeMemoryProfiles(self: *Self, result: *PGOAnalysisResult) !void {
        var iter = self.memory_profiles.iterator();
        while (iter.next()) |entry| {
            const profile = entry.value_ptr;
            
            // Prefetching candidates
            if (profile.prefetch_candidate and (profile.load_count + profile.store_count) > 100) {
                const recommendation = OptimizationRecommendation{
                    .type = .memory_prefetching,
                    .target = try std.fmt.allocPrint(self.allocator, "access_{}", .{profile.access_id}),
                    .confidence = 0.75,
                    .expected_improvement = 8.0, // 8% improvement estimate
                    .description = "Sequential memory access pattern, prefetching opportunity",
                };
                try result.recommendations.append(self.allocator, recommendation);
            }
        }
    }
    
    /// Classify functions as hot or cold
    fn classifyFunctions(self: *Self, result: *PGOAnalysisResult) !void {
        // Clear existing classifications
        self.hot_functions.clearRetainingCapacity();
        self.cold_functions.clearRetainingCapacity();
        
        // Copy from analysis results
        for (result.hot_functions.items) |func| {
            try self.hot_functions.append(allocator, func);
        }
        for (result.cold_functions.items) |func| {
            try self.cold_functions.append(allocator, func);
        }
    }
    
    /// Load profile data from file
    fn loadProfileData(self: *Self) !void {
        const file = std.fs.cwd().openFile(self.profile_data_path, .{}) catch |err| {
            return err;
        };
        defer file.close();
        
        print("📂 Loading PGO profile data from {s}\n", .{self.profile_data_path});
        
        // Read binary format: [version:u32][num_functions:u32][function_data...]
        var reader = file.reader();
        
        // Read and validate version
        const version = reader.readIntLittle(u32) catch return error.InvalidProfileFormat;
        if (version != 1) return error.UnsupportedProfileVersion;
        
        // Read function call counts
        const num_functions = reader.readIntLittle(u32) catch return error.InvalidProfileFormat;
        for (0..num_functions) |_| {
            // Read function name length and name
            const name_len = reader.readIntLittle(u32) catch break;
            const name_bytes = self.allocator.alloc(u8, name_len) catch break;
            defer self.allocator.free(name_bytes);
            _ = reader.read(name_bytes) catch break;
            
            // Read call count and frequency data
            const call_count = reader.readIntLittle(u64) catch break;
            const frequency = reader.readIntLittle(f64) catch break;
            
            // Store function profile data
            const name_owned = self.allocator.dupe(u8, name_bytes) catch continue;
            try self.function_profiles.put(name_owned, .{
                .call_count = call_count,
                .frequency = frequency,
                .is_hot = frequency > self.hot_threshold,
            });
            
            // Categorize hot/cold functions
            if (frequency > self.hot_threshold) {
                try self.hot_functions.append(self.allocator, name_owned);
            } else if (frequency < (self.hot_threshold * 0.1)) {
                try self.cold_functions.append(self.allocator, name_owned);
            }
            const call_count = reader.readIntLittle(u64) catch break;
            
            // Store in hash map
            const name_owned = self.allocator.dupe(u8, name_bytes) catch break;
            self.function_call_counts.put(name_owned, call_count) catch {};
        }
        
        print("  ✅ Loaded {s} function profiles\n", .{self.function_call_counts.count()});
    }
    
    /// Save profile data to file
    fn saveProfileData(self: *Self) !void {
        const file = try std.fs.cwd().createFile(self.profile_data_path, .{});
        defer file.close();
        
        print("💾 Saving PGO profile data to {s}\n", .{self.profile_data_path});
        
        var writer = file.writer();
        
        // Write binary format header
        try writer.writeIntLittle(u32, 1); // Version
        try writer.writeIntLittle(u32, @intCast(self.function_call_counts.count()));
        
        // Write function call counts
        var iterator = self.function_call_counts.iterator();
        while (iterator.next()) |entry| {
            const name = entry.key_ptr.*;
            const count = entry.value_ptr.*;
            
            // Write name length and name
            try writer.writeIntLittle(u32, @intCast(name.len));
            try writer.writer().writeAll(name);
            
            // Write call count
            try writer.writeIntLittle(u64, count);
        }
        
        print("  ✅ Saved {s} function profiles\n", .{self.function_call_counts.count()});
        print("  ✅ Saved {s} branch profiles\n", .{self.branch_profiles.count()});
        print("  ✅ Saved {s} loop profiles\n", .{self.loop_profiles.count()});
        print("  ✅ Saved {s} memory profiles\n", .{self.memory_profiles.count()});
    }
    
    /// Generate PGO instrumentation code for a function
    pub fn generateInstrumentation(self: *Self, function_name: []const u8) ![]const u8 {
        // Generate instrumentation code that can be injected into CURSED functions
        const instrumentation = try std.fmt.allocPrint(self.allocator,
            \\// PGO instrumentation for function: {s}
            \\sus __pgo_start_time drip = 0
            \\sus __pgo_function_name tea = "{s}"
            \\// Record function entry
            \\__pgo_start_time = get_timestamp_ns()
            \\// ... original function code ...
            \\// Record function exit
            \\record_function_exit(__pgo_function_name, get_timestamp_ns() - __pgo_start_time)
        , .{ function_name, function_name });
        
        return instrumentation;
    }
    
    /// Print comprehensive PGO statistics
    pub fn printStatistics(self: *const Self) void {
        print("\n🎯 Profile-Guided Optimization Statistics\n", .{});
        print("=========================================\n", .{});
        print("Function profiles: {s}\n", .{self.function_call_counts.count()});
        print("Branch profiles: {s}\n", .{self.branch_profiles.count()});
        print("Loop profiles: {s}\n", .{self.loop_profiles.count()});
        print("Memory access profiles: {s}\n", .{self.memory_profiles.count()});
        print("Hot functions: {s}\n", .{self.hot_functions.items.len});
        print("Cold functions: {s}\n", .{self.cold_functions.items.len});
        print("Profile collections: {s}\n", .{self.profile_collection_count});
        print("Profiling overhead: {:.2} ms\n", .{@as(f64, @floatFromInt(self.profiling_overhead_ns)) / 1_000_000.0});
        
        // Print top hot functions
        if (self.hot_functions.items.len > 0) {
            print("\n🔥 Top Hot Functions:\n", .{});
            for (self.hot_functions.items[0..@min(5, self.hot_functions.items.len)]) |func| {
                if (self.function_call_counts.get(func)) |profile| {
                    print("  {s} calls, {:.2} ms avg - {s}\n", .{
                        profile.call_count,
                        @as(f64, @floatFromInt(profile.average_execution_time_ns)) / 1_000_000.0,
                        func,
                    });
                }
            }
        }
    }
};

/// PGO analysis results with optimization recommendations
pub const PGOAnalysisResult = struct {
    allocator: std.mem.Allocator,
    hot_functions: std.ArrayList([]const u8),
    cold_functions: std.ArrayList([]const u8),
    recommendations: std.ArrayList(OptimizationRecommendation),
    total_analysis_time_ms: u64,
    
    pub fn init(allocator: std.mem.Allocator) PGOAnalysisResult {
        return PGOAnalysisResult{
            .allocator = allocator,
            .hot_functions = .{},
            .cold_functions = .{},
            .recommendations = .{},
            .total_analysis_time_ms = 0,
        };
    }
    
    pub fn deinit(self: *PGOAnalysisResult) void {
        self.hot_functions.deinit(self.allocator);
        self.cold_functions.deinit(self.allocator);
        for (self.recommendations.items) |*rec| {
            self.allocator.free(rec.target);
        }
        self.recommendations.deinit(self.allocator);
    }
    
    pub fn printSummary(self: *const PGOAnalysisResult) void {
        print("\n📊 PGO Analysis Summary:\n", .{});
        print("Hot functions identified: {s}\n", .{self.hot_functions.items.len});
        print("Cold functions identified: {s}\n", .{self.cold_functions.items.len});
        print("Optimization recommendations: {s}\n", .{self.recommendations.items.len});
        print("Analysis time: {s} ms\n", .{self.total_analysis_time_ms});
        
        if (self.recommendations.items.len > 0) {
            print("\n🎯 Top Optimization Recommendations:\n", .{});
            for (self.recommendations.items[0..@min(5, self.recommendations.items.len)]) |rec| {
                print("  {s} - {s} ({:.1}% improvement, {:.0}% confidence)\n", .{
                    rec.type,
                    rec.target,
                    rec.expected_improvement,
                    rec.confidence * 100.0,
                });
            }
        }
    }
};

/// Optimization recommendation generated from PGO analysis
pub const OptimizationRecommendation = struct {
    type: OptimizationType,
    target: []const u8, // Function name, loop ID, etc.
    confidence: f64, // 0.0 to 1.0
    expected_improvement: f64, // Percentage improvement
    description: []const u8,
};

/// Types of optimizations that can be recommended
pub const OptimizationType = enum {
    function_inlining,
    branch_prediction,
    loop_unrolling,
    vectorization,
    memory_prefetching,
    dead_code_elimination,
    constant_propagation,
    
    pub fn format(self: OptimizationType, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
        const name = switch (self) {
            .function_inlining => "Function Inlining",
            .branch_prediction => "Branch Prediction",
            .loop_unrolling => "Loop Unrolling",
            .vectorization => "SIMD Vectorization",
            .memory_prefetching => "Memory Prefetching",
            .dead_code_elimination => "Dead Code Elimination",
            .constant_propagation => "Constant Propagation",
        };
        try writer.print("{s}", .{name});
    }
};

/// Create PGO system with default configuration
pub fn createPGOSystem(allocator: std.mem.Allocator, profile_path: ?[]const u8) !PGOSystem {
    const path = profile_path orelse "cursed_pgo_profile.dat";
    return PGOSystem.init(allocator, path);
}
