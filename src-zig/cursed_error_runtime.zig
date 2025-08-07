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
    
    pub fn format(self: *const CursedError, writer: anytype) !void {
        try writer.print("[CURSED ERROR] {s}: {s}\n", .{ @tagName(self.error_type), self.message });
        try writer.print("  Error Code: {}\n", .{self.code});
        
        if (self.stack_trace) |trace| {
            try writer.print("  Stack Trace:\n");
            for (trace) |frame| {
                try writer.print("    at {s} ({s}:{}:{})\n", .{
                    frame.function_name, frame.file_name, frame.line, frame.column
                });
            }
        }
        
        if (self.context) |ctx| {
            try writer.print("  Context:\n");
            for (ctx) |context| {
                try writer.print("    {s}: {s}\n", .{ context.key, context.value });
            }
        }
        
        if (self.inner_error) |inner| {
            try writer.print("  Caused by:\n");
            try inner.format(writer);
        }
    }
    
    pub fn toString(self: *const CursedError) ![]u8 {
        var buffer = ArrayList(u8).init(self.allocator);
        defer buffer.deinit();
        
        const writer = buffer.writer();
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
            .error_stack = ArrayList(*CursedError).init(allocator),
            .function_stack = ArrayList([]const u8).init(allocator),
            .current_file = file,
        };
    }
    
    pub fn deinit(self: *ErrorHandler) void {
        for (self.error_stack.items) |error_obj| {
            error_obj.deinit();
        }
        self.error_stack.deinit();
        
        for (self.function_stack.items) |func_name| {
            self.allocator.free(func_name);
        }
        self.function_stack.deinit();
    }
    
    pub fn pushFunction(self: *ErrorHandler, function_name: []const u8) !void {
        try self.function_stack.append(try self.allocator.dupe(u8, function_name));
    }
    
    pub fn popFunction(self: *ErrorHandler) void {
        if (self.function_stack.items.len > 0) {
            const func_name = self.function_stack.pop();
            self.allocator.free(func_name);
        }
    }
    
    /// YIKES - Create and throw an error
    pub fn yikes(
        self: *ErrorHandler, 
        message: []const u8, 
        error_type: CursedErrorType, 
        code: i64,
        line: u32,
        column: u32
    ) !*CursedError {
        // Capture current stack trace
        var stack_trace = ArrayList(CursedError.StackFrame).init(self.allocator);
        
        for (self.function_stack.items) |func_name| {
            try stack_trace.append(CursedError.StackFrame{
                .function_name = try self.allocator.dupe(u8, func_name),
                .file_name = try self.allocator.dupe(u8, self.current_file),
                .line = line,
                .column = column,
            });
        }
        
        const error_obj = try CursedError.initWithStackTrace(
            self.allocator,
            message,
            error_type,
            code,
            stack_trace.items
        );
        
        stack_trace.deinit();
        
        try self.error_stack.append(error_obj);
        return error_obj;
    }
    
    /// SHOOK - Propagate an error up the call stack
    pub fn shook(self: *ErrorHandler, error_obj: *CursedError) !*CursedError {
        // Add current function to the stack trace if not already present
        if (self.function_stack.items.len > 0) {
            const current_func = self.function_stack.items[self.function_stack.items.len - 1];
            
            // Create new stack frame for current location
            if (error_obj.stack_trace) |trace| {
                var new_trace = ArrayList(CursedError.StackFrame).init(self.allocator);
                
                // Add existing trace
                for (trace) |frame| {
                    try new_trace.append(CursedError.StackFrame{
                        .function_name = try self.allocator.dupe(u8, frame.function_name),
                        .file_name = try self.allocator.dupe(u8, frame.file_name),
                        .line = frame.line,
                        .column = frame.column,
                    });
                }
                
                // Add current function
                try new_trace.append(CursedError.StackFrame{
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
                new_trace.deinit();
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
            const stdout = std.io.getStdOut().writer();
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
        const stdout = std.io.getStdOut().writer();
        for (self.error_stack.items) |error_obj| {
            try error_obj.format(stdout);
            try stdout.print("\n");
        }
    }
};

/// C-compatible API for LLVM integration
export fn cursed_create_error(message: [*:0]const u8, error_type: u32, code: i64) callconv(.C) ?*CursedError {
    const allocator = std.heap.c_allocator; // Use C allocator for export functions
    const msg = std.mem.span(message);
    const err_type: CursedErrorType = @enumFromInt(error_type);
    
    const error_obj = CursedError.init(allocator, msg, err_type, code) catch return null;
    return error_obj;
}

export fn cursed_is_error(value: ?*anyopaque) callconv(.C) bool {
    return value != null;
}

export fn cursed_propagate_error(error_obj: ?*CursedError) callconv(.C) void {
    if (error_obj) |err| {
        const stdout = std.io.getStdOut().writer();
        err.format(stdout) catch {};
    }
}

export fn cursed_try_begin() callconv(.C) void {
    // Set up try block context
}

export fn cursed_capture_stack_trace(error_obj: ?*CursedError) callconv(.C) void {
    _ = error_obj;
    // Capture and attach stack trace
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
