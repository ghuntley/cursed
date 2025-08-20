const std = @import("std");
const testing = std.testing;
const ArenaAllocator = @import("src-zig/arena_allocator.zig").ArenaAllocator;
const CursedArenaManager = @import("src-zig/arena_allocator.zig").CursedArenaManager;

/// Test suite to verify arena allocators are properly cleaned up
/// This test simulates long-running operations to detect memory accumulation
const ArenaMemoryLeakTest = struct {
    allocator: std.mem.Allocator,
    initial_memory: usize,
    
    const Self = @This();
    
    fn init(allocator: std.mem.Allocator) !Self {
        var self = Self{
            .allocator = allocator,
            .initial_memory = 0,
        };
        // Get initial memory usage
        self.initial_memory = try self.getCurrentMemoryUsage();
        return self;
    }
    
    /// Get current memory usage (approximate)
    fn getCurrentMemoryUsage(self: *Self) !usize {
        _ = self;
        var buffer: [256]u8 = undefined;
        const file = std.fs.openFileAbsolute("/proc/self/status", .{}) catch |err| switch (err) {
            error.FileNotFound => return 0, // Not on Linux, can't measure
            else => return err,
        };
        defer file.close();
        
        const bytes_read = try file.readAll(&buffer);
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
    
    /// Test basic arena allocator cleanup
    fn testBasicArenaCleanup(self: *Self) !void {
        const iterations = 1000;
        const alloc_size = 1024;
        
        for (0..iterations) |_| {
            var arena = std.heap.ArenaAllocator.init(self.allocator);
            defer arena.deinit();
            
            const arena_alloc = arena.allocator();
            
            // Allocate various sized chunks
            _ = try arena_alloc.alloc(u8, alloc_size);
            _ = try arena_alloc.alloc(u32, alloc_size / 4);
            _ = try arena_alloc.alloc(u64, alloc_size / 8);
        }
        
        // Note: Cannot easily force GC without allocator instance
    }
    
    /// Test custom arena allocator cleanup
    fn testCustomArenaCleanup(self: *Self) !void {
        const iterations = 500;
        
        for (0..iterations) |_| {
            var arena = try ArenaAllocator.init(
                self.allocator,
                .{ .initial_size = 64 * 1024, .debug_tracking = true },
                .Sequential
            );
            defer arena.deinit();
            
            // Allocate various patterns
            _ = try arena.alloc(1024);
            _ = try arena.alloc(2048);
            _ = try arena.alloc(512);
            
            // Test reset and reuse
            arena.reset();
            _ = try arena.alloc(1024);
        }
    }
    
    /// Test CursedArenaManager cleanup
    fn testArenaManagerCleanup(self: *Self) !void {
        const iterations = 100;
        
        for (0..iterations) |_| {
            var manager = try CursedArenaManager.init(self.allocator);
            defer manager.deinit();
            
            // Use all specialized allocators
            const parser_alloc = manager.getParserAllocator();
            const ast_alloc = manager.getASTAllocator();
            const runtime_alloc = manager.getRuntimeAllocator();
            const string_alloc = manager.getStringAllocator();
            const temp_alloc = manager.getTemporaryAllocator();
            
            _ = try parser_alloc.alloc(u8, 1024);
            _ = try ast_alloc.alloc(u32, 256);
            _ = try runtime_alloc.alloc(u64, 128);
            _ = try string_alloc.alloc(u8, 2048);
            _ = try temp_alloc.alloc(u8, 512);
            
            // Test reset operations
            manager.resetTemporary();
            manager.resetAll();
        }
    }
    
    /// Test stack frame cleanup
    fn testStackFrameCleanup(self: *Self) !void {
        const iterations = 200;
        
        for (0..iterations) |_| {
            var arena = try ArenaAllocator.init(
                self.allocator,
                .{ .initial_size = 32 * 1024 },
                .Stack
            );
            defer arena.deinit();
            
            // Push multiple stack frames
            try arena.pushStackFrame();
            _ = try arena.alloc(512);
            
            try arena.pushStackFrame();
            _ = try arena.alloc(1024);
            
            try arena.pushStackFrame();
            _ = try arena.alloc(256);
            
            // Pop all frames (should cleanup automatically)
            arena.popStackFrame();
            arena.popStackFrame();
            arena.popStackFrame();
        }
    }
    
    /// Test pool allocator cleanup
    fn testPoolCleanup(self: *Self) !void {
        const iterations = 150;
        
        for (0..iterations) |_| {
            var arena = try ArenaAllocator.init(
                self.allocator,
                .{ .initial_size = 64 * 1024 },
                .Pool
            );
            defer arena.deinit();
            
            // Allocate and free pool objects
            const obj1 = try arena.alloc(128);
            const obj2 = try arena.alloc(128);
            const obj3 = try arena.alloc(128);
            
            // Free objects (only supported for pool pattern)
            arena.free(obj1);
            arena.free(obj2);
            arena.free(obj3);
        }
    }
    
    /// Test concurrent arena usage
    fn testConcurrentCleanup(self: *Self) !void {
        const thread_count = 4;
        const iterations_per_thread = 50;
        
        var threads: [thread_count]std.Thread = undefined;
        var results: [thread_count]bool = .{false} ** thread_count;
        
        for (0..thread_count) |i| {
            threads[i] = try std.Thread.spawn(.{}, workerThread, .{ self.allocator, iterations_per_thread, &results[i] });
        }
        
        for (0..thread_count) |i| {
            threads[i].join();
            if (!results[i]) {
                return error.ConcurrentTestFailed;
            }
        }
    }
    
    fn workerThread(allocator: std.mem.Allocator, iterations: usize, result: *bool) void {
        for (0..iterations) |_| {
            var arena = std.heap.ArenaAllocator.init(allocator);
            defer arena.deinit();
            
            const arena_alloc = arena.allocator();
            _ = arena_alloc.alloc(u8, 1024) catch {
                result.* = false;
                return;
            };
        }
        result.* = true;
    }
    
    /// Run comprehensive memory leak test
    fn runComprehensiveTest(self: *Self) !void {
        std.debug.print("Starting arena allocator memory leak tests...\n", .{});
        
        const start_memory = try self.getCurrentMemoryUsage();
        std.debug.print("Initial memory usage: {} KB\n", .{start_memory});
        
        std.debug.print("Testing basic arena cleanup...\n", .{});
        try self.testBasicArenaCleanup();
        const after_basic = try self.getCurrentMemoryUsage();
        std.debug.print("Memory after basic test: {} KB\n", .{after_basic});
        
        std.debug.print("Testing custom arena cleanup...\n", .{});
        try self.testCustomArenaCleanup();
        const after_custom = try self.getCurrentMemoryUsage();
        std.debug.print("Memory after custom test: {} KB\n", .{after_custom});
        
        std.debug.print("Testing arena manager cleanup...\n", .{});
        try self.testArenaManagerCleanup();
        const after_manager = try self.getCurrentMemoryUsage();
        std.debug.print("Memory after manager test: {} KB\n", .{after_manager});
        
        std.debug.print("Testing stack frame cleanup...\n", .{});
        try self.testStackFrameCleanup();
        const after_stack = try self.getCurrentMemoryUsage();
        std.debug.print("Memory after stack test: {} KB\n", .{after_stack});
        
        std.debug.print("Testing pool cleanup...\n", .{});
        try self.testPoolCleanup();
        const after_pool = try self.getCurrentMemoryUsage();
        std.debug.print("Memory after pool test: {} KB\n", .{after_pool});
        
        std.debug.print("Testing concurrent cleanup...\n", .{});
        try self.testConcurrentCleanup();
        const after_concurrent = try self.getCurrentMemoryUsage();
        std.debug.print("Memory after concurrent test: {} KB\n", .{after_concurrent});
        
        // Allow some margin for measurement error
        const memory_growth = after_concurrent - start_memory;
        const acceptable_growth = 1024; // 1MB acceptable growth
        
        if (memory_growth > acceptable_growth) {
            std.debug.print("ERROR: Memory growth of {} KB exceeds acceptable limit of {} KB\n", .{ memory_growth, acceptable_growth });
            return error.MemoryLeak;
        }
        
        std.debug.print("✓ Arena allocator memory leak test passed! Memory growth: {} KB\n", .{memory_growth});
    }
};

test "arena allocator memory leak detection" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var leak_test = try ArenaMemoryLeakTest.init(allocator);
    try leak_test.runComprehensiveTest();
}

