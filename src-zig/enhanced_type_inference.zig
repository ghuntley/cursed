//! Enhanced Type Inference System with Mutual Recursion Detection and Memoization
//! Fixes P6: Generic type inference crash (mutual recursion)

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const ast = @import("ast.zig");

/// Type inference errors
pub const TypeInferenceError = error{
    CyclicTypeDependency,
    CyclicTypeReference,
    InfiniteType,
    RecursionDepthExceeded,
    TypeMismatch,
    UnboundTypeVariable,
    OutOfMemory,
    ArityMismatch,
    VarianceViolation,
    ConstraintViolation,
    TypeArgumentCountMismatch,
    GenericInstantiationFailure,
    BoundConstraintViolation,
    UnificationFailure,
    MismatchedTypes,
    UnsupportedTypeOperation,
    MemoryAllocationFailure,
};

/// Type variable with unique ID for tracking
pub const TypeVariable = struct {
    id: u32,
    name: ?[]const u8 = null,
    bounds: ArrayList(ast.Type),
    instantiated: ?ast.Type = null,
    
    pub fn init(allocator: Allocator, id: u32) TypeVariable {
        return TypeVariable{
            .id = id,
            .bounds = .empty,
        };
    }
    
    pub fn deinit(self: *TypeVariable) void {
        self.bounds.deinit();
    }
};

/// Type constraint for unification
pub const TypeConstraint = struct {
    left: ast.Type,
    right: ast.Type,
    origin: []const u8, // Error reporting
    
    pub fn init(left: ast.Type, right: ast.Type, origin: []const u8) TypeConstraint {
        return TypeConstraint{
            .left = left,
            .right = right,
            .origin = origin,
        };
    }
};

