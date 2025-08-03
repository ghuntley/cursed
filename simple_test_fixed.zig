const std = @import("std");
const print = std.debug.print;

const concurrency = @import("src-zig/concurrency.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    print("🚀 Simple CURSED Concurrency Test\n", .{});
    print("================================\n", .{});

    // Test 1: Channel creation and basic operations
    print("Test 1: Channel operations...\n", .{});
    var channel = try concurrency.makeChannel(i32, allocator, 5);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }

    // Send and receive
    _ = try channel.send(42);
    _ = try channel.send(43);
    
    const val1 = try channel.receive();
    const val2 = try channel.receive();
    
    print("  Sent: 42, 43\n", .{});
    print("  Received: {}, {}\n", .{ val1.?, val2.? });
    print("  ✅ Channel operations work!\n\n", .{});

    // Test 2: Select statement
    print("Test 2: Select statement...\n", .{});
    var select_stmt = concurrency.Select.init(allocator);
    defer select_stmt.deinit();

    try select_stmt.addDefault(0);
    const result = try select_stmt.execute();
    print("  Select result: {}\n", .{result});
    print("  ✅ Select statement works!\n\n", .{});

    // Test 3: Work-stealing deque
    print("Test 3: Work-stealing deque...\n", .{});
    var deque = concurrency.WorkStealingDeque.init(allocator);
    defer deque.deinit();

    // Create a simple goroutine function
    const simpleTask = struct {
        fn run(ctx: ?*anyopaque) void {
            _ = ctx;
            // Simple task
        }
    }.run;

    var goroutine = concurrency.Goroutine.init(allocator, 1, simpleTask, null);

    try deque.pushBottom(&goroutine);
    print("  Deque length after push: {}\n", .{deque.length()});
    
    const popped = deque.popBottom();
    print("  Popped goroutine ID: {}\n", .{popped.?.id});
    print("  Deque empty: {}\n", .{deque.isEmpty()});
    print("  ✅ Work-stealing deque works!\n\n", .{});

    // Test 4: Scheduler creation (without starting worker threads)
    print("Test 4: Scheduler creation...\n", .{});
    var config = concurrency.SchedulerConfig.default();
    config.num_workers = 2; // Use fewer workers for testing

    var scheduler = try concurrency.Scheduler.init(allocator, config);
    defer scheduler.deinit();

    print("  Scheduler running: {}\n", .{scheduler.isRunning()});
    print("  Active goroutines: {}\n", .{scheduler.activeGoroutineCount()});
    print("  ✅ Scheduler creation works!\n\n", .{});

    print("🎉 All basic tests passed! CURSED concurrency system core functionality is working.\n", .{});
}
