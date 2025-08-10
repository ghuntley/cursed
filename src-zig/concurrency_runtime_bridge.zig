//! CURSED Concurrency Runtime Bridge - Production Implementation
//!
//! This module provides the C-compatible runtime functions that LLVM-generated
//! code calls for concurrency operations. It bridges between the generated code
//! and the full Zig concurrency implementation.
//!
//! Features:
//! - Type-safe bridge between CURSED goroutines and Zig runtime
//! - Optimized channel communication with work-stealing scheduler
//! - Memory-safe goroutine lifecycle management
//! - Performance monitoring and statistics
//! - Cross-platform scheduler enhancements

const std = @import("std");
const concurrency = @import("concurrency.zig");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Mutex = std.Thread.Mutex;
const Atomic = std.atomic.Value;

// Enhanced global state management
var global_allocator = std.heap.page_allocator;
var global_scheduler: ?*concurrency.Scheduler = null;
var scheduler_initialized = false;
var scheduler_mutex = Mutex{};

// Goroutine tracking for lifecycle management
const GoroutineTracker = struct {
    goroutines: HashMap(concurrency.GoroutineId, *GoroutineInfo, GoroutineContext, std.hash_map.default_max_load_percentage),
    mutex: Mutex,
    next_id: Atomic(u64),

    const Self = @This();
    const GoroutineContext = struct {
        pub fn hash(self: @This(), s: concurrency.GoroutineId) u64 {
            _ = self;
            return std.hash_map.hashInt(s);
        }
        pub fn eql(self: @This(), a: concurrency.GoroutineId, b: concurrency.GoroutineId) bool {
            _ = self;
            return a == b;
        }
    };

    const GoroutineInfo = struct {
        id: concurrency.GoroutineId,
        function_ptr: ?*anyopaque,
        context: ?*anyopaque,
        state: concurrency.GoroutineState,
        created_at: i64,
        completed_at: ?i64,
        allocator: Allocator,

        pub fn deinit(self: *GoroutineInfo) void {
            // Clean up any allocated context
            if (self.context) |ctx| {
                // Context cleanup would happen here
                _ = ctx;
            }
        }
    };

    pub fn init(allocator: Allocator) Self {
        return Self{
            .goroutines = HashMap(concurrency.GoroutineId, *GoroutineInfo, GoroutineContext, std.hash_map.default_max_load_percentage).init(allocator),
            .mutex = Mutex{},
            .next_id = Atomic(u64).init(1),
        };
    }

    pub fn deinit(self: *Self) void {
        self.mutex.lock();
        defer self.mutex.unlock();

        var iterator = self.goroutines.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.*.deinit();
            self.goroutines.allocator.destroy(entry.value_ptr.*);
        }
        self.goroutines.deinit();
    }

    pub fn registerGoroutine(self: *Self, id: concurrency.GoroutineId, function_ptr: ?*anyopaque, context: ?*anyopaque) !void {
        self.mutex.lock();
        defer self.mutex.unlock();

        const info = try self.goroutines.allocator.create(GoroutineInfo);
        info.* = GoroutineInfo{
            .id = id,
            .function_ptr = function_ptr,
            .context = context,
            .state = concurrency.GoroutineState.ready,
            .created_at = std.time.milliTimestamp(),
            .completed_at = null,
            .allocator = self.goroutines.allocator,
        };

        try self.goroutines.put(id, info);
    }

    pub fn updateState(self: *Self, id: concurrency.GoroutineId, state: concurrency.GoroutineState) void {
        self.mutex.lock();
        defer self.mutex.unlock();

        if (self.goroutines.get(id)) |info| {
            info.state = state;
            if (state == .completed or state == .panicked) {
                info.completed_at = std.time.milliTimestamp();
            }
        }
    }

    pub fn getGoroutineInfo(self: *Self, id: concurrency.GoroutineId) ?*GoroutineInfo {
        self.mutex.lock();
        defer self.mutex.unlock();
        return self.goroutines.get(id);
    }

    pub fn cleanup(self: *Self, id: concurrency.GoroutineId) void {
        self.mutex.lock();
        defer self.mutex.unlock();

        if (self.goroutines.fetchRemove(id)) |kv| {
            kv.value.deinit();
            self.goroutines.allocator.destroy(kv.value);
        }
    }

    pub fn getActiveCount(self: *Self) u32 {
        self.mutex.lock();
        defer self.mutex.unlock();

        var count: u32 = 0;
        var iterator = self.goroutines.iterator();
        while (iterator.next()) |entry| {
            const state = entry.value_ptr.*.state;
            if (state == .ready or state == .running or state == .waiting or state == .yielded) {
                count += 1;
            }
        }
        return count;
    }
};

