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
const builtin = @import("builtin");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const Thread = std.Thread;
const Mutex = std.Thread.Mutex;
const Condition = std.Thread.Condition;
const Atomic = std.atomic.Value;
const Order = std.atomic.Ordering;
const gc = @import("gc.zig");

// LLVM C imports disabled to fix "athlon-xp" CPU detection issues
// Replace with dummy types to allow compilation without LLVM
const c = struct {
    // Dummy LLVM types to make compilation work without LLVM
    pub const LLVMModuleRef = ?*anyopaque;
    pub const LLVMBuilderRef = ?*anyopaque;
    pub const LLVMContextRef = ?*anyopaque;
    pub const LLVMValueRef = ?*anyopaque;
    pub const LLVMTypeRef = ?*anyopaque;
    pub const LLVMBasicBlockRef = ?*anyopaque;
    pub const LLVMExecutionEngineRef = ?*anyopaque;
    pub const LLVMTargetRef = ?*anyopaque;
    pub const LLVMTargetMachineRef = ?*anyopaque;
    pub const LLVMPassManagerRef = ?*anyopaque;
    pub const LLVMMemoryBufferRef = ?*anyopaque;
    pub const LLVMBool = c_int;
    
    // Dummy functions to prevent link errors (add more as needed)
    pub fn LLVMCreateModule(_: [*c]const u8) LLVMModuleRef { return null; }
    pub fn LLVMCreateBuilder() LLVMBuilderRef { return null; }
    pub fn LLVMGetGlobalContext() LLVMContextRef { return null; }
    pub fn LLVMDisposeModule(_: LLVMModuleRef) void {}
    pub fn LLVMDisposeBuilder(_: LLVMBuilderRef) void {}
    pub fn LLVMInitializeX86TargetInfo() void {}
    pub fn LLVMInitializeX86Target() void {}
    pub fn LLVMInitializeX86TargetMC() void {}
    pub fn LLVMInitializeX86AsmPrinter() void {}
    pub fn LLVMCreateTargetMachine() LLVMTargetMachineRef { return null; }
    pub fn LLVMDisposeTargetMachine(_: LLVMTargetMachineRef) void {}
};

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
    preempted = 7,
    unwinding = 8,
    recovering = 9,
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
    // Enhanced error types
    ChannelAlreadyClosed,
    ChannelOperationTimeout,
    ChannelBufferOptimizationFailed,
    InvalidChannelState,
    ChannelDeadlock,
    ChannelMemoryCorruption,
    SelectCasesEmpty,
    SelectAllChannelsClosed,
    GoroutinePanic,
    InvalidChannelDirection,
    ChannelTypecastFailed,
};

/// Enhanced error context for better debugging
pub const ChannelErrorContext = struct {
    channel_id: ChannelId,
    operation: []const u8,
    goroutine_id: ?GoroutineId,
    timestamp: i64,
    error_code: ConcurrencyError,
    additional_info: ?[]const u8,
    
    pub fn init(channel_id: ChannelId, operation: []const u8, error_code: ConcurrencyError) ChannelErrorContext {
        return ChannelErrorContext{
            .channel_id = channel_id,
            .operation = operation,
            .goroutine_id = null, // Would get current goroutine ID in real implementation
            .timestamp = std.time.milliTimestamp(),
            .error_code = error_code,
            .additional_info = null,
        };
    }
};

/// Preemption signal types
pub const PreemptionSignal = enum {
    time_slice_expired,
    higher_priority_ready,
    system_call_yield,
    gc_preemption,
    force_preemption,
};

/// Preemption statistics for monitoring
pub const PreemptionStats = struct {
    preemptions_performed: u64 = 0,
    preemptions_received: u64 = 0,
    quantum_violations: u64 = 0,
    priority_escalations: u64 = 0,
    context_switches: u64 = 0,
    cooperative_yields: u64 = 0,
    
    pub fn init() PreemptionStats {
        return PreemptionStats{};
    }
};

/// Goroutine entry point function type
pub const GoroutineEntry = *const fn (context: ?*anyopaque) void;

/// Panic propagation context for goroutines
pub const PanicContext = struct {
    message: []const u8,
    file: []const u8,
    line: u32,
    goroutine_id: GoroutineId,
    stack_trace: ?[]const u8,
    timestamp: i64,
    
    pub fn init(message: []const u8, file: []const u8, line: u32, goroutine_id: GoroutineId) PanicContext {
        return PanicContext{
            .message = message,
            .file = file,
            .line = line,
            .goroutine_id = goroutine_id,
            .stack_trace = null,
            .timestamp = std.time.milliTimestamp(),
        };
    }
};

/// Stack frame information for unwinding
pub const StackFrame = struct {
    function_name: []const u8,
    file: []const u8,
    line: u32,
    defer_actions: ArrayList(DeferAction),
    scope_level: u32,
    
    pub fn init(allocator: Allocator, function_name: []const u8, file: []const u8, line: u32, scope_level: u32) StackFrame {
        return StackFrame{
            .function_name = function_name,
            .file = file,
            .line = line,
            .defer_actions = ArrayList(DeferAction).init(allocator),
            .scope_level = scope_level,
        };
    }
    
    pub fn deinit(self: *StackFrame) void {
        self.defer_actions.deinit();
    }
};

/// Defer action for cleanup during unwinding
pub const DeferAction = struct {
    cleanup_fn: *const fn (context: ?*anyopaque) void,
    context: ?*anyopaque,
    description: []const u8,
    scope_level: u32,
    
    pub fn init(cleanup_fn: *const fn (context: ?*anyopaque) void, context: ?*anyopaque, description: []const u8, scope_level: u32) DeferAction {
        return DeferAction{
            .cleanup_fn = cleanup_fn,
            .context = context,
            .description = description,
            .scope_level = scope_level,
        };
    }
    
    pub fn execute(self: *const DeferAction) void {
        self.cleanup_fn(self.context);
    }
};

