const std = @import("std");
const builtin = @import("builtin");
const CursedArenaManager = @import("src-zig/arena_allocator.zig").CursedArenaManager;

/// Automatic arena cleanup system with scope-based memory management
const AutomaticArenaCleanup = struct {
    allocator: std.mem.Allocator,
    active_managers: std.ArrayList(*CursedArenaManager),
    cleanup_on_scope_exit: bool,
    
    const Self = @This();
    
    pub fn init(allocator: std.mem.Allocator) Self {
        return Self{
            .allocator = allocator,
            .active_managers = std.ArrayList(*CursedArenaManager).init(allocator),
            .cleanup_on_scope_exit = true,
        };
    }
    
    pub fn deinit(self: *Self) void {
        // Clean up all active arena managers
        for (self.active_managers.items) |manager| {
            manager.deinit();
            self.allocator.destroy(manager);
        }
        self.active_managers.deinit();
    }
    
    /// Create a managed arena manager that will be automatically cleaned up
    pub fn createManagedArenaManager(self: *Self) !*CursedArenaManager {
        const manager = try self.allocator.create(CursedArenaManager);
        manager.* = try CursedArenaManager.init(self.allocator);
        try self.active_managers.append(manager);
        return manager;
    }
    
    /// Cleanup specific arena manager
    pub fn cleanupArenaManager(self: *Self, manager: *CursedArenaManager) void {
        for (self.active_managers.items, 0..) |item, i| {
            if (item == manager) {
                manager.deinit();
                self.allocator.destroy(manager);
                _ = self.active_managers.orderedRemove(i);
                return;
            }
        }
    }
    
    /// Reset all arena managers (but keep them allocated)
    pub fn resetAllArenaManagers(self: *Self) void {
        for (self.active_managers.items) |manager| {
            manager.resetAll();
        }
    }
    
    /// Reset only temporary arenas (frequent operation)
    pub fn resetTemporaryArenas(self: *Self) void {
        for (self.active_managers.items) |manager| {
            manager.resetTemporary();
        }
    }
};

/// RAII Arena Manager wrapper for automatic cleanup
const ScopedArenaManager = struct {
    manager: CursedArenaManager,
    
    const SelfScoped = @This();
    
    pub fn init(allocator: std.mem.Allocator) !SelfScoped {
        return SelfScoped{
            .manager = try CursedArenaManager.init(allocator),
        };
    }
        
        pub fn deinit(self: *SelfScoped) void {
            self.manager.deinit();
        }
        
        // Delegate all methods to the underlying manager
        pub fn getParserAllocator(self: *SelfScoped) std.mem.Allocator {
            return self.manager.getParserAllocator();
        }
        
        pub fn getASTAllocator(self: *SelfScoped) std.mem.Allocator {
            return self.manager.getASTAllocator();
        }
        
        pub fn getRuntimeAllocator(self: *SelfScoped) std.mem.Allocator {
            return self.manager.getRuntimeAllocator();
        }
        
        pub fn getStringAllocator(self: *SelfScoped) std.mem.Allocator {
            return self.manager.getStringAllocator();
        }
        
        pub fn getTemporaryAllocator(self: *SelfScoped) std.mem.Allocator {
            return self.manager.getTemporaryAllocator();
        }
        
        pub fn resetAll(self: *SelfScoped) void {
            self.manager.resetAll();
        }
        
        pub fn resetTemporary(self: *SelfScoped) void {
            self.manager.resetTemporary();
        }
        
        pub fn getTotalUsage(self: *SelfScoped) @TypeOf(self.manager.getTotalUsage()) {
            return self.manager.getTotalUsage();
        }
    };

