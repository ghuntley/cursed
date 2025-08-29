//! Error Handling and Concurrency Integration
//! Provides unified integration of yikes/fam/shook and stan/dm systems
//! with the CURSED interpreter and code generation

const std = @import("std");
const Allocator = std.mem.Allocator;
const advanced_error_handling = @import("advanced_error_handling.zig");
const advanced_concurrency = @import("advanced_concurrency.zig");

/// Unified runtime that manages both error handling and concurrency
pub const UnifiedRuntime = struct {
    const Self = @This();
    
    allocator: Allocator,
    error_runtime: *advanced_error_handling.ErrorRuntime,
    concurrency_runtime: *advanced_concurrency.ConcurrencyRuntime,
    active_goroutines: std.HashMap(advanced_concurrency.GoroutineId, *GoroutineErrorContext, std.HashMap.AutoContext(advanced_concurrency.GoroutineId), 80),
    
    pub const GoroutineErrorContext = struct {
        goroutine_id: advanced_concurrency.GoroutineId,
        error_stack: std.ArrayList(*advanced_error_handling.CursedError),
        recovery_points: std.ArrayList(usize),
        allocator: Allocator,
        
        pub fn init(allocator: Allocator, goroutine_id: advanced_concurrency.GoroutineId) !*GoroutineErrorContext {
            const context = try allocator.create(GoroutineErrorContext);
            context.* = GoroutineErrorContext{
                .goroutine_id = goroutine_id,
                .error_stack = std.ArrayList(*advanced_error_handling.CursedError){},
                .recovery_points = std.ArrayList(usize){},
                .allocator = allocator,
            };
            return context;
        }
        
        pub fn deinit(self: *GoroutineErrorContext) void {
            for (self.error_stack.items) |err| {
                err.deinit();
            }
            self.error_stack.deinit(self.allocator);
            self.recovery_points.deinit(self.allocator);
            self.allocator.destroy(self);
        }
    };
    
    pub fn init(allocator: Allocator) !*Self {
        _ = allocator;
        const runtime = try allocator.create(Self);
        runtime.* = Self{
            .allocator = allocator,
            .error_runtime = try advanced_error_handling.ErrorRuntime.init(allocator),
            .concurrency_runtime = try advanced_concurrency.ConcurrencyRuntime.init(allocator),
            .active_goroutines = std.HashMap(advanced_concurrency.GoroutineId, *GoroutineErrorContext, std.HashMap.AutoContext(advanced_concurrency.GoroutineId), 80).init(allocator),
        };
        
        // Set up error handlers for goroutine isolation
        try runtime.error_runtime.registerErrorHandler(runtime.handleGoroutineError);
        try runtime.error_runtime.registerPanicHandler(runtime.handleGoroutinePanic);
        
        return runtime;
    }
    
    pub fn deinit(self: *Self) void {
        // Clean up active goroutine contexts
        var iterator = self.active_goroutines.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.*.deinit();
        }
        self.active_goroutines.deinit(self.allocator);
        
        self.error_runtime.deinit(self.allocator);
        self.concurrency_runtime.deinit(self.allocator);
        self.allocator.destroy(self);
    }
    
    /// Handle errors within goroutines
    fn handleGoroutineError(self: *Self, error_obj: *advanced_error_handling.CursedError) void {
        _ = self;
        std.debug.print("Goroutine error: {s}\n", .{error_obj.*});
        // In production, would route to appropriate goroutine context
    }
    
    /// Handle panics within goroutines  
    fn handleGoroutinePanic(self: *Self, error_obj: *advanced_error_handling.CursedError) void {
        _ = self;
        std.debug.print("Goroutine panic: {s}\n", .{error_obj.*});
        // In production, would isolate panic to specific goroutine
    }
    
    /// Spawn goroutine with integrated error handling
    pub fn stanWithErrorHandling(self: *Self, entry_fn: advanced_concurrency.GoroutineEntry, context: ?*anyopaque) !advanced_concurrency.GoroutineId {
        const goroutine_id = try self.concurrency_runtime.stan(entry_fn, context);
        
        // Create error context for this goroutine
        const error_context = try GoroutineErrorContext.init(self.allocator, goroutine_id);
        try self.active_goroutines.put(goroutine_id, error_context);
        
        return goroutine_id;
    }
    
    /// Create yikes error with goroutine context
    pub fn yikesInGoroutine(self: *Self, goroutine_id: advanced_concurrency.GoroutineId, message: []const u8, error_type: advanced_error_handling.CursedError.ErrorType, code: i64) !*advanced_error_handling.CursedError {
        const error_obj = try self.error_runtime.executeYikes(message, error_type, code);
        
        // Add to goroutine's error stack if it exists
        if (self.active_goroutines.get(goroutine_id)) |goroutine_context| {
            try goroutine_context.error_stack.append(allocator, error_obj);
        }
        
        return error_obj;
    }
    
    /// Enter fam block in goroutine context
    pub fn famInGoroutine(self: *Self, goroutine_id: advanced_concurrency.GoroutineId, cleanup_fn: ?*const fn() void) !usize {
        const recovery_point = try self.error_runtime.enterFamBlock(cleanup_fn, null);
        
        // Track recovery point in goroutine context
        if (self.active_goroutines.get(goroutine_id)) |goroutine_context| {
            try goroutine_context.recovery_points.append(allocator, recovery_point);
        }
        
        return recovery_point;
    }
    
    /// Execute shook with goroutine isolation
    pub fn shookInGoroutine(self: *Self, goroutine_id: advanced_concurrency.GoroutineId, error_obj: *advanced_error_handling.CursedError) noreturn {
        // In a goroutine, shook should be isolated and not crash the entire program
        std.debug.print("Goroutine {s} shook: {s}\n", .{goroutine_id, error_obj.*});
        
        // For now, we'll exit the current goroutine context
        // In production, this would properly unwind the goroutine stack
        self.error_runtime.executeShook(error_obj);
    }
    
    /// Clean up completed goroutine
    pub fn cleanupGoroutine(self: *Self, goroutine_id: advanced_concurrency.GoroutineId) void {
        if (self.active_goroutines.fetchRemove(goroutine_id)) |entry| {
            entry.value.deinit();
        }
    }
};

