//! Advanced Generic Type System for CURSED
//! Implements comprehensive generic functions, structs, and type constraints
//! Features:
//! - Generic functions with type parameters (slay<T>)
//! - Generic structs (squad<T>)
//! - Type constraints (T: Numeric, T: Comparable + Sized)
//! - Type inference with constraint validation
//! - Compile-time generic specialization
//! - Higher-kinded types and associated types

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

const ast = @import("ast.zig");
const generic_constraints = @import("generic_constraint_system.zig");
const type_system = @import("type_system_runtime.zig");

/// Enhanced Generic Type Information
pub const AdvancedGenericType = struct {
    name: []const u8,
    type_parameters: ArrayList(AdvancedTypeParameter),
    type_arguments: ArrayList(ast.Type),
    constraints: ArrayList(AdvancedTypeConstraint),
    specializations: HashMap([]const u8, *CompiledSpecialization),
    is_higher_kinded: bool = false,
    kind_arity: u32 = 0, // For higher-kinded types like F<A, B>
    associated_types: HashMap([]const u8, ast.Type),
    
    pub fn init(allocator: Allocator, name: []const u8) AdvancedGenericType {
        return AdvancedGenericType{
            .name = name,
            .type_parameters = ArrayList(AdvancedTypeParameter){},
            .type_arguments = ArrayList(ast.Type){},
            .constraints = ArrayList(AdvancedTypeConstraint){},
            .specializations = HashMap([]const u8, *CompiledSpecialization){},
            .associated_types = HashMap([]const u8, ast.Type){},
        };
    }
    
    pub fn deinit(self: *AdvancedGenericType) void {
        for (self.type_parameters.items) |*param| {
            param.deinit();
        }
        self.type_parameters.deinit(self.allocator);
        
        for (self.type_arguments.items) |*arg| {
            arg.deinit(self.type_parameters.allocator);
        }
        self.type_arguments.deinit(self.allocator);
        
        for (self.constraints.items) |*constraint| {
            constraint.deinit();
        }
        self.constraints.deinit(self.allocator);
        
        var spec_iter = self.specializations.iterator();
        while (spec_iter.next()) |entry| {
            entry.value_ptr.*.deinit();
            self.specializations.allocator.destroy(entry.value_ptr.*);
        }
        self.specializations.deinit(self.allocator);
        
        self.associated_types.deinit(self.allocator);
    }
    
    /// Add a type parameter with constraints
    pub fn addTypeParameter(self: *AdvancedGenericType, param: AdvancedTypeParameter) !void {
        try self.type_parameters.append(allocator, param);
    }
    
    /// Check if this generic can be instantiated with given type arguments
    pub fn canInstantiate(self: *AdvancedGenericType, type_args: []const ast.Type, type_env: *GenericTypeEnvironment) !bool {
        if (type_args.len != self.type_parameters.items.len) {
            return false;
        }
        
        for (self.type_parameters.items, 0..) |param, i| {
            const type_arg = type_args[i];
            if (!try param.satisfiesConstraints(type_arg, type_env)) {
                return false;
            }
        }
        
        return true;
    }
    
    /// Create a monomorphized instance of this generic type
    pub fn instantiate(self: *AdvancedGenericType, type_args: []const ast.Type, type_env: *GenericTypeEnvironment) !ast.Type {
        const signature = try self.createSpecializationSignature(type_args);
        defer self.type_parameters.allocator.free(signature);
        
        // Check if we already have this specialization
        if (self.specializations.get(signature)) |spec| {
            return spec.specialized_type;
        }
        
        // Create new specialization
        const specialization = try self.type_parameters.allocator.create(CompiledSpecialization);
        specialization.* = try self.createSpecialization(type_args, type_env);
        
        try self.specializations.put(try self.type_parameters.allocator.dupe(u8, signature), specialization);
        return specialization.specialized_type;
    }
    
    fn createSpecializationSignature(self: *AdvancedGenericType, type_args: []const ast.Type) ![]u8 {
        var signature = ArrayList(u8){};
        defer signature.deinit();
        
        try signature.appendSlice(self.name);
        try signature.append(allocator, '<');
        
        for (type_args, 0..) |type_arg, i| {
            if (i > 0) try signature.append(',');
            try signature.appendSlice(try type_arg.toString(self.type_parameters.allocator));
        }
        
        try signature.append(allocator, '>');
        return signature.toOwnedSlice();
    }
    
    fn createSpecialization(self: *AdvancedGenericType, type_args: []const ast.Type, type_env: *GenericTypeEnvironment) !CompiledSpecialization {
        // Create type substitution map
        var substitutions = HashMap([]const u8, ast.Type){};
        defer substitutions.deinit();
        
        for (self.type_parameters.items, 0..) |param, i| {
            try substitutions.put(param.name, type_args[i]);
        }
        
        // Apply substitutions to create specialized type
        const specialized_type = try self.applySubstitutions(substitutions, type_env);
        
        return CompiledSpecialization{
            .signature = try self.createSpecializationSignature(type_args),
            .type_arguments = try self.type_parameters.allocator.dupe(ast.Type, type_args),
            .specialized_type = specialized_type,
            .generation_timestamp = std.time.milliTimestamp(),
        };
    }
    
    fn applySubstitutions(self: *AdvancedGenericType, substitutions: HashMap([]const u8, ast.Type), type_env: *GenericTypeEnvironment) !ast.Type {
        // This would create a concrete type by replacing all type parameters
        // For now, return a custom type with the applied substitutions
        _ = type_env; // TODO: Use for constraint validation
        
        var concrete_name = ArrayList(u8){};
        defer concrete_name.deinit();
        
        try concrete_name.appendSlice(self.name);
        try concrete_name.append(allocator, '<');
        
        var sub_iter = substitutions.iterator();
        var first = true;
        while (sub_iter.next()) |entry| {
            if (!first) try concrete_name.append(',');
            first = false;
            try concrete_name.appendSlice(try entry.value_ptr.toString(self.type_parameters.allocator));
        }
        
        try concrete_name.append(allocator, '>');
        
        return ast.Type{ .Custom = try concrete_name.toOwnedSlice() };
    }
};

