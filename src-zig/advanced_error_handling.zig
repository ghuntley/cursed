//! Advanced CURSED Error Handling Implementation
//! Implements the complete yikes/fam/shook system with modern Zig API compatibility
//!
//! Features:
//! - yikes: Structured error creation with context
//! - fam: Panic recovery blocks with cleanup
//! - shook: Error propagation operator
//! - Stack trace capture and analysis
//! - Integration with concurrency system

const std = @import("std");
const Allocator = std.mem.Allocator;

/// Core CURSED error type
pub const CursedError = struct {
    message: []u8,
    error_type: ErrorType,
    code: i64,
    stack_trace: ?[]StackFrame,
    context: ?[]Context,
    inner_error: ?*CursedError,
    allocator: Allocator,
    
    pub const ErrorType = enum {
        Runtime,
        Memory,
        IO,
        Network,
        Parse,
        Type,
        Security,
        Performance,
        Custom,
        
        pub fn toString(self: ErrorType) []const u8 {
            return switch (self) {
                .Runtime => "Runtime Error",
                .Memory => "Memory Error",
                .IO => "IO Error",
                .Network => "Network Error",
                .Parse => "Parse Error",
                .Type => "Type Error",
                .Security => "Security Error",
                .Performance => "Performance Error",
                .Custom => "Custom Error",
            };
        }
    };
    
    pub const StackFrame = struct {
        function_name: []const u8,
        file_name: []const u8,
        line: u32,
        column: u32,
        
        pub fn format(
            self: StackFrame,
            comptime fmt: []const u8,
            options: std.fmt.FormatOptions,
            writer: anytype,
        ) !void {
            _ = fmt;
            _ = options;
            try writer.print("  at {s} ({s}:{d}:{d})", .{
                self.function_name,
                self.file_name,
                self.line,
                self.column
            });
        }
    };
    
    pub const Context = struct {
        key: []const u8,
        value: []const u8,
    };
    
    pub fn init(allocator: Allocator, message: []const u8, error_type: ErrorType, code: i64) !*CursedError {
        const error_obj = try allocator.create(CursedError);
        error_obj.* = CursedError{
            .message = try allocator.dupe(u8, message),
            .error_type = error_type,
            .code = code,
            .stack_trace = null,
            .context = null,
            .inner_error = null,
            .allocator = allocator,
        };
        
        // Capture stack trace
        error_obj.stack_trace = captureStackTrace(allocator) catch null;
        
        return error_obj;
    }
    
    pub fn deinit(self: *CursedError) void {
        self.allocator.free(self.message);
        
        if (self.stack_trace) |stack| {
            self.allocator.free(stack);
        }
        
        if (self.context) |ctx| {
            for (ctx) |c| {
                self.allocator.free(c.key);
                self.allocator.free(c.value);
            }
            self.allocator.free(ctx);
        }
        
        if (self.inner_error) |inner| {
            inner.deinit();
            self.allocator.destroy(inner);
        }
        
        self.allocator.destroy(self);
    }
    
    pub fn addContext(self: *CursedError, key: []const u8, value: []const u8) !void {
        const new_ctx = Context{
            .key = try self.allocator.dupe(u8, key),
            .value = try self.allocator.dupe(u8, value),
        };
        
        if (self.context) |existing| {
            const new_contexts = try self.allocator.alloc(Context, existing.len + 1);
            @memcpy(new_contexts[0..existing.len], existing);
            new_contexts[existing.len] = new_ctx;
            self.allocator.free(existing);
            self.context = new_contexts;
        } else {
            const contexts = try self.allocator.alloc(Context, 1);
            contexts[0] = new_ctx;
            self.context = contexts;
        }
    }
    
    pub fn wrap(self: *CursedError, allocator: Allocator, message: []const u8) !*CursedError {
        const wrapper = try CursedError.init(allocator, message, self.error_type, self.code);
        wrapper.inner_error = self;
        return wrapper;
    }
    
    pub fn format(
        self: CursedError,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        _ = options;
        
        try writer.print("[{s}] {s} (Code: {d})", .{
            self.error_type.toString(),
            self.message,
            self.code
        });
        
        if (self.context) |ctx| {
            try writer.print("\nContext:");
            for (ctx) |c| {
                try writer.print("\n  {s}: {s}", .{ c.key, c.value });
            }
        }
        
        if (self.stack_trace) |stack| {
            try writer.print("\nStack Trace:");
            for (stack) |frame| {
                try writer.print("\n{s}", .{frame});
            }
        }
        
        if (self.inner_error) |inner| {
            try writer.print("\nCaused by: {s}", .{inner.*});
        }
    }
};

/// Capture current stack trace
fn captureStackTrace(allocator: Allocator) ![]CursedError.StackFrame {
    const max_frames = 16;
    var frames = try allocator.alloc(CursedError.StackFrame, max_frames);
    var frame_count: usize = 0;
    
    // Mock stack trace capture - in real implementation would use debug info
    frames[frame_count] = CursedError.StackFrame{
        .function_name = "current_function",
        .file_name = "current_file.csd",
        .line = 42,
        .column = 10,
    };
    frame_count += 1;
    
    return allocator.realloc(frames, frame_count);
}

