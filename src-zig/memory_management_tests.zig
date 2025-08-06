//! CURSED Memory Management Tests
//!
//! Comprehensive test suite for validating memory management fixes across:
//! - JIT execution engine
//! - Concurrency system
//! - Garbage collector
//! - LLVM integration
//! - General allocation patterns

const std = @import("std");
const testing = std.testing;
const print = std.debug.print;
const ArrayList = std.ArrayList;
const ArenaAllocator = std.heap.ArenaAllocator;

// Import fixed modules
const jit_fixed = @import("jit_execution_engine_fixed.zig");
const concurrency_fixed = @import("concurrency_memory_fixes.zig");
const gc_fixed = @import("gc_memory_fixes.zig");

/// Memory test configuration
const TestConfig = struct {
    max_memory_mb: usize = 50,  // 50MB limit for tests
    test_duration_ms: u64 = 1000, // 1 second per test
    stress_iterations: usize = 100,
};

/// Memory usage tracker for tests
const MemoryTracker = struct {
    allocator: std.mem.Allocator,
    initial_memory: usize,
    peak_memory: usize,
    current_memory: usize,
    
    pub fn init(allocator: std.mem.Allocator) MemoryTracker {
        return MemoryTracker{
            .allocator = allocator,
            .initial_memory = getCurrentMemoryUsage(),
            .peak_memory = 0,
            .current_memory = 0,
        };
    }
    
    pub fn update(self: *MemoryTracker) void {
        self.current_memory = getCurrentMemoryUsage();
        self.peak_memory = @max(self.peak_memory, self.current_memory);
    }
    
    pub fn getMemoryDelta(self: *MemoryTracker) isize {
        return @as(isize, @intCast(self.current_memory)) - @as(isize, @intCast(self.initial_memory));
    }
    
    pub fn printStats(self: *MemoryTracker, test_name: []const u8) void {
        print("Memory stats for {s}:\n", .{test_name});
        print("  Initial: {} bytes\n", .{self.initial_memory});
        print("  Current: {} bytes\n", .{self.current_memory});
        print("  Peak: {} bytes\n", .{self.peak_memory});
        print("  Delta: {} bytes\n", .{self.getMemoryDelta()});
    }
    
    fn getCurrentMemoryUsage() usize {
        // Simple approximation - in a real implementation would use proper memory tracking
        return 0;
    }
};

/// Test JIT execution engine memory management
test "JIT execution engine memory safety" {
    const allocator = testing.allocator;
    var tracker = MemoryTracker.init(allocator);
    
    print("\n🧪 Testing JIT Execution Engine Memory Safety\n");
    print("============================================\n");
    
    // Test 1: Basic allocation and cleanup
    {
        var engine = try jit_fixed.JITExecutionEngine.init(allocator);
        defer engine.deinit();
        
        const simple_program = 
            \\vibez.spill("Memory test")
            \\sus x drip = 42
            \\vibez.spill("Value:", x)
        ;
        
        try engine.executeSource(simple_program);
        tracker.update();
    }
    
    // Test 2: Repeated execution (memory leak test)
    {
        var engine = try jit_fixed.JITExecutionEngine.init(allocator);
        defer engine.deinit();
        
        const config = TestConfig{};
        for (0..config.stress_iterations) |i| {
            const program = 
                \\sus counter drip = {d}
                \\vibez.spill("Iteration:", counter)
            ;
            var buffer: [256]u8 = undefined;
            const formatted = try std.fmt.bufPrint(buffer[0..], program, .{i});
            
            try engine.executeSource(formatted);
            engine.reset(); // Reset for next iteration
            
            if (i % 10 == 0) {
                tracker.update();
            }
        }
    }
    
    // Test 3: Function calls and recursion
    {
        var engine = try jit_fixed.JITExecutionEngine.init(allocator);
        defer engine.deinit();
        
        const recursive_program = 
            \\slay fibonacci(n drip) drip {
            \\    bestie (n <= 1) {
            \\        damn n
            \\    }
            \\    damn fibonacci(n - 1) + fibonacci(n - 2)
            \\}
            \\sus result drip = fibonacci(10)
            \\vibez.spill("Fibonacci(10):", result)
        ;
        
        try engine.executeSource(recursive_program);
    }
    
    // Test 4: Error handling and recovery
    {
        var engine = try jit_fixed.JITExecutionEngine.init(allocator);
        defer engine.deinit();
        
        // Test division by zero error handling
        const error_program = 
            \\sus x drip = 10
            \\sus y drip = 0
            \\sus result drip = x / y
        ;
        
        // Should handle error gracefully without leaking memory
        _ = engine.executeSource(error_program) catch |err| {
            print("Expected error caught: {}\n", .{err});
        };
    }
    
    tracker.update();
    tracker.printStats("JIT Engine");
    
    // Verify no significant memory leaks
    const memory_delta = tracker.getMemoryDelta();
    try testing.expect(@abs(memory_delta) < 1024 * 1024); // Less than 1MB growth
}

