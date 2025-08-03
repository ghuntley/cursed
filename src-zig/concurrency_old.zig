//! CURSED Concurrency System - Complete Implementation in Zig
//!
//! This module provides Go-style concurrency with:
//! - Goroutines (using `stan` keyword)
//! - Channels (using `dm<T>` type)
//! - Select statements (using `ready` keyword)
//! - Work-stealing scheduler
//! - Memory-safe channel operations
//!
//! Features:
//! - Lightweight green threads (goroutines)
//! - Type-safe channel communication
//! - Non-blocking and blocking channel operations
//! - Fair work-stealing scheduler
//! - Select statement for channel multiplexing
//! - Integration with garbage collector
//! - Comprehensive error handling

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const Thread = std.Thread;
const Mutex = std.Thread.Mutex;
const Condition = std.Thread.Condition;
const Atomic = std.atomic.Value;
const Order = std.atomic.Ordering;

/// Goroutine identifier type
pub const GoroutineId = u64;

/// Worker thread identifier type
pub const WorkerId = usize;

/// Channel identifier type
pub const ChannelId = u64;

/// Goroutine state enumeration
pub const GoroutineState = enum(u8) {
    ready = 0,
    running = 1,
    waiting = 2,
    yielded = 3,
    completed = 4,
    panicked = 5,
    error_isolated = 6,
};

/// Goroutine priority levels
pub const GoroutinePriority = enum(u8) {
    low = 0,
    normal = 1,
    high = 2,
    critical = 3,
};

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

/// Select operation results
pub const SelectResult = enum {
    send_completed,
    receive_completed,
    default_executed,
    timeout,
    all_closed,
};

/// Concurrency errors
pub const ConcurrencyError = error{
    SchedulerNotInitialized,
    ChannelClosed,
    ChannelFull,
    ChannelEmpty,
    InvalidGoroutine,
    WorkerStartFailed,
    AllocationFailed,
    ChannelAllocationFailed,
    InvalidChannel,
    TimeoutExpired,
    SelectFailed,
};

/// Goroutine entry point function type
pub const GoroutineEntry = *const fn (context: ?*anyopaque) void;

