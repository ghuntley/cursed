//! CURSED Concurrency Runtime Bridge
//!
//! This module provides the C-compatible runtime functions that LLVM-generated
//! code calls for concurrency operations. It bridges between the generated code
//! and the full Zig concurrency implementation.

const std = @import("std");
const concurrency = @import("concurrency.zig");
const allocator = std.heap.page_allocator;

// Global scheduler instance
var global_scheduler: ?*concurrency.Scheduler = null;
var scheduler_initialized = false;

/// Initialize the concurrency runtime (called once at program start)
pub export fn cursed_concurrency_init() void {
    if (scheduler_initialized) return;
    
    const config = concurrency.SchedulerConfig.default();
    concurrency.initializeScheduler(allocator, config) catch |err| {
        std.debug.print("Failed to initialize scheduler: {}\n", .{err});
        return;
    };
    
    global_scheduler = concurrency.getScheduler();
    scheduler_initialized = true;
}

/// Cleanup concurrency runtime (called at program exit)
pub export fn cursed_concurrency_cleanup() void {
    if (!scheduler_initialized) return;
    
    concurrency.shutdownScheduler(allocator);
    global_scheduler = null;
    scheduler_initialized = false;
}

/// Goroutine entry point wrapper
fn goroutineWrapper(context: ?*anyopaque) void {
    if (context) |ctx| {
        const entry_fn: concurrency.GoroutineEntry = @ptrCast(@alignCast(ctx));
        entry_fn(null);
    }
}

/// Spawn a goroutine (implements `stan` keyword)
/// Returns goroutine ID on success, 0 on failure
pub export fn cursed_stan_goroutine(function_ptr: ?*anyopaque, context: ?*anyopaque) u64 {
    if (!scheduler_initialized) {
        cursed_concurrency_init();
    }
    
    const scheduler = global_scheduler orelse return 0;
    
    // For now, use a simple wrapper. In full implementation, this would
    // properly handle the function pointer and context
    _ = function_ptr;
    _ = context;
    
    const goroutine_id = scheduler.spawn(goroutineWrapper, null) catch return 0;
    return goroutine_id;
}

/// Yield current goroutine (implements `yolo` keyword)
/// Returns true if yield was successful
pub export fn cursed_yolo_goroutine() bool {
    const scheduler = global_scheduler orelse return false;
    scheduler.yield() catch return false;
    return true;
}

/// Create a channel (implements `dm<T>` type creation)
/// Returns channel pointer on success, null on failure
pub export fn cursed_dm_create(element_size: u32, capacity: u32) ?*anyopaque {
    _ = element_size; // For now, we'll create a generic channel
    
    // Create a channel for i32 (simplified)
    const channel = concurrency.makeChannel(i32, allocator, capacity) catch return null;
    return @ptrCast(channel);
}

/// Send to a channel (implements `dm_send` operation)
/// Returns 0 for success, 1 for would_block, 2 for closed
pub export fn cursed_dm_send(channel_ptr: ?*anyopaque, value_ptr: ?*anyopaque, value_size: u32) u32 {
    _ = value_size;
    
    if (channel_ptr == null or value_ptr == null) return 2;
    
    const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(channel_ptr));
    const value: *i32 = @ptrCast(@alignCast(value_ptr));
    
    const result = channel.send(value.*) catch return 2;
    
    return switch (result) {
        .sent => 0,
        .would_block => 1,
        .closed => 2,
    };
}

/// Receive from a channel (implements `dm_receive` operation)  
/// Returns 0 for success, 1 for would_block, 2 for closed
pub export fn cursed_dm_receive(channel_ptr: ?*anyopaque, buffer_ptr: ?*anyopaque, buffer_size: u32) u32 {
    _ = buffer_size;
    
    if (channel_ptr == null or buffer_ptr == null) return 2;
    
    const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(channel_ptr));
    const buffer: *i32 = @ptrCast(@alignCast(buffer_ptr));
    
    const received = channel.receive() catch return 2;
    
    if (received) |value| {
        buffer.* = value;
        return 0; // success
    } else {
        return 2; // closed
    }
}

/// Execute select statement (implements `ready` keyword)
/// Returns index of selected case, or negative value for special cases
pub export fn cursed_ready_select(operations_ptr: ?*anyopaque, operation_count: u32) i32 {
    _ = operations_ptr;
    _ = operation_count;
    
    // Simplified select implementation
    // In full implementation, this would parse the operations array
    // and execute the select logic
    
    // For now, just return default case (0) or timeout (-1)
    return 0;
}

/// Channel management functions

/// Close a channel
pub export fn cursed_dm_close(channel_ptr: ?*anyopaque) void {
    if (channel_ptr == null) return;
    
    const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(channel_ptr));
    channel.close();
}

/// Check if channel is closed
pub export fn cursed_dm_is_closed(channel_ptr: ?*anyopaque) bool {
    if (channel_ptr == null) return true;
    
    const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(channel_ptr));
    return channel.isClosed();
}

/// Get channel length
pub export fn cursed_dm_length(channel_ptr: ?*anyopaque) u32 {
    if (channel_ptr == null) return 0;
    
    const channel: *concurrency.Channel(i32) = @ptrCast(@alignCast(channel_ptr));
    return @intCast(channel.length());
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

/// Get current goroutine ID
pub export fn cursed_current_goroutine_id() u64 {
    // In full implementation, this would return the actual current goroutine ID
    // For now, return a placeholder
    return 1;
}

/// Check if goroutine is still running
pub export fn cursed_goroutine_is_running(goroutine_id: u64) bool {
    _ = goroutine_id;
    // In full implementation, this would check the actual goroutine state
    return true;
}

/// Wait for goroutine to complete
pub export fn cursed_goroutine_join(goroutine_id: u64) void {
    _ = goroutine_id;
    // In full implementation, this would wait for the goroutine to complete
    // For now, just sleep briefly
    std.time.sleep(1_000_000); // 1ms
}

/// Scheduler statistics and monitoring

/// Get active goroutine count
pub export fn cursed_scheduler_active_count() u32 {
    const scheduler = global_scheduler orelse return 0;
    return scheduler.activeGoroutineCount();
}

/// Check if scheduler is running
pub export fn cursed_scheduler_is_running() bool {
    const scheduler = global_scheduler orelse return false;
    return scheduler.isRunning();
}

/// Get scheduler statistics
pub export fn cursed_scheduler_stats(stats_ptr: ?*anyopaque) void {
    if (stats_ptr == null) return;
    
    const scheduler = global_scheduler orelse return;
    const stats = scheduler.getStats();
    
    // In full implementation, this would copy stats to the provided structure
    _ = stats;
}