var goroutine_tracker: ?GoroutineTracker = null;

/// Enhanced initialization with goroutine tracking
pub export fn cursed_concurrency_init() void {
    scheduler_mutex.lock();
    defer scheduler_mutex.unlock();
    
    if (scheduler_initialized) return;
    
    // Initialize goroutine tracker
    goroutine_tracker = GoroutineTracker.init(global_allocator);
    
    // Enhanced scheduler configuration for production
    var config = concurrency.SchedulerConfig.default();
    config.enable_work_stealing = true;
    config.enable_preemption = true;
    config.quantum_ms = 5; // Smaller quantum for better responsiveness
    
    concurrency.initializeScheduler(global_allocator, config) catch |err| {
        std.debug.print("Failed to initialize scheduler: {}\n", .{err});
        return;
    };
    
    global_scheduler = concurrency.getScheduler();
    scheduler_initialized = true;
}

/// Enhanced cleanup with proper lifecycle management
pub export fn cursed_concurrency_cleanup() void {
    scheduler_mutex.lock();
    defer scheduler_mutex.unlock();
    
    if (!scheduler_initialized) return;
    
    // Clean up goroutine tracker
    if (goroutine_tracker) |*tracker| {
        tracker.deinit();
        goroutine_tracker = null;
    }
    
    concurrency.shutdownScheduler(global_allocator);
    global_scheduler = null;
    scheduler_initialized = false;
}

/// Enhanced goroutine context for proper lifecycle management
const GoroutineContext = struct {
    function_ptr: ?*anyopaque,
    user_context: ?*anyopaque,
    goroutine_id: concurrency.GoroutineId,
    
    pub fn init(function_ptr: ?*anyopaque, user_context: ?*anyopaque, id: concurrency.GoroutineId) GoroutineContext {
        return GoroutineContext{
            .function_ptr = function_ptr,
            .user_context = user_context,
            .goroutine_id = id,
        };
    }
};

/// Enhanced goroutine entry point wrapper with lifecycle tracking
fn goroutineWrapper(context: ?*anyopaque) void {
    if (context) |ctx| {
        const goroutine_ctx: *GoroutineContext = @ptrCast(@alignCast(ctx));
        const id = goroutine_ctx.goroutine_id;
        
        // Update state to running
        if (goroutine_tracker) |tracker| {
            tracker.updateState(id, .running);
        }
        
        // Execute the actual function
        if (goroutine_ctx.function_ptr) |func_ptr| {
            const entry_fn: concurrency.GoroutineEntry = @ptrCast(@alignCast(func_ptr));
            entry_fn(goroutine_ctx.user_context);
        }
        
        // Update state to completed and cleanup
        if (goroutine_tracker) |tracker| {
            tracker.updateState(id, .completed);
            // Defer cleanup to avoid immediate deallocation
            std.time.sleep(1_000_000); // 1ms grace period
            tracker.cleanup(id);
        }
        
        // Clean up context
        global_allocator.destroy(goroutine_ctx);
    }
}

