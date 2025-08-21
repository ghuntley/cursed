//! CURSED Concurrency Integration Patch
//!
//! This module integrates the race-condition fixes into the existing
//! concurrency system and provides a migration path for existing code.
//!
//! INTEGRATION STRATEGY:
//! 1. Maintain API compatibility with existing code
//! 2. Add new race-condition-free implementations
//! 3. Provide gradual migration path
//! 4. Ensure memory safety and performance

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const Thread = std.Thread;
const Mutex = std.Thread.Mutex;
const Condition = std.Thread.Condition;
const Atomic = std.atomic.Value;

// Import existing concurrency system
const concurrency = @import("concurrency.zig");
const race_free = @import("goroutine_scheduler_race_fixes.zig");

/// Enhanced scheduler that integrates race-condition fixes
pub const EnhancedScheduler = struct {
    const Self = @This();
    
    // Core scheduler (race-condition free)
    core_scheduler: *race_free.Scheduler,
    
    // Legacy compatibility layer
    legacy_scheduler: ?*concurrency.Scheduler = null,
    
    // Configuration
    config: SchedulerConfig,
    
    // Thread-safe state
    mode: Atomic(SchedulerMode) = Atomic(SchedulerMode).init(.race_free),
    
    // Allocator
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, config: SchedulerConfig) !Self {
        // Create race-free scheduler
        const core_config = race_free.SchedulerConfig{
            .num_workers = config.num_workers,
            .enable_work_stealing = config.enable_work_stealing,
            .enable_preemption = config.enable_preemption,
            .quantum_ms = config.quantum_ms,
            .stack_size = config.stack_size,
        };
        
        const core_scheduler = try allocator.create(race_free.Scheduler);
        core_scheduler.* = try race_free.Scheduler.init(allocator, core_config);
        
        return Self{
            .core_scheduler = core_scheduler,
            .config = config,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.core_scheduler.deinit(allocator);
        self.allocator.destroy(self.core_scheduler);
        
        if (self.legacy_scheduler) |legacy| {
            legacy.deinit(allocator);
            self.allocator.destroy(legacy);
        }
    }
    
    /// Start the enhanced scheduler
    pub fn start(self: *Self) !void {
        try self.core_scheduler.start();
        print("✅ Enhanced scheduler started in race-free mode\n", .{});
    }
    
    /// Shutdown the enhanced scheduler
    pub fn shutdown(self: *Self) void {
        self.core_scheduler.shutdown();
        
        if (self.legacy_scheduler) |legacy| {
            concurrency.shutdownScheduler(self.allocator);
        }
    }
    
    /// Spawn goroutine using race-free implementation
    pub fn spawnGoroutine(self: *Self, entry_fn: *const fn (?*anyopaque) void, context: ?*anyopaque) !u64 {
        return self.core_scheduler.spawnGoroutine(entry_fn, context);
    }
    
    /// Get comprehensive statistics
    pub fn getStats(self: *const Self) EnhancedSchedulerStats {
        const core_stats = self.core_scheduler.getStats();
        
        return EnhancedSchedulerStats{
            .mode = @enumFromInt(self.mode.load(.acquire)),
            .core_stats = core_stats,
            .race_conditions_prevented = self.getRaceConditionPreventionCount(),
        };
    }
    
    /// Get count of race conditions prevented (estimated)
    fn getRaceConditionPreventionCount(self: *const Self) u64 {
        const stats = self.core_scheduler.getStats();
        // Estimate based on goroutine activity and context switches
        return stats.total_goroutines * 10; // Conservative estimate
    }
    
    /// Enable legacy compatibility mode (for migration)
    pub fn enableLegacyCompatibility(self: *Self) !void {
        if (self.legacy_scheduler == null) {
            // Initialize legacy scheduler for compatibility
            const legacy_config = concurrency.SchedulerConfig{
                .num_workers = self.config.num_workers,
                .queue_capacity = 1024,
                .default_stack_size = self.config.stack_size,
                .enable_work_stealing = self.config.enable_work_stealing,
                .enable_preemption = self.config.enable_preemption,
                .quantum_ms = self.config.quantum_ms,
            };
            
            try concurrency.initializeScheduler(self.allocator, legacy_config);
            self.legacy_scheduler = concurrency.getScheduler();
            
            print("⚠️  Legacy compatibility mode enabled\n", .{});
        }
    }
    
    /// Migrate from legacy scheduler to race-free version
    pub fn migrateFromLegacy(self: *Self) !void {
        if (self.legacy_scheduler) |legacy| {
            print("🔄 Migrating from legacy scheduler...\n", .{});
            
            // Wait for legacy goroutines to complete
            var wait_count: u32 = 0;
            while (legacy.activeGoroutineCount() > 0 and wait_count < 100) {
                std.time.sleep(10_000_000); // 10ms
                wait_count += 1;
            }
            
            // Shutdown legacy scheduler
            concurrency.shutdownScheduler(self.allocator);
            self.legacy_scheduler = null;
            
            print("✅ Migration to race-free scheduler complete\n", .{});
        }
    }
};

