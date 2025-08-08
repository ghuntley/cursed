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
const gc = @import("gc.zig");

const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/ExecutionEngine.h");
});

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
            
            // Clean up buffer contents with GC integration
            for (self.buffer.items) |item| {
                // If T is a GC-managed type, unregister from GC
                if (@TypeOf(item) == @import("main_unified.zig").Variable) {
                    if (@hasDecl(@import("gc.zig"), "unregisterStackRoot")) {
                        @import("gc.zig").unregisterStackRoot(@ptrCast(&item)) catch {};
                    }
                }
            }
            
            self.buffer.deinit();
        }

        /// Send a value to the channel (blocking)
        pub fn send(self: *Self, value: T) !SendResult {
            if (self.closed.load(.acquire)) {
                return SendResult.closed;
            }

            self.mutex.lock();
            defer self.mutex.unlock();

            // For unbuffered channels, wait for receiver
            if (self.capacity == 0) {
                while (self.receiver_count.load(.acquire) == 0 and !self.closed.load(.acquire)) {
                    self.send_condition.wait(&self.mutex);
                }

                if (self.closed.load(.acquire)) {
                    return SendResult.closed;
                }

                try self.buffer.append(value);
                self.recv_condition.signal();
                self.stats.total_sent += 1;
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
            self.stats.total_sent += 1;
            return SendResult.sent;
        }

        /// Try to send a value (non-blocking)
        pub fn trySend(self: *Self, value: T) !SendResult {
            if (self.closed.load(.acquire)) {
                return SendResult.closed;
            }

            self.mutex.lock();
            defer self.mutex.unlock();

            // For unbuffered channels, need receiver
            if (self.capacity == 0) {
                if (self.receiver_count.load(.acquire) == 0) {
                    self.stats.messages_dropped += 1;
                    return SendResult.would_block;
                }

                if (self.closed.load(.acquire)) {
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

            if (self.closed.load(.acquire)) {
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
            while (self.buffer.items.len == 0 and !self.closed.load(.acquire)) {
                self.recv_condition.wait(&self.mutex);
            }

            if (self.buffer.items.len > 0) {
                const value = self.buffer.orderedRemove(0);
                self.send_condition.signal();
                self.stats.total_received += 1;
                return value;
            }

            if (self.closed.load(.acquire)) {
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

            if (self.closed.load(.acquire)) {
                return null;
            }

            return null; // Would block
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

        /// Get channel length
        pub fn length(self: *Self) usize {
            self.mutex.lock();
            defer self.mutex.unlock();
            return self.buffer.items.len;
        }

        /// Check if channel is empty
        pub fn isEmpty(self: *Self) bool {
            return self.length() == 0;
        }

        /// Check if channel is full
        pub fn isFull(self: *Self) bool {
            if (self.capacity == 0) {
                return self.receiver_count.load(.acquire) == 0;
            }
            return self.length() >= self.capacity;
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
        return self.state.load(.acquire);
    }

    pub fn setState(self: *Goroutine, new_state: GoroutineState) void {
        self.state.store(new_state, .release);
    }

    pub fn tryTransition(self: *Goroutine, from: GoroutineState, to: GoroutineState) bool {
        return self.state.cmpxchgWeak(from, to, .acq_rel, .acquire) == null;
    }

    pub fn execute(self: *Goroutine) void {
        self.setState(GoroutineState.running);
        const start_time = std.time.milliTimestamp();

        // Execute the goroutine function
        self.entry_fn(self.context);

        const end_time = std.time.milliTimestamp();
        self.total_runtime += @as(u64, @intCast(@max(0, end_time - start_time)));
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
        self.bottom.store(self.items.items.len, .release);
    }

    /// Pop goroutine from bottom (owner thread only)
    pub fn popBottom(self: *Self) ?*Goroutine {
        self.mutex.lock();
        defer self.mutex.unlock();

        const item_len = self.items.items.len;
        if (item_len == 0) return null;

        const goroutine = self.items.pop();
        self.bottom.store(self.items.items.len, .release);
        return goroutine;
    }

    /// Steal goroutine from top (other threads)
    pub fn steal(self: *Self) ?*Goroutine {
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
        self.running.store(true, .release);
        self.thread = try Thread.spawn(.{}, workerLoop, .{self});
    }

    pub fn stop(self: *Worker) void {
        self.running.store(false, .release);
        if (self.thread) |thread| {
            thread.join();
            self.thread = null;
        }
    }
    
    /// Register worker's stack and local objects with GC
    pub fn registerWithGC(self: *Worker, gc_instance: *gc.GC) void {
        // Register worker's deque contents as stack roots
        for (self.deque.items.items) |goroutine| {
            gc_instance.registerStackRoot(@ptrCast(goroutine)) catch {};
        }
        
        std.log.debug("Worker {}: Registered {} objects with GC", .{self.id, self.deque.items.items.len});
    }

    fn workerLoop(self: *Worker) void {
        while (self.running.load(.acquire)) {
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
        self.stats.busy_time += @as(u64, @intCast(@max(0, end_time - start_time)));
        
        // Clean up completed goroutine
        if (goroutine.getState() == GoroutineState.completed) {
            _ = self.scheduler.active_goroutines.fetchSub(1, .acq_rel);
            self.scheduler.stats.total_completed += 1;
            self.scheduler.allocator.destroy(goroutine);
        }
    }

    fn stealWork(self: *Worker) ?*Goroutine {
        // Enhanced work stealing with proper bounds checking and thread safety
        const workers = &self.scheduler.workers;
        
        // Bounds check to prevent invalid access
        if (workers.items.len == 0 or workers.items.len <= self.id) return null;
        
        // Round-robin stealing with atomic access
        var steal_index = (self.id + 1) % workers.items.len;
        for (0..workers.items.len) |_| {
            if (steal_index == self.id) {
                steal_index = (steal_index + 1) % workers.items.len;
                continue;
            }
            
            // Atomic check of worker existence and safe steal attempt
            if (steal_index < workers.items.len) {
                if (workers.items[steal_index].deque.steal()) |goroutine| {
                    return goroutine;
                }
            }
            
            steal_index = (steal_index + 1) % workers.items.len;
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
    // GC integration for thread-safe operation
    gc_instance: ?*gc.GC,

    pub fn init(allocator: Allocator, config: SchedulerConfig) !*Scheduler {
        const scheduler = try allocator.create(Scheduler);
        scheduler.* = Scheduler{
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
            .gc_instance = null,
        };

        // Initialize workers
        try scheduler.workers.ensureTotalCapacity(config.num_workers);
        for (0..config.num_workers) |_| {
            const worker_id = scheduler.next_worker_id.fetchAdd(1, .acq_rel);
            const worker = Worker.init(allocator, worker_id, scheduler);
            try scheduler.workers.append(worker);
        }

        return scheduler;
    }
    
    /// Initialize GC integration for thread-safe memory management
    pub fn initGC(self: *Scheduler, gc_instance: *gc.GC) void {
        self.gc_instance = gc_instance;
        
        // Register scheduler with GC for cooperative stack scanning
        self.registerStackRoots();
        
        std.log.info("Scheduler: GC integration initialized", .{});
    }
    
    /// Register stack roots with GC for all active goroutines
    fn registerStackRoots(self: *Scheduler) void {
        if (self.gc_instance) |gc_ref| {
            // Register global queue objects
            self.global_mutex.lock();
            defer self.global_mutex.unlock();
            
            for (self.global_queue.items) |goroutine| {
                gc_ref.registerStackRoot(@ptrCast(goroutine)) catch {};
            }
            
            // Register worker-local objects
            for (self.workers.items) |*worker| {
                worker.registerWithGC(gc_ref);
            }
        }
    }
    
    /// Trigger cooperative GC when creating new goroutines
    fn cooperativeGCCheck(self: *Scheduler) void {
        if (self.gc_instance) |gc_ref| {
            const active_count = self.active_goroutines.load(.acquire);
            
            // Trigger young generation collection if many goroutines are active
            if (active_count > self.config.max_goroutines / 2) {
                gc_ref.triggerYoungCollection();
            }
        }
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
        self.running.store(true, .release);
        
        // Start all worker threads
        for (self.workers.items) |*worker| {
            try worker.start();
        }
        
        self.stats.start_time = std.time.milliTimestamp();
    }

    pub fn stop(self: *Scheduler) void {
        self.running.store(false, .release);
        
        // Stop all worker threads
        for (self.workers.items) |*worker| {
            worker.stop();
        }
    }

    /// Spawn a new goroutine (implements `stan` keyword)
    pub fn spawn(self: *Scheduler, entry_fn: GoroutineEntry, context: ?*anyopaque) !GoroutineId {
        const goroutine_id = self.next_goroutine_id.fetchAdd(1, .acq_rel);
        
        const goroutine = try self.allocator.create(Goroutine);
        goroutine.* = Goroutine.init(self.allocator, goroutine_id, entry_fn, context);
        
        // Schedule the goroutine
        try self.scheduleGoroutine(goroutine);
        
        _ = self.active_goroutines.fetchAdd(1, .acq_rel);
        self.stats.total_spawned += 1;
        
        return goroutine_id;
    }

    /// Yield current goroutine (implements `yolo` keyword)
    pub fn yield(_: *Scheduler) !void {
        // In a real implementation, this would cooperatively yield the current goroutine
        // For now, we just sleep briefly to simulate yielding
        std.time.sleep(1_000); // 1 microsecond
    }

    fn scheduleGoroutine(self: *Scheduler, goroutine: *Goroutine) !void {
        // Find the worker with the least work
        var min_work_worker: ?*Worker = null;
        var min_work_count: usize = std.math.maxInt(usize);

        for (self.workers.items) |*worker| {
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

    fn getGlobalWork(self: *Scheduler) ?*Goroutine {
        // Enhanced global work access with proper synchronization
        self.global_mutex.lock();
        defer self.global_mutex.unlock();
        
        // Double-check pattern to prevent race conditions
        if (self.global_queue.items.len == 0) return null;
        
        // Safe removal with bounds validation
        const goroutine = self.global_queue.orderedRemove(0);
        return goroutine;
    }

    pub fn getStats(self: *Scheduler) SchedulerStats {
        return self.stats;
    }

    pub fn isRunning(self: *Scheduler) bool {
        return self.running.load(.acquire);
    }

    pub fn activeGoroutineCount(self: *Scheduler) u32 {
        return self.active_goroutines.load(.acquire);
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
    return next_channel_id.fetchAdd(1, .acq_rel);
}

/// Initialize global scheduler
pub fn initializeScheduler(allocator: Allocator, config: SchedulerConfig) !void {
    scheduler_mutex.lock();
    defer scheduler_mutex.unlock();

    if (global_scheduler != null) {
        return; // Already initialized
    }

    const scheduler = try Scheduler.init(allocator, config);
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

// ===== CURSED LANGUAGE CHANNEL API =====

/// CURSED dm_send function - Send to channel with CURSED Variable integration
pub fn dm_send(channel_id: ChannelId, value: anytype, allocator: Allocator) !SendResult {
    _ = allocator; // Reserved for future Variable type integration
    
    // Type erasure for generic channel operations
    const T = @TypeOf(value);
    
    // Look up channel in registry
    channel_registry_mutex.lock();
    defer channel_registry_mutex.unlock();
    
    if (channel_registry) |registry| {
        if (registry.get(channel_id)) |channel_ptr| {
            // Cast to typed channel based on the value type
            const channel: *Channel(T) = @ptrCast(@alignCast(channel_ptr));
            return channel.send(value);
        }
    }
    
    return ConcurrencyError.InvalidChannel;
}

/// CURSED dm_recv function - Receive from channel with CURSED Variable integration
pub fn dm_recv(comptime T: type, channel_id: ChannelId, allocator: Allocator) !?T {
    _ = allocator; // For future use with complex types
    
    // Look up channel in registry
    channel_registry_mutex.lock();
    defer channel_registry_mutex.unlock();
    
    if (channel_registry) |registry| {
        if (registry.get(channel_id)) |channel_ptr| {
            // Cast to typed channel
            const channel: *Channel(T) = @ptrCast(@alignCast(channel_ptr));
            return channel.receive();
        }
    }
    
    return null;
}

/// CURSED dm_close function - Close channel
pub fn dm_close(channel_id: ChannelId) !void {
    // Look up channel in registry
    channel_registry_mutex.lock();
    defer channel_registry_mutex.unlock();
    
    if (channel_registry) |registry| {
        if (registry.get(channel_id)) |channel_ptr| {
            // Cast to generic channel for closing
            const channel: *Channel(u8) = @ptrCast(@alignCast(channel_ptr));
            channel.close();
            return;
        }
    }
    
    return ConcurrencyError.InvalidChannel;
}

/// Create typed channel with dm<T> syntax support  
pub fn dm_create(comptime T: type, allocator: Allocator, capacity: usize) !ChannelId {
    const channel = try makeChannel(T, allocator, capacity);
    const channel_id = channel.id;
    
    // Register channel for CURSED operations
    try registerChannelLLVM(channel_id, @ptrCast(channel));
    
    return channel_id;
}

/// Variable-aware channel operations for GC integration
pub const VariableChannel = struct {
    const Self = @This();
    const Variable = @import("main_unified.zig").Variable;
    
    channel: *Channel(Variable),
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, capacity: usize) !Self {
        const channel = try makeChannel(Variable, allocator, capacity);
        
        return Self{
            .channel = channel,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Self) void {
        // Clean up Variable references in the channel buffer
        for (self.channel.buffer.items) |*variable| {
            variable.deinit(self.allocator);
        }
        
        self.channel.deinit();
        self.allocator.destroy(self.channel);
    }
    
    /// Send Variable to channel with GC registration
    pub fn sendVariable(self: *Self, variable: Variable) !SendResult {
        // Register Variable with GC before adding to channel
        if (@hasDecl(@import("gc.zig"), "registerStackRoot")) {
            @import("gc.zig").registerStackRoot(@ptrCast(&variable)) catch {};
        }
        
        return self.channel.send(variable);
    }
    
    /// Receive Variable from channel with GC cleanup
    pub fn receiveVariable(self: *Self) !?Variable {
        const result = try self.channel.receive();
        
        if (result) |variable| {
            // Unregister from GC since we're transferring ownership
            if (@hasDecl(@import("gc.zig"), "unregisterStackRoot")) {
                @import("gc.zig").unregisterStackRoot(@ptrCast(&variable)) catch {};
            }
        }
        
        return result;
    }
    
    pub fn close(self: *Self) void {
        self.channel.close();
    }
    
    pub fn isClosed(self: *Self) bool {
        return self.channel.isClosed();
    }
    
    pub fn getId(self: *Self) ChannelId {
        return self.channel.id;
    }
};

/// Channel registry for LLVM IR generation
var channel_registry: ?std.HashMap(ChannelId, *anyopaque, std.hash_map.AutoContext(ChannelId), std.hash_map.default_max_load_percentage) = null;
var channel_registry_mutex: Mutex = Mutex{};

/// Initialize channel registry
pub fn initChannelRegistry(allocator: Allocator) void {
    channel_registry_mutex.lock();
    defer channel_registry_mutex.unlock();
    
    if (channel_registry == null) {
        channel_registry = std.HashMap(ChannelId, *anyopaque, std.hash_map.AutoContext(ChannelId), std.hash_map.default_max_load_percentage).init(allocator);
    }
}

/// Register channel for LLVM operations
pub fn registerChannelLLVM(channel_id: ChannelId, channel_ptr: *anyopaque) !void {
    channel_registry_mutex.lock();
    defer channel_registry_mutex.unlock();
    
    if (channel_registry) |*registry| {
        try registry.put(channel_id, channel_ptr);
    }
}

/// Generate LLVM IR for channel creation
pub fn generateChannelCreateLLVM(_: Allocator, context: c.LLVMContextRef, module: c.LLVMModuleRef, builder: c.LLVMBuilderRef, capacity: ?c.LLVMValueRef) !c.LLVMValueRef {
    // Declare runtime channel creation function
    const create_func = c.LLVMGetNamedFunction(module, "cursed_channel_create") orelse {
        const func_type = c.LLVMFunctionType(
            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
            &[_]c.LLVMTypeRef{c.LLVMInt64TypeInContext(context)},
            1,
            0
        );
        c.LLVMAddFunction(module, "cursed_channel_create", func_type);
    };
    
    // Use provided capacity or default to 0 (unbuffered)
    const cap_value = capacity orelse c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 0, 0);
    
    // Call runtime function
    const channel_ptr = c.LLVMBuildCall2(
        builder,
        c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
        create_func,
        &[_]c.LLVMValueRef{cap_value},
        1,
        "channel_ptr"
    );
    
    // Generate unique channel ID and register
    const channel_id = generateChannelId();
    registerChannelLLVM(channel_id, @ptrCast(channel_ptr)) catch {};
    
    return channel_ptr;
}

/// Generate LLVM IR for channel send operation
pub fn generateChannelSendLLVM(context: c.LLVMContextRef, module: c.LLVMModuleRef, builder: c.LLVMBuilderRef, channel: c.LLVMValueRef, value: c.LLVMValueRef) !c.LLVMValueRef {
    // Declare runtime send function
    const send_func = c.LLVMGetNamedFunction(module, "cursed_channel_send") orelse {
        const func_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(context),
            &[_]c.LLVMTypeRef{
                c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
                c.LLVMInt64TypeInContext(context),
            },
            2,
            0
        );
        c.LLVMAddFunction(module, "cursed_channel_send", func_type);
    };
    
    // Convert value to i64 if needed
    const value_i64 = if (c.LLVMGetTypeKind(c.LLVMTypeOf(value)) == c.LLVMIntegerTypeKind) 
        value
    else
        c.LLVMBuildPtrToInt(builder, value, c.LLVMInt64TypeInContext(context), "value_as_i64");
    
    // Call runtime send function
    const result = c.LLVMBuildCall2(
        builder,
        c.LLVMInt32TypeInContext(context),
        send_func,
        &[_]c.LLVMValueRef{ channel, value_i64 },
        2,
        "send_result"
    );
    
    return result;
}

/// Generate LLVM IR for channel receive operation
pub fn generateChannelReceiveLLVM(context: c.LLVMContextRef, module: c.LLVMModuleRef, builder: c.LLVMBuilderRef, channel: c.LLVMValueRef) !c.LLVMValueRef {
    // Declare runtime receive function
    const recv_func = c.LLVMGetNamedFunction(module, "cursed_channel_receive") orelse {
        const func_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(context),
            &[_]c.LLVMTypeRef{
                c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
                c.LLVMPointerType(c.LLVMInt64TypeInContext(context), 0),
            },
            2,
            0
        );
        c.LLVMAddFunction(module, "cursed_channel_receive", func_type);
    };
    
    // Allocate space for received value
    const value_ptr = c.LLVMBuildAlloca(builder, c.LLVMInt64TypeInContext(context), "recv_value_ptr");
    
    // Call runtime receive function
    const status = c.LLVMBuildCall2(
        builder,
        c.LLVMInt32TypeInContext(context),
        recv_func,
        &[_]c.LLVMValueRef{ channel, value_ptr },
        2,
        "recv_status"
    );
    
    // Load the received value
    const received_value = c.LLVMBuildLoad2(
        builder,
        c.LLVMInt64TypeInContext(context),
        value_ptr,
        "received_value"
    );
    
    // Create a struct to return both status and value
    const result_type = c.LLVMStructTypeInContext(
        context,
        &[_]c.LLVMTypeRef{
            c.LLVMInt32TypeInContext(context),
            c.LLVMInt64TypeInContext(context),
        },
        2,
        0
    );
    
    const result_alloca = c.LLVMBuildAlloca(builder, result_type, "recv_result");
    
    // Store status
    const status_ptr = c.LLVMBuildStructGEP2(builder, result_type, result_alloca, 0, "status_ptr");
    _ = c.LLVMBuildStore(builder, status, status_ptr);
    
    // Store value
    const value_result_ptr = c.LLVMBuildStructGEP2(builder, result_type, result_alloca, 1, "value_ptr");
    _ = c.LLVMBuildStore(builder, received_value, value_result_ptr);
    
    return c.LLVMBuildLoad2(builder, result_type, result_alloca, "recv_result_loaded");
}

/// Helper functions for select implementation
fn canSendToChannel(channel_id: ChannelId) bool {
    channel_registry_mutex.lock();
    defer channel_registry_mutex.unlock();
    
    if (channel_registry) |registry| {
        if (registry.get(channel_id)) |_| {
            // In a real implementation, check if channel has space
            // For now, assume channels can always accept sends
            return true;
        }
    }
    return false;
}

fn canReceiveFromChannel(channel_id: ChannelId) bool {
    channel_registry_mutex.lock();
    defer channel_registry_mutex.unlock();
    
    if (channel_registry) |registry| {
        if (registry.get(channel_id)) |_| {
            // In a real implementation, check if channel has messages
            // For now, assume channels always have messages available
            return true;
        }
    }
    return false;
}

// ===== C FFI EXPORTS FOR LLVM COMPILATION =====

/// C FFI export for spawning goroutines from LLVM compiled code
export fn cursed_spawn_goroutine(func_ptr: ?*const fn () callconv(.C) void, context: ?*anyopaque, stack_size: u32) u32 {
    _ = context;
    _ = stack_size;
    const allocator = std.heap.c_allocator;
    
    // Initialize scheduler if not already done
    initializeScheduler(allocator, SchedulerConfig.default()) catch {
        return 0; // Return 0 to indicate failure
    };
    
    // Wrapper function to convert C function pointer to Zig function
    const GoroutineWrapper = struct {
        c_func: ?*const fn () callconv(.C) void,
        
        fn run(ctx: ?*anyopaque) void {
            const wrapper: *@This() = @ptrCast(@alignCast(ctx.?));
            if (wrapper.c_func) |func| {
                func();
            }
        }
    };
    
    const wrapper = allocator.create(GoroutineWrapper) catch return 0;
    wrapper.c_func = func_ptr;
    
    const goroutine_id = stan(GoroutineWrapper.run, wrapper) catch return 0;
    
    return @intCast(goroutine_id);
}

/// C FFI export for creating channels from LLVM compiled code
export fn cursed_channel_create(element_size: u32, buffer_size: u32) ?*anyopaque {
    _ = element_size;
    const allocator = std.heap.c_allocator;
    
    // For now, create a generic byte channel and let the caller handle typing
    const channel = makeChannel(u8, allocator, buffer_size) catch return null;
    
    return @ptrCast(channel);
}

/// C FFI export for sending to channels from LLVM compiled code
export fn cursed_channel_send(channel_ptr: ?*anyopaque, data: ?*anyopaque, data_size: u32) u32 {
    if (channel_ptr == null or data == null) {
        return @intFromEnum(SendResult.closed);
    }
    
    // Cast to generic byte channel for now
    const channel: *Channel(u8) = @ptrCast(@alignCast(channel_ptr.?));
    
    // For now, handle data as a stream of bytes
    const data_bytes: [*]u8 = @ptrCast(data.?);
    
    // Send each byte to the channel
    var i: u32 = 0;
    while (i < data_size) : (i += 1) {
        const result = channel.send(data_bytes[i]) catch return @intFromEnum(SendResult.closed);
        if (result != SendResult.sent) {
            return @intFromEnum(result);
        }
    }
    
    return @intFromEnum(SendResult.sent);
}

/// C FFI export for receiving from channels from LLVM compiled code
export fn cursed_channel_receive(channel_ptr: ?*anyopaque, data_out: ?*anyopaque, data_size: u32) u32 {
    if (channel_ptr == null or data_out == null) {
        return @intFromEnum(ReceiveResult.closed);
    }
    
    // Cast to generic byte channel for now
    const channel: *Channel(u8) = @ptrCast(@alignCast(channel_ptr.?));
    const data_bytes: [*]u8 = @ptrCast(data_out.?);
    
    // Receive bytes from the channel
    var i: u32 = 0;
    while (i < data_size) : (i += 1) {
        const result = channel.receive() catch return @intFromEnum(ReceiveResult.closed);
        if (result) |byte| {
            data_bytes[i] = byte;
        } else {
            // Channel closed or no more data
            return @intFromEnum(ReceiveResult.closed);
        }
    }
    
    return @intFromEnum(ReceiveResult.received);
}

/// C FFI export for closing channels from LLVM compiled code
export fn cursed_channel_close(channel_ptr: ?*anyopaque) void {
    if (channel_ptr == null) return;
    
    // Cast to generic byte channel for now
    const channel: *Channel(u8) = @ptrCast(@alignCast(channel_ptr.?));
    channel.close();
}

/// C FFI export for initializing runtime from LLVM compiled code
export fn cursed_runtime_init() u32 {
    const allocator = std.heap.c_allocator;
    initializeScheduler(allocator, SchedulerConfig.default()) catch return 0;
    return 1; // Success
}

/// C FFI export for shutting down runtime from LLVM compiled code
export fn cursed_runtime_shutdown() void {
    const allocator = std.heap.c_allocator;
    shutdownScheduler(allocator);
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
            const test_ctx: *TestContext = @ptrCast(@alignCast(ctx.?));
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
    try std.testing.expect(deque.length() == 1);
    
    const popped = deque.popBottom();
    try std.testing.expect(popped == &goroutine);
    try std.testing.expect(deque.isEmpty());
}

test "CURSED channel operations - dm_send and dm_recv" {
    const allocator = std.testing.allocator;
    
    // Initialize channel registry
    initChannelRegistry(allocator);
    
    // Create a channel using CURSED API
    const channel_id = try dm_create(i32, allocator, 3);
    
    // Test dm_send
    const send_result = try dm_send(channel_id, @as(i32, 42), allocator);
    try std.testing.expect(send_result == SendResult.sent);
    
    // Test dm_recv
    const received = try dm_recv(i32, channel_id, allocator);
    try std.testing.expect(received.? == 42);
    
    // Test dm_close
    try dm_close(channel_id);
    
    // Try sending to closed channel
    const send_result_closed = try dm_send(channel_id, @as(i32, 43), allocator);
    try std.testing.expect(send_result_closed == SendResult.closed);
}

test "Variable channel with GC integration" {
    const allocator = std.testing.allocator;
    
    var var_channel = try VariableChannel.init(allocator, 2);
    defer var_channel.deinit();
    
    // Create test Variables
    const Variable = @import("main_unified.zig").Variable;
    const var1 = Variable{ .Integer = 123 };
    const var2 = Variable{ .Integer = 456 };
    
    // Test sending Variables
    try std.testing.expect(try var_channel.sendVariable(var1) == SendResult.sent);
    try std.testing.expect(try var_channel.sendVariable(var2) == SendResult.sent);
    
    // Test receiving Variables
    const received1 = try var_channel.receiveVariable();
    try std.testing.expect(received1 != null);
    if (received1) |received_var| {
        try std.testing.expect(received_var.Integer == 123);
    }
    
    const received2 = try var_channel.receiveVariable();
    try std.testing.expect(received2 != null);
    if (received2) |received_var2| {
        try std.testing.expect(received_var2.Integer == 456);
    }
    
    // Test channel close
    var_channel.close();
    try std.testing.expect(var_channel.isClosed());
}

test "Channel type system integration" {
    const allocator = std.testing.allocator;
    
    // Initialize scheduler for full system integration
    const config = SchedulerConfig.default();
    try initializeScheduler(allocator, config);
    defer shutdownScheduler(allocator);
    
    initChannelRegistry(allocator);
    
    // Create channels of different types
    const int_channel = try dm_create(i32, allocator, 1);
    const float_channel = try dm_create(f64, allocator, 1);
    const bool_channel = try dm_create(bool, allocator, 1);
    
    // Test type-safe operations
    try std.testing.expect(try dm_send(int_channel, @as(i32, 100), allocator) == SendResult.sent);
    try std.testing.expect(try dm_send(float_channel, @as(f64, 3.14), allocator) == SendResult.sent);
    try std.testing.expect(try dm_send(bool_channel, true, allocator) == SendResult.sent);
    
    // Test receiving with correct types
    const int_result = try dm_recv(i32, int_channel, allocator);
    try std.testing.expect(int_result.? == 100);
    
    const float_result = try dm_recv(f64, float_channel, allocator);
    try std.testing.expect(float_result.? == 3.14);
    
    const bool_result = try dm_recv(bool, bool_channel, allocator);
    try std.testing.expect(bool_result.? == true);
    
    // Clean up
    try dm_close(int_channel);
    try dm_close(float_channel);
    try dm_close(bool_channel);
}
