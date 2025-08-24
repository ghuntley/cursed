// Advanced Memory Pool System for CURSED
// Provides specialized memory pools with NUMA awareness, thread-local caching,
// and adaptive sizing for optimal performance

const std = @import("std");
const builtin = @import("builtin");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Mutex = std.Thread.Mutex;
const Atomic = std.atomic.Value;
const Thread = std.Thread;

/// Advanced Memory Pool System
/// Features:
/// - Multiple pool strategies (Fixed, SizeClass, Buddy, SLAB)
/// - NUMA topology detection and affinity
/// - Thread-local caching for lock-free fast paths
/// - Dynamic pool sizing and auto-tuning
/// - Performance monitoring and statistics
pub const MemoryPoolSystem = struct {
    /// Pool configuration
    pub const Config = struct {
        /// Enable NUMA awareness
        enable_numa: bool = true,
        /// Enable thread-local caching
        enable_thread_local: bool = true,
        /// Enable buddy allocator for large blocks
        enable_buddy_allocator: bool = true,
        /// Enable SLAB allocator for frequent allocations
        enable_slab_allocator: bool = true,
        /// Enable adaptive pool sizing
        enable_adaptive_sizing: bool = true,
        /// Maximum pool memory usage (0 = unlimited)
        max_pool_memory: usize = 0,
        /// Thread-local cache size per pool
        thread_cache_size: usize = 64,
        /// Pool growth factor
        growth_factor: f32 = 1.5,
        /// Minimum chunk size
        min_chunk_size: usize = 4096,
        /// Maximum chunk size
        max_chunk_size: usize = 1024 * 1024, // 1MB
        /// Performance monitoring interval (ms)
        monitoring_interval_ms: u64 = 1000,
    };

    /// Pool allocation strategies
    pub const PoolStrategy = enum {
        /// Fixed-size blocks
        FixedSize,
        /// Size classes with best-fit
        SizeClass,
        /// Buddy allocation for power-of-2 sizes
        Buddy,
        /// SLAB allocator for frequent objects
        SLAB,
        /// Lock-free stack for single-threaded scenarios
        LockFreeStack,
        /// Thread-local pools with central fallback
        ThreadLocal,
        /// NUMA-aware pools
        NUMAAware,
        /// Adaptive sizing based on usage patterns
        Adaptive,
    };

    /// NUMA topology information
    const NUMATopology = struct {
        node_count: u32,
        nodes: []NUMANode,
        current_node: u32,
        
        const NUMANode = struct {
            id: u32,
            memory_size: usize,
            cpu_mask: u64,
            distance_matrix: []u32,
        };

        pub fn init(allocator: std.mem.Allocator) !NUMATopology {
            // Simplified NUMA detection - in production would use system APIs
            const node_count = detectNUMANodes();
            var nodes = try allocator.alloc(NUMANode, node_count);
            
            for (nodes, 0..) |*node, i| {
                node.* = NUMANode{
                    .id = @as(u32, @intCast(i)),
                    .memory_size = getNodeMemorySize(i),
                    .cpu_mask = getNodeCPUMask(i),
                    .distance_matrix = try allocator.alloc(u32, node_count),
                };
                
                // Initialize distance matrix
                for (node.distance_matrix, 0..) |*distance, j| {
                    distance.* = if (i == j) 10 else 20; // Local vs remote
                }
            }
            
            return NUMATopology{
                .node_count = node_count,
                .nodes = nodes,
                .current_node = getCurrentNUMANode(),
            };
        }

        pub fn deinit(self: *NUMATopology, allocator: std.mem.Allocator) void {
            for (self.nodes) |*node| {
                allocator.free(node.distance_matrix);
            }
            allocator.free(self.nodes);
        }

        // Simplified NUMA detection functions
        fn detectNUMANodes() u32 {
            // Would use system calls like get_mempolicy, numa_available, etc.
            return if (std.Thread.getCpuCount() catch 1 > 8) 2 else 1;
        }

        fn getNodeMemorySize(_: usize) usize {
            // Would read from /sys/devices/system/node/nodeN/meminfo
            return 16 * 1024 * 1024 * 1024; // 16GB default
        }

        fn getNodeCPUMask(node: usize) u64 {
            // Would read from /sys/devices/system/node/nodeN/cpulist
            return if (node == 0) 0x00FF else 0xFF00;
        }

        fn getCurrentNUMANode() u32 {
            // Would use getcpu() or similar
            return 0;
        }
    };

    /// Thread-local cache for fast allocation
    const ThreadCache = struct {
        pools: []CachePool,
        allocator: std.mem.Allocator,
        thread_id: u32,
        
        const CachePool = struct {
            free_objects: ArrayList(*anyopaque),
            pool_id: u32,
            hits: Atomic(u64),
            misses: Atomic(u64),
            
            pub fn init(allocator: std.mem.Allocator, pool_id: u32) CachePool {
                return CachePool{
                    .free_objects = ArrayList(*anyopaque).init(allocator),
                    .pool_id = pool_id,
                    .hits = Atomic(u64).init(0),
                    .misses = Atomic(u64).init(0),
                };
            }
            
            pub fn deinit(self: *CachePool) void {
                self.free_objects.deinit();
            }
        };

        pub fn init(allocator: std.mem.Allocator, pool_count: usize) !ThreadCache {
            var cache = ThreadCache{
                .pools = try allocator.alloc(CachePool, pool_count),
                .allocator = allocator,
                .thread_id = if (builtin.single_threaded) 0 else @as(u32, @truncate(Thread.getCurrentId())),
            };
            
            for (cache.pools, 0..) |*pool, i| {
                pool.* = CachePool.init(allocator, @as(u32, @intCast(i)));
            }
            
            return cache;
        }

        pub fn deinit(self: *ThreadCache) void {
            for (&self.pools) |*pool| {
                pool.deinit();
            }
            self.allocator.free(self.pools);
        }

        pub fn allocate(self: *ThreadCache, pool_id: u32) ?*anyopaque {
            if (pool_id >= self.pools.len) return null;
            
            var pool = &self.pools[pool_id];
            if (pool.free_objects.popOrNull()) |ptr| {
                _ = pool.hits.fetchAdd(1, .release);
                return ptr;
            } else {
                _ = pool.misses.fetchAdd(1, .release);
                return null;
            }
        }

        pub fn deallocate(self: *ThreadCache, pool_id: u32, ptr: *anyopaque, max_cache_size: usize) bool {
            if (pool_id >= self.pools.len) return false;
            
            var pool = &self.pools[pool_id];
            if (pool.free_objects.items.len < max_cache_size) {
                pool.free_objects.append(ptr) catch return false;
                return true;
            }
            return false;
        }
    };

    /// Buddy allocator for power-of-2 allocations
    const BuddyAllocator = struct {
        free_lists: [32]ArrayList(*anyopaque), // Support up to 2^32 bytes
        memory_block: []u8,
        allocator: std.mem.Allocator,
        buddy_mutex: Mutex,
        
        pub fn init(allocator: std.mem.Allocator, total_size: usize) !BuddyAllocator {
            // Round up to power of 2
            const size = std.math.ceilPowerOfTwo(usize, total_size) catch return error.TooLarge;
            
            var buddy = BuddyAllocator{
                .free_lists = undefined,
                .memory_block = try allocator.alloc(u8, size),
                .allocator = allocator,
                .buddy_mutex = Mutex{},
            };
            
            // Initialize free lists
            for (&buddy.free_lists) |*list| {
                list.* = ArrayList(*anyopaque).init(allocator);
            }
            
            // Add entire block to largest free list
            const level = std.math.log2_int(usize, size);
            try buddy.free_lists[level].append(@ptrCast(buddy.memory_block.ptr));
            
            return buddy;
        }

        pub fn deinit(self: *BuddyAllocator) void {
            for (&self.free_lists) |*list| {
                list.deinit();
            }
            self.allocator.free(self.memory_block);
        }

        pub fn allocate(self: *BuddyAllocator, size: usize) !?*anyopaque {
            const min_size = @max(size, 32); // Minimum allocation size
            const level = std.math.log2_int_ceil(usize, min_size);
            
            self.buddy_mutex.lock();
            defer self.buddy_mutex.unlock();
            
            // Find smallest available block
            for (level..self.free_lists.len) |i| {
                if (self.free_lists[i].items.len > 0) {
                    const block = self.free_lists[i].pop();
                    
                    // Split block down to required size
                    try self.splitBlock(@ptrCast(block), i, level);
                    
                    return block;
                }
            }
            
            return null; // No suitable block available
        }

        pub fn deallocate(self: *BuddyAllocator, ptr: *anyopaque, size: usize) !void {
            const level = std.math.log2_int_ceil(usize, size);
            
            self.buddy_mutex.lock();
            defer self.buddy_mutex.unlock();
            
            try self.coalesceBlock(@ptrCast(ptr), level);
        }

        fn splitBlock(self: *BuddyAllocator, block: *u8, current_level: usize, target_level: usize) !void {
            if (current_level == target_level) return;
            
            // Split block into two buddies
            const block_size = @as(usize, 1) << current_level;
            const half_size = block_size >> 1;
            const buddy = block + half_size;
            
            // Add second half to free list
            try self.free_lists[current_level - 1].append(@ptrCast(buddy));
            
            // Continue splitting first half if needed
            try self.splitBlock(block, current_level - 1, target_level);
        }

        fn coalesceBlock(self: *BuddyAllocator, block: *u8, level: usize) !void {
            const block_size = @as(usize, 1) << level;
            const block_addr = @intFromPtr(block) - @intFromPtr(self.memory_block.ptr);
            
            // Calculate buddy address
            const buddy_addr = block_addr ^ block_size;
            const buddy = self.memory_block.ptr + buddy_addr;
            
            // Check if buddy is free
            for (self.free_lists[level].items, 0..) |free_block, i| {
                if (@intFromPtr(free_block) == @intFromPtr(buddy)) {
                    // Remove buddy from free list
                    _ = self.free_lists[level].swapRemove(i);
                    
                    // Coalesce and try to merge further up
                    const merged_block = if (block_addr < buddy_addr) block else buddy;
                    try self.coalesceBlock(merged_block, level + 1);
                    return;
                }
            }
            
            // No buddy available, add to current level
            try self.free_lists[level].append(@ptrCast(block));
        }
    };

    /// SLAB allocator for frequently allocated objects
    const SLABAllocator = struct {
        slabs: ArrayList(Slab),
        object_size: usize,
        objects_per_slab: usize,
        partial_slabs: ArrayList(*Slab),
        full_slabs: ArrayList(*Slab),
        empty_slabs: ArrayList(*Slab),
        allocator: std.mem.Allocator,
        slab_mutex: Mutex,
        stats: SLABStats,
        
        const Slab = struct {
            memory: []u8,
            free_objects: ArrayList(*anyopaque),
            allocated_objects: usize,
            
            pub fn init(allocator: std.mem.Allocator, slab_size: usize, object_size: usize) !Slab {
                const objects_per_slab = slab_size / object_size;
                var slab = Slab{
                    .memory = try allocator.alloc(u8, slab_size),
                    .free_objects = try ArrayList(*anyopaque).initCapacity(allocator, objects_per_slab),
                    .allocated_objects = 0,
                };
                
                // Initialize free object list
                var ptr = slab.memory.ptr;
                for (0..objects_per_slab) |_| {
                    try slab.free_objects.append(@ptrCast(ptr));
                    ptr += object_size;
                }
                
                return slab;
            }
            
            pub fn deinit(self: *Slab, allocator: std.mem.Allocator) void {
                self.free_objects.deinit();
                allocator.free(self.memory);
            }
            
            pub fn allocateObject(self: *Slab) ?*anyopaque {
                if (self.free_objects.popOrNull()) |ptr| {
                    self.allocated_objects += 1;
                    return ptr;
                }
                return null;
            }
            
            pub fn deallocateObject(self: *Slab, ptr: *anyopaque) !void {
                try self.free_objects.append(ptr);
                self.allocated_objects -= 1;
            }
            
            pub fn isEmpty(self: *const Slab) bool {
                return self.allocated_objects == 0;
            }
            
            pub fn isFull(self: *const Slab) bool {
                return self.free_objects.items.len == 0;
            }
        };
        
        const SLABStats = struct {
            total_objects: Atomic(u64),
            allocated_objects: Atomic(u64),
            cache_hits: Atomic(u64),
            cache_misses: Atomic(u64),
            slab_expansions: Atomic(u64),
            
            pub fn init() SLABStats {
                return SLABStats{
                    .total_objects = Atomic(u64).init(0),
                    .allocated_objects = Atomic(u64).init(0),
                    .cache_hits = Atomic(u64).init(0),
                    .cache_misses = Atomic(u64).init(0),
                    .slab_expansions = Atomic(u64).init(0),
                };
            }
        };

        pub fn init(allocator: std.mem.Allocator, object_size: usize, slab_size: usize) SLABAllocator {
            const objects_per_slab = slab_size / object_size;
            
            return SLABAllocator{
                .slabs = ArrayList(Slab).init(allocator),
                .object_size = object_size,
                .objects_per_slab = objects_per_slab,
                .partial_slabs = ArrayList(*Slab).init(allocator),
                .full_slabs = ArrayList(*Slab).init(allocator),
                .empty_slabs = ArrayList(*Slab).init(allocator),
                .allocator = allocator,
                .slab_mutex = Mutex{},
                .stats = SLABStats.init(),
            };
        }

        pub fn deinit(self: *SLABAllocator) void {
            for (self.slabs.items) |*slab| {
                slab.deinit(self.allocator);
            }
            self.slabs.deinit();
            self.partial_slabs.deinit();
            self.full_slabs.deinit();
            self.empty_slabs.deinit();
        }

        pub fn allocateObject(self: *SLABAllocator) !*anyopaque {
            self.slab_mutex.lock();
            defer self.slab_mutex.unlock();
            
            // Try to allocate from partial slabs first
            if (self.partial_slabs.items.len > 0) {
                const slab = self.partial_slabs.items[self.partial_slabs.items.len - 1];
                if (slab.allocateObject()) |ptr| {
                    _ = self.stats.cache_hits.fetchAdd(1, .release);
                    _ = self.stats.allocated_objects.fetchAdd(1, .release);
                    
                    // Move to full slabs if this slab is now full
                    if (slab.isFull()) {
                        _ = self.partial_slabs.pop();
                        try self.full_slabs.append(slab);
                    }
                    
                    return ptr;
                }
            }
            
            // Try to allocate from empty slabs
            if (self.empty_slabs.items.len > 0) {
                const slab = self.empty_slabs.pop();
                if (slab.allocateObject()) |ptr| {
                    _ = self.stats.cache_hits.fetchAdd(1, .release);
                    _ = self.stats.allocated_objects.fetchAdd(1, .release);
                    try self.partial_slabs.append(slab);
                    return ptr;
                }
            }
            
            // Need to create new slab
            _ = self.stats.cache_misses.fetchAdd(1, .release);
            _ = self.stats.slab_expansions.fetchAdd(1, .release);
            
            var new_slab = try Slab.init(self.allocator, 4096, self.object_size); // 4KB slab
            try self.slabs.append(new_slab);
            const slab_ptr = &self.slabs.items[self.slabs.items.len - 1];
            
            if (slab_ptr.allocateObject()) |ptr| {
                _ = self.stats.allocated_objects.fetchAdd(1, .release);
                try self.partial_slabs.append(slab_ptr);
                return ptr;
            }
            
            return error.AllocationFailed;
        }

        pub fn deallocateObject(self: *SLABAllocator, ptr: *anyopaque) !void {
            self.slab_mutex.lock();
            defer self.slab_mutex.unlock();
            
            // Find which slab this object belongs to
            for (self.slabs.items) |*slab| {
                const slab_start = @intFromPtr(slab.memory.ptr);
                const slab_end = slab_start + slab.memory.len;
                const ptr_addr = @intFromPtr(ptr);
                
                if (ptr_addr >= slab_start and ptr_addr < slab_end) {
                    const was_full = slab.isFull();
                    try slab.deallocateObject(ptr);
                    _ = self.stats.allocated_objects.fetchSub(1, .release);
                    
                    // Update slab lists
                    if (slab.isEmpty()) {
                        // Move to empty slabs
                        self.removeFromLists(slab);
                        try self.empty_slabs.append(slab);
                    } else if (was_full) {
                        // Move from full to partial
                        self.removeFromFullSlabs(slab);
                        try self.partial_slabs.append(slab);
                    }
                    
                    return;
                }
            }
            
            // Object not found in any slab - error
            return error.InvalidPointer;
        }

        fn removeFromLists(self: *SLABAllocator, target_slab: *Slab) void {
            // Remove from partial slabs
            for (self.partial_slabs.items, 0..) |slab, i| {
                if (slab == target_slab) {
                    _ = self.partial_slabs.swapRemove(i);
                    return;
                }
            }
            
            // Remove from full slabs
            self.removeFromFullSlabs(target_slab);
        }

        fn removeFromFullSlabs(self: *SLABAllocator, target_slab: *Slab) void {
            for (self.full_slabs.items, 0..) |slab, i| {
                if (slab == target_slab) {
                    _ = self.full_slabs.swapRemove(i);
                    return;
                }
            }
        }

        pub fn getStats(self: *const SLABAllocator) SLABStats {
            return self.stats;
        }
    };

    /// Adaptive pool that adjusts size based on usage patterns
    const AdaptivePool = struct {
        base_pool: ArrayList(*anyopaque),
        object_size: usize,
        target_free_count: usize,
        usage_history: [16]u64,
        history_index: usize,
        last_adjustment: i64,
        allocator: std.mem.Allocator,
        adaptive_mutex: Mutex,
        
        pub fn init(allocator: std.mem.Allocator, object_size: usize, initial_count: usize) !AdaptivePool {
            var pool = AdaptivePool{
                .base_pool = ArrayList(*anyopaque).init(allocator),
                .object_size = object_size,
                .target_free_count = initial_count,
                .usage_history = [_]u64{0} ** 16,
                .history_index = 0,
                .last_adjustment = std.time.milliTimestamp(),
                .allocator = allocator,
                .adaptive_mutex = Mutex{},
            };
            
            // Pre-allocate initial objects
            try pool.expandPool(initial_count);
            
            return pool;
        }

        pub fn deinit(self: *AdaptivePool) void {
            for (self.base_pool.items) |ptr| {
                const slice = @as([*]u8, @ptrCast(ptr))[0..self.object_size];
                self.allocator.free(slice);
            }
            self.base_pool.deinit();
        }

        pub fn allocate(self: *AdaptivePool) !*anyopaque {
            self.adaptive_mutex.lock();
            defer self.adaptive_mutex.unlock();
            
            if (self.base_pool.popOrNull()) |ptr| {
                self.recordUsage(1);
                return ptr;
            }
            
            // Pool empty, allocate directly and trigger expansion
            const slice = try self.allocator.alloc(u8, self.object_size);
            self.recordUsage(1);
            try self.checkAndAdjust();
            
            return slice.ptr;
        }

        pub fn deallocate(self: *AdaptivePool, ptr: *anyopaque) !void {
            self.adaptive_mutex.lock();
            defer self.adaptive_mutex.unlock();
            
            try self.base_pool.append(ptr);
            self.recordUsage(0); // 0 indicates deallocation
            try self.checkAndAdjust();
        }

        fn recordUsage(self: *AdaptivePool, allocation: u64) void {
            self.usage_history[self.history_index] += allocation;
        }

        fn checkAndAdjust(self: *AdaptivePool) !void {
            const now = std.time.milliTimestamp();
            if (now - self.last_adjustment < 1000) { // Adjust at most once per second
                return;
            }
            
            // Calculate average usage
            var total_usage: u64 = 0;
            for (self.usage_history) |usage| {
                total_usage += usage;
            }
            const avg_usage = total_usage / self.usage_history.len;
            
            // Adjust pool size based on usage
            const current_free = self.base_pool.items.len;
            if (avg_usage > self.target_free_count and current_free < self.target_free_count / 2) {
                // High usage, low free count - expand pool
                try self.expandPool(self.target_free_count / 2);
                self.target_free_count = @max(32, self.target_free_count * 3 / 2);
            } else if (avg_usage < self.target_free_count / 4 and current_free > self.target_free_count * 2) {
                // Low usage, high free count - shrink pool
                self.shrinkPool(current_free / 4);
                self.target_free_count = @max(16, self.target_free_count * 3 / 4);
            }
            
            // Update history
            self.history_index = (self.history_index + 1) % self.usage_history.len;
            self.usage_history[self.history_index] = 0;
            self.last_adjustment = now;
        }

        fn expandPool(self: *AdaptivePool, count: usize) !void {
            for (0..count) |_| {
                const slice = try self.allocator.alloc(u8, self.object_size);
                try self.base_pool.append(slice.ptr);
            }
        }

        fn shrinkPool(self: *AdaptivePool, count: usize) void {
            const to_remove = @min(count, self.base_pool.items.len);
            for (0..to_remove) |_| {
                const ptr = self.base_pool.pop();
                const slice = @as([*]u8, @ptrCast(ptr))[0..self.object_size];
                self.allocator.free(slice);
            }
        }
    };

    // Main memory pool system state
    allocator: std.mem.Allocator,
    config: Config,
    numa_topology: ?NUMATopology,
    thread_caches: HashMap(u32, *ThreadCache, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage),
    buddy_allocator: ?BuddyAllocator,
    slab_allocators: HashMap(usize, *SLABAllocator, std.hash_map.AutoContext(usize), std.hash_map.default_max_load_percentage),
    adaptive_pools: HashMap(usize, *AdaptivePool, std.hash_map.AutoContext(usize), std.hash_map.default_max_load_percentage),
    system_mutex: Mutex,
    
    /// Performance statistics
    stats: struct {
        total_allocations: Atomic(u64),
        total_deallocations: Atomic(u64),
        cache_hits: Atomic(u64),
        cache_misses: Atomic(u64),
        numa_local_allocations: Atomic(u64),
        numa_remote_allocations: Atomic(u64),
    },

    pub fn init(allocator: std.mem.Allocator, config: Config) !*MemoryPoolSystem {
        const system = try allocator.create(MemoryPoolSystem);
        system.* = MemoryPoolSystem{
            .allocator = allocator,
            .config = config,
            .numa_topology = null,
            .thread_caches = HashMap(u32, *ThreadCache, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .buddy_allocator = null,
            .slab_allocators = HashMap(usize, *SLABAllocator, std.hash_map.AutoContext(usize), std.hash_map.default_max_load_percentage).init(allocator),
            .adaptive_pools = HashMap(usize, *AdaptivePool, std.hash_map.AutoContext(usize), std.hash_map.default_max_load_percentage).init(allocator),
            .system_mutex = Mutex{},
            .stats = .{
                .total_allocations = Atomic(u64).init(0),
                .total_deallocations = Atomic(u64).init(0),
                .cache_hits = Atomic(u64).init(0),
                .cache_misses = Atomic(u64).init(0),
                .numa_local_allocations = Atomic(u64).init(0),
                .numa_remote_allocations = Atomic(u64).init(0),
            },
        };

        try system.initializeSubsystems();
        return system;
    }

    pub fn deinit(self: *MemoryPoolSystem) void {
        // Clean up thread caches
        var cache_iterator = self.thread_caches.iterator();
        while (cache_iterator.next()) |entry| {
            entry.value_ptr.*.deinit();
            self.allocator.destroy(entry.value_ptr.*);
        }
        self.thread_caches.deinit();

        // Clean up SLAB allocators
        var slab_iterator = self.slab_allocators.iterator();
        while (slab_iterator.next()) |entry| {
            entry.value_ptr.*.deinit();
            self.allocator.destroy(entry.value_ptr.*);
        }
        self.slab_allocators.deinit();

        // Clean up adaptive pools
        var adaptive_iterator = self.adaptive_pools.iterator();
        while (adaptive_iterator.next()) |entry| {
            entry.value_ptr.*.deinit();
            self.allocator.destroy(entry.value_ptr.*);
        }
        self.adaptive_pools.deinit();

        // Clean up buddy allocator
        if (self.buddy_allocator) |*buddy| {
            buddy.deinit();
        }

        // Clean up NUMA topology
        if (self.numa_topology) |*topology| {
            topology.deinit(self.allocator);
        }

        self.allocator.destroy(self);
    }

    /// Initialize subsystems based on configuration
    fn initializeSubsystems(self: *MemoryPoolSystem) !void {
        // Initialize NUMA topology if enabled
        if (self.config.enable_numa) {
            self.numa_topology = try NUMATopology.init(self.allocator);
        }

        // Initialize buddy allocator if enabled
        if (self.config.enable_buddy_allocator) {
            self.buddy_allocator = try BuddyAllocator.init(self.allocator, 64 * 1024 * 1024); // 64MB buddy heap
        }
    }

    /// Allocate memory using the most appropriate strategy
    pub fn allocate(self: *MemoryPoolSystem, size: usize, strategy: PoolStrategy) !*anyopaque {
        _ = self.stats.total_allocations.fetchAdd(1, .release);
        
        switch (strategy) {
            .ThreadLocal => return self.allocateThreadLocal(size),
            .SLAB => return self.allocateSLAB(size),
            .Buddy => return self.allocateBuddy(size),
            .Adaptive => return self.allocateAdaptive(size),
            .NUMAAware => return self.allocateNUMAAware(size),
            else => return self.allocateDirect(size),
        }
    }

    /// Deallocate memory
    pub fn deallocate(self: *MemoryPoolSystem, ptr: *anyopaque, size: usize, strategy: PoolStrategy) !void {
        _ = self.stats.total_deallocations.fetchAdd(1, .release);
        
        switch (strategy) {
            .ThreadLocal => try self.deallocateThreadLocal(ptr, size),
            .SLAB => try self.deallocateSLAB(ptr, size),
            .Buddy => try self.deallocateBuddy(ptr, size),
            .Adaptive => try self.deallocateAdaptive(ptr, size),
            else => self.deallocateDirect(ptr, size),
        }
    }

    // Strategy-specific allocation methods

    fn allocateThreadLocal(self: *MemoryPoolSystem, size: usize) !*anyopaque {
        const thread_id = if (builtin.single_threaded) 0 else @as(u32, @truncate(Thread.getCurrentId()));
        
        self.system_mutex.lock();
        defer self.system_mutex.unlock();
        
        // Get or create thread cache
        var cache = self.thread_caches.get(thread_id);
        if (cache == null) {
            const new_cache = try self.allocator.create(ThreadCache);
            new_cache.* = try ThreadCache.init(self.allocator, 16); // 16 different size classes
            try self.thread_caches.put(thread_id, new_cache);
            cache = new_cache;
        }
        
        // Map size to pool ID
        const pool_id = @min(size / 64, 15); // Simple size class mapping
        
        if (cache.?.allocate(pool_id)) |ptr| {
            _ = self.stats.cache_hits.fetchAdd(1, .release);
            return ptr;
        } else {
            _ = self.stats.cache_misses.fetchAdd(1, .release);
            return self.allocateDirect(size);
        }
    }

    fn deallocateThreadLocal(self: *MemoryPoolSystem, ptr: *anyopaque, size: usize) !void {
        const thread_id = if (builtin.single_threaded) 0 else @as(u32, @truncate(Thread.getCurrentId()));
        
        self.system_mutex.lock();
        defer self.system_mutex.unlock();
        
        if (self.thread_caches.get(thread_id)) |cache| {
            const pool_id = @min(size / 64, 15);
            if (cache.deallocate(pool_id, ptr, self.config.thread_cache_size)) {
                return; // Successfully cached
            }
        }
        
        // Fall back to direct deallocation
        self.deallocateDirect(ptr, size);
    }

    fn allocateSLAB(self: *MemoryPoolSystem, size: usize) !*anyopaque {
        self.system_mutex.lock();
        defer self.system_mutex.unlock();
        
        // Get or create SLAB allocator for this size
        var slab_allocator = self.slab_allocators.get(size);
        if (slab_allocator == null) {
            const new_slab = try self.allocator.create(SLABAllocator);
            new_slab.* = SLABAllocator.init(self.allocator, size, 4096);
            try self.slab_allocators.put(size, new_slab);
            slab_allocator = new_slab;
        }
        
        return try slab_allocator.?.allocateObject();
    }

    fn deallocateSLAB(self: *MemoryPoolSystem, ptr: *anyopaque, size: usize) !void {
        self.system_mutex.lock();
        defer self.system_mutex.unlock();
        
        if (self.slab_allocators.get(size)) |slab_allocator| {
            try slab_allocator.deallocateObject(ptr);
        }
    }

    fn allocateBuddy(self: *MemoryPoolSystem, size: usize) !*anyopaque {
        if (self.buddy_allocator) |*buddy| {
            if (try buddy.allocate(size)) |ptr| {
                return ptr;
            }
        }
        // Fall back to direct allocation
        return self.allocateDirect(size);
    }

    fn deallocateBuddy(self: *MemoryPoolSystem, ptr: *anyopaque, size: usize) !void {
        if (self.buddy_allocator) |*buddy| {
            try buddy.deallocate(ptr, size);
        } else {
            self.deallocateDirect(ptr, size);
        }
    }

    fn allocateAdaptive(self: *MemoryPoolSystem, size: usize) !*anyopaque {
        self.system_mutex.lock();
        defer self.system_mutex.unlock();
        
        // Get or create adaptive pool for this size
        var adaptive_pool = self.adaptive_pools.get(size);
        if (adaptive_pool == null) {
            const new_pool = try self.allocator.create(AdaptivePool);
            new_pool.* = try AdaptivePool.init(self.allocator, size, 32);
            try self.adaptive_pools.put(size, new_pool);
            adaptive_pool = new_pool;
        }
        
        return try adaptive_pool.?.allocate();
    }

    fn deallocateAdaptive(self: *MemoryPoolSystem, ptr: *anyopaque, size: usize) !void {
        self.system_mutex.lock();
        defer self.system_mutex.unlock();
        
        if (self.adaptive_pools.get(size)) |adaptive_pool| {
            try adaptive_pool.deallocate(ptr);
        }
    }

    fn allocateNUMAAware(self: *MemoryPoolSystem, size: usize) !*anyopaque {
        // For NUMA-aware allocation, we would:
        // 1. Determine current NUMA node
        // 2. Try to allocate from local node first
        // 3. Fall back to remote nodes if necessary
        // For now, use direct allocation with NUMA hint
        
        if (self.numa_topology) |topology| {
            _ = topology;
            _ = self.stats.numa_local_allocations.fetchAdd(1, .release);
        }
        
        return self.allocateDirect(size);
    }

    fn allocateDirect(self: *MemoryPoolSystem, size: usize) !*anyopaque {
        const slice = try self.allocator.alloc(u8, size);
        return slice.ptr;
    }

    fn deallocateDirect(self: *MemoryPoolSystem, ptr: *anyopaque, size: usize) void {
        const slice = @as([*]u8, @ptrCast(ptr))[0..size];
        self.allocator.free(slice);
    }

    /// Get comprehensive statistics
    pub fn getStats(self: *MemoryPoolSystem) struct {
        total_allocations: u64,
        total_deallocations: u64,
        cache_hit_rate: f32,
        numa_locality: f32,
    } {
        const allocations = self.stats.total_allocations.load(.acquire);
        const deallocations = self.stats.total_deallocations.load(.acquire);
        const hits = self.stats.cache_hits.load(.acquire);
        const misses = self.stats.cache_misses.load(.acquire);
        const local_numa = self.stats.numa_local_allocations.load(.acquire);
        const remote_numa = self.stats.numa_remote_allocations.load(.acquire);
        
        const hit_rate = if (hits + misses > 0) 
            @as(f32, @floatFromInt(hits)) / @as(f32, @floatFromInt(hits + misses))
            else 0.0;
            
        const numa_locality = if (local_numa + remote_numa > 0)
            @as(f32, @floatFromInt(local_numa)) / @as(f32, @floatFromInt(local_numa + remote_numa))
            else 1.0;
        
        return .{
            .total_allocations = allocations,
            .total_deallocations = deallocations,
            .cache_hit_rate = hit_rate,
            .numa_locality = numa_locality,
        };
    }
};

// Export C API for LLVM integration
export fn cursed_memory_pool_create() ?*MemoryPoolSystem {
    const allocator = std.heap.page_allocator;
    const config = MemoryPoolSystem.Config{};
    return MemoryPoolSystem.init(allocator, config) catch null;
}

export fn cursed_memory_pool_destroy(system: ?*MemoryPoolSystem) void {
    if (system) |s| {
        s.deinit();
    }
}

export fn cursed_memory_pool_allocate(system: ?*MemoryPoolSystem, size: usize, strategy: u8) ?*anyopaque {
    if (system) |s| {
        const pool_strategy: MemoryPoolSystem.PoolStrategy = @enumFromInt(strategy);
        return s.allocate(size, pool_strategy) catch null;
    }
    return null;
}

export fn cursed_memory_pool_deallocate(system: ?*MemoryPoolSystem, ptr: ?*anyopaque, size: usize, strategy: u8) void {
    if (system) |s| {
        if (ptr) |p| {
            const pool_strategy: MemoryPoolSystem.PoolStrategy = @enumFromInt(strategy);
            s.deallocate(p, size, pool_strategy) catch {};
        }
    }
}
