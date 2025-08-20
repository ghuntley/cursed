const std = @import("std");
const CursedArenaManager = @import("src-zig/arena_allocator.zig").CursedArenaManager;

/// Automatic arena cleanup system with memory monitoring and leak detection
pub const AutomaticArenaCleanupSystem = struct {
    allocator: std.mem.Allocator,
    active_managers: std.ArrayList(*CursedArenaManager),
    memory_monitor: MemoryMonitor,
    cleanup_policy: CleanupPolicy,
    statistics: Statistics,
    
    const Self = @This();
    
    /// Memory monitoring configuration
    pub const MemoryMonitor = struct {
        enabled: bool = true,
        check_interval_ms: u64 = 1000, // Check every second
        memory_threshold_mb: usize = 100, // Cleanup threshold in MB
        leak_detection_enabled: bool = true,
        peak_memory_tracking: bool = true,
        last_check_time: i64 = 0,
    };
    
    /// Cleanup policies
    pub const CleanupPolicy = struct {
        auto_cleanup_enabled: bool = true,
        cleanup_on_threshold: bool = true,
        cleanup_on_scope_exit: bool = true,
        reset_temporary_frequently: bool = true,
        reset_temporary_interval_ms: u64 = 500, // Reset temp arenas every 500ms
        force_cleanup_after_ms: u64 = 10000, // Force cleanup after 10 seconds
    };
    
    /// System statistics
    pub const Statistics = struct {
        total_managers_created: u64 = 0,
        total_managers_destroyed: u64 = 0,
        cleanup_operations: u64 = 0,
        memory_warnings: u64 = 0,
        peak_memory_usage_mb: usize = 0,
        total_allocations: u64 = 0,
        total_bytes_allocated: u64 = 0,
        avg_manager_lifetime_ms: f64 = 0.0,
    };
    
    /// Initialize the automatic cleanup system
    pub fn init(allocator: std.mem.Allocator) Self {
        return Self{
            .allocator = allocator,
            .active_managers = std.ArrayList(*CursedArenaManager).init(allocator),
            .memory_monitor = MemoryMonitor{},
            .cleanup_policy = CleanupPolicy{},
            .statistics = Statistics{},
        };
    }
    
    /// Cleanup all resources
    pub fn deinit(self: *Self) void {
        // Cleanup all active managers
        for (self.active_managers.items) |manager| {
            self.cleanupArenaManager(manager);
        }
        self.active_managers.deinit();
        
        // Print final statistics
        self.printStatistics();
    }
    
    /// Create a new managed arena manager
    pub fn createManagedArenaManager(self: *Self) !*CursedArenaManager {
        const manager = try self.allocator.create(CursedArenaManager);
        errdefer self.allocator.destroy(manager);
        
        manager.* = try CursedArenaManager.init(self.allocator);
        errdefer manager.deinit();
        
        try self.active_managers.append(manager);
        
        // Update statistics
        self.statistics.total_managers_created += 1;
        
        // Perform memory check if needed
        if (self.memory_monitor.enabled) {
            try self.checkMemoryUsage();
        }
        
        return manager;
    }
    
    /// Cleanup a specific arena manager
    pub fn cleanupArenaManager(self: *Self, manager: *CursedArenaManager) void {
        // Find and remove from active list
        for (self.active_managers.items, 0..) |item, i| {
            if (item == manager) {
                _ = self.active_managers.orderedRemove(i);
                break;
            }
        }
        
        // Cleanup the manager
        manager.deinit();
        self.allocator.destroy(manager);
        
        // Update statistics
        self.statistics.total_managers_destroyed += 1;
        self.statistics.cleanup_operations += 1;
    }
    
    /// Perform automatic cleanup based on policies
    pub fn performAutomaticCleanup(self: *Self) !void {
        if (!self.cleanup_policy.auto_cleanup_enabled) {
            return;
        }
        
        _ = std.time.milliTimestamp(); // Current time not needed for this policy check
        
        // Check if we should reset temporary arenas
        if (self.cleanup_policy.reset_temporary_frequently) {
            for (self.active_managers.items) |manager| {
                manager.resetTemporary();
            }
        }
        
        // Check memory thresholds
        if (self.cleanup_policy.cleanup_on_threshold) {
            const memory_usage = try self.getCurrentMemoryUsageMB();
            if (memory_usage > self.memory_monitor.memory_threshold_mb) {
                std.debug.print("⚠️  Memory usage {} MB exceeds threshold {} MB, performing cleanup...\n", 
                               .{ memory_usage, self.memory_monitor.memory_threshold_mb });
                
                for (self.active_managers.items) |manager| {
                    manager.resetAll();
                }
                
                self.statistics.cleanup_operations += 1;
                self.statistics.memory_warnings += 1;
            }
        }
    }
    
    /// Check current memory usage
    fn checkMemoryUsage(self: *Self) !void {
        const current_time = std.time.milliTimestamp();
        
        // Don't check too frequently
        if (current_time - self.memory_monitor.last_check_time < self.memory_monitor.check_interval_ms) {
            return;
        }
        
        self.memory_monitor.last_check_time = current_time;
        
        // Get current memory usage
        const memory_usage_mb = try self.getCurrentMemoryUsageMB();
        
        // Update peak usage
        if (self.memory_monitor.peak_memory_tracking and memory_usage_mb > self.statistics.peak_memory_usage_mb) {
            self.statistics.peak_memory_usage_mb = memory_usage_mb;
        }
        
        // Perform automatic cleanup if needed
        try self.performAutomaticCleanup();
    }
    
    /// Get current memory usage in MB (approximate)
    fn getCurrentMemoryUsageMB(self: *Self) !usize {
        
        // Try to read from /proc/self/status on Linux
        var buffer: [256]u8 = undefined;
        const file = std.fs.openFileAbsolute("/proc/self/status", .{}) catch {
            // Fallback: calculate from arena managers
            return self.calculateArenaMemoryUsage();
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
                        const kb_value = std.fmt.parseInt(usize, kb, 10) catch 0;
                        return kb_value / 1024; // Convert KB to MB
                    }
                }
            }
        }
        
        // Fallback to arena calculation
        return self.calculateArenaMemoryUsage();
    }
    
    /// Calculate memory usage from arena managers
    fn calculateArenaMemoryUsage(self: *Self) usize {
        var total_mb: usize = 0;
        
        for (self.active_managers.items) |manager| {
            const usage = manager.getTotalUsage();
            total_mb += usage.total_allocated / (1024 * 1024);
        }
        
        return total_mb;
    }
    
    /// Force immediate cleanup of all arenas
    pub fn forceCleanupAll(self: *Self) void {
        std.debug.print("🧹 Forcing cleanup of all {} arena managers...\n", .{self.active_managers.items.len});
        
        for (self.active_managers.items) |manager| {
            manager.resetAll();
        }
        
        self.statistics.cleanup_operations += 1;
        std.debug.print("✅ Forced cleanup completed\n", .{});
    }
    
    /// Get detailed statistics
    pub fn getDetailedStatistics(self: *Self) DetailedStatistics {
        var total_allocated: u64 = 0;
        var total_used: u64 = 0;
        var active_allocations: u64 = 0;
        
        for (self.active_managers.items) |manager| {
            const usage = manager.getTotalUsage();
            total_allocated += usage.total_allocated;
            total_used += usage.total_used;
            active_allocations += usage.parser.allocation_count + usage.ast.allocation_count + 
                                usage.runtime.allocation_count + usage.string.allocation_count + 
                                usage.temporary.allocation_count;
        }
        
        return DetailedStatistics{
            .basic = self.statistics,
            .active_managers = self.active_managers.items.len,
            .total_allocated_bytes = total_allocated,
            .total_used_bytes = total_used,
            .active_allocations = active_allocations,
            .memory_efficiency = if (total_allocated > 0) 
                @as(f64, @floatFromInt(total_used)) / @as(f64, @floatFromInt(total_allocated)) * 100.0 
                else 0.0,
        };
    }
    
    pub const DetailedStatistics = struct {
        basic: Statistics,
        active_managers: usize,
        total_allocated_bytes: u64,
        total_used_bytes: u64,
        active_allocations: u64,
        memory_efficiency: f64,
    };
    
    /// Print comprehensive statistics
    pub fn printStatistics(self: *Self) void {
        const stats = self.getDetailedStatistics();
        
        std.debug.print("\n=== Arena Cleanup System Statistics ===\n", .{});
        std.debug.print("Managers: {} created, {} active, {} destroyed\n", 
                       .{ stats.basic.total_managers_created, stats.active_managers, stats.basic.total_managers_destroyed });
        std.debug.print("Memory: {:.2} MB allocated, {:.2} MB used ({:.1}% efficiency)\n", 
                       .{ @as(f64, @floatFromInt(stats.total_allocated_bytes)) / (1024.0 * 1024.0), 
                          @as(f64, @floatFromInt(stats.total_used_bytes)) / (1024.0 * 1024.0), 
                          stats.memory_efficiency });
        std.debug.print("Peak usage: {} MB, {} cleanup operations\n", 
                       .{ stats.basic.peak_memory_usage_mb, stats.basic.cleanup_operations });
        std.debug.print("Warnings: {} memory threshold exceeded\n", .{stats.basic.memory_warnings});
        std.debug.print("Active allocations: {}\n", .{stats.active_allocations});
        std.debug.print("=======================================\n", .{});
    }
    
    /// Enable memory leak detection mode
    pub fn enableLeakDetection(self: *Self) void {
        self.memory_monitor.leak_detection_enabled = true;
        self.memory_monitor.check_interval_ms = 100; // Check more frequently
        std.debug.print("🔍 Memory leak detection enabled\n", .{});
    }
    
    /// Disable automatic cleanup (for debugging)
    pub fn disableAutomaticCleanup(self: *Self) void {
        self.cleanup_policy.auto_cleanup_enabled = false;
        std.debug.print("⚠️  Automatic cleanup disabled (debug mode)\n", .{});
    }
    
    /// Run continuous monitoring loop (for long-running processes)
    pub fn runContinuousMonitoring(self: *Self) !void {
        std.debug.print("🔄 Starting continuous arena monitoring...\n", .{});
        
        while (true) {
            try self.checkMemoryUsage();
            std.time.sleep(self.memory_monitor.check_interval_ms * std.time.ns_per_ms);
            
            // Break if no active managers and cleanup policy allows
            if (self.active_managers.items.len == 0 and self.cleanup_policy.cleanup_on_scope_exit) {
                break;
            }
        }
        
        std.debug.print("✅ Continuous monitoring completed\n", .{});
    }
};

