const std = @import("std");
const print = std.debug.print;
const testing = std.testing;

const concurrency_complete = @import("src-zig/concurrency_complete.zig");

// Test the complete concurrency system
test "complete concurrency system integration" {
    const allocator = std.testing.allocator;
    
    print("\n=== Testing Complete CURSED Concurrency System ===\n");
    
    // Test 1: Runtime initialization
    var runtime = try concurrency_complete.ConcurrencyRuntime.init(allocator);
    defer runtime.deinit();
    
    try runtime.start();
    print("✅ Runtime initialized and started\n");
    
    // Test 2: Goroutine spawning
    var executed_count: u32 = 0;
    const TestContext = struct {
        executed_count: *u32,
        id: u32,
    };
    
    var context1 = TestContext{ .executed_count = &executed_count, .id = 1 };
    var context2 = TestContext{ .executed_count = &executed_count, .id = 2 };
    
    const testFn = struct {
        fn run(ctx: ?*anyopaque) void {
            const test_ctx: *TestContext = @ptrCast(@alignCast(ctx.?));
            _ = @atomicRmw(u32, test_ctx.executed_count, .Add, 1, .acq_rel);
            print("Goroutine {} executed\n", .{test_ctx.id});
        }
    }.run;
    
    const goroutine1_id = try runtime.spawnGoroutine(testFn, &context1);
    const goroutine2_id = try runtime.spawnGoroutine(testFn, &context2);
    
    print("✅ Spawned goroutines: {} and {}\n", .{ goroutine1_id, goroutine2_id });
    
    // Wait for execution
    var wait_count: u32 = 0;
    while (@atomicLoad(u32, &executed_count, .acquire) < 2 and wait_count < 1000) {
        std.time.sleep(1_000_000); // 1ms
        wait_count += 1;
    }
    
    try testing.expect(@atomicLoad(u32, &executed_count, .acquire) == 2);
    print("✅ Both goroutines executed successfully\n");
    
    // Test 3: Channel creation and operations
    const channel_id = try runtime.createChannel(i32, 3);
    print("✅ Created buffered channel with ID: {}\n", .{channel_id});
    
    // Test send and receive
    const send_result = try runtime.channelSend(i32, channel_id, 42);
    try testing.expect(send_result == concurrency_complete.SendResult.sent);
    print("✅ Successfully sent value 42 to channel\n");
    
    const received = try runtime.channelReceive(i32, channel_id);
    try testing.expect(received != null);
    try testing.expect(received.? == 42);
    print("✅ Successfully received value {} from channel\n", .{received.?});
    
    // Test multiple sends to buffered channel
    try testing.expect(try runtime.channelSend(i32, channel_id, 100) == concurrency_complete.SendResult.sent);
    try testing.expect(try runtime.channelSend(i32, channel_id, 200) == concurrency_complete.SendResult.sent);
    try testing.expect(try runtime.channelSend(i32, channel_id, 300) == concurrency_complete.SendResult.sent);
    print("✅ Successfully sent multiple values to buffered channel\n");
    
    // Test channel close
    try runtime.channelClose(channel_id);
    print("✅ Successfully closed channel\n");
    
    // Test sending to closed channel should fail
    const closed_send_result = try runtime.channelSend(i32, channel_id, 999);
    try testing.expect(closed_send_result == concurrency_complete.SendResult.closed);
    print("✅ Sending to closed channel correctly failed\n");
    
    // Test 4: Select statement basic functionality
    var select_stmt = concurrency_complete.SelectStatement.init(allocator);
    defer select_stmt.deinit();
    
    select_stmt.addDefaultCase(42);
    const select_result = try select_stmt.execute(runtime);
    
    switch (select_result) {
        .case_completed => |case_id| {
            try testing.expect(case_id == 42);
            print("✅ Select statement executed default case correctly\n");
        },
        else => {
            print("❌ Select statement failed\n");
            try testing.expect(false);
        },
    }
    
    // Test 5: Runtime statistics
    const stats = runtime.getStats();
    print("✅ Runtime statistics:\n");
    print("   - Total goroutines spawned: {}\n", .{stats.total_goroutines_spawned});
    print("   - Total channels created: {}\n", .{stats.total_channels_created});
    print("   - Active goroutines: {}\n", .{stats.active_goroutines});
    print("   - Active channels: {}\n", .{stats.active_channels});
    
    try testing.expect(stats.total_goroutines_spawned >= 2);
    try testing.expect(stats.total_channels_created >= 1);
    
    print("\n🎉 All concurrency tests passed! The CURSED concurrency system is working correctly.\n\n");
}

// Test C FFI exports
test "C FFI concurrency interface" {
    print("\n=== Testing C FFI Concurrency Interface ===\n");
    
    // Initialize runtime through C FFI
    const init_result = concurrency_complete.cursed_concurrency_init();
    try testing.expect(init_result == true);
    print("✅ C FFI runtime initialization successful\n");
    
    defer concurrency_complete.cursed_concurrency_shutdown();
    
    // Test goroutine spawning through C FFI
    const TestCContext = struct {
        executed: bool = false,
    };
    
    var c_context = TestCContext{};
    
    const c_test_fn = struct {
        fn run(ctx: ?*anyopaque) callconv(.C) void {
            const test_ctx: *TestCContext = @ptrCast(@alignCast(ctx.?));
            test_ctx.executed = true;
            print("C FFI goroutine executed\n");
        }
    }.run;
    
    const c_goroutine_id = concurrency_complete.cursed_stan(c_test_fn, &c_context);
    try testing.expect(c_goroutine_id > 0);
    print("✅ C FFI goroutine spawned with ID: {}\n", .{c_goroutine_id});
    
    // Wait for C goroutine execution
    var wait_attempts: u32 = 0;
    while (!c_context.executed and wait_attempts < 1000) {
        std.time.sleep(1_000_000); // 1ms
        wait_attempts += 1;
    }
    
    try testing.expect(c_context.executed);
    print("✅ C FFI goroutine executed successfully\n");
    
    // Test channel creation through C FFI
    const c_channel_id = concurrency_complete.cursed_dm_create(4, 2); // i32 size, capacity 2
    try testing.expect(c_channel_id > 0);
    print("✅ C FFI channel created with ID: {}\n", .{c_channel_id});
    
    // Test sending through C FFI
    const test_data: i32 = 123456;
    const send_result = concurrency_complete.cursed_dm_send(c_channel_id, &test_data, 4);
    try testing.expect(send_result == @intFromEnum(concurrency_complete.SendResult.sent));
    print("✅ C FFI channel send successful\n");
    
    // Test receiving through C FFI
    var received_data: i32 = 0;
    const recv_result = concurrency_complete.cursed_dm_recv(c_channel_id, &received_data, 4);
    try testing.expect(recv_result == @intFromEnum(concurrency_complete.ReceiveResult.received));
    print("✅ C FFI channel receive successful, got: {}\n", .{received_data});
    
    // Test channel close through C FFI
    concurrency_complete.cursed_dm_close(c_channel_id);
    print("✅ C FFI channel close successful\n");
    
    // Test runtime statistics through C FFI
    const c_stats = concurrency_complete.cursed_concurrency_stats();
    print("✅ C FFI runtime statistics retrieved:\n");
    print("   - Goroutines spawned: {}\n", .{c_stats.total_goroutines_spawned});
    print("   - Channels created: {}\n", .{c_stats.total_channels_created});
    
    print("\n🎉 All C FFI tests passed! The C interface is working correctly.\n\n");
}
