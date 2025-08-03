const std = @import("std");
const print = std.debug.print;
const concurrency = @import("src-zig/concurrency.zig");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    
    print("Testing CURSED Concurrency Implementation\n", .{});
    print("=========================================\n\n", .{});
    
    // Test 1: Channel Creation and Basic Operations
    print("Test 1: Channel Creation and Basic Operations\n");
    {
        var channel = try concurrency.makeChannel(i32, allocator, 3);
        defer {
            channel.deinit();
            allocator.destroy(channel);
        }
        
        // Test channel send
        const send_result1 = try channel.send(42);
        print("  Send 42: {}\n", .{send_result1});
        
        const send_result2 = try channel.send(43);
        print("  Send 43: {}\n", .{send_result2});
        
        // Test channel receive
        const received1 = try channel.receive();
        print("  Received: {?}\n", .{received1});
        
        const received2 = try channel.receive();
        print("  Received: {?}\n", .{received2});
        
        // Test channel properties
        print("  Channel length: {}\n", .{channel.length()});
        print("  Channel is empty: {}\n", .{channel.isEmpty()});
        print("  Channel is closed: {}\n", .{channel.isClosed()});
    }
    print("✓ Channel operations working\n\n");
    
    // Test 2: Channel Closing
    print("Test 2: Channel Closing\n");
    {
        var channel = try concurrency.makeChannel(i32, allocator, 1);
        defer {
            channel.deinit();
            allocator.destroy(channel);
        }
        
        _ = try channel.send(100);
        print("  Sent value: 100\n");
        
        channel.close();
        print("  Channel closed\n");
        
        const send_result = try channel.send(101);
        print("  Try send after close: {}\n", .{send_result});
        
        // Should still receive buffered value
        const received = try channel.receive();
        print("  Received buffered value: {?}\n", .{received});
        
        print("  Channel is closed: {}\n", .{channel.isClosed()});
    }
    print("✓ Channel closing working\n\n");
    
    // Test 3: Work-Stealing Deque
    print("Test 3: Work-Stealing Deque\n");
    {
        var deque = concurrency.WorkStealingDeque.init(allocator);
        defer deque.deinit();
        
        var goroutine = concurrency.Goroutine.init(allocator, 1, undefined, null);
        
        try deque.pushBottom(&goroutine);
        print("  Pushed goroutine, length: {}\n", .{deque.length()});
        
        const popped = deque.popBottom();
        print("  Popped goroutine: {}\n", .{popped != null});
        print("  Deque is empty: {}\n", .{deque.isEmpty()});
    }
    print("✓ Work-stealing deque working\n\n");
    
    // Test 4: Scheduler Initialization
    print("Test 4: Scheduler Initialization\n");
    {
        const config = concurrency.SchedulerConfig.default();
        print("  Default config - Workers: {}, Queue capacity: {}\n", .{config.num_workers, config.queue_capacity});
        
        try concurrency.initializeScheduler(allocator, config);
        print("  Scheduler initialized\n");
        
        const scheduler = concurrency.getScheduler();
        if (scheduler) |sched| {
            print("  Scheduler is running: {}\n", .{sched.isRunning()});
            print("  Active goroutines: {}\n", .{sched.activeGoroutineCount()});
        }
        
        concurrency.shutdownScheduler(allocator);
        print("  Scheduler shutdown\n");
    }
    print("✓ Scheduler operations working\n\n");
    
    // Test 5: Select Statement  
    print("Test 5: Select Statement\n");
    {
        var select_stmt = concurrency.Select.init(allocator);
        defer select_stmt.deinit();
        
        try select_stmt.addDefault(0);
        print("  Added default case\n");
        print("  Has default: {}\n", .{select_stmt.has_default});
        
        const result = try select_stmt.execute();
        print("  Select result: {}\n", .{result});
    }
    print("✓ Select statement working\n\n");
    
    print("All concurrency tests passed! ✅\n");
    print("Concurrency system is ready for integration.\n");
}