/// Long-running arena manager with periodic cleanup
const LongRunningArenaManager = struct {
    cleanup_system: AutomaticArenaCleanup,
    cleanup_interval_ms: u64,
    last_cleanup_time: i64,
    cleanup_threshold_bytes: usize,
    
    const Self = @This();
    
    pub fn init(allocator: std.mem.Allocator, cleanup_interval_ms: u64, cleanup_threshold_bytes: usize) Self {
        return Self{
            .cleanup_system = AutomaticArenaCleanup.init(allocator),
            .cleanup_interval_ms = cleanup_interval_ms,
            .last_cleanup_time = std.time.milliTimestamp(),
            .cleanup_threshold_bytes = cleanup_threshold_bytes,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.cleanup_system.deinit();
    }
    
    /// Create managed arena with automatic cleanup
    pub fn createArenaManager(self: *Self) !*CursedArenaManager {
        self.maybePerformCleanup();
        return self.cleanup_system.createManagedArenaManager();
    }
    
    /// Check if periodic cleanup should be performed
    fn maybePerformCleanup(self: *Self) void {
        const current_time = std.time.milliTimestamp();
        const time_since_cleanup = current_time - self.last_cleanup_time;
        
        if (time_since_cleanup > self.cleanup_interval_ms) {
            self.performPeriodicCleanup();
            self.last_cleanup_time = current_time;
        }
    }
    
    /// Perform periodic cleanup of temporary arenas
    fn performPeriodicCleanup(self: *Self) void {
        std.debug.print("Performing periodic arena cleanup...\n", .{});
        
        // Check memory usage and decide cleanup strategy
        var total_memory_used: usize = 0;
        for (self.cleanup_system.active_managers.items) |manager| {
            const usage = manager.getTotalUsage();
            total_memory_used += usage.total_used;
        }
        
        if (total_memory_used > self.cleanup_threshold_bytes) {
            std.debug.print("Memory usage ({} bytes) exceeds threshold, resetting all arenas\n", .{total_memory_used});
            self.cleanup_system.resetAllArenaManagers();
        } else {
            std.debug.print("Resetting temporary arenas only\n", .{});
            self.cleanup_system.resetTemporaryArenas();
        }
    }
    
    /// Force immediate cleanup
    pub fn forceCleanup(self: *Self) void {
        self.performPeriodicCleanup();
        self.last_cleanup_time = std.time.milliTimestamp();
    }
    
    /// Get statistics about managed arenas
    pub fn getStatistics(self: *Self) struct {
        manager_count: usize,
        total_allocated: usize,
        total_used: usize,
        time_since_cleanup: i64,
    } {
        var total_allocated: usize = 0;
        var total_used: usize = 0;
        
        for (self.cleanup_system.active_managers.items) |manager| {
            const usage = manager.getTotalUsage();
            total_allocated += usage.total_allocated;
            total_used += usage.total_used;
        }
        
        return .{
            .manager_count = self.cleanup_system.active_managers.items.len,
            .total_allocated = total_allocated,
            .total_used = total_used,
            .time_since_cleanup = std.time.milliTimestamp() - self.last_cleanup_time,
        };
    }
};

/// Test validation functions
fn validateArenaCleanup() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const leaked = gpa.deinit();
        if (leaked == .leak) {
            std.debug.print("ERROR: Memory leak detected!\n", .{});
        } else {
            std.debug.print("✓ No memory leaks detected\n", .{});
        }
    }
    const allocator = gpa.allocator();
    
    std.debug.print("=== Arena Cleanup Validation ===\n", .{});
    
    // Test 1: Automatic cleanup system
    std.debug.print("Testing automatic cleanup system...\n", .{});
    {
        var cleanup = AutomaticArenaCleanup.init(allocator);
        defer cleanup.deinit();
        
        const manager1 = try cleanup.createManagedArenaManager();
        const manager2 = try cleanup.createManagedArenaManager();
        
        // Use the managers
        const alloc1 = manager1.getParserAllocator();
        const alloc2 = manager2.getASTAllocator();
        
        _ = try alloc1.alloc(u8, 1024);
        _ = try alloc2.alloc(u32, 256);
        
        std.debug.print("Created and used 2 managed arena managers\n", .{});
        // cleanup.deinit() will automatically clean up both managers
    }
    
    // Test 2: RAII scoped manager
    std.debug.print("Testing RAII scoped manager...\n", .{});
    {
        var scoped = try ScopedArenaManager.init(allocator);
        defer scoped.deinit();
        
        const parser_alloc = scoped.getParserAllocator();
        const ast_alloc = scoped.getASTAllocator();
        
        _ = try parser_alloc.alloc(u8, 2048);
        _ = try ast_alloc.alloc(u64, 128);
        
        std.debug.print("Used scoped arena manager\n", .{});
        // scoped.deinit() will be called automatically
    }
    
    // Test 3: Long-running manager with periodic cleanup
    std.debug.print("Testing long-running manager...\n", .{});
    {
        var long_running = LongRunningArenaManager.init(
            allocator,
            1000, // 1 second cleanup interval
            64 * 1024 // 64KB cleanup threshold
        );
        defer long_running.deinit();
        
        // Simulate multiple operations over time
        for (0..10) |i| {
            const manager = try long_running.createArenaManager();
            const temp_alloc = manager.getTemporaryAllocator();
            _ = try temp_alloc.alloc(u8, 1024 * (i + 1));
            
            if (i % 3 == 0) {
                // Simulate passage of time
                std.time.sleep(10 * std.time.ns_per_ms);
                long_running.forceCleanup();
            }
        }
        
        const stats = long_running.getStatistics();
        std.debug.print("Long-running stats: {} managers, {} bytes allocated, {} bytes used\n", 
                       .{stats.manager_count, stats.total_allocated, stats.total_used});
    }
    
    std.debug.print("✓ All arena cleanup tests passed\n", .{});
}

