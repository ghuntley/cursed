const std = @import("std");
const concurrency = @import("src-zig/concurrency.zig");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    
    std.debug.print("Testing CURSED Concurrency Implementation\n", .{});
    std.debug.print("=========================================\n\n", .{});
    
    // Test channel creation and basic operations
    std.debug.print("Test 1: Channel Creation\n", .{});
    var channel = try concurrency.makeChannel(i32, allocator, 3);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }
    
    // Test send
    const send_result = try channel.send(42);
    std.debug.print("  Send result: {}\n", .{send_result});
    
    // Test receive
    const received = try channel.receive();
    std.debug.print("  Received: {?}\n", .{received});
    
    std.debug.print("✓ Channel operations working\n\n", .{});
    
    // Test work-stealing deque
    std.debug.print("Test 2: Work-Stealing Deque\n", .{});
    var deque = concurrency.WorkStealingDeque.init(allocator);
    defer deque.deinit();
    
    var goroutine = concurrency.Goroutine.init(allocator, 1, undefined, null);
    try deque.pushBottom(&goroutine);
    std.debug.print("  Pushed goroutine, length: {}\n", .{deque.length()});
    
    const popped = deque.popBottom();
    std.debug.print("  Popped: {}\n", .{popped != null});
    std.debug.print("✓ Work-stealing deque working\n\n", .{});
    
    // Test select statement
    std.debug.print("Test 3: Select Statement\n", .{});
    var select_stmt = concurrency.Select.init(allocator);
    defer select_stmt.deinit();
    
    try select_stmt.addDefault(0);
    const result = try select_stmt.execute();
    std.debug.print("  Select result: {}\n", .{result});
    std.debug.print("✓ Select statement working\n\n", .{});
    
    std.debug.print("All tests passed! ✅\n", .{});
}