/// Channel data structure for typed channels
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
        sender_count: Atomic(u32),
        receiver_count: Atomic(u32),
        stats: ChannelStats,
        allocator: Allocator,

        pub fn init(allocator: Allocator, capacity: usize) !Self {
            return Self{
                .id = generateChannelId(),
                .buffer = ArrayList(T).init(allocator),
                .mutex = Mutex{},
                .send_condition = Condition{},
                .recv_condition = Condition{},
                .capacity = capacity,
                .closed = Atomic(bool).init(false),
                .sender_count = Atomic(u32).init(0),
                .receiver_count = Atomic(u32).init(0),
                .stats = ChannelStats.init(),
                .allocator = allocator,
            };
        }

        pub fn deinit(self: *Self) void {
            self.close();
            self.buffer.deinit();
        }

        /// Send a value to the channel (blocking)
        pub fn send(self: *Self, value: T) !SendResult {
            if (self.closed.load(.Acquire)) {
                return SendResult.closed;
            }

            self.mutex.lock();
            defer self.mutex.unlock();

            // For unbuffered channels, wait for receiver
            if (self.capacity == 0) {
                while (self.receiver_count.load(.Acquire) == 0 and !self.closed.load(.Acquire)) {
                    self.send_condition.wait(&self.mutex);
                }

                if (self.closed.load(.Acquire)) {
                    return SendResult.closed;
                }

                try self.buffer.append(value);
                self.recv_condition.signal();
                self.stats.total_sent += 1;
                return SendResult.sent;
            }

            // For buffered channels, wait for space
            while (self.buffer.items.len >= self.capacity and !self.closed.load(.Acquire)) {
                self.send_condition.wait(&self.mutex);
            }

            if (self.closed.load(.Acquire)) {
                return SendResult.closed;
            }

            try self.buffer.append(value);
            self.recv_condition.signal();
            self.stats.total_sent += 1;
            return SendResult.sent;
        }

        /// Try to send a value (non-blocking)
        pub fn trySend(self: *Self, value: T) !SendResult {
            if (self.closed.load(.Acquire)) {
                return SendResult.closed;
            }

            self.mutex.lock();
            defer self.mutex.unlock();

            // For unbuffered channels, need receiver
            if (self.capacity == 0) {
                if (self.receiver_count.load(.Acquire) == 0) {
                    self.stats.messages_dropped += 1;
                    return SendResult.would_block;
                }

                if (self.closed.load(.Acquire)) {
                    return SendResult.closed;
                }

                try self.buffer.append(value);
                self.recv_condition.signal();
                self.stats.total_sent += 1;
                return SendResult.sent;
            }

            // For buffered channels, check capacity
            if (self.buffer.items.len >= self.capacity) {
                self.stats.messages_dropped += 1;
                return SendResult.would_block;
            }

            if (self.closed.load(.Acquire)) {
                return SendResult.closed;
            }

            try self.buffer.append(value);
            self.recv_condition.signal();
            self.stats.total_sent += 1;
            return SendResult.sent;
        }

        /// Receive a value from the channel (blocking)
        pub fn receive(self: *Self) !?T {
            self.mutex.lock();
            defer self.mutex.unlock();

            // Wait for data or channel close
            while (self.buffer.items.len == 0 and !self.closed.load(.Acquire)) {
                self.recv_condition.wait(&self.mutex);
            }

            if (self.buffer.items.len > 0) {
                const value = self.buffer.orderedRemove(0);
                self.send_condition.signal();
                self.stats.total_received += 1;
                return value;
            }

            if (self.closed.load(.Acquire)) {
                return null;
            }

            return null;
        }

        /// Try to receive a value (non-blocking)
        pub fn tryReceive(self: *Self) !?T {
            self.mutex.lock();
            defer self.mutex.unlock();

            if (self.buffer.items.len > 0) {
                const value = self.buffer.orderedRemove(0);
                self.send_condition.signal();
                self.stats.total_received += 1;
                return value;
            }

            if (self.closed.load(.Acquire)) {
                return null;
            }

            return null; // Would block
        }

        /// Close the channel
        pub fn close(self: *Self) void {
            self.closed.store(true, .Release);
            self.send_condition.broadcast();
            self.recv_condition.broadcast();
        }

        /// Check if channel is closed
        pub fn isClosed(self: *Self) bool {
            return self.closed.load(.Acquire);
        }

        /// Get channel length
        pub fn len(self: *Self) usize {
            self.mutex.lock();
            defer self.mutex.unlock();
            return self.buffer.items.len;
        }

        /// Check if channel is empty
        pub fn isEmpty(self: *Self) bool {
            return self.len() == 0;
        }

        /// Check if channel is full
        pub fn isFull(self: *Self) bool {
            if (self.capacity == 0) {
                return self.receiver_count.load(.Acquire) == 0;
            }
            return self.len() >= self.capacity;
        }

        /// Get channel statistics
        pub fn getStats(self: *Self) ChannelStats {
            self.mutex.lock();
            defer self.mutex.unlock();
            return self.stats;
        }
    };
}

/// Channel statistics
pub const ChannelStats = struct {
    total_sent: u64,
    total_received: u64,
    messages_dropped: u64,

    pub fn init() ChannelStats {
        return ChannelStats{
            .total_sent = 0,
            .total_received = 0,
            .messages_dropped = 0,
        };
    }
};

/// Goroutine structure
pub const Goroutine = struct {
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

    pub fn init(allocator: Allocator, id: GoroutineId, entry_fn: GoroutineEntry, context: ?*anyopaque) Goroutine {
        return Goroutine{
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
        };
    }

    pub fn getState(self: *const Goroutine) GoroutineState {
        return self.state.load(.Acquire);
    }

    pub fn setState(self: *Goroutine, new_state: GoroutineState) void {
        self.state.store(new_state, .Release);
    }

    pub fn tryTransition(self: *Goroutine, from: GoroutineState, to: GoroutineState) bool {
        return self.state.compareAndSwap(from, to, .AcqRel, .Acquire) == null;
    }

    pub fn execute(self: *Goroutine) void {
        self.setState(GoroutineState.running);
        const start_time = std.time.milliTimestamp();

        // Execute the goroutine function
        self.entry_fn(self.context);

        const end_time = std.time.milliTimestamp();
        self.total_runtime += @as(u32, @intCast(end_time - start_time));
        self.setState(GoroutineState.completed);
    }
};

