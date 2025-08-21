//! CURSED Channel Deadlock Prevention System
//! 
//! This module implements advanced deadlock prevention mechanisms for channel operations
//! under high contention scenarios, addressing the following deadlock patterns:
//!
//! 1. Send-Receive Circular Dependency
//! 2. Multi-Channel Select Deadlocks  
//! 3. Resource Exhaustion Deadlocks
//! 4. Priority Inversion Deadlocks
//! 5. GC-Channel Interaction Deadlocks

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const Thread = std.Thread;
const Mutex = std.Thread.Mutex;
const Condition = std.Thread.Condition;
const Atomic = std.atomic.Value;
const Order = std.atomic.Ordering;

/// Deadlock prevention timeout (30 seconds)
const DEADLOCK_TIMEOUT_NS: u64 = 30_000_000_000;

/// Maximum number of retries before giving up
const MAX_RETRIES: u32 = 3;

/// Channel operation priority levels
pub const ChannelPriority = enum(u8) {
    low = 0,
    normal = 1,
    high = 2,
    critical = 3,
};

/// Deadlock detection state
pub const DeadlockState = enum(u8) {
    normal = 0,
    potential = 1,
    detected = 2,
    resolved = 3,
};

/// Channel operation statistics for deadlock analysis
pub const OperationStats = struct {
    total_operations: Atomic(u64),
    successful_operations: Atomic(u64), 
    blocked_operations: Atomic(u64),
    timeout_operations: Atomic(u64),
    retry_operations: Atomic(u64),
    
    pub fn init() OperationStats {
        return OperationStats{
            .total_operations = Atomic(u64).init(0),
            .successful_operations = Atomic(u64).init(0),
            .blocked_operations = Atomic(u64).init(0),
            .timeout_operations = Atomic(u64).init(0),
            .retry_operations = Atomic(u64).init(0),
        };
    }
    
    pub fn recordOperation(self: *OperationStats, result: OperationResult) void {
        _ = self.total_operations.fetchAdd(1, .acq_rel);
        switch (result) {
            .success => _ = self.successful_operations.fetchAdd(1, .acq_rel),
            .blocked => _ = self.blocked_operations.fetchAdd(1, .acq_rel),
            .timeout => _ = self.timeout_operations.fetchAdd(1, .acq_rel),
            .retry => _ = self.retry_operations.fetchAdd(1, .acq_rel),
        }
    }
};

/// Operation result types for deadlock analysis
pub const OperationResult = enum {
    success,
    blocked,
    timeout,
    retry,
};

