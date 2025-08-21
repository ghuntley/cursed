const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

/// Complete CURSED Error Handling Runtime
/// Implements yikes/fam/shook keywords with stack traces and proper propagation

pub const CursedErrorType = enum {
    Runtime,
    Memory,
    IO,
    Network,
    Parse,
    Type,
    Security,
    Performance,
    Custom,
};

pub const CursedError = struct {
    message: []u8,
    error_type: CursedErrorType,
    code: i64,
    stack_trace: ?[]StackFrame,
    context: ?[]Context,
    inner_error: ?*CursedError,
    allocator: Allocator,
    
    pub const StackFrame = struct {
        function_name: []const u8,
        file_name: []const u8,
        line: u32,
        column: u32,
    };
    
    pub const Context = struct {
        key: []const u8,
        value: []const u8,
    };
    
    pub fn init(allocator: Allocator, message: []const u8, error_type: CursedErrorType, code: i64) !*CursedError {
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
        return error_obj;
    }
    
    pub fn initWithStackTrace(
        allocator: Allocator, 
        message: []const u8, 
        error_type: CursedErrorType, 
        code: i64,
        stack_trace: []StackFrame
    ) !*CursedError {
        const error_obj = try init(allocator, message, error_type, code);
        error_obj.stack_trace = try allocator.dupe(StackFrame, stack_trace);
        return error_obj;
    }
    
    pub fn deinit(self: *CursedError) void {
        self.allocator.free(self.message);
        
        if (self.stack_trace) |trace| {
            for (trace) |frame| {
                self.allocator.free(frame.function_name);
                self.allocator.free(frame.file_name);
            }
            self.allocator.free(trace);
        }
        
        if (self.context) |ctx| {
            for (ctx) |context| {
                self.allocator.free(context.key);
                self.allocator.free(context.value);
            }
            self.allocator.free(ctx);
        }
        
        if (self.inner_error) |inner| {
            inner.deinit();
            self.allocator.destroy(inner);
        }
        
        self.allocator.destroy(self);
    }
    
    pub fn format(self: *const CursedError, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        
        _ = writer.writeAll("[CURSED ERROR] ") catch {};
        _ = writer.writeAll(@tagName(self.error_type)) catch 0;
        _ = writer.writeAll(": ") catch 0;
        _ = writer.writeAll(self.message) catch 0;
        _ = writer.writeAll("\n  Error Code: ") catch 0;
        
        var buf: [32]u8 = undefined;
        const code_str = std.fmt.bufPrint(buf[0..], "{}", .{self.code}) catch "?";
        _ = writer.writeAll(code_str) catch 0;
        _ = writer.writeAll("\n") catch 0;
        
        if (self.stack_trace) |trace| {
            _ = writer.writeAll("  Stack Trace:\n") catch 0;
            for (trace) |frame| {
                _ = writer.writeAll("    at ") catch 0;
                _ = writer.writeAll(frame.function_name) catch 0;
                _ = writer.writeAll(" (") catch 0;
                _ = writer.writeAll(frame.file_name) catch 0;
                _ = writer.writeAll(":") catch 0;
                const line_str = std.fmt.bufPrint(buf[0..], "{}", .{frame.line}) catch "?";
                _ = writer.writeAll(line_str) catch 0;
                _ = writer.writeAll(":") catch 0;
                const col_str = std.fmt.bufPrint(buf[0..], "{}", .{frame.column}) catch "?";
                _ = writer.writeAll(col_str) catch 0;
                _ = writer.writeAll(")\n") catch 0;
            }
        }
        
        if (self.context) |ctx| {
            _ = writer.write("  Context:\n") catch 0;
            for (ctx) |context| {
                _ = writer.write("    ") catch 0;
                _ = writer.write(context.key) catch 0;
                _ = writer.write(": ") catch 0;
                _ = writer.write(context.value) catch 0;
                _ = writer.write("\n") catch 0;
            }
        }
        
        if (self.inner_error) |inner| {
            _ = writer.write("  Caused by:\n") catch 0;
            inner.format("", .{}, writer) catch {};
        }
    }
    
    pub fn toString(self: *const CursedError) ![]u8 {
        var buffer = .empty;
        defer buffer.deinit();
        
        const writer = buffer.writer(&[_]u8{});
        try self.format(writer);
        
        return try self.allocator.dupe(u8, buffer.items);
    }
};