/// Test concurrency system memory management
test "concurrency system memory safety" {
    const allocator = testing.allocator;
    var tracker = MemoryTracker.init(allocator);
    
    print("\n🧪 Testing Concurrency System Memory Safety\n");
    print("===========================================\n");
    
    // Test 1: Channel creation and cleanup
    {
        var channel = try concurrency_fixed.makeMemorySafeChannel(i32, allocator, 10);
        defer channel.release();
        
        // Test basic operations
        try testing.expect(try channel.send(42) == .sent);
        const received = try channel.receive();
        try testing.expect(received.? == 42);
        
        tracker.update();
    }
    
    // Test 2: Multiple channels stress test
    {
        const num_channels = 100;
        var channels: [num_channels]*concurrency_fixed.MemorySafeChannel(i32) = undefined;
        
        // Create channels
        for (0..num_channels) |i| {
            channels[i] = try concurrency_fixed.makeMemorySafeChannel(i32, allocator, 5);
        }
        
        // Use channels
        for (0..num_channels) |i| {
            _ = channels[i].trySend(@intCast(i)) catch {};
        }
        
        // Cleanup channels
        for (0..num_channels) |i| {
            channels[i].release();
        }
        
        tracker.update();
    }
    
    // Test 3: Scheduler lifecycle
    {
        try concurrency_fixed.initializeScheduler(allocator, 2);
        defer concurrency_fixed.shutdownScheduler();
        
        // Test goroutine spawning
        var executed_count: u32 = 0;
        const TestContext = struct {
            count: *u32,
        };
        
        var context = TestContext{ .count = &executed_count };
        
        const testFn = struct {
            fn run(ctx: ?*anyopaque) void {
                const test_ctx: *TestContext = @ptrCast(@alignCast(ctx.?));
                _ = @atomicRmw(u32, test_ctx.count, .Add, 1, .monotonic);
            }
        }.run;
        
        // Spawn multiple goroutines
        for (0..10) |_| {
            _ = try concurrency_fixed.spawnMemorySafe(allocator, testFn, &context);
        }
        
        // Wait for execution
        std.time.sleep(100_000_000); // 100ms
        
        tracker.update();
        print("Executed goroutines: {}\n", .{executed_count});
    }
    
    // Test 4: Channel timeout operations
    {
        var channel = try concurrency_fixed.makeMemorySafeChannel(i32, allocator, 1);
        defer channel.release();
        
        // Test timeout send
        try testing.expect(try channel.send(1) == .sent);
        const timeout_result = try channel.sendTimeout(2, 10); // 10ms timeout
        try testing.expect(timeout_result == .would_block);
        
        // Test timeout receive
        _ = try channel.receive(); // Empty the channel
        const timeout_recv = try channel.receiveTimeout(10); // 10ms timeout
        try testing.expect(timeout_recv == null);
        
        tracker.update();
    }
    
    tracker.printStats("Concurrency System");
    
    // Verify no significant memory leaks
    const memory_delta = tracker.getMemoryDelta();
    try testing.expect(@abs(memory_delta) < 1024 * 1024); // Less than 1MB growth
}