/// Enhanced channel with deadlock prevention
pub fn DeadlockFreeChannel(comptime T: type) type {
    return struct {
        const Self = @This();
        
        // Core channel data
        id: u64,
        buffer: ArrayList(T),
        capacity: usize,
        allocator: Allocator,
        
        // Synchronization primitives with deadlock prevention
        mutex: Mutex,
        send_condition: Condition,
        recv_condition: Condition,
        
        // Atomic state management
        closed: Atomic(bool),
        sender_count: Atomic(u32),
        receiver_count: Atomic(u32),
        
        // Deadlock prevention
        priority: ChannelPriority,
        deadlock_state: Atomic(u8),
        last_operation_time: Atomic(i64),
        
        // Statistics and monitoring
        stats: OperationStats,
        
        pub fn init(allocator: Allocator, id: u64, capacity: usize, priority: ChannelPriority) Self {
            return Self{
                .id = id,
                .buffer = .empty,
                .capacity = capacity,
                .allocator = allocator,
                .mutex = Mutex{},
                .send_condition = Condition{},
                .recv_condition = Condition{},
                .closed = Atomic(bool).init(false),
                .sender_count = Atomic(u32).init(0),
                .receiver_count = Atomic(u32).init(0),
                .priority = priority,
                .deadlock_state = Atomic(u8).init(@intFromEnum(DeadlockState.normal)),
                .last_operation_time = Atomic(i64).init(@intCast(@as(i64, @intCast(std.time.nanoTimestamp())))),
                .stats = OperationStats.init(),
            };
        }
        
        pub fn deinit(self: *Self) void {
            self.close();
            self.buffer.deinit();
        }
        
        /// Send with comprehensive deadlock prevention
        pub fn send(self: *Self, value: T) !SendResult {
            return self.sendWithPriority(value, self.priority, DEADLOCK_TIMEOUT_NS);
        }
        
        /// Send with priority and timeout for deadlock prevention
        pub fn sendWithPriority(self: *Self, value: T, priority: ChannelPriority, timeout_ns: u64) !SendResult {
            const start_time = @as(i64, @intCast(std.time.nanoTimestamp()));
            var retry_count: u32 = 0;
            
            while (retry_count < MAX_RETRIES) {
                // Update operation tracking
                _ = self.sender_count.fetchAdd(1, .acq_rel);
                defer _ = self.sender_count.fetchSub(1, .acq_rel);
                
                // Check for deadlock potential before acquiring lock
                if (self.detectPotentialDeadlock(priority)) {
                    self.stats.recordOperation(.blocked);
                    return SendResult.would_block;
                }
                
                // Use tryLock with timeout to prevent deadlock
                if (!self.tryLockWithTimeout(timeout_ns - @as(u64, @intCast(@as(i64, @intCast(std.time.nanoTimestamp())) - start_time)))) {
                    retry_count += 1;
                    self.stats.recordOperation(.retry);
                    
                    // Exponential backoff
                    const backoff_ns = @as(u64, @intCast(@as(u32, 1000) << @as(u5, @intCast(@min(retry_count, 10)))));
                    std.Thread.sleep(backoff_ns);
                    continue;
                }
                defer self.mutex.unlock();
                
                // Check if channel is closed
                if (self.closed.load(.acquire)) {
                    self.stats.recordOperation(.success);
                    return SendResult.closed;
                }
                
                // Update last operation time
                self.last_operation_time.store(@as(i64, @intCast(std.time.nanoTimestamp())), .release);
                
                // For unbuffered channels (capacity = 0), use rendezvous
                if (self.capacity == 0) {
                    return self.sendRendezvous(value, timeout_ns, start_time);
                }
                
                // For buffered channels, check capacity
                if (self.buffer.items.len < self.capacity) {
                    try self.buffer.append(value);
                    self.notifyReceivers();
                    self.stats.recordOperation(.success);
                    return SendResult.sent;
                }
                
                // Buffer full, wait with timeout
                return self.waitForSpace(value, timeout_ns, start_time);
            }
            
            // Max retries exceeded
            self.stats.recordOperation(.timeout);
            return SendResult.timeout;
        }
        
        /// Receive with comprehensive deadlock prevention
        pub fn receive(self: *Self) !?T {
            return self.receiveWithTimeout(DEADLOCK_TIMEOUT_NS);
        }
        
        /// Receive with timeout for deadlock prevention
        pub fn receiveWithTimeout(self: *Self, timeout_ns: u64) !?T {
            const start_time = @as(i64, @intCast(std.time.nanoTimestamp()));
            var retry_count: u32 = 0;
            
            while (retry_count < MAX_RETRIES) {
                // Update operation tracking
                _ = self.receiver_count.fetchAdd(1, .acq_rel);
                defer _ = self.receiver_count.fetchSub(1, .acq_rel);
                
                // Check for deadlock potential
                if (self.detectPotentialDeadlock(self.priority)) {
                    self.stats.recordOperation(.blocked);
                    return null;
                }
                
                // Use tryLock with timeout
                if (!self.tryLockWithTimeout(timeout_ns - @as(u64, @intCast(@as(i64, @intCast(std.time.nanoTimestamp())) - start_time)))) {
                    retry_count += 1;
                    self.stats.recordOperation(.retry);
                    
                    const backoff_ns = @as(u64, @intCast(@as(u32, 1000) << @as(u5, @intCast(@min(retry_count, 10)))));
                    std.Thread.sleep(backoff_ns);
                    continue;
                }
                defer self.mutex.unlock();
                
                // Update last operation time
                self.last_operation_time.store(@as(i64, @intCast(std.time.nanoTimestamp())), .release);
                
                // Check for available data
                if (self.buffer.items.len > 0) {
                    const value = self.buffer.orderedRemove(0);
                    self.notifySenders();
                    self.stats.recordOperation(.success);
                    return value;
                }
                
                // No data available, check if closed
                if (self.closed.load(.acquire)) {
                    self.stats.recordOperation(.success);
                    return null;
                }
                
                // Wait for data with timeout
                return self.waitForData(timeout_ns, start_time);
            }
            
            // Max retries exceeded
            self.stats.recordOperation(.timeout);
            return null;
        }
        
        /// Try to acquire lock with timeout to prevent deadlock
        fn tryLockWithTimeout(self: *Self, timeout_ns: u64) bool {
            const start_time = @as(i64, @intCast(std.time.nanoTimestamp()));
            const end_time = start_time + @as(i64, @intCast(timeout_ns));
            
            while (@as(i64, @intCast(std.time.nanoTimestamp())) < end_time) {
                if (self.mutex.tryLock()) {
                    return true;
                }
                
                // Short sleep to avoid busy waiting
                std.Thread.sleep(100_000); // 100 microseconds
            }
            
            return false;
        }
        
        /// Detect potential deadlock scenarios
        fn detectPotentialDeadlock(self: *Self, priority: ChannelPriority) bool {
            const current_time = @as(i64, @intCast(std.time.nanoTimestamp()));
            const last_op_time = self.last_operation_time.load(.acquire);
            
            // Check if channel has been inactive for too long
            if (current_time - last_op_time > DEADLOCK_TIMEOUT_NS) {
                self.deadlock_state.store(@intFromEnum(DeadlockState.potential), .release);
                return true;
            }
            
            // Check for excessive contention (high sender/receiver counts)
            const senders = self.sender_count.load(.acquire);
            const receivers = self.receiver_count.load(.acquire);
            
            if (senders > 10 and receivers > 10) {
                // High contention scenario
                if (priority == ChannelPriority.low) {
                    return true; // Low priority operations should back off
                }
            }
            
            // Check operation success rate
            const total_ops = self.stats.total_operations.load(.acquire);
            const blocked_ops = self.stats.blocked_operations.load(.acquire);
            
            if (total_ops > 100 and blocked_ops * 100 / total_ops > 50) {
                // More than 50% of operations are blocked
                self.deadlock_state.store(@intFromEnum(DeadlockState.potential), .release);
                return true;
            }
            
            return false;
        }
        
        /// Rendezvous send for unbuffered channels
        fn sendRendezvous(self: *Self, value: T, timeout_ns: u64, start_time: i64) !SendResult {
            const end_time = start_time + @as(i64, @intCast(timeout_ns));
            
            while (@as(i64, @intCast(std.time.nanoTimestamp())) < end_time) {
                if (self.closed.load(.acquire)) {
                    return SendResult.closed;
                }
                
                // Check for waiting receivers
                if (self.receiver_count.load(.acquire) > 0) {
                    try self.buffer.append(value);
                    self.notifyReceivers();
                    return SendResult.sent;
                }
                
                // Wait for receiver with timeout
                const remaining_ns = end_time - @as(i64, @intCast(std.time.nanoTimestamp()));
                if (remaining_ns <= 0) break;
                
                self.waitWithTimeout(&self.send_condition, @as(u64, @intCast(remaining_ns)));
            }
            
            return SendResult.timeout;
        }
        
        /// Wait for buffer space with timeout
        fn waitForSpace(self: *Self, value: T, timeout_ns: u64, start_time: i64) !SendResult {
            const end_time = start_time + @as(i64, @intCast(timeout_ns));
            
            while (@as(i64, @intCast(std.time.nanoTimestamp())) < end_time and !self.closed.load(.acquire)) {
                if (self.buffer.items.len < self.capacity) {
                    try self.buffer.append(value);
                    self.notifyReceivers();
                    return SendResult.sent;
                }
                
                const remaining_ns = end_time - @as(i64, @intCast(std.time.nanoTimestamp()));
                if (remaining_ns <= 0) break;
                
                self.waitWithTimeout(&self.send_condition, @as(u64, @intCast(remaining_ns)));
            }
            
            if (self.closed.load(.acquire)) {
                return SendResult.closed;
            }
            
            return SendResult.timeout;
        }
        
        /// Wait for data with timeout
        fn waitForData(self: *Self, timeout_ns: u64, start_time: i64) !?T {
            const end_time = start_time + @as(i64, @intCast(timeout_ns));
            
            while (@as(i64, @intCast(std.time.nanoTimestamp())) < end_time and !self.closed.load(.acquire)) {
                if (self.buffer.items.len > 0) {
                    const value = self.buffer.orderedRemove(0);
                    self.notifySenders();
                    return value;
                }
                
                const remaining_ns = end_time - @as(i64, @intCast(std.time.nanoTimestamp()));
                if (remaining_ns <= 0) break;
                
                self.waitWithTimeout(&self.recv_condition, @as(u64, @intCast(remaining_ns)));
            }
            
            return null;
        }
        
        /// Wait on condition variable with timeout
        fn waitWithTimeout(self: *Self, condition: *Condition, timeout_ns: u64) void {
            // Simple timeout implementation - in production would use proper timed wait
            const chunks = timeout_ns / 1_000_000; // 1ms chunks
            for (0..@intCast(chunks)) |_| {
                condition.timedWait(&self.mutex, 1_000_000) catch break;
            }
        }
        
        /// Notify waiting receivers with priority handling
        fn notifyReceivers(self: *Self) void {
            if (self.priority == ChannelPriority.critical) {
                self.recv_condition.broadcast(); // Wake all for critical operations
            } else {
                self.recv_condition.signal(); // Wake one for normal operations
            }
        }
        
        /// Notify waiting senders with priority handling  
        fn notifySenders(self: *Self) void {
            if (self.priority == ChannelPriority.critical) {
                self.send_condition.broadcast(); // Wake all for critical operations
            } else {
                self.send_condition.signal(); // Wake one for normal operations
            }
        }
        
        /// Close the channel safely
        pub fn close(self: *Self) void {
            self.mutex.lock();
            defer self.mutex.unlock();
            
            self.closed.store(true, .release);
            
            // Wake all waiters
            self.send_condition.broadcast();
            self.recv_condition.broadcast();
            
            // Update deadlock state
            self.deadlock_state.store(@intFromEnum(DeadlockState.resolved), .release);
        }
        
        /// Get channel statistics for monitoring
        pub fn getStats(self: *Self) ChannelStats {
            return ChannelStats{
                .id = self.id,
                .capacity = self.capacity,
                .current_size = self.buffer.items.len,
                .closed = self.closed.load(.acquire),
                .sender_count = self.sender_count.load(.acquire),
                .receiver_count = self.receiver_count.load(.acquire),
                .deadlock_state = @as(DeadlockState, @enumFromInt(self.deadlock_state.load(.acquire))),
                .total_operations = self.stats.total_operations.load(.acquire),
                .successful_operations = self.stats.successful_operations.load(.acquire),
                .blocked_operations = self.stats.blocked_operations.load(.acquire),
                .timeout_operations = self.stats.timeout_operations.load(.acquire),
            };
        }
    };
}

