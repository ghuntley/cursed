//! CURSED Concurrency Runtime - Working Implementation
//!
//! This module provides the actual working implementation of goroutines and channels
//! that integrates with the CURSED interpreter.

const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Mutex = std.Thread.Mutex;
const Condition = std.Thread.Condition;
const Thread = std.Thread;
const Atomic = std.atomic.Value;

const concurrency = @import("concurrency.zig");
const Variable = @import("main_unified.zig").Variable;

/// Simple channel ID counter
var next_channel_id: Atomic(u64) = Atomic(u64).init(1);

/// Runtime error types
pub const RuntimeError = error{
    ChannelNotFound,
    ChannelClosed,
    AllocationFailed,
    RuntimeNotInitialized,
};

/// Simple channel implementation
pub const SimpleChannel = struct {
    id: u64,
    buffer: ArrayList(Variable),
    mutex: Mutex,
    send_condition: Condition,
    recv_condition: Condition,
    capacity: usize,
    closed: Atomic(bool),
    allocator: Allocator,

    pub fn init(allocator: Allocator, capacity: usize) !SimpleChannel {
        return SimpleChannel{
            .id = next_channel_id.fetchAdd(1, .acq_rel),
            .buffer = .empty,
            .mutex = Mutex{},
            .send_condition = Condition{},
            .recv_condition = Condition{},
            .capacity = capacity,
            .closed = Atomic(bool).init(false),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *SimpleChannel) void {
        self.close();
        self.buffer.deinit(self.allocator);
    }

    pub fn send(self: *SimpleChannel, value: Variable) !void {
        return self.sendTimeout(value, 30_000_000_000); // 30 second timeout
    }

    pub fn sendTimeout(self: *SimpleChannel, value: Variable, timeout_ns: u64) !void {
        if (self.closed.load(.acquire)) {
            return RuntimeError.ChannelClosed;
        }

        const start_time = std.time.nanoTimestamp();

        // Use tryLock with timeout to prevent deadlock
        if (!self.tryLockWithTimeout(timeout_ns)) {
            return RuntimeError.ChannelClosed; // Treat timeout as closed for now
        }
        defer self.mutex.unlock();

        // For unbuffered channels (capacity 0), wait for receiver
        if (self.capacity == 0) {
            try self.buffer.append(self.allocator, value);
            self.recv_condition.signal();
            return;
        }

        // For buffered channels, wait for space with timeout
        while (self.buffer.items.len >= self.capacity and !self.closed.load(.acquire)) {
            const elapsed = @as(u64, @intCast(std.time.nanoTimestamp() - start_time));
            if (elapsed >= timeout_ns) {
                return RuntimeError.ChannelClosed; // Timeout
            }
            
            // Use timed wait instead of indefinite wait
            self.send_condition.timedWait(&self.mutex, 100_000_000) catch {}; // 100ms chunks
        }

        if (self.closed.load(.acquire)) {
            return RuntimeError.ChannelClosed;
        }

        try self.buffer.append(self.allocator, value);
        self.recv_condition.signal();
    }

    fn tryLockWithTimeout(self: *SimpleChannel, timeout_ns: u64) bool {
        const start_time = std.time.nanoTimestamp();
        const end_time = start_time + @as(i64, @intCast(timeout_ns));
        
        while (std.time.nanoTimestamp() < end_time) {
            if (self.mutex.tryLock()) {
                return true;
            }
            std.Thread.sleep(100_000); // 100 microseconds
        }
        
        return false;
    }

    pub fn receive(self: *SimpleChannel) !?Variable {
        return self.receiveTimeout(30_000_000_000); // 30 second timeout
    }

    pub fn receiveTimeout(self: *SimpleChannel, timeout_ns: u64) !?Variable {
        const start_time = std.time.nanoTimestamp();

        // CRITICAL FIX: Use tryLock with timeout to prevent deadlock
        if (!self.tryLockWithTimeout(timeout_ns)) {
            return null; // Timeout
        }
        defer self.mutex.unlock();

        // CRITICAL FIX: Add iteration limit to prevent infinite loops
        var iterations: u32 = 0;
        const max_iterations = 10000; // Prevent infinite loops
        
        while (self.buffer.items.len == 0 and !self.closed.load(.acquire) and iterations < max_iterations) {
            const elapsed = @as(u64, @intCast(std.time.nanoTimestamp() - start_time));
            if (elapsed >= timeout_ns) {
                return null; // Timeout
            }
            
            // CRITICAL FIX: Use shorter timed wait chunks and break on timeout
            const chunk_timeout = 10_000_000; // 10ms chunks instead of 100ms
            self.recv_condition.timedWait(&self.mutex, chunk_timeout) catch {
                // On wait timeout, check if we should continue
                iterations += 1;
                continue;
            };
            
            iterations += 1;
        }

        // CRITICAL FIX: Safety check for infinite loop detection
        if (iterations >= max_iterations) {
            std.debug.print("⚠️  Channel receive operation hit iteration limit - preventing infinite loop\n", .{});
            return null;
        }

        if (self.buffer.items.len > 0) {
            const value = self.buffer.orderedRemove(0);
            self.send_condition.signal();
            return value;
        }

        if (self.closed.load(.acquire)) {
            return null;
        }

        return null;
    }

    pub fn close(self: *SimpleChannel) void {
        self.closed.store(true, .release);
        self.send_condition.broadcast();
        self.recv_condition.broadcast();
    }

    pub fn isClosed(self: *SimpleChannel) bool {
        return self.closed.load(.acquire);
    }
};

/// Global channel registry
var channel_registry: ?HashMap(u64, *SimpleChannel, std.hash_map.AutoContext(u64), std.hash_map.default_max_load_percentage) = null;
var registry_mutex: Mutex = Mutex{};
var registry_allocator: ?Allocator = null;

/// Initialize the runtime
pub fn initRuntime(allocator: Allocator) !void {
    registry_mutex.lock();
    defer registry_mutex.unlock();

    if (channel_registry == null) {
        registry_allocator = allocator;
        channel_registry = HashMap(u64, *SimpleChannel, std.hash_map.AutoContext(u64), std.hash_map.default_max_load_percentage).init(allocator);
        
        // Initialize scheduler
        try concurrency.initializeScheduler(allocator, concurrency.SchedulerConfig.default());
    }
}

/// Shutdown the runtime
pub fn shutdownRuntime() void {
    registry_mutex.lock();
    defer registry_mutex.unlock();

    if (channel_registry) |*registry| {
        var iterator = registry.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.*.deinit();
            if (registry_allocator) |allocator| {
                allocator.destroy(entry.value_ptr.*);
            }
        }
        registry.deinit();
        channel_registry = null;
    }

    if (registry_allocator) |allocator| {
        concurrency.shutdownScheduler(allocator);
    }
}

