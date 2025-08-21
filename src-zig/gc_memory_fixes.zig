//! CURSED Garbage Collector - Memory Management Fixes
//!
//! This module provides fixes for memory leaks, allocation failures, and cleanup issues
//! in the garbage collector. Features:
//! - Fixed heap initialization and cleanup
//! - Proper thread synchronization
//! - Memory leak prevention
//! - Safe pointer handling
//! - Robust error recovery

const std = @import("std");
const builtin = @import("builtin");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Mutex = std.Thread.Mutex;
const Condition = std.Thread.Condition;
const Atomic = std.atomic.Value;
const Thread = std.Thread;
const ArenaAllocator = std.heap.ArenaAllocator;

/// Import base GC types
const gc = @import("gc.zig");
const GCStats = gc.GCStats;
const GCConfig = gc.GCConfig;
const WeakRef = gc.WeakRef;
const FinalizerFn = gc.FinalizerFn;

/// Fixed GC object header with better alignment and safety
const FixedObjectHeader = struct {
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
    /// Magic number for corruption detection
    magic: u32,
    /// Next pointer for free list or mark stack
    next: ?*FixedObjectHeader,
    
    const HEADER_SIZE = @sizeOf(FixedObjectHeader);
    const MAGIC_VALUE: u32 = 0xDEADBEEF;
    
    /// Initialize header with magic value
    fn init(size: u32, type_id: u16) FixedObjectHeader {
        return FixedObjectHeader{
            .size = size,
            .type_id = type_id,
            .color = 0, // White
            .generation = 0, // Young
            .finalize = 0,
            .reserved = 0,
            .magic = MAGIC_VALUE,
            .next = null,
        };
    }
    
    /// Validate header integrity
    fn isValid(self: *const FixedObjectHeader) bool {
        return self.magic == MAGIC_VALUE and self.size >= HEADER_SIZE;
    }
    
    /// Get pointer to user data after header
    fn getData(self: *FixedObjectHeader) *anyopaque {
        std.debug.assert(self.isValid());
        const ptr = @as([*]u8, @ptrCast(self)) + HEADER_SIZE;
        return @ptrCast(ptr);
    }
    
    /// Get header from user data pointer with validation
    fn fromData(data: *anyopaque) !*FixedObjectHeader {
        const ptr = @as([*]u8, @ptrCast(data)) - HEADER_SIZE;
        const header = @as(*FixedObjectHeader, @ptrCast(@alignCast(ptr)));
        
        if (!header.isValid()) {
            return error.CorruptedHeader;
        }
        
        return header;
    }
};

/// Fixed memory region with bounds checking
const FixedMemoryRegion = struct {
    start: [*]u8,
    size: usize,
    used: usize,
    generation: u1,
    mutex: Mutex,
    
    fn init(start: [*]u8, size: usize, generation: u1) FixedMemoryRegion {
        return FixedMemoryRegion{
            .start = start,
            .size = size,
            .used = 0,
            .generation = generation,
            .mutex = Mutex{},
        };
    }
    
    fn allocate(self: *FixedMemoryRegion, size: usize) !*FixedObjectHeader {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        const aligned_size = std.mem.alignForward(usize, size, @alignOf(FixedObjectHeader));
        
        if (self.used + aligned_size > self.size) {
            return error.OutOfMemory;
        }
        
        const obj_ptr = @as(*FixedObjectHeader, @ptrCast(@alignCast(self.start + self.used)));
        self.used += aligned_size;
        
        return obj_ptr;
    }
    
    fn reset(self: *FixedMemoryRegion) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        self.used = 0;
    }
    
    fn contains(self: *const FixedMemoryRegion, ptr: *anyopaque) bool {
        const addr = @intFromPtr(ptr);
        const start_addr = @intFromPtr(self.start);
        const end_addr = start_addr + self.size;
        return addr >= start_addr and addr < end_addr;
    }
};

