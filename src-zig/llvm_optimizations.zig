const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;

// LLVM optimization improvements for CURSED compiler
// Focuses on better register allocation, function inlining, and dead code elimination

// Register allocation optimizer
const RegisterAllocator = struct {
    available_registers: ArrayList(u32),
    used_registers: HashMap(u32, bool, std.hash_map.DefaultContext(u32), std.hash_map.default_max_load_percentage),
    register_lifetimes: HashMap(u32, Lifetime, std.hash_map.DefaultContext(u32), std.hash_map.default_max_load_percentage),
    next_virtual_register: u32,
    allocator: Allocator,
    
    const Lifetime = struct {
        start: u32,
        end: u32,
        variable_name: []const u8,
    };
    
    const Self = @This();
    
    pub fn init(allocator: Allocator) Self {
        var available = ArrayList(u32).init(allocator);
        
        // Initialize with common x86-64 registers
        const physical_registers = [_]u32{ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 };
        for (physical_registers) |reg| {
            available.append(reg) catch {};
        }
        
        return Self{
            .available_registers = available,
            .used_registers = HashMap(u32, bool, std.hash_map.DefaultContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .register_lifetimes = HashMap(u32, Lifetime, std.hash_map.DefaultContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .next_virtual_register = 1000, // Start virtual registers at 1000
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.available_registers.deinit();
        self.used_registers.deinit();
        
        // Free variable names in lifetimes
        var iter = self.register_lifetimes.iterator();
        while (iter.next()) |entry| {
            self.allocator.free(entry.value_ptr.variable_name);
        }
        self.register_lifetimes.deinit();
    }
    
    pub fn allocateRegister(self: *Self, variable_name: []const u8, instruction_index: u32) !u32 {
        // First try to reuse a register whose lifetime has ended
        for (self.available_registers.items) |reg| {
            if (self.register_lifetimes.get(reg)) |lifetime| {
                if (lifetime.end < instruction_index) {
                    // This register is available for reuse
                    const new_lifetime = Lifetime{
                        .start = instruction_index,
                        .end = instruction_index + 100, // Default lifetime
                        .variable_name = try self.allocator.dupe(u8, variable_name),
                    };
                    try self.register_lifetimes.put(reg, new_lifetime);
                    return reg;
                }
            }
        }
        
        // If no physical register available, allocate virtual register
        const virtual_reg = self.next_virtual_register;
        self.next_virtual_register += 1;
        
        const lifetime = Lifetime{
            .start = instruction_index,
            .end = instruction_index + 100,
            .variable_name = try self.allocator.dupe(u8, variable_name),
        };
        try self.register_lifetimes.put(virtual_reg, lifetime);
        
        return virtual_reg;
    }
    
    pub fn extendLifetime(self: *Self, register: u32, end_instruction: u32) !void {
        if (self.register_lifetimes.getPtr(register)) |lifetime| {
            lifetime.end = @max(lifetime.end, end_instruction);
        }
    }
    
    pub fn generateRegisterMapping(self: *Self) ArrayList(u8) {
        var mapping = ArrayList(u8).init(self.allocator);
        
        var iter = self.register_lifetimes.iterator();
        while (iter.next()) |entry| {
            const reg = entry.key_ptr.*;
            const lifetime = entry.value_ptr.*;
            
            // Generate LLVM IR register assignment
            const assignment = std.fmt.allocPrint(self.allocator, 
                "; Register {} -> {} (lifetime: {}-{})\n", 
                .{ reg, lifetime.variable_name, lifetime.start, lifetime.end }) catch continue;
            defer self.allocator.free(assignment);
            
            mapping.appendSlice(assignment) catch {};
        }
        
        return mapping;
    }
};

// Function inlining optimizer
const FunctionInliner = struct {
    function_calls: HashMap([]const u8, CallInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    function_bodies: HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    inline_threshold: u32,
    total_function_calls: u32,
    allocator: Allocator,
    
    const CallInfo = struct {
        call_count: u32,
        estimated_cost: u32,
        body_size: u32,
        parameters: u32,
    };
    
    const Self = @This();
    
    pub fn init(allocator: Allocator, inline_threshold: u32) Self {
        return Self{
            .function_calls = HashMap([]const u8, CallInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .function_bodies = HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .inline_threshold = inline_threshold,
            .total_function_calls = 0,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Self) void {
        // Free function names and bodies
        var call_iter = self.function_calls.iterator();
        while (call_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
        }
        self.function_calls.deinit();
        
        var body_iter = self.function_bodies.iterator();
        while (body_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            self.allocator.free(entry.value_ptr.*);
        }
        self.function_bodies.deinit();
    }
    
    pub fn recordFunctionCall(self: *Self, function_name: []const u8, estimated_cost: u32) !void {
        const owned_name = try self.allocator.dupe(u8, function_name);
        
        if (self.function_calls.getPtr(owned_name)) |call_info| {
            call_info.call_count += 1;
            call_info.estimated_cost = (call_info.estimated_cost + estimated_cost) / 2; // Moving average
        } else {
            const call_info = CallInfo{
                .call_count = 1,
                .estimated_cost = estimated_cost,
                .body_size = 0,
                .parameters = 0,
            };
            try self.function_calls.put(owned_name, call_info);
        }
        
        self.total_function_calls += 1;
    }
    
    pub fn setFunctionBody(self: *Self, function_name: []const u8, body: []const u8, parameter_count: u32) !void {
        const owned_name = try self.allocator.dupe(u8, function_name);
        const owned_body = try self.allocator.dupe(u8, body);
        
        try self.function_bodies.put(owned_name, owned_body);
        
        // Update function info with body size
        if (self.function_calls.getPtr(owned_name)) |call_info| {
            call_info.body_size = @as(u32, @intCast(body.len));
            call_info.parameters = parameter_count;
        }
    }
    
    pub fn shouldInline(self: *Self, function_name: []const u8) bool {
        if (self.function_calls.get(function_name)) |call_info| {
            // Inline if:
            // 1. Function is called frequently (>= 10% of all calls)
            // 2. Function body is small (< threshold)
            // 3. Estimated performance benefit exists
            
            const call_frequency = (call_info.call_count * 100) / @max(self.total_function_calls, 1);
            const is_frequently_called = call_frequency >= 10;
            const is_small = call_info.body_size < self.inline_threshold;
            const has_few_params = call_info.parameters <= 4;
            
            return is_frequently_called and is_small and has_few_params;
        }
        
        return false;
    }
    
    pub fn generateInliningDirectives(self: *Self) !ArrayList(u8) {
        var directives = ArrayList(u8).init(self.allocator);
        
        var iter = self.function_calls.iterator();
        while (iter.next()) |entry| {
            const function_name = entry.key_ptr.*;
            const call_info = entry.value_ptr.*;
            
            if (self.shouldInline(function_name)) {
                const directive = try std.fmt.allocPrint(self.allocator,
                    "; Inline function '{}' (calls: {}, size: {}, params: {})\n",
                    .{ function_name, call_info.call_count, call_info.body_size, call_info.parameters });
                defer self.allocator.free(directive);
                
                try directives.appendSlice(directive);
                
                // Add LLVM inline attribute
                const llvm_directive = try std.fmt.allocPrint(self.allocator,
                    "define internal fastcc i64 @{}() alwaysinline {{\n",
                    .{function_name});
                defer self.allocator.free(llvm_directive);
                
                try directives.appendSlice(llvm_directive);
            }
        }
        
        return directives;
    }
};

// Dead code eliminator
const DeadCodeEliminator = struct {
    live_variables: HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    live_functions: HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    variable_uses: HashMap([]const u8, u32, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    function_calls: HashMap([]const u8, u32, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator) Self {
        return Self{
            .live_variables = HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .live_functions = HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .variable_uses = HashMap([]const u8, u32, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .function_calls = HashMap([]const u8, u32, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Self) void {
        // Free all keys
        var live_var_iter = self.live_variables.iterator();
        while (live_var_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
        }
        self.live_variables.deinit();
        
        var live_func_iter = self.live_functions.iterator();
        while (live_func_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
        }
        self.live_functions.deinit();
        
        var var_use_iter = self.variable_uses.iterator();
        while (var_use_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
        }
        self.variable_uses.deinit();
        
        var func_call_iter = self.function_calls.iterator();
        while (func_call_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
        }
        self.function_calls.deinit();
    }
    
    pub fn markVariableUse(self: *Self, variable_name: []const u8) !void {
        const owned_name = try self.allocator.dupe(u8, variable_name);
        
        if (self.variable_uses.getPtr(owned_name)) |count| {
            count.* += 1;
        } else {
            try self.variable_uses.put(owned_name, 1);
        }
        
        try self.live_variables.put(owned_name, true);
    }
    
    pub fn markFunctionCall(self: *Self, function_name: []const u8) !void {
        const owned_name = try self.allocator.dupe(u8, function_name);
        
        if (self.function_calls.getPtr(owned_name)) |count| {
            count.* += 1;
        } else {
            try self.function_calls.put(owned_name, 1);
        }
        
        try self.live_functions.put(owned_name, true);
    }
    
    pub fn isVariableLive(self: *Self, variable_name: []const u8) bool {
        return self.live_variables.get(variable_name) orelse false;
    }
    
    pub fn isFunctionLive(self: *Self, function_name: []const u8) bool {
        return self.live_functions.get(function_name) orelse false;
    }
    
    pub fn getDeadCode(self: *Self, all_variables: []const []const u8, all_functions: []const []const u8) !ArrayList([]const u8) {
        var dead_code = ArrayList([]const u8).init(self.allocator);
        
        // Find dead variables
        for (all_variables) |var_name| {
            if (!self.isVariableLive(var_name)) {
                const dead_var = try self.allocator.dupe(u8, var_name);
                try dead_code.append(dead_var);
            }
        }
        
        // Find dead functions
        for (all_functions) |func_name| {
            if (!self.isFunctionLive(func_name)) {
                const dead_func = try self.allocator.dupe(u8, func_name);
                try dead_code.append(dead_func);
            }
        }
        
        return dead_code;
    }
    
    pub fn generateOptimizationReport(self: *Self) !ArrayList(u8) {
        var report = ArrayList(u8).init(self.allocator);
        
        const header = "=== Dead Code Elimination Report ===\n";
        try report.appendSlice(header);
        
        // Variable usage statistics
        const var_header = "Variable Usage:\n";
        try report.appendSlice(var_header);
        
        var var_iter = self.variable_uses.iterator();
        while (var_iter.next()) |entry| {
            const usage_line = try std.fmt.allocPrint(self.allocator,
                "  {} -> {} uses\n",
                .{ entry.key_ptr.*, entry.value_ptr.* });
            defer self.allocator.free(usage_line);
            try report.appendSlice(usage_line);
        }
        
        // Function call statistics
        const func_header = "\nFunction Calls:\n";
        try report.appendSlice(func_header);
        
        var func_iter = self.function_calls.iterator();
        while (func_iter.next()) |entry| {
            const call_line = try std.fmt.allocPrint(self.allocator,
                "  {} -> {} calls\n",
                .{ entry.key_ptr.*, entry.value_ptr.* });
            defer self.allocator.free(call_line);
            try report.appendSlice(call_line);
        }
        
        return report;
    }
};

// LLVM optimization coordinator
pub const LLVMOptimizer = struct {
    register_allocator: RegisterAllocator,
    function_inliner: FunctionInliner,
    dead_code_eliminator: DeadCodeEliminator,
    optimization_level: u32,
    allocator: Allocator,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator, optimization_level: u32, inline_threshold: u32) Self {
        return Self{
            .register_allocator = RegisterAllocator.init(allocator),
            .function_inliner = FunctionInliner.init(allocator, inline_threshold),
            .dead_code_eliminator = DeadCodeEliminator.init(allocator),
            .optimization_level = optimization_level,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.register_allocator.deinit();
        self.function_inliner.deinit();
        self.dead_code_eliminator.deinit();
    }
    
    pub fn optimizeLLVMIR(self: *Self, llvm_ir: []const u8) ![]const u8 {
        var optimized_ir = ArrayList(u8).init(self.allocator);
        
        // Add optimization level comment
        const opt_comment = try std.fmt.allocPrint(self.allocator,
            "; CURSED LLVM Optimizations (Level: {})\n\n",
            .{self.optimization_level});
        defer self.allocator.free(opt_comment);
        try optimized_ir.appendSlice(opt_comment);
        
        // Apply register allocation optimizations
        if (self.optimization_level >= 1) {
            const reg_mapping = self.register_allocator.generateRegisterMapping();
            defer reg_mapping.deinit();
            try optimized_ir.appendSlice(reg_mapping.items);
        }
        
        // Apply function inlining
        if (self.optimization_level >= 2) {
            const inline_directives = try self.function_inliner.generateInliningDirectives();
            defer inline_directives.deinit();
            try optimized_ir.appendSlice(inline_directives.items);
        }
        
        // Apply dead code elimination comments
        if (self.optimization_level >= 3) {
            const dce_report = try self.dead_code_eliminator.generateOptimizationReport();
            defer dce_report.deinit();
            try optimized_ir.appendSlice(dce_report.items);
        }
        
        // Add original IR
        try optimized_ir.appendSlice(llvm_ir);
        
        return optimized_ir.toOwnedSlice();
    }
    
    pub fn generateOptimizationStats(self: *Self) void {
        std.debug.print("=== LLVM Optimization Statistics ===\n");
        std.debug.print("Optimization Level: {}\n", .{self.optimization_level});
        std.debug.print("Register Allocation: {} virtual registers\n", .{self.register_allocator.next_virtual_register - 1000});
        std.debug.print("Function Inlining: {} functions analyzed\n", .{self.function_inliner.function_calls.count()});
        std.debug.print("Dead Code Elimination: {} variables tracked\n", .{self.dead_code_eliminator.variable_uses.count()});
    }
};