/// Create a new channel
pub fn createChannel(capacity: usize) !u64 {
    const allocator = registry_allocator orelse return RuntimeError.RuntimeNotInitialized;

    if (channel_registry == null) {
        try initRuntime(allocator);
    }

    const channel = try allocator.create(SimpleChannel);
    channel.* = try SimpleChannel.init(allocator, capacity);

    registry_mutex.lock();
    defer registry_mutex.unlock();

    if (channel_registry) |*registry| {
        try registry.put(channel.id, channel);
        return channel.id;
    }

    return RuntimeError.RuntimeNotInitialized;
}

/// Send value to channel
pub fn sendToChannel(channel_id: u64, value: Variable) !void {
    return sendToChannelTimeout(channel_id, value, 30_000_000_000); // 30 second timeout
}

/// Send value to channel with timeout
pub fn sendToChannelTimeout(channel_id: u64, value: Variable, timeout_ns: u64) !void {
    // Use tryLock with timeout on registry to prevent deadlock
    const start_time = std.time.nanoTimestamp();
    const registry_timeout = @min(timeout_ns, 1_000_000_000); // Max 1 second for registry lock
    
    while (std.time.nanoTimestamp() - start_time < registry_timeout) {
        if (registry_mutex.tryLock()) {
            defer registry_mutex.unlock();
            
            if (channel_registry) |*registry| {
                if (registry.get(channel_id)) |channel| {
                    const remaining_timeout = timeout_ns - @as(u64, @intCast(std.time.nanoTimestamp() - start_time));
                    try channel.sendTimeout(value, remaining_timeout);
                    return;
                }
            }
            
            return RuntimeError.ChannelNotFound;
        }
        std.Thread.sleep(100_000); // 100 microseconds
    }

    return RuntimeError.ChannelNotFound; // Timeout on registry lock
}

/// Receive value from channel
pub fn receiveFromChannel(channel_id: u64) !?Variable {
    return receiveFromChannelTimeout(channel_id, 30_000_000_000); // 30 second timeout
}

/// Receive value from channel with timeout
pub fn receiveFromChannelTimeout(channel_id: u64, timeout_ns: u64) !?Variable {
    // Use tryLock with timeout on registry to prevent deadlock
    const start_time = std.time.nanoTimestamp();
    const registry_timeout = @min(timeout_ns, 1_000_000_000); // Max 1 second for registry lock
    
    while (std.time.nanoTimestamp() - start_time < registry_timeout) {
        if (registry_mutex.tryLock()) {
            const channel_ptr = blk: {
                defer registry_mutex.unlock();
                if (channel_registry) |*registry| {
                    if (registry.get(channel_id)) |channel| {
                        break :blk channel;
                    }
                }
                return RuntimeError.ChannelNotFound;
            };
            
            const remaining_timeout = timeout_ns - @as(u64, @intCast(std.time.nanoTimestamp() - start_time));
            return try channel_ptr.receiveTimeout(remaining_timeout);
        }
        std.Thread.sleep(100_000); // 100 microseconds
    }
    
    return RuntimeError.ChannelNotFound; // Timeout on registry lock
}

