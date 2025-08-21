//! Runtime performance monitoring hooks for CURSED language
//!
//! Provides comprehensive performance monitoring including:
//! - Function call timing and profiling
//! - Memory allocation and deallocation tracking
//! - Goroutine lifecycle monitoring
//! - Channel operation performance
//! - Error and panic tracking
//! - Resource usage monitoring
//! - Stack walking and call trace analysis
//! - Hot path detection and bottleneck analysis

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const Timer = std.time.Timer;
const Instant = std.time.Instant;
const Thread = std.Thread;
const Mutex = std.Thread.Mutex;
const Atomic = std.atomic.Value;
const Order = std.atomic.Ordering;
const concurrency = @import("concurrency.zig");
const gc = @import("gc.zig");

/// Performance hooks configuration
pub const PerformanceHooksConfig = struct {
    enable_function_timing: bool = true,
    enable_memory_tracking: bool = true,
    enable_goroutine_monitoring: bool = true,
    enable_channel_monitoring: bool = true,
    enable_error_tracking: bool = true,
    enable_resource_monitoring: bool = true,
    enable_hot_path_detection: bool = true,
    enable_bottleneck_analysis: bool = true,
    enable_stack_walking: bool = true,
    sampling_rate: f64 = 0.1, // 10% sampling for production
    max_call_stack_depth: usize = 100,
    metrics_buffer_size: usize = 10000,
    flush_interval_ms: u64 = 1000,
    
    pub fn production() PerformanceHooksConfig {
        return PerformanceHooksConfig{
            .sampling_rate = 0.05, // 5% sampling for production
            .enable_stack_walking = false, // Disable expensive stack walking
        };
    }
    
    pub fn development() PerformanceHooksConfig {
        return PerformanceHooksConfig{
            .sampling_rate = 1.0, // 100% sampling for development
            .enable_stack_walking = true,
        };
    }
};

/// Function call performance data
pub const FunctionCallData = struct {
    function_name: []const u8,
    module_name: []const u8,
    start_time: u64, // nanoseconds since start
    duration: u64, // nanoseconds
    memory_allocated: usize,
    memory_deallocated: usize,
    call_stack_depth: usize,
    thread_id: u64,
    goroutine_id: ?u64,
    arguments_count: usize,
    return_value_size: usize,
    error_occurred: bool,
    cpu_time_ns: u64,
    stack_trace: ?[][]const u8, // Optional stack trace
    
    pub fn deinit(self: *FunctionCallData, allocator: Allocator) void {
        if (self.stack_trace) |stack| {
            for (stack) |frame| {
                allocator.free(frame);
            }
            allocator.free(stack);
        }
    }
};

/// Memory allocation event types
pub const MemoryEventType = enum {
    allocation,
    deallocation,
    reallocation,
    garbage_collection,
    out_of_memory,
};

/// Memory allocation event
pub const MemoryEvent = struct {
    timestamp: u64,
    event_type: MemoryEventType,
    size_bytes: usize,
    alignment: usize,
    thread_id: u64,
    goroutine_id: ?u64,
    allocation_site: []const u8,
    stack_trace: ?[][]const u8,
    
    pub fn deinit(self: *MemoryEvent, allocator: Allocator) void {
        if (self.stack_trace) |stack| {
            for (stack) |frame| {
                allocator.free(frame);
            }
            allocator.free(stack);
        }
    }
};

/// Goroutine lifecycle events
pub const GoroutineEventType = enum {
    created,
    started,
    yielded,
    resumed,
    completed,
    panicked,
    blocked,
    unblocked,
};

pub const GoroutineEvent = struct {
    timestamp: u64,
    event_type: GoroutineEventType,
    goroutine_id: u64,
    worker_id: usize,
    parent_goroutine_id: ?u64,
    stack_size: usize,
    priority: concurrency.GoroutinePriority,
    cpu_time_ns: u64,
};

/// Channel operation events
pub const ChannelEventType = enum {
    created,
    send,
    receive,
    closed,
    blocked_send,
    blocked_receive,
    select_case,
};

pub const ChannelEvent = struct {
    timestamp: u64,
    event_type: ChannelEventType,
    channel_id: u64,
    goroutine_id: u64,
    data_size: usize,
    queue_length: usize,
    operation_duration: u64,
};

/// Error and panic events
pub const ErrorEventType = enum {
    runtime_error,
    panic,
    recovered_panic,
    type_error,
    memory_error,
    channel_error,
    goroutine_error,
    system_error,
};

pub const ErrorEvent = struct {
    timestamp: u64,
    event_type: ErrorEventType,
    error_message: []const u8,
    location: []const u8,
    thread_id: u64,
    goroutine_id: ?u64,
    stack_trace: ?[][]const u8,
    
    pub fn deinit(self: *ErrorEvent, allocator: Allocator) void {
        if (self.stack_trace) |stack| {
            for (stack) |frame| {
                allocator.free(frame);
            }
            allocator.free(stack);
        }
    }
};

/// Resource usage snapshot
pub const ResourceSnapshot = struct {
    timestamp: u64,
    cpu_usage_percent: f64,
    memory_usage_bytes: usize,
    heap_usage_bytes: usize,
    stack_usage_bytes: usize,
    open_files: usize,
    network_connections: usize,
    threads_count: usize,
    goroutines_count: usize,
    channels_count: usize,
    gc_pressure: f64,
    load_average: f64,
};

