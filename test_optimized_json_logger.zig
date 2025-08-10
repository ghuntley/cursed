const std = @import("std");
const optimized = @import("src-zig/optimized_json_logger.zig");
const benchmark = @import("src-zig/logger_performance_benchmark.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    std.debug.print("🚀 CURSED Optimized JSON Logger Demonstration\n", .{});
    std.debug.print("{s}\n", .{"=" ** 60});
    std.debug.print("\n", .{});
    
    // Test basic functionality
    try testBasicFunctionality(allocator);
    
    // Run performance benchmarks
    try benchmark.runLoggerBenchmarks(allocator);
    
    // Run stress test
    try benchmark.runStressTest(allocator);
    
    std.debug.print("\n🎉 All tests completed successfully!\n", .{});
}

fn testBasicFunctionality(allocator: std.mem.Allocator) !void {
    std.debug.print("\n🧪 Testing Basic Functionality\n", .{});
    std.debug.print("{s}\n", .{"-" ** 40});
    std.debug.print("\n", .{});
    
    var logger = try optimized.OptimizedJsonLogger.init(allocator);
    defer logger.deinit();
    
    logger.enableHighPerformanceMode();
    
    // Test different log levels and attribute types
    const attrs = [_]optimized.LogAttribute{
        .{ .key = "user_id", .value = .{ .integer = 12345 } },
        .{ .key = "session_id", .value = .{ .string = "sess_abc123xyz" } },
        .{ .key = "request_duration", .value = .{ .float = 45.67 } },
        .{ .key = "authenticated", .value = .{ .boolean = true } },
        .{ .key = "endpoint", .value = .{ .string = "/api/v1/users/profile" } },
        .{ .key = "response_code", .value = .{ .integer = 200 } },
    };
    
    // Test each log level
    const test_cases = [_]struct {
        level: optimized.LogLevel,
        message: []const u8,
    }{
        .{ .level = .DEBUG, .message = "Debug information for troubleshooting" },
        .{ .level = .INFO, .message = "User successfully authenticated and accessed profile" },
        .{ .level = .WARN, .message = "Rate limit approaching for user session" },
        .{ .level = .ERROR, .message = "Database connection timeout during query" },
        .{ .level = .FATAL, .message = "Critical system failure detected" },
    };
    
    for (test_cases) |test_case| {
        const json_output = try logger.formatJsonOptimized(test_case.level, test_case.message, &attrs);
        
        std.debug.print("📝 {s}: {s}\n", .{ @tagName(test_case.level), json_output });
    }
    
    // Display performance metrics
    const metrics = logger.getPerformanceMetrics();
    std.debug.print("\n📊 Performance Metrics:\n", .{});
    std.debug.print("   • Logs Processed: {}\n", .{metrics.logs_processed});
    std.debug.print("   • Bytes Written: {}\n", .{metrics.bytes_written});
    std.debug.print("   • Avg Format Time: {} ns\n", .{metrics.avg_format_time_ns});
    std.debug.print("   • Throughput: {d:.2f} ops/sec\n", .{metrics.throughput_logs_per_sec});
    std.debug.print("   • Memory Efficiency: {d:.1f} bytes/log\n", .{metrics.memory_efficiency});
}
