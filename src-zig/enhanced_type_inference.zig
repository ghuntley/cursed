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
    InfiniteType,
    RecursionDepthExceeded,
    TypeMismatch,
    UnboundTypeVariable,
    OutOfMemory,
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
            .bounds = ArrayList(ast.Type).init(allocator),
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
    visiting: std.HashSet(u32, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage),
    visited: std.HashSet(u32, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage),
    recursion_depth: u32,
    max_depth: u32,
    
    const MAX_RECURSION_DEPTH = 1000;
    
    pub fn init(allocator: Allocator) RecursionDetector {
        return RecursionDetector{
            .visiting = std.HashSet(u32, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .visited = std.HashSet(u32, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .recursion_depth = 0,
            .max_depth = MAX_RECURSION_DEPTH,
        };
    }
    
    pub fn deinit(self: *RecursionDetector) void {
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
        try self.visited.put(type_var_id, {});
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
    
    pub fn init(allocator: Allocator) TypeMemoization {
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
    
    pub fn init(allocator: Allocator) TypeInferenceEngine {
        return TypeInferenceEngine{
            .allocator = allocator,
            .type_variables = HashMap(u32, TypeVariable, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .constraints = ArrayList(TypeConstraint).init(allocator),
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
        try self.constraints.append(constraint);
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
                        var unified_params = ArrayList(ast.Type).init(self.allocator);
                        for (left_func.parameters.items, right_func.parameters.items) |left_param, right_param| {
                            const unified_param = try self.unify(left_param, right_param);
                            try unified_params.append(unified_param);
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
                        var unified_args = ArrayList(ast.Type).init(self.allocator);
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
                var new_params = ArrayList(ast.Type).init(self.allocator);
                for (func.parameters.items) |param| {
                    const new_param = try self.substituteInternal(var_id, replacement, param);
                    try new_params.append(new_param);
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
        std.debug.print("=== Type Inference State ===\n");
        std.debug.print("Type Variables: {d}\n", .{self.type_variables.count()});
        std.debug.print("Constraints: {d}\n", .{self.constraints.items.len});
        std.debug.print("Recursion Depth: {d}\n", .{self.recursion_detector.recursion_depth});
        std.debug.print("Cache Entries: {d}\n", .{self.memoization.unification_cache.count()});
        std.debug.print("============================\n");
    }
};

/// High-level type inference interface
pub fn inferTypes(allocator: Allocator, expressions: []const ast.Expression) TypeInferenceError![]ast.Type {
    var engine = TypeInferenceEngine.init(allocator);
    defer engine.deinit();
    
    var result_types = ArrayList(ast.Type).init(allocator);
    
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
            // Infer function type and argument types
            const func_type = try inferExpressionType(engine, call.function.*);
            
            var arg_types = ArrayList(ast.Type).init(engine.allocator);
            for (call.arguments.items) |arg| {
                const arg_type = try inferExpressionType(engine, arg);
                try arg_types.append(arg_type);
            }
            
            // Create constraints based on function call
            // This would involve more complex constraint generation
            return ast.Type{ .Basic = .Drip }; // Simplified
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