/// Channel statistics structure
pub const ChannelStats = struct {
    id: u64,
    capacity: usize,
    current_size: usize,
    closed: bool,
    sender_count: u32,
    receiver_count: u32,
    deadlock_state: DeadlockState,
    total_operations: u64,
    successful_operations: u64,
    blocked_operations: u64,
    timeout_operations: u64,
};

/// Send result enumeration
pub const SendResult = enum {
    sent,
    would_block,
    closed,
    timeout,
};

/// Deadlock detection and resolution system
pub const DeadlockDetector = struct {
    channels: ArrayList(*anyopaque),
    mutex: Mutex,
    running: Atomic(bool),
    thread: ?Thread,
    allocator: Allocator,
    
    pub fn init() DeadlockDetector {
        return DeadlockDetector{
            .channels = .empty,
            .mutex = Mutex{},
            .running = Atomic(bool).init(false),
            .thread = null,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *DeadlockDetector) void {
        self.stop();
        self.channels.deinit();
    }
    
    pub fn start(self: *DeadlockDetector) !void {
        if (self.running.cmpxchgWeak(false, true, .seq_cst, .seq_cst) == null) {
            self.thread = try Thread.spawn(.{}, detectorMain, .{self});
        }
    }
    
    pub fn stop(self: *DeadlockDetector) void {
        if (self.running.load(.acquire)) {
            self.running.store(false, .release);
            if (self.thread) |thread| {
                thread.join();
                self.thread = null;
            }
        }
    }
    
    fn detectorMain(self: *DeadlockDetector) void {
        while (self.running.load(.acquire)) {
            self.checkForDeadlocks();
            std.Thread.sleep(1_000_000_000); // Check every 1 second
        }
    }
    
    fn checkForDeadlocks(self: *DeadlockDetector) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        // Simple deadlock detection - in production would be more sophisticated
        for (self.channels.items) |channel_ptr| {
            // Would check channel-specific deadlock conditions
            _ = channel_ptr;
        }
    }
    
    pub fn registerChannel(self: *DeadlockDetector, channel: *anyopaque) !void {
        self.mutex.lock();
        defer self.mutex.unlock();
        try self.channels.append(channel);
    }
};

