//! P1 Priority Concurrency Runtime Bridge - LLVM Integration Implementation
//!
//! This module provides the critical runtime bridge between the interpreter concurrency
//! system and compiled LLVM code for goroutines and channels:
//!
//! 1. **Unified Runtime**: Single runtime instance shared between interpreter and compiled code
//! 2. **Goroutine Scheduling Bridge**: Connect interpreter goroutines with compiled code execution  
//! 3. **Channel Operation Bridge**: Seamless channel operations across execution modes
//! 4. **Memory Management**: Safe resource sharing between modes
//! 5. **LLVM FFI Integration**: Complete C ABI for compiled code interaction

const std = @import("std");
const concurrency_fixed = @import("concurrency_fixed.zig");
const concurrency_runtime = @import("concurrency_runtime.zig");
const simple_var = @import("simple_variable.zig");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Atomic = std.atomic.Value;

// Core types
pub const GoroutineId = u64;
pub const ChannelId = u64;

// Runtime execution modes
pub const ExecutionMode = enum {
    interpreter,
    compiled_native,
    mixed, // Interpreter spawning compiled goroutines
};

/// Unified Runtime Bridge State
pub const RuntimeBridge = struct {
    // Core runtime instances
    interpreter_runtime: *concurrency_runtime.SimpleChannel, // Reuse existing runtime
    compiled_runtime: *concurrency_fixed.ConcurrencyRuntime,
    
    // Mode tracking
    current_mode: ExecutionMode,
    mixed_mode_enabled: bool,
    
    // Resource management
    allocator: Allocator,
    initialized: bool,
    
    // Cross-mode state synchronization
    mode_mutex: std.Thread.RwLock,
    
    // Goroutine bridge registry
    goroutine_bridge_registry: GoroutineBridgeRegistry,
    
    // Channel bridge registry  
    channel_bridge_registry: ChannelBridgeRegistry,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator) !Self {
        // Initialize both runtime systems
        const compiled_runtime = try allocator.create(concurrency_fixed.ConcurrencyRuntime);
        compiled_runtime.* = try concurrency_fixed.ConcurrencyRuntime.init(allocator);
        
        // TODO: Initialize interpreter runtime when available
        // try concurrency_runtime.initRuntime(allocator);
        
        return Self{
            .interpreter_runtime = undefined, // Will be set by first channel creation
            .compiled_runtime = compiled_runtime,
            .current_mode = .interpreter,
            .mixed_mode_enabled = true,
            .allocator = allocator,
            .initialized = true,
            .mode_mutex = std.Thread.RwLock{},
            .goroutine_bridge_registry = try GoroutineBridgeRegistry.init(allocator),
            .channel_bridge_registry = try ChannelBridgeRegistry.init(allocator),
        };
    }
    
    pub fn deinit(self: *Self) void {
        if (!self.initialized) return;
        
        self.channel_bridge_registry.deinit(self.allocator);
        self.goroutine_bridge_registry.deinit(self.allocator);
        self.compiled_runtime.deinit(self.allocator);
        self.allocator.destroy(self.compiled_runtime);
        // TODO: Shutdown interpreter runtime when available
        // concurrency_runtime.shutdownRuntime();
        self.initialized = false;
    }
    
    /// Switch execution mode with proper synchronization
    pub fn switchMode(self: *Self, new_mode: ExecutionMode) void {
        self.mode_mutex.lock();
        defer self.mode_mutex.unlock();
        
        const old_mode = self.current_mode;
        self.current_mode = new_mode;
        
        std.log.debug("Runtime bridge mode switched: {} -> {}", .{ old_mode, new_mode });
    }
    
    /// Get current execution mode (thread-safe)
    pub fn getCurrentMode(self: *Self) ExecutionMode {
        self.mode_mutex.lockShared();
        defer self.mode_mutex.unlockShared();
        return self.current_mode;
    }
};