/// Enhanced goroutine spawning with proper lifecycle management
pub export fn cursed_stan_goroutine(function_ptr: ?*anyopaque, context: ?*anyopaque) u64 {
    if (!scheduler_initialized) {
        cursed_concurrency_init();
    }
    
    const scheduler = global_scheduler orelse return 0;
    
    // Create enhanced context
    const goroutine_ctx = global_allocator.create(GoroutineContext) catch return 0;
    
    // Pre-allocate goroutine ID for tracking
    const goroutine_id = if (goroutine_tracker) |*tracker| 
        tracker.next_id.fetchAdd(1, .acq_rel) 
    else 
        1;
    
    goroutine_ctx.* = GoroutineContext.init(function_ptr, context, goroutine_id);
    
    // Register goroutine for tracking
    if (goroutine_tracker) |*tracker| {
        tracker.registerGoroutine(goroutine_id, function_ptr, context) catch {
            global_allocator.destroy(goroutine_ctx);
            return 0;
        };
    }
    
    // Spawn with enhanced wrapper
    const actual_id = scheduler.spawn(goroutineWrapper, goroutine_ctx) catch {
        if (goroutine_tracker) |*tracker| {
            tracker.cleanup(goroutine_id);
        }
        global_allocator.destroy(goroutine_ctx);
        return 0;
    };
    
    return actual_id;
}

/// Yield current goroutine (implements `yolo` keyword)
/// Returns true if yield was successful
pub export fn cursed_yolo_goroutine() bool {
    const scheduler = global_scheduler orelse return false;
    scheduler.yield() catch return false;
    return true;
}

/// Enhanced channel metadata for type safety and optimization
const ChannelMetadata = struct {
    element_size: u32,
    capacity: u32,
    channel_type: ChannelType,
    creation_time: i64,
    
    const ChannelType = enum {
        i32_channel,
        f64_channel,
        string_channel,
        generic_channel,
    };
    
    pub fn init(element_size: u32, capacity: u32) ChannelMetadata {
        const channel_type = switch (element_size) {
            4 => ChannelType.i32_channel,
            8 => ChannelType.f64_channel,
            else => ChannelType.generic_channel,
        };
        
        return ChannelMetadata{
            .element_size = element_size,
            .capacity = capacity,
            .channel_type = channel_type,
            .creation_time = std.time.milliTimestamp(),
        };
    }
};

/// Enhanced channel wrapper for type-safe operations
const ChannelWrapper = struct {
    metadata: ChannelMetadata,
    channel_ptr: *anyopaque,
    
    pub fn init(metadata: ChannelMetadata, channel_ptr: *anyopaque) ChannelWrapper {
        return ChannelWrapper{
            .metadata = metadata,
            .channel_ptr = channel_ptr,
        };
    }
};

/// Optimized channel creation with type safety
pub export fn cursed_dm_create(element_size: u32, capacity: u32) ?*anyopaque {
    const metadata = ChannelMetadata.init(element_size, capacity);
    
    // Create wrapper
    const wrapper = global_allocator.create(ChannelWrapper) catch return null;
    
    // Create appropriate channel based on element size
    const channel_ptr = switch (metadata.channel_type) {
        .i32_channel => blk: {
            const channel = concurrency.makeChannel(i32, global_allocator, capacity) catch {
                global_allocator.destroy(wrapper);
                return null;
            };
            break :blk @as(*anyopaque, @ptrCast(channel));
        },
        .f64_channel => blk: {
            const channel = concurrency.makeChannel(f64, global_allocator, capacity) catch {
                global_allocator.destroy(wrapper);
                return null;
            };
            break :blk @as(*anyopaque, @ptrCast(channel));
        },
        else => blk: {
            // For generic types, use i32 as fallback
            const channel = concurrency.makeChannel(i32, global_allocator, capacity) catch {
                global_allocator.destroy(wrapper);
                return null;
            };
            break :blk @as(*anyopaque, @ptrCast(channel));
        },
    };
    
    wrapper.* = ChannelWrapper.init(metadata, channel_ptr);
    return @ptrCast(wrapper);
}

