const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

// Performance optimization module for CURSED interpreter and compiler
// Focuses on function call overhead reduction, variable lookup optimization, and memory allocation improvements

// Cache for frequently accessed variables to reduce HashMap lookups
const VariableCache = struct {
    cache: HashMap(u32, *Variable, std.hash_map.DefaultContext(u32), std.hash_map.default_max_load_percentage),
    hash_cache: HashMap([]const u8, u32, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    hit_count: u64,
    miss_count: u64,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator) Self {
        return Self{
            .cache = HashMap(u32, *Variable, std.hash_map.DefaultContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .hash_cache = HashMap([]const u8, u32, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .hit_count = 0,
            .miss_count = 0,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.cache.deinit();
        self.hash_cache.deinit();
    }
    
    pub fn getVariable(self: *Self, name: []const u8) ?*Variable {
        // Fast path: check if we have the hash cached
        if (self.hash_cache.get(name)) |hash| {
            if (self.cache.get(hash)) |variable| {
                self.hit_count += 1;
                return variable;
            }
        }
        
        self.miss_count += 1;
        return null;
    }
    
    pub fn putVariable(self: *Self, allocator: Allocator, name: []const u8, variable: *Variable) !void {
        const hash = std.hash_map.hashString(name);
        try self.cache.put(hash, variable);
        
        // Store string copy for future lookups
        const name_copy = try allocator.dupe(u8, name);
        try self.hash_cache.put(name_copy, hash);
    }
    
    pub fn getCacheEfficiency(self: *Self) f64 {
        const total = self.hit_count + self.miss_count;
        if (total == 0) return 0.0;
        return @as(f64, @floatFromInt(self.hit_count)) / @as(f64, @floatFromInt(total));
    }
};

// Pre-allocated function call context to reduce allocation overhead
const OptimizedFunctionContext = struct {
    parameters: [16]Variable,  // Pre-allocated space for up to 16 parameters
    parameter_count: u32,
    return_value: ?Variable,
    local_variables: ArrayList(Variable),
    allocator: Allocator,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator) Self {
        return Self{
            .parameters = undefined,
            .parameter_count = 0,
            .return_value = null,
            .local_variables = ArrayList(Variable).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Self) void {
        for (self.parameters[0..self.parameter_count]) |*param| {
            param.deinit(self.allocator);
        }
        
        if (self.return_value) |*ret_val| {
            ret_val.deinit(self.allocator);
        }
        
        for (self.local_variables.items) |*local_var| {
            local_var.deinit(self.allocator);
        }
        self.local_variables.deinit();
    }
    
    pub fn reset(self: *Self) void {
        for (self.parameters[0..self.parameter_count]) |*param| {
            param.deinit(self.allocator);
        }
        
        if (self.return_value) |*ret_val| {
            ret_val.deinit(self.allocator);
            self.return_value = null;
        }
        
        for (self.local_variables.items) |*local_var| {
            local_var.deinit(self.allocator);
        }
        self.local_variables.clearRetainingCapacity();
        self.parameter_count = 0;
    }
    
    pub fn addParameter(self: *Self, param: Variable) !void {
        if (self.parameter_count >= 16) return error.TooManyParameters;
        self.parameters[self.parameter_count] = param;
        self.parameter_count += 1;
    }
};

// Fast string interning for variable names to reduce allocations
const StringInterner = struct {
    strings: ArrayList([]const u8),
    string_map: HashMap([]const u8, u32, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator) Self {
        return Self{
            .strings = ArrayList([]const u8).init(allocator),
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
        
        // New string, intern it
        const owned_string = try self.allocator.dupe(u8, string);
        const index = @as(u32, @intCast(self.strings.items.len));
        try self.strings.append(owned_string);
        try self.string_map.put(owned_string, index);
        
        return owned_string;
    }
};

// Optimized variable scope with faster lookups
const OptimizedScope = struct {
    variables: HashMap([]const u8, Variable, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    parent: ?*OptimizedScope,
    cache: VariableCache,
    interner: *StringInterner,
    depth: u32,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator, parent: ?*OptimizedScope, interner: *StringInterner) Self {
        const depth = if (parent) |p| p.depth + 1 else 0;
        return Self{
            .variables = HashMap([]const u8, Variable, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .parent = parent,
            .cache = VariableCache.init(allocator),
            .interner = interner,
            .depth = depth,
        };
    }
    
    pub fn deinit(self: *Self, allocator: Allocator) void {
        var iter = self.variables.iterator();
        while (iter.next()) |entry| {
            entry.value_ptr.deinit(allocator);
        }
        self.variables.deinit();
        self.cache.deinit();
    }
    
    pub fn getVariable(self: *Self, name: []const u8) ?Variable {
        // Fast path: check cache first
        if (self.cache.getVariable(name)) |cached_var| {
            return cached_var.*;
        }
        
        // Slow path: traverse scope chain
        var current_scope: ?*OptimizedScope = self;
        while (current_scope) |scope| {
            if (scope.variables.get(name)) |variable| {
                // Cache the result for faster future lookups
                self.cache.putVariable(self.variables.allocator, name, @constCast(&variable)) catch {};
                return variable;
            }
            current_scope = scope.parent;
        }
        
        return null;
    }
    
    pub fn setVariable(self: *Self, allocator: Allocator, name: []const u8, value: Variable) !void {
        const interned_name = try self.interner.intern(name);
        try self.variables.put(interned_name, value);
        
        // Update cache
        var owned_value = value;
        try self.cache.putVariable(allocator, interned_name, &owned_value);
    }
};

// Performance statistics tracker
const PerformanceStats = struct {
    function_calls: u64,
    variable_lookups: u64,
    cache_hits: u64,
    cache_misses: u64,
    allocation_count: u64,
    allocation_bytes: u64,
    execution_time_ns: u64,
    
    const Self = @This();
    
    pub fn init() Self {
        return Self{
            .function_calls = 0,
            .variable_lookups = 0,
            .cache_hits = 0,
            .cache_misses = 0,
            .allocation_count = 0,
            .allocation_bytes = 0,
            .execution_time_ns = 0,
        };
    }
    
    pub fn recordFunctionCall(self: *Self) void {
        self.function_calls += 1;
    }
    
    pub fn recordVariableLookup(self: *Self, cache_hit: bool) void {
        self.variable_lookups += 1;
        if (cache_hit) {
            self.cache_hits += 1;
        } else {
            self.cache_misses += 1;
        }
    }
    
    pub fn recordAllocation(self: *Self, bytes: usize) void {
        self.allocation_count += 1;
        self.allocation_bytes += bytes;
    }
    
    pub fn getCacheHitRatio(self: *Self) f64 {
        if (self.variable_lookups == 0) return 0.0;
        return @as(f64, @floatFromInt(self.cache_hits)) / @as(f64, @floatFromInt(self.variable_lookups));
    }
    
    pub fn print(self: *Self) void {
        std.debug.print("Performance Statistics:\n");
        std.debug.print("  Function calls: {}\n", .{self.function_calls});
        std.debug.print("  Variable lookups: {}\n", .{self.variable_lookups});
        std.debug.print("  Cache hit ratio: {d:.2}%\n", .{self.getCacheHitRatio() * 100.0});
        std.debug.print("  Allocations: {} ({} bytes)\n", .{ self.allocation_count, self.allocation_bytes });
        std.debug.print("  Execution time: {d:.2}ms\n", .{@as(f64, @floatFromInt(self.execution_time_ns)) / 1_000_000.0});
    }
};

// Import the Variable type from main_unified.zig
const Variable = @import("main_unified.zig").Variable;

// Fast function lookup table for built-in functions
const BuiltinFunctionTable = struct {
    functions: [64]?BuiltinFunction,
    name_to_index: HashMap([]const u8, u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    const BuiltinFunction = struct {
        name: []const u8,
        param_count: u8,
        func: *const fn ([]Variable, Allocator) Variable,
    };
    
    const Self = @This();
    
    pub fn init(allocator: Allocator) Self {
        return Self{
            .functions = [_]?BuiltinFunction{null} ** 64,
            .name_to_index = HashMap([]const u8, u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.name_to_index.deinit();
    }
    
    pub fn registerFunction(self: *Self, name: []const u8, param_count: u8, func: *const fn ([]Variable, Allocator) Variable) !void {
        // Find empty slot
        for (self.functions, 0..) |maybe_func, i| {
            if (maybe_func == null) {
                self.functions[i] = BuiltinFunction{
                    .name = name,
                    .param_count = param_count,
                    .func = func,
                };
                try self.name_to_index.put(name, @as(u8, @intCast(i)));
                return;
            }
        }
        return error.TooManyBuiltinFunctions;
    }
    
    pub fn callFunction(self: *Self, name: []const u8, params: []Variable, allocator: Allocator) ?Variable {
        if (self.name_to_index.get(name)) |index| {
            if (self.functions[index]) |builtin_func| {
                if (params.len == builtin_func.param_count) {
                    return builtin_func.func(params, allocator);
                }
            }
        }
        return null;
    }
};
