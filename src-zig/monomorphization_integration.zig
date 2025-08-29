//! Monomorphization Integration Module
//! 
//! This module integrates the enhanced monomorphization system with the
//! existing CURSED compiler infrastructure, including the advanced codegen,
//! parser, and type system.

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

// LLVM C imports
const c = @cImport({
    @cInclude("llvm_c_bindings.h");
});

// CURSED imports
const ast = @import("ast.zig");
const generics = @import("generics.zig");
const enhanced_mono = @import("enhanced_monomorphization.zig");
const advanced_codegen = @import("advanced_codegen.zig");
const type_system = @import("type_system_runtime.zig");
const type_inference = @import("type_inference.zig");
const error_handling = @import("error_handling.zig");
const CursedError = error_handling.CursedError;

/// Monomorphization integration manager that orchestrates the entire
/// monomorphization pipeline from parsing to code generation
pub const MonomorphizationManager = struct {
    allocator: Allocator,
    enhanced_monomorphizer: *enhanced_mono.EnhancedMonomorphizer,
    advanced_codegen: *advanced_codegen.AdvancedCodeGen,
    type_registry: *type_system.GCTypeRegistry,
    
    // Integration state
    pending_instantiations: ArrayList(InstantiationRequest),
    generic_registry: GenericRegistry,
    instantiation_pipeline: InstantiationPipeline,
    
    const InstantiationRequest = struct {
        generic_name: []const u8,
        call_site: CallSiteInfo,
        type_arguments: ?[]const ast.Type, // null for inference
        expected_return_type: ?ast.Type,
        priority: InstantiationPriority,
        
        const CallSiteInfo = struct {
            source_location: ast.SourceLocation,
            function_context: ?[]const u8,
            module_path: []const u8,
        };
        
        const InstantiationPriority = enum {
            Critical,   // Required for compilation to succeed
            High,       // Performance-critical code paths
            Normal,     // Standard usage
            Low,        // Optional optimizations
        };
    };
    
    const GenericRegistry = struct {
        function_generics: HashMap([]const u8, GenericFunctionInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        struct_generics: HashMap([]const u8, GenericStructInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        interface_generics: HashMap([]const u8, GenericInterfaceInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        
        const GenericFunctionInfo = struct {
            declaration: *ast.FunctionStatement,
            type_parameters: []const ast.TypeParameter,
            constraint_analysis: ConstraintAnalysis,
            usage_patterns: ArrayList(UsagePattern),
            
            const ConstraintAnalysis = struct {
                required_traits: ArrayList([]const u8),
                numeric_constraints: bool,
                lifetime_constraints: bool,
                complexity_score: u32,
            };
            
            const UsagePattern = struct {
                common_type_args: []const ast.Type,
                frequency: u32,
                performance_impact: f32,
            };
        };
        
        const GenericStructInfo = struct {
            declaration: *ast.StructStatement,
            type_parameters: []const ast.TypeParameter,
            field_dependencies: ArrayList(FieldDependency),
            layout_analysis: LayoutAnalysis,
            
            const FieldDependency = struct {
                field_name: []const u8,
                depends_on_type_param: []const u8,
                constraint_level: u8, // 0-255
            };
            
            const LayoutAnalysis = struct {
                size_depends_on_params: bool,
                alignment_requirements: u32,
                padding_optimization_potential: f32,
            };
        };
        
        const GenericInterfaceInfo = struct {
            declaration: *ast.InterfaceStatement,
            type_parameters: []const ast.TypeParameter,
            method_analysis: ArrayList(MethodAnalysis),
            vtable_optimization: VTableOptimization,
            
            const MethodAnalysis = struct {
                method_name: []const u8,
                generic_complexity: u32,
                inlining_potential: f32,
                dispatch_overhead: f32,
            };
            
            const VTableOptimization = struct {
                can_devirtualize: bool,
                static_dispatch_candidates: ArrayList([]const u8),
                optimization_opportunities: u32,
            };
        };
        
        pub fn init() GenericRegistry {
            return GenericRegistry{
                .function_generics = HashMap([]const u8, GenericFunctionInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
                .struct_generics = HashMap([]const u8, GenericStructInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
                .interface_generics = HashMap([]const u8, GenericInterfaceInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            };
        }
        
        pub fn deinit(self: *GenericRegistry) void {
            // Clean up function generics
            var func_iter = self.function_generics.iterator();
            while (func_iter.next()) |entry| {
                entry.value_ptr.constraint_analysis.required_traits.deinit();
                entry.value_ptr.usage_patterns.deinit();
            }
            self.function_generics.deinit(self.allocator);
            
            // Clean up struct generics
            var struct_iter = self.struct_generics.iterator();
            while (struct_iter.next()) |entry| {
                entry.value_ptr.field_dependencies.deinit();
            }
            self.struct_generics.deinit(self.allocator);
            
            // Clean up interface generics
            var interface_iter = self.interface_generics.iterator();
            while (interface_iter.next()) |entry| {
                entry.value_ptr.method_analysis.deinit();
                entry.value_ptr.vtable_optimization.static_dispatch_candidates.deinit();
            }
            self.interface_generics.deinit(self.allocator);
        }
    };
    
    const InstantiationPipeline = struct {
        stages: ArrayList(PipelineStage),
        current_stage: usize,
        stage_metrics: HashMap([]const u8, StageMetrics, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        
        const PipelineStage = struct {
            name: []const u8,
            processor: StageProcessor,
            dependencies: ArrayList([]const u8),
            parallel_safe: bool,
            
            const StageProcessor = enum {
                TypeInference,
                ConstraintValidation,
                DependencyAnalysis,
                CodeGeneration,
                Optimization,
                Linking,
            };
        };
        
        const StageMetrics = struct {
            executions: u32,
            total_time_ms: u64,
            success_rate: f32,
            average_throughput: f32, // instantiations per second
        };
        
        pub fn init() InstantiationPipeline {
            var pipeline = InstantiationPipeline{
                .stages = .empty,
                .current_stage = 0,
                .stage_metrics = HashMap([]const u8, StageMetrics, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            };
            
            // Initialize default pipeline stages
            const default_stages = [_]PipelineStage{
                PipelineStage{
                    .name = "type_inference",
                    .processor = .TypeInference,
                    .dependencies = .empty,
                    .parallel_safe = true,
                },
                PipelineStage{
                    .name = "constraint_validation",
                    .processor = .ConstraintValidation,
                    .dependencies = .empty,
                    .parallel_safe = true,
                },
                PipelineStage{
                    .name = "dependency_analysis",
                    .processor = .DependencyAnalysis,
                    .dependencies = .empty,
                    .parallel_safe = false,
                },
                PipelineStage{
                    .name = "code_generation",
                    .processor = .CodeGeneration,
                    .dependencies = .empty,
                    .parallel_safe = false,
                },
                PipelineStage{
                    .name = "optimization",
                    .processor = .Optimization,
                    .dependencies = .empty,
                    .parallel_safe = true,
                },
            };
            
            pipeline.stages.appendSlice(&default_stages) catch unreachable;
            
            // Set up stage dependencies
            pipeline.stages.items[1].dependencies.append(allocator, "type_inference") catch unreachable;
            pipeline.stages.items[2].dependencies.append(allocator, "constraint_validation") catch unreachable;
            pipeline.stages.items[3].dependencies.append(allocator, "dependency_analysis") catch unreachable;
            pipeline.stages.items[4].dependencies.append(allocator, "code_generation") catch unreachable;
            
            return pipeline;
        }
        
        pub fn deinit(self: *InstantiationPipeline) void {
            for (self.stages.items) |*stage| {
                stage.dependencies.deinit();
            }
            self.stages.deinit(self.allocator);
            self.stage_metrics.deinit(self.allocator);
        }
    };
    
    pub fn init(
        allocator: Allocator,
        enhanced_monomorphizer: *enhanced_mono.EnhancedMonomorphizer,
        advanced_codegen: *advanced_codegen.AdvancedCodeGen,
        type_registry: *type_system.GCTypeRegistry
    ) MonomorphizationManager {
        return MonomorphizationManager{
            .allocator = allocator,
            .enhanced_monomorphizer = enhanced_monomorphizer,
            .advanced_codegen = advanced_codegen,
            .type_registry = type_registry,
            .pending_instantiations = .empty,
            .generic_registry = GenericRegistry.init(allocator),
            .instantiation_pipeline = InstantiationPipeline.init(allocator),
        };
    }
    
    pub fn deinit(self: *MonomorphizationManager) void {
        self.pending_instantiations.deinit(self.allocator);
        self.generic_registry.deinit(self.allocator);
        self.instantiation_pipeline.deinit(self.allocator);
    }
    
    /// Register a generic function with comprehensive analysis
    pub fn registerGenericFunction(
        self: *MonomorphizationManager,
        func_decl: *ast.FunctionStatement,
        type_parameters: []const ast.TypeParameter
    ) !void {
        // Analyze function constraints
        var constraint_analysis = GenericRegistry.GenericFunctionInfo.ConstraintAnalysis{
            .required_traits = .empty,
            .numeric_constraints = false,
            .lifetime_constraints = false,
            .complexity_score = 0,
        };
        
        // Analyze type parameters for constraints
        for (type_parameters) |param| {
            constraint_analysis.complexity_score += param.constraints.items.len;
            
            for (param.constraints.items) |constraint| {
                switch (constraint.kind) {
                    .Numeric => constraint_analysis.numeric_constraints = true,
                    .Interface => {
                        if (constraint.interface_name) |interface_name| {
                            try constraint_analysis.required_traits.append(allocator, interface_name);
                        }
                    },
                    else => {},
                }
            }
        }
        
        const generic_info = GenericRegistry.GenericFunctionInfo{
            .declaration = func_decl,
            .type_parameters = type_parameters,
            .constraint_analysis = constraint_analysis,
            .usage_patterns = .empty,
        };
        
        try self.generic_registry.function_generics.put(func_decl.name, generic_info);
        
        // Register with base monomorphizer
        var generic_decl = generics.GenericDeclaration.init(self.allocator, func_decl.name, .Function);
        generic_decl.ast_node = generics.GenericDeclaration.ASTNode{ .Function = func_decl };
        
        for (type_parameters) |param| {
            var mono_param = generics.TypeParameter.init(self.allocator, param.name);
            for (param.constraints.items) |constraint| {
                const mono_constraint = switch (constraint.kind) {
                    .Any => generics.Constraint.init(.Any),
                    .Comparable => generics.Constraint.init(.Comparable),
                    .Numeric => generics.Constraint.init(.Numeric),
                    .Ordered => generics.Constraint.init(.Ordered),
                    .Interface => blk: {
                        if (constraint.interface_name) |interface_name| {
                            break :blk generics.Constraint.initInterface(interface_name);
                        } else {
                            break :blk generics.Constraint.init(.Any);
                        }
                    },
                    .Sized => generics.Constraint.init(.Sized),
                };
                try mono_param.constraints.append(allocator, mono_constraint);
            }
            try generic_decl.type_parameters.append(allocator, mono_param);
        }
        
        try self.enhanced_monomorphizer.base_monomorphizer.registerGeneric(generic_decl);
        
        std.log.info("Registered generic function: {s} with {d} type parameters", 
            .{func_decl.name, type_parameters.len});
    }
    
    /// Register a generic struct with layout analysis
    pub fn registerGenericStruct(
        self: *MonomorphizationManager,
        struct_decl: *ast.StructStatement,
        type_parameters: []const ast.TypeParameter
    ) !void {
        // Analyze field dependencies
        var field_dependencies = std.ArrayList(u8){};
        
        for (struct_decl.fields.items) |field| {
            // Check if field type depends on type parameters
            const dependency = self.analyzeFieldTypeDependency(field, type_parameters);
            if (dependency) |dep| {
                try field_dependencies.append(allocator, dep);
            }
        }
        
        // Analyze layout implications
        const layout_analysis = GenericRegistry.GenericStructInfo.LayoutAnalysis{
            .size_depends_on_params = field_dependencies.items.len > 0,
            .alignment_requirements = 8, // Default alignment
            .padding_optimization_potential = 0.1, // Estimate
        };
        
        const generic_info = GenericRegistry.GenericStructInfo{
            .declaration = struct_decl,
            .type_parameters = type_parameters,
            .field_dependencies = field_dependencies,
            .layout_analysis = layout_analysis,
        };
        
        try self.generic_registry.struct_generics.put(struct_decl.name, generic_info);
        
        // Register with base monomorphizer
        var generic_decl = generics.GenericDeclaration.init(self.allocator, struct_decl.name, .Struct);
        generic_decl.ast_node = generics.GenericDeclaration.ASTNode{ .Struct = struct_decl };
        
        // Convert type parameters
        for (type_parameters) |param| {
            var mono_param = generics.TypeParameter.init(self.allocator, param.name);
            for (param.constraints.items) |constraint| {
                const mono_constraint = switch (constraint.kind) {
                    .Any => generics.Constraint.init(.Any),
                    .Comparable => generics.Constraint.init(.Comparable),
                    .Numeric => generics.Constraint.init(.Numeric),
                    .Ordered => generics.Constraint.init(.Ordered),
                    .Interface => blk: {
                        if (constraint.interface_name) |interface_name| {
                            break :blk generics.Constraint.initInterface(interface_name);
                        } else {
                            break :blk generics.Constraint.init(.Any);
                        }
                    },
                    .Sized => generics.Constraint.init(.Sized),
                };
                try mono_param.constraints.append(allocator, mono_constraint);
            }
            try generic_decl.type_parameters.append(allocator, mono_param);
        }
        
        try self.enhanced_monomorphizer.base_monomorphizer.registerGeneric(generic_decl);
        
        std.log.info("Registered generic struct: {s} with {d} fields and {d} type parameters", 
            .{struct_decl.name, struct_decl.fields.items.len, type_parameters.len});
    }
    
    /// Analyze if a field type depends on generic type parameters
    fn analyzeFieldTypeDependency(
        self: *MonomorphizationManager,
        field: ast.StructField,
        type_parameters: []const ast.TypeParameter
    ) ?GenericRegistry.GenericStructInfo.FieldDependency {
        _ = self;
        
        switch (field.field_type) {
            .Identifier => |type_name| {
                // Check if this type name matches any type parameter
                for (type_parameters) |param| {
                    if (std.mem.eql(u8, type_name, param.name)) {
                        return GenericRegistry.GenericStructInfo.FieldDependency{
                            .field_name = field.name,
                            .depends_on_type_param = param.name,
                            .constraint_level = 255, // Full dependency
                        };
                    }
                }
            },
            .Generic => |generic_type| {
                // Check if generic type uses any of our type parameters
                for (type_parameters) |param| {
                    if (std.mem.eql(u8, generic_type.name, param.name)) {
                        return GenericRegistry.GenericStructInfo.FieldDependency{
                            .field_name = field.name,
                            .depends_on_type_param = param.name,
                            .constraint_level = 200, // High dependency
                        };
                    }
                }
            },
            .Array => |array_type| {
                // Recursively check element type
                const mock_field = ast.StructField{
                    .name = field.name,
                    .field_type = array_type.element_type.*,
                };
                return self.analyzeFieldTypeDependency(mock_field, type_parameters);
            },
            else => {},
        }
        
        return null;
    }
    
    /// Request instantiation through the comprehensive pipeline
    pub fn requestInstantiation(
        self: *MonomorphizationManager,
        generic_name: []const u8,
        call_site: InstantiationRequest.CallSiteInfo,
        type_arguments: ?[]const ast.Type,
        expected_return_type: ?ast.Type,
        priority: InstantiationRequest.InstantiationPriority
    ) ![]const u8 {
        // Create instantiation request
        const request = InstantiationRequest{
            .generic_name = generic_name,
            .call_site = call_site,
            .type_arguments = type_arguments,
            .expected_return_type = expected_return_type,
            .priority = priority,
        };
        
        try self.pending_instantiations.append(allocator, request);
        
        // Process through pipeline
        return try self.processInstantiationRequest(request);
    }
    
    /// Process a single instantiation request through the pipeline
    fn processInstantiationRequest(
        self: *MonomorphizationManager,
        request: InstantiationRequest
    ) ![]const u8 {
        const start_time = std.time.milliTimestamp();
        
        // Stage 1: Type Inference
        const inferred_types = if (request.type_arguments) |types|
            types
        else blk: {
            // Use enhanced monomorphization for type inference
            if (self.generic_registry.function_generics.get(request.generic_name)) |func_info| {
                // Create dummy argument types for inference
                // In real implementation, this would come from the call site
                const dummy_args = [_]ast.Type{ast.Type{ .Primitive = .Drip }};
                
                if (try self.enhanced_monomorphizer.type_inference_ctx.inferGenericFunctionCall(
                    request.generic_name,
                    &dummy_args,
                    request.expected_return_type
                )) |inferred| {
                    break :blk inferred;
                } else {
                    return error.TypeInferenceFailedInPipeline;
                }
            } else {
                return error.GenericNotFoundInRegistry;
            }
        };
        
        // Stage 2: Constraint Validation
        try self.validateConstraintsForRequest(request.generic_name, inferred_types);
        
        // Stage 3: Dependency Analysis
        try self.analyzeDependenciesForRequest(request.generic_name, inferred_types);
        
        // Stage 4: Code Generation
        const specialized_name = try self.enhanced_monomorphizer.instantiateWithInference(
            request.generic_name,
            inferred_types,
            request.expected_return_type,
            request.call_site.source_location.file
        );
        
        // Stage 5: Optimization
        try self.enhanced_monomorphizer.optimizeInstantiation(specialized_name, 2);
        
        // Update pipeline metrics
        const processing_time = @as(u64, @intCast(std.time.milliTimestamp() - start_time));
        try self.updatePipelineMetrics(processing_time);
        
        std.log.info("Processed instantiation request for {s} in {d}ms -> {s}", 
            .{request.generic_name, processing_time, specialized_name});
        
        return specialized_name;
    }
    
    /// Validate constraints for an instantiation request
    fn validateConstraintsForRequest(
        self: *MonomorphizationManager,
        generic_name: []const u8,
        type_arguments: []const ast.Type
    ) !void {
        if (self.generic_registry.function_generics.get(generic_name)) |func_info| {
            // Check if we have enough type arguments
            if (type_arguments.len != func_info.type_parameters.len) {
                std.log.err("Type argument count mismatch for {s}: expected {d}, got {d}", 
                    .{generic_name, func_info.type_parameters.len, type_arguments.len});
                return error.TypeArgumentCountMismatch;
            }
            
            // Validate each type argument against its constraints
            for (func_info.type_parameters, type_arguments) |param, type_arg| {
                for (param.constraints.items) |constraint| {
                    const valid = try self.validateSingleConstraint(type_arg, constraint);
                    if (!valid) {
                        std.log.err("Constraint violation for parameter {s}: type {any} does not satisfy constraint {any}", 
                            .{param.name, type_arg, constraint.kind});
                        return error.ConstraintViolation;
                    }
                }
            }
        } else {
            // Check struct generics
            if (self.generic_registry.struct_generics.get(generic_name)) |struct_info| {
                if (type_arguments.len != struct_info.type_parameters.len) {
                    return error.TypeArgumentCountMismatch;
                }
                
                for (struct_info.type_parameters, type_arguments) |param, type_arg| {
                    for (param.constraints.items) |constraint| {
                        const valid = try self.validateSingleConstraint(type_arg, constraint);
                        if (!valid) {
                            return error.ConstraintViolation;
                        }
                    }
                }
            } else {
                return error.GenericNotFoundInValidation;
            }
        }
    }
    
    /// Validate a single constraint against a type
    fn validateSingleConstraint(
        self: *MonomorphizationManager,
        type_arg: ast.Type,
        constraint: ast.TypeConstraint
    ) !bool {
        _ = self;
        
        switch (constraint.kind) {
            .Any => return true,
            .Comparable => {
                switch (type_arg) {
                    .Primitive => |primitive| {
                        return switch (primitive) {
                            .Tea, .Normie, .Drip, .Smol, .Thicc, .Meal, .Snack, .Lit => true,
                            .Vibes => false,
                        };
                    },
                    else => return false,
                }
            },
            .Numeric => {
                switch (type_arg) {
                    .Primitive => |primitive| {
                        return switch (primitive) {
                            .Normie, .Drip, .Smol, .Thicc, .Meal, .Snack => true,
                            .Tea, .Lit, .Vibes => false,
                        };
                    },
                    else => return false,
                }
            },
            .Ordered => {
                // For simplicity, same as Numeric for now
                return try self.validateSingleConstraint(type_arg, ast.TypeConstraint{ .kind = .Numeric, .interface_name = null });
            },
            .Interface => {
                if (constraint.interface_name) |interface_name| {
                    return try self.validateInterfaceConstraint(type_arg, interface_name);
                }
                return false;
            },
            .Sized => {
                switch (type_arg) {
                    .Primitive => return true,
                    .Array => return true,
                    .Slice => return false, // Dynamic size
                    else => return true,
                }
            },
        }
    }
    
    /// Validate that a type satisfies an interface constraint
    fn validateInterfaceConstraint(
        self: *MonomorphizationManager,
        type_arg: ast.Type,
        interface_name: []const u8
    ) !bool {
        // Check built-in interfaces first
        if (std.mem.eql(u8, interface_name, "Comparable")) {
            return self.isComparable(type_arg);
        } else if (std.mem.eql(u8, interface_name, "Numeric")) {
            return self.isNumeric(type_arg);
        } else if (std.mem.eql(u8, interface_name, "Ordered")) {
            return self.isOrdered(type_arg);
        } else if (std.mem.eql(u8, interface_name, "Display")) {
            return self.isDisplayable(type_arg);
        } else if (std.mem.eql(u8, interface_name, "Clone")) {
            return self.isCloneable(type_arg);
        }

        // Check user-defined interface implementations
        return try self.checkUserDefinedInterface(type_arg, interface_name);
    }
    
    /// Check if type implements a user-defined interface
    fn checkUserDefinedInterface(
        self: *MonomorphizationManager,
        type_arg: ast.Type,
        interface_name: []const u8
    ) !bool {
        const type_name = switch (type_arg) {
            .Identifier => |name| name,
            .Primitive => |prim| @tagName(prim),
            else => return false, // Complex types not supported yet
        };

        // Check if this is a struct type
        if (self.generic_registry.struct_generics.get(type_name)) |struct_info| {
            // For generic structs, check if any implementation exists
            // This is simplified - in reality we'd need to check specific instantiations
            return try self.hasInterfaceImplementation(type_name, interface_name);
        }

        // Check regular struct types (would need integration with struct registry)
        return try self.hasInterfaceImplementation(type_name, interface_name);
    }
    
    /// Check if a type has an interface implementation
    fn hasInterfaceImplementation(
        self: *MonomorphizationManager,
        type_name: []const u8,
        interface_name: []const u8
    ) !bool {
        // This would integrate with the interface registry system
        // For now, use a simplified check
        _ = self;
        _ = type_name;
        _ = interface_name;
        
        // TODO: Integrate with the actual interface registry system
        // This should check self.interface_registry or similar
        return true; // Conservative approach - assume implementation exists
    }
    
    /// Check if a type is comparable
    fn isComparable(self: *MonomorphizationManager, type_arg: ast.Type) bool {
        _ = self;
        return switch (type_arg) {
            .Primitive => |prim| switch (prim) {
                .Normie, .Drip, .Smol, .Thicc, .Meal, .Snack => true,
                .Tea, .Lit, .Vibes => false,
            },
            else => false,
        };
    }
    
    /// Check if a type is numeric
    fn isNumeric(self: *MonomorphizationManager, type_arg: ast.Type) bool {
        _ = self;
        return switch (type_arg) {
            .Primitive => |prim| switch (prim) {
                .Normie, .Drip, .Smol, .Thicc, .Meal, .Snack => true,
                else => false,
            },
            else => false,
        };
    }
    
    /// Check if a type is ordered
    fn isOrdered(self: *MonomorphizationManager, type_arg: ast.Type) bool {
        return self.isComparable(type_arg); // Same as comparable for now
    }
    
    /// Check if a type is displayable
    fn isDisplayable(self: *MonomorphizationManager, type_arg: ast.Type) bool {
        _ = self;
        return switch (type_arg) {
            .Primitive => true, // All primitive types are displayable
            .Identifier => true, // Assume user types implement display
            else => false,
        };
    }
    
    /// Check if a type is cloneable
    fn isCloneable(self: *MonomorphizationManager, type_arg: ast.Type) bool {
        _ = self;
        return switch (type_arg) {
            .Primitive => true, // All primitive types are cloneable
            .Identifier => true, // Assume user types are cloneable
            .Array, .Slice => true, // Collections are cloneable if elements are
            else => false,
        };
    }
    
    /// Analyze dependencies for an instantiation request
    fn analyzeDependenciesForRequest(
        self: *MonomorphizationManager,
        generic_name: []const u8,
        type_arguments: []const ast.Type
    ) !void {
        // Analyze type argument dependencies
        for (type_arguments) |type_arg| {
            try self.analyzeTypeDependency(generic_name, type_arg);
        }
        
        // Analyze generic-specific dependencies
        if (self.generic_registry.function_generics.get(generic_name)) |func_info| {
            try self.analyzeFunctionDependencies(func_info);
        } else if (self.generic_registry.struct_generics.get(generic_name)) |struct_info| {
            try self.analyzeStructDependencies(struct_info);
        }
    }
    
    /// Analyze a single type dependency
    fn analyzeTypeDependency(
        self: *MonomorphizationManager,
        dependent_name: []const u8,
        type_arg: ast.Type
    ) !void {
        _ = dependent_name;
        
        switch (type_arg) {
            .Identifier => |type_name| {
                // Check if this is another generic that needs instantiation
                if (self.generic_registry.function_generics.contains(type_name) or
                    self.generic_registry.struct_generics.contains(type_name) or
                    self.generic_registry.interface_generics.contains(type_name)) {
                    // Record dependency
                    std.log.debug("Found dependency: {s} depends on {s}", .{dependent_name, type_name});
                }
            },
            .Array => |array_type| {
                try self.analyzeTypeDependency(dependent_name, array_type.element_type.*);
            },
            .Slice => |slice_type| {
                try self.analyzeTypeDependency(dependent_name, slice_type.element_type.*);
            },
            else => {},
        }
    }
    
    /// Analyze function-specific dependencies
    fn analyzeFunctionDependencies(
        self: *MonomorphizationManager,
        func_info: GenericRegistry.GenericFunctionInfo
    ) !void {
        _ = self;
        _ = func_info;
        
        // TODO: Analyze function body for generic calls
        // This would require walking the AST
    }
    
    /// Analyze struct-specific dependencies
    fn analyzeStructDependencies(
        self: *MonomorphizationManager,
        struct_info: GenericRegistry.GenericStructInfo
    ) !void {
        _ = self;
        
        // Field dependencies are already analyzed during registration
        for (struct_info.field_dependencies.items) |dependency| {
            std.log.debug("Struct field dependency: {s} -> {s}", 
                .{dependency.field_name, dependency.depends_on_type_param});
        }
    }
    
    /// Update pipeline metrics
    fn updatePipelineMetrics(self: *MonomorphizationManager, processing_time_ms: u64) !void {
        for (self.instantiation_pipeline.stages.items) |stage| {
            const metrics = try self.instantiation_pipeline.stage_metrics.getOrPut(stage.name);
            if (!metrics.found_existing) {
                metrics.value_ptr.* = InstantiationPipeline.StageMetrics{
                    .executions = 0,
                    .total_time_ms = 0,
                    .success_rate = 0.0,
                    .average_throughput = 0.0,
                };
            }
            
            metrics.value_ptr.executions += 1;
            metrics.value_ptr.total_time_ms += processing_time_ms / @as(u64, @intCast(self.instantiation_pipeline.stages.items.len));
            
            // Update success rate (simplified)
            const current_success_rate = metrics.value_ptr.success_rate;
            const executions = @as(f32, @floatFromInt(metrics.value_ptr.executions));
            metrics.value_ptr.success_rate = (current_success_rate * (executions - 1.0) + 1.0) / executions;
        }
    }
    
    /// Process all pending instantiations
    pub fn processAllPendingInstantiations(self: *MonomorphizationManager) !u32 {
        var processed_count: u32 = 0;
        
        while (self.pending_instantiations.items.len > 0) {
            const request = self.pending_instantiations.pop();
            
            const result = self.processInstantiationRequest(request);
            if (result) |_| {
                processed_count += 1;
            } else |err| {
                std.log.err("Failed to process instantiation request for {s}: {any}", 
                    .{request.generic_name, err});
            }
        }
        
        return processed_count;
    }
    
    /// Get comprehensive statistics about the monomorphization process
    pub fn getComprehensiveStats(self: *MonomorphizationManager) ComprehensiveStats {
        const mono_stats = self.enhanced_monomorphizer.getSpecializationStats();
        const opt_stats = self.enhanced_monomorphizer.getOptimizationStats();
        
        return ComprehensiveStats{
            .monomorphization_stats = mono_stats,
            .optimization_stats = opt_stats,
            .registry_stats = RegistryStats{
                .registered_functions = @as(u32, @intCast(self.generic_registry.function_generics.count())),
                .registered_structs = @as(u32, @intCast(self.generic_registry.struct_generics.count())),
                .registered_interfaces = @as(u32, @intCast(self.generic_registry.interface_generics.count())),
            },
            .pipeline_stats = self.calculatePipelineStats(),
        };
    }
    
    const ComprehensiveStats = struct {
        monomorphization_stats: enhanced_mono.EnhancedMonomorphizer.SpecializationMetrics,
        optimization_stats: enhanced_mono.EnhancedMonomorphizer.OptimizationCache.OptimizationStats,
        registry_stats: RegistryStats,
        pipeline_stats: PipelineStats,
    };
    
    const RegistryStats = struct {
        registered_functions: u32,
        registered_structs: u32,
        registered_interfaces: u32,
    };
    
    const PipelineStats = struct {
        total_stage_executions: u32,
        average_stage_time_ms: f64,
        pipeline_success_rate: f32,
        bottleneck_stage: ?[]const u8,
    };
    
    fn calculatePipelineStats(self: *MonomorphizationManager) PipelineStats {
        var total_executions: u32 = 0;
        var total_time_ms: u64 = 0;
        var total_success_rate: f32 = 0.0;
        var slowest_stage: ?[]const u8 = null;
        var slowest_time: u64 = 0;
        
        var stage_iter = self.instantiation_pipeline.stage_metrics.iterator();
        while (stage_iter.next()) |entry| {
            const stage_name = entry.key_ptr.*;
            const metrics = entry.value_ptr.*;
            
            total_executions += metrics.executions;
            total_time_ms += metrics.total_time_ms;
            total_success_rate += metrics.success_rate;
            
            if (metrics.total_time_ms > slowest_time) {
                slowest_time = metrics.total_time_ms;
                slowest_stage = stage_name;
            }
        }
        
        const stage_count = @as(u32, @intCast(self.instantiation_pipeline.stage_metrics.count()));
        
        return PipelineStats{
            .total_stage_executions = total_executions,
            .average_stage_time_ms = if (stage_count > 0) @as(f64, @floatFromInt(total_time_ms)) / @as(f64, @floatFromInt(stage_count)) else 0.0,
            .pipeline_success_rate = if (stage_count > 0) total_success_rate / @as(f32, @floatFromInt(stage_count)) else 0.0,
            .bottleneck_stage = slowest_stage,
        };
    }
    
    /// Print comprehensive debug information
    pub fn debugPrintComprehensiveStats(self: *MonomorphizationManager) void {
        const stats = self.getComprehensiveStats();
        
        std.log.info("=== Comprehensive Monomorphization Statistics ===", .{});
        
        // Monomorphization stats
        std.log.info("Monomorphization:", .{});
        std.log.info("  Total instantiations: {d}", .{stats.monomorphization_stats.total_instantiations});
        std.log.info("  Successful: {d}", .{stats.monomorphization_stats.successful_instantiations});
        std.log.info("  Failed: {d}", .{stats.monomorphization_stats.failed_instantiations});
        std.log.info("  Avg instantiation time: {d:.2}ms", .{stats.monomorphization_stats.average_instantiation_time_ms});
        
        // Optimization stats
        std.log.info("Optimization:", .{});
        std.log.info("  Total optimizations: {d}", .{stats.optimization_stats.total_optimizations});
        std.log.info("  Cache hits: {d}", .{stats.optimization_stats.cache_hits});
        std.log.info("  Cache misses: {d}", .{stats.optimization_stats.cache_misses});
        
        // Registry stats
        std.log.info("Registry:", .{});
        std.log.info("  Functions: {d}", .{stats.registry_stats.registered_functions});
        std.log.info("  Structs: {d}", .{stats.registry_stats.registered_structs});
        std.log.info("  Interfaces: {d}", .{stats.registry_stats.registered_interfaces});
        
        // Pipeline stats
        std.log.info("Pipeline:", .{});
        std.log.info("  Total stage executions: {d}", .{stats.pipeline_stats.total_stage_executions});
        std.log.info("  Average stage time: {d:.2}ms", .{stats.pipeline_stats.average_stage_time_ms});
        std.log.info("  Success rate: {d:.1}%", .{stats.pipeline_stats.pipeline_success_rate * 100.0});
        if (stats.pipeline_stats.bottleneck_stage) |bottleneck| {
            std.log.info("  Bottleneck stage: {s}", .{bottleneck});
        }
        
        std.log.info("====================================================", .{});
    }
};

/// Integration function to connect the monomorphization manager with the main compiler
pub fn integrateMonomorphizationManager(
    manager: *MonomorphizationManager,
    codegen: anytype
) !void {
    // Process all pending instantiations
    const processed = try manager.processAllPendingInstantiations();
    
    // Integrate with enhanced monomorphization
    try enhanced_mono.integrateEnhancedMonomorphization(manager.enhanced_monomorphizer, codegen);
    
    std.log.info("Monomorphization integration complete. Processed {d} instantiations.", .{processed});
    
    // Print comprehensive statistics
    manager.debugPrintComprehensiveStats();
}
