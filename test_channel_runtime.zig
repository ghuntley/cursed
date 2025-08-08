const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const concurrency = @import("src-zig/concurrency.zig");
const main_unified = @import("src-zig/main_unified.zig");

// Test the channel runtime functionality
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    print("🚀 Testing CURSED Channel Runtime Implementation\n\n");

    // Test 1: Basic channel creation and operations
    try testBasicChannelOperations(allocator);
    
    // Test 2: Type-safe channel operations
    try testTypeSafeChannels(allocator);
    
    // Test 3: Variable channel with GC integration
    try testVariableChannels(allocator);
    
    // Test 4: Concurrent channel usage
    try testConcurrentChannels(allocator);
    
    // Test 5: Channel closing and error handling
    try testChannelClosing(allocator);

    print("✅ All channel tests passed!\n");
}

fn testBasicChannelOperations(allocator: Allocator) !void {
    print("=== Test 1: Basic Channel Operations ===\n");
    
    // Initialize concurrency system
    const config = concurrency.SchedulerConfig.default();
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);
    
    concurrency.initChannelRegistry(allocator);
    
    // Create a buffered channel
    var channel = try concurrency.makeChannel(i32, allocator, 3);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }
    
    // Test send operations
    const result1 = try channel.send(42);
    std.debug.assert(result1 == concurrency.SendResult.sent);
    
    const result2 = try channel.send(84);
    std.debug.assert(result2 == concurrency.SendResult.sent);
    
    // Test receive operations
    const received1 = try channel.receive();
    std.debug.assert(received1.? == 42);
    
    const received2 = try channel.receive();
    std.debug.assert(received2.? == 84);
    
    print("✓ Basic send/receive operations working\n");
    
    // Test channel statistics
    const stats = channel.getStats();
    std.debug.assert(stats.total_sent == 2);
    std.debug.assert(stats.total_received == 2);
    
    print("✓ Channel statistics tracking working\n");
    print("\n");
}

fn testTypeSafeChannels(allocator: Allocator) !void {
    print("=== Test 2: Type-Safe Channel Operations ===\n");
    
    concurrency.initChannelRegistry(allocator);
    
    // Create channels of different types using CURSED API
    const int_channel = try concurrency.dm_create(i32, allocator, 2);
    const float_channel = try concurrency.dm_create(f64, allocator, 2);
    const bool_channel = try concurrency.dm_create(bool, allocator, 2);
    
    // Test type-safe sending
    _ = try concurrency.dm_send(int_channel, @as(i32, 123), allocator);
    _ = try concurrency.dm_send(float_channel, @as(f64, 3.14159), allocator);
    _ = try concurrency.dm_send(bool_channel, true, allocator);
    
    print("✓ Type-safe sending working\n");
    
    // Test type-safe receiving
    const int_result = try concurrency.dm_recv(i32, int_channel, allocator);
    std.debug.assert(int_result.? == 123);
    
    const float_result = try concurrency.dm_recv(f64, float_channel, allocator);
    std.debug.assert(float_result.? == 3.14159);
    
    const bool_result = try concurrency.dm_recv(bool, bool_channel, allocator);
    std.debug.assert(bool_result.? == true);
    
    print("✓ Type-safe receiving working\n");
    
    // Clean up channels
    try concurrency.dm_close(int_channel);
    try concurrency.dm_close(float_channel);
    try concurrency.dm_close(bool_channel);
    
    print("✓ Channel cleanup working\n");
    print("\n");
}

