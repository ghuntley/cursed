//! Comprehensive Generic Constraint System for CURSED
//! Fixes constraint violations and implements proper generic type checking

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const ast = @import("ast.zig");
const type_system = @import("type_system_runtime.zig");

/// Generic constraint kinds
pub const ConstraintKind = enum {
    Any,           // No constraint - T
    Numeric,       // T: Numeric - supports +, -, *, /
    Comparable,    // T: Comparable - supports ==, !=, <, >
    Ordered,       // T: Ordered - supports <, >, <=, >=
    Sized,         // T: Sized - has known size at compile time
    Send,          // T: Send - can be sent across goroutines
    Sync,          // T: Sync - can be shared between goroutines
    Interface,     // T: InterfaceName - implements interface
    ConstGeneric,  // const N: Type - compile-time constant parameter
};

/// Type constraint with bounds and validation
pub const TypeConstraint = struct {
    kind: ConstraintKind,
    bound_type: ?ast.Type = null,
    interface_name: ?[]const u8 = null,
    const_bounds: ?ConstGenericBounds = null,
    error_message: ?[]const u8 = null,
    
    pub const ConstGenericBounds = struct {
        min_value: ?i64 = null,
        max_value: ?i64 = null,
        allowed_values: ?[]const i64 = null,
        
        pub fn validate(self: *const ConstGenericBounds, value: i64) !void {
            if (self.min_value) |min| {
                if (value < min) return error.BoundsViolation;
            }
            
            if (self.max_value) |max| {
                if (value > max) return error.BoundsViolation;
            }
            
            if (self.allowed_values) |allowed| {
                for (allowed) |allowed_val| {
                    if (value == allowed_val) return;
                }
                return error.ValueNotAllowed;
            }
        }
    };
    
    pub fn init(kind: ConstraintKind) TypeConstraint {
        return TypeConstraint{ .kind = kind };
    }
    
    pub fn initInterface(interface_name: []const u8) TypeConstraint {
        return TypeConstraint{
            .kind = .Interface,
            .interface_name = interface_name,
        };
    }
    
    pub fn initConstGeneric(bounds: ConstGenericBounds) TypeConstraint {
        return TypeConstraint{
            .kind = .ConstGeneric,
            .const_bounds = bounds,
        };
    }
};

/// Generic type parameter with constraints
pub const GenericTypeParameter = struct {
    name: []const u8,
    constraints: ArrayList(TypeConstraint),
    default_type: ?ast.Type = null,
    variance: Variance = .Invariant,
    
    pub const Variance = enum {
        Invariant,     // T - exact type match required
        Covariant,     // out T - can use subtypes
        Contravariant, // in T - can use supertypes
    };
    
    pub fn init(allocator: Allocator, name: []const u8) GenericTypeParameter {
        return GenericTypeParameter{
            .name = name,
            .constraints = ArrayList(TypeConstraint).init(allocator),
        };
    }
    
    pub fn deinit(self: *GenericTypeParameter) void {
        self.constraints.deinit();
    }
    
    pub fn addConstraint(self: *GenericTypeParameter, constraint: TypeConstraint) !void {
        try self.constraints.append(constraint);
    }
};

/// Constraint validation result
pub const ConstraintValidationResult = struct {
    valid: bool,
    error_message: ?[]const u8,
    suggestion: ?[]const u8 = null,
    
    pub fn success() ConstraintValidationResult {
        return ConstraintValidationResult{ .valid = true, .error_message = null };
    }
    
    pub fn failure(message: []const u8) ConstraintValidationResult {
        return ConstraintValidationResult{ .valid = false, .error_message = message };
    }
    
    pub fn failureWithSuggestion(message: []const u8, suggestion: []const u8) ConstraintValidationResult {
        return ConstraintValidationResult{
            .valid = false,
            .error_message = message,
            .suggestion = suggestion,
        };
    }
};

