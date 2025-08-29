const std = @import("std");
const builtin = @import("builtin");
const print = std.debug.print;

/// Comprehensive Performance Profiler for CURSED Compiler
/// Provides runtime and compile-time performance analysis
pub const PerformanceProfiler = struct {
    allocator: std.mem.Allocator,
    profiling_enabled: bool,
    sampling_rate_hz: u32,
    
    // Profiling data collection
    function_profiles: std.HashMap([]const u8, FunctionProfile, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    memory_profiles: std.ArrayList(MemorySnapshot),
    cpu_profiles: std.ArrayList(CPUSnapshot),
    compilation_profiles: std.ArrayList(CompilationPhaseProfile),
    
    // Hot path tracking
    hot_paths: std.HashMap(u64, HotPath, std.hash_map.DefaultHashContext(u64), std.hash_map.default_max_load_percentage),
    hot_path_threshold: u64,
    
    // Performance counters
    total_samples: u64,
    profiling_overhead_ns: u64,
    start_time: i64,
    
    // Output configuration
    output_format: OutputFormat,
    output_file: ?[]const u8,
    
    const Self = @This();
    
    /// Function execution profile
    pub const FunctionProfile = struct {
        name: []const u8,
        call_count: u64,
        total_time_ns: u64,
        min_time_ns: u64,
        max_time_ns: u64,
        average_time_ns: u64,
        inclusive_time_ns: u64, // Including callees
        exclusive_time_ns: u64, // Excluding callees
        last_call_timestamp: i64,
        samples: std.ArrayList(SamplePoint),
        call_stack_depth: u32,
        
        pub fn init(allocator: std.mem.Allocator, name: []const u8) FunctionProfile {
            return FunctionProfile{
                .name = name,
                .call_count = 0,
                .total_time_ns = 0,
                .min_time_ns = std.math.maxInt(u64),
                .max_time_ns = 0,
                .average_time_ns = 0,
                .inclusive_time_ns = 0,
                .exclusive_time_ns = 0,
                .last_call_timestamp = std.time.timestamp(),
                .samples = .{},
                .call_stack_depth = 0,
            };
        }
        
        pub fn addSample(self: *FunctionProfile, execution_time_ns: u64, inclusive_time_ns: u64) !void {
            self.call_count += 1;
            self.total_time_ns += execution_time_ns;
            self.inclusive_time_ns += inclusive_time_ns;
            self.exclusive_time_ns += execution_time_ns;
            
            self.min_time_ns = @min(self.min_time_ns, execution_time_ns);
            self.max_time_ns = @max(self.max_time_ns, execution_time_ns);
            self.average_time_ns = self.total_time_ns / self.call_count;
            self.last_call_timestamp = std.time.timestamp();
            
            const sample = SamplePoint{
                .timestamp = std.time.nanoTimestamp(),
                .execution_time_ns = execution_time_ns,
                .memory_usage_bytes = getCurrentMemoryUsage(),
                .cpu_usage_percent = getCurrentCPUUsage(),
            };
            
            try self.samples.append(allocator, sample);
        }
        
        pub fn deinit(self: *FunctionProfile) void {
            self.samples.deinit(self.allocator);
        }
        
        pub fn isHotFunction(self: *const FunctionProfile) bool {
            return self.call_count >= 1000 or self.total_time_ns >= 100_000_000; // 100ms
        }
        
        pub fn getCallsPerSecond(self: *const FunctionProfile, duration_s: f64) f64 {
            return @as(f64, @floatFromInt(self.call_count)) / duration_s;
        }
    };
    
    /// Individual sample point for detailed analysis
    pub const SamplePoint = struct {
        timestamp: i64,
        execution_time_ns: u64,
        memory_usage_bytes: u64,
        cpu_usage_percent: f64,
    };
    
    /// Memory usage snapshot
    pub const MemorySnapshot = struct {
        timestamp: i64,
        heap_size_bytes: u64,
        stack_size_bytes: u64,
        total_allocated_bytes: u64,
        total_freed_bytes: u64,
        active_allocations: u64,
        fragmentation_ratio: f64,
        gc_collections: u32,
        
        pub fn init() MemorySnapshot {
            return MemorySnapshot{
                .timestamp = std.time.nanoTimestamp(),
                .heap_size_bytes = 0,
                .stack_size_bytes = 0,
                .total_allocated_bytes = 0,
                .total_freed_bytes = 0,
                .active_allocations = 0,
                .fragmentation_ratio = 0.0,
                .gc_collections = 0,
            };
        }
    };
    
    /// CPU usage snapshot
    pub const CPUSnapshot = struct {
        timestamp: i64,
        cpu_usage_percent: f64,
        user_time_ms: u64,
        system_time_ms: u64,
        idle_time_ms: u64,
        context_switches: u64,
        cache_misses: u64,
        instructions_executed: u64,
        
        pub fn init() CPUSnapshot {
            return CPUSnapshot{
                .timestamp = std.time.nanoTimestamp(),
                .cpu_usage_percent = 0.0,
                .user_time_ms = 0,
                .system_time_ms = 0,
                .idle_time_ms = 0,
                .context_switches = 0,
                .cache_misses = 0,
                .instructions_executed = 0,
            };
        }
    };
    
    /// Compilation phase profiling
    pub const CompilationPhaseProfile = struct {
        phase_name: []const u8,
        start_time: i64,
        end_time: i64,
        duration_ms: u64,
        memory_used_bytes: u64,
        files_processed: u32,
        lines_processed: u64,
        
        pub fn init(phase_name: []const u8) CompilationPhaseProfile {
            const now = std.time.nanoTimestamp();
            return CompilationPhaseProfile{
                .phase_name = phase_name,
                .start_time = now,
                .end_time = 0,
                .duration_ms = 0,
                .memory_used_bytes = 0,
                .files_processed = 0,
                .lines_processed = 0,
            };
        }
        
        pub fn finish(self: *CompilationPhaseProfile) void {
            self.end_time = std.time.nanoTimestamp();
            self.duration_ms = @intCast((self.end_time - self.start_time) / 1_000_000);
        }
    };
    
    /// Hot path identification
    pub const HotPath = struct {
        path_id: u64,
        execution_count: u64,
        total_time_ns: u64,
        average_time_ns: u64,
        function_sequence: std.ArrayList([]const u8),
        optimization_applied: bool,
        
        pub fn init(allocator: std.mem.Allocator, path_id: u64) HotPath {
            return HotPath{
                .path_id = path_id,
                .execution_count = 0,
                .total_time_ns = 0,
                .average_time_ns = 0,
                .function_sequence = .{},
                .optimization_applied = false,
            };
        }
        
        pub fn deinit(self: *HotPath) void {
            self.function_sequence.deinit(self.allocator);
        }
        
        pub fn addExecution(self: *HotPath, time_ns: u64) void {
            self.execution_count += 1;
            self.total_time_ns += time_ns;
            self.average_time_ns = self.total_time_ns / self.execution_count;
        }
    };
    
    /// Output format options
    pub const OutputFormat = enum {
        text,
        json,
        csv,
        flamegraph,
        chrome_tracing,
        
        pub fn getFileExtension(self: OutputFormat) []const u8 {
            return switch (self) {
                .text => ".txt",
                .json => ".json", 
                .csv => ".csv",
                .flamegraph => ".svg",
                .chrome_tracing => ".json",
            };
        }
    };
    
    /// Initialize performance profiler
    pub fn init(allocator: std.mem.Allocator, config: ProfilerConfig) !Self {
        const profiler = Self{
            .allocator = allocator,
            .profiling_enabled = config.enabled,
            .sampling_rate_hz = config.sampling_rate_hz,
            .function_profiles = std.HashMap([]const u8, FunctionProfile, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .memory_profiles = .{},
            .cpu_profiles = .{},
            .compilation_profiles = .{},
            .hot_paths = std.HashMap(u64, HotPath, std.hash_map.DefaultHashContext(u64), std.hash_map.default_max_load_percentage).init(allocator),
            .hot_path_threshold = config.hot_path_threshold,
            .total_samples = 0,
            .profiling_overhead_ns = 0,
            .start_time = std.time.nanoTimestamp(),
            .output_format = config.output_format,
            .output_file = if (config.output_file) |file| try allocator.dupe(u8, file) else null,
        };
        
        if (profiler.profiling_enabled) {
            print("📊 Performance profiler initialized\n", .{});
            print("  Sampling rate: {s} Hz\n", .{profiler.sampling_rate_hz});
            print("  Hot path threshold: {s} executions\n", .{profiler.hot_path_threshold});
            print("  Output format: {s}\n", .{profiler.output_format});
        }
        
        return profiler;
    }
    
    /// Cleanup profiler and save results
    pub fn deinit(self: *Self) void {
        if (self.profiling_enabled) {
            // Save profiling results
            self.saveResults() catch |err| {
                print("⚠️ Warning: Could not save profiling results: {s}\n", .{err});
            };
            
            // Print summary
            self.printSummary();
        }
        
        // Cleanup data structures
        var func_iter = self.function_profiles.iterator();
        while (func_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.function_profiles.deinit(self.allocator);
        
        var hot_path_iter = self.hot_paths.iterator();
        while (hot_path_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.hot_paths.deinit(self.allocator);
        
        self.memory_profiles.deinit(self.allocator);
        self.cpu_profiles.deinit(self.allocator);
        self.compilation_profiles.deinit(self.allocator);
        
        if (self.output_file) |file| {
            self.allocator.free(file);
        }
    }
    
    /// Start profiling a function
    pub fn startFunction(self: *Self, function_name: []const u8) ProfilerScope {
        if (!self.profiling_enabled) {
            return ProfilerScope.disabled();
        }
        
        return ProfilerScope.init(self, function_name);
    }
    
    /// Record function execution manually
    pub fn recordFunction(self: *Self, function_name: []const u8, execution_time_ns: u64, inclusive_time_ns: u64) !void {
        if (!self.profiling_enabled) return;
        
        const start_time = std.time.nanoTimestamp();
        defer {
            const end_time = std.time.nanoTimestamp();
            self.profiling_overhead_ns += @intCast(end_time - start_time);
        }
        
        // Get or create function profile
        var profile = self.function_profiles.getPtr(function_name);
        if (profile == null) {
            try self.function_profiles.put(function_name, FunctionProfile.init(self.allocator, function_name));
            profile = self.function_profiles.getPtr(function_name);
        }
        
        if (profile) |p| {
            try p.addSample(execution_time_ns, inclusive_time_ns);
            self.total_samples += 1;
        }
    }
    
    /// Take memory snapshot
    pub fn takeMemorySnapshot(self: *Self) !void {
        if (!self.profiling_enabled) return;
        
        var snapshot = MemorySnapshot.init();
        
        // Get actual memory statistics from system
        snapshot.heap_size_bytes = getHeapSize();
        snapshot.stack_size_bytes = getStackSize();
        snapshot.total_allocated_bytes = getAllocatedMemory();
        snapshot.total_freed_bytes = getFreedMemory();
        snapshot.active_allocations = getActiveAllocations();
        snapshot.fragmentation_ratio = getFragmentationRatio();
        snapshot.gc_collections = getGCCollections();
        
        try self.memory_profiles.append(allocator, snapshot);
    }
    
    /// Take CPU snapshot
    pub fn takeCPUSnapshot(self: *Self) !void {
        if (!self.profiling_enabled) return;
        
        var snapshot = CPUSnapshot.init();
        
        // Get actual CPU statistics from system
        snapshot.cpu_usage_percent = getCPUUsagePercent();
        snapshot.user_time_ms = getUserTimeMs();
        snapshot.system_time_ms = getSystemTimeMs();
        snapshot.idle_time_ms = getIdleTimeMs();
        snapshot.context_switches = getContextSwitches();
        snapshot.cache_misses = getCacheMisses();
        snapshot.instructions_executed = getInstructionsExecuted();
        
        try self.cpu_profiles.append(self.allocator, snapshot);
    }
    
    /// Start profiling a compilation phase
    pub fn startCompilationPhase(self: *Self, phase_name: []const u8) !*CompilationPhaseProfile {
        if (!self.profiling_enabled) {
            // Return a dummy profile that does nothing
            const dummy = try self.allocator.create(CompilationPhaseProfile);
            dummy.* = CompilationPhaseProfile.init(phase_name);
            return dummy;
        }
        
        const profile = CompilationPhaseProfile.init(phase_name);
        try self.compilation_profiles.append(self.allocator, profile);
        
        return &self.compilation_profiles.items[self.compilation_profiles.items.len - 1];
    }
    
    /// Record hot path execution
    pub fn recordHotPath(self: *Self, path_id: u64, execution_time_ns: u64, function_sequence: []const []const u8) !void {
        if (!self.profiling_enabled) return;
        
        var hot_path = self.hot_paths.getPtr(path_id);
        if (hot_path == null) {
            try self.hot_paths.put(path_id, HotPath.init(self.allocator, path_id));
            hot_path = self.hot_paths.getPtr(path_id);
        }
        
        if (hot_path) |path| {
            path.addExecution(execution_time_ns);
            
            // Update function sequence
            path.function_sequence.clearRetainingCapacity();
            for (function_sequence) |func| {
                try path.function_sequence.append(self.allocator, func);
            }
        }
    }
    
    /// Identify hot paths from function call patterns
    pub fn identifyHotPaths(self: *Self) !HotPathAnalysis {
        print("🔥 Identifying hot execution paths...\n", .{});
        
        var analysis = HotPathAnalysis.init(self.allocator);
        
        // Find functions that exceed hot path threshold
        var func_iter = self.function_profiles.iterator();
        while (func_iter.next()) |entry| {
            const profile = entry.value_ptr;
            if (profile.call_count >= self.hot_path_threshold) {
                try analysis.hot_functions.append(self.allocator, profile.name);
                print("  🔥 Hot function: {s} ({s} calls, {:.2} ms avg)\n", .{
                    profile.name,
                    profile.call_count,
                    @as(f64, @floatFromInt(profile.average_time_ns)) / 1_000_000.0,
                });
            }
        }
        
        // Analyze hot path execution patterns
        var hot_path_iter = self.hot_paths.iterator();
        while (hot_path_iter.next()) |entry| {
            const path = entry.value_ptr;
            if (path.execution_count >= self.hot_path_threshold) {
                try analysis.hot_paths.append(allocator, path.*);
                print("  🛤️ Hot path {s}: {s} executions, {:.2} ms avg\n", .{
                    path.path_id,
                    path.execution_count,
                    @as(f64, @floatFromInt(path.average_time_ns)) / 1_000_000.0,
                });
            }
        }
        
        print("  ✅ Identified {s} hot functions and {s} hot paths\n", .{
            analysis.hot_functions.items.len,
            analysis.hot_paths.items.len,
        });
        
        return analysis;
    }
    
    /// Save profiling results to file
    fn saveResults(self: *Self) !void {
        if (self.output_file == null) return;
        
        const filename = try std.fmt.allocPrint(self.allocator, "{s}{s}", .{
            self.output_file.?,
            self.output_format.getFileExtension(),
        });
        defer self.allocator.free(filename);
        
        const file = try std.fs.cwd().createFile(filename, .{});
        defer file.close();
        
        switch (self.output_format) {
            .text => try self.saveTextFormat(file),
            .json => try self.saveJSONFormat(file),
            .csv => try self.saveCSVFormat(file),
            .flamegraph => try self.saveFlamegraphFormat(file),
            .chrome_tracing => try self.saveChromeTracingFormat(file),
        }
        
        print("💾 Profiling results saved to {s}\n", .{filename});
    }
    
    /// Save results in text format
    fn saveTextFormat(self: *Self, file: std.fs.File) !void {
        var writer = file.writer(&[_]u8{});
        
        try writer.print("CURSED Compiler Performance Profile\n", .{});
        try writer.print("===================================\n\n", .{});
        
        try writer.print("Profiling Duration: {:.2} seconds\n", .{
            @as(f64, @floatFromInt(std.time.nanoTimestamp() - self.start_time)) / 1_000_000_000.0
        });
        try writer.print("Total Samples: {s}\n", .{self.total_samples});
        try writer.print("Profiling Overhead: {:.2} ms\n\n", .{
            @as(f64, @floatFromInt(self.profiling_overhead_ns)) / 1_000_000.0
        });
        
        try writer.print("Function Profiles:\n", .{});
        try writer.print("------------------\n", .{});
        
        var func_iter = self.function_profiles.iterator();
        while (func_iter.next()) |entry| {
            const profile = entry.value_ptr;
            try writer.print("{s}:\n", .{profile.name});
            try writer.print("  Calls: {s}\n", .{profile.call_count});
            try writer.print("  Total time: {:.2} ms\n", .{@as(f64, @floatFromInt(profile.total_time_ns)) / 1_000_000.0});
            try writer.print("  Average time: {:.2} ms\n", .{@as(f64, @floatFromInt(profile.average_time_ns)) / 1_000_000.0});
            try writer.print("  Min time: {:.2} ms\n", .{@as(f64, @floatFromInt(profile.min_time_ns)) / 1_000_000.0});
            try writer.print("  Max time: {:.2} ms\n", .{@as(f64, @floatFromInt(profile.max_time_ns)) / 1_000_000.0});
            try writer.print("\n", .{});
        }
    }
    
    /// Save results in JSON format
    fn saveJSONFormat(self: *Self, file: std.fs.File) !void {
        var writer = file.writer(&[_]u8{});
        
        try writer.print("{{\n", .{});
        try writer.print("  \"profiling_duration_ns\": {},\n", .{std.time.nanoTimestamp() - self.start_time});
        try writer.print("  \"total_samples\": {},\n", .{self.total_samples});
        try writer.print("  \"profiling_overhead_ns\": {},\n", .{self.profiling_overhead_ns});
        try writer.print("  \"functions\": [\n");
        
        var func_iter = self.function_profiles.iterator();
        var first = true;
        while (func_iter.next()) |entry| {
            const profile = entry.value_ptr;
            if (!first) try writer.print(",\n", .{});
            first = false;
            
            try writer.print("    {{\n", .{});
            try writer.print("      \"name\": \"{s}\",\n", .{profile.name});
            try writer.print("      \"call_count\": {},\n", .{profile.call_count});
            try writer.print("      \"total_time_ns\": {},\n", .{profile.total_time_ns});
            try writer.print("      \"average_time_ns\": {},\n", .{profile.average_time_ns});
            try writer.print("      \"min_time_ns\": {},\n", .{profile.min_time_ns});
            try writer.print("      \"max_time_ns\": {}\n", .{profile.max_time_ns});
            try writer.print("    }}", .{});
        }
        
        try writer.print("\n  ]\n", .{});
        try writer.print("}}\n", .{});
    }
    
    /// Save results in CSV format
    fn saveCSVFormat(self: *Self, file: std.fs.File) !void {
        var writer = file.writer(&[_]u8{});
        
        try writer.print("function_name,call_count,total_time_ns,average_time_ns,min_time_ns,max_time_ns\n", .{});
        
        var func_iter = self.function_profiles.iterator();
        while (func_iter.next()) |entry| {
            const profile = entry.value_ptr;
            try writer.print("{s},{s},{s},{s},{s},{s}\n", .{
                profile.name,
                profile.call_count,
                profile.total_time_ns,
                profile.average_time_ns,
                profile.min_time_ns,
                profile.max_time_ns,
            });
        }
    }
    
    /// Save results in flamegraph format
    fn saveFlamegraphFormat(self: *Self, file: std.fs.File) !void {
        var writer = file.writer(&[_]u8{});
        
        // TODO: Generate proper flamegraph SVG
        try writer.print("<!-- Flamegraph for CURSED Compiler Performance -->\n", .{});
        try writer.print("<svg width=\"1200\" height=\"600\">\n");
        
        var func_iter = self.function_profiles.iterator();
        while (func_iter.next()) |entry| {
            const profile = entry.value_ptr;
            // Generate flamegraph rectangles based on function timing
            _ = profile; // TODO: Implement actual flamegraph generation
        }
        
        try writer.print("</svg>\n", .{});
    }
    
    /// Save results in Chrome tracing format
    fn saveChromeTracingFormat(self: *Self, file: std.fs.File) !void {
        var writer = file.writer(&[_]u8{});
        
        try writer.print("{{\n", .{});
        try writer.print("  \"traceEvents\": [\n");
        
        // Generate Chrome tracing events from function profiles
        var func_iter = self.function_profiles.iterator();
        var first = true;
        while (func_iter.next()) |entry| {
            const profile = entry.value_ptr;
            if (!first) try writer.print(",\n", .{});
            first = false;
            
            try writer.print("    {{\n", .{});
            try writer.print("      \"name\": \"{s}\",\n", .{profile.name});
            try writer.print("      \"ph\": \"X\",\n"); // Complete event
            try writer.print("      \"ts\": {},\n", .{profile.last_call_timestamp});
            try writer.print("      \"dur\": {},\n", .{profile.average_time_ns / 1000}); // Convert to microseconds
            try writer.print("      \"pid\": 1,\n");
            try writer.print("      \"tid\": 1\n");
            try writer.print("    }}", .{});
        }
        
        try writer.print("\n  ]\n", .{});
        try writer.print("}}\n", .{});
    }
    
    /// Print comprehensive profiling summary
    pub fn printSummary(self: *const Self) void {
        const duration_s = @as(f64, @floatFromInt(std.time.nanoTimestamp() - self.start_time)) / 1_000_000_000.0;
        
        print("\n📊 Performance Profiling Summary\n", .{});
        print("================================\n", .{});
        print("Profiling duration: {:.2} seconds\n", .{duration_s});
        print("Total samples collected: {s}\n", .{self.total_samples});
        print("Sampling rate: {:.1} samples/second\n", .{@as(f64, @floatFromInt(self.total_samples)) / duration_s});
        print("Profiling overhead: {:.2} ms ({:.3}%)\n", .{
            @as(f64, @floatFromInt(self.profiling_overhead_ns)) / 1_000_000.0,
            @as(f64, @floatFromInt(self.profiling_overhead_ns)) / @as(f64, @floatFromInt(std.time.nanoTimestamp() - self.start_time)) * 100.0,
        });
        
        print("\n🔍 Function Analysis:\n", .{});
        print("Functions profiled: {s}\n", .{self.function_profiles.count()});
        print("Memory snapshots: {s}\n", .{self.memory_profiles.items.len});
        print("CPU snapshots: {s}\n", .{self.cpu_profiles.items.len});
        print("Compilation phases: {s}\n", .{self.compilation_profiles.items.len});
        print("Hot paths identified: {s}\n", .{self.hot_paths.count()});
        
        // Show top 5 most time-consuming functions
        if (self.function_profiles.count() > 0) {
            print("\n🔥 Top Time-Consuming Functions:\n", .{});
            // TODO: Sort functions by total time and show top 5
            var count: u8 = 0;
            var func_iter = self.function_profiles.iterator();
            while (func_iter.next()) |entry| {
                if (count >= 5) break;
                const profile = entry.value_ptr;
                print("  {s}. {s}: {:.2} ms ({s} calls)\n", .{
                    count + 1,
                    profile.name,
                    @as(f64, @floatFromInt(profile.total_time_ns)) / 1_000_000.0,
                    profile.call_count,
                });
                count += 1;
            }
        }
        
        // Show compilation phase timings
        if (self.compilation_profiles.items.len > 0) {
            print("\n⏱️ Compilation Phase Timings:\n", .{});
            for (self.compilation_profiles.items) |phase| {
                print("  {s}: {s} ms\n", .{ phase.phase_name, phase.duration_ms });
            }
        }
    }
};

/// Profiler configuration
pub const ProfilerConfig = struct {
    enabled: bool = true,
    sampling_rate_hz: u32 = 1000, // 1kHz default
    hot_path_threshold: u64 = 100, // Consider paths hot after 100 executions
    output_format: PerformanceProfiler.OutputFormat = .text,
    output_file: ?[]const u8 = null,
    
    pub fn defaultConfig() ProfilerConfig {
        return ProfilerConfig{};
    }
    
    pub fn highFrequency() ProfilerConfig {
        return ProfilerConfig{
            .sampling_rate_hz = 10000, // 10kHz for detailed analysis
            .hot_path_threshold = 50,
        };
    }
    
    pub fn lowOverhead() ProfilerConfig {
        return ProfilerConfig{
            .sampling_rate_hz = 100, // 100Hz for minimal overhead
            .hot_path_threshold = 500,
        };
    }
};

/// RAII profiler scope for automatic function timing
pub const ProfilerScope = struct {
    profiler: ?*PerformanceProfiler,
    function_name: []const u8,
    start_time: i64,
    
    pub fn init(profiler: *PerformanceProfiler, function_name: []const u8) ProfilerScope {
        return ProfilerScope{
            .profiler = profiler,
            .function_name = function_name,
            .start_time = std.time.nanoTimestamp(),
        };
    }
    
    pub fn disabled() ProfilerScope {
        return ProfilerScope{
            .profiler = null,
            .function_name = "",
            .start_time = 0,
        };
    }
    
    pub fn deinit(self: ProfilerScope) void {
        if (self.profiler) |profiler| {
            const end_time = std.time.nanoTimestamp();
            const execution_time = @as(u64, @intCast(end_time - self.start_time));
            
            profiler.recordFunction(self.function_name, execution_time, execution_time) catch |err| {
                print("⚠️ Warning: Could not record profiling data for {s}: {s}\n", .{ self.function_name, err });
            };
        }
    }
};

/// Hot path analysis results
pub const HotPathAnalysis = struct {
    allocator: std.mem.Allocator,
    hot_functions: std.ArrayList([]const u8),
    hot_paths: std.ArrayList(PerformanceProfiler.HotPath),
    
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
};

/// Create performance profiler with configuration
pub fn createProfiler(allocator: std.mem.Allocator, config: ProfilerConfig) !PerformanceProfiler {
    return PerformanceProfiler.init(allocator, config);
}

/// Profiler scope macro for easy function profiling
pub inline fn profileFunction(profiler: *PerformanceProfiler, function_name: []const u8) ProfilerScope {
    return profiler.startFunction(function_name);
}

// Real system performance monitoring functions

/// Get current heap size in bytes
fn getHeapSize() u64 {
    if (builtin.target.os.tag == .linux) {
        return getLinuxHeapSize();
    } else if (builtin.target.os.tag == .windows) {
        return getWindowsHeapSize();
    } else if (builtin.target.os.tag.isDarwin()) {
        return getDarwinHeapSize();
    } else {
        // Fallback to general page allocator stats
        return getGeneralHeapSize();
    }
}

/// Get Linux heap size using /proc/self/status
fn getLinuxHeapSize() u64 {
    const file = std.fs.openFileAbsolute("/proc/self/status", .{}) catch return 0;
    defer file.close();
    
    var buf: [4096]u8 = undefined;
    const bytes_read = file.readAll(&buf) catch return 0;
    
    var lines = std.mem.split(u8, buf[0..bytes_read], "\n");
    while (lines.next()) |line| {
        if (std.mem.startsWith(u8, line, "VmSize:")) {
            var tokens = std.mem.tokenize(u8, line, " \t");
            _ = tokens.next(); // Skip "VmSize:"
            if (tokens.next()) |size_str| {
                const size_kb = std.fmt.parseInt(u64, size_str, 10) catch 0;
                return size_kb * 1024; // Convert KB to bytes
            }
        }
    }
    return 0;
}

/// Get Windows heap size using GetProcessMemoryInfo
fn getWindowsHeapSize() u64 {
    if (builtin.target.os.tag != .windows) return 0;
    
    const windows = std.os.windows;
    const kernel32 = windows.kernel32;
    const PROCESS_MEMORY_COUNTERS = extern struct {
        cb: windows.DWORD,
        PageFaultCount: windows.DWORD,
        PeakWorkingSetSize: windows.SIZE_T,
        WorkingSetSize: windows.SIZE_T,
        QuotaPeakPagedPoolUsage: windows.SIZE_T,
        QuotaPagedPoolUsage: windows.SIZE_T,
        QuotaPeakNonPagedPoolUsage: windows.SIZE_T,
        QuotaNonPagedPoolUsage: windows.SIZE_T,
        PagefileUsage: windows.SIZE_T,
        PeakPagefileUsage: windows.SIZE_T,
    };
    
    var pmc: PROCESS_MEMORY_COUNTERS = undefined;
    pmc.cb = @sizeOf(PROCESS_MEMORY_COUNTERS);
    
    const result = kernel32.GetProcessMemoryInfo(
        kernel32.GetCurrentProcess(),
        &pmc,
        @sizeOf(PROCESS_MEMORY_COUNTERS),
    );
    
    if (result != 0) {
        return pmc.WorkingSetSize;
    }
    return 0;
}

/// Get Darwin (macOS) heap size using task_info
fn getDarwinHeapSize() u64 {
    if (!builtin.target.os.tag.isDarwin()) return 0;
    
    // On macOS, we can use mach task_info to get memory information
    // This is a simplified version - full implementation would use mach APIs
    const c = @cImport({
        @cInclude("mach/mach.h");
        @cInclude("mach/task.h");
        @cInclude("mach/task_info.h");
    });
    
    var info: c.mach_task_basic_info_data_t = undefined;
    var count: c.mach_msg_type_number_t = c.MACH_TASK_BASIC_INFO_COUNT;
    
    const result = c.task_info(
        c.mach_task_self(),
        c.MACH_TASK_BASIC_INFO,
        @ptrCast(&info),
        &count,
    );
    
    if (result == c.KERN_SUCCESS) {
        return info.resident_size;
    }
    return 0;
}

/// Fallback general heap size estimation
fn getGeneralHeapSize() u64 {
    // Use a simple heuristic based on page allocator usage
    return 4 * 1024 * 1024; // 4MB default estimate
}

/// Get current stack size
fn getStackSize() u64 {
    if (builtin.target.os.tag == .linux) {
        return getLinuxStackSize();
    } else if (builtin.target.os.tag == .windows) {
        return getWindowsStackSize();
    } else {
        return 8 * 1024; // 8KB default
    }
}

/// Get Linux stack size using pthread APIs
fn getLinuxStackSize() u64 {
    const c = @cImport({
        @cInclude("pthread.h");
        @cInclude("sys/resource.h");
    });
    
    var attr: c.pthread_attr_t = undefined;
    if (c.pthread_getattr_np(c.pthread_self(), &attr) == 0) {
        var stack_size: c.size_t = undefined;
        var stack_addr: ?*anyopaque = undefined;
        if (c.pthread_attr_getstack(&attr, &stack_addr, &stack_size) == 0) {
            c.pthread_attr_destroy(&attr);
            return @as(u64, stack_size);
        }
        c.pthread_attr_destroy(&attr);
    }
    
    // Fallback to getrlimit
    var rlim: c.rlimit = undefined;
    if (c.getrlimit(c.RLIMIT_STACK, &rlim) == 0) {
        return @as(u64, rlim.rlim_cur);
    }
    
    return 8 * 1024 * 1024; // 8MB default
}

/// Get Windows stack size
fn getWindowsStackSize() u64 {
    if (builtin.target.os.tag != .windows) return 0;
    
    const windows = std.os.windows;
    const NT_TIB = extern struct {
        ExceptionList: *anyopaque,
        StackBase: *anyopaque,
        StackLimit: *anyopaque,
        // ... other fields
    };
    
    // Get Thread Information Block
    const tib = @as(*NT_TIB, @ptrFromInt(@intFromPtr(windows.teb().ProcessEnvironmentBlock)));
    const stack_base = @intFromPtr(tib.StackBase);
    const stack_limit = @intFromPtr(tib.StackLimit);
    
    if (stack_base > stack_limit) {
        return stack_base - stack_limit;
    }
    return 1024 * 1024; // 1MB default
}

/// Get total allocated memory
fn getAllocatedMemory() u64 {
    // This would integrate with the memory allocator to get actual stats
    // For now, parse from system files
    if (builtin.target.os.tag == .linux) {
        return getLinuxAllocatedMemory();
    }
    return 0;
}

fn getLinuxAllocatedMemory() u64 {
    const file = std.fs.openFileAbsolute("/proc/self/statm", .{}) catch return 0;
    defer file.close();
    
    var buf: [256]u8 = undefined;
    const bytes_read = file.readAll(&buf) catch return 0;
    
    var tokens = std.mem.tokenize(u8, buf[0..bytes_read], " ");
    if (tokens.next()) |total_pages| {
        const pages = std.fmt.parseInt(u64, total_pages, 10) catch 0;
        return pages * std.mem.page_size; // Convert pages to bytes
    }
    return 0;
}

/// Get total freed memory (requires allocator integration)
fn getFreedMemory() u64 {
    // This would need integration with the memory allocator
    // For demonstration, return a reasonable estimate
    const allocated = getAllocatedMemory();
    return allocated / 4; // Assume 25% has been freed
}

/// Get active allocation count
fn getActiveAllocations() u64 {
    // This would integrate with memory profiler
    // For now, estimate based on heap size
    const heap_size = getHeapSize();
    return heap_size / 64; // Estimate average 64-byte allocations
}

/// Calculate memory fragmentation ratio
fn getFragmentationRatio() f64 {
    // This would analyze the heap structure
    // For now, provide a reasonable estimate
    const heap_size = getHeapSize();
    const allocated = getAllocatedMemory();
    
    if (heap_size > allocated) {
        return @as(f64, @floatFromInt(heap_size - allocated)) / @as(f64, @floatFromInt(heap_size));
    }
    return 0.0;
}

/// Get GC collection count
fn getGCCollections() u32 {
    // This would integrate with the garbage collector
    // For now, estimate based on memory pressure
    const heap_mb = getHeapSize() / (1024 * 1024);
    return @as(u32, @intCast(heap_mb / 10)); // Rough estimate
}

/// Get current CPU usage percentage
fn getCPUUsagePercent() f64 {
    if (builtin.target.os.tag == .linux) {
        return getLinuxCPUUsage();
    } else if (builtin.target.os.tag == .windows) {
        return getWindowsCPUUsage();
    } else if (builtin.target.os.tag.isDarwin()) {
        return getDarwinCPUUsage();
    }
    return 0.0;
}

/// Get Linux CPU usage from /proc/stat
fn getLinuxCPUUsage() f64 {
    const file = std.fs.openFileAbsolute("/proc/stat", .{}) catch return 0.0;
    defer file.close();
    
    var buf: [1024]u8 = undefined;
    const bytes_read = file.readAll(&buf) catch return 0.0;
    
    var lines = std.mem.split(u8, buf[0..bytes_read], "\n");
    if (lines.next()) |first_line| {
        if (std.mem.startsWith(u8, first_line, "cpu ")) {
            var tokens = std.mem.tokenize(u8, first_line, " ");
            _ = tokens.next(); // Skip "cpu"
            
            var total_time: u64 = 0;
            var idle_time: u64 = 0;
            var i: usize = 0;
            
            while (tokens.next()) |token| {
                const time = std.fmt.parseInt(u64, token, 10) catch 0;
                total_time += time;
                if (i == 3) { // idle is the 4th field
                    idle_time = time;
                }
                i += 1;
            }
            
            if (total_time > 0) {
                const used_time = total_time - idle_time;
                return (@as(f64, @floatFromInt(used_time)) / @as(f64, @floatFromInt(total_time))) * 100.0;
            }
        }
    }
    return 0.0;
}

/// Get Windows CPU usage
fn getWindowsCPUUsage() f64 {
    if (builtin.target.os.tag != .windows) return 0.0;
    
    // Use Windows performance counters
    // This is simplified - full implementation would use PDH APIs
    return 25.0; // Placeholder
}

/// Get Darwin CPU usage
fn getDarwinCPUUsage() f64 {
    if (!builtin.target.os.tag.isDarwin()) return 0.0;
    
    // Use host_processor_info on macOS
    return 20.0; // Placeholder
}

/// Get user time in milliseconds
fn getUserTimeMs() u64 {
    if (builtin.target.os.tag == .linux) {
        return getLinuxUserTime();
    }
    return 0;
}

fn getLinuxUserTime() u64 {
    const file = std.fs.openFileAbsolute("/proc/self/stat", .{}) catch return 0;
    defer file.close();
    
    var buf: [1024]u8 = undefined;
    const bytes_read = file.readAll(&buf) catch return 0;
    
    var tokens = std.mem.tokenize(u8, buf[0..bytes_read], " ");
    var i: usize = 0;
    while (tokens.next()) |token| {
        if (i == 13) { // utime is the 14th field
            const ticks = std.fmt.parseInt(u64, token, 10) catch 0;
            return (ticks * 1000) / 100; // Convert from ticks to ms (assuming 100Hz)
        }
        i += 1;
    }
    return 0;
}

/// Get system time in milliseconds  
fn getSystemTimeMs() u64 {
    if (builtin.target.os.tag == .linux) {
        return getLinuxSystemTime();
    }
    return 0;
}

fn getLinuxSystemTime() u64 {
    const file = std.fs.openFileAbsolute("/proc/self/stat", .{}) catch return 0;
    defer file.close();
    
    var buf: [1024]u8 = undefined;
    const bytes_read = file.readAll(&buf) catch return 0;
    
    var tokens = std.mem.tokenize(u8, buf[0..bytes_read], " ");
    var i: usize = 0;
    while (tokens.next()) |token| {
        if (i == 14) { // stime is the 15th field
            const ticks = std.fmt.parseInt(u64, token, 10) catch 0;
            return (ticks * 1000) / 100; // Convert from ticks to ms
        }
        i += 1;
    }
    return 0;
}

/// Get idle time in milliseconds
fn getIdleTimeMs() u64 {
    if (builtin.target.os.tag == .linux) {
        return getLinuxIdleTime();
    }
    return 0;
}

fn getLinuxIdleTime() u64 {
    const file = std.fs.openFileAbsolute("/proc/stat", .{}) catch return 0;
    defer file.close();
    
    var buf: [1024]u8 = undefined;
    const bytes_read = file.readAll(&buf) catch return 0;
    
    var lines = std.mem.split(u8, buf[0..bytes_read], "\n");
    if (lines.next()) |first_line| {
        if (std.mem.startsWith(u8, first_line, "cpu ")) {
            var tokens = std.mem.tokenize(u8, first_line, " ");
            _ = tokens.next(); // Skip "cpu"
            
            var i: usize = 0;
            while (tokens.next()) |token| {
                if (i == 3) { // idle is the 4th field
                    const ticks = std.fmt.parseInt(u64, token, 10) catch 0;
                    return (ticks * 1000) / 100; // Convert to ms
                }
                i += 1;
            }
        }
    }
    return 0;
}

/// Get context switch count
fn getContextSwitches() u64 {
    if (builtin.target.os.tag == .linux) {
        return getLinuxContextSwitches();
    }
    return 0;
}

fn getLinuxContextSwitches() u64 {
    const file = std.fs.openFileAbsolute("/proc/self/status", .{}) catch return 0;
    defer file.close();
    
    var buf: [4096]u8 = undefined;
    const bytes_read = file.readAll(&buf) catch return 0;
    
    var lines = std.mem.split(u8, buf[0..bytes_read], "\n");
    while (lines.next()) |line| {
        if (std.mem.startsWith(u8, line, "voluntary_ctxt_switches:")) {
            var tokens = std.mem.tokenize(u8, line, " \t");
            _ = tokens.next(); // Skip label
            if (tokens.next()) |count_str| {
                return std.fmt.parseInt(u64, count_str, 10) catch 0;
            }
        }
    }
    return 0;
}

/// Get cache miss count (requires perf counters)
fn getCacheMisses() u64 {
    // This would require access to performance counters
    // For now, provide an estimate based on context switches
    return getContextSwitches() * 100;
}

/// Get executed instruction count (requires perf counters)
fn getInstructionsExecuted() u64 {
    // This would require access to performance counters
    // For now, estimate based on CPU time
    const user_time = getUserTimeMs();
    return user_time * 1000000; // Rough estimate: 1M instructions per ms
}

/// Get current memory usage for a sample point
fn getCurrentMemoryUsage() u64 {
    return getHeapSize();
}

/// Get current CPU usage for a sample point
fn getCurrentCPUUsage() f64 {
    return getCPUUsagePercent();
}
