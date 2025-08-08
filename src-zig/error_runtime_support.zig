const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const ErrorContext = @import("error_handling.zig").ErrorContext;
const CursedError = @import("error_handling.zig").CursedError;
const ErrorPropagation = @import("error_propagation.zig").ErrorPropagation;

/// Runtime support for error propagation in CURSED
/// These functions are called by LLVM-generated code
var global_allocator: ?Allocator = null;
var global_error_propagator: ?*ErrorPropagation = null;

/// Initialize the error runtime system
export fn cursed_error_runtime_init(allocator_ptr: ?*anyopaque) void {
    if (allocator_ptr) |ptr| {
        global_allocator = @as(*Allocator, @ptrCast(@alignCast(ptr))).*;
        
        // Initialize global error propagator
        const allocator = global_allocator.?;
        global_error_propagator = allocator.create(ErrorPropagation) catch null;
        if (global_error_propagator) |prop| {
            prop.* = ErrorPropagation.init(allocator);
        }
    }
}

/// Cleanup the error runtime system
export fn cursed_error_runtime_deinit() void {
    if (global_error_propagator) |prop| {
        if (global_allocator) |allocator| {
            prop.deinit();
            allocator.destroy(prop);
        }
        global_error_propagator = null;
    }
    global_allocator = null;
}

/// Create a yikes error (called by LLVM IR)
export fn cursed_create_yikes_error(message_ptr: [*:0]const u8, code: i64) ?*anyopaque {
    const allocator = global_allocator orelse return null;
    const message = std.mem.span(message_ptr);
    
    const location = ErrorContext.SourceLocation{
        .file = "generated",
        .line = 0,
        .column = 0,
    };
    
    const error_ctx = ErrorContext.initWithLocation(
        allocator,
        CursedError.RuntimeError,
        message,
        location
    ) catch return null;
    
    // Allocate space for the error context
    const error_ptr = allocator.create(ErrorContext) catch return null;
    error_ptr.* = error_ctx;
    
    return @ptrCast(error_ptr);
}

/// Check if a value is an error (called by LLVM IR)
export fn cursed_is_error(value_ptr: ?*anyopaque) bool {
    if (value_ptr == null) return false;
    
    // In a real implementation, this would check the value's type tag
    // For now, we assume all non-null pointers are potentially errors
    // This is a placeholder implementation
    _ = value_ptr;
    return false;  // TODO: Implement proper error value checking
}

/// Propagate an error (called by LLVM IR)
export fn cursed_propagate_error(error_ptr: ?*anyopaque) void {
    if (error_ptr == null) return;
    
    const error_ctx = @as(*ErrorContext, @ptrCast(@alignCast(error_ptr)));
    
    if (global_error_propagator) |prop| {
        // Use the error propagation system
        const should_continue = prop.propagateError(error_ctx.*, true) catch false;
        if (!should_continue) {
            // Print error and exit/return
            const stdout = std.io.getStdOut().writer();
            stdout.print("Fatal error: ") catch {};
            error_ctx.format(stdout) catch {};
            // In a real implementation, this might call exit() or longjmp()
        }
    } else {
        // Fallback: print error
        const stdout = std.io.getStdOut().writer();
        stdout.print("Error propagation system not initialized: ") catch {};
        error_ctx.format(stdout) catch {};
    }
}

/// Begin a try block (called by LLVM IR for fam statements)
export fn cursed_try_begin() void {
    // Set up try block context
    // In a real implementation, this might use setjmp/longjmp or exception tables
    
    // For now, this is a placeholder
    if (global_error_propagator) |prop| {
        // Mark the beginning of a try block
        // This would typically involve stack management
        _ = prop;
    }
}

/// End a try block (called by LLVM IR)
export fn cursed_try_end() void {
    // Clean up try block context
    // Placeholder implementation
    if (global_error_propagator) |prop| {
        // Clean up any try block state
        _ = prop;
    }
}

/// Enter a catch block with specific error type (called by LLVM IR)
export fn cursed_catch_enter(error_type_ptr: [*:0]const u8) bool {
    if (global_error_propagator == null) return false;
    
    const error_type = std.mem.span(error_type_ptr);
    
    // Check if the current error matches the catch block type
    if (global_error_propagator.?.getCurrentError()) |current_error| {
        // Simple type matching - in real implementation would be more sophisticated
        const error_name = @errorName(current_error.error_code);
        return std.mem.eql(u8, error_name, error_type);
    }
    
    return false;
}

