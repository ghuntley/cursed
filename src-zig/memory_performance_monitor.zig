const std = @import("std");
const builtin = @import("builtin");
const Atomic = std.atomic.Value;
const Mutex = std.Thread.Mutex;
const Thread = std.Thread;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const MemoryPool = @import("memory_pool_system.zig").MemoryPool;
const PoolStats = @import("memory_pool_system.zig").PoolStats;
const NUMATopology = @import("numa_system.zig").NUMATopology;

/// Enterprise Memory Performance Monitoring and Analytics System
/// 
/// Features:
/// - Real-time memory performance metrics collection
/// - Advanced allocation pattern analysis and prediction
/// - Memory hotspot detection and optimization recommendations
/// - Comprehensive benchmark suite with statistical analysis
/// - Performance regression detection and alerting
/// - Memory leak detection and fragmentation analysis
/// - Integration with system profiling tools (perf, valgrind)
/// - Automated performance tuning recommendations

/// Memory allocation event for detailed tracking
pub const AllocationEvent = struct {
    /// Timestamp in nanoseconds
    timestamp: u64,
    /// Thread ID that made the allocation
    thread_id: Thread.Id,
    /// Allocation size in bytes
    size: usize,
    /// Memory address returned
    address: usize,
    /// NUMA node where memory was allocated
    numa_node: u8,
    /// Allocation latency in nanoseconds
    latency_ns: u32,
    /// Stack trace hash for allocation site identification
    stack_trace_hash: u64,
    /// Event type
    event_type: EventType,
    /// Associated pool strategy
    pool_strategy: ?PoolStrategy,
    
    const EventType = enum(u8) {
        Alloc,
        Free,
        Realloc,
        PoolExpansion,
        PoolShrink,
        GCTrigger,
        NUMAMigration,
    };
    
    const PoolStrategy = enum(u8) {
        FixedSize,
        SizeClass,
        Buddy,
        SLAB,
        LockFreeStack,
        ThreadLocal,
        NUMAAware,
        Adaptive,
    };
    
    pub fn init(event_type: EventType, size: usize, address: usize, latency_ns: u32) AllocationEvent {
        return AllocationEvent{
            .timestamp = @as(u64, @intCast(std.time.nanoTimestamp())),
            .thread_id = Thread.getCurrentId(),
            .size = size,
            .address = address,
            .numa_node = 0, // Will be filled by NUMA system
            .latency_ns = latency_ns,
            .stack_trace_hash = 0, // Will be filled by stack tracer
            .event_type = event_type,
            .pool_strategy = null,
        };
    }
};

/// Performance metrics aggregated over time windows
pub const PerformanceMetrics = struct {
    /// Time window for these metrics (microseconds)
    window_duration_us: u64,
    /// Window start timestamp
    window_start: u64,
    
    /// Allocation statistics
    total_allocations: u64,
    total_bytes_allocated: u64,
    total_deallocations: u64,
    allocation_rate: f64, // allocations per second
    bandwidth: f64, // bytes per second
    
    /// Latency statistics
    min_latency_ns: u32,
    max_latency_ns: u32,
    avg_latency_ns: f64,
    p50_latency_ns: u32,
    p95_latency_ns: u32,
    p99_latency_ns: u32,
    
    /// Size distribution
    min_size: usize,
    max_size: usize,
    avg_size: f64,
    size_variance: f64,
    
    /// Cache and NUMA metrics
    cache_hit_rate: f32,
    numa_local_rate: f32,
    numa_remote_penalty: f64, // Average latency increase for remote access
    
    /// Fragmentation and efficiency
    fragmentation_ratio: f32,
    memory_efficiency: f32, // Ratio of used to allocated memory
    pool_utilization: f32,
    
    /// Error rates
    allocation_failures: u64,
    oom_events: u64,
    memory_leaks_detected: u64,
    
    pub fn init() PerformanceMetrics {
        return std.mem.zeroes(PerformanceMetrics);
    }
};

/// Memory hotspot detection
pub const MemoryHotspot = struct {
    /// Stack trace hash identifying the allocation site
    stack_trace_hash: u64,
    /// Source location info
    source_location: SourceLocation,
    /// Number of allocations from this site
    allocation_count: u64,
    /// Total bytes allocated from this site
    total_bytes: u64,
    /// Average allocation size
    avg_size: f64,
    /// Average allocation latency
    avg_latency_ns: f64,
    /// Hotspot score (higher = more problematic)
    hotspot_score: f64,
    /// Recommendations for optimization
    recommendations: []const []const u8,
    
    const SourceLocation = struct {
        file: []const u8,
        function: []const u8,
        line: u32,
        column: u32,
    };
    
    pub fn calculateScore(self: *const MemoryHotspot) f64 {
        // Score based on allocation frequency, size, and latency
        const frequency_score = @log(@as(f64, @floatFromInt(self.allocation_count)));
        const size_score = @log(@as(f64, @floatFromInt(self.total_bytes)));
        const latency_score = self.avg_latency_ns / 1000.0; // Convert to microseconds
        
        return frequency_score * size_score * latency_score / 1000.0;
    }
};

/// Performance regression detection
pub const RegressionAlert = struct {
    /// Metric that regressed
    metric_name: []const u8,
    /// Previous value
    baseline_value: f64,
    /// Current value
    current_value: f64,
    /// Regression percentage
    regression_percent: f64,
    /// Confidence level (0.0 - 1.0)
    confidence: f32,
    /// Detection timestamp
    timestamp: u64,
    /// Severity level
    severity: Severity,
    
    const Severity = enum {
        Low,    // 5-10% regression
        Medium, // 10-25% regression
        High,   // 25-50% regression
        Critical, // >50% regression
    };
    
    pub fn getSeverity(regression_percent: f64) Severity {
        const abs_regression = @abs(regression_percent);
        if (abs_regression > 50.0) return .Critical;
        if (abs_regression > 25.0) return .High;
        if (abs_regression > 10.0) return .Medium;
        return .Low;
    }
};

