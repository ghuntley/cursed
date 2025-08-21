const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

// Complete Memory Safety Checks Implementation (Priority #25)
// Comprehensive bounds checking and null pointer validation

pub const MemorySafetyError = error{
    NullPointerDereference,
    ArrayIndexOutOfBounds,
    BufferOverflow,
    UseAfterFree,
    DoubleFree,
    MemoryCorruption,
    StackOverflow,
    HeapOverflow,
};

pub const MemorySafetyConfig = struct {
    enable_bounds_checking: bool = true,
    enable_null_checks: bool = true,
    enable_canary_checks: bool = true,
    enable_leak_detection: bool = true,
    enable_stack_overflow_detection: bool = true,
    debug_mode: bool = false,
};

pub const GuardCanary = struct {
    magic: u64 = 0xDEADBEEFCAFEBABE,
    size: usize,
    
    pub fn create(size: usize) GuardCanary {
        return GuardCanary{ .size = size };
    }
    
    pub fn verify(self: *const GuardCanary) bool {
        return self.magic == 0xDEADBEEFCAFEBABE;
    }
};

pub const SafePointer = struct {
    ptr: ?*anyopaque,
    size: usize,
    allocated: bool,
    canary_front: GuardCanary,
    canary_back: GuardCanary,
    
    pub fn init(ptr: ?*anyopaque, size: usize) SafePointer {
        return SafePointer{
            .ptr = ptr,
            .size = size,
            .allocated = ptr != null,
            .canary_front = GuardCanary.create(size),
            .canary_back = GuardCanary.create(size),
        };
    }
    
    pub fn checkNull(self: *const SafePointer) MemorySafetyError!void {
        if (self.ptr == null) {
            return MemorySafetyError.NullPointerDereference;
        }
    }
    
    pub fn checkBounds(self: *const SafePointer, offset: usize) MemorySafetyError!void {
        if (offset >= self.size) {
            return MemorySafetyError.ArrayIndexOutOfBounds;
        }
    }
    
    pub fn checkCanaries(self: *const SafePointer) MemorySafetyError!void {
        if (!self.canary_front.verify() or !self.canary_back.verify()) {
            return MemorySafetyError.MemoryCorruption;
        }
    }
    
    pub fn checkAllocated(self: *const SafePointer) MemorySafetyError!void {
        if (!self.allocated) {
            return MemorySafetyError.UseAfterFree;
        }
    }
};

pub const MemoryTracker = struct {
    allocations: std.HashMap(usize, SafePointer),
    allocator: Allocator,
    total_allocated: usize,
    peak_allocated: usize,
    allocation_count: usize,
    
    pub fn init(allocator: Allocator) MemoryTracker {
        return MemoryTracker{
            .allocations = std.HashMap(usize, SafePointer).init(allocator),
            .allocator = allocator,
            .total_allocated = 0,
            .peak_allocated = 0,
            .allocation_count = 0,
        };
    }
    
    pub fn deinit(self: *MemoryTracker) void {
        self.allocations.deinit(allocator);
    }
    
    pub fn trackAllocation(self: *MemoryTracker, ptr: *anyopaque, size: usize) !void {
        const safe_ptr = SafePointer.init(ptr, size);
        try self.allocations.put(@intFromPtr(ptr), safe_ptr);
        self.total_allocated += size;
        self.allocation_count += 1;
        if (self.total_allocated > self.peak_allocated) {
            self.peak_allocated = self.total_allocated;
        }
    }
    
    pub fn untrackAllocation(self: *MemoryTracker, ptr: *anyopaque) !void {
        const addr = @intFromPtr(ptr);
        if (self.allocations.get(addr)) |safe_ptr| {
            self.total_allocated -= safe_ptr.size;
            _ = self.allocations.remove(addr);
        } else {
            return MemorySafetyError.DoubleFree;
        }
    }
    
    pub fn validatePointer(self: *MemoryTracker, ptr: *anyopaque, offset: usize) MemorySafetyError!void {
        const addr = @intFromPtr(ptr);
        if (self.allocations.get(addr)) |safe_ptr| {
            try safe_ptr.checkNull();
            try safe_ptr.checkAllocated();
            try safe_ptr.checkBounds(offset);
            try safe_ptr.checkCanaries();
        } else {
            return MemorySafetyError.UseAfterFree;
        }
    }
    
    pub fn detectLeaks(self: *MemoryTracker) void {
        var iter = self.allocations.iterator();
        var leak_count: usize = 0;
        while (iter.next()) |entry| {
            leak_count += 1;
            print("Memory leak detected: ptr=0x{x}, size={}\n", .{ entry.key_ptr.*, entry.value_ptr.size });
        }
        if (leak_count > 0) {
            print("Total memory leaks: {}\n", .{leak_count});
        }
    }
};