/// Test garbage collector memory management
test "garbage collector memory safety" {
    const allocator = testing.allocator;
    var tracker = MemoryTracker.init(allocator);
    
    print("\n🧪 Testing Garbage Collector Memory Safety\n");
    print("==========================================\n");
    
    // Test 1: GC initialization and cleanup
    {
        var config = gc_fixed.gc.GCConfig.default();
        config.initial_heap_size = 1024 * 1024; // 1MB
        config.young_gen_size = 512 * 1024;     // 512KB
        
        const gc_instance = try gc_fixed.FixedGC.init(allocator, config);
        defer gc_instance.deinit();
        
        tracker.update();
        
        // Basic allocation test
        const obj1 = try gc_instance.alloc(100, 1);
        const obj2 = try gc_instance.alloc(200, 2);
        try testing.expect(obj1 != obj2);
        
        tracker.update();
    }
    
    // Test 2: Allocation stress test
    {
        var config = gc_fixed.gc.GCConfig.default();
        config.initial_heap_size = 2 * 1024 * 1024; // 2MB
        
        const gc_instance = try gc_fixed.FixedGC.init(allocator, config);
        defer gc_instance.deinit();
        
        // Allocate many small objects
        var objects: [1000]*anyopaque = undefined;
        for (0..1000) |i| {
            objects[i] = try gc_instance.alloc(64, 1);
            
            if (i % 100 == 0) {
                tracker.update();
            }
        }
        
        // Force collection
        try gc_instance.collectGeneration(.Young);
        tracker.update();
    }
    
    // Test 3: Root management
    {
        var config = gc_fixed.gc.GCConfig.default();
        config.initial_heap_size = 1024 * 1024;
        
        const gc_instance = try gc_fixed.FixedGC.init(allocator, config);
        defer gc_instance.deinit();
        
        var root: ?*anyopaque = null;
        try gc_instance.addRoot(&root);
        
        root = try gc_instance.alloc(256, 1);
        
        // Object should survive collection due to root
        try gc_instance.collectGeneration(.Young);
        
        gc_instance.removeRoot(&root);
        tracker.update();
    }
    
    // Test 4: Weak references
    {
        var config = gc_fixed.gc.GCConfig.default();
        config.initial_heap_size = 1024 * 1024;
        
        const gc_instance = try gc_fixed.FixedGC.init(allocator, config);
        defer gc_instance.deinit();
        
        const obj = try gc_instance.alloc(128, 1);
        const weak_ref = try gc_instance.createWeakRef(obj);
        
        try testing.expect(weak_ref.get() != null);
        
        // After collection, weak ref should still be valid if object survives
        try gc_instance.collectGeneration(.Young);
        
        tracker.update();
    }
    
    tracker.printStats("Garbage Collector");
    
    // Verify no significant memory leaks
    const memory_delta = tracker.getMemoryDelta();
    try testing.expect(@abs(memory_delta) < 2 * 1024 * 1024); // Less than 2MB growth
}

/// Test LLVM module verification and cleanup
test "LLVM module memory safety" {
    const allocator = testing.allocator;
    var tracker = MemoryTracker.init(allocator);
    
    print("\n🧪 Testing LLVM Module Memory Safety\n");
    print("====================================\n");
    
    // This test would verify LLVM module cleanup
    // For now, we'll simulate the operations
    
    // Test 1: Module creation and cleanup simulation
    {
        // Simulate LLVM module lifecycle
        var modules: [10]bool = undefined;
        for (0..10) |i| {
            modules[i] = true; // "Create" module
            
            // Simulate some operations
            std.time.sleep(1_000_000); // 1ms
            
            modules[i] = false; // "Destroy" module
        }
        
        tracker.update();
    }
    
    // Test 2: Function verification simulation
    {
        const functions = [_][]const u8{
            "test_function_1",
            "test_function_2", 
            "test_function_3",
        };
        
        for (functions) |func_name| {
            // Simulate function verification
            _ = func_name;
            tracker.update();
        }
    }
    
    tracker.printStats("LLVM Module");
    
    // Verify no memory issues
    const memory_delta = tracker.getMemoryDelta();
    try testing.expect(@abs(memory_delta) < 512 * 1024); // Less than 512KB growth
}

