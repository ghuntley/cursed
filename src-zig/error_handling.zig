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
    PatternMatchFailed,
    StackOverflow,
    
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

/// Enhanced error recovery mechanism with runtime support
pub const ErrorRecovery = struct {
    allocator: Allocator,
    errors: ArrayList(ErrorContext),
    max_errors: usize,
    stack_traces: ArrayList([][]const u8),
    current_function_stack: ArrayList([]const u8),
    
    pub fn init(allocator: Allocator, max_errors: usize) ErrorRecovery {
        return ErrorRecovery{
            .allocator = allocator,
            .errors = ArrayList(ErrorContext).init(allocator),
            .max_errors = max_errors,
            .stack_traces = ArrayList([][]const u8).init(allocator),
            .current_function_stack = ArrayList([]const u8).init(allocator),
        };
    }
    
    pub fn deinit(self: *ErrorRecovery) void {
        for (self.errors.items) |*error_ctx| {
            error_ctx.deinit();
        }
        self.errors.deinit();
        
        for (self.stack_traces.items) |trace| {
            for (trace) |frame| {
                self.allocator.free(frame);
            }
            self.allocator.free(trace);
        }
        self.stack_traces.deinit();
        
        for (self.current_function_stack.items) |func_name| {
            self.allocator.free(func_name);
        }
        self.current_function_stack.deinit();
    }
    
    pub fn pushFunction(self: *ErrorRecovery, function_name: []const u8) !void {
        try self.current_function_stack.append(try self.allocator.dupe(u8, function_name));
    }
    
    pub fn popFunction(self: *ErrorRecovery) void {
        if (self.current_function_stack.items.len > 0) {
            _ = self.current_function_stack.pop();
            // Note: Memory will be freed when ErrorRecovery is deinitialized
        }
    }
    
    pub fn captureStackTrace(self: *ErrorRecovery) ![][]const u8 {
        var trace = try self.allocator.alloc([]const u8, self.current_function_stack.items.len);
        for (self.current_function_stack.items, 0..) |func_name, i| {
            trace[i] = try self.allocator.dupe(u8, func_name);
        }
        return trace;
    }
    
    pub fn addError(self: *ErrorRecovery, error_ctx: ErrorContext) !void {
        if (self.errors.items.len >= self.max_errors) {
            return CursedError.SystemError;
        }
        
        var ctx = error_ctx;
        // Capture current stack trace
        ctx.stack_trace = try self.captureStackTrace();
        
        try self.errors.append(ctx);
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
    
    pub fn createYikesError(self: *ErrorRecovery, message: []const u8, line: u32, column: u32, file: []const u8) !ErrorContext {
        const location = ErrorContext.SourceLocation{
            .file = try self.allocator.dupe(u8, file),
            .line = line,
            .column = column,
        };
        
        var ctx = try ErrorContext.initWithLocation(
            self.allocator,
            CursedError.RuntimeError,
            message,
            location
        );
        
        // Capture stack trace
        ctx.stack_trace = try self.captureStackTrace();
        
        return ctx;
    }
};

/// LLVM IR generation for error handling
pub const ErrorHandlingLLVM = struct {
    allocator: Allocator,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator) Self {
        return Self{
            .allocator = allocator,
        };
    }
    
    /// Generate LLVM IR for yikes (error creation) statement
    pub fn generateYikesLLVM(self: *Self, context: c.LLVMContextRef, module: c.LLVMModuleRef, builder: c.LLVMBuilderRef, message: []const u8) !c.LLVMValueRef {
        _ = self;
        
        // Declare runtime error creation function
        const create_error_func = c.LLVMGetNamedFunction(module, "cursed_create_error") orelse {
            const func_type = c.LLVMFunctionType(
                c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
                &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)},
                1,
                0
            );
            return c.LLVMAddFunction(module, "cursed_create_error", func_type);
        };
        
        // Create string constant for error message
        const message_str = c.LLVMBuildGlobalStringPtr(builder, message.ptr, "error_msg");
        
        // Call error creation function
        const error_obj = c.LLVMBuildCall2(
            builder,
            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
            create_error_func,
            &[_]c.LLVMValueRef{message_str},
            1,
            "error_obj"
        );
        
        return error_obj;
    }
    
    /// Generate LLVM IR for shook (error propagation) expression
    pub fn generateShookLLVM(self: *Self, context: c.LLVMContextRef, module: c.LLVMModuleRef, builder: c.LLVMBuilderRef, value: c.LLVMValueRef) !c.LLVMValueRef {
        _ = self;
        
        // Declare runtime error checking function
        const is_error_func = c.LLVMGetNamedFunction(module, "cursed_is_error") orelse {
            const func_type = c.LLVMFunctionType(
                c.LLVMInt1TypeInContext(context),
                &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)},
                1,
                0
            );
            return c.LLVMAddFunction(module, "cursed_is_error", func_type);
        };
        
        // Check if value is an error
        const is_error = c.LLVMBuildCall2(
            builder,
            c.LLVMInt1TypeInContext(context),
            is_error_func,
            &[_]c.LLVMValueRef{value},
            1,
            "is_error"
        );
        
        // Create blocks for error propagation and normal execution
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(builder));
        const error_block = c.LLVMAppendBasicBlockInContext(context, current_func, "error_propagate");
        const normal_block = c.LLVMAppendBasicBlockInContext(context, current_func, "normal_continue");
        
        // Branch based on error check
        _ = c.LLVMBuildCondBr(builder, is_error, error_block, normal_block);
        
        // Error propagation block
        c.LLVMPositionBuilderAtEnd(builder, error_block);
        
        // Call error propagation function
        const propagate_func = c.LLVMGetNamedFunction(module, "cursed_propagate_error") orelse {
            const func_type = c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(context),
                &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)},
                1,
                0
            );
            return c.LLVMAddFunction(module, "cursed_propagate_error", func_type);
        };
        
        _ = c.LLVMBuildCall2(
            builder,
            c.LLVMVoidTypeInContext(context),
            propagate_func,
            &[_]c.LLVMValueRef{value},
            1,
            ""
        );
        
        // Return error value
        _ = c.LLVMBuildRet(builder, value);
        
        // Continue in normal block
        c.LLVMPositionBuilderAtEnd(builder, normal_block);
        
        return value;
    }
    
    /// Generate LLVM IR for fam (error recovery) block
    pub fn generateFamLLVM(self: *Self, context: c.LLVMContextRef, module: c.LLVMModuleRef, builder: c.LLVMBuilderRef) !struct { 
        try_block: c.LLVMBasicBlockRef,
        catch_block: c.LLVMBasicBlockRef,
        end_block: c.LLVMBasicBlockRef,
    } {
        _ = self;
        
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(builder));
        
        // Create blocks for try/catch/end
        const try_block = c.LLVMAppendBasicBlockInContext(context, current_func, "fam_try");
        const catch_block = c.LLVMAppendBasicBlockInContext(context, current_func, "fam_catch");
        const end_block = c.LLVMAppendBasicBlockInContext(context, current_func, "fam_end");
        
        // Declare runtime try begin function
        const try_begin_func = c.LLVMGetNamedFunction(module, "cursed_try_begin") orelse {
            const func_type = c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(context),
                null,
                0,
                0
            );
            return c.LLVMAddFunction(module, "cursed_try_begin", func_type);
        };
        
        // Start try block
        _ = c.LLVMBuildCall2(
            builder,
            c.LLVMVoidTypeInContext(context),
            try_begin_func,
            null,
            0,
            ""
        );
        
        // Branch to try block
        _ = c.LLVMBuildBr(builder, try_block);
        
        return .{
            .try_block = try_block,
            .catch_block = catch_block,
            .end_block = end_block,
        };
    }
    
    /// Generate stack trace capture
    pub fn generateStackTraceLLVM(self: *Self, context: c.LLVMContextRef, module: c.LLVMModuleRef, builder: c.LLVMBuilderRef, error_obj: c.LLVMValueRef) !void {
        _ = self;
        
        // Declare runtime stack trace function
        const capture_trace_func = c.LLVMGetNamedFunction(module, "cursed_capture_stack_trace") orelse {
            const func_type = c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(context),
                &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)},
                1,
                0
            );
            return c.LLVMAddFunction(module, "cursed_capture_stack_trace", func_type);
        };
        
        // Call stack trace capture
        _ = c.LLVMBuildCall2(
            builder,
            c.LLVMVoidTypeInContext(context),
            capture_trace_func,
            &[_]c.LLVMValueRef{error_obj},
            1,
            ""
        );
    }
};

// Import LLVM C headers
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/ExecutionEngine.h");
});

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
