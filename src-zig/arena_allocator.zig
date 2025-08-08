const std = @import("std");
const builtin = @import("builtin");
const Mutex = std.Thread.Mutex;
const Atomic = std.atomic.Value;

/// Arena allocator optimized for different allocation patterns in CURSED
/// Provides fast, bulk allocation and deallocation with automatic cleanup
pub const ArenaAllocator = struct {
    /// Arena configuration for different use cases
    pub const ArenaConfig = struct {
        /// Initial buffer size
        initial_size: usize = 64 * 1024, // 64KB
        /// Growth factor when expanding
        growth_factor: f32 = 2.0,
        /// Maximum single allocation size
        max_alloc_size: usize = 1024 * 1024, // 1MB
        /// Alignment requirement
        alignment: u29 = @alignOf(u64),
        /// Enable debug tracking
        debug_tracking: bool = false,
        /// Memory pattern for initialization
        fill_pattern: ?u8 = null,
        /// Enable bounds checking in debug mode
        bounds_checking: bool = std.debug.runtime_safety,
    };

    /// Arena allocation patterns for optimization
    pub const AllocationPattern = enum {
        /// Sequential allocations, no deallocation until reset
        Sequential,
        /// Stack-like allocations (LIFO deallocation)
        Stack,
        /// Pool-like allocations (fixed-size objects)
        Pool,
        /// Temporary allocations (short-lived)
        Temporary,
        /// String interning pattern
        StringIntern,
        /// AST node allocation pattern
        ASTNodes,
        /// Runtime value allocation
        RuntimeValues,
    };

    /// Memory buffer in the arena
    const Buffer = struct {
        data: []u8,
        used: usize,
        next: ?*Buffer,
        
        fn init(allocator_param: std.mem.Allocator, size: usize) !*Buffer {
            const buffer = try allocator_param.create(Buffer);
            buffer.* = Buffer{
                .data = try allocator_param.alloc(u8, size),
                .used = 0,
                .next = null,
            };
            return buffer;
        }
        
        fn deinit(self: *Buffer, allocator_param: std.mem.Allocator) void {
            allocator_param.free(self.data);
            allocator_param.destroy(self);
        }
        
        fn remaining(self: *Buffer) usize {
            return self.data.len - self.used;
        }
        
        fn allocate(self: *Buffer, size: usize, alignment: u29) ?[]u8 {
            const aligned_offset = std.mem.alignForward(usize, self.used, alignment);
            const end_offset = aligned_offset + size;
            
            if (end_offset > self.data.len) {
                return null; // Not enough space
            }
            
            const result = self.data[aligned_offset..end_offset];
            self.used = end_offset;
            return result;
        }
    };

    /// Stack frame for stack-like allocation pattern
    const StackFrame = struct {
        buffer: *Buffer,
        saved_used: usize,
        prev_frame: ?*StackFrame,
    };

    /// Allocation tracking for debug mode
    const AllocationTrack = struct {
        ptr: usize,
        size: usize,
        alignment: u29,
        timestamp: u64,
        source_location: ?[]const u8,
    };

    /// Arena state
    backing_allocator: std.mem.Allocator,
    config: ArenaConfig,
    pattern: AllocationPattern,
    
    /// Buffer management
    first_buffer: ?*Buffer,
    current_buffer: ?*Buffer,
    total_allocated: usize,
    total_used: usize,
    
    /// Stack management for stack pattern
    current_frame: ?*StackFrame,
    frame_allocator: std.heap.FixedBufferAllocator,
    frame_buffer: [4096]u8, // 4KB for stack frames
    
    /// Pool management for pool pattern
    pool_objects: ?[][]u8,
    pool_free_list: ?[]?usize,
    pool_object_size: usize,
    
    /// Thread safety
    mutex: Mutex,
    
    /// Debug tracking
    allocations: if (std.debug.runtime_safety) std.ArrayList(AllocationTrack) else void,
    
    /// Statistics
    allocation_count: Atomic(u64),
    peak_usage: Atomic(usize),
    
    pub fn init(backing_allocator: std.mem.Allocator, config: ArenaConfig, pattern: AllocationPattern) !ArenaAllocator {
        var arena = ArenaAllocator{
            .backing_allocator = backing_allocator,
            .config = config,
            .pattern = pattern,
            .first_buffer = null,
            .current_buffer = null,
            .total_allocated = 0,
            .total_used = 0,
            .current_frame = null,
            .frame_allocator = std.heap.FixedBufferAllocator.init(&[_]u8{}),
            .frame_buffer = undefined,
            .pool_objects = null,
            .pool_free_list = null,
            .pool_object_size = 0,
            .mutex = Mutex{},
            .allocations = if (std.debug.runtime_safety) std.ArrayList(AllocationTrack).init(backing_allocator) else {},
            .allocation_count = Atomic(u64).init(0),
            .peak_usage = Atomic(usize).init(0),
        };
        
        // Initialize frame allocator for stack pattern
        arena.frame_allocator = std.heap.FixedBufferAllocator.init(&arena.frame_buffer);
        
        // Create initial buffer
        try arena.ensureCapacity(config.initial_size);
        
        return arena;
    }
    
    pub fn deinit(self: *ArenaAllocator) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        // Free all buffers
        var buffer = self.first_buffer;
        while (buffer) |buf| {
            const next = buf.next;
            buf.deinit(self.backing_allocator);
            buffer = next;
        }
        
        // Free pool objects
        if (self.pool_objects) |objects| {
            for (objects) |obj| {
                self.backing_allocator.free(obj);
            }
            self.backing_allocator.free(objects);
        }
        
        if (self.pool_free_list) |free_list| {
            self.backing_allocator.free(free_list);
        }
        
        // Clean up debug tracking
        if (std.debug.runtime_safety) {
            self.allocations.deinit();
        }
    }
    
    /// Allocate memory from the arena
    pub fn alloc(self: *ArenaAllocator, size: usize) ![]u8 {
        return self.allocAligned(size, self.config.alignment);
    }
    
    /// Allocate aligned memory from the arena
    pub fn allocAligned(self: *ArenaAllocator, size: usize, alignment: u29) ![]u8 {
        if (size > self.config.max_alloc_size) {
            return error.AllocationTooLarge;
        }
        
        self.mutex.lock();
        defer self.mutex.unlock();
        
        // Handle different allocation patterns
        switch (self.pattern) {
            .Sequential => return self.allocSequential(size, alignment),
            .Stack => return self.allocStack(size, alignment),
            .Pool => return self.allocPool(size, alignment),
            .Temporary => return self.allocTemporary(size, alignment),
            .StringIntern => return self.allocStringIntern(size, alignment),
            .ASTNodes => return self.allocASTNode(size, alignment),
            .RuntimeValues => return self.allocRuntimeValue(size, alignment),
        }
    }
    
    /// Sequential allocation pattern
    fn allocSequential(self: *ArenaAllocator, size: usize, alignment: u29) ![]u8 {
        if (self.current_buffer == null or self.current_buffer.?.remaining() < size + alignment - 1) {
            _ = try self.addBuffer(size + alignment);
        }
        
        const result = self.current_buffer.?.allocate(size, alignment) orelse {
            return error.OutOfMemory;
        };
        
        self.updateStats(size);
        self.fillPattern(result);
        self.trackAllocation(result, size, alignment);
        
        return result;
    }
    
    /// Stack allocation pattern (LIFO deallocation)
    fn allocStack(self: *ArenaAllocator, size: usize, alignment: u29) ![]u8 {
        // Ensure we have a current frame
        if (self.current_frame == null) {
            try self.pushStackFrame();
        }
        
        return self.allocSequential(size, alignment);
    }
    
    /// Pool allocation pattern (fixed-size objects)
    fn allocPool(self: *ArenaAllocator, size: usize, alignment: u29) ![]u8 {
        if (self.pool_object_size == 0) {
            self.pool_object_size = std.mem.alignForward(usize, size, alignment);
            try self.initializePool();
        }
        
        if (size > self.pool_object_size) {
            return error.ObjectTooLarge;
        }
        
        // Find free object in pool
        if (self.pool_free_list) |free_list| {
            for (free_list, 0..) |maybe_index, i| {
                if (maybe_index) |index| {
                    free_list[i] = null;
                    const result = self.pool_objects.?[index];
                    self.updateStats(self.pool_object_size);
                    self.fillPattern(result[0..size]);
                    return result[0..size];
                }
            }
        }
        
        // No free objects, allocate new one
        return self.allocSequential(self.pool_object_size, alignment);
    }
    
    /// Temporary allocation pattern (optimized for short-lived objects)
    fn allocTemporary(self: *ArenaAllocator, size: usize, alignment: u29) ![]u8 {
        // For temporary allocations, prefer smaller initial allocation
        // but allow for quick cleanup
        return self.allocSequential(size, alignment);
    }
    
    /// String interning pattern
    fn allocStringIntern(self: *ArenaAllocator, size: usize, alignment: u29) ![]u8 {
        // String interning typically allocates strings sequentially
        // and doesn't free individual strings
        return self.allocSequential(size, alignment);
    }
    
    /// AST node allocation pattern
    fn allocASTNode(self: *ArenaAllocator, size: usize, alignment: u29) ![]u8 {
        // AST nodes are typically allocated during parsing and freed together
        return self.allocSequential(size, alignment);
    }
    
    /// Runtime value allocation pattern
    fn allocRuntimeValue(self: *ArenaAllocator, size: usize, alignment: u29) ![]u8 {
        // Runtime values may need more sophisticated management
        return self.allocSequential(size, alignment);
    }
    
    /// Push a new stack frame for stack-like allocation
    pub fn pushStackFrame(self: *ArenaAllocator) !void {
        const frame = try self.frame_allocator.allocator().create(StackFrame);
        frame.* = StackFrame{
            .buffer = self.current_buffer orelse try self.addBuffer(self.config.initial_size),
            .saved_used = frame.buffer.used,
            .prev_frame = self.current_frame,
        };
        self.current_frame = frame;
    }
    
    /// Pop current stack frame and deallocate all allocations in it
    pub fn popStackFrame(self: *ArenaAllocator) void {
        if (self.current_frame) |frame| {
            // Restore buffer state
            frame.buffer.used = frame.saved_used;
            self.current_frame = frame.prev_frame;
            
            // Free the frame
            self.frame_allocator.allocator().destroy(frame);
        }
    }
    
    /// Reset the entire arena
    pub fn reset(self: *ArenaAllocator) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        // Reset all buffers
        var buffer = self.first_buffer;
        while (buffer) |buf| {
            buf.used = 0;
            buffer = buf.next;
        }
        
        // Reset state
        self.current_buffer = self.first_buffer;
        self.total_used = 0;
        self.current_frame = null;
        
        // Reset frame allocator
        self.frame_allocator.reset();
        
        // Reset pool
        if (self.pool_free_list) |free_list| {
            for (free_list, 0..) |_, i| {
                free_list[i] = i;
            }
        }
        
        // Clear debug tracking
        if (std.debug.runtime_safety) {
            self.allocations.clearRetainingCapacity();
        }
    }
    
    /// Free a specific allocation (only supported for pool pattern)
    pub fn free(self: *ArenaAllocator, ptr: []u8) void {
        if (self.pattern != .Pool) {
            return; // Free not supported for other patterns
        }
        
        self.mutex.lock();
        defer self.mutex.unlock();
        
        // Find the object in pool and add to free list
        if (self.pool_objects) |objects| {
            for (objects, 0..) |obj, i| {
                if (ptr.ptr == obj.ptr) {
                    if (self.pool_free_list) |free_list| {
                        // Find empty slot in free list
                        for (free_list, 0..) |maybe_index, j| {
                            if (maybe_index == null) {
                                free_list[j] = i;
                                return;
                            }
                        }
                    }
                    return;
                }
            }
        }
    }
    
    /// Ensure arena has at least the specified capacity
    fn ensureCapacity(self: *ArenaAllocator, min_size: usize) !void {
        if (self.first_buffer == null) {
            const size = @max(min_size, self.config.initial_size);
            self.first_buffer = try Buffer.init(self.backing_allocator, size);
            self.current_buffer = self.first_buffer;
            self.total_allocated = size;
        }
    }
    
    /// Add a new buffer to the arena
    fn addBuffer(self: *ArenaAllocator, min_size: usize) !*Buffer {
        const growth_size = @as(usize, @intFromFloat(@as(f32, @floatFromInt(self.total_allocated)) * self.config.growth_factor));
        const new_size = @max(min_size, growth_size);
        
        const new_buffer = try Buffer.init(self.backing_allocator, new_size);
        
        if (self.current_buffer) |current| {
            current.next = new_buffer;
        } else {
            self.first_buffer = new_buffer;
        }
        
        self.current_buffer = new_buffer;
        self.total_allocated += new_size;
        
        return new_buffer;
    }
    
    /// Initialize pool for pool allocation pattern
    fn initializePool(self: *ArenaAllocator) !void {
        const pool_size = 1024; // Start with 1024 objects
        
        self.pool_objects = try self.backing_allocator.alloc([]u8, pool_size);
        self.pool_free_list = try self.backing_allocator.alloc(?usize, pool_size);
        
        // Initialize free list
        for (self.pool_free_list.?, 0..) |*slot, i| {
            slot.* = i;
        }
        
        // Pre-allocate pool objects
        for (self.pool_objects.?) |*obj| {
            obj.* = try self.backing_allocator.alloc(u8, self.pool_object_size);
        }
    }
    
    /// Update statistics
    fn updateStats(self: *ArenaAllocator, size: usize) void {
        _ = self.allocation_count.fetchAdd(1, .release);
        
        self.total_used += size;
        var current_peak = self.peak_usage.load(.acquire);
        while (self.total_used > current_peak) {
            const old_peak = self.peak_usage.cmpxchgWeak(current_peak, self.total_used, .acq_rel, .acquire) orelse break;
            current_peak = old_peak;
        }
    }
    
    /// Fill allocation with pattern if configured
    fn fillPattern(self: *ArenaAllocator, ptr: []u8) void {
        if (self.config.fill_pattern) |pattern| {
            @memset(ptr, pattern);
        }
    }
    
    /// Track allocation for debugging
    fn trackAllocation(self: *ArenaAllocator, ptr: []u8, size: usize, alignment: u29) void {
        if (std.debug.runtime_safety and self.config.debug_tracking) {
            const track = AllocationTrack{
                .ptr = @intFromPtr(ptr.ptr),
                .size = size,
                .alignment = alignment,
                .timestamp = @as(u64, @intCast(std.time.microTimestamp())),
                .source_location = null, // Could be filled by caller
            };
            self.allocations.append(track) catch {};
        }
    }
    
    /// Get current usage statistics
    pub fn getUsage(self: *ArenaAllocator) struct {
        total_allocated: usize,
        total_used: usize,
        allocation_count: u64,
        peak_usage: usize,
        fragmentation: f32,
    } {
        return .{
            .total_allocated = self.total_allocated,
            .total_used = self.total_used,
            .allocation_count = self.allocation_count.load(.acquire),
            .peak_usage = self.peak_usage.load(.acquire),
            .fragmentation = if (self.total_allocated > 0) 
                1.0 - (@as(f32, @floatFromInt(self.total_used)) / @as(f32, @floatFromInt(self.total_allocated)))
                else 0.0,
        };
    }
    
    /// Create an Allocator interface
    pub fn allocator(self: *ArenaAllocator) std.mem.Allocator {
        return std.mem.Allocator{
            .ptr = self,
            .vtable = &.{
                .alloc = allocFn,
                .resize = resizeFn,
                .free = freeFn,
                .remap = std.mem.Allocator.noRemap,
            },
        };
    }
    
    fn allocFn(ptr: *anyopaque, len: usize, alignment: std.mem.Alignment, _: usize) ?[*]u8 {
        const self: *ArenaAllocator = @ptrCast(@alignCast(ptr));
        const alignment_value = @as(u29, @intCast(alignment.toByteUnits()));
        const slice = self.allocAligned(len, alignment_value) catch return null;
        return slice.ptr;
    }
    
    fn resizeFn(_: *anyopaque, _: []u8, _: std.mem.Alignment, _: usize, _: usize) bool {
        return false; // Arena allocator doesn't support resize
    }
    
    fn freeFn(ptr: *anyopaque, buf: []u8, _: std.mem.Alignment, _: usize) void {
        const self: *ArenaAllocator = @ptrCast(@alignCast(ptr));
        self.free(buf);
    }
};

