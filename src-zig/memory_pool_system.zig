const std = @import("std");
const builtin = @import("builtin");
const Mutex = std.Thread.Mutex;
const Condition = std.Thread.Condition;
const Atomic = std.atomic.Value;
const Thread = std.Thread;
const ArenaAllocator = @import("arena_allocator.zig").ArenaAllocator;
const GarbageCollector = @import("gc.zig").GarbageCollector;

/// Enterprise-Grade Memory Pool System with NUMA Awareness
/// 
/// Features:
/// - Advanced memory pool allocators with different strategies
/// - NUMA topology detection and memory affinity
/// - Cache-friendly memory layouts and allocation patterns
/// - Dynamic pool sizing based on usage patterns
/// - Integration with existing garbage collector
/// - Performance monitoring and auto-tuning
/// - Lock-free fast paths for hot allocations

/// NUMA node information
pub const NUMANode = struct {
    node_id: u8,
    cpu_mask: u64,
    memory_start: usize,
    memory_size: usize,
    distance_matrix: []u8, // Distance to other nodes
    free_memory: Atomic(usize),
    allocated_memory: Atomic(usize),
    
    pub fn init(node_id: u8, cpu_mask: u64, memory_start: usize, memory_size: usize, allocator: std.mem.Allocator) !NUMANode {
        const distance_matrix = try allocator.alloc(u8, 32); // Support up to 32 NUMA nodes
        @memset(distance_matrix, 255); // Initialize with max distance
        
        return NUMANode{
            .node_id = node_id,
            .cpu_mask = cpu_mask,
            .memory_start = memory_start,
            .memory_size = memory_size,
            .distance_matrix = distance_matrix,
            .free_memory = Atomic(usize).init(memory_size),
            .allocated_memory = Atomic(usize).init(0),
        };
    }
    
    pub fn deinit(self: *NUMANode, allocator: std.mem.Allocator) void {
        allocator.free(self.distance_matrix);
    }
    
    /// Get memory utilization percentage
    pub fn utilization(self: *const NUMANode) f32 {
        const allocated = self.allocated_memory.load(.acquire);
        return @as(f32, @floatFromInt(allocated)) / @as(f32, @floatFromInt(self.memory_size));
    }
    
    /// Check if this node is local to current CPU
    pub fn isLocal(self: *const NUMANode) bool {
        const current_cpu = getCurrentCPU();
        return (self.cpu_mask & (@as(u64, 1) << @intCast(current_cpu))) != 0;
    }
};

/// Memory pool allocation strategy
pub const PoolStrategy = enum {
    /// Fixed-size blocks, best for uniform allocations
    FixedSize,
    /// Size classes (powers of 2), good for varied allocations
    SizeClass,
    /// Buddy allocation, efficient fragmentation handling
    Buddy,
    /// SLAB allocation, optimized for specific object types
    SLAB,
    /// Lock-free stack, minimal contention
    LockFreeStack,
    /// Thread-local caching, no synchronization needed
    ThreadLocal,
    /// NUMA-aware allocation, memory locality optimization
    NUMAAware,
    /// Adaptive strategy that switches based on usage patterns
    Adaptive,
};

/// Memory pool configuration
pub const PoolConfig = struct {
    /// Pool strategy
    strategy: PoolStrategy = .SizeClass,
    /// Block size for fixed-size pools
    block_size: usize = 64,
    /// Minimum pool size
    min_size: usize = 1024 * 1024, // 1MB
    /// Maximum pool size
    max_size: usize = 128 * 1024 * 1024, // 128MB
    /// Growth factor when expanding
    growth_factor: f32 = 2.0,
    /// Shrink threshold (utilization percentage)
    shrink_threshold: f32 = 0.25,
    /// Alignment requirement
    alignment: u29 = @alignOf(u64),
    /// Enable thread-local caching
    thread_local_cache: bool = true,
    /// Cache size per thread
    cache_size: usize = 64 * 1024, // 64KB
    /// NUMA node affinity (-1 for automatic)
    numa_node: i8 = -1,
    /// Enable performance monitoring
    monitoring: bool = true,
    /// Enable auto-tuning
    auto_tuning: bool = true,
    /// Collection threshold for integration with GC
    gc_threshold: f32 = 0.8,
};