/// Interpreter value type for integration
pub const InterpreterValue = union(enum) {
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    Array: []InterpreterValue,
    Error: *advanced_error_handling.CursedError,
    Channel: *anyopaque, // Type-erased channel pointer
    Goroutine: advanced_concurrency.GoroutineId,
    Null,
    
    pub fn format(
        self: InterpreterValue,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        _ = options;
        
        switch (self) {
            .Integer => |val| try writer.print("{d}", .{val}),
            .Float => |val| try writer.print("{d}", .{val}),
            .String => |val| try writer.print("\"{}\"", .{std.fmt.fmtSliceEscapeLower(val)}),
            .Boolean => |val| try writer.print("{s}", .{val}),
            .Array => |arr| {
                try writer.print("[");
                for (arr, 0..) |item, i| {
                    if (i > 0) try writer.print(", ");
                    try writer.print("{s}", .{item});
                }
                try writer.print("]");
            },
            .Error => |err| try writer.print("Error({s})", .{err.*}),
            .Channel => try writer.print("Channel({*})", .{self.Channel}),
            .Goroutine => |id| try writer.print("Goroutine({s})", .{id}),
            .Null => try writer.print("null"),
        }
    }
    
    pub fn isError(self: InterpreterValue) bool {
        return self == .Error;
    }
    
    pub fn isTruthy(self: InterpreterValue) bool {
        return switch (self) {
            .Integer => |val| val != 0,
            .Float => |val| val != 0.0,
            .String => |val| val.len > 0,
            .Boolean => |val| val,
            .Array => |arr| arr.len > 0,
            .Error => false,
            .Channel => true,
            .Goroutine => true,
            .Null => false,
        };
    }
};

/// Integration functions for interpreter execution

/// Execute yikes statement in interpreter
pub fn executeYikesStatement(runtime: *UnifiedRuntime, message: []const u8, error_type: advanced_error_handling.CursedError.ErrorType, code: i64) !InterpreterValue {
    const error_obj = try runtime.error_runtime.executeYikes(message, error_type, code);
    return InterpreterValue{ .Error = error_obj };
}