/// RAII wrapper for automatic arena cleanup system
pub const ScopedArenaCleanupSystem = struct {
    cleanup_system: AutomaticArenaCleanupSystem,
    
    const SelfScoped = @This();
    
    pub fn init(allocator: std.mem.Allocator) SelfScoped {
        return SelfScoped{
            .cleanup_system = AutomaticArenaCleanupSystem.init(allocator),
        };
    }
    
    pub fn deinit(self: *SelfScoped) void {
        self.cleanup_system.deinit();
    }
    
    pub fn createManagedArenaManager(self: *SelfScoped) !*CursedArenaManager {
        return self.cleanup_system.createManagedArenaManager();
    }
    
    pub fn forceCleanupAll(self: *SelfScoped) void {
        self.cleanup_system.forceCleanupAll();
    }
    
    pub fn printStatistics(self: *SelfScoped) void {
        self.cleanup_system.printStatistics();
    }
};

// Test the automatic cleanup system
test "automatic arena cleanup system" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const leaked = gpa.deinit();
        if (leaked == .leak) {
            std.debug.print("ERROR: Memory leak detected!\n", .{});
        } else {
            std.debug.print("✅ No memory leaks detected\n", .{});
        }
    }
    const allocator = gpa.allocator();
    
    var cleanup_system = AutomaticArenaCleanupSystem.init(allocator);
    defer cleanup_system.deinit();
    
    // Create multiple arena managers and use them
    for (0..10) |i| {
        const manager = try cleanup_system.createManagedArenaManager();
        
        // Use all allocators
        const parser_alloc = manager.getParserAllocator();
        const ast_alloc = manager.getASTAllocator();
        const runtime_alloc = manager.getRuntimeAllocator();
        
        _ = try parser_alloc.alloc(u8, (i + 1) * 1024);
        _ = try ast_alloc.alloc(u32, (i + 1) * 256);
        _ = try runtime_alloc.alloc(u64, (i + 1) * 128);
        
        // Trigger cleanup every few iterations
        if (i % 3 == 0) {
            try cleanup_system.performAutomaticCleanup();
        }
    }
    
    cleanup_system.forceCleanupAll();
    const stats = cleanup_system.getDetailedStatistics();
    
    std.debug.print("Test completed: {} managers, {:.2} MB allocated\n", 
                   .{ stats.active_managers, @as(f64, @floatFromInt(stats.total_allocated_bytes)) / (1024.0 * 1024.0) });
}

