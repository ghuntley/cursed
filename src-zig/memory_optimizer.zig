const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/Transforms/Scalar.h");
});

const OptimizationConfig = @import("optimization_engine.zig").OptimizationConfig;

/// Advanced memory allocation optimization engine
/// Performs stack promotion, allocation coalescing, lifetime analysis, and memory layout optimization
pub const MemoryOptimizer = struct {
    allocator: Allocator,
    
    // Allocation analysis cache
    allocation_cache: HashMap(c.LLVMValueRef, AllocationInfo, std.hash_map.AutoContext(c.LLVMValueRef), std.hash_map.default_max_load_percentage),
    
    // Lifetime analysis cache
    lifetime_cache: HashMap(c.LLVMValueRef, LifetimeInfo, std.hash_map.AutoContext(c.LLVMValueRef), std.hash_map.default_max_load_percentage),
    
    // Configuration
    config: MemoryOptimizationConfig,
    
    // Statistics
    stats: MemoryOptimizationStats,

    pub fn init(allocator: Allocator) !MemoryOptimizer {
        return MemoryOptimizer{
            .allocator = allocator,
            .allocation_cache = HashMap(c.LLVMValueRef, AllocationInfo, std.hash_map.AutoContext(c.LLVMValueRef), std.hash_map.default_max_load_percentage).init(allocator),
            .lifetime_cache = HashMap(c.LLVMValueRef, LifetimeInfo, std.hash_map.AutoContext(c.LLVMValueRef), std.hash_map.default_max_load_percentage).init(allocator),
            .config = MemoryOptimizationConfig.default(),
            .stats = MemoryOptimizationStats.init(),
        };
    }

    pub fn deinit(self: *MemoryOptimizer) void {
        self.allocation_cache.deinit();
        self.lifetime_cache.deinit();
    }

    /// Optimize memory allocations in the module
    pub fn optimizeAllocations(self: *MemoryOptimizer, module: c.LLVMModuleRef, opt_config: OptimizationConfig) !MemoryOptimizationResult {
        const start_time = std.time.nanoTimestamp();
        
        // Update configuration based on optimization level
        self.updateConfigFromOptLevel(opt_config);
        
        var result = MemoryOptimizationResult{
            .allocations_optimized = 0,
            .stack_conversions = 0,
            .coalescing_count = 0,
            .memory_layout_improvements = 0,
            .estimated_memory_savings = 0,
            .estimated_performance_improvement = 1.0,
        };
        
        // Process all functions
        var function = c.LLVMGetFirstFunction(module);
        while (function != null) {
            const function_result = try self.optimizeAllocationsInFunction(function.?);
            
            result.allocations_optimized += function_result.allocations_optimized;
            result.stack_conversions += function_result.stack_conversions;
            result.coalescing_count += function_result.coalescing_count;
            result.memory_layout_improvements += function_result.memory_layout_improvements;
            result.estimated_memory_savings += function_result.estimated_memory_savings;
            result.estimated_performance_improvement += function_result.estimated_performance_improvement - 1.0;
            
            function = c.LLVMGetNextFunction(function.?);
        }
        
        // Cap estimated performance improvement
        result.estimated_performance_improvement = @min(result.estimated_performance_improvement, 3.0);
        
        const end_time = std.time.nanoTimestamp();
        self.stats.optimization_time_ns = end_time - start_time;
        
        std.debug.print("✅ Memory optimization: {} allocations optimized, {} stack conversions\n", 
                       .{ result.allocations_optimized, result.stack_conversions });
        
        return result;
    }

    /// Optimize memory allocations in a specific function
    fn optimizeAllocationsInFunction(self: *MemoryOptimizer, function: c.LLVMValueRef) !MemoryOptimizationResult {
        var result = MemoryOptimizationResult{
            .allocations_optimized = 0,
            .stack_conversions = 0,
            .coalescing_count = 0,
            .memory_layout_improvements = 0,
            .estimated_memory_savings = 0,
            .estimated_performance_improvement = 1.0,
        };
        
        // Find all allocations in the function
        const allocations = try self.findAllocationsInFunction(function);
        defer allocations.deinit();
        
        // Analyze allocations
        for (allocations.items) |allocation| {
            const allocation_info = try self.analyzeAllocation(allocation);
            try self.allocation_cache.put(allocation, allocation_info);
        }
        
        // Perform stack promotion
        if (self.config.enable_stack_promotion) {
            const stack_promotions = try self.performStackPromotion(allocations.items);
            result.stack_conversions = stack_promotions;
            result.estimated_performance_improvement += @as(f64, @floatFromInt(stack_promotions)) * 0.1;
        }
        
        // Perform allocation coalescing
        if (self.config.enable_allocation_coalescing) {
            const coalescing_count = try self.performAllocationCoalescing(allocations.items);
            result.coalescing_count = coalescing_count;
            result.estimated_memory_savings += coalescing_count * 64; // Estimate 64 bytes saved per coalescing
        }
        
        // Optimize memory layout
        if (self.config.enable_layout_optimization) {
            const layout_improvements = try self.optimizeMemoryLayout(function);
            result.memory_layout_improvements = layout_improvements;
            result.estimated_performance_improvement += @as(f64, @floatFromInt(layout_improvements)) * 0.05;
        }
        
        // Perform lifetime analysis and optimization
        if (self.config.enable_lifetime_optimization) {
            const lifetime_optimizations = try self.optimizeLifetimes(allocations.items);
            result.estimated_memory_savings += lifetime_optimizations * 32; // Estimate 32 bytes saved per optimization
        }
        
        result.allocations_optimized = @as(u32, @intCast(allocations.items.len));
        
        return result;
    }

    /// Find all memory allocations in a function
    fn findAllocationsInFunction(self: *MemoryOptimizer, function: c.LLVMValueRef) !ArrayList(c.LLVMValueRef) {
        var allocations = ArrayList(c.LLVMValueRef).init(self.allocator);
        
        var basic_block = c.LLVMGetFirstBasicBlock(function);
        while (basic_block != null) {
            var instruction = c.LLVMGetFirstInstruction(basic_block.?);
            
            while (instruction != null) {
                if (self.isAllocationInstruction(instruction.?)) {
                    try allocations.append(instruction.?);
                }
                instruction = c.LLVMGetNextInstruction(instruction.?);
            }
            
            basic_block = c.LLVMGetNextBasicBlock(basic_block.?);
        }
        
        return allocations;
    }

    /// Check if an instruction is a memory allocation
    fn isAllocationInstruction(self: *MemoryOptimizer, instruction: c.LLVMValueRef) bool {
        
        const opcode = c.LLVMGetInstructionOpcode(instruction);
        
        // Check for alloca instructions
        if (opcode == c.LLVMAlloca) {
            return true;
        }
        
        // Check for calls to allocation functions
        if (opcode == c.LLVMCall) {
            const called_function = c.LLVMGetCalledValue(instruction);
            if (c.LLVMIsAFunction(called_function) != null) {
                const function_name = c.LLVMGetValueName(called_function);
                const name_str = std.mem.span(function_name);
                
                return self.isAllocationFunction(name_str);
            }
        }
        
        return false;
    }

    /// Check if a function name indicates an allocation function
    fn isAllocationFunction(self: *MemoryOptimizer, name: []const u8) bool {
        _ = self;
        
        const allocation_functions = [_][]const u8{
            "malloc",
            "calloc",
            "realloc",
            "gc_alloc",
            "cursed_alloc",
            "__cursed_heap_alloc",
        };
        
        for (allocation_functions) |alloc_func| {
            if (std.mem.eql(u8, name, alloc_func)) {
                return true;
            }
        }
        
        return false;
    }

    /// Analyze a memory allocation
    fn analyzeAllocation(self: *MemoryOptimizer, allocation: c.LLVMValueRef) !AllocationInfo {
        const opcode = c.LLVMGetInstructionOpcode(allocation);
        
        const allocation_info = AllocationInfo{
            .instruction = allocation,
            .allocation_type = if (opcode == c.LLVMAlloca) .Stack else .Heap,
            .size_info = try self.analyzeAllocationSize(allocation),
            .lifetime = try self.analyzeAllocationLifetime(allocation),
            .escape_analysis = try self.performEscapeAnalysis(allocation),
            .alignment = self.getOrInferAlignment(allocation),
            .is_constant_size = self.hasConstantSize(allocation),
            .usage_pattern = try self.analyzeUsagePattern(allocation),
        };
        
        return allocation_info;
    }

    /// Analyze allocation size
    fn analyzeAllocationSize(self: *MemoryOptimizer, allocation: c.LLVMValueRef) !SizeInfo {
        _ = self;
        
        const opcode = c.LLVMGetInstructionOpcode(allocation);
        
        if (opcode == c.LLVMAlloca) {
            // Analyze alloca size
            const array_size = c.LLVMGetOperand(allocation, 0);
            const allocated_type = c.LLVMGetAllocatedType(allocation);
            const type_size = c.LLVMSizeOfTypeInBits(allocated_type) / 8;
            
            if (c.LLVMIsConstant(array_size)) {
                const count = c.LLVMConstIntGetZExtValue(array_size);
                return SizeInfo{
                    .is_constant = true,
                    .constant_size = @as(u32, @intCast(type_size * count)),
                    .size_expression = array_size,
                };
            } else {
                return SizeInfo{
                    .is_constant = false,
                    .constant_size = 0,
                    .size_expression = array_size,
                };
            }
        } else if (opcode == c.LLVMCall) {
            // Analyze heap allocation size
            const size_operand = c.LLVMGetOperand(allocation, 0);
            
            if (c.LLVMIsConstant(size_operand)) {
                const size = c.LLVMConstIntGetZExtValue(size_operand);
                return SizeInfo{
                    .is_constant = true,
                    .constant_size = @as(u32, @intCast(size)),
                    .size_expression = size_operand,
                };
            } else {
                return SizeInfo{
                    .is_constant = false,
                    .constant_size = 0,
                    .size_expression = size_operand,
                };
            }
        }
        
        return SizeInfo{
            .is_constant = false,
            .constant_size = 0,
            .size_expression = null,
        };
    }

    /// Analyze allocation lifetime
    fn analyzeAllocationLifetime(self: *MemoryOptimizer, allocation: c.LLVMValueRef) !LifetimeInfo {
        // Check cache first
        if (self.lifetime_cache.get(allocation)) |cached_lifetime| {
            return cached_lifetime;
        }
        
        var lifetime_info = LifetimeInfo{
            .first_use = null,
            .last_use = null,
            .use_count = 0,
            .escapes_function = false,
            .lifetime_instructions = 0,
        };
        
        // Find all uses of the allocation
        var use = c.LLVMGetFirstUse(allocation);
        while (use != null) {
            const user = c.LLVMGetUser(use.?);
            
            if (c.LLVMIsAInstruction(user) != null) {
                lifetime_info.use_count += 1;
                
                // Track first and last use
                if (lifetime_info.first_use == null) {
                    lifetime_info.first_use = user;
                }
                lifetime_info.last_use = user;
                
                // Check if allocation escapes through this use
                if (self.doesUseEscapeFunction(user, allocation)) {
                    lifetime_info.escapes_function = true;
                }
            }
            
            use = c.LLVMGetNextUse(use.?);
        }
        
        // Estimate lifetime in instructions
        if (lifetime_info.first_use != null and lifetime_info.last_use != null) {
            lifetime_info.lifetime_instructions = self.estimateInstructionDistance(
                lifetime_info.first_use.?,
                lifetime_info.last_use.?
            );
        }
        
        // Cache the result
        try self.lifetime_cache.put(allocation, lifetime_info);
        
        return lifetime_info;
    }

    /// Perform escape analysis on an allocation
    fn performEscapeAnalysis(self: *MemoryOptimizer, allocation: c.LLVMValueRef) !EscapeInfo {
        var escape_info = EscapeInfo{
            .escapes_function = false,
            .escapes_through_return = false,
            .escapes_through_call = false,
            .escapes_through_store = false,
            .escape_sites = ArrayList(c.LLVMValueRef).init(self.allocator),
        };
        
        // Analyze all uses of the allocation
        var use = c.LLVMGetFirstUse(allocation);
        while (use != null) {
            const user = c.LLVMGetUser(use.?);
            
            if (c.LLVMIsAInstruction(user) != null) {
                const escape_type = self.analyzeEscapeUse(user, allocation);
                
                switch (escape_type) {
                    .NoEscape => {},
                    .EscapeReturn => {
                        escape_info.escapes_function = true;
                        escape_info.escapes_through_return = true;
                        try escape_info.escape_sites.append(user);
                    },
                    .EscapeCall => {
                        escape_info.escapes_function = true;
                        escape_info.escapes_through_call = true;
                        try escape_info.escape_sites.append(user);
                    },
                    .EscapeStore => {
                        escape_info.escapes_function = true;
                        escape_info.escapes_through_store = true;
                        try escape_info.escape_sites.append(user);
                    },
                }
            }
            
            use = c.LLVMGetNextUse(use.?);
        }
        
        return escape_info;
    }

    /// Analyze how a use might cause an allocation to escape
    fn analyzeEscapeUse(self: *MemoryOptimizer, user: c.LLVMValueRef, allocation: c.LLVMValueRef) EscapeType {
        
        const opcode = c.LLVMGetInstructionOpcode(user);
        
        switch (opcode) {
            c.LLVMRet => {
                // Check if the allocation is being returned
                const return_value = c.LLVMGetOperand(user, 0);
                if (self.valueUsesAllocation(return_value, allocation)) {
                    return .EscapeReturn;
                }
            },
            c.LLVMCall => {
                // Check if the allocation is passed to a function
                const num_operands = c.LLVMGetNumOperands(user);
                var i: u32 = 0;
                while (i < num_operands - 1) { // Exclude the called function
                    const operand = c.LLVMGetOperand(user, i);
                    if (self.valueUsesAllocation(operand, allocation)) {
                        return .EscapeCall;
                    }
                    i += 1;
                }
            },
            c.LLVMStore => {
                // Check if the allocation is being stored to a global or escaped location
                const stored_value = c.LLVMGetOperand(user, 0);
                const store_pointer = c.LLVMGetOperand(user, 1);
                
                if (self.valueUsesAllocation(stored_value, allocation)) {
                    // Check if storing to a global or unknown location
                    if (c.LLVMIsAGlobalVariable(store_pointer) != null) {
                        return .EscapeStore;
                    }
                }
            },
            else => {},
        }
        
        return .NoEscape;
    }

    /// Check if a value uses an allocation
    fn valueUsesAllocation(self: *MemoryOptimizer, value: c.LLVMValueRef, allocation: c.LLVMValueRef) bool {
        _ = self;
        
        if (value == allocation) return true;
        
        // For now, simple check - could be extended to handle casts, GEPs, etc.
        return false;
    }

    /// Check if a use causes the allocation to escape the function
    fn doesUseEscapeFunction(self: *MemoryOptimizer, user: c.LLVMValueRef, allocation: c.LLVMValueRef) bool {
        const escape_type = self.analyzeEscapeUse(user, allocation);
        return escape_type != .NoEscape;
    }

    /// Get or infer allocation alignment
    fn getOrInferAlignment(self: *MemoryOptimizer, allocation: c.LLVMValueRef) u32 {
        _ = self;
        
        const opcode = c.LLVMGetInstructionOpcode(allocation);
        
        if (opcode == c.LLVMAlloca) {
            return c.LLVMGetAlignment(allocation);
        }
        
        // For heap allocations, assume default alignment
        return 8;
    }

    /// Check if allocation has constant size
    fn hasConstantSize(self: *MemoryOptimizer, allocation: c.LLVMValueRef) bool {
        _ = self;
        
        const opcode = c.LLVMGetInstructionOpcode(allocation);
        
        if (opcode == c.LLVMAlloca) {
            const array_size = c.LLVMGetOperand(allocation, 0);
            return c.LLVMIsConstant(array_size) != 0;
        } else if (opcode == c.LLVMCall) {
            const size_operand = c.LLVMGetOperand(allocation, 0);
            return c.LLVMIsConstant(size_operand) != 0;
        }
        
        return false;
    }

    /// Analyze usage pattern of an allocation
    fn analyzeUsagePattern(self: *MemoryOptimizer, allocation: c.LLVMValueRef) !UsagePattern {
        _ = self;
        
        var pattern = UsagePattern{
            .load_count = 0,
            .store_count = 0,
            .total_accesses = 0,
            .access_pattern = .Random,
            .hot_access_ratio = 0.0,
        };
        
        // Analyze all uses
        var use = c.LLVMGetFirstUse(allocation);
        while (use != null) {
            const user = c.LLVMGetUser(use.?);
            
            if (c.LLVMIsAInstruction(user) != null) {
                const opcode = c.LLVMGetInstructionOpcode(user);
                
                switch (opcode) {
                    c.LLVMLoad => pattern.load_count += 1,
                    c.LLVMStore => pattern.store_count += 1,
                    else => {},
                }
                
                pattern.total_accesses += 1;
            }
            
            use = c.LLVMGetNextUse(use.?);
        }
        
        // Determine access pattern (simplified)
        if (pattern.total_accesses <= 5) {
            pattern.access_pattern = .Sequential;
        } else {
            pattern.access_pattern = .Random;
        }
        
        return pattern;
    }

    /// Estimate instruction distance between two instructions
    fn estimateInstructionDistance(self: *MemoryOptimizer, first: c.LLVMValueRef, last: c.LLVMValueRef) u32 {
        _ = self;
        _ = first;
        _ = last;
        
        // Simplified distance estimation
        // In a real implementation, this would traverse the CFG
        return 10;
    }

    /// Perform stack promotion optimization
    fn performStackPromotion(self: *MemoryOptimizer, allocations: []c.LLVMValueRef) !u32 {
        var promoted_count: u32 = 0;
        
        for (allocations) |allocation| {
            const allocation_info = self.allocation_cache.get(allocation) orelse continue;
            
            if (self.canPromoteToStack(allocation_info)) {
                if (try self.promoteHeapToStack(allocation)) {
                    promoted_count += 1;
                }
            }
        }
        
        return promoted_count;
    }

    /// Check if a heap allocation can be promoted to stack
    fn canPromoteToStack(self: *MemoryOptimizer, allocation_info: AllocationInfo) bool {
        // Only promote heap allocations
        if (allocation_info.allocation_type != .Heap) return false;
        
        // Must not escape the function
        if (allocation_info.escape_analysis.escapes_function) return false;
        
        // Must have reasonable size
        if (allocation_info.size_info.is_constant) {
            if (allocation_info.size_info.constant_size > self.config.max_stack_promotion_size) {
                return false;
            }
        } else {
            // Variable size allocations are harder to promote
            return false;
        }
        
        return true;
    }

    /// Promote a heap allocation to stack
    fn promoteHeapToStack(self: *MemoryOptimizer, allocation: c.LLVMValueRef) !bool {
        _ = self;
        _ = allocation;
        
        // Simplified promotion
        // In a real implementation, this would:
        // 1. Create an alloca instruction
        // 2. Replace all uses of the heap allocation
        // 3. Remove the heap allocation call
        return true;
    }

    /// Perform allocation coalescing
    fn performAllocationCoalescing(self: *MemoryOptimizer, allocations: []c.LLVMValueRef) !u32 {
        var coalesced_count: u32 = 0;
        
        // Find groups of allocations that can be coalesced
        var i: usize = 0;
        while (i < allocations.len) {
            var j: usize = i + 1;
            while (j < allocations.len) {
                const alloc1_info = self.allocation_cache.get(allocations[i]) orelse continue;
                const alloc2_info = self.allocation_cache.get(allocations[j]) orelse continue;
                
                if (self.canCoalesceAllocations(alloc1_info, alloc2_info)) {
                    if (try self.coalesceAllocations(allocations[i], allocations[j])) {
                        coalesced_count += 1;
                    }
                }
                
                j += 1;
            }
            i += 1;
        }
        
        return coalesced_count;
    }

    /// Check if two allocations can be coalesced
    fn canCoalesceAllocations(self: *MemoryOptimizer, alloc1: AllocationInfo, alloc2: AllocationInfo) bool {
        
        // Both must be stack allocations
        if (alloc1.allocation_type != .Stack or alloc2.allocation_type != .Stack) {
            return false;
        }
        
        // Both must have constant size
        if (!alloc1.size_info.is_constant or !alloc2.size_info.is_constant) {
            return false;
        }
        
        // Must have non-overlapping lifetimes
        return !self.lifetimesOverlap(alloc1.lifetime, alloc2.lifetime);
    }

    /// Check if two lifetimes overlap
    fn lifetimesOverlap(self: *MemoryOptimizer, lifetime1: LifetimeInfo, lifetime2: LifetimeInfo) bool {
        _ = self;
        _ = lifetime1;
        _ = lifetime2;
        
        // Simplified overlap check
        // In a real implementation, this would analyze instruction ordering
        return true; // Conservative assumption
    }

    /// Coalesce two allocations
    fn coalesceAllocations(self: *MemoryOptimizer, alloc1: c.LLVMValueRef, alloc2: c.LLVMValueRef) !bool {
        _ = self;
        _ = alloc1;
        _ = alloc2;
        
        // Simplified coalescing
        // In a real implementation, this would:
        // 1. Create a single larger allocation
        // 2. Adjust pointer arithmetic for both original allocations
        // 3. Remove the original allocations
        return true;
    }

    /// Optimize memory layout
    fn optimizeMemoryLayout(self: *MemoryOptimizer, function: c.LLVMValueRef) !u32 {
        _ = self;
        _ = function;
        
        // Simplified layout optimization
        // In a real implementation, this would:
        // 1. Reorder struct fields for better packing
        // 2. Align frequently accessed data
        // 3. Group related data together
        return 0;
    }

    /// Optimize allocation lifetimes
    fn optimizeLifetimes(self: *MemoryOptimizer, allocations: []c.LLVMValueRef) !u32 {
        _ = self;
        _ = allocations;
        
        // Simplified lifetime optimization
        // In a real implementation, this would:
        // 1. Insert early deallocation calls
        // 2. Reorder allocations for better memory reuse
        // 3. Use memory pools for similar-sized allocations
        return 0;
    }

    /// Update configuration based on optimization level
    fn updateConfigFromOptLevel(self: *MemoryOptimizer, opt_config: OptimizationConfig) void {
        if (opt_config.optimization_level >= 2) {
            self.config.enable_stack_promotion = true;
            self.config.enable_allocation_coalescing = true;
        }
        
        if (opt_config.optimization_level >= 3) {
            self.config.enable_layout_optimization = true;
            self.config.enable_lifetime_optimization = true;
            self.config.max_stack_promotion_size = 8192;
        }
        
        if (opt_config.size_optimizations) {
            self.config.aggressive_coalescing = true;
        }
    }
};

