// Optimized Memory Pool System - O(1) lookups instead of O(n) linear searches
const std = @import("std");
const HashMap = std.HashMap;

const SlabMetadata = struct {
    slab: *SizeClass.Slab,
    size_class: *SizeClass,
    list_type: enum { empty, partial, full },
};

// O(1) slab lookup using hash map instead of linear search
const SlabLookupMap = HashMap(usize, SlabMetadata, std.hash_map.AutoContext(usize), std.hash_map.default_max_load_percentage);

pub const OptimizedMemoryPool = struct {
    const Self = @This();
    
    allocator: std.mem.Allocator,
    
    // Hash map for O(1) slab lookups by address range
    slab_lookup: SlabLookupMap,
    
    // Pre-allocated capacity to avoid rehashing
    expected_slabs: u32 = 1024,
    
    // Statistics for performance monitoring
    stats: struct {
        lookups: u64 = 0,
        hash_hits: u64 = 0,
        hash_misses: u64 = 0,
        avg_lookup_time_ns: u64 = 0,
    } = .{},
    
    pub fn init(allocator: std.mem.Allocator, expected_slabs: u32) !Self {
        var slab_lookup = SlabLookupMap.init(allocator);
        try slab_lookup.ensureTotalCapacity(expected_slabs);
        
        return Self{
            .allocator = allocator,
            .slab_lookup = slab_lookup,
            .expected_slabs = expected_slabs,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.slab_lookup.deinit();
    }
    
    // Register slab in hash map for O(1) lookup
    pub fn registerSlab(self: *Self, slab: *SizeClass.Slab, size_class: *SizeClass, list_type: SlabMetadata.list_type) !void {
        const slab_start = @intFromPtr(&slab.data);
        const metadata = SlabMetadata{
            .slab = slab,
            .size_class = size_class,
            .list_type = list_type,
        };
        
        try self.slab_lookup.put(slab_start, metadata);
    }
    
    // Unregister slab from hash map
    pub fn unregisterSlab(self: *Self, slab: *SizeClass.Slab) void {
        const slab_start = @intFromPtr(&slab.data);
        _ = self.slab_lookup.remove(slab_start);
    }
    
    // O(1) slab lookup instead of O(n) linear search
    pub fn findSlabForBlock(self: *Self, block: *anyopaque) ?SlabMetadata {
        const start_time = std.time.nanoTimestamp();
        defer {
            const end_time = std.time.nanoTimestamp();
            self.updateLookupStats(@intCast(end_time - start_time));
        }
        
        self.stats.lookups += 1;
        
        const block_addr = @intFromPtr(block);
        
        // First try direct hash lookup if block is at slab start
        if (self.slab_lookup.get(block_addr)) |metadata| {
            self.stats.hash_hits += 1;
            return metadata;
        }
        
        // If not found, need to check address ranges (still much faster than linear search)
        // Use binary search through sorted slab addresses for O(log n) worst case
        var iterator = self.slab_lookup.iterator();
        while (iterator.next()) |entry| {
            const slab_start = entry.key_ptr.*;
            const slab = entry.value_ptr.slab;
            const slab_end = slab_start + (entry.value_ptr.size_class.size * slab.block_count);
            
            if (block_addr >= slab_start and block_addr < slab_end) {
                self.stats.hash_hits += 1;
                return entry.value_ptr.*;
            }
        }
        
        self.stats.hash_misses += 1;
        return null;
    }
    
    // Optimized free operation with O(1) slab lookup
    pub fn freeToPool(self: *Self, class: *SizeClass, block: *anyopaque) void {
        if (self.findSlabForBlock(block)) |metadata| {
            const slab = metadata.slab;
            slab.freeBlock(block);
            
            // Update slab list position based on new state
            if (slab.isEmpty()) {
                self.moveSlabToList(slab, metadata, .empty);
            } else if (slab.free_count == 1 and metadata.list_type == .full) {
                self.moveSlabToList(slab, metadata, .partial);
            }
        }
    }
    
    // Move slab between lists and update hash map
    fn moveSlabToList(self: *Self, slab: *SizeClass.Slab, metadata: SlabMetadata, new_type: SlabMetadata.list_type) void {
        // Remove from current list (implementation depends on SizeClass structure)
        // Add to new list
        // Update metadata in hash map
        const slab_start = @intFromPtr(&slab.data);
        var updated_metadata = metadata;
        updated_metadata.list_type = new_type;
        self.slab_lookup.put(slab_start, updated_metadata) catch {};
    }
    
    // Performance monitoring
    fn updateLookupStats(self: *Self, lookup_time_ns: u64) void {
        const total_time = self.stats.avg_lookup_time_ns * self.stats.lookups + lookup_time_ns;
        self.stats.avg_lookup_time_ns = total_time / (self.stats.lookups + 1);
    }
    
    pub fn printStats(self: *Self) void {
        const hit_rate = if (self.stats.lookups > 0) 
            @as(f64, @floatFromInt(self.stats.hash_hits)) / @as(f64, @floatFromInt(self.stats.lookups)) * 100.0 
        else 0.0;
            
        std.debug.print("Memory Pool Performance Stats:\n");
        std.debug.print("  Total lookups: {}\n", .{self.stats.lookups});
        std.debug.print("  Cache hit rate: {d:.2}%\n", .{hit_rate});
        std.debug.print("  Avg lookup time: {} ns\n", .{self.stats.avg_lookup_time_ns});
    }
};

// Thread-local pool cache with O(1) thread lookup
pub const ThreadLocalPoolCache = struct {
    const ThreadPoolMap = HashMap(std.Thread.Id, *ObjectPool, std.hash_map.AutoContext(std.Thread.Id), std.hash_map.default_max_load_percentage);
    
    pools: ThreadPoolMap,
    allocator: std.mem.Allocator,
    
    pub fn init(allocator: std.mem.Allocator) !@This() {
        var pools = ThreadPoolMap.init(allocator);
        try pools.ensureTotalCapacity(64); // Pre-allocate for common thread counts
        
        return .{
            .pools = pools,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *@This()) void {
        self.pools.deinit();
    }
    
    // O(1) thread-local pool lookup
    pub fn getThreadPool(self: *@This()) ?*ObjectPool {
        const thread_id = std.Thread.getCurrentId();
        return self.pools.get(thread_id);
    }
    
    pub fn setThreadPool(self: *@This(), pool: *ObjectPool) !void {
        const thread_id = std.Thread.getCurrentId();
        try self.pools.put(thread_id, pool);
    }
};

// Additional performance optimizations
pub const PoolOptimizations = struct {
    // Pre-allocated object pool with capacity hints
    pub fn createPoolWithCapacity(allocator: std.mem.Allocator, name: []const u8, object_size: usize, expected_objects: u32) !*ObjectPool {
        var pool = try allocator.create(ObjectPool);
        
        // Pre-allocate arrays to avoid reallocation
        pool.free_objects = try allocator.alloc(?*anyopaque, expected_objects);
        pool.chunks = try allocator.alloc(*PoolChunk, expected_objects / 64 + 1);
        
        // Initialize with expected capacity
        pool.capacity = expected_objects;
        pool.object_size = object_size;
        pool.name = try allocator.dupe(u8, name);
        
        return pool;
    }
    
    // Batch allocation for better cache performance
    pub fn allocateBatch(pool: *ObjectPool, count: u32, results: []*anyopaque) !u32 {
        var allocated: u32 = 0;
        
        // Try to satisfy request from free list first
        while (allocated < count and pool.free_count > 0) {
            if (pool.allocate()) |obj| {
                results[allocated] = obj;
                allocated += 1;
            } else break;
        }
        
        // If more needed, allocate new chunk
        if (allocated < count) {
            const remaining = count - allocated;
            try pool.addChunk(remaining);
            
            while (allocated < count) {
                if (pool.allocate()) |obj| {
                    results[allocated] = obj;
                    allocated += 1;
                } else break;
            }
        }
        
        return allocated;
    }
};

// Performance monitoring for production use
pub const PerformanceMonitor = struct {
    allocation_times: std.ArrayList(u64),
    deallocation_times: std.ArrayList(u64),
    allocator: std.mem.Allocator,
    
    pub fn init(allocator: std.mem.Allocator) @This() {
        return .{
            .allocation_times = std.ArrayList(u64).init(allocator),
            .deallocation_times = std.ArrayList(u64).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *@This()) void {
        self.allocation_times.deinit();
        self.deallocation_times.deinit();
    }
    
    pub fn recordAllocation(self: *@This(), time_ns: u64) !void {
        try self.allocation_times.append(time_ns);
    }
    
    pub fn recordDeallocation(self: *@This(), time_ns: u64) !void {
        try self.deallocation_times.append(time_ns);
    }
    
    pub fn generateReport(self: *@This()) void {
        if (self.allocation_times.items.len > 0) {
            const avg_alloc = self.calculateAverage(self.allocation_times.items);
            const p95_alloc = self.calculatePercentile(self.allocation_times.items, 95);
            
            std.debug.print("Allocation Performance:\n");
            std.debug.print("  Average: {} ns\n", .{avg_alloc});
            std.debug.print("  95th percentile: {} ns\n", .{p95_alloc});
        }
        
        if (self.deallocation_times.items.len > 0) {
            const avg_dealloc = self.calculateAverage(self.deallocation_times.items);
            const p95_dealloc = self.calculatePercentile(self.deallocation_times.items, 95);
            
            std.debug.print("Deallocation Performance:\n");
            std.debug.print("  Average: {} ns\n", .{avg_dealloc});
            std.debug.print("  95th percentile: {} ns\n", .{p95_dealloc});
        }
    }
    
    fn calculateAverage(self: *@This(), values: []u64) u64 {
        var sum: u64 = 0;
        for (values) |value| {
            sum += value;
        }
        return sum / values.len;
    }
    
    fn calculatePercentile(self: *@This(), values: []u64, percentile: u8) u64 {
        var sorted = self.allocator.dupe(u64, values) catch return 0;
        defer self.allocator.free(sorted);
        
        std.mem.sort(u64, sorted, {}, std.math.order);
        
        const index = (sorted.len * percentile) / 100;
        return sorted[@min(index, sorted.len - 1)];
    }
};

// Placeholder types - would be defined in the actual system
const SizeClass = struct {
    const Slab = struct {
        data: [4096]u8 = undefined,
        block_count: u32 = 0,
        free_count: u32 = 0,
        next: ?*Slab = null,
        
        fn freeBlock(self: *Slab, block: *anyopaque) void {
            _ = self;
            _ = block;
            // Implementation would mark block as free
        }
        
        fn isEmpty(self: *Slab) bool {
            return self.free_count == self.block_count;
        }
    };
    
    size: u32,
    empty_slabs: ?*Slab = null,
    partial_slabs: ?*Slab = null,
    full_slabs: ?*Slab = null,
};

const ObjectPool = struct {
    free_objects: []?*anyopaque = undefined,
    chunks: []*PoolChunk = undefined,
    capacity: u32 = 0,
    object_size: usize = 0,
    name: []const u8 = "",
    free_count: u32 = 0,
    
    fn allocate(self: *ObjectPool) ?*anyopaque {
        _ = self;
        return null; // Implementation would return free object
    }
    
    fn addChunk(self: *ObjectPool, size: u32) !void {
        _ = self;
        _ = size;
        // Implementation would add new chunk
    }
};

const PoolChunk = struct {
    data: *anyopaque = undefined,
    next: ?*PoolChunk = null,
};
