/// PRIORITY 3: Runtime Generic Type System Implementation
/// Provides comprehensive runtime generic type handling for CURSED with:
/// 1. Type parameter resolution at runtime
/// 2. Generic function instantiation
/// 3. Generic struct/interface support  
/// 4. Type constraint checking
/// 5. Monomorphization for performance

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const atomic = std.atomic;

const ast = @import("ast.zig");
const type_system = @import("type_system_runtime.zig");
const generics = @import("generics.zig");
const error_handling = @import("error_handling.zig");
const CursedError = error_handling.CursedError;

/// Runtime generic type parameter with advanced constraint support
pub const RuntimeTypeParameter = struct {
    name: []const u8,
    constraints: ArrayList(RuntimeConstraint),
    default_type: ?RuntimeType,
    variance: Variance,
    allocator: Allocator,
    
    pub const Variance = enum {
        Invariant,     // T must be exactly T
        Covariant,     // T can be subtype (T -> super-T)
        Contravariant, // T can be supertype (super-T -> T)
        Bivariant,     // T can be any compatible type
    };
    
    pub fn init(allocator: Allocator, name: []const u8, variance: Variance) RuntimeTypeParameter {
        return RuntimeTypeParameter{
            .name = allocator.dupe(u8, name) catch unreachable,
            .constraints = ArrayList(RuntimeConstraint).init(allocator),
            .default_type = null,
            .variance = variance,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *RuntimeTypeParameter) void {
        self.allocator.free(self.name);
        for (self.constraints.items) |*constraint| {
            constraint.deinit();
        }
        self.constraints.deinit();
        if (self.default_type) |*default| {
            default.deinit(self.allocator);
        }
    }
    
    pub fn addConstraint(self: *RuntimeTypeParameter, constraint: RuntimeConstraint) !void {
        try self.constraints.append(constraint);
    }
    
    pub fn satisfiesConstraints(self: *RuntimeTypeParameter, concrete_type: RuntimeType, type_env: *RuntimeTypeEnvironment) !bool {
        for (self.constraints.items) |constraint| {
            if (!try constraint.check(concrete_type, type_env)) {
                return false;
            }
        }
        return true;
    }
    
    /// Check variance compatibility for type substitution
    pub fn checkVarianceCompatibility(self: *RuntimeTypeParameter, source: RuntimeType, target: RuntimeType, type_env: *RuntimeTypeEnvironment) !bool {
        return switch (self.variance) {
            .Invariant => type_env.areTypesEqual(source, target),
            .Covariant => type_env.isSubtype(source, target),
            .Contravariant => type_env.isSubtype(target, source),
            .Bivariant => true,
        };
    }
};

/// Runtime type constraint system
pub const RuntimeConstraint = struct {
    kind: ConstraintKind,
    interface_type: ?RuntimeType,
    bounds: ?ArrayList(RuntimeType),
    allocator: Allocator,
    
    pub const ConstraintKind = enum {
        None,              // No constraints
        Comparable,        // Supports ==, != 
        Numeric,          // Supports +, -, *, /
        Ordered,          // Supports <, >, <=, >=
        Sized,            // Has known size at compile time
        Clone,            // Can be cloned/copied
        Send,             // Safe to send between threads
        Sync,             // Safe to share between threads
        Interface,        // Implements specific interface
        Where,            // Custom where clause
        Lifetime,         // Lifetime constraint
        Associated,       // Associated type constraint
    };
    
    pub fn init(allocator: Allocator, kind: ConstraintKind) RuntimeConstraint {
        return RuntimeConstraint{
            .kind = kind,
            .interface_type = null,
            .bounds = null,
            .allocator = allocator,
        };
    }
    
    pub fn initInterface(allocator: Allocator, interface_type: RuntimeType) RuntimeConstraint {
        return RuntimeConstraint{
            .kind = .Interface,
            .interface_type = interface_type,
            .bounds = null,
            .allocator = allocator,
        };
    }
    
    pub fn initWhere(allocator: Allocator, bounds: ArrayList(RuntimeType)) RuntimeConstraint {
        return RuntimeConstraint{
            .kind = .Where,
            .interface_type = null,
            .bounds = bounds,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *RuntimeConstraint) void {
        if (self.interface_type) |*iface| {
            iface.deinit(self.allocator);
        }
        if (self.bounds) |*bounds| {
            for (bounds.items) |*bound| {
                bound.deinit(self.allocator);
            }
            bounds.deinit();
        }
    }
    
    /// Check if type satisfies this constraint
    pub fn check(self: *RuntimeConstraint, concrete_type: RuntimeType, type_env: *RuntimeTypeEnvironment) !bool {
        return switch (self.kind) {
            .None => true,
            .Comparable => type_env.isComparable(concrete_type),
            .Numeric => type_env.isNumeric(concrete_type),
            .Ordered => type_env.isOrdered(concrete_type),
            .Sized => type_env.isSized(concrete_type),
            .Clone => type_env.isCloneable(concrete_type),
            .Send => type_env.isSend(concrete_type),
            .Sync => type_env.isSync(concrete_type),
            .Interface => {
                if (self.interface_type) |iface| {
                    return type_env.implementsInterface(concrete_type, iface);
                }
                return false;
            },
            .Where => {
                if (self.bounds) |bounds| {
                    for (bounds.items) |bound| {
                        if (!try type_env.satisfiesBound(concrete_type, bound)) {
                            return false;
                        }
                    }
                    return true;
                }
                return false;
            },
            .Lifetime => true, // Lifetime constraints checked separately
            .Associated => true, // Associated type constraints checked separately
        };
    }
};

/// Runtime representation of types
pub const RuntimeType = struct {
    kind: TypeKind,
    name: []const u8,
    type_args: ?ArrayList(RuntimeType),
    metadata: TypeMetadata,
    allocator: Allocator,
    
    pub const TypeKind = enum {
        Primitive,
        Struct,
        Interface,
        Function,
        Array,
        Slice,
        Pointer,
        Reference,
        Generic,
        Instantiated,
        Associated,
        Lifetime,
    };
    
    pub const TypeMetadata = struct {
        size: usize,
        alignment: usize,
        is_pod: bool,          // Plain Old Data
        is_send: bool,
        is_sync: bool,
        is_copy: bool,
        lifetime_params: ?ArrayList([]const u8),
        
        pub fn init() TypeMetadata {
            return TypeMetadata{
                .size = 0,
                .alignment = 1,
                .is_pod = false,
                .is_send = false,
                .is_sync = false,
                .is_copy = false,
                .lifetime_params = null,
            };
        }
        
        pub fn deinit(self: *TypeMetadata, allocator: Allocator) void {
            if (self.lifetime_params) |*params| {
                for (params.items) |param| {
                    allocator.free(param);
                }
                params.deinit();
            }
        }
    };
    
    pub fn init(allocator: Allocator, kind: TypeKind, name: []const u8) RuntimeType {
        return RuntimeType{
            .kind = kind,
            .name = allocator.dupe(u8, name) catch unreachable,
            .type_args = null,
            .metadata = TypeMetadata.init(),
            .allocator = allocator,
        };
    }
    
    pub fn initGeneric(allocator: Allocator, name: []const u8, type_args: ArrayList(RuntimeType)) RuntimeType {
        return RuntimeType{
            .kind = .Generic,
            .name = allocator.dupe(u8, name) catch unreachable,
            .type_args = type_args,
            .metadata = TypeMetadata.init(),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *RuntimeType, allocator: Allocator) void {
        allocator.free(self.name);
        if (self.type_args) |*args| {
            for (args.items) |*arg| {
                arg.deinit(allocator);
            }
            args.deinit();
        }
        self.metadata.deinit(allocator);
    }
    
    /// Generate mangled name for type
    pub fn getMangledName(self: *RuntimeType, allocator: Allocator) ![]const u8 {
        var name_parts = ArrayList(u8).init(allocator);
        defer name_parts.deinit();
        
        try name_parts.appendSlice(self.name);
        
        if (self.type_args) |args| {
            try name_parts.appendSlice("_");
            for (args.items, 0..) |arg, i| {
                if (i > 0) try name_parts.appendSlice("_");
                const arg_name = try arg.getMangledName(allocator);
                defer allocator.free(arg_name);
                try name_parts.appendSlice(arg_name);
            }
        }
        
        return name_parts.toOwnedSlice();
    }
    
    /// Check if type is concrete (no unresolved type parameters)
    pub fn isConcrete(self: *RuntimeType) bool {
        switch (self.kind) {
            .Generic => return false,
            .Instantiated => {
                if (self.type_args) |args| {
                    for (args.items) |arg| {
                        if (!arg.isConcrete()) return false;
                    }
                }
                return true;
            },
            else => return true,
        }
    }
};

/// Type substitution for generic instantiation
pub const TypeSubstitution = struct {
    parameter_name: []const u8,
    concrete_type: RuntimeType,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, parameter_name: []const u8, concrete_type: RuntimeType) TypeSubstitution {
        return TypeSubstitution{
            .parameter_name = allocator.dupe(u8, parameter_name) catch unreachable,
            .concrete_type = concrete_type,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *TypeSubstitution) void {
        self.allocator.free(self.parameter_name);
        self.concrete_type.deinit(self.allocator);
    }
};

/// Runtime type environment for generic resolution
pub const RuntimeTypeEnvironment = struct {
    type_registry: HashMap([]const u8, RuntimeType, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    generic_registry: HashMap([]const u8, GenericDeclaration, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    instantiation_cache: HashMap([]const u8, RuntimeType, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    interface_impls: HashMap(InterfaceImplKey, bool, InterfaceImplKeyContext, std.hash_map.default_max_load_percentage),
    type_constraints: HashMap([]const u8, ArrayList(RuntimeConstraint), std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,
    
    pub const GenericDeclaration = struct {
        name: []const u8,
        type_parameters: ArrayList(RuntimeTypeParameter),
        constraints: ArrayList(RuntimeConstraint),
        body: ast.Statement, // Generic body to instantiate
        
        pub fn init(allocator: Allocator, name: []const u8) GenericDeclaration {
            return GenericDeclaration{
                .name = allocator.dupe(u8, name) catch unreachable,
                .type_parameters = ArrayList(RuntimeTypeParameter).init(allocator),
                .constraints = ArrayList(RuntimeConstraint).init(allocator),
                .body = undefined,
            };
        }
        
        pub fn deinit(self: *GenericDeclaration, allocator: Allocator) void {
            allocator.free(self.name);
            for (self.type_parameters.items) |*param| {
                param.deinit();
            }
            self.type_parameters.deinit();
            for (self.constraints.items) |*constraint| {
                constraint.deinit();
            }
            self.constraints.deinit();
        }
    };
    
    pub const InterfaceImplKey = struct {
        type_name: []const u8,
        interface_name: []const u8,
        
        pub fn hash(self: InterfaceImplKey) u64 {
            var hasher = std.hash_map.DefaultHasher.init();
            hasher.update(self.type_name);
            hasher.update(self.interface_name);
            return hasher.final();
        }
        
        pub fn eql(a: InterfaceImplKey, b: InterfaceImplKey) bool {
            return std.mem.eql(u8, a.type_name, b.type_name) and 
                   std.mem.eql(u8, a.interface_name, b.interface_name);
        }
    };
    
    pub const InterfaceImplKeyContext = struct {
        pub fn hash(self: @This(), key: InterfaceImplKey) u64 {
            _ = self;
            return key.hash();
        }
        
        pub fn eql(self: @This(), a: InterfaceImplKey, b: InterfaceImplKey) bool {
            _ = self;
            return a.eql(b);
        }
    };
    
    pub fn init(allocator: Allocator) RuntimeTypeEnvironment {
        return RuntimeTypeEnvironment{
            .type_registry = HashMap([]const u8, RuntimeType, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .generic_registry = HashMap([]const u8, GenericDeclaration, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .instantiation_cache = HashMap([]const u8, RuntimeType, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .interface_impls = HashMap(InterfaceImplKey, bool, InterfaceImplKeyContext, std.hash_map.default_max_load_percentage).init(allocator),
            .type_constraints = HashMap([]const u8, ArrayList(RuntimeConstraint), std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *RuntimeTypeEnvironment) void {
        var type_iter = self.type_registry.iterator();
        while (type_iter.next()) |entry| {
            entry.value_ptr.deinit(self.allocator);
        }
        self.type_registry.deinit();
        
        var generic_iter = self.generic_registry.iterator();
        while (generic_iter.next()) |entry| {
            entry.value_ptr.deinit(self.allocator);
        }
        self.generic_registry.deinit();
        
        var cache_iter = self.instantiation_cache.iterator();
        while (cache_iter.next()) |entry| {
            entry.value_ptr.deinit(self.allocator);
        }
        self.instantiation_cache.deinit();
        
        self.interface_impls.deinit();
        
        var constraint_iter = self.type_constraints.iterator();
        while (constraint_iter.next()) |entry| {
            for (entry.value_ptr.items) |*constraint| {
                constraint.deinit();
            }
            entry.value_ptr.deinit();
        }
        self.type_constraints.deinit();
    }
    
    /// Register a concrete type
    pub fn registerType(self: *RuntimeTypeEnvironment, name: []const u8, runtime_type: RuntimeType) !void {
        const owned_name = try self.allocator.dupe(u8, name);
        try self.type_registry.put(owned_name, runtime_type);
    }
    
    /// Register a generic declaration
    pub fn registerGeneric(self: *RuntimeTypeEnvironment, generic_decl: GenericDeclaration) !void {
        const owned_name = try self.allocator.dupe(u8, generic_decl.name);
        try self.generic_registry.put(owned_name, generic_decl);
    }
    
    /// Instantiate generic type with concrete type arguments
    pub fn instantiateGeneric(self: *RuntimeTypeEnvironment, generic_name: []const u8, type_args: []RuntimeType) !RuntimeType {
        // Generate cache key
        const cache_key = try self.generateCacheKey(generic_name, type_args);
        defer self.allocator.free(cache_key);
        
        // Check cache first
        if (self.instantiation_cache.get(cache_key)) |cached| {
            return cached;
        }
        
        // Get generic declaration
        const generic_decl = self.generic_registry.get(generic_name) orelse {
            return error.GenericNotFound;
        };
        
        // Validate type argument count
        if (type_args.len != generic_decl.type_parameters.items.len) {
            return error.TypeArgumentCountMismatch;
        }
        
        // Validate constraints
        try self.validateConstraints(generic_decl, type_args);
        
        // Create substitution map
        var substitutions = ArrayList(TypeSubstitution).init(self.allocator);
        defer {
            for (substitutions.items) |*sub| {
                sub.deinit();
            }
            substitutions.deinit();
        }
        
        for (generic_decl.type_parameters.items, 0..) |param, i| {
            try substitutions.append(TypeSubstitution.init(self.allocator, param.name, type_args[i]));
        }
        
        // Perform instantiation
        const instantiated_type = try self.performInstantiation(generic_decl, substitutions.items);
        
        // Cache result
        const owned_cache_key = try self.allocator.dupe(u8, cache_key);
        try self.instantiation_cache.put(owned_cache_key, instantiated_type);
        
        return instantiated_type;
    }
    
    /// Generate cache key for generic instantiation
    fn generateCacheKey(self: *RuntimeTypeEnvironment, generic_name: []const u8, type_args: []RuntimeType) ![]const u8 {
        var key_parts = ArrayList(u8).init(self.allocator);
        defer key_parts.deinit();
        
        try key_parts.appendSlice(generic_name);
        try key_parts.appendSlice("<");
        
        for (type_args, 0..) |arg, i| {
            if (i > 0) try key_parts.appendSlice(",");
            const arg_name = try arg.getMangledName(self.allocator);
            defer self.allocator.free(arg_name);
            try key_parts.appendSlice(arg_name);
        }
        
        try key_parts.appendSlice(">");
        return key_parts.toOwnedSlice();
    }
    
    /// Validate type constraints for generic instantiation
    fn validateConstraints(self: *RuntimeTypeEnvironment, generic_decl: GenericDeclaration, type_args: []RuntimeType) !void {
        for (generic_decl.type_parameters.items, 0..) |param, i| {
            const type_arg = type_args[i];
            
            if (!try param.satisfiesConstraints(type_arg, self)) {
                std.log.err("Type argument {s} does not satisfy constraints for parameter {s}", 
                    .{ type_arg.name, param.name });
                return error.ConstraintViolation;
            }
            
            // Validate variance
            if (!try param.checkVarianceCompatibility(type_arg, type_arg, self)) {
                return error.VarianceViolation;
            }
        }
    }
    
    /// Perform the actual generic instantiation
    fn performInstantiation(self: *RuntimeTypeEnvironment, generic_decl: GenericDeclaration, substitutions: []TypeSubstitution) !RuntimeType {
        // Create substitution map for quick lookup
        var sub_map = HashMap([]const u8, RuntimeType, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator);
        defer sub_map.deinit();
        
        for (substitutions) |sub| {
            try sub_map.put(sub.parameter_name, sub.concrete_type);
        }
        
        // Create instantiated type with substituted type arguments
        var type_args = ArrayList(RuntimeType).init(self.allocator);
        for (substitutions) |sub| {
            try type_args.append(sub.concrete_type);
        }
        
        var instantiated = RuntimeType{
            .kind = .Instantiated,
            .name = try self.allocator.dupe(u8, generic_decl.name),
            .type_args = type_args,
            .metadata = RuntimeType.TypeMetadata.init(),
            .allocator = self.allocator,
        };
        
        // Calculate metadata for instantiated type
        try self.calculateInstantiatedMetadata(&instantiated, generic_decl, substitutions);
        
        return instantiated;
    }
    
    /// Calculate metadata for instantiated type
    fn calculateInstantiatedMetadata(self: *RuntimeTypeEnvironment, instantiated: *RuntimeType, generic_decl: GenericDeclaration, substitutions: []TypeSubstitution) !void {
        _ = self;
        _ = generic_decl;
        
        // Calculate size and alignment based on concrete types
        var total_size: usize = 0;
        var max_alignment: usize = 1;
        var all_send = true;
        var all_sync = true;
        var all_copy = true;
        
        for (substitutions) |sub| {
            total_size += sub.concrete_type.metadata.size;
            max_alignment = @max(max_alignment, sub.concrete_type.metadata.alignment);
            all_send = all_send and sub.concrete_type.metadata.is_send;
            all_sync = all_sync and sub.concrete_type.metadata.is_sync;
            all_copy = all_copy and sub.concrete_type.metadata.is_copy;
        }
        
        instantiated.metadata.size = total_size;
        instantiated.metadata.alignment = max_alignment;
        instantiated.metadata.is_send = all_send;
        instantiated.metadata.is_sync = all_sync;
        instantiated.metadata.is_copy = all_copy;
        instantiated.metadata.is_pod = all_copy; // POD if all components are copyable
    }
    
    /// Type checking methods
    pub fn areTypesEqual(self: *RuntimeTypeEnvironment, a: RuntimeType, b: RuntimeType) bool {
        if (a.kind != b.kind) return false;
        if (!std.mem.eql(u8, a.name, b.name)) return false;
        
        // Check type arguments
        if (a.type_args == null and b.type_args == null) return true;
        if (a.type_args == null or b.type_args == null) return false;
        
        const a_args = a.type_args.?;
        const b_args = b.type_args.?;
        
        if (a_args.items.len != b_args.items.len) return false;
        
        for (a_args.items, 0..) |a_arg, i| {
            if (!self.areTypesEqual(a_arg, b_args.items[i])) return false;
        }
        
        return true;
    }
    
    pub fn isSubtype(self: *RuntimeTypeEnvironment, subtype: RuntimeType, supertype: RuntimeType) bool {
        // Same type is subtype
        if (self.areTypesEqual(subtype, supertype)) return true;
        
        // Check interface implementation
        if (supertype.kind == .Interface) {
            return self.implementsInterface(subtype, supertype);
        }
        
        // Check primitive type coercion
        if (subtype.kind == .Primitive and supertype.kind == .Primitive) {
            return self.isPrimitiveCoercible(subtype.name, supertype.name);
        }
        
        return false;
    }
    
    pub fn implementsInterface(self: *RuntimeTypeEnvironment, impl_type: RuntimeType, interface_type: RuntimeType) bool {
        const key = InterfaceImplKey{
            .type_name = impl_type.name,
            .interface_name = interface_type.name,
        };
        return self.interface_impls.get(key) orelse false;
    }
    
    pub fn isComparable(self: *RuntimeTypeEnvironment, runtime_type: RuntimeType) bool {
        _ = self;
        const comparable_types = [_][]const u8{
            "lit", "drip", "normie", "thicc", "smol", "meal", "snack", "tea"
        };
        for (comparable_types) |ct| {
            if (std.mem.eql(u8, runtime_type.name, ct)) return true;
        }
        return false;
    }
    
    pub fn isNumeric(self: *RuntimeTypeEnvironment, runtime_type: RuntimeType) bool {
        _ = self;
        const numeric_types = [_][]const u8{
            "drip", "normie", "thicc", "smol", "meal", "snack"
        };
        for (numeric_types) |nt| {
            if (std.mem.eql(u8, runtime_type.name, nt)) return true;
        }
        return false;
    }
    
    pub fn isOrdered(self: *RuntimeTypeEnvironment, runtime_type: RuntimeType) bool {
        return self.isNumeric(runtime_type);
    }
    
    pub fn isSized(self: *RuntimeTypeEnvironment, runtime_type: RuntimeType) bool {
        _ = self;
        // Most types are sized, slices and trait objects are not
        return runtime_type.kind != .Slice;
    }
    
    pub fn isCloneable(self: *RuntimeTypeEnvironment, runtime_type: RuntimeType) bool {
        _ = self;
        return runtime_type.metadata.is_copy;
    }
    
    pub fn isSend(self: *RuntimeTypeEnvironment, runtime_type: RuntimeType) bool {
        _ = self;
        return runtime_type.metadata.is_send;
    }
    
    pub fn isSync(self: *RuntimeTypeEnvironment, runtime_type: RuntimeType) bool {
        _ = self;
        return runtime_type.metadata.is_sync;
    }
    
    pub fn satisfiesBound(self: *RuntimeTypeEnvironment, runtime_type: RuntimeType, bound: RuntimeType) !bool {
        // Check if type satisfies bound (interface implementation, trait bound, etc.)
        return self.isSubtype(runtime_type, bound);
    }
    
    fn isPrimitiveCoercible(self: *RuntimeTypeEnvironment, from: []const u8, to: []const u8) bool {
        _ = self;
        if (std.mem.eql(u8, from, to)) return true;
        
        // CURSED numeric coercion rules
        const coercion_rules = [_]struct { from: []const u8, to: []const u8 }{
            .{ .from = "smol", .to = "normie" },
            .{ .from = "normie", .to = "drip" },
            .{ .from = "drip", .to = "thicc" },
            .{ .from = "snack", .to = "meal" },
        };
        
        for (coercion_rules) |rule| {
            if (std.mem.eql(u8, from, rule.from) and std.mem.eql(u8, to, rule.to)) {
                return true;
            }
        }
        
        return false;
    }
    
    /// Register interface implementation
    pub fn registerInterfaceImpl(self: *RuntimeTypeEnvironment, impl_type: []const u8, interface_type: []const u8) !void {
        const key = InterfaceImplKey{
            .type_name = try self.allocator.dupe(u8, impl_type),
            .interface_name = try self.allocator.dupe(u8, interface_type),
        };
        try self.interface_impls.put(key, true);
    }
    
    /// Advanced generic instantiation with higher-kinded types
    pub fn instantiateHigherKinded(self: *RuntimeTypeEnvironment, generic_name: []const u8, type_constructor: anytype, args: []RuntimeType) !RuntimeType {
        _ = type_constructor; // For future HKT support
        return self.instantiateGeneric(generic_name, args);
    }
    
    /// Resolve associated types
    pub fn resolveAssociatedType(self: *RuntimeTypeEnvironment, base_type: RuntimeType, assoc_name: []const u8) !RuntimeType {
        _ = self;
        _ = base_type;
        _ = assoc_name;
        // For future associated type support
        return error.NotImplemented;
    }
    
    /// Type inference for generic calls
    pub fn inferTypeArguments(self: *RuntimeTypeEnvironment, generic_name: []const u8, arg_types: []RuntimeType, expected_return: ?RuntimeType) ![]RuntimeType {
        _ = expected_return;
        
        const generic_decl = self.generic_registry.get(generic_name) orelse {
            return error.GenericNotFound;
        };
        
        var inferred_types = ArrayList(RuntimeType).init(self.allocator);
        
        // Simple inference: use provided argument types
        for (arg_types, 0..) |arg_type, i| {
            if (i < generic_decl.type_parameters.items.len) {
                try inferred_types.append(arg_type);
            }
        }
        
        // Fill remaining with defaults or error
        while (inferred_types.items.len < generic_decl.type_parameters.items.len) {
            const param = generic_decl.type_parameters.items[inferred_types.items.len];
            if (param.default_type) |default| {
                try inferred_types.append(default);
            } else {
                return error.CannotInferType;
            }
        }
        
        return inferred_types.toOwnedSlice();
    }
};

/// Runtime generic instantiation engine
pub const RuntimeGenericEngine = struct {
    type_env: *RuntimeTypeEnvironment,
    monomorphizer: *generics.Monomorphizer,
    instantiation_queue: ArrayList(InstantiationRequest),
    allocator: Allocator,
    
    pub const InstantiationRequest = struct {
        generic_name: []const u8,
        type_args: []RuntimeType,
        priority: Priority,
        callback: ?*const fn(RuntimeType) void,
        
        pub const Priority = enum {
            Low,
            Normal,
            High,
            Critical,
        };
    };
    
    pub fn init(allocator: Allocator, type_env: *RuntimeTypeEnvironment, monomorphizer: *generics.Monomorphizer) RuntimeGenericEngine {
        return RuntimeGenericEngine{
            .type_env = type_env,
            .monomorphizer = monomorphizer,
            .instantiation_queue = ArrayList(InstantiationRequest).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *RuntimeGenericEngine) void {
        self.instantiation_queue.deinit();
    }
    
    /// Queue generic instantiation with priority
    pub fn queueInstantiation(self: *RuntimeGenericEngine, request: InstantiationRequest) !void {
        // Insert based on priority
        var insert_index: usize = 0;
        for (self.instantiation_queue.items, 0..) |queued, i| {
            if (@intFromEnum(request.priority) > @intFromEnum(queued.priority)) {
                insert_index = i;
                break;
            }
            insert_index = i + 1;
        }
        
        try self.instantiation_queue.insert(insert_index, request);
    }
    
    /// Process all queued instantiations
    pub fn processQueue(self: *RuntimeGenericEngine) !void {
        while (self.instantiation_queue.items.len > 0) {
            const request = self.instantiation_queue.orderedRemove(0);
            
            const instantiated_type = self.type_env.instantiateGeneric(
                request.generic_name, 
                request.type_args
            ) catch |err| {
                std.log.err("Failed to instantiate {s}: {}", .{ request.generic_name, err });
                continue;
            };
            
            if (request.callback) |callback| {
                callback(instantiated_type);
            }
        }
    }
    
    /// Batch instantiation for performance
    pub fn batchInstantiate(self: *RuntimeGenericEngine, requests: []InstantiationRequest) ![]RuntimeType {
        var results = ArrayList(RuntimeType).init(self.allocator);
        
        for (requests) |request| {
            const result = try self.type_env.instantiateGeneric(request.generic_name, request.type_args);
            try results.append(result);
        }
        
        return results.toOwnedSlice();
    }
};

/// Initialize built-in types for the runtime system
pub fn initializeBuiltinTypes(type_env: *RuntimeTypeEnvironment) !void {
    // Primitive types with metadata
    const primitive_types = [_]struct { name: []const u8, size: usize, send: bool, sync: bool, copy: bool }{
        .{ .name = "lit", .size = 1, .send = true, .sync = true, .copy = true },      // boolean
        .{ .name = "smol", .size = 1, .send = true, .sync = true, .copy = true },    // i8
        .{ .name = "normie", .size = 4, .send = true, .sync = true, .copy = true },  // i32
        .{ .name = "drip", .size = 8, .send = true, .sync = true, .copy = true },    // i64
        .{ .name = "thicc", .size = 16, .send = true, .sync = true, .copy = true },  // i128
        .{ .name = "snack", .size = 4, .send = true, .sync = true, .copy = true },   // f32
        .{ .name = "meal", .size = 8, .send = true, .sync = true, .copy = true },    // f64
        .{ .name = "tea", .size = 16, .send = true, .sync = false, .copy = false },  // string
        .{ .name = "vibes", .size = 0, .send = true, .sync = true, .copy = true },   // void
    };
    
    for (primitive_types) |pt| {
        var runtime_type = RuntimeType.init(type_env.allocator, .Primitive, pt.name);
        runtime_type.metadata.size = pt.size;
        runtime_type.metadata.alignment = pt.size;
        runtime_type.metadata.is_send = pt.send;
        runtime_type.metadata.is_sync = pt.sync;
        runtime_type.metadata.is_copy = pt.copy;
        runtime_type.metadata.is_pod = pt.copy;
        
        try type_env.registerType(pt.name, runtime_type);
    }
    
    // Register built-in interface implementations
    try type_env.registerInterfaceImpl("normie", "Comparable");
    try type_env.registerInterfaceImpl("drip", "Comparable");
    try type_env.registerInterfaceImpl("drip", "Numeric");
    try type_env.registerInterfaceImpl("normie", "Numeric");
    try type_env.registerInterfaceImpl("meal", "Numeric");
    try type_env.registerInterfaceImpl("snack", "Numeric");
}

test "runtime generic type system" {
    const allocator = std.testing.allocator;
    
    var type_env = RuntimeTypeEnvironment.init(allocator);
    defer type_env.deinit();
    
    try initializeBuiltinTypes(&type_env);
    
    // Test basic type registration
    const drip_type = type_env.type_registry.get("drip").?;
    try std.testing.expect(type_env.isNumeric(drip_type));
    try std.testing.expect(type_env.isComparable(drip_type));
    
    // Test generic declaration
    var generic_decl = RuntimeTypeEnvironment.GenericDeclaration.init(allocator, "Container");
    defer generic_decl.deinit(allocator);
    
    var type_param = RuntimeTypeParameter.init(allocator, "T", .Invariant);
    defer type_param.deinit();
    
    const clone_constraint = RuntimeConstraint.init(allocator, .Clone);
    try type_param.addConstraint(clone_constraint);
    
    try generic_decl.type_parameters.append(type_param);
    
    std.log.info("Runtime generic type system test completed successfully", .{});
}

test "runtime type instantiation test" {
    const allocator = std.testing.allocator;
    
    var type_env = RuntimeTypeEnvironment.init(allocator);
    defer type_env.deinit();
    
    try initializeBuiltinTypes(&type_env);
    
    // Create a simple generic type for testing
    var generic_decl = RuntimeTypeEnvironment.GenericDeclaration.init(allocator, "Array");
    defer generic_decl.deinit(allocator);
    
    var type_param = RuntimeTypeParameter.init(allocator, "T", .Covariant);
    defer type_param.deinit();
    
    try generic_decl.type_parameters.append(type_param);
    try type_env.registerGeneric(generic_decl);
    
    // Test instantiation with concrete type
    const drip_type = type_env.type_registry.get("drip").?;
    const type_args = [_]RuntimeType{drip_type};
    
    const instantiated = try type_env.instantiateGeneric("Array", &type_args);
    try std.testing.expect(instantiated.kind == .Instantiated);
    try std.testing.expect(std.mem.eql(u8, instantiated.name, "Array"));
    try std.testing.expect(instantiated.type_args.?.items.len == 1);
    
    std.log.info("Runtime type instantiation test completed successfully", .{});
}