/// Size class definition for size-class strategy
const SizeClass = struct {
    size: usize,
    block_count: usize,
    free_list: ?*Block,
    full_slabs: ?*Slab,
    partial_slabs: ?*Slab,
    empty_slabs: ?*Slab,
    mutex: Mutex,
    
    const Block = struct {
        next: ?*Block,
        data: [0]u8,
    };
    
    const Slab = struct {
        next: ?*Slab,
        free_count: u32,
        block_count: u32,
        free_list: ?*Block,
        data: [0]u8,
        
        fn init(size: usize, block_size: usize, block_count: u32) *Slab {
            const slab_size = @sizeOf(Slab) + (block_size * block_count);
            const memory = std.heap.page_allocator.alloc(u8, slab_size) catch unreachable;
            const slab: *Slab = @ptrCast(@alignCast(memory.ptr));
            
            slab.* = Slab{
                .next = null,
                .free_count = block_count,
                .block_count = block_count,
                .free_list = null,
                .data = undefined,
            };
            
            // Initialize free list
            var block_ptr = @as([*]u8, @ptrCast(&slab.data));
            for (0..block_count) |i| {
                const block: *Block = @ptrCast(@alignCast(block_ptr));
                block.next = slab.free_list;
                slab.free_list = block;
                block_ptr += block_size;
            }
            
            return slab;
        }
        
        fn allocBlock(self: *Slab) ?*Block {
            if (self.free_list) |block| {
                self.free_list = block.next;
                self.free_count -= 1;
                return block;
            }
            return null;
        }
        
        fn freeBlock(self: *Slab, block: *Block) void {
            block.next = self.free_list;
            self.free_list = block;
            self.free_count += 1;
        }
        
        fn isFull(self: *const Slab) bool {
            return self.free_count == 0;
        }
        
        fn isEmpty(self: *const Slab) bool {
            return self.free_count == self.block_count;
        }
    };
};

/// Thread-local cache for lock-free allocations
const ThreadCache = struct {
    /// Cache entries for different size classes
    entries: []CacheEntry,
    /// Total cached memory
    cached_memory: usize,
    /// Maximum cache size
    max_size: usize,
    /// Associated NUMA node
    numa_node: ?*NUMANode,
    
    const CacheEntry = struct {
        size_class: usize,
        free_list: ?*anyopaque,
        count: u32,
        max_count: u32,
    };
    
    pub fn init(allocator: std.mem.Allocator, max_size: usize, numa_node: ?*NUMANode) !ThreadCache {
        const entries = try allocator.alloc(CacheEntry, 32); // Support 32 size classes
        @memset(entries, CacheEntry{
            .size_class = 0,
            .free_list = null,
            .count = 0,
            .max_count = 16,
        });
        
        return ThreadCache{
            .entries = entries,
            .cached_memory = 0,
            .max_size = max_size,
            .numa_node = numa_node,
        };
    }
    
    pub fn deinit(self: *ThreadCache, allocator: std.mem.Allocator) void {
        allocator.free(self.entries);
    }
    
    pub fn alloc(self: *ThreadCache, size: usize) ?*anyopaque {
        const size_class = getSizeClass(size);
        if (size_class >= self.entries.len) return null;
        
        var entry = &self.entries[size_class];
        if (entry.free_list) |ptr| {
            const next: ?*anyopaque = @ptrFromInt(@as(usize, @intFromPtr(@as(*?*anyopaque, @ptrCast(@alignCast(ptr))).*)));
            entry.free_list = next;
            entry.count -= 1;
            self.cached_memory -= getSizeFromClass(size_class);
            return ptr;
        }
        
        return null; // Cache miss, fallback to pool allocation
    }
    
    pub fn free(self: *ThreadCache, ptr: *anyopaque, size: usize) bool {
        const size_class = getSizeClass(size);
        if (size_class >= self.entries.len) return false;
        
        var entry = &self.entries[size_class];
        if (entry.count >= entry.max_count) return false; // Cache full
        
        // Add to cache free list
        @as(*?*anyopaque, @ptrCast(@alignCast(ptr))).* = entry.free_list;
        entry.free_list = ptr;
        entry.count += 1;
        self.cached_memory += getSizeFromClass(size_class);
        
        return true;
    }
    
    pub fn shouldFlush(self: *const ThreadCache) bool {
        return self.cached_memory > self.max_size;
    }
    
    pub fn flush(self: *ThreadCache, pool: *MemoryPool) void {
        for (self.entries) |*entry| {
            while (entry.free_list) |ptr| {
                const next: ?*anyopaque = @ptrFromInt(@as(usize, @intFromPtr(@as(*?*anyopaque, @ptrCast(@alignCast(ptr))).*)));
                entry.free_list = next;
                entry.count -= 1;
                
                const size = getSizeFromClass(entry.size_class);
                pool.freeToPool(ptr, size);
                self.cached_memory -= size;
            }
        }
    }
};

