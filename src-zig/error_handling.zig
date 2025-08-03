const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

/// Comprehensive error handling system for CURSED Zig compiler
/// Replaces all @panic() calls with proper error propagation

pub const CursedError = error{
    // Memory errors
    OutOfMemory,
    InvalidAllocation,
    MemoryCorruption,
    
    // Parsing errors
    ParseError,
    UnexpectedToken,
    UnexpectedEndOfFile,
    InvalidSyntax,
    MissingToken,
    
    // Compilation errors
    CompilationError,
    TypeMismatch,
    UndefinedSymbol,
    DuplicateDefinition,
    CircularDependency,
    
    // Runtime errors
    RuntimeError,
    UndefinedVariable,
    UndefinedFunction,
    UndefinedStruct,
    UndefinedInterface,
    UndefinedField,
    UndefinedMethod,
    DivisionByZero,
    NullPointerDereference,
    IndexOutOfBounds,
    InvalidOperation,
    
    // File/IO errors
    FileNotFound,
    PermissionDenied,
    ReadError,
    WriteError,
    InvalidPath,
    
    // Interface/struct errors
    InterfaceNotImplemented,
    InvalidStructField,
    MissingInterface,
    
    // Concurrency errors
    ChannelClosed,
    DeadlockDetected,
    RaceCondition,
    ThreadError,
    
    // System errors
    SystemError,
    PlatformNotSupported,
    InvalidConfiguration,
    
    // Generic catch-all
    UnknownError,
};