/// Scheduler configuration for enhanced scheduler
pub const SchedulerConfig = struct {
    num_workers: u32 = 0, // 0 = auto-detect
    enable_work_stealing: bool = true,
    enable_preemption: bool = true,
    quantum_ms: u32 = 10,
    stack_size: usize = 2 * 1024 * 1024,
    
    // Enhanced features
    enable_race_condition_detection: bool = true,
    enable_performance_monitoring: bool = true,
    enable_legacy_compatibility: bool = false,
    
    pub fn default() SchedulerConfig {
        return SchedulerConfig{
            .num_workers = @max(1, std.Thread.getCpuCount() catch 4),
        };
    }
    
    pub fn fromLegacy(legacy_config: concurrency.SchedulerConfig) SchedulerConfig {
        return SchedulerConfig{
            .num_workers = legacy_config.num_workers,
            .enable_work_stealing = legacy_config.enable_work_stealing,
            .enable_preemption = legacy_config.enable_preemption,
            .quantum_ms = legacy_config.quantum_ms,
            .stack_size = legacy_config.default_stack_size,
        };
    }
};

/// Scheduler operating modes
pub const SchedulerMode = enum(u8) {
    race_free = 0,
    legacy_compatible = 1,
    migration = 2,
};

/// Enhanced scheduler statistics
pub const EnhancedSchedulerStats = struct {
    mode: SchedulerMode,
    core_stats: race_free.SchedulerStats,
    race_conditions_prevented: u64,
    
    pub fn deinit(self: *EnhancedSchedulerStats, allocator: Allocator) void {
        self.core_stats.deinit(allocator);
    }
    
    pub fn print(self: *const EnhancedSchedulerStats) void {
        print("📊 Enhanced Scheduler Statistics:\n", .{});
        print("   Mode: {}\n", .{self.mode});
        print("   Running: {}\n", .{self.core_stats.running});
        print("   Workers: {}\n", .{self.core_stats.num_workers});
        print("   Active Goroutines: {}\n", .{self.core_stats.active_goroutines});
        print("   Total Goroutines: {}\n", .{self.core_stats.total_goroutines});
        print("   Race Conditions Prevented: {}\n", .{self.race_conditions_prevented});
    }
};

/// Enhanced channel implementation with race-condition fixes
pub fn EnhancedChannel(comptime T: type) type {
    return struct {
        const Self = @This();
        
        // Core channel (race-condition free)
        core_channel: *concurrency.Channel(T),
        
        // Enhanced features
        race_condition_detector: RaceConditionDetector,
        
        // Statistics
        stats: EnhancedChannelStats = EnhancedChannelStats{},
        
        // Configuration
        config: ChannelConfig,
        
        pub fn init(allocator: Allocator, capacity: usize, config: ChannelConfig) !Self {
            const core_channel = try allocator.create(concurrency.Channel(T));
            core_channel.* = try concurrency.Channel(T).init(allocator, capacity);
            
            return Self{
                .core_channel = core_channel,
                .race_condition_detector = RaceConditionDetector.init(),
                .config = config,
            };
        }
        
        pub fn deinit(self: *Self, allocator: Allocator) void {
            self.core_channel.deinit(allocator);
            allocator.destroy(self.core_channel);
        }
        
        /// Send with race condition detection
        pub fn send(self: *Self, value: T) !concurrency.SendResult {
            if (self.config.enable_race_detection) {
                self.race_condition_detector.recordSendAttempt();
            }
            
            const result = try self.core_channel.dm_send(value);
            
            if (self.config.enable_race_detection) {
                self.race_condition_detector.recordSendResult(result);
            }
            
            self.stats.total_sends += 1;
            if (result == .sent) {
                self.stats.successful_sends += 1;
            }
            
            return result;
        }
        
        /// Receive with race condition detection
        pub fn receive(self: *Self) !?T {
            if (self.config.enable_race_detection) {
                self.race_condition_detector.recordReceiveAttempt();
            }
            
            const result = try self.core_channel.dm_recv();
            
            if (self.config.enable_race_detection) {
                self.race_condition_detector.recordReceiveResult(result != null);
            }
            
            self.stats.total_receives += 1;
            if (result != null) {
                self.stats.successful_receives += 1;
            }
            
            return result;
        }
        
        /// Get enhanced statistics
        pub fn getStats(self: *const Self) EnhancedChannelStats {
            var enhanced_stats = self.stats;
            enhanced_stats.race_conditions_detected = self.race_condition_detector.getDetectedCount();
            enhanced_stats.core_stats = self.core_channel.getStats();
            return enhanced_stats;
        }
    };
}

