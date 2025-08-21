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
                .memory_usage_bytes = 0, // TODO: Get actual memory usage
                .cpu_usage_percent = 0.0, // TODO: Get actual CPU usage
            };
            
            try self.samples.append(sample);
        }
        
        pub fn deinit(self: *FunctionProfile) void {
            self.samples.deinit();
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
            self.function_sequence.deinit();
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
            .function_profiles = std.HashMap([]const u8, FunctionProfile, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
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
            print("  Sampling rate: {} Hz\n", .{profiler.sampling_rate_hz});
            print("  Hot path threshold: {} executions\n", .{profiler.hot_path_threshold});
            print("  Output format: {}\n", .{profiler.output_format});
        }
        
        return profiler;
    }
    
    /// Cleanup profiler and save results
    pub fn deinit(self: *Self) void {
        if (self.profiling_enabled) {
            // Save profiling results
            self.saveResults() catch |err| {
                print("⚠️ Warning: Could not save profiling results: {}\n", .{err});
            };
            
            // Print summary
            self.printSummary();
        }
        
        // Cleanup data structures
        var func_iter = self.function_profiles.iterator();
        while (func_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.function_profiles.deinit();
        
        var hot_path_iter = self.hot_paths.iterator();
        while (hot_path_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.hot_paths.deinit();
        
        self.memory_profiles.deinit();
        self.cpu_profiles.deinit();
        self.compilation_profiles.deinit();
        
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
        
        // TODO: Get actual memory statistics from system
        // For now, use placeholder values
        snapshot.heap_size_bytes = 1024 * 1024; // 1MB placeholder
        snapshot.stack_size_bytes = 8 * 1024;   // 8KB placeholder
        
        try self.memory_profiles.append(snapshot);
    }
    
    /// Take CPU snapshot
    pub fn takeCPUSnapshot(self: *Self) !void {
        if (!self.profiling_enabled) return;
        
        var snapshot = CPUSnapshot.init();
        
        // TODO: Get actual CPU statistics from system
        // For now, use placeholder values
        snapshot.cpu_usage_percent = 50.0; // Placeholder
        
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
                print("  🔥 Hot function: {s} ({} calls, {:.2} ms avg)\n", .{
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
                try analysis.hot_paths.append(path.*);
                print("  🛤️ Hot path {}: {} executions, {:.2} ms avg\n", .{
                    path.path_id,
                    path.execution_count,
                    @as(f64, @floatFromInt(path.average_time_ns)) / 1_000_000.0,
                });
            }
        }
        
        print("  ✅ Identified {} hot functions and {} hot paths\n", .{
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
        try writer.print("Total Samples: {}\n", .{self.total_samples});
        try writer.print("Profiling Overhead: {:.2} ms\n\n", .{
            @as(f64, @floatFromInt(self.profiling_overhead_ns)) / 1_000_000.0
        });
        
        try writer.print("Function Profiles:\n", .{});
        try writer.print("------------------\n", .{});
        
        var func_iter = self.function_profiles.iterator();
        while (func_iter.next()) |entry| {
            const profile = entry.value_ptr;
            try writer.print("{s}:\n", .{profile.name});
            try writer.print("  Calls: {}\n", .{profile.call_count});
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
            try writer.print("{s},{},{},{},{},{}\n", .{
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
        print("Total samples collected: {}\n", .{self.total_samples});
        print("Sampling rate: {:.1} samples/second\n", .{@as(f64, @floatFromInt(self.total_samples)) / duration_s});
        print("Profiling overhead: {:.2} ms ({:.3}%)\n", .{
            @as(f64, @floatFromInt(self.profiling_overhead_ns)) / 1_000_000.0,
            @as(f64, @floatFromInt(self.profiling_overhead_ns)) / @as(f64, @floatFromInt(std.time.nanoTimestamp() - self.start_time)) * 100.0,
        });
        
        print("\n🔍 Function Analysis:\n", .{});
        print("Functions profiled: {}\n", .{self.function_profiles.count()});
        print("Memory snapshots: {}\n", .{self.memory_profiles.items.len});
        print("CPU snapshots: {}\n", .{self.cpu_profiles.items.len});
        print("Compilation phases: {}\n", .{self.compilation_profiles.items.len});
        print("Hot paths identified: {}\n", .{self.hot_paths.count()});
        
        // Show top 5 most time-consuming functions
        if (self.function_profiles.count() > 0) {
            print("\n🔥 Top Time-Consuming Functions:\n", .{});
            // TODO: Sort functions by total time and show top 5
            var count: u8 = 0;
            var func_iter = self.function_profiles.iterator();
            while (func_iter.next()) |entry| {
                if (count >= 5) break;
                const profile = entry.value_ptr;
                print("  {}. {s}: {:.2} ms ({} calls)\n", .{
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
                print("  {s}: {} ms\n", .{ phase.phase_name, phase.duration_ms });
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
                print("⚠️ Warning: Could not record profiling data for {s}: {}\n", .{ self.function_name, err });
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
        self.hot_functions.deinit();
        self.hot_paths.deinit();
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