pub const ErrorContext = struct {
    message: []const u8,
    location: ?SourceLocation,
    stack_trace: ?[][]const u8,
    error_code: CursedError,
    inner_error: ?*ErrorContext,
    allocator: Allocator,
    
    pub const SourceLocation = struct {
        file: []const u8,
        line: u32,
        column: u32,
    };
    
    pub fn init(allocator: Allocator, error_code: CursedError, message: []const u8) !ErrorContext {
        return ErrorContext{
            .message = try allocator.dupe(u8, message),
            .location = null,
            .stack_trace = null,
            .error_code = error_code,
            .inner_error = null,
            .allocator = allocator,
        };
    }
    
    pub fn initWithLocation(
        allocator: Allocator, 
        error_code: CursedError, 
        message: []const u8,
        location: SourceLocation
    ) !ErrorContext {
        var ctx = try init(allocator, error_code, message);
        ctx.location = location;
        return ctx;
    }
    
    pub fn initWithInner(
        allocator: Allocator,
        error_code: CursedError,
        message: []const u8,
        inner: *ErrorContext
    ) !ErrorContext {
        var ctx = try init(allocator, error_code, message);
        ctx.inner_error = inner;
        return ctx;
    }
    
    pub fn deinit(self: *ErrorContext) void {
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
    
    pub fn format(self: ErrorContext, writer: anytype) !void {
        try writer.print("Error: {s} - {s}\n", .{ @errorName(self.error_code), self.message });
        
        if (self.location) |loc| {
            try writer.print("  at {s}:{}:{}\n", .{ loc.file, loc.line, loc.column });
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
    
    pub fn toString(self: ErrorContext, allocator: Allocator) ![]u8 {
        var buffer = ArrayList(u8).init(allocator);
        defer buffer.deinit();
        
        const writer = buffer.writer();
        try self.format(writer);
        
        return try allocator.dupe(u8, buffer.items);
    }
};

/// Safe memory allocation with proper error handling
pub fn safeAlloc(allocator: Allocator, comptime T: type, count: usize) CursedError![]T {
    return allocator.alloc(T, count) catch |err| switch (err) {
        error.OutOfMemory => CursedError.OutOfMemory,
    };
}

/// Safe memory duplication with proper error handling
pub fn safeDupe(allocator: Allocator, comptime T: type, data: []const T) CursedError![]T {
    return allocator.dupe(T, data) catch |err| switch (err) {
        error.OutOfMemory => CursedError.OutOfMemory,
    };
}

/// Safe string duplication with proper error handling
pub fn safeDupeString(allocator: Allocator, str: []const u8) CursedError![]u8 {
    return safeDupe(allocator, u8, str);
}

/// Convert std.mem.Allocator.Error to CursedError
pub fn mapAllocatorError(err: Allocator.Error) CursedError {
    return switch (err) {
        error.OutOfMemory => CursedError.OutOfMemory,
    };
}

/// Convert std.fs.File.OpenError to CursedError
pub fn mapFileOpenError(err: std.fs.File.OpenError) CursedError {
    return switch (err) {
        error.FileNotFound => CursedError.FileNotFound,
        error.AccessDenied => CursedError.PermissionDenied,
        error.InvalidUtf8 => CursedError.InvalidPath,
        error.BadPathName => CursedError.InvalidPath,
        error.SymLinkLoop => CursedError.InvalidPath,
        error.ProcessFdQuotaExceeded => CursedError.SystemError,
        error.SystemFdQuotaExceeded => CursedError.SystemError,
        error.NoDevice => CursedError.SystemError,
        error.SystemResources => CursedError.SystemError,
        error.Unexpected => CursedError.UnknownError,
        else => CursedError.SystemError,
    };
}

/// Convert std.fs.File.ReadError to CursedError
pub fn mapFileReadError(err: std.fs.File.ReadError) CursedError {
    return switch (err) {
        error.InputOutput => CursedError.ReadError,
        error.SystemResources => CursedError.SystemError,
        error.IsDir => CursedError.InvalidOperation,
        error.OperationAborted => CursedError.SystemError,
        error.BrokenPipe => CursedError.ReadError,
        error.ConnectionResetByPeer => CursedError.ReadError,
        error.ConnectionTimedOut => CursedError.SystemError,
        error.NotOpenForReading => CursedError.InvalidOperation,
        error.SocketNotConnected => CursedError.SystemError,
        error.Unexpected => CursedError.UnknownError,
        else => CursedError.ReadError,
    };
}

/// Convert std.fs.File.WriteError to CursedError
pub fn mapFileWriteError(err: std.fs.File.WriteError) CursedError {
    return switch (err) {
        error.DiskQuota => CursedError.SystemError,
        error.FileTooBig => CursedError.SystemError,
        error.InputOutput => CursedError.WriteError,
        error.NoSpaceLeft => CursedError.SystemError,
        error.DeviceBusy => CursedError.SystemError,
        error.InvalidArgument => CursedError.InvalidOperation,
        error.AccessDenied => CursedError.PermissionDenied,
        error.BrokenPipe => CursedError.WriteError,
        error.SystemResources => CursedError.SystemError,
        error.OperationAborted => CursedError.SystemError,
        error.NotOpenForWriting => CursedError.InvalidOperation,
        error.ConnectionResetByPeer => CursedError.WriteError,
        error.Unexpected => CursedError.UnknownError,
        else => CursedError.WriteError,
    };
}

/// Safe file reading with proper error handling
pub fn safeReadFile(allocator: Allocator, path: []const u8) CursedError![]u8 {
    const file = std.fs.cwd().openFile(path, .{}) catch |err| {
        return mapFileOpenError(err);
    };
    defer file.close();
    
    const file_size = file.getEndPos() catch {
        return CursedError.ReadError;
    };
    
    const contents = allocator.alloc(u8, file_size) catch |err| {
        return mapAllocatorError(err);
    };
    
    _ = file.readAll(contents) catch |err| {
        allocator.free(contents);
        return mapFileReadError(err);
    };
    
    return contents;
}

/// Safe file writing with proper error handling
pub fn safeWriteFile(allocator: Allocator, path: []const u8, contents: []const u8) CursedError!void {
    _ = allocator; // unused in this function but kept for consistency
    
    const file = std.fs.cwd().createFile(path, .{}) catch |err| {
        return mapFileOpenError(err);
    };
    defer file.close();
    
    file.writeAll(contents) catch |err| {
        return mapFileWriteError(err);
    };
}

/// Wrapper for operations that might panic - converts to errors
pub fn safePanic(comptime operation: anytype, args: anytype) CursedError!@TypeOf(operation(args)) {
    return operation(args) catch |err| switch (err) {
        error.OutOfMemory => CursedError.OutOfMemory,
        else => CursedError.UnknownError,
    };
}

/// Error recovery mechanism
pub const ErrorRecovery = struct {
    allocator: Allocator,
    errors: ArrayList(ErrorContext),
    max_errors: usize,
    
    pub fn init(allocator: Allocator, max_errors: usize) ErrorRecovery {
        return ErrorRecovery{
            .allocator = allocator,
            .errors = ArrayList(ErrorContext).init(allocator),
            .max_errors = max_errors,
        };
    }
    
    pub fn deinit(self: *ErrorRecovery) void {
        for (self.errors.items) |*error_ctx| {
            error_ctx.deinit();
        }
        self.errors.deinit();
    }
    
    pub fn addError(self: *ErrorRecovery, error_ctx: ErrorContext) !void {
        if (self.errors.items.len >= self.max_errors) {
            return CursedError.SystemError;
        }
        try self.errors.append(error_ctx);
    }
    
    pub fn hasErrors(self: *ErrorRecovery) bool {
        return self.errors.items.len > 0;
    }
    
    pub fn getErrors(self: *ErrorRecovery) []const ErrorContext {
        return self.errors.items;
    }
    
    pub fn printErrors(self: *ErrorRecovery, writer: anytype) !void {
        for (self.errors.items) |error_ctx| {
            try error_ctx.format(writer);
            try writer.print("\n");
        }
    }
};

test "error handling system" {
    const allocator = std.testing.allocator;
    
    // Test basic error context
    var ctx = try ErrorContext.init(allocator, CursedError.OutOfMemory, "Test error message");
    defer ctx.deinit();
    
    try std.testing.expect(ctx.error_code == CursedError.OutOfMemory);
    try std.testing.expect(std.mem.eql(u8, ctx.message, "Test error message"));
    
    // Test error recovery
    var recovery = ErrorRecovery.init(allocator, 10);
    defer recovery.deinit();
    
    const error1 = try ErrorContext.init(allocator, CursedError.ParseError, "Parse failed");
    try recovery.addError(error1);
    
    try std.testing.expect(recovery.hasErrors());
    try std.testing.expect(recovery.getErrors().len == 1);
    
    // Test safe string duplication
    const original = "test string";
    const duplicated = try safeDupeString(allocator, original);
    defer allocator.free(duplicated);
    
    try std.testing.expect(std.mem.eql(u8, original, duplicated));
}