/// Advanced type parameter with enhanced constraint support
pub const AdvancedTypeParameter = struct {
    name: []const u8,
    constraints: ArrayList(AdvancedTypeConstraint),
    default_type: ?ast.Type = null,
    variance: Variance = .Invariant,
    bounds: ArrayList(TypeBound),
    associated_types: HashMap([]const u8, AssociatedType),
    lifetime_bounds: ArrayList(LifetimeBound),
    
    pub const Variance = enum {
        Invariant,     // T - exact type match required
        Covariant,     // out T - can use subtypes  
        Contravariant, // in T - can use supertypes
        Bivariant,     // T - can use any type (unsafe but sometimes needed)
    };
    
    pub const TypeBound = struct {
        bound_type: ast.Type,
        is_exact: bool = false, // T = ConcreteType vs T: SomeInterface
        
        pub fn init(bound_type: ast.Type, is_exact: bool) TypeBound {
            return TypeBound{
                .bound_type = bound_type,
                .is_exact = is_exact,
            };
        }
    };
    
    pub const AssociatedType = struct {
        name: []const u8,
        default_type: ?ast.Type = null,
        constraints: ArrayList(AdvancedTypeConstraint),
        
        pub fn init(allocator: Allocator, name: []const u8) AssociatedType {
            return AssociatedType{
                .name = name,
                .constraints = ArrayList(AdvancedTypeConstraint){},
            };
        }
        
        pub fn deinit(self: *AssociatedType) void {
            for (self.constraints.items) |*constraint| {
                constraint.deinit();
            }
            self.constraints.deinit(self.allocator);
        }
    };
    
    pub const LifetimeBound = struct {
        lifetime_name: []const u8,
        outlives: []const u8, // 'a: 'b means 'a outlives 'b
        
        pub fn init(lifetime_name: []const u8, outlives: []const u8) LifetimeBound {
            return LifetimeBound{
                .lifetime_name = lifetime_name,
                .outlives = outlives,
            };
        }
    };
    
    pub fn init(allocator: Allocator, name: []const u8) AdvancedTypeParameter {
        return AdvancedTypeParameter{
            .name = name,
            .constraints = ArrayList(AdvancedTypeConstraint){},
            .bounds = ArrayList(TypeBound){},
            .associated_types = HashMap([]const u8, AssociatedType){},
            .lifetime_bounds = ArrayList(LifetimeBound){},
        };
    }
    
    pub fn deinit(self: *AdvancedTypeParameter) void {
        for (self.constraints.items) |*constraint| {
            constraint.deinit();
        }
        self.constraints.deinit(self.allocator);
        
        self.bounds.deinit(self.allocator);
        
        var assoc_iter = self.associated_types.iterator();
        while (assoc_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.associated_types.deinit(self.allocator);
        
        self.lifetime_bounds.deinit(self.allocator);
        
        if (self.default_type) |*default| {
            default.deinit(self.constraints.allocator);
        }
    }
    
    /// Add constraint to this type parameter
    pub fn addConstraint(self: *AdvancedTypeParameter, constraint: AdvancedTypeConstraint) !void {
        try self.constraints.append(allocator, constraint);
    }
    
    /// Add type bound (T: SomeInterface or T = ConcreteType)
    pub fn addBound(self: *AdvancedTypeParameter, bound: TypeBound) !void {
        try self.bounds.append(allocator, bound);
    }
    
    /// Add associated type declaration
    pub fn addAssociatedType(self: *AdvancedTypeParameter, assoc_type: AssociatedType) !void {
        try self.associated_types.put(assoc_type.name, assoc_type);
    }
    
    /// Check if a concrete type satisfies all constraints of this parameter
    pub fn satisfiesConstraints(self: *AdvancedTypeParameter, concrete_type: ast.Type, type_env: *GenericTypeEnvironment) !bool {
        // Check all constraints
        for (self.constraints.items) |constraint| {
            if (!try constraint.isSatisfiedBy(concrete_type, type_env)) {
                return false;
            }
        }
        
        // Check type bounds
        for (self.bounds.items) |bound| {
            if (bound.is_exact) {
                // Exact type match required
                if (!try type_env.areTypesEqual(concrete_type, bound.bound_type)) {
                    return false;
                }
            } else {
                // Interface/trait bound
                if (!try type_env.implementsInterface(concrete_type, bound.bound_type)) {
                    return false;
                }
            }
        }
        
        return true;
    }
    
    /// Check variance compatibility for type substitution
    pub fn isVarianceCompatible(self: *AdvancedTypeParameter, source: ast.Type, target: ast.Type, type_env: *GenericTypeEnvironment) !bool {
        return switch (self.variance) {
            .Invariant => type_env.areTypesEqual(source, target),
            .Covariant => type_env.isSubtype(source, target),
            .Contravariant => type_env.isSubtype(target, source),
            .Bivariant => true, // Always compatible (unsafe)
        };
    }
};

/// Advanced type constraints with complex operations
pub const AdvancedTypeConstraint = struct {
    kind: ConstraintKind,
    interface_name: ?[]const u8 = null,
    bound_type: ?ast.Type = null,
    const_bounds: ?ConstGenericBounds = null,
    where_clause: ?WhereClause = null,
    compound_constraints: ArrayList(*AdvancedTypeConstraint),
    constraint_operator: ConstraintOperator = .And,
    
    pub const ConstraintKind = enum {
        // Built-in trait constraints
        Numeric,       // T: Numeric - supports +, -, *, /
        Comparable,    // T: Comparable - supports ==, !=
        Ordered,       // T: Ordered - supports <, >, <=, >=
        Sized,         // T: Sized - has known size at compile time
        Clone,         // T: Clone - can be cloned
        Copy,          // T: Copy - can be copied (implies Clone)
        Send,          // T: Send - can be sent across goroutines
        Sync,          // T: Sync - can be shared between goroutines
        
        // Advanced constraints
        Interface,     // T: InterfaceName - implements interface
        Subtype,       // T <: SuperType - is subtype of 
        Supertype,     // T >: SubType - is supertype of
        Equality,      // T = ConcreteType - exact type match
        WhereClause,   // where T.method() -> ReturnType
        Associated,    // T::AssocType = ConcreteType
        ConstGeneric,  // const N: Type - compile-time constant
        
        // Higher-kinded constraints
        HigherKinded,  // F<_> - type constructor
        Functor,       // F: Functor - supports map
        Monad,         // M: Monad - supports bind/flatMap
        
        // Compound constraints
        Union,         // T: A | B - either A or B
        Intersection,  // T: A & B - both A and B
        Negation,      // T: !A - not A
    };
    
    pub const ConstraintOperator = enum {
        And,    // &
        Or,     // |
        Not,    // !
    };
    
    pub const ConstGenericBounds = struct {
        min_value: ?i64 = null,
        max_value: ?i64 = null,
        allowed_values: ?[]const i64 = null,
        type_constraint: ?ast.Type = null,
        
        pub fn validate(self: *const ConstGenericBounds, value: i64) !void {
            if (self.min_value) |min| {
                if (value < min) return error.ConstGenericBoundsViolation;
            }
            
            if (self.max_value) |max| {
                if (value > max) return error.ConstGenericBoundsViolation;
            }
            
            if (self.allowed_values) |allowed| {
                for (allowed) |allowed_val| {
                    if (value == allowed_val) return;
                }
                return error.ConstGenericValueNotAllowed;
            }
        }
    };
    
    pub const WhereClause = struct {
        type_parameter: []const u8,
        method_name: []const u8,
        parameters: ArrayList(ast.Type),
        return_type: ast.Type,
        
        pub fn init(allocator: Allocator, type_parameter: []const u8, method_name: []const u8, return_type: ast.Type) WhereClause {
            return WhereClause{
                .type_parameter = type_parameter,
                .method_name = method_name,
                .parameters = ArrayList(ast.Type){},
                .return_type = return_type,
            };
        }
        
        pub fn deinit(self: *WhereClause) void {
            for (self.parameters.items) |*param| {
                param.deinit(self.parameters.allocator);
            }
            self.parameters.deinit(self.allocator);
            self.return_type.deinit(self.parameters.allocator);
        }
    };
    
    pub fn init(allocator: Allocator, kind: ConstraintKind) AdvancedTypeConstraint {
        return AdvancedTypeConstraint{
            .kind = kind,
            .compound_constraints = ArrayList(*AdvancedTypeConstraint){},
        };
    }
    
    pub fn initInterface(allocator: Allocator, interface_name: []const u8) AdvancedTypeConstraint {
        return AdvancedTypeConstraint{
            .kind = .Interface,
            .interface_name = interface_name,
            .compound_constraints = ArrayList(*AdvancedTypeConstraint){},
        };
    }
    
    pub fn initConstGeneric(allocator: Allocator, bounds: ConstGenericBounds) AdvancedTypeConstraint {
        return AdvancedTypeConstraint{
            .kind = .ConstGeneric,
            .const_bounds = bounds,
            .compound_constraints = ArrayList(*AdvancedTypeConstraint){},
        };
    }
    
    pub fn initWhereClause(allocator: Allocator, where_clause: WhereClause) AdvancedTypeConstraint {
        return AdvancedTypeConstraint{
            .kind = .WhereClause,
            .where_clause = where_clause,
            .compound_constraints = ArrayList(*AdvancedTypeConstraint){},
        };
    }
    
    pub fn deinit(self: *AdvancedTypeConstraint) void {
        if (self.bound_type) |*bound| {
            bound.deinit(self.compound_constraints.allocator);
        }
        
        if (self.where_clause) |*where| {
            where.deinit();
        }
        
        for (self.compound_constraints.items) |constraint| {
            constraint.deinit();
            self.compound_constraints.allocator.destroy(constraint);
        }
        self.compound_constraints.deinit(self.allocator);
    }
    
    /// Add a compound constraint (for A & B or A | B)
    pub fn addCompoundConstraint(self: *AdvancedTypeConstraint, constraint: *AdvancedTypeConstraint) !void {
        try self.compound_constraints.append(allocator, constraint);
    }
    
    /// Check if a concrete type satisfies this constraint
    pub fn isSatisfiedBy(self: *AdvancedTypeConstraint, concrete_type: ast.Type, type_env: *GenericTypeEnvironment) !bool {
        return switch (self.kind) {
            .Numeric => type_env.isNumeric(concrete_type),
            .Comparable => type_env.isComparable(concrete_type),
            .Ordered => type_env.isOrdered(concrete_type),
            .Sized => type_env.isSized(concrete_type),
            .Clone => type_env.isCloneable(concrete_type),
            .Copy => type_env.isCopyable(concrete_type),
            .Send => type_env.isSend(concrete_type),
            .Sync => type_env.isSync(concrete_type),
            .Interface => type_env.implementsInterface(concrete_type, ast.Type{ .Custom = self.interface_name.? }),
            .Subtype => type_env.isSubtype(concrete_type, self.bound_type.?),
            .Supertype => type_env.isSubtype(self.bound_type.?, concrete_type),
            .Equality => type_env.areTypesEqual(concrete_type, self.bound_type.?),
            .WhereClause => self.validateWhereClause(concrete_type, type_env),
            .Associated => self.validateAssociatedType(concrete_type, type_env),
            .ConstGeneric => false, // Not applicable to regular types
            .HigherKinded => type_env.isHigherKinded(concrete_type),
            .Functor => type_env.isFunctor(concrete_type),
            .Monad => type_env.isMonad(concrete_type),
            .Union => self.validateUnionConstraint(concrete_type, type_env),
            .Intersection => self.validateIntersectionConstraint(concrete_type, type_env),
            .Negation => !try self.validateNegationConstraint(concrete_type, type_env),
        };
    }
    
    fn validateWhereClause(self: *AdvancedTypeConstraint, concrete_type: ast.Type, type_env: *GenericTypeEnvironment) !bool {
        if (self.where_clause) |where| {
            return type_env.hasMethod(concrete_type, where.method_name, where.parameters.items, where.return_type);
        }
        return false;
    }
    
    fn validateAssociatedType(self: *AdvancedTypeConstraint, concrete_type: ast.Type, type_env: *GenericTypeEnvironment) !bool {
        // TODO: Implement associated type validation
        _ = concrete_type;
        _ = type_env;
        return true;
    }
    
    fn validateUnionConstraint(self: *AdvancedTypeConstraint, concrete_type: ast.Type, type_env: *GenericTypeEnvironment) !bool {
        // At least one constraint must be satisfied
        for (self.compound_constraints.items) |constraint| {
            if (try constraint.isSatisfiedBy(concrete_type, type_env)) {
                return true;
            }
        }
        return false;
    }
    
    fn validateIntersectionConstraint(self: *AdvancedTypeConstraint, concrete_type: ast.Type, type_env: *GenericTypeEnvironment) !bool {
        // All constraints must be satisfied
        for (self.compound_constraints.items) |constraint| {
            if (!try constraint.isSatisfiedBy(concrete_type, type_env)) {
                return false;
            }
        }
        return true;
    }
    
    fn validateNegationConstraint(self: *AdvancedTypeConstraint, concrete_type: ast.Type, type_env: *GenericTypeEnvironment) !bool {
        // The first constraint must NOT be satisfied
        if (self.compound_constraints.items.len > 0) {
            return self.compound_constraints.items[0].isSatisfiedBy(concrete_type, type_env);
        }
        return false;
    }
};

/// Compiled specialization of a generic type
pub const CompiledSpecialization = struct {
    signature: []const u8,
    type_arguments: []const ast.Type,
    specialized_type: ast.Type,
    generation_timestamp: i64,
    
    pub fn deinit(self: *CompiledSpecialization) void {
        // Note: signature and type_arguments are owned by the allocator
        // specialized_type cleanup is handled by AST
        _ = self;
    }
};

/// Generic type environment for constraint validation
pub const GenericTypeEnvironment = struct {
    allocator: Allocator,
    type_definitions: HashMap([]const u8, *AdvancedGenericType),
    interface_definitions: HashMap([]const u8, InterfaceDefinition),
    builtin_constraints: HashMap([]const u8, BuiltinConstraintValidator),
    
    pub const InterfaceDefinition = struct {
        name: []const u8,
        methods: ArrayList(MethodSignature),
        associated_types: HashMap([]const u8, ast.Type),
        
        pub const MethodSignature = struct {
            name: []const u8,
            parameters: ArrayList(ast.Type),
            return_type: ast.Type,
            is_static: bool = false,
        };
        
        pub fn init(allocator: Allocator, name: []const u8) InterfaceDefinition {
            return InterfaceDefinition{
                .name = name,
                .methods = ArrayList(MethodSignature){},
                .associated_types = HashMap([]const u8, ast.Type){},
            };
        }
        
        pub fn deinit(self: *InterfaceDefinition) void {
            for (self.methods.items) |*method| {
                for (method.parameters.items) |*param| {
                    param.deinit(self.methods.allocator);
                }
                method.parameters.deinit();
                method.return_type.deinit(self.methods.allocator);
            }
            self.methods.deinit(self.allocator);
            
            var assoc_iter = self.associated_types.iterator();
            while (assoc_iter.next()) |entry| {
                entry.value_ptr.deinit(self.associated_types.allocator);
            }
            self.associated_types.deinit(self.allocator);
        }
    };
    
    pub const BuiltinConstraintValidator = struct {
        name: []const u8,
        validator_fn: *const fn (ast.Type) bool,
        
        pub fn init(name: []const u8, validator_fn: *const fn (ast.Type) bool) BuiltinConstraintValidator {
            return BuiltinConstraintValidator{
                .name = name,
                .validator_fn = validator_fn,
            };
        }
    };
    
    pub fn init(allocator: Allocator) GenericTypeEnvironment {
        _ = allocator;
        var env = GenericTypeEnvironment{
            .allocator = allocator,
            .type_definitions = HashMap([]const u8, *AdvancedGenericType){},
            .interface_definitions = HashMap([]const u8, InterfaceDefinition){},
            .builtin_constraints = HashMap([]const u8, BuiltinConstraintValidator){},
        };
        
        // Initialize builtin constraints
        env.initBuiltinConstraints() catch {};
        
        return env;
    }
    
    pub fn deinit(self: *GenericTypeEnvironment) void {
        // Clean up type definitions
        var type_iter = self.type_definitions.iterator();
        while (type_iter.next()) |entry| {
            entry.value_ptr.*.deinit();
            self.allocator.destroy(entry.value_ptr.*);
        }
        self.type_definitions.deinit(self.allocator);
        
        // Clean up interface definitions
        var interface_iter = self.interface_definitions.iterator();
        while (interface_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.interface_definitions.deinit(self.allocator);
        
        self.builtin_constraints.deinit(self.allocator);
    }
    
    fn initBuiltinConstraints(self: *GenericTypeEnvironment) !void {
        // Register built-in constraint validators
        try self.builtin_constraints.put("Numeric", BuiltinConstraintValidator.init("Numeric", isNumericType));
        try self.builtin_constraints.put("Comparable", BuiltinConstraintValidator.init("Comparable", isComparableType));
        try self.builtin_constraints.put("Ordered", BuiltinConstraintValidator.init("Ordered", isOrderedType));
        try self.builtin_constraints.put("Sized", BuiltinConstraintValidator.init("Sized", isSizedType));
        try self.builtin_constraints.put("Clone", BuiltinConstraintValidator.init("Clone", isCloneableType));
        try self.builtin_constraints.put("Copy", BuiltinConstraintValidator.init("Copy", isCopyableType));
        try self.builtin_constraints.put("Send", BuiltinConstraintValidator.init("Send", isSendType));
        try self.builtin_constraints.put("Sync", BuiltinConstraintValidator.init("Sync", isSyncType));
    }
    
    /// Register a generic type definition
    pub fn registerGenericType(self: *GenericTypeEnvironment, generic_type: *AdvancedGenericType) !void {
        try self.type_definitions.put(generic_type.name, generic_type);
    }
    
    /// Register an interface definition
    pub fn registerInterface(self: *GenericTypeEnvironment, interface_def: InterfaceDefinition) !void {
        try self.interface_definitions.put(interface_def.name, interface_def);
    }
    
    /// Check if two types are equal
    pub fn areTypesEqual(self: *GenericTypeEnvironment, type1: ast.Type, type2: ast.Type) !bool {
        _ = self;
        // Simple structural comparison for now
        return std.meta.eql(type1, type2);
    }
    
    /// Check if source type is a subtype of target type
    pub fn isSubtype(self: *GenericTypeEnvironment, source: ast.Type, target: ast.Type) !bool {
        _ = self;
        // For now, only exact matches are considered subtypes
        return std.meta.eql(source, target);
    }
    
    /// Check if type implements an interface
    pub fn implementsInterface(self: *GenericTypeEnvironment, type_to_check: ast.Type, interface_type: ast.Type) !bool {
        _ = self;
        _ = type_to_check;
        _ = interface_type;
        // TODO: Implement interface compatibility checking
        return true;
    }
    
    /// Check if type has a specific method
    pub fn hasMethod(self: *GenericTypeEnvironment, type_to_check: ast.Type, method_name: []const u8, parameters: []const ast.Type, return_type: ast.Type) !bool {
        _ = self;
        _ = type_to_check;
        _ = method_name;
        _ = parameters;
        _ = return_type;
        // TODO: Implement method existence checking
        return true;
    }
    
    // Built-in constraint checkers
    pub fn isNumeric(self: *GenericTypeEnvironment, type_to_check: ast.Type) bool {
        if (self.builtin_constraints.get("Numeric")) |validator| {
            return validator.validator_fn(type_to_check);
        }
        return false;
    }
    
    pub fn isComparable(self: *GenericTypeEnvironment, type_to_check: ast.Type) bool {
        if (self.builtin_constraints.get("Comparable")) |validator| {
            return validator.validator_fn(type_to_check);
        }
        return false;
    }
    
    pub fn isOrdered(self: *GenericTypeEnvironment, type_to_check: ast.Type) bool {
        if (self.builtin_constraints.get("Ordered")) |validator| {
            return validator.validator_fn(type_to_check);
        }
        return false;
    }
    
    pub fn isSized(self: *GenericTypeEnvironment, type_to_check: ast.Type) bool {
        if (self.builtin_constraints.get("Sized")) |validator| {
            return validator.validator_fn(type_to_check);
        }
        return false;
    }
    
    pub fn isCloneable(self: *GenericTypeEnvironment, type_to_check: ast.Type) bool {
        if (self.builtin_constraints.get("Clone")) |validator| {
            return validator.validator_fn(type_to_check);
        }
        return false;
    }
    
    pub fn isCopyable(self: *GenericTypeEnvironment, type_to_check: ast.Type) bool {
        if (self.builtin_constraints.get("Copy")) |validator| {
            return validator.validator_fn(type_to_check);
        }
        return false;
    }
    
    pub fn isSend(self: *GenericTypeEnvironment, type_to_check: ast.Type) bool {
        if (self.builtin_constraints.get("Send")) |validator| {
            return validator.validator_fn(type_to_check);
        }
        return false;
    }
    
    pub fn isSync(self: *GenericTypeEnvironment, type_to_check: ast.Type) bool {
        if (self.builtin_constraints.get("Sync")) |validator| {
            return validator.validator_fn(type_to_check);
        }
        return false;
    }
    
    pub fn isHigherKinded(self: *GenericTypeEnvironment, type_to_check: ast.Type) bool {
        _ = self;
        _ = type_to_check;
        // TODO: Implement higher-kinded type checking
        return false;
    }
    
    pub fn isFunctor(self: *GenericTypeEnvironment, type_to_check: ast.Type) bool {
        _ = self;
        _ = type_to_check;
        // TODO: Implement Functor trait checking
        return false;
    }
    
    pub fn isMonad(self: *GenericTypeEnvironment, type_to_check: ast.Type) bool {
        _ = self;
        _ = type_to_check;
        // TODO: Implement Monad trait checking
        return false;
    }
};

// Built-in constraint validator functions
fn isNumericType(type_to_check: ast.Type) bool {
    return switch (type_to_check) {
        .Basic => |basic| switch (basic) {
            .Drip, .Normie, .Thicc, .Smol, .Mid, .Snack, .Meal => true,
            else => false,
        },
        else => false,
    };
}

fn isComparableType(type_to_check: ast.Type) bool {
    return switch (type_to_check) {
        .Basic => |basic| switch (basic) {
            .Drip, .Normie, .Thicc, .Smol, .Mid, .Tea, .Txt, .Lit => true,
            else => false,
        },
        else => false,
    };
}

fn isOrderedType(type_to_check: ast.Type) bool {
    return isNumericType(type_to_check);
}

fn isSizedType(type_to_check: ast.Type) bool {
    return switch (type_to_check) {
        .Basic => true, // All basic types have known size
        .Array => true, // Arrays have known size
        else => false,
    };
}

fn isCloneableType(type_to_check: ast.Type) bool {
    return switch (type_to_check) {
        .Basic => true, // All basic types are cloneable
        .Array => true, // Arrays are cloneable if elements are
        else => false,
    };
}

fn isCopyableType(type_to_check: ast.Type) bool {
    return switch (type_to_check) {
        .Basic => |basic| switch (basic) {
            .Drip, .Normie, .Thicc, .Smol, .Mid, .Lit => true, // Simple value types
            else => false,
        },
        else => false,
    };
}

fn isSendType(type_to_check: ast.Type) bool {
    _ = type_to_check;
    // For now, assume all types are Send unless proven otherwise
    return true;
}

fn isSyncType(type_to_check: ast.Type) bool {
    _ = type_to_check;
    // For now, assume all types are Sync unless proven otherwise
    return true;
}

// Type system integration
pub const GenericTypeChecker = struct {
    allocator: Allocator,
    type_env: *GenericTypeEnvironment,
    instantiation_cache: HashMap([]const u8, ast.Type),
    
    pub fn init(allocator: Allocator, type_env: *GenericTypeEnvironment) GenericTypeChecker {
        return GenericTypeChecker{
            .allocator = allocator,
            .type_env = type_env,
            .instantiation_cache = HashMap([]const u8, ast.Type){},
        };
    }
    
    pub fn deinit(self: *GenericTypeChecker) void {
        var cache_iter = self.instantiation_cache.iterator();
        while (cache_iter.next()) |entry| {
            entry.value_ptr.deinit(self.allocator);
        }
        self.instantiation_cache.deinit(self.allocator);
    }
    
    /// Check if a generic function call is valid
    pub fn checkGenericFunctionCall(self: *GenericTypeChecker, func_name: []const u8, type_args: []const ast.Type, arg_types: []const ast.Type) !ast.Type {
        // TODO: Implement generic function call checking
        _ = self;
        _ = func_name;
        _ = type_args;
        _ = arg_types;
        
        // For now, return a placeholder type
        return ast.Type{ .Basic = ast.BasicType.Normie };
    }
    
    /// Infer type arguments for a generic function call
    pub fn inferTypeArguments(self: *GenericTypeChecker, func_name: []const u8, arg_types: []const ast.Type) ![]ast.Type {
        // TODO: Implement type inference
        _ = self;
        _ = func_name;
        _ = arg_types;
        
        return &[_]ast.Type{};
    }
    
    /// Validate constraint satisfaction for a type instantiation
    pub fn validateConstraints(self: *GenericTypeChecker, generic_name: []const u8, type_args: []const ast.Type) !bool {
        if (self.type_env.type_definitions.get(generic_name)) |generic_type| {
            return generic_type.canInstantiate(type_args, self.type_env);
        }
        return false;
    }
};

// Tests
test "advanced generic type creation" {
    const allocator = std.testing.allocator;
    
    var type_env = GenericTypeEnvironment.init(allocator);
    defer type_env.deinit();
    
    var generic_type = AdvancedGenericType.init(allocator, "ArrayList");
    defer generic_type.deinit();
    
    var type_param = AdvancedTypeParameter.init(allocator, "T");
    defer type_param.deinit();
    
    var constraint = AdvancedTypeConstraint.init(allocator, .Clone);
    defer constraint.deinit();
    
    try type_param.addConstraint(constraint);
    try generic_type.addTypeParameter(type_param);
    
    try std.testing.expect(generic_type.type_parameters.items.len == 1);
    try std.testing.expect(std.mem.eql(u8, generic_type.type_parameters.items[0].name, "T"));
}

test "constraint validation" {
    const allocator = std.testing.allocator;
    
    var type_env = GenericTypeEnvironment.init(allocator);
    defer type_env.deinit();
    
    const numeric_type = ast.Type{ .Basic = ast.BasicType.Drip };
    const string_type = ast.Type{ .Basic = ast.BasicType.Tea };
    
    try std.testing.expect(type_env.isNumeric(numeric_type));
    try std.testing.expect(!type_env.isNumeric(string_type));
    
    try std.testing.expect(type_env.isComparable(numeric_type));
    try std.testing.expect(type_env.isComparable(string_type));
}
