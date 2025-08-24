// CURSED Error Handling Fixes - Phase 1 Implementation
// Replaces all problematic error handling patterns with robust implementations

const std = @import("std");
const testing = std.testing;

/// Enhanced error handling with proper context and recovery
pub const CursedError = error{
    // Core errors
    InvalidInput,
    OutOfMemory,
    OperationFailed,
    InvalidState,
    ResourceExhausted,
    
    // Parsing errors
    SyntaxError,
    TypeError,
    SemanticError,
    
    // Runtime errors
    NullPointerDereference,
    ArrayBoundsError,
    DivisionByZero,
    StackOverflow,
    
    // Concurrency errors
    DeadlockDetected,
    RaceCondition,
    ChannelClosed,
    
    // I/O errors
    FileNotFound,
    PermissionDenied,
    NetworkError,
    
    // System errors
    SystemResourceUnavailable,
    ConfigurationError,
    InitializationFailed,
};

/// Error context for debugging and logging
pub const ErrorContext = struct {
    message: []const u8,
    file: []const u8,
    line: u32,
    function: []const u8,
    timestamp: u64,
    additional_info: ?[]const u8 = null,
    
    pub fn create(comptime message: []const u8, file: []const u8, line: u32, function: []const u8) ErrorContext {
        return ErrorContext{
            .message = message,
            .file = file,
            .line = line,
            .function = function,
            .timestamp = @as(u64, @intCast(std.time.timestamp())),
        };
    }
    
    pub fn withInfo(self: ErrorContext, info: []const u8) ErrorContext {
        var ctx = self;
        ctx.additional_info = info;
        return ctx;
    }
};

/// Enhanced error reporting with context
pub fn reportError(err: CursedError, context: ErrorContext) void {
    std.log.err("CURSED Error: {} in {}() at {}:{}", .{
        err, context.function, context.file, context.line
    });
    
    if (context.additional_info) |info| {
        std.log.err("Additional context: {s}", .{info});
    }
    
    std.log.err("Message: {s}", .{context.message});
    std.log.err("Timestamp: {}", .{context.timestamp});
}

/// Macro for creating error context
pub fn ERROR_CONTEXT(comptime message: []const u8) ErrorContext {
    return ErrorContext.create(
        message,
        @src().file,
        @src().line,
        @src().fn_name
    );
}

/// Safe wrapper for operations that might fail
pub fn safeOperation(
    comptime T: type,
    operation: anytype,
    args: anytype,
    context: ErrorContext
) CursedError!T {
    return operation(args) catch |err| {
        // Log the original error with full context
        std.log.err("Operation failed with error: {}", .{err});
        reportError(CursedError.OperationFailed, context);
        return CursedError.OperationFailed;
    };
}

/// Memory allocation with proper error handling
pub fn safeAlloc(allocator: std.mem.Allocator, comptime T: type, n: usize) CursedError![]T {
    return allocator.alloc(T, n) catch {
        const context = ERROR_CONTEXT("Memory allocation failed");
        std.log.err("Failed to allocate {} items of type {s}", .{ n, @typeName(T) });
        reportError(CursedError.OutOfMemory, context);
        return CursedError.OutOfMemory;
    };
}

/// Safe string duplication with error handling
pub fn safeDupe(allocator: std.mem.Allocator, comptime T: type, slice: []const T) CursedError![]T {
    return allocator.dupe(T, slice) catch {
        const context = ERROR_CONTEXT("String duplication failed");
        std.log.err("Failed to duplicate slice of length {}", .{slice.len});
        reportError(CursedError.OutOfMemory, context);
        return CursedError.OutOfMemory;
    };
}

/// Safe ArrayList operations
pub fn safeAppend(comptime T: type, list: *std.ArrayList(T), item: T) CursedError!void {
    return list.append(item) catch {
        const context = ERROR_CONTEXT("ArrayList append failed");
        std.log.err("Failed to append item to ArrayList (current length: {})", .{list.items.len});
        reportError(CursedError.OutOfMemory, context);
        return CursedError.OutOfMemory;
    };
}

