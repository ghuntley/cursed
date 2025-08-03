const std = @import("std");
const runtime = @import("src-zig/concurrency_runtime_bridge.zig");

// External function declarations for testing
extern fn cursed_concurrency_init() void;
extern fn cursed_dm_create(element_size: u32, capacity: u32) ?*anyopaque;
extern fn cursed_dm_send(channel_ptr: ?*anyopaque, value_ptr: ?*anyopaque, value_size: u32) u32;
extern fn cursed_dm_receive(channel_ptr: ?*anyopaque, buffer_ptr: ?*anyopaque, buffer_size: u32) u32;
extern fn cursed_stan_goroutine(function_ptr: ?*anyopaque, context: ?*anyopaque) u64;
extern fn cursed_ready_select(operations_ptr: ?*anyopaque, operation_count: u32) i32;

pub fn main() !void {
    std.debug.print("Testing CURSED Runtime Bridge\n", .{});
    std.debug.print("=============================\n\n", .{});
    
    // Test runtime initialization
    std.debug.print("Test 1: Runtime Initialization\n", .{});
    cursed_concurrency_init();
    std.debug.print("✓ Runtime initialized\n\n", .{});
    
    // Test channel creation
    std.debug.print("Test 2: Channel Creation\n", .{});
    const channel = cursed_dm_create(4, 3); // i32, capacity 3
    if (channel != null) {
        std.debug.print("✓ Channel created successfully\n", .{});
    } else {
        std.debug.print("✗ Channel creation failed\n", .{});
    }
    std.debug.print("\n", .{});
    
    // Test channel send/receive
    if (channel != null) {
        std.debug.print("Test 3: Channel Send/Receive\n", .{});
        
        var value: i32 = 42;
        const send_result = cursed_dm_send(channel, &value, 4);
        std.debug.print("  Send result: {} (0=success, 1=would_block, 2=closed)\n", .{send_result});
        
        var buffer: i32 = 0;
        const receive_result = cursed_dm_receive(channel, &buffer, 4);
        std.debug.print("  Receive result: {} (0=success, 1=would_block, 2=closed)\n", .{receive_result});
        std.debug.print("  Received value: {}\n", .{buffer});
        
        if (send_result == 0 and receive_result == 0 and buffer == 42) {
            std.debug.print("✓ Channel send/receive working\n", .{});
        } else {
            std.debug.print("✗ Channel send/receive failed\n", .{});
        }
        std.debug.print("\n", .{});
    }
    
    // Test goroutine spawning
    std.debug.print("Test 4: Goroutine Spawning\n", .{});
    const goroutine_id = cursed_stan_goroutine(null, null);
    std.debug.print("  Goroutine ID: {}\n", .{goroutine_id});
    if (goroutine_id > 0) {
        std.debug.print("✓ Goroutine spawning working\n", .{});
    } else {
        std.debug.print("✗ Goroutine spawning failed\n", .{});
    }
    std.debug.print("\n", .{});
    
    // Test select statement
    std.debug.print("Test 5: Select Statement\n", .{});
    const select_result = cursed_ready_select(null, 0);
    std.debug.print("  Select result: {}\n", .{select_result});
    if (select_result >= 0) {
        std.debug.print("✓ Select statement working\n", .{});
    } else {
        std.debug.print("✗ Select statement failed\n", .{});
    }
    std.debug.print("\n", .{});
    
    std.debug.print("Runtime bridge tests completed! ✅\n", .{});
}
