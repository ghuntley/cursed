//! P29: Enhanced Type Inference for Generics
//! Allows generic functions to infer type parameters from arguments

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const ast = @import("ast_advanced.zig");
const generics = @import("generics.zig");
const type_system = @import("type_system_runtime.zig");

/// Type inference context for generic instantiation
pub const TypeInferenceContext = struct {
    allocator: Allocator,
    monomorphizer: *generics.Monomorphizer,
    type_registry: *type_system.GCTypeRegistry,
    
    // Inference state
    inferred_types: HashMap([]const u8, ast.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    constraint_queue: ArrayList(Constraint),
    unification_cache: HashMap(UnificationKey, bool, UnificationKeyContext, std.hash_map.default_max_load_percentage),
    
    const Constraint = struct {
        type_param: []const u8,
        concrete_type: ast.Type,
        source: ConstraintSource,
        
        const ConstraintSource = enum {
            Argument,      // From function argument
            ReturnType,    // From expected return type
            Assignment,    // From variable assignment
            Field,         // From struct field access
        };
    };
    
    const UnificationKey = struct {
        type1: ast.Type,
        type2: ast.Type,
    };
    
    const UnificationKeyContext = struct {
        pub fn hash(self: @This(), key: UnificationKey) u64 {
            _ = self;
            var hasher = std.hash.Wyhash.init(0);
            hasher.update(std.mem.asBytes(&key.type1));
            hasher.update(std.mem.asBytes(&key.type2));
            return hasher.final();
        }
        
        pub fn eql(self: @This(), a: UnificationKey, b: UnificationKey) bool {
            _ = self;
            return std.meta.eql(a.type1, b.type1) and std.meta.eql(a.type2, b.type2);
        }
    };
    
    pub fn init(allocator: Allocator, monomorphizer: *generics.Monomorphizer, type_registry: *type_system.GCTypeRegistry) TypeInferenceContext {
        return TypeInferenceContext{
            .allocator = allocator,
            .monomorphizer = monomorphizer,
            .type_registry = type_registry,
            .inferred_types = HashMap([]const u8, ast.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .constraint_queue = .empty,
            .unification_cache = HashMap(UnificationKey, bool, UnificationKeyContext, std.hash_map.default_max_load_percentage){},
        };
    }
    
    pub fn deinit(self: *TypeInferenceContext) void {
        self.inferred_types.deinit(self.allocator);
        self.constraint_queue.deinit(self.allocator);
        self.unification_cache.deinit(self.allocator);
    }
    
    /// Infer type parameters for a generic function call
    pub fn inferGenericFunctionCall(self: *TypeInferenceContext, func_name: []const u8, arg_types: []const ast.Type, expected_return_type: ?ast.Type) !?[]const ast.Type {
        // Clear previous inference state
        self.inferred_types.clearRetainingCapacity();
        self.constraint_queue.clearRetainingCapacity();
        
        // Get generic declaration
        const generic_decl = self.monomorphizer.generic_declarations.get(func_name) orelse {
            return null; // Not a generic function
        };
        
        if (generic_decl.kind != .Function) {
            return error.NotAGenericFunction;
        }
        
        const func_decl = generic_decl.ast_node.Function;
        
        // Generate constraints from function arguments
        try self.generateArgumentConstraints(func_decl, arg_types);
        
        // Generate constraints from expected return type
        if (expected_return_type) |ret_type| {
            if (func_decl.return_type) |func_ret_type| {
                try self.addConstraint("RETURN", ret_type, .ReturnType);
                try self.unifyTypes(func_ret_type, ret_type);
            }
        }
        
        // Solve constraints to infer type parameters
        try self.solveConstraints();
        
        // Extract inferred type arguments
        var type_args = std.ArrayList(u8){};
        defer type_args.deinit();
        
        for (generic_decl.type_parameters.items) |param| {
            if (self.inferred_types.get(param.name)) |inferred_type| {
                try type_args.append(allocator, inferred_type);
            } else {
                // Could not infer this type parameter
                return null;
            }
        }
        
        return type_args.toOwnedSlice();
    }
    
    /// Generate constraints from function arguments with enhanced generic support
    fn generateArgumentConstraints(self: *TypeInferenceContext, func_decl: *ast.FunctionStatement, arg_types: []const ast.Type) !void {
        if (arg_types.len != func_decl.parameters.items.len) {
            return error.ArgumentCountMismatch;
        }
        
        for (func_decl.parameters.items, 0..) |param, i| {
            const arg_type = arg_types[i];
            
            // Enhanced constraint generation with parameter name context
            const constraint_name = try std.fmt.allocPrint(self.allocator, "{s}_arg{d}", .{ func_decl.name, i });
            defer self.allocator.free(constraint_name);
            
            try self.unifyTypes(param.param_type, arg_type);
            
            // Add explicit constraint for better tracking
            if (self.isTypeParameter(param.param_type)) {
                try self.addConstraint(self.extractTypeParameterName(param.param_type), arg_type, .Argument);
            }
        }
    }
    
    /// Check if a type is a type parameter
    fn isTypeParameter(self: *TypeInferenceContext, type_expr: ast.Type) bool {
        return switch (type_expr) {
            .Identifier => true, // Type parameters are represented as identifiers
            else => false,
        };
    }
    
    /// Extract type parameter name from type expression
    fn extractTypeParameterName(self: *TypeInferenceContext, type_expr: ast.Type) []const u8 {
        return switch (type_expr) {
            .Identifier => |ident| ident,
            else => "", // Should not happen if isTypeParameter returned true
        };
    }
    
    /// Add a type constraint with enhanced validation
    fn addConstraint(self: *TypeInferenceContext, type_param: []const u8, concrete_type: ast.Type, source: Constraint.ConstraintSource) !void {
        // Validate constraint before adding
        if (type_param.len == 0) {
            return error.InvalidTypeParameter;
        }
        
        // Check for immediate contradictions
        if (self.inferred_types.get(type_param)) |existing_type| {
            if (!self.typesAreCompatible(existing_type, concrete_type)) {
                return error.ConflictingTypeConstraints;
            }
        }
        
        try self.constraint_queue.append(self.allocator, Constraint{
            .type_param = type_param,
            .concrete_type = concrete_type,
            .source = source,
        });
    }
    
    /// Unify two types to generate inference constraints
    fn unifyTypes(self: *TypeInferenceContext, formal_type: ast.Type, actual_type: ast.Type) !void {
        const key = UnificationKey{ .type1 = formal_type, .type2 = actual_type };
        
        // Check cache to avoid infinite recursion
        if (self.unification_cache.contains(key)) {
            return;
        }
        try self.unification_cache.put(key, true);
        
        switch (formal_type) {
            .Identifier => |type_param| {
                // This is a type parameter - add constraint
                try self.addConstraint(type_param, actual_type, .Argument);
            },
            .Array => |formal_array| {
                switch (actual_type) {
                    .Array => |actual_array| {
                        // Unify element types
                        try self.unifyTypes(formal_array.element_type.*, actual_array.element_type.*);
                        
                        // Check array sizes match
                        if (formal_array.size != actual_array.size) {
                            return error.ArraySizeMismatch;
                        }
                    },
                    else => return error.TypeMismatch,
                }
            },
            .Slice => |formal_slice| {
                switch (actual_type) {
                    .Slice => |actual_slice| {
                        try self.unifyTypes(formal_slice.element_type.*, actual_slice.element_type.*);
                    },
                    .Array => |actual_array| {
                        // Array can be coerced to slice
                        try self.unifyTypes(formal_slice.element_type.*, actual_array.element_type.*);
                    },
                    else => return error.TypeMismatch,
                }
            },
            .Generic => |formal_generic| {
                // Handle nested generics
                switch (actual_type) {
                    .Generic => |actual_generic| {
                        if (!std.mem.eql(u8, formal_generic.name, actual_generic.name)) {
                            return error.GenericTypeMismatch;
                        }
                        
                        // Unify type arguments
                        if (formal_generic.type_arguments.items.len != actual_generic.type_arguments.items.len) {
                            return error.GenericArityMismatch;
                        }
                        
                        for (formal_generic.type_arguments.items, 0..) |formal_arg, i| {
                            const actual_arg = actual_generic.type_arguments.items[i];
                            try self.unifyTypes(formal_arg, actual_arg);
                        }
                    },
                    else => return error.TypeMismatch,
                }
            },
            .Primitive => |formal_primitive| {
                switch (actual_type) {
                    .Primitive => |actual_primitive| {
                        if (formal_primitive != actual_primitive) {
                            // Check if types are compatible (e.g., numeric coercion)
                            if (!self.areCompatiblePrimitives(formal_primitive, actual_primitive)) {
                                return error.IncompatiblePrimitives;
                            }
                        }
                    },
                    else => return error.TypeMismatch,
                }
            },
            else => {
                // For non-generic types, they must match exactly
                if (!std.meta.eql(formal_type, actual_type)) {
                    return error.TypeMismatch;
                }
            },
        }
    }
    
    /// Check if two primitive types are compatible
    fn areCompatiblePrimitives(self: *TypeInferenceContext, formal: ast.Type.PrimitiveType, actual: ast.Type.PrimitiveType) bool {
        _ = self;
        
        // CURSED numeric type compatibility
        const numeric_groups = [_][]const ast.Type.PrimitiveType{
            &[_]ast.Type.PrimitiveType{ .Normie, .Drip, .Thicc },
            &[_]ast.Type.PrimitiveType{ .Snack, .Meal },
        };
        
        for (numeric_groups) |group| {
            var formal_in_group = false;
            var actual_in_group = false;
            
            for (group) |prim_type| {
                if (formal == prim_type) formal_in_group = true;
                if (actual == prim_type) actual_in_group = true;
            }
            
            if (formal_in_group and actual_in_group) return true;
        }
        
        return false;
    }
    
    /// Solve constraints to determine type parameter bindings
    fn solveConstraints(self: *TypeInferenceContext) !void {
        // Process constraints in order
        for (self.constraint_queue.items) |constraint| {
            // Check if we already have a binding for this type parameter
            if (self.inferred_types.get(constraint.type_param)) |existing_type| {
                // Verify consistency
                if (!self.typesAreCompatible(existing_type, constraint.concrete_type)) {
                    return error.InconsistentTypeInference;
                }
            } else {
                // Add new binding
                try self.inferred_types.put(constraint.type_param, constraint.concrete_type);
            }
        }
    }
    
    /// Enhanced constraint solving with iterative propagation
    fn solveConstraintsWithPropagation(self: *TypeInferenceContext) !void {
        var changed = true;
        var iteration = 0;
        const max_iterations = 100;
        
        // Iteratively solve constraints until convergence
        while (changed and iteration < max_iterations) {
            changed = false;
            iteration += 1;
            
            for (self.constraint_queue.items) |constraint| {
                // Check if we already have a binding for this type parameter
                if (self.inferred_types.get(constraint.type_param)) |existing_type| {
                    // Verify consistency with enhanced checking
                    if (!self.typesAreCompatibleEnhanced(existing_type, constraint.concrete_type, constraint.source)) {
                        return error.InconsistentTypeInference;
                    }
                } else {
                    // Add new binding with propagation
                    try self.inferred_types.put(constraint.type_param, constraint.concrete_type);
                    changed = true;
                    
                    // Propagate constraints to dependent type parameters
                    try self.propagateConstraints(constraint.type_param, constraint.concrete_type);
                }
            }
        }
        
        if (iteration >= max_iterations) {
            return error.InferenceConvergenceFailed;
        }
    }
    
    /// Propagate constraints to dependent type parameters
    fn propagateConstraints(self: *TypeInferenceContext, type_param: []const u8, concrete_type: ast.Type) !void {
        // Look for dependent constraints that can be resolved
        for (self.constraint_queue.items) |constraint| {
            if (!std.mem.eql(u8, constraint.type_param, type_param)) {
                // Check if this constraint depends on the newly resolved type parameter
                if (self.constraintDependsOn(constraint, type_param)) {
                    try self.propagateToConstraint(constraint, type_param, concrete_type);
                }
            }
        }
    }
    
    /// Check if a constraint depends on a specific type parameter
    fn constraintDependsOn(self: *TypeInferenceContext, constraint: Constraint, type_param: []const u8) bool {
        return self.typeDependsOn(constraint.concrete_type, type_param);
    }
    
    /// Check if a type expression depends on a specific type parameter
    fn typeDependsOn(self: *TypeInferenceContext, type_expr: ast.Type, type_param: []const u8) bool {
        switch (type_expr) {
            .Identifier => |ident| return std.mem.eql(u8, ident, type_param),
            .Array => |array_type| return self.typeDependsOn(array_type.element_type.*, type_param),
            .Slice => |slice_type| return self.typeDependsOn(slice_type.element_type.*, type_param),
            .Function => |func_type| {
                if (self.typeDependsOn(func_type.return_type.*, type_param)) return true;
                for (func_type.parameter_types.items) |param_type| {
                    if (self.typeDependsOn(param_type, type_param)) return true;
                }
                return false;
            },
            else => return false,
        }
    }
    
    /// Propagate concrete type to a dependent constraint
    fn propagateToConstraint(self: *TypeInferenceContext, constraint: Constraint, type_param: []const u8, concrete_type: ast.Type) !void {
        // Substitute the concrete type in the constraint's type expression
        const substituted_type = try self.substituteTypeInExpression(constraint.concrete_type, type_param, concrete_type);
        
        // Update the constraint with the substituted type
        if (self.inferred_types.getPtr(constraint.type_param)) |existing_ptr| {
            // Try to unify with existing binding
            if (!self.typesAreCompatible(existing_ptr.*, substituted_type)) {
                return error.TypePropagationConflict;
            }
            existing_ptr.* = substituted_type;
        } else {
            try self.inferred_types.put(constraint.type_param, substituted_type);
        }
    }
    
    /// Substitute a type parameter with concrete type in a type expression
    fn substituteTypeInExpression(self: *TypeInferenceContext, type_expr: ast.Type, type_param: []const u8, concrete_type: ast.Type) !ast.Type {
        switch (type_expr) {
            .Identifier => |ident| {
                if (std.mem.eql(u8, ident, type_param)) {
                    return concrete_type;
                }
                return type_expr;
            },
            .Array => |array_type| {
                const new_element_type = try self.substituteTypeInExpression(array_type.element_type.*, type_param, concrete_type);
                const element_type_ptr = try self.allocator.create(ast.Type);
                element_type_ptr.* = new_element_type;
                return ast.Type{ .Array = ast.ArrayType{
                    .element_type = element_type_ptr,
                    .size = array_type.size,
                }};
            },
            .Slice => |slice_type| {
                const new_element_type = try self.substituteTypeInExpression(slice_type.element_type.*, type_param, concrete_type);
                const element_type_ptr = try self.allocator.create(ast.Type);
                element_type_ptr.* = new_element_type;
                return ast.Type{ .Slice = ast.SliceType{
                    .element_type = element_type_ptr,
                }};
            },
            else => return type_expr,
        }
    }
    
    /// Enhanced type compatibility checking with source context
    fn typesAreCompatibleEnhanced(self: *TypeInferenceContext, type1: ast.Type, type2: ast.Type, source: ConstraintSource) bool {
        if (std.meta.eql(type1, type2)) return true;
        
        // Context-sensitive compatibility checking
        switch (source) {
            .ReturnType => return self.checkReturnTypeCompatibility(type1, type2),
            .Argument => return self.checkArgumentTypeCompatibility(type1, type2),
            else => return self.typesAreCompatible(type1, type2),
        }
    }
    
    /// Check return type compatibility (covariant)
    fn checkReturnTypeCompatibility(self: *TypeInferenceContext, expected: ast.Type, actual: ast.Type) bool {
        // Return types are covariant
        return self.isSubtype(actual, expected);
    }
    
    /// Check argument type compatibility (contravariant)
    fn checkArgumentTypeCompatibility(self: *TypeInferenceContext, expected: ast.Type, actual: ast.Type) bool {
        // Argument types are contravariant
        return self.isSubtype(expected, actual);
    }
    
    /// Subtype checking for variance
    fn isSubtype(self: *TypeInferenceContext, subtype: ast.Type, supertype: ast.Type) bool {
        // Simplified subtype relation
        switch (subtype) {
            .Primitive => |sub_prim| {
                switch (supertype) {
                    .Primitive => |super_prim| return sub_prim == super_prim,
                    else => return false,
                }
            },
            else => return self.typesAreCompatible(subtype, supertype),
        }
    }
    
    /// Propagate constraints to dependent type parameters
    fn propagateConstraints(self: *TypeInferenceContext, type_param: []const u8, concrete_type: ast.Type) !void {
        // Find all constraints that depend on this type parameter
        for (self.constraint_queue.items) |constraint| {
            if (self.typeContainsParameter(constraint.concrete_type, type_param)) {
                // Substitute the resolved type and create new constraints
                const substituted_type = self.substituteType(constraint.concrete_type, type_param, concrete_type);
                if (!std.meta.eql(substituted_type, constraint.concrete_type)) {
                    try self.constraint_queue.append(.{
                        .type_param = constraint.type_param,
                        .concrete_type = substituted_type,
                        .source = constraint.source,
                    });
                }
            }
        }
    }
    
    /// Check if a type contains a specific type parameter
    fn typeContainsParameter(self: *TypeInferenceContext, type_to_check: ast.Type, param_name: []const u8) bool {
        switch (type_to_check) {
            .Custom => |name| return std.mem.eql(u8, name, param_name),
            .Generic => |generic| {
                if (std.mem.eql(u8, generic.name, param_name)) return true;
                for (generic.type_args.items) |arg| {
                    if (self.typeContainsParameter(arg, param_name)) return true;
                }
                return false;
            },
            else => return false,
        }
    }
    
    /// Substitute a type parameter with a concrete type
    fn substituteType(self: *TypeInferenceContext, original: ast.Type, param_name: []const u8, replacement: ast.Type) ast.Type {
        switch (original) {
            .Custom => |name| {
                if (std.mem.eql(u8, name, param_name)) {
                    return replacement;
                }
                return original;
            },
            .Generic => |generic| {
                if (std.mem.eql(u8, generic.name, param_name)) {
                    return replacement;
                }
                // Recursively substitute in type arguments
                var new_args = std.ArrayList(ast.Type){};
                for (generic.type_args.items) |arg| {
                    try new_args.append(self.substituteType(arg, param_name, replacement));
                }
                return ast.Type{ .Generic = .{
                    .name = generic.name,
                    .type_args = new_args,
                }};
            },
            else => return original,
        }
    }
    
    /// Check if two types are compatible for inference
    fn typesAreCompatible(self: *TypeInferenceContext, type1: ast.Type, type2: ast.Type) bool {
        if (std.meta.eql(type1, type2)) return true;
        
        // Check primitive compatibility
        switch (type1) {
            .Primitive => |prim1| {
                switch (type2) {
                    .Primitive => |prim2| {
                        return self.areCompatiblePrimitives(prim1, prim2);
                    },
                    .Identifier => |name| {
                        // Type variable can unify with any primitive
                        _ = name;
                        return true;
                    },
                    else => return false,
                }
            },
            .Identifier => |name| {
                // Type variables are compatible with anything for inference
                _ = name;
                return true;
            },
            .Array => |arr1| {
                switch (type2) {
                    .Array => |arr2| {
                        if (arr1.size != arr2.size) return false;
                        return self.typesAreCompatible(arr1.element_type.*, arr2.element_type.*);
                    },
                    .Identifier => return true, // Type variable
                    else => return false,
                }
            },
            .Generic => |gen1| {
                switch (type2) {
                    .Generic => |gen2| {
                        if (!std.mem.eql(u8, gen1.name, gen2.name)) return false;
                        if (gen1.type_arguments.items.len != gen2.type_arguments.items.len) return false;
                        
                        for (gen1.type_arguments.items, gen2.type_arguments.items) |arg1, arg2| {
                            if (!self.typesAreCompatible(arg1, arg2)) return false;
                        }
                        return true;
                    },
                    .Identifier => return true, // Type variable
                    else => return false,
                }
            },
            else => return false,
        }
    }
};

/// Enhanced generic function call resolution
pub const GenericCallResolver = struct {
    inference_context: *TypeInferenceContext,
    monomorphizer: *generics.Monomorphizer,
    allocator: Allocator,
    
    // Scope information for type resolution
    current_scope: ?*ScopeInfo = null,
    global_types: HashMap([]const u8, ast.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    const ScopeInfo = struct {
        variables: HashMap([]const u8, ast.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        
        pub fn init() ScopeInfo {
            return ScopeInfo{
                .variables = HashMap([]const u8, ast.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            };
        }
        
        pub fn deinit(self: *ScopeInfo) void {
            self.variables.deinit(self.allocator);
        }
    };
    
    pub fn init(allocator: Allocator, inference_context: *TypeInferenceContext, monomorphizer: *generics.Monomorphizer) GenericCallResolver {
        return GenericCallResolver{
            .inference_context = inference_context,
            .monomorphizer = monomorphizer,
            .allocator = allocator,
            .global_types = HashMap([]const u8, ast.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
        };
    }
    
    pub fn deinit(self: *GenericCallResolver) void {
        if (self.current_scope) |scope| {
            scope.deinit();
        }
        self.global_types.deinit(self.allocator);
    }
    
    /// Resolve generic function call with automatic type inference
    pub fn resolveGenericCall(self: *GenericCallResolver, func_name: []const u8, args: []const ast.Expression, expected_return_type: ?ast.Type) !?[]const u8 {
        // Extract argument types
        var arg_types = std.ArrayList(u8){};
        defer arg_types.deinit();
        
        for (args) |arg| {
            const arg_type = try self.inferExpressionType(arg);
            try arg_types.append(self.allocator, arg_type);
        }
        
        // Attempt type inference
        const inferred_types = try self.inference_context.inferGenericFunctionCall(func_name, arg_types.items, expected_return_type);
        
        if (inferred_types) |type_args| {
            defer self.allocator.free(type_args);
            
            // Request monomorphization with inferred types
            return self.monomorphizer.requestInstantiation(func_name, type_args, "type_inferred");
        }
        
        return null; // Could not infer types
    }
    
    /// Infer the type of an expression (simplified)
    fn inferExpressionType(self: *GenericCallResolver, expr: ast.Expression) !ast.Type {
        return switch (expr) {
            .Literal => |literal| switch (literal) {
                .Integer => ast.Type{ .Primitive = .Normie },
                .Float => ast.Type{ .Primitive = .Meal },
                .String => ast.Type{ .Primitive = .Tea },
                .Boolean => ast.Type{ .Primitive = .Lit },
            },
            .Identifier => |identifier| {
                // Look up variable type
                return self.lookupVariableType(identifier.name);
            },
            .FunctionCall => |call| {
                // Recursively infer function call result type
                return self.inferFunctionCallType(call);
            },
            .ArrayLiteral => |array| {
                if (array.elements.items.len > 0) {
                    const element_type = try self.inferExpressionType(array.elements.items[0]);
                    const element_type_ptr = try self.allocator.create(ast.Type);
                    element_type_ptr.* = element_type;
                    return ast.Type{ .Array = ast.ArrayType{
                        .element_type = element_type_ptr,
                        .size = array.elements.items.len,
                    }};
                } else {
                    return error.CannotInferEmptyArrayType;
                }
            },
            else => error.CannotInferExpressionType,
        };
    }
    
    /// Look up variable type from context
    fn lookupVariableType(self: *GenericCallResolver, var_name: []const u8) !ast.Type {
        // Check local scope first
        if (self.current_scope) |scope| {
            var iter = scope.variables.iterator();
            while (iter.next()) |entry| {
                if (std.mem.eql(u8, entry.key_ptr.*, var_name)) {
                    return entry.value_ptr.*;
                }
            }
        }
        
        // Check global scope
        var iter = self.global_types.iterator();
        while (iter.next()) |entry| {
            if (std.mem.eql(u8, entry.key_ptr.*, var_name)) {
                return entry.value_ptr.*;
            }
        }
        
        // Default to normie if not found
        return ast.Type{ .Primitive = .Normie };
    }
    
    /// Infer function call result type with enhanced generic support
    fn inferFunctionCallType(self: *GenericCallResolver, call: ast.FunctionCall) !ast.Type {
        // First check for generic functions with proper type inference
        if (self.inference_context.monomorphizer.generic_declarations.get(call.name)) |generic_decl| {
            if (generic_decl.kind == .Function) {
                // Extract argument types for generic inference
                var arg_types = std.ArrayList(ast.Type){};
                defer arg_types.deinit();
                
                for (call.arguments.items) |arg| {
                    const arg_type = try self.inferExpressionType(arg);
                    try arg_types.append(allocator, arg_type);
                }
                
                // Attempt generic type inference
                const inferred_types = try self.inference_context.inferGenericFunctionCall(
                    call.name, 
                    arg_types.items, 
                    null // No expected return type constraint
                );
                
                if (inferred_types) |type_args| {
                    defer self.allocator.free(type_args);
                    
                    // Substitute type parameters in return type
                    const func_decl = generic_decl.ast_node.Function;
                    if (func_decl.return_type) |ret_type| {
                        return try self.substituteTypeParameters(ret_type, generic_decl.type_parameters.items, type_args);
                    }
                }
            }
        }
        
        // Look up monomorphic functions in registered functions
        var iter = self.global_types.iterator();
        while (iter.next()) |entry| {
            if (std.mem.eql(u8, entry.key_ptr.*, call.name)) {
                switch (entry.value_ptr.*) {
                    .Function => |func_type| return func_type.return_type.*,
                    else => continue,
                }
            }
        }
        
        // Check if it's a built-in function
        if (std.mem.eql(u8, call.name, "len")) {
            return ast.Type{ .Primitive = .Normie };
        } else if (std.mem.eql(u8, call.name, "spill")) {
            return ast.Type{ .Primitive = .Void };
        } else if (std.mem.startsWith(u8, call.name, "abs_")) {
            return ast.Type{ .Primitive = .Normie };
        }
        
        // Enhanced default inference with pattern matching
        return self.inferReturnTypeFromContext(call);
    }
    
    /// Substitute type parameters in a type expression
    fn substituteTypeParameters(self: *GenericCallResolver, type_expr: ast.Type, type_params: []const ast.TypeParameter, type_args: []const ast.Type) !ast.Type {
        switch (type_expr) {
            .Identifier => |ident| {
                // Check if this is a type parameter
                for (type_params, 0..) |param, i| {
                    if (std.mem.eql(u8, param.name, ident)) {
                        if (i < type_args.len) {
                            return type_args[i];
                        }
                    }
                }
                return type_expr; // Not a type parameter
            },
            .Array => |array_type| {
                const new_element_type = try self.substituteTypeParameters(array_type.element_type.*, type_params, type_args);
                const element_type_ptr = try self.allocator.create(ast.Type);
                element_type_ptr.* = new_element_type;
                return ast.Type{ .Array = ast.ArrayType{
                    .element_type = element_type_ptr,
                    .size = array_type.size,
                }};
            },
            .Slice => |slice_type| {
                const new_element_type = try self.substituteTypeParameters(slice_type.element_type.*, type_params, type_args);
                const element_type_ptr = try self.allocator.create(ast.Type);
                element_type_ptr.* = new_element_type;
                return ast.Type{ .Slice = ast.SliceType{
                    .element_type = element_type_ptr,
                }};
            },
            .Function => |func_type| {
                // Substitute in parameter and return types
                var new_param_types = std.ArrayList(ast.Type){};
                defer new_param_types.deinit(self.allocator);
                
                for (func_type.parameter_types.items) |param_type| {
                    const substituted = try self.substituteTypeParameters(param_type, type_params, type_args);
                    try new_param_types.append(self.allocator, substituted);
                }
                
                const new_return_type = try self.substituteTypeParameters(func_type.return_type.*, type_params, type_args);
                const return_type_ptr = try self.allocator.create(ast.Type);
                return_type_ptr.* = new_return_type;
                
                return ast.Type{ .Function = ast.FunctionType{
                    .parameter_types = try new_param_types.toOwnedSlice(self.allocator),
                    .return_type = return_type_ptr,
                }};
            },
            else => return type_expr, // No substitution needed
        }
    }
    
    /// Infer return type from call context and patterns
    fn inferReturnTypeFromContext(self: *GenericCallResolver, call: ast.FunctionCall) ast.Type {
        // Pattern-based inference for common function patterns
        if (std.mem.endsWith(u8, call.name, "_int") or std.mem.endsWith(u8, call.name, "_count")) {
            return ast.Type{ .Primitive = .Normie };
        } else if (std.mem.endsWith(u8, call.name, "_string") or std.mem.endsWith(u8, call.name, "_text")) {
            return ast.Type{ .Primitive = .Tea };
        } else if (std.mem.endsWith(u8, call.name, "_bool") or std.mem.endsWith(u8, call.name, "_check")) {
            return ast.Type{ .Primitive = .Lit };
        } else if (std.mem.endsWith(u8, call.name, "_float") or std.mem.endsWith(u8, call.name, "_decimal")) {
            return ast.Type{ .Primitive = .Meal };
        }
        
        // Default to normie for unknown functions
        return ast.Type{ .Primitive = .Normie };
    }
};

/// Integration with pattern matching for generic pattern inference
pub const PatternTypeInference = struct {
    inference_context: *TypeInferenceContext,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, inference_context: *TypeInferenceContext) PatternTypeInference {
        return PatternTypeInference{
            .inference_context = inference_context,
            .allocator = allocator,
        };
    }
    
    /// Infer type parameters from pattern matching
    pub fn inferFromPattern(self: *PatternTypeInference, pattern: ast.Pattern, matched_type: ast.Type) !void {
        switch (pattern) {
            .Variable => |var_pattern| {
                // Variable patterns capture the matched type
                if (var_pattern.type_hint) |hint| {
                    try self.inference_context.unifyTypes(hint, matched_type);
                }
            },
             0..) |sub_pattern, i| {
                                try self.inferFromPattern(sub_pattern, tuple_type.element_types[i]);
                            }
                        }
                    },
                    else => {},
                }
            },
            .Array => |array_pattern| {
                switch (matched_type) {
                    .Array => |array_type| {
                        for (array_pattern.patterns) |sub_pattern| {
                            try self.inferFromPattern(sub_pattern, array_type.element_type.*);
                        }
                    },
                    else => {},
                }
            },
            else => {},
        }
    }
};

