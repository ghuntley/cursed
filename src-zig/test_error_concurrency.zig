//! Comprehensive Test Suite for Error Handling and Concurrency
//! Tests the complete yikes/fam/shook and stan/dm implementations

const std = @import("std");
const testing = std.testing;
const Allocator = std.mem.Allocator;

const advanced_error_handling = @import("advanced_error_handling.zig");
const advanced_concurrency = @import("advanced_concurrency.zig");
const integration = @import("error_concurrency_integration.zig");

// Test basic error creation and handling
test "yikes error creation and formatting" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    // Test basic error creation
    const error_obj = try advanced_error_handling.CursedError.init(
        allocator,
        "Test error message",
        .Runtime,
        500
    );
    
    // Verify error properties
    try testing.expect(std.mem.eql(u8, error_obj.message, "Test error message"));
    try testing.expect(error_obj.error_type == .Runtime);
    try testing.expect(error_obj.code == 500);
    try testing.expect(error_obj.stack_trace != null);
    
    // Test error context addition
    try error_obj.addContext("operation", "test_function");
    try error_obj.addContext("input", "invalid_value");
    
    try testing.expect(error_obj.context != null);
    try testing.expect(error_obj.context.?.len == 2);
    
    // Test error wrapping
    const wrapped = try error_obj.wrap(allocator, "Operation failed");
    try testing.expect(wrapped.inner_error == error_obj);
    try testing.expect(std.mem.eql(u8, wrapped.message, "Operation failed"));
    
    std.debug.print("Error formatting test: {}\n", .{error_obj.*});
}

// Test error runtime operations
test "error runtime yikes/fam/shook operations" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    const runtime = try advanced_error_handling.ErrorRuntime.init(allocator);
    defer runtime.deinit();
    
    // Test yikes execution
    const error_obj = try runtime.executeYikes("Test runtime error", .Memory, 1001);
    try testing.expect(error_obj.code == 1001);
    try testing.expect(error_obj.error_type == .Memory);
    
    // Test fam block entry/exit
    const recovery_point = try runtime.enterFamBlock(null, null);
    try testing.expect(recovery_point == 0); // First recovery point
    
    runtime.exitFamBlock(recovery_point);
    try testing.expect(runtime.recovery_stack.items.len == 0);
    
    // Test panic recovery (without actual panic)
    const no_panic = runtime.recoverPanic();
    try testing.expect(no_panic == null);
    
    error_obj.deinit();
}

// Test basic goroutine operations  
test "stan goroutine spawning and execution" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    const scheduler = try advanced_concurrency.Scheduler.init(allocator, 2);
    defer scheduler.deinit();
    
    try scheduler.start();
    
    // Test goroutine spawning
    var test_value: i32 = 0;
    const test_context = &test_value;
    
    const goroutine_id = try scheduler.spawn(testGoroutineEntry, test_context);
    try testing.expect(goroutine_id > 0);
    
    // Give goroutine time to execute
    std.time.sleep(50_000_000); // 50ms
    
    try testing.expect(test_value == 42); // Should be set by goroutine
}

fn testGoroutineEntry(context: ?*anyopaque) void {
    if (context) |ctx| {
        const value_ptr = @as(*i32, @ptrCast(@alignCast(ctx)));
        value_ptr.* = 42;
    }
}

// Test dm channel operations
test "dm channel creation and operations" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    // Test unbuffered channel
    const unbuffered = try advanced_concurrency.Channel(i64).init(allocator, 1, 0);
    defer unbuffered.deinit();
    
    try testing.expect(unbuffered.capacity == 0);
    try testing.expect(!unbuffered.isClosed());
    try testing.expect(unbuffered.len() == 0);
    
    // Test buffered channel
    const buffered = try advanced_concurrency.Channel(i64).init(allocator, 2, 5);
    defer buffered.deinit();
    
    try testing.expect(buffered.capacity == 5);
    
    // Test send operations
    const success1 = try buffered.send(100);
    try testing.expect(success1);
    try testing.expect(buffered.len() == 1);
    
    const success2 = try buffered.send(200);
    try testing.expect(success2);
    try testing.expect(buffered.len() == 2);
    
    // Test receive operations
    const value1 = buffered.receive();
    try testing.expect(value1 != null and value1.? == 100);
    try testing.expect(buffered.len() == 1);
    
    const value2 = buffered.receive();
    try testing.expect(value2 != null and value2.? == 200);
    try testing.expect(buffered.len() == 0);
    
    // Test channel closing
    buffered.close();
    try testing.expect(buffered.isClosed());
    
    const closed_send = try buffered.send(300);
    try testing.expect(!closed_send); // Should fail on closed channel
    
    const closed_recv = buffered.receive();
    try testing.expect(closed_recv == null); // Should return null for closed empty channel
}

// Test concurrent channel operations
test "concurrent dm channel operations" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    const runtime = try advanced_concurrency.ConcurrencyRuntime.init(allocator);
    defer runtime.deinit();
    
    const channel = try runtime.dmMake(i64, 3);
    defer channel.deinit();
    
    // Test concurrent send/receive
    const send_context = struct {
        channel: *advanced_concurrency.Channel(i64),
        runtime: *advanced_concurrency.ConcurrencyRuntime,
        
        fn senderFn(ctx: ?*anyopaque) void {
            if (ctx) |c| {
                const context = @as(*@This(), @ptrCast(@alignCast(c)));
                for (0..5) |i| {
                    const success = context.runtime.dmSend(context.channel, @intCast(i)) catch false;
                    if (success) {
                        std.debug.print("Sent: {}\n", .{i});
                    }
                    std.time.sleep(10_000_000); // 10ms
                }
                context.runtime.dmClose(context.channel);
            }
        }
    }{ .channel = channel, .runtime = runtime };
    
    // Spawn sender goroutine
    _ = try runtime.stan(send_context.senderFn, @constCast(&send_context));
    
    // Receive from main thread
    var received_count: usize = 0;
    while (true) {
        if (runtime.dmRecv(channel)) |value| {
            std.debug.print("Received: {}\n", .{value});
            received_count += 1;
        } else {
            break; // Channel closed and empty
        }
        std.time.sleep(5_000_000); // 5ms
    }
    
    std.debug.print("Total received: {}\n", .{received_count});
    try testing.expect(received_count == 5);
}