/// Comprehensive constraint validator
pub const ConstraintValidator = struct {
    allocator: Allocator,
    type_registry: *type_system.GCTypeRegistry,
    builtin_interfaces: HashMap([]const u8, InterfaceInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    const InterfaceInfo = struct {
        methods: ArrayList(MethodSignature),
        
        const MethodSignature = struct {
            name: []const u8,
            parameters: []const ast.Type,
            return_type: ?ast.Type,
        };
        
        pub fn init(allocator: Allocator) InterfaceInfo {
            return InterfaceInfo{
                .methods = ArrayList(MethodSignature).init(allocator),
            };
        }
        
        pub fn deinit(self: *InterfaceInfo) void {
            self.methods.deinit();
        }
    };
    
    pub fn init(allocator: Allocator, type_registry: *type_system.GCTypeRegistry) ConstraintValidator {
        var validator = ConstraintValidator{
            .allocator = allocator,
            .type_registry = type_registry,
            .builtin_interfaces = HashMap([]const u8, InterfaceInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
        
        validator.initBuiltinInterfaces() catch |err| {
            std.log.err("Failed to initialize builtin interfaces: {}", .{err});
        };
        
        return validator;
    }
    
    pub fn deinit(self: *ConstraintValidator) void {
        var iter = self.builtin_interfaces.iterator();
        while (iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.builtin_interfaces.deinit();
    }
    
    /// Initialize built-in constraint interfaces
    fn initBuiltinInterfaces(self: *ConstraintValidator) !void {
        // Numeric interface
        var numeric_interface = InterfaceInfo.init(self.allocator);
        try numeric_interface.methods.append(InterfaceInfo.MethodSignature{
            .name = "add",
            .parameters = &[_]ast.Type{ast.Type{ .Identifier = "Self" }},
            .return_type = ast.Type{ .Identifier = "Self" },
        });
        try self.builtin_interfaces.put("Numeric", numeric_interface);
        
        // Comparable interface
        var comparable_interface = InterfaceInfo.init(self.allocator);
        try comparable_interface.methods.append(InterfaceInfo.MethodSignature{
            .name = "eq",
            .parameters = &[_]ast.Type{ast.Type{ .Identifier = "Self" }},
            .return_type = ast.Type{ .Primitive = .Lit },
        });
        try self.builtin_interfaces.put("Comparable", comparable_interface);
        
        // Ordered interface (extends Comparable)
        var ordered_interface = InterfaceInfo.init(self.allocator);
        try ordered_interface.methods.append(InterfaceInfo.MethodSignature{
            .name = "lt",
            .parameters = &[_]ast.Type{ast.Type{ .Identifier = "Self" }},
            .return_type = ast.Type{ .Primitive = .Lit },
        });
        try self.builtin_interfaces.put("Ordered", ordered_interface);
    }
    
    /// Validate a concrete type against a constraint
    pub fn validateConstraint(self: *ConstraintValidator, concrete_type: ast.Type, constraint: TypeConstraint) ConstraintValidationResult {
        return switch (constraint.kind) {
            .Any => ConstraintValidationResult.success(),
            .Numeric => self.validateNumericConstraint(concrete_type),
            .Comparable => self.validateComparableConstraint(concrete_type),
            .Ordered => self.validateOrderedConstraint(concrete_type),
            .Sized => self.validateSizedConstraint(concrete_type),
            .Send => self.validateSendConstraint(concrete_type),
            .Sync => self.validateSyncConstraint(concrete_type),
            .Interface => self.validateInterfaceConstraint(concrete_type, constraint.interface_name.?),
            .ConstGeneric => self.validateConstGenericConstraint(concrete_type, constraint.const_bounds.?),
        };
    }
    
    /// Validate numeric constraint
    fn validateNumericConstraint(self: *ConstraintValidator, concrete_type: ast.Type) ConstraintValidationResult {
        _ = self;
        
        return switch (concrete_type) {
            .Primitive => |prim| switch (prim) {
                .Normie, .Drip, .Smol, .Thicc, .Meal, .Snack => ConstraintValidationResult.success(),
                else => ConstraintValidationResult.failure("Type does not satisfy Numeric constraint"),
            },
            .Identifier => |name| {
                // Check if it's a type parameter that might be numeric
                if (std.mem.startsWith(u8, name, "T") or std.mem.eql(u8, name, "Self")) {
                    return ConstraintValidationResult.success(); // Assume valid for inference
                }
                return ConstraintValidationResult.failureWithSuggestion(
                    "Unknown type for Numeric constraint", 
                    "Use normie, drip, smol, thicc, meal, or snack"
                );
            },
            else => ConstraintValidationResult.failure("Type does not satisfy Numeric constraint"),
        };
    }
    
    /// Validate comparable constraint
    fn validateComparableConstraint(self: *ConstraintValidator, concrete_type: ast.Type) ConstraintValidationResult {
        _ = self;
        
        return switch (concrete_type) {
            .Primitive => |prim| switch (prim) {
                .Normie, .Drip, .Smol, .Thicc, .Meal, .Snack, .Tea, .Lit, .Sip => ConstraintValidationResult.success(),
                .Vibes => ConstraintValidationResult.failure("vibes type is not comparable"),
                else => ConstraintValidationResult.failure("Type does not satisfy Comparable constraint"),
            },
            .Identifier => |name| {
                // Type parameters assumed valid during inference
                if (std.mem.startsWith(u8, name, "T") or std.mem.eql(u8, name, "Self")) {
                    return ConstraintValidationResult.success();
                }
                return ConstraintValidationResult.failure("Unknown type for Comparable constraint");
            },
            else => ConstraintValidationResult.failure("Type does not satisfy Comparable constraint"),
        };
    }
    
    /// Validate ordered constraint
    fn validateOrderedConstraint(self: *ConstraintValidator, concrete_type: ast.Type) ConstraintValidationResult {
        // Ordered types are a subset of comparable types (excluding boolean)
        const comparable_result = self.validateComparableConstraint(concrete_type);
        if (!comparable_result.valid) return comparable_result;
        
        return switch (concrete_type) {
            .Primitive => |prim| switch (prim) {
                .Lit => ConstraintValidationResult.failureWithSuggestion(
                    "lit (boolean) type is not ordered", 
                    "Use numeric or string types for ordering"
                ),
                else => ConstraintValidationResult.success(),
            },
            else => ConstraintValidationResult.success(),
        };
    }
    
    /// Validate sized constraint
    fn validateSizedConstraint(self: *ConstraintValidator, concrete_type: ast.Type) ConstraintValidationResult {
        _ = self;
        
        return switch (concrete_type) {
            .Primitive => ConstraintValidationResult.success(), // All primitives have known size
            .Array => ConstraintValidationResult.success(), // Fixed-size arrays are sized
            .Slice => ConstraintValidationResult.failure("Slices have dynamic size"),
            .Identifier => ConstraintValidationResult.success(), // Assume valid for inference
            else => ConstraintValidationResult.success(), // Most types are sized by default
        };
    }
    
    /// Validate Send constraint (can be sent across goroutines)
    fn validateSendConstraint(self: *ConstraintValidator, concrete_type: ast.Type) ConstraintValidationResult {
        _ = self;
        
        // In CURSED, most types are Send by default except for certain reference types
        return switch (concrete_type) {
            .Primitive => ConstraintValidationResult.success(),
            .Array => ConstraintValidationResult.success(),
            .Slice => ConstraintValidationResult.success(),
            .Identifier => ConstraintValidationResult.success(),
            // Function types with closures might not be Send
            .Function => ConstraintValidationResult.failure("Function types may not be Send"),
            else => ConstraintValidationResult.success(),
        };
    }
    
    /// Validate Sync constraint (can be shared between goroutines)
    fn validateSyncConstraint(self: *ConstraintValidator, concrete_type: ast.Type) ConstraintValidationResult {
        _ = self;
        
        // More restrictive than Send - requires thread-safe sharing
        return switch (concrete_type) {
            .Primitive => ConstraintValidationResult.success(),
            .Array => ConstraintValidationResult.success(), // Immutable arrays are Sync
            .Slice => ConstraintValidationResult.failure("Mutable slices are not Sync"),
            .Identifier => ConstraintValidationResult.success(),
            else => ConstraintValidationResult.failure("Type may not be Sync without explicit implementation"),
        };
    }
    
    /// Validate interface constraint
    fn validateInterfaceConstraint(self: *ConstraintValidator, concrete_type: ast.Type, interface_name: []const u8) ConstraintValidationResult {
        // Check built-in interfaces
        if (self.builtin_interfaces.get(interface_name)) |interface_info| {
            return self.checkInterfaceImplementation(concrete_type, interface_info);
        }
        
        // For user-defined interfaces, we'd need to look them up in the type registry
        // For now, assume unknown interfaces are satisfied for type parameters
        return switch (concrete_type) {
            .Identifier => ConstraintValidationResult.success(), // Type parameter
            else => ConstraintValidationResult.failure("Interface not implemented"),
        };
    }
    
    /// Check if type implements an interface
    fn checkInterfaceImplementation(self: *ConstraintValidator, concrete_type: ast.Type, interface_info: InterfaceInfo) ConstraintValidationResult {
        _ = self;
        _ = interface_info;
        
        // Simplified interface checking - in a full implementation,
        // we'd check if the type has all required methods
        return switch (concrete_type) {
            .Primitive => |prim| switch (prim) {
                .Normie, .Drip, .Smol, .Thicc, .Meal, .Snack => ConstraintValidationResult.success(), // Numeric types
                .Tea, .Lit => ConstraintValidationResult.success(), // Other primitives
                else => ConstraintValidationResult.failure("Primitive type does not implement interface"),
            },
            .Identifier => ConstraintValidationResult.success(), // Type parameters assumed valid
            else => ConstraintValidationResult.failure("Type does not implement interface"),
        };
    }
    
    /// Validate const generic constraint
    fn validateConstGenericConstraint(self: *ConstraintValidator, concrete_type: ast.Type, bounds: TypeConstraint.ConstGenericBounds) ConstraintValidationResult {
        _ = self;
        _ = bounds;
        
        // Const generics validation would happen at compile time with actual values
        // For type checking, we just ensure the type is appropriate for const generics
        return switch (concrete_type) {
            .Primitive => |prim| switch (prim) {
                .Normie, .Drip, .Smol, .Thicc => ConstraintValidationResult.success(), // Integer types for const generics
                else => ConstraintValidationResult.failure("Only integer types allowed for const generics"),
            },
            else => ConstraintValidationResult.failure("Invalid type for const generic parameter"),
        };
    }
    
    /// Validate all constraints for a generic instantiation
    pub fn validateGenericInstantiation(
        self: *ConstraintValidator,
        type_parameters: []const GenericTypeParameter,
        concrete_types: []const ast.Type,
    ) ![]ConstraintValidationResult {
        if (type_parameters.len != concrete_types.len) {
            return error.TypeArgumentCountMismatch;
        }
        
        var results = ArrayList(ConstraintValidationResult).init(self.allocator);
        
        for (type_parameters, concrete_types) |type_param, concrete_type| {
            // Validate each constraint for this type parameter
            for (type_param.constraints.items) |constraint| {
                const result = self.validateConstraint(concrete_type, constraint);
                try results.append(result);
                
                // If any constraint fails, we could early exit or collect all errors
                if (!result.valid) {
                    std.log.err("Constraint violation for type parameter '{}': {s}", .{ type_param.name, result.error_message.? });
                    if (result.suggestion) |suggestion| {
                        std.log.info("Suggestion: {s}", .{suggestion});
                    }
                }
            }
        }
        
        return results.toOwnedSlice();
    }
    
    /// Check if a type satisfies multiple constraints
    pub fn satisfiesAllConstraints(
        self: *ConstraintValidator,
        concrete_type: ast.Type,
        constraints: []const TypeConstraint,
    ) bool {
        for (constraints) |constraint| {
            const result = self.validateConstraint(concrete_type, constraint);
            if (!result.valid) return false;
        }
        return true;
    }
    
    /// Get suggested types for a failed constraint
    pub fn getSuggestedTypes(self: *ConstraintValidator, constraint: TypeConstraint) []const []const u8 {
        _ = self;
        
        return switch (constraint.kind) {
            .Numeric => &[_][]const u8{ "normie", "drip", "smol", "thicc", "meal", "snack" },
            .Comparable => &[_][]const u8{ "normie", "drip", "tea", "lit", "smol", "thicc", "meal", "snack" },
            .Ordered => &[_][]const u8{ "normie", "drip", "tea", "smol", "thicc", "meal", "snack" },
            .Sized => &[_][]const u8{ "normie", "drip", "tea", "lit", "[]T" },
            else => &[_][]const u8{},
        };
    }
};

/// Generic function signature with constraints
pub const GenericFunctionSignature = struct {
    name: []const u8,
    type_parameters: ArrayList(GenericTypeParameter),
    parameters: ArrayList(FunctionParameter),
    return_type: ?ast.Type,
    
    pub const FunctionParameter = struct {
        name: []const u8,
        param_type: ast.Type,
    };
    
    pub fn init(allocator: Allocator, name: []const u8) GenericFunctionSignature {
        return GenericFunctionSignature{
            .name = name,
            .type_parameters = ArrayList(GenericTypeParameter).init(allocator),
            .parameters = ArrayList(FunctionParameter).init(allocator),
            .return_type = null,
        };
    }
    
    pub fn deinit(self: *GenericFunctionSignature) void {
        for (self.type_parameters.items) |*param| {
            param.deinit();
        }
        self.type_parameters.deinit();
        self.parameters.deinit();
    }
    
    pub fn addTypeParameter(self: *GenericFunctionSignature, type_param: GenericTypeParameter) !void {
        try self.type_parameters.append(type_param);
    }
    
    pub fn addParameter(self: *GenericFunctionSignature, param: FunctionParameter) !void {
        try self.parameters.append(param);
    }
};

test "constraint validation - numeric types" {
    var type_registry = type_system.GCTypeRegistry.init(std.testing.allocator);
    defer type_registry.deinit();
    
    var validator = ConstraintValidator.init(std.testing.allocator, &type_registry);
    defer validator.deinit();
    
    const numeric_constraint = TypeConstraint.init(.Numeric);
    
    // Valid numeric types
    const drip_type = ast.Type{ .Primitive = .Drip };
    const result = validator.validateConstraint(drip_type, numeric_constraint);
    try std.testing.expect(result.valid);
    
    // Invalid numeric type
    const bool_type = ast.Type{ .Primitive = .Lit };
    const invalid_result = validator.validateConstraint(bool_type, numeric_constraint);
    try std.testing.expect(!invalid_result.valid);
}

test "constraint validation - const generics" {
    var type_registry = type_system.GCTypeRegistry.init(std.testing.allocator);
    defer type_registry.deinit();
    
    var validator = ConstraintValidator.init(std.testing.allocator, &type_registry);
    defer validator.deinit();
    
    const const_bounds = TypeConstraint.ConstGenericBounds{
        .min_value = 0,
        .max_value = 1000,
    };
    
    const const_constraint = TypeConstraint.initConstGeneric(const_bounds);
    
    // Valid const generic type
    const int_type = ast.Type{ .Primitive = .Normie };
    const result = validator.validateConstraint(int_type, const_constraint);
    try std.testing.expect(result.valid);
    
    // Invalid const generic type
    const float_type = ast.Type{ .Primitive = .Meal };
    const invalid_result = validator.validateConstraint(float_type, const_constraint);
    try std.testing.expect(!invalid_result.valid);
}
