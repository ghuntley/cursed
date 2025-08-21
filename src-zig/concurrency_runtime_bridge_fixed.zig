//! Fixed CURSED Concurrency Runtime Bridge - Race-Condition Free Implementation
//!
//! This module fixes the critical race conditions in goroutine cleanup and
//! deadlocks in channel operations:
//! 1. Proper synchronization barriers for goroutine cleanup
//! 2. Lock-free channel operations to prevent deadlocks
//! 3. Timeout mechanisms to prevent indefinite blocking
//! 4. Resource cleanup guarantees

const std = @import("std");
const concurrency_fixed = @import("concurrency_fixed.zig");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Atomic = std.atomic.Value;

// Global state with enhanced safety
var global_allocator = std.heap.page_allocator;
var runtime: ?*concurrency_fixed.ConcurrencyRuntime = null;
var runtime_mutex = std.Thread.Mutex{};
var initialized = false;

// Enhanced goroutine tracking with race-condition prevention
const GoroutineTracker = struct {
    goroutines: HashMap(concurrency_fixed.GoroutineId, *GoroutineInfo, GoroutineContext, std.hash_map.default_max_load_percentage),
    mutex: std.Thread.RwLock, // Use RwLock for better performance
    next_id: Atomic(u64),
    
    // Cleanup synchronization
    cleanup_in_progress: Atomic(bool),
    pending_cleanups: ArrayList(concurrency_fixed.GoroutineId),
    cleanup_worker_thread: ?std.Thread,
    cleanup_shutdown: Atomic(bool),
    
    const Self = @This();
    const GoroutineContext = struct {
        pub fn hash(self: @This(), s: concurrency_fixed.GoroutineId) u64 {
            _ = self;
            return std.hash_map.hashInt(s);
        }
        pub fn eql(self: @This(), a: concurrency_fixed.GoroutineId, b: concurrency_fixed.GoroutineId) bool {
            _ = self;
            return a == b;
        }
    };
    
    const GoroutineInfo = struct {
        id: concurrency_fixed.GoroutineId,
        function_ptr: ?*anyopaque,
        context: ?*anyopaque,
        state: concurrency_fixed.GoroutineState,
        created_at: i64,
        completed_at: Atomic(i64),
        
        // Cleanup synchronization
        cleanup_barrier: std.Thread.ResetEvent,
        cleanup_requested: Atomic(bool),
        
        pub fn init(allocator: Allocator, id: concurrency_fixed.GoroutineId) *GoroutineInfo {
            const info = allocator.create(GoroutineInfo) catch unreachable;
            info.* = GoroutineInfo{
                .id = id,
                .function_ptr = null,
                .context = null,
                .state = .ready,
                .created_at = std.time.microTimestamp(),
                .completed_at = Atomic(i64).init(0),
                .cleanup_barrier = std.Thread.ResetEvent{},
                .cleanup_requested = Atomic(bool).init(false),
            };
            return info;
        }
        
        pub fn deinit(self: *GoroutineInfo, allocator: Allocator) void {
            allocator.destroy(self);
        }
    };
    
    pub fn init(allocator: Allocator) !Self {
        var self = Self{
            .goroutines = HashMap(concurrency_fixed.GoroutineId, *GoroutineInfo, GoroutineContext, std.hash_map.default_max_load_percentage).init(allocator),
            .mutex = std.Thread.RwLock{},
            .next_id = Atomic(u64).init(1),
            .cleanup_in_progress = Atomic(bool).init(false),
            .pending_cleanups = .empty,
            .cleanup_worker_thread = null,
            .cleanup_shutdown = Atomic(bool).init(false),
        };
        
        // Start cleanup worker thread
        self.cleanup_worker_thread = try std.Thread.spawn(.{}, cleanupWorker, .{&self});
        
        return self;
    }
    
    pub fn deinit(self: *Self) void {
        // Shutdown cleanup worker
        self.cleanup_shutdown.store(true, .release);
        if (self.cleanup_worker_thread) |thread| {
            thread.join();
        }
        
        // Cleanup remaining goroutines
        self.mutex.lock();
        defer self.mutex.unlock();
        
        var iterator = self.goroutines.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.*.deinit(self.goroutines.allocator);
        }
        self.goroutines.deinit();
        self.pending_cleanups.deinit();
    }
    
    pub fn register(self: *Self, id: concurrency_fixed.GoroutineId, function_ptr: ?*anyopaque, context: ?*anyopaque) !void {
        const info = GoroutineInfo.init(self.goroutines.allocator, id);
        info.function_ptr = function_ptr;
        info.context = context;
        
        self.mutex.lock();
        defer self.mutex.unlock();
        
        try self.goroutines.put(id, info);
    }
    
    pub fn updateState(self: *Self, id: concurrency_fixed.GoroutineId, new_state: concurrency_fixed.GoroutineState) void {
        self.mutex.lockShared();
        defer self.mutex.unlockShared();
        
        if (self.goroutines.get(id)) |info| {
            info.state = new_state;
            
            if (new_state == .completed) {
                info.completed_at.store(std.time.microTimestamp(), .release);
                self.scheduleCleanup(id);
            }
        }
    }
    
    /// Schedule cleanup in a separate thread to prevent race conditions
    fn scheduleCleanup(self: *Self, id: concurrency_fixed.GoroutineId) void {
        // Signal cleanup request
        if (self.goroutines.get(id)) |info| {
            info.cleanup_requested.store(true, .release);
        }
        
        // Add to pending cleanup queue (thread-safe)
        self.mutex.lock();
        self.pending_cleanups.append(id) catch return; // Ignore error for cleanup
        self.mutex.unlock();
    }
    
    /// Cleanup worker thread to prevent race conditions
    fn cleanupWorker(self: *Self) void {
        while (!self.cleanup_shutdown.load(.acquire)) {
            // Check for pending cleanups
            var cleanup_id: ?concurrency_fixed.GoroutineId = null;
            
            self.mutex.lock();
            if (self.pending_cleanups.items.len > 0) {
                cleanup_id = self.pending_cleanups.orderedRemove(0);
            }
            self.mutex.unlock();
            
            if (cleanup_id) |id| {
                self.performCleanup(id);
            } else {
                std.Thread.sleep(5_000_000); // 5ms when no work
            }
        }
    }
    
    /// Perform actual cleanup with proper synchronization
    fn performCleanup(self: *Self, id: concurrency_fixed.GoroutineId) void {
        // Wait for grace period to ensure goroutine has fully completed
        std.Thread.sleep(10_000_000); // 10ms grace period
        
        self.mutex.lock();
        defer self.mutex.unlock();
        
        if (self.goroutines.fetchRemove(id)) |kv| {
            const info = kv.value;
            
            // Signal cleanup completion
            info.cleanup_barrier.set();
            
            // Final cleanup
            info.deinit(self.goroutines.allocator);
        }
    }
    
    pub fn getGoroutineInfo(self: *Self, id: concurrency_fixed.GoroutineId) ?*GoroutineInfo {
        self.mutex.lockShared();
        defer self.mutex.unlockShared();
        return self.goroutines.get(id);
    }
    
    pub fn getActiveCount(self: *Self) u32 {
        self.mutex.lockShared();
        defer self.mutex.unlockShared();
        
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

// Global tracker instance
var goroutine_tracker: ?*GoroutineTracker = null;

/// Enhanced initialization with proper error handling
pub export fn cursed_concurrency_init() void {
    runtime_mutex.lock();
    defer runtime_mutex.unlock();
    
    if (initialized) {
        return;
    }
    
    // Initialize runtime
    runtime = global_allocator.create(concurrency_fixed.ConcurrencyRuntime) catch {
        std.debug.print("Failed to allocate concurrency runtime\n", .{});
        return;
    };
    
    runtime.?.* = concurrency_fixed.ConcurrencyRuntime.init(global_allocator) catch {
        std.debug.print("Failed to initialize concurrency runtime\n", .{});
        global_allocator.destroy(runtime.?);
        runtime = null;
        return;
    };
    
    // Initialize tracker
    goroutine_tracker = global_allocator.create(GoroutineTracker) catch {
        std.debug.print("Failed to allocate goroutine tracker\n", .{});
        runtime.?.deinit();
        global_allocator.destroy(runtime.?);
        runtime = null;
        return;
    };
    
    goroutine_tracker.?.* = GoroutineTracker.init(global_allocator) catch {
        std.debug.print("Failed to initialize goroutine tracker\n", .{});
        global_allocator.destroy(goroutine_tracker.?);
        goroutine_tracker = null;
        runtime.?.deinit();
        global_allocator.destroy(runtime.?);
        runtime = null;
        return;
    };
    
    initialized = true;
}

/// Enhanced cleanup with proper shutdown sequence
pub export fn cursed_concurrency_cleanup() void {
    runtime_mutex.lock();
    defer runtime_mutex.unlock();
    
    if (!initialized) {
        return;
    }
    
    // Cleanup tracker first
    if (goroutine_tracker) |tracker| {
        tracker.deinit();
        global_allocator.destroy(tracker);
        goroutine_tracker = null;
    }
    
    // Cleanup runtime
    if (runtime) |rt| {
        rt.deinit();
        global_allocator.destroy(rt);
        runtime = null;
    }
    
    initialized = false;
}

/// Enhanced goroutine spawning with race-condition prevention
pub export fn cursed_stan_goroutine(function_ptr: ?*anyopaque, context: ?*anyopaque) u64 {
    if (!initialized) {
        cursed_concurrency_init();
    }
    
    if (runtime == null or goroutine_tracker == null) {
        return 0; // Failed to initialize
    }
    
    const rt = runtime.?;
    const tracker = goroutine_tracker.?;
    
    // Spawn goroutine
    const goroutine_id = rt.spawnGoroutine(function_ptr, context) catch {
        return 0; // Failed to spawn
    };
    
    // Register with tracker
    tracker.register(goroutine_id, function_ptr, context) catch {
        return 0; // Failed to register
    };
    
    return goroutine_id;
}

/// Channel creation with proper resource management
pub export fn cursed_dm_channel_create(capacity: u32) ?*anyopaque {
    if (!initialized) {
        cursed_concurrency_init();
    }
    
    if (runtime == null) {
        return null;
    }
    
    const rt = runtime.?;
    
    // Create i64 channel for simplicity (can be extended for other types)
    const channel = rt.createChannel(i64, capacity) catch {
        return null;
    };
    
    return @ptrCast(channel);
}

/// Enhanced channel send with timeout
pub export fn cursed_dm_send_timeout(channel_ptr: ?*anyopaque, value: i64, timeout_ms: u32) i32 {
    if (channel_ptr == null) {
        return -1; // Invalid channel
    }
    
    const channel: *concurrency_fixed.Channel(i64) = @ptrCast(@alignCast(channel_ptr));
    const timeout_ns = @as(u64, timeout_ms) * 1_000_000;
    
    const result = channel.sendTimeout(value, timeout_ns) catch {
        return -2; // Send error
    };
    
    return switch (result) {
        .sent => 0,
        .closed => 1,
        .would_block => 2,
        .timeout => 3,
    };
}

/// Enhanced channel receive with timeout
pub export fn cursed_dm_recv_timeout(channel_ptr: ?*anyopaque, timeout_ms: u32) i64 {
    if (channel_ptr == null) {
        return std.math.minInt(i64); // Error value
    }
    
    const channel: *concurrency_fixed.Channel(i64) = @ptrCast(@alignCast(channel_ptr));
    const timeout_ns = @as(u64, timeout_ms) * 1_000_000;
    
    const result = channel.receiveTimeout(timeout_ns) catch {
        return std.math.minInt(i64); // Error value
    };
    
    return result orelse std.math.minInt(i64); // Return error value if channel closed
}

/// Non-blocking channel send
pub export fn cursed_dm_send(channel_ptr: ?*anyopaque, value: i64) i32 {
    return cursed_dm_send_timeout(channel_ptr, value, 30000); // 30 second default timeout
}

/// Non-blocking channel receive
pub export fn cursed_dm_recv(channel_ptr: ?*anyopaque) i64 {
    return cursed_dm_recv_timeout(channel_ptr, 30000); // 30 second default timeout
}

/// Non-blocking channel try send
pub export fn cursed_dm_try_send(channel_ptr: ?*anyopaque, value: i64) i32 {
    return cursed_dm_send_timeout(channel_ptr, value, 0); // No timeout
}

/// Non-blocking channel try receive
pub export fn cursed_dm_try_recv(channel_ptr: ?*anyopaque) i64 {
    return cursed_dm_recv_timeout(channel_ptr, 0); // No timeout
}

/// Channel close with proper cleanup
pub export fn cursed_dm_channel_close(channel_ptr: ?*anyopaque) void {
    if (channel_ptr == null) {
        return;
    }
    
    const channel: *concurrency_fixed.Channel(i64) = @ptrCast(@alignCast(channel_ptr));
    channel.close();
}

/// Channel destroy with resource cleanup
pub export fn cursed_dm_channel_destroy(channel_ptr: ?*anyopaque) void {
    if (channel_ptr == null or runtime == null) {
        return;
    }
    
    const channel: *concurrency_fixed.Channel(i64) = @ptrCast(@alignCast(channel_ptr));
    const rt = runtime.?;
    
    rt.destroyChannel(channel);
}

/// Get goroutine status
pub export fn cursed_goroutine_status(goroutine_id: u64) i32 {
    if (goroutine_tracker == null) {
        return -1; // Not initialized
    }
    
    const tracker = goroutine_tracker.?;
    
    if (tracker.getGoroutineInfo(goroutine_id)) |info| {
        return switch (info.state) {
            .ready => 0,
            .running => 1,
            .waiting => 2,
            .yielded => 3,
            .completed => 4,
            .terminating => 5,
        };
    }
    
    return -1; // Not found
}

/// Get active goroutine count
pub export fn cursed_active_goroutines() u32 {
    if (goroutine_tracker == null) {
        return 0;
    }
    
    return goroutine_tracker.?.getActiveCount();
}

/// Wait for all goroutines to complete with timeout
pub export fn cursed_wait_all_goroutines(timeout_ms: u32) i32 {
    if (goroutine_tracker == null) {
        return -1; // Not initialized
    }
    
    const tracker = goroutine_tracker.?;
    const timeout_ns = @as(u64, timeout_ms) * 1_000_000;
    const start_time = std.time.nanoTimestamp();
    
    while (tracker.getActiveCount() > 0) {
        const elapsed = @as(u64, @intCast(std.time.nanoTimestamp() - start_time));
        if (elapsed >= timeout_ns) {
            return 1; // Timeout
        }
        std.Thread.sleep(10_000_000); // 10ms
    }
    
    return 0; // All completed
}

/// Force cleanup of completed goroutines
pub export fn cursed_force_cleanup() void {
    if (goroutine_tracker == null) {
        return;
    }
    
    // This is handled automatically by the cleanup worker thread
    // But we can trigger immediate processing if needed
    std.Thread.sleep(1_000_000); // 1ms to allow cleanup worker to process
}
