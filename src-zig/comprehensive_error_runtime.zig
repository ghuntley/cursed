const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const builtin = @import("builtin");

const ast = @import("ast.zig");
const ErrorContext = @import("error_handling.zig").ErrorContext;
const CursedError = @import("error_handling.zig").CursedError;

/// Comprehensive Error Handling Runtime System for CURSED
/// Implements full specification compliance with yikes/shook/fam operators
/// Includes error propagation, recovery, cleanup patterns, stack traces, and memory safety

/// Error types matching CURSED specification
pub const ErrorType = enum {
    memory_yikes,      // Memory allocation/management errors
    io_yikes,          // Input/output and file system errors  
    network_yikes,     // Network and communication errors
    parse_yikes,       // Parsing and syntax errors
    type_yikes,        // Type system and validation errors
    runtime_yikes,     // Runtime execution errors
    security_yikes,    // Security and permission errors
    performance_yikes, // Performance and resource errors
    
    pub fn fromString(str: []const u8) ErrorType {
        const mapping = std.ComptimeStringMap(ErrorType, .{
            .{ "memory_yikes", .memory_yikes },
            .{ "io_yikes", .io_yikes },
            .{ "network_yikes", .network_yikes },
            .{ "parse_yikes", .parse_yikes },
            .{ "type_yikes", .type_yikes },
            .{ "runtime_yikes", .runtime_yikes },
            .{ "security_yikes", .security_yikes },
            .{ "performance_yikes", .performance_yikes },
        });
        return mapping.get(str) orelse .runtime_yikes;
    }
};

/// Error severity levels from specification
pub const ErrorSeverity = enum(u8) {
    info = 0,        // Informational - no action needed
    warning = 1,     // Warning - should be noted
    error = 2,       // Error - affects operation but recoverable
    critical = 3,    // Critical - requires immediate attention
    fatal = 4,       // Fatal - may cause system instability
    
    pub fn toString(self: ErrorSeverity) []const u8 {
        return switch (self) {
            .info => "INFO",
            .warning => "WARNING", 
            .error => "ERROR",
            .critical => "CRITICAL",
            .fatal => "FATAL",
        };
    }
};

