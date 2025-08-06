const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const builtin = @import("builtin");
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/ExecutionEngine.h");
});

/// Enhanced Error Propagation System with Stack Traces and Context Preservation
/// Implements production-quality error handling for CURSED with yikes/shook/fam

/// Stack frame information for error tracing
pub const StackFrame = struct {
    function_name: []const u8,
    file_path: []const u8,
    line_number: u32,
    column_number: u32,
    scope_id: u32,
    frame_address: ?usize,
    locals: HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,

    pub fn init(allocator: Allocator, function_name: []const u8, file_path: []const u8, line: u32, column: u32) !StackFrame {
        return StackFrame{
            .function_name = try allocator.dupe(u8, function_name),
            .file_path = try allocator.dupe(u8, file_path),
            .line_number = line,
            .column_number = column,
            .scope_id = 0,
            .frame_address = null,
            .locals = HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *StackFrame) void {
        self.allocator.free(self.function_name);
        self.allocator.free(self.file_path);
        
        var iterator = self.locals.iterator();
        while (iterator.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            self.allocator.free(entry.value_ptr.*);
        }
        self.locals.deinit();
    }

    pub fn addLocal(self: *StackFrame, name: []const u8, value: []const u8) !void {
        const name_copy = try self.allocator.dupe(u8, name);
        const value_copy = try self.allocator.dupe(u8, value);
        try self.locals.put(name_copy, value_copy);
    }

    pub fn format(self: StackFrame, writer: anytype) !void {
        try writer.print("  at {s}() in {s}:{}:{}\n", .{ self.function_name, self.file_path, self.line_number, self.column_number });
        
        if (self.locals.count() > 0) {
            try writer.print("    locals: ");
            var iterator = self.locals.iterator();
            var first = true;
            while (iterator.next()) |entry| {
                if (!first) try writer.print(", ");
                try writer.print("{s}={s}", .{ entry.key_ptr.*, entry.value_ptr.* });
                first = false;
            }
            try writer.print("\n");
        }
    }
};

/// Enhanced stack trace with runtime capture
pub const StackTrace = struct {
    frames: ArrayList(StackFrame),
    captured_at: i64, // timestamp
    thread_id: u32,
    allocator: Allocator,

    pub fn init(allocator: Allocator) StackTrace {
        return StackTrace{
            .frames = ArrayList(StackFrame).init(allocator),
            .captured_at = std.time.timestamp(),
            .thread_id = 0, // TODO: Get actual thread ID
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *StackTrace) void {
        for (self.frames.items) |*frame| {
            frame.deinit();
        }
        self.frames.deinit();
    }

    pub fn addFrame(self: *StackTrace, frame: StackFrame) !void {
        try self.frames.append(frame);
    }

    pub fn capture(allocator: Allocator) !StackTrace {
        var trace = StackTrace.init(allocator);
        
        // Capture current stack using platform-specific methods
        if (builtin.os.tag == .linux or builtin.os.tag == .macos) {
            try captureUnixStack(&trace);
        } else if (builtin.os.tag == .windows) {
            try captureWindowsStack(&trace);
        } else {
            // Fallback: add a generic frame
            const frame = try StackFrame.init(allocator, "unknown", "unknown", 0, 0);
            try trace.addFrame(frame);
        }
        
        return trace;
    }

    fn captureUnixStack(trace: *StackTrace) !void {
        // Use backtrace() on Unix systems
        var addresses: [128]?*anyopaque = undefined;
        const num_addresses = if (@hasDecl(std.c, "backtrace")) std.c.backtrace(addresses[0..]) else 0;
        
        for (addresses[0..@intCast(num_addresses)]) |addr| {
            if (addr) |address| {
                // Create frame from address (simplified)
                const frame = try StackFrame.init(
                    trace.allocator,
                    "native_function",
                    "native",
                    0,
                    0
                );
                try trace.addFrame(frame);
            }
        }
        
        // If no native backtrace, add runtime frame
        if (num_addresses == 0) {
            const frame = try StackFrame.init(
                trace.allocator,
                "cursed_runtime",
                "runtime",
                0,
                0
            );
            try trace.addFrame(frame);
        }
    }

    fn captureWindowsStack(trace: *StackTrace) !void {
        // Windows-specific stack capture would go here
        // For now, add a placeholder frame
        const frame = try StackFrame.init(
            trace.allocator,
            "windows_function",
            "windows",
            0,
            0
        );
        try trace.addFrame(frame);
    }

    pub fn format(self: StackTrace, writer: anytype) !void {
        try writer.print("Stack trace ({} frames):\n", .{self.frames.items.len});
        for (self.frames.items) |frame| {
            try frame.format(writer);
        }
    }
};

/// Enhanced YIKES error with full context
pub const YikesError = struct {
    message: []const u8,
    error_code: i64,
    source_location: ?SourceLocation,
    stack_trace: ?StackTrace,
    context_data: HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    inner_error: ?*YikesError,
    error_type: ErrorType,
    severity: ErrorSeverity,
    recoverable: bool,
    allocator: Allocator,

    pub const SourceLocation = struct {
        file: []const u8,
        line: u32,
        column: u32,
        function: []const u8,
    };

    pub const ErrorType = enum {
        Runtime,
        Parse,
        Type,
        Memory,
        IO,
        Network,
        Concurrency,
        Security,
        User,
    };

    pub const ErrorSeverity = enum {
        Info,
        Warning,
        Error,
        Critical,
        Fatal,
    };

    pub fn init(
        allocator: Allocator,
        message: []const u8,
        code: i64,
        error_type: ErrorType,
        severity: ErrorSeverity
    ) !YikesError {
        return YikesError{
            .message = try allocator.dupe(u8, message),
            .error_code = code,
            .source_location = null,
            .stack_trace = null,
            .context_data = HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .inner_error = null,
            .error_type = error_type,
            .severity = severity,
            .recoverable = severity != .Fatal,
            .allocator = allocator,
        };
    }

    pub fn initWithStackTrace(
        allocator: Allocator,
        message: []const u8,
        code: i64,
        error_type: ErrorType,
        severity: ErrorSeverity
    ) !YikesError {
        var err = try init(allocator, message, code, error_type, severity);
        err.stack_trace = try StackTrace.capture(allocator);
        return err;
    }

    pub fn initWithLocation(
        allocator: Allocator,
        message: []const u8,
        code: i64,
        error_type: ErrorType,
        severity: ErrorSeverity,
        location: SourceLocation
    ) !YikesError {
        var err = try init(allocator, message, code, error_type, severity);
        err.source_location = SourceLocation{
            .file = try allocator.dupe(u8, location.file),
            .line = location.line,
            .column = location.column,
            .function = try allocator.dupe(u8, location.function),
        };
        err.stack_trace = try StackTrace.capture(allocator);
        return err;
    }

    pub fn deinit(self: *YikesError) void {
        self.allocator.free(self.message);
        
        if (self.source_location) |*loc| {
            self.allocator.free(loc.file);
            self.allocator.free(loc.function);
        }
        
        if (self.stack_trace) |*trace| {
            trace.deinit();
        }
        
        var context_iterator = self.context_data.iterator();
        while (context_iterator.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            self.allocator.free(entry.value_ptr.*);
        }
        self.context_data.deinit();
        
        if (self.inner_error) |inner| {
            inner.deinit();
            self.allocator.destroy(inner);
        }
    }

    pub fn addContext(self: *YikesError, key: []const u8, value: []const u8) !void {
        const key_copy = try self.allocator.dupe(u8, key);
        const value_copy = try self.allocator.dupe(u8, value);
        try self.context_data.put(key_copy, value_copy);
    }

    pub fn setInnerError(self: *YikesError, inner: *YikesError) void {
        self.inner_error = inner;
    }

    pub fn format(self: YikesError, writer: anytype) !void {
        // Error header
        try writer.print("[{s}] yikes: {s} (code: {})\n", .{ 
            @tagName(self.severity), 
            self.message, 
            self.error_code 
        });
        
        // Source location
        if (self.source_location) |loc| {
            try writer.print("  at {s}() in {s}:{}:{}\n", .{ 
                loc.function, 
                loc.file, 
                loc.line, 
                loc.column 
            });
        }
        
        // Context data
        if (self.context_data.count() > 0) {
            try writer.print("  context: ");
            var iterator = self.context_data.iterator();
            var first = true;
            while (iterator.next()) |entry| {
                if (!first) try writer.print(", ");
                try writer.print("{s}={s}", .{ entry.key_ptr.*, entry.value_ptr.* });
                first = false;
            }
            try writer.print("\n");
        }
        
        // Stack trace
        if (self.stack_trace) |trace| {
            try trace.format(writer);
        }
        
        // Inner error
        if (self.inner_error) |inner| {
            try writer.print("\nCaused by:\n");
            try inner.format(writer);
        }
    }

    pub fn toString(self: YikesError) ![]u8 {
        var buffer = ArrayList(u8).init(self.allocator);
        defer buffer.deinit();
        
        const writer = buffer.writer();
        try self.format(writer);
        
        return try self.allocator.dupe(u8, buffer.items);
    }
};

/// Enhanced SHOOK result with context preservation
pub const ShookResult = union(enum) {
    Ok: Value,
    Error: YikesError,

    pub const Value = union(enum) {
        Integer: i64,
        Float: f64,
        String: []const u8,
        Boolean: bool,
        Void: void,
        Pointer: *anyopaque,
    };

    pub fn ok(value: Value) ShookResult {
        return ShookResult{ .Ok = value };
    }

    pub fn err(error_value: YikesError) ShookResult {
        return ShookResult{ .Error = error_value };
    }

    pub fn isOk(self: ShookResult) bool {
        return switch (self) {
            .Ok => true,
            .Error => false,
        };
    }

    pub fn isError(self: ShookResult) bool {
        return !self.isOk();
    }

    pub fn propagate(self: ShookResult, allocator: Allocator, current_function: []const u8) !Value {
        return switch (self) {
            .Ok => |value| value,
            .Error => |error_value| {
                // Add propagation context
                var propagated_error = error_value;
                try propagated_error.addContext("propagated_from", current_function);
                
                // Return appropriate error type
                const cursed_error = switch (error_value.error_type) {
                    .Runtime => error.RuntimeError,
                    .Parse => error.ParseError,
                    .Type => error.TypeMismatch,
                    .Memory => error.OutOfMemory,
                    .IO => error.ReadError,
                    .Network => error.SystemError,
                    .Concurrency => error.ThreadError,
                    .Security => error.PermissionDenied,
                    .User => error.InvalidOperation,
                };
                return cursed_error;
            },
        };
    }

    pub fn deinit(self: *ShookResult, allocator: Allocator) void {
        switch (self.*) {
            .Ok => |*value| {
                switch (value.*) {
                    .String => |str| allocator.free(str),
                    else => {},
                }
            },
            .Error => |*error_value| error_value.deinit(),
        }
    }
};

/// Defer integration for error unwinding
const DeferEntry = struct {
    cleanup_func: *const fn () void,
    scope_id: u32,
    context: []const u8,
};

pub const DeferStack = struct {
    entries: ArrayList(DeferEntry),
    current_scope: u32,
    allocator: Allocator,

    pub fn init(allocator: Allocator) DeferStack {
        return DeferStack{
            .entries = ArrayList(DeferEntry).init(allocator),
            .current_scope = 0,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *DeferStack) void {
        // Execute all remaining defers before cleanup
        self.executeAll();
        self.entries.deinit();
    }

    pub fn push(self: *DeferStack, cleanup_func: *const fn () void, context: []const u8) !void {
        const entry = DeferEntry{
            .cleanup_func = cleanup_func,
            .scope_id = self.current_scope,
            .context = try self.allocator.dupe(u8, context),
        };
        try self.entries.append(entry);
    }

    pub fn enterScope(self: *DeferStack) void {
        self.current_scope += 1;
    }

    pub fn exitScope(self: *DeferStack) void {
        // Execute all defers for current scope
        var i = self.entries.items.len;
        while (i > 0) {
            i -= 1;
            const entry = self.entries.items[i];
            if (entry.scope_id == self.current_scope) {
                entry.cleanup_func();
                self.allocator.free(entry.context);
                _ = self.entries.orderedRemove(i);
            }
        }
        
        if (self.current_scope > 0) {
            self.current_scope -= 1;
        }
    }

    pub fn executeAll(self: *DeferStack) void {
        // Execute in LIFO order
        while (self.entries.items.len > 0) {
            const entry = self.entries.pop();
            entry.cleanup_func();
            self.allocator.free(entry.context);
        }
    }
};

/// Enhanced FAM (panic recovery) block with defer integration
pub const FamBlock = struct {
    defer_stack: DeferStack,
    error_handlers: ArrayList(ErrorHandler),
    finally_handler: ?FinallyHandler,
    allocator: Allocator,

    pub const ErrorHandler = struct {
        error_type: YikesError.ErrorType,
        handler_func: *const fn (YikesError) ShookResult,
    };

    pub const FinallyHandler = struct {
        handler_func: *const fn () void,
    };

    pub fn init(allocator: Allocator) FamBlock {
        return FamBlock{
            .defer_stack = DeferStack.init(allocator),
            .error_handlers = ArrayList(ErrorHandler).init(allocator),
            .finally_handler = null,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *FamBlock) void {
        self.defer_stack.deinit();
        self.error_handlers.deinit();
    }

    pub fn addErrorHandler(self: *FamBlock, error_type: YikesError.ErrorType, handler: *const fn (YikesError) ShookResult) !void {
        try self.error_handlers.append(ErrorHandler{
            .error_type = error_type,
            .handler_func = handler,
        });
    }

    pub fn setFinallyHandler(self: *FamBlock, handler: *const fn () void) void {
        self.finally_handler = FinallyHandler{ .handler_func = handler };
    }

    pub fn execute(self: *FamBlock, try_func: *const fn () ShookResult) ShookResult {
        self.defer_stack.enterScope();
        defer {
            self.defer_stack.exitScope();
            if (self.finally_handler) |handler| {
                handler.handler_func();
            }
        }

        const result = try_func();
        
        if (result.isError()) {
            const error_value = switch (result) {
                .Error => |err| err,
                else => unreachable,
            };
            
            // Try to find appropriate error handler
            for (self.error_handlers.items) |handler| {
                if (handler.error_type == error_value.error_type) {
                    return handler.handler_func(error_value);
                }
            }
            
            // No handler found, propagate error
            return result;
        }
        
        return result;
    }
};

/// Runtime error context manager
pub const ErrorContext = struct {
    current_function: []const u8,
    current_file: []const u8,
    current_line: u32,
    current_column: u32,
    defer_stack: DeferStack,
    allocator: Allocator,

    pub fn init(allocator: Allocator) ErrorContext {
        return ErrorContext{
            .current_function = "unknown",
            .current_file = "unknown",
            .current_line = 0,
            .current_column = 0,
            .defer_stack = DeferStack.init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *ErrorContext) void {
        self.defer_stack.deinit();
    }

    pub fn setLocation(self: *ErrorContext, function: []const u8, file: []const u8, line: u32, column: u32) void {
        self.current_function = function;
        self.current_file = file;
        self.current_line = line;
        self.current_column = column;
    }

    pub fn createError(self: *ErrorContext, message: []const u8, code: i64, error_type: YikesError.ErrorType, severity: YikesError.ErrorSeverity) !YikesError {
        const location = YikesError.SourceLocation{
            .file = self.current_file,
            .line = self.current_line,
            .column = self.current_column,
            .function = self.current_function,
        };
        return YikesError.initWithLocation(self.allocator, message, code, error_type, severity, location);
    }
};

// Export C functions for LLVM integration
export fn cursed_error_create(message_ptr: [*]const u8, message_len: usize, code: i64, error_type: u32) *YikesError {
    const allocator = std.heap.page_allocator;
    const message = message_ptr[0..message_len];
    
    const err_type = switch (error_type) {
        0 => YikesError.ErrorType.Runtime,
        1 => YikesError.ErrorType.Parse,
        2 => YikesError.ErrorType.Type,
        3 => YikesError.ErrorType.Memory,
        4 => YikesError.ErrorType.IO,
        5 => YikesError.ErrorType.Network,
        6 => YikesError.ErrorType.Concurrency,
        7 => YikesError.ErrorType.Security,
        else => YikesError.ErrorType.User,
    };
    
    const error_obj = allocator.create(YikesError) catch return null;
    error_obj.* = YikesError.initWithStackTrace(allocator, message, code, err_type, .Error) catch return null;
    return error_obj;
}

export fn cursed_error_destroy(error_ptr: *YikesError) void {
    const allocator = std.heap.page_allocator;
    error_ptr.deinit();
    allocator.destroy(error_ptr);
}

export fn cursed_stack_trace_capture() *StackTrace {
    const allocator = std.heap.page_allocator;
    const trace_obj = allocator.create(StackTrace) catch return null;
    trace_obj.* = StackTrace.capture(allocator) catch return null;
    return trace_obj;
}

export fn cursed_stack_trace_destroy(trace_ptr: *StackTrace) void {
    const allocator = std.heap.page_allocator;
    trace_ptr.deinit();
    allocator.destroy(trace_ptr);
}

// Error propagation tests
test "enhanced error system" {
    const allocator = std.testing.allocator;
    
    // Test enhanced YIKES error creation
    var err = try YikesError.initWithStackTrace(
        allocator,
        "Test error with stack trace",
        42,
        .Runtime,
        .Error
    );
    defer err.deinit();
    
    // Add context
    try err.addContext("user_id", "12345");
    try err.addContext("operation", "file_read");
    
    // Test error formatting
    const error_string = try err.toString();
    defer allocator.free(error_string);
    
    try std.testing.expect(std.mem.indexOf(u8, error_string, "Test error with stack trace") != null);
    try std.testing.expect(std.mem.indexOf(u8, error_string, "user_id=12345") != null);
    
    // Test SHOOK propagation
    const shook_result = ShookResult.err(err);
    try std.testing.expect(shook_result.isError());
    
    // Test defer stack
    var defer_stack = DeferStack.init(allocator);
    defer defer_stack.deinit();
    
    defer_stack.enterScope();
    defer_stack.exitScope();
    
    // Test FAM block
    var fam = FamBlock.init(allocator);
    defer fam.deinit();
    
    // Add error handler
    const test_handler = struct {
        fn handle(error_value: YikesError) ShookResult {
            _ = error_value;
            return ShookResult.ok(ShookResult.Value{ .Integer = 42 });
        }
    }.handle;
    
    try fam.addErrorHandler(.Runtime, test_handler);
}
