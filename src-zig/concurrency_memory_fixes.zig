//! CURSED Concurrency System - Memory Management Fixes
//!
//! This module provides fixes for memory leaks, allocation failures, and cleanup issues
//! in the concurrency system. Features:
//! - Arena-based allocation for goroutines
//! - Channel lifetime management
//! - Proper cleanup on scheduler shutdown
//! - Memory leak prevention
//! - Safe channel operations

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const ArenaAllocator = std.heap.ArenaAllocator;
const Thread = std.Thread;
const Mutex = std.Thread.Mutex;
const Condition = std.Thread.Condition;
const Atomic = std.atomic.Value;
const Order = std.atomic.Ordering;

/// Import base concurrency types
const concurrency = @import("concurrency.zig");
const GoroutineId = concurrency.GoroutineId;
const WorkerId = concurrency.WorkerId;
const ChannelId = concurrency.ChannelId;
const GoroutineState = concurrency.GoroutineState;
const GoroutinePriority = concurrency.GoroutinePriority;
const SendResult = concurrency.SendResult;
const ReceiveResult = concurrency.ReceiveResult;
const GoroutineEntry = concurrency.GoroutineEntry;

/// Memory-safe Channel implementation with proper cleanup
pub fn MemorySafeChannel(comptime T: type) type {
    return struct {
        const Self = @This();

        id: ChannelId,
        buffer: ArrayList(T),
        mutex: Mutex,
        send_condition: Condition,
        recv_condition: Condition,
        capacity: usize,
        closed: Atomic(bool),
        sender_count: Atomic(u32),
        receiver_count: Atomic(u32),
        allocator: Allocator,
        arena: ArenaAllocator,
        arena_allocator: Allocator,
        ref_count: Atomic(u32),
        
        pub fn init(allocator: Allocator, capacity: usize) !*Self {
            const self = try allocator.create(Self);
            var arena = ArenaAllocator.init(allocator);
            const arena_allocator = arena.allocator();
            
            self.* = Self{
                .id = generateChannelId(),
                .buffer = .empty,
                .mutex = Mutex{},
                .send_condition = Condition{},
                .recv_condition = Condition{},
                .capacity = capacity,
                .closed = Atomic(bool).init(false),
                .sender_count = Atomic(u32).init(0),
                .receiver_count = Atomic(u32).init(0),
                .allocator = allocator,
                .arena = arena,
                .arena_allocator = arena_allocator,
                .ref_count = Atomic(u32).init(1),
            };
            
            return self;
        }

        pub fn deinit(self: *Self) void {
            self.close();
            
            // Wait for all operations to complete
            self.mutex.lock();
            defer self.mutex.unlock();
            
            // Clean up arena and main allocation
            self.arena.deinit();
            self.allocator.destroy(self);
        }
        
        pub fn retain(self: *Self) void {
            _ = self.ref_count.fetchAdd(1, .acq_rel);
        }
        
        pub fn release(self: *Self) void {
            const old_count = self.ref_count.fetchSub(1, .acq_rel);
            if (old_count == 1) {
                self.deinit();
            }
        }

        /// Send a value to the channel (blocking) with timeout
        pub fn send(self: *Self, value: T) !SendResult {
            return self.sendTimeout(value, null);
        }
        
        pub fn sendTimeout(self: *Self, value: T, timeout_ms: ?u64) !SendResult {
            if (self.closed.load(.acquire)) {
                return SendResult.closed;
            }

            const start_time = std.time.milliTimestamp();
            
            self.mutex.lock();
            defer self.mutex.unlock();

            // For unbuffered channels, wait for receiver
            if (self.capacity == 0) {
                while (self.receiver_count.load(.acquire) == 0 and !self.closed.load(.acquire)) {
                    if (timeout_ms) |timeout| {
                        const elapsed = std.time.milliTimestamp() - start_time;
                        if (elapsed >= timeout) {
                            return SendResult.would_block;
                        }
                    }
                    
                    // Use timed wait if timeout specified
                    if (timeout_ms) |timeout| {
                        const remaining = timeout - @as(u64, @intCast(std.time.milliTimestamp() - start_time));
                        self.send_condition.timedWait(&self.mutex, remaining * 1000000) catch {
                            return SendResult.would_block;
                        };
                    } else {
                        self.send_condition.wait(&self.mutex);
                    }
                }

                if (self.closed.load(.acquire)) {
                    return SendResult.closed;
                }

                self.buffer.append(value) catch return error.OutOfMemory;
                self.recv_condition.signal();
                return SendResult.sent;
            }

            // For buffered channels, wait for space
            while (self.buffer.items.len >= self.capacity and !self.closed.load(.acquire)) {
                if (timeout_ms) |timeout| {
                    const elapsed = std.time.milliTimestamp() - start_time;
                    if (elapsed >= timeout) {
                        return SendResult.would_block;
                    }
                }
                
                if (timeout_ms) |timeout| {
                    const remaining = timeout - @as(u64, @intCast(std.time.milliTimestamp() - start_time));
                    self.send_condition.timedWait(&self.mutex, remaining * 1000000) catch {
                        return SendResult.would_block;
                    };
                } else {
                    self.send_condition.wait(&self.mutex);
                }
            }

            if (self.closed.load(.acquire)) {
                return SendResult.closed;
            }

            self.buffer.append(value) catch return error.OutOfMemory;
            self.recv_condition.signal();
            return SendResult.sent;
        }

        /// Try to send a value (non-blocking)
        pub fn trySend(self: *Self, value: T) !SendResult {
            if (self.closed.load(.acquire)) {
                return SendResult.closed;
            }

            if (!self.mutex.tryLock()) {
                return SendResult.would_block;
            }
            defer self.mutex.unlock();

            // For unbuffered channels, need receiver
            if (self.capacity == 0) {
                if (self.receiver_count.load(.acquire) == 0) {
                    return SendResult.would_block;
                }

                if (self.closed.load(.acquire)) {
                    return SendResult.closed;
                }

                self.buffer.append(value) catch return error.OutOfMemory;
                self.recv_condition.signal();
                return SendResult.sent;
            }

            // For buffered channels, check capacity
            if (self.buffer.items.len >= self.capacity) {
                return SendResult.would_block;
            }

            if (self.closed.load(.acquire)) {
                return SendResult.closed;
            }

            self.buffer.append(value) catch return error.OutOfMemory;
            self.recv_condition.signal();
            return SendResult.sent;
        }

        /// Receive a value from the channel (blocking) with timeout
        pub fn receive(self: *Self) !?T {
            return self.receiveTimeout(null);
        }
        
        pub fn receiveTimeout(self: *Self, timeout_ms: ?u64) !?T {
            const start_time = std.time.milliTimestamp();
            
            self.mutex.lock();
            defer self.mutex.unlock();

            // Wait for data or channel close
            while (self.buffer.items.len == 0 and !self.closed.load(.acquire)) {
                if (timeout_ms) |timeout| {
                    const elapsed = std.time.milliTimestamp() - start_time;
                    if (elapsed >= timeout) {
                        return null; // Timeout
                    }
                }
                
                if (timeout_ms) |timeout| {
                    const remaining = timeout - @as(u64, @intCast(std.time.milliTimestamp() - start_time));
                    self.recv_condition.timedWait(&self.mutex, remaining * 1000000) catch {
                        return null; // Timeout
                    };
                } else {
                    self.recv_condition.wait(&self.mutex);
                }
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

        /// Try to receive a value (non-blocking)
        pub fn tryReceive(self: *Self) !?T {
            if (!self.mutex.tryLock()) {
                return null; // Would block
            }
            defer self.mutex.unlock();

            if (self.buffer.items.len > 0) {
                const value = self.buffer.orderedRemove(0);
                self.send_condition.signal();
                return value;
            }

            if (self.closed.load(.acquire)) {
                return null;
            }

            return null; // Would block
        }

        /// Close the channel safely
        pub fn close(self: *Self) void {
            self.closed.store(true, .release);
            self.send_condition.broadcast();
            self.recv_condition.broadcast();
        }

        /// Check if channel is closed
        pub fn isClosed(self: *Self) bool {
            return self.closed.load(.acquire);
        }

        /// Get channel length safely
        pub fn length(self: *Self) usize {
            self.mutex.lock();
            defer self.mutex.unlock();
            return self.buffer.items.len;
        }
    };
}

/// Memory-safe Goroutine with proper cleanup
pub const MemorySafeGoroutine = struct {
    id: GoroutineId,
    state: Atomic(GoroutineState),
    priority: GoroutinePriority,
    entry_fn: GoroutineEntry,
    context: ?*anyopaque,
    parent_id: ?GoroutineId,
    created_at: i64,
    total_runtime: u64,
    stack_size: usize,
    allocator: Allocator,
    arena: ArenaAllocator,
    arena_allocator: Allocator,
    cleanup_fn: ?*const fn(context: ?*anyopaque) void,

    pub fn init(allocator: Allocator, id: GoroutineId, entry_fn: GoroutineEntry, context: ?*anyopaque) !*MemorySafeGoroutine {
        const self = try allocator.create(MemorySafeGoroutine);
        var arena = ArenaAllocator.init(allocator);
        const arena_allocator = arena.allocator();
        
        self.* = MemorySafeGoroutine{
            .id = id,
            .state = Atomic(GoroutineState).init(GoroutineState.ready),
            .priority = GoroutinePriority.normal,
            .entry_fn = entry_fn,
            .context = context,
            .parent_id = null,
            .created_at = std.time.milliTimestamp(),
            .total_runtime = 0,
            .stack_size = 2 * 1024 * 1024, // 2MB default stack
            .allocator = allocator,
            .arena = arena,
            .arena_allocator = arena_allocator,
            .cleanup_fn = null,
        };
        
        return self;
    }

    pub fn deinit(self: *MemorySafeGoroutine) void {
        // Call cleanup function if provided
        if (self.cleanup_fn) |cleanup| {
            cleanup(self.context);
        }
        
        // Clean up arena and main allocation
        self.arena.deinit();
        self.allocator.destroy(self);
    }

    pub fn getState(self: *const MemorySafeGoroutine) GoroutineState {
        return self.state.load(.acquire);
    }

    pub fn setState(self: *MemorySafeGoroutine, new_state: GoroutineState) void {
        self.state.store(new_state, .release);
    }

    pub fn setCleanupFn(self: *MemorySafeGoroutine, cleanup_fn: *const fn(context: ?*anyopaque) void) void {
        self.cleanup_fn = cleanup_fn;
    }

    pub fn execute(self: *MemorySafeGoroutine) void {
        self.setState(GoroutineState.running);
        const start_time = std.time.milliTimestamp();

        // Execute the goroutine function with error handling
        self.entry_fn(self.context);

        const end_time = std.time.milliTimestamp();
        self.total_runtime += @as(u64, @intCast(@max(0, end_time - start_time)));
        self.setState(GoroutineState.completed);
    }
};

/// Memory-safe Work-stealing deque
pub const MemorySafeWorkStealingDeque = struct {
    const Self = @This();

    items: ArrayList(*MemorySafeGoroutine),
    mutex: Mutex,
    top: Atomic(usize),
    bottom: Atomic(usize),
    allocator: Allocator,
    arena: ArenaAllocator,
    arena_allocator: Allocator,

    pub fn init(allocator: Allocator) !*Self {
        const self = try allocator.create(Self);
        var arena = ArenaAllocator.init(allocator);
        const arena_allocator = arena.allocator();
        
        self.* = Self{
            .items = .empty,
            .mutex = Mutex{},
            .top = Atomic(usize).init(0),
            .bottom = Atomic(usize).init(0),
            .allocator = allocator,
            .arena = arena,
            .arena_allocator = arena_allocator,
        };
        
        return self;
    }

    pub fn deinit(self: *Self) void {
        self.mutex.lock();
        
        // Clean up any remaining goroutines
        for (self.items.items) |goroutine| {
            goroutine.deinit();
        }
        
        self.mutex.unlock();
        
        // Clean up arena and main allocation
        self.arena.deinit();
        self.allocator.destroy(self);
    }

    /// Push goroutine to bottom (owner thread only)
    pub fn pushBottom(self: *Self, goroutine: *MemorySafeGoroutine) !void {
        self.mutex.lock();
        defer self.mutex.unlock();

        try self.items.append(self.allocator, goroutine);
        self.bottom.store(self.items.items.len, .release);
    }

    /// Pop goroutine from bottom (owner thread only)
    pub fn popBottom(self: *Self) ?*MemorySafeGoroutine {
        self.mutex.lock();
        defer self.mutex.unlock();

        const item_len = self.items.items.len;
        if (item_len == 0) return null;

        const goroutine = self.items.pop();
        self.bottom.store(self.items.items.len, .release);
        return goroutine;
    }

    /// Steal goroutine from top (other threads)
    pub fn steal(self: *Self) ?*MemorySafeGoroutine {
        self.mutex.lock();
        defer self.mutex.unlock();

        if (self.items.items.len == 0) return null;

        const goroutine = self.items.orderedRemove(0);
        self.top.store(1, .release);
        return goroutine;
    }

    pub fn length(self: *Self) usize {
        self.mutex.lock();
        defer self.mutex.unlock();
        return self.items.items.len;
    }

    pub fn isEmpty(self: *Self) bool {
        return self.length() == 0;
    }
};

/// Memory-safe Worker thread
pub const MemorySafeWorker = struct {
    id: WorkerId,
    deque: *MemorySafeWorkStealingDeque,
    thread: ?Thread,
    scheduler: *MemorySafeScheduler,
    running: Atomic(bool),
    allocator: Allocator,
    arena: ArenaAllocator,
    arena_allocator: Allocator,

    pub fn init(allocator: Allocator, id: WorkerId, scheduler: *MemorySafeScheduler) !*MemorySafeWorker {
        const self = try allocator.create(MemorySafeWorker);
        var arena = ArenaAllocator.init(allocator);
        const arena_allocator = arena.allocator();
        
        const deque = try MemorySafeWorkStealingDeque.init(arena_allocator);
        
        self.* = MemorySafeWorker{
            .id = id,
            .deque = deque,
            .thread = null,
            .scheduler = scheduler,
            .running = Atomic(bool).init(false),
            .allocator = allocator,
            .arena = arena,
            .arena_allocator = arena_allocator,
        };
        
        return self;
    }

    pub fn deinit(self: *MemorySafeWorker) void {
        self.stop();
        self.deque.deinit();
        self.arena.deinit();
        self.allocator.destroy(self);
    }

    pub fn start(self: *MemorySafeWorker) !void {
        self.running.store(true, .release);
        self.thread = try Thread.spawn(.{}, workerLoop, .{self});
    }

    pub fn stop(self: *MemorySafeWorker) void {
        self.running.store(false, .release);
        if (self.thread) |thread| {
            thread.join();
            self.thread = null;
        }
    }

    fn workerLoop(self: *MemorySafeWorker) void {
        while (self.running.load(.acquire)) {
            // Try to get work from local deque
            if (self.deque.popBottom()) |goroutine| {
                self.executeGoroutine(goroutine);
                continue;
            }

            // Try to steal work from other workers
            if (self.stealWork()) |goroutine| {
                self.executeGoroutine(goroutine);
                continue;
            }

            // Try to get work from global queue
            if (self.scheduler.getGlobalWork()) |goroutine| {
                self.executeGoroutine(goroutine);
                continue;
            }

            // No work available, yield CPU briefly
            std.time.sleep(1000); // 1 microsecond
        }
    }

    fn executeGoroutine(self: *MemorySafeWorker, goroutine: *MemorySafeGoroutine) void {
        _ = self;
        goroutine.execute();
        // Goroutine cleanup is handled by the scheduler
    }

    fn stealWork(self: *MemorySafeWorker) ?*MemorySafeGoroutine {
        // Try to steal from other workers
        const workers = self.scheduler.getWorkers();
        
        for (workers) |worker| {
            if (worker.id != self.id) {
                if (worker.deque.steal()) |goroutine| {
                    return goroutine;
                }
            }
        }
        
        return null;
    }
};

/// Memory-safe Scheduler
pub const MemorySafeScheduler = struct {
    workers: ArrayList(*MemorySafeWorker),
    global_queue: ArrayList(*MemorySafeGoroutine),
    global_mutex: Mutex,
    next_goroutine_id: Atomic(GoroutineId),
    next_worker_id: Atomic(WorkerId),
    active_goroutines: Atomic(u32),
    running: Atomic(bool),
    allocator: Allocator,
    arena: ArenaAllocator,
    arena_allocator: Allocator,
    num_workers: usize,

    pub fn init(allocator: Allocator, num_workers: usize) !*MemorySafeScheduler {
        const self = try allocator.create(MemorySafeScheduler);
        var arena = ArenaAllocator.init(allocator);
        const arena_allocator = arena.allocator();
        
        self.* = MemorySafeScheduler{
            .workers = .empty,
            .global_queue = .empty,
            .global_mutex = Mutex{},
            .next_goroutine_id = Atomic(GoroutineId).init(1),
            .next_worker_id = Atomic(WorkerId).init(0),
            .active_goroutines = Atomic(u32).init(0),
            .running = Atomic(bool).init(false),
            .allocator = allocator,
            .arena = arena,
            .arena_allocator = arena_allocator,
            .num_workers = num_workers,
        };

        // Initialize workers
        try self.workers.ensureTotalCapacity(allocator, num_workers);
        for (0..num_workers) |_| {
            const worker_id = self.next_worker_id.fetchAdd(1, .acq_rel);
            const worker = try MemorySafeWorker.init(arena_allocator, worker_id, self);
            try self.workers.append(worker);
        }

        return self;
    }

    pub fn deinit(self: *MemorySafeScheduler) void {
        self.stop();
        
        // Clean up workers
        for (self.workers.items) |worker| {
            worker.deinit();
        }
        
        // Clean up remaining goroutines in global queue
        self.global_mutex.lock();
        for (self.global_queue.items) |goroutine| {
            goroutine.deinit();
        }
        self.global_mutex.unlock();
        
        // Clean up arena and main allocation
        self.arena.deinit();
        self.allocator.destroy(self);
    }

    pub fn start(self: *MemorySafeScheduler) !void {
        self.running.store(true, .release);
        
        // Start all worker threads
        for (self.workers.items) |worker| {
            try worker.start();
        }
    }

    pub fn stop(self: *MemorySafeScheduler) void {
        self.running.store(false, .release);
        
        // Stop all worker threads
        for (self.workers.items) |worker| {
            worker.stop();
        }
    }

    /// Spawn a new goroutine with memory safety
    pub fn spawn(self: *MemorySafeScheduler, entry_fn: GoroutineEntry, context: ?*anyopaque) !GoroutineId {
        const goroutine_id = self.next_goroutine_id.fetchAdd(1, .acq_rel);
        
        const goroutine = try MemorySafeGoroutine.init(self.arena_allocator, goroutine_id, entry_fn, context);
        
        // Schedule the goroutine
        try self.scheduleGoroutine(goroutine);
        
        _ = self.active_goroutines.fetchAdd(1, .acq_rel);
        
        return goroutine_id;
    }

    fn scheduleGoroutine(self: *MemorySafeScheduler, goroutine: *MemorySafeGoroutine) !void {
        // Find the worker with the least work
        var min_work_worker: ?*MemorySafeWorker = null;
        var min_work_count: usize = std.math.maxInt(usize);

        for (self.workers.items) |worker| {
            const work_count = worker.deque.length();
            if (work_count < min_work_count) {
                min_work_count = work_count;
                min_work_worker = worker;
            }
        }

        if (min_work_worker) |worker| {
            try worker.deque.pushBottom(goroutine);
        } else {
            // Fallback to global queue
            self.global_mutex.lock();
            defer self.global_mutex.unlock();
            try self.global_queue.append(goroutine);
        }
    }

    pub fn getGlobalWork(self: *MemorySafeScheduler) ?*MemorySafeGoroutine {
        self.global_mutex.lock();
        defer self.global_mutex.unlock();
        
        if (self.global_queue.items.len == 0) return null;
        
        return self.global_queue.orderedRemove(0);
    }

    pub fn getWorkers(self: *MemorySafeScheduler) []*MemorySafeWorker {
        return self.workers.items;
    }

    pub fn isRunning(self: *MemorySafeScheduler) bool {
        return self.running.load(.acquire);
    }

    pub fn activeGoroutineCount(self: *MemorySafeScheduler) u32 {
        return self.active_goroutines.load(.acquire);
    }
};

// Global state with proper cleanup
var next_channel_id: Atomic(ChannelId) = Atomic(ChannelId).init(1);
var global_scheduler: ?*MemorySafeScheduler = null;
var scheduler_mutex: Mutex = Mutex{};

/// Generate unique channel ID
fn generateChannelId() ChannelId {
    return next_channel_id.fetchAdd(1, .acq_rel);
}

/// Initialize global scheduler with memory safety
pub fn initializeScheduler(allocator: Allocator, num_workers: usize) !void {
    scheduler_mutex.lock();
    defer scheduler_mutex.unlock();

    if (global_scheduler != null) {
        return; // Already initialized
    }

    const scheduler = try MemorySafeScheduler.init(allocator, num_workers);
    global_scheduler = scheduler;
    
    try scheduler.start();
}

/// Get global scheduler
pub fn getScheduler() ?*MemorySafeScheduler {
    scheduler_mutex.lock();
    defer scheduler_mutex.unlock();
    return global_scheduler;
}

/// Shutdown global scheduler with proper cleanup
pub fn shutdownScheduler() void {
    scheduler_mutex.lock();
    defer scheduler_mutex.unlock();

    if (global_scheduler) |scheduler| {
        scheduler.deinit();
        global_scheduler = null;
    }
}

/// Memory-safe channel creation
pub fn makeMemorySafeChannel(comptime T: type, allocator: Allocator, capacity: usize) !*MemorySafeChannel(T) {
    return MemorySafeChannel(T).init(allocator, capacity);
}

/// Memory-safe goroutine spawning
pub fn spawnMemorySafe(allocator: Allocator, entry_fn: GoroutineEntry, context: ?*anyopaque) !GoroutineId {
    const scheduler = getScheduler() orelse {
        // Initialize with default number of workers if not already done
        const num_workers = @max(1, std.Thread.getCpuCount() catch 1);
        try initializeScheduler(allocator, num_workers);
        return spawnMemorySafe(allocator, entry_fn, context);
    };
    
    return scheduler.spawn(entry_fn, context);
}

// Tests for memory-safe concurrency
test "memory safe channel creation and cleanup" {
    const allocator = std.testing.allocator;
    
    var channel = try makeMemorySafeChannel(i32, allocator, 3);
    defer channel.release();

    // Test basic operations
    try std.testing.expect(try channel.send(42) == SendResult.sent);
    const received = try channel.receive();
    try std.testing.expect(received.? == 42);
}

test "memory safe goroutine execution" {
    const allocator = std.testing.allocator;
    
    var executed = false;
    const TestContext = struct {
        executed: *bool,
    };
    
    var context = TestContext{ .executed = &executed };
    
    const testFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx: *TestContext = @ptrCast(@alignCast(ctx.?));
            test_ctx.executed.* = true;
        }
    }.run;

    const goroutine_id = try spawnMemorySafe(allocator, testFn, &context);
    defer shutdownScheduler();
    
    // Wait a bit for execution
    std.time.sleep(10_000_000); // 10ms
    
    try std.testing.expect(executed);
    try std.testing.expect(goroutine_id > 0);
}

test "memory safe scheduler lifecycle" {
    const allocator = std.testing.allocator;
    
    try initializeScheduler(allocator, 2);
    
    const scheduler = getScheduler();
    try std.testing.expect(scheduler != null);
    try std.testing.expect(scheduler.?.isRunning());
    
    shutdownScheduler();
    
    const scheduler_after = getScheduler();
    try std.testing.expect(scheduler_after == null);
}