// Test integrated error handling and concurrency
test "integrated error handling in goroutines" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    try integration.initUnifiedRuntime(allocator);
    defer integration.deinitUnifiedRuntime();
    
    const runtime = integration.getUnifiedRuntime();
    
    // Test yikes in main thread
    const error_result = try integration.executeYikesStatement(
        runtime,
        "Test integrated error",
        .Runtime,
        2001
    );
    
    try testing.expect(error_result.isError());
    try testing.expect(error_result == .Error);
    
    // Test shook operation
    const shook_result = integration.executeShookExpression(runtime, error_result);
    try testing.expect(shook_result.isError());
    
    // Test channel creation and operations
    const channel_result = try integration.createDmChannel(runtime, i64, 3);
    try testing.expect(channel_result == .Channel);
    
    const send_result = try integration.dmSendOperation(
        runtime,
        channel_result,
        integration.InterpreterValue{ .Integer = 123 }
    );
    try testing.expect(send_result == .Boolean);
    try testing.expect(send_result.Boolean == true);
    
    const recv_result = try integration.dmRecvOperation(runtime, channel_result);
    try testing.expect(recv_result == .Integer);
    try testing.expect(recv_result.Integer == 123);
    
    std.debug.print("Integrated test results: send={}, recv={}\n", .{ send_result, recv_result });
}

// Test error propagation in fam blocks
test "fam block error recovery" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    try integration.initUnifiedRuntime(allocator);
    defer integration.deinitUnifiedRuntime();
    
    const runtime = integration.getUnifiedRuntime();
    
    // Test successful fam block
    const success_result = integration.executeFamStatement(
        runtime,
        struct {
            fn tryBlock() integration.InterpreterValue {
                return integration.InterpreterValue{ .Integer = 42 };
            }
        }.tryBlock,
        null
    );
    
    try testing.expect(success_result == .Integer);
    try testing.expect(success_result.Integer == 42);
    
    // Test fam block with error
    const error_result = integration.executeFamStatement(
        runtime,
        struct {
            fn tryBlock() integration.InterpreterValue {
                // Create an error
                const runtime_local = integration.getUnifiedRuntime();
                const error_obj = runtime_local.error_runtime.executeYikes(
                    "Test fam error",
                    .Runtime,
                    3001
                ) catch unreachable;
                return integration.InterpreterValue{ .Error = error_obj };
            }
        }.tryBlock,
        struct {
            fn catchBlock(error_obj: *advanced_error_handling.CursedError) integration.InterpreterValue {
                std.debug.print("Caught error in fam: {}\n", .{error_obj.*});
                return integration.InterpreterValue{ .String = "Error handled" };
            }
        }.catchBlock
    );
    
    try testing.expect(error_result == .String);
    try testing.expect(std.mem.eql(u8, error_result.String, "Error handled"));
}

// Test performance and resource management
test "performance and resource cleanup" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    const start_time = std.time.timestamp();
    
    try integration.initUnifiedRuntime(allocator);
    defer integration.deinitUnifiedRuntime();
    
    const runtime = integration.getUnifiedRuntime();
    
    // Create many errors and ensure proper cleanup
    var errors = std.ArrayList(*advanced_error_handling.CursedError).init(allocator);
    defer {
        for (errors.items) |err| {
            err.deinit();
        }
        errors.deinit();
    }
    
    for (0..100) |i| {
        const error_obj = try runtime.error_runtime.executeYikes(
            "Performance test error",
            .Performance,
            @intCast(i)
        );
        try errors.append(error_obj);
    }
    
    // Create many channels and ensure proper cleanup
    var channels = std.ArrayList(*advanced_concurrency.Channel(i64)).init(allocator);
    defer {
        for (channels.items) |ch| {
            ch.deinit();
        }
        channels.deinit();
    }
    
    for (0..50) |_| {
        const channel = try runtime.concurrency_runtime.dmMake(i64, 5);
        try channels.append(channel);
    }
    
    const end_time = std.time.timestamp();
    const duration = end_time - start_time;
    
    std.debug.print("Performance test completed in {}ms\n", .{duration});
    std.debug.print("Created {} errors and {} channels\n", .{ errors.items.len, channels.items.len });
    
    // Verify resource counts
    try testing.expect(errors.items.len == 100);
    try testing.expect(channels.items.len == 50);
}

// Integration test runner function
pub fn runAllTests(allocator: Allocator) !void {
    std.debug.print("=== Running Comprehensive Error Handling and Concurrency Tests ===\n", .{});
    
    try advanced_error_handling.testErrorHandling(allocator);
    std.debug.print("✓ Error handling test passed\n", .{});
    
    try advanced_concurrency.testConcurrency(allocator);
    std.debug.print("✓ Concurrency test passed\n", .{});
    
    try integration.testIntegration(allocator);
    std.debug.print("✓ Integration test passed\n", .{});
    
    std.debug.print("=== All Tests Passed ===\n", .{});
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    try runAllTests(allocator);
}
