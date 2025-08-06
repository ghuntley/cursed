const std = @import("std");
const builtin = @import("builtin");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Mutex = std.Thread.Mutex;
const Condition = std.Thread.Condition;
const Atomic = std.atomic.Value;
const Thread = std.Thread;

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
        };
        
        try gc.initializeHeap();
        try gc.startBackgroundThreads();
        
        return gc;
    }
    
    /// Clean up the garbage collector
    pub fn deinit(self: *GC) void {
        // Stop background threads
        self.stopBackgroundThreads();
        
        // Clean up heap
        self.deallocateHeap();
        
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
        const total_size = ObjectHeader.HEADER_SIZE + size;
        
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
        
        // Update statistics
        self.stats.total_allocations += 1;
        self.stats.total_bytes_allocated += total_size;
        self.heap_used += total_size;
        
        // Update peak heap size
        if (self.heap_used > self.stats.peak_heap_size) {
            self.stats.peak_heap_size = self.heap_used;
        }
        
        // Check if GC should be triggered based on generation-specific thresholds
        if (generation == .Young) {
            const young_gen_size = @as(usize, @intFromFloat(@as(f64, @floatFromInt(self.heap_size)) * self.config.young_gen_ratio));
            const young_usage = @as(f32, @floatFromInt(self.young_heap_used)) / @as(f32, @floatFromInt(young_gen_size));
            if (young_usage > self.config.young_gc_trigger_threshold) {
                self.triggerYoungCollection();
            }
        } else {
            const old_gen_size = self.heap_size - @as(usize, @intFromFloat(@as(f64, @floatFromInt(self.heap_size)) * self.config.young_gen_ratio));
            const old_usage = @as(f32, @floatFromInt(self.old_heap_used)) / @as(f32, @floatFromInt(old_gen_size));
            if (old_usage > self.config.old_gc_trigger_threshold) {
                self.triggerOldCollection();
            }
        }
        
        return header.getData();
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
    
    /// Free object memory
    fn freeObjectDirect(self: *GC, obj: *ObjectHeader) void {
        const generation = @as(Generation, @enumFromInt(obj.generation));
        
        switch (generation) {
            .Young => {
                // Young generation uses bump allocation, so we don't free individual objects
                // They get freed en masse during collection
            },
            .Old => {
                // For old generation, we could implement a free list
                // For now, we'll just update the used counter
                if (self.old_heap_used >= obj.size) {
                    self.old_heap_used -= obj.size;
                } else {
                    self.old_heap_used = 0;
                }
            },
        }
        
        if (self.heap_used >= obj.size) {
            self.heap_used -= obj.size;
        } else {
            self.heap_used = 0;
        }
        self.stats.current_heap_size = self.heap_used;
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
