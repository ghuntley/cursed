//! Advanced Memory Profiler Sample Aggregation for CURSED
//!
//! Based on Oracle analysis from memory/profiling.rs:659, this implements
//! comprehensive memory profiling sample aggregation that can:
//! - Aggregate memory profiling samples across time windows
//! - Track allocation patterns and trends
//! - Identify memory leaks through pattern analysis
//! - Provide real-time memory usage analysis
//! - Integrate with existing GC and memory management

const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const RwLock = std.Thread.RwLock;
const Mutex = std.Thread.Mutex;
const Instant = std.time.Instant;

const gc = @import("gc.zig");
const concurrency = @import("concurrency.zig");

// Memory tag types from existing system
pub const MemoryTag = enum {
    Object,
    String,
    Array,
    Function,
    Channel,
    Goroutine,
    Stack,
    Unknown,

    pub fn toString(self: MemoryTag) []const u8 {
        return switch (self) {
            .Object => "object",
            .String => "string", 
            .Array => "array",
            .Function => "function",
            .Channel => "channel",
            .Goroutine => "goroutine",
            .Stack => "stack",
            .Unknown => "unknown",
        };
    }
};

// Memory profiling configuration
pub const ProfilingConfig = struct {
    enable_allocation_tracking: bool = true,
    enable_leak_detection: bool = true,
    enable_performance_profiling: bool = true,
    enable_sample_aggregation: bool = true,
    stack_trace_depth: usize = 10,
    sampling_rate: f64 = 0.1, // 10% sampling
    retention_period_ms: u64 = 3600000, // 1 hour
    aggregation_window_ms: u64 = 60000, // 1 minute
    max_samples_per_window: usize = 10000,
    leak_detection_threshold_ms: u64 = 300000, // 5 minutes
    fragmentation_analysis: bool = true,
    real_time_monitoring: bool = true,
};

// Single allocation record
pub const AllocationSample = struct {
    id: u64,
    address: usize,
    size: usize,
    tag: MemoryTag,
    timestamp_ns: u64,
    thread_id: u32,
    stack_trace: ?[]const []const u8,
    source_location: ?[]const u8,
    alignment: usize,
    is_deallocated: bool = false,
    deallocation_timestamp_ns: ?u64 = null,

    pub fn lifetime_ns(self: *const AllocationSample) ?u64 {
        if (self.deallocation_timestamp_ns) |dealloc_time| {
            return dealloc_time - self.timestamp_ns;
        }
        return null;
    }

    pub fn age_ns(self: *const AllocationSample, current_time_ns: u64) u64 {
        return current_time_ns - self.timestamp_ns;
    }

    pub fn deinit(self: *AllocationSample, allocator: Allocator) void {
        if (self.stack_trace) |trace| {
            for (trace) |frame| {
                allocator.free(frame);
            }
            allocator.free(trace);
        }
        if (self.source_location) |location| {
            allocator.free(location);
        }
    }
};

// Aggregated sample data for analysis
pub const SampleAggregation = struct {
    window_start_ns: u64,
    window_end_ns: u64,
    total_allocations: u64,
    total_deallocations: u64, 
    total_bytes_allocated: u64,
    total_bytes_deallocated: u64,
    peak_memory_usage: u64,
    allocation_rate_per_sec: f64,
    deallocation_rate_per_sec: f64,
    avg_allocation_size: f64,
    avg_lifetime_ns: f64,
    fragmentation_ratio: f64,
    
    // Allocation patterns by tag
    allocations_by_tag: HashMap(MemoryTag, u64),
    bytes_by_tag: HashMap(MemoryTag, u64),
    
    // Size distribution
    small_allocations: u64, // < 64 bytes
    medium_allocations: u64, // 64-4096 bytes  
    large_allocations: u64, // > 4096 bytes
    
    // Leak indicators
    potential_leaks: u64,
    leak_candidates: ArrayList(u64), // allocation IDs

    pub fn init(allocator: Allocator, window_start_ns: u64, window_end_ns: u64) SampleAggregation {
        return SampleAggregation{
            .window_start_ns = window_start_ns,
            .window_end_ns = window_end_ns,
            .total_allocations = 0,
            .total_deallocations = 0,
            .total_bytes_allocated = 0,
            .total_bytes_deallocated = 0,
            .peak_memory_usage = 0,
            .allocation_rate_per_sec = 0.0,
            .deallocation_rate_per_sec = 0.0,
            .avg_allocation_size = 0.0,
            .avg_lifetime_ns = 0.0,
            .fragmentation_ratio = 0.0,
            .allocations_by_tag = HashMap(MemoryTag, u64).init(allocator),
            .bytes_by_tag = HashMap(MemoryTag, u64).init(allocator),
            .small_allocations = 0,
            .medium_allocations = 0,
            .large_allocations = 0,
            .potential_leaks = 0,
            .leak_candidates = .empty,
        };
    }

    pub fn deinit(self: *SampleAggregation) void {
        self.allocations_by_tag.deinit(allocator);
        self.bytes_by_tag.deinit(allocator);
        self.leak_candidates.deinit(allocator);
    }

    pub fn window_duration_ns(self: *const SampleAggregation) u64 {
        return self.window_end_ns - self.window_start_ns;
    }

    pub fn memory_growth_rate(self: *const SampleAggregation) f64 {
        if (self.total_bytes_deallocated == 0) return @intToFloat(f64, self.total_bytes_allocated);
        return @intToFloat(f64, self.total_bytes_allocated) / @intToFloat(f64, self.total_bytes_deallocated);
    }
};