// Test cases for type inference
test "basic generic function type inference" {
    var registry = type_system.GCTypeRegistry.init(std.testing.allocator);
    defer registry.deinit();
    
    var monomorphizer = generics.Monomorphizer.init(std.testing.allocator, null, null);
    defer monomorphizer.deinit();
    
    var inference_ctx = TypeInferenceContext.init(std.testing.allocator, &monomorphizer, &registry);
    defer inference_ctx.deinit();
    
    // Test inferring T from function call: foo[T](arg: T) with arg of type normie
    
    // Enhanced generic function constraint propagation
    try inference_ctx.addConstraint("T", ast.Type{ .Primitive = .Normie }, .Argument);
    try inference_ctx.addConstraint("T", ast.Type{ .Primitive = .Normie }, .ReturnType);
    
    // Propagate constraints iteratively with dependency tracking
    try inference_ctx.solveConstraintsWithPropagation();
    
    const inferred_type = inference_ctx.inferred_types.get("T");
    try std.testing.expect(inferred_type != null);
    try std.testing.expect(inferred_type.?.Primitive == .Normie);
}

test "array type inference" {
    var registry = type_system.GCTypeRegistry.init(std.testing.allocator);
    defer registry.deinit();
    
    var monomorphizer = generics.Monomorphizer.init(std.testing.allocator, null, null);
    defer monomorphizer.deinit();
    
    var inference_ctx = TypeInferenceContext.init(std.testing.allocator, &monomorphizer, &registry);
    defer inference_ctx.deinit();
    
    // Test array type unification
    const element_type = try std.testing.allocator.create(ast.Type);
    defer std.testing.allocator.destroy(element_type);
    element_type.* = ast.Type{ .Primitive = .Normie };
    
    const formal_type = ast.Type{ .Array = ast.ArrayType{
        .element_type = element_type,
        .size = 5,
    }};
    
    const actual_element_type = try std.testing.allocator.create(ast.Type);
    defer std.testing.allocator.destroy(actual_element_type);
    actual_element_type.* = ast.Type{ .Primitive = .Normie };
    
    const actual_type = ast.Type{ .Array = ast.ArrayType{
        .element_type = actual_element_type,
        .size = 5,
    }};
    
    // This should succeed without error
    try inference_ctx.unifyTypes(formal_type, actual_type);
}
