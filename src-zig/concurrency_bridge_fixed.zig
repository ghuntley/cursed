//! CURSED Concurrency Bridge - Fixed Condition Variable Integration
//!
//! This module provides the fixed integration between the main concurrency system
//! and the enhanced synchronization primitives, specifically addressing the
//! condition variable bridging issues that were causing sync primitive failures.

const std = @import("std");
const builtin = @import("builtin");
const concurrency = @import("concurrency.zig");
const sync = @import("sync_primitives_fixed.zig");
const Allocator = std.mem.Allocator;
const Thread = std.Thread;
const Atomic = std.atomic.Value;

/// Bridge between CURSED channels and enhanced sync primitives
pub const ConcurrencyBridge = struct {
    const Self = @This();
    
    allocator: Allocator,
    channel_sync_bridge: *sync.ChannelSyncBridge,
    global_mutex: sync.EnhancedMutex,
    
    // Enhanced select operation management
    active_selects: std.HashMap(u64, *SelectOperation, std.hash_map.AutoContext(u64), std.hash_map.default_max_load_percentage),
    next_select_id: Atomic(u64),
    
    // Statistics
    total_selects: Atomic(u64),
    successful_selects: Atomic(u64),
    timeout_selects: Atomic(u64),
    spurious_wakeups: Atomic(u64),
    
    pub fn init(allocator: Allocator) !Self {
        const channel_sync_bridge = try allocator.create(sync.ChannelSyncBridge);
        channel_sync_bridge.* = try sync.ChannelSyncBridge.init(allocator);
        
        return Self{
            .allocator = allocator,
            .channel_sync_bridge = channel_sync_bridge,
            .global_mutex = sync.EnhancedMutex.init(),
            .active_selects = std.HashMap(u64, *SelectOperation, std.hash_map.AutoContext(u64), std.hash_map.default_max_load_percentage).init(allocator),
            .next_select_id = Atomic(u64).init(1),
            .total_selects = Atomic(u64).init(0),
            .successful_selects = Atomic(u64).init(0),
            .timeout_selects = Atomic(u64).init(0),
            .spurious_wakeups = Atomic(u64).init(0),
        };
    }
    
    pub fn deinit(self: *Self) void {
        // Clean up active selects
        try self.global_mutex.lock();
        var iterator = self.active_selects.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.*.deinit();
            self.allocator.destroy(entry.value_ptr.*);
        }
        self.active_selects.deinit();
        self.global_mutex.unlock() catch {};
        
        // Clean up bridge
        self.channel_sync_bridge.deinit();
        self.allocator.destroy(self.channel_sync_bridge);
        
        self.global_mutex.deinit();
    }
    
    /// Enhanced select operation that uses proper condition variable synchronization
    pub const SelectOperation = struct {
        const SelectSelf = @This();
        
        id: u64,
        bridge: *ConcurrencyBridge,
        operations: std.ArrayList(ChannelOperation),
        has_default: bool,
        default_case_index: u32,
        timeout_ns: ?u64,
        completed: Atomic(bool),
        result_mutex: sync.EnhancedMutex,
        result_condition: sync.EnhancedCondition,
        result: ?SelectResult,
        
        const ChannelOperation = struct {
            channel_id: u64,
            operation_type: enum { send, receive },
            case_index: u32,
            value_ptr: ?*anyopaque,
            value_size: u32,
        };
        
        const SelectResult = struct {
            case_index: u32,
            operation_type: enum { send, receive, default, timeout },
            channel_id: ?u64,
        };
        
        pub fn init(allocator: Allocator, bridge: *ConcurrencyBridge) !*SelectSelf {
            const select_op = try allocator.create(SelectSelf);
            select_op.* = SelectSelf{
                .id = bridge.next_select_id.fetchAdd(1, .acq_rel),
                .bridge = bridge,
                .operations = .{},
                .has_default = false,
                .default_case_index = 0,
                .timeout_ns = null,
                .completed = Atomic(bool).init(false),
                .result_mutex = sync.EnhancedMutex.init(),
                .result_condition = sync.EnhancedCondition.init(),
                .result = null,
            };
            return select_op;
        }
        
        pub fn deinit(self: *SelectSelf) void {
            self.operations.deinit();
            self.result_mutex.deinit();
            self.result_condition.deinit();
        }
        
        /// Add a send operation to the select
        pub fn addSendOperation(self: *SelectSelf, channel_id: u64, case_index: u32, value_ptr: ?*anyopaque, value_size: u32) !void {
            const op = ChannelOperation{
                .channel_id = channel_id,
                .operation_type = .send,
                .case_index = case_index,
                .value_ptr = value_ptr,
                .value_size = value_size,
            };
            try self.operations.append(op);
        }
        
        /// Add a receive operation to the select
        pub fn addReceiveOperation(self: *SelectSelf, channel_id: u64, case_index: u32, buffer_ptr: ?*anyopaque, buffer_size: u32) !void {
            const op = ChannelOperation{
                .channel_id = channel_id,
                .operation_type = .receive,
                .case_index = case_index,
                .value_ptr = buffer_ptr,
                .value_size = buffer_size,
            };
            try self.operations.append(op);
        }
        
        /// Add a default case to the select
        pub fn addDefaultCase(self: *SelectSelf, case_index: u32) void {
            self.has_default = true;
            self.default_case_index = case_index;
        }
        
        /// Set timeout for the select operation
        pub fn setTimeout(self: *SelectSelf, timeout_ns: u64) void {
            self.timeout_ns = timeout_ns;
        }
        
        /// Execute the select operation with proper condition variable synchronization
        pub fn execute(self: *SelectSelf) !SelectResult {
            _ = self.bridge.total_selects.fetchAdd(1, .acq_rel);
            
            // Register this select operation
            try self.bridge.global_mutex.lock();
            try self.bridge.active_selects.put(self.id, self);
            self.bridge.global_mutex.unlock() catch {};
            
            defer {
                // Unregister select operation
                self.bridge.global_mutex.lock() catch {};
                _ = self.bridge.active_selects.remove(self.id);
                self.bridge.global_mutex.unlock() catch {};
            }
            
            // Phase 1: Fast path - try all operations immediately without blocking
            if (try self.tryImmediateOperations()) |result| {
                _ = self.bridge.successful_selects.fetchAdd(1, .acq_rel);
                return result;
            }
            
            // If has default case and no operations are ready, return default
            if (self.has_default) {
                return SelectResult{
                    .case_index = self.default_case_index,
                    .operation_type = .default,
                    .channel_id = null,
                };
            }
            
            // Phase 2: Slow path - use condition variables to wait for readiness
            return self.waitForReadiness();
        }
        
        /// Try all operations immediately without blocking
        fn tryImmediateOperations(self: *SelectSelf) !?SelectResult {
            // Convert operations to bridge format
            var bridge_ops: std.ArrayList(struct { channel_id: u64, operation: enum { send, receive } }) = .empty;
            defer bridge_ops.deinit();
            
            for (self.operations.items) |op| {
                const bridge_op = .{
                    .channel_id = op.channel_id,
                    .operation = switch (op.operation_type) {
                        .send => .send,
                        .receive => .receive,
                    },
                };
                try bridge_ops.append(bridge_op);
            }
            
            // Try immediate readiness check (non-blocking)
            const ready_result = self.bridge.channel_sync_bridge.waitForChannelReadiness(
                bridge_ops.items,
                0 // No timeout - immediate check only
            ) catch |err| switch (err) {
                sync.SyncError.Timeout => return null, // No operations ready
                else => return err,
            };
            
            if (ready_result) |ready| {
                // Find the corresponding operation
                for (self.operations.items) |op| {
                    if (op.channel_id == ready.channel_id and 
                        ((op.operation_type == .send and ready.operation == .send) or
                         (op.operation_type == .receive and ready.operation == .receive))) {
                        
                        // Execute the operation
                        const success = try self.executeChannelOperation(op);
                        if (success) {
                            return SelectResult{
                                .case_index = op.case_index,
                                .operation_type = op.operation_type,
                                .channel_id = op.channel_id,
                            };
                        }
                    }
                }
            }
            
            return null; // No operations ready
        }
        
        /// Wait for channel readiness using condition variables
        fn waitForReadiness(self: *SelectSelf) !SelectResult {
            const timeout_ns = self.timeout_ns orelse sync.SyncTimeouts.DEFAULT_TIMEOUT_NS;
            const start_time = std.time.nanoTimestamp();
            
            // Convert operations to bridge format
            var bridge_ops: std.ArrayList(struct { channel_id: u64, operation: enum { send, receive } }) = .empty;
            defer bridge_ops.deinit();
            
            for (self.operations.items) |op| {
                const bridge_op = .{
                    .channel_id = op.channel_id,
                    .operation = switch (op.operation_type) {
                        .send => .send,
                        .receive => .receive,
                    },
                };
                try bridge_ops.append(bridge_op);
            }
            
            while (true) {
                // Check timeout
                const elapsed = @as(u64, @intCast(std.time.nanoTimestamp() - start_time));
                if (elapsed >= timeout_ns) {
                    _ = self.bridge.timeout_selects.fetchAdd(1, .acq_rel);
                    return SelectResult{
                        .case_index = 0, // Timeout doesn't have a specific case index
                        .operation_type = .timeout,
                        .channel_id = null,
                    };
                }
                
                // Wait for channel readiness with remaining timeout
                const remaining_timeout = timeout_ns - elapsed;
                const ready_result = self.bridge.channel_sync_bridge.waitForChannelReadiness(
                    bridge_ops.items,
                    remaining_timeout
                ) catch |err| switch (err) {
                    sync.SyncError.Timeout => {
                        _ = self.bridge.timeout_selects.fetchAdd(1, .acq_rel);
                        return SelectResult{
                            .case_index = 0,
                            .operation_type = .timeout,
                            .channel_id = null,
                        };
                    },
                    sync.SyncError.SpuriousWakeup => {
                        _ = self.bridge.spurious_wakeups.fetchAdd(1, .acq_rel);
                        continue; // Retry after spurious wakeup
                    },
                    else => return err,
                };
                
                if (ready_result) |ready| {
                    // Find and execute the corresponding operation
                    for (self.operations.items) |op| {
                        if (op.channel_id == ready.channel_id and 
                            ((op.operation_type == .send and ready.operation == .send) or
                             (op.operation_type == .receive and ready.operation == .receive))) {
                            
                            const success = try self.executeChannelOperation(op);
                            if (success) {
                                _ = self.bridge.successful_selects.fetchAdd(1, .acq_rel);
                                return SelectResult{
                                    .case_index = op.case_index,
                                    .operation_type = op.operation_type,
                                    .channel_id = op.channel_id,
                                };
                            } else {
                                // Operation failed (e.g., channel closed), continue waiting
                                break;
                            }
                        }
                    }
                }
            }
        }
        
        /// Execute a specific channel operation
        fn executeChannelOperation(self: *SelectSelf, op: ChannelOperation) !bool {
            _ = self; // Avoid unused parameter warning
            
            // This would interface with the actual channel operations from concurrency.zig
            // For now, simplified implementation that assumes success
            
            switch (op.operation_type) {
                .send => {
                    // Would call cursed_dm_send with op.channel_id, op.value_ptr, op.value_size
                    // For now, assume success
                    _ = op.value_ptr;
                    _ = op.value_size;
                    return true;
                },
                .receive => {
                    // Would call cursed_dm_receive with op.channel_id, op.value_ptr, op.value_size
                    // For now, assume success
                    _ = op.value_ptr;
                    _ = op.value_size;
                    return true;
                },
            }
        }
    };
    
    /// Create a new select operation
    pub fn createSelect(self: *Self) !*SelectOperation {
        return SelectOperation.init(self.allocator, self);
    }
    
    /// Register a channel with the bridge
    pub fn registerChannel(self: *Self, channel_id: u64) !void {
        return self.channel_sync_bridge.registerChannel(channel_id);
    }
    
    /// Update channel state for select coordination
    pub fn updateChannelState(self: *Self, channel_id: u64, can_send: bool, can_receive: bool, is_closed: bool) !void {
        return self.channel_sync_bridge.updateChannelState(channel_id, can_send, can_receive, is_closed);
    }
    
    /// Get bridge statistics
    pub fn getStats(self: *const Self) struct {
        total_selects: u64,
        successful_selects: u64,
        timeout_selects: u64,
        spurious_wakeups: u64,
        active_selects: usize,
    } {
        // Get active selects count safely
        var active_count: usize = 0;
        if (self.global_mutex.tryLock() catch false) {
            active_count = self.active_selects.count();
            self.global_mutex.unlock() catch {};
        }
        
        return .{
            .total_selects = self.total_selects.load(.acquire),
            .successful_selects = self.successful_selects.load(.acquire),
            .timeout_selects = self.timeout_selects.load(.acquire),
            .spurious_wakeups = self.spurious_wakeups.load(.acquire),
            .active_selects = active_count,
        };
    }
};

