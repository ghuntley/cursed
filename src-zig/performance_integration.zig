//! Integration layer for performance hooks with CURSED interpreter
//! Provides seamless integration of performance monitoring into the runtime

const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const performance_hooks = @import("performance_hooks.zig");
const concurrency = @import("concurrency.zig");

/// Performance-aware function call wrapper
pub const PerformanceFunctionCall = struct {
    hooks: ?*performance_hooks.PerformanceHooks,
    function_name: []const u8,
    module_name: []const u8,
    start_time: ?u64,
    
    pub fn init(function_name: []const u8, module_name: []const u8) PerformanceFunctionCall {
        const hooks_opt = performance_hooks.getGlobalHooks();
        var start_time_opt: ?u64 = null;
        
        if (hooks_opt) |hooks| {
            start_time_opt = hooks.functionCallStart(function_name, module_name);
        }
        
        return PerformanceFunctionCall{
            .hooks = hooks_opt,
            .function_name = function_name,
            .module_name = module_name,
            .start_time = start_time_opt,
        };
    }
    
    pub fn end(self: *PerformanceFunctionCall, args_count: usize, return_size: usize, error_occurred: bool) void {
        if (self.hooks) |hooks| {
            hooks.functionCallEnd(
                self.start_time,
                self.function_name,
                self.module_name,
                args_count,
                return_size,
                error_occurred,
            );
        }
    }
};

/// Performance-aware memory allocator wrapper
pub const PerformanceAllocator = struct {
    child_allocator: Allocator,
    allocation_site: []const u8,
    
    pub fn init(child_allocator: Allocator, allocation_site: []const u8) PerformanceAllocator {
        return PerformanceAllocator{
            .child_allocator = child_allocator,
            .allocation_site = allocation_site,
        };
    }
    
    pub fn allocator(self: *PerformanceAllocator) Allocator {
        return Allocator{
            .ptr = self,
            .vtable = &.{
                .alloc = alloc,
                .resize = resize,
                .free = free,
            },
        };
    }
    
    fn alloc(ctx: *anyopaque, len: usize, ptr_align: u8, ret_addr: usize) ?[*]u8 {
        const self: *PerformanceAllocator = @ptrCast(@alignCast(ctx));
        
        const result = self.child_allocator.rawAlloc(len, ptr_align, ret_addr);
        
        if (result != null) {
            performance_hooks.recordMemoryAllocation(len, @as(usize, @intCast(ptr_align)), self.allocation_site);
        }
        
        return result;
    }
    
    fn resize(ctx: *anyopaque, buf: []u8, buf_align: u8, new_len: usize, ret_addr: usize) bool {
        const self: *PerformanceAllocator = @ptrCast(@alignCast(ctx));
        
        const result = self.child_allocator.rawResize(buf, buf_align, new_len, ret_addr);
        
        if (result) {
            if (new_len > buf.len) {
                performance_hooks.recordMemoryAllocation(new_len - buf.len, @as(usize, @intCast(buf_align)), self.allocation_site);
            } else if (new_len < buf.len) {
                performance_hooks.recordMemoryDeallocation(buf.len - new_len);
            }
        }
        
        return result;
    }
    
    fn free(ctx: *anyopaque, buf: []u8, buf_align: u8, ret_addr: usize) void {
        const self: *PerformanceAllocator = @ptrCast(@alignCast(ctx));
        
        performance_hooks.recordMemoryDeallocation(buf.len);
        self.child_allocator.rawFree(buf, buf_align, ret_addr);
    }
};

/// Performance-aware goroutine launcher
pub fn launchGoroutineWithProfiling(
    scheduler: *concurrency.Scheduler,
    function: anytype,
    args: anytype,
    priority: concurrency.GoroutinePriority,
) !u64 {
    const goroutine_id = try scheduler.spawn(function, args, priority);
    
    // Record goroutine creation
    performance_hooks.recordGoroutineEvent(
        .created,
        goroutine_id,
        0, // worker_id will be assigned by scheduler
        null, // parent_id TODO: track parent goroutine
        8192, // default stack size
        priority,
    );
    
    return goroutine_id;
}

