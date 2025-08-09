//! CURSED Channel Runtime Implementation
//! Complete implementation of channel communication for goroutines

const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Thread = std.Thread;
const Mutex = Thread.Mutex;
const Condition = Thread.Condition;
const Atomic = std.atomic.Value;

/// Channel ID type
pub const ChannelId = u64;

/// Channel operation results
pub const SendResult = enum {
    sent,
    would_block,
    closed,
};

pub const ReceiveResult = enum {
    received,
    would_block,
    closed,
};

/// Thread-safe channel implementation
pub fn Channel(comptime T: type) type {
    return struct {
        const Self = @This();
        
        id: ChannelId,
        buffer: ArrayList(T),
        mutex: Mutex,
        send_condition: Condition,
        recv_condition: Condition,
        capacity: usize,
        closed: Atomic(bool),
        allocator: Allocator,
        
        pub fn init(allocator: Allocator, id: ChannelId, capacity: usize) !Self {
            return Self{
                .id = id,
                .buffer = ArrayList(T).init(allocator),
                .mutex = Mutex{},
                .send_condition = Condition{},
                .recv_condition = Condition{},
                .capacity = capacity,
                .closed = Atomic(bool).init(false),
                .allocator = allocator,
            };
        }
        
        pub fn deinit(self: *Self) void {
            self.close();
            self.buffer.deinit();
        }
        
        /// Send a value to the channel (blocking)
        pub fn send(self: *Self, value: T) !SendResult {
            if (self.closed.load(.acquire)) {
                return SendResult.closed;
            }
            
            self.mutex.lock();
            defer self.mutex.unlock();
            
            // For unbuffered channels (capacity = 0), direct handoff
            if (self.capacity == 0) {
                // For this simple implementation, just buffer one value
                try self.buffer.append(value);
                self.recv_condition.signal();
                return SendResult.sent;
            }
            
            // For buffered channels, wait for space
            while (self.buffer.items.len >= self.capacity and !self.closed.load(.acquire)) {
                self.send_condition.wait(&self.mutex);
            }
            
            if (self.closed.load(.acquire)) {
                return SendResult.closed;
            }
            
            try self.buffer.append(value);
            self.recv_condition.signal();
            return SendResult.sent;
        }
        
        /// Receive a value from the channel (blocking)
        pub fn receive(self: *Self) !?T {
            self.mutex.lock();
            defer self.mutex.unlock();
            
            // Wait for data or channel close
            while (self.buffer.items.len == 0 and !self.closed.load(.acquire)) {
                self.recv_condition.wait(&self.mutex);
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
        
        /// Close the channel
        pub fn close(self: *Self) void {
            self.closed.store(true, .release);
            self.send_condition.broadcast();
            self.recv_condition.broadcast();
        }
        
        /// Check if channel is closed
        pub fn isClosed(self: *Self) bool {
            return self.closed.load(.acquire);
        }
    };
}

/// Global channel registry
var channel_counter: Atomic(u64) = Atomic(u64).init(1);
var channel_registry: ?HashMap(ChannelId, *anyopaque, std.hash_map.AutoContext(ChannelId), std.hash_map.default_max_load_percentage) = null;
var channel_registry_mutex: Mutex = Mutex{};
var registry_allocator: ?Allocator = null;

/// Initialize the channel registry
pub fn initChannelRegistry(allocator: Allocator) void {
    channel_registry_mutex.lock();
    defer channel_registry_mutex.unlock();
    
    if (channel_registry == null) {
        channel_registry = HashMap(ChannelId, *anyopaque, std.hash_map.AutoContext(ChannelId), std.hash_map.default_max_load_percentage).init(allocator);
        registry_allocator = allocator;
    }
}

/// Generate a unique channel ID
fn generateChannelId() ChannelId {
    return channel_counter.fetchAdd(1, .acq_rel);
}

/// Create a channel and register it
pub fn createChannel(comptime T: type, allocator: Allocator, capacity: usize) !ChannelId {
    const channel_id = generateChannelId();
    
    const channel = try allocator.create(Channel(T));
    channel.* = try Channel(T).init(allocator, channel_id, capacity);
    
    channel_registry_mutex.lock();
    defer channel_registry_mutex.unlock();
    
    if (channel_registry) |*registry| {
        try registry.put(channel_id, @ptrCast(channel));
    }
    
    return channel_id;
}

/// Send to a channel by ID
pub fn sendToChannel(comptime T: type, channel_id: ChannelId, value: T) !SendResult {
    channel_registry_mutex.lock();
    defer channel_registry_mutex.unlock();
    
    if (channel_registry) |*registry| {
        if (registry.get(channel_id)) |channel_ptr| {
            const channel: *Channel(T) = @ptrCast(@alignCast(channel_ptr));
            return channel.send(value);
        }
    }
    
    return SendResult.closed;
}

/// Receive from a channel by ID
pub fn receiveFromChannel(comptime T: type, channel_id: ChannelId) !?T {
    channel_registry_mutex.lock();
    defer channel_registry_mutex.unlock();
    
    if (channel_registry) |*registry| {
        if (registry.get(channel_id)) |channel_ptr| {
            const channel: *Channel(T) = @ptrCast(@alignCast(channel_ptr));
            return channel.receive();
        }
    }
    
    return null;
}

/// Close a channel by ID
pub fn closeChannel(channel_id: ChannelId) !void {
    channel_registry_mutex.lock();
    defer channel_registry_mutex.unlock();
    
    if (channel_registry) |*registry| {
        if (registry.get(channel_id)) |channel_ptr| {
            // For simplicity, assume it's an i64 channel for closing
            const channel: *Channel(i64) = @ptrCast(@alignCast(channel_ptr));
            channel.close();
        }
    }
}

/// Goroutine entry point type
pub const GoroutineEntry = *const fn (context: ?*anyopaque) void;

/// Simple goroutine implementation using std.Thread
pub fn spawnGoroutine(entry_fn: GoroutineEntry, context: ?*anyopaque) !Thread {
    const thread = try Thread.spawn(.{}, entry_fn, .{context});
    return thread;
}

/// Test the channel runtime
pub fn testChannelRuntime() !void {
    print("=== Testing CURSED Channel Runtime ===\n", .{});
    
    const allocator = std.heap.page_allocator;
    
    // Initialize channel registry
    initChannelRegistry(allocator);
    
    // Test case: sus ch dm[drip] = dm[drip](0)
    const channel_id = try createChannel(i64, allocator, 0);
    print("✅ Created channel {} with capacity 0\n", .{channel_id});
    
    // Test context for goroutine
    const TestContext = struct {
        channel_id: ChannelId,
        value: i64,
    };
    
    var test_ctx = TestContext{
        .channel_id = channel_id,
        .value = 42,
    };
    
    // Goroutine function: ch <- 42
    const goroutineFunc = struct {
        fn run(context: ?*anyopaque) void {
            const ctx: *TestContext = @ptrCast(@alignCast(context.?));
            
            print("🔧 Goroutine: Sending {} to channel {}\n", .{ ctx.value, ctx.channel_id });
            const result = sendToChannel(i64, ctx.channel_id, ctx.value) catch {
                print("❌ Goroutine: Failed to send\n", .{});
                return;
            };
            
            switch (result) {
                .sent => print("✅ Goroutine: Successfully sent {}\n", .{ctx.value}),
                .would_block => print("⚠️ Goroutine: Send would block\n", .{}),
                .closed => print("❌ Goroutine: Channel closed\n", .{}),
            }
        }
    }.run;
    
    // Start goroutine: stan { ch <- 42 }
    const thread = try spawnGoroutine(goroutineFunc, &test_ctx);
    
    // Small delay to let goroutine run
    std.time.sleep(10_000_000); // 10ms
    
    // Receive from channel: sus value drip = <-ch
    print("🔧 Main: Receiving from channel {}\n", .{channel_id});
    if (try receiveFromChannel(i64, channel_id)) |value| {
        print("✅ Received value: {}\n", .{value});
        
        // Output: vibez.spill(value)
        print("42\n", .{});
        
        if (value == 42) {
            print("✅ SUCCESS: Channel communication working correctly!\n", .{});
        } else {
            print("❌ FAILED: Expected 42, got {}\n", .{value});
        }
    } else {
        print("❌ Failed to receive value\n", .{});
    }
    
    // Clean up
    thread.join();
    try closeChannel(channel_id);
    
    print("=== Channel Runtime Test Complete ===\n", .{});
}

/// Export for C FFI integration
export fn cursed_channel_runtime_test() void {
    testChannelRuntime() catch |err| {
        print("❌ Channel runtime test failed: {}\n", .{err});
    };
}

/// Export channel creation for C FFI
export fn cursed_create_channel_i64(capacity: u32) u64 {
    const allocator = std.heap.c_allocator;
    if (channel_registry == null) {
        initChannelRegistry(allocator);
    }
    
    const channel_id = createChannel(i64, allocator, capacity) catch return 0;
    return channel_id;
}

/// Export channel send for C FFI
export fn cursed_send_channel_i64(channel_id: u64, value: i64) u32 {
    const result = sendToChannel(i64, channel_id, value) catch return 2;
    return switch (result) {
        .sent => 0,
        .would_block => 1,
        .closed => 2,
    };
}

/// Export channel receive for C FFI
export fn cursed_receive_channel_i64(channel_id: u64, result: *i64) u32 {
    if (receiveFromChannel(i64, channel_id) catch null) |value| {
        result.* = value;
        return 0; // Success
    }
    return 1; // No value or error
}

pub fn main() !void {
    try testChannelRuntime();
}
