//! CURSED Concurrency Runtime Integration
//!
//! This module provides the runtime bridge between CURSED's concurrency keywords
//! (stan, dm<T>, ready) and the underlying Zig concurrency implementation.
//!
//! Features:
//! - Integration with CURSED interpreter and compiler
//! - Runtime support for goroutines, channels, and select statements
//! - Memory management and garbage collection integration
//! - Performance monitoring and debugging

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

const concurrency = @import("concurrency.zig");
const ast = @import("ast_simple.zig");

/// Runtime value types for concurrency
pub const ConcurrencyValue = union(enum) {
    goroutine_id: concurrency.GoroutineId,
    channel_i32: *concurrency.Channel(i32),
    channel_string: *concurrency.Channel([]const u8),
    channel_bool: *concurrency.Channel(bool),
    select_result: concurrency.SelectResult,
    void_value: void,

    pub fn deinit(self: *ConcurrencyValue, allocator: Allocator) void {
        switch (self.*) {
            .channel_i32 => |ch| {
                ch.deinit();
                allocator.destroy(ch);
            },
            .channel_string => |ch| {
                ch.deinit();
                allocator.destroy(ch);
            },
            .channel_bool => |ch| {
                ch.deinit();
                allocator.destroy(ch);
            },
            else => {},
        }
    }
};

/// Concurrency runtime context
pub const ConcurrencyRuntime = struct {
    allocator: Allocator,
    scheduler: ?*concurrency.Scheduler,
    channels: HashMap(concurrency.ChannelId, ConcurrencyValue),
    goroutines: HashMap(concurrency.GoroutineId, *concurrency.Goroutine),
    active: bool,
    stats: RuntimeStats,

    pub fn init(allocator: Allocator) !ConcurrencyRuntime {
        // Initialize scheduler with default configuration
        const config = concurrency.SchedulerConfig.default();
        try concurrency.initializeScheduler(allocator, config);

        return ConcurrencyRuntime{
            .allocator = allocator,
            .scheduler = concurrency.getScheduler(),
            .channels = HashMap(concurrency.ChannelId, ConcurrencyValue).init(allocator),
            .goroutines = HashMap(concurrency.GoroutineId, *concurrency.Goroutine).init(allocator),
            .active = true,
            .stats = RuntimeStats.init(),
        };
    }

    pub fn deinit(self: *ConcurrencyRuntime) void {
        // Clean up channels
        var channel_iter = self.channels.iterator();
        while (channel_iter.next()) |entry| {
            var value = entry.value_ptr;
            value.deinit(self.allocator);
        }
        self.channels.deinit();

        // Clean up goroutines
        self.goroutines.deinit();

        // Shutdown scheduler
        concurrency.shutdownScheduler(self.allocator);
        self.active = false;
    }

    /// Spawn goroutine from CURSED code (implements `stan` keyword)
    pub fn spawnGoroutine(self: *ConcurrencyRuntime, function_ast: *ast.FunctionLiteral, context: ?*anyopaque) !concurrency.GoroutineId {
        if (!self.active or self.scheduler == null) {
            return error.RuntimeNotActive;
        }

        // Create wrapper function that calls CURSED function
        const wrapper_context = try self.allocator.create(GoroutineContext);
        wrapper_context.* = GoroutineContext{
            .runtime = self,
            .function_ast = function_ast,
            .user_context = context,
        };

        const goroutine_id = try concurrency.stan(executeGoroutineWrapper, wrapper_context);
        self.stats.total_goroutines_spawned += 1;

        return goroutine_id;
    }

    /// Create channel from CURSED code (implements `dm<T>` type)
    pub fn createChannel(self: *ConcurrencyRuntime, channel_type: ChannelType, capacity: usize) !concurrency.ChannelId {
        if (!self.active) {
            return error.RuntimeNotActive;
        }

        const channel_id: concurrency.ChannelId = switch (channel_type) {
            .integer => blk: {
                const channel = try concurrency.makeChannel(i32, self.allocator, capacity);
                const value = ConcurrencyValue{ .channel_i32 = channel };
                try self.channels.put(channel.id, value);
                break :blk channel.id;
            },
            .string => blk: {
                const channel = try concurrency.makeChannel([]const u8, self.allocator, capacity);
                const value = ConcurrencyValue{ .channel_string = channel };
                try self.channels.put(channel.id, value);
                break :blk channel.id;
            },
            .boolean => blk: {
                const channel = try concurrency.makeChannel(bool, self.allocator, capacity);
                const value = ConcurrencyValue{ .channel_bool = channel };
                try self.channels.put(channel.id, value);
                break :blk channel.id;
            },
        };

        self.stats.total_channels_created += 1;
        return channel_id;
    }

    /// Send value to channel
    pub fn sendToChannel(self: *ConcurrencyRuntime, channel_id: concurrency.ChannelId, value: ConcurrencyValue) !concurrency.SendResult {
        const channel_value = self.channels.get(channel_id) orelse return error.ChannelNotFound;

        const result = switch (channel_value) {
            .channel_i32 => |ch| switch (value) {
                .goroutine_id => |v| try ch.send(@intCast(v)),
                else => return error.TypeMismatch,
            },
            .channel_string => |_| switch (value) {
                else => return error.TypeMismatch,
            },
            .channel_bool => |_| switch (value) {
                else => return error.TypeMismatch,
            },
            else => return error.InvalidChannelType,
        };

        if (result == .sent) {
            self.stats.total_messages_sent += 1;
        }
        return result;
    }

    /// Receive value from channel
    pub fn receiveFromChannel(self: *ConcurrencyRuntime, channel_id: concurrency.ChannelId) !?ConcurrencyValue {
        const channel_value = self.channels.get(channel_id) orelse return error.ChannelNotFound;

        const result = switch (channel_value) {
            .channel_i32 => |ch| blk: {
                const received = try ch.receive();
                if (received) |val| {
                    break :blk ConcurrencyValue{ .goroutine_id = @intCast(val) };
                } else {
                    break :blk null;
                }
            },
            .channel_string => |ch| blk: {
                const received = try ch.receive();
                if (received) |val| {
                    // Note: In real implementation, need proper string handling
                    _ = val;
                    break :blk ConcurrencyValue{ .void_value = {} };
                } else {
                    break :blk null;
                }
            },
            .channel_bool => |ch| blk: {
                const received = try ch.receive();
                if (received) |val| {
                    _ = val;
                    break :blk ConcurrencyValue{ .void_value = {} };
                } else {
                    break :blk null;
                }
            },
            else => return error.InvalidChannelType,
        };

        if (result != null) {
            self.stats.total_messages_received += 1;
        }
        return result;
    }

    /// Execute select statement (implements `ready` keyword)
    pub fn executeSelect(self: *ConcurrencyRuntime, operations: []const SelectOperation) !concurrency.SelectResult {
        if (!self.active) {
            return error.RuntimeNotActive;
        }

        var select_stmt = concurrency.Select.init(self.allocator);
        defer select_stmt.deinit();

        // Convert CURSED select operations to Zig select operations
        for (operations, 0..) |op, i| {
            switch (op.type) {
                .send => try select_stmt.addSend(op.channel_id, i),
                .receive => try select_stmt.addReceive(op.channel_id, i),
                .default => try select_stmt.addDefault(i),
            }
        }

        const result = try select_stmt.execute();
        self.stats.total_select_operations += 1;

        return result;
    }

    /// Yield current goroutine (implements `yolo` keyword)
    pub fn yieldGoroutine(self: *ConcurrencyRuntime) !void {
        if (!self.active or self.scheduler == null) {
            return error.RuntimeNotActive;
        }

        try concurrency.yolo();
    }

    /// Get runtime statistics
    pub fn getStats(self: *ConcurrencyRuntime) RuntimeStats {
        return self.stats;
    }

    /// Check if runtime is active
    pub fn isActive(self: *ConcurrencyRuntime) bool {
        return self.active;
    }
};