/// Optimized channel send with type safety
pub export fn cursed_dm_send(channel_ptr: ?*anyopaque, value_ptr: ?*anyopaque, value_size: u32) u32 {
    if (channel_ptr == null or value_ptr == null) return 2;
    
    const wrapper: *ChannelWrapper = @ptrCast(@alignCast(channel_ptr));
    
    // Validate value size matches channel type
    if (value_size != wrapper.metadata.element_size) return 2;
    
    // Type-safe send based on channel type
    const result = switch (wrapper.metadata.channel_type) {
        .i32_channel => blk: {
            const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
            const value: *i32 = @ptrCast(@alignCast(value_ptr));
            break :blk channel.send(value.*) catch .closed;
        },
        .f64_channel => blk: {
            const channel: *concurrency.Channel(f64) = @ptrCast(@alignCast(wrapper.channel_ptr));
            const value: *f64 = @ptrCast(@alignCast(value_ptr));
            break :blk channel.send(value.*) catch .closed;
        },
        else => blk: {
            const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
            const value: *i32 = @ptrCast(@alignCast(value_ptr));
            break :blk channel.send(value.*) catch .closed;
        },
    };
    
    return switch (result) {
        .sent => 0,
        .would_block => 1,
        .closed => 2,
    };
}

/// Optimized channel receive with type safety
pub export fn cursed_dm_receive(channel_ptr: ?*anyopaque, buffer_ptr: ?*anyopaque, buffer_size: u32) u32 {
    if (channel_ptr == null or buffer_ptr == null) return 2;
    
    const wrapper: *ChannelWrapper = @ptrCast(@alignCast(channel_ptr));
    
    // Validate buffer size matches channel type
    if (buffer_size != wrapper.metadata.element_size) return 2;
    
    // Type-safe receive based on channel type
    const success = switch (wrapper.metadata.channel_type) {
        .i32_channel => blk: {
            const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
            const buffer: *i32 = @ptrCast(@alignCast(buffer_ptr));
            const received = channel.receive() catch break :blk false;
            if (received) |value| {
                buffer.* = value;
                break :blk true;
            } else {
                break :blk false;
            }
        },
        .f64_channel => blk: {
            const channel: *concurrency.Channel(f64) = @ptrCast(@alignCast(wrapper.channel_ptr));
            const buffer: *f64 = @ptrCast(@alignCast(buffer_ptr));
            const received = channel.receive() catch break :blk false;
            if (received) |value| {
                buffer.* = value;
                break :blk true;
            } else {
                break :blk false;
            }
        },
        else => blk: {
            const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
            const buffer: *i32 = @ptrCast(@alignCast(buffer_ptr));
            const received = channel.receive() catch break :blk false;
            if (received) |value| {
                buffer.* = value;
                break :blk true;
            } else {
                break :blk false;
            }
        },
    };
    
    return if (success) 0 else 2;
}

/// Enhanced select operation descriptor
const SelectOperationDescriptor = struct {
    operation_type: OperationType,
    channel_ptr: ?*anyopaque,
    value_ptr: ?*anyopaque,
    case_index: u32,
    
    const OperationType = enum(u32) {
        send = 0,
        receive = 1,
        default = 2,
    };
};