/// Hot path analysis data
pub const HotPathData = struct {
    function_name: []const u8,
    total_calls: u64,
    total_time: u64, // nanoseconds
    average_time: u64, // nanoseconds
    min_time: u64,
    max_time: u64,
    p95_time: u64,
    p99_time: u64,
    call_frequency: f64, // calls per second
    cpu_percentage: f64,
    memory_pressure: f64,
    
    pub fn updateStats(self: *HotPathData, duration: u64) void {
        self.total_calls += 1;
        self.total_time += duration;
        self.average_time = self.total_time / self.total_calls;
        self.min_time = @min(self.min_time, duration);
        self.max_time = @max(self.max_time, duration);
        
        // TODO: Implement percentile tracking with histogram
    }
};

/// Performance bottleneck types
pub const BottleneckType = enum {
    cpu_bound,
    memory_bound,
    io_bound,
    network_bound,
    lock_contention,
    garbage_collection,
    cache_miss,
    algorithmic_complexity,
};

/// Bottleneck severity levels
pub const BottleneckSeverity = enum {
    minor,
    moderate,
    major,
    critical,
};

/// Performance bottleneck data
pub const BottleneckData = struct {
    bottleneck_type: BottleneckType,
    location: []const u8,
    severity: BottleneckSeverity,
    impact_score: f64,
    description: []const u8,
    suggested_fix: []const u8,
    measured_at: u64,
    affected_functions: [][]const u8,
    
    pub fn deinit(self: *BottleneckData, allocator: Allocator) void {
        for (self.affected_functions) |func| {
            allocator.free(func);
        }
        allocator.free(self.affected_functions);
    }
};

/// Aggregated performance metrics
pub const PerformanceMetrics = struct {
    timestamp: u64,
    total_function_calls: u64,
    total_memory_allocations: u64,
    total_goroutines_created: u64,
    total_channel_operations: u64,
    total_errors: u64,
    average_function_time: u64,
    memory_allocation_rate: f64,
    goroutine_creation_rate: f64,
    channel_operation_rate: f64,
    error_rate: f64,
    hot_paths: []HotPathData,
    bottlenecks: []BottleneckData,
    resource_usage: ResourceSnapshot,
    
    pub fn deinit(self: *PerformanceMetrics, allocator: Allocator) void {
        allocator.free(self.hot_paths);
        for (self.bottlenecks) |*bottleneck| {
            bottleneck.deinit(allocator);
        }
        allocator.free(self.bottlenecks);
    }
};

/// Performance hooks statistics
pub const PerformanceHooksStats = struct {
    total_function_calls: u64,
    total_memory_allocations: u64,
    total_goroutines: u64,
    total_channel_operations: u64,
    total_errors: u64,
    uptime_ns: u64,
    is_active: bool,
    sampling_rate: f64,
};

/// Stack frame information
pub const StackFrame = struct {
    function_name: []const u8,
    file_name: []const u8,
    line_number: u32,
    column_number: u32,
    address: usize,
    
    pub fn deinit(self: *StackFrame, allocator: Allocator) void {
        allocator.free(self.function_name);
        allocator.free(self.file_name);
    }
};

/// Stack walker for collecting call traces
pub const StackWalker = struct {
    allocator: Allocator,
    max_depth: usize,
    
    pub fn init(allocator: Allocator, max_depth: usize) StackWalker {
        return StackWalker{
            .allocator = allocator,
            .max_depth = max_depth,
        };
    }
    
    /// Walk the current call stack and return stack frames
    pub fn walkStack(self: *StackWalker) ![]StackFrame {
        var frames = .empty;
        defer frames.deinit(allocator);
        
        // Use builtin stack trace functionality
        var stack_trace = std.debug.StackTrace{};
        std.debug.captureStackTrace(null, &stack_trace);
        
        var i: usize = 0;
        while (i < stack_trace.index and i < self.max_depth) : (i += 1) {
            const addr = stack_trace.instruction_addresses[i];
            
            // Try to get symbol information
            const frame = self.addressToFrame(addr) catch StackFrame{
                .function_name = try self.allocator.dupe(u8, "<unknown>"),
                .file_name = try self.allocator.dupe(u8, "<unknown>"),
                .line_number = 0,
                .column_number = 0,
                .address = addr,
            };
            
            try frames.append(self.allocator, frame);
        }
        
        return frames.toOwnedSlice(self.allocator);
    }
    
    /// Convert address to stack frame information
    fn addressToFrame(self: *StackWalker, addr: usize) !StackFrame {
        // Use debug info to resolve symbol if available
        if (std.debug.getSelfDebugInfo()) |debug_info| {
            if (debug_info.getSymbolAtAddress(self.allocator, addr)) |symbol| {
                defer symbol.deinit(allocator);
                
                return StackFrame{
                    .function_name = try self.allocator.dupe(u8, symbol.symbol_name orelse "<unknown>"),
                    .file_name = try self.allocator.dupe(u8, symbol.file_name orelse "<unknown>"),
                    .line_number = symbol.line_info.?.line,
                    .column_number = symbol.line_info.?.column,
                    .address = addr,
                };
            } else |_| {
                // Fallback if symbol resolution fails
            }
        } else |_| {
            // No debug info available
        }
        
        return StackFrame{
            .function_name = try self.allocator.dupe(u8, "<unknown>"),
            .file_name = try self.allocator.dupe(u8, "<unknown>"),
            .line_number = 0,
            .column_number = 0,
            .address = addr,
        };
    }
    
    /// Convert stack frames to string array for storage
    pub fn framesToStrings(self: *StackWalker, frames: []StackFrame) ![][]const u8 {
        var strings = .empty;
        defer strings.deinit(allocator);
        
        for (frames) |frame| {
            const frame_str = try std.fmt.allocPrint(
                self.allocator,
                "{s} at {s}:{}:{}",
                .{ frame.function_name, frame.file_name, frame.line_number, frame.column_number }
            );
            try strings.append(self.allocator, frame_str);
        }
        
        return strings.toOwnedSlice(self.allocator);
    }
};