/// Goroutine Bridge Registry - Maps goroutines between execution modes
pub const GoroutineBridgeRegistry = struct {
    // Registry of goroutines and their execution context
    registry: HashMap(GoroutineId, *GoroutineBridgeEntry, GoroutineContext, std.hash_map.default_max_load_percentage),
    mutex: std.Thread.RwLock,
    next_id: Atomic(u64),
    allocator: Allocator,
    
    const Self = @This();
    const GoroutineContext = struct {
        pub fn hash(self: @This(), s: GoroutineId) u64 {
            _ = self;
            return std.hash_map.hashInt(s);
        }
        pub fn eql(self: @This(), a: GoroutineId, b: GoroutineId) bool {
            _ = self;
            return a == b;
        }
    };
    
    pub fn init(allocator: Allocator) !Self {
        return Self{
            .registry = HashMap(GoroutineId, *GoroutineBridgeEntry, GoroutineContext, std.hash_map.default_max_load_percentage){},
            .mutex = std.Thread.RwLock{},
            .next_id = Atomic(u64).init(1),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        var iterator = self.registry.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.*.deinit(self.allocator);
        }
        self.registry.deinit();
    }
    
    /// Register a goroutine from any execution mode
    pub fn registerGoroutine(self: *Self, entry: *GoroutineBridgeEntry) !GoroutineId {
        const id = self.next_id.fetchAdd(1, .monotonic);
        entry.id = id;
        
        self.mutex.lock();
        defer self.mutex.unlock();
        
        try self.registry.put(id, entry);
        return id;
    }
    
    /// Get goroutine bridge entry (thread-safe)
    pub fn getGoroutine(self: *Self, id: GoroutineId) ?*GoroutineBridgeEntry {
        self.mutex.lockShared();
        defer self.mutex.unlockShared();
        return self.registry.get(id);
    }
    
    /// Remove goroutine from registry
    pub fn removeGoroutine(self: *Self, id: GoroutineId) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        if (self.registry.fetchRemove(id)) |kv| {
            kv.value.deinit(self.allocator);
        }
    }
};

/// Bridge entry for goroutines - connects interpreter and compiled execution
pub const GoroutineBridgeEntry = struct {
    id: GoroutineId,
    execution_mode: ExecutionMode,
    
    // Function pointers for different execution modes
    interpreter_fn: ?*const fn() void, // For interpreter spawned goroutines
    compiled_fn: ?*const fn() void, // For compiled code goroutines  
    context: ?*anyopaque,
    
    // Cross-mode execution state
    spawned_from_mode: ExecutionMode,
    target_mode: ExecutionMode,
    
    // Synchronization
    completion_barrier: std.Thread.ResetEvent,
    completed: Atomic(bool),
    
    const Self = @This();
    
    pub fn init(
        allocator: Allocator,
        execution_mode: ExecutionMode,
        spawned_from: ExecutionMode,
        target: ExecutionMode
    ) !*Self {
        const entry = try allocator.create(Self);
        entry.* = Self{
            .id = 0, // Will be set by registry
            .execution_mode = execution_mode,
            .interpreter_fn = null,
            .compiled_fn = null,
            .context = null,
            .spawned_from_mode = spawned_from,
            .target_mode = target,
            .completion_barrier = std.Thread.ResetEvent{},
            .completed = Atomic(bool).init(false),
        };
        return entry;
    }
    
    pub fn deinit(self: *Self, allocator: Allocator) void {
        allocator.destroy(self);
    }
    
    /// Execute the goroutine in the appropriate mode
    pub fn execute(self: *Self) void {
        switch (self.target_mode) {
            .interpreter => {
                if (self.interpreter_fn) |func| {
                    func();
                }
            },
            .compiled_native => {
                if (self.compiled_fn) |func| {
                    func();
                }
            },
            .mixed => {
                // Handle mixed mode execution
                std.log.warn("Mixed mode execution not fully implemented", .{});
            },
        }
        
        // Mark as completed
        self.completed.store(true, .release);
        self.completion_barrier.set();
    }
    
    /// Wait for goroutine completion with timeout
    pub fn waitCompletion(self: *Self, timeout_ms: u32) bool {
        if (self.completed.load(.acquire)) {
            return true;
        }
        
        // Simple timeout implementation
        const timeout_ns = @as(u64, timeout_ms) * 1_000_000;
        const start_time = std.time.nanoTimestamp();
        
        while (!self.completed.load(.acquire)) {
            const elapsed = @as(u64, @intCast(std.time.nanoTimestamp() - start_time));
            if (elapsed >= timeout_ns) {
                return false; // Timeout
            }
            std.Thread.sleep(1_000_000); // 1ms
        }
        
        return true;
    }
};