/// Close channel
pub fn closeChannel(channel_id: u64) !void {
    registry_mutex.lock();
    defer registry_mutex.unlock();

    if (channel_registry) |*registry| {
        if (registry.get(channel_id)) |channel| {
            channel.close();
            return;
        }
    }

    return RuntimeError.ChannelNotFound;
}

/// Check if channel is closed
pub fn isChannelClosed(channel_id: u64) !bool {
    registry_mutex.lock();
    defer registry_mutex.unlock();

    if (channel_registry) |*registry| {
        if (registry.get(channel_id)) |channel| {
            return channel.isClosed();
        }
    }

    return RuntimeError.ChannelNotFound;
}

/// Spawn goroutine from interpreter
pub fn spawnGoroutine(entry_fn: concurrency.GoroutineEntry, context: ?*anyopaque) !u64 {
    return try concurrency.stan(entry_fn, context);
}

/// High-level API functions for CURSED integration

/// Execute stan statement from interpreter (spawn goroutine with custom function)
pub fn executeStanFromInterpreter(context: ?*anyopaque, entry_function: concurrency.GoroutineEntry) !concurrency.GoroutineId {
    // Ensure runtime is initialized
    if (registry_allocator == null) {
        try initRuntime(std.heap.page_allocator);
    }
    
    // Spawn goroutine using the provided entry function
    const goroutine_id = try concurrency.stan(entry_function, context);
    
    std.log.debug("Spawned goroutine {} from interpreter", .{goroutine_id});
    return goroutine_id;
}

// C-style exports for LLVM compilation

/// C FFI export for creating channels from LLVM compiled code
export fn cursed_make_channel(capacity: u32) u64 {
    if (registry_allocator == null) {
        initRuntime(std.heap.page_allocator) catch return 0;
    }
    
    return createChannel(capacity) catch 0;
}

/// C FFI export for sending to channels from LLVM compiled code
export fn cursed_send_channel(channel_id: u64, value_int: i64) u32 {
    const value = Variable{ .Integer = value_int };
    sendToChannel(channel_id, value) catch return 0;
    return 1; // Success
}

/// C FFI export for receiving from channels from LLVM compiled code
export fn cursed_recv_channel(channel_id: u64) i64 {
    const result = receiveFromChannel(channel_id) catch return 0;
    if (result) |value| {
        switch (value) {
            .Integer => |int_val| return int_val,
            else => return 0,
        }
    }
    return 0;
}

/// C FFI export for closing channels from LLVM compiled code
export fn cursed_close_channel(channel_id: u64) u32 {
    closeChannel(channel_id) catch return 0;
    return 1; // Success
}

/// C FFI export for spawning goroutines from LLVM compiled code (simple version)
export fn cursed_spawn_goroutine_simple(func_ptr: ?*const fn () callconv(.c) void) u64 {
    if (registry_allocator == null) {
        initRuntime(std.heap.page_allocator) catch return 0;
    }
    
    // Wrapper function to convert C function pointer to Zig function
    const GoroutineWrapper = struct {
        c_func: ?*const fn () callconv(.c) void,
        
        fn run(ctx: ?*anyopaque) void {
            const wrapper: *@This() = @ptrCast(@alignCast(ctx.?));
            if (wrapper.c_func) |func| {
                func();
            }
        }
    };
    
    const allocator = registry_allocator.?;
    const wrapper = allocator.create(GoroutineWrapper) catch return 0;
    wrapper.c_func = func_ptr;
    
    const goroutine_id = spawnGoroutine(GoroutineWrapper.run, wrapper) catch return 0;
    
    return goroutine_id;
}

// Tests
const testing = std.testing;

test "channel creation and communication" {
    try initRuntime(testing.allocator);
    defer shutdownRuntime();

    const channel_id = try createChannel(3);
    
    const value1 = Variable{ .Integer = 42 };
    const value2 = Variable{ .Integer = 100 };
    
    try sendToChannel(channel_id, value1);
    try sendToChannel(channel_id, value2);
    
    const received1 = try receiveFromChannel(channel_id);
    try testing.expect(received1 != null);
    if (received1) |val| {
        try testing.expect(val.Integer == 42);
    }
    
    const received2 = try receiveFromChannel(channel_id);
    try testing.expect(received2 != null);
    if (received2) |val| {
        try testing.expect(val.Integer == 100);
    }
}

test "channel close behavior" {
    try initRuntime(testing.allocator);
    defer shutdownRuntime();

    const channel_id = try createChannel(1);
    
    const value = Variable{ .Integer = 50 };
    try sendToChannel(channel_id, value);
    
    try closeChannel(channel_id);
    try testing.expect(try isChannelClosed(channel_id));
    
    // Should still be able to receive buffered value
    const received = try receiveFromChannel(channel_id);
    try testing.expect(received != null);
    if (received) |val| {
        try testing.expect(val.Integer == 50);
    }
}