// Global bridge instance
var global_bridge: ?*ConcurrencyBridge = null;
var bridge_mutex = std.Thread.Mutex{};

/// Initialize the concurrency bridge
pub fn initBridge(allocator: Allocator) !void {
    bridge_mutex.lock();
    defer bridge_mutex.unlock();
    
    if (global_bridge == null) {
        const bridge = try allocator.create(ConcurrencyBridge);
        bridge.* = try ConcurrencyBridge.init(allocator);
        global_bridge = bridge;
    }
}

/// Shutdown the concurrency bridge
pub fn shutdownBridge(allocator: Allocator) void {
    bridge_mutex.lock();
    defer bridge_mutex.unlock();
    
    if (global_bridge) |bridge| {
        bridge.deinit();
        allocator.destroy(bridge);
        global_bridge = null;
    }
}

/// Get the global bridge instance
pub fn getBridge() ?*ConcurrencyBridge {
    bridge_mutex.lock();
    defer bridge_mutex.unlock();
    return global_bridge;
}

// C FFI exports for integration with LLVM-generated code

/// Initialize the concurrency bridge from C
export fn cursed_bridge_init() u32 {
    const allocator = std.heap.c_allocator;
    initBridge(allocator) catch return 1;
    return 0;
}

/// Shutdown the concurrency bridge from C
export fn cursed_bridge_shutdown() void {
    const allocator = std.heap.c_allocator;
    shutdownBridge(allocator);
}