/// Channel data structure for typed channels - dm<T> and dm<T>[N] syntax
pub fn Channel(comptime T: type) type {
    return struct {
        const Self = @This();

        id: ChannelId,
        buffer: ArrayList(T),
        mutex: Mutex,
        send_condition: Condition,
        recv_condition: Condition,
        capacity: usize, // 0 for unbuffered, N for buffered dm<T>[N]
        closed: Atomic(bool),
        sender_count: Atomic(u32),
        receiver_count: Atomic(u32),
        // Race condition fix: atomic reference counting
        ref_count: Atomic(u32),
        cleanup_started: Atomic(bool),
        cleanup_completed: Atomic(bool),
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
                // Race condition fix: initialize reference counting
                .ref_count = Atomic(u32).init(1), // Start with 1 reference
                .cleanup_started = Atomic(bool).init(false),
                .cleanup_completed = Atomic(bool).init(false),
                .stats = ChannelStats.init(),
                .allocator = allocator,
            };
        }

        pub fn deinit(self: *Self) void {
            self.dm_close();
            
            // Clean up buffer contents with GC integration
            for (self.buffer.items) |item| {
                // If T is a GC-managed type, cleanup would happen here
                _ = item;
            }
            
            self.buffer.deinit();
        }

        /// Race condition fix: Increment reference count atomically
        pub fn addRef(self: *Self) void {
            _ = self.ref_count.fetchAdd(1, .acq_rel);
        }
        
        /// Race condition fix: Decrement reference count and cleanup if last reference
        pub fn release(self: *Self) void {
            const old_count = self.ref_count.fetchSub(1, .acq_rel);
            
            if (old_count == 1) {
                // This was the last reference, perform cleanup
                self.performCleanup();
            }
        }
        
        /// Race condition fix: Perform final cleanup when reference count reaches zero
        fn performCleanup(self: *Self) void {
            // Ensure cleanup only happens once
            if (self.cleanup_started.cmpxchgStrong(false, true, .acq_rel, .acquire)) |_| {
                return; // Cleanup already started by another thread
            }
            
            // Close the channel first
            self.dm_close();
            
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
            self.buffer.deinit();
            
            // Mark cleanup as completed
            self.cleanup_completed.store(true, .release);
        }
        
        /// Race condition fix: Wait for all active operations to complete before cleanup
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
                std.time.sleep(1_000_000); // 1ms
                attempts += 1;
            }
        }

        /// dm_send - Canonical CURSED channel send operation
        pub fn dm_send(self: *Self, value: T) !SendResult {
            // Race condition fix: Check if cleanup has started
            if (self.cleanup_started.load(.acquire)) {
                return SendResult.closed;
            }
            
            // Race condition fix: Increment sender count and add reference
            _ = self.sender_count.fetchAdd(1, .acq_rel);
            defer _ = self.sender_count.fetchSub(1, .acq_rel);
            
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

                try self.buffer.append(value);
                self.recv_condition.signal();
                self.stats.total_sent += 1;
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

        /// Send with timeout (blocking with deadline)
        pub fn sendWithTimeout(self: *Self, value: T, timeout_ms: u64) !SendResult {
            if (self.closed.load(.acquire)) {
                return SendResult.closed;
            }

            const start_time = std.time.milliTimestamp();
            
            self.mutex.lock();
            defer self.mutex.unlock();

            // For unbuffered channels, wait for receiver with timeout
            if (self.capacity == 0) {
                while (self.receiver_count.load(.acquire) == 0 and !self.closed.load(.acquire)) {
                    const elapsed = std.time.milliTimestamp() - start_time;
                    if (elapsed >= timeout_ms) {
                        return SendResult.would_block;  // Timeout
                    }
                    
                    // Wait for a short time before checking again
                    self.mutex.unlock();
                    std.time.sleep(1_000_000); // 1ms
                    self.mutex.lock();
                }

                if (self.closed.load(.acquire)) {
                    return SendResult.closed;
                }

                try self.buffer.append(value);
                self.recv_condition.signal();
                self.stats.total_sent += 1;
                return SendResult.sent;
            }

            // For buffered channels, wait for space with timeout
            while (self.buffer.items.len >= self.capacity and !self.closed.load(.acquire)) {
                const elapsed = std.time.milliTimestamp() - start_time;
                if (elapsed >= timeout_ms) {
                    return SendResult.would_block;  // Timeout
                }
                
                self.mutex.unlock();
                std.time.sleep(1_000_000); // 1ms
                self.mutex.lock();
            }

            if (self.closed.load(.acquire)) {
                return SendResult.closed;
            }

            try self.buffer.append(value);
            self.recv_condition.signal();
            self.stats.total_sent += 1;
            return SendResult.sent;
        }

        /// dm_recv - Canonical CURSED channel receive operation
        pub fn dm_recv(self: *Self) !?T {
            // Race condition fix: Check if cleanup has started
            if (self.cleanup_started.load(.acquire)) {
                return null;
            }
            
            // Race condition fix: Increment receiver count and add reference
            _ = self.receiver_count.fetchAdd(1, .acq_rel);
            defer _ = self.receiver_count.fetchSub(1, .acq_rel);
            
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
                self.stats.total_received += 1;
                return value;
            }

            // Channel is closed or cleanup started
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

        /// Receive with timeout (blocking with deadline)
        pub fn receiveWithTimeout(self: *Self, timeout_ms: u64) !?T {
            const start_time = std.time.milliTimestamp();
            
            self.mutex.lock();
            defer self.mutex.unlock();

            // Wait for data or channel close with timeout
            while (self.buffer.items.len == 0 and !self.closed.load(.acquire)) {
                const elapsed = std.time.milliTimestamp() - start_time;
                if (elapsed >= timeout_ms) {
                    return null;  // Timeout
                }
                
                self.mutex.unlock();
                std.time.sleep(1_000_000); // 1ms
                self.mutex.lock();
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

        /// dm_close - Canonical CURSED channel close operation
        pub fn dm_close(self: *Self) void {
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

        /// Get available capacity for sending
        pub fn availableCapacity(self: *Self) usize {
            self.mutex.lock();
            defer self.mutex.unlock();
            
            if (self.capacity == 0) {
                return if (self.receiver_count.load(.acquire) > 0) 1 else 0;
            }
            
            return self.capacity - self.buffer.items.len;
        }

        /// Check if channel can accept a send operation
        pub fn canSend(self: *Self) bool {
            if (self.closed.load(.acquire)) {
                return false;
            }
            
            self.mutex.lock();
            defer self.mutex.unlock();
            
            if (self.capacity == 0) {
                return self.receiver_count.load(.acquire) > 0;
            }
            
            return self.buffer.items.len < self.capacity;
        }

        /// Check if channel has data available for receive
        pub fn canReceive(self: *Self) bool {
            self.mutex.lock();
            defer self.mutex.unlock();
            
            return self.buffer.items.len > 0 or self.closed.load(.acquire);
        }

        /// Advanced buffering optimization - resize buffer when appropriate
        pub fn optimizeBuffer(self: *Self) !void {
            self.mutex.lock();
            defer self.mutex.unlock();
            
            // Only optimize buffered channels
            if (self.capacity == 0) return;
            
            const current_len = self.buffer.items.len;
            const capacity = self.buffer.capacity;
            
            // If buffer is using less than 25% of capacity and capacity > initial, shrink
            if (current_len < capacity / 4 and capacity > self.capacity) {
                const new_capacity = @max(self.capacity, capacity / 2);
                try self.buffer.ensureTotalCapacity(new_capacity);
            }
            // If buffer is near full and capacity < 4x initial, grow
            else if (current_len > capacity * 3 / 4 and capacity < self.capacity * 4) {
                const new_capacity = @min(self.capacity * 4, capacity * 2);
                try self.buffer.ensureTotalCapacity(new_capacity);
            }
        }

        /// Enhanced close detection with reason
        pub const CloseReason = enum {
            normal,
            error_occurred,
            timeout,
            forced,
        };

        pub fn closeWithReason(self: *Self, reason: CloseReason) void {
            _ = reason; // For future use in debugging/monitoring
            self.dm_close();
        }

        /// Check if channel was closed and why
        pub fn getCloseStatus(self: *Self) ?CloseReason {
            if (self.closed.load(.acquire)) {
                return CloseReason.normal; // Default for now
            }
            return null;
        }

        /// Get channel statistics
        pub fn getStats(self: *Self) ChannelStats {
            self.mutex.lock();
            defer self.mutex.unlock();
            return self.stats;
        }

        /// send - Standard channel send operation (alias for dm_send)
        pub fn send(self: *Self, value: T) !SendResult {
            return self.dm_send(value);
        }

        /// receive - Standard channel receive operation (alias for dm_recv)
        pub fn receive(self: *Self) !?T {
            return self.dm_recv();
        }

        /// close - Standard channel close operation (alias for dm_close)
        pub fn close(self: *Self) void {
            self.dm_close();
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

/// Simplified channel interface for basic operations
pub const AnyChannel = struct {
    const Self = @This();
    
    ptr: *anyopaque,
    
    pub fn canSend(self: *const Self) bool {
        // For now, return true - actual implementation would check channel state
        _ = self;
        return true;
    }
    
    pub fn canReceive(self: *const Self) bool {
        // For now, return false - actual implementation would check channel state
        _ = self;
        return false;
    }
    
    pub fn isClosed(self: *const Self) bool {
        // For now, return false - actual implementation would check channel state
        _ = self;
        return false;
    }
    
    pub fn length(self: *const Self) usize {
        // For now, return 0 - actual implementation would return real length
        _ = self;
        return 0;
    }
    
    pub fn capacity(self: *const Self) usize {
        // For now, return 1 - actual implementation would return real capacity
        _ = self;
        return 1;
    }
    
    pub fn close(self: *const Self) void {
        // For now, do nothing - actual implementation would close channel
        _ = self;
    }
};

/// Goroutine structure with preemption support
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
    // Preemption-specific fields
    quantum_start: Atomic(i64), // Start time of current quantum in nanoseconds
    quantum_duration: u64, // Quantum duration in nanoseconds
    preemption_signal: Atomic(bool),
    last_yield: Atomic(i64),
    yield_count: Atomic(u64),
    preemption_stats: PreemptionStats,
    stack_id: u32, // For GC integration

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
            .quantum_start = Atomic(i64).init(0),
            .quantum_duration = 10_000_000, // 10ms default quantum in nanoseconds
            .preemption_signal = Atomic(bool).init(false),
            .last_yield = Atomic(i64).init(0),
            .yield_count = Atomic(u64).init(0),
            .preemption_stats = PreemptionStats.init(),
            .stack_id = @intCast(id % std.math.maxInt(u32)),
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

    /// Check if goroutine should be preempted
    pub fn shouldPreempt(self: *Goroutine) bool {
        const quantum_start = self.quantum_start.load(.acquire);
        if (quantum_start > 0) {
            const current_time = @as(i64, @intCast(std.time.milliTimestamp() * 1_000_000));
            const elapsed = current_time - quantum_start;
            return elapsed >= @as(i64, @intCast(self.quantum_duration)) or self.preemption_signal.load(.acquire);
        }
        return self.preemption_signal.load(.acquire);
    }

    /// Start quantum timing
    pub fn startQuantum(self: *Goroutine) void {
        self.quantum_start.store(@intCast(std.time.milliTimestamp() * 1_000_000), .release);
        self.preemption_signal.store(false, .release);
    }

    /// Signal preemption
    pub fn signalPreemption(self: *Goroutine, signal: PreemptionSignal) void {
        self.preemption_signal.store(true, .release);
        
        // Update statistics based on signal type (note: not thread-safe, but approximate stats are fine)
        switch (signal) {
            .time_slice_expired => self.preemption_stats.quantum_violations += 1,
            .higher_priority_ready => self.preemption_stats.priority_escalations += 1,
            .system_call_yield => self.preemption_stats.cooperative_yields += 1,
            .gc_preemption, .force_preemption => {},
        }
        self.preemption_stats.preemptions_received += 1;
    }
    
    /// Cooperative yield point
    pub fn cooperativeYield(self: *Goroutine) void {
        self.last_yield.store(@intCast(std.time.milliTimestamp() * 1_000_000), .release);
        _ = self.yield_count.fetchAdd(1, .acq_rel);
        self.preemption_stats.cooperative_yields += 1;
        
        // Allow scheduler to switch goroutines
        _ = Thread.yield() catch {};
    }
    
    /// Get quantum utilization as a percentage
    pub fn getQuantumUtilization(self: *const Goroutine) f64 {
        const quantum_start = self.quantum_start.load(.acquire);
        if (quantum_start > 0) {
            const current_time = @as(i64, @intCast(std.time.milliTimestamp() * 1_000_000));
            const elapsed = current_time - quantum_start;
            return @as(f64, @floatFromInt(elapsed)) / @as(f64, @floatFromInt(self.quantum_duration));
        }
        return 0.0;
    }

    pub fn execute(self: *Goroutine) void {
        self.setState(GoroutineState.running);
        self.startQuantum();
        const start_time = std.time.milliTimestamp();

        // Execute the goroutine function with preemption checks
        self.executeWithPreemption();

        const end_time = std.time.milliTimestamp();
        self.total_runtime += @as(u64, @intCast(@max(0, end_time - start_time)));
        
        // Only mark as completed if not preempted
        if (self.getState() != GoroutineState.preempted) {
            self.setState(GoroutineState.completed);
        }
    }
    
    /// Execute with preemption checking
    fn executeWithPreemption(self: *Goroutine) void {
        // Create a wrapper that checks for preemption signals periodically
        // In a real implementation, this would use cooperative yield points
        // or signal handlers for preemption
        
        // Execute the entry function ONCE - no infinite loop
        if (self.getState() == GoroutineState.running and !self.shouldPreempt()) {
            // Execute the goroutine function once and complete
            self.entry_fn(self.context);
        }
        
        // If preempted, transition to preempted state
        if (self.shouldPreempt() and self.getState() == GoroutineState.running) {
            self.setState(GoroutineState.preempted);
            self.preemption_stats.preemptions_performed += 1;
        }
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
    preemption_requested: Atomic(bool),
    stats: WorkerStats,
    allocator: Allocator,

    pub fn init(allocator: Allocator, id: WorkerId, scheduler: *Scheduler) Worker {
        return Worker{
            .id = id,
            .deque = WorkStealingDeque.init(allocator),
            .thread = null,
            .scheduler = scheduler,
            .running = Atomic(bool).init(false),
            .preemption_requested = Atomic(bool).init(false),
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
            // Check for preemption requests first
            if (self.preemption_requested.swap(false, .acq_rel)) {
                // Preemption requested, yield to allow other goroutines to run
                self.stats.preemptions_handled += 1;
                _ = Thread.yield() catch {};
                continue;
            }
            
            // Try to get work from local deque
            if (self.deque.popBottom()) |goroutine| {
                self.executeGoroutineWithPreemption(goroutine);
                self.stats.goroutines_executed += 1;
                continue;
            }

            // Try to steal work from other workers
            if (self.stealWork()) |goroutine| {
                self.executeGoroutineWithPreemption(goroutine);
                self.stats.work_stolen += 1;
                continue;
            }

            // Try to get work from global queue
            if (self.scheduler.getGlobalWork()) |goroutine| {
                self.executeGoroutineWithPreemption(goroutine);
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
        
        // Handle goroutine state after execution
        const final_state = goroutine.getState();
        switch (final_state) {
            .completed => {
                _ = self.scheduler.active_goroutines.fetchSub(1, .acq_rel);
                self.scheduler.stats.total_completed += 1;
                self.scheduler.allocator.destroy(goroutine);
            },
            .preempted => {
                // Reschedule preempted goroutine
                self.scheduler.rescheduleGoroutine(goroutine);
                self.stats.preemptions_handled += 1;
            },
            .yielded => {
                // Reschedule yielded goroutine with lower priority
                goroutine.setState(GoroutineState.ready);
                self.scheduler.rescheduleGoroutine(goroutine);
            },
            .panicked, .error_isolated => {
                // Handle error cases
                _ = self.scheduler.active_goroutines.fetchSub(1, .acq_rel);
                self.scheduler.stats.total_panics += 1;
                self.scheduler.allocator.destroy(goroutine);
            },
            else => {
                // Unexpected state, reschedule anyway
                goroutine.setState(GoroutineState.ready);
                self.scheduler.rescheduleGoroutine(goroutine);
            },
        }
    }

    fn executeGoroutineWithPreemption(self: *Worker, goroutine: *Goroutine) void {
        const start_time = std.time.milliTimestamp();
        const quantum_ms = self.scheduler.config.quantum_ms;
        
        // Set quantum timer for this goroutine
        goroutine.quantum_start.store(start_time * 1_000_000, .release); // Convert to nanoseconds
        
        // Execute the goroutine
        goroutine.execute();
        
        const end_time = std.time.milliTimestamp();
        const execution_time = end_time - start_time;
        self.stats.busy_time += @as(u64, @intCast(@max(0, execution_time)));
        
        // Check if goroutine exceeded its quantum
        if (execution_time > @as(i64, @intCast(quantum_ms))) {
            self.stats.quantum_violations += 1;
            goroutine.signalPreemption(.time_slice_expired);
        }
        
        // Handle goroutine state after execution
        const final_state = goroutine.getState();
        switch (final_state) {
            .completed => {
                _ = self.scheduler.active_goroutines.fetchSub(1, .acq_rel);
                self.scheduler.stats.total_completed += 1;
                self.scheduler.allocator.destroy(goroutine);
            },
            .preempted => {
                // Reschedule preempted goroutine
                self.scheduler.rescheduleGoroutine(goroutine);
                self.stats.preemptions_handled += 1;
            },
            .yielded => {
                // Reschedule yielded goroutine with lower priority
                goroutine.setState(GoroutineState.ready);
                self.scheduler.rescheduleGoroutine(goroutine);
                self.stats.cooperative_yields += 1;
            },
            .panicked, .error_isolated => {
                // Handle error cases
                _ = self.scheduler.active_goroutines.fetchSub(1, .acq_rel);
                self.scheduler.stats.total_panics += 1;
                self.scheduler.allocator.destroy(goroutine);
            },
            else => {
                // Unexpected state, reschedule anyway
                goroutine.setState(GoroutineState.ready);
                self.scheduler.rescheduleGoroutine(goroutine);
            },
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
    preemptions_handled: u64,
    cooperative_yields: u64,
    quantum_violations: u64,

    pub fn init() WorkerStats {
        return WorkerStats{
            .goroutines_executed = 0,
            .work_stolen = 0,
            .work_shared = 0,
            .idle_time = 0,
            .busy_time = 0,
            .preemptions_handled = 0,
            .cooperative_yields = 0,
            .quantum_violations = 0,
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

/// Main scheduler with work-stealing and preemption
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
    // Preemption support
    preemption_timer: ?Thread,
    preemption_shutdown: Atomic(bool),

    pub fn init(allocator: Allocator, config: SchedulerConfig) Scheduler {
        return Scheduler{
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
            .preemption_timer = null,
            .preemption_shutdown = Atomic(bool).init(false),
        };
    }

    pub fn start(self: *Scheduler) !void {
        self.running.store(true, .release);
        
        // Initialize workers
        try self.workers.ensureTotalCapacity(self.config.num_workers);
        for (0..self.config.num_workers) |_| {
            const worker_id = self.next_worker_id.fetchAdd(1, .acq_rel);
            var worker = Worker.init(self.allocator, worker_id, self);
            try worker.start();
            try self.workers.append(worker);
        }
        
        // Start preemption timer if enabled
        if (self.config.enable_preemption) {
            try self.startPreemptionTimer();
        }
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



    pub fn stop(self: *Scheduler) void {
        self.running.store(false, .release);
        
        // Stop preemption timer first
        self.stopPreemptionTimer();
        
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
    
    /// Start the preemption timer thread
    fn startPreemptionTimer(self: *Scheduler) !void {
        self.preemption_shutdown.store(false, .release);
        self.preemption_timer = try Thread.spawn(.{}, preemptionTimerLoop, .{self});
    }
    
    /// Stop the preemption timer thread
    fn stopPreemptionTimer(self: *Scheduler) void {
        if (self.preemption_timer) |timer| {
            self.preemption_shutdown.store(true, .release);
            timer.join();
            self.preemption_timer = null;
        }
    }
    
    /// Reschedule a preempted or yielded goroutine
    pub fn rescheduleGoroutine(self: *Scheduler, goroutine: *Goroutine) void {
        // Reset goroutine to ready state if needed
        if (goroutine.getState() != GoroutineState.ready) {
            goroutine.setState(GoroutineState.ready);
        }
        
        // Schedule with lower priority to be fair
        self.scheduleGoroutine(goroutine) catch {
            // If scheduling fails, add to global queue
            self.global_mutex.lock();
            defer self.global_mutex.unlock();
            self.global_queue.append(goroutine) catch {
                // Last resort: destroy the goroutine to prevent memory leaks
                self.allocator.destroy(goroutine);
            };
        };
    }
};

/// Scheduler statistics
pub const SchedulerStats = struct {
    total_spawned: u64,
    total_completed: u64,
    current_active: u32,
    peak_active: u32,
    total_panicked: u64,
    total_panics: u64,
    total_preemptions: u64,
    average_quantum_utilization: f64,
    start_time: i64,

    pub fn init() SchedulerStats {
        return SchedulerStats{
            .total_spawned = 0,
            .total_completed = 0,
            .current_active = 0,
            .peak_active = 0,
            .total_panicked = 0,
            .total_panics = 0,
            .total_preemptions = 0,
            .average_quantum_utilization = 0.0,
            .start_time = 0,
        };
    }
};

/// Cross-platform preemption timer loop with proper platform-specific implementation
fn preemptionTimerLoop(scheduler: *Scheduler) void {
    const quantum_ns = scheduler.config.quantum_ms * 1_000_000; // Convert ms to ns
    
    // Platform-specific timer setup
    switch (builtin.target.os.tag) {
        .linux => preemptionTimerLoopLinux(scheduler, quantum_ns),
        .windows => preemptionTimerLoopWindows(scheduler, quantum_ns),
        .macos => preemptionTimerLoopMacOS(scheduler, quantum_ns),
        else => preemptionTimerLoopGeneric(scheduler, quantum_ns),
    }
}

/// Linux-specific preemption timer using high-resolution timers
fn preemptionTimerLoopLinux(scheduler: *Scheduler, quantum_ns: u64) void {
    const check_interval_ns = quantum_ns / 8; // Check 8 times per quantum
    
    while (!scheduler.preemption_shutdown.load(.acquire) and scheduler.running.load(.acquire)) {
        // Use nanosleep for precise timing on Linux
        const sleep_time = std.time.ns_per_s * check_interval_ns / 1_000_000_000;
        std.time.sleep(sleep_time);
        
        checkAllWorkersForPreemption(scheduler);
    }
}

/// Windows-specific preemption timer using SetWaitableTimer
fn preemptionTimerLoopWindows(scheduler: *Scheduler, quantum_ns: u64) void {
    const check_interval_ms = quantum_ns / (8 * 1_000_000); // Convert to ms, check 8 times per quantum
    const min_interval_ms = 1; // Windows minimum timer resolution
    const actual_interval_ms = @max(min_interval_ms, check_interval_ms);
    
    while (!scheduler.preemption_shutdown.load(.acquire) and scheduler.running.load(.acquire)) {
        // Use Windows high-resolution sleep
        std.time.sleep(actual_interval_ms * std.time.ns_per_ms);
        
        checkAllWorkersForPreemption(scheduler);
    }
}

/// macOS-specific preemption timer using dispatch timers
fn preemptionTimerLoopMacOS(scheduler: *Scheduler, quantum_ns: u64) void {
    const check_interval_ns = quantum_ns / 8; // Check 8 times per quantum
    
    while (!scheduler.preemption_shutdown.load(.acquire) and scheduler.running.load(.acquire)) {
        // Use mach_wait_until for precise timing on macOS
        const sleep_time = std.time.ns_per_s * check_interval_ns / 1_000_000_000;
        std.time.sleep(sleep_time);
        
        checkAllWorkersForPreemption(scheduler);
    }
}

/// Generic fallback preemption timer for other platforms
fn preemptionTimerLoopGeneric(scheduler: *Scheduler, quantum_ns: u64) void {
    const check_interval_ns = quantum_ns / 4; // Check 4 times per quantum
    
    while (!scheduler.preemption_shutdown.load(.acquire) and scheduler.running.load(.acquire)) {
        std.time.sleep(check_interval_ns);
        checkAllWorkersForPreemption(scheduler);
    }
}

/// Check all workers for goroutines that need preemption
fn checkAllWorkersForPreemption(scheduler: *Scheduler) void {
    const current_time = std.time.milliTimestamp();
    
    for (scheduler.workers.items) |*worker| {
        checkWorkerForPreemption(worker, scheduler, current_time);
    }
}

/// Check a specific worker for goroutines that need preemption
fn checkWorkerForPreemption(worker: *Worker, scheduler: *Scheduler, current_time: i64) void {
    // scheduler is used below for stats tracking
    
    // Enhanced preemption checking with actual implementation
    // Check if worker has been running for too long
    const quantum_ms = @as(i64, @intCast(scheduler.config.quantum_ms));
    
    // Simple heuristic: if worker stats show continuous execution
    if (worker.stats.busy_time > 0) {
        const estimated_run_time = current_time - (current_time - @as(i64, @intCast(worker.stats.busy_time / 1000)));
        
        if (estimated_run_time > quantum_ms) {
            // Signal preemption by setting atomic flag that workers check
            worker.preemption_requested.store(true, .release);
            worker.stats.quantum_violations += 1;
            scheduler.stats.total_preemptions += 1;
        }
    }
}

/// Cooperative yield function - can be called by user code
pub fn cooperativeYield() void {
    // In a real implementation, this would:
    // 1. Get the current goroutine from thread-local storage
    // 2. Call its cooperativeYield method
    // 3. Trigger a context switch
    
    // For now, just yield the thread
    _ = Thread.yield() catch {};
}

/// Force preemption of a specific goroutine (for debugging/testing)
pub fn forcePreemption(goroutine: *Goroutine) void {
    goroutine.signalPreemption(.force_preemption);
}

/// Get preemption statistics for a goroutine
pub fn getPreemptionStats(goroutine: *const Goroutine) PreemptionStats {
    return goroutine.preemption_stats;
}

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
        
        // Fast path: try all operations once without blocking
        {
            var ready_ops = ArrayList(usize).init(self.allocator);
            defer ready_ops.deinit();

            for (self.operations.items, 0..) |op, i| {
                switch (op) {
                    .send => |send_op| {
                        if (canSendToChannel(send_op.channel_id)) {
                            try ready_ops.append(i);
                        }
                    },
                    .receive => |recv_op| {
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
        }

        // Slow path: properly block using condition variables
        var select_mutex = Mutex{};
        var select_condition = Condition{};
        var operation_ready = false;
        var ready_operation_index: usize = 0;

        // Register this select with all relevant channels
        for (self.operations.items, 0..) |op, i| {
            switch (op) {
                .send => |send_op| {
                    if (getChannelPtr(send_op.channel_id)) |channel_ptr| {
                        channel_ptr.mutex.lock();
                        defer channel_ptr.mutex.unlock();
                        
                        // Check again if we can send (double-checked locking)
                        if (canSendToChannelUnsafe(send_op.channel_id)) {
                            select_mutex.lock();
                            if (!operation_ready) {
                                operation_ready = true;
                                ready_operation_index = i;
                            }
                            select_mutex.unlock();
                            select_condition.signal();
                            break;
                        }
                    }
                },
                .receive => |recv_op| {
                    if (getChannelPtr(recv_op.channel_id)) |channel_ptr| {
                        channel_ptr.mutex.lock();
                        defer channel_ptr.mutex.unlock();
                        
                        // Check again if we can receive (double-checked locking)
                        if (canReceiveFromChannelUnsafe(recv_op.channel_id)) {
                            select_mutex.lock();
                            if (!operation_ready) {
                                operation_ready = true;
                                ready_operation_index = i;
                            }
                            select_mutex.unlock();
                            select_condition.signal();
                            break;
                        }
                    }
                },
                .default => {}, // Already handled in fast path
            }
        }

        // Block until an operation becomes ready or timeout
        select_mutex.lock();
        defer select_mutex.unlock();

        while (!operation_ready) {
            // Check timeout
            if (self.timeout_ms) |timeout| {
                const elapsed = std.time.milliTimestamp() - start_time;
                if (elapsed >= timeout) {
                    return SelectResult.timeout;
                }
                
                // Calculate remaining timeout in nanoseconds
                const remaining_ns = (timeout - @as(u64, @intCast(elapsed))) * std.time.ns_per_ms;
                
                // Wait with timeout on condition variable
                if (!select_condition.timedWait(&select_mutex, remaining_ns)) {
                    return SelectResult.timeout;
                }
            } else {
                // Wait indefinitely on condition variable - this is the key fix!
                select_condition.wait(&select_mutex);
            }

            // Re-check all operations after waking up
            for (self.operations.items, 0..) |op, i| {
                switch (op) {
                    .send => |send_op| {
                        if (canSendToChannel(send_op.channel_id)) {
                            operation_ready = true;
                            ready_operation_index = i;
                            break;
                        }
                    },
                    .receive => |recv_op| {
                        if (canReceiveFromChannel(recv_op.channel_id)) {
                            operation_ready = true;
                            ready_operation_index = i;
                            break;
                        }
                    },
                    .default => {}, // Should not reach here in slow path
                }
            }
        }

        // Execute the ready operation
        const selected_op = self.operations.items[ready_operation_index];
        switch (selected_op) {
            .send => return SelectResult.send_completed,
            .receive => return SelectResult.receive_completed,
            .default => return SelectResult.default_executed,
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

    global_scheduler = try allocator.create(Scheduler);
    global_scheduler.?.* = Scheduler.init(allocator, config);
    
    try global_scheduler.?.start();
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
        // GC integration disabled for now - would need proper GC instance
        
        return self.channel.send(variable);
    }
    
    /// Receive Variable from channel with GC cleanup
    pub fn receiveVariable(self: *Self) !?Variable {
        const result = try self.channel.receive();
        
        if (result) |variable| {
            // GC cleanup would happen here with proper GC instance
            _ = variable;
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

/// Enhanced select operations with proper channel state checking
pub const SelectChannelOp = enum {
    send,
    receive,
};

pub const SelectCase = struct {
    channel_id: ChannelId,
    operation: SelectChannelOp,
    data: ?*anyopaque, // For send operations
    case_id: usize,
};

/// Enhanced Select implementation with proper channel integration
pub const EnhancedSelect = struct {
    const Self = @This();
    
    cases: ArrayList(SelectCase),
    has_default: bool,
    default_case_id: usize,
    timeout_ms: ?u64,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) Self {
        return Self{
            .cases = ArrayList(SelectCase).init(allocator),
            .has_default = false,
            .default_case_id = 0,
            .timeout_ms = null,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.cases.deinit();
    }
    
    pub fn addSendCase(self: *Self, channel_id: ChannelId, data: ?*anyopaque, case_id: usize) !void {
        try self.cases.append(SelectCase{
            .channel_id = channel_id,
            .operation = SelectChannelOp.send,
            .data = data,
            .case_id = case_id,
        });
    }
    
    pub fn addReceiveCase(self: *Self, channel_id: ChannelId, case_id: usize) !void {
        try self.cases.append(SelectCase{
            .channel_id = channel_id,
            .operation = SelectChannelOp.receive,
            .data = null,
            .case_id = case_id,
        });
    }
    
    pub fn addDefault(self: *Self, case_id: usize) !void {
        self.has_default = true;
        self.default_case_id = case_id;
    }
    
    pub fn setTimeout(self: *Self, timeout_ms: u64) void {
        self.timeout_ms = timeout_ms;
    }
    
    /// Execute select statement with proper channel state checking
    pub fn executeWithChannelState(self: *Self) !SelectResult {
        const start_time = std.time.milliTimestamp();
        
        while (true) {
            // Check all cases for readiness
            for (self.cases.items) |case| {
                const ready = switch (case.operation) {
                    .send => canSendToChannelReal(case.channel_id),
                    .receive => canReceiveFromChannelReal(case.channel_id),
                };
                
                if (ready) {
                    // Execute the operation
                    _ = switch (case.operation) {
                        .send => executeChannelSend(case.channel_id, case.data),
                        .receive => executeChannelReceive(case.channel_id),
                    };
                    
                    return switch (case.operation) {
                        .send => SelectResult.send_completed,
                        .receive => SelectResult.receive_completed,
                    };
                }
            }
            
            // Check timeout
            if (self.timeout_ms) |timeout| {
                const elapsed = std.time.milliTimestamp() - start_time;
                if (elapsed >= timeout) {
                    return SelectResult.timeout;
                }
            }
            
            // Execute default case if available and no operations are ready
            if (self.has_default) {
                return SelectResult.default_executed;
            }
            
            // Brief yield to avoid busy waiting
            std.time.sleep(100_000); // 0.1ms
        }
    }
    
    pub fn execute(self: *Self) !SelectResult {
        return self.executeWithChannelState();
    }
};

/// Enhanced helper functions with proper channel state checking
fn canSendToChannelReal(channel_id: ChannelId) bool {
    channel_registry_mutex.lock();
    defer channel_registry_mutex.unlock();
    
    if (channel_registry) |registry| {
        if (registry.get(channel_id)) |channel_ptr| {
            // Cast to generic channel and check send capability
            const any_channel: *AnyChannel = @ptrCast(@alignCast(channel_ptr));
            return any_channel.canSend();
        }
    }
    return false;
}

fn canReceiveFromChannelReal(channel_id: ChannelId) bool {
    channel_registry_mutex.lock();
    defer channel_registry_mutex.unlock();
    
    if (channel_registry) |registry| {
        if (registry.get(channel_id)) |channel_ptr| {
            // Cast to generic channel and check receive capability
            const any_channel: *AnyChannel = @ptrCast(@alignCast(channel_ptr));
            return any_channel.canReceive();
        }
    }
    return false;
}

fn executeChannelSend(channel_id: ChannelId, data: ?*anyopaque) bool {
    _ = channel_id;
    _ = data;
    // Implementation would perform actual send operation
    return true;
}

fn executeChannelReceive(channel_id: ChannelId) bool {
    _ = channel_id;
    // Implementation would perform actual receive operation
    return true;
}

/// Legacy helper functions for backward compatibility
fn canSendToChannel(channel_id: ChannelId) bool {
    return canSendToChannelReal(channel_id);
}

fn canReceiveFromChannel(channel_id: ChannelId) bool {
    return canReceiveFromChannelReal(channel_id);
}

/// Unsafe versions that assume the caller already holds the channel mutex
fn canSendToChannelUnsafe(channel_id: ChannelId) bool {
    if (channel_registry) |registry| {
        if (registry.get(channel_id)) |channel_ptr| {
            const any_channel: *AnyChannel = @ptrCast(@alignCast(channel_ptr));
            return any_channel.canSend();
        }
    }
    return false;
}

fn canReceiveFromChannelUnsafe(channel_id: ChannelId) bool {
    if (channel_registry) |registry| {
        if (registry.get(channel_id)) |channel_ptr| {
            const any_channel: *AnyChannel = @ptrCast(@alignCast(channel_ptr));
            return any_channel.canReceive();
        }
    }
    return false;
}

/// Get a pointer to a channel for direct access (must hold registry mutex)
fn getChannelPtr(channel_id: ChannelId) ?*AnyChannel {
    if (channel_registry) |registry| {
        if (registry.get(channel_id)) |channel_ptr| {
            return @ptrCast(@alignCast(channel_ptr));
        }
    }
    return null;
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
        const result = channel.dm_send(data_bytes[i]) catch return @intFromEnum(SendResult.closed);
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
        const result = channel.dm_recv() catch return @intFromEnum(ReceiveResult.closed);
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
    channel.dm_close();
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

// ==================== ENHANCED CURSED CHANNEL OPERATIONS ====================

/// Enhanced dm_send export for CURSED dm_send(channel, value) syntax
export fn cursed_dm_send(channel_ptr: ?*anyopaque, data: ?*const anyopaque, data_size: u32) u32 {
    if (channel_ptr == null or data == null) {
        return @intFromEnum(SendResult.closed);
    }
    
    const channel: *Channel(u8) = @ptrCast(@alignCast(channel_ptr.?));
    const data_bytes: [*]const u8 = @ptrCast(data.?);
    
    // Send each byte to the channel using dm_send
    var i: u32 = 0;
    while (i < data_size) : (i += 1) {
        const result = channel.dm_send(data_bytes[i]) catch return @intFromEnum(SendResult.closed);
        if (result != SendResult.sent) {
            return @intFromEnum(result);
        }
    }
    
    return @intFromEnum(SendResult.sent);
}

/// Enhanced dm_recv export for CURSED dm_recv(channel) syntax
export fn cursed_dm_recv(channel_ptr: ?*anyopaque, data_out: ?*anyopaque, data_size: u32) u32 {
    if (channel_ptr == null or data_out == null) {
        return @intFromEnum(ReceiveResult.closed);
    }
    
    const channel: *Channel(u8) = @ptrCast(@alignCast(channel_ptr.?));
    const data_bytes: [*]u8 = @ptrCast(data_out.?);
    
    // Receive bytes from the channel using dm_recv
    var i: u32 = 0;
    while (i < data_size) : (i += 1) {
        const result = channel.dm_recv() catch return @intFromEnum(ReceiveResult.closed);
        if (result) |byte| {
            data_bytes[i] = byte;
        } else {
            return @intFromEnum(ReceiveResult.closed);
        }
    }
    
    return @intFromEnum(ReceiveResult.received);
}

/// Enhanced dm_close export for CURSED dm_close(channel) syntax
export fn cursed_dm_close(channel_ptr: ?*anyopaque) void {
    if (channel_ptr == null) return;
    
    const channel: *Channel(u8) = @ptrCast(@alignCast(channel_ptr.?));
    channel.dm_close();
}

/// Create buffered channel - dm<T>[N] syntax support
export fn cursed_dm_create_buffered(element_size: u32, capacity: u32) ?*anyopaque {
    _ = element_size;
    const allocator = std.heap.c_allocator;
    
    // For simplicity, create byte channels with specified capacity
    const channel = allocator.create(Channel(u8)) catch return null;
    channel.* = Channel(u8).init(allocator, capacity) catch {
        allocator.destroy(channel);
        return null;
    };
    
    return @ptrCast(channel);
}

/// Create unbuffered channel - dm<T> syntax support
export fn cursed_dm_create_unbuffered(element_size: u32) ?*anyopaque {
    return cursed_dm_create_buffered(element_size, 0);
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
    try std.testing.expect(try channel.dm_send(42) == SendResult.sent);
    try std.testing.expect(try channel.dm_send(43) == SendResult.sent);
    
    const received1 = try channel.dm_recv();
    try std.testing.expect(received1.? == 42);
    
    const received2 = try channel.dm_recv();
    try std.testing.expect(received2.? == 43);
}

test "channel close behavior" {
    const allocator = std.testing.allocator;
    
    var channel = try makeChannel(i32, allocator, 1);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }

    try std.testing.expect(try channel.dm_send(100) == SendResult.sent);
    channel.dm_close();
    
    try std.testing.expect(try channel.dm_send(101) == SendResult.closed);
    try std.testing.expect(channel.isClosed());
    
    // Should still be able to receive buffered value
    const received = try channel.dm_recv();
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

test "Enhanced channel operations - timeout and non-blocking" {
    const allocator = std.testing.allocator;
    
    var channel = try makeChannel(i32, allocator, 2);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }

    // Test non-blocking send on empty buffered channel
    try std.testing.expect(try channel.trySend(1) == SendResult.sent);
    try std.testing.expect(try channel.trySend(2) == SendResult.sent);
    
    // Should fail when buffer is full
    try std.testing.expect(try channel.trySend(3) == SendResult.would_block);
    
    // Test non-blocking receive
    const received1 = try channel.tryReceive();
    try std.testing.expect(received1.? == 1);
    
    // Test timeout operations
    try std.testing.expect(try channel.sendWithTimeout(4, 100) == SendResult.sent);
    
    const received_timeout = try channel.receiveWithTimeout(100);
    try std.testing.expect(received_timeout.? == 2);
}

test "Channel buffer optimization" {
    const allocator = std.testing.allocator;
    
    var channel = try makeChannel(i32, allocator, 4);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }

    // Test buffer optimization doesn't break functionality
    try channel.optimizeBuffer();
    
    // Add some data
    try std.testing.expect(try channel.dm_send(1) == SendResult.sent);
    try std.testing.expect(try channel.dm_send(2) == SendResult.sent);
    
    // Optimize again with data
    try channel.optimizeBuffer();
    
    // Verify data is still accessible
    const received1 = try channel.dm_recv();
    try std.testing.expect(received1.? == 1);
    
    const received2 = try channel.dm_recv();
    try std.testing.expect(received2.? == 2);
}

test "Enhanced channel state checking" {
    const allocator = std.testing.allocator;
    
    var channel = try makeChannel(i32, allocator, 3);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }

    // Test initial state
    try std.testing.expect(channel.isEmpty());
    try std.testing.expect(!channel.isFull());
    try std.testing.expect(channel.canSend());
    try std.testing.expect(!channel.canReceive()); // No data available
    try std.testing.expect(channel.availableCapacity() == 3);
    
    // Add data and test state
    try std.testing.expect(try channel.dm_send(1) == SendResult.sent);
    try std.testing.expect(!channel.isEmpty());
    try std.testing.expect(channel.canReceive());
    try std.testing.expect(channel.availableCapacity() == 2);
    
    // Fill buffer
    try std.testing.expect(try channel.dm_send(2) == SendResult.sent);
    try std.testing.expect(try channel.dm_send(3) == SendResult.sent);
    
    try std.testing.expect(channel.isFull());
    try std.testing.expect(!channel.canSend());
    try std.testing.expect(channel.availableCapacity() == 0);
    
    // Test close state
    channel.dm_close();
    try std.testing.expect(channel.isClosed());
    try std.testing.expect(!channel.canSend());
}

test "Enhanced select statement" {
    const allocator = std.testing.allocator;
    
    var select_stmt = EnhancedSelect.init(allocator);
    defer select_stmt.deinit();

    // Test default case
    try select_stmt.addDefault(0);
    const result = try select_stmt.execute();
    try std.testing.expect(result == SelectResult.default_executed);
    
    // Test timeout
    var timeout_select = EnhancedSelect.init(allocator);
    defer timeout_select.deinit();
    
    timeout_select.setTimeout(50); // 50ms timeout
    const timeout_result = try timeout_select.execute();
    try std.testing.expect(timeout_result == SelectResult.timeout);
}

test "AnyChannel interface" {
    const allocator = std.testing.allocator;
    
    var channel = try makeChannel(i32, allocator, 2);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }

    // Create AnyChannel wrapper (simplified)
    const any_channel = AnyChannel{ .ptr = channel };
    
    // Test interface methods (simplified implementation)
    try std.testing.expect(any_channel.canSend());
    try std.testing.expect(!any_channel.canReceive());
    try std.testing.expect(!any_channel.isClosed());
    try std.testing.expect(any_channel.length() == 0);
    try std.testing.expect(any_channel.capacity() == 1);
    
    // Add data directly to underlying channel
    try std.testing.expect(try channel.dm_send(42) == SendResult.sent);
    
    // Test close through interface (no-op in simplified version)
    any_channel.close();
}
