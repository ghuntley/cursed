// CURSED Signal Handling FFI Bindings
// Provides C-compatible API for CURSED signal handling integration
// Connects CURSED stdlib signal modules to Zig signal handling platform

const std = @import("std");
const signal_platform = @import("signal_handling_platform.zig");
const Allocator = std.mem.Allocator;

// Global signal handler instance
var global_signal_handler: ?*signal_platform.RealSignalHandler = null;
var global_allocator: ?Allocator = null;

// C-compatible error codes
pub const CURSED_SIGNAL_SUCCESS: c_int = 0;
pub const CURSED_SIGNAL_ERROR: c_int = -1;
pub const CURSED_SIGNAL_INVALID_ARG: c_int = -2;
pub const CURSED_SIGNAL_NOT_SUPPORTED: c_int = -3;
pub const CURSED_SIGNAL_TIMEOUT: c_int = -4;

// C-compatible signal context structure
pub const CursedSignalContext = extern struct {
    signal_num: c_int,
    sender_pid: c_int,
    timestamp: c_long,
    signal_value: c_int,
    errno_value: c_int,
    user_data: ?*anyopaque,
};

// C-compatible callback function type
pub const CursedSignalCallback = ?*const fn (signal: c_int, context: ?*CursedSignalContext) callconv(.C) void;

// Storage for registered callbacks
var signal_callbacks: [64]?CursedSignalCallback = [_]?CursedSignalCallback{null} ** 64;

// =============================================================================
// INITIALIZATION AND CLEANUP
// =============================================================================

// Initialize signal handling system
export fn cursed_signal_init() c_int {
    if (global_signal_handler != null) {
        return CURSED_SIGNAL_SUCCESS; // Already initialized
    }
    
    // Use GPA for signal handling
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    global_allocator = gpa.allocator();
    
    // Initialize signal handler
    global_signal_handler = global_allocator.?.create(signal_platform.RealSignalHandler) catch {
        return CURSED_SIGNAL_ERROR;
    };
    
    global_signal_handler.?.* = signal_platform.RealSignalHandler.init(global_allocator.?) catch {
        global_allocator.?.destroy(global_signal_handler.?);
        global_signal_handler = null;
        return CURSED_SIGNAL_ERROR;
    };
    
    return CURSED_SIGNAL_SUCCESS;
}

// Clean up signal handling system
export fn cursed_signal_cleanup() c_int {
    if (global_signal_handler) |handler| {
        handler.cleanup();
        handler.deinit();
        global_allocator.?.destroy(handler);
        global_signal_handler = null;
    }
    
    // Clear callbacks
    for (&signal_callbacks) |*callback| {
        callback.* = null;
    }
    
    return CURSED_SIGNAL_SUCCESS;
}

// =============================================================================
// SIGNAL REGISTRATION
// =============================================================================

// Signal handler wrapper that bridges Zig to C callback
fn signalHandlerWrapper(signal: signal_platform.SignalType, context: ?*signal_platform.SignalContext) void {
    const signal_num = @intFromEnum(signal);
    
    if (signal_num > 0 and signal_num <= 64) {
        if (signal_callbacks[signal_num - 1]) |callback| {
            // Convert context to C-compatible structure
            var c_context: CursedSignalContext = undefined;
            if (context) |ctx| {
                c_context = CursedSignalContext{
                    .signal_num = @intCast(signal_num),
                    .sender_pid = @intCast(ctx.sender_pid),
                    .timestamp = @intCast(ctx.timestamp),
                    .signal_value = ctx.signal_value,
                    .errno_value = 0, // Not available in our context
                    .user_data = ctx.user_data,
                };
                callback(signal_num, &c_context);
            } else {
                c_context = CursedSignalContext{
                    .signal_num = @intCast(signal_num),
                    .sender_pid = 0,
                    .timestamp = std.time.milliTimestamp(),
                    .signal_value = 0,
                    .errno_value = 0,
                    .user_data = null,
                };
                callback(signal_num, &c_context);
            }
        }
    }
}