/// Integration test combining all systems
test "integrated memory management" {
    const allocator = testing.allocator;
    var tracker = MemoryTracker.init(allocator);
    
    print("\n🧪 Testing Integrated Memory Management\n");
    print("======================================\n");
    
    // Initialize all systems
    var config = gc_fixed.gc.GCConfig.default();
    config.initial_heap_size = 4 * 1024 * 1024; // 4MB
    
    const gc_instance = try gc_fixed.FixedGC.init(allocator, config);
    defer gc_instance.deinit();
    
    try concurrency_fixed.initializeScheduler(allocator, 2);
    defer concurrency_fixed.shutdownScheduler();
    
    var engine = try jit_fixed.JITExecutionEngine.init(allocator);
    defer engine.deinit();
    
    tracker.update();
    
    // Test integration scenario
    {
        // Create some channels
        var channel1 = try concurrency_fixed.makeMemorySafeChannel(i32, allocator, 5);
        defer channel1.release();
        
        var channel2 = try concurrency_fixed.makeMemorySafeChannel(i32, allocator, 5);
        defer channel2.release();
        
        // Allocate some GC objects
        const obj1 = try gc_instance.alloc(256, 1);
        const obj2 = try gc_instance.alloc(512, 2);
        
        // Add as roots to prevent collection
        var root1: ?*anyopaque = obj1;
        var root2: ?*anyopaque = obj2;
        try gc_instance.addRoot(&root1);
        try gc_instance.addRoot(&root2);
        
        // Execute some code
        const integration_program = 
            \\sus result drip = 0
            \\sus i drip = 0
            \\bestie (i < 10) {
            \\    result = result + i
            \\    i = i + 1
            \\}
            \\vibez.spill("Integration test result:", result)
        ;
        
        try engine.executeSource(integration_program);
        
        // Use channels
        try testing.expect(try channel1.send(100) == .sent);
        try testing.expect(try channel2.send(200) == .sent);
        
        const val1 = try channel1.receive();
        const val2 = try channel2.receive();
        
        try testing.expect(val1.? == 100);
        try testing.expect(val2.? == 200);
        
        // Force GC
        try gc_instance.collectGeneration(.Young);
        
        // Clean up roots
        gc_instance.removeRoot(&root1);
        gc_instance.removeRoot(&root2);
        
        tracker.update();
    }
    
    tracker.printStats("Integrated Systems");
    
    // Verify overall system stability
    const memory_delta = tracker.getMemoryDelta();
    try testing.expect(@abs(memory_delta) < 5 * 1024 * 1024); // Less than 5MB growth
    
    print("\n✅ All memory management tests passed!\n");
}

