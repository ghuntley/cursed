//! Enhanced Generic Monomorphization for CURSED
//! 
//! This module provides an advanced monomorphization system that integrates
//! with the existing type system and code generation to produce efficient,
//! specialized versions of generic functions and types.

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

// Mock LLVM types for compilation without libc dependency
const c = struct {
    pub const LLVMContextRef = *anyopaque;
    pub const LLVMModuleRef = *anyopaque;
    pub const LLVMBuilderRef = *anyopaque;
    pub const LLVMValueRef = *anyopaque;
    pub const LLVMTypeRef = *anyopaque;
    
    // Mock function declarations
    pub fn LLVMCreateFunctionPassManagerForModule(module: LLVMModuleRef) *anyopaque {
        return module;
    }
    pub fn LLVMDisposeFunctionPassManager(fpm: *anyopaque) void { _ = fpm; }
    pub fn LLVMAddInstructionCombiningPass(fpm: *anyopaque) void { _ = fpm; }
    pub fn LLVMAddReassociatePass(fpm: *anyopaque) void { _ = fpm; }
    pub fn LLVMAddGVNPass(fpm: *anyopaque) void { _ = fpm; }
    pub fn LLVMAddCFGSimplificationPass(fpm: *anyopaque) void { _ = fpm; }
    pub fn LLVMAddAggressiveDCEPass(fpm: *anyopaque) void { _ = fpm; }
    pub fn LLVMAddMemCpyOptPass(fpm: *anyopaque) void { _ = fpm; }
    pub fn LLVMAddLoopUnrollPass(fpm: *anyopaque) void { _ = fpm; }
    pub fn LLVMAddInlinePass(fpm: *anyopaque, threshold: u32) void { _ = fpm; _ = threshold; }
    pub fn LLVMAddTailCallEliminationPass(fpm: *anyopaque) void { _ = fpm; }
    pub fn LLVMInitializeFunctionPassManager(fpm: *anyopaque) void { _ = fpm; }
    pub fn LLVMRunFunctionPassManager(fpm: *anyopaque, func: LLVMValueRef) u32 { _ = fpm; _ = func; return 1; }
    pub fn LLVMFinalizeFunctionPassManager(fpm: *anyopaque) void { _ = fpm; }
    pub fn LLVMReplaceAllUsesWith(old: LLVMValueRef, new: LLVMValueRef) void { _ = old; _ = new; }
};

const ast = @import("ast.zig");
const generics = @import("generics.zig");
const type_system = @import("type_system_runtime.zig");
const type_inference = @import("type_inference.zig");
const error_handling = @import("error_handling.zig");
const CursedError = error_handling.CursedError;

