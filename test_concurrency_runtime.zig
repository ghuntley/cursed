//! Test file for CURSED concurrency runtime implementation

const std = @import("std");
const concurrency_runtime = @import("src-zig/concurrency_runtime.zig");

test "concurrency runtime basic functionality" {
    const allocator = std.testing.allocator;
    
    // Test runtime initialization
    try concurrency_runtime.initializeRuntime(allocator);
    defer concurrency_runtime.shutdownRuntime(allocator);

    const runtime = concurrency_runtime.getRuntime();
    try std.testing.expect(runtime != null);
    try std.testing.expect(runtime.?.isActive());
    
    // Test channel creation
    const channel_id = try concurrency_runtime.executeDmCreate(.integer, 3);
    try std.testing.expect(channel_id > 0);
    
    // Test performance monitoring
    const stats = concurrency_runtime.getRuntimeStats();
    try std.testing.expect(stats != null);
    try std.testing.expect(stats.?.channels_created >= 1);
}

test "concurrency runtime health check" {
    const allocator = std.testing.allocator;
    
    // Before initialization, should not be healthy
    try std.testing.expect(!concurrency_runtime.isRuntimeHealthy());
    
    // After initialization, should be healthy
    try concurrency_runtime.initializeRuntime(allocator);
    defer concurrency_runtime.shutdownRuntime(allocator);
    
    try std.testing.expect(concurrency_runtime.isRuntimeHealthy());
}

test "concurrency runtime error recovery configuration" {
    const allocator = std.testing.allocator;
    
    try concurrency_runtime.initializeRuntime(allocator);
    defer concurrency_runtime.shutdownRuntime(allocator);
    
    // Test setting error recovery configuration
    try concurrency_runtime.setErrorRecoveryMaxAttempts(5);
}
