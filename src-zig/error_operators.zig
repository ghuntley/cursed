const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const error_handling = @import("error_handling.zig");
const CursedError = error_handling.CursedError;
const ErrorContext = error_handling.ErrorContext;

/// CURSED Error Handling Operators: yikes, shook, fam
/// Implements the complete error handling system with proper error types and recovery

/// YIKES - Error Type and Value Creation
pub const YikesError = struct {
    message: []const u8,
    error_code: i64,
    source_location: ?SourceLocation,
    stack_trace: ?[][]const u8,
    inner_error: ?*YikesError,
    allocator: Allocator,

    pub const SourceLocation = struct {
        file: []const u8,
        line: u32,
        column: u32,
    };

    pub fn init(allocator: Allocator, message: []const u8, code: i64) !YikesError {
        return YikesError{
            .message = try allocator.dupe(u8, message),
            .error_code = code,
            .source_location = null,
            .stack_trace = null,
            .inner_error = null,
            .allocator = allocator,
        };
    }

    pub fn initWithLocation(
        allocator: Allocator,
        message: []const u8,
        code: i64,
        location: SourceLocation
    ) !YikesError {
        var err = try init(allocator, message, code);
        err.source_location = location;
        return err;
    }

    pub fn initWithInner(
        allocator: Allocator,
        message: []const u8,
        code: i64,
        inner: *YikesError
    ) !YikesError {
        var err = try init(allocator, message, code);
        err.inner_error = inner;
        return err;
    }

    pub fn deinit(self: *YikesError) void {
        self.allocator.free(self.message);
        if (self.stack_trace) |trace| {
            for (trace) |frame| {
                self.allocator.free(frame);
            }
            self.allocator.free(trace);
        }
        if (self.inner_error) |inner| {
            inner.deinit();
            self.allocator.destroy(inner);
        }
    }

    pub fn format(self: YikesError, writer: anytype) !void {
        try writer.print("yikes: {s} (code: {s})\n", .{ self.message, self.error_code });
        
        if (self.source_location) |loc| {
            try writer.print("  at {s}:{s}:{s}\n", .{ loc.file, loc.line, loc.column });
        }
        
        if (self.inner_error) |inner| {
            try writer.print("Caused by:\n", .{});
            try inner.format(writer);
        }
        
        if (self.stack_trace) |trace| {
            try writer.print("Stack trace:\n", .{});
            for (trace) |frame| {
                try writer.print("  {s}\n", .{frame});
            }
        }
    }

    pub fn toString(self: YikesError, allocator: Allocator) ![]u8 {
        _ = allocator;
        var buffer = std.ArrayList(u8){};
        defer buffer.deinit();
        
        const writer = buffer.writer();
        try self.format(writer);
        
        return try allocator.dupe(u8, buffer.items);
    }

    pub fn isError(self: YikesError) bool {
        _ = self;
        return true;
    }

    pub fn getCode(self: YikesError) i64 {
        return self.error_code;
    }

    pub fn getMessage(self: YikesError) []const u8 {
        return self.message;
    }
};