// Register signal handler
export fn cursed_signal_register(signal_num: c_int, callback: CursedSignalCallback) c_int {
    if (global_signal_handler == null) {
        if (cursed_signal_init() != CURSED_SIGNAL_SUCCESS) {
            return CURSED_SIGNAL_ERROR;
        }
    }
    
    if (signal_num <= 0 or signal_num > 64) {
        return CURSED_SIGNAL_INVALID_ARG;
    }
    
    const signal_type = signal_platform.SignalType.fromUnixSignal(signal_num) orelse {
        return CURSED_SIGNAL_INVALID_ARG;
    };
    
    if (!signal_type.canBeCaught()) {
        return CURSED_SIGNAL_NOT_SUPPORTED;
    }
    
    // Store callback
    signal_callbacks[@intCast(signal_num - 1)] = callback;
    
    // Register with signal handler
    global_signal_handler.?.registerHandler(signal_type, signalHandlerWrapper) catch {
        signal_callbacks[@intCast(signal_num - 1)] = null;
        return CURSED_SIGNAL_ERROR;
    };
    
    return CURSED_SIGNAL_SUCCESS;
}

// Unregister signal handler
export fn cursed_signal_unregister(signal_num: c_int) c_int {
    if (global_signal_handler == null) {
        return CURSED_SIGNAL_ERROR;
    }
    
    if (signal_num <= 0 or signal_num > 64) {
        return CURSED_SIGNAL_INVALID_ARG;
    }
    
    const signal_type = signal_platform.SignalType.fromUnixSignal(signal_num) orelse {
        return CURSED_SIGNAL_INVALID_ARG;
    };
    
    // Clear callback
    signal_callbacks[@intCast(signal_num - 1)] = null;
    
    // Unregister from signal handler
    global_signal_handler.?.unregisterHandler(signal_type) catch {
        return CURSED_SIGNAL_ERROR;
    };
    
    return CURSED_SIGNAL_SUCCESS;
}

// =============================================================================
// SIGNAL MASKING
// =============================================================================

// Block signals using mask
export fn cursed_signal_block_mask(mask_value: c_long) c_int {
    if (global_signal_handler == null) {
        return CURSED_SIGNAL_ERROR;
    }
    
    var mask = signal_platform.SignalMask.init();
    mask.mask = @intCast(mask_value);
    
    global_signal_handler.?.blockSignals(mask) catch {
        return CURSED_SIGNAL_ERROR;
    };
    
    return CURSED_SIGNAL_SUCCESS;
}

// Unblock signals using mask
export fn cursed_signal_unblock_mask(mask_value: c_long) c_int {
    if (global_signal_handler == null) {
        return CURSED_SIGNAL_ERROR;
    }
    
    var mask = signal_platform.SignalMask.init();
    mask.mask = @intCast(mask_value);
    
    global_signal_handler.?.unblockSignals(mask) catch {
        return CURSED_SIGNAL_ERROR;
    };
    
    return CURSED_SIGNAL_SUCCESS;
}

// Check if signal is blocked
export fn cursed_signal_is_blocked(signal_num: c_int) c_int {
    if (global_signal_handler == null) {
        return CURSED_SIGNAL_ERROR;
    }
    
    if (signal_num <= 0 or signal_num > 64) {
        return CURSED_SIGNAL_INVALID_ARG;
    }
    
    const signal_type = signal_platform.SignalType.fromUnixSignal(signal_num) orelse {
        return CURSED_SIGNAL_INVALID_ARG;
    };
    
    // Check if signal is in current mask
    if (global_signal_handler.?.signal_mask.hasSignal(signal_type)) {
        return 1; // Blocked
    } else {
        return 0; // Not blocked
    }
}

// =============================================================================
// SIGNAL SENDING
// =============================================================================

// Send signal to process
export fn cursed_signal_send_to_process(pid: c_int, signal_num: c_int) c_int {
    if (pid <= 0 or signal_num <= 0 or signal_num > 64) {
        return CURSED_SIGNAL_INVALID_ARG;
    }
    
    const signal_type = signal_platform.SignalType.fromUnixSignal(signal_num) orelse {
        return CURSED_SIGNAL_INVALID_ARG;
    };
    
    signal_platform.RealSignalHandler.sendSignal(@intCast(pid), signal_type) catch {
        return CURSED_SIGNAL_ERROR;
    };
    
    return CURSED_SIGNAL_SUCCESS;
}