/// Specialized arena for different CURSED runtime patterns
pub const CursedArenaManager = struct {
    /// Specialized arenas for different allocation patterns
    parser_arena: ArenaAllocator,
    ast_arena: ArenaAllocator,
    runtime_arena: ArenaAllocator,
    string_arena: ArenaAllocator,
    temporary_arena: ArenaAllocator,
    
    backing_allocator: std.mem.Allocator,
    
    pub fn init(backing_allocator: std.mem.Allocator) !CursedArenaManager {
        return CursedArenaManager{
            .parser_arena = try ArenaAllocator.init(
                backing_allocator,
                .{ .initial_size = 256 * 1024, .fill_pattern = 0 }, // 256KB for parser
                .Sequential
            ),
            .ast_arena = try ArenaAllocator.init(
                backing_allocator,
                .{ .initial_size = 512 * 1024, .debug_tracking = true }, // 512KB for AST
                .ASTNodes
            ),
            .runtime_arena = try ArenaAllocator.init(
                backing_allocator,
                .{ .initial_size = 1024 * 1024, .growth_factor = 1.5 }, // 1MB for runtime
                .Stack
            ),
            .string_arena = try ArenaAllocator.init(
                backing_allocator,
                .{ .initial_size = 128 * 1024, .max_alloc_size = 64 * 1024 }, // 128KB for strings
                .StringIntern
            ),
            .temporary_arena = try ArenaAllocator.init(
                backing_allocator,
                .{ .initial_size = 64 * 1024, .fill_pattern = 0xCC }, // 64KB for temporary
                .Temporary
            ),
            .backing_allocator = backing_allocator,
        };
    }
    
    pub fn deinit(self: *CursedArenaManager) void {
        self.parser_arena.deinit();
        self.ast_arena.deinit();
        self.runtime_arena.deinit();
        self.string_arena.deinit();
        self.temporary_arena.deinit();
    }
    
    /// Reset all arenas (typically done between compilation units)
    pub fn resetAll(self: *CursedArenaManager) void {
        self.parser_arena.reset();
        self.ast_arena.reset();
        self.runtime_arena.reset();
        self.string_arena.reset();
        self.temporary_arena.reset();
    }
    
    /// Reset only temporary arenas (done frequently during execution)
    pub fn resetTemporary(self: *CursedArenaManager) void {
        self.temporary_arena.reset();
    }
    
    /// Get allocator for specific use case
    pub fn getParserAllocator(self: *CursedArenaManager) std.mem.Allocator {
        return self.parser_arena.allocator();
    }
    
    pub fn getASTAllocator(self: *CursedArenaManager) std.mem.Allocator {
        return self.ast_arena.allocator();
    }
    
    pub fn getRuntimeAllocator(self: *CursedArenaManager) std.mem.Allocator {
        return self.runtime_arena.allocator();
    }
    
    pub fn getStringAllocator(self: *CursedArenaManager) std.mem.Allocator {
        return self.string_arena.allocator();
    }
    
    pub fn getTemporaryAllocator(self: *CursedArenaManager) std.mem.Allocator {
        return self.temporary_arena.allocator();
    }
    
    /// Get combined usage statistics
    pub fn getTotalUsage(self: *CursedArenaManager) struct {
        parser: @TypeOf(self.parser_arena.getUsage()),
        ast: @TypeOf(self.ast_arena.getUsage()),
        runtime: @TypeOf(self.runtime_arena.getUsage()),
        string: @TypeOf(self.string_arena.getUsage()),
        temporary: @TypeOf(self.temporary_arena.getUsage()),
        total_allocated: usize,
        total_used: usize,
    } {
        const parser_usage = self.parser_arena.getUsage();
        const ast_usage = self.ast_arena.getUsage();
        const runtime_usage = self.runtime_arena.getUsage();
        const string_usage = self.string_arena.getUsage();
        const temporary_usage = self.temporary_arena.getUsage();
        
        return .{
            .parser = parser_usage,
            .ast = ast_usage,
            .runtime = runtime_usage,
            .string = string_usage,
            .temporary = temporary_usage,
            .total_allocated = parser_usage.total_allocated + ast_usage.total_allocated + 
                              runtime_usage.total_allocated + string_usage.total_allocated + 
                              temporary_usage.total_allocated,
            .total_used = parser_usage.total_used + ast_usage.total_used + 
                         runtime_usage.total_used + string_usage.total_used + 
                         temporary_usage.total_used,
        };
    }
};