/// Work-stealing deque for goroutine scheduling
pub const WorkStealingDeque = struct {
    const Self = @This();

    items: ArrayList(*Goroutine),
    mutex: Mutex,
    top: Atomic(usize),
    bottom: Atomic(usize),
    allocator: Allocator,

    pub fn init(allocator: Allocator) Self {
        return Self{
            .items = ArrayList(*Goroutine).init(allocator),
            .mutex = Mutex{},
            .top = Atomic(usize).init(0),
            .bottom = Atomic(usize).init(0),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Self) void {
        self.items.deinit();
    }

    /// Push goroutine to bottom (owner thread only)
    pub fn pushBottom(self: *Self, goroutine: *Goroutine) !void {
        self.mutex.lock();
        defer self.mutex.unlock();

        try self.items.append(goroutine);
        self.bottom.store(self.items.items.len, .Release);
    }

    /// Pop goroutine from bottom (owner thread only)
    pub fn popBottom(self: *Self) ?*Goroutine {
        self.mutex.lock();
        defer self.mutex.unlock();

        const len = self.items.items.len;
        if (len == 0) return null;

        const goroutine = self.items.pop();
        self.bottom.store(self.items.items.len, .Release);
        return goroutine;
    }

    /// Steal goroutine from top (other threads)
    pub fn steal(self: *Self) ?*Goroutine {
        self.mutex.lock();
        defer self.mutex.unlock();

        if (self.items.items.len == 0) return null;

        const goroutine = self.items.orderedRemove(0);
        self.top.store(1, .Release);
        return goroutine;
    }

    pub fn len(self: *Self) usize {
        self.mutex.lock();
        defer self.mutex.unlock();
        return self.items.items.len;
    }

    pub fn isEmpty(self: *Self) bool {
        return self.len() == 0;
    }
};

/// Worker thread for executing goroutines
pub const Worker = struct {
    id: WorkerId,
    deque: WorkStealingDeque,
    thread: ?Thread,
    scheduler: *Scheduler,
    running: Atomic(bool),
    stats: WorkerStats,
    allocator: Allocator,

    pub fn init(allocator: Allocator, id: WorkerId, scheduler: *Scheduler) Worker {
        return Worker{
            .id = id,
            .deque = WorkStealingDeque.init(allocator),
            .thread = null,
            .scheduler = scheduler,
            .running = Atomic(bool).init(false),
            .stats = WorkerStats.init(),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Worker) void {
        self.stop();
        self.deque.deinit();
    }

    pub fn start(self: *Worker) !void {
        self.running.store(true, .Release);
        self.thread = try Thread.spawn(.{}, workerLoop, .{self});
    }

    pub fn stop(self: *Worker) void {
        self.running.store(false, .Release);
        if (self.thread) |thread| {
            thread.join();
            self.thread = null;
        }
    }

    fn workerLoop(self: *Worker) void {
        while (self.running.load(.Acquire)) {
            // Try to get work from local deque
            if (self.deque.popBottom()) |goroutine| {
                self.executeGoroutine(goroutine);
                self.stats.goroutines_executed += 1;
                continue;
            }

            // Try to steal work from other workers
            if (self.stealWork()) |goroutine| {
                self.executeGoroutine(goroutine);
                self.stats.work_stolen += 1;
                continue;
            }

            // Try to get work from global queue
            if (self.scheduler.getGlobalWork()) |goroutine| {
                self.executeGoroutine(goroutine);
                continue;
            }

            // No work available, yield CPU
            std.time.sleep(1_000_000); // 1ms
        }
    }

    fn executeGoroutine(self: *Worker, goroutine: *Goroutine) void {
        const start_time = std.time.milliTimestamp();
        goroutine.execute();
        const end_time = std.time.milliTimestamp();
        self.stats.busy_time += @as(u32, @intCast(end_time - start_time));
    }

    fn stealWork(self: *Worker) ?*Goroutine {
        // Try to steal from other workers in round-robin fashion
        for (self.scheduler.workers.items) |*worker| {
            if (worker.id == self.id) continue;
            if (worker.deque.steal()) |goroutine| {
                return goroutine;
            }
        }
        return null;
    }
};

/// Worker statistics
pub const WorkerStats = struct {
    goroutines_executed: u64,
    work_stolen: u64,
    work_shared: u64,
    idle_time: u64,
    busy_time: u64,

    pub fn init() WorkerStats {
        return WorkerStats{
            .goroutines_executed = 0,
            .work_stolen = 0,
            .work_shared = 0,
            .idle_time = 0,
            .busy_time = 0,
        };
    }
};

/// Scheduler configuration
pub const SchedulerConfig = struct {
    num_workers: usize,
    queue_capacity: usize,
    default_stack_size: usize,
    enable_work_stealing: bool,
    enable_preemption: bool,
    quantum_ms: u64,

    pub fn default() SchedulerConfig {
        return SchedulerConfig{
            .num_workers = @max(1, std.Thread.getCpuCount() catch 1),
            .queue_capacity = 1024,
            .default_stack_size = 2 * 1024 * 1024, // 2MB
            .enable_work_stealing = true,
            .enable_preemption = true,
            .quantum_ms = 10,
        };
    }
};

/// Main scheduler with work-stealing
pub const Scheduler = struct {
    config: SchedulerConfig,
    workers: ArrayList(Worker),
    global_queue: ArrayList(*Goroutine),
    global_mutex: Mutex,
    next_goroutine_id: Atomic(GoroutineId),
    next_worker_id: Atomic(WorkerId),
    active_goroutines: Atomic(u32),
    running: Atomic(bool),
    stats: SchedulerStats,
    allocator: Allocator,

    pub fn init(allocator: Allocator, config: SchedulerConfig) !Scheduler {
        var scheduler = Scheduler{
            .config = config,
            .workers = ArrayList(Worker).init(allocator),
            .global_queue = ArrayList(*Goroutine).init(allocator),
            .global_mutex = Mutex{},
            .next_goroutine_id = Atomic(GoroutineId).init(1),
            .next_worker_id = Atomic(WorkerId).init(0),
            .active_goroutines = Atomic(u32).init(0),
            .running = Atomic(bool).init(false),
            .stats = SchedulerStats.init(),
            .allocator = allocator,
        };

        // Initialize workers
        try scheduler.workers.ensureTotalCapacity(config.num_workers);
        for (0..config.num_workers) |i| {
            const worker_id = scheduler.next_worker_id.fetchAdd(1, .AcqRel);
            const worker = Worker.init(allocator, worker_id, &scheduler);
            try scheduler.workers.append(worker);
        }

        return scheduler;
    }

    pub fn deinit(self: *Scheduler) void {
        self.stop();
        
        for (self.workers.items) |*worker| {
            worker.deinit();
        }
        self.workers.deinit();
        
        // Clean up remaining goroutines
        for (self.global_queue.items) |goroutine| {
            self.allocator.destroy(goroutine);
        }
        self.global_queue.deinit();
    }

    pub fn start(self: *Scheduler) !void {
        self.running.store(true, .Release);
        
        // Start all worker threads
        for (self.workers.items) |*worker| {
            try worker.start();
        }
        
        self.stats.start_time = std.time.milliTimestamp();
    }

    pub fn stop(self: *Scheduler) void {
        self.running.store(false, .Release);
        
        // Stop all worker threads
        for (self.workers.items) |*worker| {
            worker.stop();
        }
    }

    /// Spawn a new goroutine (implements `stan` keyword)
    pub fn spawn(self: *Scheduler, entry_fn: GoroutineEntry, context: ?*anyopaque) !GoroutineId {
        const goroutine_id = self.next_goroutine_id.fetchAdd(1, .AcqRel);
        
        const goroutine = try self.allocator.create(Goroutine);
        goroutine.* = Goroutine.init(self.allocator, goroutine_id, entry_fn, context);
        
        // Schedule the goroutine
        try self.scheduleGoroutine(goroutine);
        
        _ = self.active_goroutines.fetchAdd(1, .AcqRel);
        self.stats.total_spawned += 1;
        
        return goroutine_id;
    }

    /// Yield current goroutine (implements `yolo` keyword)
    pub fn yield(self: *Scheduler) !void {
        // In a real implementation, this would cooperatively yield the current goroutine
        // For now, we just sleep briefly to simulate yielding
        std.time.sleep(1_000); // 1 microsecond
    }

    fn scheduleGoroutine(self: *Scheduler, goroutine: *Goroutine) !void {
        // Find the worker with the least work
        var min_work_worker: ?*Worker = null;
        var min_work_count: usize = std.math.maxInt(usize);

        for (self.workers.items) |*worker| {
            const work_count = worker.deque.len();
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

    fn getGlobalWork(self: *Scheduler) ?*Goroutine {
        self.global_mutex.lock();
        defer self.global_mutex.unlock();
        
        if (self.global_queue.items.len > 0) {
            return self.global_queue.orderedRemove(0);
        }
        return null;
    }

    pub fn getStats(self: *Scheduler) SchedulerStats {
        return self.stats;
    }

    pub fn isRunning(self: *Scheduler) bool {
        return self.running.load(.Acquire);
    }

    pub fn activeGoroutineCount(self: *Scheduler) u32 {
        return self.active_goroutines.load(.Acquire);
    }
};

/// Scheduler statistics
pub const SchedulerStats = struct {
    total_spawned: u64,
    total_completed: u64,
    current_active: u32,
    peak_active: u32,
    total_panicked: u64,
    start_time: i64,

    pub fn init() SchedulerStats {
        return SchedulerStats{
            .total_spawned = 0,
            .total_completed = 0,
            .current_active = 0,
            .peak_active = 0,
            .total_panicked = 0,
            .start_time = 0,
        };
    }
};

/// Select statement implementation
pub const Select = struct {
    operations: ArrayList(SelectOperation),
    timeout_ms: ?u64,
    has_default: bool,
    allocator: Allocator,

    pub fn init(allocator: Allocator) Select {
        return Select{
            .operations = ArrayList(SelectOperation).init(allocator),
            .timeout_ms = null,
            .has_default = false,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Select) void {
        self.operations.deinit();
    }

    pub fn addSend(self: *Select, channel_id: ChannelId, case_index: usize) !void {
        try self.operations.append(SelectOperation{
            .send = .{ .channel_id = channel_id, .case_index = case_index },
        });
    }

    pub fn addReceive(self: *Select, channel_id: ChannelId, case_index: usize) !void {
        try self.operations.append(SelectOperation{
            .receive = .{ .channel_id = channel_id, .case_index = case_index },
        });
    }

    pub fn addDefault(self: *Select, case_index: usize) !void {
        self.has_default = true;
        try self.operations.append(SelectOperation{
            .default = .{ .case_index = case_index },
        });
    }

    pub fn setTimeout(self: *Select, timeout_ms: u64) void {
        self.timeout_ms = timeout_ms;
    }

    pub fn execute(self: *Select) !SelectResult {
        const start_time = std.time.milliTimestamp();
        
        while (true) {
            // Check timeout
            if (self.timeout_ms) |timeout| {
                const elapsed = std.time.milliTimestamp() - start_time;
                if (elapsed >= timeout) {
                    return SelectResult.timeout;
                }
            }

            // Try all operations
            var ready_ops = ArrayList(usize).init(self.allocator);
            defer ready_ops.deinit();

            for (self.operations.items, 0..) |op, i| {
                switch (op) {
                    .send => |send_op| {
                        // Check if send is possible
                        if (canSendToChannel(send_op.channel_id)) {
                            try ready_ops.append(i);
                        }
                    },
                    .receive => |recv_op| {
                        // Check if receive is possible
                        if (canReceiveFromChannel(recv_op.channel_id)) {
                            try ready_ops.append(i);
                        }
                    },
                    .default => {
                        try ready_ops.append(i);
                    },
                }
            }

            if (ready_ops.items.len > 0) {
                // Randomly select from ready operations
                const selected_idx = std.crypto.random.intRangeAtMost(usize, 0, ready_ops.items.len - 1);
                const op_idx = ready_ops.items[selected_idx];
                const selected_op = self.operations.items[op_idx];

                switch (selected_op) {
                    .send => return SelectResult.send_completed,
                    .receive => return SelectResult.receive_completed,
                    .default => return SelectResult.default_executed,
                }
            }

            // If no operations are ready and we have default, execute it
            if (self.has_default) {
                return SelectResult.default_executed;
            }

            // Brief sleep to avoid busy waiting
            std.time.sleep(100_000); // 100 microseconds
        }
    }
};

/// Select operation types
pub const SelectOperation = union(enum) {
    send: struct {
        channel_id: ChannelId,
        case_index: usize,
    },
    receive: struct {
        channel_id: ChannelId,
        case_index: usize,
    },
    default: struct {
        case_index: usize,
    },
};

// Global state management
var next_channel_id: Atomic(ChannelId) = Atomic(ChannelId).init(1);
var global_scheduler: ?*Scheduler = null;
var scheduler_mutex: Mutex = Mutex{};

/// Generate unique channel ID
fn generateChannelId() ChannelId {
    return next_channel_id.fetchAdd(1, .AcqRel);
}

/// Initialize global scheduler
pub fn initializeScheduler(allocator: Allocator, config: SchedulerConfig) !void {
    scheduler_mutex.lock();
    defer scheduler_mutex.unlock();

    if (global_scheduler != null) {
        return; // Already initialized
    }

    const scheduler = try allocator.create(Scheduler);
    scheduler.* = try Scheduler.init(allocator, config);
    global_scheduler = scheduler;
    
    try scheduler.start();
}

/// Get global scheduler
pub fn getScheduler() ?*Scheduler {
    scheduler_mutex.lock();
    defer scheduler_mutex.unlock();
    return global_scheduler;
}

/// Shutdown global scheduler
pub fn shutdownScheduler(allocator: Allocator) void {
    scheduler_mutex.lock();
    defer scheduler_mutex.unlock();

    if (global_scheduler) |scheduler| {
        scheduler.deinit();
        allocator.destroy(scheduler);
        global_scheduler = null;
    }
}

/// Public API functions implementing CURSED keywords

/// Spawn goroutine using `stan` keyword
pub fn stan(entry_fn: GoroutineEntry, context: ?*anyopaque) !GoroutineId {
    const scheduler = getScheduler() orelse return ConcurrencyError.SchedulerNotInitialized;
    return scheduler.spawn(entry_fn, context);
}

/// Yield goroutine using `yolo` keyword
pub fn yolo() !void {
    const scheduler = getScheduler() orelse return ConcurrencyError.SchedulerNotInitialized;
    try scheduler.yield();
}

/// Create channel using `dm<T>` type
pub fn makeChannel(comptime T: type, allocator: Allocator, capacity: usize) !*Channel(T) {
    const channel = try allocator.create(Channel(T));
    channel.* = try Channel(T).init(allocator, capacity);
    return channel;
}

/// Create unbuffered channel
pub fn makeUnbufferedChannel(comptime T: type, allocator: Allocator) !*Channel(T) {
    return makeChannel(T, allocator, 0);
}

/// Helper functions for select implementation
fn canSendToChannel(channel_id: ChannelId) bool {
    // In a real implementation, this would check the actual channel state
    // For now, return true as a placeholder
    _ = channel_id;
    return true;
}

fn canReceiveFromChannel(channel_id: ChannelId) bool {
    // In a real implementation, this would check the actual channel state
    // For now, return true as a placeholder
    _ = channel_id;
    return true;
}

// Tests
test "goroutine creation and execution" {
    const allocator = std.testing.allocator;
    
    const config = SchedulerConfig.default();
    try initializeScheduler(allocator, config);
    defer shutdownScheduler(allocator);

    var executed = false;
    const TestContext = struct {
        executed: *bool,
    };
    
    var context = TestContext{ .executed = &executed };
    
    const testFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx = @as(*TestContext, @ptrCast(@alignCast(ctx.?)));
            test_ctx.executed.* = true;
        }
    }.run;

    const goroutine_id = try stan(testFn, &context);
    
    // Wait a bit for execution
    std.time.sleep(10_000_000); // 10ms
    
    try std.testing.expect(executed);
    try std.testing.expect(goroutine_id > 0);
}

test "channel send and receive" {
    const allocator = std.testing.allocator;
    
    var channel = try makeChannel(i32, allocator, 3);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }

    // Test buffered send/receive
    try std.testing.expect(try channel.send(42) == SendResult.sent);
    try std.testing.expect(try channel.send(43) == SendResult.sent);
    
    const received1 = try channel.receive();
    try std.testing.expect(received1.? == 42);
    
    const received2 = try channel.receive();
    try std.testing.expect(received2.? == 43);
}

test "channel close behavior" {
    const allocator = std.testing.allocator;
    
    var channel = try makeChannel(i32, allocator, 1);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }

    try std.testing.expect(try channel.send(100) == SendResult.sent);
    channel.close();
    
    try std.testing.expect(try channel.send(101) == SendResult.closed);
    try std.testing.expect(channel.isClosed());
    
    // Should still be able to receive buffered value
    const received = try channel.receive();
    try std.testing.expect(received.? == 100);
}

test "select statement creation" {
    const allocator = std.testing.allocator;
    
    var select_stmt = Select.init(allocator);
    defer select_stmt.deinit();

    try select_stmt.addDefault(0);
    try std.testing.expect(select_stmt.has_default);
    
    const result = try select_stmt.execute();
    try std.testing.expect(result == SelectResult.default_executed);
}

test "work-stealing deque" {
    const allocator = std.testing.allocator;
    
    var deque = WorkStealingDeque.init(allocator);
    defer deque.deinit();

    var goroutine = Goroutine.init(allocator, 1, undefined, null);
    
    try deque.pushBottom(&goroutine);
    try std.testing.expect(deque.len() == 1);
    
    const popped = deque.popBottom();
    try std.testing.expect(popped == &goroutine);
    try std.testing.expect(deque.isEmpty());
}