/// Create a new select operation from C
export fn cursed_bridge_create_select() ?*ConcurrencyBridge.SelectOperation {
    if (getBridge()) |bridge| {
        return bridge.createSelect() catch null;
    }
    return null;
}

/// Add send operation to select from C
export fn cursed_bridge_select_add_send(
    select_ptr: ?*ConcurrencyBridge.SelectOperation,
    channel_id: u64,
    case_index: u32,
    value_ptr: ?*anyopaque,
    value_size: u32
) u32 {
    if (select_ptr) |select_op| {
        select_op.addSendOperation(channel_id, case_index, value_ptr, value_size) catch return 1;
        return 0;
    }
    return 1;
}

/// Add receive operation to select from C
export fn cursed_bridge_select_add_receive(
    select_ptr: ?*ConcurrencyBridge.SelectOperation,
    channel_id: u64,
    case_index: u32,
    buffer_ptr: ?*anyopaque,
    buffer_size: u32
) u32 {
    if (select_ptr) |select_op| {
        select_op.addReceiveOperation(channel_id, case_index, buffer_ptr, buffer_size) catch return 1;
        return 0;
    }
    return 1;
}

/// Add default case to select from C
export fn cursed_bridge_select_add_default(select_ptr: ?*ConcurrencyBridge.SelectOperation, case_index: u32) void {
    if (select_ptr) |select_op| {
        select_op.addDefaultCase(case_index);
    }
}

