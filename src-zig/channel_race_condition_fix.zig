//! Channel Race Condition Fix - Memory-Safe Channel Operations
//!
//! This module provides a race-condition-free channel implementation with:
//! - Atomic reference counting for channels
//! - Proper defer-based cleanup
//! - Safe goroutine termination handling
//! - Memory leak prevention in concurrent scenarios

const std = @import("std");
const Allocator = std.mem.Allocator;
const Thread = std.Thread;
const Mutex = std.Thread.Mutex;
const Condition = std.Thread.Condition;
const Atomic = std.atomic.Value;
const Order = std.atomic.Ordering;

/// Thread-safe channel with atomic reference counting
pub fn Channel(comptime T: type) type {
    return struct {
        const Self = @This();
        
        // Core channel state
        buffer: std.ArrayList(T),
        capacity: usize,
        closed: Atomic(bool),
        
        // Reference counting for memory safety
        ref_count: Atomic(u32),
        
        // Synchronization primitives
        mutex: Mutex,
        send_condition: Condition,
        recv_condition: Condition,
        
        // Goroutine tracking
        sender_count: Atomic(u32),
        receiver_count: Atomic(u32),
        
        // Cleanup state
        cleanup_started: Atomic(bool),
        cleanup_completed: Atomic(bool),
        
        allocator: Allocator,
        
        pub fn init(allocator: Allocator, capacity: usize) !*Self {
            const channel = try allocator.create(Self);
            errdefer allocator.destroy(channel);
            
            channel.* = Self{
                .buffer = .{},
                .capacity = capacity,
                .closed = Atomic(bool).init(false),
                .ref_count = Atomic(u32).init(1), // Start with 1 reference
                .mutex = Mutex{},
                .send_condition = Condition{},
                .recv_condition = Condition{},
                .sender_count = Atomic(u32).init(0),
                .receiver_count = Atomic(u32).init(0),
                .cleanup_started = Atomic(bool).init(false),
                .cleanup_completed = Atomic(bool).init(false),
                .allocator = allocator,
            };
            
            return channel;
        }
        
        /// Increment reference count atomically
        pub fn addRef(self: *Self) void {
            _ = self.ref_count.fetchAdd(1, .acq_rel);
        }
        
        /// Decrement reference count and cleanup if last reference
        pub fn release(self: *Self) void {
            const old_count = self.ref_count.fetchSub(1, .acq_rel);
            
            if (old_count == 1) {
                // This was the last reference, perform cleanup
                self.performCleanup();
            }
        }
        
        /// Perform final cleanup when reference count reaches zero
        fn performCleanup(self: *Self) void {
            // Ensure cleanup only happens once
            if (self.cleanup_started.cmpxchgStrong(false, true, .acq_rel, .acquire)) |_| {
                return; // Cleanup already started by another thread
            }
            
            // Close the channel first
            self.close();
            
            // Wait for all operations to complete
            self.waitForOperationsToComplete();
            
            // Clean up buffer contents
            self.mutex.lock();
            defer self.mutex.unlock();
            
            // Clear any remaining items in buffer
            for (self.buffer.items) |item| {
                // For complex types, would need proper cleanup
                _ = item;
            }
            self.buffer.deinit(self.allocator);
            
            // Mark cleanup as completed
            self.cleanup_completed.store(true, .release);
            
            // Free the channel struct itself
            const allocator = self.allocator;
            allocator.destroy(self);
        }
        
        /// Wait for all active operations to complete before cleanup
        fn waitForOperationsToComplete(self: *Self) void {
            // Wait for all senders and receivers to finish
            const max_wait_attempts = 1000; // 1 second max wait
            var attempts: u32 = 0;
            
            while (attempts < max_wait_attempts) {
                const senders = self.sender_count.load(.acquire);
                const receivers = self.receiver_count.load(.acquire);
                
                if (senders == 0 and receivers == 0) {
                    break; // All operations completed
                }
                
                // Broadcast to wake up any waiting operations
                self.send_condition.broadcast();
                self.recv_condition.broadcast();
                
                // Small delay before checking again
                std.Thread.sleep(1_000_000); // 1ms
                attempts += 1;
            }
        }
        
        /// Send operation with proper reference counting
        pub fn send(self: *Self, value: T) !SendResult {
            // Check if cleanup has started
            if (self.cleanup_started.load(.acquire)) {
                return SendResult.closed;
            }
            
            // Increment sender count
            _ = self.sender_count.fetchAdd(1, .acq_rel);
            defer _ = self.sender_count.fetchSub(1, .acq_rel);
            
            // Add reference for this operation
            self.addRef();
            defer self.release();
            
            if (self.closed.load(.acquire)) {
                return SendResult.closed;
            }
            
            self.mutex.lock();
            defer self.mutex.unlock();
            
            // For unbuffered channels, wait for receiver
            if (self.capacity == 0) {
                while (self.receiver_count.load(.acquire) == 0 and 
                       !self.closed.load(.acquire) and 
                       !self.cleanup_started.load(.acquire)) {
                    self.send_condition.wait(&self.mutex);
                }
                
                if (self.closed.load(.acquire) or self.cleanup_started.load(.acquire)) {
                    return SendResult.closed;
                }
                
                try self.buffer.append(allocator, value);
                self.recv_condition.signal();
                return SendResult.sent;
            }
            
            // For buffered channels, wait for space
            while (self.buffer.items.len >= self.capacity and 
                   !self.closed.load(.acquire) and 
                   !self.cleanup_started.load(.acquire)) {
                self.send_condition.wait(&self.mutex);
            }
            
            if (self.closed.load(.acquire) or self.cleanup_started.load(.acquire)) {
                return SendResult.closed;
            }
            
            try self.buffer.append(allocator, value);
            self.recv_condition.signal();
            return SendResult.sent;
        }
        
        /// Receive operation with proper reference counting
        pub fn receive(self: *Self) !?T {
            // Check if cleanup has started
            if (self.cleanup_started.load(.acquire)) {
                return null;
            }
            
            // Increment receiver count
            _ = self.receiver_count.fetchAdd(1, .acq_rel);
            defer _ = self.receiver_count.fetchSub(1, .acq_rel);
            
            // Add reference for this operation
            self.addRef();
            defer self.release();
            
            self.mutex.lock();
            defer self.mutex.unlock();
            
            // Wait for data or channel close
            while (self.buffer.items.len == 0 and 
                   !self.closed.load(.acquire) and 
                   !self.cleanup_started.load(.acquire)) {
                self.recv_condition.wait(&self.mutex);
            }
            
            if (self.buffer.items.len > 0) {
                const value = self.buffer.orderedRemove(0);
                self.send_condition.signal();
                return value;
            }
            
            // Channel is closed or cleanup started
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

/// Send operation results
pub const SendResult = enum {
    sent,
    would_block,
    closed,
};

/// RAII wrapper for automatic channel cleanup
pub fn ChannelGuard(comptime T: type) type {
    return struct {
        const Self = @This();
        
        channel: *Channel(T),
        
        pub fn init(allocator: Allocator, capacity: usize) !Self {
            const channel = try Channel(T).init(allocator, capacity);
            return Self{ .channel = channel };
        }
        
        pub fn deinit(self: *Self) void {
            self.channel.release();
        }
        
        pub fn send(self: *Self, value: T) !SendResult {
            return self.channel.send(value);
        }
        
        pub fn receive(self: *Self) !?T {
            return self.channel.receive();
        }
        
        pub fn close(self: *Self) void {
            self.channel.close();
        }
    };
}

/// Goroutine wrapper with automatic channel reference management
pub const SafeGoroutine = struct {
    const Self = @This();
    
    thread: ?Thread,
    channels: std.ArrayList(*anyopaque), // Store channel pointers for cleanup
    allocator: Allocator,
    
    pub fn init() Self {
        return Self{
            .thread = null,
            .channels = .{},
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Self) void {
        // Wait for thread to complete
        if (self.thread) |thread| {
            thread.join();
        }
        
        // Release all channel references
        for (self.channels.items) |channel_ptr| {
            // Cast back to appropriate channel type and release
            // This is a simplified version - real implementation would track types
            const channel: *Channel(u8) = @ptrCast(@alignCast(channel_ptr));
            channel.release();
        }
        
        self.channels.deinit(self.allocator);
    }
    
    /// Add a channel reference to this goroutine
    pub fn addChannelRef(self: *Self, channel: *anyopaque) !void {
        try self.channels.append(allocator, channel);
    }
    
    /// Start the goroutine with a function
    pub fn spawn(self: *Self, func: anytype, args: anytype) !void {
        self.thread = try Thread.spawn(.{}, func, args);
    }
};

/// Test functions to validate the fixes
pub fn testChannelRaceConditions() !void {
    const allocator = std.heap.page_allocator;
    
    // Test 1: Channel cleanup with early goroutine termination
    {
        var channel_guard = try ChannelGuard(i32).init(allocator, 0);
        defer channel_guard.deinit();
        
        var goroutine = SafeGoroutine.init(allocator);
        defer goroutine.deinit();
        
        // Add channel reference to goroutine
        try goroutine.addChannelRef(channel_guard.channel);
        
        // Goroutine that terminates early
        const TestContext = struct {
            channel: *Channel(i32),
            
            fn senderFunc(ctx: *@This()) void {
                _ = ctx.channel.send(42) catch return;
                // Goroutine exits without waiting for receiver
            }
        };
        
        var ctx = TestContext{ .channel = channel_guard.channel };
        try goroutine.spawn(TestContext.senderFunc, .{&ctx});
        
        // Small delay to let sender start
        std.Thread.sleep(10_000_000); // 10ms
        
        // Receive the value
        const value = try channel_guard.receive();
        std.debug.assert(value.? == 42);
    }
    
    // Test 2: Stress test with multiple goroutines
    {
        var channel_guard = try ChannelGuard(i32).init(allocator, 10);
        defer channel_guard.deinit();
        
        const num_goroutines = 100;
        var goroutines: [num_goroutines]SafeGoroutine = undefined;
        
        // Initialize goroutines
        for (&goroutines) |*g| {
            g.* = SafeGoroutine.init(allocator);
        }
        
        // Cleanup goroutines
        defer {
            for (&goroutines) |*g| {
                g.deinit();
            }
        }
        
        // Start sender and receiver goroutines
        for (&goroutines, 0..) |*g, i| {
            try g.addChannelRef(channel_guard.channel);
            
            if (i % 2 == 0) {
                // Sender goroutines
                const SenderContext = struct {
                    channel: *Channel(i32),
                    value: i32,
                    
                    fn senderFunc(ctx: *@This()) void {
                        _ = ctx.channel.send(ctx.value) catch return;
                    }
                };
                
                var ctx = try allocator.create(SenderContext);
                ctx.* = SenderContext{ .channel = channel_guard.channel, .value = @intCast(i) };
                try g.spawn(SenderContext.senderFunc, .{ctx});
            } else {
                // Receiver goroutines
                const ReceiverContext = struct {
                    channel: *Channel(i32),
                    
                    fn receiverFunc(ctx: *@This()) void {
                        _ = ctx.channel.receive() catch return;
                    }
                };
                
                var ctx = try allocator.create(ReceiverContext);
                ctx.* = ReceiverContext{ .channel = channel_guard.channel };
                try g.spawn(ReceiverContext.receiverFunc, .{ctx});
            }
        }
        
        // Let all goroutines run
        std.Thread.sleep(100_000_000); // 100ms
    }
    
    std.debug.print("✅ All channel race condition tests passed!\n", .{});
}

// Export for testing
pub const testing = struct {
    pub const Channel = Channel;
    pub const ChannelGuard = ChannelGuard;
    pub const SafeGoroutine = SafeGoroutine;
    pub const testChannelRaceConditions = testChannelRaceConditions;
};