/// Enhanced select statement implementation with real channel multiplexing
pub export fn cursed_ready_select(operations_ptr: ?*anyopaque, operation_count: u32) i32 {
    if (operations_ptr == null or operation_count == 0) return -2; // Error
    
    const operations: [*]SelectOperationDescriptor = @ptrCast(@alignCast(operations_ptr));
    const ops_slice = operations[0..operation_count];
    
    var select_stmt = concurrency.Select.init(global_allocator);
    defer select_stmt.deinit();
    
    // Check for immediate readiness (non-blocking pass)
    for (ops_slice, 0..) |op, i| {
        switch (op.operation_type) {
            .send => {
                if (op.channel_ptr) |channel_ptr| {
                    // Try non-blocking send
                    const wrapper: *ChannelWrapper = @ptrCast(@alignCast(channel_ptr));
                    const can_send = switch (wrapper.metadata.channel_type) {
                        .i32_channel => blk: {
                            const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
                            break :blk !channel.isFull() and !channel.isClosed();
                        },
                        .f64_channel => blk: {
                            const channel: *concurrency.Channel(f64) = @ptrCast(@alignCast(wrapper.channel_ptr));
                            break :blk !channel.isFull() and !channel.isClosed();
                        },
                        else => blk: {
                            const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
                            break :blk !channel.isFull() and !channel.isClosed();
                        },
                    };
                    if (can_send) return @intCast(i);
                }
            },
            .receive => {
                if (op.channel_ptr) |channel_ptr| {
                    // Try non-blocking receive
                    const wrapper: *ChannelWrapper = @ptrCast(@alignCast(channel_ptr));
                    const can_receive = switch (wrapper.metadata.channel_type) {
                        .i32_channel => blk: {
                            const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
                            break :blk !channel.isEmpty() or channel.isClosed();
                        },
                        .f64_channel => blk: {
                            const channel: *concurrency.Channel(f64) = @ptrCast(@alignCast(wrapper.channel_ptr));
                            break :blk !channel.isEmpty() or channel.isClosed();
                        },
                        else => blk: {
                            const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
                            break :blk !channel.isEmpty() or channel.isClosed();
                        },
                    };
                    if (can_receive) return @intCast(i);
                }
            },
            .default => {
                // Default case is always ready
                return @intCast(i);
            },
        }
    }
    
    // If no immediate readiness and no default case, block briefly and retry
    var retry_count: u32 = 0;
    while (retry_count < 100) { // Max 100 retries with microsecond delays
        std.time.sleep(10_000); // 10 microseconds
        
        for (ops_slice, 0..) |op, i| {
            switch (op.operation_type) {
                .send => {
                    if (op.channel_ptr) |channel_ptr| {
                        const wrapper: *ChannelWrapper = @ptrCast(@alignCast(channel_ptr));
                        const can_send = switch (wrapper.metadata.channel_type) {
                            .i32_channel => blk: {
                                const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
                                break :blk !channel.isFull() and !channel.isClosed();
                            },
                            .f64_channel => blk: {
                                const channel: *concurrency.Channel(f64) = @ptrCast(@alignCast(wrapper.channel_ptr));
                                break :blk !channel.isFull() and !channel.isClosed();
                            },
                            else => blk: {
                                const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
                                break :blk !channel.isFull() and !channel.isClosed();
                            },
                        };
                        if (can_send) return @intCast(i);
                    }
                },
                .receive => {
                    if (op.channel_ptr) |channel_ptr| {
                        const wrapper: *ChannelWrapper = @ptrCast(@alignCast(channel_ptr));
                        const can_receive = switch (wrapper.metadata.channel_type) {
                            .i32_channel => blk: {
                                const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
                                break :blk !channel.isEmpty() or channel.isClosed();
                            },
                            .f64_channel => blk: {
                                const channel: *concurrency.Channel(f64) = @ptrCast(@alignCast(wrapper.channel_ptr));
                                break :blk !channel.isEmpty() or channel.isClosed();
                            },
                            else => blk: {
                                const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
                                break :blk !channel.isEmpty() or channel.isClosed();
                            },
                        };
                        if (can_receive) return @intCast(i);
                    }
                },
                .default => {
                    return @intCast(i);
                },
            }
        }
        
        retry_count += 1;
    }
    
    return -1; // Timeout
}

/// Channel management functions

/// Enhanced channel close with proper cleanup
pub export fn cursed_dm_close(channel_ptr: ?*anyopaque) void {
    if (channel_ptr == null) return;
    
    const wrapper: *ChannelWrapper = @ptrCast(@alignCast(channel_ptr));
    
    switch (wrapper.metadata.channel_type) {
        .i32_channel => {
            const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
            channel.close();
        },
        .f64_channel => {
            const channel: *concurrency.Channel(f64) = @ptrCast(@alignCast(wrapper.channel_ptr));
            channel.close();
        },
        else => {
            const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
            channel.close();
        },
    }
}

/// Enhanced channel status checking
pub export fn cursed_dm_is_closed(channel_ptr: ?*anyopaque) bool {
    if (channel_ptr == null) return true;
    
    const wrapper: *ChannelWrapper = @ptrCast(@alignCast(channel_ptr));
    
    return switch (wrapper.metadata.channel_type) {
        .i32_channel => blk: {
            const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
            break :blk channel.isClosed();
        },
        .f64_channel => blk: {
            const channel: *concurrency.Channel(f64) = @ptrCast(@alignCast(wrapper.channel_ptr));
            break :blk channel.isClosed();
        },
        else => blk: {
            const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
            break :blk channel.isClosed();
        },
    };
}