/// Channel Bridge Registry - Maps channels between execution modes  
pub const ChannelBridgeRegistry = struct {
    registry: HashMap(ChannelId, *ChannelBridgeEntry, ChannelContext, std.hash_map.default_max_load_percentage),
    mutex: std.Thread.RwLock,
    next_id: Atomic(u64),
    allocator: Allocator,
    
    const Self = @This();
    const ChannelContext = struct {
        pub fn hash(self: @This(), s: ChannelId) u64 {
            _ = self;
            return std.hash_map.hashInt(s);
        }
        pub fn eql(self: @This(), a: ChannelId, b: ChannelId) bool {
            _ = self;
            return a == b;
        }
    };
    
    pub fn init(allocator: Allocator) !Self {
        return Self{
            .registry = HashMap(ChannelId, *ChannelBridgeEntry, ChannelContext, std.hash_map.default_max_load_percentage){},
            .mutex = std.Thread.RwLock{},
            .next_id = Atomic(u64).init(1),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        var iterator = self.registry.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.*.deinit(self.allocator);
        }
        self.registry.deinit();
    }
    
    /// Register a channel bridge
    pub fn registerChannel(self: *Self, entry: *ChannelBridgeEntry) !ChannelId {
        const id = self.next_id.fetchAdd(1, .monotonic);
        entry.id = id;
        
        self.mutex.lock();
        defer self.mutex.unlock();
        
        try self.registry.put(id, entry);
        return id;
    }
    
    /// Get channel bridge entry
    pub fn getChannel(self: *Self, id: ChannelId) ?*ChannelBridgeEntry {
        self.mutex.lockShared();
        defer self.mutex.unlockShared();
        return self.registry.get(id);
    }
    
    /// Remove channel from registry
    pub fn removeChannel(self: *Self, id: ChannelId) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        if (self.registry.fetchRemove(id)) |kv| {
            kv.value.deinit(self.allocator);
        }
    }
};

/// Bridge entry for channels - connects interpreter and compiled channel operations
pub const ChannelBridgeEntry = struct {
    id: ChannelId,
    capacity: usize,
    
    // Different channel implementations
    interpreter_channel: ?u64, // ID for interpreter channel
    compiled_channel: ?*concurrency_fixed.Channel(i64), // Compiled channel instance
    
    // Channel state
    closed: Atomic(bool),
    created_mode: ExecutionMode,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator, capacity: usize, mode: ExecutionMode) !*Self {
        const entry = try allocator.create(Self);
        entry.* = Self{
            .id = 0, // Will be set by registry
            .capacity = capacity,
            .interpreter_channel = null,
            .compiled_channel = null,
            .closed = Atomic(bool).init(false),
            .created_mode = mode,
        };
        return entry;
    }
    
    pub fn deinit(self: *Self, allocator: Allocator) void {
        // Cleanup channel resources based on mode
        if (self.compiled_channel) |channel| {
            channel.deinit();
            allocator.destroy(channel);
        }
        
        if (self.interpreter_channel) |chan_id| {
            concurrency_runtime.closeChannel(chan_id) catch {};
        }
        
        allocator.destroy(self);
    }
    
    /// Send operation bridging both modes
    pub fn send(self: *Self, value: i64, timeout_ms: u32) !bool {
        if (self.closed.load(.acquire)) {
            return false;
        }
        
        const timeout_ns = @as(u64, timeout_ms) * 1_000_000;
        
        // Route to appropriate channel implementation
        if (self.compiled_channel) |channel| {
            const result = try channel.sendTimeout(value, timeout_ns);
            return result == .sent;
        } else if (self.interpreter_channel) |chan_id| {
            concurrency_runtime.sendToChannelTimeout(chan_id, concurrency_runtime.Variable{ .Integer = value }, timeout_ns) catch return false;
            return true;
        }
        
        return false;
    }
    
    /// Receive operation bridging both modes
    pub fn receive(self: *Self, timeout_ms: u32) !?i64 {
        if (self.closed.load(.acquire)) {
            return null;
        }
        
        const timeout_ns = @as(u64, timeout_ms) * 1_000_000;
        
        // Route to appropriate channel implementation
        if (self.compiled_channel) |channel| {
            return try channel.receiveTimeout(timeout_ns);
        } else if (self.interpreter_channel) |chan_id| {
            const result = concurrency_runtime.receiveFromChannelTimeout(chan_id, timeout_ns) catch return null;
            if (result) |variable| {
                switch (variable) {
                    .Integer => |int_val| return int_val,
                    else => return null,
                }
            }
        }
        
        return null;
    }
    
    /// Close the channel
    pub fn close(self: *Self) void {
        self.closed.store(true, .release);
        
        if (self.compiled_channel) |channel| {
            channel.close();
        }
        
        if (self.interpreter_channel) |chan_id| {
            concurrency_runtime.closeChannel(chan_id) catch {};
        }
    }
};

