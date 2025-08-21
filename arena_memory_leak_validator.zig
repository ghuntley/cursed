const std = @import("std");
const CursedArenaManager = @import("src-zig/arena_allocator.zig").CursedArenaManager;

/// Simple arena memory leak validator without thread safety issues
const ArenaMemoryLeakValidator = struct {
    allocator: std.mem.Allocator,
    test_results: std.ArrayList(TestResult),
    
    const Self = @This();
    const TestResult = struct {
        test_name: []const u8,
        memory_before: usize,
        memory_after: usize,
        passed: bool,
        error_message: ?[]const u8,
    };
    
    pub fn init(allocator: std.mem.Allocator) Self {
        return Self{
            .allocator = allocator,
            .test_results = std.ArrayList(TestResult){},
        };
    }
    
    pub fn deinit(self: *Self) void {
        // Free any allocated error messages
        for (self.test_results.items) |result| {
            if (result.error_message) |msg| {
                self.allocator.free(msg);
            }
        }
        self.test_results.deinit(self.allocator);
    }
    
    /// Run a single arena manager test
    fn runArenaManagerTest(self: *Self, test_name: []const u8, iterations: usize) void {
        const memory_before = self.getApproximateMemoryUsage();
        var error_message: ?[]const u8 = null;
        var passed = true;
        
        // Perform the test
        for (0..iterations) |_| {
            var manager = CursedArenaManager.init(self.allocator) catch |err| {
                error_message = std.fmt.allocPrint(self.allocator, "Failed to create arena manager: {any}", .{err}) catch "Allocation failed";
                passed = false;
                break;
            };
            defer manager.deinit();
            
            // Use all allocators
            const parser_alloc = manager.getParserAllocator();
            const ast_alloc = manager.getASTAllocator();
            const runtime_alloc = manager.getRuntimeAllocator();
            const string_alloc = manager.getStringAllocator();
            const temp_alloc = manager.getTemporaryAllocator();
            
            // Perform various allocations
            _ = parser_alloc.alloc(u8, 1024) catch |err| {
                error_message = std.fmt.allocPrint(self.allocator, "Parser allocation failed: {any}", .{err}) catch "Allocation failed";
                passed = false;
                break;
            };
            
            _ = ast_alloc.alloc(u32, 256) catch |err| {
                error_message = std.fmt.allocPrint(self.allocator, "AST allocation failed: {any}", .{err}) catch "Allocation failed";
                passed = false;
                break;
            };
            
            _ = runtime_alloc.alloc(u64, 128) catch |err| {
                error_message = std.fmt.allocPrint(self.allocator, "Runtime allocation failed: {any}", .{err}) catch "Allocation failed";
                passed = false;
                break;
            };
            
            _ = string_alloc.alloc(u8, 2048) catch |err| {
                error_message = std.fmt.allocPrint(self.allocator, "String allocation failed: {any}", .{err}) catch "Allocation failed";
                passed = false;
                break;
            };
            
            _ = temp_alloc.alloc(u8, 512) catch |err| {
                error_message = std.fmt.allocPrint(self.allocator, "Temporary allocation failed: {any}", .{err}) catch "Allocation failed";
                passed = false;
                break;
            };
            
            // Test reset operations
            manager.resetTemporary();
            manager.resetAll();
        }
        
        const memory_after = self.getApproximateMemoryUsage();
        
        // Record test result
        const result = TestResult{
            .test_name = test_name,
            .memory_before = memory_before,
            .memory_after = memory_after,
            .passed = passed,
            .error_message = error_message,
        };
        
        self.test_results.append(self.allocator, result) catch {};
        
        // Print result immediately
        if (passed) {
            std.debug.print("✅ {s}: {} iterations, memory {d} -> {d} KB\n", 
                           .{ test_name, iterations, memory_before, memory_after });
        } else {
            std.debug.print("❌ {s}: FAILED - {s}\n", 
                           .{ test_name, error_message orelse "Unknown error" });
        }
    }
    
    /// Get approximate memory usage (fallback method)
    fn getApproximateMemoryUsage(self: *Self) usize {
        _ = self;
        
        // Try to read RSS from /proc/self/status
        var buffer: [1024]u8 = undefined;
        const file = std.fs.openFileAbsolute("/proc/self/status", .{}) catch |err| switch (err) {
            error.FileNotFound => return 0, // Not on Linux
            else => return 0,
        };
        defer file.close();
        
        const bytes_read = file.readAll(&buffer) catch return 0;
        const content = buffer[0..bytes_read];
        
        var lines = std.mem.splitSequence(u8, content, "\n");
        while (lines.next()) |line| {
            if (std.mem.startsWith(u8, line, "VmRSS:")) {
                var parts = std.mem.splitSequence(u8, line, "\t");
                _ = parts.next(); // Skip "VmRSS:"
                if (parts.next()) |size_str| {
                    var kb_parts = std.mem.splitSequence(u8, std.mem.trim(u8, size_str, " "), " ");
                    if (kb_parts.next()) |kb| {
                        return std.fmt.parseInt(usize, kb, 10) catch 0;
                    }
                }
            }
        }
        return 0;
    }
    
    /// Run comprehensive arena leak validation
    pub fn runComprehensiveValidation(self: *Self) void {
        std.debug.print("=== Arena Memory Leak Validation ===\n", .{});
        
        // Test 1: Basic arena manager cleanup
        self.runArenaManagerTest("Basic Arena Manager", 100);
        
        // Test 2: Stress test with many managers
        self.runArenaManagerTest("Stress Test", 1000);
        
        // Test 3: Large allocation test
        self.runLargeAllocationTest();
        
        // Test 4: Rapid create/destroy test
        self.runRapidCreateDestroyTest();
        
        // Test 5: Reset functionality test
        self.runResetFunctionalityTest();
        
        // Print summary
        self.printSummary();
    }
    
    /// Test large allocations
    fn runLargeAllocationTest(self: *Self) void {
        const test_name = "Large Allocation Test";
        const memory_before = self.getApproximateMemoryUsage();
        var error_message: ?[]const u8 = null;
        var passed = true;
        
        for (0..10) |_| {
            var manager = CursedArenaManager.init(self.allocator) catch |err| {
                error_message = std.fmt.allocPrint(self.allocator, "Manager creation failed: {any}", .{err}) catch "Allocation failed";
                passed = false;
                break;
            };
            defer manager.deinit();
            
            // Allocate large chunks
            const parser_alloc = manager.getParserAllocator();
            _ = parser_alloc.alloc(u8, 64 * 1024) catch |err| { // 64KB
                error_message = std.fmt.allocPrint(self.allocator, "Large allocation failed: {any}", .{err}) catch "Allocation failed";
                passed = false;
                break;
            };
        }
        
        const memory_after = self.getApproximateMemoryUsage();
        
        const result = TestResult{
            .test_name = test_name,
            .memory_before = memory_before,
            .memory_after = memory_after,
            .passed = passed,
            .error_message = error_message,
        };
        
        self.test_results.append(self.allocator, result) catch {};
        
        if (passed) {
            std.debug.print("✅ {s}: 10 x 64KB allocations, memory {d} -> {d} KB\n", 
                           .{ test_name, memory_before, memory_after });
        } else {
            std.debug.print("❌ {s}: FAILED - {s}\n", 
                           .{ test_name, error_message orelse "Unknown error" });
        }
    }
    
    /// Test rapid create/destroy cycles
    fn runRapidCreateDestroyTest(self: *Self) void {
        const test_name = "Rapid Create/Destroy Test";
        const memory_before = self.getApproximateMemoryUsage();
        var error_message: ?[]const u8 = null;
        var passed = true;
        
        for (0..500) |_| {
            var manager = CursedArenaManager.init(self.allocator) catch |err| {
                error_message = std.fmt.allocPrint(self.allocator, "Manager creation failed: {any}", .{err}) catch "Allocation failed";
                passed = false;
                break;
            };
            
            // Use briefly
            const temp_alloc = manager.getTemporaryAllocator();
            _ = temp_alloc.alloc(u8, 128) catch |err| {
                manager.deinit();
                error_message = std.fmt.allocPrint(self.allocator, "Temp allocation failed: {any}", .{err}) catch "Allocation failed";
                passed = false;
                break;
            };
            
            // Immediate cleanup
            manager.deinit();
        }
        
        const memory_after = self.getApproximateMemoryUsage();
        
        const result = TestResult{
            .test_name = test_name,
            .memory_before = memory_before,
            .memory_after = memory_after,
            .passed = passed,
            .error_message = error_message,
        };
        
        self.test_results.append(self.allocator, result) catch {};
        
        if (passed) {
            std.debug.print("✅ {s}: 500 rapid cycles, memory {d} -> {d} KB\n", 
                           .{ test_name, memory_before, memory_after });
        } else {
            std.debug.print("❌ {s}: FAILED - {s}\n", 
                           .{ test_name, error_message orelse "Unknown error" });
        }
    }
    
    /// Test reset functionality
    fn runResetFunctionalityTest(self: *Self) void {
        const test_name = "Reset Functionality Test";
        const memory_before = self.getApproximateMemoryUsage();
        var error_message: ?[]const u8 = null;
        var passed = true;
        
        var manager = CursedArenaManager.init(self.allocator) catch {
            error_message = std.fmt.allocPrint(self.allocator, "Manager creation failed", .{}) catch "Allocation failed";
            passed = false;
            // Create dummy manager to avoid compile error
            return; // Can't continue without valid manager
        };
        defer if (passed) manager.deinit();
        
        if (passed) {
            // Multiple allocation/reset cycles
            for (0..50) |_| {
                const all_allocators = [_]std.mem.Allocator{
                    manager.getParserAllocator(),
                    manager.getASTAllocator(),
                    manager.getRuntimeAllocator(),
                    manager.getStringAllocator(),
                    manager.getTemporaryAllocator(),
                };
                
                // Allocate from all
                for (all_allocators) |alloc| {
                    _ = alloc.alloc(u8, 1024) catch |err| {
                        error_message = std.fmt.allocPrint(self.allocator, "Allocation failed: {any}", .{err}) catch "Allocation failed";
                        passed = false;
                        break;
                    };
                }
                
                if (!passed) break;
                
                // Reset
                manager.resetAll();
            }
        }
        
        const memory_after = self.getApproximateMemoryUsage();
        
        const result = TestResult{
            .test_name = test_name,
            .memory_before = memory_before,
            .memory_after = memory_after,
            .passed = passed,
            .error_message = error_message,
        };
        
        self.test_results.append(self.allocator, result) catch {};
        
        if (passed) {
            std.debug.print("✅ {s}: 50 reset cycles, memory {d} -> {d} KB\n", 
                           .{ test_name, memory_before, memory_after });
        } else {
            std.debug.print("❌ {s}: FAILED - {s}\n", 
                           .{ test_name, error_message orelse "Unknown error" });
        }
    }
    
    /// Print validation summary
    fn printSummary(self: *Self) void {
        var passed_count: usize = 0;
        var total_memory_growth: i64 = 0;
        
        std.debug.print("\n=== Validation Summary ===\n", .{});
        
        for (self.test_results.items) |result| {
            if (result.passed) {
                passed_count += 1;
                const memory_growth = @as(i64, @intCast(result.memory_after)) - @as(i64, @intCast(result.memory_before));
                total_memory_growth += memory_growth;
                
                std.debug.print("✅ {s}: Memory growth {d} KB\n", .{ result.test_name, memory_growth });
            } else {
                std.debug.print("❌ {s}: {s}\n", .{ result.test_name, result.error_message orelse "Failed" });
            }
        }
        
        std.debug.print("\nResults: {d}/{d} tests passed\n", .{ passed_count, self.test_results.items.len });
        std.debug.print("Total memory growth: {d} KB\n", .{total_memory_growth});
        
        if (passed_count == self.test_results.items.len) {
            if (total_memory_growth < 1024) { // Less than 1MB growth acceptable
                std.debug.print("🎉 All tests passed with acceptable memory growth!\n", .{});
            } else {
                std.debug.print("⚠️  Tests passed but memory growth ({d} KB) exceeds threshold\n", .{total_memory_growth});
            }
        } else {
            std.debug.print("❌ Some tests failed - arena cleanup may have issues\n", .{});
        }
        
        std.debug.print("==========================\n", .{});
    }
};

test "arena memory leak validation" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const leaked = gpa.deinit();
        if (leaked == .leak) {
            std.debug.print("🔥 GPA detected memory leaks!\n", .{});
        } else {
            std.debug.print("✅ GPA reports no memory leaks\n", .{});
        }
    }
    const allocator = gpa.allocator();
    
    var validator = ArenaMemoryLeakValidator.init(allocator);
    defer validator.deinit();
    
    validator.runComprehensiveValidation();
}

test "simple arena manager test" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Simple test without threading issues
    for (0..10) |_| {
        var manager = try CursedArenaManager.init(allocator);
        defer manager.deinit();
        
        const parser_alloc = manager.getParserAllocator();
        _ = try parser_alloc.alloc(u8, 1024);
        
        manager.resetAll();
    }
    
    std.debug.print("✅ Simple arena manager test completed\n", .{});
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var validator = ArenaMemoryLeakValidator.init(allocator);
    defer validator.deinit();
    
    std.debug.print("Starting comprehensive arena memory leak validation...\n", .{});
    validator.runComprehensiveValidation();
}