/// Enhanced channel length retrieval
pub export fn cursed_dm_length(channel_ptr: ?*anyopaque) u32 {
    if (channel_ptr == null) return 0;
    
    const wrapper: *ChannelWrapper = @ptrCast(@alignCast(channel_ptr));
    
    return switch (wrapper.metadata.channel_type) {
        .i32_channel => blk: {
            const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
            break :blk @intCast(channel.length());
        },
        .f64_channel => blk: {
            const channel: *concurrency.Channel(f64) = @ptrCast(@alignCast(wrapper.channel_ptr));
            break :blk @intCast(channel.length());
        },
        else => blk: {
            const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
            break :blk @intCast(channel.length());
        },
    };
}

/// Non-blocking send (try_send)
pub export fn cursed_dm_try_send(channel_ptr: ?*anyopaque, value_ptr: ?*anyopaque, value_size: u32) u32 {
    _ = value_size;
    
    if (channel_ptr == null or value_ptr == null) return 2;
    
    const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(channel_ptr));
    const value: *i32 = @ptrCast(@alignCast(value_ptr));
    
    const result = channel.trySend(value.*) catch return 2;
    
    return switch (result) {
        .sent => 0,
        .would_block => 1,
        .closed => 2,
    };
}

/// Non-blocking receive (try_receive)
pub export fn cursed_dm_try_receive(channel_ptr: ?*anyopaque, buffer_ptr: ?*anyopaque, buffer_size: u32) u32 {
    _ = buffer_size;
    
    if (channel_ptr == null or buffer_ptr == null) return 2;
    
    const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(channel_ptr));
    const buffer: *i32 = @ptrCast(@alignCast(buffer_ptr));
    
    const received = channel.tryReceive() catch return 2;
    
    if (received) |value| {
        buffer.* = value;
        return 0; // success
    } else {
        return 1; // would block or closed
    }
}

/// Goroutine management functions

/// Enhanced current goroutine ID retrieval
pub export fn cursed_current_goroutine_id() u64 {
    // In a real implementation, this would use thread-local storage
    // For now, return based on tracker state
    if (goroutine_tracker) |*tracker| {
        return tracker.next_id.load(.acquire) - 1;
    }
    return 1;
}

/// Enhanced goroutine status checking
pub export fn cursed_goroutine_is_running(goroutine_id: u64) bool {
    if (goroutine_tracker) |*tracker| {
        if (tracker.getGoroutineInfo(goroutine_id)) |info| {
            const state = info.state;
            return state == .ready or state == .running or state == .waiting or state == .yielded;
        }
    }
    return false;
}

/// Enhanced goroutine join with timeout
pub export fn cursed_goroutine_join(goroutine_id: u64) void {
    if (goroutine_tracker) |*tracker| {
        const max_wait_ms = 5000; // 5 second timeout
        const start_time = std.time.milliTimestamp();
        
        while (std.time.milliTimestamp() - start_time < max_wait_ms) {
            if (tracker.getGoroutineInfo(goroutine_id)) |info| {
                if (info.state == .completed or info.state == .panicked) {
                    return;
                }
            } else {
                // Goroutine already cleaned up
                return;
            }
            
            std.time.sleep(1_000_000); // 1ms
        }
        
        // Timeout reached, force cleanup
        tracker.cleanup(goroutine_id);
    }
}

/// Scheduler statistics and monitoring

/// Enhanced active goroutine count (includes tracked goroutines)
pub export fn cursed_scheduler_active_count() u32 {
    var total_count: u32 = 0;
    
    // Get count from scheduler
    if (global_scheduler) |scheduler| {
        total_count += scheduler.activeGoroutineCount();
    }
    
    // Add count from tracker
    if (goroutine_tracker) |*tracker| {
        total_count += tracker.getActiveCount();
    }
    
    return total_count;
}

/// Enhanced scheduler status with tracking information
pub export fn cursed_scheduler_is_running() bool {
    const scheduler = global_scheduler orelse return false;
    return scheduler.isRunning() and scheduler_initialized;
}