// Export C API for LLVM integration
export fn cursed_arena_create_manager() ?*CursedArenaManager {
    const allocator = std.heap.page_allocator;
    const manager = allocator.create(CursedArenaManager) catch return null;
    manager.* = CursedArenaManager.init(allocator) catch {
        allocator.destroy(manager);
        return null;
    };
    return manager;
}

export fn cursed_arena_destroy_manager(manager: ?*CursedArenaManager) void {
    if (manager) |m| {
        m.deinit();
        std.heap.page_allocator.destroy(m);
    }
}

export fn cursed_arena_alloc_parser(manager: ?*CursedArenaManager, size: usize) ?*anyopaque {
    if (manager) |m| {
        const slice = m.parser_arena.alloc(size) catch return null;
        return slice.ptr;
    }
    return null;
}

export fn cursed_arena_alloc_ast(manager: ?*CursedArenaManager, size: usize) ?*anyopaque {
    if (manager) |m| {
        const slice = m.ast_arena.alloc(size) catch return null;
        return slice.ptr;
    }
    return null;
}

export fn cursed_arena_alloc_runtime(manager: ?*CursedArenaManager, size: usize) ?*anyopaque {
    if (manager) |m| {
        const slice = m.runtime_arena.alloc(size) catch return null;
        return slice.ptr;
    }
    return null;
}

export fn cursed_arena_alloc_string(manager: ?*CursedArenaManager, size: usize) ?*anyopaque {
    if (manager) |m| {
        const slice = m.string_arena.alloc(size) catch return null;
        return slice.ptr;
    }
    return null;
}

export fn cursed_arena_alloc_temporary(manager: ?*CursedArenaManager, size: usize) ?*anyopaque {
    if (manager) |m| {
        const slice = m.temporary_arena.alloc(size) catch return null;
        return slice.ptr;
    }
    return null;
}

export fn cursed_arena_reset_all(manager: ?*CursedArenaManager) void {
    if (manager) |m| {
        m.resetAll();
    }
}

export fn cursed_arena_reset_temporary(manager: ?*CursedArenaManager) void {
    if (manager) |m| {
        m.resetTemporary();
    }
}

export fn cursed_arena_push_stack_frame(manager: ?*CursedArenaManager) void {
    if (manager) |m| {
        m.runtime_arena.pushStackFrame() catch {};
    }
}

export fn cursed_arena_pop_stack_frame(manager: ?*CursedArenaManager) void {
    if (manager) |m| {
        m.runtime_arena.popStackFrame();
    }
}
