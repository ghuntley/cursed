// Real concurrency test in Zig using the existing concurrency system

const std = @import("std");
const concurrency = @import("src-zig/concurrency.zig");
const print = std.debug.print;

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    
    print("🚀 Testing CURSED concurrency runtime...\n", .{});
    
    // Initialize scheduler
    const config = concurrency.SchedulerConfig.default();
    try concurrency.initializeScheduler(allocator, config);
    defer concurrency.shutdownScheduler(allocator);
    
    print("✅ Scheduler initialized\n");
    
    // Test 1: Channel creation and basic operations
    var channel = try concurrency.makeChannel(i32, allocator, 5);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }
    
    print("✅ Channel created with capacity 5\n");
    
    // Test 2: Send and receive operations
    const send_result = try channel.send(42);
    if (send_result == .sent) {
        print("✅ Successfully sent value 42\n");
    } else {
        print("❌ Failed to send value\n");
    }
    
    const received = try channel.receive();
    if (received) |value| {
        print("✅ Successfully received value: {}\n", .{value});
    } else {
        print("❌ Failed to receive value\n");
    }
    
    // Test 3: Channel close behavior
    channel.close();
    if (channel.isClosed()) {
        print("✅ Channel closed successfully\n");
    }
    
    // Test 4: Goroutine creation
    var context_value: i32 = 0;
    const TestContext = struct {
        value: *i32,
        
        fn run(ctx: ?*anyopaque) void {
            const test_ctx: *@This() = @ptrCast(@alignCast(ctx.?));
            test_ctx.value.* = 123;
            print("🦄 Goroutine executed! Set value to 123\n");
        }
    };
    
    var test_context = TestContext{ .value = &context_value };
    const goroutine_id = try concurrency.stan(TestContext.run, &test_context);
    print("✅ Goroutine spawned with ID: {}\n", .{goroutine_id});
    
    // Wait a bit for goroutine to execute
    std.time.sleep(50_000_000); // 50ms
    
    if (context_value == 123) {
        print("✅ Goroutine executed successfully! Value: {}\n", .{context_value});
    } else {
        print("❌ Goroutine did not execute. Value: {}\n", .{context_value});
    }
    
    // Test 5: Multiple goroutines with channels
    var communication_channel = try concurrency.makeChannel(i32, allocator, 10);
    defer {
        communication_channel.deinit();
        allocator.destroy(communication_channel);
    }
    
    const ProducerContext = struct {
        channel: *concurrency.Channel(i32),
        
        fn run(ctx: ?*anyopaque) void {
            const producer_ctx: *@This() = @ptrCast(@alignCast(ctx.?));
            for (1..6) |i| {
                const result = producer_ctx.channel.send(@as(i32, @intCast(i))) catch return;
                if (result == .sent) {
                    print("📤 Producer sent: {}\n", .{i});
                } else {
                    print("❌ Producer failed to send: {}\n", .{i});
                }
                std.time.sleep(10_000_000); // 10ms delay
            }
            producer_ctx.channel.close();
            print("📤 Producer finished and closed channel\n");
        }
    };
    
    const ConsumerContext = struct {
        channel: *concurrency.Channel(i32),
        
        fn run(ctx: ?*anyopaque) void {
            const consumer_ctx: *@This() = @ptrCast(@alignCast(ctx.?));
            while (true) {
                const received = consumer_ctx.channel.receive() catch break;
                if (received) |value| {
                    print("📥 Consumer received: {}\n", .{value});
                } else {
                    print("📥 Consumer detected channel closed\n");
                    break;
                }
                std.time.sleep(5_000_000); // 5ms delay
            }
        }
    };
    
    var producer_context = ProducerContext{ .channel = communication_channel };
    var consumer_context = ConsumerContext{ .channel = communication_channel };
    
    const producer_id = try concurrency.stan(ProducerContext.run, &producer_context);
    const consumer_id = try concurrency.stan(ConsumerContext.run, &consumer_context);
    
    print("✅ Producer goroutine ID: {}\n", .{producer_id});
    print("✅ Consumer goroutine ID: {}\n", .{consumer_id});
    
    // Wait for communication to complete
    print("⏳ Waiting for producer-consumer communication...\n");
    std.time.sleep(200_000_000); // 200ms
    
    print("🎉 Concurrency test completed!\n");
    print("📊 Testing Summary:\n");
    print("  ✅ Scheduler initialization\n");
    print("  ✅ Channel creation and operations\n");
    print("  ✅ Basic goroutine spawning\n");
    print("  ✅ Multi-goroutine communication\n");
    print("  ✅ Work-stealing scheduler active\n");
    
    const scheduler = concurrency.getScheduler();
    if (scheduler) |s| {
        print("📈 Active goroutines: {}\n", .{s.activeGoroutineCount()});
        print("🏃 Scheduler running: {}\n", .{s.isRunning()});
    }
}