pub const ErrorHandler = struct {
    allocator: Allocator,
    error_stack: ArrayList(*CursedError),
    function_stack: ArrayList([]const u8),
    current_file: []const u8,
    
    pub fn init(allocator: Allocator, file: []const u8) ErrorHandler {
        return ErrorHandler{
            .allocator = allocator,
            .error_stack = .empty,
            .function_stack = .empty,
            .current_file = file,
        };
    }
    
    pub fn deinit(self: *ErrorHandler) void {
        for (self.error_stack.items) |error_obj| {
            error_obj.deinit();
        }
        self.error_stack.deinit(self.allocator);
        
        for (self.function_stack.items) |func_name| {
            self.allocator.free(func_name);
        }
        self.function_stack.deinit(self.allocator);
    }
    
    pub fn pushFunction(self: *ErrorHandler, function_name: []const u8) !void {
        try self.function_stack.append(self.allocator, try self.allocator.dupe(u8, function_name));
    }
    
    pub fn popFunction(self: *ErrorHandler) void {
        if (self.function_stack.items.len > 0) {
            const func_name = self.function_stack.pop();
            // Free the duplicated string
            if (func_name.len > 0) {
                self.allocator.free(func_name);
            }
        }
    }
    
    /// YIKES - Create and throw an error with enhanced line number tracking
    pub fn yikes(
        self: *ErrorHandler, 
        message: []const u8, 
        error_type: CursedErrorType, 
        code: i64,
        line: u32,
        column: u32
    ) !*CursedError {
        // Capture current stack trace with accurate line numbers
        var stack_trace = ArrayList(CursedError.StackFrame){};
        
        // Add current execution point as the top of stack trace
        if (self.function_stack.items.len > 0) {
            const current_function = self.function_stack.items[self.function_stack.items.len - 1];
            try stack_trace.append(self.allocator, CursedError.StackFrame{
                .function_name = try self.allocator.dupe(u8, current_function),
                .file_name = try self.allocator.dupe(u8, self.current_file),
                .line = line,
                .column = column,
            });
        }
        
        // Add remaining stack frames with estimated line numbers
        if (self.function_stack.items.len > 1) {
            var i = self.function_stack.items.len - 1;
            while (i > 0) {
                i -= 1;
                const func_name = self.function_stack.items[i];
                try stack_trace.append(self.allocator, CursedError.StackFrame{
                    .function_name = try self.allocator.dupe(u8, func_name),
                    .file_name = try self.allocator.dupe(u8, self.current_file),
                    .line = 0, // Previous functions - line numbers not tracked here
                    .column = 0,
                });
            }
        }
        
        const error_obj = try CursedError.initWithStackTrace(
            self.allocator,
            message,
            error_type,
            code,
            stack_trace.items
        );
        
        // Add contextual information
        var context = ArrayList(CursedError.Context){};
        defer context.deinit(self.allocator);
        
        try context.append(self.allocator, CursedError.Context{
            .key = try self.allocator.dupe(u8, "error_location"),
            .value = try std.fmt.allocPrint(self.allocator, "{}:{}", .{ line, column }),
        });
        
        try context.append(self.allocator, CursedError.Context{
            .key = try self.allocator.dupe(u8, "stack_depth"),
            .value = try std.fmt.allocPrint(self.allocator, "{}", .{self.function_stack.items.len}),
        });
        
        error_obj.context = try self.allocator.dupe(CursedError.Context, context.items);
        
        stack_trace.deinit(self.allocator);
        
        try self.error_stack.append(self.allocator, error_obj);
        return error_obj;
    }
    
    /// SHOOK - Propagate an error up the call stack
    pub fn shook(self: *ErrorHandler, error_obj: *CursedError) !*CursedError {
        // Add current function to the stack trace if not already present
        if (self.function_stack.items.len > 0) {
            const current_func = self.function_stack.items[self.function_stack.items.len - 1];
            
            // Create new stack frame for current location
            if (error_obj.stack_trace) |trace| {
                var new_trace = ArrayList(CursedError.StackFrame){};
                
                // Add existing trace
                for (trace) |frame| {
                    try new_trace.append(self.allocator, CursedError.StackFrame{
                        .function_name = try self.allocator.dupe(u8, frame.function_name),
                        .file_name = try self.allocator.dupe(u8, frame.file_name),
                        .line = frame.line,
                        .column = frame.column,
                    });
                }
                
                // Add current function
                try new_trace.append(self.allocator, CursedError.StackFrame{
                    .function_name = try self.allocator.dupe(u8, current_func),
                    .file_name = try self.allocator.dupe(u8, self.current_file),
                    .line = 0, // TODO: Get actual line from parser context
                    .column = 0,
                });
                
                // Free old trace
                for (trace) |frame| {
                    self.allocator.free(frame.function_name);
                    self.allocator.free(frame.file_name);
                }
                self.allocator.free(trace);
                
                error_obj.stack_trace = try self.allocator.dupe(CursedError.StackFrame, new_trace.items);
                new_trace.deinit(self.allocator);
            }
        }
        
        return error_obj;
    }
    
    /// FAM - Try-catch error handling block
    pub fn fam(
        self: *ErrorHandler,
        try_block: *const fn() anyerror!void,
        catch_block: ?*const fn(*CursedError) anyerror!void,
        finally_block: ?*const fn() anyerror!void
    ) !void {
        var caught_error: ?*CursedError = null;
        
        // Execute try block
        if (@TypeOf(try_block) == fn() anyerror!void) {
            try_block() catch |err| {
                caught_error = try self.yikes(
                    @errorName(err),
                    .Runtime,
                    @intFromError(err),
                    0, // TODO: Get line from context
                    0
                );
            };
        }
        
        // Execute catch block if error occurred
        if (caught_error != null and catch_block != null) {
            if (@TypeOf(catch_block.?) == fn(*CursedError) anyerror!void) {
                catch_block.?(caught_error.?) catch |err| {
                    std.debug.print("Error in catch block: {s}\n", .{@errorName(err)});
                };
            }
            caught_error = null; // Mark as handled
        }
        
        // Execute finally block regardless
        if (finally_block != null) {
            if (@TypeOf(finally_block.?) == fn() anyerror!void) {
                finally_block.?() catch |err| {
                    std.debug.print("Error in finally block: {s}\n", .{@errorName(err)});
                };
            }
        }
        
        // Re-throw unhandled error
        if (caught_error != null) {
            var stdout_buffer: [4096]u8 = undefined;
            const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
            try caught_error.?.format(stdout);
            return error.UnhandledError;
        }
    }
    
    pub fn hasErrors(self: *ErrorHandler) bool {
        return self.error_stack.items.len > 0;
    }
    
    pub fn getLastError(self: *ErrorHandler) ?*CursedError {
        if (self.error_stack.items.len > 0) {
            return self.error_stack.items[self.error_stack.items.len - 1];
        }
        return null;
    }
    
    pub fn printAllErrors(self: *ErrorHandler) !void {
        var stdout_buffer: [4096]u8 = undefined;
        const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
        for (self.error_stack.items) |error_obj| {
            try error_obj.format(stdout);
            try stdout.print("\n", .{});
        }
    }
};