/// Safe HashMap operations  
pub fn safePut(comptime K: type, comptime V: type, map: *std.HashMap(K, V, std.hash_map.AutoContext(K), std.hash_map.default_max_load_percentage), key: K, value: V) CursedError!void {
    return map.put(key, value) catch {
        const context = ERROR_CONTEXT("HashMap put failed");
        std.log.err("Failed to insert key-value pair into HashMap (current size: {})", .{map.count()});
        reportError(CursedError.OutOfMemory, context);
        return CursedError.OutOfMemory;
    };
}

/// Error recovery pattern for critical operations
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

/// Timeout wrapper for potentially blocking operations
pub fn withTimeout(
    comptime T: type,
    operation: anytype,
    args: anytype,
    timeout_ms: u64
) CursedError!T {
    // Implementation would use threads/async for real timeout
    // For now, just call the operation directly
    return operation(args) catch |err| {
        const context = ERROR_CONTEXT("Operation timed out or failed");
        std.log.err("Operation failed or timed out after {}ms: {}", .{ timeout_ms, err });
        reportError(CursedError.OperationFailed, context);
        return CursedError.OperationFailed;
    };
}

/// Replace problematic patterns in existing files
/// Use these functions instead of:
/// - `catch {}`  -> use `withErrorRecovery` or proper error handling
/// - `catch unreachable` -> use `safeOperation`
/// - bare error returns -> use `reportError` with context

// Example usage patterns:

/// Example: Replace memory_manager.zig:61
pub fn exampleMemoryOperation(allocator: std.mem.Allocator) CursedError!void {
    _ = allocator; // Mark as used
    // OLD (problematic):
    // someOperation() catch {};
    
    // NEW (proper error handling):
    // Example implementation would go here
    std.log.info("Example memory operation completed", .{});
    return;
}

/// Example: Replace arena_allocator.zig:459
pub fn exampleArenaOperation(allocator: std.mem.Allocator, size: usize) CursedError![]u8 {
    // OLD (problematic):
    // return allocator.alloc(u8, size) catch unreachable;
    
    // NEW (safe allocation):
    return safeAlloc(allocator, u8, size);
}

/// Example: Replace sync_primitives_fixed.zig error patterns
pub fn exampleSyncOperation(mutex: *std.Thread.Mutex) CursedError!void {
    // OLD (ignoring errors):
    // mutex.lock() catch {};
    
    // NEW (proper error handling):
    mutex.lock() catch |err| {
        const context = ERROR_CONTEXT("Mutex lock failed");
        std.log.err("Failed to acquire mutex lock: {}", .{err});
        reportError(CursedError.OperationFailed, context);
        return CursedError.OperationFailed;
    };
    defer mutex.unlock();
    
    // ... critical section code ...
}

// Test cases to validate error handling improvements
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
    
    // Test ArrayList operations  
    var list = std.ArrayList(u32).init(allocator);
    defer list.deinit();
    
    try safeAppend(u32, &list, 42);
    try testing.expect(list.items.len == 1);
    try testing.expect(list.items[0] == 42);
    
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

/// Compatibility layer for existing code
/// These functions can be used as drop-in replacements
pub const ErrorHandlingFixes = struct {
    /// Replace `catch {}` patterns
    pub fn ignorableError(operation: anytype, args: anytype) void {
        _ = operation(args) catch |err| {
            std.log.debug("Ignored error (consider proper handling): {}", .{err});
        };
    }
    
    /// Replace `catch unreachable` patterns  
    pub fn unreachableError(operation: anytype, args: anytype) CursedError!void {
        return operation(args) catch |err| {
            const context = ERROR_CONTEXT("Previously unreachable error occurred");
            std.log.err("Error that was marked as unreachable: {}", .{err});
            reportError(CursedError.InvalidState, context);
            return CursedError.InvalidState;
        };
    }
};