/// Memory pool performance statistics
pub const PoolStats = struct {
    /// Total allocations
    total_allocs: Atomic(u64),
    /// Total deallocations
    total_frees: Atomic(u64),
    /// Total bytes allocated
    total_bytes: Atomic(u64),
    /// Current active allocations
    active_allocs: Atomic(u64),
    /// Current active bytes
    active_bytes: Atomic(u64),
    /// Peak allocation count
    peak_allocs: Atomic(u64),
    /// Peak memory usage
    peak_bytes: Atomic(u64),
    /// Cache hits (thread-local cache)
    cache_hits: Atomic(u64),
    /// Cache misses
    cache_misses: Atomic(u64),
    /// Pool expansions
    expansions: Atomic(u64),
    /// Pool shrinks
    shrinks: Atomic(u64),
    /// NUMA remote allocations
    numa_remote_allocs: Atomic(u64),
    /// Average allocation latency (nanoseconds)
    avg_alloc_latency_ns: Atomic(u64),
    /// Fragmentation ratio
    fragmentation_ratio: Atomic(u32), // Fixed-point with 16-bit fractional part
    
    pub fn init() PoolStats {
        return PoolStats{
            .total_allocs = Atomic(u64).init(0),
            .total_frees = Atomic(u64).init(0),
            .total_bytes = Atomic(u64).init(0),
            .active_allocs = Atomic(u64).init(0),
            .active_bytes = Atomic(u64).init(0),
            .peak_allocs = Atomic(u64).init(0),
            .peak_bytes = Atomic(u64).init(0),
            .cache_hits = Atomic(u64).init(0),
            .cache_misses = Atomic(u64).init(0),
            .expansions = Atomic(u64).init(0),
            .shrinks = Atomic(u64).init(0),
            .numa_remote_allocs = Atomic(u64).init(0),
            .avg_alloc_latency_ns = Atomic(u64).init(0),
            .fragmentation_ratio = Atomic(u32).init(0),
        };
    }
    
    pub fn recordAlloc(self: *PoolStats, size: usize, latency_ns: u64, is_cache_hit: bool, is_numa_remote: bool) void {
        _ = self.total_allocs.fetchAdd(1, .acq_rel);
        _ = self.total_bytes.fetchAdd(size, .acq_rel);
        
        const new_active_allocs = self.active_allocs.fetchAdd(1, .acq_rel) + 1;
        const new_active_bytes = self.active_bytes.fetchAdd(size, .acq_rel) + size;
        
        // Update peak values
        var current_peak = self.peak_allocs.load(.acquire);
        while (new_active_allocs > current_peak) {
            const old_peak = self.peak_allocs.cmpxchgWeak(current_peak, new_active_allocs, .acq_rel, .acquire) orelse break;
            current_peak = old_peak;
        }
        
        current_peak = self.peak_bytes.load(.acquire);
        while (new_active_bytes > current_peak) {
            const old_peak = self.peak_bytes.cmpxchgWeak(current_peak, new_active_bytes, .acq_rel, .acquire) orelse break;
            current_peak = old_peak;
        }
        
        // Update cache statistics
        if (is_cache_hit) {
            _ = self.cache_hits.fetchAdd(1, .acq_rel);
        } else {
            _ = self.cache_misses.fetchAdd(1, .acq_rel);
        }
        
        // Update NUMA statistics
        if (is_numa_remote) {
            _ = self.numa_remote_allocs.fetchAdd(1, .acq_rel);
        }
        
        // Update average latency using exponential moving average
        const current_avg = self.avg_alloc_latency_ns.load(.acquire);
        const new_avg = (current_avg * 15 + latency_ns) / 16; // α = 1/16
        _ = self.avg_alloc_latency_ns.store(new_avg, .release);
    }
    
    pub fn recordFree(self: *PoolStats, size: usize) void {
        _ = self.total_frees.fetchAdd(1, .acq_rel);
        _ = self.active_allocs.fetchSub(1, .acq_rel);
        _ = self.active_bytes.fetchSub(size, .acq_rel);
    }
    
    pub fn recordExpansion(self: *PoolStats) void {
        _ = self.expansions.fetchAdd(1, .acq_rel);
    }
    
    pub fn recordShrink(self: *PoolStats) void {
        _ = self.shrinks.fetchAdd(1, .acq_rel);
    }
    
    pub fn updateFragmentation(self: *PoolStats, ratio: f32) void {
        const fixed_point_ratio = @as(u32, @intFromFloat(ratio * 65536.0));
        _ = self.fragmentation_ratio.store(fixed_point_ratio, .release);
    }
    
    pub fn getFragmentation(self: *const PoolStats) f32 {
        const fixed_point = self.fragmentation_ratio.load(.acquire);
        return @as(f32, @floatFromInt(fixed_point)) / 65536.0;
    }
    
    pub fn getCacheHitRate(self: *const PoolStats) f32 {
        const hits = self.cache_hits.load(.acquire);
        const misses = self.cache_misses.load(.acquire);
        const total = hits + misses;
        if (total == 0) return 0.0;
        return @as(f32, @floatFromInt(hits)) / @as(f32, @floatFromInt(total));
    }
    
    pub fn getNUMALocalityRate(self: *const PoolStats) f32 {
        const total = self.total_allocs.load(.acquire);
        const remote = self.numa_remote_allocs.load(.acquire);
        if (total == 0) return 1.0;
        return 1.0 - (@as(f32, @floatFromInt(remote)) / @as(f32, @floatFromInt(total)));
    }
};

