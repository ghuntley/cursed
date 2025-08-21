const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const ErrorContext = @import("error_handling.zig").ErrorContext;
const CursedError = @import("error_handling.zig").CursedError;
const ErrorPropagation = @import("error_propagation.zig").ErrorPropagation;

/// Try-catch context for stack management
const TryContext = struct {
    error_handler: ?*const fn(*ErrorContext) void,
    finally_handler: ?*const fn() void,
    scope_id: u32,
};

/// Defer entry for cleanup management
const DeferEntry = struct {
    cleanup_fn: *const fn() void,
    context_data: []const u8,
    scope_id: u32,
};

/// Enhanced defer stack with proper scope management
const DeferStack = struct {
    entries: ArrayList(DeferEntry),
    current_scope: u32,
    allocator: Allocator,
    
    fn init(allocator: Allocator) DeferStack {
        return DeferStack{
            .entries = .empty,
            .current_scope = 0,
            .allocator = allocator,
        };
    }
    
    fn deinit(self: *DeferStack) void {
        // Execute all remaining cleanup functions
        self.executeAll();
        self.entries.deinit();
    }
    
    fn enterScope(self: *DeferStack) !void {
        self.current_scope += 1;
    }
    
    fn exitScope(self: *DeferStack) void {
        // Execute all cleanup functions in current scope (LIFO order)
        var i = self.entries.items.len;
        while (i > 0) {
            i -= 1;
            const entry = self.entries.items[i];
            if (entry.scope_id == self.current_scope) {
                entry.cleanup_fn();
                self.allocator.free(entry.context_data);
                _ = self.entries.orderedRemove(i);
            }
        }
        
        if (self.current_scope > 0) {
            self.current_scope -= 1;
        }
    }
    
    fn executeAll(self: *DeferStack) void {
        while (self.entries.items.len > 0) {
            const entry = self.entries.pop();
            entry.cleanup_fn();
            self.allocator.free(entry.context_data);
        }
    }
    
    fn push(self: *DeferStack, cleanup_fn: *const fn() void, context: []const u8) !void {
        const context_copy = try self.allocator.dupe(u8, context);
        try self.entries.append(self.allocator, DeferEntry{
            .cleanup_fn = cleanup_fn,
            .context_data = context_copy,
            .scope_id = self.current_scope,
        });
    }
};

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
    
    // Check if the pointer points to an ErrorContext structure
    // We use a magic header pattern to identify error values
    const ERROR_MAGIC_HEADER: u32 = 0xCECE_DEAD; // "CURSED" error magic
    
    // Cast to potential error structure
    const potential_error = @as(*align(1) const [*]u8, @ptrCast(value_ptr.?))[0..@sizeOf(u32)];
    const magic_header = std.mem.readInt(u32, potential_error[0..4], .little);
    
    // Check if it matches our error magic header
    if (magic_header == ERROR_MAGIC_HEADER) {
        return true;
    }
    
    // Additional heuristic: check if it's a valid ErrorContext pointer
    const error_ctx: ?*ErrorContext = @alignCast(@ptrCast(value_ptr));
    if (error_ctx) |ctx| {
        // Validate that this looks like a proper ErrorContext
        if (ctx.message.len > 0 and ctx.message.len < 1024 * 1024) { // Reasonable message length
            return true;
        }
    }
    
    return false;
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
            var stdout_buffer: [4096]u8 = undefined;
            const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
            stdout.print("Fatal error: ") catch {};
            error_ctx.format(stdout) catch {};
            // In a real implementation, this might call exit() or longjmp()
        }
    } else {
        // Fallback: print error
        var stdout_buffer: [4096]u8 = undefined;
        const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
        stdout.print("Error propagation system not initialized: ") catch {};
        error_ctx.format(stdout) catch {};
    }
}

/// Begin a try block (called by LLVM IR for fam statements)
export fn cursed_try_begin() void {
    if (global_error_propagator) |prop| {
        // Push a new try-catch context onto the stack
        const try_context = TryContext{
            .error_handler = null,
            .finally_handler = null,
            .scope_id = prop.current_scope_id,
        };
        prop.try_catch_stack.append(try_context) catch {};
        prop.current_scope_id += 1;
        
        // Mark beginning of defer scope for this try block
        prop.defer_entries.enterScope() catch {};
    }
}

/// End a try block (called by LLVM IR)
export fn cursed_try_end() void {
    if (global_error_propagator) |prop| {
        // Pop the try-catch context
        if (prop.try_catch_stack.items.len > 0) {
            var try_context = prop.try_catch_stack.pop();
            
            // Execute finally block if present
            if (try_context.finally_handler) |finally_fn| {
                finally_fn();
            }
        }
        
        // Exit defer scope and execute cleanup functions
        prop.defer_entries.exitScope();
        
        if (prop.current_scope_id > 0) {
            prop.current_scope_id -= 1;
        }
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
    const writer = fbs.writer(&[_]u8{});
    
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
        var stdout_buffer: [4096]u8 = undefined;
        const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
        prop.formatErrorStack(stdout) catch {};
    }
}

/// Register a defer cleanup function (called by LLVM IR)
export fn cursed_defer_register(cleanup_fn: *const fn() void, context_ptr: [*:0]const u8) void {
    if (global_error_propagator) |prop| {
        const context = std.mem.span(context_ptr);
        prop.defer_entries.push(cleanup_fn, context) catch {};
    }
}

/// Set line number context for error reporting
export fn cursed_set_line_context(file_ptr: [*:0]const u8, line: u32, column: u32) void {
    // Store current execution context for error reporting
    if (global_error_propagator) |prop| {
        const file = std.mem.span(file_ptr);
        
        // Update current location in error propagator
        // This would be used when creating new errors
        prop.current_file = prop.allocator.dupe(u8, file) catch return;
        prop.current_line = line;
        prop.current_column = column;
    }
}

/// Execute error unwinding and cleanup
export fn cursed_unwind_error(error_ptr: ?*anyopaque, target_scope: u32) void {
    if (error_ptr == null or global_error_propagator == null) return;
    
    const error_ctx = @as(*ErrorContext, @ptrCast(@alignCast(error_ptr.?)));
    const prop = global_error_propagator.?;
    
    // Execute cleanup functions in reverse scope order until target scope
    while (prop.current_scope_id > target_scope) {
        prop.defer_entries.exitScope();
        if (prop.current_scope_id > 0) {
            prop.current_scope_id -= 1;
        }
    }
    
    // Log unwinding for debugging
    var stdout_buffer: [4096]u8 = undefined;
    const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
    stdout.print("Error unwound to scope {}, error: ", .{target_scope}) catch {};
    error_ctx.format(stdout) catch {};
}

/// Create error with full stack context (line number tracking)
export fn cursed_create_contextual_error(
    message_ptr: [*:0]const u8,
    file_ptr: [*:0]const u8,
    function_ptr: [*:0]const u8,
    line: u32,
    column: u32,
    error_type: i32
) ?*anyopaque {
    const allocator = global_allocator orelse return null;
    
    const message = std.mem.span(message_ptr);
    const file = std.mem.span(file_ptr);
    const function = std.mem.span(function_ptr);
    
    const location = ErrorContext.SourceLocation{
        .file = file,
        .line = line,
        .column = column,
        .function = function,
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
    
    // Add magic header for error identification
    const ERROR_MAGIC_HEADER: u32 = 0xCECE_DEAD;
    const header_ptr = @as(*u32, @ptrCast(@alignCast(error_ptr)));
    header_ptr.* = ERROR_MAGIC_HEADER;
    
    return @ptrCast(error_ptr);
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