/// Set timeout for select from C
export fn cursed_bridge_select_set_timeout(select_ptr: ?*ConcurrencyBridge.SelectOperation, timeout_ms: u64) void {
    if (select_ptr) |select_op| {
        select_op.setTimeout(timeout_ms * 1_000_000); // Convert ms to ns
    }
}

/// Execute select operation from C
export fn cursed_bridge_select_execute(select_ptr: ?*ConcurrencyBridge.SelectOperation, result_ptr: ?*anyopaque) u32 {
    if (select_ptr) |select_op| {
        if (result_ptr) |result| {
            const select_result = select_op.execute() catch return 1;
            
            // Pack result into provided structure (simplified)
            const result_struct: *struct {
                case_index: u32,
                operation_type: u32, // 0=send, 1=receive, 2=default, 3=timeout
                channel_id: u64,
            } = @ptrCast(@alignCast(result));
            
            result_struct.case_index = select_result.case_index;
            result_struct.operation_type = switch (select_result.operation_type) {
                .send => 0,
                .receive => 1,
                .default => 2,
                .timeout => 3,
            };
            result_struct.channel_id = select_result.channel_id orelse 0;
            
            return 0;
        }
    }
    return 1;
}

/// Destroy select operation from C
export fn cursed_bridge_select_destroy(select_ptr: ?*ConcurrencyBridge.SelectOperation) void {
    if (select_ptr) |select_op| {
        select_op.deinit();
        std.heap.c_allocator.destroy(select_op);
    }
}