/// Execute shook expression in interpreter  
pub fn executeShookExpression(runtime: *UnifiedRuntime, value: InterpreterValue) InterpreterValue {
    switch (value) {
        .Error => |error_obj| {
            // In interpreter mode, we don't actually panic, just return the error
            std.debug.print("shook executed with error: {s}\n", .{error_obj.*});
            return value;
        },
        else => {
            // shook with non-error value creates a runtime error
            const error_obj = runtime.error_runtime.executeYikes(
                "shook called on non-error value",
                .Runtime,
                1001
            ) catch unreachable;
            return InterpreterValue{ .Error = error_obj };
        },
    }
}

/// Execute fam statement in interpreter
pub fn executeFamStatement(runtime: *UnifiedRuntime, try_block: *const fn() InterpreterValue, catch_block: ?*const fn(*advanced_error_handling.CursedError) InterpreterValue) InterpreterValue {
    const recovery_point = runtime.error_runtime.enterFamBlock(null, null) catch {
        const error_obj = runtime.error_runtime.executeYikes(
            "Failed to enter fam block",
            .Runtime,
            1002
        ) catch unreachable;
        return InterpreterValue{ .Error = error_obj };
    };
    
    defer runtime.error_runtime.exitFamBlock(recovery_point);
    
    // Execute try block
    const result = try_block();
    
    // Check if an error occurred
    switch (result) {
        .Error => |error_obj| {
            if (catch_block) |handler| {
                return handler(error_obj);
            }
            return result;
        },
        else => return result,
    }
}

/// Execute stan statement in interpreter
pub fn executeStanStatement(runtime: *UnifiedRuntime, entry_fn: advanced_concurrency.GoroutineEntry, context: ?*anyopaque) !InterpreterValue {
    const goroutine_id = try runtime.stanWithErrorHandling(entry_fn, context);
    return InterpreterValue{ .Goroutine = goroutine_id };
}

/// Create dm channel in interpreter
pub fn createDmChannel(runtime: *UnifiedRuntime, element_type: type, capacity: usize) !InterpreterValue {
    _ = element_type; // For now, we'll use a generic approach
    
    const channel = try runtime.concurrency_runtime.dmMake(i64, capacity);
    return InterpreterValue{ .Channel = channel };
}

/// Send to dm channel in interpreter
pub fn dmSendOperation(runtime: *UnifiedRuntime, channel_value: InterpreterValue, value: InterpreterValue) !InterpreterValue {
    switch (channel_value) {
        .Channel => |channel_ptr| {
            const channel = @as(*advanced_concurrency.Channel(i64), @ptrCast(@alignCast(channel_ptr)));
            switch (value) {
                .Integer => |int_val| {
                    const success = try runtime.concurrency_runtime.dmSend(channel, int_val);
                    return InterpreterValue{ .Boolean = success };
                },
                else => {
                    const error_obj = try runtime.error_runtime.executeYikes(
                        "Invalid value type for channel send",
                        .Type,
                        2001
                    );
                    return InterpreterValue{ .Error = error_obj };
                },
            }
        },
        else => {
            const error_obj = try runtime.error_runtime.executeYikes(
                "dm_send called on non-channel value",
                .Type,
                2002
            );
            return InterpreterValue{ .Error = error_obj };
        },
    }
}

/// Receive from dm channel in interpreter
pub fn dmRecvOperation(runtime: *UnifiedRuntime, channel_value: InterpreterValue) !InterpreterValue {
    switch (channel_value) {
        .Channel => |channel_ptr| {
            const channel = @as(*advanced_concurrency.Channel(i64), @ptrCast(@alignCast(channel_ptr)));
            if (runtime.concurrency_runtime.dmRecv(channel)) |value| {
                return InterpreterValue{ .Integer = value };
            } else {
                return InterpreterValue.Null;
            }
        },
        else => {
            const error_obj = try runtime.error_runtime.executeYikes(
                "dm_recv called on non-channel value",
                .Type,
                2003
            );
            return InterpreterValue{ .Error = error_obj };
        },
    }
}