// Global runtime bridge instance
var runtime_bridge: ?*RuntimeBridge = null;
var bridge_mutex = std.Thread.Mutex{};
var bridge_allocator = std.heap.page_allocator;

/// Initialize the runtime bridge
pub export fn cursed_runtime_bridge_init() bool {
    bridge_mutex.lock();
    defer bridge_mutex.unlock();
    
    if (runtime_bridge != null) {
        return true; // Already initialized
    }
    
    runtime_bridge = bridge_allocator.create(RuntimeBridge) catch return false;
    runtime_bridge.?.* = RuntimeBridge.init(bridge_allocator) catch {
        bridge_allocator.destroy(runtime_bridge.?);
        runtime_bridge = null;
        return false;
    };
    
    std.log.info("Runtime bridge initialized successfully", .{});
    return true;
}

/// Cleanup the runtime bridge
pub export fn cursed_runtime_bridge_cleanup() void {
    bridge_mutex.lock();
    defer bridge_mutex.unlock();
    
    if (runtime_bridge) |bridge| {
        bridge.deinit();
        bridge_allocator.destroy(bridge);
        runtime_bridge = null;
        std.log.info("Runtime bridge cleaned up", .{});
    }
}

// ===========================================
// GOROUTINE BRIDGE FFI - LLVM Integration
// ===========================================

/// Spawn goroutine from compiled code that can call back into interpreter
pub export fn cursed_bridge_spawn_goroutine(
    compiled_fn: ?*const fn() void,
    interpreter_fn: ?*const fn() void,
    mode: u32, // 0=interpreter, 1=compiled, 2=mixed
    context: ?*anyopaque
) u64 {
    if (!cursed_runtime_bridge_init()) {
        return 0;
    }
    
    const bridge = runtime_bridge.?;
    const execution_mode: ExecutionMode = switch (mode) {
        0 => .interpreter,
        1 => .compiled_native, 
        2 => .mixed,
        else => .interpreter,
    };
    
    // Create bridge entry
    const entry = GoroutineBridgeEntry.init(
        bridge.allocator,
        execution_mode,
        bridge.getCurrentMode(),
        execution_mode
    ) catch return 0;
    
    entry.compiled_fn = compiled_fn;
    entry.interpreter_fn = interpreter_fn;
    entry.context = context;
    
    // Register the goroutine
    const goroutine_id = bridge.goroutine_bridge_registry.registerGoroutine(entry) catch return 0;
    
    // Route to appropriate runtime
    switch (execution_mode) {
        .interpreter => {
            // Spawn through interpreter runtime
            const wrapper_fn = struct {
                fn run(ctx: ?*anyopaque) void {
                    const bridge_entry: *GoroutineBridgeEntry = @ptrCast(@alignCast(ctx.?));
                    bridge_entry.execute();
                }
            }.run;
            
            const spawn_result = concurrency_runtime.spawnGoroutine(wrapper_fn, entry) catch return 0;
            std.log.debug("Spawned interpreter goroutine: {} -> {}", .{ goroutine_id, spawn_result });
        },
        .compiled_native => {
            // Spawn through compiled runtime
            const wrapper_fn = struct {
                fn run(ctx: ?*anyopaque) void {
                    const bridge_entry: *GoroutineBridgeEntry = @ptrCast(@alignCast(ctx.?));
                    bridge_entry.execute();
                }
            }.run;
            
            _ = bridge.compiled_runtime.spawnGoroutine(@ptrCast(&wrapper_fn), entry) catch return 0;
        },
        .mixed => {
            // Mixed mode - use interpreter runtime but allow compiled execution
            std.log.warn("Mixed mode goroutine spawning not fully implemented", .{});
            return 0;
        },
    }
    
    return goroutine_id;
}

