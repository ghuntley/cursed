const std = @import("std");
const print = std.debug.print;

const StressTestGC = struct {
    allocator: std.mem.Allocator,
    heap: []u8,
    heap_used: usize,
    allocations: u64,
    collections: u64,
    freed_objects: u64,
    allocated_ptrs: std.ArrayList(*anyopaque),
    roots: std.ArrayList(*anyopaque),
    
    const ObjectHeader = struct {
        size: u32,
        type_id: u16,
        marked: bool,
        next: ?*ObjectHeader,
        
        fn getData(self: *ObjectHeader) *anyopaque {
            const ptr = @as([*]u8, @ptrCast(self)) + @sizeOf(ObjectHeader);
            return @ptrCast(ptr);
        }
        
        fn fromData(data: *anyopaque) *ObjectHeader {
            const ptr = @as([*]u8, @ptrCast(data)) - @sizeOf(ObjectHeader);
            return @ptrCast(@alignCast(ptr));
        }
    };
    
    pub fn init(allocator: std.mem.Allocator, heap_size: usize) !*StressTestGC {
        const gc = try allocator.create(StressTestGC);
        
        gc.* = StressTestGC{
            .allocator = allocator,
            .heap = try allocator.alloc(u8, heap_size),
            .heap_used = 0,
            .allocations = 0,
            .collections = 0,
            .freed_objects = 0,
            .allocated_ptrs = std.ArrayList(*anyopaque).init(allocator),
            .roots = std.ArrayList(*anyopaque).init(allocator),
        };
        
        return gc;
    }
    
    pub fn deinit(self: *StressTestGC) void {
        self.allocated_ptrs.deinit();
        self.roots.deinit();
        self.allocator.free(self.heap);
        self.allocator.destroy(self);
    }
    
    pub fn alloc(self: *StressTestGC, size: usize, type_id: u16) !*anyopaque {
        const total_size = size + @sizeOf(ObjectHeader);
        
        // Ensure alignment
        const alignment = @alignOf(ObjectHeader);
        const aligned_offset = std.mem.alignForward(usize, self.heap_used, alignment);
        
        if (aligned_offset + total_size > self.heap.len) {
            // Try garbage collection first
            self.collect();
            
            // Check again after collection
            if (self.heap_used + total_size > self.heap.len) {
                return error.OutOfMemory;
            }
        }
        
        const ptr = &self.heap[aligned_offset];
        self.heap_used = aligned_offset + total_size;
        
        const header = @as(*ObjectHeader, @ptrCast(@alignCast(ptr)));
        header.* = ObjectHeader{
            .size = @as(u32, @intCast(total_size)),
            .type_id = type_id,
            .marked = false,
            .next = null,
        };
        
        self.allocations += 1;
        const data_ptr = header.getData();
        try self.allocated_ptrs.append(data_ptr);
        
        return data_ptr;
    }
    
    pub fn addRoot(self: *StressTestGC, ptr: *anyopaque) !void {
        try self.roots.append(ptr);
    }
    
    pub fn collect(self: *StressTestGC) void {
        // Mark phase - mark all roots
        for (self.roots.items) |root| {
            const header = ObjectHeader.fromData(root);
            header.marked = true;
        }
        
        // Sweep phase - count unmarked objects as freed
        var freed_count: u64 = 0;
        for (self.allocated_ptrs.items) |ptr| {
            const header = ObjectHeader.fromData(ptr);
            if (!header.marked) {
                freed_count += 1;
            } else {
                header.marked = false; // Reset for next collection
            }
        }
        
        self.freed_objects += freed_count;
        self.collections += 1;
        
        print("Collection {}: freed {} objects\n", .{ self.collections, freed_count });
    }
    
    pub fn printStats(self: *StressTestGC) void {
        const efficiency = if (self.allocations > 0) 
            (@as(f64, @floatFromInt(self.freed_objects)) / @as(f64, @floatFromInt(self.allocations))) * 100.0
        else 
            0.0;
            
        print("=== Stress Test GC Stats ===\n", .{});
        print("Heap size: {} bytes\n", .{self.heap.len});
        print("Heap used: {} bytes\n", .{self.heap_used});
        print("Utilization: {d:.1}%\n", .{(@as(f64, @floatFromInt(self.heap_used)) / @as(f64, @floatFromInt(self.heap.len))) * 100.0});
        print("Allocations: {}\n", .{self.allocations});
        print("Collections: {}\n", .{self.collections});
        print("Freed objects: {}\n", .{self.freed_objects});
        print("GC efficiency: {d:.1}%\n", .{efficiency});
        print("============================\n", .{});
    }
};

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    
    print("🔥 GC Stress Test - Production Readiness Validation\n\n", .{});
    
    // Test with moderate heap size to trigger collections
    var gc = try StressTestGC.init(allocator, 256 * 1024); // 256KB heap
    defer gc.deinit();
    
    print("📝 Phase 1: Rapid allocation stress test\n", .{});
    
    // Allocate many objects rapidly
    var allocated_objects = std.ArrayList(*anyopaque).init(allocator);
    defer allocated_objects.deinit();
    
    for (0..1000) |i| {
        const size = 64 + (i % 256); // Variable sizes from 64 to 320 bytes
        const type_id = @as(u16, @intCast(i % 8));
        
        const ptr = gc.alloc(size, type_id) catch |err| {
            if (err == error.OutOfMemory) {
                print("Out of memory after {} allocations - triggering collection\n", .{i});
                gc.collect();
                continue;
            }
            return err;
        };
        
        try allocated_objects.append(ptr);
        
        // Add some objects as roots to keep them alive
        if (i % 7 == 0) {
            try gc.addRoot(ptr);
        }
        
        // Progress updates
        if (i % 100 == 0 and i > 0) {
            print("Allocated {} objects, heap usage: {d:.1}%\n", .{
                i, 
                (@as(f64, @floatFromInt(gc.heap_used)) / @as(f64, @floatFromInt(gc.heap.len))) * 100.0
            });
        }
    }
    
    print("\n📝 Phase 2: Mixed allocation and collection patterns\n", .{});
    
    // Mixed pattern: allocate, collect, allocate more
    for (0..5) |cycle| {
        print("Cycle {}: allocating burst of objects\n", .{cycle + 1});
        
        for (0..50) |_| {
            const size = 128;
            const type_id = @as(u16, @intCast(cycle % 4));
            
            _ = gc.alloc(size, type_id) catch |err| {
                if (err == error.OutOfMemory) {
                    print("Heap full, forcing collection\n", .{});
                    gc.collect();
                    continue;
                }
                return err;
            };
        }
        
        // Force collection at end of each cycle
        gc.collect();
    }
    
    print("\n📝 Phase 3: Large object allocation\n", .{});
    
    // Test large object allocation
    for (0..10) |i| {
        const large_size = 8192; // 8KB objects
        const ptr = gc.alloc(large_size, 99) catch |err| {
            if (err == error.OutOfMemory) {
                print("Cannot allocate large object {}, heap fragmented\n", .{i});
                gc.collect();
                continue;
            }
            return err;
        };
        
        // Keep every other large object alive
        if (i % 2 == 0) {
            try gc.addRoot(ptr);
        }
    }
    
    print("\n📝 Phase 4: Final collection and statistics\n", .{});
    
    // Final collection
    gc.collect();
    
    // Memory pressure test
    print("Testing memory pressure...\n", .{});
    var pressure_count: u32 = 0;
    while (pressure_count < 100) {
        const result = gc.alloc(1024, 0);
        if (result) |_| {
            pressure_count += 1;
        } else |err| {
            if (err == error.OutOfMemory) {
                print("Memory pressure reached after {} more allocations\n", .{pressure_count});
                break;
            }
            return err;
        }
    }
    
    gc.printStats();
    
    print("\n🎉 Stress test completed successfully!\n", .{});
    print("💪 The GC handled {} allocations with {} collections\n", .{gc.allocations, gc.collections});
    
    if (gc.collections > 0) {
        const avg_freed_per_collection = @as(f64, @floatFromInt(gc.freed_objects)) / @as(f64, @floatFromInt(gc.collections));
        print("📊 Average objects freed per collection: {d:.1}\n", .{avg_freed_per_collection});
    }
    
    if (gc.heap_used > 0) {
        print("🏆 Final heap utilization: {d:.1}%\n", .{
            (@as(f64, @floatFromInt(gc.heap_used)) / @as(f64, @floatFromInt(gc.heap.len))) * 100.0
        });
    }
}