// Send signal to process group
export fn cursed_signal_send_to_group(pgid: c_int, signal_num: c_int) c_int {
    if (pgid <= 0 or signal_num <= 0 or signal_num > 64) {
        return CURSED_SIGNAL_INVALID_ARG;
    }
    
    const signal_type = signal_platform.SignalType.fromUnixSignal(signal_num) orelse {
        return CURSED_SIGNAL_INVALID_ARG;
    };
    
    // Send to process group (negative PID)
    signal_platform.RealSignalHandler.sendSignal(@intCast(-pgid), signal_type) catch {
        return CURSED_SIGNAL_ERROR;
    };
    
    return CURSED_SIGNAL_SUCCESS;
}

// =============================================================================
// SIGNAL WAITING AND CHECKING
// =============================================================================

// Wait for specific signal with timeout
export fn cursed_signal_wait_for_signal(timeout_ms: c_int) c_int {
    if (global_signal_handler == null) {
        return CURSED_SIGNAL_ERROR;
    }
    
    const timeout: ?u32 = if (timeout_ms < 0) null else @intCast(timeout_ms);
    
    const signal = global_signal_handler.?.waitForSignal(timeout) catch {
        return CURSED_SIGNAL_ERROR;
    };
    
    if (signal) |sig| {
        return @intFromEnum(sig);
    } else {
        return CURSED_SIGNAL_TIMEOUT;
    }
}

// Process pending signals
export fn cursed_signal_process_pending() c_int {
    if (global_signal_handler == null) {
        return CURSED_SIGNAL_ERROR;
    }
    
    global_signal_handler.?.processPendingSignals() catch {
        return CURSED_SIGNAL_ERROR;
    };
    
    return CURSED_SIGNAL_SUCCESS;
}

// =============================================================================
// SIGNAL SAFETY AND CONTEXT
// =============================================================================

// Check if currently in signal-safe context
export fn cursed_signal_is_signal_safe() c_int {
    if (global_signal_handler == null) {
        return 0; // Not initialized, assume not safe
    }
    
    return if (global_signal_handler.?.isInSignalHandler()) 1 else 0;
}

// Register cleanup handler
export fn cursed_signal_register_cleanup(cleanup_fn: ?*const fn () callconv(.C) void) c_int {
    if (global_signal_handler == null) {
        return CURSED_SIGNAL_ERROR;
    }
    
    if (cleanup_fn) |fn_ptr| {
        // Wrap C function for Zig calling convention
        const wrapped_fn = struct {
            var c_fn: *const fn () callconv(.C) void = undefined;
            
            fn wrapper() void {
                c_fn();
            }
        };
        
        wrapped_fn.c_fn = fn_ptr;
        
        global_signal_handler.?.registerCleanupHandler(wrapped_fn.wrapper) catch {
            return CURSED_SIGNAL_ERROR;
        };
    }
    
    return CURSED_SIGNAL_SUCCESS;
}

// =============================================================================
// LEGACY COMPATIBILITY FUNCTIONS
// =============================================================================

// These functions provide compatibility with the existing CURSED stdlib modules

export fn cursed_real_signal_init() c_int {
    return cursed_signal_init();
}

export fn cursed_real_signal_register(signal_num: c_int, handler_id: c_int) c_int {
    // Legacy function - handler_id is ignored, assuming callback is pre-registered
    _ = handler_id;
    // This would need additional logic to map handler_id to actual callback
    return CURSED_SIGNAL_SUCCESS;
}

export fn cursed_real_signal_unregister(signal_num: c_int) c_int {
    return cursed_signal_unregister(signal_num);
}

export fn cursed_real_signal_send(pid: c_int, signal_num: c_int) c_int {
    return cursed_signal_send_to_process(pid, signal_num);
}

export fn cursed_real_signal_wait_sync(timeout_ms: c_int) c_int {
    return cursed_signal_wait_for_signal(timeout_ms);
}