/// Global unified runtime instance
var global_unified_runtime: ?*UnifiedRuntime = null;

/// Initialize global unified runtime
pub fn initUnifiedRuntime(allocator: Allocator) !void {
        _ = allocator;
    global_unified_runtime = try UnifiedRuntime.init(allocator);
}

/// Deinitialize global unified runtime
pub fn deinitUnifiedRuntime() void {
    if (global_unified_runtime) |runtime| {
        runtime.deinit();
        global_unified_runtime = null;
    }
}

/// Get global unified runtime
pub fn getUnifiedRuntime() *UnifiedRuntime {
    return global_unified_runtime orelse {
        std.debug.panic("Unified runtime not initialized", .{});
    };
}

/// C FFI exports for integration with existing codebase
export fn cursed_unified_init(allocator_ptr: ?*Allocator) c_int {
    if (allocator_ptr) |allocator| {
        initUnifiedRuntime(allocator.*) catch return 0;
        return 1;
    }
    return 0;
}

export fn cursed_unified_deinit() void {
    deinitUnifiedRuntime();
}

export fn cursed_yikes_create(message_ptr: [*:0]const u8, error_type: c_int, code: c_long) ?*advanced_error_handling.CursedError {
    const runtime = getUnifiedRuntime();
    const message = std.mem.span(message_ptr);
    const err_type: advanced_error_handling.CursedError.ErrorType = @enumFromInt(@as(u8, @intCast(error_type)));
    
    return runtime.error_runtime.executeYikes(message, err_type, code) catch null;
}

export fn cursed_stan_spawn(entry_fn: advanced_concurrency.GoroutineEntry, context: ?*anyopaque) u64 {
    const runtime = getUnifiedRuntime();
    return runtime.stanWithErrorHandling(entry_fn, context) catch 0;
}

export fn cursed_dm_make_int(capacity: c_ulong) ?*advanced_concurrency.Channel(i64) {
    const runtime = getUnifiedRuntime();
    return runtime.concurrency_runtime.dmMake(i64, capacity) catch null;
}

// Testing function
fn testGoroutineWithErrors(context: ?*anyopaque) void {
    _ = context;
    const runtime = getUnifiedRuntime();
    
    // Create an error within the goroutine
    const error_obj = runtime.error_runtime.executeYikes(
        "Test error from goroutine",
        .Runtime,
        3001
    ) catch return;
    
    std.debug.print("Goroutine created error: {s}\n", .{error_obj.*});
    error_obj.deinit();
}

pub fn testIntegration(allocator: Allocator) !void {
        _ = allocator;
    try initUnifiedRuntime(allocator);
    defer deinitUnifiedRuntime();
    
    const runtime = getUnifiedRuntime();
    
    std.debug.print("=== Testing Error Handling ===\n");
    
    // Test yikes
    const error_result = try executeYikesStatement(runtime, "Test error message", .Runtime, 500);
    std.debug.print("Created yikes: {s}\n", .{error_result});
    
    // Test shook
    const shook_result = executeShookExpression(runtime, error_result);
    std.debug.print("Shook result: {s}\n", .{shook_result});
    
    std.debug.print("\n=== Testing Concurrency ===\n");
    
    // Test stan
    const goroutine_result = try executeStanStatement(runtime, testGoroutineWithErrors, null);
    std.debug.print("Spawned goroutine: {s}\n", .{goroutine_result});
    
    // Test dm channel
    const channel_result = try createDmChannel(runtime, i64, 5);
    std.debug.print("Created channel: {s}\n", .{channel_result});
    
    // Test dm_send
    const send_result = try dmSendOperation(runtime, channel_result, InterpreterValue{ .Integer = 42 });
    std.debug.print("Send result: {s}\n", .{send_result});
    
    // Test dm_recv
    const recv_result = try dmRecvOperation(runtime, channel_result);
    std.debug.print("Received value: {s}\n", .{recv_result});
    
    // Give goroutine time to execute
    std.time.sleep(10_000_000); // 10ms
    
    std.debug.print("\n=== Integration Test Complete ===\n");
}
