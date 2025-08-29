// CURSED Error Handling Fixes - Simplified Implementation
// Provides drop-in replacements for problematic error handling patterns

const std = @import("std");
const testing = std.testing;

/// Standard CURSED error types
pub const CursedError = error{
    InvalidInput,
    OutOfMemory,
    OperationFailed,
    InvalidState,
    ResourceExhausted,
    SyntaxError,
    TypeError,
    SemanticError,
    NullPointerDereference,
    ArrayBoundsError,
    DivisionByZero,
    StackOverflow,
    DeadlockDetected,
    RaceCondition,
    ChannelClosed,
    FileNotFound,
    PermissionDenied,
    NetworkError,
    SystemResourceUnavailable,
    ConfigurationError,
    InitializationFailed,
};

/// Error context for debugging
pub const ErrorContext = struct {
    message: []const u8,
    file: []const u8,
    line: u32,
    function: []const u8,
    
    pub fn create(comptime message: []const u8, file: []const u8, line: u32, function: []const u8) ErrorContext {
        return ErrorContext{
            .message = message,
            .file = file,
            .line = line,
            .function = function,
        };
    }
};

/// Enhanced error reporting
pub fn reportError(err: CursedError, context: ErrorContext) void {
    std.log.err("CURSED Error: {} in {s}() at {s}:{}", .{
        err, context.function, context.file, context.line
    });
    std.log.err("Message: {s}", .{context.message});
}

/// Create error context macro
pub fn ERROR_CONTEXT(comptime message: []const u8) ErrorContext {
    return ErrorContext.create(
        message,
        @src().file,
        @src().line,
        @src().fn_name
    );
}

/// Safe memory allocation
pub fn safeAlloc(allocator: std.mem.Allocator, comptime T: type, n: usize) CursedError![]T {
    return allocator.alloc(T, n) catch {
        const context = ERROR_CONTEXT("Memory allocation failed");
        std.log.err("Failed to allocate {} items of type {s}", .{ n, @typeName(T) });
        reportError(CursedError.OutOfMemory, context);
        return CursedError.OutOfMemory;
    };
}

/// Safe string duplication
pub fn safeDupe(allocator: std.mem.Allocator, comptime T: type, slice: []const T) CursedError![]T {
    return allocator.dupe(T, slice) catch {
        const context = ERROR_CONTEXT("String duplication failed");
        std.log.err("Failed to duplicate slice of length {}", .{slice.len});
        reportError(CursedError.OutOfMemory, context);
        return CursedError.OutOfMemory;
    };
}

/// Safe ArrayList append
pub fn safeAppend(comptime T: type, list: *std.ArrayList(T), item: T) CursedError!void {
    return list.append(allocator, item) catch {
        const context = ERROR_CONTEXT("ArrayList append failed");
        std.log.err("Failed to append item to ArrayList (current length: {})", .{list.items.len});
        reportError(CursedError.OutOfMemory, context);
        return CursedError.OutOfMemory;
    };
}

/// Safe HashMap put operation
pub fn safePut(comptime K: type, comptime V: type, map: anytype, key: K, value: V) CursedError!void {
    return map.put(key, value) catch {
        const context = ERROR_CONTEXT("HashMap put failed");
        std.log.err("Failed to insert key-value pair into HashMap", .{});
        reportError(CursedError.OutOfMemory, context);
        return CursedError.OutOfMemory;
    };
}

/// Error recovery with fallback
pub fn withErrorRecovery(
    comptime T: type,
    operation: anytype,
    args: anytype,
    fallback: T,
    context: ErrorContext
) T {
    return operation(args) catch |err| {
        std.log.warn("Operation failed, using fallback: {}", .{err});
        reportError(CursedError.OperationFailed, context);
        return fallback;
    };
}

/// Replacement patterns for existing problematic code
pub const ErrorHandlingFixes = struct {
    /// Replace `catch {}` patterns
    pub fn ignorableError(operation: anytype, args: anytype) void {
        _ = operation(args) catch |err| {
            std.log.debug("Ignored error (consider proper handling): {}", .{err});
        };
    }
    
    /// Replace `catch unreachable` patterns
    pub fn safeUnreachable(operation: anytype, args: anytype) CursedError!void {
        return operation(args) catch |err| {
            const context = ERROR_CONTEXT("Previously unreachable error occurred");
            std.log.err("Error that was marked as unreachable: {}", .{err});
            reportError(CursedError.InvalidState, context);
            return CursedError.InvalidState;
        };
    }
};

// Test the error handling improvements
test "error handling improvements" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    // Test safe allocation
    const memory = try safeAlloc(allocator, u8, 100);
    try testing.expect(memory.len == 100);
    
    // Test safe duplication
    const original = "test string";
    const duplicated = try safeDupe(allocator, u8, original);
    try testing.expectEqualStrings(original, duplicated);
    
    std.log.info("✅ Error handling improvements test passed", .{});
}

test "error context and reporting" {
    const context = ERROR_CONTEXT("Test error message");
    
    try testing.expect(context.line > 0);
    try testing.expect(context.message.len > 0);
    try testing.expect(context.file.len > 0);
    try testing.expect(context.function.len > 0);
    
    // Test error reporting (should not crash)
    reportError(CursedError.InvalidInput, context);
    
    std.log.info("✅ Error context and reporting test passed", .{});
}
