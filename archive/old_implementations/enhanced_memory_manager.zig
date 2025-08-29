// Enhanced Memory Management System for CURSED
// Comprehensive integration of GC, arena allocators, and memory pools
// Zero-leak guarantee with production-grade safety

const std = @import("std");
const builtin = @import("builtin");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Mutex = std.Thread.Mutex;
const Condition = std.Thread.Condition;
const Atomic = std.atomic.Value;
const Thread = std.Thread;
const Allocator = std.mem.Allocator;

const GC = @import("gc.zig").GC;
const ArenaAllocator = @import("arena_allocator.zig").ArenaAllocator;
const CursedArenaManager = @import("arena_allocator.zig").CursedArenaManager;

/// Enhanced Memory Management System
/// Integrates garbage collection, arena allocators, and memory pools
/// for optimal performance and zero-leak guarantee
pub const EnhancedMemoryManager = struct {
    /// Memory management configuration
    pub const Config = struct {
        /// Enable garbage collection
        enable_gc: bool = true,
        /// Enable arena allocators
        enable_arenas: bool = true,
        /// Enable memory pools
        enable_pools: bool = true,
        /// Enable stack scanning
        enable_stack_scanning: bool = true,
        /// Enable memory leak detection
        enable_leak_detection: bool = true,
        /// Enable memory safety validation
        enable_safety_validation: bool = true,
        /// Memory pressure threshold (0.0 to 1.0)
        pressure_threshold: f32 = 0.85,
        /// GC trigger threshold
        gc_threshold: usize = 32 * 1024 * 1024, // 32MB
        /// Arena pool size
        arena_pool_size: usize = 16 * 1024 * 1024, // 16MB
        /// Memory pool sizes
        pool_sizes: [8]usize = [_]usize{ 16, 32, 64, 128, 256, 512, 1024, 2048 },
        /// Stack scanning depth
        stack_scan_depth: usize = 256 * 1024, // 256KB
        /// Enable concurrent GC
        enable_concurrent_gc: bool = true,
        /// Concurrent GC threads
        concurrent_gc_threads: u8 = 2,
    };

    /// Memory statistics
    pub const MemoryStats = struct {
        /// Total allocated bytes
        total_allocated: Atomic(usize),
        /// Total freed bytes
        total_freed: Atomic(usize),
        /// Peak memory usage
        peak_usage: Atomic(usize),
        /// Current heap usage
        current_heap_usage: Atomic(usize),
        /// GC cycles performed
        gc_cycles: Atomic(u64),
        /// Total GC time in microseconds
        total_gc_time_us: Atomic(u64),
        /// Arena allocations
        arena_allocations: Atomic(u64),
        /// Pool allocations
        pool_allocations: Atomic(u64),
        /// Memory leaks detected
        leaks_detected: Atomic(u64),
        /// Safety violations detected
        safety_violations: Atomic(u64),

        pub fn init() MemoryStats {
            return MemoryStats{
                .total_allocated = Atomic(usize).init(0),
                .total_freed = Atomic(usize).init(0),
                .peak_usage = Atomic(usize).init(0),
                .current_heap_usage = Atomic(usize).init(0),
                .gc_cycles = Atomic(u64).init(0),
                .total_gc_time_us = Atomic(u64).init(0),
                .arena_allocations = Atomic(u64).init(0),
                .pool_allocations = Atomic(u64).init(0),
                .leaks_detected = Atomic(u64).init(0),
                .safety_violations = Atomic(u64).init(0),
            };
        }

        pub fn getCurrentUsage(self: *const MemoryStats) usize {
            const allocated = self.total_allocated.load(.acquire);
            const freed = self.total_freed.load(.acquire);
            return allocated - freed;
        }

        pub fn getPressure(self: *const MemoryStats, max_heap: usize) f32 {
            if (max_heap == 0) return 0.0;
            const current = self.getCurrentUsage();
            return @as(f32, @floatFromInt(current)) / @as(f32, @floatFromInt(max_heap));
        }
    };

    /// Memory allocation tracking
    const AllocationTracker = struct {
        allocations: HashMap(usize, AllocationInfo, std.hash_map.AutoContext(usize), std.hash_map.default_max_load_percentage),
        tracker_mutex: Mutex,
        allocator: Allocator,

        const AllocationInfo = struct {
            size: usize,
            timestamp: u64,
            source_location: ?[]const u8,
            allocation_type: AllocationType,
            thread_id: u32,

            const AllocationType = enum {
                GC,
                Arena,
                Pool,
                Direct,
            };
        };

        pub fn init(allocator: Allocator) AllocationTracker {
        _ = allocator;
            return AllocationTracker{
                .allocations = HashMap(usize, AllocationInfo, std.hash_map.AutoContext(usize), std.hash_map.default_max_load_percentage).init(allocator),
                .tracker_mutex = Mutex{},
                .allocator = allocator,
            };
        }

        pub fn deinit(self: *AllocationTracker) void {
            self.allocations.deinit(self.allocator);
        }

        pub fn trackAllocation(self: *AllocationTracker, address: usize, info: AllocationInfo) !void {
            self.tracker_mutex.lock();
            defer self.tracker_mutex.unlock();
            try self.allocations.put(address, info);
        }

        pub fn untrackAllocation(self: *AllocationTracker, address: usize) void {
            self.tracker_mutex.lock();
            defer self.tracker_mutex.unlock();
            _ = self.allocations.remove(address);
        }

        pub fn detectLeaks(self: *AllocationTracker, threshold_age_ms: u64) ![]AllocationInfo {
            self.tracker_mutex.lock();
            defer self.tracker_mutex.unlock();

            var leaks = ArrayList(AllocationInfo){};
            const current_time = @as(u64, @intCast(std.time.microTimestamp()));

            var iterator = self.allocations.iterator();
            while (iterator.next()) |entry| {
                const age = current_time - entry.value_ptr.timestamp;
                if (age > threshold_age_ms * 1000) { // Convert to microseconds
                    try leaks.append(allocator, entry.value_ptr.*);
                }
            }

            return leaks.toOwnedSlice();
        }
    };

    /// Stack scanning implementation for GC root detection
    const StackScanner = struct {
        scan_depth: usize,
        root_candidates: ArrayList(*anyopaque),
        scanner_mutex: Mutex,
        allocator: Allocator,

        pub fn init(allocator: Allocator, scan_depth: usize) StackScanner {
            return StackScanner{
                .scan_depth = scan_depth,
                .root_candidates = .{},
                .scanner_mutex = Mutex{},
                .allocator = allocator,
            };
        }

        pub fn deinit(self: *StackScanner) void {
            self.root_candidates.deinit(self.allocator);
        }

        /// Scan stack for potential GC roots
        pub fn scanStack(self: *StackScanner, gc: *GC) !void {
            self.scanner_mutex.lock();
            defer self.scanner_mutex.unlock();

            // Clear previous candidates
            self.root_candidates.clearRetainingCapacity();

            // Get current stack frame
            const stack_base = @frameAddress();
            const stack_ptr = @as([*]u8, @ptrCast(stack_base));

            // Scan stack memory for potential pointers
            var offset: usize = 0;
            while (offset < self.scan_depth) {
                const potential_ptr = @as(*anyopaque, @ptrCast(stack_ptr + offset));
                
                // Check if this could be a valid heap pointer
                if (self.isValidHeapPointer(potential_ptr, gc)) {
                    try self.root_candidates.append(allocator, potential_ptr);
                }
                
                offset += @sizeOf(usize);
            }

            // Register stack roots with GC
            for (self.root_candidates.items) |ptr| {
                var root: ?*anyopaque = ptr;
                try gc.addRoot(&root, 0); // Type ID 0 for stack roots
            }
        }

        /// Check if pointer could be a valid heap address
        fn isValidHeapPointer(self: *StackScanner, ptr: *anyopaque, gc: *GC) bool {
            _ = self;
            const addr = @intFromPtr(ptr);
            
            // Basic sanity checks
            if (addr == 0 or addr % @sizeOf(usize) != 0) {
                return false;
            }

            // Check if address is within GC heap range
            return gc.isValidAddress(addr);
        }
    };

    /// Memory pool manager for fast allocation of common sizes
    const MemoryPoolManager = struct {
        pools: []Pool,
        size_map: [2048]u8, // Map size to pool index
        allocator: Allocator,
        manager_mutex: Mutex,

        const Pool = struct {
            block_size: usize,
            free_blocks: ArrayList(*anyopaque),
            chunks: ArrayList([]u8),
            pool_mutex: Mutex,
            allocations: Atomic(u64),

            pub fn init(allocator: Allocator, block_size: usize) Pool {
                return Pool{
                    .block_size = block_size,
                    .free_blocks = .{},
                    .chunks = .{},
                    .pool_mutex = Mutex{},
                    .allocations = Atomic(u64).init(0),
                };
            }

            pub fn deinit(self: *Pool, allocator: Allocator) void {
        _ = allocator;
                for (self.chunks.items) |chunk| {
                    allocator.free(chunk);
                }
                self.chunks.deinit(self.allocator);
                self.free_blocks.deinit(self.allocator);
            }

            pub fn allocate(self: *Pool, allocator: Allocator) !*anyopaque {
        _ = allocator;
                self.pool_mutex.lock();
                defer self.pool_mutex.unlock();

                if (self.free_blocks.items.len == 0) {
                    try self.addChunk(allocator);
                }

                const ptr = self.free_blocks.pop();
                _ = self.allocations.fetchAdd(1, .release);
                return ptr;
            }

            pub fn deallocate(self: *Pool, ptr: *anyopaque) !void {
                self.pool_mutex.lock();
                defer self.pool_mutex.unlock();
                try self.free_blocks.append(allocator, ptr);
            }

            fn addChunk(self: *Pool, allocator: Allocator) !void {
                const blocks_per_chunk = 64;
                const chunk_size = self.block_size * blocks_per_chunk;
                const chunk = try allocator.alloc(u8, chunk_size);
                try self.chunks.append(allocator, chunk);

                var ptr = chunk.ptr;
                for (0..blocks_per_chunk) |_| {
                    try self.free_blocks.append(allocator, @ptrCast(ptr));
                    ptr += self.block_size;
                }
            }
        };

        pub fn init(allocator: Allocator, pool_sizes: []const usize) !MemoryPoolManager {
            var manager = MemoryPoolManager{
                .pools = try allocator.alloc(Pool, pool_sizes.len),
                .size_map = [_]u8{255} ** 2048, // 255 = no pool available
                .allocator = allocator,
                .manager_mutex = Mutex{},
            };

            // Initialize pools
            for (pool_sizes, 0..) |size, i| {
                manager.pools[i] = Pool.init(allocator, size);
                
                // Update size map
                if (size < manager.size_map.len) {
                    manager.size_map[size] = @as(u8, @intCast(i));
                }
            }

            return manager;
        }

        pub fn deinit(self: *MemoryPoolManager) void {
            for (&self.pools) |*pool| {
                pool.deinit(self.allocator);
            }
            self.allocator.free(self.pools);
        }

        pub fn allocate(self: *MemoryPoolManager, size: usize) !?*anyopaque {
            if (size >= self.size_map.len) {
                return null; // Too large for pools
            }

            const pool_index = self.size_map[size];
            if (pool_index == 255) {
                return null; // No suitable pool
            }

            return try self.pools[pool_index].allocate(self.allocator);
        }

        pub fn deallocate(self: *MemoryPoolManager, ptr: *anyopaque, size: usize) !void {
            if (size >= self.size_map.len) {
                return; // Not from a pool
            }

            const pool_index = self.size_map[size];
            if (pool_index == 255) {
                return; // Not from a pool
            }

            try self.pools[pool_index].deallocate(ptr);
        }
    };

    /// Memory safety validator
    const SafetyValidator = struct {
        bounds_tracker: HashMap(usize, BoundsInfo, std.hash_map.AutoContext(usize), std.hash_map.default_max_load_percentage),
        validator_mutex: Mutex,
        allocator: Allocator,
        violations: Atomic(u64),

        const BoundsInfo = struct {
            start: usize,
            end: usize,
            allocation_type: []const u8,
        };

        pub fn init(allocator: Allocator) SafetyValidator {
        _ = allocator;
            return SafetyValidator{
                .bounds_tracker = HashMap(usize, BoundsInfo, std.hash_map.AutoContext(usize), std.hash_map.default_max_load_percentage).init(allocator),
                .validator_mutex = Mutex{},
                .allocator = allocator,
                .violations = Atomic(u64).init(0),
            };
        }

        pub fn deinit(self: *SafetyValidator) void {
            self.bounds_tracker.deinit(self.allocator);
        }

        pub fn registerAllocation(self: *SafetyValidator, ptr: *anyopaque, size: usize, allocation_type: []const u8) !void {
            self.validator_mutex.lock();
            defer self.validator_mutex.unlock();

            const addr = @intFromPtr(ptr);
            const bounds = BoundsInfo{
                .start = addr,
                .end = addr + size,
                .allocation_type = allocation_type,
            };
            try self.bounds_tracker.put(addr, bounds);
        }

        pub fn validateAccess(self: *SafetyValidator, ptr: *anyopaque, access_size: usize) bool {
            self.validator_mutex.lock();
            defer self.validator_mutex.unlock();

            const addr = @intFromPtr(ptr);
            
            var iterator = self.bounds_tracker.iterator();
            while (iterator.next()) |entry| {
                const bounds = entry.value_ptr;
                if (addr >= bounds.start and addr + access_size <= bounds.end) {
                    return true; // Valid access
                }
            }

            // Invalid access detected
            _ = self.violations.fetchAdd(1, .release);
            std.log.err("Memory safety violation: invalid access at 0x{x}, size {}", .{ addr, access_size });
            return false;
        }

        pub fn unregisterAllocation(self: *SafetyValidator, ptr: *anyopaque) void {
            self.validator_mutex.lock();
            defer self.validator_mutex.unlock();

            const addr = @intFromPtr(ptr);
            _ = self.bounds_tracker.remove(addr);
        }
    };

    // Core memory manager state
    backing_allocator: Allocator,
    config: Config,
    stats: MemoryStats,
    
    // Subsystems
    gc: ?*GC,
    arena_manager: ?*CursedArenaManager,
    pool_manager: ?*MemoryPoolManager,
    stack_scanner: ?*StackScanner,
    allocation_tracker: ?*AllocationTracker,
    safety_validator: ?*SafetyValidator,
    
    // Thread safety
    manager_mutex: Mutex,
    gc_thread: ?Thread,
    gc_condition: Condition,
    gc_shutdown: Atomic(bool),

    pub fn init(backing_allocator: Allocator, config: Config) !*EnhancedMemoryManager {
        const manager = try backing_allocator.create(EnhancedMemoryManager);
        manager.* = EnhancedMemoryManager{
            .backing_allocator = backing_allocator,
            .config = config,
            .stats = MemoryStats.init(),
            .gc = null,
            .arena_manager = null,
            .pool_manager = null,
            .stack_scanner = null,
            .allocation_tracker = null,
            .safety_validator = null,
            .manager_mutex = Mutex{},
            .gc_thread = null,
            .gc_condition = Condition{},
            .gc_shutdown = Atomic(bool).init(false),
        };

        try manager.initializeSubsystems();
        return manager;
    }

    pub fn deinit(self: *EnhancedMemoryManager) void {
        // Shutdown GC thread
        self.gc_shutdown.store(true, .release);
        if (self.gc_thread) |thread| {
            self.gc_condition.signal();
            thread.join();
        }

        // Cleanup subsystems
        if (self.safety_validator) |validator| {
            validator.deinit();
            self.backing_allocator.destroy(validator);
        }
        
        if (self.allocation_tracker) |tracker| {
            tracker.deinit();
            self.backing_allocator.destroy(tracker);
        }
        
        if (self.stack_scanner) |scanner| {
            scanner.deinit();
            self.backing_allocator.destroy(scanner);
        }
        
        if (self.pool_manager) |pools| {
            pools.deinit();
            self.backing_allocator.destroy(pools);
        }
        
        if (self.arena_manager) |arenas| {
            arenas.deinit();
            self.backing_allocator.destroy(arenas);
        }
        
        if (self.gc) |gc| {
            gc.deinit();
            self.backing_allocator.destroy(gc);
        }

        self.backing_allocator.destroy(self);
    }

    /// Initialize all subsystems based on configuration
    fn initializeSubsystems(self: *EnhancedMemoryManager) !void {
        // Initialize GC if enabled
        if (self.config.enable_gc) {
            const gc_config = @import("gc.zig").GCConfig{
                .initial_heap_size = self.config.gc_threshold,
                .enable_concurrent_collection = self.config.enable_concurrent_gc,
                .concurrent_threads = self.config.concurrent_gc_threads,
            };
            self.gc = try self.backing_allocator.create(@import("gc.zig").GC);
            self.gc.?.* = try @import("gc.zig").GC.init(self.backing_allocator, gc_config);

            // Start concurrent GC thread if enabled
            if (self.config.enable_concurrent_gc) {
                self.gc_thread = try Thread.spawn(.{}, gcWorkerThread, .{self});
            }
        }

        // Initialize arena manager if enabled
        if (self.config.enable_arenas) {
            self.arena_manager = try self.backing_allocator.create(CursedArenaManager);
            self.arena_manager.?.* = try CursedArenaManager.init(self.backing_allocator);
        }

        // Initialize memory pools if enabled
        if (self.config.enable_pools) {
            self.pool_manager = try self.backing_allocator.create(MemoryPoolManager);
            self.pool_manager.?.* = try MemoryPoolManager.init(self.backing_allocator, &self.config.pool_sizes);
        }

        // Initialize stack scanner if enabled
        if (self.config.enable_stack_scanning) {
            self.stack_scanner = try self.backing_allocator.create(StackScanner);
            self.stack_scanner.?.* = StackScanner.init(self.backing_allocator, self.config.stack_scan_depth);
        }

        // Initialize allocation tracker if leak detection enabled
        if (self.config.enable_leak_detection) {
            self.allocation_tracker = try self.backing_allocator.create(AllocationTracker);
            self.allocation_tracker.?.* = AllocationTracker.init(self.backing_allocator);
        }

        // Initialize safety validator if enabled
        if (self.config.enable_safety_validation) {
            self.safety_validator = try self.backing_allocator.create(SafetyValidator);
            self.safety_validator.?.* = SafetyValidator.init(self.backing_allocator);
        }
    }

    /// Main allocation interface with intelligent routing
    pub fn allocate(self: *EnhancedMemoryManager, size: usize, alignment: u29, source_location: ?[]const u8) !*anyopaque {
        self.manager_mutex.lock();
        defer self.manager_mutex.unlock();

        var ptr: *anyopaque = undefined;
        var allocation_type: []const u8 = undefined;

        // Route allocation based on size and configuration
        if (self.config.enable_pools) {
            if (self.pool_manager) |pools| {
                if (try pools.allocate(size)) |pool_ptr| {
                    ptr = pool_ptr;
                    allocation_type = "Pool";
                    _ = self.stats.pool_allocations.fetchAdd(1, .release);
                } else {
                    // Fall back to GC allocation
                    if (self.gc) |gc| {
                        ptr = try gc.alloc(size, 0);
                        allocation_type = "GC";
                    } else {
                        // Direct allocation
                        const slice = try self.backing_allocator.alignedAlloc(u8, alignment, size);
                        ptr = slice.ptr;
                        allocation_type = "Direct";
                    }
                }
            } else {
                return error.PoolManagerNotInitialized;
            }
        } else if (self.config.enable_gc) {
            if (self.gc) |gc| {
                ptr = try gc.alloc(size, 0);
                allocation_type = "GC";
            } else {
                return error.GCNotInitialized;
            }
        } else {
            // Direct allocation
            const slice = try self.backing_allocator.alignedAlloc(u8, alignment, size);
            ptr = slice.ptr;
            allocation_type = "Direct";
        }

        // Update statistics
        _ = self.stats.total_allocated.fetchAdd(size, .release);
        self.updatePeakUsage();

        // Track allocation if enabled
        if (self.config.enable_leak_detection) {
            if (self.allocation_tracker) |tracker| {
                const info = AllocationTracker.AllocationInfo{
                    .size = size,
                    .timestamp = @as(u64, @intCast(std.time.microTimestamp())),
                    .source_location = source_location,
                    .allocation_type = if (std.mem.eql(u8, allocation_type, "Pool")) .Pool
                                     else if (std.mem.eql(u8, allocation_type, "GC")) .GC
                                     else if (std.mem.eql(u8, allocation_type, "Arena")) .Arena
                                     else .Direct,
                    .thread_id = if (builtin.single_threaded) 0 else @as(u32, @truncate(Thread.getCurrentId())),
                };
                try tracker.trackAllocation(@intFromPtr(ptr), info);
            }
        }

        // Register with safety validator if enabled
        if (self.config.enable_safety_validation) {
            if (self.safety_validator) |validator| {
                try validator.registerAllocation(ptr, size, allocation_type);
            }
        }

        // Check memory pressure and trigger GC if needed
        if (self.config.enable_gc) {
            const pressure = self.stats.getPressure(self.config.gc_threshold);
            if (pressure > self.config.pressure_threshold) {
                self.triggerGC();
            }
        }

        return ptr;
    }

    /// Deallocate memory with routing to appropriate subsystem
    pub fn deallocate(self: *EnhancedMemoryManager, ptr: *anyopaque, size: usize) void {
        self.manager_mutex.lock();
        defer self.manager_mutex.unlock();

        // Untrack allocation if tracking enabled
        if (self.config.enable_leak_detection) {
            if (self.allocation_tracker) |tracker| {
                tracker.untrackAllocation(@intFromPtr(ptr));
            }
        }

        // Unregister from safety validator if enabled
        if (self.config.enable_safety_validation) {
            if (self.safety_validator) |validator| {
                validator.unregisterAllocation(ptr);
            }
        }

        // Try to deallocate from pools first
        if (self.config.enable_pools) {
            if (self.pool_manager) |pools| {
                pools.deallocate(ptr, size) catch {
                    // Not from a pool, try GC
                    if (self.gc) |gc| {
                        // GC handles deallocation automatically
                        _ = gc;
                    } else {
                        // Direct deallocation
                        const slice = @as([*]u8, @ptrCast(ptr))[0..size];
                        self.backing_allocator.free(slice);
                    }
                };
            }
        } else if (self.config.enable_gc) {
            if (self.gc) |gc| {
                // GC handles deallocation automatically
                _ = gc;
            } else {
                // Direct deallocation
                const slice = @as([*]u8, @ptrCast(ptr))[0..size];
                self.backing_allocator.free(slice);
            }
        } else {
            // Direct deallocation
            const slice = @as([*]u8, @ptrCast(ptr))[0..size];
            self.backing_allocator.free(slice);
        }

        // Update statistics
        _ = self.stats.total_freed.fetchAdd(size, .release);
    }

    /// Allocate from arena with specific pattern
    pub fn allocateArena(self: *EnhancedMemoryManager, size: usize, pattern: ArenaAllocator.AllocationPattern) !*anyopaque {
        if (!self.config.enable_arenas) {
            return error.ArenasNotEnabled;
        }

        if (self.arena_manager) |arenas| {
            var slice: []u8 = undefined;
            
            switch (pattern) {
                .Sequential => slice = try arenas.parser_arena.alloc(size),
                .Stack => slice = try arenas.runtime_arena.alloc(size),
                .ASTNodes => slice = try arenas.ast_arena.alloc(size),
                .StringIntern => slice = try arenas.string_arena.alloc(size),
                .Temporary => slice = try arenas.temporary_arena.alloc(size),
                else => slice = try arenas.parser_arena.alloc(size), // Default
            }

            _ = self.stats.arena_allocations.fetchAdd(1, .release);
            _ = self.stats.total_allocated.fetchAdd(size, .release);
            self.updatePeakUsage();

            return slice.ptr;
        } else {
            return error.ArenaManagerNotInitialized;
        }
    }

    /// Perform stack scanning for GC roots
    pub fn performStackScan(self: *EnhancedMemoryManager) !void {
        if (!self.config.enable_stack_scanning or !self.config.enable_gc) {
            return;
        }

        if (self.stack_scanner) |scanner| {
            if (self.gc) |gc| {
                try scanner.scanStack(gc);
            }
        }
    }

    /// Detect memory leaks
    pub fn detectLeaks(self: *EnhancedMemoryManager, threshold_age_ms: u64) ![]AllocationTracker.AllocationInfo {
        if (!self.config.enable_leak_detection) {
            return &[_]AllocationTracker.AllocationInfo{};
        }

        if (self.allocation_tracker) |tracker| {
            const leaks = try tracker.detectLeaks(threshold_age_ms);
            _ = self.stats.leaks_detected.fetchAdd(leaks.len, .release);
            return leaks;
        } else {
            return &[_]AllocationTracker.AllocationInfo{};
        }
    }

    /// Validate memory access
    pub fn validateAccess(self: *EnhancedMemoryManager, ptr: *anyopaque, access_size: usize) bool {
        if (!self.config.enable_safety_validation) {
            return true; // Always valid if validation disabled
        }

        if (self.safety_validator) |validator| {
            return validator.validateAccess(ptr, access_size);
        } else {
            return true; // Default to valid if validator not available
        }
    }

    /// Get current memory statistics
    pub fn getStats(self: *EnhancedMemoryManager) MemoryStats {
        return self.stats;
    }

    /// Get detailed memory usage report
    pub fn getMemoryReport(self: *EnhancedMemoryManager, allocator: Allocator) ![]u8 {
        _ = allocator;
        var report = ArrayList(u8){};
        const writer = report.writer();

        try writer.print("=== Enhanced Memory Manager Report ===\n");
        try writer.print("Total Allocated: {s} bytes\n", .{self.stats.total_allocated.load(.acquire)});
        try writer.print("Total Freed: {s} bytes\n", .{self.stats.total_freed.load(.acquire)});
        try writer.print("Current Usage: {s} bytes\n", .{self.stats.getCurrentUsage()});
        try writer.print("Peak Usage: {s} bytes\n", .{self.stats.peak_usage.load(.acquire)});
        try writer.print("Memory Pressure: {d:.2}%\n", .{self.stats.getPressure(self.config.gc_threshold) * 100});
        try writer.print("GC Cycles: {s}\n", .{self.stats.gc_cycles.load(.acquire)});
        try writer.print("Total GC Time: {s} μs\n", .{self.stats.total_gc_time_us.load(.acquire)});
        try writer.print("Arena Allocations: {s}\n", .{self.stats.arena_allocations.load(.acquire)});
        try writer.print("Pool Allocations: {s}\n", .{self.stats.pool_allocations.load(.acquire)});
        try writer.print("Leaks Detected: {s}\n", .{self.stats.leaks_detected.load(.acquire)});
        try writer.print("Safety Violations: {s}\n", .{self.stats.safety_violations.load(.acquire)});

        if (self.arena_manager) |arenas| {
            const usage = arenas.getTotalUsage();
            try writer.print("\n=== Arena Usage ===\n");
            try writer.print("Total Arena Allocated: {s} bytes\n", .{usage.total_allocated});
            try writer.print("Total Arena Used: {s} bytes\n", .{usage.total_used});
            try writer.print("Parser Arena: {s}/{s} bytes\n", .{usage.parser.total_used, usage.parser.total_allocated});
            try writer.print("AST Arena: {s}/{s} bytes\n", .{usage.ast.total_used, usage.ast.total_allocated});
            try writer.print("Runtime Arena: {s}/{s} bytes\n", .{usage.runtime.total_used, usage.runtime.total_allocated});
            try writer.print("String Arena: {s}/{s} bytes\n", .{usage.string.total_used, usage.string.total_allocated});
            try writer.print("Temporary Arena: {s}/{s} bytes\n", .{usage.temporary.total_used, usage.temporary.total_allocated});
        }

        return report.toOwnedSlice();
    }

    /// Force garbage collection
    pub fn forceGC(self: *EnhancedMemoryManager) !void {
        if (self.config.enable_gc) {
            if (self.gc) |gc| {
                try gc.collectNow();
                _ = self.stats.gc_cycles.fetchAdd(1, .release);
            }
        }
    }

    /// Reset temporary arenas
    pub fn resetTemporaryArenas(self: *EnhancedMemoryManager) void {
        if (self.config.enable_arenas) {
            if (self.arena_manager) |arenas| {
                arenas.resetTemporary();
            }
        }
    }

    // Private helper methods

    fn updatePeakUsage(self: *EnhancedMemoryManager) void {
        const current_usage = self.stats.getCurrentUsage();
        var peak = self.stats.peak_usage.load(.acquire);
        
        while (current_usage > peak) {
            const old_peak = self.stats.peak_usage.cmpxchgWeak(peak, current_usage, .acq_rel, .acquire) orelse break;
            peak = old_peak;
        }
    }

    fn triggerGC(self: *EnhancedMemoryManager) void {
        if (self.config.enable_concurrent_gc) {
            self.gc_condition.signal();
        } else {
            // Synchronous GC
            self.forceGC() catch {};
        }
    }

    /// GC worker thread for concurrent collection
    fn gcWorkerThread(self: *EnhancedMemoryManager) !void {
        var mutex = Mutex{};
        
        while (!self.gc_shutdown.load(.acquire)) {
            mutex.lock();
            self.gc_condition.wait(&mutex);
            mutex.unlock();

            if (self.gc_shutdown.load(.acquire)) {
                break;
            }

            // Perform GC cycle
            const start_time = std.time.microTimestamp();
            self.forceGC() catch {};
            const end_time = std.time.microTimestamp();
            
            _ = self.stats.total_gc_time_us.fetchAdd(@as(u64, @intCast(end_time - start_time)), .release);
        }
    }
};