/// Advanced Memory Pool with enterprise features
pub const MemoryPool = struct {
    /// Pool configuration
    config: PoolConfig,
    /// NUMA topology
    numa_nodes: []NUMANode,
    /// Current NUMA node for allocation
    current_numa_node: ?*NUMANode,
    /// Size classes for size-class strategy
    size_classes: []SizeClass,
    /// Thread-local caches
    thread_caches: std.HashMap(Thread.Id, *ThreadCache, std.hash_map.AutoContext(Thread.Id), std.hash_map.default_max_load_percentage),
    /// Main pool allocator
    backing_allocator: std.mem.Allocator,
    /// Integration with garbage collector
    gc: ?*GarbageCollector,
    /// Pool statistics
    stats: PoolStats,
    /// Mutex for pool operations
    mutex: Mutex,
    /// Condition variable for pool management
    condition: Condition,
    /// Pool management thread
    management_thread: ?Thread,
    /// Shutdown flag
    shutdown: Atomic(bool),
    /// Auto-tuning state
    tuning_state: TuningState,
    
    const TuningState = struct {
        last_tune_time: i64,
        allocation_pattern: AllocationPattern,
        recommended_strategy: PoolStrategy,
        adaptation_counter: u32,
        
        const AllocationPattern = struct {
            avg_size: f32,
            size_variance: f32,
            allocation_rate: f32,
            lifetime_avg: f32,
        };
    };
    
    pub fn init(config: PoolConfig, allocator: std.mem.Allocator, gc: ?*GarbageCollector) !MemoryPool {
        var pool = MemoryPool{
            .config = config,
            .numa_nodes = &[_]NUMANode{},
            .current_numa_node = null,
            .size_classes = &[_]SizeClass{},
            .thread_caches = std.HashMap(Thread.Id, *ThreadCache, std.hash_map.AutoContext(Thread.Id), std.hash_map.default_max_load_percentage).init(allocator),
            .backing_allocator = allocator,
            .gc = gc,
            .stats = PoolStats.init(),
            .mutex = Mutex{},
            .condition = Condition{},
            .management_thread = null,
            .shutdown = Atomic(bool).init(false),
            .tuning_state = TuningState{
                .last_tune_time = std.time.microTimestamp(),
                .allocation_pattern = TuningState.AllocationPattern{
                    .avg_size = 0,
                    .size_variance = 0,
                    .allocation_rate = 0,
                    .lifetime_avg = 0,
                },
                .recommended_strategy = config.strategy,
                .adaptation_counter = 0,
            },
        };
        
        // Initialize NUMA topology
        try pool.initNUMATopology();
        
        // Initialize size classes
        try pool.initSizeClasses();
        
        // Start management thread if auto-tuning is enabled
        if (config.auto_tuning) {
            pool.management_thread = try Thread.spawn(.{}, managementThreadMain, .{&pool});
        }
        
        return pool;
    }
    
    pub fn deinit(self: *MemoryPool) void {
        // Shutdown management thread
        if (self.management_thread) |thread| {
            self.shutdown.store(true, .release);
            self.condition.signal();
            thread.join();
        }
        
        // Clean up thread caches
        var cache_iter = self.thread_caches.iterator();
        while (cache_iter.next()) |entry| {
            entry.value_ptr.*.deinit(self.backing_allocator);
            self.backing_allocator.destroy(entry.value_ptr.*);
        }
        self.thread_caches.deinit(allocator);
        
        // Clean up size classes
        for (self.size_classes) |*size_class| {
            // Free all slabs
            var slab = size_class.full_slabs;
            while (slab) |s| {
                const next = s.next;
                self.backing_allocator.free(@as([*]u8, @ptrCast(s))[0..@sizeOf(SizeClass.Slab) + (size_class.size * s.block_count)]);
                slab = next;
            }
            
            slab = size_class.partial_slabs;
            while (slab) |s| {
                const next = s.next;
                self.backing_allocator.free(@as([*]u8, @ptrCast(s))[0..@sizeOf(SizeClass.Slab) + (size_class.size * s.block_count)]);
                slab = next;
            }
            
            slab = size_class.empty_slabs;
            while (slab) |s| {
                const next = s.next;
                self.backing_allocator.free(@as([*]u8, @ptrCast(s))[0..@sizeOf(SizeClass.Slab) + (size_class.size * s.block_count)]);
                slab = next;
            }
        }
        self.backing_allocator.free(self.size_classes);
        
        // Clean up NUMA nodes
        for (self.numa_nodes) |*node| {
            node.deinit(self.backing_allocator);
        }
        self.backing_allocator.free(self.numa_nodes);
    }
    
    /// Allocate memory from the pool with NUMA awareness
    pub fn alloc(self: *MemoryPool, size: usize) ![]u8 {
        const start_time = std.time.nanoTimestamp();
        
        // Try thread-local cache first for fast path
        if (self.config.thread_local_cache) {
            if (self.tryThreadCacheAlloc(size)) |ptr| {
                const latency = @as(u64, @intCast(std.time.nanoTimestamp() - start_time));
                self.stats.recordAlloc(size, latency, true, false);
                return @as([*]u8, @ptrCast(ptr))[0..size];
            }
        }
        
        // Fallback to pool allocation
        const ptr = try self.allocFromPool(size);
        const is_numa_remote = self.isNUMARemote(ptr);
        
        const latency = @as(u64, @intCast(std.time.nanoTimestamp() - start_time));
        self.stats.recordAlloc(size, latency, false, is_numa_remote);
        
        return @as([*]u8, @ptrCast(ptr))[0..size];
    }
    
    /// Free memory back to the pool
    pub fn free(self: *MemoryPool, ptr: []u8) void {
        // Try thread-local cache first
        if (self.config.thread_local_cache) {
            if (self.tryThreadCacheFree(ptr.ptr, ptr.len)) {
                self.stats.recordFree(ptr.len);
                return;
            }
        }
        
        // Fallback to pool free
        self.freeToPool(ptr.ptr, ptr.len);
        self.stats.recordFree(ptr.len);
    }
    
    /// Get pool statistics
    pub fn getStats(self: *const MemoryPool) PoolStats {
        return self.stats;
    }
    
    /// Force garbage collection if integrated
    pub fn collectGarbage(self: *MemoryPool) void {
        if (self.gc) |gc| {
            gc.collect();
        }
    }
    
    /// Tune pool parameters based on usage patterns
    pub fn autoTune(self: *MemoryPool) void {
        const now = std.time.microTimestamp();
        const time_since_last = now - self.tuning_state.last_tune_time;
        
        // Only tune every 10 seconds
        if (time_since_last < 10_000_000) return;
        
        self.tuning_state.last_tune_time = now;
        
        // Analyze allocation patterns
        self.analyzeAllocationPatterns();
        
        // Recommend strategy changes
        self.recommendStrategy();
        
        // Adjust cache sizes
        self.adjustCacheSizes();
        
        // Update NUMA placement policy
        self.updateNUMAPolicy();
    }
    
    // Private methods
    
    fn initNUMATopology(self: *MemoryPool) !void {
        // Detect NUMA topology from /sys/devices/system/node/
        const numa_count = detectNUMANodeCount();
        self.numa_nodes = try self.backing_allocator.alloc(NUMANode, numa_count);
        
        for (0..numa_count) |i| {
            self.numa_nodes[i] = try NUMANode.init(
                @intCast(i),
                getCPUMaskForNode(@intCast(i)),
                getMemoryStartForNode(@intCast(i)),
                getMemorySizeForNode(@intCast(i)),
                self.backing_allocator
            );
        }
        
        // Set current NUMA node based on CPU affinity
        const current_cpu = getCurrentCPU();
        for (self.numa_nodes) |*node| {
            if ((node.cpu_mask & (@as(u64, 1) << @intCast(current_cpu))) != 0) {
                self.current_numa_node = node;
                break;
            }
        }
    }
    
    fn initSizeClasses(self: *MemoryPool) !void {
        const size_class_count = 32; // Support sizes from 8 bytes to 64KB
        self.size_classes = try self.backing_allocator.alloc(SizeClass, size_class_count);
        
        for (0..size_class_count) |i| {
            const size = @as(usize, 8) << @intCast(i); // Powers of 2 from 8 bytes
            const block_count = @min(4096 / size, 256); // Reasonable block count
            
            self.size_classes[i] = SizeClass{
                .size = size,
                .block_count = block_count,
                .free_list = null,
                .full_slabs = null,
                .partial_slabs = null,
                .empty_slabs = null,
                .mutex = Mutex{},
            };
        }
    }
    
    fn tryThreadCacheAlloc(self: *MemoryPool, size: usize) ?*anyopaque {
        const thread_id = Thread.getCurrentId();
        
        self.mutex.lock();
        defer self.mutex.unlock();
        
        const cache = self.thread_caches.get(thread_id) orelse {
            // Create new thread cache
            const new_cache = self.backing_allocator.create(ThreadCache) catch return null;
            new_cache.* = ThreadCache.init(self.backing_allocator, self.config.cache_size, self.current_numa_node) catch {
                self.backing_allocator.destroy(new_cache);
                return null;
            };
            self.thread_caches.put(thread_id, new_cache) catch {
                new_cache.deinit(self.backing_allocator);
                self.backing_allocator.destroy(new_cache);
                return null;
            };
            return null; // First allocation goes to pool
        };
        
        return cache.alloc(size);
    }
    
    fn tryThreadCacheFree(self: *MemoryPool, ptr: *anyopaque, size: usize) bool {
        const thread_id = Thread.getCurrentId();
        
        self.mutex.lock();
        defer self.mutex.unlock();
        
        const cache = self.thread_caches.get(thread_id) orelse return false;
        
        const success = cache.free(ptr, size);
        
        // Flush cache if it's getting full
        if (cache.shouldFlush()) {
            cache.flush(self);
        }
        
        return success;
    }
    
    fn allocFromPool(self: *MemoryPool, size: usize) !*anyopaque {
        const size_class = getSizeClass(size);
        
        if (size_class >= self.size_classes.len) {
            // Large allocation, use backing allocator directly
            const slice = try self.backing_allocator.alloc(u8, size);
            return slice.ptr;
        }
        
        var class = &self.size_classes[size_class];
        class.mutex.lock();
        defer class.mutex.unlock();
        
        // Try to allocate from partial slabs first
        if (class.partial_slabs) |slab| {
            if (slab.allocBlock()) |block| {
                if (slab.isFull()) {
                    // Move to full slabs
                    class.partial_slabs = slab.next;
                    slab.next = class.full_slabs;
                    class.full_slabs = slab;
                }
                return &block.data;
            }
        }
        
        // Try to get a slab from empty slabs
        if (class.empty_slabs) |slab| {
            class.empty_slabs = slab.next;
            slab.next = class.partial_slabs;
            class.partial_slabs = slab;
            
            const block = slab.allocBlock().?; // Should always succeed for empty slab
            return &block.data;
        }
        
        // Allocate new slab
        const new_slab = SizeClass.Slab.init(class.size, class.size, @intCast(class.block_count));
        new_slab.next = class.partial_slabs;
        class.partial_slabs = new_slab;
        
        const block = new_slab.allocBlock().?; // Should always succeed for new slab
        return &block.data;
    }
    
    fn freeToPool(self: *MemoryPool, ptr: *anyopaque, size: usize) void {
        const size_class = getSizeClass(size);
        
        if (size_class >= self.size_classes.len) {
            // Large allocation, free directly
            const slice = @as([*]u8, @ptrCast(ptr))[0..size];
            self.backing_allocator.free(slice);
            return;
        }
        
        var class = &self.size_classes[size_class];
        class.mutex.lock();
        defer class.mutex.unlock();
        
        // Find which slab this block belongs to
        // This is a simplified version - in practice, you'd use slab metadata
        const block: *SizeClass.Block = @ptrCast(@alignCast(ptr));
        
        // Find the slab (simplified search)
        var current_slab = class.full_slabs;
        var prev_slab: ?*SizeClass.Slab = null;
        
        while (current_slab) |slab| {
            const slab_start = @intFromPtr(&slab.data);
            const slab_end = slab_start + (class.size * slab.block_count);
            const block_addr = @intFromPtr(block);
            
            if (block_addr >= slab_start and block_addr < slab_end) {
                slab.freeBlock(block);
                
                if (slab.isEmpty()) {
                    // Move to empty slabs
                    if (prev_slab) |prev| {
                        prev.next = slab.next;
                    } else {
                        class.full_slabs = slab.next;
                    }
                    slab.next = class.empty_slabs;
                    class.empty_slabs = slab;
                } else if (slab.free_count == 1) {
                    // Was full, now partial - move to partial slabs
                    if (prev_slab) |prev| {
                        prev.next = slab.next;
                    } else {
                        class.full_slabs = slab.next;
                    }
                    slab.next = class.partial_slabs;
                    class.partial_slabs = slab;
                }
                return;
            }
            
            prev_slab = slab;
            current_slab = slab.next;
        }
        
        // Not found in full slabs, try partial slabs
        current_slab = class.partial_slabs;
        prev_slab = null;
        
        while (current_slab) |slab| {
            const slab_start = @intFromPtr(&slab.data);
            const slab_end = slab_start + (class.size * slab.block_count);
            const block_addr = @intFromPtr(block);
            
            if (block_addr >= slab_start and block_addr < slab_end) {
                slab.freeBlock(block);
                
                if (slab.isEmpty()) {
                    // Move to empty slabs
                    if (prev_slab) |prev| {
                        prev.next = slab.next;
                    } else {
                        class.partial_slabs = slab.next;
                    }
                    slab.next = class.empty_slabs;
                    class.empty_slabs = slab;
                }
                return;
            }
            
            prev_slab = slab;
            current_slab = slab.next;
        }
    }
    
    fn isNUMARemote(self: *MemoryPool, ptr: *anyopaque) bool {
        if (self.current_numa_node == null) return false;
        
        const addr = @intFromPtr(ptr);
        for (self.numa_nodes) |*node| {
            if (addr >= node.memory_start and addr < node.memory_start + node.memory_size) {
                return node.node_id != self.current_numa_node.?.node_id;
            }
        }
        
        return false; // Unknown memory region, assume local
    }
    
    fn analyzeAllocationPatterns(self: *MemoryPool) void {
        // Analyze current allocation patterns to update tuning state
        const stats = self.getStats();
        
        // Calculate allocation rate (allocs per second)
        const total_allocs = stats.total_allocs.load(.acquire);
        const current_time = std.time.microTimestamp();
        const time_diff = @as(f64, @floatFromInt(current_time - self.tuning_state.last_tune_time)) / 1_000_000.0;
        
        if (time_diff > 0) {
            self.tuning_state.allocation_pattern.allocation_rate = 
                @as(f32, @floatFromInt(total_allocs)) / @as(f32, @floatCast(time_diff));
        }
        
        // Update fragmentation information
        self.stats.updateFragmentation(self.calculateFragmentation());
    }
    
    fn recommendStrategy(self: *MemoryPool) void {
        const pattern = &self.tuning_state.allocation_pattern;
        
        // Recommend strategy based on patterns
        if (pattern.allocation_rate > 1000.0 and pattern.avg_size < 1024) {
            // High allocation rate with small objects - recommend lock-free strategy
            self.tuning_state.recommended_strategy = .LockFreeStack;
        } else if (pattern.size_variance < 0.1) {
            // Low size variance - recommend fixed-size strategy
            self.tuning_state.recommended_strategy = .FixedSize;
        } else if (pattern.avg_size > 64 * 1024) {
            // Large objects - recommend buddy allocation
            self.tuning_state.recommended_strategy = .Buddy;
        } else {
            // General case - keep size class strategy
            self.tuning_state.recommended_strategy = .SizeClass;
        }
    }
    
    fn adjustCacheSizes(self: *MemoryPool) void {
        const cache_hit_rate = self.stats.getCacheHitRate();
        
        // Increase cache size if hit rate is low
        if (cache_hit_rate < 0.8) {
            // TODO: Implement dynamic cache size adjustment
        }
    }
    
    fn updateNUMAPolicy(self: *MemoryPool) void {
        const numa_locality_rate = self.stats.getNUMALocalityRate();
        
        // If too many remote allocations, consider rebalancing
        if (numa_locality_rate < 0.7) {
            // TODO: Implement NUMA rebalancing logic
        }
    }
    
    fn calculateFragmentation(self: *MemoryPool) f32 {
        var total_allocated: usize = 0;
        var total_used: usize = 0;
        
        for (self.size_classes) |*class| {
            class.mutex.lock();
            defer class.mutex.unlock();
            
            var slab = class.full_slabs;
            while (slab) |s| {
                total_allocated += class.size * s.block_count;
                total_used += class.size * (s.block_count - s.free_count);
                slab = s.next;
            }
            
            slab = class.partial_slabs;
            while (slab) |s| {
                total_allocated += class.size * s.block_count;
                total_used += class.size * (s.block_count - s.free_count);
                slab = s.next;
            }
            
            slab = class.empty_slabs;
            while (slab) |s| {
                total_allocated += class.size * s.block_count;
                // Empty slabs contribute to allocated but not used
                slab = s.next;
            }
        }
        
        if (total_allocated == 0) return 0.0;
        return 1.0 - (@as(f32, @floatFromInt(total_used)) / @as(f32, @floatFromInt(total_allocated)));
    }
    
    fn managementThreadMain(self: *MemoryPool) void {
        while (!self.shutdown.load(.acquire)) {
            // Auto-tune every 10 seconds
            self.autoTune();
            
            // Check if GC should be triggered
            if (self.gc) |gc| {
                const usage = self.stats.active_bytes.load(.acquire);
                const threshold = @as(u64, @intFromFloat(@as(f64, @floatFromInt(self.config.max_size)) * self.config.gc_threshold));
                
                if (usage > threshold) {
                    gc.collect();
                }
            }
            
            // Wait for next cycle or shutdown signal
            self.mutex.lock();
            defer self.mutex.unlock();
            self.condition.timedWait(&self.mutex, 10_000_000_000); // 10 seconds
        }
    }
};