/// Fixed garbage collector with comprehensive memory safety
pub const FixedGC = struct {
    /// Configuration
    config: GCConfig,
    /// Statistics
    stats: GCStats,
    /// Main allocator for GC metadata
    allocator: std.mem.Allocator,
    /// Arena for temporary allocations
    arena: ArenaAllocator,
    arena_allocator: std.mem.Allocator,
    
    /// Memory regions
    young_region: FixedMemoryRegion,
    old_region: FixedMemoryRegion,
    heap_memory: []u8,
    
    /// Object tracking
    all_objects: ?*FixedObjectHeader,
    free_list: ?*FixedObjectHeader,
    mark_stack: ArrayList(*FixedObjectHeader),
    
    /// Root set management
    roots: ArrayList(*?*anyopaque),
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
    finalization_queue: ArrayList(*FixedObjectHeader),
    finalization_mutex: Mutex,
    finalization_thread: ?Thread,
    
    /// Weak references
    weak_refs: ArrayList(*WeakRef),
    weak_ref_mutex: Mutex,
    
    /// Performance monitoring
    last_gc_time: i64,
    
    /// Initialize the fixed garbage collector
    pub fn init(allocator: std.mem.Allocator, config: GCConfig) !*FixedGC {
        const gc_instance = try allocator.create(FixedGC);
        errdefer allocator.destroy(gc_instance);
        
        var arena = ArenaAllocator.init(allocator);
        errdefer arena.deinit();
        const arena_allocator = arena.allocator();
        
        // Allocate heap memory
        const heap_memory = try allocator.alloc(u8, config.initial_heap_size);
        errdefer allocator.free(heap_memory);
        
        // Set up memory regions
        const young_size = config.young_gen_size;
        const old_size = config.initial_heap_size - young_size;
        
        const young_region = FixedMemoryRegion.init(heap_memory.ptr, young_size, 0);
        const old_region = FixedMemoryRegion.init(heap_memory.ptr + young_size, old_size, 1);
        
        gc_instance.* = FixedGC{
            .config = config,
            .stats = GCStats.init(),
            .allocator = allocator,
            .arena = arena,
            .arena_allocator = arena_allocator,
            .young_region = young_region,
            .old_region = old_region,
            .heap_memory = heap_memory,
            .all_objects = null,
            .free_list = null,
            .mark_stack = .empty,
            .roots = .empty,
            .stack_roots = .empty,
            .roots_mutex = Mutex{},
            .collection_mutex = Mutex{},
            .collection_condition = Condition{},
            .collection_thread = null,
            .collection_running = Atomic(bool).init(false),
            .stop_collection = Atomic(bool).init(false),
            .write_barriers = .empty,
            .write_barrier_mutex = Mutex{},
            .finalizers = .empty,
            .finalization_queue = .empty,
            .finalization_mutex = Mutex{},
            .finalization_thread = null,
            .weak_refs = .empty,
            .weak_ref_mutex = Mutex{},
            .last_gc_time = std.time.milliTimestamp(),
        };
        
        try gc_instance.startBackgroundThreads();
        
        std.log.info("FixedGC: Initialized with {} bytes heap ({} young, {} old)", .{
            config.initial_heap_size, young_size, old_size
        });
        
        return gc_instance;
    }
    
    /// Clean up the garbage collector safely
    pub fn deinit(self: *FixedGC) void {
        // Stop background threads first
        self.stopBackgroundThreads();
        
        // Clean up weak references
        self.cleanupWeakReferences();
        
        // Run final finalization
        self.runFinalFinalization();
        
        // Clean up heap memory
        self.allocator.free(self.heap_memory);
        
        // Clean up arena
        self.arena.deinit();
        
        // Free the GC instance
        self.allocator.destroy(self);
        
        std.log.info("FixedGC: Cleanup completed");
    }
    
    /// Start background collection and finalization threads safely
    fn startBackgroundThreads(self: *FixedGC) !void {
        // Start concurrent collection thread
        self.collection_thread = Thread.spawn(.{}, concurrentCollectionWorker, .{self}) catch |err| {
            std.log.err("FixedGC: Failed to start collection thread: {}", .{err});
            return err;
        };
        
        // Start finalization thread if enabled
        if (self.config.enable_finalization) {
            self.finalization_thread = Thread.spawn(.{}, finalizationWorker, .{self}) catch |err| {
                std.log.err("FixedGC: Failed to start finalization thread: {}", .{err});
                // Stop collection thread if finalization fails
                self.stopCollectionThread();
                return err;
            };
        }
    }
    
    /// Stop background threads safely
    fn stopBackgroundThreads(self: *FixedGC) void {
        // Signal threads to stop
        self.stop_collection.store(true, .release);
        
        self.stopCollectionThread();
        self.stopFinalizationThread();
    }
    
    fn stopCollectionThread(self: *FixedGC) void {
        // Wake up collection thread
        self.collection_mutex.lock();
        self.collection_condition.signal();
        self.collection_mutex.unlock();
        
        // Wait for collection thread to finish
        if (self.collection_thread) |thread| {
            thread.join();
            self.collection_thread = null;
        }
    }
    
    fn stopFinalizationThread(self: *FixedGC) void {
        // Wait for finalization thread to finish
        if (self.finalization_thread) |thread| {
            thread.join();
            self.finalization_thread = null;
        }
    }
    
    /// Allocate object with comprehensive safety checks
    pub fn alloc(self: *FixedGC, size: usize, type_id: u16) !*anyopaque {
        if (size == 0) {
            return error.InvalidSize;
        }
        
        const total_size = FixedObjectHeader.HEADER_SIZE + size;
        
        // Choose generation based on allocation pattern
        const use_old = self.shouldAllocateInOld();
        const region = if (use_old) &self.old_region else &self.young_region;
        
        // Try allocation
        const header = region.allocate(total_size) catch |err| {
            switch (err) {
                error.OutOfMemory => {
                    // Try collection and retry
                    try self.collectGeneration(if (use_old) .Old else .Young);
                    return region.allocate(total_size) catch |retry_err| {
                        std.log.err("FixedGC: Allocation failed after collection: {}", .{retry_err});
                        return retry_err;
                    };
                },
                else => return err,
            }
        };
        
        // Initialize header safely
        header.* = FixedObjectHeader.init(@intCast(total_size), type_id);
        header.generation = if (use_old) 1 else 0;
        
        // Link to object list
        self.linkObject(header);
        
        // Update statistics
        self.updateAllocStats(total_size);
        
        // Check if GC should be triggered
        self.checkGCTrigger();
        
        return header.getData();
    }
    
    fn linkObject(self: *FixedGC, header: *FixedObjectHeader) void {
        // This should be atomic, but for simplicity we'll use a mutex
        self.collection_mutex.lock();
        defer self.collection_mutex.unlock();
        
        header.next = self.all_objects;
        self.all_objects = header;
    }
    
    fn updateAllocStats(self: *FixedGC, size: usize) void {
        // Atomic updates to avoid race conditions
        _ = @atomicRmw(u64, &self.stats.total_allocations, .Add, 1, .monotonic);
        _ = @atomicRmw(u64, &self.stats.total_bytes_allocated, .Add, size, .monotonic);
        
        // Update current heap size (non-atomic for simplicity)
        self.stats.current_heap_size += size;
        
        // Update peak heap size
        if (self.stats.current_heap_size > self.stats.peak_heap_size) {
            self.stats.peak_heap_size = self.stats.current_heap_size;
        }
    }
    
    fn shouldAllocateInOld(self: *FixedGC) bool {
        // Simple heuristic: every Nth allocation goes to old generation
        return self.stats.total_allocations % self.config.promotion_threshold == 0;
    }
    
    fn checkGCTrigger(self: *FixedGC) void {
        const heap_usage = @as(f32, @floatFromInt(self.stats.current_heap_size)) / 
                          @as(f32, @floatFromInt(self.config.initial_heap_size));
        
        if (heap_usage > self.config.gc_trigger_threshold) {
            self.triggerCollection();
        }
    }
    
    fn triggerCollection(self: *FixedGC) void {
        self.collection_mutex.lock();
        self.collection_condition.signal();
        self.collection_mutex.unlock();
    }
    
    /// Collect specific generation safely
    fn collectGeneration(self: *FixedGC, generation: Generation) !void {
        const start_time = std.time.nanoTimestamp();
        defer {
            const end_time = std.time.nanoTimestamp();
            const pause_time = @divTrunc(end_time - start_time, 1000); // microseconds
            self.stats.total_pause_time_us += @intCast(pause_time);
            self.stats.max_pause_time_us = @max(self.stats.max_pause_time_us, @intCast(pause_time));
        }
        
        std.log.debug("FixedGC: Starting collection for generation {}", .{generation});
        
        // Mark phase
        try self.markPhase();
        
        // Sweep phase
        const collected = self.sweepPhase(generation);
        
        // Update statistics
        self.stats.gc_cycles += 1;
        switch (generation) {
            .Young => self.stats.young_collections += collected,
            .Old => self.stats.old_collections += collected,
        }
        
        // Update weak references
        self.updateWeakReferences();
        
        std.log.debug("FixedGC: Collected {} objects from generation {}", .{collected, generation});
    }
    
    /// Mark phase with proper error handling
    fn markPhase(self: *FixedGC) !void {
        // Clear mark stack
        self.mark_stack.clearAndFree();
        
        // Mark roots
        try self.markRoots();
        
        // Process mark stack
        while (self.mark_stack.items.len > 0) {
            const obj = self.mark_stack.pop();
            try self.markObject(obj);
        }
    }
    
    fn markRoots(self: *FixedGC) !void {
        self.roots_mutex.lock();
        defer self.roots_mutex.unlock();
        
        // Mark registered roots
        for (self.roots.items) |root_ptr| {
            if (root_ptr.*) |obj_ptr| {
                if (self.isValidHeapPointer(obj_ptr)) {
                    const header = FixedObjectHeader.fromData(obj_ptr) catch continue;
                    if (header.color == 0) { // White
                        header.color = 1; // Gray
                        try self.mark_stack.append(header);
                    }
                }
            }
        }
        
        // Mark stack roots (conservative scanning)
        try self.markStackRoots();
    }
    
    fn markStackRoots(self: *FixedGC) !void {
        for (self.stack_roots.items) |root_ptr| {
            // Conservative stack scanning around the root pointer
            const scan_size = 1024; // 1KB around the root
            const start_addr = @intFromPtr(root_ptr);
            
            var scan_addr = start_addr - scan_size / 2;
            const end_addr = start_addr + scan_size / 2;
            
            while (scan_addr < end_addr) {
                const potential_ptr = @as(*?*anyopaque, @ptrFromInt(scan_addr)).*;
                
                if (potential_ptr) |ptr| {
                    if (self.isValidHeapPointer(ptr)) {
                        const header = FixedObjectHeader.fromData(ptr) catch {
                            scan_addr += @sizeOf(*anyopaque);
                            continue;
                        };
                        
                        if (header.color == 0) { // White
                            header.color = 1; // Gray
                            self.mark_stack.append(header) catch {};
                        }
                    }
                }
                
                scan_addr += @sizeOf(*anyopaque);
            }
        }
    }
    
    fn markObject(self: *FixedGC, header: *FixedObjectHeader) !void {
        if (header.color != 1) return; // Not gray
        
        header.color = 2; // Black
        
        // Mark children based on type
        try self.markObjectChildren(header);
    }
    
    fn markObjectChildren(self: *FixedGC, header: *FixedObjectHeader) !void {
        const data = header.getData();
        
        // Type-specific marking
        switch (header.type_id) {
            1 => {}, // Primitive types have no children
            2 => try self.markArrayChildren(data, header.size - FixedObjectHeader.HEADER_SIZE),
            3 => try self.markStructChildren(data, header.size - FixedObjectHeader.HEADER_SIZE),
            else => {},
        }
    }
    
    fn markArrayChildren(self: *FixedGC, data: *anyopaque, size: usize) !void {
        const ptr_size = @sizeOf(*anyopaque);
        var offset: usize = 0;
        
        while (offset + ptr_size <= size) {
            const child_ptr_ptr = @as(*?*anyopaque, @ptrFromInt(@intFromPtr(data) + offset));
            
            if (child_ptr_ptr.*) |child_ptr| {
                if (self.isValidHeapPointer(child_ptr)) {
                    const child_header = FixedObjectHeader.fromData(child_ptr) catch {
                        offset += ptr_size;
                        continue;
                    };
                    
                    if (child_header.color == 0) { // White
                        child_header.color = 1; // Gray
                        try self.mark_stack.append(child_header);
                    }
                }
            }
            
            offset += ptr_size;
        }
    }
    
    fn markStructChildren(self: *FixedGC, data: *anyopaque, size: usize) !void {
        // Simplified struct marking - in reality would use type metadata
        try self.markArrayChildren(data, size);
    }
    
    /// Sweep phase with proper cleanup
    fn sweepPhase(self: *FixedGC, generation: Generation) u64 {
        var collected: u64 = 0;
        var prev: ?*FixedObjectHeader = null;
        var current = self.all_objects;
        
        while (current) |obj| {
            const next = obj.next;
            
            // Check if object belongs to the generation being collected
            const obj_generation: Generation = if (obj.generation == 0) .Young else .Old;
            const should_collect = (generation == .Old) or (obj_generation == generation);
            
            if (should_collect and obj.color == 0) { // White objects are garbage
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
                obj.color = 0; // White
                prev = obj;
            }
            
            current = next;
        }
        
        return collected;
    }
    
    fn queueForFinalization(self: *FixedGC, obj: *FixedObjectHeader) void {
        self.finalization_mutex.lock();
        defer self.finalization_mutex.unlock();
        
        self.finalization_queue.append(obj) catch {
            // If we can't queue for finalization, free immediately
            self.freeObjectDirect(obj);
        };
    }
    
    fn freeObject(self: *FixedGC, obj: *FixedObjectHeader, prev: ?*FixedObjectHeader) void {
        // Remove from object list
        if (prev) |p| {
            p.next = obj.next;
        } else {
            self.all_objects = obj.next;
        }
        
        self.freeObjectDirect(obj);
    }
    
    fn freeObjectDirect(self: *FixedGC, obj: *FixedObjectHeader) void {
        // Invalidate the header to catch use-after-free
        obj.magic = 0xDEADDEAD;
        
        // Update statistics
        if (self.stats.current_heap_size >= obj.size) {
            self.stats.current_heap_size -= obj.size;
        } else {
            self.stats.current_heap_size = 0;
        }
    }
    
    /// Check if pointer is valid heap pointer
    fn isValidHeapPointer(self: *FixedGC, ptr: *anyopaque) bool {
        return self.young_region.contains(ptr) or self.old_region.contains(ptr);
    }
    
    /// Update weak references after collection
    fn updateWeakReferences(self: *FixedGC) void {
        self.weak_ref_mutex.lock();
        defer self.weak_ref_mutex.unlock();
        
        var i: usize = 0;
        while (i < self.weak_refs.items.len) {
            const weak_ref = self.weak_refs.items[i];
            
            if (weak_ref.header) |header| {
                if (!header.isValid() or header.color == 0) { // Invalid or collected
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
    
    /// Clean up all weak references
    fn cleanupWeakReferences(self: *FixedGC) void {
        self.weak_ref_mutex.lock();
        defer self.weak_ref_mutex.unlock();
        
        for (self.weak_refs.items) |weak_ref| {
            weak_ref.target = null;
            weak_ref.header = null;
            self.allocator.destroy(weak_ref);
        }
        
        self.weak_refs.clearAndFree(self.allocator);
    }
    
    /// Run final finalization before shutdown
    fn runFinalFinalization(self: *FixedGC) void {
        self.finalization_mutex.lock();
        defer self.finalization_mutex.unlock();
        
        // Process any remaining finalization queue
        for (self.finalization_queue.items) |obj| {
            // Find and run finalizer
            for (self.finalizers.items) |finalizer| {
                if (finalizer.object == obj) {
                    finalizer.fn_ptr(obj.getData());
                    self.stats.finalized_objects += 1;
                    break;
                }
            }
            
            self.freeObjectDirect(obj);
        }
        
        self.finalization_queue.clearAndFree();
        self.finalizers.clearAndFree();
    }
    
    /// Add root reference safely
    pub fn addRoot(self: *FixedGC, ptr: *?*anyopaque) !void {
        self.roots_mutex.lock();
        defer self.roots_mutex.unlock();
        try self.roots.append(ptr);
    }
    
    /// Remove root reference safely  
    pub fn removeRoot(self: *FixedGC, ptr: *?*anyopaque) void {
        self.roots_mutex.lock();
        defer self.roots_mutex.unlock();
        
        for (self.roots.items, 0..) |root, i| {
            if (root == ptr) {
                _ = self.roots.swapRemove(i);
                break;
            }
        }
    }
    
    /// Create weak reference safely
    pub fn createWeakRef(self: *FixedGC, target: *anyopaque) !*WeakRef {
        const weak_ref = try self.allocator.create(WeakRef);
        errdefer self.allocator.destroy(weak_ref);
        
        weak_ref.target = target;
        weak_ref.header = FixedObjectHeader.fromData(target) catch {
            self.allocator.destroy(weak_ref);
            return error.InvalidTarget;
        };
        
        self.weak_ref_mutex.lock();
        defer self.weak_ref_mutex.unlock();
        try self.weak_refs.append(self.allocator, weak_ref);
        
        return weak_ref;
    }
    
    /// Get current GC statistics
    pub fn getStats(self: *FixedGC) GCStats {
        return self.stats;
    }
    
    /// Print GC statistics safely
    pub fn printStats(self: *FixedGC) void {
        const stats = self.getStats();
        
        std.log.info("=== Fixed GC Statistics ===", .{});
        std.log.info("Total allocations: {}", .{stats.total_allocations});
        std.log.info("Total bytes allocated: {} bytes", .{stats.total_bytes_allocated});
        std.log.info("GC cycles: {}", .{stats.gc_cycles});
        std.log.info("Total pause time: {} μs", .{stats.total_pause_time_us});
        std.log.info("Max pause time: {} μs", .{stats.max_pause_time_us});
        if (stats.gc_cycles > 0) {
            std.log.info("Average pause time: {} μs", .{stats.total_pause_time_us / stats.gc_cycles});
        }
        std.log.info("Young collections: {}", .{stats.young_collections});
        std.log.info("Old collections: {}", .{stats.old_collections});
        std.log.info("Current heap size: {} bytes", .{stats.current_heap_size});
        std.log.info("Peak heap size: {} bytes", .{stats.peak_heap_size});
        std.log.info("============================", .{});
    }
};

/// Generation enum for type safety
const Generation = enum {
    Young,
    Old,
};

/// Write barrier record
const WriteBarrier = struct {
    old_ref: *anyopaque,
    new_ref: *anyopaque,
    timestamp: u64,
};

/// Finalizer registration
const Finalizer = struct {
    object: *FixedObjectHeader,
    fn_ptr: FinalizerFn,
};

/// Background worker for concurrent collection
fn concurrentCollectionWorker(gc_instance: *FixedGC) void {
    while (!gc_instance.stop_collection.load(.acquire)) {
        gc_instance.collection_mutex.lock();
        
        // Wait for collection trigger or timeout
        gc_instance.collection_condition.timedWait(&gc_instance.collection_mutex, 100_000_000) catch {}; // 100ms
        
        if (!gc_instance.stop_collection.load(.acquire)) {
            gc_instance.collection_running.store(true, .release);
            gc_instance.collection_mutex.unlock();
            
            // Perform minor collection (young generation)
            gc_instance.collectGeneration(.Young) catch |err| {
                std.log.err("FixedGC: Minor collection failed: {}", .{err});
            };
            
            gc_instance.collection_running.store(false, .release);
        } else {
            gc_instance.collection_mutex.unlock();
        }
    }
}

/// Background worker for finalization
fn finalizationWorker(gc_instance: *FixedGC) void {
    while (!gc_instance.stop_collection.load(.acquire)) {
        gc_instance.finalization_mutex.lock();
        
        if (gc_instance.finalization_queue.items.len > 0) {
            // Take ownership of objects to finalize
            const objects_to_finalize = gc_instance.finalization_queue.toOwnedSlice() catch {
                gc_instance.finalization_mutex.unlock();
                std.time.sleep(10_000_000); // 10ms
                continue;
            };
            gc_instance.finalization_mutex.unlock();
            
            // Run finalizers outside of lock
            for (objects_to_finalize) |obj| {
                // Find and run finalizer
                for (gc_instance.finalizers.items) |finalizer| {
                    if (finalizer.object == obj) {
                        finalizer.fn_ptr(obj.getData());
                        gc_instance.stats.finalized_objects += 1;
                        break;
                    }
                }
                
                // Free the object
                gc_instance.freeObjectDirect(obj);
            }
            
            gc_instance.arena_allocator.free(objects_to_finalize);
        } else {
            gc_instance.finalization_mutex.unlock();
            
            // Sleep for a bit before checking again
            std.time.sleep(10_000_000); // 10ms
        }
    }
}

// Export C API for integration with LLVM-generated code
export fn cursed_fixed_gc_init(initial_heap_size: usize) ?*FixedGC {
    const allocator = std.heap.page_allocator;
    var config = GCConfig.default();
    config.initial_heap_size = initial_heap_size;
    
    return FixedGC.init(allocator, config) catch null;
}

export fn cursed_fixed_gc_deinit(gc_instance: ?*FixedGC) void {
    if (gc_instance) |gc| {
        gc.deinit();
    }
}

export fn cursed_fixed_gc_alloc(gc_instance: ?*FixedGC, size: usize, type_id: u16) ?*anyopaque {
    if (gc_instance) |gc| {
        return gc.alloc(size, type_id) catch null;
    }
    return null;
}

export fn cursed_fixed_gc_add_root(gc_instance: ?*FixedGC, ptr: *?*anyopaque) void {
    if (gc_instance) |gc| {
        gc.addRoot(ptr) catch {};
    }
}

export fn cursed_fixed_gc_remove_root(gc_instance: ?*FixedGC, ptr: *?*anyopaque) void {
    if (gc_instance) |gc| {
        gc.removeRoot(ptr);
    }
}

export fn cursed_fixed_gc_collect(gc_instance: ?*FixedGC) void {
    if (gc_instance) |gc| {
        gc.collectGeneration(.Young) catch {};
    }
}

export fn cursed_fixed_gc_print_stats(gc_instance: ?*FixedGC) void {
    if (gc_instance) |gc| {
        gc.printStats();
    }
}

// Tests for fixed GC
test "fixed gc initialization and cleanup" {
    const allocator = std.testing.allocator;
    
    var config = GCConfig.default();
    config.initial_heap_size = 1024 * 1024; // 1MB for testing
    
    const gc_instance = try FixedGC.init(allocator, config);
    defer gc_instance.deinit();
    
    const stats = gc_instance.getStats();
    try std.testing.expect(stats.total_allocations == 0);
}

test "fixed gc allocation and collection" {
    const allocator = std.testing.allocator;
    
    var config = GCConfig.default();
    config.initial_heap_size = 1024 * 1024; // 1MB
    config.young_gen_size = 512 * 1024;     // 512KB
    
    const gc_instance = try FixedGC.init(allocator, config);
    defer gc_instance.deinit();
    
    // Allocate some objects
    const obj1 = try gc_instance.alloc(100, 1);
    const obj2 = try gc_instance.alloc(200, 2);
    
    try std.testing.expect(obj1 != obj2);
    
    const stats = gc_instance.getStats();
    try std.testing.expect(stats.total_allocations >= 2);
}

test "fixed gc root management" {
    const allocator = std.testing.allocator;
    
    var config = GCConfig.default();
    config.initial_heap_size = 1024 * 1024;
    
    const gc_instance = try FixedGC.init(allocator, config);
    defer gc_instance.deinit();
    
    var root: ?*anyopaque = null;
    try gc_instance.addRoot(&root);
    
    root = try gc_instance.alloc(50, 1);
    
    // Trigger collection - root should keep object alive
    try gc_instance.collectGeneration(.Young);
    
    gc_instance.removeRoot(&root);
}