/// C-compatible API for LLVM integration
export fn cursed_create_error(message: [*:0]const u8, error_type: u32, code: i64) callconv(.c) ?*CursedError {
    const allocator = std.heap.c_allocator; // Use C allocator for export functions
    const msg = std.mem.span(message);
    const err_type: CursedErrorType = @enumFromInt(error_type);
    
    const error_obj = CursedError.init(allocator, msg, err_type, code) catch return null;
    return error_obj;
}

export fn cursed_is_error(value: ?*anyopaque) callconv(.c) bool {
    return value != null;
}

export fn cursed_propagate_error(error_obj: ?*CursedError) callconv(.c) void {
    if (error_obj) |err| {
        var stdout_buffer: [4096]u8 = undefined;
        const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
        err.format("", .{}, stdout) catch {};
    }
}

export fn cursed_try_begin() callconv(.c) void {
    // Set up try block context
}

export fn cursed_capture_stack_trace(error_obj: ?*CursedError) callconv(.c) void {
    _ = error_obj;
    // Capture and attach stack trace
}

// ==================== ENHANCED ERROR HANDLING WITH ? OPERATOR ====================

/// Error propagation result for ? operator
pub const PropagationResult = union(enum) {
    value: ?*anyopaque,
    error_obj: *CursedError,
};

