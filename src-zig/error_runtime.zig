const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

/// CURSED Error Handling Runtime
/// Implements yikes (throw) and fam (catch) error handling system with stack unwinding

pub const ErrorValue = struct {
    message: []const u8,
    error_type: []const u8,
    stack_trace: ArrayList(StackFrame),
    location: SourceLocation,
    allocator: Allocator,
    
    pub const SourceLocation = struct {
        file: []const u8,
        line: u32,
        column: u32,
    };
    
    pub const StackFrame = struct {
        function_name: []const u8,
        file: []const u8,
        line: u32,
        column: u32,
    };
    
    pub fn init(allocator: Allocator, message: []const u8, error_type: []const u8, location: SourceLocation) !ErrorValue {
        return ErrorValue{
            .message = try allocator.dupe(u8, message),
            .error_type = try allocator.dupe(u8, error_type),
            .stack_trace = ArrayList(StackFrame).init(allocator),
            .location = SourceLocation{
                .file = try allocator.dupe(u8, location.file),
                .line = location.line,
                .column = location.column,
            },
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *ErrorValue) void {
        self.allocator.free(self.message);
        self.allocator.free(self.error_type);
        self.allocator.free(self.location.file);
        
        for (self.stack_trace.items) |frame| {
            self.allocator.free(frame.function_name);
            self.allocator.free(frame.file);
        }
        self.stack_trace.deinit();
    }
    
    pub fn addStackFrame(self: *ErrorValue, function_name: []const u8, file: []const u8, line: u32, column: u32) !void {
        try self.stack_trace.append(StackFrame{
            .function_name = try self.allocator.dupe(u8, function_name),
            .file = try self.allocator.dupe(u8, file),
            .line = line,
            .column = column,
        });
    }
    
    pub fn format(self: ErrorValue, writer: anytype) !void {
        try writer.print("Error: {s}\n", .{self.message});
        try writer.print("Type: {s}\n", .{self.error_type});
        try writer.print("Location: {s}:{}:{}\n", .{ self.location.file, self.location.line, self.location.column });
        
        if (self.stack_trace.items.len > 0) {
            try writer.print("Stack trace:\n");
            for (self.stack_trace.items) |frame| {
                try writer.print("  at {s} ({s}:{}:{})\n", .{ frame.function_name, frame.file, frame.line, frame.column });
            }
        }
    }
};

pub const ErrorHandler = struct {
    allocator: Allocator,
    current_error: ?ErrorValue,
    error_stack: ArrayList(ErrorValue),
    function_stack: ArrayList([]const u8),
    try_catch_stack: ArrayList(TryCatchFrame),
    
    pub const TryCatchFrame = struct {
        try_block_start: usize,
        catch_block_start: ?usize,
        error_variable: ?[]const u8,
        handler_type: HandlerType,
        
        pub const HandlerType = enum {
            Catch,
            Finally,
            Both,
        };
    };
    
    pub fn init(allocator: Allocator) ErrorHandler {
        return ErrorHandler{
            .allocator = allocator,
            .current_error = null,
            .error_stack = ArrayList(ErrorValue).init(allocator),
            .function_stack = ArrayList([]const u8).init(allocator),
            .try_catch_stack = ArrayList(TryCatchFrame).init(allocator),
        };
    }
    
    pub fn deinit(self: *ErrorHandler) void {
        if (self.current_error) |*err| {
            err.deinit();
        }
        
        for (self.error_stack.items) |*err| {
            err.deinit();
        }
        self.error_stack.deinit();
        
        for (self.function_stack.items) |func_name| {
            self.allocator.free(func_name);
        }
        self.function_stack.deinit();
        
        for (self.try_catch_stack.items) |frame| {
            if (frame.error_variable) |var_name| {
                self.allocator.free(var_name);
            }
        }
        self.try_catch_stack.deinit();
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
    
    pub fn pushTryCatch(self: *ErrorHandler, try_block_start: usize, error_variable: ?[]const u8, handler_type: TryCatchFrame.HandlerType) !void {
        try self.try_catch_stack.append(TryCatchFrame{
            .try_block_start = try_block_start,
            .catch_block_start = null,
            .error_variable = if (error_variable) |var_name| try self.allocator.dupe(u8, var_name) else null,
            .handler_type = handler_type,
        });
    }
    
    pub fn popTryCatch(self: *ErrorHandler) void {
        if (self.try_catch_stack.items.len > 0) {
            const frame = self.try_catch_stack.pop();
            if (frame.error_variable) |var_name| {
                self.allocator.free(var_name);
            }
        }
    }
    
    pub fn yikes(self: *ErrorHandler, message: []const u8, error_type: []const u8, file: []const u8, line: u32, column: u32) !void {
        var error_value = try ErrorValue.init(self.allocator, message, error_type, ErrorValue.SourceLocation{
            .file = file,
            .line = line,
            .column = column,
        });
        
        // Add current function stack to error
        for (self.function_stack.items) |func_name| {
            try error_value.addStackFrame(func_name, file, line, column);
        }
        
        if (self.current_error) |*current| {
            current.deinit();
        }
        
        self.current_error = error_value;
        
        // Propagate error up the stack
        try self.propagateError();
    }
    
    pub fn hasError(self: *ErrorHandler) bool {
        return self.current_error != null;
    }
    
    pub fn getError(self: *ErrorHandler) ?*ErrorValue {
        if (self.current_error) |*err| {
            return err;
        }
        return null;
    }
    
    pub fn clearError(self: *ErrorHandler) void {
        if (self.current_error) |*err| {
            err.deinit();
            self.current_error = null;
        }
    }
    
    pub fn handleError(self: *ErrorHandler, error_variable_name: ?[]const u8) !bool {
        if (self.current_error == null) {
            return false;
        }
        
        // If we're in a try-catch block, handle the error
        if (self.try_catch_stack.items.len > 0) {
            const frame = &self.try_catch_stack.items[self.try_catch_stack.items.len - 1];
            
            // Store error in catch variable if specified
            if (error_variable_name) |var_name| {
                if (frame.error_variable == null) {
                    frame.error_variable = try self.allocator.dupe(u8, var_name);
                }
            }
            
            return true;
        }
        
        return false;
    }
    
    fn propagateError(self: *ErrorHandler) !void {
        // If we're not in a try-catch block, unwind the stack
        if (self.try_catch_stack.items.len == 0) {
            // No error handler, terminate with error message
            if (self.current_error) |err| {
                const stderr = std.io.getStdErr().writer();
                try err.format(stderr);
                std.process.exit(1);
            }
        }
    }
    
    pub fn formatCurrentError(self: *ErrorHandler, writer: anytype) !void {
        if (self.current_error) |err| {
            try err.format(writer);
        } else {
            try writer.print("No current error\n", .{});
        }
    }
};

// C-compatible runtime functions for LLVM IR
export fn cursed_error_handler_init() *ErrorHandler {
    const allocator = std.heap.c_allocator;
    const handler = allocator.create(ErrorHandler) catch |err| {
        std.debug.panic("Failed to create error handler: {}\n", .{err});
    };
    handler.* = ErrorHandler.init(allocator);
    return handler;
}

export fn cursed_error_handler_deinit(handler: *ErrorHandler) void {
    const allocator = handler.allocator;
    handler.deinit();
    allocator.destroy(handler);
}

export fn cursed_yikes(handler: *ErrorHandler, message: [*:0]const u8, error_type: [*:0]const u8, file: [*:0]const u8, line: u32, column: u32) void {
    const msg = std.mem.span(message);
    const err_type = std.mem.span(error_type);
    const file_name = std.mem.span(file);
    
    handler.yikes(msg, err_type, file_name, line, column) catch |err| {
        std.debug.panic("Failed to create error: {}\n", .{err});
    };
}

export fn cursed_has_error(handler: *ErrorHandler) bool {
    return handler.hasError();
}

export fn cursed_handle_error(handler: *ErrorHandler, error_variable: ?[*:0]const u8) bool {
    const var_name = if (error_variable) |var_ptr| std.mem.span(var_ptr) else null;
    return handler.handleError(var_name) catch false;
}

export fn cursed_clear_error(handler: *ErrorHandler) void {
    handler.clearError();
}

export fn cursed_push_function(handler: *ErrorHandler, function_name: [*:0]const u8) void {
    const func_name = std.mem.span(function_name);
    handler.pushFunction(func_name) catch |err| {
        std.debug.panic("Failed to push function: {}\n", .{err});
    };
}

export fn cursed_pop_function(handler: *ErrorHandler) void {
    handler.popFunction();
}

export fn cursed_push_try_catch(handler: *ErrorHandler, try_block_start: usize, error_variable: ?[*:0]const u8, handler_type: u32) void {
    const var_name = if (error_variable) |var_ptr| std.mem.span(var_ptr) else null;
    const h_type: ErrorHandler.TryCatchFrame.HandlerType = switch (handler_type) {
        0 => .Catch,
        1 => .Finally,
        2 => .Both,
        else => .Catch,
    };
    
    handler.pushTryCatch(try_block_start, var_name, h_type) catch |err| {
        std.debug.panic("Failed to push try-catch: {}\n", .{err});
    };
}

export fn cursed_pop_try_catch(handler: *ErrorHandler) void {
    handler.popTryCatch();
}

test "error handling runtime" {
    const allocator = std.testing.allocator;
    var handler = ErrorHandler.init(allocator);
    defer handler.deinit();
    
    // Test function stack
    try handler.pushFunction("main");
    try handler.pushFunction("test_function");
    
    // Test error creation
    try handler.yikes("Test error message", "RuntimeError", "test.csd", 10, 5);
    
    try std.testing.expect(handler.hasError());
    
    const error_value = handler.getError().?;
    try std.testing.expectEqualStrings("Test error message", error_value.message);
    try std.testing.expectEqualStrings("RuntimeError", error_value.error_type);
    try std.testing.expect(error_value.stack_trace.items.len == 2);
    
    handler.clearError();
    try std.testing.expect(!handler.hasError());
    
    handler.popFunction();
    handler.popFunction();
}
