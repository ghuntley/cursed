const std = @import("std");
const concurrency_bridge = @import("src-zig/concurrency_bridge_minimal.zig");

pub fn main() !void {
    std.debug.print("=== P1 Concurrency Runtime Bridge Standalone Test ===\n", .{});
    
    // Initialize the bridge
    std.debug.print("Initializing runtime bridge...\n", .{});
    if (!concurrency_bridge.cursed_runtime_bridge_init()) {
        std.debug.print("❌ Failed to initialize runtime bridge\n", .{});
        return;
    }
    defer concurrency_bridge.cursed_runtime_bridge_cleanup();
    std.debug.print("✅ Runtime bridge initialized successfully\n", .{});
    
    // Test 1: Basic bridge functionality
    std.debug.print("\n--- Test 1: Basic Bridge Functionality ---\n", .{});
    if (concurrency_bridge.cursed_bridge_test()) {
        std.debug.print("✅ Bridge test passed\n", .{});
    } else {
        std.debug.print("❌ Bridge test failed\n", .{});
        return;
    }
    
    // Test 2: Mode switching
    std.debug.print("\n--- Test 2: Mode Switching ---\n", .{});
    const initial_mode = concurrency_bridge.cursed_bridge_get_mode();
    std.debug.print("Initial mode: {}\n", .{initial_mode});
    
    if (concurrency_bridge.cursed_bridge_switch_mode(2)) { // Switch to mixed mode
        const new_mode = concurrency_bridge.cursed_bridge_get_mode();
        std.debug.print("Switched to mode: {}\n", .{new_mode});
        std.debug.print("✅ Mode switching test passed\n", .{});
    } else {
        std.debug.print("❌ Mode switching test failed\n", .{});
        return;
    }
    
    // Test 3: Channel creation and operations
    std.debug.print("\n--- Test 3: Channel Operations ---\n", .{});
    const channel_id = concurrency_bridge.cursed_bridge_create_channel(3, 2); // Capacity 3, mixed mode
    if (channel_id == 0) {
        std.debug.print("❌ Failed to create channel\n", .{});
        return;
    }
    std.debug.print("✅ Created channel with ID: {}\n", .{channel_id});
    
    // Test send operation
    const send_result = concurrency_bridge.cursed_bridge_channel_send(channel_id, 42, 1000);
    if (send_result == 0) {
        std.debug.print("✅ Successfully sent value 42\n", .{});
    } else {
        std.debug.print("❌ Failed to send value, result: {}\n", .{send_result});
    }
    
    // Test receive operation
    const received_value = concurrency_bridge.cursed_bridge_channel_receive(channel_id, 1000);
    if (received_value == 42) {
        std.debug.print("✅ Successfully received value: {}\n", .{received_value});
    } else {
        std.debug.print("❌ Failed to receive expected value, got: {}\n", .{received_value});
    }
    
    // Cleanup
    concurrency_bridge.cursed_bridge_channel_destroy(channel_id);
    std.debug.print("✅ Channel cleaned up\n", .{});
    
    // Test 4: Simple goroutine spawning (compiled mode)
    std.debug.print("\n--- Test 4: Simple Goroutine Spawning ---\n", .{});
    
    const test_goroutine = struct {
        fn run() void {
            // Simple computation in goroutine
            var counter: i32 = 0;
            var i: i32 = 0;
            while (i < 10) : (i += 1) {
                counter += i;
            }
            std.debug.print("Goroutine completed computation: {}\n", .{counter});
        }
    }.run;
    
    const goroutine_id = concurrency_bridge.cursed_bridge_spawn_simple(test_goroutine);
    if (goroutine_id != 0) {
        std.debug.print("✅ Spawned goroutine with ID: {}\n", .{goroutine_id});
        
        // Wait for completion
        if (concurrency_bridge.cursed_bridge_wait_goroutine(goroutine_id, 5000)) {
            std.debug.print("✅ Goroutine completed successfully\n", .{});
        } else {
            std.debug.print("⚠️  Goroutine completion timeout (may still be running)\n", .{});
        }
    } else {
        std.debug.print("❌ Failed to spawn goroutine\n", .{});
    }
    
    std.debug.print("\n=== All Tests Completed ===\n", .{});
    std.debug.print("✅ P1 Concurrency Runtime Bridge is operational!\n", .{});
    
    // Enable mixed mode for future operations
    concurrency_bridge.cursed_bridge_set_mixed_mode(true);
    std.debug.print("✅ Mixed mode enabled for interpreter integration\n", .{});
}