/// Enhanced monomorphization engine that provides:
/// - Intelligent type parameter inference
/// - Dependency-aware instantiation ordering
/// - Optimization opportunities detection
/// - Cache-aware compilation
/// - Integration with LLVM optimization passes
pub const EnhancedMonomorphizer = struct {
    allocator: Allocator,
    base_monomorphizer: *generics.Monomorphizer,
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    
    // Enhanced state management
    instantiation_graph: InstantiationGraph,
    optimization_cache: OptimizationCache,
    specialization_metrics: SpecializationMetrics,
    type_inference_ctx: type_inference.TypeInferenceContext,
    
    // Advanced caching
    monomorphization_cache: HashMap(MonomorphizationKey, CachedInstantiation, MonomorphizationKeyContext, std.hash_map.default_max_load_percentage),
    dependency_tracker: DependencyTracker,
    
    pub const InstantiationGraph = struct {
        nodes: HashMap([]const u8, InstantiationNode, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        edges: ArrayList(InstantiationEdge),
        
        const InstantiationNode = struct {
            name: []const u8,
            kind: NodeKind,
            type_arguments: []ast.Type,
            status: InstantiationStatus,
            depends_on: ArrayList([]const u8),
            dependents: ArrayList([]const u8),
            priority: u32,
            
            const NodeKind = enum {
                Function,
                Struct,
                Interface,
                Alias,
            };
            
            const InstantiationStatus = enum {
                Pending,
                InProgress,
                Complete,
                Failed,
            };
        };
        
        const InstantiationEdge = struct {
            from: []const u8,
            to: []const u8,
            weight: u32, // Higher weight = stronger dependency
        };
        
        pub fn init(allocator: Allocator) InstantiationGraph {
            return InstantiationGraph{
                .nodes = HashMap([]const u8, InstantiationNode, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
                .edges = .empty,
            };
        }
        
        pub fn deinit(self: *InstantiationGraph) void {
            var node_iter = self.nodes.iterator();
            while (node_iter.next()) |entry| {
                entry.value_ptr.depends_on.deinit(allocator);
                entry.value_ptr.dependents.deinit(allocator);
            }
            self.nodes.deinit(allocator);
            self.edges.deinit(allocator);
        }
        
        /// Add dependency relationship between two instantiations
        pub fn addDependency(self: *InstantiationGraph, dependent: []const u8, dependency: []const u8, weight: u32) !void {
            // Add edge
            try self.edges.append(allocator, InstantiationEdge{
                .from = dependent,
                .to = dependency,
                .weight = weight,
            });
            
            // Update node dependency lists
            if (self.nodes.getPtr(dependent)) |dependent_node| {
                try dependent_node.depends_on.append(allocator, dependency);
            }
            
            if (self.nodes.getPtr(dependency)) |dependency_node| {
                try dependency_node.dependents.append(allocator, dependent);
            }
        }
        
        /// Perform topological sort to determine instantiation order
        pub fn getInstantiationOrder(self: *InstantiationGraph, allocator: Allocator) ![][]const u8 {
            var sorted = .empty;
            var visited = HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
            defer visited.deinit(allocator);
            
            var temp_visited = HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
            defer temp_visited.deinit(allocator);
            
            // Initialize all nodes as unvisited
            var node_iter = self.nodes.iterator();
            while (node_iter.next()) |entry| {
                try visited.put(entry.key_ptr.*, false);
                try temp_visited.put(entry.key_ptr.*, false);
            }
            
            // Perform DFS for each unvisited node
            node_iter = self.nodes.iterator();
            while (node_iter.next()) |entry| {
                const node_name = entry.key_ptr.*;
                if (!visited.get(node_name).?) {
                    try self.topologicalSortUtil(node_name, &visited, &temp_visited, &sorted);
                }
            }
            
            // Reverse to get correct order (dependencies first)
            std.mem.reverse([]const u8, sorted.items);
            
            return sorted.toOwnedSlice(allocator);
        }
        
        fn topologicalSortUtil(
            self: *InstantiationGraph,
            node_name: []const u8,
            visited: *HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
            temp_visited: *HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
            sorted: *ArrayList([]const u8)
        ) !void {
            try temp_visited.put(node_name, true);
            
            const node = self.nodes.get(node_name) orelse return;
            
            // Visit all dependencies first
            for (node.depends_on.items) |dependency| {
                if (temp_visited.get(dependency).?) {
                    // Cycle detected
                    std.log.err("Circular dependency detected: {s} -> {s}", .{node_name, dependency});
                    return error.CircularDependency;
                }
                
                if (!visited.get(dependency).?) {
                    try self.topologicalSortUtil(dependency, visited, temp_visited, sorted);
                }
            }
            
            try temp_visited.put(node_name, false);
            try visited.put(node_name, true);
            try sorted.append(allocator, node_name);
        }
    };
    
    const OptimizationCache = struct {
        cached_optimizations: HashMap(OptimizationKey, OptimizationResult, OptimizationKeyContext, std.hash_map.default_max_load_percentage),
        optimization_stats: OptimizationStats,
        
        const OptimizationKey = struct {
            function_hash: u64,
            optimization_level: u8,
            target_features: u64, // Hash of target feature set
        };
        
        const OptimizationKeyContext = struct {
            pub fn hash(self: @This(), key: OptimizationKey) u64 {
                _ = self;
                var hasher = std.hash.Wyhash.init(0);
                hasher.update(std.mem.asBytes(&key.function_hash));
                hasher.update(std.mem.asBytes(&key.optimization_level));
                hasher.update(std.mem.asBytes(&key.target_features));
                return hasher.final();
            }
            
            pub fn eql(self: @This(), a: OptimizationKey, b: OptimizationKey) bool {
                _ = self;
                return a.function_hash == b.function_hash and 
                       a.optimization_level == b.optimization_level and
                       a.target_features == b.target_features;
            }
        };
        
        const OptimizationResult = struct {
            optimized_function: c.LLVMValueRef,
            optimization_time_ms: u64,
            code_size_reduction: f32,
            estimated_speedup: f32,
        };
        
        const OptimizationStats = struct {
            total_optimizations: u32,
            cache_hits: u32,
            cache_misses: u32,
            total_optimization_time_ms: u64,
            average_speedup: f32,
        };
        
        pub fn init(allocator: Allocator) OptimizationCache {
            return OptimizationCache{
                .cached_optimizations = HashMap(OptimizationKey, OptimizationResult, OptimizationKeyContext, std.hash_map.default_max_load_percentage).init(allocator),
                .optimization_stats = OptimizationStats{
                    .total_optimizations = 0,
                    .cache_hits = 0,
                    .cache_misses = 0,
                    .total_optimization_time_ms = 0,
                    .average_speedup = 0.0,
                },
            };
        }
        
        pub fn deinit(self: *OptimizationCache) void {
            self.cached_optimizations.deinit(allocator);
        }
    };
    
    const SpecializationMetrics = struct {
        total_instantiations: u32,
        successful_instantiations: u32,
        failed_instantiations: u32,
        type_inference_successes: u32,
        type_inference_failures: u32,
        average_instantiation_time_ms: f64,
        code_size_growth_factor: f32,
        compilation_time_ms: u64,
        
        pub fn init() SpecializationMetrics {
            return SpecializationMetrics{
                .total_instantiations = 0,
                .successful_instantiations = 0,
                .failed_instantiations = 0,
                .type_inference_successes = 0,
                .type_inference_failures = 0,
                .average_instantiation_time_ms = 0.0,
                .code_size_growth_factor = 1.0,
                .compilation_time_ms = 0,
            };
        }
    };
    
    const MonomorphizationKey = struct {
        generic_name: []const u8,
        type_arguments_hash: u64,
        constraint_hash: u64,
        
        pub fn create(generic_name: []const u8, type_arguments: []const ast.Type, constraints: []const generics.Constraint) MonomorphizationKey {
            var type_hasher = std.hash.Wyhash.init(0);
            for (type_arguments) |type_arg| {
                type_hasher.update(std.mem.asBytes(&type_arg));
            }
            
            var constraint_hasher = std.hash.Wyhash.init(0);
            for (constraints) |constraint| {
                constraint_hasher.update(std.mem.asBytes(&constraint.kind));
                if (constraint.interface_name) |interface_name| {
                    constraint_hasher.update(interface_name);
                }
            }
            
            return MonomorphizationKey{
                .generic_name = generic_name,
                .type_arguments_hash = type_hasher.final(),
                .constraint_hash = constraint_hasher.final(),
            };
        }
    };
    
    const MonomorphizationKeyContext = struct {
        pub fn hash(self: @This(), key: MonomorphizationKey) u64 {
            _ = self;
            var hasher = std.hash.Wyhash.init(0);
            hasher.update(key.generic_name);
            hasher.update(std.mem.asBytes(&key.type_arguments_hash));
            hasher.update(std.mem.asBytes(&key.constraint_hash));
            return hasher.final();
        }
        
        pub fn eql(self: @This(), a: MonomorphizationKey, b: MonomorphizationKey) bool {
            _ = self;
            return std.mem.eql(u8, a.generic_name, b.generic_name) and
                   a.type_arguments_hash == b.type_arguments_hash and
                   a.constraint_hash == b.constraint_hash;
        }
    };
    
    const CachedInstantiation = struct {
        specialized_name: []const u8,
        llvm_function: ?c.LLVMValueRef,
        llvm_type: ?c.LLVMTypeRef,
        instantiation_time_ms: u64,
        optimization_level: u8,
        successful: bool,
    };
    
    const DependencyTracker = struct {
        dependencies: HashMap([]const u8, ArrayList([]const u8), std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        reverse_dependencies: HashMap([]const u8, ArrayList([]const u8), std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        
        pub fn init(allocator: Allocator) DependencyTracker {
            return DependencyTracker{
                .dependencies = HashMap([]const u8, ArrayList([]const u8), std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
                .reverse_dependencies = HashMap([]const u8, ArrayList([]const u8), std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            };
        }
        
        pub fn deinit(self: *DependencyTracker) void {
            var iter = self.dependencies.iterator();
            while (iter.next()) |entry| {
                entry.value_ptr.deinit(allocator);
            }
            self.dependencies.deinit(allocator);
            
            var rev_iter = self.reverse_dependencies.iterator();
            while (rev_iter.next()) |entry| {
                entry.value_ptr.deinit(allocator);
            }
            self.reverse_dependencies.deinit(allocator);
        }
        
        pub fn addDependency(self: *DependencyTracker, dependent: []const u8, dependency: []const u8) !void {
            // Add to forward dependencies
            const deps = try self.dependencies.getOrPut(dependent);
            if (!deps.found_existing) {
                deps.value_ptr.* = .empty;
            }
            try deps.value_ptr.append(allocator, dependency);
            
            // Add to reverse dependencies
            const rev_deps = try self.reverse_dependencies.getOrPut(dependency);
            if (!rev_deps.found_existing) {
                rev_deps.value_ptr.* = .empty;
            }
            try rev_deps.value_ptr.append(allocator, dependent);
        }
        
        pub fn getDependencies(self: *DependencyTracker, name: []const u8) ?[]const []const u8 {
            if (self.dependencies.get(name)) |deps| {
                return deps.items;
            }
            return null;
        }
        
        pub fn getDependents(self: *DependencyTracker, name: []const u8) ?[]const []const u8 {
            if (self.reverse_dependencies.get(name)) |deps| {
                return deps.items;
            }
            return null;
        }
    };
    
    pub fn init(
        allocator: Allocator,
        base_monomorphizer: *generics.Monomorphizer,
        context: c.LLVMContextRef,
        module: c.LLVMModuleRef,
        builder: c.LLVMBuilderRef,
        type_registry: *type_system.GCTypeRegistry
    ) EnhancedMonomorphizer {
        return EnhancedMonomorphizer{
            .allocator = allocator,
            .base_monomorphizer = base_monomorphizer,
            .context = context,
            .module = module,
            .builder = builder,
            .instantiation_graph = InstantiationGraph.init(allocator),
            .optimization_cache = OptimizationCache.init(allocator),
            .specialization_metrics = SpecializationMetrics.init(),
            .type_inference_ctx = type_inference.TypeInferenceContext.init(allocator, base_monomorphizer, type_registry),
            .monomorphization_cache = HashMap(MonomorphizationKey, CachedInstantiation, MonomorphizationKeyContext, std.hash_map.default_max_load_percentage).init(allocator),
            .dependency_tracker = DependencyTracker.init(allocator),
        };
    }
    
    pub fn deinit(self: *EnhancedMonomorphizer) void {
        self.instantiation_graph.deinit(allocator);
        self.optimization_cache.deinit(allocator);
        self.type_inference_ctx.deinit(allocator);
        self.monomorphization_cache.deinit(allocator);
        self.dependency_tracker.deinit(allocator);
    }
    
    /// Enhanced instantiation with automatic type inference
    pub fn instantiateWithInference(
        self: *EnhancedMonomorphizer,
        generic_name: []const u8,
        arg_types: []const ast.Type,
        expected_return_type: ?ast.Type,
        usage_location: []const u8
    ) ![]const u8 {
        const start_time = std.time.milliTimestamp();
        self.specialization_metrics.total_instantiations += 1;
        
        // Try type inference first
        if (try self.type_inference_ctx.inferGenericFunctionCall(generic_name, arg_types, expected_return_type)) |inferred_types| {
            self.specialization_metrics.type_inference_successes += 1;
            
            // Check cache first
            const cache_key = MonomorphizationKey.create(generic_name, inferred_types, &[_]generics.Constraint{});
            if (self.monomorphization_cache.get(cache_key)) |cached| {
                if (cached.successful) {
                    return cached.specialized_name;
                }
            }
            
            // Perform instantiation with inferred types
            const specialized_name = try self.performInstantiation(generic_name, inferred_types, usage_location);
            
            // Cache the result
            const instantiation_time = @as(u64, @intCast(std.time.milliTimestamp() - start_time));
            try self.monomorphization_cache.put(cache_key, CachedInstantiation{
                .specialized_name = specialized_name,
                .llvm_function = self.base_monomorphizer.getInstantiatedFunction(specialized_name),
                .llvm_type = self.base_monomorphizer.getInstantiatedType(specialized_name),
                .instantiation_time_ms = instantiation_time,
                .optimization_level = 2, // Default optimization level
                .successful = true,
            });
            
            self.specialization_metrics.successful_instantiations += 1;
            self.updateAverageInstantiationTime(instantiation_time);
            
            return specialized_name;
        } else {
            self.specialization_metrics.type_inference_failures += 1;
            self.specialization_metrics.failed_instantiations += 1;
            return error.TypeInferenceFailed;
        }
    }
    
    /// Perform instantiation with dependency tracking
    fn performInstantiation(
        self: *EnhancedMonomorphizer,
        generic_name: []const u8,
        type_arguments: []const ast.Type,
        usage_location: []const u8
    ) ![]const u8 {
        // Register in instantiation graph
        try self.registerInstantiationNode(generic_name, type_arguments);
        
        // Analyze dependencies
        try self.analyzeDependencies(generic_name, type_arguments);
        
        // Get instantiation order
        const instantiation_order = try self.instantiation_graph.getInstantiationOrder(self.allocator);
        defer self.allocator.free(instantiation_order);
        
        // Process instantiations in dependency order
        for (instantiation_order) |inst_name| {
            if (self.instantiation_graph.nodes.getPtr(inst_name)) |node| {
                if (node.status == .Pending) {
                    try self.processInstantiation(node);
                }
            }
        }
        
        // Delegate to base monomorphizer for actual instantiation
        return try self.base_monomorphizer.requestInstantiation(generic_name, type_arguments, usage_location);
    }
    
    /// Register a new instantiation node in the graph
    fn registerInstantiationNode(
        self: *EnhancedMonomorphizer,
        generic_name: []const u8,
        type_arguments: []const ast.Type
    ) !void {
        const specialized_name = try self.generateSpecializedName(generic_name, type_arguments);
        
        // Determine node kind
        const generic_decl = self.base_monomorphizer.generic_declarations.get(generic_name) orelse {
            return error.GenericNotFound;
        };
        
        const node_kind: InstantiationGraph.InstantiationNode.NodeKind = switch (generic_decl.kind) {
            .Function => .Function,
            .Struct => .Struct,
            .Interface => .Interface,
        };
        
        const node = InstantiationGraph.InstantiationNode{
            .name = specialized_name,
            .kind = node_kind,
            .type_arguments = type_arguments,
            .status = .Pending,
            .depends_on = .empty,
            .dependents = .empty,
            .priority = self.calculateInstantiationPriority(generic_name, type_arguments),
        };
        
        try self.instantiation_graph.nodes.put(specialized_name, node);
    }
    
    /// Analyze dependencies for a generic instantiation
    fn analyzeDependencies(
        self: *EnhancedMonomorphizer,
        generic_name: []const u8,
        type_arguments: []const ast.Type
    ) !void {
        const generic_decl = self.base_monomorphizer.generic_declarations.get(generic_name) orelse {
            return error.GenericNotFound;
        };
        
        const specialized_name = try self.generateSpecializedName(generic_name, type_arguments);
        
        // Analyze type argument dependencies
        for (type_arguments) |type_arg| {
            try self.analyzeTypeDependencies(specialized_name, type_arg);
        }
        
        // Analyze declaration-specific dependencies
        switch (generic_decl.kind) {
            .Function => try self.analyzeFunctionDependencies(specialized_name, generic_decl.ast_node.Function),
            .Struct => try self.analyzeStructDependencies(specialized_name, generic_decl.ast_node.Struct),
            .Interface => try self.analyzeInterfaceDependencies(specialized_name, generic_decl.ast_node.Interface),
        }
    }
    
    /// Analyze dependencies in a type
    fn analyzeTypeDependencies(self: *EnhancedMonomorphizer, dependent: []const u8, type_arg: ast.Type) !void {
        switch (type_arg) {
            .Identifier => |type_name| {
                // Check if this type is also a generic that needs instantiation
                if (self.base_monomorphizer.generic_declarations.contains(type_name)) {
                    try self.dependency_tracker.addDependency(dependent, type_name);
                    try self.instantiation_graph.addDependency(dependent, type_name, 1);
                }
            },
            .Generic => |generic_type| {
                try self.dependency_tracker.addDependency(dependent, generic_type.name);
                try self.instantiation_graph.addDependency(dependent, generic_type.name, 2);
            },
            .Array => |array_type| {
                try self.analyzeTypeDependencies(dependent, array_type.element_type.*);
            },
            .Slice => |slice_type| {
                try self.analyzeTypeDependencies(dependent, slice_type.element_type.*);
            },
            else => {}, // Primitive types have no dependencies
        }
    }
    
    /// Analyze function-specific dependencies
    fn analyzeFunctionDependencies(
        self: *EnhancedMonomorphizer,
        specialized_name: []const u8,
        func_decl: *ast.FunctionStatement
    ) !void {
        // Analyze parameter types
        for (func_decl.parameters.items) |param| {
            try self.analyzeTypeDependencies(specialized_name, param.param_type);
        }
        
        // Analyze return type
        if (func_decl.return_type) |return_type| {
            try self.analyzeTypeDependencies(specialized_name, return_type);
        }
        
        // TODO: Analyze function body for generic function calls
        // This would require walking the AST to find all generic function calls
    }
    
    /// Analyze struct-specific dependencies
    fn analyzeStructDependencies(
        self: *EnhancedMonomorphizer,
        specialized_name: []const u8,
        struct_decl: *ast.StructStatement
    ) !void {
        // Analyze field types
        for (struct_decl.fields.items) |field| {
            try self.analyzeTypeDependencies(specialized_name, field.field_type);
        }
    }
    
    /// Analyze interface-specific dependencies
    fn analyzeInterfaceDependencies(
        self: *EnhancedMonomorphizer,
        specialized_name: []const u8,
        interface_decl: *ast.InterfaceStatement
    ) !void {
        // Analyze method signatures
        for (interface_decl.methods.items) |method| {
            // Analyze parameter types
            for (method.parameters.items) |param| {
                try self.analyzeTypeDependencies(specialized_name, param.param_type);
            }
            
            // Analyze return type
            if (method.return_type) |return_type| {
                try self.analyzeTypeDependencies(specialized_name, return_type);
            }
        }
    }
    
    /// Process a single instantiation
    fn processInstantiation(self: *EnhancedMonomorphizer, node: *InstantiationGraph.InstantiationNode) !void {
        node.status = .InProgress;
        
        // Check if all dependencies are complete
        for (node.depends_on.items) |dependency| {
            if (self.instantiation_graph.nodes.get(dependency)) |dep_node| {
                if (dep_node.status != .Complete) {
                    // Dependency not ready, mark as failed for now
                    node.status = .Failed;
                    return error.DependencyNotReady;
                }
            }
        }
        
        // All dependencies are ready, mark as complete
        node.status = .Complete;
    }
    
    /// Calculate instantiation priority based on usage patterns
    fn calculateInstantiationPriority(
        self: *EnhancedMonomorphizer,
        generic_name: []const u8,
        type_arguments: []const ast.Type
    ) u32 {
        _ = self;
        _ = type_arguments;
        
        // Simple heuristic: common generic names get higher priority
        const common_generics = [_][]const u8{
            "Array", "List", "Map", "Set", "Optional", "Result"
        };
        
        for (common_generics) |common| {
            if (std.mem.eql(u8, generic_name, common)) {
                return 10;
            }
        }
        
        return 1; // Default priority
    }
    
    /// Generate specialized name (delegates to base monomorphizer)
    fn generateSpecializedName(
        self: *EnhancedMonomorphizer,
        generic_name: []const u8,
        type_arguments: []const ast.Type
    ) ![]const u8 {
        return try self.base_monomorphizer.generateSpecializedName(generic_name, type_arguments);
    }
    
    /// Update average instantiation time metric
    fn updateAverageInstantiationTime(self: *EnhancedMonomorphizer, new_time_ms: u64) void {
        const total_successful = self.specialization_metrics.successful_instantiations;
        if (total_successful == 1) {
            self.specialization_metrics.average_instantiation_time_ms = @as(f64, @floatFromInt(new_time_ms));
        } else {
            const current_avg = self.specialization_metrics.average_instantiation_time_ms;
            const new_avg = (current_avg * @as(f64, @floatFromInt(total_successful - 1)) + @as(f64, @floatFromInt(new_time_ms))) / @as(f64, @floatFromInt(total_successful));
            self.specialization_metrics.average_instantiation_time_ms = new_avg;
        }
    }
    
    /// Optimize instantiated function
    pub fn optimizeInstantiation(
        self: *EnhancedMonomorphizer,
        specialized_name: []const u8,
        optimization_level: u8
    ) !void {
        if (self.base_monomorphizer.getInstantiatedFunction(specialized_name)) |llvm_function| {
            // Create optimization key
            const function_hash = self.computeFunctionHash(llvm_function);
            const target_features = self.computeTargetFeaturesHash();
            
            const opt_key = OptimizationCache.OptimizationKey{
                .function_hash = function_hash,
                .optimization_level = optimization_level,
                .target_features = target_features,
            };
            
            // Check cache first
            if (self.optimization_cache.cached_optimizations.get(opt_key)) |cached_result| {
                self.optimization_cache.optimization_stats.cache_hits += 1;
                
                // Replace function with optimized version
                c.LLVMReplaceAllUsesWith(llvm_function, cached_result.optimized_function);
                return;
            }
            
            self.optimization_cache.optimization_stats.cache_misses += 1;
            
            // Perform optimization
            const start_time = std.time.milliTimestamp();
            try self.applyOptimizations(llvm_function, optimization_level);
            const optimization_time = @as(u64, @intCast(std.time.milliTimestamp() - start_time));
            
            // Cache the result
            const opt_result = OptimizationCache.OptimizationResult{
                .optimized_function = llvm_function,
                .optimization_time_ms = optimization_time,
                .code_size_reduction = 0.1, // Placeholder
                .estimated_speedup = 1.2,   // Placeholder
            };
            
            try self.optimization_cache.cached_optimizations.put(opt_key, opt_result);
            
            // Update stats
            self.optimization_cache.optimization_stats.total_optimizations += 1;
            self.optimization_cache.optimization_stats.total_optimization_time_ms += optimization_time;
        }
    }
    
    /// Apply LLVM optimizations to a function
    fn applyOptimizations(self: *EnhancedMonomorphizer, function: c.LLVMValueRef, optimization_level: u8) !void {
        // Create function pass manager
        const fpm = c.LLVMCreateFunctionPassManagerForModule(self.module);
        defer c.LLVMDisposeFunctionPassManager(fpm);
        
        // Add optimization passes based on level
        switch (optimization_level) {
            0 => {
                // No optimizations
            },
            1 => {
                // Basic optimizations
                c.LLVMAddInstructionCombiningPass(fpm);
                c.LLVMAddReassociatePass(fpm);
                c.LLVMAddGVNPass(fpm);
                c.LLVMAddCFGSimplificationPass(fpm);
            },
            2 => {
                // Standard optimizations
                c.LLVMAddInstructionCombiningPass(fpm);
                c.LLVMAddReassociatePass(fpm);
                c.LLVMAddGVNPass(fpm);
                c.LLVMAddCFGSimplificationPass(fpm);
                c.LLVMAddAggressiveDCEPass(fpm);
                c.LLVMAddMemCpyOptPass(fpm);
            },
            3 => {
                // Aggressive optimizations
                c.LLVMAddInstructionCombiningPass(fpm);
                c.LLVMAddReassociatePass(fpm);
                c.LLVMAddGVNPass(fpm);
                c.LLVMAddCFGSimplificationPass(fpm);
                c.LLVMAddAggressiveDCEPass(fpm);
                c.LLVMAddMemCpyOptPass(fpm);
                c.LLVMAddLoopUnrollPass(fpm);
                c.LLVMAddInlinePass(fpm, 225); // Inline threshold
            },
            else => {
                // Maximum optimizations
                c.LLVMAddInstructionCombiningPass(fpm);
                c.LLVMAddReassociatePass(fpm);
                c.LLVMAddGVNPass(fpm);
                c.LLVMAddCFGSimplificationPass(fpm);
                c.LLVMAddAggressiveDCEPass(fpm);
                c.LLVMAddMemCpyOptPass(fpm);
                c.LLVMAddLoopUnrollPass(fpm);
                c.LLVMAddInlinePass(fpm, 275);
                c.LLVMAddTailCallEliminationPass(fpm);
            },
        }
        
        // Initialize and run passes
        c.LLVMInitializeFunctionPassManager(fpm);
        _ = c.LLVMRunFunctionPassManager(fpm, function);
        c.LLVMFinalizeFunctionPassManager(fpm);
    }
    
    /// Compute hash of LLVM function for caching
    fn computeFunctionHash(self: *EnhancedMonomorphizer, function: c.LLVMValueRef) u64 {
        _ = self;
        _ = function;
        
        // TODO: Implement proper function hashing
        // This would involve hashing the LLVM IR representation
        return 0x1234567890ABCDEF;
    }
    
    /// Compute hash of target features for caching
    fn computeTargetFeaturesHash(self: *EnhancedMonomorphizer) u64 {
        _ = self;
        
        // TODO: Implement proper target feature hashing
        // This would involve hashing the current target triple and features
        return 0xFEDCBA0987654321;
    }
    
    /// Get comprehensive specialization statistics
    pub fn getSpecializationStats(self: *EnhancedMonomorphizer) SpecializationMetrics {
        return self.specialization_metrics;
    }
    
    /// Get optimization cache statistics
    pub fn getOptimizationStats(self: *EnhancedMonomorphizer) OptimizationCache.OptimizationStats {
        return self.optimization_cache.optimization_stats;
    }
    
    /// Print debug information about instantiations
    pub fn debugPrintInstantiationGraph(self: *EnhancedMonomorphizer) void {
        std.log.info("=== Instantiation Graph ===", .{});
        std.log.info("Nodes: {d}", .{self.instantiation_graph.nodes.count()});
        std.log.info("Edges: {d}", .{self.instantiation_graph.edges.items.len});
        
        var node_iter = self.instantiation_graph.nodes.iterator();
        while (node_iter.next()) |entry| {
            const node = entry.value_ptr.*;
            std.log.info("Node: {s} (status: {s}, priority: {d}, deps: {d})", 
                .{node.name, @tagName(node.status), node.priority, node.depends_on.items.len});
        }
        
        std.log.info("=== Specialization Metrics ===", .{});
        std.log.info("Total instantiations: {d}", .{self.specialization_metrics.total_instantiations});
        std.log.info("Successful: {d}", .{self.specialization_metrics.successful_instantiations});
        std.log.info("Failed: {d}", .{self.specialization_metrics.failed_instantiations});
        std.log.info("Type inference successes: {d}", .{self.specialization_metrics.type_inference_successes});
        std.log.info("Type inference failures: {d}", .{self.specialization_metrics.type_inference_failures});
        std.log.info("Average instantiation time: {d:.2}ms", .{self.specialization_metrics.average_instantiation_time_ms});
        
        std.log.info("=== Optimization Cache ===", .{});
        const opt_stats = self.optimization_cache.optimization_stats;
        std.log.info("Total optimizations: {d}", .{opt_stats.total_optimizations});
        std.log.info("Cache hits: {d}", .{opt_stats.cache_hits});
        std.log.info("Cache misses: {d}", .{opt_stats.cache_misses});
        
        if (opt_stats.total_optimizations > 0) {
            const hit_rate = @as(f32, @floatFromInt(opt_stats.cache_hits)) / @as(f32, @floatFromInt(opt_stats.total_optimizations)) * 100.0;
            std.log.info("Cache hit rate: {d:.1}%", .{hit_rate});
        }
        
        std.log.info("============================", .{});
    }
};

/// Integration function to connect enhanced monomorphization with codegen
pub fn integrateEnhancedMonomorphization(
    enhanced_mono: *EnhancedMonomorphizer,
    codegen: anytype
) !void {
    // Process all pending instantiations with dependency ordering
    try enhanced_mono.base_monomorphizer.processInstantiations();
    
    // Get instantiation order from dependency graph
    const instantiation_order = try enhanced_mono.instantiation_graph.getInstantiationOrder(enhanced_mono.allocator);
    defer enhanced_mono.allocator.free(instantiation_order);
    
    // Register instantiations with codegen in dependency order
    for (instantiation_order) |specialized_name| {
        if (enhanced_mono.base_monomorphizer.instances.get(specialized_name)) |instance| {
            if (instance.generated) {
                if (instance.llvm_function) |llvm_func| {
                    try codegen.registerFunction(specialized_name, llvm_func);
                    
                    // Apply optimizations
                    try enhanced_mono.optimizeInstantiation(specialized_name, 2);
                }
                
                if (instance.llvm_type) |llvm_type| {
                    try codegen.registerType(specialized_name, llvm_type);
                }
            }
        }
    }
    
    std.log.info("Enhanced monomorphization integration complete. Processed {d} instantiations.", 
        .{enhanced_mono.specialization_metrics.successful_instantiations});
}