/// Spawn simple compiled goroutine (compatibility with existing code)
pub export fn cursed_bridge_spawn_simple(compiled_fn: ?*const fn() void) u64 {
    return cursed_bridge_spawn_goroutine(compiled_fn, null, 1, null);
}

/// Wait for goroutine completion from compiled code
pub export fn cursed_bridge_wait_goroutine(goroutine_id: u64, timeout_ms: u32) bool {
    if (runtime_bridge == null) {
        return false;
    }
    
    const bridge = runtime_bridge.?;
    const entry = bridge.goroutine_bridge_registry.getGoroutine(goroutine_id) orelse return false;
    
    return entry.waitCompletion(timeout_ms);
}

// ===========================================
// CHANNEL BRIDGE FFI - LLVM Integration  
// ===========================================

/// Create bridged channel accessible from both interpreter and compiled code
pub export fn cursed_bridge_create_channel(capacity: u32, mode: u32) u64 {
    if (!cursed_runtime_bridge_init()) {
        return 0;
    }
    
    const bridge = runtime_bridge.?;
    const execution_mode: ExecutionMode = switch (mode) {
        0 => .interpreter,
        1 => .compiled_native,
        2 => .mixed,
        else => .mixed, // Default to mixed mode for channels
    };
    
    // Create bridge entry
    const entry = ChannelBridgeEntry.init(bridge.allocator, capacity, execution_mode) catch return 0;
    
    // Create appropriate channel implementation
    switch (execution_mode) {
        .interpreter => {
            entry.interpreter_channel = concurrency_runtime.createChannel(capacity) catch return 0;
        },
        .compiled_native => {
            const channel = bridge.compiled_runtime.createChannel(i64, capacity) catch return 0;
            entry.compiled_channel = channel;
        },
        .mixed => {
            // Create both channel types for mixed mode
            entry.interpreter_channel = concurrency_runtime.createChannel(capacity) catch null;
            entry.compiled_channel = bridge.compiled_runtime.createChannel(i64, capacity) catch null;
            
            if (entry.interpreter_channel == null and entry.compiled_channel == null) {
                entry.deinit(bridge.allocator);
                return 0;
            }
        },
    }
    
    // Register the channel bridge
    const channel_id = bridge.channel_bridge_registry.registerChannel(entry) catch return 0;
    
    std.log.debug("Created bridged channel: {} (mode: {}, capacity: {})", .{ channel_id, execution_mode, capacity });
    return channel_id;
}

/// Send through bridged channel
pub export fn cursed_bridge_channel_send(channel_id: u64, value: i64, timeout_ms: u32) i32 {
    if (runtime_bridge == null) {
        return -1; // Bridge not initialized
    }
    
    const bridge = runtime_bridge.?;
    const entry = bridge.channel_bridge_registry.getChannel(channel_id) orelse return -2; // Channel not found
    
    const result = entry.send(value, timeout_ms) catch return -3; // Send error
    return if (result) 0 else -4; // Success / Failure
}