/// Recursion detection state
pub const RecursionDetector = struct {
    visiting: HashMap(u32, bool, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage),
    visited: HashMap(u32, bool, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage),
    recursion_depth: u32,
    max_depth: u32,
    
    const MAX_RECURSION_DEPTH = 1000;
    
    pub fn init() RecursionDetector {
        return RecursionDetector{
            .visiting = HashMap(u32, bool, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .visited = HashMap(u32, bool, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .recursion_depth = 0,
            .max_depth = MAX_RECURSION_DEPTH,
        };
    }
    
    pub fn deinit(self: *RecursionDetector, allocator: Allocator) void {
        self.visiting.deinit();
        self.visited.deinit();
    }
    
    /// Check if entering a type variable would create a cycle
    pub fn checkCycle(self: *RecursionDetector, type_var_id: u32) TypeInferenceError!bool {
        if (self.recursion_depth >= self.max_depth) {
            return TypeInferenceError.RecursionDepthExceeded;
        }
        
        if (self.visiting.contains(type_var_id)) {
            return TypeInferenceError.CyclicTypeDependency;
        }
        
        return false;
    }
    
    /// Enter a type variable for processing
    pub fn enter(self: *RecursionDetector, type_var_id: u32) TypeInferenceError!void {
        if (try self.checkCycle(type_var_id)) {
            return TypeInferenceError.CyclicTypeDependency;
        }
        
        try self.visiting.put(type_var_id, {});
        self.recursion_depth += 1;
    }
    
    /// Exit a type variable after processing
    pub fn exit(self: *RecursionDetector, type_var_id: u32) void {
        _ = self.visiting.remove(type_var_id);
        // Never fail on exit - just log if visited tracking fails
        self.visited.put(type_var_id, {}) catch |err| {
            std.log.warn("Failed to track visited type variable {}: {}", .{ type_var_id, err });
        };
        self.recursion_depth -= 1;
    }
    
    /// Check if already visited (for memoization)
    pub fn isVisited(self: *RecursionDetector, type_var_id: u32) bool {
        return self.visited.contains(type_var_id);
    }
};

/// Type memoization cache for performance
pub const TypeMemoization = struct {
    unification_cache: HashMap(UnificationKey, ast.Type, UnificationKeyContext, std.hash_map.default_max_load_percentage),
    substitution_cache: HashMap(SubstitutionKey, ast.Type, SubstitutionKeyContext, std.hash_map.default_max_load_percentage),
    
    const UnificationKey = struct {
        left_hash: u64,
        right_hash: u64,
    };
    
    const SubstitutionKey = struct {
        type_hash: u64,
        var_id: u32,
        replacement_hash: u64,
    };
    
    const UnificationKeyContext = struct {
        pub fn hash(self: @This(), key: UnificationKey) u64 {
            _ = self;
            var hasher = std.hash.Wyhash.init(0);
            hasher.update(std.mem.asBytes(&key.left_hash));
            hasher.update(std.mem.asBytes(&key.right_hash));
            return hasher.final();
        }
        
        pub fn eql(self: @This(), a: UnificationKey, b: UnificationKey) bool {
            _ = self;
            return a.left_hash == b.left_hash and a.right_hash == b.right_hash;
        }
    };
    
    const SubstitutionKeyContext = struct {
        pub fn hash(self: @This(), key: SubstitutionKey) u64 {
            _ = self;
            var hasher = std.hash.Wyhash.init(0);
            hasher.update(std.mem.asBytes(&key.type_hash));
            hasher.update(std.mem.asBytes(&key.var_id));
            hasher.update(std.mem.asBytes(&key.replacement_hash));
            return hasher.final();
        }
        
        pub fn eql(self: @This(), a: SubstitutionKey, b: SubstitutionKey) bool {
            _ = self;
            return a.type_hash == b.type_hash and a.var_id == b.var_id and a.replacement_hash == b.replacement_hash;
        }
    };
    
    pub fn init() TypeMemoization {
        return TypeMemoization{
            .unification_cache = HashMap(UnificationKey, ast.Type, UnificationKeyContext, std.hash_map.default_max_load_percentage).init(allocator),
            .substitution_cache = HashMap(SubstitutionKey, ast.Type, SubstitutionKeyContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *TypeMemoization) void {
        self.unification_cache.deinit();
        self.substitution_cache.deinit();
    }
    
    /// Hash a type for memoization
    pub fn hashType(self: *TypeMemoization, type_info: ast.Type) u64 {
        _ = self;
        var hasher = std.hash.Wyhash.init(0);
        
        switch (type_info) {
            .Basic => |basic| {
                hasher.update("Basic");
                hasher.update(std.mem.asBytes(&basic));
            },
            .Custom => |name| {
                hasher.update("Custom");
                hasher.update(name);
            },
            .Array => |array| {
                hasher.update("Array");
                hasher.update(std.mem.asBytes(&array.size));
                // Recursive hash would go here
            },
            .Pointer => |ptr| {
                hasher.update("Pointer");
                // Recursive hash of target type would go here
                _ = ptr;
            },
            else => {
                hasher.update("Other");
            }
        }
        
        return hasher.final();
    }
    
    /// Try to get cached unification result
    pub fn getUnification(self: *TypeMemoization, left: ast.Type, right: ast.Type) ?ast.Type {
        const key = UnificationKey{
            .left_hash = self.hashType(left),
            .right_hash = self.hashType(right),
        };
        return self.unification_cache.get(key);
    }
    
    /// Cache unification result
    pub fn cacheUnification(self: *TypeMemoization, left: ast.Type, right: ast.Type, result: ast.Type) !void {
        const key = UnificationKey{
            .left_hash = self.hashType(left),
            .right_hash = self.hashType(right),
        };
        try self.unification_cache.put(key, result);
    }
};

/// Enhanced Type Inference Engine
pub const TypeInferenceEngine = struct {
    allocator: Allocator,
    type_variables: HashMap(u32, TypeVariable, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage),
    constraints: ArrayList(TypeConstraint),
    recursion_detector: RecursionDetector,
    memoization: TypeMemoization,
    next_var_id: u32,
    
    pub fn init() TypeInferenceEngine {
        return TypeInferenceEngine{
            .allocator = allocator,
            .type_variables = HashMap(u32, TypeVariable, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .constraints = .empty,
            .recursion_detector = RecursionDetector.init(allocator),
            .memoization = TypeMemoization.init(allocator),
            .next_var_id = 1,
        };
    }
    
    pub fn deinit(self: *TypeInferenceEngine) void {
        var iter = self.type_variables.iterator();
        while (iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.type_variables.deinit();
        self.constraints.deinit();
        self.recursion_detector.deinit();
        self.memoization.deinit();
    }
    
    /// Create a fresh type variable
    pub fn createTypeVariable(self: *TypeInferenceEngine, name: ?[]const u8) !u32 {
        const var_id = self.next_var_id;
        self.next_var_id += 1;
        
        var type_var = TypeVariable.init(self.allocator, var_id);
        type_var.name = name;
        
        try self.type_variables.put(var_id, type_var);
        return var_id;
    }
    
    /// Add a type constraint
    pub fn addConstraint(self: *TypeInferenceEngine, left: ast.Type, right: ast.Type, origin: []const u8) !void {
        const constraint = TypeConstraint.init(left, right, origin);
        try self.constraints.append(self.allocator, constraint);
    }
    
    /// Unify two types with recursion detection and memoization
    pub fn unify(self: *TypeInferenceEngine, left: ast.Type, right: ast.Type) TypeInferenceError!ast.Type {
        // Check memoization cache first
        if (self.memoization.getUnification(left, right)) |cached| {
            return cached;
        }
        
        const result = try self.unifyInternal(left, right);
        
        // Cache the result
        try self.memoization.cacheUnification(left, right, result);
        
        return result;
    }
    
    /// Internal unification implementation
    fn unifyInternal(self: *TypeInferenceEngine, left: ast.Type, right: ast.Type) TypeInferenceError!ast.Type {
        // Handle identical types
        if (std.meta.eql(left, right)) {
            return left;
        }
        
        switch (left) {
            .Basic => |left_basic| {
                switch (right) {
                    .Basic => |right_basic| {
                        if (left_basic == right_basic) {
                            return left;
                        } else {
                            return TypeInferenceError.TypeMismatch;
                        }
                    },
                    else => return TypeInferenceError.TypeMismatch,
                }
            },
            .Custom => |left_name| {
                switch (right) {
                    .Custom => |right_name| {
                        if (std.mem.eql(u8, left_name, right_name)) {
                            return left;
                        } else {
                            return TypeInferenceError.TypeMismatch;
                        }
                    },
                    else => return TypeInferenceError.TypeMismatch,
                }
            },
            .Array => |left_array| {
                switch (right) {
                    .Array => |right_array| {
                        if (left_array.size != right_array.size) {
                            return TypeInferenceError.TypeMismatch;
                        }
                        
                        // Recursively unify element types with cycle detection
                        const unified_element = try self.unify(left_array.element_type.*, right_array.element_type.*);
                        
                        // Create new array type with unified element type
                        const unified_element_ptr = try self.allocator.create(ast.Type);
                        unified_element_ptr.* = unified_element;
                        
                        return ast.Type{
                            .Array = .{
                                .element_type = unified_element_ptr,
                                .size = left_array.size,
                            }
                        };
                    },
                    else => return TypeInferenceError.TypeMismatch,
                }
            },
            .Pointer => |left_ptr| {
                switch (right) {
                    .Pointer => |right_ptr| {
                        // Recursively unify target types
                        const unified_target = try self.unify(left_ptr.target_type.*, right_ptr.target_type.*);
                        
                        const unified_target_ptr = try self.allocator.create(ast.Type);
                        unified_target_ptr.* = unified_target;
                        
                        return ast.Type{
                            .Pointer = .{
                                .target_type = unified_target_ptr,
                            }
                        };
                    },
                    else => return TypeInferenceError.TypeMismatch,
                }
            },
            .Function => |left_func| {
                switch (right) {
                    .Function => |right_func| {
                        // Check parameter count
                        if (left_func.parameters.items.len != right_func.parameters.items.len) {
                            return TypeInferenceError.TypeMismatch;
                        }
                        
                        // Unify parameters
                        var unified_params = .empty;
                        for (left_func.parameters.items, right_func.parameters.items) |left_param, right_param| {
                            const unified_param = try self.unify(left_param, right_param);
                            try unified_params.append(self.allocator, unified_param);
                        }
                        
                        // Unify return types
                        const unified_return = if (left_func.return_type != null and right_func.return_type != null)
                            try self.allocator.create(ast.Type)
                        else
                            null;
                            
                        if (unified_return) |return_ptr| {
                            return_ptr.* = try self.unify(left_func.return_type.?.*, right_func.return_type.?.*);
                        }
                        
                        return ast.Type{
                            .Function = .{
                                .parameters = unified_params,
                                .return_type = unified_return,
                            }
                        };
                    },
                    else => return TypeInferenceError.TypeMismatch,
                }
            },
            .Generic => |left_generic| {
                switch (right) {
                    .Generic => |right_generic| {
                        if (!std.mem.eql(u8, left_generic.name, right_generic.name)) {
                            return TypeInferenceError.TypeMismatch;
                        }
                        
                        if (left_generic.type_arguments.items.len != right_generic.type_arguments.items.len) {
                            return TypeInferenceError.TypeMismatch;
                        }
                        
                        // Unify type arguments
                        var unified_args = .empty;
                        for (left_generic.type_arguments.items, right_generic.type_arguments.items) |left_arg, right_arg| {
                            const unified_arg = try self.unify(left_arg, right_arg);
                            try unified_args.append(unified_arg);
                        }
                        
                        return ast.Type{
                            .Generic = .{
                                .name = left_generic.name,
                                .type_arguments = unified_args,
                            }
                        };
                    },
                    else => return TypeInferenceError.TypeMismatch,
                }
            },
            else => return TypeInferenceError.TypeMismatch,
        }
    }
    
    /// Perform occurs check to prevent infinite types
    pub fn occursCheck(self: *TypeInferenceEngine, var_id: u32, type_info: ast.Type) TypeInferenceError!bool {
        try self.recursion_detector.enter(var_id);
        defer self.recursion_detector.exit(var_id);
        
        const result = try self.occursCheckInternal(var_id, type_info);
        return result;
    }
    
    fn occursCheckInternal(self: *TypeInferenceEngine, var_id: u32, type_info: ast.Type) TypeInferenceError!bool {
        switch (type_info) {
            .Array => |array| {
                return self.occursCheckInternal(var_id, array.element_type.*);
            },
            .Pointer => |ptr| {
                return self.occursCheckInternal(var_id, ptr.target_type.*);
            },
            .Function => |func| {
                // Check parameters
                for (func.parameters.items) |param| {
                    if (try self.occursCheckInternal(var_id, param)) {
                        return true;
                    }
                }
                
                // Check return type
                if (func.return_type) |return_type| {
                    return self.occursCheckInternal(var_id, return_type.*);
                }
                
                return false;
            },
            .Generic => |generic| {
                for (generic.type_arguments.items) |arg| {
                    if (try self.occursCheckInternal(var_id, arg)) {
                        return true;
                    }
                }
                return false;
            },
            .Custom => |name| {
                // Check if this is actually a type variable reference
                if (std.fmt.parseInt(u32, name, 10)) |parsed_id| {
                    return parsed_id == var_id;
                } else |_| {
                    return false;
                }
            },
            else => return false,
        }
    }
    
    /// Substitute type variable with concrete type
    pub fn substitute(self: *TypeInferenceEngine, var_id: u32, replacement: ast.Type, in_type: ast.Type) TypeInferenceError!ast.Type {
        // Perform occurs check first
        if (try self.occursCheck(var_id, replacement)) {
            return TypeInferenceError.InfiniteType;
        }
        
        return self.substituteInternal(var_id, replacement, in_type);
    }
    
    fn substituteInternal(self: *TypeInferenceEngine, var_id: u32, replacement: ast.Type, in_type: ast.Type) TypeInferenceError!ast.Type {
        switch (in_type) {
            .Custom => |name| {
                if (std.fmt.parseInt(u32, name, 10)) |parsed_id| {
                    if (parsed_id == var_id) {
                        return replacement;
                    }
                } else |_| {}
                return in_type;
            },
            .Array => |array| {
                const new_element = try self.substituteInternal(var_id, replacement, array.element_type.*);
                const new_element_ptr = try self.allocator.create(ast.Type);
                new_element_ptr.* = new_element;
                
                return ast.Type{
                    .Array = .{
                        .element_type = new_element_ptr,
                        .size = array.size,
                    }
                };
            },
            .Pointer => |ptr| {
                const new_target = try self.substituteInternal(var_id, replacement, ptr.target_type.*);
                const new_target_ptr = try self.allocator.create(ast.Type);
                new_target_ptr.* = new_target;
                
                return ast.Type{
                    .Pointer = .{
                        .target_type = new_target_ptr,
                    }
                };
            },
            .Function => |func| {
                var new_params = .empty;
                for (func.parameters.items) |param| {
                    const new_param = try self.substituteInternal(var_id, replacement, param);
                    try new_params.append(self.allocator, new_param);
                }
                
                const new_return = if (func.return_type) |return_type| blk: {
                    const ptr = try self.allocator.create(ast.Type);
                    ptr.* = try self.substituteInternal(var_id, replacement, return_type.*);
                    break :blk ptr;
                } else null;
                
                return ast.Type{
                    .Function = .{
                        .parameters = new_params,
                        .return_type = new_return,
                    }
                };
            },
            else => return in_type,
        }
    }
    
    /// Solve all accumulated constraints
    pub fn solveConstraints(self: *TypeInferenceEngine) TypeInferenceError!void {
        var changed = true;
        var iterations: u32 = 0;
        const max_iterations = 1000;
        
        while (changed and iterations < max_iterations) {
            changed = false;
            iterations += 1;
            
            for (self.constraints.items) |constraint| {
                // Try to unify the constraint
                const unified = self.unify(constraint.left, constraint.right) catch |err| switch (err) {
                    TypeInferenceError.TypeMismatch => {
                        std.debug.print("Type constraint failed at: {s}\n", .{constraint.origin});
                        return err;
                    },
                    else => return err,
                };
                
                // Apply any substitutions that result from unification
                _ = unified; // Use the unified type somehow
                changed = true;
            }
        }
        
        if (iterations >= max_iterations) {
            return TypeInferenceError.RecursionDepthExceeded;
        }
    }
    
    /// Get the final type for a type variable
    pub fn resolveTypeVariable(self: *TypeInferenceEngine, var_id: u32) ?ast.Type {
        const type_var = self.type_variables.get(var_id) orelse return null;
        return type_var.instantiated;
    }
    
    /// Debug print type inference state
    pub fn debugPrint(self: *TypeInferenceEngine) void {
        std.debug.print("=== Type Inference State ===\n", .{});
        std.debug.print("Type Variables: {d}\n", .{self.type_variables.count()});
        std.debug.print("Constraints: {d}\n", .{self.constraints.items.len});
        std.debug.print("Recursion Depth: {d}\n", .{self.recursion_detector.recursion_depth});
        std.debug.print("Cache Entries: {d}\n", .{self.memoization.unification_cache.count()});
        std.debug.print("============================\n", .{});
    }
};

/// High-level type inference interface
pub fn inferTypes(allocator: Allocator, expressions: []const ast.Expression) TypeInferenceError![]ast.Type {
    var engine = TypeInferenceEngine.init(allocator);
    defer engine.deinit();
    
    var result_types = .empty;
    
    // Generate constraints for each expression
    for (expressions) |expr| {
        const expr_type = try inferExpressionType(&engine, expr);
        try result_types.append(expr_type);
    }
    
    // Solve all constraints
    try engine.solveConstraints();
    
    return result_types.toOwnedSlice();
}

/// Infer type of a single expression
fn inferExpressionType(engine: *TypeInferenceEngine, expr: ast.Expression) TypeInferenceError!ast.Type {
    switch (expr) {
        .Integer => return ast.Type{ .Basic = .Drip },
        .String => return ast.Type{ .Basic = .Tea },
        .Boolean => return ast.Type{ .Basic = .Lit },
        .Identifier => |name| {
            // Create type variable for unknown identifiers
            const var_id = try engine.createTypeVariable(name);
            const var_name = try std.fmt.allocPrint(engine.allocator, "{d}", .{var_id});
            return ast.Type{ .Custom = var_name };
        },
        .FunctionCall => |call| {
            // Infer function type and argument types with advanced constraint generation
            const func_type = try inferExpressionType(engine, call.function.*);
            
            var arg_types: std.ArrayList(ast.Type) = .empty;
            defer arg_types.deinit();
            
            for (call.arguments.items) |arg| {
                const arg_type = try inferExpressionType(engine, arg);
                try arg_types.append(arg_type);
            }
            
            // Generate complex constraints for advanced type scenarios
            const constraints = try generateComplexConstraints(engine, func_type, arg_types.items);
            
            // Advanced cycle detection with memoization
            if (try detectTypeCycleWithMemo(engine, func_type, constraints)) {
                return TypeInferenceError.CyclicTypeReference;
            }
            
            // Enhanced variance checking for nested generics and multiple constraints
            if (try checkAdvancedVarianceConstraints(engine, func_type, arg_types.items, constraints)) |validated_type| {
                return validated_type;
            }
            
            // Fallback with proper constraint resolution
            return try resolveFallbackWithConstraints(engine, func_type, arg_types.items)
        },
        else => {
            // Create fresh type variable for unknown expressions
            const var_id = try engine.createTypeVariable(null);
            const var_name = try std.fmt.allocPrint(engine.allocator, "{d}", .{var_id});
            return ast.Type{ .Custom = var_name };
        },
    }
}

test "type inference with recursion detection" {
    const allocator = std.testing.allocator;
    
    var engine = TypeInferenceEngine.init(allocator);
    defer engine.deinit();
    
    // Create two type variables that would form a cycle
    const var1 = try engine.createTypeVariable("T1");
    const var2 = try engine.createTypeVariable("T2");
    
    // Test occurs check
    const var1_name = try std.fmt.allocPrint(allocator, "{d}", .{var1});
    defer allocator.free(var1_name);
    
    const occurs = try engine.occursCheck(var1, ast.Type{ .Custom = var1_name });
    try std.testing.expect(occurs);
}

test "type memoization" {
    const allocator = std.testing.allocator;
    
    var memoization = TypeMemoization.init(allocator);
    defer memoization.deinit();
    
    const type1 = ast.Type{ .Basic = .Drip };
    const type2 = ast.Type{ .Basic = .Tea };
    
    const hash1 = memoization.hashType(type1);
    const hash2 = memoization.hashType(type2);
    
    try std.testing.expect(hash1 != hash2);
}

/// Detect type cycles in function call resolution
fn detectTypeCycle(engine: *TypeInferenceEngine, func_type: ast.Type) !bool {
    switch (func_type) {
        .Custom => |name| {
            // Check if we've already started processing this type
            if (engine.recursion_detector.checkCycle(name)) {
                return true;
            }
            
            engine.recursion_detector.enter(name);
            defer engine.recursion_detector.exit(name);
            
            return false; // No cycle detected
        },
        .Generic => |generic| {
            for (generic.type_args.items) |arg| {
                if (try detectTypeCycle(engine, arg)) {
                    return true;
                }
            }
            return false;
        },
        else => return false,
    }
}

/// Variance constraints for function type checking
fn checkVarianceConstraints(engine: *TypeInferenceEngine, func_type: ast.Type, arg_types: []ast.Type) !?ast.Type {
    switch (func_type) {
        .Function => |func| {
            // Check parameter variance (contravariant)
            if (func.parameters.items.len != arg_types.len) {
                return error.ArityMismatch;
            }
            
            for (func.parameters.items, 0..) |param, i| {
                // Contravariant check: arg_type must be subtype of param_type
                if (!isSubtype(arg_types[i], param.parameter_type)) {
                    return error.VarianceViolation;
                }
            }
            
            // Return type is covariant
            return func.return_type;
        },
        .Generic => |generic| {
            // Generic variance checking with constraints
            var resolved_args: std.ArrayList(ast.Type) = .empty;
            defer resolved_args.deinit();
            
            for (arg_types) |arg_type| {
                // Resolve generic type parameters with variance constraints
                const resolved = try resolveGenericWithVariance(engine, generic, arg_type);
                try resolved_args.append(resolved);
            }
            
            return ast.Type{ .Generic = .{
                .name = generic.name,
                .type_args = resolved_args,
            }};
        },
        else => return null,
    }
}

/// Subtype checking for variance validation
fn isSubtype(subtype: ast.Type, supertype: ast.Type) bool {
    switch (subtype) {
        .Basic => |sub_basic| {
            switch (supertype) {
                .Basic => |super_basic| return sub_basic == super_basic,
                else => return false,
            }
        },
        .Primitive => |sub_prim| {
            switch (supertype) {
                .Primitive => |super_prim| return sub_prim == super_prim,
                else => return false,
            }
        },
        else => return false, // Conservative approach
    }
}

/// Complex constraint container for advanced type scenarios
const ComplexConstraints = struct {
    nested_generics: std.ArrayList(NestedGenericConstraint),
    variance_constraints: std.ArrayList(VarianceConstraint),
    bound_constraints: std.ArrayList(BoundConstraint),
    
    const NestedGenericConstraint = struct {
        outer_type: ast.Type,
        inner_types: []ast.Type,
        depth_level: u32,
    };
    
    const VarianceConstraint = struct {
        type_var: u32,
        variance: Variance,
        required_bound: ?ast.Type,
    };
    
    const BoundConstraint = struct {
        type_var: u32,
        lower_bound: ?ast.Type,
        upper_bound: ?ast.Type,
    };
};

/// Generate complex constraints for advanced type scenarios
fn generateComplexConstraints(engine: *TypeInferenceEngine, func_type: ast.Type, arg_types: []ast.Type) !ComplexConstraints {
    var constraints = ComplexConstraints{
        .nested_generics = std..empty,
        .variance_constraints = std..empty,
        .bound_constraints = std..empty,
    };
    
    // Analyze nested generics depth and constraints
    switch (func_type) {
        .Generic => |generic| {
            const depth = try analyzeNestedGenericDepth(generic, 0);
            if (depth > 3) { // Complex nesting detected
                const inner_types = try extractNestedGenericTypes(engine.allocator, generic);
                try constraints.nested_generics.append(.{
                    .outer_type = func_type,
                    .inner_types = inner_types,
                    .depth_level = depth,
                });
            }
        },
        .Function => |func| {
            // Generate constraints for function parameter variance
            for (func.parameters.items, 0..) |param, i| {
                if (i < arg_types.len) {
                    const var_constraint = try generateVarianceConstraint(engine, param.parameter_type, arg_types[i]);
                    if (var_constraint) |vc| {
                        try constraints.variance_constraints.append(vc);
                    }
                }
            }
        },
        else => {},
    }
    
    return constraints;
}

/// Analyze depth of nested generic types
fn analyzeNestedGenericDepth(generic: ast.GenericType, current_depth: u32) !u32 {
    var max_depth = current_depth;
    
    for (generic.type_args.items) |arg| {
        switch (arg) {
            .Generic => |nested_generic| {
                const nested_depth = try analyzeNestedGenericDepth(nested_generic, current_depth + 1);
                max_depth = @max(max_depth, nested_depth);
            },
            else => {},
        }
    }
    
    return max_depth;
}

/// Extract nested generic types for constraint analysis
fn extractNestedGenericTypes(allocator: Allocator, generic: ast.GenericType) ![]ast.Type {
    var types: std.ArrayList(ast.Type) = .empty;
    
    for (generic.type_args.items) |arg| {
        try types.append(arg);
        switch (arg) {
            .Generic => |nested| {
                const nested_types = try extractNestedGenericTypes(allocator, nested);
                for (nested_types) |nested_type| {
                    try types.append(nested_type);
                }
            },
            else => {},
        }
    }
    
    return types.toOwnedSlice();
}

/// Generate variance constraint for type pairs
fn generateVarianceConstraint(engine: *TypeInferenceEngine, param_type: ast.Type, arg_type: ast.Type) !?ComplexConstraints.VarianceConstraint {
    switch (param_type) {
        .Generic => |generic| {
            const var_id = try engine.createTypeVariable(generic.name);
            return ComplexConstraints.VarianceConstraint{
                .type_var = var_id,
                .variance = .Contravariant, // Function parameters are contravariant
                .required_bound = arg_type,
            };
        },
        else => return null,
    }
}

/// Advanced cycle detection with memoization and constraint awareness
fn detectTypeCycleWithMemo(engine: *TypeInferenceEngine, func_type: ast.Type, constraints: ComplexConstraints) !bool {
    // Check memoization cache first
    const memo_key = TypeMemoization.UnificationKey{ .left = func_type, .right = func_type };
    if (engine.type_memo.unification_cache.get(memo_key)) |_| {
        return false; // Already processed, no cycle
    }
    
    // Enhanced cycle detection considering constraints
    const has_cycle = try detectTypeCycle(engine, func_type);
    if (has_cycle) return true;
    
    // Check nested generic constraints for cycles
    for (constraints.nested_generics.items) |nested_constraint| {
        for (nested_constraint.inner_types) |inner_type| {
            if (try detectTypeCycle(engine, inner_type)) {
                return true;
            }
        }
    }
    
    // Cache negative result
    try engine.type_memo.unification_cache.put(memo_key, func_type);
    return false;
}

/// Enhanced variance checking for nested generics and multiple constraints
fn checkAdvancedVarianceConstraints(engine: *TypeInferenceEngine, func_type: ast.Type, arg_types: []ast.Type, constraints: ComplexConstraints) !?ast.Type {
    // First perform standard variance checking
    const standard_result = try checkVarianceConstraints(engine, func_type, arg_types);
    if (standard_result) |result| {
        // Validate against complex constraints
        if (try validateComplexConstraints(engine, result, constraints)) {
            return result;
        }
    }
    
    // Handle nested generic constraints
    for (constraints.nested_generics.items) |nested_constraint| {
        const resolved_nested = try resolveNestedGenericConstraints(engine, nested_constraint);
        if (resolved_nested) |resolved| {
            return resolved;
        }
    }
    
    // Handle variance constraints with bounds
    for (constraints.variance_constraints.items) |var_constraint| {
        if (var_constraint.required_bound) |bound| {
            const resolved_bound = try resolveBoundConstraint(engine, var_constraint.type_var, bound);
            if (resolved_bound) |resolved| {
                return resolved;
            }
        }
    }
    
    return null;
}

/// Validate complex constraints against resolved type
fn validateComplexConstraints(engine: *TypeInferenceEngine, resolved_type: ast.Type, constraints: ComplexConstraints) !bool {
    // Check nested generic depth limits
    for (constraints.nested_generics.items) |nested_constraint| {
        if (nested_constraint.depth_level > 10) { // Prevent excessive nesting
            return false;
        }
        
        // Verify type compatibility at each nesting level
        if (!try isTypeCompatibleWithNesting(resolved_type, nested_constraint)) {
            return false;
        }
    }
    
    // Check variance constraint satisfaction
    for (constraints.variance_constraints.items) |var_constraint| {
        if (!try isVarianceConstraintSatisfied(engine, resolved_type, var_constraint)) {
            return false;
        }
    }
    
    return true;
}

/// Check type compatibility with nested generic constraints
fn isTypeCompatibleWithNesting(resolved_type: ast.Type, nested_constraint: ComplexConstraints.NestedGenericConstraint) !bool {
    switch (resolved_type) {
        .Generic => |generic| {
            // Check if resolved generic matches expected structure
            if (generic.type_args.items.len != nested_constraint.inner_types.len) {
                return false;
            }
            
            for (generic.type_args.items, 0..) |arg, i| {
                if (!areTypesCompatible(arg, nested_constraint.inner_types[i])) {
                    return false;
                }
            }
            return true;
        },
        else => return nested_constraint.depth_level == 0, // Only compatible if no nesting expected
    }
}

/// Check if variance constraint is satisfied
fn isVarianceConstraintSatisfied(engine: *TypeInferenceEngine, resolved_type: ast.Type, constraint: ComplexConstraints.VarianceConstraint) !bool {
    if (constraint.required_bound) |bound| {
        switch (constraint.variance) {
            .Covariant => return isSubtype(resolved_type, bound),
            .Contravariant => return isSubtype(bound, resolved_type),
            .Invariant => return areTypesEqual(resolved_type, bound),
            .Bivariant => return true, // Bivariant accepts any type
        }
    }
    return true;
}

/// Resolve nested generic constraints
fn resolveNestedGenericConstraints(engine: *TypeInferenceEngine, nested_constraint: ComplexConstraints.NestedGenericConstraint) !?ast.Type {
    // Create fresh type variables for deeply nested generics
    var resolved_args: std.ArrayList(ast.Type) = .empty;
    defer resolved_args.deinit();
    
    for (nested_constraint.inner_types) |inner_type| {
        const resolved_inner = try resolveTypeWithFreshVars(engine, inner_type);
        try resolved_args.append(resolved_inner);
    }
    
    switch (nested_constraint.outer_type) {
        .Generic => |generic| {
            return ast.Type{ .Generic = .{
                .name = generic.name,
                .type_args = try resolved_args.toOwnedSlice(),
            }};
        },
        else => return null,
    }
}

/// Resolve bound constraint for type variable
fn resolveBoundConstraint(engine: *TypeInferenceEngine, type_var_id: u32, bound: ast.Type) !?ast.Type {
    if (engine.type_variables.get(type_var_id)) |type_var| {
        // Check if bound is compatible with existing constraints
        for (type_var.constraints.items) |constraint| {
            if (!areTypesCompatible(bound, constraint)) {
                return TypeInferenceError.ConstraintViolation;
            }
        }
        
        // Update type variable with resolved bound
        var updated_var = type_var;
        updated_var.resolved_type = bound;
        try engine.type_variables.put(type_var_id, updated_var);
        
        return bound;
    }
    
    return null;
}

/// Resolve type with fresh type variables
fn resolveTypeWithFreshVars(engine: *TypeInferenceEngine, type_to_resolve: ast.Type) !ast.Type {
    switch (type_to_resolve) {
        .Generic => |generic| {
            var fresh_args: std.ArrayList(ast.Type) = .empty;
            defer fresh_args.deinit();
            
            for (generic.type_args.items) |arg| {
                const fresh_arg = try resolveTypeWithFreshVars(engine, arg);
                try fresh_args.append(fresh_arg);
            }
            
            return ast.Type{ .Generic = .{
                .name = generic.name,
                .type_args = try fresh_args.toOwnedSlice(),
            }};
        },
        .Custom => |name| {
            // Create fresh type variable
            const var_id = try engine.createTypeVariable(name);
            const var_name = try std.fmt.allocPrint(engine.allocator, "fresh_{s}_{d}", .{ name, var_id });
            return ast.Type{ .Custom = var_name };
        },
        else => return type_to_resolve,
    }
}

/// Fallback resolution with constraint awareness
fn resolveFallbackWithConstraints(engine: *TypeInferenceEngine, func_type: ast.Type, arg_types: []ast.Type) !ast.Type {
    // Try to infer return type from function signature
    switch (func_type) {
        .Function => |func| return func.return_type.*,
        .Generic => |generic| {
            // For generic functions, try to instantiate with argument types
            if (arg_types.len > 0) {
                return try instantiateGenericType(engine, generic, arg_types[0]);
            }
        },
        else => {},
    }
    
    // Ultimate fallback - create constrained type variable
    const var_id = try engine.createTypeVariable("fallback");
    const var_name = try std.fmt.allocPrint(engine.allocator, "fallback_{d}", .{var_id});
    return ast.Type{ .Custom = var_name };
}

/// Instantiate generic type with concrete argument
fn instantiateGenericType(engine: *TypeInferenceEngine, generic: ast.GenericType, concrete_type: ast.Type) !ast.Type {
    // Simple instantiation - replace first type parameter with concrete type
    if (generic.type_args.items.len > 0) {
        var instantiated_args: std.ArrayList(ast.Type) = .empty;
        defer instantiated_args.deinit();
        
        try instantiated_args.append(concrete_type);
        for (generic.type_args.items[1..]) |arg| {
            try instantiated_args.append(arg);
        }
        
        return ast.Type{ .Generic = .{
            .name = generic.name,
            .type_args = try instantiated_args.toOwnedSlice(),
        }};
    }
    
    return ast.Type{ .Generic = generic };
}

/// Check if two types are equal
fn areTypesEqual(type1: ast.Type, type2: ast.Type) bool {
    switch (type1) {
        .Basic => |basic1| {
            switch (type2) {
                .Basic => |basic2| return basic1 == basic2,
                else => return false,
            }
        },
        .Primitive => |prim1| {
            switch (type2) {
                .Primitive => |prim2| return prim1 == prim2,
                else => return false,
            }
        },
        .Custom => |name1| {
            switch (type2) {
                .Custom => |name2| return std.mem.eql(u8, name1, name2),
                else => return false,
            }
        },
        .Generic => |gen1| {
            switch (type2) {
                .Generic => |gen2| {
                    if (!std.mem.eql(u8, gen1.name, gen2.name)) return false;
                    if (gen1.type_args.items.len != gen2.type_args.items.len) return false;
                    for (gen1.type_args.items, 0..) |arg1, i| {
                        if (!areTypesEqual(arg1, gen2.type_args.items[i])) return false;
                    }
                    return true;
                },
                else => return false,
            }
        },
        else => return false,
    }
}

/// Check if two types are compatible (less strict than equality)
fn areTypesCompatible(type1: ast.Type, type2: ast.Type) bool {
    if (areTypesEqual(type1, type2)) return true;
    
    // Additional compatibility rules
    switch (type1) {
        .Basic => |basic1| {
            switch (type2) {
                .Primitive => |prim2| {
                    // Allow some basic/primitive compatibility
                    return (basic1 == .Drip and prim2 == .Int) or 
                           (basic1 == .Tea and prim2 == .String);
                },
                else => return false,
            }
        },
        .Custom => return true, // Type variables are compatible with anything
        else => return false,
    }
}

/// Resolve generic types with variance constraints
fn resolveGenericWithVariance(engine: *TypeInferenceEngine, generic: ast.GenericType, arg_type: ast.Type) !ast.Type {
    // Create type variable with enhanced constraint tracking
    const var_id = engine.next_var_id;
    engine.next_var_id += 1;
    
    var constraints: std.ArrayList(ast.Type) = .empty;
    try constraints.append(arg_type);
    
    const type_var = TypeVariable{
        .id = var_id,
        .constraints = constraints,
        .resolved_type = null,
        .variance = .Covariant, // Default to covariant
    };
    
    try engine.type_variables.put(var_id, type_var);
    
    // Try to unify with generic constraints
    return try unifyGenericWithConstraints(engine, generic, arg_type);
}

/// Unify generic type with constraints
fn unifyGenericWithConstraints(engine: *TypeInferenceEngine, generic: ast.GenericType, concrete_type: ast.Type) !ast.Type {
    // Create substitution map for generic parameters
    var substitution_map = std.HashMap([]const u8, ast.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(engine.allocator);
    defer substitution_map.deinit();
    
    // If we have type arguments, try to match them with concrete types
    if (generic.type_args.items.len > 0) {
        // Simple case: match first type arg with concrete type
        const type_param_name = try std.fmt.allocPrint(engine.allocator, "T_{s}_0", .{generic.name});
        try substitution_map.put(type_param_name, concrete_type);
    }
    
    return try applySubstitutions(engine, ast.Type{ .Generic = generic }, substitution_map);
}

/// Apply type substitutions
fn applySubstitutions(engine: *TypeInferenceEngine, original_type: ast.Type, substitutions: std.HashMap([]const u8, ast.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)) !ast.Type {
    switch (original_type) {
        .Generic => |generic| {
            var substituted_args: std.ArrayList(ast.Type) = .empty;
            defer substituted_args.deinit();
            
            for (generic.type_args.items) |arg| {
                const substituted_arg = try applySubstitutions(engine, arg, substitutions);
                try substituted_args.append(substituted_arg);
            }
            
            return ast.Type{ .Generic = .{
                .name = generic.name,
                .type_args = try substituted_args.toOwnedSlice(),
            }};
        },
        .Custom => |name| {
            if (substitutions.get(name)) |substituted_type| {
                return substituted_type;
            }
            return original_type;
        },
        else => return original_type,
    }
}