/// Enhanced stack frame with local variable capture
pub const StackFrame = struct {
    function_name: []const u8,
    file_path: []const u8,
    line_number: u32,
    column_number: u32,
    scope_id: u32,
    locals: HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,

    pub fn init(allocator: Allocator, function: []const u8, file: []const u8, line: u32, column: u32) !StackFrame {
        return StackFrame{
            .function_name = try allocator.dupe(u8, function),
            .file_path = try allocator.dupe(u8, file),
            .line_number = line,
            .column_number = column,
            .scope_id = 0,
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
        self.locals.deinit(allocator);
    }

    pub fn addLocal(self: *StackFrame, name: []const u8, value: []const u8) !void {
        const name_copy = try self.allocator.dupe(u8, name);
        const value_copy = try self.allocator.dupe(u8, value);
        try self.locals.put(name_copy, value_copy);
    }

    pub fn format(self: StackFrame, writer: anytype) !void {
        try writer.print("  at {s}() in {s}:{}:{}\n", .{ 
            self.function_name, 
            self.file_path, 
            self.line_number, 
            self.column_number 
        });
        
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

/// Comprehensive YIKES error implementation
pub const YikesError = struct {
    message: []const u8,
    error_code: i64,
    error_type: ErrorType,
    severity: ErrorSeverity,
    source_location: ?SourceLocation,
    stack_trace: ?ArrayList(StackFrame),
    context_data: HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    inner_error: ?*YikesError,
    timestamp: i64,
    recoverable: bool,
    allocator: Allocator,

    pub const SourceLocation = struct {
        file: []const u8,
        line: u32,
        column: u32,
        function: []const u8,
    };

    pub fn init(
        allocator: Allocator,
        message: []const u8,
        error_type: ErrorType,
        severity: ErrorSeverity,
        code: i64
    ) !YikesError {
        return YikesError{
            .message = try allocator.dupe(u8, message),
            .error_code = code,
            .error_type = error_type,
            .severity = severity,
            .source_location = null,
            .stack_trace = null,
            .context_data = HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .inner_error = null,
            .timestamp = std.time.timestamp(),
            .recoverable = severity != .fatal,
            .allocator = allocator,
        };
    }

    pub fn initWithLocation(
        allocator: Allocator,
        message: []const u8,
        error_type: ErrorType,
        severity: ErrorSeverity,
        code: i64,
        location: SourceLocation
    ) !YikesError {
        var err = try init(allocator, message, error_type, severity, code);
        err.source_location = SourceLocation{
            .file = try allocator.dupe(u8, location.file),
            .line = location.line,
            .column = location.column,
            .function = try allocator.dupe(u8, location.function),
        };
        return err;
    }

    pub fn deinit(self: *YikesError) void {
        self.allocator.free(self.message);
        
        if (self.source_location) |*loc| {
            self.allocator.free(loc.file);
            self.allocator.free(loc.function);
        }
        
        if (self.stack_trace) |*trace| {
            for (trace.items) |*frame| {
                frame.deinit(allocator);
            }
            trace.deinit(allocator);
        }
        
        var iterator = self.context_data.iterator();
        while (iterator.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            self.allocator.free(entry.value_ptr.*);
        }
        self.context_data.deinit(allocator);
        
        if (self.inner_error) |inner| {
            inner.deinit(allocator);
            self.allocator.destroy(inner);
        }
    }

    pub fn addContext(self: *YikesError, key: []const u8, value: []const u8) !void {
        const key_copy = try self.allocator.dupe(u8, key);
        const value_copy = try self.allocator.dupe(u8, value);
        try self.context_data.put(key_copy, value_copy);
    }

    pub fn captureStackTrace(self: *YikesError, runtime: *ErrorRuntime) !void {
        self.stack_trace = try runtime.captureCurrentStackTrace();
    }

    pub fn wrapError(self: *YikesError, inner: *YikesError) void {
        self.inner_error = inner;
    }

    pub fn format(self: YikesError, writer: anytype) !void {
        // Error header with severity and type
        try writer.print("[{s}] {s}: {s} (code: {})\n", .{
            self.severity.toString(),
            @tagName(self.error_type),
            self.message,
            self.error_code,
        });

        // Source location
        if (self.source_location) |loc| {
            try writer.print("  at {s}() in {s}:{}:{}\n", .{
                loc.function,
                loc.file,
                loc.line,
                loc.column,
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
            try writer.print("Stack trace ({} frames):\n", .{trace.items.len});
            for (trace.items) |frame| {
                try frame.format(writer);
            }
        }

        // Inner error
        if (self.inner_error) |inner| {
            try writer.print("\nCaused by:\n");
            try inner.format(writer);
        }
    }
};

/// SHOOK result type for error propagation
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

    pub fn unwrap(self: ShookResult) CursedError!Value {
        return switch (self) {
            .Ok => |value| value,
            .Error => |error_value| {
                return switch (error_value.error_type) {
                    .memory_yikes => CursedError.OutOfMemory,
                    .io_yikes => CursedError.ReadError,
                    .network_yikes => CursedError.SystemError,
                    .parse_yikes => CursedError.ParseError,
                    .type_yikes => CursedError.TypeMismatch,
                    .runtime_yikes => CursedError.RuntimeError,
                    .security_yikes => CursedError.PermissionDenied,
                    .performance_yikes => CursedError.SystemError,
                };
            },
        };
    }

    pub fn unwrapOr(self: ShookResult, default: Value) Value {
        return switch (self) {
            .Ok => |value| value,
            .Error => default,
        };
    }

    pub fn propagate(self: ShookResult, runtime: *ErrorRuntime, context: []const u8) CursedError!Value {
        return switch (self) {
            .Ok => |value| value,
            .Error => |error_value| {
                var propagated_error = error_value;
                try propagated_error.addContext("propagated_from", context);
                try runtime.propagateError(&propagated_error);
                return self.unwrap();
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
            .Error => |*error_value| error_value.deinit(allocator),
        }
    }
};

/// Defer stack for cleanup management
pub const DeferStack = struct {
    entries: ArrayList(DeferEntry),
    current_scope: u32,
    allocator: Allocator,

    const DeferEntry = struct {
        cleanup_func: *const fn () void,
        scope_id: u32,
        context: []const u8,
    };

    pub fn init(allocator: Allocator) DeferStack {
        return DeferStack{
            .entries = .empty,
            .current_scope = 0,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *DeferStack) void {
        self.executeAll();
        self.entries.deinit(allocator);
    }

    pub fn push(self: *DeferStack, cleanup_func: *const fn () void, context: []const u8) !void {
        const entry = DeferEntry{
            .cleanup_func = cleanup_func,
            .scope_id = self.current_scope,
            .context = try self.allocator.dupe(u8, context),
        };
        try self.entries.append(self.allocator, entry);
    }

    pub fn enterScope(self: *DeferStack) void {
        self.current_scope += 1;
    }

    pub fn exitScope(self: *DeferStack) void {
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
        while (self.entries.items.len > 0) {
            const entry = self.entries.pop();
            entry.cleanup_func();
            self.allocator.free(entry.context);
        }
    }
};

/// FAM block for try-catch-finally error handling
pub const FamBlock = struct {
    try_handlers: ArrayList(TryHandler),
    catch_handlers: ArrayList(CatchHandler),
    finally_handler: ?FinallyHandler,
    defer_stack: DeferStack,
    allocator: Allocator,

    pub const TryHandler = struct {
        try_func: *const fn (*ErrorRuntime) ShookResult,
    };

    pub const CatchHandler = struct {
        error_type: ?ErrorType,
        pattern: ?[]const u8,
        handler_func: *const fn (YikesError, *ErrorRuntime) ShookResult,
    };

    pub const FinallyHandler = struct {
        handler_func: *const fn (*ErrorRuntime) void,
    };

    pub fn init(allocator: Allocator) FamBlock {
        return FamBlock{
            .try_handlers = .empty,
            .catch_handlers = .empty,
            .finally_handler = null,
            .defer_stack = DeferStack.init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *FamBlock) void {
        self.try_handlers.deinit(allocator);
        self.catch_handlers.deinit(allocator);
        self.defer_stack.deinit(allocator);
    }

    pub fn addTryHandler(self: *FamBlock, handler: *const fn (*ErrorRuntime) ShookResult) !void {
        try self.try_handlers.append(allocator, TryHandler{ .try_func = handler });
    }

    pub fn addCatchHandler(
        self: *FamBlock,
        error_type: ?ErrorType,
        pattern: ?[]const u8,
        handler: *const fn (YikesError, *ErrorRuntime) ShookResult
    ) !void {
        try self.catch_handlers.append(allocator, CatchHandler{
            .error_type = error_type,
            .pattern = pattern,
            .handler_func = handler,
        });
    }

    pub fn setFinallyHandler(self: *FamBlock, handler: *const fn (*ErrorRuntime) void) void {
        self.finally_handler = FinallyHandler{ .handler_func = handler };
    }

    pub fn execute(self: *FamBlock, runtime: *ErrorRuntime) ShookResult {
        self.defer_stack.enterScope();
        defer {
            self.defer_stack.exitScope();
            if (self.finally_handler) |handler| {
                handler.handler_func(runtime);
            }
        }

        var last_result = ShookResult.ok(ShookResult.Value{ .Void = {} });

        // Execute try handlers
        for (self.try_handlers.items) |try_handler| {
            const result = try_handler.try_func(runtime);
            
            if (result.isError()) {
                const error_value = switch (result) {
                    .Error => |err| err,
                    else => unreachable,
                };

                // Find matching catch handler
                for (self.catch_handlers.items) |catch_handler| {
                    const matches = blk: {
                        if (catch_handler.error_type) |expected_type| {
                            if (error_value.error_type != expected_type) break :blk false;
                        }
                        if (catch_handler.pattern) |pattern| {
                            if (!std.mem.indexOf(u8, error_value.message, pattern)) |_| break :blk false;
                        }
                        break :blk true;
                    };

                    if (matches) {
                        return catch_handler.handler_func(error_value, runtime);
                    }
                }

                // No handler found, propagate error
                return result;
            }

            last_result = result;
        }

        return last_result;
    }
};

/// Main error runtime system
pub const ErrorRuntime = struct {
    call_stack: ArrayList(StackFrame),
    error_stack: ArrayList(YikesError),
    defer_stack: DeferStack,
    fam_stack: ArrayList(*FamBlock),
    error_handlers: HashMap(ErrorType, ArrayList(*const fn (YikesError) void), std.hash_map.AutoContext(ErrorType), std.hash_map.default_max_load_percentage),
    allocator: Allocator,

    pub fn init(allocator: Allocator) ErrorRuntime {
        return ErrorRuntime{
            .call_stack = .empty,
            .error_stack = .empty,
            .defer_stack = DeferStack.init(allocator),
            .fam_stack = .empty,
            .error_handlers = HashMap(ErrorType, ArrayList(*const fn (YikesError) void), std.hash_map.AutoContext(ErrorType), std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *ErrorRuntime) void {
        // Clean up call stack
        for (self.call_stack.items) |*frame| {
            frame.deinit(allocator);
        }
        self.call_stack.deinit(allocator);

        // Clean up error stack
        for (self.error_stack.items) |*error_value| {
            error_value.deinit(allocator);
        }
        self.error_stack.deinit(allocator);

        self.defer_stack.deinit(allocator);
        self.fam_stack.deinit(allocator);

        // Clean up error handlers
        var iterator = self.error_handlers.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.deinit(allocator);
        }
        self.error_handlers.deinit(allocator);
    }

    // YIKES - Error creation and throwing
    pub fn yikes(
        self: *ErrorRuntime,
        message: []const u8,
        error_type: ErrorType,
        severity: ErrorSeverity,
        code: i64,
        location: ?YikesError.SourceLocation
    ) !YikesError {
        var error_obj = if (location) |loc|
            try YikesError.initWithLocation(self.allocator, message, error_type, severity, code, loc)
        else
            try YikesError.init(self.allocator, message, error_type, severity, code);

        // Capture stack trace
        try error_obj.captureStackTrace(self);

        // Add to error stack for tracking
        try self.error_stack.append(self.allocator, error_obj);

        // Call registered error handlers
        if (self.error_handlers.get(error_type)) |handlers| {
            for (handlers.items) |handler| {
                handler(error_obj);
            }
        }

        return error_obj;
    }

    // SHOOK - Error propagation
    pub fn shook(self: *ErrorRuntime, result: ShookResult, context: []const u8) !ShookResult {
        return switch (result) {
            .Ok => result,
            .Error => |*error_value| {
                // Add propagation context
                try error_value.addContext("propagated_through", context);
                
                // Check if we're in a fam block
                if (self.fam_stack.items.len > 0) {
                    // Let fam block handle the error
                    return result;
                } else {
                    // No error handler, propagate up
                    try self.propagateError(error_value);
                    return result;
                }
            },
        };
    }

    // FAM - Enter error recovery block
    pub fn famEnter(self: *ErrorRuntime, fam_block: *FamBlock) !void {
        try self.fam_stack.append(allocator, fam_block);
        self.defer_stack.enterScope();
    }

    // FAM - Exit error recovery block
    pub fn famExit(self: *ErrorRuntime) void {
        if (self.fam_stack.items.len > 0) {
            _ = self.fam_stack.pop();
        }
        self.defer_stack.exitScope();
    }

    // Stack trace capture
    pub fn captureCurrentStackTrace(self: *ErrorRuntime) !ArrayList(StackFrame) {
        var trace = .empty;
        
        // Copy current call stack
        for (self.call_stack.items) |frame| {
            var frame_copy = try StackFrame.init(
                self.allocator,
                frame.function_name,
                frame.file_path,
                frame.line_number,
                frame.column_number
            );
            
            // Copy locals
            var locals_iterator = frame.locals.iterator();
            while (locals_iterator.next()) |entry| {
                try frame_copy.addLocal(entry.key_ptr.*, entry.value_ptr.*);
            }
            
            try trace.append(self.allocator, frame_copy);
        }
        
        return trace;
    }

    // Function call tracking
    pub fn enterFunction(self: *ErrorRuntime, function: []const u8, file: []const u8, line: u32, column: u32) !void {
        const frame = try StackFrame.init(self.allocator, function, file, line, column);
        try self.call_stack.append(self.allocator, frame);
    }

    pub fn exitFunction(self: *ErrorRuntime) void {
        if (self.call_stack.items.len > 0) {
            var frame = self.call_stack.pop();
            frame.deinit(allocator);
        }
    }

    // Error propagation
    pub fn propagateError(self: *ErrorRuntime, error_obj: *YikesError) !void {
        // Add current function context
        if (self.call_stack.items.len > 0) {
            const current_frame = &self.call_stack.items[self.call_stack.items.len - 1];
            try error_obj.addContext("current_function", current_frame.function_name);
            try error_obj.addContext("current_file", current_frame.file_path);
        }

        // Check severity
        if (error_obj.severity == .fatal) {
            // Fatal errors should terminate execution
            std.log.err("FATAL ERROR: {s}", .{error_obj.message});
            std.process.exit(1);
        }
    }

    // Error handler registration
    pub fn registerErrorHandler(self: *ErrorRuntime, error_type: ErrorType, handler: *const fn (YikesError) void) !void {
        var handlers = self.error_handlers.get(error_type) orelse ArrayList(*const fn (YikesError) void).init(self.allocator);
        try handlers.append(self.allocator, handler);
        try self.error_handlers.put(error_type, handlers);
    }

    // Defer management
    pub fn addDefer(self: *ErrorRuntime, cleanup_func: *const fn () void, context: []const u8) !void {
        try self.defer_stack.push(cleanup_func, context);
    }

    // Variable tracking for stack traces
    pub fn setLocalVariable(self: *ErrorRuntime, name: []const u8, value: []const u8) !void {
        if (self.call_stack.items.len > 0) {
            const current_frame = &self.call_stack.items[self.call_stack.items.len - 1];
            try current_frame.addLocal(name, value);
        }
    }

    // Error recovery patterns
    pub fn retryOperation(
        self: *ErrorRuntime,
        operation: *const fn () ShookResult,
        max_attempts: u32,
        backoff_ms: u64
    ) ShookResult {
        var attempt: u32 = 0;
        
        while (attempt < max_attempts) {
            const result = operation();
            
            if (result.isOk()) {
                return result;
            }
            
            attempt += 1;
            if (attempt < max_attempts) {
                // Exponential backoff
                const delay = backoff_ms * (@as(u64, 1) << @intCast(attempt));
                std.time.sleep(delay * std.time.ns_per_ms);
            }
        }
        
        // All attempts failed
        const error_obj = YikesError.init(
            self.allocator,
            "Operation failed after maximum retry attempts",
            .runtime_yikes,
            .error,
            -1
        ) catch unreachable;
        
        return ShookResult.err(error_obj);
    }

    // Circuit breaker pattern
    pub const CircuitBreaker = struct {
        failure_count: u32,
        failure_threshold: u32,
        timeout_ms: u64,
        last_failure_time: i64,
        state: State,
        
        const State = enum { closed, open, half_open };
        
        pub fn init(failure_threshold: u32, timeout_ms: u64) CircuitBreaker {
            return CircuitBreaker{
                .failure_count = 0,
                .failure_threshold = failure_threshold,
                .timeout_ms = timeout_ms,
                .last_failure_time = 0,
                .state = .closed,
            };
        }
        
        pub fn call(self: *CircuitBreaker, operation: *const fn () ShookResult) ShookResult {
            const now = std.time.timestamp();
            
            switch (self.state) {
                .open => {
                    if (now - self.last_failure_time > @as(i64, @intCast(self.timeout_ms))) {
                        self.state = .half_open;
                    } else {
                        const error_obj = YikesError.init(
                            std.heap.page_allocator,
                            "Circuit breaker is open",
                            .runtime_yikes,
                            .error,
                            -2
                        ) catch unreachable;
                        return ShookResult.err(error_obj);
                    }
                },
                .half_open => {},
                .closed => {},
            }
            
            const result = operation();
            
            if (result.isError()) {
                self.onFailure();
            } else {
                self.onSuccess();
            }
            
            return result;
        }
        
        fn onSuccess(self: *CircuitBreaker) void {
            self.failure_count = 0;
            self.state = .closed;
        }
        
        fn onFailure(self: *CircuitBreaker) void {
            self.failure_count += 1;
            self.last_failure_time = std.time.timestamp();
            
            if (self.failure_count >= self.failure_threshold) {
                self.state = .open;
            }
        }
    };

    // Error statistics tracking
    pub const ErrorStats = struct {
        total_errors: u64,
        errors_by_type: HashMap(ErrorType, u64, std.hash_map.AutoContext(ErrorType), std.hash_map.default_max_load_percentage),
        errors_by_severity: HashMap(ErrorSeverity, u64, std.hash_map.AutoContext(ErrorSeverity), std.hash_map.default_max_load_percentage),
        
        pub fn init(allocator: Allocator) ErrorStats {
            return ErrorStats{
                .total_errors = 0,
                .errors_by_type = HashMap(ErrorType, u64, std.hash_map.AutoContext(ErrorType), std.hash_map.default_max_load_percentage).init(allocator),
                .errors_by_severity = HashMap(ErrorSeverity, u64, std.hash_map.AutoContext(ErrorSeverity), std.hash_map.default_max_load_percentage).init(allocator),
            };
        }
        
        pub fn deinit(self: *ErrorStats) void {
            self.errors_by_type.deinit(allocator);
            self.errors_by_severity.deinit(allocator);
        }
        
        pub fn recordError(self: *ErrorStats, error_obj: YikesError) void {
            self.total_errors += 1;
            
            const type_count = self.errors_by_type.get(error_obj.error_type) orelse 0;
            self.errors_by_type.put(error_obj.error_type, type_count + 1) catch {};
            
            const severity_count = self.errors_by_severity.get(error_obj.severity) orelse 0;
            self.errors_by_severity.put(error_obj.severity, severity_count + 1) catch {};
        }
    };

    pub fn getErrorStats(self: *ErrorRuntime) ErrorStats {
        var stats = ErrorStats.init(self.allocator);
        
        for (self.error_stack.items) |error_obj| {
            stats.recordError(error_obj);
        }
        
        return stats;
    }
};

// C-compatible exports for LLVM integration
export fn cursed_error_runtime_create(allocator_ptr: ?*anyopaque) ?*ErrorRuntime {
    if (allocator_ptr == null) return null;
    
    const allocator = @as(*Allocator, @ptrCast(@alignCast(allocator_ptr))).*;
    const runtime = allocator.create(ErrorRuntime) catch return null;
    runtime.* = ErrorRuntime.init(allocator);
    return runtime;
}

export fn cursed_error_runtime_destroy(runtime_ptr: ?*ErrorRuntime) void {
    if (runtime_ptr) |runtime| {
        const allocator = runtime.allocator;
        runtime.deinit(allocator);
        allocator.destroy(runtime);
    }
}

export fn cursed_yikes_create(
    runtime_ptr: ?*ErrorRuntime,
    message_ptr: [*:0]const u8,
    error_type: u32,
    severity: u32,
    code: i64
) ?*YikesError {
    if (runtime_ptr == null) return null;
    
    const runtime = runtime_ptr.?;
    const message = std.mem.span(message_ptr);
    
    const err_type = switch (error_type) {
        0 => ErrorType.memory_yikes,
        1 => ErrorType.io_yikes,
        2 => ErrorType.network_yikes,
        3 => ErrorType.parse_yikes,
        4 => ErrorType.type_yikes,
        5 => ErrorType.runtime_yikes,
        6 => ErrorType.security_yikes,
        7 => ErrorType.performance_yikes,
        else => ErrorType.runtime_yikes,
    };
    
    const err_severity = switch (severity) {
        0 => ErrorSeverity.info,
        1 => ErrorSeverity.warning,
        2 => ErrorSeverity.error,
        3 => ErrorSeverity.critical,
        4 => ErrorSeverity.fatal,
        else => ErrorSeverity.error,
    };
    
    const error_obj = runtime.allocator.create(YikesError) catch return null;
    error_obj.* = runtime.yikes(message, err_type, err_severity, code, null) catch return null;
    return error_obj;
}

export fn cursed_yikes_destroy(error_ptr: ?*YikesError) void {
    if (error_ptr) |err| {
        const allocator = err.allocator;
        err.deinit(allocator);
        allocator.destroy(err);
    }
}

export fn cursed_shook_propagate(
    runtime_ptr: ?*ErrorRuntime,
    result_ptr: ?*ShookResult,
    context_ptr: [*:0]const u8
) ?*ShookResult {
    if (runtime_ptr == null or result_ptr == null) return null;
    
    const runtime = runtime_ptr.?;
    const result = result_ptr.*;
    const context = std.mem.span(context_ptr);
    
    const propagated_result = runtime.shook(result, context) catch return null;
    
    const result_obj = runtime.allocator.create(ShookResult) catch return null;
    result_obj.* = propagated_result;
    return result_obj;
}

export fn cursed_fam_create(runtime_ptr: ?*ErrorRuntime) ?*FamBlock {
    if (runtime_ptr == null) return null;
    
    const runtime = runtime_ptr.?;
    const fam_block = runtime.allocator.create(FamBlock) catch return null;
    fam_block.* = FamBlock.init(runtime.allocator);
    return fam_block;
}

export fn cursed_fam_destroy(fam_ptr: ?*FamBlock) void {
    if (fam_ptr) |fam| {
        const allocator = fam.allocator;
        fam.deinit(allocator);
        allocator.destroy(fam);
    }
}

// Comprehensive test suite
test "comprehensive error handling system" {
    const allocator = std.testing.allocator;
    
    // Test error runtime initialization
    var runtime = ErrorRuntime.init(allocator);
    defer runtime.deinit(allocator);
    
    // Test YIKES error creation
    const location = YikesError.SourceLocation{
        .file = "test.csd",
        .line = 42,
        .column = 10,
        .function = "test_function",
    };
    
    var error_obj = try runtime.yikes(
        "Test error message",
        .runtime_yikes,
        .error,
        100,
        location
    );
    
    // Test error context
    try error_obj.addContext("user_id", "12345");
    try error_obj.addContext("operation", "test_operation");
    
    // Test SHOOK propagation
    const shook_result = ShookResult.err(error_obj);
    try std.testing.expect(shook_result.isError());
    
    const propagated = try runtime.shook(shook_result, "test_context");
    try std.testing.expect(propagated.isError());
    
    // Test FAM block
    var fam_block = FamBlock.init(allocator);
    defer fam_block.deinit(allocator);
    
    // Test function call tracking
    try runtime.enterFunction("test_function", "test.csd", 1, 1);
    try runtime.setLocalVariable("x", "42");
    runtime.exitFunction();
    
    // Test defer stack
    const cleanup_called = struct {
        var called = false;
        fn cleanup() void {
            called = true;
        }
    };
    
    try runtime.addDefer(cleanup_called.cleanup, "test_defer");
    runtime.defer_stack.exitScope();
    
    // Test error stats
    var stats = runtime.getErrorStats();
    defer stats.deinit(allocator);
    
    try std.testing.expect(stats.total_errors > 0);
}

test "error recovery patterns" {
    const allocator = std.testing.allocator;
    
    var runtime = ErrorRuntime.init(allocator);
    defer runtime.deinit(allocator);
    
    // Test retry operation
    const operation = struct {
        var attempt_count: u32 = 0;
        
        fn operation() ShookResult {
            attempt_count += 1;
            if (attempt_count < 3) {
                const error_obj = YikesError.init(
                    std.heap.page_allocator,
                    "Temporary failure",
                    .runtime_yikes,
                    .error,
                    -1
                ) catch unreachable;
                return ShookResult.err(error_obj);
            }
            return ShookResult.ok(ShookResult.Value{ .Integer = 42 });
        }
    };
    
    const result = runtime.retryOperation(operation.operation, 5, 10);
    try std.testing.expect(result.isOk());
    
    // Test circuit breaker
    var circuit_breaker = ErrorRuntime.CircuitBreaker.init(3, 1000);
    
    const failing_operation = struct {
        fn operation() ShookResult {
            const error_obj = YikesError.init(
                std.heap.page_allocator,
                "Always fails",
                .runtime_yikes,
                .error,
                -1
            ) catch unreachable;
            return ShookResult.err(error_obj);
        }
    };
    
    // Should fail and eventually open circuit
    for (0..5) |_| {
        const result = circuit_breaker.call(failing_operation.operation);
        try std.testing.expect(result.isError());
    }
    
    try std.testing.expect(circuit_breaker.state == .open);
}

test "memory safety during error conditions" {
    const allocator = std.testing.allocator;
    
    var runtime = ErrorRuntime.init(allocator);
    defer runtime.deinit(allocator);
    
    // Test that errors are properly cleaned up
    {
        var error_obj = try runtime.yikes(
            "Memory test error",
            .memory_yikes,
            .error,
            1,
            null
        );
        
        try error_obj.addContext("test", "value");
        // error_obj will be cleaned up when runtime is deinitialized
    }
    
    // Test stack trace memory management
    try runtime.enterFunction("test_func", "test.csd", 1, 1);
    try runtime.setLocalVariable("test_var", "test_value");
    runtime.exitFunction();
    
    // Test defer stack memory safety
    const cleanup_func = struct {
        fn cleanup() void {}
    }.cleanup;
    
    try runtime.addDefer(cleanup_func, "test_context");
    runtime.defer_stack.exitScope();
}
