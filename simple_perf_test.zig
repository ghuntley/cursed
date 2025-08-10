const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print("=== Simple Performance Hooks Demo ===\n", .{});

    // Test basic structures
    const config = @import("src-zig/performance_hooks.zig").PerformanceHooksConfig{};
    std.debug.print("Config sampling rate: {d:.1}%\n", .{config.sampling_rate * 100});

    // Test function call data
    const FunctionCallData = @import("src-zig/performance_hooks.zig").FunctionCallData;
    const call_data = FunctionCallData{
        .function_name = "test_function",
        .module_name = "test_module",
        .start_time = 0,
        .duration = 1000000, // 1ms in nanoseconds
        .memory_allocated = 1024,
        .memory_deallocated = 0,
        .call_stack_depth = 5,
        .thread_id = 12345,
        .goroutine_id = null,
        .arguments_count = 2,
        .return_value_size = 8,
        .error_occurred = false,
        .cpu_time_ns = 1000000,
        .stack_trace = null,
    };

    std.debug.print("Function call data:\n", .{});
    std.debug.print("  Function: {s}.{s}\n", .{ call_data.module_name, call_data.function_name });
    std.debug.print("  Duration: {d:.3}ms\n", .{@as(f64, @floatFromInt(call_data.duration)) / 1_000_000.0});
    std.debug.print("  Memory allocated: {d} bytes\n", .{call_data.memory_allocated});

    // Test performance metrics
    const ResourceSnapshot = @import("src-zig/performance_hooks.zig").ResourceSnapshot;
    
    var hot_paths = std.ArrayList(@import("src-zig/performance_hooks.zig").HotPathData).init(allocator);
    defer hot_paths.deinit();
    
    var bottlenecks = std.ArrayList(@import("src-zig/performance_hooks.zig").BottleneckData).init(allocator);
    defer bottlenecks.deinit();

    const resource_snapshot = ResourceSnapshot{
        .timestamp = 0,
        .cpu_usage_percent = 25.5,
        .memory_usage_bytes = 128 * 1024 * 1024, // 128MB
        .heap_usage_bytes = 64 * 1024 * 1024,    // 64MB
        .stack_usage_bytes = 8 * 1024,           // 8KB
        .open_files = 42,
        .network_connections = 5,
        .threads_count = 8,
        .goroutines_count = 150,
        .channels_count = 25,
        .gc_pressure = 0.1,
        .load_average = 1.5,
    };

    std.debug.print("\nResource usage snapshot:\n", .{});
    std.debug.print("  CPU: {d:.1}%\n", .{resource_snapshot.cpu_usage_percent});
    std.debug.print("  Memory: {d:.1}MB\n", .{@as(f64, @floatFromInt(resource_snapshot.memory_usage_bytes)) / 1_048_576.0});
    std.debug.print("  Goroutines: {d}\n", .{resource_snapshot.goroutines_count});
    std.debug.print("  Channels: {d}\n", .{resource_snapshot.channels_count});

    std.debug.print("\n=== Performance hooks structures tested successfully ===\n", .{});
}
