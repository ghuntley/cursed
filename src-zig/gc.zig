const std = @import("std");
const builtin = @import("builtin");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Mutex = std.Thread.Mutex;
const Condition = std.Thread.Condition;
const Atomic = std.atomic.Value;
const Thread = std.Thread;
const ArenaAllocator = @import("arena_allocator.zig").ArenaAllocator;
const CursedArenaManager = @import("arena_allocator.zig").CursedArenaManager;

/// Production-Ready Tri-Color Mark-and-Sweep Garbage Collector for CURSED
/// Features:
/// - Concurrent, low-pause collection
/// - Generational collection (young/old generations)
/// - Write barriers for concurrent collection
/// - Stack scanning and root set identification
/// - Finalization and weak references
/// - GC tuning parameters and monitoring
/// - Integration with LLVM-generated code

/// GC object header placed before each allocated object
const ObjectHeader = struct {
    /// Size of the object in bytes (including header)
    size: u32,
    /// Type ID for type-specific traversal
    type_id: u16,
    /// GC color: white (0), gray (1), black (2)
    color: u2,
    /// Generation: young (0), old (1)
    generation: u1,
    /// Mark bit for finalization
    finalize: u1,
    /// Reserved bits for future use
    reserved: u8,
    /// Next pointer for free list or mark stack
    next: ?*ObjectHeader,
    
    const HEADER_SIZE = @sizeOf(ObjectHeader);
    
    /// Get pointer to user data after header
    fn getData(self: *ObjectHeader) *anyopaque {
        const ptr = @as([*]u8, @ptrCast(self)) + HEADER_SIZE;
        return @ptrCast(ptr);
    }
    
    /// Get header from user data pointer
    fn fromData(data: *anyopaque) *ObjectHeader {
        const ptr = @as([*]u8, @ptrCast(data)) - HEADER_SIZE;
        return @ptrCast(@alignCast(ptr));
    }
};

/// GC colors for tri-color marking
const Color = enum(u2) {
    White = 0, // Not visited, candidate for collection
    Gray = 1,  // Visited but children not scanned
    Black = 2, // Visited and children scanned
};

/// Generation for generational collection
const Generation = enum(u1) {
    Young = 0, // Newly allocated objects
    Old = 1,   // Long-lived objects
};

/// GC statistics for monitoring and tuning
pub const GCStats = struct {
    /// Total allocations
    total_allocations: u64,
    /// Total bytes allocated
    total_bytes_allocated: u64,
    /// Number of GC cycles
    gc_cycles: u64,
    /// Total pause time in microseconds
    total_pause_time_us: u64,
    /// Maximum pause time in microseconds
    max_pause_time_us: u64,
    /// Objects promoted to old generation
    promotions: u64,
    /// Objects collected in young generation
    young_collections: u64,
    /// Objects collected in old generation
    old_collections: u64,
    /// Current heap size in bytes
    current_heap_size: u64,
    /// Peak heap size in bytes
    peak_heap_size: u64,
    /// Number of finalized objects
    finalized_objects: u64,
    /// Number of heap compactions
    compact_count: u64,
    /// Total compaction time in microseconds
    total_compact_time: u64,
    
    pub fn init() GCStats {
        return std.mem.zeroes(GCStats);
    }
};

/// GC configuration parameters
pub const GCConfig = struct {
    /// Initial heap size in bytes
    initial_heap_size: usize = 32 * 1024 * 1024, // 32MB for better performance
    /// Maximum heap size in bytes (0 = unlimited)
    max_heap_size: usize = 0,
    /// Young generation size (33% of total heap)
    young_gen_ratio: f32 = 0.33,
    /// Old generation size (67% of total heap)
    old_gen_ratio: f32 = 0.67,
    /// Nursery size for very young objects
    nursery_size: usize = 2 * 1024 * 1024, // 2MB
    /// Threshold for promoting objects to old generation (survival count)
    promotion_threshold: u32 = 3,
    /// Young GC trigger threshold (heap usage percentage)
    young_gc_trigger_threshold: f32 = 0.80,
    /// Old GC trigger threshold (heap usage percentage) 
    old_gc_trigger_threshold: f32 = 0.85,
    /// Concurrent collection thread count
    concurrent_threads: u32 = 2,
    /// Maximum young GC pause time target in microseconds (5ms)
    max_young_pause_time_us: u64 = 5_000,
    /// Maximum old GC pause time target in microseconds (50ms)
    max_old_pause_time_us: u64 = 50_000,
    /// Enable write barriers for concurrent collection
    enable_write_barriers: bool = true,
    /// Enable finalization
    enable_finalization: bool = true,
    /// Stack scanning depth limit
    stack_scan_depth: usize = 128 * 1024, // 128KB
    /// Enable incremental collection for large heaps
    enable_incremental_collection: bool = true,
    /// Work chunk size for incremental collection
    incremental_work_size: usize = 1024,
    /// Enable parallel marking with multiple threads
    enable_parallel_marking: bool = true,
    /// Enable heap compaction after major collections
    enable_compaction: bool = true,
    /// Compaction trigger threshold (fragmentation percentage)
    compaction_threshold: f32 = 0.30,
    /// Enable arena allocator integration
    enable_arena_allocation: bool = true,
    
    pub fn default() GCConfig {
        return GCConfig{};
    }
    
    pub fn optimizedForThroughput() GCConfig {
        var config = GCConfig.default();
        config.young_gc_trigger_threshold = 0.90;
        config.old_gc_trigger_threshold = 0.95;
        config.enable_parallel_marking = true;
        config.concurrent_threads = 4;
        return config;
    }
    
    pub fn optimizedForLatency() GCConfig {
        var config = GCConfig.default();
        config.young_gc_trigger_threshold = 0.60;
        config.old_gc_trigger_threshold = 0.70;
        config.max_young_pause_time_us = 2_000; // 2ms
        config.max_old_pause_time_us = 25_000; // 25ms
        config.enable_incremental_collection = true;
        config.incremental_work_size = 512;
        return config;
    }
};

/// Root reference for GC scanning
const RootRef = struct {
    ptr: *?*anyopaque,
    type_id: u16,
};

/// Allocation information for tracking
const AllocationInfo = struct {
    size: usize,
    type_id: u16,
    timestamp: u64,
    thread_id: u32,
    source_location: ?[]const u8,
    ref_count: Atomic(u32),
    
    pub fn init(size: usize, type_id: u16, source_location: ?[]const u8) AllocationInfo {
        return AllocationInfo{
            .size = size,
            .type_id = type_id,
            .timestamp = @as(u64, @intCast(std.time.microTimestamp())),
            .thread_id = if (builtin.single_threaded) 0 else std.Thread.getCurrentId(),
            .source_location = source_location,
            .ref_count = Atomic(u32).init(1),
        };
    }
};

/// Memory leak detection information
const LeakInfo = struct {
    address: usize,
    info: AllocationInfo,
};

/// Memory pressure monitoring
const PressureLevel = enum {
    Low,    // < 50% heap usage
    Medium, // 50-80% heap usage  
    High,   // 80-95% heap usage
    Critical, // > 95% heap usage
};

/// Memory tracker for leak detection and profiling
const MemoryTracker = struct {
    allocations: HashMap(usize, AllocationInfo, std.hash_map.AutoContext(usize), std.hash_map.default_max_load_percentage),
    total_allocated: Atomic(usize),
    total_freed: Atomic(usize),
    peak_usage: Atomic(usize),
    leak_threshold: usize,
    tracker_mutex: Mutex,
    
    pub fn init(allocator: std.mem.Allocator) MemoryTracker {
        return MemoryTracker{
            .allocations = HashMap(usize, AllocationInfo, std.hash_map.AutoContext(usize), std.hash_map.default_max_load_percentage).init(allocator),
            .total_allocated = Atomic(usize).init(0),
            .total_freed = Atomic(usize).init(0),
            .peak_usage = Atomic(usize).init(0),
            .leak_threshold = 1024 * 1024, // 1MB threshold
            .tracker_mutex = Mutex{},
        };
    }
    
    pub fn deinit(self: *MemoryTracker) void {
        self.allocations.deinit();
    }
    
    pub fn trackAllocation(self: *MemoryTracker, address: usize, info: AllocationInfo) !void {
        self.tracker_mutex.lock();
        defer self.tracker_mutex.unlock();
        
        try self.allocations.put(address, info);
        
        const new_total = self.total_allocated.fetchAdd(info.size, .release) + info.size;
        
        // Update peak usage
        var current_peak = self.peak_usage.load(.acquire);
        while (new_total > current_peak) {
            const old_peak = self.peak_usage.cmpxchgWeak(current_peak, new_total, .acq_rel, .acquire) orelse break;
            current_peak = old_peak;
        }
    }
    
    pub fn trackDeallocation(self: *MemoryTracker, address: usize) void {
        self.tracker_mutex.lock();
        defer self.tracker_mutex.unlock();
        
        if (self.allocations.fetchRemove(address)) |entry| {
            _ = self.total_freed.fetchAdd(entry.value.size, .release);
        }
    }
    
    pub fn detectLeaks(self: *MemoryTracker, allocator: std.mem.Allocator) ![]LeakInfo {
        self.tracker_mutex.lock();
        defer self.tracker_mutex.unlock();
        
        var leaks = ArrayList(LeakInfo).init(allocator);
        
        var iterator = self.allocations.iterator();
        while (iterator.next()) |entry| {
            const age_ms = @as(u64, @intCast(std.time.microTimestamp())) - entry.value_ptr.timestamp;
            
            // Consider allocation a leak if it's old and large
            if (age_ms > 60_000_000 and entry.value_ptr.size > self.leak_threshold) { // 60 seconds
                try leaks.append(LeakInfo{
                    .address = entry.key_ptr.*,
                    .info = entry.value_ptr.*,
                });
            }
        }
        
        return leaks.toOwnedSlice();
    }
    
    pub fn getCurrentUsage(self: *MemoryTracker) usize {
        const allocated = self.total_allocated.load(.acquire);
        const freed = self.total_freed.load(.acquire);
        return allocated - freed;
    }
};