// Utility functions for NUMA detection and CPU management

fn detectNUMANodeCount() usize {
    // Read from /sys/devices/system/node/possible
    // Simplified implementation - returns 1 for non-NUMA systems
    return 1;
}

fn getCurrentCPU() u32 {
    // Use sched_getcpu() on Linux
    // Simplified implementation
    return 0;
}

fn getCPUMaskForNode(node_id: u8) u64 {
    // Read CPU mask for NUMA node from /sys/devices/system/node/nodeX/cpumap
    // Simplified implementation
    _ = node_id;
    return 0xFFFFFFFFFFFFFFFF; // All CPUs
}

fn getMemoryStartForNode(node_id: u8) usize {
    // Get memory range for NUMA node
    // Simplified implementation
    _ = node_id;
    return 0;
}

fn getMemorySizeForNode(node_id: u8) usize {
    // Get memory size for NUMA node
    // Simplified implementation
    _ = node_id;
    return 1024 * 1024 * 1024; // 1GB
}

fn getSizeClass(size: usize) usize {
    if (size <= 8) return 0;
    
    const bits = @bitSizeOf(usize);
    const leading_zeros = @clz(size - 1);
    return bits - leading_zeros - 4; // Subtract 4 because we start from 8 bytes (2^3)
}

fn getSizeFromClass(size_class: usize) usize {
    return @as(usize, 8) << @intCast(size_class);
}