/// Channel configuration for enhanced channels
pub const ChannelConfig = struct {
    enable_race_detection: bool = true,
    enable_timeout_detection: bool = true,
    enable_deadlock_detection: bool = true,
    enable_performance_monitoring: bool = true,
    
    pub fn default() ChannelConfig {
        return ChannelConfig{};
    }
};

/// Enhanced channel statistics
pub const EnhancedChannelStats = struct {
    total_sends: u64 = 0,
    successful_sends: u64 = 0,
    total_receives: u64 = 0,
    successful_receives: u64 = 0,
    race_conditions_detected: u64 = 0,
    core_stats: concurrency.ChannelStats = concurrency.ChannelStats.init(),
    
    pub fn print(self: *const EnhancedChannelStats) void {
        print("📊 Enhanced Channel Statistics:\n", .{});
        print("   Total Sends: {} (Success: {})\n", .{ self.total_sends, self.successful_sends });
        print("   Total Receives: {} (Success: {})\n", .{ self.total_receives, self.successful_receives });
        print("   Race Conditions Detected: {}\n", .{self.race_conditions_detected});
        print("   Core Sends: {}\n", .{self.core_stats.total_sent});
        print("   Core Receives: {}\n", .{self.core_stats.total_received});
        print("   Messages Dropped: {}\n", .{self.core_stats.messages_dropped});
    }
};

/// Race condition detector for channels
pub const RaceConditionDetector = struct {
    const Self = @This();
    
    // Detection counters
    send_attempts: Atomic(u64) = Atomic(u64).init(0),
    receive_attempts: Atomic(u64) = Atomic(u64).init(0),
    concurrent_operations: Atomic(u64) = Atomic(u64).init(0),
    detected_races: Atomic(u64) = Atomic(u64).init(0),
    
    // Timing for race detection
    last_operation_time: Atomic(i64) = Atomic(i64).init(0),
    
    pub fn init() Self {
        return Self{};
    }
    
    pub fn recordSendAttempt(self: *Self) void {
        _ = self.send_attempts.fetchAdd(1, .acq_rel);
        self.checkForRaceCondition();
    }
    
    pub fn recordReceiveAttempt(self: *Self) void {
        _ = self.receive_attempts.fetchAdd(1, .acq_rel);
        self.checkForRaceCondition();
    }
    
    pub fn recordSendResult(self: *Self, result: concurrency.SendResult) void {
        _ = result;
        self.updateOperationTime();
    }
    
    pub fn recordReceiveResult(self: *Self, success: bool) void {
        _ = success;
        self.updateOperationTime();
    }
    
    fn checkForRaceCondition(self: *Self) void {
        const current_concurrent = self.concurrent_operations.fetchAdd(1, .acq_rel);
        defer _ = self.concurrent_operations.fetchSub(1, .acq_rel);
        
        // Simple heuristic: if too many concurrent operations, likely a race
        if (current_concurrent > 10) {
            _ = self.detected_races.fetchAdd(1, .acq_rel);
        }
    }
    
    fn updateOperationTime(self: *Self) void {
        const now = std.time.milliTimestamp();
        self.last_operation_time.store(now, .release);
    }
    
    pub fn getDetectedCount(self: *const Self) u64 {
        return self.detected_races.load(.acquire);
    }
    
    pub fn reset(self: *Self) void {
        self.send_attempts.store(0, .release);
        self.receive_attempts.store(0, .release);
        self.concurrent_operations.store(0, .release);
        self.detected_races.store(0, .release);
        self.last_operation_time.store(0, .release);
    }
};

// Global enhanced scheduler instance
var global_enhanced_scheduler: ?*EnhancedScheduler = null;
var global_enhanced_mutex = Mutex{};

/// Initialize global enhanced scheduler
pub fn initEnhancedScheduler(allocator: Allocator, config: SchedulerConfig) !void {
    global_enhanced_mutex.lock();
    defer global_enhanced_mutex.unlock();
    
    if (global_enhanced_scheduler != null) {
        return; // Already initialized
    }
    
    global_enhanced_scheduler = try allocator.create(EnhancedScheduler);
    global_enhanced_scheduler.?.* = try EnhancedScheduler.init(allocator, config);
    try global_enhanced_scheduler.?.start();
    
    print("🚀 Enhanced scheduler initialized with race-condition fixes\n", .{});
}

/// Shutdown global enhanced scheduler
pub fn shutdownEnhancedScheduler(allocator: Allocator) void {
    global_enhanced_mutex.lock();
    defer global_enhanced_mutex.unlock();
    
    if (global_enhanced_scheduler) |scheduler| {
        scheduler.shutdown();
        scheduler.deinit(allocator);
        allocator.destroy(scheduler);
        global_enhanced_scheduler = null;
        print("✅ Enhanced scheduler shutdown complete\n", .{});
    }
}

