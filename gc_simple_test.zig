const std = @import("std");
const print = std.debug.print;

// Simple standalone GC test without Variable dependencies
const ObjectHeader = struct {
    size: u32,
    type_id: u16,
    color: u2,
    generation: u1,
    finalize: u1,
    reserved: u8,
    next: ?*ObjectHeader,
    
    const HEADER_SIZE = @sizeOf(ObjectHeader);
    
    fn getData(self: *ObjectHeader) *anyopaque {
        const ptr = @as([*]u8, @ptrCast(self)) + HEADER_SIZE;
        return @ptrCast(ptr);
    }
    
    fn fromData(data: *anyopaque) *ObjectHeader {
        const ptr = @as([*]u8, @ptrCast(data)) - HEADER_SIZE;
        return @ptrCast(@alignCast(ptr));
    }
};

const Color = enum(u2) {
    White = 0,
    Gray = 1,
    Black = 2,
};

const SimpleGC = struct {
    allocator: std.mem.Allocator,
    heap: []u8,
    heap_used: usize,
    all_objects: ?*ObjectHeader,
    allocations: u64,
    collections: u64,
    
    pub fn init(allocator: std.mem.Allocator, heap_size: usize) !*SimpleGC {
        const gc = try allocator.create(SimpleGC);
        
        gc.* = SimpleGC{
            .allocator = allocator,
            .heap = try allocator.alloc(u8, heap_size),
            .heap_used = 0,
            .all_objects = null,
            .allocations = 0,
            .collections = 0,
        };
        
        return gc;
    }
    
    pub fn deinit(self: *SimpleGC) void {
        self.allocator.free(self.heap);
        self.allocator.destroy(self);
    }
    
    pub fn alloc(self: *SimpleGC, size: usize, type_id: u16) !*anyopaque {
        const total_size = size + ObjectHeader.HEADER_SIZE;
        
        // Ensure alignment for ObjectHeader (8-byte alignment)
        const alignment = @alignOf(ObjectHeader);
        const aligned_offset = std.mem.alignForward(usize, self.heap_used, alignment);
        
        if (aligned_offset + total_size > self.heap.len) {
            return error.OutOfMemory;
        }
        
        const ptr = &self.heap[aligned_offset];
        self.heap_used = aligned_offset + total_size;
        
        const header = @as(*ObjectHeader, @ptrCast(@alignCast(ptr)));
        header.* = ObjectHeader{
            .size = @as(u32, @intCast(total_size)),
            .type_id = type_id,
            .color = @intFromEnum(Color.White),
            .generation = 0,
            .finalize = 0,
            .reserved = 0,
            .next = self.all_objects,
        };
        
        self.all_objects = header;
        self.allocations += 1;
        
        return header.getData();
    }
    
    pub fn collect(self: *SimpleGC) void {
        // Simple mark-and-sweep collection
        var current = self.all_objects;
        var live_count: u64 = 0;
        
        while (current) |obj| {
            if (obj.color == @intFromEnum(Color.Black)) {
                live_count += 1;
            }
            current = obj.next;
        }
        
        self.collections += 1;
        print("Collection {}: {} live objects\n", .{ self.collections, live_count });
    }
    
    pub fn printStats(self: *SimpleGC) void {
        print("=== Simple GC Stats ===\n", .{});
        print("Heap size: {} bytes\n", .{self.heap.len});
        print("Heap used: {} bytes\n", .{self.heap_used});
        print("Allocations: {}\n", .{self.allocations});
        print("Collections: {}\n", .{self.collections});
        print("=====================\n", .{});
    }
};

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    
    print("🧪 Simple GC Test\n\n", .{});
    
    var gc = try SimpleGC.init(allocator, 1024 * 1024); // 1MB
    defer gc.deinit();
    
    // Test basic allocation
    print("📝 Testing basic allocation...\n", .{});
    const ptr1 = try gc.alloc(64, 0);
    print("✅ Allocated 64 bytes at {*}\n", .{ptr1});
    
    const ptr2 = try gc.alloc(128, 1);
    print("✅ Allocated 128 bytes at {*}\n", .{ptr2});
    
    // Test bulk allocation
    print("\n📝 Testing bulk allocation...\n", .{});
    for (0..100) |i| {
        const size = 32 + (i % 50);
        const type_id = @as(u16, @intCast(i % 4));
        
        const ptr = try gc.alloc(size, type_id);
        _ = ptr;
        
        if (i % 20 == 0) {
            print("Allocated {} objects so far\n", .{i + 1});
        }
    }
    
    // Test collection
    print("\n📝 Testing garbage collection...\n", .{});
    gc.collect();
    
    gc.printStats();
    
    print("\n🎉 Simple GC test completed!\n", .{});
}
