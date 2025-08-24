// Performance Monitoring System - Detects and prevents performance regressions
const std = @import("std");
const HashMap = std.HashMap;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Performance metric types
pub const MetricType = enum {
    allocation_time,
    deallocation_time,
    lookup_time,
    string_operation_time,
    memory_usage,
    cache_hit_rate,
};

// Performance sample
pub const PerformanceSample = struct {
    timestamp: i64,
    value: f64,
    operation_name: []const u8,
    context: []const u8,
};

// Performance threshold configuration
pub const PerformanceThreshold = struct {
    metric_type: MetricType,
    operation_name: []const u8,
    warning_threshold: f64,
    error_threshold: f64,
    sample_window_size: u32,
    enabled: bool = true,
};

// Performance regression alert
pub const RegressionAlert = struct {
    metric_type: MetricType,
    operation_name: []const u8,
    current_value: f64,
    baseline_value: f64,
    regression_factor: f64,
    timestamp: i64,
    severity: enum { warning, error, critical },
};

// Main performance monitor
pub const PerformanceMonitor = struct {
    const Self = @This();
    
    // Sample storage by metric type and operation
    const SampleMap = HashMap([]const u8, ArrayList(PerformanceSample), std.hash_map.StringContext, std.hash_map.default_max_load_percentage);
    const ThresholdMap = HashMap([]const u8, PerformanceThreshold, std.hash_map.StringContext, std.hash_map.default_max_load_percentage);
    
    allocator: Allocator,
    samples: SampleMap,
    thresholds: ThresholdMap,
    alerts: ArrayList(RegressionAlert),
    
    // Configuration
    max_samples_per_metric: u32 = 10000,
    baseline_calculation_window: u32 = 1000,
    regression_detection_enabled: bool = true,
    
    // Statistics
    total_samples: u64 = 0,
    total_alerts: u64 = 0,
    monitoring_start_time: i64,
    
    pub fn init(allocator: Allocator) Self {
        return Self{
            .allocator = allocator,
            .samples = SampleMap.init(allocator),
            .thresholds = ThresholdMap.init(allocator),
            .alerts = ArrayList(RegressionAlert).init(allocator),
            .monitoring_start_time = std.time.nanoTimestamp(),
        };
    }
    
    pub fn deinit(self: *Self) void {
        // Clean up sample storage
        var sample_iterator = self.samples.iterator();
        while (sample_iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.samples.deinit();
        
        // Clean up thresholds
        self.thresholds.deinit();
        
        // Clean up alerts
        self.alerts.deinit();
    }
    
    // Record performance sample
    pub fn recordSample(self: *Self, metric_type: MetricType, operation_name: []const u8, value: f64, context: []const u8) !void {
        const key = try self.generateKey(metric_type, operation_name);
        defer self.allocator.free(key);
        
        const sample = PerformanceSample{
            .timestamp = std.time.nanoTimestamp(),
            .value = value,
            .operation_name = try self.allocator.dupe(u8, operation_name),
            .context = try self.allocator.dupe(u8, context),
        };
        
        // Get or create sample list for this metric
        var sample_list = self.samples.getPtr(key);
        if (sample_list == null) {
            var new_list = ArrayList(PerformanceSample).init(self.allocator);
            try new_list.ensureTotalCapacity(self.max_samples_per_metric);
            try self.samples.put(try self.allocator.dupe(u8, key), new_list);
            sample_list = self.samples.getPtr(key).?;
        }
        
        // Add sample (maintain circular buffer if at capacity)
        if (sample_list.items.len >= self.max_samples_per_metric) {
            // Remove oldest sample to make room
            const oldest = sample_list.orderedRemove(0);
            self.allocator.free(oldest.operation_name);
            self.allocator.free(oldest.context);
        }
        
        try sample_list.append(sample);
        self.total_samples += 1;
        
        // Check for regressions
        if (self.regression_detection_enabled) {
            try self.checkForRegressions(metric_type, operation_name, value);
        }
    }
    
    // Configure performance thresholds
    pub fn setThreshold(self: *Self, threshold: PerformanceThreshold) !void {
        const key = try self.generateKey(threshold.metric_type, threshold.operation_name);
        try self.thresholds.put(key, threshold);
    }
    
    // Check for performance regressions
    fn checkForRegressions(self: *Self, metric_type: MetricType, operation_name: []const u8, current_value: f64) !void {
        const key = try self.generateKey(metric_type, operation_name);
        defer self.allocator.free(key);
        
        // Get threshold configuration
        const threshold = self.thresholds.get(key);
        if (threshold == null or !threshold.?.enabled) {
            return;
        }
        
        // Calculate baseline from historical samples
        const baseline = try self.calculateBaseline(metric_type, operation_name);
        if (baseline == null) {
            return; // Not enough data yet
        }
        
        const baseline_value = baseline.?;
        const regression_factor = current_value / baseline_value;
        
        // Check thresholds
        var severity: ?@TypeOf(RegressionAlert.severity) = null;
        
        if (regression_factor > threshold.?.error_threshold) {
            severity = .critical;
        } else if (regression_factor > threshold.?.warning_threshold) {
            severity = .warning;
        }
        
        if (severity != null) {
            const alert = RegressionAlert{
                .metric_type = metric_type,
                .operation_name = try self.allocator.dupe(u8, operation_name),
                .current_value = current_value,
                .baseline_value = baseline_value,
                .regression_factor = regression_factor,
                .timestamp = std.time.nanoTimestamp(),
                .severity = severity.?,
            };
            
            try self.alerts.append(alert);
            self.total_alerts += 1;
            
            // Log alert
            self.logRegressionAlert(alert);
        }
    }
    
    // Calculate performance baseline
    fn calculateBaseline(self: *Self, metric_type: MetricType, operation_name: []const u8) !?f64 {
        const key = try self.generateKey(metric_type, operation_name);
        defer self.allocator.free(key);
        
        const sample_list = self.samples.get(key);
        if (sample_list == null or sample_list.?.items.len < self.baseline_calculation_window) {
            return null;
        }
        
        // Use the first N samples as baseline (historical performance)
        const baseline_samples = sample_list.?.items[0..@min(self.baseline_calculation_window, sample_list.?.items.len)];
        
        var sum: f64 = 0;
        for (baseline_samples) |sample| {
            sum += sample.value;
        }
        
        return sum / @as(f64, @floatFromInt(baseline_samples.len));
    }
    
    // Generate performance report
    pub fn generateReport(self: *Self) !void {
        std.debug.print("=== PERFORMANCE MONITORING REPORT ===\n");
        std.debug.print("Monitoring duration: {} seconds\n", .{(std.time.nanoTimestamp() - self.monitoring_start_time) / 1_000_000_000});
        std.debug.print("Total samples collected: {}\n", .{self.total_samples});
        std.debug.print("Total alerts generated: {}\n", .{self.total_alerts});
        std.debug.print("\n");
        
        // Report by metric type
        var sample_iterator = self.samples.iterator();
        while (sample_iterator.next()) |entry| {
            const key = entry.key_ptr.*;
            const samples = entry.value_ptr.*;
            
            if (samples.items.len == 0) continue;
            
            // Parse key to get metric info
            const parsed = try self.parseKey(key);
            
            std.debug.print("Metric: {} - {s}\n", .{ parsed.metric_type, parsed.operation_name });
            
            // Calculate statistics
            const stats = self.calculateStatistics(samples.items);
            std.debug.print("  Samples: {}\n", .{samples.items.len});
            std.debug.print("  Average: {d:.2} ns\n", .{stats.mean});
            std.debug.print("  Median: {d:.2} ns\n", .{stats.median});
            std.debug.print("  95th percentile: {d:.2} ns\n", .{stats.p95});
            std.debug.print("  99th percentile: {d:.2} ns\n", .{stats.p99});
            std.debug.print("  Min: {d:.2} ns\n", .{stats.min});
            std.debug.print("  Max: {d:.2} ns\n", .{stats.max});
            std.debug.print("  Std deviation: {d:.2} ns\n", .{stats.std_dev});
            
            // Check for recent performance trends
            const trend = self.calculateTrend(samples.items);
            if (trend > 1.1) {
                std.debug.print("  ⚠️  Performance declining (trend: {d:.2}x slower)\n", .{trend});
            } else if (trend < 0.9) {
                std.debug.print("  ✅ Performance improving (trend: {d:.2}x faster)\n", .{1.0 / trend});
            } else {
                std.debug.print("  📊 Performance stable\n");
            }
            
            std.debug.print("\n");
        }
        
        // Report recent alerts
        if (self.alerts.items.len > 0) {
            std.debug.print("=== RECENT PERFORMANCE ALERTS ===\n");
            const recent_alerts = self.alerts.items[0..@min(10, self.alerts.items.len)];
            
            for (recent_alerts) |alert| {
                const severity_emoji = switch (alert.severity) {
                    .warning => "⚠️ ",
                    .error => "❌",
                    .critical => "🚨",
                };
                
                std.debug.print("{s} {} - {s}: {d:.2} ns (was {d:.2} ns, {d:.2}x regression)\n", 
                    .{ severity_emoji, alert.metric_type, alert.operation_name, alert.current_value, alert.baseline_value, alert.regression_factor });
            }
            std.debug.print("\n");
        }
        
        std.debug.print("=== END PERFORMANCE REPORT ===\n");
    }
    
    // Performance statistics calculation
    const Statistics = struct {
        mean: f64,
        median: f64,
        p95: f64,
        p99: f64,
        min: f64,
        max: f64,
        std_dev: f64,
    };
    
    fn calculateStatistics(self: *Self, samples: []PerformanceSample) Statistics {
        if (samples.len == 0) {
            return Statistics{ .mean = 0, .median = 0, .p95 = 0, .p99 = 0, .min = 0, .max = 0, .std_dev = 0 };
        }
        
        // Extract values and sort for percentile calculations
        var values = self.allocator.alloc(f64, samples.len) catch unreachable;
        defer self.allocator.free(values);
        
        for (samples, 0..) |sample, i| {
            values[i] = sample.value;
        }
        
        std.mem.sort(f64, values, {}, std.math.order);
        
        // Calculate statistics
        var sum: f64 = 0;
        for (values) |value| {
            sum += value;
        }
        const mean = sum / @as(f64, @floatFromInt(values.len));
        
        const median = if (values.len % 2 == 0)
            (values[values.len / 2 - 1] + values[values.len / 2]) / 2.0
        else
            values[values.len / 2];
        
        const p95_idx = @min(values.len - 1, (values.len * 95) / 100);
        const p99_idx = @min(values.len - 1, (values.len * 99) / 100);
        
        const p95 = values[p95_idx];
        const p99 = values[p99_idx];
        const min = values[0];
        const max = values[values.len - 1];
        
        // Standard deviation
        var variance_sum: f64 = 0;
        for (values) |value| {
            const diff = value - mean;
            variance_sum += diff * diff;
        }
        const variance = variance_sum / @as(f64, @floatFromInt(values.len));
        const std_dev = @sqrt(variance);
        
        return Statistics{
            .mean = mean,
            .median = median,
            .p95 = p95,
            .p99 = p99,
            .min = min,
            .max = max,
            .std_dev = std_dev,
        };
    }
    
    // Calculate performance trend (recent vs historical)
    fn calculateTrend(self: *Self, samples: []PerformanceSample) f64 {
        if (samples.len < 20) return 1.0; // Not enough data
        
        const recent_size = samples.len / 4; // Recent 25%
        const historical_size = samples.len / 4; // Historical 25% (from beginning)
        
        if (recent_size == 0 or historical_size == 0) return 1.0;
        
        // Calculate averages
        var recent_sum: f64 = 0;
        for (samples[samples.len - recent_size..]) |sample| {
            recent_sum += sample.value;
        }
        const recent_avg = recent_sum / @as(f64, @floatFromInt(recent_size));
        
        var historical_sum: f64 = 0;
        for (samples[0..historical_size]) |sample| {
            historical_sum += sample.value;
        }
        const historical_avg = historical_sum / @as(f64, @floatFromInt(historical_size));
        
        return recent_avg / historical_avg;
    }
    
    // Utility functions
    fn generateKey(self: *Self, metric_type: MetricType, operation_name: []const u8) ![]u8 {
        return std.fmt.allocPrint(self.allocator, "{s}:{s}", .{ @tagName(metric_type), operation_name });
    }
    
    const ParsedKey = struct {
        metric_type: MetricType,
        operation_name: []const u8,
    };
    
    fn parseKey(self: *Self, key: []const u8) !ParsedKey {
        const colon_pos = std.mem.indexOf(u8, key, ":") orelse return error.InvalidKey;
        
        const metric_name = key[0..colon_pos];
        const operation_name = key[colon_pos + 1..];
        
        const metric_type = std.meta.stringToEnum(MetricType, metric_name) orelse return error.InvalidMetricType;
        
        return ParsedKey{
            .metric_type = metric_type,
            .operation_name = operation_name,
        };
    }
    
    fn logRegressionAlert(self: *Self, alert: RegressionAlert) void {
        _ = self;
        
        const severity_str = switch (alert.severity) {
            .warning => "WARNING",
            .error => "ERROR", 
            .critical => "CRITICAL",
        };
        
        std.debug.print("[PERFORMANCE {}] {} - {s}: Performance regression detected!\n", 
            .{ severity_str, alert.metric_type, alert.operation_name });
        std.debug.print("  Current: {d:.2} ns, Baseline: {d:.2} ns, Regression: {d:.2}x\n",
            .{ alert.current_value, alert.baseline_value, alert.regression_factor });
    }
};

// Convenience functions for common performance monitoring
pub const PerformanceTimer = struct {
    start_time: i64,
    monitor: *PerformanceMonitor,
    metric_type: MetricType,
    operation_name: []const u8,
    context: []const u8,
    
    pub fn start(monitor: *PerformanceMonitor, metric_type: MetricType, operation_name: []const u8, context: []const u8) PerformanceTimer {
        return PerformanceTimer{
            .start_time = std.time.nanoTimestamp(),
            .monitor = monitor,
            .metric_type = metric_type,
            .operation_name = operation_name,
            .context = context,
        };
    }
    
    pub fn end(self: *PerformanceTimer) void {
        const end_time = std.time.nanoTimestamp();
        const duration_ns = @as(f64, @floatFromInt(end_time - self.start_time));
        
        self.monitor.recordSample(self.metric_type, self.operation_name, duration_ns, self.context) catch |err| {
            std.debug.print("Failed to record performance sample: {}\n", .{err});
        };
    }
};

// Default performance thresholds for common operations
pub const DEFAULT_THRESHOLDS = [_]PerformanceThreshold{
    // Memory operations should be fast
    .{
        .metric_type = .allocation_time,
        .operation_name = "pool_allocation",
        .warning_threshold = 2.0, // 2x slower than baseline
        .error_threshold = 5.0,   // 5x slower than baseline
        .sample_window_size = 1000,
    },
    .{
        .metric_type = .deallocation_time,
        .operation_name = "pool_deallocation",
        .warning_threshold = 2.0,
        .error_threshold = 5.0,
        .sample_window_size = 1000,
    },
    // Lookup operations should be O(1)
    .{
        .metric_type = .lookup_time,
        .operation_name = "pool_lookup",
        .warning_threshold = 1.5, // Even small regressions matter for lookups
        .error_threshold = 3.0,
        .sample_window_size = 1000,
    },
    .{
        .metric_type = .lookup_time,
        .operation_name = "slab_lookup",
        .warning_threshold = 1.5,
        .error_threshold = 3.0,
        .sample_window_size = 1000,
    },
    // String operations
    .{
        .metric_type = .string_operation_time,
        .operation_name = "string_concatenation",
        .warning_threshold = 2.0,
        .error_threshold = 10.0, // String ops can vary more
        .sample_window_size = 500,
    },
    .{
        .metric_type = .string_operation_time,
        .operation_name = "string_cloning",
        .warning_threshold = 2.0,
        .error_threshold = 5.0,
        .sample_window_size = 500,
    },
    // Cache performance
    .{
        .metric_type = .cache_hit_rate,
        .operation_name = "string_intern_cache",
        .warning_threshold = 0.8, // Hit rate below 80% is concerning
        .error_threshold = 0.6,   // Hit rate below 60% is critical
        .sample_window_size = 1000,
    },
};

// Global performance monitor instance
var global_performance_monitor: ?*PerformanceMonitor = null;

pub fn initGlobalPerformanceMonitor(allocator: Allocator) !void {
    var monitor = try allocator.create(PerformanceMonitor);
    monitor.* = PerformanceMonitor.init(allocator);
    
    // Set default thresholds
    for (DEFAULT_THRESHOLDS) |threshold| {
        try monitor.setThreshold(threshold);
    }
    
    global_performance_monitor = monitor;
}

pub fn deinitGlobalPerformanceMonitor(allocator: Allocator) void {
    if (global_performance_monitor) |monitor| {
        monitor.deinit();
        allocator.destroy(monitor);
        global_performance_monitor = null;
    }
}

pub fn getGlobalPerformanceMonitor() *PerformanceMonitor {
    return global_performance_monitor.?;
}

// Macro-like functions for easy performance monitoring
pub fn recordAllocationTime(operation_name: []const u8, time_ns: f64, context: []const u8) void {
    if (global_performance_monitor) |monitor| {
        monitor.recordSample(.allocation_time, operation_name, time_ns, context) catch {};
    }
}

pub fn recordLookupTime(operation_name: []const u8, time_ns: f64, context: []const u8) void {
    if (global_performance_monitor) |monitor| {
        monitor.recordSample(.lookup_time, operation_name, time_ns, context) catch {};
    }
}

pub fn recordStringOperationTime(operation_name: []const u8, time_ns: f64, context: []const u8) void {
    if (global_performance_monitor) |monitor| {
        monitor.recordSample(.string_operation_time, operation_name, time_ns, context) catch {};
    }
}

pub fn recordCacheHitRate(operation_name: []const u8, hit_rate: f64, context: []const u8) void {
    if (global_performance_monitor) |monitor| {
        monitor.recordSample(.cache_hit_rate, operation_name, hit_rate, context) catch {};
    }
}