/// Channel type enumeration for CURSED types
pub const ChannelType = enum {
    integer,
    string,
    boolean,
};

/// Select operation for CURSED select statements
pub const SelectOperation = struct {
    type: enum { send, receive, default },
    channel_id: concurrency.ChannelId,
    value: ?ConcurrencyValue,
};

/// Goroutine execution context
const GoroutineContext = struct {
    runtime: *ConcurrencyRuntime,
    function_ast: *ast.FunctionLiteral,
    user_context: ?*anyopaque,
};

/// Wrapper function for executing CURSED goroutines
fn executeGoroutineWrapper(context: ?*anyopaque) void {
    const ctx: *GoroutineContext = @ptrCast(@alignCast(context.?));
    
    // Execute CURSED function AST
    // Note: This would need integration with CURSED interpreter/compiler
    _ = ctx.function_ast;
    _ = ctx.user_context;
    
    // Clean up context
    ctx.runtime.allocator.destroy(ctx);
}

/// Runtime statistics
pub const RuntimeStats = struct {
    total_goroutines_spawned: u64,
    total_channels_created: u64,
    total_messages_sent: u64,
    total_messages_received: u64,
    total_select_operations: u64,

    pub fn init() RuntimeStats {
        return RuntimeStats{
            .total_goroutines_spawned = 0,
            .total_channels_created = 0,
            .total_messages_sent = 0,
            .total_messages_received = 0,
            .total_select_operations = 0,
        };
    }
};

/// Global concurrency runtime instance
var global_runtime: ?*ConcurrencyRuntime = null;
var runtime_mutex: std.Thread.Mutex = std.Thread.Mutex{};

/// Initialize global concurrency runtime
pub fn initializeRuntime(allocator: Allocator) !void {
    runtime_mutex.lock();
    defer runtime_mutex.unlock();

    if (global_runtime != null) {
        return; // Already initialized
    }

    const runtime = try allocator.create(ConcurrencyRuntime);
    runtime.* = try ConcurrencyRuntime.init(allocator);
    global_runtime = runtime;
}