test "single arena basic cleanup" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const leaked = gpa.deinit();
        if (leaked == .leak) {
            std.debug.print("ERROR: Memory leak detected in basic arena cleanup test!\n", .{});
            std.testing.expect(false) catch unreachable;
        }
    }
    const allocator = gpa.allocator();
    
    var arena = std.heap.ArenaAllocator.init(allocator);
    defer arena.deinit();
    
    const arena_alloc = arena.allocator();
    _ = try arena_alloc.alloc(u8, 1024);
    _ = try arena_alloc.alloc(u32, 256);
}

test "custom arena manager cleanup" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const leaked = gpa.deinit();
        if (leaked == .leak) {
            std.debug.print("ERROR: Memory leak detected in custom arena manager test!\n", .{});
            std.testing.expect(false) catch unreachable;
        }
    }
    const allocator = gpa.allocator();
    
    var manager = try CursedArenaManager.init(allocator);
    defer manager.deinit();
    
    // Use all allocators
    const parser_alloc = manager.getParserAllocator();
    const ast_alloc = manager.getASTAllocator();
    const runtime_alloc = manager.getRuntimeAllocator();
    const string_alloc = manager.getStringAllocator();
    const temp_alloc = manager.getTemporaryAllocator();
    
    _ = try parser_alloc.alloc(u8, 1024);
    _ = try ast_alloc.alloc(u32, 256);
    _ = try runtime_alloc.alloc(u64, 128);
    _ = try string_alloc.alloc(u8, 2048);
    _ = try temp_alloc.alloc(u8, 512);
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var leak_test = try ArenaMemoryLeakTest.init(allocator);
    try leak_test.runComprehensiveTest();
}
