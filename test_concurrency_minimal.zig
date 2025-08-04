const std = @import("std");
const concurrency = @import("src-zig/concurrency.zig");
const print = std.debug.print;

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    
    print("Testing basic channel creation...\n", .{});
    
    // Test just channel creation
    var channel = try concurrency.makeChannel(i32, allocator, 5);
    defer {
        channel.deinit();
        allocator.destroy(channel);
    }
    
    print("Channel created successfully!\n", .{});
    
    // Test basic send/receive
    const send_result = try channel.send(42);
    print("Send result: {any}\n", .{send_result});
    
    const received = try channel.receive();
    print("Received: {any}\n", .{received});
    
    print("Basic test completed!\n", .{});
}