/// Main performance hooks manager
pub const PerformanceHooks = struct {
    allocator: Allocator,
    config: PerformanceHooksConfig,
    
    // Data storage
    function_calls: ArrayList(FunctionCallData),
    memory_events: ArrayList(MemoryEvent),
    goroutine_events: ArrayList(GoroutineEvent),
    channel_events: ArrayList(ChannelEvent),
    error_events: ArrayList(ErrorEvent),
    resource_snapshots: ArrayList(ResourceSnapshot),
    
    // Analysis data
    hot_paths: HashMap([]const u8, HotPathData, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    bottlenecks: ArrayList(BottleneckData),
    
    // Monitoring state
    active: Atomic(bool),
    monitor_thread: ?Thread,
    
    // Performance counters
    function_call_counter: Atomic(u64),
    memory_allocation_counter: Atomic(u64),
    goroutine_counter: Atomic(u64),
    channel_operation_counter: Atomic(u64),
    error_counter: Atomic(u64),
    
    // Timing
    start_time: Instant,
    last_flush: Atomic(u64),
    
    // Stack walker
    stack_walker: StackWalker,
    
    // Thread safety
    mutex: Mutex,
    
    pub fn init(allocator: Allocator, config: PerformanceHooksConfig) !*PerformanceHooks {
        const hooks = try allocator.create(PerformanceHooks);
        hooks.* = PerformanceHooks{
            .allocator = allocator,
            .config = config,
            .function_calls = .empty,
            .memory_events = .empty,
            .goroutine_events = .empty,
            .channel_events = .empty,
            .error_events = .empty,
            .resource_snapshots = .empty,
            .hot_paths = HashMap([]const u8, HotPathData, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .bottlenecks = .empty,
            .active = Atomic(bool).init(false),
            .monitor_thread = null,
            .function_call_counter = Atomic(u64).init(0),
            .memory_allocation_counter = Atomic(u64).init(0),
            .goroutine_counter = Atomic(u64).init(0),
            .channel_operation_counter = Atomic(u64).init(0),
            .error_counter = Atomic(u64).init(0),
            .start_time = try Instant.now(),
            .last_flush = Atomic(u64).init(0),
            .stack_walker = StackWalker.init(allocator, config.max_call_stack_depth),
            .mutex = Mutex{},
        };
        
        return hooks;
    }
    
    pub fn deinit(self: *PerformanceHooks) void {
        self.stop();
        
        // Clean up data structures
        for (self.function_calls.items) |*call| {
            call.deinit(allocator);
        }
        self.function_calls.deinit(allocator);
        
        for (self.memory_events.items) |*event| {
            event.deinit(allocator);
        }
        self.memory_events.deinit(allocator);
        
        self.goroutine_events.deinit(allocator);
        self.channel_events.deinit(allocator);
        
        for (self.error_events.items) |*event| {
            event.deinit(allocator);
        }
        self.error_events.deinit(allocator);
        
        self.resource_snapshots.deinit(allocator);
        
        // Clean up hot paths
        var iterator = self.hot_paths.iterator();
        while (iterator.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
        }
        self.hot_paths.deinit(allocator);
        
        for (self.bottlenecks.items) |*bottleneck| {
            bottleneck.deinit(allocator);
        }
        self.bottlenecks.deinit(allocator);
        
        self.allocator.destroy(self);
    }
    
    /// Start performance monitoring
    pub fn start(self: *PerformanceHooks) !void {
        self.active.store(true, .seq_cst);
        
        // Start monitoring thread
        if (self.config.enable_resource_monitoring) {
            self.monitor_thread = try Thread.spawn(.{}, monitoringThread, .{self});
        }
        
        print("Performance hooks started with {d:.1}% sampling rate\n", .{self.config.sampling_rate * 100});
    }
    
    /// Stop performance monitoring
    pub fn stop(self: *PerformanceHooks) void {
        self.active.store(false, .seq_cst);
        
        if (self.monitor_thread) |thread| {
            thread.join();
            self.monitor_thread = null;
        }
        
        print("Performance hooks stopped\n");
    }
    
    /// Check if we should sample this event based on sampling rate
    fn shouldSample(self: *PerformanceHooks) bool {
        if (self.config.sampling_rate >= 1.0) return true;
        if (self.config.sampling_rate <= 0.0) return false;
        
        // Use thread ID for consistent sampling
        const thread_id = self.getCurrentThreadId();
        const threshold = @as(u64, @intFromFloat(self.config.sampling_rate * @as(f64, @floatFromInt(std.math.maxInt(u64)))));
        return (thread_id % std.math.maxInt(u64)) < threshold;
    }
    
    /// Get current thread ID
    fn getCurrentThreadId(self: *PerformanceHooks) u64 {
        _ = self;
        // Use a simple hash of the thread handle
        const thread_id = Thread.getCurrentId();
        var hasher = std.hash_map.DefaultHasher{};
        hasher.update(std.mem.asBytes(&thread_id));
        return hasher.final();
    }
    
    /// Get current time in nanoseconds since start
    fn getCurrentTime(self: *PerformanceHooks) u64 {
        return self.start_time.since();
    }
    
    /// Record function call start
    pub fn functionCallStart(self: *PerformanceHooks, function_name: []const u8, module_name: []const u8) ?u64 {
        _ = function_name; // TODO: Use for detailed tracking
        _ = module_name; // TODO: Use for detailed tracking
        if (!self.config.enable_function_timing or !self.shouldSample()) return null;
        
        _ = self.function_call_counter.fetchAdd(1, .seq_cst);
        return self.getCurrentTime();
    }
    
    /// Record function call end
    pub fn functionCallEnd(
        self: *PerformanceHooks,
        start_time_opt: ?u64,
        function_name: []const u8,
        module_name: []const u8,
        args_count: usize,
        return_size: usize,
        error_occurred: bool,
    ) void {
        const start_time = start_time_opt orelse return;
        const end_time = self.getCurrentTime();
        const duration = end_time - start_time;
        
        self.mutex.lock();
        defer self.mutex.unlock();
        
        // Create function call data
        var stack_trace: ?[][]const u8 = null;
        if (self.config.enable_stack_walking) {
            if (self.stack_walker.walkStack()) |frames| {
                defer {
                    for (frames) |*frame| frame.deinit(allocator);
                    self.allocator.free(frames);
                }
                stack_trace = self.stack_walker.framesToStrings(frames) catch null;
            } else |_| {}
        }
        
        const call_data = FunctionCallData{
            .function_name = function_name,
            .module_name = module_name,
            .start_time = start_time,
            .duration = duration,
            .memory_allocated = 0, // TODO: Track from memory hooks
            .memory_deallocated = 0,
            .call_stack_depth = if (stack_trace) |st| st.len else 0,
            .thread_id = self.getCurrentThreadId(),
            .goroutine_id = self.getCurrentGoroutineId(),
            .arguments_count = args_count,
            .return_value_size = return_size,
            .error_occurred = error_occurred,
            .cpu_time_ns = duration, // Approximate
            .stack_trace = stack_trace,
        };
        
        self.function_calls.append(allocator, call_data) catch {};
        
        // Update hot paths
        if (self.config.enable_hot_path_detection) {
            self.updateHotPath(function_name, duration);
        }
        
        // Trim buffer if needed
        if (self.function_calls.items.len > self.config.metrics_buffer_size) {
            var old_call = self.function_calls.orderedRemove(0);
            old_call.deinit(allocator);
        }
    }
    
    /// Record memory allocation
    pub fn memoryAllocated(self: *PerformanceHooks, size: usize, alignment: usize, allocation_site: []const u8) void {
        if (!self.config.enable_memory_tracking or !self.shouldSample()) return;
        
        _ = self.memory_allocation_counter.fetchAdd(1, .seq_cst);
        
        self.mutex.lock();
        defer self.mutex.unlock();
        
        var stack_trace: ?[][]const u8 = null;
        if (self.config.enable_stack_walking) {
            if (self.stack_walker.walkStack()) |frames| {
                defer {
                    for (frames) |*frame| frame.deinit(allocator);
                    self.allocator.free(frames);
                }
                stack_trace = self.stack_walker.framesToStrings(frames) catch null;
            } else |_| {}
        }
        
        const event = MemoryEvent{
            .timestamp = self.getCurrentTime(),
            .event_type = .allocation,
            .size_bytes = size,
            .alignment = alignment,
            .thread_id = self.getCurrentThreadId(),
            .goroutine_id = self.getCurrentGoroutineId(),
            .allocation_site = allocation_site,
            .stack_trace = stack_trace,
        };
        
        self.memory_events.append(self.allocator, event) catch {};
        
        // Trim buffer if needed
        if (self.memory_events.items.len > self.config.metrics_buffer_size) {
            var old_event = self.memory_events.orderedRemove(0);
            old_event.deinit(allocator);
        }
    }
    
    /// Record memory deallocation
    pub fn memoryDeallocated(self: *PerformanceHooks, size: usize) void {
        if (!self.config.enable_memory_tracking or !self.shouldSample()) return;
        
        self.mutex.lock();
        defer self.mutex.unlock();
        
        const event = MemoryEvent{
            .timestamp = self.getCurrentTime(),
            .event_type = .deallocation,
            .size_bytes = size,
            .alignment = 0,
            .thread_id = self.getCurrentThreadId(),
            .goroutine_id = self.getCurrentGoroutineId(),
            .allocation_site = "<deallocation>",
            .stack_trace = null,
        };
        
        self.memory_events.append(self.allocator, event) catch {};
        
        // Trim buffer if needed
        if (self.memory_events.items.len > self.config.metrics_buffer_size) {
            var old_event = self.memory_events.orderedRemove(0);
            old_event.deinit(allocator);
        }
    }
    
    /// Record goroutine event
    pub fn goroutineEvent(
        self: *PerformanceHooks,
        event_type: GoroutineEventType,
        goroutine_id: u64,
        worker_id: usize,
        parent_id: ?u64,
        stack_size: usize,
        priority: concurrency.GoroutinePriority,
    ) void {
        if (!self.config.enable_goroutine_monitoring or !self.shouldSample()) return;
        
        if (event_type == .created) {
            _ = self.goroutine_counter.fetchAdd(1, .seq_cst);
        }
        
        self.mutex.lock();
        defer self.mutex.unlock();
        
        const event = GoroutineEvent{
            .timestamp = self.getCurrentTime(),
            .event_type = event_type,
            .goroutine_id = goroutine_id,
            .worker_id = worker_id,
            .parent_goroutine_id = parent_id,
            .stack_size = stack_size,
            .priority = priority,
            .cpu_time_ns = 0, // TODO: Track CPU time per goroutine
        };
        
        self.goroutine_events.append(allocator, event) catch {};
        
        // Trim buffer if needed
        if (self.goroutine_events.items.len > self.config.metrics_buffer_size) {
            _ = self.goroutine_events.orderedRemove(0);
        }
    }
    
    /// Record channel operation
    pub fn channelOperation(
        self: *PerformanceHooks,
        event_type: ChannelEventType,
        channel_id: u64,
        goroutine_id: u64,
        data_size: usize,
        queue_length: usize,
        duration: u64,
    ) void {
        if (!self.config.enable_channel_monitoring or !self.shouldSample()) return;
        
        _ = self.channel_operation_counter.fetchAdd(1, .seq_cst);
        
        self.mutex.lock();
        defer self.mutex.unlock();
        
        const event = ChannelEvent{
            .timestamp = self.getCurrentTime(),
            .event_type = event_type,
            .channel_id = channel_id,
            .goroutine_id = goroutine_id,
            .data_size = data_size,
            .queue_length = queue_length,
            .operation_duration = duration,
        };
        
        self.channel_events.append(allocator, event) catch {};
        
        // Trim buffer if needed
        if (self.channel_events.items.len > self.config.metrics_buffer_size) {
            _ = self.channel_events.orderedRemove(0);
        }
    }
    
    /// Record error event
    pub fn errorOccurred(
        self: *PerformanceHooks,
        event_type: ErrorEventType,
        error_message: []const u8,
        location: []const u8,
    ) void {
        if (!self.config.enable_error_tracking) return;
        
        _ = self.error_counter.fetchAdd(1, .seq_cst);
        
        self.mutex.lock();
        defer self.mutex.unlock();
        
        var stack_trace: ?[][]const u8 = null;
        if (self.config.enable_stack_walking) {
            if (self.stack_walker.walkStack()) |frames| {
                defer {
                    for (frames) |*frame| frame.deinit(allocator);
                    self.allocator.free(frames);
                }
                stack_trace = self.stack_walker.framesToStrings(frames) catch null;
            } else |_| {}
        }
        
        const event = ErrorEvent{
            .timestamp = self.getCurrentTime(),
            .event_type = event_type,
            .error_message = error_message,
            .location = location,
            .thread_id = self.getCurrentThreadId(),
            .goroutine_id = self.getCurrentGoroutineId(),
            .stack_trace = stack_trace,
        };
        
        self.error_events.append(self.allocator, event) catch {};
        
        // Trim buffer if needed
        if (self.error_events.items.len > self.config.metrics_buffer_size) {
            var old_event = self.error_events.orderedRemove(0);
            old_event.deinit(allocator);
        }
    }
    
    /// Update hot path data
    fn updateHotPath(self: *PerformanceHooks, function_name: []const u8, duration: u64) void {
        const key = self.allocator.dupe(u8, function_name) catch return;
        
        if (self.hot_paths.getPtr(key)) |hot_path| {
            hot_path.updateStats(duration);
            
            // Update frequency
            const elapsed_seconds = @as(f64, @floatFromInt(self.getCurrentTime())) / 1_000_000_000.0;
            if (elapsed_seconds > 0.0) {
                hot_path.call_frequency = @as(f64, @floatFromInt(hot_path.total_calls)) / elapsed_seconds;
            }
        } else {
            const hot_path = HotPathData{
                .function_name = key,
                .total_calls = 1,
                .total_time = duration,
                .average_time = duration,
                .min_time = duration,
                .max_time = duration,
                .p95_time = duration,
                .p99_time = duration,
                .call_frequency = 0.0,
                .cpu_percentage = 0.0,
                .memory_pressure = 0.0,
            };
            self.hot_paths.put(key, hot_path) catch {
                self.allocator.free(key);
            };
        }
    }
    
    /// Get current goroutine ID (placeholder - integrate with actual concurrency system)
    fn getCurrentGoroutineId(self: *PerformanceHooks) ?u64 {
        _ = self;
        // TODO: Integrate with actual goroutine system
        return null;
    }
    
    /// Collect resource usage snapshot
    fn collectResourceSnapshot(self: *PerformanceHooks) ResourceSnapshot {
        return ResourceSnapshot{
            .timestamp = self.getCurrentTime(),
            .cpu_usage_percent = self.getCpuUsage(),
            .memory_usage_bytes = self.getMemoryUsage(),
            .heap_usage_bytes = self.getHeapUsage(),
            .stack_usage_bytes = self.getStackUsage(),
            .open_files = self.getOpenFiles(),
            .network_connections = self.getNetworkConnections(),
            .threads_count = self.getThreadsCount(),
            .goroutines_count = self.getGoroutinesCount(),
            .channels_count = self.getChannelsCount(),
            .gc_pressure = self.getGcPressure(),
            .load_average = self.getLoadAverage(),
        };
    }
    
    /// Get current performance statistics
    pub fn getStats(self: *PerformanceHooks) PerformanceHooksStats {
        return PerformanceHooksStats{
            .total_function_calls = self.function_call_counter.load(.seq_cst),
            .total_memory_allocations = self.memory_allocation_counter.load(.seq_cst),
            .total_goroutines = self.goroutine_counter.load(.seq_cst),
            .total_channel_operations = self.channel_operation_counter.load(.seq_cst),
            .total_errors = self.error_counter.load(.seq_cst),
            .uptime_ns = self.getCurrentTime(),
            .is_active = self.active.load(.seq_cst),
            .sampling_rate = self.config.sampling_rate,
        };
    }
    
    /// Get current performance metrics
    pub fn getCurrentMetrics(self: *PerformanceHooks) !PerformanceMetrics {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        // Collect hot paths
        var hot_paths = .empty;
        var iterator = self.hot_paths.iterator();
        while (iterator.next()) |entry| {
            try hot_paths.append(allocator, entry.value_ptr.*);
        }
        
        // Detect bottlenecks
        try self.detectBottlenecks();
        
        const resource_usage = self.collectResourceSnapshot();
        const uptime_seconds = @as(f64, @floatFromInt(self.getCurrentTime())) / 1_000_000_000.0;
        
        return PerformanceMetrics{
            .timestamp = self.getCurrentTime(),
            .total_function_calls = self.function_call_counter.load(.seq_cst),
            .total_memory_allocations = self.memory_allocation_counter.load(.seq_cst),
            .total_goroutines_created = self.goroutine_counter.load(.seq_cst),
            .total_channel_operations = self.channel_operation_counter.load(.seq_cst),
            .total_errors = self.error_counter.load(.seq_cst),
            .average_function_time = if (self.function_calls.items.len > 0) blk: {
                var total: u64 = 0;
                for (self.function_calls.items) |call| {
                    total += call.duration;
                }
                break :blk total / self.function_calls.items.len;
            } else 0,
            .memory_allocation_rate = if (uptime_seconds > 0) @as(f64, @floatFromInt(self.memory_allocation_counter.load(.seq_cst))) / uptime_seconds else 0.0,
            .goroutine_creation_rate = if (uptime_seconds > 0) @as(f64, @floatFromInt(self.goroutine_counter.load(.seq_cst))) / uptime_seconds else 0.0,
            .channel_operation_rate = if (uptime_seconds > 0) @as(f64, @floatFromInt(self.channel_operation_counter.load(.seq_cst))) / uptime_seconds else 0.0,
            .error_rate = if (uptime_seconds > 0) @as(f64, @floatFromInt(self.error_counter.load(.seq_cst))) / uptime_seconds else 0.0,
            .hot_paths = try hot_paths.toOwnedSlice(allocator),
            .bottlenecks = try self.bottlenecks.clone(),
            .resource_usage = resource_usage,
        };
    }
    
    /// Print performance report
    pub fn printReport(self: *PerformanceHooks) void {
        const stats = self.getStats();
        const uptime_seconds = @as(f64, @floatFromInt(stats.uptime_ns)) / 1_000_000_000.0;
        
        print("\n=== CURSED PERFORMANCE REPORT ===\n");
        print("Status: {s}\n", .{if (stats.is_active) "Active" else "Inactive"});
        print("Uptime: {d:.2}s\n", .{uptime_seconds});
        print("Sampling Rate: {d:.1}%\n", .{stats.sampling_rate * 100});
        print("\n--- Counters ---\n");
        print("Function Calls: {d}\n", .{stats.total_function_calls});
        print("Memory Allocations: {d}\n", .{stats.total_memory_allocations});
        print("Goroutines Created: {d}\n", .{stats.total_goroutines});
        print("Channel Operations: {d}\n", .{stats.total_channel_operations});
        print("Errors: {d}\n", .{stats.total_errors});
        
        if (uptime_seconds > 0) {
            print("\n--- Rates (per second) ---\n");
            print("Function Calls: {d:.1}\n", .{@as(f64, @floatFromInt(stats.total_function_calls)) / uptime_seconds});
            print("Memory Allocations: {d:.1}\n", .{@as(f64, @floatFromInt(stats.total_memory_allocations)) / uptime_seconds});
            print("Goroutines: {d:.1}\n", .{@as(f64, @floatFromInt(stats.total_goroutines)) / uptime_seconds});
            print("Channel Operations: {d:.1}\n", .{@as(f64, @floatFromInt(stats.total_channel_operations)) / uptime_seconds});
            print("Errors: {d:.1}\n", .{@as(f64, @floatFromInt(stats.total_errors)) / uptime_seconds});
        }
        
        // Print hot paths
        if (self.hot_paths.count() > 0) {
            print("\n--- Hot Paths ---\n");
            var iterator = self.hot_paths.iterator();
            var i: usize = 0;
            while (iterator.next()) |entry| : (i += 1) {
                if (i >= 10) break; // Show top 10
                const hot_path = entry.value_ptr.*;
                print("{s}: {d} calls, {d:.3}ms avg\n", .{
                    hot_path.function_name,
                    hot_path.total_calls,
                    @as(f64, @floatFromInt(hot_path.average_time)) / 1_000_000.0,
                });
            }
        }
        
        print("================================\n\n");
    }
    
    // Platform-specific resource monitoring functions
    fn getCpuUsage(self: *PerformanceHooks) f64 {
        _ = self;
        // TODO: Implement platform-specific CPU usage
        return 0.0;
    }
    
    fn getMemoryUsage(self: *PerformanceHooks) usize {
        _ = self;
        // TODO: Implement platform-specific memory usage
        return 0;
    }
    
    fn getHeapUsage(self: *PerformanceHooks) usize {
        _ = self;
        // TODO: Implement heap usage tracking
        return 0;
    }
    
    fn getStackUsage(self: *PerformanceHooks) usize {
        _ = self;
        // TODO: Implement stack usage tracking
        return 0;
    }
    
    fn getOpenFiles(self: *PerformanceHooks) usize {
        _ = self;
        // TODO: Implement open files count
        return 0;
    }
    
    fn getNetworkConnections(self: *PerformanceHooks) usize {
        _ = self;
        // TODO: Implement network connections count
        return 0;
    }
    
    fn getThreadsCount(self: *PerformanceHooks) usize {
        _ = self;
        // TODO: Implement threads count
        return 1; // At least current thread
    }
    
    fn getGoroutinesCount(self: *PerformanceHooks) usize {
        _ = self;
        // TODO: Integrate with concurrency system
        return 0;
    }
    
    fn getChannelsCount(self: *PerformanceHooks) usize {
        _ = self;
        // TODO: Integrate with concurrency system
        return 0;
    }
    
    fn getGcPressure(self: *PerformanceHooks) f64 {
        _ = self;
        // TODO: Integrate with GC system
        return 0.0;
    }
    
    fn getLoadAverage(self: *PerformanceHooks) f64 {
        _ = self;
        // TODO: Implement load average
        return 0.0;
    }
    
    /// Detect performance bottlenecks
    fn detectBottlenecks(self: *PerformanceHooks) !void {
        if (!self.config.enable_bottleneck_analysis) return;
        
        // Clear old bottlenecks
        for (self.bottlenecks.items) |*bottleneck| {
            bottleneck.deinit(allocator);
        }
        self.bottlenecks.clearRetainingCapacity();
        
        // Analyze hot paths for bottlenecks
        var iterator = self.hot_paths.iterator();
        while (iterator.next()) |entry| {
            const hot_path = entry.value_ptr.*;
            
            // Detect slow functions
            if (hot_path.average_time > 100_000_000) { // > 100ms
                const severity: BottleneckSeverity = if (hot_path.average_time > 1_000_000_000) .critical else if (hot_path.average_time > 500_000_000) .major else .moderate;
                
                var affected_functions = .empty;
                try affected_functions.append(self.allocator, try self.allocator.dupe(u8, hot_path.function_name));
                
                const bottleneck = BottleneckData{
                    .bottleneck_type = .cpu_bound,
                    .location = try self.allocator.dupe(u8, hot_path.function_name),
                    .severity = severity,
                    .impact_score = @as(f64, @floatFromInt(hot_path.average_time)) / 1_000_000.0, // Impact in milliseconds
                    .description = try std.fmt.allocPrint(self.allocator, "Function {s} has high average execution time: {d:.2}ms", .{ hot_path.function_name, @as(f64, @floatFromInt(hot_path.average_time)) / 1_000_000.0 }),
                    .suggested_fix = try self.allocator.dupe(u8, "Consider optimizing algorithm or caching results"),
                    .measured_at = self.getCurrentTime(),
                    .affected_functions = try affected_functions.toOwnedSlice(self.allocator),
                };
                
                try self.bottlenecks.append(self.allocator, bottleneck);
            }
            
            // Detect high frequency functions
            if (hot_path.call_frequency > 1000.0) { // > 1000 calls/sec
                var affected_functions = .empty;
                try affected_functions.append(self.allocator, try self.allocator.dupe(u8, hot_path.function_name));
                
                const bottleneck = BottleneckData{
                    .bottleneck_type = .algorithmic_complexity,
                    .location = try self.allocator.dupe(u8, hot_path.function_name),
                    .severity = .moderate,
                    .impact_score = hot_path.call_frequency,
                    .description = try std.fmt.allocPrint(self.allocator, "Function {s} called very frequently: {d:.1} calls/sec", .{ hot_path.function_name, hot_path.call_frequency }),
                    .suggested_fix = try self.allocator.dupe(u8, "Consider reducing call frequency or optimizing for speed"),
                    .measured_at = self.getCurrentTime(),
                    .affected_functions = try affected_functions.toOwnedSlice(self.allocator),
                };
                
                try self.bottlenecks.append(self.allocator, bottleneck);
            }
        }
        
        // Analyze memory allocation patterns
        if (self.memory_events.items.len > 100) {
            var allocation_rate: f64 = 0;
            var recent_allocations: usize = 0;
            const current_time = self.getCurrentTime();
            const one_second_ago = current_time - 1_000_000_000; // 1 second in nanoseconds
            
            for (self.memory_events.items) |event| {
                if (event.timestamp > one_second_ago and event.event_type == .allocation) {
                    recent_allocations += 1;
                }
            }
            
            allocation_rate = @as(f64, @floatFromInt(recent_allocations));
            
            if (allocation_rate > 100.0) { // > 100 allocations/sec
                var affected_functions = .empty;
                
                const bottleneck = BottleneckData{
                    .bottleneck_type = .memory_bound,
                    .location = try self.allocator.dupe(u8, "memory_allocator"),
                    .severity = if (allocation_rate > 1000.0) .major else .moderate,
                    .impact_score = allocation_rate,
                    .description = try std.fmt.allocPrint(self.allocator, "High memory allocation rate: {d:.1} allocations/sec", .{allocation_rate}),
                    .suggested_fix = try self.allocator.dupe(u8, "Consider object pooling or reducing allocations"),
                    .measured_at = current_time,
                    .affected_functions = try affected_functions.toOwnedSlice(self.allocator),
                };
                
                try self.bottlenecks.append(self.allocator, bottleneck);
            }
        }
    }
};

/// Monitoring thread function
fn monitoringThread(hooks: *PerformanceHooks) void {
    while (hooks.active.load(.seq_cst)) {
        Thread.sleep(hooks.config.flush_interval_ms * 1_000_000); // Convert ms to ns
        
        if (!hooks.active.load(.seq_cst)) break;
        
        // Collect resource snapshot
        hooks.mutex.lock();
        const snapshot = hooks.collectResourceSnapshot();
        hooks.resource_snapshots.append(allocator, snapshot) catch {};
        
        // Trim snapshots buffer
        if (hooks.resource_snapshots.items.len > hooks.config.metrics_buffer_size) {
            _ = hooks.resource_snapshots.orderedRemove(0);
        }
        hooks.mutex.unlock();
        
        // Update last flush time
        _ = hooks.last_flush.store(hooks.getCurrentTime(), .seq_cst);
    }
}

// Global performance hooks instance
var global_hooks: ?*PerformanceHooks = null;

/// Initialize global performance hooks
pub fn initGlobalHooks(allocator: Allocator, config: PerformanceHooksConfig) !void {
    if (global_hooks != null) return; // Already initialized
    
    global_hooks = try PerformanceHooks.init(allocator, config);
    try global_hooks.?.start();
}

/// Deinitialize global performance hooks
pub fn deinitGlobalHooks() void {
    if (global_hooks) |hooks| {
        hooks.deinit(allocator);
        global_hooks = null;
    }
}

/// Get global performance hooks instance
pub fn getGlobalHooks() ?*PerformanceHooks {
    return global_hooks;
}

// Convenience functions for global hooks
pub fn recordFunctionCall(function_name: []const u8, module_name: []const u8, args_count: usize, return_size: usize, error_occurred: bool, duration: u64) void {
    if (getGlobalHooks()) |hooks| {
        const start_time = hooks.getCurrentTime() - duration;
        hooks.functionCallEnd(start_time, function_name, module_name, args_count, return_size, error_occurred);
    }
}

pub fn recordMemoryAllocation(size: usize, alignment: usize, allocation_site: []const u8) void {
    if (getGlobalHooks()) |hooks| {
        hooks.memoryAllocated(size, alignment, allocation_site);
    }
}

pub fn recordMemoryDeallocation(size: usize) void {
    if (getGlobalHooks()) |hooks| {
        hooks.memoryDeallocated(size);
    }
}

pub fn recordGoroutineEvent(event_type: GoroutineEventType, goroutine_id: u64, worker_id: usize, parent_id: ?u64, stack_size: usize, priority: concurrency.GoroutinePriority) void {
    if (getGlobalHooks()) |hooks| {
        hooks.goroutineEvent(event_type, goroutine_id, worker_id, parent_id, stack_size, priority);
    }
}

pub fn recordChannelOperation(event_type: ChannelEventType, channel_id: u64, goroutine_id: u64, data_size: usize, queue_length: usize, duration: u64) void {
    if (getGlobalHooks()) |hooks| {
        hooks.channelOperation(event_type, channel_id, goroutine_id, data_size, queue_length, duration);
    }
}

pub fn recordError(event_type: ErrorEventType, error_message: []const u8, location: []const u8) void {
    if (getGlobalHooks()) |hooks| {
        hooks.errorOccurred(event_type, error_message, location);
    }
}

pub fn printGlobalReport() void {
    if (getGlobalHooks()) |hooks| {
        hooks.printReport();
    } else {
        print("Performance hooks not initialized\n");
    }
}
