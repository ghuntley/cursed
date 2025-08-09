//! CURSED Channel Operations Implementation
//! Priority 0 Critical: Channel communication for goroutines implementation

const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

// Import the existing concurrency runtime
const concurrency = @import("src-zig/concurrency.zig");

/// Channel variable type for the CURSED runtime
pub const ChannelVariable = struct {
    channel_id: concurrency.ChannelId,
    element_type: []const u8,
    capacity: usize,
    
    pub fn init(channel_id: concurrency.ChannelId, element_type: []const u8, capacity: usize) ChannelVariable {
        return ChannelVariable{
            .channel_id = channel_id,
            .element_type = element_type,
            .capacity = capacity,
        };
    }
};

/// Initialize channel runtime
pub fn initChannelRuntime(allocator: Allocator) !void {
    // Initialize the scheduler
    const config = concurrency.SchedulerConfig.default();
    try concurrency.initializeScheduler(allocator, config);
    
    // Initialize channel registry
    concurrency.initChannelRegistry(allocator);
}

/// Shutdown channel runtime
pub fn shutdownChannelRuntime(allocator: Allocator) void {
    concurrency.shutdownScheduler(allocator);
}

/// Create a channel with the specified type and capacity
pub fn createChannel(allocator: Allocator, element_type: []const u8, capacity: usize) !ChannelVariable {
    // For now, create an i64 channel regardless of type (simplification)
    const channel_id = try concurrency.dm_create(i64, allocator, capacity);
    
    print("[CHANNEL] Created channel ID {} of type {} with capacity {}\n", .{ channel_id, element_type, capacity });
    
    return ChannelVariable.init(channel_id, element_type, capacity);
}

/// Send a value to a channel
pub fn sendToChannel(allocator: Allocator, channel: ChannelVariable, value: i64) !void {
    const result = try concurrency.dm_send(channel.channel_id, value, allocator);
    
    switch (result) {
        .sent => {
            print("[CHANNEL] Successfully sent {} to channel {}\n", .{ value, channel.channel_id });
        },
        .would_block => {
            print("[CHANNEL] Send would block for channel {}\n", .{channel.channel_id});
        },
        .closed => {
            print("[CHANNEL] Channel {} is closed\n", .{channel.channel_id});
            return error.ChannelClosed;
        },
    }
}

/// Receive a value from a channel
pub fn receiveFromChannel(allocator: Allocator, channel: ChannelVariable) !?i64 {
    const result = try concurrency.dm_recv(i64, channel.channel_id, allocator);
    
    if (result) |value| {
        print("[CHANNEL] Successfully received {} from channel {}\n", .{ value, channel.channel_id });
        return value;
    } else {
        print("[CHANNEL] Channel {} is closed or empty\n", .{channel.channel_id});
        return null;
    }
}

/// Close a channel
pub fn closeChannel(channel: ChannelVariable) !void {
    try concurrency.dm_close(channel.channel_id);
    print("[CHANNEL] Closed channel {}\n", .{channel.channel_id});
}

/// Spawn a goroutine
pub fn spawnGoroutine(entry_fn: concurrency.GoroutineEntry, context: ?*anyopaque) !concurrency.GoroutineId {
    return try concurrency.stan(entry_fn, context);
}

// Test the channel implementation
const TestContext = struct {
    channel: ChannelVariable,
    allocator: Allocator,
};

fn testGoroutineFunction(context: ?*anyopaque) void {
    const test_ctx: *TestContext = @ptrCast(@alignCast(context.?));
    
    // Send value 42 to the channel
    sendToChannel(test_ctx.allocator, test_ctx.channel, 42) catch |err| {
        print("[ERROR] Failed to send to channel: {}\n", .{err});
    };
    
    print("[GOROUTINE] Sent value 42 to channel\n");
}

/// Test the complete channel implementation
pub fn testChannelImplementation(allocator: Allocator) !void {
    print("=== Testing CURSED Channel Implementation ===\n");
    
    // Initialize runtime
    try initChannelRuntime(allocator);
    defer shutdownChannelRuntime(allocator);
    
    // Create a channel: dm[drip](0) - unbuffered channel
    const channel = try createChannel(allocator, "drip", 0);
    
    // Create test context
    var test_ctx = TestContext{
        .channel = channel,
        .allocator = allocator,
    };
    
    // Spawn goroutine: stan { ch <- 42 }
    const goroutine_id = try spawnGoroutine(testGoroutineFunction, &test_ctx);
    print("[TEST] Spawned goroutine {}\n", .{goroutine_id});
    
    // Give the goroutine time to execute
    std.time.sleep(50_000_000); // 50ms
    
    // Receive from channel: sus value drip = <-ch
    if (try receiveFromChannel(allocator, channel)) |value| {
        print("[TEST] Received value: {}\n", .{value});
        print("RESULT: {}\n", .{value});
    } else {
        print("[TEST] No value received\n");
    }
    
    // Close the channel
    try closeChannel(channel);
    
    print("=== Channel Implementation Test Complete ===\n");
}

/// Export for C FFI
export fn cursed_test_channels() void {
    const allocator = std.heap.c_allocator;
    testChannelImplementation(allocator) catch |err| {
        print("[ERROR] Channel test failed: {}\n", .{err});
    };
}

/// Export runtime initialization for C FFI
export fn cursed_init_channel_runtime() u32 {
    const allocator = std.heap.c_allocator;
    initChannelRuntime(allocator) catch return 0;
    return 1;
}

/// Export runtime shutdown for C FFI
export fn cursed_shutdown_channel_runtime() void {
    const allocator = std.heap.c_allocator;
    shutdownChannelRuntime(allocator);
}

test "channel operations" {
    const allocator = std.testing.allocator;
    try testChannelImplementation(allocator);
}
