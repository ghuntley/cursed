const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

/// CURSED Defer Runtime System
/// Implements LIFO (Last In, First Out) defer execution
/// Integrates with LLVM code generation for proper cleanup semantics

const DeferStackError = error{
    StackOverflow,
    StackUnderflow,
    InvalidFunction,
};

const MAX_DEFER_STACK_SIZE = 1000;

/// Function pointer type for cleanup functions
const CleanupFuncPtr = *const fn () void;

/// Defer stack entry
const DeferEntry = struct {
    cleanup_func: CleanupFuncPtr,
    scope_id: u32,
};

/// Global defer stack for runtime
var global_defer_stack: [MAX_DEFER_STACK_SIZE]DeferEntry = undefined;
var defer_stack_count: usize = 0;
var current_scope_id: u32 = 0;

/// Defer runtime initialization
pub fn init() void {
    defer_stack_count = 0;
    current_scope_id = 0;
    std.debug.print("✅ Defer runtime initialized\n");
}

/// Defer runtime cleanup
pub fn deinit() void {
    executeAll();
    std.debug.print("✅ Defer runtime cleanup complete\n");
}

/// Push a cleanup function onto the defer stack
pub fn push(cleanup_func: CleanupFuncPtr) DeferStackError!void {
    if (defer_stack_count >= MAX_DEFER_STACK_SIZE) {
        return DeferStackError.StackOverflow;
    }
    
    global_defer_stack[defer_stack_count] = DeferEntry{
        .cleanup_func = cleanup_func,
        .scope_id = current_scope_id,
    };
    defer_stack_count += 1;
    
    std.debug.print("Defer pushed: function, stack size: {d}\n", .{defer_stack_count});
}

/// Pop the most recent defer function (for early cleanup)
pub fn pop() void {
    if (defer_stack_count == 0) {
        return; // Nothing to pop
    }
    
    defer_stack_count -= 1;
    std.debug.print("Defer popped: stack size: {d}\n", .{defer_stack_count});
}

/// Execute all defer functions in LIFO order
pub fn executeAll() void {
    std.debug.print("Executing {d} defer functions\n", .{defer_stack_count});
    
    // Execute in reverse order (LIFO)
    while (defer_stack_count > 0) {
        defer_stack_count -= 1;
        const entry = global_defer_stack[defer_stack_count];
        
        std.debug.print("Executing defer function\n");
        entry.cleanup_func();
    }
}

/// Execute defer functions up to a specific count (for scoped cleanup)
pub fn executeToCount(target_count: usize) void {
    std.debug.print("Executing defer functions from {d} to {d}\n", .{ defer_stack_count, target_count });
    
    while (defer_stack_count > target_count) {
        defer_stack_count -= 1;
        const entry = global_defer_stack[defer_stack_count];
        
        std.debug.print("Executing scoped defer function\n");
        entry.cleanup_func();
    }
}

/// Get current defer stack size (for scope management)
pub fn getStackSize() usize {
    return defer_stack_count;
}

/// Enter a new scope
pub fn enterScope() u32 {
    current_scope_id += 1;
    std.debug.print("Entered scope {d}\n", .{current_scope_id});
    return current_scope_id;
}

/// Exit a scope and execute its defers
pub fn exitScope(scope_id: u32) void {
    std.debug.print("Exiting scope {d}\n", .{scope_id});
    
    // Execute all defers for this scope in LIFO order
    var i = defer_stack_count;
    while (i > 0) {
        i -= 1;
        const entry = global_defer_stack[i];
        
        if (entry.scope_id == scope_id) {
            std.debug.print("Executing defer for scope {d}\n", .{scope_id});
            entry.cleanup_func();
            
            // Remove this entry by shifting remaining entries
            var j = i;
            while (j < defer_stack_count - 1) {
                global_defer_stack[j] = global_defer_stack[j + 1];
                j += 1;
            }
            defer_stack_count -= 1;
        }
    }
}