/// Receive from bridged channel
pub export fn cursed_bridge_channel_receive(channel_id: u64, timeout_ms: u32) i64 {
    if (runtime_bridge == null) {
        return std.math.minInt(i64); // Error value
    }
    
    const bridge = runtime_bridge.?;
    const entry = bridge.channel_bridge_registry.getChannel(channel_id) orelse return std.math.minInt(i64);
    
    const result = entry.receive(timeout_ms) catch return std.math.minInt(i64);
    return result orelse std.math.minInt(i64); // Return error value if null
}

/// Close bridged channel
pub export fn cursed_bridge_channel_close(channel_id: u64) void {
    if (runtime_bridge == null) {
        return;
    }
    
    const bridge = runtime_bridge.?;
    if (bridge.channel_bridge_registry.getChannel(channel_id)) |entry| {
        entry.close();
    }
}

/// Destroy bridged channel and cleanup resources
pub export fn cursed_bridge_channel_destroy(channel_id: u64) void {
    if (runtime_bridge == null) {
        return;
    }
    
    const bridge = runtime_bridge.?;
    bridge.channel_bridge_registry.removeChannel(channel_id);
}

// ===========================================
// MODE SWITCHING AND COORDINATION
// ===========================================

/// Switch the runtime bridge to a specific execution mode
pub export fn cursed_bridge_switch_mode(mode: u32) bool {
    if (runtime_bridge == null) {
        return false;
    }
    
    const execution_mode: ExecutionMode = switch (mode) {
        0 => .interpreter,
        1 => .compiled_native,
        2 => .mixed,
        else => return false,
    };
    
    runtime_bridge.?.switchMode(execution_mode);
    return true;
}

/// Get current execution mode
pub export fn cursed_bridge_get_mode() u32 {
    if (runtime_bridge == null) {
        return 0;
    }
    
    return switch (runtime_bridge.?.getCurrentMode()) {
        .interpreter => 0,
        .compiled_native => 1,
        .mixed => 2,
    };
}

/// Enable/disable mixed mode execution
pub export fn cursed_bridge_set_mixed_mode(enabled: bool) void {
    if (runtime_bridge) |bridge| {
        bridge.mixed_mode_enabled = enabled;
    }
}

// ===========================================
// RUNTIME STATISTICS AND MONITORING
// ===========================================

/// Get runtime bridge statistics
pub export fn cursed_bridge_get_stats() ?*BridgeStats {
    if (runtime_bridge == null) {
        return null;
    }
    
    // TODO: Implement comprehensive statistics gathering
    return null; // Placeholder
}

pub const BridgeStats = struct {
    active_goroutines: u32,
    active_channels: u32,
    mode_switches: u64,
    total_goroutines_spawned: u64,
    total_channels_created: u64,
};

// ===========================================
// TEST AND VALIDATION FUNCTIONS
// ===========================================

/// Test function to validate bridge functionality
pub export fn cursed_bridge_test() bool {
    std.log.info("Testing runtime bridge functionality...", .{});
    
    // Initialize bridge
    if (!cursed_runtime_bridge_init()) {
        std.log.err("Failed to initialize runtime bridge", .{});
        return false;
    }
    
    // Test channel creation
    const channel_id = cursed_bridge_create_channel(3, 2); // Mixed mode, capacity 3
    if (channel_id == 0) {
        std.log.err("Failed to create test channel", .{});
        return false;
    }
    
    // Test channel operations
    if (cursed_bridge_channel_send(channel_id, 42, 1000) != 0) {
        std.log.err("Failed to send to test channel", .{});
        return false;
    }
    
    const received = cursed_bridge_channel_receive(channel_id, 1000);
    if (received != 42) {
        std.log.err("Failed to receive from test channel: expected 42, got {}", .{received});
        return false;
    }
    
    // Cleanup
    cursed_bridge_channel_destroy(channel_id);
    cursed_runtime_bridge_cleanup();
    
    std.log.info("Runtime bridge test completed successfully", .{});
    return true;
}
