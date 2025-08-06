const std = @import("std");
const Allocator = std.mem.Allocator;

/// Allocation guard to ensure paired allocations either both succeed or both fail
pub const AllocationGuard = struct {
    allocator: Allocator,
    allocations: std.ArrayList(*anyopaque),
    
    pub fn init(allocator: Allocator) AllocationGuard {
        return AllocationGuard{
            .allocator = allocator,
            .allocations = std.ArrayList(*anyopaque).init(allocator),
        };
    }
    
    pub fn deinit(self: *AllocationGuard) void {
        // Clean up any remaining allocations
        for (self.allocations.items) |ptr| {
            self.allocator.destroy(@as(*u8, @ptrCast(ptr)));
        }
        self.allocations.deinit();
    }
    
    /// Create a guarded allocation that will be cleaned up on guard destruction
    pub fn create(self: *AllocationGuard, comptime T: type) !*T {
        const ptr = try self.allocator.create(T);
        try self.allocations.append(@ptrCast(ptr));
        return ptr;
    }
    
    /// Create multiple guarded allocations atomically
    pub fn createPair(self: *AllocationGuard, comptime T1: type, comptime T2: type) !struct { *T1, *T2 } {
        const ptr1 = self.allocator.create(T1) catch return error.OutOfMemory;
        errdefer self.allocator.destroy(ptr1);
        
        const ptr2 = self.allocator.create(T2) catch {
            self.allocator.destroy(ptr1);
            return error.OutOfMemory;
        };
        
        // Both succeeded, add to guard
        try self.allocations.append(@ptrCast(ptr1));
        try self.allocations.append(@ptrCast(ptr2));
        
        return .{ ptr1, ptr2 };
    }
    
    /// Release a specific allocation from the guard (caller takes ownership)
    pub fn release(self: *AllocationGuard, ptr: *anyopaque) void {
        for (self.allocations.items, 0..) |allocation, i| {
            if (allocation == ptr) {
                _ = self.allocations.swapRemove(i);
                break;
            }
        }
    }
    
    /// Commit all allocations (remove from guard without freeing)
    pub fn commitAll(self: *AllocationGuard) void {
        self.allocations.clearRetainingCapacity();
    }
};

/// Arena-based allocation guard for temporary operations
pub const ArenaGuard = struct {
    arena: std.heap.ArenaAllocator,
    
    pub fn init(backing_allocator: Allocator) ArenaGuard {
        return ArenaGuard{
            .arena = std.heap.ArenaAllocator.init(backing_allocator),
        };
    }
    
    pub fn deinit(self: *ArenaGuard) void {
        self.arena.deinit();
    }
    
    pub fn allocator(self: *ArenaGuard) Allocator {
        return self.arena.allocator();
    }
    
    /// Reset the arena, freeing all allocations
    pub fn reset(self: *ArenaGuard) void {
        _ = self.arena.reset(.retain_capacity);
    }
};

/// Expression allocation helper for parser
pub const ExpressionAllocator = struct {
    guard: AllocationGuard,
    
    pub fn init(allocator: Allocator) ExpressionAllocator {
        return ExpressionAllocator{
            .guard = AllocationGuard.init(allocator),
        };
    }
    
    pub fn deinit(self: *ExpressionAllocator) void {
        self.guard.deinit();
    }
    
    /// Create a binary expression with guaranteed cleanup on failure
    pub fn createBinaryExpression(self: *ExpressionAllocator, comptime ExprType: type, left_expr: ExprType, right_expr: ExprType) !struct { *ExprType, *ExprType } {
        _ = left_expr;
        _ = right_expr;
        return try self.guard.createPair(ExprType, ExprType);
    }
    
    /// Commit successful expression creation
    pub fn commit(self: *ExpressionAllocator) void {
        self.guard.commitAll();
    }
};

// Tests
test "allocation guard basic usage" {
    const allocator = std.testing.allocator;
    
    var guard = AllocationGuard.init(allocator);
    defer guard.deinit();
    
    // Test successful allocation
    const ptr1 = try guard.create(i32);
    ptr1.* = 42;
    
    const ptr2 = try guard.create(f64);
    ptr2.* = 3.14;
    
    // Guard will clean up automatically
}

test "allocation guard paired allocation" {
    const allocator = std.testing.allocator;
    
    var guard = AllocationGuard.init(allocator);
    defer guard.deinit();
    
    // Test paired allocation
    const pair = try guard.createPair(i32, f64);
    pair[0].* = 42;
    pair[1].* = 3.14;
    
    try std.testing.expect(pair[0].* == 42);
    try std.testing.expect(pair[1].* == 3.14);
}

test "arena guard usage" {
    const allocator = std.testing.allocator;
    
    var arena_guard = ArenaGuard.init(allocator);
    defer arena_guard.deinit();
    
    const arena_alloc = arena_guard.allocator();
    
    // Multiple allocations in arena
    const ptr1 = try arena_alloc.create(i32);
    const ptr2 = try arena_alloc.create(f64);
    const slice = try arena_alloc.alloc(u8, 100);
    
    ptr1.* = 42;
    ptr2.* = 3.14;
    @memset(slice, 0);
    
    // All will be freed when arena_guard.deinit() is called
}