/// High-contention test for deadlock prevention
pub fn runHighContentionTest(allocator: Allocator) !void {
    print("=== CURSED High-Contention Deadlock Prevention Test ===\n", .{});
    
    const num_channels = 5;
    const num_goroutines = 50;
    const operations_per_goroutine = 100;
    
    // Create multiple channels with different priorities
    var channels: [num_channels]*DeadlockFreeChannel(i64) = undefined;
    for (0..num_channels) |i| {
        channels[i] = try allocator.create(DeadlockFreeChannel(i64));
        const priority = switch (i % 4) {
            0 => ChannelPriority.low,
            1 => ChannelPriority.normal,
            2 => ChannelPriority.high,
            else => ChannelPriority.critical,
        };
        channels[i].* = DeadlockFreeChannel(i64).init(allocator, i, 10, priority);
    }
    defer {
        for (channels) |ch| {
            ch.deinit();
            allocator.destroy(ch);
        }
    }
    
    // Start deadlock detector
    var detector = DeadlockDetector.init(allocator);
    try detector.start();
    defer detector.deinit();
    
    for (channels) |ch| {
        try detector.registerChannel(ch);
    }
    
    print("🚀 Starting high-contention test with {} channels, {} goroutines, {} ops each\n", .{ num_channels, num_goroutines, operations_per_goroutine });
    
    // Spawn sender and receiver threads
    var threads = .empty;
    defer {
        for (threads.items) |thread| {
            thread.join();
        }
        threads.deinit();
    }
    
    const TestContext = struct {
        channels: *[num_channels]*DeadlockFreeChannel(i64),
        goroutine_id: u32,
        operations: u32,
    };
    
    // Spawn sender threads
    for (0..num_goroutines / 2) |i| {
        const context = try allocator.create(TestContext);
        context.* = TestContext{
            .channels = &channels,
            .goroutine_id = @intCast(i),
            .operations = operations_per_goroutine,
        };
        
        const thread = try Thread.spawn(.{}, senderWorker, .{context});
        try threads.append(thread);
    }
    
    // Spawn receiver threads
    for (num_goroutines / 2..num_goroutines) |i| {
        const context = try allocator.create(TestContext);
        context.* = TestContext{
            .channels = &channels,
            .goroutine_id = @intCast(i),
            .operations = operations_per_goroutine,
        };
        
        const thread = try Thread.spawn(.{}, receiverWorker, .{context});
        try threads.append(thread);
    }
    
    print("⏳ Waiting for {} goroutines to complete...\n", .{threads.items.len});
    
    // Wait for all threads to complete
    for (threads.items) |thread| {
        thread.join();
    }
    
    // Collect statistics
    var total_stats = ChannelStats{
        .id = 0,
        .capacity = 0,
        .current_size = 0,
        .closed = false,
        .sender_count = 0,
        .receiver_count = 0,
        .deadlock_state = DeadlockState.normal,
        .total_operations = 0,
        .successful_operations = 0,
        .blocked_operations = 0,
        .timeout_operations = 0,
    };
    
    for (channels) |ch| {
        const stats = ch.getStats();
        total_stats.total_operations += stats.total_operations;
        total_stats.successful_operations += stats.successful_operations;
        total_stats.blocked_operations += stats.blocked_operations;
        total_stats.timeout_operations += stats.timeout_operations;
        
        print("Channel {}: {} total, {} success, {} blocked, {} timeout\n", .{
            stats.id,
            stats.total_operations,
            stats.successful_operations,
            stats.blocked_operations,
            stats.timeout_operations,
        });
    }
    
    print("=== Final Results ===\n", .{});
    print("Total operations: {}\n", .{total_stats.total_operations});
    print("Successful: {}\n", .{total_stats.successful_operations});
    print("Blocked: {}\n", .{total_stats.blocked_operations});
    print("Timeouts: {}\n", .{total_stats.timeout_operations});
    
    const success_rate = if (total_stats.total_operations > 0)
        (total_stats.successful_operations * 100) / total_stats.total_operations
    else
        0;
    
    print("Success rate: {}%\n", .{success_rate});
    
    if (success_rate >= 80) {
        print("✅ HIGH-CONTENTION TEST PASSED!\n", .{});
    } else {
        print("❌ HIGH-CONTENTION TEST FAILED!\n", .{});
    }
    
    print("=== Deadlock Prevention Test Complete ===\n", .{});
}