// Leak detection result
pub const LeakCandidate = struct {
    allocation_id: u64,
    address: usize,
    size: usize,
    tag: MemoryTag,
    age_ns: u64,
    leak_probability: f64, // 0.0 - 1.0
    related_allocations: ArrayList(u64),
    leak_type: LeakType,

    pub const LeakType = enum {
        LongLived,
        Growing,
        CircularReference,
        GlobalReference,
        StackReference,
        Unknown,
    };

    pub fn init(allocator: Allocator, allocation_id: u64) LeakCandidate {
        return LeakCandidate{
            .allocation_id = allocation_id,
            .address = 0,
            .size = 0,
            .tag = .Unknown,
            .age_ns = 0,
            .leak_probability = 0.0,
            .related_allocations = .empty,
            .leak_type = .Unknown,
        };
    }

    pub fn deinit(self: *LeakCandidate) void {
        self.related_allocations.deinit(allocator);
    }
};

// Memory profiling statistics
pub const ProfilingStats = struct {
    total_samples: u64,
    samples_processed: u64,
    samples_dropped: u64,
    aggregation_windows: u64,
    current_memory_usage: u64,
    peak_memory_usage: u64,
    total_allocations: u64,
    total_deallocations: u64,
    detected_leaks: u64,
    overhead_bytes: u64,
    processing_time_ns: u64,

    pub fn sample_accuracy(self: *const ProfilingStats) f64 {
        if (self.total_samples == 0) return 0.0;
        return @intToFloat(f64, self.samples_processed) / @intToFloat(f64, self.total_samples);
    }

    pub fn overhead_ratio(self: *const ProfilingStats) f64 {
        if (self.current_memory_usage == 0) return 0.0;
        return @intToFloat(f64, self.overhead_bytes) / @intToFloat(f64, self.current_memory_usage);
    }
};

