const std = @import("std");
const print = std.debug.print;
const testing = std.testing;

const concurrency = @import("src-zig/concurrency.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    print("Testing fixes for Unbuffered Channel and Channel Throughput\n", .{});
    
    // Test 1: Basic unbuffered channel
    {
        print("Test 1: Basic unbuffered channel creation...\n", .{});
        const config = concurrency.SchedulerConfig.default();
        
        try concurrency.initializeScheduler(allocator, config);
        defer concurrency.shutdownScheduler(allocator);
        
        var channel = try concurrency.makeUnbufferedChannel(i32, allocator);
        defer {
            channel.deinit();
            allocator.destroy(channel);
        }
        
        // Should not be able to send without receiver
        const send_result = try channel.trySend(42);
        try testing.expect(send_result == concurrency.SendResult.would_block);
        print("✅ Basic unbuffered channel works\n", .{});
    }
    
    // Test 2: Basic buffered channel throughput
    {
        print("Test 2: Basic channel throughput...\n", .{});
        const config = concurrency.SchedulerConfig.default();
        
        try concurrency.initializeScheduler(allocator, config);
        defer concurrency.shutdownScheduler(allocator);
        
        var channel = try concurrency.makeChannel(u64, allocator, 10);
        defer {
            channel.deinit();
            allocator.destroy(channel);
        }
        
        // Simple send/receive test
        try testing.expect(try channel.send(123) == concurrency.SendResult.sent);
        const received = try channel.receive();
        try testing.expect(received.? == 123);
        print("✅ Basic channel throughput works\n", .{});
    }
    
    print("✅ All specific fixes working!\n", .{});
}
