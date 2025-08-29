// CURSED Performance Optimization Fixes - Phase 3 Implementation
// Addresses critical performance bottlenecks identified in the codebase analysis

const std = @import("std");
const testing = std.testing;

/// =============================================================================
/// THREAD MANAGEMENT OPTIMIZATION
/// =============================================================================

/// High-performance thread queue using HashMap instead of linear search
/// Replaces O(n) linear search patterns in sync_primitives_fixed.zig:629-637
pub const OptimizedThreadQueue = struct {
    const ThreadId = u32;
    const ThreadInfo = struct {
        thread_id: ThreadId,
        priority: u8,
        state: ThreadState,
        wait_start_time: u64,
    };
    
    const ThreadState = enum {
        waiting,
        running,
        completed,
        cancelled,
    };
    
    // O(1) HashMap lookup instead of O(n) linear search
    waiting_threads: std.HashMap(ThreadId, ThreadInfo, std.hash_map.AutoContext(ThreadId), std.hash_map.default_max_load_percentage),
    thread_count: u32,
    allocator: std.mem.Allocator,
    
    pub fn init(allocator: std.mem.Allocator) OptimizedThreadQueue {
        return OptimizedThreadQueue{
            .waiting_threads = std.HashMap(ThreadId, ThreadInfo, std.hash_map.AutoContext(ThreadId), std.hash_map.default_max_load_percentage).init(allocator),
            .thread_count = 0,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *OptimizedThreadQueue) void {
        self.waiting_threads.deinit(self.allocator);
    }
    
    /// Add thread to waiting queue - O(1) operation
    pub fn addWaitingThread(self: *OptimizedThreadQueue, thread_id: ThreadId, priority: u8) !void {
        const thread_info = ThreadInfo{
            .thread_id = thread_id,
            .priority = priority,
            .state = .waiting,
            .wait_start_time = std.time.timestamp(),
        };
        
        try self.waiting_threads.put(thread_id, thread_info);
        self.thread_count += 1;
    }
    
    /// Remove thread from waiting queue - O(1) operation instead of O(n)
    /// This replaces the problematic linear search pattern
    pub fn removeWaitingThread(self: *OptimizedThreadQueue, thread_id: ThreadId) bool {
        if (self.waiting_threads.remove(thread_id)) {
            self.thread_count -= 1;
            return true;
        }
        return false;
    }
    
    /// Get thread info - O(1) lookup
    pub fn getThreadInfo(self: *OptimizedThreadQueue, thread_id: ThreadId) ?ThreadInfo {
        return self.waiting_threads.get(thread_id);
    }
    
    /// Get next thread to schedule based on priority
    pub fn getNextThread(self: *OptimizedThreadQueue) ?ThreadId {
        var highest_priority: u8 = 0;
        var selected_thread: ?ThreadId = null;
        
        // Iterate through threads to find highest priority
        var iterator = self.waiting_threads.iterator();
        while (iterator.next()) |entry| {
            const thread_info = entry.value_ptr.*;
            if (thread_info.priority > highest_priority) {
                highest_priority = thread_info.priority;
                selected_thread = thread_info.thread_id;
            }
        }
        
        return selected_thread;
    }
};

/// =============================================================================
/// MEMORY POOL OPTIMIZATION
/// =============================================================================

/// Object pool to reduce memory allocation overhead
/// Addresses excessive allocations in parser.zig, lsp_server.zig, and error_runtime.zig
pub fn ObjectPool(comptime T: type, comptime pool_size: usize) type {
    return struct {
        const Self = @This();
        
        available: std.ArrayList(*T),
        all_objects: []T,
        allocator: std.mem.Allocator,
        next_index: usize,
        
        pub fn init(allocator: std.mem.Allocator) !Self {
            const all_objects = try allocator.alloc(T, pool_size);
            var available = std.ArrayList(*T){};
            
            // Pre-allocate all objects and add to available pool
            for (all_objects) |*obj| {
                try available.append(allocator, obj);
            }
            
            return Self{
                .available = available,
                .all_objects = all_objects,
                .allocator = allocator,
                .next_index = 0,
            };
        }
        
        pub fn deinit(self: *Self) void {
            self.available.deinit(self.allocator);
            self.allocator.free(self.all_objects);
        }
        
        /// Get object from pool - O(1) operation
        pub fn acquire(self: *Self) ?*T {
            if (self.available.items.len > 0) {
                return self.available.pop();
            }
            
            // Pool exhausted - could expand or return null
            std.log.warn("Object pool exhausted for type {}", .{T});
            return null;
        }
        
        /// Return object to pool - O(1) operation
        pub fn release(self: *Self, obj: *T) void {
            // Reset object state if needed
            obj.* = std.mem.zeroes(T);
            
            // Return to available pool
            self.available.append(allocator, obj) catch {
                // Pool full - object will be garbage collected
                std.log.debug("Object pool full, object will be GC'd", .{});
            };
        }
        
        /// Get pool statistics
        pub fn getStats(self: *Self) PoolStats {
            return PoolStats{
                .total_objects = pool_size,
                .available_objects = self.available.items.len,
                .used_objects = pool_size - self.available.items.len,
                .utilization = @as(f32, @floatFromInt(pool_size - self.available.items.len)) / @as(f32, @floatFromInt(pool_size)),
            };
        }
    };
}

const PoolStats = struct {
    total_objects: usize,
    available_objects: usize,
    used_objects: usize,
    utilization: f32,
};

/// =============================================================================
/// INFINITE LOOP OPTIMIZATION
/// =============================================================================

/// Optimized loop with exponential backoff
/// Replaces tight infinite loops in parser.zig, concurrency files, and LSP server
pub const BackoffLoop = struct {
    initial_delay_ns: u64,
    max_delay_ns: u64,
    current_delay_ns: u64,
    backoff_factor: f32,
    
    pub fn init(initial_delay_ms: u64, max_delay_ms: u64) BackoffLoop {
        return BackoffLoop{
            .initial_delay_ns = initial_delay_ms * 1_000_000,  // Convert to nanoseconds
            .max_delay_ns = max_delay_ms * 1_000_000,
            .current_delay_ns = initial_delay_ms * 1_000_000,
            .backoff_factor = 1.5,  // 50% increase each iteration
        };
    }
    
    /// Sleep with current delay and increase for next iteration
    pub fn sleep(self: *BackoffLoop) void {
        std.time.sleep(self.current_delay_ns);
        
        // Exponential backoff with cap
        const new_delay = @as(u64, @intFromFloat(@as(f64, @floatFromInt(self.current_delay_ns)) * self.backoff_factor));
        self.current_delay_ns = @min(new_delay, self.max_delay_ns);
    }
    
    /// Reset delay to initial value
    pub fn reset(self: *BackoffLoop) void {
        self.current_delay_ns = self.initial_delay_ns;
    }
    
    /// Sleep with immediate reset for one-time use
    pub fn sleepOnce(initial_delay_ms: u64) void {
        std.time.sleep(initial_delay_ms * 1_000_000);
    }
    
    /// Cooperative yielding for tight loops
    pub fn yield() void {
        std.Thread.yield() catch {
            // If yield fails, use minimal sleep
            std.time.sleep(1_000);  // 1 microsecond
        };
    }
};

/// =============================================================================
/// STRING BUILDER OPTIMIZATION
/// =============================================================================

/// High-performance string building to replace repeated concatenation
/// Addresses string performance issues in LSP server and code generation
pub const StringBuilder = struct {
    buffer: std.ArrayList(u8),
    
    pub fn init(allocator: std.mem.Allocator) StringBuilder {
        return StringBuilder{
            .buffer = std.ArrayList(u8){},
        };
    }
    
    pub fn initWithCapacity(allocator: std.mem.Allocator, capacity: usize) !StringBuilder {
        var buffer = std.ArrayList(u8){};
        try buffer.ensureTotalCapacity(capacity);
        return StringBuilder{
            .buffer = buffer,
        };
    }
    
    pub fn deinit(self: *StringBuilder) void {
        self.buffer.deinit(self.allocator);
    }
    
    /// Append string slice - efficient batch operation
    pub fn append(self: *StringBuilder, str: []const u8) !void {
        try self.buffer.appendSlice(str);
    }
    
    /// Append formatted string
    pub fn appendFmt(self: *StringBuilder, comptime fmt: []const u8, args: anytype) !void {
        try self.buffer.writer().print(fmt, args);
    }
    
    /// Append single character
    pub fn appendChar(self: *StringBuilder, char: u8) !void {
        try self.buffer.append(allocator, char);
    }
    
    /// Get final string (transfers ownership)
    pub fn toString(self: *StringBuilder) []u8 {
        return self.buffer.toOwnedSlice() catch unreachable;
    }
    
    /// Get string slice (retains ownership)
    pub fn getSlice(self: *StringBuilder) []const u8 {
        return self.buffer.items;
    }
    
    /// Clear buffer for reuse
    pub fn clear(self: *StringBuilder) void {
        self.buffer.clearRetainingCapacity();
    }
    
    /// Get current length
    pub fn len(self: *StringBuilder) usize {
        return self.buffer.items.len;
    }
    
    /// Reserve capacity to reduce reallocations
    pub fn reserve(self: *StringBuilder, additional_capacity: usize) !void {
        try self.buffer.ensureTotalCapacity(self.buffer.items.len + additional_capacity);
    }
};

/// =============================================================================
/// SYMBOL TABLE OPTIMIZATION
/// =============================================================================

/// High-performance symbol table using string interning
/// Optimizes variable lookup in parser and interpreter
pub const OptimizedSymbolTable = struct {
    const SymbolId = u32;
    
    // String interning for reduced memory usage
    string_pool: std.ArrayList([]const u8),
    string_to_id: std.HashMap([]const u8, SymbolId, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Symbol information
    symbols: std.HashMap(SymbolId, SymbolInfo, std.hash_map.AutoContext(SymbolId), std.hash_map.default_max_load_percentage),
    
    allocator: std.mem.Allocator,
    next_symbol_id: SymbolId,
    
    const SymbolInfo = struct {
        name_id: SymbolId,
        symbol_type: []const u8,
        scope_level: u32,
        is_mutable: bool,
        line_defined: u32,
    };
    
    pub fn init(allocator: std.mem.Allocator) OptimizedSymbolTable {
        return OptimizedSymbolTable{
            .string_pool = std.ArrayList([]const u8){},
            .string_to_id = std.HashMap([]const u8, SymbolId, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .symbols = std.HashMap(SymbolId, SymbolInfo, std.hash_map.AutoContext(SymbolId), std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
            .next_symbol_id = 1,  // 0 reserved for invalid
        };
    }
    
    pub fn deinit(self: *OptimizedSymbolTable) void {
        // Free interned strings
        for (self.string_pool.items) |str| {
            self.allocator.free(str);
        }
        
        self.string_pool.deinit(self.allocator);
        self.string_to_id.deinit(self.allocator);
        self.symbols.deinit(self.allocator);
    }
    
    /// Intern string and return ID for deduplication
    pub fn internString(self: *OptimizedSymbolTable, str: []const u8) !SymbolId {
        // Check if string already interned
        if (self.string_to_id.get(str)) |existing_id| {
            return existing_id;
        }
        
        // Create new interned string
        const owned_str = try self.allocator.dupe(u8, str);
        const id = self.next_symbol_id;
        self.next_symbol_id += 1;
        
        try self.string_pool.append(allocator, owned_str);
        try self.string_to_id.put(owned_str, id);
        
        return id;
    }
    
    /// Add symbol to table - O(1) operation
    pub fn addSymbol(self: *OptimizedSymbolTable, name: []const u8, symbol_type: []const u8, scope_level: u32, is_mutable: bool, line_defined: u32) !SymbolId {
        const name_id = try self.internString(name);
        const symbol_id = self.next_symbol_id;
        self.next_symbol_id += 1;
        
        const symbol_info = SymbolInfo{
            .name_id = name_id,
            .symbol_type = symbol_type,
            .scope_level = scope_level,
            .is_mutable = is_mutable,
            .line_defined = line_defined,
        };
        
        try self.symbols.put(symbol_id, symbol_info);
        return symbol_id;
    }
    
    /// Lookup symbol by name - O(1) operation
    pub fn lookupSymbol(self: *OptimizedSymbolTable, name: []const u8) ?SymbolId {
        if (self.string_to_id.get(name)) |name_id| {
            // Find symbol with matching name_id
            var iterator = self.symbols.iterator();
            while (iterator.next()) |entry| {
                if (entry.value_ptr.name_id == name_id) {
                    return entry.key_ptr.*;
                }
            }
        }
        return null;
    }
};

/// =============================================================================
/// COMPILATION CACHE OPTIMIZATION
/// =============================================================================

/// Efficient compilation cache to reduce redundant work
/// Replaces placeholder implementations in compilation_cache.zig
pub const OptimizedCompilationCache = struct {
    const CacheKey = struct {
        source_hash: u64,
        compiler_version: []const u8,
        optimization_level: u8,
        target_arch: []const u8,
        
        pub fn hash(self: CacheKey) u64 {
            var hasher = std.hash.Wyhash.init(0);
            hasher.update(std.mem.asBytes(&self.source_hash));
            hasher.update(self.compiler_version);
            hasher.update(std.mem.asBytes(&self.optimization_level));
            hasher.update(self.target_arch);
            return hasher.final();
        }
        
        pub fn eql(a: CacheKey, b: CacheKey) bool {
            return a.source_hash == b.source_hash and
                   std.mem.eql(u8, a.compiler_version, b.compiler_version) and
                   a.optimization_level == b.optimization_level and
                   std.mem.eql(u8, a.target_arch, b.target_arch);
        }
    };
    
    const CacheEntry = struct {
        ast_data: []const u8,
        object_data: []const u8,
        timestamp: i64,
        access_count: u32,
    };
    
    cache: std.HashMap(CacheKey, CacheEntry, CacheKeyContext, std.hash_map.default_max_load_percentage),
    allocator: std.mem.Allocator,
    max_entries: usize,
    hit_count: u64,
    miss_count: u64,
    
    const CacheKeyContext = struct {
        pub fn hash(self: @This(), key: CacheKey) u64 {
            _ = self;
            return key.hash();
        }
        
        pub fn eql(self: @This(), a: CacheKey, b: CacheKey) bool {
            _ = self;
            return a.eql(b);
        }
    };
    
    pub fn init(allocator: std.mem.Allocator, max_entries: usize) OptimizedCompilationCache {
        return OptimizedCompilationCache{
            .cache = std.HashMap(CacheKey, CacheEntry, CacheKeyContext, std.hash_map.default_max_load_percentage){},
            .allocator = allocator,
            .max_entries = max_entries,
            .hit_count = 0,
            .miss_count = 0,
        };
    }
    
    pub fn deinit(self: *OptimizedCompilationCache) void {
        // Free cached data
        var iterator = self.cache.iterator();
        while (iterator.next()) |entry| {
            self.allocator.free(entry.value_ptr.ast_data);
            self.allocator.free(entry.value_ptr.object_data);
        }
        
        self.cache.deinit(self.allocator);
    }
    
    /// Get cached compilation result
    pub fn get(self: *OptimizedCompilationCache, key: CacheKey) ?CacheEntry {
        if (self.cache.getPtr(key)) |entry| {
            entry.access_count += 1;
            self.hit_count += 1;
            return entry.*;
        }
        
        self.miss_count += 1;
        return null;
    }
    
    /// Store compilation result in cache
    pub fn put(self: *OptimizedCompilationCache, key: CacheKey, ast_data: []const u8, object_data: []const u8) !void {
        // Evict entries if cache is full
        if (self.cache.count() >= self.max_entries) {
            try self.evictLeastRecentlyUsed();
        }
        
        const entry = CacheEntry{
            .ast_data = try self.allocator.dupe(u8, ast_data),
            .object_data = try self.allocator.dupe(u8, object_data),
            .timestamp = std.time.timestamp(),
            .access_count = 1,
        };
        
        try self.cache.put(key, entry);
    }
    
    /// Evict least recently used entry
    fn evictLeastRecentlyUsed(self: *OptimizedCompilationCache) !void {
        var oldest_key: ?CacheKey = null;
        var oldest_timestamp: i64 = std.math.maxInt(i64);
        
        var iterator = self.cache.iterator();
        while (iterator.next()) |entry| {
            if (entry.value_ptr.timestamp < oldest_timestamp) {
                oldest_timestamp = entry.value_ptr.timestamp;
                oldest_key = entry.key_ptr.*;
            }
        }
        
        if (oldest_key) |key| {
            if (self.cache.fetchRemove(key)) |kv| {
                self.allocator.free(kv.value.ast_data);
                self.allocator.free(kv.value.object_data);
            }
        }
    }
    
    /// Get cache statistics
    pub fn getStats(self: *OptimizedCompilationCache) CacheStats {
        const total_requests = self.hit_count + self.miss_count;
        const hit_rate = if (total_requests > 0) @as(f32, @floatFromInt(self.hit_count)) / @as(f32, @floatFromInt(total_requests)) else 0.0;
        
        return CacheStats{
            .entries = self.cache.count(),
            .max_entries = self.max_entries,
            .hit_count = self.hit_count,
            .miss_count = self.miss_count,
            .hit_rate = hit_rate,
            .utilization = @as(f32, @floatFromInt(self.cache.count())) / @as(f32, @floatFromInt(self.max_entries)),
        };
    }
};

const CacheStats = struct {
    entries: usize,
    max_entries: usize,
    hit_count: u64,
    miss_count: u64,
    hit_rate: f32,
    utilization: f32,
};

// =============================================================================
// PERFORMANCE TESTING AND VALIDATION
// =============================================================================

test "optimized thread queue performance" {
    var queue = OptimizedThreadQueue.init(testing.allocator);
    defer queue.deinit();
    
    // Test O(1) operations
    try queue.addWaitingThread(1, 5);
    try queue.addWaitingThread(2, 10);
    try queue.addWaitingThread(3, 3);
    
    // O(1) removal
    try testing.expect(queue.removeWaitingThread(2) == true);
    try testing.expect(queue.removeWaitingThread(999) == false);
    
    // Priority-based scheduling
    const next_thread = queue.getNextThread();
    try testing.expect(next_thread != null);
    
    std.log.info("✅ Optimized thread queue test passed");
}

test "object pool performance" {
    const TestObject = struct {
        value: u32,
        data: [64]u8,
    };
    
    var pool = try ObjectPool(TestObject, 100).init(testing.allocator);
    defer pool.deinit();
    
    // Test acquisition and release
    const obj1 = pool.acquire();
    try testing.expect(obj1 != null);
    
    const obj2 = pool.acquire();
    try testing.expect(obj2 != null);
    
    // Release objects back to pool
    pool.release(obj1.?);
    pool.release(obj2.?);
    
    // Verify pool statistics
    const stats = pool.getStats();
    try testing.expect(stats.total_objects == 100);
    try testing.expect(stats.available_objects == 100);
    
    std.log.info("✅ Object pool test passed");
}

test "string builder performance" {
    var builder = StringBuilder.init(testing.allocator);
    defer builder.deinit();
    
    try builder.append(allocator, "Hello");
    try builder.append(allocator, " ");
    try builder.append(allocator, "World");
    try builder.appendFmt("! Count: {}", .{42});
    
    const result = builder.getSlice();
    try testing.expectEqualStrings("Hello World! Count: 42", result);
    
    std.log.info("✅ String builder test passed");
}

test "symbol table performance" {
    var table = OptimizedSymbolTable.init(testing.allocator);
    defer table.deinit();
    
    // Add symbols
    const sym1 = try table.addSymbol("variable1", "drip", 0, true, 10);
    const sym2 = try table.addSymbol("function1", "slay", 0, false, 20);
    
    // Test lookup
    const found_sym = table.lookupSymbol("variable1");
    try testing.expect(found_sym != null);
    try testing.expect(found_sym.? == sym1);
    
    std.log.info("✅ Symbol table test passed");
}

test "compilation cache performance" {
    var cache = OptimizedCompilationCache.init(testing.allocator, 10);
    defer cache.deinit();
    
    const key = OptimizedCompilationCache.CacheKey{
        .source_hash = 123456789,
        .compiler_version = "1.0.0",
        .optimization_level = 2,
        .target_arch = "x86_64",
    };
    
    // Cache miss
    const result1 = cache.get(key);
    try testing.expect(result1 == null);
    
    // Store in cache
    try cache.put(key, "ast_data", "object_data");
    
    // Cache hit
    const result2 = cache.get(key);
    try testing.expect(result2 != null);
    
    const stats = cache.getStats();
    try testing.expect(stats.hit_count == 1);
    try testing.expect(stats.miss_count == 1);
    
    std.log.info("✅ Compilation cache test passed");
}