/// Performance-aware channel operations
pub const PerformanceChannel = struct {
    channel_id: u64,
    
    pub fn init(channel_id: u64) PerformanceChannel {
        performance_hooks.recordChannelOperation(
            .created,
            channel_id,
            0, // TODO: get current goroutine ID
            0, // data_size
            0, // queue_length
            0, // duration
        );
        
        return PerformanceChannel{
            .channel_id = channel_id,
        };
    }
    
    pub fn recordSend(self: *PerformanceChannel, data_size: usize, queue_length: usize, duration: u64, blocked: bool) void {
        performance_hooks.recordChannelOperation(
            if (blocked) .blocked_send else .send,
            self.channel_id,
            0, // TODO: get current goroutine ID
            data_size,
            queue_length,
            duration,
        );
    }
    
    pub fn recordReceive(self: *PerformanceChannel, data_size: usize, queue_length: usize, duration: u64, blocked: bool) void {
        performance_hooks.recordChannelOperation(
            if (blocked) .blocked_receive else .receive,
            self.channel_id,
            0, // TODO: get current goroutine ID
            data_size,
            queue_length,
            duration,
        );
    }
    
    pub fn recordClose(self: *PerformanceChannel) void {
        performance_hooks.recordChannelOperation(
            .closed,
            self.channel_id,
            0, // TODO: get current goroutine ID
            0, // data_size
            0, // queue_length
            0, // duration
        );
    }
};

/// Performance-aware error handling
pub fn recordRuntimeError(error_type: performance_hooks.ErrorEventType, message: []const u8, location: []const u8) void {
    performance_hooks.recordError(error_type, message, location);
}

/// Macro for instrumenting function calls
pub fn instrumentFunction(comptime function_name: []const u8, comptime module_name: []const u8) type {
    return struct {
        pub fn call(function: anytype, args: anytype) @TypeOf(@call(.auto, function, args)) {
            var perf_call = PerformanceFunctionCall.init(function_name, module_name);
            defer perf_call.end(
                @typeInfo(@TypeOf(args)).Struct.fields.len,
                @sizeOf(@TypeOf(@call(.auto, function, args))),
                false, // TODO: detect if error occurred
            );
            
            return @call(.auto, function, args);
        }
    };
}

/// Instrumentation for interpreter main loop
pub const InterpreterInstrumentation = struct {
    lexer_calls: PerformanceFunctionCall,
    parser_calls: PerformanceFunctionCall,
    evaluator_calls: PerformanceFunctionCall,
    
    pub fn init() InterpreterInstrumentation {
        return InterpreterInstrumentation{
            .lexer_calls = PerformanceFunctionCall.init("lexer", "core"),
            .parser_calls = PerformanceFunctionCall.init("parser", "core"),
            .evaluator_calls = PerformanceFunctionCall.init("evaluator", "core"),
        };
    }
    
    pub fn startLexing(self: *InterpreterInstrumentation) void {
        self.lexer_calls = PerformanceFunctionCall.init("tokenize", "lexer");
    }
    
    pub fn endLexing(self: *InterpreterInstrumentation, token_count: usize, error_occurred: bool) void {
        self.lexer_calls.end(1, token_count * @sizeOf(u32), error_occurred);
    }
    
    pub fn startParsing(self: *InterpreterInstrumentation) void {
        self.parser_calls = PerformanceFunctionCall.init("parse", "parser");
    }
    
    pub fn endParsing(self: *InterpreterInstrumentation, node_count: usize, error_occurred: bool) void {
        self.parser_calls.end(1, node_count * 64, error_occurred); // Estimate AST node size
    }
    
    pub fn startEvaluation(self: *InterpreterInstrumentation) void {
        self.evaluator_calls = PerformanceFunctionCall.init("evaluate", "evaluator");
    }
    
    pub fn endEvaluation(self: *InterpreterInstrumentation, result_size: usize, error_occurred: bool) void {
        self.evaluator_calls.end(1, result_size, error_occurred);
    }
};

/// Hook integration for VM operations
pub const VMInstrumentation = struct {
    pub fn recordInstruction(opcode: []const u8, operand_count: usize, execution_time: u64) void {
        _ = execution_time; // TODO: Use for timing calculation
        var perf_call = PerformanceFunctionCall.init(opcode, "vm");
        perf_call.end(operand_count, @sizeOf(u64), false);
    }
    
    pub fn recordStackOperation(operation: []const u8, stack_size: usize) void {
        var perf_call = PerformanceFunctionCall.init(operation, "vm_stack");
        perf_call.end(1, stack_size, false);
    }
    
    pub fn recordGarbageCollection(duration: u64, bytes_collected: usize) void {
        _ = duration; // TODO: Use for timing calculation
        performance_hooks.recordMemoryAllocation(0, 0, "gc_collection");
        performance_hooks.recordMemoryDeallocation(bytes_collected);
        
        var perf_call = PerformanceFunctionCall.init("garbage_collect", "gc");
        perf_call.end(0, bytes_collected, false);
    }
};

/// Initialize performance hooks for development vs production
pub fn initializePerformanceHooks(allocator: Allocator, is_production: bool) !void {
    const config = if (is_production) 
        performance_hooks.PerformanceHooksConfig.production() 
    else 
        performance_hooks.PerformanceHooksConfig.development();
        
    try performance_hooks.initGlobalHooks(allocator, config);
    
    print("Performance hooks initialized for {} mode\n", .{if (is_production) "production" else "development"});
}