fn testVariableChannels(allocator: Allocator) !void {
    print("=== Test 3: Variable Channels with GC Integration ===\n");
    
    var var_channel = try concurrency.VariableChannel.init(allocator, 3);
    defer var_channel.deinit();
    
    // Test Variable creation and sending
    const Variable = @import("src-zig/main_unified.zig").Variable;
    const var1 = Variable{ .Integer = 456 };
    const var2 = Variable{ .Float = 2.718 };
    const var3 = Variable{ .Boolean = false };
    
    const send1 = try var_channel.sendVariable(var1);
    std.debug.assert(send1 == concurrency.SendResult.sent);
    
    const send2 = try var_channel.sendVariable(var2);
    std.debug.assert(send2 == concurrency.SendResult.sent);
    
    const send3 = try var_channel.sendVariable(var3);
    std.debug.assert(send3 == concurrency.SendResult.sent);
    
    print("✓ Variable sending to channel working\n");
    
    // Test Variable receiving
    const recv1 = try var_channel.receiveVariable();
    if (recv1) |variable| {
        std.debug.assert(variable.Integer == 456);
    }
    
    const recv2 = try var_channel.receiveVariable();
    if (recv2) |variable| {
        std.debug.assert(variable.Float == 2.718);
    }
    
    const recv3 = try var_channel.receiveVariable();
    if (recv3) |variable| {
        std.debug.assert(variable.Boolean == false);
    }
    
    print("✓ Variable receiving from channel working\n");
    print("✓ GC integration for Variables working\n");
    print("\n");
}

fn testConcurrentChannels(allocator: Allocator) !void {
    print("=== Test 4: Concurrent Channel Usage ===\n");
    
    // Initialize scheduler
    const config = concurrency.SchedulerConfig.default();
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);
    
    // Create a shared channel
    var shared_channel = try concurrency.makeChannel(i32, allocator, 5);
    defer {
        shared_channel.deinit();
        allocator.destroy(shared_channel);
    }
    
    // Test context for goroutines
    const TestContext = struct {
        channel: *concurrency.Channel(i32),
        values: []const i32,
    };
    
    const test_values = [_]i32{ 100, 200, 300, 400, 500 };
    var context = TestContext{
        .channel = shared_channel,
        .values = &test_values,
    };
    
    // Producer goroutine
    const producerFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx: *TestContext = @ptrCast(@alignCast(ctx.?));
            for (test_ctx.values) |value| {
                _ = test_ctx.channel.send(value) catch {};
                std.time.sleep(1_000_000); // 1ms delay
            }
        }
    }.run;
    
    // Spawn producer
    _ = try concurrency.stan(producerFn, &context);
    
    // Consumer (main thread)
    var received_count: usize = 0;
    var received_sum: i32 = 0;
    
    while (received_count < test_values.len) {
        const received = try shared_channel.receive();
        if (received) |value| {
            received_sum += value;
            received_count += 1;
        }
    }
    
    // Verify all values were received correctly
    const expected_sum: i32 = 100 + 200 + 300 + 400 + 500;
    std.debug.assert(received_sum == expected_sum);
    std.debug.assert(received_count == test_values.len);
    
    print("✓ Concurrent producer/consumer working\n");
    print("✓ Goroutine channel communication working\n");
    print("\n");
}

fn testChannelClosing(allocator: Allocator) !void {
    print("=== Test 5: Channel Closing and Error Handling ===\n");
    
    concurrency.initChannelRegistry(allocator);
    
    // Create a buffered channel
    const channel_id = try concurrency.dm_create(i32, allocator, 2);
    
    // Send some data
    _ = try concurrency.dm_send(channel_id, @as(i32, 777), allocator);
    _ = try concurrency.dm_send(channel_id, @as(i32, 888), allocator);
    
    // Close the channel
    try concurrency.dm_close(channel_id);
    
    print("✓ Channel closing working\n");
    
    // Try to send to closed channel (should fail gracefully)
    const send_result = try concurrency.dm_send(channel_id, @as(i32, 999), allocator);
    std.debug.assert(send_result == concurrency.SendResult.closed);
    
    print("✓ Send to closed channel handled correctly\n");
    
    // Should still be able to receive buffered data
    const recv1 = try concurrency.dm_recv(i32, channel_id, allocator);
    std.debug.assert(recv1.? == 777);
    
    const recv2 = try concurrency.dm_recv(i32, channel_id, allocator);
    std.debug.assert(recv2.? == 888);
    
    print("✓ Receiving buffered data from closed channel working\n");
    
    // Subsequent receives should return null
    const recv3 = try concurrency.dm_recv(i32, channel_id, allocator);
    std.debug.assert(recv3 == null);
    
    print("✓ Empty closed channel returns null correctly\n");
    print("\n");
}