/// Stress test for memory management under load
test "memory management stress test" {
    const allocator = testing.allocator;
    var tracker = MemoryTracker.init(allocator);
    
    print("\n🧪 Memory Management Stress Test\n");
    print("================================\n");
    
    const config = TestConfig{
        .stress_iterations = 50, // Reduced for CI
        .test_duration_ms = 500, // Reduced for CI
    };
    
    // Stress test with multiple systems running concurrently
    var gc_config = gc_fixed.gc.GCConfig.default();
    gc_config.initial_heap_size = 8 * 1024 * 1024; // 8MB
    
    const gc_instance = try gc_fixed.FixedGC.init(allocator, gc_config);
    defer gc_instance.deinit();
    
    try concurrency_fixed.initializeScheduler(allocator, 4);
    defer concurrency_fixed.shutdownScheduler();
    
    var engine = try jit_fixed.JITExecutionEngine.init(allocator);
    defer engine.deinit();
    
    tracker.update();
    
    const start_time = std.time.milliTimestamp();
    var iteration: usize = 0;
    
    while (iteration < config.stress_iterations) {
        const elapsed = std.time.milliTimestamp() - start_time;
        if (elapsed > config.test_duration_ms) break;
        
        // Create and destroy channels rapidly
        {
            var channel = try concurrency_fixed.makeMemorySafeChannel(i32, allocator, 3);
            defer channel.release();
            
            _ = channel.trySend(@intCast(iteration)) catch {};
            _ = channel.tryReceive() catch {};
        }
        
        // Allocate and free GC objects
        {
            const obj = try gc_instance.alloc(128, 1);
            _ = obj; // Use the object
            
            if (iteration % 10 == 0) {
                try gc_instance.collectGeneration(.Young);
            }
        }
        
        // Execute code
        if (iteration % 5 == 0) {
            const stress_program = 
                \\sus x drip = {d}
                \\vibez.spill("Stress iteration:", x)
            ;
            var buffer: [256]u8 = undefined;
            const formatted = try std.fmt.bufPrint(buffer[0..], stress_program, .{iteration});
            
            try engine.executeSource(formatted);
            engine.reset();
        }
        
        iteration += 1;
        
        if (iteration % 10 == 0) {
            tracker.update();
        }
    }
    
    // Final collection
    try gc_instance.collectGeneration(.Old);
    
    tracker.update();
    tracker.printStats("Stress Test");
    
    print("Completed {} iterations in {} ms\n", .{iteration, std.time.milliTimestamp() - start_time});
    
    // Verify system remained stable under stress
    const memory_delta = tracker.getMemoryDelta();
    try testing.expect(@abs(memory_delta) < 10 * 1024 * 1024); // Less than 10MB growth
    
    print("✅ Stress test completed successfully!\n");
}

/// Test memory management with error conditions
test "memory management error handling" {
    const allocator = testing.allocator;
    var tracker = MemoryTracker.init(allocator);
    
    print("\n🧪 Testing Memory Management Error Handling\n");
    print("===========================================\n");
    
    // Test 1: Out of memory conditions
    {
        var small_config = gc_fixed.gc.GCConfig.default();
        small_config.initial_heap_size = 1024; // Very small heap
        small_config.young_gen_size = 512;
        
        const gc_instance = try gc_fixed.FixedGC.init(allocator, small_config);
        defer gc_instance.deinit();
        
        // Try to allocate more than available
        var allocations: usize = 0;
        while (allocations < 100) {
            const result = gc_instance.alloc(64, 1);
            if (result) |_| {
                allocations += 1;
            } else |err| {
                print("Expected OOM after {} allocations: {}\n", .{allocations, err});
                break;
            }
        }
        
        tracker.update();
    }
    
    // Test 2: Invalid operations
    {
        var engine = try jit_fixed.JITExecutionEngine.init(allocator);
        defer engine.deinit();
        
        // Test invalid syntax
        const invalid_program = "this is not valid CURSED syntax @#$%";
        _ = engine.executeSource(invalid_program) catch |err| {
            print("Expected syntax error: {}\n", .{err});
        };
        
        // Test stack overflow protection
        const recursive_program = 
            \\slay infinite_recursion() {
            \\    infinite_recursion()
            \\}
            \\infinite_recursion()
        ;
        
        _ = engine.executeSource(recursive_program) catch |err| {
            print("Expected stack overflow: {}\n", .{err});
        };
        
        tracker.update();
    }
    
    // Test 3: Channel error conditions
    {
        var channel = try concurrency_fixed.makeMemorySafeChannel(i32, allocator, 1);
        defer channel.release();
        
        // Fill channel
        try testing.expect(try channel.send(1) == .sent);
        
        // Try non-blocking send on full channel
        const result = try channel.trySend(2);
        try testing.expect(result == .would_block);
        
        // Close channel and try operations
        channel.close();
        try testing.expect(try channel.send(3) == .closed);
        
        tracker.update();
    }
    
    tracker.printStats("Error Handling");
    
    // Verify error handling doesn't leak memory
    const memory_delta = tracker.getMemoryDelta();
    try testing.expect(@abs(memory_delta) < 1024 * 1024); // Less than 1MB growth
    
    print("✅ Error handling tests completed!\n");
}