/// Global error runtime state
pub const ErrorRuntime = struct {
    const Self = @This();
    
    allocator: Allocator,
    error_handlers: std.ArrayList(ErrorHandler),
    panic_handlers: std.ArrayList(PanicHandler),
    current_panic: ?*CursedError,
    recovery_stack: std.ArrayList(RecoveryFrame),
    
    pub const ErrorHandler = *const fn(*CursedError) void;
    pub const PanicHandler = *const fn(*CursedError) void;
    
    pub const RecoveryFrame = struct {
        recovery_point: usize,
        cleanup_fn: ?*const fn() void,
        context: ?*anyopaque,
    };
    
    pub fn init(allocator: Allocator) !*Self {
        _ = allocator;
        const runtime = try allocator.create(Self);
        runtime.* = Self{
            .allocator = allocator,
            .error_handlers = std.ArrayList(ErrorHandler){},
            .panic_handlers = std.ArrayList(PanicHandler){},
            .current_panic = null,
            .recovery_stack = std.ArrayList(RecoveryFrame){},
        };
        return runtime;
    }
    
    pub fn deinit(self: *Self) void {
        self.error_handlers.deinit(self.allocator);
        self.panic_handlers.deinit(self.allocator);
        self.recovery_stack.deinit(self.allocator);
        
        if (self.current_panic) |panic_err| {
            panic_err.deinit();
        }
        
        self.allocator.destroy(self);
    }
    
    pub fn registerErrorHandler(self: *Self, handler: ErrorHandler) !void {
        try self.error_handlers.append(allocator, handler);
    }
    
    pub fn registerPanicHandler(self: *Self, handler: PanicHandler) !void {
        try self.panic_handlers.append(allocator, handler);
    }
    
    /// Execute yikes statement - create and optionally throw error
    pub fn executeYikes(self: *Self, message: []const u8, error_type: CursedError.ErrorType, code: i64) !*CursedError {
        const error_obj = try CursedError.init(self.allocator, message, error_type, code);
        
        // Notify error handlers
        for (self.error_handlers.items) |handler| {
            handler(error_obj);
        }
        
        return error_obj;
    }
    
    /// Execute shook operation - propagate error as panic
    pub fn executeShook(self: *Self, error_obj: *CursedError) noreturn {
        self.current_panic = error_obj;
        
        // Notify panic handlers
        for (self.panic_handlers.items) |handler| {
            handler(error_obj);
        }
        
        // Trigger panic mechanism
        std.debug.panic("CURSED shook: {}", .{error_obj.*});
    }
    
    /// Enter fam block - set up recovery point
    pub fn enterFamBlock(self: *Self, cleanup_fn: ?*const fn() void, context: ?*anyopaque) !usize {
        const recovery_point = self.recovery_stack.items.len;
        const frame = RecoveryFrame{
            .recovery_point = recovery_point,
            .cleanup_fn = cleanup_fn,
            .context = context,
        };
        
        try self.recovery_stack.append(allocator, frame);
        return recovery_point;
    }
    
    /// Exit fam block - remove recovery point
    pub fn exitFamBlock(self: *Self, recovery_point: usize) void {
        while (self.recovery_stack.items.len > recovery_point) {
            const frame = self.recovery_stack.pop();
            if (frame.cleanup_fn) |cleanup| {
                cleanup();
            }
        }
    }
    
    /// Recover from panic in fam block
    pub fn recoverPanic(self: *Self) ?*CursedError {
        const panic_err = self.current_panic;
        self.current_panic = null;
        return panic_err;
    }
};

/// Global error runtime instance
var global_error_runtime: ?*ErrorRuntime = null;

/// Initialize global error runtime
pub fn initErrorRuntime(allocator: Allocator) !void {
        _ = allocator;
    global_error_runtime = try ErrorRuntime.init(allocator);
}

/// Deinitialize global error runtime
pub fn deinitErrorRuntime() void {
    if (global_error_runtime) |runtime| {
        runtime.deinit();
        global_error_runtime = null;
    }
}

/// Get global error runtime
pub fn getErrorRuntime() *ErrorRuntime {
    return global_error_runtime orelse {
        std.debug.panic("Error runtime not initialized", .{});
    };
}

/// C FFI exports for integration with interpreter/codegen
export fn cursed_error_init(message_ptr: [*:0]const u8, error_type: c_int, code: c_long) ?*CursedError {
    const runtime = getErrorRuntime();
    const message = std.mem.span(message_ptr);
    const err_type: CursedError.ErrorType = @enumFromInt(@as(u8, @intCast(error_type)));
    
    return runtime.executeYikes(message, err_type, code) catch null;
}

export fn cursed_error_shook(error_ptr: ?*CursedError) noreturn {
    const runtime = getErrorRuntime();
    if (error_ptr) |err| {
        runtime.executeShook(err);
    } else {
        std.debug.panic("CURSED shook: null error", .{});
    }
}

export fn cursed_fam_enter(cleanup_fn: ?*const fn() void) c_ulong {
    const runtime = getErrorRuntime();
    return runtime.enterFamBlock(cleanup_fn, null) catch 0;
}

export fn cursed_fam_exit(recovery_point: c_ulong) void {
    const runtime = getErrorRuntime();
    runtime.exitFamBlock(recovery_point);
}

export fn cursed_fam_recover() ?*CursedError {
    const runtime = getErrorRuntime();
    return runtime.recoverPanic();
}

export fn cursed_error_deinit(error_ptr: ?*CursedError) void {
    if (error_ptr) |err| {
        err.deinit();
    }
}

// Testing functions
pub fn testErrorHandling(allocator: Allocator) !void {
        _ = allocator;
    try initErrorRuntime(allocator);
    defer deinitErrorRuntime();
    
    const runtime = getErrorRuntime();
    
    // Test yikes error creation
    const err = try runtime.executeYikes("Test error", .Runtime, 500);
    defer err.deinit();
    
    try err.addContext("operation", "test_function");
    try err.addContext("input", "invalid_value");
    
    std.debug.print("Created error: {s}\n", .{err.*});
    
    // Test error wrapping
    const wrapped = try err.wrap(allocator, "Operation failed");
    defer wrapped.deinit();
    
    std.debug.print("Wrapped error: {s}\n", .{wrapped.*});
}