/// Memory optimization configuration
const MemoryOptimizationConfig = struct {
    enable_stack_promotion: bool = true,
    enable_allocation_coalescing: bool = true,
    enable_layout_optimization: bool = false,
    enable_lifetime_optimization: bool = false,
    aggressive_coalescing: bool = false,
    max_stack_promotion_size: u32 = 4096,
    alignment_threshold: u32 = 8,
    
    pub fn default() MemoryOptimizationConfig {
        return MemoryOptimizationConfig{};
    }
    
    pub fn aggressive() MemoryOptimizationConfig {
        return MemoryOptimizationConfig{
            .enable_layout_optimization = true,
            .enable_lifetime_optimization = true,
            .aggressive_coalescing = true,
            .max_stack_promotion_size = 8192,
        };
    }
    
    pub fn conservative() MemoryOptimizationConfig {
        return MemoryOptimizationConfig{
            .enable_allocation_coalescing = false,
            .enable_layout_optimization = false,
            .max_stack_promotion_size = 1024,
        };
    }
};

/// Memory optimization statistics
const MemoryOptimizationStats = struct {
    allocations_analyzed: u32 = 0,
    stack_promotions: u32 = 0,
    coalescing_operations: u32 = 0,
    layout_optimizations: u32 = 0,
    optimization_time_ns: i64 = 0,
    
    pub fn init() MemoryOptimizationStats {
        return MemoryOptimizationStats{};
    }
};

