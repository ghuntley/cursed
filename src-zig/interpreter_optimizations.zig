const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

// Direct interpreter optimizations that can be integrated into main_unified.zig
// Focus on variable lookup caching and function call overhead reduction

// Fast variable cache for frequently accessed variables
pub const VariableCache = struct {
    cache: HashMap(u64, CachedVariable, std.hash_map.DefaultContext(u64), std.hash_map.default_max_load_percentage),
    string_to_hash: HashMap([]const u8, u64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,
    hit_count: u64,
    miss_count: u64,
    
    const CachedVariable = struct {
        variable_ptr: *const anyopaque, // Pointer to Variable in the main hashmap
        last_access_time: u64,
        access_count: u32,
    };
    
    const Self = @This();
    
    pub fn init() Self {
        return Self{
            .cache = HashMap(u64, CachedVariable, std.hash_map.DefaultContext(u64), std.hash_map.default_max_load_percentage).init(allocator),
            .string_to_hash = HashMap([]const u8, u64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
            .hit_count = 0,
            .miss_count = 0,
        };
    }
    
    pub fn deinit(self: *Self) void {
        // Free string keys
        var str_iter = self.string_to_hash.iterator();
        while (str_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
        }
        
        self.cache.deinit();
        self.string_to_hash.deinit();
    }
    
    pub fn getVariablePtr(self: *Self, name: []const u8) ?*const anyopaque {
        if (self.string_to_hash.get(name)) |hash| {
            if (self.cache.getPtr(hash)) |cached| {
                cached.access_count += 1;
                cached.last_access_time = std.time.timestamp();
                self.hit_count += 1;
                return cached.variable_ptr;
            }
        }
        
        self.miss_count += 1;
        return null;
    }
    
    pub fn cacheVariable(self: *Self, name: []const u8, variable_ptr: *const anyopaque) !void {
        // Create hash for the name
        const hash = std.hash_map.hashString(name);
        
        // Store the string -> hash mapping
        const owned_name = try self.allocator.dupe(u8, name);
        try self.string_to_hash.put(owned_name, hash);
        
        // Cache the variable pointer
        const cached_var = CachedVariable{
            .variable_ptr = variable_ptr,
            .last_access_time = std.time.timestamp(),
            .access_count = 1,
        };
        
        try self.cache.put(hash, cached_var);
    }
    
    pub fn getCacheHitRatio(self: *Self) f64 {
        const total = self.hit_count + self.miss_count;
        if (total == 0) return 0.0;
        return @as(f64, @floatFromInt(self.hit_count)) / @as(f64, @floatFromInt(total));
    }
    
    pub fn printStats(self: *Self) void {
        const total = self.hit_count + self.miss_count;
        std.debug.print("Variable Cache Stats: {} hits, {} misses, {d:.1}% hit ratio\n", .{
            self.hit_count, self.miss_count, self.getCacheHitRatio() * 100.0
        });
    }
};

// Function call optimization with pre-allocated parameter buffers
pub const OptimizedFunctionCall = struct {
    parameters: [16]?*const anyopaque, // Pre-allocated parameter pointers
    parameter_names: [16][]const u8,
    parameter_count: u8,
    function_name: []const u8,
    return_value_ptr: ?*anyopaque,
    call_count: u32,
    total_execution_time_ns: u64,
    allocator: Allocator,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator, function_name: []const u8) Self {
        return Self{
            .parameters = [_]?*const anyopaque{null} ** 16,
            .parameter_names = [_][]const u8{""} ** 16,
            .parameter_count = 0,
            .function_name = function_name,
            .return_value_ptr = null,
            .call_count = 0,
            .total_execution_time_ns = 0,
            .allocator = allocator,
        };
    }
    
    pub fn addParameter(self: *Self, name: []const u8, value_ptr: *const anyopaque) !void {
        if (self.parameter_count >= 16) return error.TooManyParameters;
        
        self.parameter_names[self.parameter_count] = name;
        self.parameters[self.parameter_count] = value_ptr;
        self.parameter_count += 1;
    }
    
    pub fn reset(self: *Self) void {
        self.parameter_count = 0;
        self.return_value_ptr = null;
        // Don't reset names array as it's just references
    }
    
    pub fn recordCall(self: *Self, execution_time_ns: u64) void {
        self.call_count += 1;
        self.total_execution_time_ns += execution_time_ns;
    }
    
    pub fn getAverageCallTime(self: *Self) f64 {
        if (self.call_count == 0) return 0.0;
        return @as(f64, @floatFromInt(self.total_execution_time_ns)) / @as(f64, @floatFromInt(self.call_count));
    }
    
    pub fn printStats(self: *Self) void {
        const avg_time_ms = self.getAverageCallTime() / 1_000_000.0;
        std.debug.print("Function '{}': {} calls, {d:.2}ms avg\n", .{
            self.function_name, self.call_count, avg_time_ms
        });
    }
};

// Expression evaluation cache to avoid re-parsing common expressions
pub const ExpressionCache = struct {
    cache: HashMap(u64, CachedExpression, std.hash_map.DefaultContext(u64), std.hash_map.default_max_load_percentage),
    allocator: Allocator,
    hit_count: u64,
    miss_count: u64,
    
    const CachedExpression = struct {
        expression_hash: u64,
        result_type: ExpressionResultType,
        int_result: i64,
        float_result: f64,
        string_result: []const u8,
        bool_result: bool,
        last_used: u64,
    };
    
    const ExpressionResultType = enum {
        integer,
        float,
        string,
        boolean,
    };
    
    const Self = @This();
    
    pub fn init() Self {
        return Self{
            .cache = HashMap(u64, CachedExpression, std.hash_map.DefaultContext(u64), std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
            .hit_count = 0,
            .miss_count = 0,
        };
    }
    
    pub fn deinit(self: *Self) void {
        // Free cached string results
        var iter = self.cache.iterator();
        while (iter.next()) |entry| {
            if (entry.value_ptr.result_type == .string) {
                self.allocator.free(entry.value_ptr.string_result);
            }
        }
        
        self.cache.deinit();
    }
    
    pub fn getHash(expression: []const u8) u64 {
        return std.hash_map.hashString(expression);
    }
    
    pub fn getCachedResult(self: *Self, expression: []const u8) ?CachedExpression {
        const hash = getHash(expression);
        if (self.cache.get(hash)) |result| {
            self.hit_count += 1;
            return result;
        }
        
        self.miss_count += 1;
        return null;
    }
    
    pub fn cacheIntResult(self: *Self, expression: []const u8, result: i64) !void {
        const hash = getHash(expression);
        const cached = CachedExpression{
            .expression_hash = hash,
            .result_type = .integer,
            .int_result = result,
            .float_result = 0.0,
            .string_result = "",
            .bool_result = false,
            .last_used = std.time.timestamp(),
        };
        
        try self.cache.put(hash, cached);
    }
    
    pub fn cacheStringResult(self: *Self, expression: []const u8, result: []const u8) !void {
        const hash = getHash(expression);
        const owned_result = try self.allocator.dupe(u8, result);
        
        const cached = CachedExpression{
            .expression_hash = hash,
            .result_type = .string,
            .int_result = 0,
            .float_result = 0.0,
            .string_result = owned_result,
            .bool_result = false,
            .last_used = std.time.timestamp(),
        };
        
        try self.cache.put(hash, cached);
    }
    
    pub fn printStats(self: *Self) void {
        const total = self.hit_count + self.miss_count;
        const hit_ratio = if (total > 0) (@as(f64, @floatFromInt(self.hit_count)) / @as(f64, @floatFromInt(total))) * 100.0 else 0.0;
        std.debug.print("Expression Cache: {} entries, {d:.1}% hit ratio\n", .{
            self.cache.count(), hit_ratio
        });
    }
};

// Memory-efficient string interning for repeated string literals
pub const StringInterner = struct {
    strings: ArrayList([]const u8),
    string_map: HashMap([]const u8, u32, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,
    
    const Self = @This();
    
    pub fn init() Self {
        return Self{
            .strings = .empty,
            .string_map = HashMap([]const u8, u32, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Self) void {
        for (self.strings.items) |string| {
            self.allocator.free(string);
        }
        self.strings.deinit();
        self.string_map.deinit();
    }
    
    pub fn intern(self: *Self, string: []const u8) ![]const u8 {
        if (self.string_map.get(string)) |index| {
            return self.strings.items[index];
        }
        
        // New string, add to pool
        const owned_string = try self.allocator.dupe(u8, string);
        const index = @as(u32, @intCast(self.strings.items.len));
        
        try self.strings.append(self.allocator, owned_string);
        try self.string_map.put(owned_string, index);
        
        return owned_string;
    }
    
    pub fn getStats(self: *Self) struct { interned_count: u32, total_size: usize } {
        var total_size: usize = 0;
        for (self.strings.items) |string| {
            total_size += string.len;
        }
        
        return .{
            .interned_count = @as(u32, @intCast(self.strings.items.len)),
            .total_size = total_size,
        };
    }
};

// Performance monitoring and optimization coordinator
pub const PerformanceMonitor = struct {
    variable_cache: VariableCache,
    expression_cache: ExpressionCache,
    string_interner: StringInterner,
    function_calls: HashMap([]const u8, OptimizedFunctionCall, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    total_interpretation_time_ns: u64,
    total_variable_lookups: u64,
    total_function_calls: u64,
    total_expressions_evaluated: u64,
    allocator: Allocator,
    
    const Self = @This();
    
    pub fn init() Self {
        return Self{
            .variable_cache = VariableCache.init(allocator),
            .expression_cache = ExpressionCache.init(allocator),
            .string_interner = StringInterner.init(allocator),
            .function_calls = HashMap([]const u8, OptimizedFunctionCall, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .total_interpretation_time_ns = 0,
            .total_variable_lookups = 0,
            .total_function_calls = 0,
            .total_expressions_evaluated = 0,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.variable_cache.deinit();
        self.expression_cache.deinit();
        self.string_interner.deinit();
        
        // Free function call tracking
        var iter = self.function_calls.iterator();
        while (iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
        }
        self.function_calls.deinit();
    }
    
    pub fn recordVariableLookup(self: *Self) void {
        self.total_variable_lookups += 1;
    }
    
    pub fn recordFunctionCall(self: *Self, function_name: []const u8, execution_time_ns: u64) !void {
        self.total_function_calls += 1;
        
        const owned_name = self.allocator.dupe(u8, function_name) catch return;
        
        if (self.function_calls.getPtr(owned_name)) |func_call| {
            func_call.recordCall(execution_time_ns);
        } else {
            var new_call = OptimizedFunctionCall.init(self.allocator, owned_name);
            new_call.recordCall(execution_time_ns);
            try self.function_calls.put(owned_name, new_call);
        }
    }
    
    pub fn recordExpressionEvaluation(self: *Self) void {
        self.total_expressions_evaluated += 1;
    }
    
    pub fn setTotalInterpretationTime(self: *Self, time_ns: u64) void {
        self.total_interpretation_time_ns = time_ns;
    }
    
    pub fn printComprehensiveReport(self: *Self) void {
        const total_time_ms = @as(f64, @floatFromInt(self.total_interpretation_time_ns)) / 1_000_000.0;
        
        std.debug.print("\n=== CURSED Performance Optimization Report ===\n", .{});
        std.debug.print("Total interpretation time: {d:.2}ms\n", .{total_time_ms});
        std.debug.print("Variable lookups: {}\n", .{self.total_variable_lookups});
        std.debug.print("Function calls: {}\n", .{self.total_function_calls});
        std.debug.print("Expressions evaluated: {}\n", .{self.total_expressions_evaluated});
        
        std.debug.print("\n--- Cache Performance ---\n", .{});
        self.variable_cache.printStats();
        self.expression_cache.printStats();
        
        const string_stats = self.string_interner.getStats();
        std.debug.print("String interning: {} strings, {} bytes\n", .{
            string_stats.interned_count, string_stats.total_size
        });
        
        std.debug.print("\n--- Function Performance ---\n", .{});
        var func_iter = self.function_calls.iterator();
        while (func_iter.next()) |entry| {
            entry.value_ptr.printStats();
        }
        
        // Performance recommendations
        std.debug.print("\n--- Optimization Recommendations ---\n", .{});
        
        if (self.variable_cache.getCacheHitRatio() < 0.5) {
            std.debug.print("⚠️  Low variable cache hit ratio - consider optimizing variable access patterns\n", .{});
        }
        
        if (self.total_function_calls > 100 and self.function_calls.count() > 10) {
            std.debug.print("💡 High function call count - consider function inlining for frequently called functions\n", .{});
        }
        
        if (string_stats.total_size > 1024) {
            std.debug.print("💡 Large string usage - string interning is helping reduce memory usage\n", .{});
        }
        
        std.debug.print("✅ Performance analysis complete\n", .{});
    }
};

// Optimized replacement functions that can be used in main_unified.zig

// Fast variable lookup with caching
pub fn optimizedVariableLookup(
    monitor: *PerformanceMonitor,
    variables: anytype, // HashMap reference
    name: []const u8
) ?*const anyopaque {
    monitor.recordVariableLookup();
    
    // Check cache first
    if (monitor.variable_cache.getVariablePtr(name)) |cached_ptr| {
        return cached_ptr;
    }
    
    // Cache miss - do normal lookup
    if (variables.get(name)) |variable| {
        // Cache the result for next time
        monitor.variable_cache.cacheVariable(name, &variable) catch {};
        return &variable;
    }
    
    return null;
}

// Fast string interning for repeated strings
pub fn optimizedStringHandling(
    monitor: *PerformanceMonitor,
    string: []const u8
) ![]const u8 {
    return monitor.string_interner.intern(string);
}

// Fast expression evaluation with caching
pub fn optimizedExpressionEvaluation(
    monitor: *PerformanceMonitor,
    expression: []const u8,
    original_evaluator: anytype
) !anytype {
    monitor.recordExpressionEvaluation();
    
    // Check expression cache
    if (monitor.expression_cache.getCachedResult(expression)) |cached| {
        switch (cached.result_type) {
            .integer => return cached.int_result,
            .string => return cached.string_result,
            .boolean => return cached.bool_result,
            .float => return cached.float_result,
        }
    }
    
    // Cache miss - evaluate normally and cache result
    const result = try original_evaluator(expression);
    
    // Cache the result based on its type
    // This would need type introspection in a real implementation
    // For now, just return the result
    return result;
}