/// Generate performance report and save to file
pub fn generatePerformanceReport(allocator: Allocator, output_path: []const u8) !void {
    if (performance_hooks.getGlobalHooks()) |hooks| {
        const metrics = try hooks.getCurrentMetrics();
        defer metrics.deinit();
        
        // Create report content
        var report: std.ArrayList(u8) = .empty;
        defer report.deinit();
        
        const writer = report.writer();
        
        try writer.print("# CURSED Performance Report\n\n", .{});
        try writer.print("Generated: {}\n", .{std.time.timestamp()});
        try writer.print("Uptime: {d:.2}s\n\n", .{@as(f64, @floatFromInt(metrics.timestamp)) / 1_000_000_000.0});
        
        try writer.print("## Summary\n", .{});
        try writer.print("- Function Calls: {d}\n", .{metrics.total_function_calls});
        try writer.print("- Memory Allocations: {d}\n", .{metrics.total_memory_allocations});
        try writer.print("- Goroutines Created: {d}\n", .{metrics.total_goroutines_created});
        try writer.print("- Channel Operations: {d}\n", .{metrics.total_channel_operations});
        try writer.print("- Errors: {d}\n", .{metrics.total_errors});
        try writer.print("- Average Function Time: {d:.3}ms\n", .{@as(f64, @floatFromInt(metrics.average_function_time)) / 1_000_000.0});
        
        try writer.print("\n## Hot Paths\n", .{});
        for (metrics.hot_paths, 0..) |hot_path, i| {
            if (i >= 20) break; // Top 20 hot paths
            try writer.print("{}. **{}**: {} calls, {d:.3}ms avg, {d:.1} calls/sec\n", .{
                i + 1,
                hot_path.function_name,
                hot_path.total_calls,
                @as(f64, @floatFromInt(hot_path.average_time)) / 1_000_000.0,
                hot_path.call_frequency,
            });
        }
        
        try writer.print("\n## Bottlenecks\n", .{});
        for (metrics.bottlenecks, 0..) |bottleneck, i| {
            try writer.print("{}. **{}** ({}): {s}\n", .{
                i + 1,
                @tagName(bottleneck.severity),
                @tagName(bottleneck.bottleneck_type),
                bottleneck.description,
            });
            try writer.print("   - Location: {s}\n", .{bottleneck.location});
            try writer.print("   - Impact Score: {d:.2}\n", .{bottleneck.impact_score});
            try writer.print("   - Suggested Fix: {s}\n\n", .{bottleneck.suggested_fix});
        }
        
        try writer.print("## Resource Usage\n", .{});
        try writer.print("- CPU Usage: {d:.1}%\n", .{metrics.resource_usage.cpu_usage_percent});
        try writer.print("- Memory Usage: {d:.2} MB\n", .{@as(f64, @floatFromInt(metrics.resource_usage.memory_usage_bytes)) / 1_048_576.0});
        try writer.print("- Heap Usage: {d:.2} MB\n", .{@as(f64, @floatFromInt(metrics.resource_usage.heap_usage_bytes)) / 1_048_576.0});
        try writer.print("- Goroutines: {d}\n", .{metrics.resource_usage.goroutines_count});
        try writer.print("- Channels: {d}\n", .{metrics.resource_usage.channels_count});
        
        // Write report to file
        const file = try std.fs.cwd().createFile(output_path, .{});
        defer file.close();
        try file.writeAll(report.items);
        
        print("Performance report saved to: {s}\n", .{output_path});
    } else {
        print("Performance hooks not initialized\n", .{});
    }
}

/// Cleanup performance hooks
pub fn cleanupPerformanceHooks() void {
    performance_hooks.deinitGlobalHooks();
    print("Performance hooks cleaned up\n", .{});
}

/// Test function for performance hooks integration
pub fn testPerformanceIntegration(allocator: Allocator) !void {
    // Initialize hooks for development
    try initializePerformanceHooks(allocator, false);
    defer cleanupPerformanceHooks();
    
    // Test function call instrumentation
    {
        var perf_call = PerformanceFunctionCall.init("test_function", "test_module");
        defer perf_call.end(2, 8, false);
        
        // Simulate some work
        std.time.sleep(1_000_000); // 1ms
    }
    
    // Test memory allocation instrumentation
    {
        var perf_allocator = PerformanceAllocator.init(allocator, "test_allocation");
        const allocator_wrapped = perf_allocator.allocator();
        
        const test_memory = try allocator_wrapped.alloc(u8, 1024);
        defer allocator_wrapped.free(test_memory);
    }
    
    // Test error recording
    recordRuntimeError(.runtime_error, "Test error message", "test_location");
    
    // Generate test report
    try generatePerformanceReport(allocator, "performance_test_report.md");
    
    // Print summary
    performance_hooks.printGlobalReport();
    
    print("Performance integration test completed successfully\n", .{});
}