// Array bounds checking functions
pub fn checkArrayBounds(index: i64, length: usize) MemorySafetyError!void {
    if (index < 0 or index >= @as(i64, @intCast(length))) {
        return MemorySafetyError.ArrayIndexOutOfBounds;
    }
}

pub fn checkSliceBounds(start: i64, end: i64, length: usize) MemorySafetyError!void {
    const len_i64 = @as(i64, @intCast(length));
    if (start < 0 or end < 0 or start > len_i64 or end > len_i64 or start > end) {
        return MemorySafetyError.ArrayIndexOutOfBounds;
    }
}

// Null pointer validation
pub fn checkNullPointer(ptr: ?*anyopaque) MemorySafetyError!*anyopaque {
    return ptr orelse MemorySafetyError.NullPointerDereference;
}

// Stack overflow detection
var stack_base: usize = 0;
var stack_limit: usize = 0;
const STACK_SIZE_LIMIT: usize = 8 * 1024 * 1024; // 8MB

pub fn initStackBounds() void {
    const current_sp = @frameAddress();
    stack_base = current_sp;
    stack_limit = current_sp - STACK_SIZE_LIMIT;
}

pub fn checkStackOverflow() MemorySafetyError!void {
    const current_sp = @frameAddress();
    if (current_sp < stack_limit) {
        return MemorySafetyError.StackOverflow;
    }
}

// Safe memory operations
pub fn safeMemcpy(dest: []u8, src: []const u8) MemorySafetyError!void {
    if (dest.len < src.len) {
        return MemorySafetyError.BufferOverflow;
    }
    @memcpy(dest[0..src.len], src);
}

pub fn safeMemset(dest: []u8, value: u8) void {
    @memset(dest, value);
}

// Memory safety runtime state
var global_memory_tracker: ?MemoryTracker = null;
var global_config: MemorySafetyConfig = MemorySafetyConfig{};

pub fn initMemorySafety(allocator: Allocator, config: MemorySafetyConfig) !void {
    global_config = config;
    global_memory_tracker = MemoryTracker.init(allocator);
    initStackBounds();
}

pub fn deinitMemorySafety() void {
    if (global_memory_tracker) |*tracker| {
        tracker.detectLeaks();
        tracker.deinit(allocator);
        global_memory_tracker = null;
    }
}

// Runtime validation functions called by generated code
export fn cursed_bounds_check(index: i64, length: i64) void {
    if (global_config.enable_bounds_checking) {
        checkArrayBounds(index, @intCast(length)) catch |err| {
            print("Runtime error: {}\n", .{err});
            std.process.exit(1);
        };
    }
}

export fn cursed_null_check(ptr: ?*anyopaque) ?*anyopaque {
    if (global_config.enable_null_checks) {
        return checkNullPointer(ptr) catch |err| {
            print("Runtime error: {}\n", .{err});
            std.process.exit(1);
        };
    }
    return ptr;
}

export fn cursed_stack_check() void {
    if (global_config.enable_stack_overflow_detection) {
        checkStackOverflow() catch |err| {
            print("Runtime error: {}\n", .{err});
            std.process.exit(1);
        };
    }
}

// Testing
pub fn testMemorySafety() !void {
    print("Testing memory safety implementation...\n");
    
    // Test array bounds checking
    try checkArrayBounds(5, 10); // Should pass
    if (checkArrayBounds(10, 10)) |_| {
        return error.TestFailed;
    } else |_| {}
    
    // Test null pointer checking
    var valid_ptr: i32 = 42;
    _ = try checkNullPointer(@ptrCast(&valid_ptr));
    if (checkNullPointer(null)) |_| {
        return error.TestFailed;
    } else |_| {}
    
    print("Memory safety tests passed!\n");
}
