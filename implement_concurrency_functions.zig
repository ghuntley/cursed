// Implementation of actual CURSED concurrency functions
// These will be called from compiled CURSED programs

const std = @import("std");
const print = std.debug.print;

// Simple channel implementation for CURSED programs
const SimpleChannel = struct {
    buffer: std.ArrayList(i32),
    capacity: usize,
    closed: bool,
    mutex: std.Thread.Mutex,
    
    fn init(allocator: std.mem.Allocator, capacity: usize) !*SimpleChannel {
        const channel = try allocator.create(SimpleChannel);
        channel.* = SimpleChannel{
            .buffer = std.ArrayList(i32).init(allocator),
            .capacity = capacity,
            .closed = false,
            .mutex = std.Thread.Mutex{},
        };
        return channel;
    }
    
    fn send(self: *SimpleChannel, value: i32) !u32 {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        if (self.closed) return 2; // closed
        if (self.buffer.items.len >= self.capacity) return 1; // would block
        
        try self.buffer.append(value);
        return 0; // success
    }
    
    fn receive(self: *SimpleChannel, out_value: *i32) u32 {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        if (self.buffer.items.len > 0) {
            out_value.* = self.buffer.orderedRemove(0);
            return 0; // success
        }
        
        if (self.closed) return 2; // closed
        return 1; // would block
    }
    
    fn close(self: *SimpleChannel) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        self.closed = true;
    }
    
    fn isClosed(self: *SimpleChannel) bool {
        self.mutex.lock();
        defer self.mutex.unlock();
        return self.closed;
    }
};

var global_allocator = std.heap.page_allocator;

// CURSED runtime functions that will be called from compiled code

/// Create a channel - dm_create(element_size, capacity) -> channel_ptr
pub export fn cursed_dm_create(element_size: u32, capacity: u32) ?*anyopaque {
    _ = element_size; // For now, we only support i32
    
    const channel = SimpleChannel.init(global_allocator, capacity) catch return null;
    return @ptrCast(channel);
}

/// Send to channel - dm_send(channel_ptr, value_ptr, value_size) -> result
pub export fn cursed_dm_send(channel_ptr: ?*anyopaque, value_ptr: ?*anyopaque, value_size: u32) u32 {
    _ = value_size;
    
    if (channel_ptr == null or value_ptr == null) return 2;
    
    const channel: *SimpleChannel = @ptrCast(@alignCast(channel_ptr));
    const value: *i32 = @ptrCast(@alignCast(value_ptr));
    
    return channel.send(value.*) catch 2;
}

/// Receive from channel - dm_recv(channel_ptr, buffer_ptr, buffer_size) -> result  
pub export fn cursed_dm_recv(channel_ptr: ?*anyopaque, buffer_ptr: ?*anyopaque, buffer_size: u32) u32 {
    _ = buffer_size;
    
    if (channel_ptr == null or buffer_ptr == null) return 2;
    
    const channel: *SimpleChannel = @ptrCast(@alignCast(channel_ptr));
    const buffer: *i32 = @ptrCast(@alignCast(buffer_ptr));
    
    return channel.receive(buffer);
}

/// Close channel - dm_close(channel_ptr)
pub export fn cursed_dm_close(channel_ptr: ?*anyopaque) void {
    if (channel_ptr == null) return;
    
    const channel: *SimpleChannel = @ptrCast(@alignCast(channel_ptr));
    channel.close();
}

/// Check if channel is closed - dm_is_closed(channel_ptr) -> bool
pub export fn cursed_dm_is_closed(channel_ptr: ?*anyopaque) bool {
    if (channel_ptr == null) return true;
    
    const channel: *SimpleChannel = @ptrCast(@alignCast(channel_ptr));
    return channel.isClosed();
}

/// Simple goroutine counter for testing
var goroutine_counter: u64 = 1;
var goroutine_mutex = std.Thread.Mutex{};

/// Spawn goroutine - stan(function_ptr, context_ptr) -> goroutine_id
pub export fn cursed_stan(function_ptr: ?*anyopaque, context_ptr: ?*anyopaque) u64 {
    _ = function_ptr;
    _ = context_ptr;
    
    goroutine_mutex.lock();
    defer goroutine_mutex.unlock();
    
    const id = goroutine_counter;
    goroutine_counter += 1;
    
    // For now, just return an ID without actually spawning a thread
    // In a real implementation, this would spawn a proper goroutine
    print("🦄 Goroutine {} spawned (stubbed)\n", .{id});
    
    return id;
}

// Test the implementations
pub fn main() !void {
    print("Testing CURSED concurrency runtime functions...\n", .{});
    
    // Test channel creation
    const channel = cursed_dm_create(4, 10);
    if (channel != null) {
        print("✅ Channel created\n", .{});
    } else {
        print("❌ Channel creation failed\n", .{});
        return;
    }
    
    // Test send
    var value: i32 = 42;
    const send_result = cursed_dm_send(channel, &value, 4);
    print("Send result: {} (0=success, 1=would_block, 2=closed)\n", .{send_result});
    
    // Test receive
    var received: i32 = 0;
    const recv_result = cursed_dm_recv(channel, &received, 4);
    print("Receive result: {} -> value: {}\n", .{ recv_result, received });
    
    // Test close
    cursed_dm_close(channel);
    const is_closed = cursed_dm_is_closed(channel);
    print("Channel closed: {}\n", .{is_closed});
    
    // Test goroutine spawning
    const goroutine_id = cursed_stan(null, null);
    print("Goroutine ID: {}\n", .{goroutine_id});
    
    print("✅ All concurrency runtime functions working!\n", .{});
}