export fn cursed_real_signal_is_blocked(signal_num: c_int) c_int {
    return cursed_signal_is_blocked(signal_num);
}

export fn cursed_real_signal_cleanup() c_int {
    return cursed_signal_cleanup();
}

// Native signal handling functions for advanced signal modules
export fn cursed_native_signal_init() c_int {
    return cursed_signal_init();
}

export fn cursed_native_signal_register(signal_num: c_int, handler_ptr: ?*anyopaque) c_int {
    // Convert handler pointer to callback - this is a simplified approach
    _ = handler_ptr;
    // In a real implementation, this would properly handle the callback registration
    return cursed_signal_register(signal_num, null);
}

export fn cursed_native_signal_unregister(signal_num: c_int) c_int {
    return cursed_signal_unregister(signal_num);
}

export fn cursed_native_signal_block_mask(mask: c_long) c_int {
    return cursed_signal_block_mask(mask);
}

export fn cursed_native_signal_unblock_mask(mask: c_long) c_int {
    return cursed_signal_unblock_mask(mask);
}

export fn cursed_native_signal_send_process(pid: c_int, signal_num: c_int) c_int {
    return cursed_signal_send_to_process(pid, signal_num);
}

export fn cursed_native_signal_send_group(pgid: c_int, signal_num: c_int) c_int {
    return cursed_signal_send_to_group(pgid, signal_num);
}

export fn cursed_native_signal_wait_sync(signal_num: c_int, timeout_ms: c_int) c_int {
    // This version waits for a specific signal, not any signal
    _ = signal_num; // In a full implementation, would filter for specific signal
    return cursed_signal_wait_for_signal(timeout_ms);
}

export fn cursed_native_signal_is_safe_context() c_int {
    return cursed_signal_is_signal_safe();
}

export fn cursed_native_signal_cleanup() c_int {
    return cursed_signal_cleanup();
}

// Pending signals check - returns a simple structure
export const CursedPendingSignals = extern struct {
    count: c_int,
    signals: [64]c_int,
    timestamps: [64]c_long,
    sender_pids: [64]c_int,
};

export fn cursed_native_signal_check_pending() CursedPendingSignals {
    // Simplified implementation - returns empty pending signals
    // In a real implementation, this would check the OS signal queue
    return CursedPendingSignals{
        .count = 0,
        .signals = [_]c_int{0} ** 64,
        .timestamps = [_]c_long{0} ** 64,
        .sender_pids = [_]c_int{0} ** 64,
    };
}

// =============================================================================
// TESTING SUPPORT
// =============================================================================

// Function to test signal delivery without actual OS signals
export fn cursed_signal_test_deliver(signal_num: c_int) c_int {
    if (signal_num <= 0 or signal_num > 64) {
        return CURSED_SIGNAL_INVALID_ARG;
    }
    
    const signal_type = signal_platform.SignalType.fromUnixSignal(signal_num) orelse {
        return CURSED_SIGNAL_INVALID_ARG;
    };
    
    if (global_signal_handler) |handler| {
        signal_platform.TestSignalSupport.simulateSignal(handler, signal_type) catch {
            return CURSED_SIGNAL_ERROR;
        };
        return CURSED_SIGNAL_SUCCESS;
    }
    
    return CURSED_SIGNAL_ERROR;
}

// Get number of registered handlers
export fn cursed_signal_test_get_handler_count() c_int {
    if (global_signal_handler) |handler| {
        return @intCast(signal_platform.TestSignalSupport.getRegisteredSignalCount(handler));
    }
    return 0;
}

// Check if specific signal is registered
export fn cursed_signal_test_is_registered(signal_num: c_int) c_int {
    if (global_signal_handler == null or signal_num <= 0 or signal_num > 64) {
        return 0;
    }
    
    const signal_type = signal_platform.SignalType.fromUnixSignal(signal_num) orelse {
        return 0;
    };
    
    return if (signal_platform.TestSignalSupport.isSignalRegistered(global_signal_handler.?, signal_type)) 1 else 0;
}