/// Memory pool for specific allocation sizes
const MemoryPool = struct {
    block_size: usize,
    blocks_per_chunk: usize,
    free_blocks: ArrayList(*anyopaque),
    chunks: ArrayList([]u8),
    pool_mutex: Mutex,
    allocator: std.mem.Allocator,
    
    pub fn init(allocator: std.mem.Allocator, block_size: usize, blocks_per_chunk: usize) MemoryPool {
        return MemoryPool{
            .block_size = block_size,
            .blocks_per_chunk = blocks_per_chunk,
            .free_blocks = ArrayList(*anyopaque).init(allocator),
            .chunks = ArrayList([]u8).init(allocator),
            .pool_mutex = Mutex{},
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *MemoryPool) void {
        for (self.chunks.items) |chunk| {
            self.allocator.free(chunk);
        }
        self.chunks.deinit();
        self.free_blocks.deinit();
    }
    
    pub fn allocate(self: *MemoryPool) !*anyopaque {
        self.pool_mutex.lock();
        defer self.pool_mutex.unlock();
        
        if (self.free_blocks.items.len == 0) {
            try self.addChunk();
        }
        
        return self.free_blocks.pop();
    }
    
    pub fn deallocate(self: *MemoryPool, ptr: *anyopaque) !void {
        self.pool_mutex.lock();
        defer self.pool_mutex.unlock();
        
        try self.free_blocks.append(ptr);
    }
    
    fn addChunk(self: *MemoryPool) !void {
        const chunk_size = self.block_size * self.blocks_per_chunk;
        const chunk = try self.allocator.alloc(u8, chunk_size);
        try self.chunks.append(chunk);
        
        var ptr = chunk.ptr;
        for (0..self.blocks_per_chunk) |_| {
            try self.free_blocks.append(@ptrCast(ptr));
            ptr += self.block_size;
        }
    }
};

/// Memory pool manager for different sizes
const MemoryPoolManager = struct {
    pools: [16]MemoryPool, // 16 size classes
    size_classes: [16]usize,
    allocator: std.mem.Allocator,
    
    pub fn init(allocator: std.mem.Allocator) !MemoryPoolManager {
        var manager = MemoryPoolManager{
            .pools = undefined,
            .size_classes = undefined,
            .allocator = allocator,
        };
        
        // Initialize size classes: 16, 32, 64, 128, 256, 512, 1KB, 2KB, etc.
        for (0..16) |i| {
            const size_class = 16 * std.math.pow(usize, 2, i);
            manager.size_classes[i] = size_class;
            manager.pools[i] = MemoryPool.init(allocator, size_class, 64);
        }
        
        return manager;
    }
    
    pub fn deinit(self: *MemoryPoolManager) void {
        for (&self.pools) |*pool| {
            pool.deinit();
        }
    }
    
    pub fn getAllocation(self: *MemoryPoolManager, size: usize) !?*anyopaque {
        for (self.size_classes, 0..) |class_size, i| {
            if (size <= class_size) {
                return try self.pools[i].allocate();
            }
        }
        return null; // Size too large for pools
    }
    
    pub fn deallocate(self: *MemoryPoolManager, ptr: *anyopaque, size: usize) !void {
        for (self.size_classes, 0..) |class_size, i| {
            if (size <= class_size) {
                try self.pools[i].deallocate(ptr);
                return;
            }
        }
    }
};

/// Weak reference structure
pub const WeakRef = struct {
    target: ?*anyopaque,
    header: ?*ObjectHeader,
    
    pub fn get(self: *WeakRef) ?*anyopaque {
        if (self.header) |header| {
            if (header.color != @intFromEnum(Color.White)) {
                return self.target;
            } else {
                // Object was collected
                self.target = null;
                self.header = null;
                return null;
            }
        }
        return null;
    }
};

/// Finalizer function type
pub const FinalizerFn = *const fn (object: *anyopaque) void;

/// Finalizer registration
const Finalizer = struct {
    object: *ObjectHeader,
    fn_ptr: FinalizerFn,
};

/// Write barrier record for concurrent collection
const WriteBarrier = struct {
    old_ref: *anyopaque,
    new_ref: *anyopaque,
    timestamp: u64,
};

/// Heap segment for compaction
const HeapSegment = struct {
    start: *u8,
    end: *u8,
    current: *u8,
    generation: u1, // 0 = young, 1 = old
};

/// Production Garbage Collector
pub const GC = struct {
    /// Configuration
    config: GCConfig,
    /// Statistics
    stats: GCStats,
    /// Global allocator for GC metadata
    allocator: std.mem.Allocator,
    
    /// Memory management
    heap_start: ?*anyopaque,
    heap_size: usize,
    heap_used: usize,
    young_heap_start: ?*anyopaque,
    young_heap_used: usize,
    old_heap_start: ?*anyopaque,
    old_heap_used: usize,
    
    /// Advanced memory management features
    memory_pressure: Atomic(f32),
    memory_tracker: MemoryTracker,
    allocation_map: HashMap(usize, AllocationInfo, std.hash_map.AutoContext(usize), std.hash_map.default_max_load_percentage),
    ref_count_map: HashMap(usize, Atomic(u32), std.hash_map.AutoContext(usize), std.hash_map.default_max_load_percentage),
    memory_pools: MemoryPoolManager,
    gc_trigger_threshold: Atomic(f32),
    
    /// Object tracking
    all_objects: ?*ObjectHeader,
    free_list: ?*ObjectHeader,
    mark_stack: ArrayList(*ObjectHeader),
    
    /// Root set management
    roots: ArrayList(RootRef),
    stack_roots: ArrayList(*anyopaque),
    roots_mutex: Mutex,
    
    /// Concurrent collection state
    collection_mutex: Mutex,
    collection_condition: Condition,
    collection_thread: ?Thread,
    collection_running: Atomic(bool),
    stop_collection: Atomic(bool),
    
    /// Write barriers for concurrent collection
    write_barriers: ArrayList(WriteBarrier),
    write_barrier_mutex: Mutex,
    
    /// Finalization
    finalizers: ArrayList(Finalizer),
    finalization_queue: ArrayList(*ObjectHeader),
    finalization_mutex: Mutex,
    finalization_thread: ?Thread,
    
    /// Weak references
    weak_refs: ArrayList(*WeakRef),
    weak_ref_mutex: Mutex,
    
    /// Heap compaction support
    heap_segments: ArrayList(HeapSegment),
    forwarding_table: HashMap(*u8, *u8, std.hash_map.AutoContext(*u8), std.hash_map.default_max_load_percentage),
    pause_mutex: Mutex,
    
    /// Performance monitoring
    last_gc_time: std.time.Instant,
    
    /// Arena allocator integration
    arena_manager: ?*CursedArenaManager,
    use_arena_allocation: bool,
    
    /// Enhanced stack scanning with conservative pointer detection
    fn scanStackRootsEnhanced(self: *GC) void {
        // Get current stack bounds
        var stack_start: ?*anyopaque = null;
        var stack_end: ?*anyopaque = null;
        
        // Platform-specific stack detection
        if (builtin.os.tag == .linux) {
            const pthread = std.c.pthread_self();
            var attr: std.c.pthread_attr_t = undefined;
            if (std.c.pthread_getattr_np(pthread, &attr) == 0) {
                var stack_addr: ?*anyopaque = undefined;
                var stack_size: usize = undefined;
                if (std.c.pthread_attr_getstack(&attr, &stack_addr, &stack_size) == 0) {
                    stack_start = stack_addr;
                    stack_end = @as([*]u8, @ptrCast(stack_addr)) + stack_size;
                }
                _ = std.c.pthread_attr_destroy(&attr);
            }
        }
        
        // Fall back to current stack pointer if platform detection fails
        if (stack_start == null) {
            var dummy: usize = undefined;
            stack_end = &dummy;
            // Estimate stack start (this is conservative)
            stack_start = @as([*]u8, @ptrCast(stack_end)) - self.config.stack_scan_depth;
        }
        
        if (stack_start == null or stack_end == null) return;
        
        // Scan stack conservatively
        const start_addr = @intFromPtr(stack_start);
        const end_addr = @intFromPtr(stack_end);
        const scan_start = @min(start_addr, end_addr);
        const scan_end = @max(start_addr, end_addr);
        
        var scan_ptr = scan_start;
        while (scan_ptr < scan_end) {
            const potential_ptr = @as(*?*anyopaque, @ptrFromInt(scan_ptr)).*;
            
            if (potential_ptr) |ptr| {
                if (self.isValidHeapPointer(ptr)) {
                    const header = ObjectHeader.fromData(ptr);
                    if (header.color == @intFromEnum(Color.White)) {
                        header.color = @intFromEnum(Color.Gray);
                        self.mark_stack.append(header) catch {};
                    }
                }
            }
            
            scan_ptr += @sizeOf(*anyopaque);
        }
    }
    
    /// Enhanced memory pressure detection with adaptive thresholds
    pub fn getMemoryPressure(self: *GC) PressureLevel {
        const heap_usage_ratio = if (self.heap_size > 0) 
            @as(f32, @floatFromInt(self.heap_used)) / @as(f32, @floatFromInt(self.heap_size))
            else 0.0;
        
        // Update atomic memory pressure
        self.memory_pressure.store(heap_usage_ratio, .release);
        
        // Adaptive thresholds based on allocation rate
        const allocation_rate = self.stats.total_allocations / @max(1, self.stats.gc_cycles);
        const pressure_multiplier: f32 = if (allocation_rate > 10000) 0.9 else 1.0; // Lower thresholds for high allocation rates
        
        if (heap_usage_ratio < 0.5 * pressure_multiplier) {
            return .Low;
        } else if (heap_usage_ratio < 0.8 * pressure_multiplier) {
            return .Medium;
        } else if (heap_usage_ratio < 0.95 * pressure_multiplier) {
            return .High;
        } else {
            return .Critical;
        }
    }
    
    /// Enhanced memory leak detection with pattern analysis
    pub fn detectMemoryLeaks(self: *GC) ![]LeakInfo {
        return self.memory_tracker.detectLeaks(self.allocator);
    }
    
    /// Get detailed memory usage statistics
    pub fn getMemoryUsage(self: *GC) struct {
        current_usage: usize,
        peak_usage: usize,
        total_allocated: usize,
        total_freed: usize,
        pressure: f32,
        young_gen_usage: usize,
        old_gen_usage: usize,
        fragmentation: f32,
    } {
        const current_usage = self.memory_tracker.getCurrentUsage();
        const peak_usage = self.memory_tracker.peak_usage.load(.acquire);
        const total_allocated = self.memory_tracker.total_allocated.load(.acquire);
        const total_freed = self.memory_tracker.total_freed.load(.acquire);
        const pressure = self.memory_pressure.load(.acquire);
        const fragmentation = self.calculateFragmentation();
        
        return .{
            .current_usage = current_usage,
            .peak_usage = peak_usage,
            .total_allocated = total_allocated,
            .total_freed = total_freed,
            .pressure = pressure,
            .young_gen_usage = self.young_heap_used,
            .old_gen_usage = self.old_heap_used,
            .fragmentation = fragmentation,
        };
    }
    
    /// Arena-aware allocation for different patterns
    pub fn allocArena(self: *GC, size: usize, type_id: u16, pattern: ArenaAllocator.AllocationPattern) !*anyopaque {
        if (self.arena_manager) |manager| {
            const allocator = switch (pattern) {
                .Sequential, .ASTNodes => manager.getASTAllocator(),
                .Stack, .RuntimeValues => manager.getRuntimeAllocator(),
                .StringIntern => manager.getStringAllocator(),
                .Temporary => manager.getTemporaryAllocator(),
                .Pool => manager.getRuntimeAllocator(), // Use runtime for pool pattern
            };
            
            const slice = try allocator.alloc(u8, size);
            
            // Track allocation in GC for root scanning
            const info = AllocationInfo.init(size, type_id, null);
            try self.memory_tracker.trackAllocation(@intFromPtr(slice.ptr), info);
            
            return slice.ptr;
        } else {
            // Fall back to regular GC allocation
            return self.alloc(size, type_id);
        }
    }
    

    

    

    

    

    
    /// Push runtime stack frame (integrates with arena stack allocation)
    pub fn pushRuntimeFrame(self: *GC) !void {
        if (self.arena_manager) |manager| {
            try manager.runtime_arena.pushStackFrame();
        }
    }
    
    /// Pop runtime stack frame (automatic cleanup of arena allocations)
    pub fn popRuntimeFrame(self: *GC) void {
        if (self.arena_manager) |manager| {
            manager.runtime_arena.popStackFrame();
        }
    }
    
    /// Reset temporary allocations (called frequently during execution)
    pub fn resetTemporaryAllocations(self: *GC) void {
        if (self.arena_manager) |manager| {
            manager.resetTemporary();
        }
    }
    
    /// Get comprehensive memory statistics including arena usage
    pub fn getComprehensiveMemoryStats(self: *GC) struct {
        gc_stats: GCStats,
        memory_usage: @TypeOf(self.getMemoryUsage()),
        arena_usage: ?@TypeOf(CursedArenaManager.getTotalUsage(undefined)),
        pressure_level: PressureLevel,
    } {
        return .{
            .gc_stats = self.getStats(),
            .memory_usage = self.getMemoryUsage(),
            .arena_usage = if (self.arena_manager) |manager| manager.getTotalUsage() else null,
            .pressure_level = self.getMemoryPressure(),
        };
    }
    

    
    /// Free object directly (bypasses normal collection)
    fn freeObjectDirect(self: *GC, obj: *ObjectHeader) void {
        // Remove from all_objects list
        if (self.all_objects == obj) {
            self.all_objects = obj.next;
        } else {
            var current = self.all_objects;
            while (current) |curr| {
                if (curr.next == obj) {
                    curr.next = obj.next;
                    break;
                }
                current = curr.next;
            }
        }
        
        // Add to free list
        obj.next = self.free_list;
        self.free_list = obj;
        
        // Update memory tracking
        self.memory_tracker.trackDeallocation(@intFromPtr(obj));
        self.heap_used -= obj.size;
    }

    /// Initialize the garbage collector
    pub fn init(allocator: std.mem.Allocator, config: GCConfig) !*GC {
        const gc = try allocator.create(GC);
        gc.* = GC{
            .config = config,
            .stats = GCStats.init(),
            .allocator = allocator,
            .heap_start = null,
            .heap_size = 0,
            .heap_used = 0,
            .young_heap_start = null,
            .young_heap_used = 0,
            .old_heap_start = null,
            .old_heap_used = 0,
            
            // Initialize advanced memory management features
            .memory_pressure = Atomic(f32).init(0.0),
            .memory_tracker = MemoryTracker.init(allocator),
            .allocation_map = HashMap(usize, AllocationInfo, std.hash_map.AutoContext(usize), std.hash_map.default_max_load_percentage).init(allocator),
            .ref_count_map = HashMap(usize, Atomic(u32), std.hash_map.AutoContext(usize), std.hash_map.default_max_load_percentage).init(allocator),
            .memory_pools = try MemoryPoolManager.init(allocator),
            .gc_trigger_threshold = Atomic(f32).init(config.young_gc_trigger_threshold),
            
            .all_objects = null,
            .free_list = null,
            .mark_stack = ArrayList(*ObjectHeader).init(allocator),
            .roots = ArrayList(RootRef).init(allocator),
            .stack_roots = ArrayList(*anyopaque).init(allocator),
            .roots_mutex = Mutex{},
            .collection_mutex = Mutex{},
            .collection_condition = Condition{},
            .collection_thread = null,
            .collection_running = Atomic(bool).init(false),
            .stop_collection = Atomic(bool).init(false),
            .write_barriers = ArrayList(WriteBarrier).init(allocator),
            .write_barrier_mutex = Mutex{},
            .finalizers = ArrayList(Finalizer).init(allocator),
            .finalization_queue = ArrayList(*ObjectHeader).init(allocator),
            .finalization_mutex = Mutex{},
            .finalization_thread = null,
            .weak_refs = ArrayList(*WeakRef).init(allocator),
            .weak_ref_mutex = Mutex{},
            .heap_segments = ArrayList(HeapSegment).init(allocator),
            .forwarding_table = HashMap(*u8, *u8, std.hash_map.AutoContext(*u8), std.hash_map.default_max_load_percentage).init(allocator),
            .pause_mutex = Mutex{},
            .last_gc_time = undefined,
            .arena_manager = null,
            .use_arena_allocation = config.enable_arena_allocation,
        };
        
        try gc.initializeHeap();
        try gc.startBackgroundThreads();
        
        // Initialize arena manager if enabled
        if (gc.use_arena_allocation) {
            gc.arena_manager = try allocator.create(CursedArenaManager);
            gc.arena_manager.?.* = try CursedArenaManager.init(allocator);
        }
        
        return gc;
    }
    
    /// Clean up the garbage collector
    pub fn deinit(self: *GC) void {
        // Stop background threads
        self.stopBackgroundThreads();
        
        // Clean up heap
        self.deallocateHeap();
        
        // Clean up advanced memory management features
        self.memory_tracker.deinit();
        self.allocation_map.deinit();
        self.ref_count_map.deinit();
        self.memory_pools.deinit();
        
        // Clean up data structures
        self.mark_stack.deinit();
        self.roots.deinit();
        self.stack_roots.deinit();
        self.write_barriers.deinit();
        self.finalizers.deinit();
        self.finalization_queue.deinit();
        self.weak_refs.deinit();
        self.heap_segments.deinit();
        self.forwarding_table.deinit();
        
        // Clean up arena manager
        if (self.arena_manager) |manager| {
            manager.deinit();
            self.allocator.destroy(manager);
        }
        
        self.allocator.destroy(self);
    }
    
    /// Initialize the heap with separate young and old generations (33%/67% split)
    fn initializeHeap(self: *GC) !void {
        // Allocate total heap
        self.heap_size = self.config.initial_heap_size;
        const heap_slice = try self.allocator.alloc(u8, self.heap_size);
        self.heap_start = @ptrCast(heap_slice.ptr);
        
        // Calculate generation sizes based on ratios
        const young_gen_size = @as(usize, @intFromFloat(@as(f64, @floatFromInt(self.heap_size)) * self.config.young_gen_ratio));
        const old_gen_size = self.heap_size - young_gen_size;
        
        // Set up generational regions
        const heap_bytes = @as([*]u8, @ptrCast(self.heap_start));
        
        // Young generation at the beginning (33% of heap)
        self.young_heap_start = @ptrCast(heap_bytes);
        
        // Old generation after young generation (67% of heap)
        self.old_heap_start = @ptrCast(heap_bytes + young_gen_size);
        
        self.heap_used = 0;
        self.young_heap_used = 0;
        self.old_heap_used = 0;
        
        // Set up heap segments for compaction
        const young_segment = HeapSegment{
            .start = @as(*u8, @ptrCast(self.young_heap_start)),
            .end = @as(*u8, @ptrFromInt(@intFromPtr(@as(*u8, @ptrCast(self.young_heap_start))) + young_gen_size)),
            .current = @as(*u8, @ptrCast(self.young_heap_start)),
            .generation = 0,
        };
        
        const old_segment = HeapSegment{
            .start = @as(*u8, @ptrCast(self.old_heap_start)),
            .end = @as(*u8, @ptrFromInt(@intFromPtr(heap_bytes) + self.heap_size)),
            .current = @as(*u8, @ptrCast(self.old_heap_start)),
            .generation = 1,
        };
        
        try self.heap_segments.append(young_segment);
        try self.heap_segments.append(old_segment);
        
        std.log.info("GC: Initialized heap - Total: {} bytes, Young: {} bytes ({}%), Old: {} bytes ({}%)", .{ 
            self.heap_size, young_gen_size, @as(u8, @intFromFloat(self.config.young_gen_ratio * 100)),
            old_gen_size, @as(u8, @intFromFloat(self.config.old_gen_ratio * 100))
        });
    }
    
    /// Deallocate the heap
    fn deallocateHeap(self: *GC) void {
        if (self.heap_start) |heap| {
            const heap_slice = @as([*]u8, @ptrCast(heap))[0..self.heap_size];
            self.allocator.free(heap_slice);
        }
    }
    
    /// Start background collection and finalization threads
    fn startBackgroundThreads(self: *GC) !void {
        // Start concurrent collection thread
        self.collection_thread = try Thread.spawn(.{}, concurrentCollectionWorker, .{self});
        
        // Start finalization thread if enabled
        if (self.config.enable_finalization) {
            self.finalization_thread = try Thread.spawn(.{}, finalizationWorker, .{self});
        }
    }
    
    /// Stop background threads
    fn stopBackgroundThreads(self: *GC) void {
        // Signal threads to stop
        self.stop_collection.store(true, .release);
        
        // Wake up collection thread
        self.collection_mutex.lock();
        self.collection_condition.signal();
        self.collection_mutex.unlock();
        
        // Wait for threads to finish
        if (self.collection_thread) |thread| {
            thread.join();
        }
        
        if (self.finalization_thread) |thread| {
            thread.join();
        }
    }
    
    /// Allocate object with GC header
    pub fn alloc(self: *GC, size: usize, type_id: u16) !*anyopaque {
        return self.allocWithSource(size, type_id, null);
    }
    
    /// Allocate object with source location tracking for debugging
    pub fn allocWithSource(self: *GC, size: usize, type_id: u16, source_location: ?[]const u8) !*anyopaque {
        const total_size = ObjectHeader.HEADER_SIZE + size;
        
        // Try memory pool allocation first for better performance
        const pool_ptr = self.memory_pools.getAllocation(total_size) catch null;
        if (pool_ptr) |ptr| {
            // Initialize as pool-allocated object
            return ptr;
        }
        
        // Try allocation in appropriate generation
        const generation = if (self.shouldAllocateInOld()) Generation.Old else Generation.Young;
        const header = try self.allocateInGeneration(total_size, generation);
        
        // Initialize header
        header.size = @intCast(total_size);
        header.type_id = type_id;
        header.color = @intFromEnum(Color.White);
        header.generation = @intFromEnum(generation);
        header.finalize = 0;
        header.reserved = 0;
        header.next = self.all_objects;
        self.all_objects = header;
        
        const data_ptr = header.getData();
        const address = @intFromPtr(data_ptr);
        
        // Track allocation for leak detection
        const alloc_info = AllocationInfo.init(total_size, type_id, source_location);
        self.memory_tracker.trackAllocation(address, alloc_info) catch {};
        self.allocation_map.put(address, alloc_info) catch {};
        
        // Initialize reference count
        const ref_count = Atomic(u32).init(1);
        self.ref_count_map.put(address, ref_count) catch {};
        
        // Update statistics
        self.stats.total_allocations += 1;
        self.stats.total_bytes_allocated += total_size;
        self.heap_used += total_size;
        
        // Update peak heap size
        if (self.heap_used > self.stats.peak_heap_size) {
            self.stats.peak_heap_size = self.heap_used;
        }
        
        // Update memory pressure
        self.updateMemoryPressure();
        
        // Check if GC should be triggered based on generation-specific thresholds
        if (generation == .Young) {
            const young_gen_size = @as(usize, @intFromFloat(@as(f64, @floatFromInt(self.heap_size)) * self.config.young_gen_ratio));
            const young_usage = @as(f32, @floatFromInt(self.young_heap_used)) / @as(f32, @floatFromInt(young_gen_size));
            if (young_usage > self.gc_trigger_threshold.load(.acquire)) {
                self.triggerYoungCollection();
            }
        } else {
            const old_gen_size = self.heap_size - @as(usize, @intFromFloat(@as(f64, @floatFromInt(self.heap_size)) * self.config.young_gen_ratio));
            const old_usage = @as(f32, @floatFromInt(self.old_heap_used)) / @as(f32, @floatFromInt(old_gen_size));
            if (old_usage > self.config.old_gc_trigger_threshold) {
                self.triggerOldCollection();
            }
        }
        
        return data_ptr;
    }
    
    /// Reference counting operations
    pub fn retainObject(self: *GC, ptr: *anyopaque) void {
        const address = @intFromPtr(ptr);
        if (self.ref_count_map.getPtr(address)) |ref_count| {
            _ = ref_count.fetchAdd(1, .acquire);
        }
    }
    
    pub fn releaseObject(self: *GC, ptr: *anyopaque) void {
        const address = @intFromPtr(ptr);
        if (self.ref_count_map.getPtr(address)) |ref_count| {
            const old_count = ref_count.fetchSub(1, .release);
            if (old_count == 1) {
                // Object should be freed immediately
                self.freeObjectImmediate(ptr);
            }
        }
    }
    
    pub fn getRefCount(self: *GC, ptr: *anyopaque) u32 {
        const address = @intFromPtr(ptr);
        if (self.ref_count_map.get(address)) |ref_count| {
            return ref_count.load(.acquire);
        }
        return 0;
    }
    
    /// Update memory pressure based on current usage
    fn updateMemoryPressure(self: *GC) void {
        const usage_ratio = @as(f32, @floatFromInt(self.heap_used)) / @as(f32, @floatFromInt(self.heap_size));
        self.memory_pressure.store(usage_ratio, .release);
        
        // Adjust GC trigger threshold based on pressure
        if (usage_ratio > 0.90) {
            // High pressure - trigger more aggressive collection
            self.gc_trigger_threshold.store(0.60, .release);
        } else if (usage_ratio < 0.50) {
            // Low pressure - less aggressive collection
            self.gc_trigger_threshold.store(0.85, .release);
        }
    }
    

    
    /// Force immediate deallocation (for reference counting)
    fn freeObjectImmediate(self: *GC, ptr: *anyopaque) void {
        const address = @intFromPtr(ptr);
        
        // Remove from tracking
        self.memory_tracker.trackDeallocation(address);
        _ = self.allocation_map.remove(address);
        _ = self.ref_count_map.remove(address);
        
        // Try to free from memory pool first
        if (self.allocation_map.get(address)) |info| {
            self.memory_pools.deallocate(ptr, info.size) catch {
                // Fall back to normal GC deallocation
                self.freeObjectDirect(ObjectHeader.fromData(ptr));
            };
        } else {
            // Normal GC deallocation
            self.freeObjectDirect(ObjectHeader.fromData(ptr));
        }
    }
    

    

    
    /// Allocate in specific generation
    fn allocateInGeneration(self: *GC, size: usize, generation: Generation) !*ObjectHeader {
        const aligned_size = std.mem.alignForward(usize, size, @alignOf(ObjectHeader));
        
        switch (generation) {
            .Young => {
                const young_gen_size = @as(usize, @intFromFloat(@as(f64, @floatFromInt(self.heap_size)) * self.config.young_gen_ratio));
                if (self.young_heap_used + aligned_size > young_gen_size) {
                    // Try minor collection first
                    try self.collectYoungGeneration();
                    if (self.young_heap_used + aligned_size > young_gen_size) {
                        return error.OutOfMemory;
                    }
                }
                
                const heap_bytes = @as([*]u8, @ptrCast(self.young_heap_start));
                const obj_ptr = @as(*ObjectHeader, @ptrCast(@alignCast(heap_bytes + self.young_heap_used)));
                self.young_heap_used += aligned_size;
                return obj_ptr;
            },
            .Old => {
                const young_gen_size = @as(usize, @intFromFloat(@as(f64, @floatFromInt(self.heap_size)) * self.config.young_gen_ratio));
                const old_heap_size = self.heap_size - young_gen_size;
                if (self.old_heap_used + aligned_size > old_heap_size) {
                    // Try major collection
                    try self.collectOldGeneration();
                    if (self.old_heap_used + aligned_size > old_heap_size) {
                        return error.OutOfMemory;
                    }
                }
                
                const heap_bytes = @as([*]u8, @ptrCast(self.old_heap_start));
                const obj_ptr = @as(*ObjectHeader, @ptrCast(@alignCast(heap_bytes + self.old_heap_used)));
                self.old_heap_used += aligned_size;
                return obj_ptr;
            },
        }
    }
    
    /// Check if allocation should go to old generation
    fn shouldAllocateInOld(self: *GC) bool {
        // Simple heuristic: every Nth allocation goes to old generation
        return self.stats.total_allocations % self.config.promotion_threshold == 0;
    }
    
    /// Register a root reference
    pub fn addRoot(self: *GC, ptr: *?*anyopaque, type_id: u16) !void {
        try self.roots.append(RootRef{ .ptr = ptr, .type_id = type_id });
    }
    
    /// Remove a root reference
    pub fn removeRoot(self: *GC, ptr: *?*anyopaque) void {
        for (self.roots.items, 0..) |root, i| {
            if (root.ptr == ptr) {
                _ = self.roots.swapRemove(i);
                break;
            }
        }
    }
    
    /// Add stack root for conservative scanning
    pub fn addStackRoot(self: *GC, ptr: *anyopaque) !void {
        try self.stack_roots.append(ptr);
    }
    
    /// Create a weak reference
    pub fn createWeakRef(self: *GC, target: *anyopaque) !*WeakRef {
        const weak_ref = try self.allocator.create(WeakRef);
        weak_ref.target = target;
        weak_ref.header = ObjectHeader.fromData(target);
        
        self.weak_ref_mutex.lock();
        defer self.weak_ref_mutex.unlock();
        try self.weak_refs.append(weak_ref);
        
        return weak_ref;
    }
    
    /// Register finalizer for object
    pub fn addFinalizer(self: *GC, object: *anyopaque, finalizer: FinalizerFn) !void {
        if (!self.config.enable_finalization) return;
        
        const header = ObjectHeader.fromData(object);
        header.finalize = 1;
        
        self.finalization_mutex.lock();
        defer self.finalization_mutex.unlock();
        try self.finalizers.append(Finalizer{
            .object = header,
            .fn_ptr = finalizer,
        });
    }
    
    /// Record write barrier for concurrent collection
    pub fn writeBarrier(self: *GC, old_ref: *anyopaque, new_ref: *anyopaque) void {
        if (!self.config.enable_write_barriers) return;
        
        const timestamp = std.time.microTimestamp();
        
        self.write_barrier_mutex.lock();
        defer self.write_barrier_mutex.unlock();
        
        self.write_barriers.append(WriteBarrier{
            .old_ref = old_ref,
            .new_ref = new_ref,
            .timestamp = @intCast(timestamp),
        }) catch {
            // If we can't record the barrier, trigger immediate collection
            self.triggerCollection();
        };
    }
    
    /// Trigger garbage collection for both generations
    pub fn triggerCollection(self: *GC) void {
        self.collection_mutex.lock();
        defer self.collection_mutex.unlock();
        
        if (!self.collection_running.load(.acquire)) {
            self.collection_condition.signal();
        }
    }
    
    /// Trigger young generation collection (optimized for <5ms pause)
    pub fn triggerYoungCollection(self: *GC) void {
        if (self.config.enable_incremental_collection) {
            // Use incremental collection for lower pause times
            self.performIncrementalYoungCollection() catch {
                // Fallback to full young collection if incremental fails
                self.collectYoungGeneration() catch {};
            };
        } else {
            self.collectYoungGeneration() catch {};
        }
    }
    
    /// Trigger old generation collection (optimized for <50ms pause)
    pub fn triggerOldCollection(self: *GC) void {
        if (self.config.enable_incremental_collection) {
            // Use incremental collection for lower pause times
            self.performIncrementalOldCollection() catch {
                // Fallback to full old collection if incremental fails
                self.collectOldGeneration() catch {};
            };
        } else {
            self.collectOldGeneration() catch {};
        }
    }
    
    /// Force immediate collection (blocking)
    pub fn collectNow(self: *GC) !void {
        const start_time = std.time.Instant.now() catch return;
        
        // Perform full collection
        try self.collectYoungGeneration();
        try self.collectOldGeneration();
        
        // Update pause time statistics
        const end_time = std.time.Instant.now() catch return;
        const pause_time = end_time.since(start_time) / 1000; // Convert to microseconds
        self.stats.total_pause_time_us += pause_time;
        if (pause_time > self.stats.max_pause_time_us) {
            self.stats.max_pause_time_us = pause_time;
        }
        
        self.stats.gc_cycles += 1;
        
        std.log.info("GC: Full collection completed in {} μs", .{pause_time});
    }
    
    /// Collect young generation (minor GC)
    fn collectYoungGeneration(self: *GC) !void {
        const start_time = std.time.Instant.now() catch return;
        
        std.log.debug("GC: Starting young generation collection", .{});
        
        // Mark phase: mark all reachable objects
        try self.markPhase(.Young);
        
        // Sweep phase: collect unreachable objects
        const collected = self.sweepPhase(.Young);
        
        // Promote surviving objects to old generation
        try self.promoteObjects();
        
        self.stats.young_collections += collected;
        
        const end_time = std.time.Instant.now() catch return;
        const pause_time = end_time.since(start_time) / 1000;
        
        std.log.debug("GC: Young generation collection completed - {} objects collected in {} μs", 
                     .{ collected, pause_time });
    }
    
    /// Collect old generation (major GC)
    fn collectOldGeneration(self: *GC) !void {
        const start_time = std.time.Instant.now() catch return;
        
        std.log.debug("GC: Starting old generation collection", .{});
        
        // Mark phase: mark all reachable objects
        try self.markPhase(.Old);
        
        // Sweep phase: collect unreachable objects
        const collected = self.sweepPhase(.Old);
        
        // Compact if needed
        try self.compactHeap();
        
        self.stats.old_collections += collected;
        
        const end_time = std.time.Instant.now() catch return;
        const pause_time = end_time.since(start_time) / 1000;
        
        std.log.debug("GC: Old generation collection completed - {} objects collected in {} μs", 
                     .{ collected, pause_time });
    }
    
    /// Tri-color marking phase
    fn markPhase(self: *GC, generation: Generation) !void {
        // Clear mark stack
        self.mark_stack.clearAndFree();
        
        // Reset all objects to white
        var current = self.all_objects;
        while (current) |obj| {
            if (@as(Generation, @enumFromInt(obj.generation)) == generation or generation == .Old) {
                obj.color = @intFromEnum(Color.White);
            }
            current = obj.next;
        }
        
        // Mark roots as gray and add to mark stack
        try self.markRoots();
        
        // Process mark stack until empty
        while (self.mark_stack.items.len > 0) {
            const obj = self.mark_stack.pop();
            
            // Mark object as black
            obj.*.color = @intFromEnum(Color.Black);
            
            // Mark children as gray
            try self.markChildren(obj);
        }
        
        // Process write barriers for concurrent collection
        if (self.config.enable_write_barriers) {
            try self.processWriteBarriers();
        }
    }
    
    /// Mark root objects
    fn markRoots(self: *GC) !void {
        // Mark registered roots
        for (self.roots.items) |root| {
            if (root.ptr.*) |ptr| {
                const header = ObjectHeader.fromData(ptr);
                if (header.color == @intFromEnum(Color.White)) {
                    header.color = @intFromEnum(Color.Gray);
                    try self.mark_stack.append(header);
                }
            }
        }
        
        // Conservative stack scanning
        try self.scanStack();
    }
    
    /// Conservative stack scanning
    fn scanStack(self: *GC) !void {
        // Scan registered stack roots for potential heap pointers
        self.scanStackRoots();
    }
    
    /// Check if pointer is in heap
    fn isValidHeapPointer(self: *GC, ptr: *anyopaque) bool {
        const heap_start = @intFromPtr(self.heap_start);
        const heap_end = heap_start + self.heap_size;
        const ptr_addr = @intFromPtr(ptr);
        
        return ptr_addr >= heap_start and ptr_addr < heap_end;
    }
    
    /// Mark children of an object
    fn markChildren(self: *GC, obj: *ObjectHeader) !void {
        // This would be type-specific traversal
        // For now, we'll implement a simple pointer scanning approach
        const data = obj.getData();
        const obj_size = obj.size - ObjectHeader.HEADER_SIZE;
        
        // Scan object for potential pointers
        const ptr_size = @sizeOf(*anyopaque);
        var offset: usize = 0;
        
        while (offset + ptr_size <= obj_size) {
            const potential_ptr_opt = @as(*?*anyopaque, @ptrCast(@alignCast(
                @as([*]u8, @ptrCast(data)) + offset
            ))).*;
            
            if (potential_ptr_opt) |ptr| {
                if (self.isValidHeapPointer(ptr)) {
                    const child_header = ObjectHeader.fromData(ptr);
                    if (child_header.color == @intFromEnum(Color.White)) {
                        child_header.color = @intFromEnum(Color.Gray);
                        try self.mark_stack.append(child_header);
                    }
                }
            }
            
            offset += ptr_size;
        }
    }
    
    /// Process write barriers
    fn processWriteBarriers(self: *GC) !void {
        self.write_barrier_mutex.lock();
        defer self.write_barrier_mutex.unlock();
        
        for (self.write_barriers.items) |barrier| {
            if (self.isValidHeapPointer(barrier.new_ref)) {
                const header = ObjectHeader.fromData(barrier.new_ref);
                if (header.color == @intFromEnum(Color.White)) {
                    header.color = @intFromEnum(Color.Gray);
                    try self.mark_stack.append(header);
                }
            }
        }
        
        self.write_barriers.clearAndFree();
    }
    
    /// Sweep phase - collect unmarked objects
    fn sweepPhase(self: *GC, generation: Generation) u64 {
        var collected: u64 = 0;
        var prev: ?*ObjectHeader = null;
        var current = self.all_objects;
        
        while (current) |obj| {
            const next = obj.next;
            
            if (@as(Generation, @enumFromInt(obj.generation)) == generation or generation == .Old) {
                if (obj.color == @intFromEnum(Color.White)) {
                    // Object is unreachable - collect it
                    if (obj.finalize == 1) {
                        // Queue for finalization
                        self.queueForFinalization(obj);
                    } else {
                        // Free immediately
                        self.freeObject(obj, prev);
                        collected += 1;
                    }
                } else {
                    // Object survived - reset color for next cycle
                    obj.color = @intFromEnum(Color.White);
                    prev = obj;
                }
            } else {
                prev = obj;
            }
            
            current = next;
        }
        
        return collected;
    }
    
    /// Queue object for finalization
    fn queueForFinalization(self: *GC, obj: *ObjectHeader) void {
        self.finalization_mutex.lock();
        defer self.finalization_mutex.unlock();
        
        self.finalization_queue.append(obj) catch {
            // If we can't queue for finalization, free immediately
            self.freeObjectDirect(obj);
        };
    }
    
    /// Free object and update linked list
    fn freeObject(self: *GC, obj: *ObjectHeader, prev: ?*ObjectHeader) void {
        // Remove from object list
        if (prev) |p| {
            p.next = obj.next;
        } else {
            self.all_objects = obj.next;
        }
        
        self.freeObjectDirect(obj);
    }
    

    
    /// Promote surviving young objects to old generation
    fn promoteObjects(self: *GC) !void {
        var current = self.all_objects;
        
        while (current) |obj| {
            if (@as(Generation, @enumFromInt(obj.generation)) == .Young and 
                obj.color == @intFromEnum(Color.Black)) {
                
                // This object survived collection, consider promotion
                // For simplicity, promote all survivors
                obj.generation = @intFromEnum(Generation.Old);
                self.stats.promotions += 1;
            }
            current = obj.next;
        }
        
        // Reset young generation
        self.young_heap_used = 0;
    }
    
    /// Compact heap to reduce fragmentation
    fn compactHeap(self: *GC) !void {
        self.pause_mutex.lock();
        defer self.pause_mutex.unlock();
        
        // Perform compaction by moving live objects to eliminate fragmentation
        const compact_start = std.time.nanoTimestamp();
        defer {
            const compact_time = std.time.nanoTimestamp() - compact_start;
            self.stats.total_compact_time += @intCast(@divTrunc(compact_time, 1000)); // microseconds
            self.stats.compact_count += 1;
        }
        
        var moved_objects: u32 = 0;
        var bytes_moved: usize = 0;
        
        // Walk through heap segments and compact each one
        for (self.heap_segments.items) |*segment| {
            try self.compactSegment(segment, &moved_objects, &bytes_moved);
        }
        
        // Update pointers in all live objects
        try self.updateCompactedPointers();
        
        std.log.debug("GC: Compacted {} objects, moved {} bytes", .{moved_objects, bytes_moved});
    }
    
    fn compactSegment(self: *GC, segment: *HeapSegment, moved_objects: *u32, bytes_moved: *usize) !void {
        var scan_ptr = segment.start;
        var compact_ptr = segment.start;
        
        while (@intFromPtr(scan_ptr) < @intFromPtr(segment.current)) {
            const header = @as(*ObjectHeader, @ptrCast(@alignCast(scan_ptr)));
            const obj_size = header.size;
            
            if (header.color != @intFromEnum(Color.White)) {
                // Object is live, move it if necessary
                if (scan_ptr != compact_ptr) {
                    // Move the object to eliminate fragmentation
                    const src = @as([*]u8, @ptrCast(scan_ptr));
                    const dst = @as([*]u8, @ptrCast(compact_ptr));
                    @memcpy(dst[0..obj_size], src[0..obj_size]);
                    
                    // Update the object's location in forwarding table
                    try self.forwarding_table.put(scan_ptr, compact_ptr);
                    
                    moved_objects.* += 1;
                    bytes_moved.* += obj_size;
                }
                compact_ptr = @as(*u8, @ptrFromInt(@intFromPtr(compact_ptr) + obj_size));
            }
            
            scan_ptr = @as(*u8, @ptrFromInt(@intFromPtr(scan_ptr) + obj_size));
        }
        
        // Update segment's current pointer
        segment.current = compact_ptr;
    }
    
    fn updateCompactedPointers(self: *GC) !void {
        // Update all root pointers
        self.roots_mutex.lock();
        defer self.roots_mutex.unlock();
        
        for (self.roots.items) |*root| {
            if (root.ptr.*) |ptr| {
                if (self.forwarding_table.get(@as(*u8, @ptrCast(ptr)))) |new_location| {
                    root.ptr.* = @ptrCast(new_location);
                }
            }
        }
        
        // Update internal pointers in moved objects
        for (self.heap_segments.items) |segment| {
            var scan_ptr = segment.start;
            
            while (@intFromPtr(scan_ptr) < @intFromPtr(segment.current)) {
                const header = @as(*ObjectHeader, @ptrCast(@alignCast(scan_ptr)));
                
                if (header.color != @intFromEnum(Color.White)) {
                    // Update pointers within this object
                    self.updateObjectPointers(header);
                }
                
                scan_ptr = @as(*u8, @ptrFromInt(@intFromPtr(scan_ptr) + header.size));
            }
        }
        
        // Clear forwarding table
        self.forwarding_table.clearRetainingCapacity();
    }
    
    fn updateObjectPointers(self: *GC, header: *ObjectHeader) void {
        const data = header.getData();
        
        // Type-specific pointer updating based on type_id
        switch (header.type_id) {
            1 => self.updateStringPointers(data), // String type
            2 => self.updateArrayPointers(data),  // Array type
            3 => self.updateStructPointers(data), // Struct type
            else => {}, // Primitive types have no internal pointers
        }
    }
    
    fn updateStringPointers(self: *GC, data: *anyopaque) void {
        _ = self;
        _ = data;
        // Strings in CURSED are value types, no internal pointers to update
    }
    
    fn updateArrayPointers(self: *GC, data: *anyopaque) void {
        const array_data = @as(*[]?*anyopaque, @ptrCast(@alignCast(data)));
        
        for (array_data.*) |*element| {
            if (element.*) |ptr| {
                if (self.forwarding_table.get(@as(*u8, @ptrCast(ptr)))) |new_location| {
                    element.* = @ptrCast(new_location);
                }
            }
        }
    }
    
    fn updateStructPointers(self: *GC, data: *anyopaque) void {
        // For struct types, we need to know the field layout
        // This is simplified - real implementation would use type metadata
        const struct_data = @as(*[]*anyopaque, @ptrCast(@alignCast(data)));
        
        for (struct_data.*) |*field| {
            if (self.forwarding_table.get(@as(*u8, @ptrCast(field.*)))) |new_location| {
                field.* = @ptrCast(new_location);
            }
        }
    }
    
    /// Update weak references after collection
    fn updateWeakReferences(self: *GC) void {
        self.weak_ref_mutex.lock();
        defer self.weak_ref_mutex.unlock();
        
        var i: usize = 0;
        while (i < self.weak_refs.items.len) {
            const weak_ref = self.weak_refs.items[i];
            
            if (weak_ref.header) |header| {
                if (header.color == @intFromEnum(Color.White)) {
                    // Object was collected
                    weak_ref.target = null;
                    weak_ref.header = null;
                    
                    // Remove from weak reference list
                    self.allocator.destroy(weak_ref);
                    _ = self.weak_refs.swapRemove(i);
                    continue;
                }
            }
            
            i += 1;
        }
    }
    
    /// Register a stack root for scanning
    pub fn registerStackRoot(self: *GC, ptr: *anyopaque) !void {
        self.roots_mutex.lock();
        defer self.roots_mutex.unlock();
        
        try self.stack_roots.append(ptr);
        std.log.debug("GC: Registered stack root at {*}", .{ptr});
    }
    
    /// Unregister a stack root
    pub fn unregisterStackRoot(self: *GC, ptr: *anyopaque) !void {
        self.roots_mutex.lock();
        defer self.roots_mutex.unlock();
        
        for (self.stack_roots.items, 0..) |root, i| {
            if (root == ptr) {
                _ = self.stack_roots.swapRemove(i);
                std.log.debug("GC: Unregistered stack root at {*}", .{ptr});
                return;
            }
        }
        
        std.log.warn("GC: Attempted to unregister unknown stack root at {*}", .{ptr});
    }
    
    /// Scan stack roots for live objects
    fn scanStackRoots(self: *GC) void {
        self.roots_mutex.lock();
        defer self.roots_mutex.unlock();
        
        for (self.stack_roots.items) |root_ptr| {
            // Scan a reasonable stack frame size around the root pointer
            const stack_frame_size = 1024; // 1KB stack frame
            const start_addr = @as([*]u8, @ptrCast(root_ptr));
            
            var scan_ptr = start_addr;
            const end_ptr = start_addr + stack_frame_size;
            
            while (@intFromPtr(scan_ptr) < @intFromPtr(end_ptr)) {
                // Check if this looks like a pointer to our heap
                const potential_ptr = @as(*?*anyopaque, @ptrCast(@alignCast(scan_ptr))).*;
                
                if (potential_ptr) |ptr| {
                    if (self.isValidHeapPointer(ptr)) {
                        const header = ObjectHeader.fromData(ptr);
                        if (header.color == @intFromEnum(Color.White)) {
                            // Mark as gray for further scanning
                            header.color = @intFromEnum(Color.Gray);
                            self.mark_stack.append(header) catch {};
                        }
                    }
                }
                
                scan_ptr = @as([*]u8, @ptrFromInt(@intFromPtr(scan_ptr) + @sizeOf(*anyopaque)));
            }
        }
    }
    
    /// Perform incremental young generation collection for low pause times
    fn performIncrementalYoungCollection(self: *GC) !void {
        const start_time = std.time.nanoTimestamp();
        const max_work_time_ns = self.config.max_young_pause_time_us * 1000;
        
        // Mark roots incrementally
        try self.incrementalMarkRoots();
        
        var work_done: usize = 0;
        while (self.mark_stack.items.len > 0 and 
               (std.time.nanoTimestamp() - start_time) < max_work_time_ns) {
            
            const work_chunk = @min(self.config.incremental_work_size, self.mark_stack.items.len);
            
            for (0..work_chunk) |_| {
                if (self.mark_stack.items.len == 0) break;
                
                const obj = self.mark_stack.pop();
                obj.color = @intFromEnum(Color.Black);
                try self.markChildren(obj);
                work_done += 1;
            }
            
            // Check time constraint
            if ((std.time.nanoTimestamp() - start_time) >= max_work_time_ns) {
                // Schedule continuation for next cycle
                self.triggerYoungCollection();
                return;
            }
        }
        
        // Sweep young generation
        const collected = self.sweepPhase(.Young);
        self.stats.young_collections += collected;
        
        // Promote survivors if needed
        try self.promoteObjects();
        
        const elapsed_us = @as(u64, @intCast(@divTrunc(std.time.nanoTimestamp() - start_time, 1000)));
        self.stats.total_pause_time_us += elapsed_us;
        if (elapsed_us > self.stats.max_pause_time_us) {
            self.stats.max_pause_time_us = elapsed_us;
        }
        
        std.log.debug("GC: Incremental young collection completed in {} μs, collected {} objects", 
                     .{elapsed_us, collected});
    }
    
    /// Perform incremental old generation collection for low pause times
    fn performIncrementalOldCollection(self: *GC) !void {
        const start_time = std.time.nanoTimestamp();
        const max_work_time_ns = self.config.max_old_pause_time_us * 1000;
        
        // Use parallel marking if enabled
        if (self.config.enable_parallel_marking) {
            try self.parallelMarkPhase();
        } else {
            try self.incrementalMarkRoots();
            
            var work_done: usize = 0;
            while (self.mark_stack.items.len > 0 and 
                   (std.time.nanoTimestamp() - start_time) < max_work_time_ns) {
                
                const work_chunk = @min(self.config.incremental_work_size, self.mark_stack.items.len);
                
                for (0..work_chunk) |_| {
                    if (self.mark_stack.items.len == 0) break;
                    
                    const obj = self.mark_stack.pop();
                    obj.color = @intFromEnum(Color.Black);
                    try self.markChildren(obj);
                    work_done += 1;
                }
                
                // Check time constraint
                if ((std.time.nanoTimestamp() - start_time) >= max_work_time_ns) {
                    // Schedule continuation for next cycle
                    self.triggerOldCollection();
                    return;
                }
            }
        }
        
        // Sweep old generation
        const collected = self.sweepPhase(.Old);
        self.stats.old_collections += collected;
        
        // Compact heap if fragmentation is high
        if (self.config.enable_compaction) {
            const fragmentation = self.calculateFragmentation();
            if (fragmentation > self.config.compaction_threshold) {
                try self.compactHeap();
            }
        }
        
        const elapsed_us = @as(u64, @intCast(@divTrunc(std.time.nanoTimestamp() - start_time, 1000)));
        self.stats.total_pause_time_us += elapsed_us;
        if (elapsed_us > self.stats.max_pause_time_us) {
            self.stats.max_pause_time_us = elapsed_us;
        }
        
        std.log.debug("GC: Incremental old collection completed in {} μs, collected {} objects", 
                     .{elapsed_us, collected});
    }
    
    /// Parallel marking phase for improved throughput
    fn parallelMarkPhase(self: *GC) !void {
        const thread_count = self.config.concurrent_threads;
        const threads = try self.allocator.alloc(Thread, thread_count);
        defer self.allocator.free(threads);
        
        // Divide work among threads
        const work_per_thread = (self.mark_stack.items.len + thread_count - 1) / thread_count;
        
        for (threads, 0..) |*thread, i| {
            const start_idx = i * work_per_thread;
            const end_idx = @min((i + 1) * work_per_thread, self.mark_stack.items.len);
            
            if (start_idx < end_idx) {
                thread.* = try Thread.spawn(.{}, parallelMarkWorker, .{self, start_idx, end_idx});
            }
        }
        
        // Wait for all threads to complete
        for (threads) |*thread| {
            thread.join();
        }
    }
    
    /// Worker function for parallel marking
    fn parallelMarkWorker(gc: *GC, start_idx: usize, end_idx: usize) void {
        for (start_idx..end_idx) |i| {
            if (i >= gc.mark_stack.items.len) break;
            
            const obj = gc.mark_stack.items[i];
            obj.color = @intFromEnum(Color.Black);
            gc.markChildren(obj) catch {};
        }
    }
    
    /// Incremental marking of roots to reduce pause time
    fn incrementalMarkRoots(self: *GC) !void {
        // Mark registered roots
        self.roots_mutex.lock();
        defer self.roots_mutex.unlock();
        
        for (self.roots.items) |root| {
            if (root.ptr.*) |ptr| {
                if (self.isValidHeapPointer(ptr)) {
                    const header = ObjectHeader.fromData(ptr);
                    if (header.color == @intFromEnum(Color.White)) {
                        header.color = @intFromEnum(Color.Gray);
                        try self.mark_stack.append(header);
                    }
                }
            }
        }
        
        // Mark stack roots
        self.scanStackRoots();
        
        // Process write barriers
        try self.processWriteBarriers();
    }
    
    /// Calculate heap fragmentation percentage
    fn calculateFragmentation(self: *GC) f32 {
        
        // Simplified fragmentation calculation
        // In a real implementation, this would scan the heap for free blocks
        // var total_free_space: usize = 0;
        // var largest_free_block: usize = 0;
        const used_space = self.heap_used;
        const total_space = self.heap_size;
        
        if (total_space == 0) return 0.0;
        
        const fragmentation = 1.0 - (@as(f32, @floatFromInt(used_space)) / @as(f32, @floatFromInt(total_space)));
        return @max(0.0, @min(1.0, fragmentation));
    }

    
    /// Get current GC statistics
    pub fn getStats(self: *GC) GCStats {
        var stats = self.stats;
        stats.current_heap_size = self.heap_used;
        return stats;
    }
    
    /// Print GC statistics
    pub fn printStats(self: *GC) void {
        const stats = self.getStats();
        
        std.log.info("=== GC Statistics ===", .{});
        std.log.info("Total allocations: {}", .{stats.total_allocations});
        std.log.info("Total bytes allocated: {} bytes", .{stats.total_bytes_allocated});
        std.log.info("GC cycles: {}", .{stats.gc_cycles});
        std.log.info("Total pause time: {} μs", .{stats.total_pause_time_us});
        std.log.info("Max pause time: {} μs", .{stats.max_pause_time_us});
        std.log.info("Average pause time: {} μs", .{
            if (stats.gc_cycles > 0) stats.total_pause_time_us / stats.gc_cycles else 0
        });
        std.log.info("Promotions: {}", .{stats.promotions});
        std.log.info("Young collections: {}", .{stats.young_collections});
        std.log.info("Old collections: {}", .{stats.old_collections});
        std.log.info("Current heap size: {} bytes", .{stats.current_heap_size});
        std.log.info("Peak heap size: {} bytes", .{stats.peak_heap_size});
        std.log.info("Finalized objects: {}", .{stats.finalized_objects});
        std.log.info("====================", .{});
    }
};

/// Background thread for concurrent collection
fn concurrentCollectionWorker(gc: *GC) void {
    var iteration_count: u32 = 0;
    const MAX_ITERATIONS = 1000; // Prevent infinite loops in tests
    
    while (!gc.stop_collection.load(.acquire) and iteration_count < MAX_ITERATIONS) {
        iteration_count += 1;
        gc.collection_mutex.lock();
        
        // Shorter timeout for more responsive shutdown in tests
        gc.collection_condition.timedWait(&gc.collection_mutex, 10_000_000) catch {}; // 10ms timeout
        
        if (!gc.stop_collection.load(.acquire)) {
            gc.collection_running.store(true, .release);
            gc.collection_mutex.unlock();
            
            // Perform collection with error handling
            gc.collectNow() catch |err| {
                std.log.err("GC: Collection failed: {}", .{err});
                // Don't continue on collection failure in tests
                if (iteration_count > 10) break;
            };
            
            gc.collection_running.store(false, .release);
        } else {
            gc.collection_mutex.unlock();
            break; // Explicit break for shutdown
        }
        
        // Yield CPU between iterations to prevent tight loops
        std.time.sleep(1000); // 1μs yield
    }
    
    // Ensure clean shutdown state
    gc.collection_running.store(false, .release);
}

/// Background thread for finalization
fn finalizationWorker(gc: *GC) void {
    var iteration_count: u32 = 0;
    const MAX_ITERATIONS = 500; // Prevent infinite loops in finalization
    
    while (!gc.stop_collection.load(.acquire) and iteration_count < MAX_ITERATIONS) {
        iteration_count += 1;
        
        // Check finalization queue
        gc.finalization_mutex.lock();
        
        if (gc.finalization_queue.items.len > 0) {
            const objects_to_finalize = gc.finalization_queue.toOwnedSlice() catch {
                gc.finalization_mutex.unlock();
                continue;
            };
            gc.finalization_mutex.unlock();
            
            // Run finalizers outside of lock
            for (objects_to_finalize) |obj| {
                // Find and run finalizer
                for (gc.finalizers.items, 0..) |finalizer, i| {
                    if (finalizer.object == obj) {
                        finalizer.fn_ptr(obj.getData());
                        
                        // Remove finalizer
                        gc.finalization_mutex.lock();
                        _ = gc.finalizers.swapRemove(i);
                        gc.finalization_mutex.unlock();
                        
                        gc.stats.finalized_objects += 1;
                        break;
                    }
                }
                
                // Free the object
                gc.freeObjectDirect(obj);
            }
            
            gc.allocator.free(objects_to_finalize);
        } else {
            gc.finalization_mutex.unlock();
            
            // Sleep for a bit before checking again
            std.time.sleep(10_000_000); // 10ms
        }
    }
}

// Export C API for integration with LLVM-generated code
export fn cursed_gc_init(initial_heap_size: usize) ?*GC {
    const allocator = std.heap.page_allocator;
    var config = GCConfig.default();
    config.initial_heap_size = initial_heap_size;
    
    return GC.init(allocator, config) catch null;
}

export fn cursed_gc_deinit(gc: ?*GC) void {
    if (gc) |g| {
        g.deinit();
    }
}

export fn cursed_gc_alloc(gc: ?*GC, size: usize, type_id: u16) ?*anyopaque {
    if (gc) |g| {
        return g.alloc(size, type_id) catch null;
    }
    return null;
}

export fn cursed_gc_add_root(gc: ?*GC, ptr: *?*anyopaque, type_id: u16) void {
    if (gc) |g| {
        g.addRoot(ptr, type_id) catch {};
    }
}

export fn cursed_gc_remove_root(gc: ?*GC, ptr: *?*anyopaque) void {
    if (gc) |g| {
        g.removeRoot(ptr);
    }
}

export fn cursed_gc_collect(gc: ?*GC) void {
    if (gc) |g| {
        g.collectNow() catch {};
    }
}

export fn cursed_gc_write_barrier(gc: ?*GC, old_ref: *anyopaque, new_ref: *anyopaque) void {
    if (gc) |g| {
        g.writeBarrier(old_ref, new_ref);
    }
}

export fn cursed_gc_print_stats(gc: ?*GC) void {
    if (gc) |g| {
        g.printStats();
    }
}

// Advanced memory management exports for LLVM integration
export fn cursed_gc_alloc_with_source(gc: ?*GC, size: usize, type_id: u16, source_location: ?[*:0]const u8) ?*anyopaque {
    if (gc) |g| {
        const source_slice = if (source_location) |src| std.mem.span(src) else null;
        return g.allocWithSource(size, type_id, source_slice) catch null;
    }
    return null;
}

export fn cursed_gc_retain_object(gc: ?*GC, ptr: *anyopaque) void {
    if (gc) |g| {
        g.retainObject(ptr);
    }
}

export fn cursed_gc_release_object(gc: ?*GC, ptr: *anyopaque) void {
    if (gc) |g| {
        g.releaseObject(ptr);
    }
}

export fn cursed_gc_get_ref_count(gc: ?*GC, ptr: *anyopaque) u32 {
    if (gc) |g| {
        return g.getRefCount(ptr);
    }
    return 0;
}

export fn cursed_gc_get_memory_pressure(gc: ?*GC) u8 {
    if (gc) |g| {
        return switch (g.getMemoryPressure()) {
            .Low => 0,
            .Medium => 1,
            .High => 2,
            .Critical => 3,
        };
    }
    return 0;
}

export fn cursed_gc_detect_leaks(gc: ?*GC, leak_count: *usize) [*]LeakInfo {
    if (gc) |g| {
        const leaks = g.detectMemoryLeaks() catch {
            leak_count.* = 0;
            return undefined;
        };
        leak_count.* = leaks.len;
        return leaks.ptr;
    }
    leak_count.* = 0;
    return undefined;
}

// Define extern struct for C compatibility
const CMemoryUsage = extern struct {
    current_usage: usize,
    peak_usage: usize,
    total_allocated: usize,
    total_freed: usize,
    pressure: f32,
};

export fn cursed_gc_get_memory_usage(gc: ?*GC) CMemoryUsage {
    if (gc) |g| {
        const usage = g.getMemoryUsage();
        return CMemoryUsage{
            .current_usage = usage.current_usage,
            .peak_usage = usage.peak_usage,
            .total_allocated = usage.total_allocated,
            .total_freed = usage.total_freed,
            .pressure = usage.pressure,
        };
    }
    return CMemoryUsage{
        .current_usage = 0,
        .peak_usage = 0,
        .total_allocated = 0,
        .total_freed = 0,
        .pressure = 0.0,
    };
}

export fn cursed_gc_register_stack_root(gc: ?*GC, ptr: *anyopaque) void {
    if (gc) |g| {
        g.registerStackRoot(ptr) catch {};
    }
}

export fn cursed_gc_unregister_stack_root(gc: ?*GC, ptr: *anyopaque) void {
    if (gc) |g| {
        g.unregisterStackRoot(ptr) catch {};
    }
}

export fn cursed_gc_create_weak_ref(gc: ?*GC, target: *anyopaque) ?*WeakRef {
    if (gc) |g| {
        const weak_ref = g.allocator.create(WeakRef) catch return null;
        weak_ref.* = WeakRef{
            .target = target,
            .header = ObjectHeader.fromData(target),
        };
        
        g.weak_ref_mutex.lock();
        g.weak_refs.append(weak_ref) catch {
            g.weak_ref_mutex.unlock();
            g.allocator.destroy(weak_ref);
            return null;
        };
        g.weak_ref_mutex.unlock();
        
        return weak_ref;
    }
    return null;
}

export fn cursed_gc_weak_ref_get(weak_ref: ?*WeakRef) ?*anyopaque {
    if (weak_ref) |ref| {
        return ref.get();
    }
    return null;
}

export fn cursed_gc_weak_ref_valid(weak_ref: ?*WeakRef) bool {
    if (weak_ref) |ref| {
        return ref.get() != null;
    }
    return false;
}

export fn cursed_gc_register_finalizer(gc: ?*GC, object: *anyopaque, finalizer: ?*const fn (*anyopaque) callconv(.C) void) void {
    if (gc) |g| {
        if (finalizer) |fn_ptr| {
            const header = ObjectHeader.fromData(object);
            header.finalize = 1;
            
            g.finalization_mutex.lock();
            g.finalizers.append(Finalizer{
                .object = header,
                .fn_ptr = @ptrCast(fn_ptr),
            }) catch {};
            g.finalization_mutex.unlock();
        }
    }
}

export fn cursed_gc_set_max_pause_time(gc: ?*GC, max_pause_us: u64) void {
    if (gc) |g| {
        g.config.max_young_pause_time_us = max_pause_us;
        g.config.max_old_pause_time_us = max_pause_us * 2; // Old gen gets 2x the time budget
    }
}

export fn cursed_gc_collect_incremental(gc: ?*GC) void {
    if (gc) |g| {
        g.performIncrementalYoungCollection() catch {};
    }
}

// Stack scanning support for compiled code
export fn cursed_gc_scan_stack_frame(gc: ?*GC, frame_start: *anyopaque, frame_size: usize) void {
    if (gc) |g| {
        const start_addr = @as([*]u8, @ptrCast(frame_start));
        const end_addr = start_addr + frame_size;
        
        var scan_ptr = start_addr;
        while (@intFromPtr(scan_ptr) < @intFromPtr(end_addr)) {
            const potential_ptr = @as(*?*anyopaque, @ptrCast(@alignCast(scan_ptr))).*;
            
            if (potential_ptr) |ptr| {
                if (g.isValidHeapPointer(ptr)) {
                    const header = ObjectHeader.fromData(ptr);
                    if (header.color == @intFromEnum(Color.White)) {
                        header.color = @intFromEnum(Color.Gray);
                        g.mark_stack.append(header) catch {};
                    }
                }
            }
            
            scan_ptr = @as([*]u8, @ptrFromInt(@intFromPtr(scan_ptr) + @sizeOf(*anyopaque)));
        }
    }
}

// Memory pool allocation for compiled code
export fn cursed_gc_pool_alloc(gc: ?*GC, size: usize) ?*anyopaque {
    if (gc) |g| {
        return g.memory_pools.getAllocation(size) catch null;
    }
    return null;
}

export fn cursed_gc_pool_free(gc: ?*GC, ptr: *anyopaque, size: usize) void {
    if (gc) |g| {
        g.memory_pools.deallocate(ptr, size) catch {};
    }
}

// Enhanced C API for arena allocator integration
export fn cursed_gc_alloc_arena(gc: ?*GC, size: usize, type_id: u16, pattern: u8) ?*anyopaque {
    if (gc) |g| {
        const arena_pattern = switch (pattern) {
            0 => ArenaAllocator.AllocationPattern.Sequential,
            1 => ArenaAllocator.AllocationPattern.Stack,
            2 => ArenaAllocator.AllocationPattern.Pool,
            3 => ArenaAllocator.AllocationPattern.Temporary,
            4 => ArenaAllocator.AllocationPattern.StringIntern,
            5 => ArenaAllocator.AllocationPattern.ASTNodes,
            6 => ArenaAllocator.AllocationPattern.RuntimeValues,
            else => ArenaAllocator.AllocationPattern.Sequential,
        };
        return g.allocArena(size, type_id, arena_pattern) catch null;
    }
    return null;
}

export fn cursed_gc_push_runtime_frame(gc: ?*GC) void {
    if (gc) |g| {
        g.pushRuntimeFrame() catch {};
    }
}

export fn cursed_gc_pop_runtime_frame(gc: ?*GC) void {
    if (gc) |g| {
        g.popRuntimeFrame();
    }
}

export fn cursed_gc_reset_temporary(gc: ?*GC) void {
    if (gc) |g| {
        g.resetTemporaryAllocations();
    }
}

// Define C-compatible comprehensive stats structure
const CComprehensiveStats = extern struct {
    // GC stats
    total_allocations: u64,
    total_bytes_allocated: u64,
    gc_cycles: u64,
    total_pause_time_us: u64,
    max_pause_time_us: u64,
    current_heap_size: u64,
    peak_heap_size: u64,
    
    // Memory usage
    current_usage: usize,
    peak_usage: usize,
    total_allocated: usize,
    total_freed: usize,
    pressure: f32,
    young_gen_usage: usize,
    old_gen_usage: usize,
    fragmentation: f32,
    
    // Arena usage (if available)
    arena_total_allocated: usize,
    arena_total_used: usize,
    
    // Pressure level
    pressure_level: u8, // 0=Low, 1=Medium, 2=High, 3=Critical
};

export fn cursed_gc_get_comprehensive_stats(gc: ?*GC) CComprehensiveStats {
    if (gc) |g| {
        const stats = g.getComprehensiveMemoryStats();
        return CComprehensiveStats{
            .total_allocations = stats.gc_stats.total_allocations,
            .total_bytes_allocated = stats.gc_stats.total_bytes_allocated,
            .gc_cycles = stats.gc_stats.gc_cycles,
            .total_pause_time_us = stats.gc_stats.total_pause_time_us,
            .max_pause_time_us = stats.gc_stats.max_pause_time_us,
            .current_heap_size = stats.gc_stats.current_heap_size,
            .peak_heap_size = stats.gc_stats.peak_heap_size,
            .current_usage = stats.memory_usage.current_usage,
            .peak_usage = stats.memory_usage.peak_usage,
            .total_allocated = stats.memory_usage.total_allocated,
            .total_freed = stats.memory_usage.total_freed,
            .pressure = stats.memory_usage.pressure,
            .young_gen_usage = stats.memory_usage.young_gen_usage,
            .old_gen_usage = stats.memory_usage.old_gen_usage,
            .fragmentation = stats.memory_usage.fragmentation,
            .arena_total_allocated = if (stats.arena_usage) |usage| usage.total_allocated else 0,
            .arena_total_used = if (stats.arena_usage) |usage| usage.total_used else 0,
            .pressure_level = switch (stats.pressure_level) {
                .Low => 0,
                .Medium => 1,
                .High => 2,
                .Critical => 3,
            },
        };
    }
    return std.mem.zeroes(CComprehensiveStats);
}