/// Get global concurrency runtime
pub fn getRuntime() ?*ConcurrencyRuntime {
    runtime_mutex.lock();
    defer runtime_mutex.unlock();
    return global_runtime;
}

/// Shutdown global concurrency runtime
pub fn shutdownRuntime(allocator: Allocator) void {
    runtime_mutex.lock();
    defer runtime_mutex.unlock();

    if (global_runtime) |runtime| {
        runtime.deinit();
        allocator.destroy(runtime);
        global_runtime = null;
    }
}

/// High-level API functions for CURSED integration

/// Execute CURSED goroutine spawn statement
/// Execute stan statement from interpreter (spawn goroutine with custom function)
pub fn executeStanFromInterpreter(context: ?*anyopaque, entry_function: concurrency.GoroutineEntry) !concurrency.GoroutineId {
    // Ensure scheduler is initialized
    if (global_runtime == null) {
        try initializeRuntime();
    }
    
    // Spawn goroutine using the provided entry function
    const goroutine_id = try concurrency.stan(entry_function, context);
    
    std.log.debug("Spawned goroutine {} from interpreter", .{goroutine_id});
    return goroutine_id;
}

pub fn executeStan(function_ast: *ast.FunctionLiteral, context: ?*anyopaque) !concurrency.GoroutineId {
    const runtime = getRuntime() orelse return error.RuntimeNotInitialized;
    return runtime.spawnGoroutine(function_ast, context);
}

/// Execute CURSED channel creation
pub fn executeDmCreate(channel_type: ChannelType, capacity: usize) !concurrency.ChannelId {
    const runtime = getRuntime() orelse return error.RuntimeNotInitialized;
    return runtime.createChannel(channel_type, capacity);
}

/// Execute CURSED channel send
pub fn executeDmSend(channel_id: concurrency.ChannelId, value: ConcurrencyValue) !concurrency.SendResult {
    const runtime = getRuntime() orelse return error.RuntimeNotInitialized;
    return runtime.sendToChannel(channel_id, value);
}

/// Execute CURSED channel receive
pub fn executeDmReceive(channel_id: concurrency.ChannelId) !?ConcurrencyValue {
    const runtime = getRuntime() orelse return error.RuntimeNotInitialized;
    return runtime.receiveFromChannel(channel_id);
}

/// Execute CURSED select statement
pub fn executeReady(operations: []const SelectOperation) !concurrency.SelectResult {
    const runtime = getRuntime() orelse return error.RuntimeNotInitialized;
    return runtime.executeSelect(operations);
}

/// Execute CURSED yield statement
pub fn executeYolo() !void {
    const runtime = getRuntime() orelse return error.RuntimeNotInitialized;
    try runtime.yieldGoroutine();
}

// Tests
test "concurrency runtime initialization" {
    const allocator = std.testing.allocator;
    
    try initializeRuntime(allocator);
    defer shutdownRuntime(allocator);

    const runtime = getRuntime();
    try std.testing.expect(runtime != null);
    try std.testing.expect(runtime.?.isActive());
}

test "channel creation and operations" {
    const allocator = std.testing.allocator;
    
    try initializeRuntime(allocator);
    defer shutdownRuntime(allocator);

    const channel_id = try executeDmCreate(.integer, 3);
    try std.testing.expect(channel_id > 0);

    const value = ConcurrencyValue{ .goroutine_id = 42 };
    const send_result = try executeDmSend(channel_id, value);
    try std.testing.expect(send_result == .sent);

    const received = try executeDmReceive(channel_id);
    try std.testing.expect(received != null);
}

test "goroutine spawning" {
    const allocator = std.testing.allocator;
    
    try initializeRuntime(allocator);
    defer shutdownRuntime(allocator);

    // Create mock function AST
    var function_ast = ast.FunctionLiteral{
        .parameters = ArrayList(*ast.Identifier).init(allocator),
        .body = ast.BlockStatement{ .statements = ArrayList(ast.Statement).init(allocator) },
    };
    defer function_ast.parameters.deinit();
    defer function_ast.body.statements.deinit();

    const goroutine_id = try executeStan(&function_ast, null);
    try std.testing.expect(goroutine_id > 0);

    // Wait a bit for execution
    std.time.sleep(1_000_000); // 1ms

    const runtime = getRuntime().?;
    const stats = runtime.getStats();
    try std.testing.expect(stats.total_goroutines_spawned == 1);
}

test "select statement execution" {
    const allocator = std.testing.allocator;
    
    try initializeRuntime(allocator);
    defer shutdownRuntime(allocator);

    const channel_id = try executeDmCreate(.integer, 1);
    
    var operations = [_]SelectOperation{
        SelectOperation{
            .type = .default,
            .channel_id = channel_id,
            .value = null,
        },
    };

    const result = try executeReady(&operations);
    try std.testing.expect(result == .default_executed);
}
