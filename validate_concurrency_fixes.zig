const std = @import("std");
const print = std.debug.print;
const testing = std.testing;

const concurrency = @import("src-zig/concurrency.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    print("🚀 CURSED Concurrency Validation Suite\n", .{});
    print("======================================\n\n", .{});
    
    var passed: u32 = 0;
    var failed: u32 = 0;
    
    // Test 1: Basic goroutine creation and execution
    {
        print("Test 1: Basic Goroutine...", .{});
        const config = concurrency.SchedulerConfig.default();
        try concurrency.initializeScheduler(allocator, config);
        defer concurrency.shutdownScheduler(allocator);
        
        var executed = false;
        const testFn = struct {
            fn run(ctx: ?*anyopaque) void {
                const flag = @as(*bool, @ptrCast(@alignCast(ctx.?)));
                flag.* = true;
            }
        }.run;
        
        const goroutine_id = try concurrency.stan(testFn, &executed);
        std.time.sleep(50_000_000); // 50ms
        
        if (executed and goroutine_id > 0) {
            print(" ✅ PASSED\n", .{});
            passed += 1;
        } else {
            print(" ❌ FAILED\n", .{});
            failed += 1;
        }
    }
    
    // Test 2: Channel operations 
    {
        print("Test 2: Channel Operations...", .{});
        const config = concurrency.SchedulerConfig.default();
        try concurrency.initializeScheduler(allocator, config);
        defer concurrency.shutdownScheduler(allocator);
        
        var channel = try concurrency.makeChannel(i32, allocator, 3);
        defer {
            channel.deinit();
            allocator.destroy(channel);
        }
        
        const send_result = try channel.send(42);
        const received = try channel.receive();
        
        if (send_result == concurrency.SendResult.sent and received.? == 42) {
            print(" ✅ PASSED\n", .{});
            passed += 1;
        } else {
            print(" ❌ FAILED\n", .{});
            failed += 1;
        }
    }
    
    // Test 3: Multiple goroutines
    {
        print("Test 3: Multiple Goroutines...", .{});
        const config = concurrency.SchedulerConfig.default();
        try concurrency.initializeScheduler(allocator, config);
        defer concurrency.shutdownScheduler(allocator);
        
        var counter: u32 = 0;
        var mutex = std.Thread.Mutex{};
        
        const Context = struct {
            counter: *u32,
            mutex: *std.Thread.Mutex,
        };
        var context = Context{ .counter = &counter, .mutex = &mutex };
        
        const taskFn = struct {
            fn run(ctx: ?*anyopaque) void {
                const test_ctx = @as(*Context, @ptrCast(@alignCast(ctx.?)));
                test_ctx.mutex.lock();
                defer test_ctx.mutex.unlock();
                test_ctx.counter.* += 1;
            }
        }.run;
        
        // Spawn 10 goroutines
        for (0..10) |_| {
            _ = try concurrency.stan(taskFn, &context);
        }
        
        std.time.sleep(100_000_000); // 100ms
        
        mutex.lock();
        const final_count = counter;
        mutex.unlock();
        
        if (final_count == 10) {
            print(" ✅ PASSED\n", .{});
            passed += 1;
        } else {
            print(" ❌ FAILED (count: {})\n", .{final_count});
            failed += 1;
        }
    }
    
    // Test 4: Previously failing unbuffered channel
    {
        print("Test 4: Unbuffered Channel...", .{});
        const config = concurrency.SchedulerConfig.default();
        try concurrency.initializeScheduler(allocator, config);
        defer concurrency.shutdownScheduler(allocator);
        
        var channel = try concurrency.makeUnbufferedChannel(i32, allocator);
        defer {
            channel.deinit();
            allocator.destroy(channel);
        }
        
        const send_result = try channel.trySend(42);
        
        if (send_result == concurrency.SendResult.would_block) {
            print(" ✅ PASSED\n", .{});
            passed += 1;
        } else {
            print(" ❌ FAILED\n", .{});
            failed += 1;
        }
    }
    
    print("\n📊 Final Results:\n", .{});
    print("✅ Passed: {}\n", .{passed});
    print("❌ Failed: {}\n", .{failed});
    print("📈 Success rate: {d:.1}%\n", .{@as(f64, @floatFromInt(passed)) * 100.0 / @as(f64, @floatFromInt(passed + failed))});
    
    if (failed == 0) {
        print("\n🎉 All validation tests passed! Concurrency fixes are working correctly.\n", .{});
    } else {
        print("\n⚠️  Some validation tests failed.\n", .{});
    }
}