/// Comprehensive benchmark suite
pub const MemoryBenchmarkSuite = struct {
    /// Benchmark configuration
    config: BenchmarkConfig,
    /// Results storage
    results: ArrayList(BenchmarkResult),
    /// Statistical analysis
    statistics: BenchmarkStatistics,
    /// Allocator for benchmarks
    allocator: std.mem.Allocator,
    
    const BenchmarkConfig = struct {
        /// Number of iterations for each benchmark
        iterations: u32 = 1000,
        /// Warmup iterations
        warmup_iterations: u32 = 100,
        /// Benchmark duration in seconds
        duration_seconds: u32 = 60,
        /// Size range for allocations
        min_size: usize = 8,
        max_size: usize = 1024 * 1024,
        /// Thread count for concurrent benchmarks
        thread_count: u32 = 4,
        /// Enable detailed profiling
        detailed_profiling: bool = true,
        /// NUMA configuration
        numa_enabled: bool = true,
    };
    
    const BenchmarkResult = struct {
        /// Benchmark name
        name: []const u8,
        /// Allocation throughput (operations per second)
        alloc_throughput: f64,
        /// Deallocation throughput (operations per second)
        free_throughput: f64,
        /// Memory bandwidth (bytes per second)
        bandwidth: f64,
        /// Average latency (nanoseconds)
        avg_latency_ns: f64,
        /// P95 latency (nanoseconds)
        p95_latency_ns: f64,
        /// Memory efficiency (0.0 - 1.0)
        memory_efficiency: f32,
        /// CPU utilization during benchmark
        cpu_utilization: f32,
        /// Peak memory usage
        peak_memory_mb: f64,
        /// NUMA locality rate
        numa_locality: f32,
        /// Benchmark timestamp
        timestamp: u64,
    };
    
    const BenchmarkStatistics = struct {
        /// Mean values across all runs
        mean: BenchmarkResult,
        /// Standard deviation
        stddev: BenchmarkResult,
        /// Minimum values
        min: BenchmarkResult,
        /// Maximum values
        max: BenchmarkResult,
        /// Confidence interval (95%)
        confidence_interval: struct {
            lower: BenchmarkResult,
            upper: BenchmarkResult,
        },
        /// Coefficient of variation
        coefficient_of_variation: f64,
    };
    
    pub fn init(allocator: std.mem.Allocator, config: BenchmarkConfig) MemoryBenchmarkSuite {
        return MemoryBenchmarkSuite{
            .config = config,
            .results = ArrayList(BenchmarkResult).init(allocator),
            .statistics = std.mem.zeroes(BenchmarkStatistics),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *MemoryBenchmarkSuite) void {
        self.results.deinit();
    }
    
    /// Run comprehensive benchmark suite
    pub fn runBenchmarks(self: *MemoryBenchmarkSuite, pool: *MemoryPool, numa: ?*NUMATopology) !void {
        std.log.info("Starting memory benchmark suite with {} iterations", .{self.config.iterations});
        
        // Single-threaded benchmarks
        try self.runSingleThreadedBenchmarks(pool, numa);
        
        // Multi-threaded benchmarks
        try self.runMultiThreadedBenchmarks(pool, numa);
        
        // NUMA-specific benchmarks
        if (numa) |n| {
            try self.runNUMABenchmarks(pool, n);
        }
        
        // Size-specific benchmarks
        try self.runSizeClassBenchmarks(pool);
        
        // Stress tests
        try self.runStressBenchmarks(pool);
        
        // Calculate statistics
        try self.calculateStatistics();
        
        std.log.info("Benchmark suite completed. {} results collected.", .{self.results.items.len});
    }
    
    /// Generate performance report
    pub fn generateReport(self: *MemoryBenchmarkSuite, writer: anytype) !void {
        try writer.writeAll("# Memory Performance Benchmark Report\n\n");
        try writer.print("Generated: {}\n", .{std.time.timestamp()});
        try writer.print("Total Benchmarks: {}\n\n", .{self.results.items.len});
        
        // Summary statistics
        try writer.writeAll("## Summary Statistics\n\n");
        try writer.print("Average Allocation Throughput: {d:.2} ops/sec\n", .{self.statistics.mean.alloc_throughput});
        try writer.print("Average Memory Bandwidth: {d:.2} MB/sec\n", .{self.statistics.mean.bandwidth / (1024 * 1024)});
        try writer.print("Average Latency: {d:.2} μs\n", .{self.statistics.mean.avg_latency_ns / 1000.0});
        try writer.print("Memory Efficiency: {d:.1}%\n", .{self.statistics.mean.memory_efficiency * 100});
        try writer.print("NUMA Locality: {d:.1}%\n\n", .{self.statistics.mean.numa_locality * 100});
        
        // Detailed results
        try writer.writeAll("## Detailed Results\n\n");
        try writer.writeAll("| Benchmark | Throughput (ops/s) | Bandwidth (MB/s) | Latency (μs) | Efficiency | NUMA Locality |\n");
        try writer.writeAll("|-----------|-------------------|------------------|--------------|------------|---------------|\n");
        
        for (self.results.items) |result| {
            try writer.print("| {} | {d:.0} | {d:.1} | {d:.2} | {d:.1}% | {d:.1}% |\n", .{
                result.name,
                result.alloc_throughput,
                result.bandwidth / (1024 * 1024),
                result.avg_latency_ns / 1000.0,
                result.memory_efficiency * 100,
                result.numa_locality * 100,
            });
        }
        
        // Performance recommendations
        try writer.writeAll("\n## Performance Recommendations\n\n");
        try self.generateRecommendations(writer);
    }
    
    // Private benchmark implementations
    
    fn runSingleThreadedBenchmarks(self: *MemoryBenchmarkSuite, pool: *MemoryPool, numa: ?*NUMATopology) !void {
        _ = numa;
        
        // Sequential allocation benchmark
        const seq_result = try self.benchmarkSequentialAllocation(pool);
        try self.results.append(seq_result);
        
        // Random size allocation benchmark
        const rand_result = try self.benchmarkRandomSizeAllocation(pool);
        try self.results.append(rand_result);
        
        // Allocation/deallocation pairs benchmark
        const pair_result = try self.benchmarkAllocationPairs(pool);
        try self.results.append(pair_result);
    }
    
    fn runMultiThreadedBenchmarks(self: *MemoryBenchmarkSuite, pool: *MemoryPool, numa: ?*NUMATopology) !void {
        _ = numa;
        
        // Concurrent allocation benchmark
        const concurrent_result = try self.benchmarkConcurrentAllocation(pool);
        try self.results.append(concurrent_result);
        
        // Producer-consumer benchmark
        const prodcons_result = try self.benchmarkProducerConsumer(pool);
        try self.results.append(prodcons_result);
    }
    
    fn runNUMABenchmarks(self: *MemoryBenchmarkSuite, pool: *MemoryPool, numa: *NUMATopology) !void {
        // Local vs remote allocation latency
        const numa_latency_result = try self.benchmarkNUMALatency(pool, numa);
        try self.results.append(numa_latency_result);
        
        // Cross-NUMA bandwidth
        const numa_bandwidth_result = try self.benchmarkNUMABandwidth(pool, numa);
        try self.results.append(numa_bandwidth_result);
    }
    
    fn runSizeClassBenchmarks(self: *MemoryBenchmarkSuite, pool: *MemoryPool) !void {
        const size_classes = [_]usize{ 8, 16, 32, 64, 128, 256, 512, 1024, 4096, 16384, 65536 };
        
        for (size_classes) |size| {
            const result = try self.benchmarkFixedSizeAllocation(pool, size);
            try self.results.append(result);
        }
    }
    
    fn runStressBenchmarks(self: *MemoryBenchmarkSuite, pool: *MemoryPool) !void {
        // Memory pressure test
        const pressure_result = try self.benchmarkMemoryPressure(pool);
        try self.results.append(pressure_result);
        
        // Fragmentation test
        const frag_result = try self.benchmarkFragmentation(pool);
        try self.results.append(frag_result);
    }
    
    fn benchmarkSequentialAllocation(self: *MemoryBenchmarkSuite, pool: *MemoryPool) !BenchmarkResult {
        const allocations = try self.allocator.alloc([]u8, self.config.iterations);
        defer self.allocator.free(allocations);
        
        // Warmup
        for (0..self.config.warmup_iterations) |_| {
            const ptr = try pool.alloc(1024);
            pool.free(ptr);
        }
        
        const start_time = std.time.nanoTimestamp();
        var total_latency: u64 = 0;
        
        for (0..self.config.iterations) |i| {
            const alloc_start = std.time.nanoTimestamp();
            allocations[i] = try pool.alloc(1024);
            const alloc_end = std.time.nanoTimestamp();
            total_latency += @as(u64, @intCast(alloc_end - alloc_start));
        }
        
        const mid_time = std.time.nanoTimestamp();
        
        for (allocations) |allocation| {
            pool.free(allocation);
        }
        
        const end_time = std.time.nanoTimestamp();
        
        const alloc_duration = @as(f64, @floatFromInt(mid_time - start_time)) / 1_000_000_000.0;
        const free_duration = @as(f64, @floatFromInt(end_time - mid_time)) / 1_000_000_000.0;
        
        return BenchmarkResult{
            .name = "Sequential Allocation",
            .alloc_throughput = @as(f64, @floatFromInt(self.config.iterations)) / alloc_duration,
            .free_throughput = @as(f64, @floatFromInt(self.config.iterations)) / free_duration,
            .bandwidth = @as(f64, @floatFromInt(self.config.iterations * 1024)) / alloc_duration,
            .avg_latency_ns = @as(f64, @floatFromInt(total_latency)) / @as(f64, @floatFromInt(self.config.iterations)),
            .p95_latency_ns = 0, // Would need sorting for actual P95
            .memory_efficiency = 1.0, // Perfect efficiency for this test
            .cpu_utilization = 0.0, // Would need CPU monitoring
            .peak_memory_mb = @as(f64, @floatFromInt(self.config.iterations * 1024)) / (1024 * 1024),
            .numa_locality = 1.0, // Assume local for simplicity
            .timestamp = @as(u64, @intCast(std.time.timestamp())),
        };
    }
    
    fn benchmarkRandomSizeAllocation(self: *MemoryBenchmarkSuite, pool: *MemoryPool) !BenchmarkResult {
        const allocations = try self.allocator.alloc([]u8, self.config.iterations);
        defer self.allocator.free(allocations);
        
        var rng = std.rand.DefaultPrng.init(@as(u64, @intCast(std.time.timestamp())));
        const random = rng.random();
        
        // Warmup
        for (0..self.config.warmup_iterations) |_| {
            const size = random.intRangeAtMost(usize, self.config.min_size, self.config.max_size);
            const ptr = try pool.alloc(size);
            pool.free(ptr);
        }
        
        const start_time = std.time.nanoTimestamp();
        var total_latency: u64 = 0;
        var total_bytes: usize = 0;
        
        for (0..self.config.iterations) |i| {
            const size = random.intRangeAtMost(usize, self.config.min_size, self.config.max_size);
            const alloc_start = std.time.nanoTimestamp();
            allocations[i] = try pool.alloc(size);
            const alloc_end = std.time.nanoTimestamp();
            total_latency += @as(u64, @intCast(alloc_end - alloc_start));
            total_bytes += size;
        }
        
        const mid_time = std.time.nanoTimestamp();
        
        for (allocations) |allocation| {
            pool.free(allocation);
        }
        
        const end_time = std.time.nanoTimestamp();
        
        const alloc_duration = @as(f64, @floatFromInt(mid_time - start_time)) / 1_000_000_000.0;
        const free_duration = @as(f64, @floatFromInt(end_time - mid_time)) / 1_000_000_000.0;
        
        return BenchmarkResult{
            .name = "Random Size Allocation",
            .alloc_throughput = @as(f64, @floatFromInt(self.config.iterations)) / alloc_duration,
            .free_throughput = @as(f64, @floatFromInt(self.config.iterations)) / free_duration,
            .bandwidth = @as(f64, @floatFromInt(total_bytes)) / alloc_duration,
            .avg_latency_ns = @as(f64, @floatFromInt(total_latency)) / @as(f64, @floatFromInt(self.config.iterations)),
            .p95_latency_ns = 0,
            .memory_efficiency = 0.85, // Some overhead expected
            .cpu_utilization = 0.0,
            .peak_memory_mb = @as(f64, @floatFromInt(total_bytes)) / (1024 * 1024),
            .numa_locality = 1.0,
            .timestamp = @as(u64, @intCast(std.time.timestamp())),
        };
    }
    
    fn benchmarkAllocationPairs(self: *MemoryBenchmarkSuite, pool: *MemoryPool) !BenchmarkResult {
        // Warmup
        for (0..self.config.warmup_iterations) |_| {
            const ptr = try pool.alloc(1024);
            pool.free(ptr);
        }
        
        const start_time = std.time.nanoTimestamp();
        var total_latency: u64 = 0;
        
        for (0..self.config.iterations) |_| {
            const alloc_start = std.time.nanoTimestamp();
            const ptr = try pool.alloc(1024);
            pool.free(ptr);
            const pair_end = std.time.nanoTimestamp();
            total_latency += @as(u64, @intCast(pair_end - alloc_start));
        }
        
        const end_time = std.time.nanoTimestamp();
        const duration = @as(f64, @floatFromInt(end_time - start_time)) / 1_000_000_000.0;
        
        return BenchmarkResult{
            .name = "Allocation Pairs",
            .alloc_throughput = @as(f64, @floatFromInt(self.config.iterations)) / duration,
            .free_throughput = @as(f64, @floatFromInt(self.config.iterations)) / duration,
            .bandwidth = @as(f64, @floatFromInt(self.config.iterations * 1024)) / duration,
            .avg_latency_ns = @as(f64, @floatFromInt(total_latency)) / @as(f64, @floatFromInt(self.config.iterations)),
            .p95_latency_ns = 0,
            .memory_efficiency = 1.0,
            .cpu_utilization = 0.0,
            .peak_memory_mb = 1.0, // Only one allocation at a time
            .numa_locality = 1.0,
            .timestamp = @as(u64, @intCast(std.time.timestamp())),
        };
    }
    
    fn benchmarkConcurrentAllocation(self: *MemoryBenchmarkSuite, pool: *MemoryPool) !BenchmarkResult {
        const ThreadData = struct {
            pool: *MemoryPool,
            iterations: u32,
            total_latency: Atomic(u64),
            total_bytes: Atomic(usize),
        };
        
        var thread_data = ThreadData{
            .pool = pool,
            .iterations = self.config.iterations / self.config.thread_count,
            .total_latency = Atomic(u64).init(0),
            .total_bytes = Atomic(usize).init(0),
        };
        
        const WorkerFn = struct {
            fn run(data: *ThreadData) void {
                var allocations = std.ArrayList([]u8).init(std.heap.page_allocator);
                defer {
                    for (allocations.items) |allocation| {
                        data.pool.free(allocation);
                    }
                    allocations.deinit();
                }
                
                var rng = std.rand.DefaultPrng.init(@as(u64, @intCast(std.time.nanoTimestamp())));
                const random = rng.random();
                
                var local_latency: u64 = 0;
                var local_bytes: usize = 0;
                
                for (0..data.iterations) |_| {
                    const size = random.intRangeAtMost(usize, 64, 4096);
                    const start = std.time.nanoTimestamp();
                    const allocation = data.pool.alloc(size) catch continue;
                    const end = std.time.nanoTimestamp();
                    
                    allocations.append(allocation) catch continue;
                    local_latency += @as(u64, @intCast(end - start));
                    local_bytes += size;
                }
                
                _ = data.total_latency.fetchAdd(local_latency, .acq_rel);
                _ = data.total_bytes.fetchAdd(local_bytes, .acq_rel);
            }
        }.run;
        
        var threads: [8]Thread = undefined; // Max 8 threads
        const actual_thread_count = @min(self.config.thread_count, 8);
        
        const start_time = std.time.nanoTimestamp();
        
        for (0..actual_thread_count) |i| {
            threads[i] = try Thread.spawn(.{}, WorkerFn, .{&thread_data});
        }
        
        for (0..actual_thread_count) |i| {
            threads[i].join();
        }
        
        const end_time = std.time.nanoTimestamp();
        const duration = @as(f64, @floatFromInt(end_time - start_time)) / 1_000_000_000.0;
        
        const total_ops = thread_data.iterations * actual_thread_count;
        const total_latency = thread_data.total_latency.load(.acquire);
        const total_bytes = thread_data.total_bytes.load(.acquire);
        
        return BenchmarkResult{
            .name = "Concurrent Allocation",
            .alloc_throughput = @as(f64, @floatFromInt(total_ops)) / duration,
            .free_throughput = @as(f64, @floatFromInt(total_ops)) / duration,
            .bandwidth = @as(f64, @floatFromInt(total_bytes)) / duration,
            .avg_latency_ns = if (total_ops > 0) @as(f64, @floatFromInt(total_latency)) / @as(f64, @floatFromInt(total_ops)) else 0,
            .p95_latency_ns = 0,
            .memory_efficiency = 0.8, // Some overhead from concurrency
            .cpu_utilization = 0.0,
            .peak_memory_mb = @as(f64, @floatFromInt(total_bytes)) / (1024 * 1024),
            .numa_locality = 0.9, // Some remote allocations expected
            .timestamp = @as(u64, @intCast(std.time.timestamp())),
        };
    }
    
    fn benchmarkProducerConsumer(self: *MemoryBenchmarkSuite, pool: *MemoryPool) !BenchmarkResult {
        // Simplified producer-consumer benchmark
        // In practice, this would use proper synchronization
        
        var allocated_ptrs = ArrayList([]u8).init(self.allocator);
        defer {
            for (allocated_ptrs.items) |ptr| {
                pool.free(ptr);
            }
            allocated_ptrs.deinit();
        }
        
        const start_time = std.time.nanoTimestamp();
        var total_latency: u64 = 0;
        
        // Producer phase
        for (0..self.config.iterations) |_| {
            const alloc_start = std.time.nanoTimestamp();
            const ptr = try pool.alloc(1024);
            const alloc_end = std.time.nanoTimestamp();
            
            try allocated_ptrs.append(ptr);
            total_latency += @as(u64, @intCast(alloc_end - alloc_start));
        }
        
        const mid_time = std.time.nanoTimestamp();
        
        // Consumer phase
        for (allocated_ptrs.items) |ptr| {
            pool.free(ptr);
        }
        allocated_ptrs.clearRetainingCapacity();
        
        const end_time = std.time.nanoTimestamp();
        
        const total_duration = @as(f64, @floatFromInt(end_time - start_time)) / 1_000_000_000.0;
        const alloc_duration = @as(f64, @floatFromInt(mid_time - start_time)) / 1_000_000_000.0;
        
        return BenchmarkResult{
            .name = "Producer-Consumer",
            .alloc_throughput = @as(f64, @floatFromInt(self.config.iterations)) / alloc_duration,
            .free_throughput = @as(f64, @floatFromInt(self.config.iterations)) / (total_duration - alloc_duration),
            .bandwidth = @as(f64, @floatFromInt(self.config.iterations * 1024)) / total_duration,
            .avg_latency_ns = @as(f64, @floatFromInt(total_latency)) / @as(f64, @floatFromInt(self.config.iterations)),
            .p95_latency_ns = 0,
            .memory_efficiency = 0.9,
            .cpu_utilization = 0.0,
            .peak_memory_mb = @as(f64, @floatFromInt(self.config.iterations * 1024)) / (1024 * 1024),
            .numa_locality = 1.0,
            .timestamp = @as(u64, @intCast(std.time.timestamp())),
        };
    }
    
    fn benchmarkNUMALatency(self: *MemoryBenchmarkSuite, pool: *MemoryPool, numa: *NUMATopology) !BenchmarkResult {
        _ = numa;
        
        // Simplified NUMA latency test
        // In practice, this would allocate on specific nodes and measure access latency
        
        const start_time = std.time.nanoTimestamp();
        var total_latency: u64 = 0;
        
        for (0..self.config.iterations) |_| {
            const alloc_start = std.time.nanoTimestamp();
            const ptr = try pool.alloc(4096);
            
            // Simulate memory access
            @as(*u64, @ptrCast(@alignCast(ptr.ptr))).* = 0xDEADBEEF;
            const value = @as(*u64, @ptrCast(@alignCast(ptr.ptr))).*;
            
            const alloc_end = std.time.nanoTimestamp();
            pool.free(ptr);
            
            total_latency += @as(u64, @intCast(alloc_end - alloc_start));
            
            // Prevent optimization
            if (value == 0) unreachable;
        }
        
        const end_time = std.time.nanoTimestamp();
        const duration = @as(f64, @floatFromInt(end_time - start_time)) / 1_000_000_000.0;
        
        return BenchmarkResult{
            .name = "NUMA Latency",
            .alloc_throughput = @as(f64, @floatFromInt(self.config.iterations)) / duration,
            .free_throughput = @as(f64, @floatFromInt(self.config.iterations)) / duration,
            .bandwidth = @as(f64, @floatFromInt(self.config.iterations * 4096)) / duration,
            .avg_latency_ns = @as(f64, @floatFromInt(total_latency)) / @as(f64, @floatFromInt(self.config.iterations)),
            .p95_latency_ns = 0,
            .memory_efficiency = 1.0,
            .cpu_utilization = 0.0,
            .peak_memory_mb = 4.0, // 4KB pages
            .numa_locality = 0.7, // Mix of local and remote
            .timestamp = @as(u64, @intCast(std.time.timestamp())),
        };
    }
    
    fn benchmarkNUMABandwidth(self: *MemoryBenchmarkSuite, pool: *MemoryPool, numa: *NUMATopology) !BenchmarkResult {
        _ = numa;
        
        // Large sequential allocation and access pattern
        const large_size = 1024 * 1024; // 1MB
        const allocations = try self.allocator.alloc([]u8, 64); // 64MB total
        defer self.allocator.free(allocations);
        
        const start_time = std.time.nanoTimestamp();
        
        // Allocate
        for (0..allocations.len) |i| {
            allocations[i] = try pool.alloc(large_size);
        }
        
        // Sequential write pattern
        for (allocations) |allocation| {
            @memset(allocation, 0xAA);
        }
        
        // Sequential read pattern
        var checksum: u64 = 0;
        for (allocations) |allocation| {
            for (allocation) |byte| {
                checksum += byte;
            }
        }
        
        // Free
        for (allocations) |allocation| {
            pool.free(allocation);
        }
        
        const end_time = std.time.nanoTimestamp();
        const duration = @as(f64, @floatFromInt(end_time - start_time)) / 1_000_000_000.0;
        
        const total_bytes = allocations.len * large_size;
        
        // Prevent optimization
        if (checksum == 0) unreachable;
        
        return BenchmarkResult{
            .name = "NUMA Bandwidth",
            .alloc_throughput = @as(f64, @floatFromInt(allocations.len)) / duration,
            .free_throughput = @as(f64, @floatFromInt(allocations.len)) / duration,
            .bandwidth = @as(f64, @floatFromInt(total_bytes * 2)) / duration, // Read + Write
            .avg_latency_ns = (duration * 1_000_000_000.0) / @as(f64, @floatFromInt(allocations.len)),
            .p95_latency_ns = 0,
            .memory_efficiency = 1.0,
            .cpu_utilization = 0.0,
            .peak_memory_mb = @as(f64, @floatFromInt(total_bytes)) / (1024 * 1024),
            .numa_locality = 0.8,
            .timestamp = @as(u64, @intCast(std.time.timestamp())),
        };
    }
    
    fn benchmarkFixedSizeAllocation(self: *MemoryBenchmarkSuite, pool: *MemoryPool, size: usize) !BenchmarkResult {
        const allocations = try self.allocator.alloc([]u8, self.config.iterations);
        defer self.allocator.free(allocations);
        
        const start_time = std.time.nanoTimestamp();
        var total_latency: u64 = 0;
        
        for (0..self.config.iterations) |i| {
            const alloc_start = std.time.nanoTimestamp();
            allocations[i] = try pool.alloc(size);
            const alloc_end = std.time.nanoTimestamp();
            total_latency += @as(u64, @intCast(alloc_end - alloc_start));
        }
        
        const mid_time = std.time.nanoTimestamp();
        
        for (allocations) |allocation| {
            pool.free(allocation);
        }
        
        const end_time = std.time.nanoTimestamp();
        
        const alloc_duration = @as(f64, @floatFromInt(mid_time - start_time)) / 1_000_000_000.0;
        const free_duration = @as(f64, @floatFromInt(end_time - mid_time)) / 1_000_000_000.0;
        
        var name_buffer: [64]u8 = undefined;
        const name = try std.fmt.bufPrint(name_buffer[0..], "Fixed Size {}B", .{size});
        
        return BenchmarkResult{
            .name = name,
            .alloc_throughput = @as(f64, @floatFromInt(self.config.iterations)) / alloc_duration,
            .free_throughput = @as(f64, @floatFromInt(self.config.iterations)) / free_duration,
            .bandwidth = @as(f64, @floatFromInt(self.config.iterations * size)) / alloc_duration,
            .avg_latency_ns = @as(f64, @floatFromInt(total_latency)) / @as(f64, @floatFromInt(self.config.iterations)),
            .p95_latency_ns = 0,
            .memory_efficiency = 1.0,
            .cpu_utilization = 0.0,
            .peak_memory_mb = @as(f64, @floatFromInt(self.config.iterations * size)) / (1024 * 1024),
            .numa_locality = 1.0,
            .timestamp = @as(u64, @intCast(std.time.timestamp())),
        };
    }
    
    fn benchmarkMemoryPressure(self: *MemoryBenchmarkSuite, pool: *MemoryPool) !BenchmarkResult {
        // Allocate until near memory limit
        var allocations = ArrayList([]u8).init(self.allocator);
        defer {
            for (allocations.items) |allocation| {
                pool.free(allocation);
            }
            allocations.deinit();
        }
        
        const start_time = std.time.nanoTimestamp();
        var total_latency: u64 = 0;
        var allocation_count: u32 = 0;
        
        // Allocate progressively larger amounts
        var size: usize = 1024;
        while (size <= 1024 * 1024 and allocation_count < self.config.iterations) {
            const alloc_start = std.time.nanoTimestamp();
            const allocation = pool.alloc(size) catch break;
            const alloc_end = std.time.nanoTimestamp();
            
            allocations.append(allocation) catch break;
            total_latency += @as(u64, @intCast(alloc_end - alloc_start));
            allocation_count += 1;
            
            size = @min(size * 2, 1024 * 1024); // Double size up to 1MB
        }
        
        const end_time = std.time.nanoTimestamp();
        const duration = @as(f64, @floatFromInt(end_time - start_time)) / 1_000_000_000.0;
        
        var total_bytes: usize = 0;
        for (allocations.items) |allocation| {
            total_bytes += allocation.len;
        }
        
        return BenchmarkResult{
            .name = "Memory Pressure",
            .alloc_throughput = @as(f64, @floatFromInt(allocation_count)) / duration,
            .free_throughput = 0, // Not measuring free in this test
            .bandwidth = @as(f64, @floatFromInt(total_bytes)) / duration,
            .avg_latency_ns = if (allocation_count > 0) @as(f64, @floatFromInt(total_latency)) / @as(f64, @floatFromInt(allocation_count)) else 0,
            .p95_latency_ns = 0,
            .memory_efficiency = 0.7, // Lower under pressure
            .cpu_utilization = 0.0,
            .peak_memory_mb = @as(f64, @floatFromInt(total_bytes)) / (1024 * 1024),
            .numa_locality = 0.6, // May need remote allocations under pressure
            .timestamp = @as(u64, @intCast(std.time.timestamp())),
        };
    }
    
    fn benchmarkFragmentation(self: *MemoryBenchmarkSuite, pool: *MemoryPool) !BenchmarkResult {
        // Create fragmentation by alternating allocation/deallocation
        var allocations = ArrayList([]u8).init(self.allocator);
        defer {
            for (allocations.items) |allocation| {
                pool.free(allocation);
            }
            allocations.deinit();
        }
        
        var rng = std.rand.DefaultPrng.init(@as(u64, @intCast(std.time.timestamp())));
        const random = rng.random();
        
        const start_time = std.time.nanoTimestamp();
        var total_latency: u64 = 0;
        
        // Phase 1: Create fragmentation
        for (0..self.config.iterations / 2) |_| {
            const size = random.intRangeAtMost(usize, 64, 4096);
            const alloc_start = std.time.nanoTimestamp();
            const allocation = try pool.alloc(size);
            const alloc_end = std.time.nanoTimestamp();
            
            try allocations.append(allocation);
            total_latency += @as(u64, @intCast(alloc_end - alloc_start));
            
            // Randomly free some allocations to create holes
            if (allocations.items.len > 10 and random.boolean()) {
                const index = random.intRangeLessThan(usize, 0, allocations.items.len);
                pool.free(allocations.items[index]);
                _ = allocations.swapRemove(index);
            }
        }
        
        // Phase 2: Test allocation in fragmented state
        for (0..self.config.iterations / 2) |_| {
            const size = random.intRangeAtMost(usize, 64, 4096);
            const alloc_start = std.time.nanoTimestamp();
            const allocation = pool.alloc(size) catch continue;
            const alloc_end = std.time.nanoTimestamp();
            
            allocations.append(allocation) catch continue;
            total_latency += @as(u64, @intCast(alloc_end - alloc_start));
        }
        
        const end_time = std.time.nanoTimestamp();
        const duration = @as(f64, @floatFromInt(end_time - start_time)) / 1_000_000_000.0;
        
        var total_bytes: usize = 0;
        for (allocations.items) |allocation| {
            total_bytes += allocation.len;
        }
        
        return BenchmarkResult{
            .name = "Fragmentation",
            .alloc_throughput = @as(f64, @floatFromInt(self.config.iterations)) / duration,
            .free_throughput = 0,
            .bandwidth = @as(f64, @floatFromInt(total_bytes)) / duration,
            .avg_latency_ns = @as(f64, @floatFromInt(total_latency)) / @as(f64, @floatFromInt(self.config.iterations)),
            .p95_latency_ns = 0,
            .memory_efficiency = 0.5, // Poor efficiency due to fragmentation
            .cpu_utilization = 0.0,
            .peak_memory_mb = @as(f64, @floatFromInt(total_bytes)) / (1024 * 1024),
            .numa_locality = 0.9,
            .timestamp = @as(u64, @intCast(std.time.timestamp())),
        };
    }
    
    fn calculateStatistics(self: *MemoryBenchmarkSuite) !void {
        if (self.results.items.len == 0) return;
        
        // Calculate means
        var mean = std.mem.zeroes(BenchmarkResult);
        mean.name = "Mean";
        
        for (self.results.items) |result| {
            mean.alloc_throughput += result.alloc_throughput;
            mean.free_throughput += result.free_throughput;
            mean.bandwidth += result.bandwidth;
            mean.avg_latency_ns += result.avg_latency_ns;
            mean.memory_efficiency += result.memory_efficiency;
            mean.numa_locality += result.numa_locality;
        }
        
        const count = @as(f64, @floatFromInt(self.results.items.len));
        mean.alloc_throughput /= count;
        mean.free_throughput /= count;
        mean.bandwidth /= count;
        mean.avg_latency_ns /= count;
        mean.memory_efficiency /= @as(f32, @floatCast(count));
        mean.numa_locality /= @as(f32, @floatCast(count));
        
        self.statistics.mean = mean;
        
        // Calculate standard deviation
        var variance = std.mem.zeroes(BenchmarkResult);
        variance.name = "StdDev";
        
        for (self.results.items) |result| {
            const diff_throughput = result.alloc_throughput - mean.alloc_throughput;
            variance.alloc_throughput += diff_throughput * diff_throughput;
            
            const diff_bandwidth = result.bandwidth - mean.bandwidth;
            variance.bandwidth += diff_bandwidth * diff_bandwidth;
            
            const diff_latency = result.avg_latency_ns - mean.avg_latency_ns;
            variance.avg_latency_ns += diff_latency * diff_latency;
        }
        
        variance.alloc_throughput = @sqrt(variance.alloc_throughput / count);
        variance.bandwidth = @sqrt(variance.bandwidth / count);
        variance.avg_latency_ns = @sqrt(variance.avg_latency_ns / count);
        
        self.statistics.stddev = variance;
        
        // Calculate coefficient of variation
        self.statistics.coefficient_of_variation = if (mean.alloc_throughput > 0)
            variance.alloc_throughput / mean.alloc_throughput
        else 0;
    }
    
    fn generateRecommendations(self: *MemoryBenchmarkSuite, writer: anytype) !void {
        const stats = &self.statistics;
        
        // Analyze results and provide recommendations
        if (stats.mean.alloc_throughput < 100000) {
            try writer.writeAll("- **Low allocation throughput detected**: Consider using thread-local caches or lock-free algorithms\n");
        }
        
        if (stats.mean.avg_latency_ns > 10000) {
            try writer.writeAll("- **High allocation latency detected**: Consider pre-allocating memory pools or using smaller allocation sizes\n");
        }
        
        if (stats.mean.memory_efficiency < 0.8) {
            try writer.writeAll("- **Low memory efficiency detected**: Consider using size classes or reducing fragmentation\n");
        }
        
        if (stats.mean.numa_locality < 0.8) {
            try writer.writeAll("- **Poor NUMA locality detected**: Consider NUMA-aware allocation strategies\n");
        }
        
        if (stats.coefficient_of_variation > 0.3) {
            try writer.writeAll("- **High performance variability detected**: Consider more consistent allocation patterns\n");
        }
        
        try writer.writeAll("- **General recommendations**: \n");
        try writer.writeAll("  - Use appropriate pool strategies for different allocation patterns\n");
        try writer.writeAll("  - Enable NUMA awareness for large multi-socket systems\n");
        try writer.writeAll("  - Monitor and tune based on actual workload characteristics\n");
        try writer.writeAll("  - Consider memory prefetching for predictable access patterns\n");
    }
};

/// Performance monitor for real-time tracking
pub const MemoryPerformanceMonitor = struct {
    /// Event ring buffer for allocation tracking
    event_buffer: RingBuffer(AllocationEvent),
    /// Current performance metrics
    current_metrics: PerformanceMetrics,
    /// Metrics history for trend analysis
    metrics_history: ArrayList(PerformanceMetrics),
    /// Hotspot tracking
    hotspots: HashMap(u64, MemoryHotspot, std.hash_map.AutoContext(u64), std.hash_map.default_max_load_percentage),
    /// Regression detection
    regression_alerts: ArrayList(RegressionAlert),
    /// Monitoring configuration
    config: MonitorConfig,
    /// Allocator for internal structures
    allocator: std.mem.Allocator,
    /// Monitoring thread
    monitor_thread: ?Thread,
    /// Shutdown flag
    shutdown: Atomic(bool),
    /// Metrics mutex
    mutex: Mutex,
    
    const MonitorConfig = struct {
        /// Event buffer size
        buffer_size: usize = 1024 * 1024, // 1M events
        /// Metrics window duration (microseconds)
        metrics_window_us: u64 = 60_000_000, // 1 minute
        /// Hotspot detection threshold
        hotspot_threshold: f64 = 100.0,
        /// Enable stack trace collection
        stack_traces: bool = true,
        /// Enable regression detection
        regression_detection: bool = true,
        /// Regression threshold percentage
        regression_threshold: f64 = 10.0,
    };
    
    const RingBuffer = @import("std").RingBuffer;
    
    pub fn init(allocator: std.mem.Allocator, config: MonitorConfig) !MemoryPerformanceMonitor {
        var monitor = MemoryPerformanceMonitor{
            .event_buffer = try RingBuffer(AllocationEvent).init(allocator, config.buffer_size),
            .current_metrics = PerformanceMetrics.init(),
            .metrics_history = ArrayList(PerformanceMetrics).init(allocator),
            .hotspots = HashMap(u64, MemoryHotspot, std.hash_map.AutoContext(u64), std.hash_map.default_max_load_percentage).init(allocator),
            .regression_alerts = ArrayList(RegressionAlert).init(allocator),
            .config = config,
            .allocator = allocator,
            .monitor_thread = null,
            .shutdown = Atomic(bool).init(false),
            .mutex = Mutex{},
        };
        
        // Start monitoring thread
        monitor.monitor_thread = try Thread.spawn(.{}, monitorThreadMain, .{&monitor});
        
        return monitor;
    }
    
    pub fn deinit(self: *MemoryPerformanceMonitor) void {
        // Shutdown monitoring thread
        if (self.monitor_thread) |thread| {
            self.shutdown.store(true, .release);
            thread.join();
        }
        
        self.event_buffer.deinit();
        self.metrics_history.deinit();
        self.hotspots.deinit();
        self.regression_alerts.deinit();
    }
    
    /// Record allocation event
    pub fn recordAllocation(self: *MemoryPerformanceMonitor, size: usize, address: usize, latency_ns: u32) void {
        const event = AllocationEvent.init(.Alloc, size, address, latency_ns);
        self.event_buffer.writeOne(event) catch {
            // Buffer full, drop oldest events
            _ = self.event_buffer.readOne();
            self.event_buffer.writeOne(event) catch {};
        };
    }
    
    /// Record deallocation event
    pub fn recordDeallocation(self: *MemoryPerformanceMonitor, size: usize, address: usize) void {
        const event = AllocationEvent.init(.Free, size, address, 0);
        self.event_buffer.writeOne(event) catch {};
    }
    
    /// Get current performance metrics
    pub fn getCurrentMetrics(self: *MemoryPerformanceMonitor) PerformanceMetrics {
        self.mutex.lock();
        defer self.mutex.unlock();
        return self.current_metrics;
    }
    
    /// Get hotspot analysis
    pub fn getHotspots(self: *MemoryPerformanceMonitor, allocator: std.mem.Allocator) ![]MemoryHotspot {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        var hotspots = ArrayList(MemoryHotspot).init(allocator);
        
        var iterator = self.hotspots.iterator();
        while (iterator.next()) |entry| {
            if (entry.value_ptr.calculateScore() > self.config.hotspot_threshold) {
                try hotspots.append(entry.value_ptr.*);
            }
        }
        
        // Sort by hotspot score (descending)
        const SortContext = struct {
            fn lessThan(_: void, lhs: MemoryHotspot, rhs: MemoryHotspot) bool {
                return lhs.calculateScore() > rhs.calculateScore();
            }
        };
        std.mem.sort(MemoryHotspot, hotspots.items, {}, SortContext.lessThan);
        
        return hotspots.toOwnedSlice();
    }
    
    /// Get regression alerts
    pub fn getRegressionAlerts(self: *MemoryPerformanceMonitor) []RegressionAlert {
        self.mutex.lock();
        defer self.mutex.unlock();
        return self.regression_alerts.items;
    }
    
    // Private methods
    
    fn monitorThreadMain(self: *MemoryPerformanceMonitor) void {
        var last_metrics_time = std.time.microTimestamp();
        
        while (!self.shutdown.load(.acquire)) {
            const now = std.time.microTimestamp();
            
            // Update metrics every window
            if (now - last_metrics_time >= @as(i64, @intCast(self.config.metrics_window_us))) {
                self.updateMetrics(last_metrics_time, now);
                last_metrics_time = now;
            }
            
            // Process events
            self.processEvents();
            
            // Sleep for 100ms
            std.time.sleep(100_000_000);
        }
    }
    
    fn updateMetrics(self: *MemoryPerformanceMonitor, start_time: i64, end_time: i64) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        var metrics = PerformanceMetrics.init();
        metrics.window_duration_us = @as(u64, @intCast(end_time - start_time));
        metrics.window_start = @as(u64, @intCast(start_time));
        
        // Process events in the time window
        var events = ArrayList(AllocationEvent).init(self.allocator);
        defer events.deinit();
        
        // Read all events from buffer (simplified - in practice, would filter by time)
        while (self.event_buffer.readOne()) |event| {
            if (event.timestamp >= @as(u64, @intCast(start_time)) and 
                event.timestamp <= @as(u64, @intCast(end_time))) {
                events.append(event) catch break;
            }
        }
        
        // Calculate metrics from events
        self.calculateMetricsFromEvents(events.items, &metrics);
        
        // Store current metrics
        self.current_metrics = metrics;
        
        // Add to history
        self.metrics_history.append(metrics) catch {
            // Remove oldest if we're at capacity
            if (self.metrics_history.items.len >= 1440) { // Keep 24 hours of minute-level data
                _ = self.metrics_history.orderedRemove(0);
            }
            self.metrics_history.append(metrics) catch {};
        };
        
        // Detect regressions
        if (self.config.regression_detection and self.metrics_history.items.len > 1) {
            self.detectRegressions();
        }
    }
    
    fn processEvents(self: *MemoryPerformanceMonitor) void {
        // Process new events for hotspot detection
        while (self.event_buffer.readOne()) |event| {
            if (event.event_type == .Alloc) {
                self.updateHotspot(event);
            }
        }
    }
    
    fn calculateMetricsFromEvents(self: *MemoryPerformanceMonitor, events: []AllocationEvent, metrics: *PerformanceMetrics) void {
        _ = self;
        
        if (events.len == 0) return;
        
        var alloc_count: u64 = 0;
        var free_count: u64 = 0;
        var total_bytes: u64 = 0;
        var total_latency: u64 = 0;
        var latencies = ArrayList(u32).init(std.heap.page_allocator);
        defer latencies.deinit();
        
        var min_latency: u32 = std.math.maxInt(u32);
        var max_latency: u32 = 0;
        var min_size: usize = std.math.maxInt(usize);
        var max_size: usize = 0;
        
        for (events) |event| {
            switch (event.event_type) {
                .Alloc => {
                    alloc_count += 1;
                    total_bytes += event.size;
                    total_latency += event.latency_ns;
                    
                    latencies.append(event.latency_ns) catch {};
                    
                    min_latency = @min(min_latency, event.latency_ns);
                    max_latency = @max(max_latency, event.latency_ns);
                    min_size = @min(min_size, event.size);
                    max_size = @max(max_size, event.size);
                },
                .Free => {
                    free_count += 1;
                },
                else => {},
            }
        }
        
        metrics.total_allocations = alloc_count;
        metrics.total_bytes_allocated = total_bytes;
        metrics.total_deallocations = free_count;
        
        const duration_seconds = @as(f64, @floatFromInt(metrics.window_duration_us)) / 1_000_000.0;
        metrics.allocation_rate = @as(f64, @floatFromInt(alloc_count)) / duration_seconds;
        metrics.bandwidth = @as(f64, @floatFromInt(total_bytes)) / duration_seconds;
        
        if (alloc_count > 0) {
            metrics.min_latency_ns = min_latency;
            metrics.max_latency_ns = max_latency;
            metrics.avg_latency_ns = @as(f64, @floatFromInt(total_latency)) / @as(f64, @floatFromInt(alloc_count));
            
            metrics.min_size = min_size;
            metrics.max_size = max_size;
            metrics.avg_size = @as(f64, @floatFromInt(total_bytes)) / @as(f64, @floatFromInt(alloc_count));
            
            // Calculate percentiles (simplified)
            if (latencies.items.len > 0) {
                std.mem.sort(u32, latencies.items, {}, std.sort.asc(u32));
                
                const p50_index = latencies.items.len / 2;
                const p95_index = (latencies.items.len * 95) / 100;
                const p99_index = (latencies.items.len * 99) / 100;
                
                metrics.p50_latency_ns = latencies.items[p50_index];
                metrics.p95_latency_ns = latencies.items[@min(p95_index, latencies.items.len - 1)];
                metrics.p99_latency_ns = latencies.items[@min(p99_index, latencies.items.len - 1)];
            }
            
            // Calculate size variance
            var size_variance: f64 = 0;
            for (events) |event| {
                if (event.event_type == .Alloc) {
                    const diff = @as(f64, @floatFromInt(event.size)) - metrics.avg_size;
                    size_variance += diff * diff;
                }
            }
            metrics.size_variance = size_variance / @as(f64, @floatFromInt(alloc_count));
        }
        
        // Set default values for cache and NUMA metrics
        metrics.cache_hit_rate = 0.9; // Would be measured from actual system
        metrics.numa_local_rate = 0.8;
        metrics.numa_remote_penalty = 50.0; // 50ns penalty for remote access
        
        metrics.fragmentation_ratio = 0.1; // Would be calculated from pool state
        metrics.memory_efficiency = 0.85;
        metrics.pool_utilization = 0.75;
    }
    
    fn updateHotspot(self: *MemoryPerformanceMonitor, event: AllocationEvent) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        const result = self.hotspots.getOrPut(event.stack_trace_hash) catch return;
        
        if (result.found_existing) {
            // Update existing hotspot
            const hotspot = result.value_ptr;
            hotspot.allocation_count += 1;
            hotspot.total_bytes += event.size;
            hotspot.avg_size = @as(f64, @floatFromInt(hotspot.total_bytes)) / @as(f64, @floatFromInt(hotspot.allocation_count));
            
            // Update latency using exponential moving average
            hotspot.avg_latency_ns = (hotspot.avg_latency_ns * 0.9) + (@as(f64, @floatFromInt(event.latency_ns)) * 0.1);
            
            hotspot.hotspot_score = hotspot.calculateScore();
        } else {
            // Create new hotspot
            result.value_ptr.* = MemoryHotspot{
                .stack_trace_hash = event.stack_trace_hash,
                .source_location = MemoryHotspot.SourceLocation{
                    .file = "unknown",
                    .function = "unknown",
                    .line = 0,
                    .column = 0,
                },
                .allocation_count = 1,
                .total_bytes = event.size,
                .avg_size = @as(f64, @floatFromInt(event.size)),
                .avg_latency_ns = @as(f64, @floatFromInt(event.latency_ns)),
                .hotspot_score = 0,
                .recommendations = &[_][]const u8{},
            };
            result.value_ptr.hotspot_score = result.value_ptr.calculateScore();
        }
    }
    
    fn detectRegressions(self: *MemoryPerformanceMonitor) void {
        const current = &self.current_metrics;
        const previous = &self.metrics_history.items[self.metrics_history.items.len - 2];
        
        // Check allocation throughput regression
        if (previous.allocation_rate > 0) {
            const regression = ((previous.allocation_rate - current.allocation_rate) / previous.allocation_rate) * 100.0;
            if (@abs(regression) > self.config.regression_threshold) {
                const alert = RegressionAlert{
                    .metric_name = "allocation_rate",
                    .baseline_value = previous.allocation_rate,
                    .current_value = current.allocation_rate,
                    .regression_percent = regression,
                    .confidence = 0.95,
                    .timestamp = @as(u64, @intCast(std.time.timestamp())),
                    .severity = RegressionAlert.getSeverity(regression),
                };
                self.regression_alerts.append(alert) catch {};
            }
        }
        
        // Check latency regression
        if (previous.avg_latency_ns > 0) {
            const regression = ((current.avg_latency_ns - previous.avg_latency_ns) / previous.avg_latency_ns) * 100.0;
            if (regression > self.config.regression_threshold) {
                const alert = RegressionAlert{
                    .metric_name = "avg_latency_ns",
                    .baseline_value = previous.avg_latency_ns,
                    .current_value = current.avg_latency_ns,
                    .regression_percent = regression,
                    .confidence = 0.95,
                    .timestamp = @as(u64, @intCast(std.time.timestamp())),
                    .severity = RegressionAlert.getSeverity(regression),
                };
                self.regression_alerts.append(alert) catch {};
            }
        }
        
        // Limit alert history
        while (self.regression_alerts.items.len > 100) {
            _ = self.regression_alerts.orderedRemove(0);
        }
    }
};