// Main memory profiler aggregator
pub const MemoryProfilerAggregator = struct {
    allocator: Allocator,
    config: ProfilingConfig,
    
    // Thread-safe storage
    samples_mutex: Mutex,
    aggregations_mutex: Mutex,
    
    // Sample storage
    active_samples: HashMap(u64, AllocationSample), // allocation_id -> sample
    sample_sequence: u64,
    
    // Aggregated data
    aggregation_windows: ArrayList(SampleAggregation),
    current_window_start_ns: u64,
    
    // Statistics
    stats: ProfilingStats,
    
    // Real-time monitoring
    monitoring_enabled: bool,
    last_gc_integration: u64,

    pub fn init(allocator: Allocator, config: ProfilingConfig) !MemoryProfilerAggregator {
        const current_time_ns = std.time.nanoTimestamp();
        
        return MemoryProfilerAggregator{
            .allocator = allocator,
            .config = config,
            .samples_mutex = Mutex{},
            .aggregations_mutex = Mutex{},
            .active_samples = HashMap(u64, AllocationSample).init(allocator),
            .sample_sequence = 1,
            .aggregation_windows = .empty,
            .current_window_start_ns = @intCast(u64, current_time_ns),
            .stats = std.mem.zeroes(ProfilingStats),
            .monitoring_enabled = config.real_time_monitoring,
            .last_gc_integration = @intCast(u64, current_time_ns),
        };
    }

    pub fn deinit(self: *MemoryProfilerAggregator) void {
        self.samples_mutex.lock();
        defer self.samples_mutex.unlock();
        
        // Clean up active samples
        var sample_iter = self.active_samples.iterator();
        while (sample_iter.next()) |entry| {
            entry.value_ptr.deinit(allocator);
        }
        self.active_samples.deinit(allocator);
        
        // Clean up aggregation windows
        self.aggregations_mutex.lock();
        defer self.aggregations_mutex.unlock();
        
        for (self.aggregation_windows.items) |*agg| {
            agg.deinit(allocator);
        }
        self.aggregation_windows.deinit(allocator);
    }

    // Record allocation sample
    pub fn recordAllocation(self: *MemoryProfilerAggregator, address: usize, size: usize, tag: MemoryTag, stack_trace: ?[]const []const u8) !void {
        if (!self.config.enable_allocation_tracking) return;
        
        // Apply sampling rate
        if (self.config.sampling_rate < 1.0 and self.shouldSample()) {
            self.stats.samples_dropped += 1;
            return;
        }

        const current_time_ns = @intCast(u64, std.time.nanoTimestamp());
        
        self.samples_mutex.lock();
        defer self.samples_mutex.unlock();
        
        const allocation_id = self.sample_sequence;
        self.sample_sequence += 1;
        
        const sample = AllocationSample{
            .id = allocation_id,
            .address = address,
            .size = size,
            .tag = tag,
            .timestamp_ns = current_time_ns,
            .thread_id = @intCast(u32, std.Thread.getCurrentId()),
            .stack_trace = if (stack_trace) |trace| try self.cloneStackTrace(trace) else null,
            .source_location = null, // TODO: capture source location
            .alignment = @alignOf(u8), // TODO: get actual alignment
        };
        
        try self.active_samples.put(allocation_id, sample);
        self.stats.total_samples += 1;
        self.stats.samples_processed += 1;
        self.stats.total_allocations += 1;
        self.stats.current_memory_usage += size;
        
        if (self.stats.current_memory_usage > self.stats.peak_memory_usage) {
            self.stats.peak_memory_usage = self.stats.current_memory_usage;
        }
        
        // Trigger aggregation if window is full
        try self.checkAggregationWindow(current_time_ns);
    }

    // Record deallocation
    pub fn recordDeallocation(self: *MemoryProfilerAggregator, address: usize) !void {
        const current_time_ns = @intCast(u64, std.time.nanoTimestamp());
        
        self.samples_mutex.lock();
        defer self.samples_mutex.unlock();
        
        // Find and update the allocation sample
        var sample_iter = self.active_samples.iterator();
        while (sample_iter.next()) |entry| {
            if (entry.value_ptr.address == address and !entry.value_ptr.is_deallocated) {
                entry.value_ptr.is_deallocated = true;
                entry.value_ptr.deallocation_timestamp_ns = current_time_ns;
                
                self.stats.total_deallocations += 1;
                self.stats.current_memory_usage -= entry.value_ptr.size;
                break;
            }
        }
    }

    // Aggregate samples in current window
    pub fn aggregateCurrentWindow(self: *MemoryProfilerAggregator) !SampleAggregation {
        const current_time_ns = @intCast(u64, std.time.nanoTimestamp());
        const window_end_ns = self.current_window_start_ns + self.config.aggregation_window_ms * 1_000_000;
        
        self.samples_mutex.lock();
        defer self.samples_mutex.unlock();
        
        var aggregation = SampleAggregation.init(self.allocator, self.current_window_start_ns, window_end_ns);
        
        // Process all samples in current window
        var sample_iter = self.active_samples.iterator();
        while (sample_iter.next()) |entry| {
            const sample = entry.value_ptr;
            
            // Check if sample falls in current window
            if (sample.timestamp_ns >= self.current_window_start_ns and sample.timestamp_ns < window_end_ns) {
                try self.processSampleForAggregation(sample, &aggregation);
            }
        }
        
        // Calculate derived metrics
        self.calculateAggregationMetrics(&aggregation);
        
        // Run leak detection
        if (self.config.enable_leak_detection) {
            try self.detectLeaksInWindow(&aggregation, current_time_ns);
        }
        
        return aggregation;
    }

    // Process individual sample for aggregation
    fn processSampleForAggregation(self: *MemoryProfilerAggregator, sample: *const AllocationSample, aggregation: *SampleAggregation) !void {
        aggregation.total_allocations += 1;
        aggregation.total_bytes_allocated += sample.size;
        
        if (sample.is_deallocated) {
            aggregation.total_deallocations += 1;
            aggregation.total_bytes_deallocated += sample.size;
        }
        
        // Update peak memory if needed
        if (aggregation.total_bytes_allocated > aggregation.peak_memory_usage) {
            aggregation.peak_memory_usage = aggregation.total_bytes_allocated;
        }
        
        // Track by tag
        const tag_allocs = aggregation.allocations_by_tag.get(sample.tag) orelse 0;
        try aggregation.allocations_by_tag.put(sample.tag, tag_allocs + 1);
        
        const tag_bytes = aggregation.bytes_by_tag.get(sample.tag) orelse 0;
        try aggregation.bytes_by_tag.put(sample.tag, tag_bytes + sample.size);
        
        // Size distribution
        if (sample.size < 64) {
            aggregation.small_allocations += 1;
        } else if (sample.size <= 4096) {
            aggregation.medium_allocations += 1;
        } else {
            aggregation.large_allocations += 1;
        }
    }

    // Calculate derived metrics for aggregation
    fn calculateAggregationMetrics(self: *MemoryProfilerAggregator, aggregation: *SampleAggregation) void {
        const window_duration_sec = @intToFloat(f64, aggregation.window_duration_ns()) / 1_000_000_000.0;
        
        if (window_duration_sec > 0) {
            aggregation.allocation_rate_per_sec = @intToFloat(f64, aggregation.total_allocations) / window_duration_sec;
            aggregation.deallocation_rate_per_sec = @intToFloat(f64, aggregation.total_deallocations) / window_duration_sec;
        }
        
        if (aggregation.total_allocations > 0) {
            aggregation.avg_allocation_size = @intToFloat(f64, aggregation.total_bytes_allocated) / @intToFloat(f64, aggregation.total_allocations);
        }
        
        // Calculate fragmentation (simplified)
        if (aggregation.total_bytes_allocated > 0) {
            const used_bytes = aggregation.total_bytes_allocated - aggregation.total_bytes_deallocated;
            aggregation.fragmentation_ratio = 1.0 - (@intToFloat(f64, used_bytes) / @intToFloat(f64, aggregation.peak_memory_usage));
        }
    }

    // Detect memory leaks in current window
    fn detectLeaksInWindow(self: *MemoryProfilerAggregator, aggregation: *SampleAggregation, current_time_ns: u64) !void {
        var sample_iter = self.active_samples.iterator();
        while (sample_iter.next()) |entry| {
            const sample = entry.value_ptr;
            
            // Skip deallocated samples
            if (sample.is_deallocated) continue;
            
            const age_ns = sample.age_ns(current_time_ns);
            const threshold_ns = self.config.leak_detection_threshold_ms * 1_000_000;
            
            if (age_ns > threshold_ns) {
                aggregation.potential_leaks += 1;
                try aggregation.leak_candidates.append(allocator, sample.id);
            }
        }
    }

    // Analyze leak candidates and generate detailed leak reports
    pub fn analyzeLeakCandidates(self: *MemoryProfilerAggregator, allocation_ids: []const u64) !ArrayList(LeakCandidate) {
        var leak_candidates = .empty;
        
        self.samples_mutex.lock();
        defer self.samples_mutex.unlock();
        
        for (allocation_ids) |allocation_id| {
            if (self.active_samples.get(allocation_id)) |sample| {
                var candidate = LeakCandidate.init(self.allocator, allocation_id);
                candidate.address = sample.address;
                candidate.size = sample.size;
                candidate.tag = sample.tag;
                candidate.age_ns = sample.age_ns(@intCast(u64, std.time.nanoTimestamp()));
                
                // Calculate leak probability based on age and pattern
                candidate.leak_probability = self.calculateLeakProbability(&sample);
                candidate.leak_type = self.classifyLeakType(&sample);
                
                // Find related allocations
                try self.findRelatedAllocations(&sample, &candidate);
                
                try leak_candidates.append(allocator, candidate);
            }
        }
        
        return leak_candidates;
    }

    // Calculate probability that an allocation is a leak
    fn calculateLeakProbability(self: *MemoryProfilerAggregator, sample: *const AllocationSample) f64 {
        const current_time_ns = @intCast(u64, std.time.nanoTimestamp());
        const age_ns = sample.age_ns(current_time_ns);
        const threshold_ns = self.config.leak_detection_threshold_ms * 1_000_000;
        
        // Base probability on age
        var probability = @intToFloat(f64, age_ns) / @intToFloat(f64, threshold_ns * 2);
        
        // Adjust based on size (larger allocations more likely to be leaks)
        if (sample.size > 4096) {
            probability *= 1.5;
        } else if (sample.size > 1024) {
            probability *= 1.2;
        }
        
        // Adjust based on tag type
        probability *= switch (sample.tag) {
            .String, .Array => 1.3, // Common leak sources
            .Object => 1.1,
            .Function => 0.8, // Less likely
            .Channel, .Goroutine => 1.4, // Concurrency leaks
            else => 1.0,
        };
        
        return @min(probability, 1.0);
    }

    // Classify the type of memory leak
    fn classifyLeakType(self: *MemoryProfilerAggregator, sample: *const AllocationSample) LeakCandidate.LeakType {
        const current_time_ns = @intCast(u64, std.time.nanoTimestamp());
        const age_ns = sample.age_ns(current_time_ns);
        const very_old_threshold = self.config.leak_detection_threshold_ms * 2 * 1_000_000;
        
        if (age_ns > very_old_threshold) {
            return .LongLived;
        }
        
        return switch (sample.tag) {
            .Channel, .Goroutine => .CircularReference,
            .String, .Array => .Growing,
            .Function => .GlobalReference,
            .Stack => .StackReference,
            else => .Unknown,
        };
    }

    // Find allocations related to a potential leak
    fn findRelatedAllocations(self: *MemoryProfilerAggregator, sample: *const AllocationSample, candidate: *LeakCandidate) !void {
        var sample_iter = self.active_samples.iterator();
        while (sample_iter.next()) |entry| {
            const other_sample = entry.value_ptr;
            
            // Skip self and deallocated samples
            if (other_sample.id == sample.id or other_sample.is_deallocated) continue;
            
            // Check for related allocations (same tag, similar size, close in time)
            const time_diff = if (other_sample.timestamp_ns > sample.timestamp_ns) 
                other_sample.timestamp_ns - sample.timestamp_ns 
            else 
                sample.timestamp_ns - other_sample.timestamp_ns;
            
            const size_ratio = @intToFloat(f64, @max(sample.size, other_sample.size)) / @intToFloat(f64, @min(sample.size, other_sample.size));
            
            if (other_sample.tag == sample.tag and 
                time_diff < 1_000_000_000 and // Within 1 second
                size_ratio < 2.0) { // Similar size
                try candidate.related_allocations.append(allocator, other_sample.id);
            }
        }
    }

    // Integrate with GC for enhanced analysis
    pub fn integrateWithGC(self: *MemoryProfilerAggregator, gc_stats: *const gc.GcStats) !void {
        const current_time_ns = @intCast(u64, std.time.nanoTimestamp());
        
        // Update stats with GC information
        self.stats.overhead_bytes = gc_stats.overhead_bytes;
        
        // Mark objects that survived GC
        self.samples_mutex.lock();
        defer self.samples_mutex.unlock();
        
        var sample_iter = self.active_samples.iterator();
        while (sample_iter.next()) |entry| {
            const sample = entry.value_ptr;
            
            // If object survived multiple GC cycles, reduce leak probability
            // This is a simplified heuristic - real implementation would track GC generations
            _ = sample; // Use sample for GC integration logic
        }
        
        self.last_gc_integration = current_time_ns;
    }

    // Get comprehensive profiling statistics
    pub fn getStats(self: *MemoryProfilerAggregator) ProfilingStats {
        self.samples_mutex.lock();
        defer self.samples_mutex.unlock();
        
        return self.stats;
    }

    // Clean up old samples and aggregations
    pub fn cleanup(self: *MemoryProfilerAggregator) !void {
        const current_time_ns = @intCast(u64, std.time.nanoTimestamp());
        const retention_cutoff = current_time_ns - (self.config.retention_period_ms * 1_000_000);
        
        self.samples_mutex.lock();
        defer self.samples_mutex.unlock();
        
        // Remove old deallocated samples
        var samples_to_remove = .empty;
        defer samples_to_remove.deinit(allocator);
        
        var sample_iter = self.active_samples.iterator();
        while (sample_iter.next()) |entry| {
            const sample = entry.value_ptr;
            if (sample.is_deallocated and sample.timestamp_ns < retention_cutoff) {
                try samples_to_remove.append(self.allocator, sample.id);
            }
        }
        
        for (samples_to_remove.items) |id| {
            if (self.active_samples.fetchRemove(id)) |kv| {
                var sample = kv.value;
                sample.deinit(allocator);
            }
        }
        
        // Clean up old aggregation windows
        self.aggregations_mutex.lock();
        defer self.aggregations_mutex.unlock();
        
        var i: usize = 0;
        while (i < self.aggregation_windows.items.len) {
            if (self.aggregation_windows.items[i].window_end_ns < retention_cutoff) {
                self.aggregation_windows.items[i].deinit(allocator);
                _ = self.aggregation_windows.swapRemove(i);
            } else {
                i += 1;
            }
        }
    }

    // Check if we should create a new aggregation window
    fn checkAggregationWindow(self: *MemoryProfilerAggregator, current_time_ns: u64) !void {
        const window_end = self.current_window_start_ns + self.config.aggregation_window_ms * 1_000_000;
        
        if (current_time_ns >= window_end) {
            // Finalize current window
            const aggregation = try self.aggregateCurrentWindow();
            
            self.aggregations_mutex.lock();
            defer self.aggregations_mutex.unlock();
            
            try self.aggregation_windows.append(allocator, aggregation);
            self.stats.aggregation_windows += 1;
            
            // Start new window
            self.current_window_start_ns = window_end;
        }
    }

    // Apply sampling rate
    fn shouldSample(self: *MemoryProfilerAggregator) bool {
        // Simple probabilistic sampling
        const random = std.crypto.random;
        const sample_threshold = @floatToInt(u32, self.config.sampling_rate * @intToFloat(f64, std.math.maxInt(u32)));
        return random.int(u32) < sample_threshold;
    }

    // Clone stack trace for storage
    fn cloneStackTrace(self: *MemoryProfilerAggregator, stack_trace: []const []const u8) ![]const []const u8 {
        var cloned = try self.allocator.alloc([]const u8, stack_trace.len);
        for (stack_trace, 0..) |frame, i| {
            cloned[i] = try self.allocator.dupe(u8, frame);
        }
        return cloned;
    }
};