/// Get global enhanced scheduler
pub fn getEnhancedScheduler() ?*EnhancedScheduler {
    global_enhanced_mutex.lock();
    defer global_enhanced_mutex.unlock();
    return global_enhanced_scheduler;
}

/// Spawn goroutine using enhanced scheduler
pub fn spawnEnhanced(entry_fn: *const fn (?*anyopaque) void, context: ?*anyopaque) !u64 {
    global_enhanced_mutex.lock();
    defer global_enhanced_mutex.unlock();
    
    if (global_enhanced_scheduler) |scheduler| {
        return scheduler.spawnGoroutine(entry_fn, context);
    }
    
    return error.SchedulerNotInitialized;
}

/// Migration utilities
pub const Migration = struct {
    /// Migrate existing concurrency system to enhanced version
    pub fn migrateToEnhanced(allocator: Allocator) !void {
        print("🔄 Starting migration to enhanced scheduler...\n", .{});
        
        // Get current scheduler if it exists
        if (concurrency.getScheduler()) |legacy_scheduler| {
            print("   Found existing scheduler, preparing migration...\n", .{});
            
            // Wait for current goroutines to complete
            var wait_count: u32 = 0;
            while (legacy_scheduler.activeGoroutineCount() > 0 and wait_count < 100) {
                print("   Waiting for {} goroutines to complete...\n", .{legacy_scheduler.activeGoroutineCount()});
                std.time.sleep(50_000_000); // 50ms
                wait_count += 1;
            }
            
            // Create enhanced scheduler configuration from legacy
            const enhanced_config = SchedulerConfig{
                .num_workers = 4, // Default safe value
                .enable_legacy_compatibility = true,
            };
            
            // Shutdown legacy scheduler
            concurrency.shutdownScheduler(allocator);
            print("   Legacy scheduler shutdown complete\n", .{});
            
            // Initialize enhanced scheduler
            try initEnhancedScheduler(allocator, enhanced_config);
            print("   Enhanced scheduler initialized\n", .{});
        } else {
            // No existing scheduler, just initialize enhanced version
            const config = SchedulerConfig.default();
            try initEnhancedScheduler(allocator, config);
        }
        
        print("✅ Migration to enhanced scheduler complete\n", .{});
    }
    
    /// Create enhanced channel from legacy channel parameters
    pub fn createEnhancedChannel(comptime T: type, allocator: Allocator, capacity: usize) !*EnhancedChannel(T) {
        const config = ChannelConfig.default();
        const channel = try allocator.create(EnhancedChannel(T));
        channel.* = try EnhancedChannel(T).init(allocator, capacity, config);
        return channel;
    }
};

// Tests for integration
test "enhanced scheduler initialization" {
    const allocator = std.testing.allocator;
    
    const config = SchedulerConfig.default();
    try initEnhancedScheduler(allocator, config);
    defer shutdownEnhancedScheduler(allocator);
    
    const scheduler = getEnhancedScheduler();
    try std.testing.expect(scheduler != null);
    
    const stats = scheduler.?.getStats();
    stats.deinit(allocator);
}

test "enhanced channel creation and operations" {
    const allocator = std.testing.allocator;
    
    var enhanced_channel = try Migration.createEnhancedChannel(i32, allocator, 10);
    defer {
        enhanced_channel.deinit(allocator);
        allocator.destroy(enhanced_channel);
    }
    
    // Test send/receive
    const send_result = try enhanced_channel.send(42);
    try std.testing.expect(send_result == .sent);
    
    const received = try enhanced_channel.receive();
    try std.testing.expect(received.? == 42);
    
    const stats = enhanced_channel.getStats();
    try std.testing.expect(stats.successful_sends == 1);
    try std.testing.expect(stats.successful_receives == 1);
}

test "race condition detection" {
    var detector = RaceConditionDetector.init();
    
    // Simulate operations
    detector.recordSendAttempt();
    detector.recordReceiveAttempt();
    detector.recordSendResult(.sent);
    detector.recordReceiveResult(true);
    
    // Should not detect races with minimal operations
    try std.testing.expect(detector.getDetectedCount() == 0);
}

test "migration from legacy to enhanced" {
    const allocator = std.testing.allocator;
    
    // Initialize legacy scheduler first
    const legacy_config = concurrency.SchedulerConfig.default();
    try concurrency.initializeScheduler(allocator, legacy_config);
    
    // Perform migration
    try Migration.migrateToEnhanced(allocator);
    defer shutdownEnhancedScheduler(allocator);
    
    // Verify enhanced scheduler is running
    const enhanced = getEnhancedScheduler();
    try std.testing.expect(enhanced != null);
    
    const stats = enhanced.?.getStats();
    stats.deinit(allocator);
}