/// Get current error message for catch block variable binding (called by LLVM IR)
export fn cursed_get_error_message() ?[*:0]const u8 {
    if (global_error_propagator) |prop| {
        if (prop.getCurrentError()) |current_error| {
            // Return the error message
            // Note: This should be a null-terminated string for C compatibility
            return current_error.message.ptr;
        }
    }
    return null;
}

/// Clear the current error (called when error is handled)
export fn cursed_clear_current_error() void {
    if (global_error_propagator) |prop| {
        // Remove the top error from the stack
        if (prop.error_stack.items.len > 0) {
            var error_ctx = prop.error_stack.pop();
            error_ctx.deinit();
        }
    }
}

/// Format error for debugging (called by LLVM IR)
export fn cursed_format_error(error_ptr: ?*anyopaque, buffer: [*]u8, buffer_size: usize) usize {
    if (error_ptr == null) return 0;
    
    const error_ctx = @as(*ErrorContext, @ptrCast(@alignCast(error_ptr)));
    
    var fbs = std.io.fixedBufferStream(buffer[0..buffer_size]);
    const writer = fbs.writer();
    
    error_ctx.format(writer) catch return 0;
    
    return fbs.pos;
}

/// Create error with detailed context (called by LLVM IR)
export fn cursed_create_error_with_context(
    message_ptr: [*:0]const u8,
    file_ptr: [*:0]const u8,
    line: u32,
    column: u32,
    error_type: i32
) ?*anyopaque {
    const allocator = global_allocator orelse return null;
    
    const message = std.mem.span(message_ptr);
    const file = std.mem.span(file_ptr);
    
    const location = ErrorContext.SourceLocation{
        .file = file,
        .line = line,
        .column = column,
    };
    
    const cursed_error = switch (error_type) {
        0 => CursedError.RuntimeError,
        1 => CursedError.ParseError,
        2 => CursedError.TypeMismatch,
        3 => CursedError.DivisionByZero,
        4 => CursedError.UndefinedVariable,
        5 => CursedError.OutOfMemory,
        else => CursedError.UnknownError,
    };
    
    const error_ctx = ErrorContext.initWithLocation(
        allocator,
        cursed_error,
        message,
        location
    ) catch return null;
    
    const error_ptr = allocator.create(ErrorContext) catch return null;
    error_ptr.* = error_ctx;
    
    return @ptrCast(error_ptr);
}

/// Add error to propagation stack
export fn cursed_add_error_to_stack(error_ptr: ?*anyopaque) void {
    if (error_ptr == null) return;
    if (global_error_propagator == null) return;
    
    const error_ctx = @as(*ErrorContext, @ptrCast(@alignCast(error_ptr)));
    
    // Add to error stack
    global_error_propagator.?.error_stack.append(error_ctx.*) catch {};
}

/// Check if we're in a try block
export fn cursed_in_try_block() bool {
    if (global_error_propagator) |prop| {
        return prop.try_catch_stack.items.len > 0;
    }
    return false;
}

/// Get error stack depth
export fn cursed_error_stack_depth() usize {
    if (global_error_propagator) |prop| {
        return prop.error_stack.items.len;
    }
    return 0;
}

/// Print error stack for debugging
export fn cursed_print_error_stack() void {
    if (global_error_propagator) |prop| {
        const stdout = std.io.getStdOut().writer();
        prop.formatErrorStack(stdout) catch {};
    }
}

test "error runtime support" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Test runtime initialization
    cursed_error_runtime_init(@ptrCast(&allocator));
    defer cursed_error_runtime_deinit();
    
    // Test error creation
    const error_ptr = cursed_create_yikes_error("Test error", 500);
    try std.testing.expect(error_ptr != null);
    
    // Test error checking
    const is_err = cursed_is_error(error_ptr);
    _ = is_err; // Placeholder test
    
    // Test error stack depth
    const depth_before = cursed_error_stack_depth();
    cursed_add_error_to_stack(error_ptr);
    const depth_after = cursed_error_stack_depth();
    try std.testing.expect(depth_after > depth_before);
}