/// Stress test for arena cleanup under load
fn stressTestArenaCleanup() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const leaked = gpa.deinit();
        if (leaked == .leak) {
            std.debug.print("ERROR: Memory leak detected in stress test!\n", .{});
        } else {
            std.debug.print("✓ No memory leaks in stress test\n", .{});
        }
    }
    const allocator = gpa.allocator();
    
    std.debug.print("=== Arena Cleanup Stress Test ===\n", .{});
    
    var cleanup = AutomaticArenaCleanup.init(allocator);
    defer cleanup.deinit();
    
    // Create many arena managers and use them heavily
    const manager_count = 100;
    const allocation_count = 1000;
    
    for (0..manager_count) |i| {
        const manager = try cleanup.createManagedArenaManager();
        
        // Use all types of allocators
        const parser_alloc = manager.getParserAllocator();
        const ast_alloc = manager.getASTAllocator();
        const runtime_alloc = manager.getRuntimeAllocator();
        const string_alloc = manager.getStringAllocator();
        const temp_alloc = manager.getTemporaryAllocator();
        
        for (0..allocation_count) |j| {
            const size = ((i + j) % 1024) + 1;
            _ = try parser_alloc.alloc(u8, size);
            _ = try ast_alloc.alloc(u32, size / 4 + 1);
            _ = try runtime_alloc.alloc(u64, size / 8 + 1);
            _ = try string_alloc.alloc(u8, size * 2);
            _ = try temp_alloc.alloc(u8, size / 2 + 1);
        }
        
        // Periodically reset arenas
        if (i % 10 == 0) {
            cleanup.resetAllArenaManagers();
        } else if (i % 3 == 0) {
            cleanup.resetTemporaryArenas();
        }
        
        if (i % 50 == 0) {
            std.debug.print("Processed {} arena managers\n", .{i + 1});
        }
    }
    
    std.debug.print("✓ Stress test completed: {} managers with {} allocations each\n", 
                   .{manager_count, allocation_count});
}

test "arena cleanup validation" {
    try validateArenaCleanup();
}

test "arena cleanup stress test" {
    try stressTestArenaCleanup();
}

pub fn main() !void {
    try validateArenaCleanup();
    try stressTestArenaCleanup();
}