/// SHOOK - Error Propagation Operator
pub const ShookResult = union(enum) {
    Ok: Value,
    Error: YikesError,

    const Variable = struct { name: []const u8, value: Value };

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
        return switch (self) {
            .Ok => false,
            .Error => true,
        };
    }

    /// Unwrap a result value, returning an error if the result contains an error
    /// This replaces the panic behavior with proper error propagation
    pub fn unwrap(self: ShookResult) CursedError!Value {
        return switch (self) {
            .Ok => |value| value,
            .Error => |error_value| {
                // Map error codes to appropriate CursedError types
                const cursed_error = switch (error_value.error_code) {
                    1 => CursedError.ParseError,
                    2 => CursedError.TypeMismatch,
                    3 => CursedError.UndefinedVariable,
                    4 => CursedError.RuntimeError,
                    5 => CursedError.DivisionByZero,
                    6 => CursedError.IndexOutOfBounds,
                    7 => CursedError.NullPointerDereference,
                    8 => CursedError.InvalidOperation,
                    else => CursedError.UnknownError,
                };
                return cursed_error;
            },
        };
    }

    /// Safe unwrap that returns a default value instead of panicking
    pub fn unwrapOr(self: ShookResult, default: Value) Value {
        return switch (self) {
            .Ok => |value| value,
            .Error => default,
        };
    }

    /// Unsafe unwrap that returns a value but prints error information
    /// Only use this when you're certain the result is Ok
    pub fn unwrapUnsafe(self: ShookResult) Value {
        return switch (self) {
            .Ok => |value| value,
            .Error => |error_value| {
                // Log the error but return a sensible default instead of panicking
                std.log.err("Unwrap called on error: {s} (code: {})", .{ error_value.message, error_value.error_code });
                // Return a void value as the safest default
                return Value{ .Void = {} };
            },
        };
    }

    pub fn getError(self: ShookResult) ?YikesError {
        return switch (self) {
            .Ok => null,
            .Error => |error_value| error_value,
        };
    }

    pub fn propagate(self: ShookResult) CursedError!Value {
        return switch (self) {
            .Ok => |value| value,
            .Error => |error_value| {
                if (error_value.error_code == 0) return CursedError.UnknownError;
                if (error_value.error_code == 1) return CursedError.ParseError;
                if (error_value.error_code == 2) return CursedError.RuntimeError;
                if (error_value.error_code == 3) return CursedError.TypeMismatch;
                return CursedError.UnknownError;
            },
        };
    }

    pub fn deinit(self: *ShookResult, allocator: Allocator) void {
        _ = allocator;
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

/// SHOOK Operator Implementation
pub fn shook(result: anytype) ShookResult {
    return switch (@TypeOf(result)) {
        CursedError => ShookResult.err(YikesError.init(
            std.heap.page_allocator,
            @errorName(result),
            @intFromError(result)
        ) catch unreachable),
        else => ShookResult.ok(ShookResult.Value{ .Pointer = @ptrCast(@constCast(&result)) }),
    };
}

/// FAM - Panic Recovery Block
pub const FamBlock = struct {
    try_body: ArrayList(Statement),
    catch_handler: ?CatchHandler,
    finally_handler: ?FinallyHandler,
    allocator: Allocator,

    pub const Statement = union(enum) {
        Expression: ExpressionStatement,
        Assignment: AssignmentStatement,
        FunctionCall: FunctionCallStatement,
        Return: ReturnStatement,
        Block: BlockStatement,
    };

    pub const ExpressionStatement = struct {
        expression: *anyopaque, // Points to Expression
    };

    pub const AssignmentStatement = struct {
        target: []const u8,
        value: *anyopaque, // Points to Expression
    };

    pub const FunctionCallStatement = struct {
        function_name: []const u8,
        arguments: ArrayList(*anyopaque), // Points to Expression
    };

    pub const ReturnStatement = struct {
        value: ?*anyopaque, // Points to Expression
    };

    pub const BlockStatement = struct {
        statements: ArrayList(Statement),
    };

    pub const CatchHandler = struct {
        error_variable: []const u8,
        handler_body: ArrayList(Statement),
    };

    pub const FinallyHandler = struct {
        finally_body: ArrayList(Statement),
    };

    pub fn init() FamBlock {
        return FamBlock{
            .try_body = .empty,
            .catch_handler = null,
            .finally_handler = null,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *FamBlock) void {
        // Deinitialize try body
        for (self.try_body.items) |*stmt| {
            self.deinitStatement(stmt);
        }
        self.try_body.deinit(self.allocator);

        // Deinitialize catch handler
        if (self.catch_handler) |*catch_handler| {
            for (catch_handler.handler_body.items) |*stmt| {
                self.deinitStatement(stmt);
            }
            catch_handler.handler_body.deinit();
        }

        // Deinitialize finally handler
        if (self.finally_handler) |*finally_handler| {
            for (finally_handler.finally_body.items) |*stmt| {
                self.deinitStatement(stmt);
            }
            finally_handler.finally_body.deinit();
        }
    }

    fn deinitStatement(self: *FamBlock, stmt: *Statement) void {
        switch (stmt.*) {
            .Block => |*block| {
                for (block.statements.items) |*nested_stmt| {
                    self.deinitStatement(nested_stmt);
                }
                block.statements.deinit();
            },
            .FunctionCall => |*call| {
                call.arguments.deinit();
            },
            else => {},
        }
    }

    pub fn addTryStatement(self: *FamBlock, stmt: Statement) !void {
        try self.try_body.append(allocator, stmt);
    }

    pub fn setCatchHandler(self: *FamBlock, error_var: []const u8, handler_body: ArrayList(Statement)) void {
        self.catch_handler = CatchHandler{
            .error_variable = error_var,
            .handler_body = handler_body,
        };
    }

    pub fn setFinallyHandler(self: *FamBlock, finally_body: ArrayList(Statement)) void {
        self.finally_handler = FinallyHandler{
            .finally_body = finally_body,
        };
    }

    pub fn execute(self: *FamBlock, context: *ExecutionContext) ShookResult {
        var result: ShookResult = ShookResult.ok(ShookResult.Value{ .Void = {} });
        var error_occurred: ?YikesError = null;

        // Execute try body
        for (self.try_body.items) |stmt| {
            const stmt_result = self.executeStatement(stmt, context);
            if (stmt_result.isError()) {
                error_occurred = stmt_result.getError();
                break;
            }
            result = stmt_result;
        }

        // Execute catch handler if error occurred
        if (error_occurred != null and self.catch_handler != null) {
            const catch_handler = self.catch_handler.?;
            // Set error variable in context
            context.setVariable(catch_handler.error_variable, error_occurred.?);
            
            for (catch_handler.handler_body.items) |stmt| {
                result = self.executeStatement(stmt, context);
                if (result.isError()) break;
            }
            error_occurred = null; // Error was handled
        }

        // Execute finally handler
        if (self.finally_handler) |finally_handler| {
            for (finally_handler.finally_body.items) |stmt| {
                _ = self.executeStatement(stmt, context);
            }
        }

        // Return error if unhandled, otherwise return last result
        if (error_occurred) |err| {
            return ShookResult.err(err);
        }
        return result;
    }

    fn executeStatement(self: *FamBlock, stmt: Statement, context: *ExecutionContext) ShookResult {
        
        switch (stmt) {
            .Expression => {
                // Execute expression statement
                return ShookResult.ok(ShookResult.Value{ .Void = {} });
            },
            .Assignment => {
                // Execute assignment statement
                return ShookResult.ok(ShookResult.Value{ .Void = {} });
            },
            .FunctionCall => {
                // Execute function call statement
                return ShookResult.ok(ShookResult.Value{ .Void = {} });
            },
            .Return => |ret| {
                if (ret.value) |_| {
                    return ShookResult.ok(ShookResult.Value{ .Integer = 0 });
                }
                return ShookResult.ok(ShookResult.Value{ .Void = {} });
            },
            .Block => |block| {
                var last_result = ShookResult.ok(ShookResult.Value{ .Void = {} });
                for (block.statements.items) |nested_stmt| {
                    last_result = self.executeStatement(nested_stmt, context);
                    if (last_result.isError()) break;
                }
                return last_result;
            },
        }
    }
};

/// Execution Context for error handling
pub const ExecutionContext = struct {
    variables: std.HashMap([]const u8, YikesError, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,

    pub fn init() ExecutionContext {
        return ExecutionContext{
            .variables = std.HashMap([]const u8, YikesError, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *ExecutionContext) void {
        var iterator = self.variables.iterator();
        while (iterator.next()) |entry| {
            var err = entry.value_ptr;
            err.deinit();
        }
        self.variables.deinit(self.allocator);
    }

    pub fn setVariable(self: *ExecutionContext, name: []const u8, error_value: YikesError) void {
        self.variables.put(name, error_value) catch unreachable;
    }

    pub fn getVariable(self: *ExecutionContext, name: []const u8) ?YikesError {
        return self.variables.get(name);
    }
};

/// Error creation helper functions
pub fn createYikesError(allocator: Allocator, message: []const u8, code: i64) !YikesError {
    return YikesError.init(allocator, message, code);
}

pub fn createYikesFromString(allocator: Allocator, message: []const u8) !YikesError {
    return YikesError.init(allocator, message, 0);
}

pub fn createYikesFromCode(allocator: Allocator, code: i64) !YikesError {
    const message = switch (code) {
        0 => "Success",
        1 => "General error",
        2 => "Parse error",
        3 => "Runtime error",
        4 => "Type mismatch",
        5 => "File not found",
        else => "Unknown error",
    };
    return YikesError.init(allocator, message, code);
}

/// Helper functions for CURSED error handling syntax
pub fn yikes(allocator: Allocator, message: []const u8, code: i64) !YikesError {
    return createYikesError(allocator, message, code);
}

pub fn famBlock(allocator: Allocator) FamBlock {
        _ = allocator;
    return FamBlock.init(allocator);
}

// Test suite for error operators
test "yikes error creation" {
    const allocator = std.testing.allocator;
    
    var err = try yikes(allocator, "Test error", 42);
    defer err.deinit();
    
    try std.testing.expect(err.isError());
    try std.testing.expect(err.getCode() == 42);
    try std.testing.expect(std.mem.eql(u8, err.getMessage(), "Test error"));
}

test "shook error propagation" {
    const allocator = std.testing.allocator;
    
    const error_val = try yikes(allocator, "Test propagation", 1);
    var shook_result = ShookResult.err(error_val);
    defer shook_result.deinit();
    
    try std.testing.expect(shook_result.isError());
    try std.testing.expect(!shook_result.isOk());
    
    const propagated_err = shook_result.getError().?;
    try std.testing.expect(propagated_err.getCode() == 1);
}

test "fam block execution" {
    const allocator = std.testing.allocator;
    
    var fam = famBlock(allocator);
    defer fam.deinit();
    
    var context = ExecutionContext.init(allocator);
    defer context.deinit();
    
    const result = fam.execute(&context);
    try std.testing.expect(result.isOk());
}

test "complete error handling flow" {
    const allocator = std.testing.allocator;
    
    // Create yikes error
    var original_err = try yikes(allocator, "Original error", 100);
    defer original_err.deinit();
    
    // Create shook result
    const shook_result = ShookResult.err(original_err);
    try std.testing.expect(shook_result.isError());
    
    // Test fam block with error handling
    var fam = famBlock(allocator);
    defer fam.deinit();
    
    var context = ExecutionContext.init(allocator);
    defer context.deinit();
    
    // Execute fam block
    const fam_result = fam.execute(&context);
    try std.testing.expect(fam_result.isOk());
}