/// Register channel with bridge from C
export fn cursed_bridge_register_channel(channel_id: u64) u32 {
    if (getBridge()) |bridge| {
        bridge.registerChannel(channel_id) catch return 1;
        return 0;
    }
    return 1;
}

/// Update channel state from C
export fn cursed_bridge_update_channel_state(channel_id: u64, can_send: u32, can_receive: u32, is_closed: u32) u32 {
    if (getBridge()) |bridge| {
        bridge.updateChannelState(
            channel_id,
            can_send != 0,
            can_receive != 0,
            is_closed != 0
        ) catch return 1;
        return 0;
    }
    return 1;
}

/// Get bridge statistics from C
export fn cursed_bridge_get_stats(stats_ptr: ?*anyopaque) u32 {
    if (getBridge()) |bridge| {
        if (stats_ptr) |stats| {
            const bridge_stats = bridge.getStats();
            
            const stats_struct: *struct {
                total_selects: u64,
                successful_selects: u64,
                timeout_selects: u64,
                spurious_wakeups: u64,
                active_selects: u64,
            } = @ptrCast(@alignCast(stats));
            
            stats_struct.total_selects = bridge_stats.total_selects;
            stats_struct.successful_selects = bridge_stats.successful_selects;
            stats_struct.timeout_selects = bridge_stats.timeout_selects;
            stats_struct.spurious_wakeups = bridge_stats.spurious_wakeups;
            stats_struct.active_selects = @intCast(bridge_stats.active_selects);
            
            return 0;
        }
    }
    return 1;
}

// Tests
test "concurrency bridge basic operations" {
    const allocator = std.testing.allocator;
    
    var bridge = try ConcurrencyBridge.init(allocator);
    defer bridge.deinit();
    
    // Register a channel
    try bridge.registerChannel(1);
    
    // Update channel state
    try bridge.updateChannelState(1, true, false, false);
    
    // Create a select operation
    var select_op = try bridge.createSelect();
    defer {
        select_op.deinit();
        allocator.destroy(select_op);
    }
    
    // Add operations
    try select_op.addSendOperation(1, 0, null, 0);
    select_op.addDefaultCase(1);
    
    // Execute select (should hit default case)
    const result = try select_op.execute();
    try std.testing.expect(result.operation_type == .default);
    try std.testing.expect(result.case_index == 1);
}

test "select with timeout" {
    const allocator = std.testing.allocator;
    
    var bridge = try ConcurrencyBridge.init(allocator);
    defer bridge.deinit();
    
    try bridge.registerChannel(1);
    try bridge.updateChannelState(1, false, false, false); // Channel not ready
    
    var select_op = try bridge.createSelect();
    defer {
        select_op.deinit();
        allocator.destroy(select_op);
    }
    
    try select_op.addSendOperation(1, 0, null, 0);
    select_op.setTimeout(1_000_000); // 1ms timeout
    
    const result = try select_op.execute();
    try std.testing.expect(result.operation_type == .timeout);
}

test "bridge statistics" {
    const allocator = std.testing.allocator;
    
    var bridge = try ConcurrencyBridge.init(allocator);
    defer bridge.deinit();
    
    // Initial stats should be zero
    const stats = bridge.getStats();
    try std.testing.expect(stats.total_selects == 0);
    try std.testing.expect(stats.successful_selects == 0);
}