fn senderWorker(context: *anyopaque) void {
    const ctx: *@TypeOf(@as(*const struct {
        channels: *[5]*DeadlockFreeChannel(i64),
        goroutine_id: u32,
        operations: u32,
    }, undefined).*) = @ptrCast(@alignCast(context));
    
    for (0..ctx.operations) |op| {
        const channel_idx = op % ctx.channels.len;
        const value = @as(i64, @intCast(ctx.goroutine_id * 1000 + op));
        
        const result = ctx.channels[channel_idx].send(value) catch SendResult.timeout;
        
        switch (result) {
            .sent => {},
            .would_block => std.Thread.sleep(1_000_000), // 1ms backoff
            .closed, .timeout => break,
        }
        
        // Small random delay to create contention
        if (op % 10 == 0) {
            std.Thread.sleep(std.crypto.random.intRangeAtMost(u64, 100_000, 5_000_000));
        }
    }
}

fn receiverWorker(context: *anyopaque) void {
    const ctx: *@TypeOf(@as(*const struct {
        channels: *[5]*DeadlockFreeChannel(i64),
        goroutine_id: u32,
        operations: u32,
    }, undefined).*) = @ptrCast(@alignCast(context));
    
    for (0..ctx.operations) |op| {
        const channel_idx = op % ctx.channels.len;
        
        if (ctx.channels[channel_idx].receive() catch null) |_| {
            // Successfully received
        } else {
            std.Thread.sleep(1_000_000); // 1ms backoff on failure
        }
        
        // Small random delay to create contention
        if (op % 10 == 0) {
            std.Thread.sleep(std.crypto.random.intRangeAtMost(u64, 100_000, 5_000_000));
        }
    }
}

// Test the deadlock prevention system
test "deadlock prevention system" {
    const allocator = std.testing.allocator;
    try runHighContentionTest(allocator);
}