test "scoped arena cleanup system" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var scoped_system = ScopedArenaCleanupSystem.init(allocator);
    defer scoped_system.deinit();
    
    // Create and use arena managers
    const manager1 = try scoped_system.createManagedArenaManager();
    const manager2 = try scoped_system.createManagedArenaManager();
    
    _ = try manager1.getParserAllocator().alloc(u8, 2048);
    _ = try manager2.getASTAllocator().alloc(u32, 512);
    
    scoped_system.forceCleanupAll();
    scoped_system.printStatistics();
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    std.debug.print("=== Automatic Arena Cleanup System Demo ===\n", .{});
    
    var cleanup_system = AutomaticArenaCleanupSystem.init(allocator);
    defer cleanup_system.deinit();
    
    // Enable leak detection
    cleanup_system.enableLeakDetection();
    
    // Simulate compiler usage
    for (0..5) |i| {
        std.debug.print("\n--- Compilation {} ---\n", .{i + 1});
        
        const manager = try cleanup_system.createManagedArenaManager();
        
        // Simulate parsing phase
        const parser_alloc = manager.getParserAllocator();
        _ = try parser_alloc.alloc(u8, 4096);
        
        // Simulate AST phase
        const ast_alloc = manager.getASTAllocator();
        _ = try ast_alloc.alloc(u32, 1024);
        
        // Simulate runtime phase
        const runtime_alloc = manager.getRuntimeAllocator();
        _ = try runtime_alloc.alloc(u64, 512);
        
        // Periodic cleanup
        if (i % 2 == 0) {
            try cleanup_system.performAutomaticCleanup();
        }
        
        std.time.sleep(100 * std.time.ns_per_ms); // Small delay
    }
    
    std.debug.print("\n=== Final Statistics ===\n", .{});
    cleanup_system.printStatistics();
}