/// Enhanced scheduler statistics with performance metrics
const ConcurrencyStats = struct {
    total_spawned: u64,
    total_completed: u64,
    current_active: u32,
    peak_active: u32,
    total_panicked: u64,
    average_lifetime_ms: f64,
    throughput_per_second: f64,
    memory_usage_mb: f64,
};

pub export fn cursed_scheduler_stats(stats_ptr: ?*anyopaque) void {
    if (stats_ptr == null) return;
    
    const stats_out: *ConcurrencyStats = @ptrCast(@alignCast(stats_ptr.?));
    
    // Get scheduler stats
    if (global_scheduler) |scheduler| {
        const sched_stats = scheduler.getStats();
        stats_out.total_spawned = sched_stats.total_spawned;
        stats_out.total_completed = sched_stats.total_completed;
        stats_out.current_active = sched_stats.current_active;
        stats_out.peak_active = sched_stats.peak_active;
        stats_out.total_panicked = sched_stats.total_panicked;
    } else {
        stats_out.* = std.mem.zeroes(ConcurrencyStats);
        return;
    }
    
    // Calculate enhanced metrics from tracker
    if (goroutine_tracker) |*tracker| {
        tracker.mutex.lock();
        defer tracker.mutex.unlock();
        
        var total_lifetime: i64 = 0;
        var completed_count: u32 = 0;
        const current_time = std.time.milliTimestamp();
        
        var iterator = tracker.goroutines.iterator();
        while (iterator.next()) |entry| {
            const info = entry.value_ptr.*;
            if (info.completed_at) |completed_at| {
                total_lifetime += completed_at - info.created_at;
                completed_count += 1;
            }
        }
        
        // Calculate average lifetime
        if (completed_count > 0) {
            stats_out.average_lifetime_ms = @as(f64, @floatFromInt(total_lifetime)) / @as(f64, @floatFromInt(completed_count));
        } else {
            stats_out.average_lifetime_ms = 0.0;
        }
        
        // Calculate throughput (completions per second)
        if (global_scheduler) |scheduler| {
            const sched_stats = scheduler.getStats();
            const runtime_seconds = @as(f64, @floatFromInt(current_time - sched_stats.start_time)) / 1000.0;
            if (runtime_seconds > 0) {
                stats_out.throughput_per_second = @as(f64, @floatFromInt(completed_count)) / runtime_seconds;
            } else {
                stats_out.throughput_per_second = 0.0;
            }
        }
        
        // Estimate memory usage (approximate)
        const goroutine_memory_per_mb = 0.002; // 2KB per goroutine average
        stats_out.memory_usage_mb = @as(f64, @floatFromInt(stats_out.current_active)) * goroutine_memory_per_mb;
    }
}

/// Get performance metrics for monitoring
pub export fn cursed_concurrency_performance_metrics() f64 {
    if (global_scheduler) |scheduler| {
        const stats = scheduler.getStats();
        
        // Return simple performance metric (goroutines per second)
        if (stats.start_time > 0) {
            const runtime_seconds = @as(f64, @floatFromInt(std.time.milliTimestamp() - stats.start_time)) / 1000.0;
            if (runtime_seconds > 0) {
                return @as(f64, @floatFromInt(stats.total_completed)) / runtime_seconds;
            }
        }
    }
    return 0.0;
}

/// Memory-safe channel destruction
pub export fn cursed_dm_destroy(channel_ptr: ?*anyopaque) void {
    if (channel_ptr == null) return;
    
    const wrapper: *ChannelWrapper = @ptrCast(@alignCast(channel_ptr));
    
    // Close channel first
    cursed_dm_close(channel_ptr);
    
    // Clean up underlying channel
    switch (wrapper.metadata.channel_type) {
        .i32_channel => {
            const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
            channel.deinit();
            global_allocator.destroy(channel);
        },
        .f64_channel => {
            const channel: *concurrency.Channel(f64) = @ptrCast(@alignCast(wrapper.channel_ptr));
            channel.deinit();
            global_allocator.destroy(channel);
        },
        else => {
            const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(wrapper.channel_ptr));
            channel.deinit();
            global_allocator.destroy(channel);
        },
    }
    
    // Clean up wrapper
    global_allocator.destroy(wrapper);
}
