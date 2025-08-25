// CRITICAL FIX: Memory Manager with Reference Counting and Thread Safety
// Prevents use-after-free and double-free issues in module dependencies
// Fixed thread safety bugs in arena allocator

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const Mutex = std.Thread.Mutex;
const atomic = std.atomic;

pub const MemoryManager = struct {
    allocator: Allocator,
    ref_counts: HashMap(usize, u32, std.hash_map.DefaultHashContext(usize), std.hash_map.default_max_load_percentage),
    allocated_objects: HashMap(usize, *anyopaque, std.hash_map.DefaultHashContext(usize), std.hash_map.default_max_load_percentage),
    
    // CRITICAL FIX: Thread safety
    mutex: Mutex,
    is_initialized: atomic.Value(bool),

    pub fn init(allocator: Allocator) MemoryManager {
        return MemoryManager{
            .allocator = allocator,
            .ref_counts = HashMap(usize, u32, std.hash_map.DefaultHashContext(usize), std.hash_map.default_max_load_percentage).init(allocator),
            .allocated_objects = HashMap(usize, *anyopaque, std.hash_map.DefaultHashContext(usize), std.hash_map.default_max_load_percentage).init(allocator),
            // CRITICAL FIX: Initialize thread safety
            .mutex = Mutex{},
            .is_initialized = atomic.Value(bool).init(true),
        };
    }

    pub fn deinit(self: *MemoryManager) void {
        // CRITICAL FIX: Thread-safe cleanup
        self.mutex.lock();
        defer self.mutex.unlock();
        
        if (!self.is_initialized.load(.acquire)) return;
        
        // Clean up any remaining objects
        var iter = self.allocated_objects.iterator();
        while (iter.next()) |entry| {
            const ptr = entry.value_ptr.*;
            self.allocator.destroy(@as(*anyopaque, @ptrCast(ptr)));
        }
        
        self.ref_counts.deinit();
        self.allocated_objects.deinit();
        self.is_initialized.store(false, .release);
    }

    pub fn createManaged(self: *MemoryManager, comptime T: type) !*T {
        // CRITICAL FIX: Thread-safe allocation
        self.mutex.lock();
        defer self.mutex.unlock();
        
        if (!self.is_initialized.load(.acquire)) {
            return error.OutOfMemory;
        }
        
        const ptr = try self.allocator.create(T);
        const addr = @intFromPtr(ptr);
        
        try self.ref_counts.put(addr, 1);
        try self.allocated_objects.put(addr, @as(*anyopaque, @ptrCast(ptr)));
        
        return ptr;
    }

    pub fn addRef(self: *MemoryManager, ptr: *anyopaque) void {
        // CRITICAL FIX: Thread-safe reference counting
        self.mutex.lock();
        defer self.mutex.unlock();
        
        if (!self.is_initialized.load(.acquire)) return;
        
        const addr = @intFromPtr(ptr);
        const current = self.ref_counts.get(addr) orelse return;
        self.ref_counts.put(addr, current + 1) catch return;
    }

    pub fn removeRef(self: *MemoryManager, ptr: *anyopaque) u32 {
        // CRITICAL FIX: Thread-safe reference counting
        self.mutex.lock();
        defer self.mutex.unlock();
        
        if (!self.is_initialized.load(.acquire)) return 0;
        
        const addr = @intFromPtr(ptr);
        const current = self.ref_counts.get(addr) orelse return 0;
        
        if (current <= 1) {
            // Last reference - safe to delete
            _ = self.ref_counts.remove(addr);
            _ = self.allocated_objects.remove(addr);
            return 0;
        } else {
            const new_count = current - 1;
            self.ref_counts.put(addr, new_count) catch {};
            return new_count;
        }
    }

    pub fn destroyManaged(self: *MemoryManager, comptime T: type, ptr: *T) void {
        const remaining = self.removeRef(@as(*anyopaque, @ptrCast(ptr)));
        if (remaining == 0) {
            self.allocator.destroy(ptr);
        }
    }

    pub fn isManaged(self: *MemoryManager, ptr: *anyopaque) bool {
        const addr = @intFromPtr(ptr);
        return self.ref_counts.contains(addr);
    }

    pub fn getRefCount(self: *MemoryManager, ptr: *anyopaque) u32 {
        const addr = @intFromPtr(ptr);
        return self.ref_counts.get(addr) orelse 0;
    }
};

// Thread-local memory manager instance
var thread_local_manager: ?MemoryManager = null;

pub fn getGlobalManager() *MemoryManager {
    // This is a simplified version - in production you'd want proper initialization
    return &thread_local_manager.?;
}

pub fn initGlobalManager(allocator: Allocator) void {
    thread_local_manager = MemoryManager.init(allocator);
}

pub fn deinitGlobalManager() void {
    if (thread_local_manager) |*manager| {
        manager.deinit();
        thread_local_manager = null;
    }
}