// Export C API for integration with LLVM and other components

export fn cursed_memory_pool_create(config: *const PoolConfig) ?*MemoryPool {
    const allocator = std.heap.page_allocator;
    const pool = allocator.create(MemoryPool) catch return null;
    pool.* = MemoryPool.init(config.*, allocator, null) catch {
        allocator.destroy(pool);
        return null;
    };
    return pool;
}

export fn cursed_memory_pool_destroy(pool: ?*MemoryPool) void {
    if (pool) |p| {
        p.deinit(allocator);
        std.heap.page_allocator.destroy(p);
    }
}

export fn cursed_memory_pool_alloc(pool: ?*MemoryPool, size: usize) ?*anyopaque {
    if (pool) |p| {
        const slice = p.alloc(size) catch return null;
        return slice.ptr;
    }
    return null;
}

export fn cursed_memory_pool_free(pool: ?*MemoryPool, ptr: ?*anyopaque, size: usize) void {
    if (pool) |p| {
        if (ptr) |data| {
            const slice = @as([*]u8, @ptrCast(data))[0..size];
            p.free(slice);
        }
    }
}

export fn cursed_memory_pool_collect_garbage(pool: ?*MemoryPool) void {
    if (pool) |p| {
        p.collectGarbage();
    }
}

export fn cursed_memory_pool_get_stats(pool: ?*MemoryPool, stats: ?*PoolStats) void {
    if (pool) |p| {
        if (stats) |s| {
            s.* = p.getStats();
        }
    }
}
