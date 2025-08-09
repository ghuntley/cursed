const std = @import("std");
const channel_ops = @import("channel_operations_implementation.zig");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    
    std.debug.print("Testing Channel Implementation\n");
    
    try channel_ops.testChannelImplementation(allocator);
    
    std.debug.print("Test completed\n");
}