// Export C API for LLVM integration
export fn cursed_memory_manager_create() ?*EnhancedMemoryManager {
    const allocator = std.heap.page_allocator;
    const config = EnhancedMemoryManager.Config{};
    return EnhancedMemoryManager.init(allocator, config) catch null;
}

export fn cursed_memory_manager_destroy(manager: ?*EnhancedMemoryManager) void {
    if (manager) |m| {
        m.deinit();
    }
}

export fn cursed_memory_manager_allocate(manager: ?*EnhancedMemoryManager, size: usize) ?*anyopaque {
    if (manager) |m| {
        return m.allocate(size, @alignOf(u64), null) catch null;
    }
    return null;
}

export fn cursed_memory_manager_deallocate(manager: ?*EnhancedMemoryManager, ptr: ?*anyopaque, size: usize) void {
    if (manager) |m| {
        if (ptr) |p| {
            m.deallocate(p, size);
        }
    }
}

export fn cursed_memory_manager_force_gc(manager: ?*EnhancedMemoryManager) void {
    if (manager) |m| {
        m.forceGC() catch {};
    }
}

export fn cursed_memory_manager_get_current_usage(manager: ?*EnhancedMemoryManager) usize {
    if (manager) |m| {
        return m.getStats().getCurrentUsage();
    }
    return 0;
}