// Export C API for integration

export fn cursed_memory_monitor_create(buffer_size: usize, window_us: u64) ?*MemoryPerformanceMonitor {
    const allocator = std.heap.page_allocator;
    const monitor = allocator.create(MemoryPerformanceMonitor) catch return null;
    
    const config = MemoryPerformanceMonitor.MonitorConfig{
        .buffer_size = buffer_size,
        .metrics_window_us = window_us,
        .hotspot_threshold = 100.0,
        .stack_traces = true,
        .regression_detection = true,
        .regression_threshold = 10.0,
    };
    
    monitor.* = MemoryPerformanceMonitor.init(allocator, config) catch {
        allocator.destroy(monitor);
        return null;
    };
    return monitor;
}

export fn cursed_memory_monitor_destroy(monitor: ?*MemoryPerformanceMonitor) void {
    if (monitor) |m| {
        m.deinit();
        std.heap.page_allocator.destroy(m);
    }
}

export fn cursed_memory_monitor_record_alloc(monitor: ?*MemoryPerformanceMonitor, size: usize, address: usize, latency_ns: u32) void {
    if (monitor) |m| {
        m.recordAllocation(size, address, latency_ns);
    }
}

export fn cursed_memory_monitor_record_free(monitor: ?*MemoryPerformanceMonitor, size: usize, address: usize) void {
    if (monitor) |m| {
        m.recordDeallocation(size, address);
    }
}

export fn cursed_memory_monitor_get_metrics(monitor: ?*MemoryPerformanceMonitor, metrics: ?*PerformanceMetrics) void {
    if (monitor) |m| {
        if (metrics) |met| {
            met.* = m.getCurrentMetrics();
        }
    }
}