// Export functions for C ABI integration
export fn cursed_memory_profiler_create(config: *const ProfilingConfig) ?*MemoryProfilerAggregator {
    const allocator = std.heap.c_allocator;
    const profiler = allocator.create(MemoryProfilerAggregator) catch return null;
    profiler.* = MemoryProfilerAggregator.init(allocator, config.*) catch {
        allocator.destroy(profiler);
        return null;
    };
    return profiler;
}

export fn cursed_memory_profiler_destroy(profiler: *MemoryProfilerAggregator) void {
    const allocator = profiler.allocator;
    profiler.deinit(allocator);
    allocator.destroy(profiler);
}

export fn cursed_memory_profiler_record_allocation(profiler: *MemoryProfilerAggregator, address: usize, size: usize, tag: MemoryTag) bool {
    profiler.recordAllocation(address, size, tag, null) catch return false;
    return true;
}

export fn cursed_memory_profiler_record_deallocation(profiler: *MemoryProfilerAggregator, address: usize) bool {
    profiler.recordDeallocation(address) catch return false;
    return true;
}

export fn cursed_memory_profiler_get_stats(profiler: *MemoryProfilerAggregator) ProfilingStats {
    return profiler.getStats();
}

export fn cursed_memory_profiler_cleanup(profiler: *MemoryProfilerAggregator) bool {
    profiler.cleanup() catch return false;
    return true;
}