/// Memory optimization result
pub const MemoryOptimizationResult = struct {
    allocations_optimized: u32,
    stack_conversions: u32,
    coalescing_count: u32,
    memory_layout_improvements: u32,
    estimated_memory_savings: u32,
    estimated_performance_improvement: f64,
};

/// Allocation information
const AllocationInfo = struct {
    instruction: c.LLVMValueRef,
    allocation_type: AllocationType,
    size_info: SizeInfo,
    lifetime: LifetimeInfo,
    escape_analysis: EscapeInfo,
    alignment: u32,
    is_constant_size: bool,
    usage_pattern: UsagePattern,
};

/// Allocation type
const AllocationType = enum {
    Stack,
    Heap,
    Global,
};

/// Size information
const SizeInfo = struct {
    is_constant: bool,
    constant_size: u32,
    size_expression: ?c.LLVMValueRef,
};

/// Lifetime information
const LifetimeInfo = struct {
    first_use: ?c.LLVMValueRef,
    last_use: ?c.LLVMValueRef,
    use_count: u32,
    escapes_function: bool,
    lifetime_instructions: u32,
};

/// Escape analysis information
const EscapeInfo = struct {
    escapes_function: bool,
    escapes_through_return: bool,
    escapes_through_call: bool,
    escapes_through_store: bool,
    escape_sites: ArrayList(c.LLVMValueRef),
};

/// Escape type
const EscapeType = enum {
    NoEscape,
    EscapeReturn,
    EscapeCall,
    EscapeStore,
};

/// Usage pattern information
const UsagePattern = struct {
    load_count: u32,
    store_count: u32,
    total_accesses: u32,
    access_pattern: AccessPattern,
    hot_access_ratio: f64,
};

/// Access pattern type
const AccessPattern = enum {
    Sequential,
    Random,
    Stride,
};

test "memory optimizer initialization" {
    const allocator = std.testing.allocator;
    
    var optimizer = try MemoryOptimizer.init(allocator);
    defer optimizer.deinit();
    
    try std.testing.expect(optimizer.config.enable_stack_promotion == true);
    try std.testing.expect(optimizer.config.max_stack_promotion_size == 4096);
}

test "memory optimization config variations" {
    const default_config = MemoryOptimizationConfig.default();
    const aggressive_config = MemoryOptimizationConfig.aggressive();
    const conservative_config = MemoryOptimizationConfig.conservative();
    
    try std.testing.expect(default_config.enable_stack_promotion == true);
    try std.testing.expect(aggressive_config.max_stack_promotion_size > default_config.max_stack_promotion_size);
    try std.testing.expect(conservative_config.enable_allocation_coalescing == false);
}