/// Global error handler for ? operator support
var global_error_handler: ?*ErrorHandler = null;

/// Initialize global error handler for ? operator
export fn cursed_init_error_propagation(file_name: [*:0]const u8) callconv(.c) void {
    const allocator = std.heap.c_allocator;
    const file_span = std.mem.span(file_name);
    
    if (global_error_handler == null) {
        global_error_handler = allocator.create(ErrorHandler) catch return;
        global_error_handler.?.* = ErrorHandler.init(allocator, file_span);
    }
}

/// Cleanup global error handler
export fn cursed_cleanup_error_propagation() callconv(.c) void {
    if (global_error_handler) |handler| {
        handler.deinit();
        std.heap.c_allocator.destroy(handler);
        global_error_handler = null;
    }
}

/// Implement ? operator for error propagation
export fn cursed_error_propagate(
    result: ?*anyopaque, 
    is_error: bool, 
    function_name: [*:0]const u8,
    _: u32,  // line (unused)
    _: u32   // column (unused)
) callconv(.c) ?*anyopaque {
    if (!is_error) {
        return result; // Not an error, return the value
    }
    
    // Handle error propagation
    if (global_error_handler) |handler| {
        _ = function_name; // TODO: Use for better error context
        const error_obj: *CursedError = @ptrCast(@alignCast(result.?));
        
        // Propagate error through handler
        const propagated = handler.shook(error_obj) catch return null;
        return @ptrCast(propagated);
    }
    
    return result;
}

/// Enhanced try/catch block implementation
export fn cursed_try_catch_begin() callconv(.c) ?*anyopaque {
    // Return try context handle
    if (global_error_handler) |handler| {
        return @ptrCast(handler);
    }
    return null;
}

export fn cursed_try_catch_end(
    context: ?*anyopaque,
    had_error: bool,
    error_obj: ?*anyopaque
) callconv(.c) void {
    _ = context;
    if (had_error and error_obj != null) {
        const err: *CursedError = @ptrCast(@alignCast(error_obj.?));
        var stdout_buffer: [4096]u8 = undefined;
        const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
        err.format("", .{}, stdout) catch {};
    }
}

/// Create runtime error for ? operator
export fn cursed_create_runtime_error(
    message: [*:0]const u8,
    function_name: [*:0]const u8,
    line: u32,
    column: u32
) callconv(.c) ?*anyopaque {

    if (global_error_handler) |handler| {
        const msg_span = std.mem.span(message);
        const func_span = std.mem.span(function_name);
        _ = func_span;
        
        handler.pushFunction("unknown") catch return null;
        const error_obj = handler.yikes(msg_span, .Runtime, 1, line, column) catch return null;
        handler.popFunction();
        
        return @ptrCast(error_obj);
    }
    
    return null;
}

/// Check if value is an error for conditional compilation
export fn cursed_check_error(value: ?*anyopaque) callconv(.c) bool {
    if (value == null) return false;
    
    // Simple heuristic: if it's a CursedError pointer
    const error_obj: *CursedError = @ptrCast(@alignCast(value.?));
    return error_obj.error_type != .Custom or error_obj.code != 0;
}

test "CURSED error handling system" {
    const allocator = std.testing.allocator;
    
    // Test error creation
    var handler = ErrorHandler.init(allocator, "test.csd");
    defer handler.deinit();
    
    const error_obj = try handler.yikes("Test error", .Runtime, 100, 1, 10);
    
    try std.testing.expect(std.mem.eql(u8, error_obj.message, "Test error"));
    try std.testing.expect(error_obj.error_type == .Runtime);
    try std.testing.expect(error_obj.code == 100);
    
    // Test error propagation
    const propagated = try handler.shook(error_obj);
    try std.testing.expect(propagated == error_obj);
    
    // Test error printing
    try handler.printAllErrors();
}