/// Clear all defer functions (emergency cleanup)
pub fn clearAll() void {
    defer_stack_count = 0;
    std.debug.print("Defer stack cleared\n");
}

// C-compatible export functions for LLVM integration
export fn cursed_defer_push(cleanup_func: ?CleanupFuncPtr) void {
    if (cleanup_func) |func| {
        push(func) catch {
            std.debug.print("Error: Failed to push defer function\n");
        };
    }
}

export fn cursed_defer_pop() void {
    pop();
}

export fn cursed_defer_execute_all() void {
    executeAll();
}

export fn cursed_defer_execute_to_count(target_count: usize) void {
    executeToCount(target_count);
}

export fn cursed_defer_get_stack_size() usize {
    return getStackSize();
}

export fn cursed_defer_clear_all() void {
    clearAll();
}

export fn cursed_defer_init() void {
    init();
}

export fn cursed_defer_cleanup() void {
    deinit();
}

export fn cursed_defer_enter_scope() u32 {
    return enterScope();
}

export fn cursed_defer_exit_scope(scope_id: u32) void {
    exitScope(scope_id);
}

// Test functions for validation
pub fn testBasicDefer() !void {
    init();
    defer deinit();
    
    var test_value: i32 = 0;
    
    const TestCleanup = struct {
        var value_ptr: *i32 = undefined;
        
        fn cleanup() void {
            value_ptr.* = 42;
        }
    };
    
    TestCleanup.value_ptr = &test_value;
    try push(TestCleanup.cleanup);
    
    executeAll();
    
    if (test_value != 42) {
        return error.DeferNotExecuted;
    }
    
    std.debug.print("✅ Basic defer test passed\n");
}

pub fn testDeferOrder() !void {
    init();
    defer deinit();
    
    var execution_order: [3]i32 = [_]i32{ 0, 0, 0 };
    var index: usize = 0;
    
    const TestCleanup1 = struct {
        var order_ptr: *[3]i32 = undefined;
        var index_ptr: *usize = undefined;
        
        fn cleanup() void {
            order_ptr.*[index_ptr.*] = 1;
            index_ptr.* += 1;
        }
    };
    
    const TestCleanup2 = struct {
        var order_ptr: *[3]i32 = undefined;
        var index_ptr: *usize = undefined;
        
        fn cleanup() void {
            order_ptr.*[index_ptr.*] = 2;
            index_ptr.* += 1;
        }
    };
    
    const TestCleanup3 = struct {
        var order_ptr: *[3]i32 = undefined;
        var index_ptr: *usize = undefined;
        
        fn cleanup() void {
            order_ptr.*[index_ptr.*] = 3;
            index_ptr.* += 1;
        }
    };
    
    TestCleanup1.order_ptr = &execution_order;
    TestCleanup1.index_ptr = &index;
    TestCleanup2.order_ptr = &execution_order;
    TestCleanup2.index_ptr = &index;
    TestCleanup3.order_ptr = &execution_order;
    TestCleanup3.index_ptr = &index;
    
    try push(TestCleanup1.cleanup);
    try push(TestCleanup2.cleanup);
    try push(TestCleanup3.cleanup);
    
    executeAll();
    
    // Should execute in LIFO order: 3, 2, 1
    if (execution_order[0] != 3 or execution_order[1] != 2 or execution_order[2] != 1) {
        std.debug.print("Expected order: [3, 2, 1], got: [{d}, {d}, {d}]\n", .{ execution_order[0], execution_order[1], execution_order[2] });
        return error.IncorrectDeferOrder;
    }
    
    std.debug.print("✅ Defer order test passed\n");
}

// Run tests
pub fn runTests() !void {
    std.debug.print("Running defer runtime tests...\n");
    try testBasicDefer();
    try testDeferOrder();
    std.debug.print("✅ All defer runtime tests passed\n");
}
