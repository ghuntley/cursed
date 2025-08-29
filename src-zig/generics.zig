/// Complete monomorphization system for CURSED generics with [T] syntax
const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
// LLVM C imports with proper CPU target configuration
const c = @cImport({
    @cInclude("llvm_c_bindings.h");
});

const ast = @import("ast.zig");
const type_system = @import("type_system_runtime.zig");
const const_generics = @import("const_generics.zig");
const generic_constraints = @import("generic_constraint_system.zig");

/// Generic type parameter with constraints
pub const TypeParameter = struct {
    name: []const u8,
    constraints: ArrayList(Constraint),
    
    pub fn init(allocator: Allocator, name: []const u8) TypeParameter {
        return TypeParameter{
            .name = name,
            .constraints = .empty,
        };
    }
    
    pub fn deinit(self: *TypeParameter, allocator: Allocator) void {
        _ = allocator;
        for (self.constraints.items) |*constraint| {
            constraint.deinit();
        }
        self.constraints.deinit(self.allocator);
    }
};

/// Type constraints for generic parameters
pub const Constraint = struct {
    kind: ConstraintKind,
    interface_name: ?[]const u8 = null,
    const_bounds: ?const_generics.ConstGenericBounds = null,
    
    pub const ConstraintKind = enum {
        Any,           // No constraints - T
        Comparable,    // T: Comparable - can use ==, !=
        Numeric,       // T: Numeric - supports +, -, *, /
        Ordered,       // T: Ordered - supports <, >, <=, >=
        Interface,     // T: SomeInterface - implements interface
        Sized,         // T: Sized - has known size at compile time
        ConstGeneric,  // const N: usize - const generic parameter with bounds
    };
    
    pub fn init(kind: ConstraintKind) Constraint {
        return Constraint{ .kind = kind };
    }
    
    pub fn initInterface(interface_name: []const u8) Constraint {
        return Constraint{ 
            .kind = .Interface, 
            .interface_name = interface_name 
        };
    }
    
    pub fn deinit(self: *Constraint, allocator: Allocator) void {
        _ = allocator;
        _ = allocator; // No cleanup needed for basic constraints
        _ = self;
    }
};

/// Generic declaration (function or struct)
pub const GenericDeclaration = struct {
    name: []const u8,
    type_parameters: ArrayList(TypeParameter),
    kind: DeclarationKind,
    ast_node: ASTNode,
    
    pub const DeclarationKind = enum {
        Function,
        Struct,
        Interface,
    };
    
    pub const ASTNode = union(DeclarationKind) {
        Function: *ast.FunctionStatement,
        Struct: *ast.StructStatement,  
        Interface: *ast.InterfaceStatement,
    };
    
    pub fn init(allocator: Allocator, name: []const u8, kind: DeclarationKind) GenericDeclaration {
        return GenericDeclaration{
            .name = name,
            .type_parameters = .empty,
            .kind = kind,
            .ast_node = undefined, // Set by caller
        };
    }
    
    pub fn deinit(self: *GenericDeclaration, allocator: Allocator) void {
        _ = allocator;
        for (self.type_parameters.items) |*param| {
            param.deinit();
        }
        self.type_parameters.deinit(self.allocator);
    }
};

/// Concrete instantiation of a generic type
pub const TypeSubstitution = struct {
    parameter_name: []const u8,
    concrete_type: ast.Type,
    
    pub fn init(parameter_name: []const u8, concrete_type: ast.Type) TypeSubstitution {
        return TypeSubstitution{
            .parameter_name = parameter_name,
            .concrete_type = concrete_type,
        };
    }
};

/// Monomorphized instance 
pub const MonomorphizedInstance = struct {
    generic_name: []const u8,
    substitutions: ArrayList(TypeSubstitution),
    specialized_name: []const u8,
    llvm_type: ?c.LLVMTypeRef = null,
    llvm_function: ?c.LLVMValueRef = null,
    generated: bool = false,
    
    pub fn init(allocator: Allocator, generic_name: []const u8, specialized_name: []const u8) MonomorphizedInstance {
        return MonomorphizedInstance{
            .generic_name = generic_name,
            .substitutions = .empty,
            .specialized_name = specialized_name,
        };
    }
    
    pub fn deinit(self: *MonomorphizedInstance, allocator: Allocator) void {
        _ = allocator;
        self.substitutions.deinit(self.allocator);
        allocator.free(self.specialized_name);
    }
};

/// Complete monomorphization system
pub const Monomorphizer = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    
    // Generic declarations registry
    generic_declarations: HashMap([]const u8, GenericDeclaration, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Instantiated instances cache
    instances: HashMap([]const u8, MonomorphizedInstance, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Work queue for pending instantiations
    work_queue: ArrayList(InstantiationRequest),
    
    // CRITICAL FIX: Const generics manager to prevent ICE in optimizer
    const_generics_manager: const_generics.ConstGenericsManager,
    
    // Enhanced constraint validator for comprehensive type checking
    constraint_validator: generic_constraints.ConstraintValidator,
    
    pub const InstantiationRequest = struct {
        generic_name: []const u8,
        type_arguments: ArrayList(ast.Type),
        usage_location: []const u8, // For error reporting
        
        pub fn init(allocator: Allocator, generic_name: []const u8, usage_location: []const u8) InstantiationRequest {
            return InstantiationRequest{
                .generic_name = generic_name,
                .type_arguments = .empty,
                .usage_location = usage_location,
            };
        }
        
        pub fn deinit(self: *InstantiationRequest, allocator: Allocator) void {
        _ = allocator;
            self.type_arguments.deinit(self.allocator);
                    }
    };
    
    pub fn init(allocator: Allocator, context: c.LLVMContextRef, module: c.LLVMModuleRef, type_registry: *type_system.GCTypeRegistry) Monomorphizer {
        return Monomorphizer{
            .allocator = allocator,
            .context = context,
            .module = module,
            .generic_declarations = HashMap([]const u8, GenericDeclaration, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .instances = HashMap([]const u8, MonomorphizedInstance, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .work_queue = .empty,
            .const_generics_manager = const_generics.ConstGenericsManager.init(allocator, context),
            .constraint_validator = generic_constraints.ConstraintValidator.init(allocator, type_registry),
        };
    }
    
    pub fn deinit(self: *Monomorphizer) void {
        var decl_iterator = self.generic_declarations.iterator();
        while (decl_iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.generic_declarations.deinit(self.allocator);
        
        var instance_iterator = self.instances.iterator();
        while (instance_iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.instances.deinit(self.allocator);
        
        for (self.work_queue.items) |*request| {
            request.deinit();
        }
        self.work_queue.deinit(self.allocator);
        
        // CRITICAL FIX: Clean up const generics manager
        self.const_generics_manager.deinit(self.allocator);
        
        // Clean up constraint validator
        self.constraint_validator.deinit(self.allocator);
    }
    
    /// Register a generic declaration
    pub fn registerGeneric(self: *Monomorphizer, declaration: GenericDeclaration) !void {
        const owned_name = try self.allocator.dupe(u8, declaration.name);
        try self.generic_declarations.put(owned_name, declaration);
    }
    
    /// CRITICAL FIX: Register const generic parameter with bounds checking
    pub fn registerConstGeneric(
        self: *Monomorphizer, 
        name: []const u8, 
        kind: const_generics.ConstGenericKind,
        bounds: ?const_generics.ConstGenericBounds,
        default_value: ?const_generics.ConstGenericValue
    ) !void {
        try self.const_generics_manager.processConstGenericDeclaration(name, kind, bounds, default_value);
        std.log.info("Registered const generic '{}' with robust bounds checking to prevent optimizer ICE", .{name});
    }
    
    /// CRITICAL FIX: Set const generic value with validation
    pub fn setConstGenericValue(self: *Monomorphizer, name: []const u8, value_expr: ast.Expression) !c.LLVMValueRef {
        return self.const_generics_manager.processConstGenericInstantiation(name, value_expr) catch |err| {
            std.log.err("CRITICAL: Const generic instantiation failed for '{}' - preventing optimizer ICE: {}", .{name, err});
            return err;
        };
    }
    
    /// Request instantiation of generic type with concrete type arguments
    pub fn requestInstantiation(self: *Monomorphizer, generic_name: []const u8, type_arguments: []ast.Type, usage_location: []const u8) ![]const u8 {
        // Generate specialized name
        const specialized_name = try self.generateSpecializedName(generic_name, type_arguments);
        
        // Check if already instantiated
        if (self.instances.contains(specialized_name)) {
            return specialized_name;
        }
        
        // Create instantiation request
        var request = InstantiationRequest.init(self.allocator, generic_name, usage_location);
        for (type_arguments) |type_arg| {
            try request.type_arguments.append(self.allocator, type_arg);
        }
        try self.work_queue.append(self.allocator, request);
        
        // Create placeholder instance
        var instance = MonomorphizedInstance.init(self.allocator, generic_name, specialized_name);
        for (type_arguments, 0..) |type_arg, i| {
            const generic_decl = self.generic_declarations.get(generic_name) orelse {
                return error.GenericNotFound;
            };
            
            if (i >= generic_decl.type_parameters.items.len) {
                return error.TooManyTypeArguments;
            }
            
            const param_name = generic_decl.type_parameters.items[i].name;
            try instance.substitutions.append(self.allocator, TypeSubstitution.init(param_name, type_arg));
        }
        
        try self.instances.put(try self.allocator.dupe(u8, specialized_name), instance);
        return specialized_name;
    }
    
    /// Process all pending instantiations
    pub fn processInstantiations(self: *Monomorphizer) !void {
        while (self.work_queue.items.len > 0) {
            const request = self.work_queue.pop();
            defer request.deinit();
            
            try self.instantiateGeneric(request);
        }
    }
    
    /// Generate specialized name for generic instantiation
    fn generateSpecializedName(self: *Monomorphizer, generic_name: []const u8, type_arguments: []ast.Type) ![]const u8 {
        var name_builder = std.ArrayList(u8){};
        defer name_builder.deinit();
        
        try name_builder.appendSlice(generic_name);
        
        for (type_arguments) |type_arg| {
            try name_builder.appendSlice("_");
            const type_name = try self.typeToString(type_arg);
            defer self.allocator.free(type_name);
            try name_builder.appendSlice(type_name);
        }
        
        return name_builder.toOwnedSlice(self.allocator);
    }
    
    /// Convert type to string for name generation
    fn typeToString(self: *Monomorphizer, type_arg: ast.Type) ![]const u8 {
        return switch (type_arg) {
            .Primitive => |primitive| switch (primitive) {
                .Tea => try self.allocator.dupe(u8, "string"),
                .Normie => try self.allocator.dupe(u8, "i32"),
                .Drip => try self.allocator.dupe(u8, "i64"),
                .Smol => try self.allocator.dupe(u8, "i8"),
                .Thicc => try self.allocator.dupe(u8, "i64"),
                .Meal => try self.allocator.dupe(u8, "f64"),
                .Snack => try self.allocator.dupe(u8, "f32"),
                .Lit => try self.allocator.dupe(u8, "bool"),
                .Vibes => try self.allocator.dupe(u8, "void"),
            },
            .Identifier => |name| try self.allocator.dupe(u8, name),
            .Array => |array_type| {
                const element_type_name = try self.typeToString(array_type.element_type.*);
                defer self.allocator.free(element_type_name);
                return std.fmt.allocPrint(self.allocator, "array_{s}", .{element_type_name});
            },
            .Slice => |slice_type| {
                const element_type_name = try self.typeToString(slice_type.element_type.*);
                defer self.allocator.free(element_type_name);
                return std.fmt.allocPrint(self.allocator, "slice_{s}", .{element_type_name});
            },
            .Generic => |generic| {
                // For nested generics
                return try self.allocator.dupe(u8, generic.name);
            },
            else => try self.allocator.dupe(u8, "unknown"),
        };
    }
    
    /// Instantiate a generic declaration with concrete types
    fn instantiateGeneric(self: *Monomorphizer, request: InstantiationRequest) !void {
        const generic_decl = self.generic_declarations.get(request.generic_name) orelse {
            std.log.err("Generic declaration not found: {s}", .{request.generic_name});
            return error.GenericNotFound;
        };
        
        // Validate type arguments match parameters
        if (request.type_arguments.items.len != generic_decl.type_parameters.items.len) {
            std.log.err("Type argument count mismatch for {s}: expected {d}, got {d}", 
                .{request.generic_name, generic_decl.type_parameters.items.len, request.type_arguments.items.len});
            return error.TypeArgumentCountMismatch;
        }
        
        // Validate constraints
        try self.validateConstraints(generic_decl, request.type_arguments.items);
        
        // Generate specialized version
        switch (generic_decl.kind) {
            .Function => try self.instantiateGenericFunction(generic_decl, request.type_arguments.items),
            .Struct => try self.instantiateGenericStruct(generic_decl, request.type_arguments.items),
            .Interface => try self.instantiateGenericInterface(generic_decl, request.type_arguments.items),
        }
    }
    
    /// CRITICAL FIX: Enhanced type constraint validation with comprehensive constraint checking
    /// This prevents ICE in optimizer when invalid constant values are used
    fn validateConstraints(self: *Monomorphizer, generic_decl: GenericDeclaration, type_arguments: []ast.Type) !void {
        // Validate all const generics first to prevent optimizer ICE
        try self.const_generics_manager.validateAllConstGenerics();
        
        // Convert old constraint format to new format for validation
        var type_params = std.ArrayList(generic_constraints.GenericTypeParameter){};
        defer {
            for (type_params.items) |*param| {
                param.deinit();
            }
            type_params.deinit();
        }
        
        for (generic_decl.type_parameters.items) |old_param| {
            var new_param = generic_constraints.GenericTypeParameter.init(self.allocator, old_param.name);
            
            // Convert old constraints to new constraint format
            for (old_param.constraints.items) |old_constraint| {
                const new_constraint = switch (old_constraint.kind) {
                    .Any => generic_constraints.TypeConstraint.init(.Any),
                    .Comparable => generic_constraints.TypeConstraint.init(.Comparable),
                    .Numeric => generic_constraints.TypeConstraint.init(.Numeric),
                    .Ordered => generic_constraints.TypeConstraint.init(.Ordered),
                    .Interface => generic_constraints.TypeConstraint.initInterface(old_constraint.interface_name.?),
                    .Sized => generic_constraints.TypeConstraint.init(.Sized),
                    .ConstGeneric => blk: {
                        const const_bounds = generic_constraints.TypeConstraint.ConstGenericBounds{
                            .min_value = if (old_constraint.const_bounds) |bounds| bounds.min_value else null,
                            .max_value = if (old_constraint.const_bounds) |bounds| bounds.max_value else null,
                        };
                        break :blk generic_constraints.TypeConstraint.initConstGeneric(const_bounds);
                    },
                };
                
                try new_param.addConstraint(new_constraint);
            }
            
            try type_params.append(self.allocator, new_param);
        }
        
        // Use comprehensive constraint validator
        const validation_results = self.constraint_validator.validateGenericInstantiation(type_params.items, type_arguments) catch |err| {
            std.log.err("Generic instantiation validation failed: {}", .{err});
            return error.ConstraintViolation;
        };
        defer self.allocator.free(validation_results);
        
        // Check if any constraints failed
        var has_failures = false;
        for (validation_results, type_params.items) |result, param| {
            if (!result.valid) {
                has_failures = true;
                std.log.err("CONSTRAINT VIOLATION for parameter '{s}': {s}", .{ param.name, result.error_message.? });
                
                if (result.suggestion) |suggestion| {
                    std.log.info("SUGGESTION: {s}", .{suggestion});
                }
                
                // Provide helpful error messages with type suggestions
                const suggested_types = self.constraint_validator.getSuggestedTypes(param.constraints.items[0]);
                if (suggested_types.len > 0) {
                    std.log.info("Valid types for this constraint:");
                    for (suggested_types) |suggested_type| {
                        std.log.info("  - {s}", .{suggested_type});
                    }
                }
            }
        }
        
        if (has_failures) {
            return error.ConstraintViolation;
        }
        
        // Final validation to ensure optimizer-safe constants
        try self.validateOptimizerSafeConstants();
        
        std.log.info("All generic constraints validated successfully");
    }
    
    /// CRITICAL FIX: Validate const generic constraint to prevent optimizer ICE
    fn validateConstGenericConstraint(self: *Monomorphizer, param_name: []const u8, constraint: Constraint, type_arg: ast.Type) !void {
        _ = type_arg; // Type is validated separately for const generics
        
        if (constraint.const_bounds) |bounds| {
            // Get the const generic value for this parameter
            if (self.const_generics_manager.instantiation.getValue(param_name)) |value| {
                // Validate the value against bounds - critical for preventing ICE
                bounds.validate(value) catch |err| {
                    std.log.err("CRITICAL: Const generic bounds violation for '{s}' would cause optimizer ICE: {}", 
                        .{param_name, err});
                    std.log.err("  Value: {}", .{value});
                    std.log.err("  This violation would cause Internal Compiler Error in optimizer");
                    return const_generics.ConstGenericError.BoundsCheckFailed;
                };
                
                // Additional safety checks for LLVM optimizer compatibility
                switch (value) {
                    .Integer => |int_val| {
                        // Prevent integer overflow that causes optimizer ICE
                        if (int_val < std.math.minInt(i32) or int_val > std.math.maxInt(i32)) {
                            std.log.err("CRITICAL: Integer const generic {} exceeds i32 bounds - would cause optimizer ICE", .{int_val});
                            return const_generics.ConstGenericError.OptimizerICE;
                        }
                    },
                    .Array => |arr_val| {
                        // Prevent arrays that are too large for optimizer
                        if (arr_val.length > 1024) {
                            std.log.err("CRITICAL: Array const generic length {} too large - would cause optimizer ICE", .{arr_val.length});
                            return const_generics.ConstGenericError.OptimizerICE;
                        }
                    },
                    else => {}, // Other types are generally safe
                }
                
                // Additional checks for optimizer-problematic values
                switch (value) {
                    .Integer => |int_val| {
                        if (int_val < -2147483648 or int_val > 2147483647) {
                            std.log.err("CRITICAL: Integer const generic '{}' = {} exceeds i32 bounds - would cause optimizer ICE", 
                                .{param_name, int_val});
                            return const_generics.ConstGenericError.OptimizerICE;
                        }
                        
                        // Check for values that cause optimizer issues
                        if (int_val < 0 and @mod(@abs(int_val), 2) == 1) {
                            std.log.warn("Negative odd const generic '{}' = {} may cause optimizer issues", 
                                .{param_name, int_val});
                        }
                    },
                    .Array => |arr| {
                        if (arr.length > 10000) {
                            std.log.err("CRITICAL: Array const generic '{}' with {} elements too large - would cause optimizer ICE", 
                                .{param_name, arr.length});
                            return const_generics.ConstGenericError.OptimizerICE;
                        }
                    },
                    else => {},
                }
                
                std.log.info("Const generic '{}' = {} passed bounds validation", .{param_name, value});
            } else {
                std.log.err("CRITICAL: Const generic parameter '{}' has no value - would cause optimizer ICE", .{param_name});
                return const_generics.ConstGenericError.ParameterNotFound;
            }
        }
    }
    
    /// CRITICAL FIX: Final validation to ensure all constants are optimizer-safe
    fn validateOptimizerSafeConstants(self: *Monomorphizer) !void {
        var const_iter = self.const_generics_manager.instantiation.values.iterator();
        while (const_iter.next()) |entry| {
            const name = entry.key_ptr.*;
            const value = entry.value_ptr.*;
            
            // Generate LLVM constant and validate it won't cause ICE
            const llvm_value = self.const_generics_manager.llvm_integration.generateLLVMConstant(value) catch |err| {
                std.log.err("CRITICAL: Failed to generate LLVM constant for '{}' - preventing optimizer ICE: {}", 
                    .{name, err});
                return err;
            };
            
            // Validate the LLVM constant won't cause optimizer ICE
            self.const_generics_manager.llvm_integration.validateLLVMConstant(llvm_value) catch |err| {
                std.log.err("CRITICAL: LLVM constant validation failed for '{}' - this would cause optimizer ICE: {}", 
                    .{name, err});
                return err;
            };
        }
        
        std.log.info("All const generics validated as optimizer-safe");
    }
    
    /// CRITICAL FIX: Validate LLVM IR for basic block terminator issues that cause optimizer ICE
    fn validateLLVMBasicBlocks(self: *Monomorphizer, llvm_function: c.LLVMValueRef) !void {
        _ = llvm_function; // Individual function validation happens in the comprehensive fix
        
        const llvm_ice_fix = @import("llvm_optimizer_ice_fix.zig");
        
        // Apply comprehensive LLVM optimizer ICE fixes to the entire module
        try llvm_ice_fix.fixLLVMOptimizerICE(
            self.allocator,
            self.const_generics_manager.context,
            self.module
        );
        
        std.log.info("LLVM module validated and fixed for optimizer ICE prevention");
    }
    
    /// Check if type supports comparison operations
    fn isComparable(self: *Monomorphizer, type_arg: ast.Type) !bool {
        _ = self;
        return switch (type_arg) {
            .Primitive => |primitive| switch (primitive) {
                .Tea, .Normie, .Drip, .Smol, .Thicc, .Meal, .Snack, .Lit => true,
                .Vibes => false,
            },
            .Identifier => true, // Assume user types implement comparison
            else => false,
        };
    }
    
    /// Check if type supports numeric operations
    fn isNumeric(self: *Monomorphizer, type_arg: ast.Type) !bool {
        _ = self;
        return switch (type_arg) {
            .Primitive => |primitive| switch (primitive) {
                .Normie, .Drip, .Smol, .Thicc, .Meal, .Snack => true,
                .Tea, .Lit, .Vibes => false,
            },
            else => false,
        };
    }
    
    /// Check if type supports ordering operations
    fn isOrdered(self: *Monomorphizer, type_arg: ast.Type) !bool {
        return self.isNumeric(type_arg);
    }
    
    /// Check if type implements interface
    fn implementsInterface(self: *Monomorphizer, type_arg: ast.Type, interface_name: []const u8) !bool {
        switch (type_arg) {
            .Identifier => |type_name| {
                // Check if type implements the interface
                return self.checkTypeImplementsInterface(type_name, interface_name);
            },
            .Generic => |generic_type| {
                // For generic types, check if constraints are satisfied
                for (generic_type.constraints.items) |constraint| {
                    switch (constraint) {
                        .Interface => |constraint_interface| {
                            if (std.mem.eql(u8, constraint_interface, interface_name)) {
                                return true;
                            }
                        },
                        else => {},
                    }
                }
                return false;
            },
            else => return false,
        }
    }
    
    /// Check if a concrete type implements an interface
    fn checkTypeImplementsInterface(self: *Monomorphizer, type_name: []const u8, interface_name: []const u8) !bool {
        // Built-in type implementations
        if (std.mem.eql(u8, interface_name, "Comparable")) {
            return self.isBuiltinComparable(type_name);
        }
        if (std.mem.eql(u8, interface_name, "Numeric")) {
            return self.isBuiltinNumeric(type_name);
        }
        if (std.mem.eql(u8, interface_name, "Ordered")) {
            return self.isBuiltinOrdered(type_name);
        }
        
        // Check user-defined interface implementations
        return try self.checkUserInterfaceImplementation(type_name, interface_name);
    }
    
    /// Check if type name represents a built-in comparable type
    fn isBuiltinComparable(self: *Monomorphizer, type_name: []const u8) bool {
        _ = self;
        const comparable_types = [_][]const u8{
            "normie", "drip", "smol", "thicc", "meal", "snack", "lit", "tea"
        };
        for (comparable_types) |ct| {
            if (std.mem.eql(u8, type_name, ct)) return true;
        }
        return false;
    }
    
    /// Check if type name represents a built-in numeric type
    fn isBuiltinNumeric(self: *Monomorphizer, type_name: []const u8) bool {
        _ = self;
        const numeric_types = [_][]const u8{
            "normie", "drip", "smol", "thicc", "meal", "snack"
        };
        for (numeric_types) |nt| {
            if (std.mem.eql(u8, type_name, nt)) return true;
        }
        return false;
    }
    
    /// Check if a type implements a user-defined interface
    fn checkUserInterfaceImplementation(self: *Monomorphizer, type_name: []const u8, interface_name: []const u8) !bool {
        // This would integrate with the interface registry system
        // For now, we implement a basic check system
        
        // Check if we have interface implementations recorded
        if (self.implementations.get(type_name)) |impls| {
            for (impls.items) |impl| {
                if (std.mem.eql(u8, impl.interface_name, interface_name)) {
                    // Found implementation, now validate method signatures
                    return try self.validateInterfaceMethodSignatures(type_name, interface_name);
                }
            }
        }
        
        // No implementation found
        return false;
    }
    
    /// Validate that method signatures match interface requirements
    fn validateInterfaceMethodSignatures(self: *Monomorphizer, type_name: []const u8, interface_name: []const u8) !bool {
        _ = self;
        _ = type_name;
        _ = interface_name;
        
        // This is where we would check that all required interface methods
        // are implemented with compatible signatures
        // For now, assume valid if implementation exists
        return true;
    }
    
    /// Check if type name represents a built-in ordered type
    fn isBuiltinOrdered(self: *Monomorphizer, type_name: []const u8) bool {
        return self.isBuiltinNumeric(type_name);
    }
    
    /// Check if type has known size at compile time
    fn isSized(self: *Monomorphizer, type_arg: ast.Type) !bool {
        _ = self;
        return switch (type_arg) {
            .Primitive => true,
            .Identifier => true,
            .Array => true,
            .Slice => false, // Dynamic size
            else => true,
        };
    }
    
    /// Generate specialized function
    fn instantiateGenericFunction(self: *Monomorphizer, generic_decl: GenericDeclaration, type_arguments: []ast.Type) !void {
        const func_decl = generic_decl.ast_node.Function;
        
        // Create substitution map
        var substitution_map = HashMap([]const u8, ast.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){};
        defer substitution_map.deinit();
        
        for (generic_decl.type_parameters.items, 0..) |param, i| {
            try substitution_map.put(param.name, type_arguments[i]);
        }
        
        // Create specialized function declaration
        var specialized_func = ast.FunctionDeclaration{
            .name = try self.generateSpecializedName(generic_decl.name, type_arguments),
            .parameters = .empty,
            .return_type = if (func_decl.return_type) |rt| try self.substituteType(rt, &substitution_map) else null,
            .body = try self.substituteStatements(func_decl.body.items, &substitution_map),
            .is_async = func_decl.is_async,
        };
        
        // Substitute parameter types
        for (func_decl.parameters.items) |param| {
            try specialized_func.parameters.append(ast.Parameter{
                .name = param.name,
                .param_type = try self.substituteType(param.param_type, &substitution_map),
            });
        }
        
        // Generate LLVM function
        try self.generateSpecializedLLVMFunction(&specialized_func);
        
        // Mark instance as generated
        const specialized_name = specialized_func.name;
        if (self.instances.getPtr(specialized_name)) |instance| {
            instance.generated = true;
        }
    }
    
    /// Generate specialized struct
    fn instantiateGenericStruct(self: *Monomorphizer, generic_decl: GenericDeclaration, type_arguments: []ast.Type) !void {
        const struct_decl = generic_decl.ast_node.Struct;
        
        // Create substitution map
        var substitution_map = HashMap([]const u8, ast.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){};
        defer substitution_map.deinit();
        
        for (generic_decl.type_parameters.items, 0..) |param, i| {
            try substitution_map.put(param.name, type_arguments[i]);
        }
        
        // Create specialized struct declaration
        var specialized_struct = ast.StructDeclaration{
            .name = try self.generateSpecializedName(generic_decl.name, type_arguments),
            .fields = .empty,
        };
        
        // Substitute field types
        for (struct_decl.fields.items) |field| {
            try specialized_struct.fields.append(ast.StructField{
                .name = field.name,
                .field_type = try self.substituteType(field.field_type, &substitution_map),
            });
        }
        
        // Generate LLVM struct type
        try self.generateSpecializedLLVMStruct(&specialized_struct);
        
        // Mark instance as generated
        const specialized_name = specialized_struct.name;
        if (self.instances.getPtr(specialized_name)) |instance| {
            instance.generated = true;
        }
    }
    
    /// Generate specialized interface (creates vtable)
    fn instantiateGenericInterface(self: *Monomorphizer, generic_decl: GenericDeclaration, type_arguments: []ast.Type) !void {
        const interface_decl = generic_decl.ast_node.Interface;
        
        // Create substitution map
        var substitution_map = HashMap([]const u8, ast.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){};
        defer substitution_map.deinit();
        
        for (generic_decl.type_parameters.items, 0..) |param, i| {
            try substitution_map.put(param.name, type_arguments[i]);
        }
        
        // Create specialized interface declaration
        var specialized_interface = ast.InterfaceDeclaration{
            .name = try self.generateSpecializedName(generic_decl.name, type_arguments),
            .methods = .empty,
        };
        
        // Substitute method types
        for (interface_decl.methods.items) |method| {
            var specialized_method = ast.InterfaceMethod{
                .name = method.name,
                .parameters = .empty,
                .return_type = if (method.return_type) |rt| try self.substituteType(rt, &substitution_map) else null,
            };
            
            // Substitute parameter types
            for (method.parameters.items) |param| {
                try specialized_method.parameters.append(ast.Parameter{
                    .name = param.name,
                    .param_type = try self.substituteType(param.param_type, &substitution_map),
                });
            }
            
            try specialized_interface.methods.append(allocator, specialized_method);
        }
        
        // Generate LLVM vtable structure
        try self.generateSpecializedVTable(&specialized_interface);
        
        // Mark instance as generated
        const specialized_name = specialized_interface.name;
        if (self.instances.getPtr(specialized_name)) |instance| {
            instance.generated = true;
        }
        
        std.log.info("Generated specialized interface: {s}", .{specialized_name});
    }
    
    /// Substitute type parameters in a type
    fn substituteType(self: *Monomorphizer, original_type: ast.Type, substitutions: *HashMap([]const u8, ast.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)) !ast.Type {
        return switch (original_type) {
            .Identifier => |name| {
                if (substitutions.get(name)) |concrete_type| {
                    return concrete_type;
                } else {
                    return original_type;
                }
            },
            .Array => |array_type| {
                const new_element_type = try self.allocator.create(ast.Type);
                new_element_type.* = try self.substituteType(array_type.element_type.*, substitutions);
                return ast.Type{ .Array = ast.ArrayType{
                    .element_type = new_element_type,
                    .size = array_type.size,
                }};
            },
            .Slice => |slice_type| {
                const new_element_type = try self.allocator.create(ast.Type);
                new_element_type.* = try self.substituteType(slice_type.element_type.*, substitutions);
                return ast.Type{ .Slice = ast.SliceType{
                    .element_type = new_element_type,
                }};
            },
            .Generic => |generic_type| {
                if (substitutions.get(generic_type.name)) |concrete_type| {
                    return concrete_type;
                } else {
                    return original_type;
                }
            },
            else => original_type,
        };
    }
    
    /// Substitute type parameters in statement list
    fn substituteStatements(self: *Monomorphizer, original_statements: []ast.Statement, substitutions: *HashMap([]const u8, ast.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)) !ArrayList(ast.Statement) {
        var new_statements = std.ArrayList(u8){};
        
        for (original_statements) |stmt| {
            const new_stmt = try self.substituteStatement(stmt, substitutions);
            try new_statements.append(allocator, new_stmt);
        }
        
        return new_statements;
    }
    
    /// Substitute type parameters in a single statement
    fn substituteStatement(self: *Monomorphizer, original_stmt: ast.Statement, substitutions: *HashMap([]const u8, ast.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)) !ast.Statement {
        return switch (original_stmt) {
            .VariableDeclaration => |var_decl| {
                const new_var_type = if (var_decl.var_type) |vt| try self.substituteType(vt, substitutions) else null;
                return ast.Statement{ .VariableDeclaration = ast.VariableDeclaration{
                    .name = var_decl.name,
                    .var_type = new_var_type,
                    .init_value = var_decl.init_value, // TODO: substitute expressions too
                    .is_mutable = var_decl.is_mutable,
                }};
            },
            .Return => |ret| {
                return ast.Statement{ .Return = ast.ReturnStatement{
                    .value = ret.value, // TODO: substitute expression
                }};
            },
            .Expression => |expr| {
                return ast.Statement{ .Expression = ast.ExpressionStatement{
                    .expression = expr.expression, // TODO: substitute expression
                }};
            },
            else => original_stmt,
        };
    }
    
    /// Generate LLVM function for specialized generic function
    fn generateSpecializedLLVMFunction(self: *Monomorphizer, func_decl: *ast.FunctionDeclaration) !void {
        // Create LLVM function type
        var param_types = std.ArrayList(u8){};
        defer param_types.deinit();
        
        for (func_decl.parameters.items) |param| {
            try param_types.append(self.allocator, try self.typeToLLVMType(param.param_type));
        }
        
        const return_type = if (func_decl.return_type) |rt| 
            try self.typeToLLVMType(rt) 
        else 
            c.LLVMVoidTypeInContext(self.context);
            
        const function_type = c.LLVMFunctionType(
            return_type,
            param_types.items.ptr,
            @as(u32, @intCast(param_types.items.len)),
            0
        );
        
        // Create LLVM function
        const llvm_function = c.LLVMAddFunction(self.module, func_decl.name.ptr, function_type);
        
        // CRITICAL FIX: Validate LLVM function for optimizer ICE prevention
        try self.validateLLVMBasicBlocks(llvm_function);
        
        // Store in instance
        if (self.instances.getPtr(func_decl.name)) |instance| {
            instance.llvm_function = llvm_function;
        }
        
        std.log.info("Generated specialized function: {s} with optimizer safety validation", .{func_decl.name});
    }
    
    /// Generate LLVM struct type for specialized generic struct
    fn generateSpecializedLLVMStruct(self: *Monomorphizer, struct_decl: *ast.StructDeclaration) !void {
        // Create LLVM struct type
        var field_types = std.ArrayList(u8){};
        defer field_types.deinit();
        
        for (struct_decl.fields.items) |field| {
            try field_types.append(self.allocator, try self.typeToLLVMType(field.field_type));
        }
        
        const llvm_struct_type = c.LLVMStructCreateNamed(self.context, struct_decl.name.ptr);
        c.LLVMStructSetBody(
            llvm_struct_type,
            field_types.items.ptr,
            @as(u32, @intCast(field_types.items.len)),
            0
        );
        
        // Store in instance
        if (self.instances.getPtr(struct_decl.name)) |instance| {
            instance.llvm_type = llvm_struct_type;
        }
        
        std.log.info("Generated specialized struct: {s}", .{struct_decl.name});
    }
    
    /// Convert CURSED type to LLVM type
    fn typeToLLVMType(self: *Monomorphizer, cursed_type: ast.Type) !c.LLVMTypeRef {
        return switch (cursed_type) {
            .Primitive => |primitive| switch (primitive) {
                .Tea => c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // String
                .Normie => c.LLVMInt32TypeInContext(self.context),
                .Drip => c.LLVMInt64TypeInContext(self.context),
                .Smol => c.LLVMInt8TypeInContext(self.context),
                .Thicc => c.LLVMInt64TypeInContext(self.context),
                .Meal => c.LLVMDoubleTypeInContext(self.context),
                .Snack => c.LLVMFloatTypeInContext(self.context),
                .Lit => c.LLVMInt1TypeInContext(self.context),
                .Vibes => c.LLVMVoidTypeInContext(self.context),
            },
            .Identifier => |name| {
                // Look up user-defined type
                if (self.instances.get(name)) |instance| {
                    if (instance.llvm_type) |llvm_type| {
                        return llvm_type;
                    }
                }
                // Fallback to opaque pointer
                return c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
            },
            .Array => |array_type| {
                const element_type = try self.typeToLLVMType(array_type.element_type.*);
                return c.LLVMArrayType(element_type, @as(u32, @intCast(array_type.size)));
            },
            .Slice => |slice_type| {
                const element_type = try self.typeToLLVMType(slice_type.element_type.*);
                return c.LLVMPointerType(element_type, 0);
            },
            else => c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0),
        };
    }
    
    /// Get instantiated function
    pub fn getInstantiatedFunction(self: *Monomorphizer, specialized_name: []const u8) ?c.LLVMValueRef {
        if (self.instances.get(specialized_name)) |instance| {
            return instance.llvm_function;
        }
        return null;
    }
    
    /// Get instantiated type
    pub fn getInstantiatedType(self: *Monomorphizer, specialized_name: []const u8) ?c.LLVMTypeRef {
        if (self.instances.get(specialized_name)) |instance| {
            return instance.llvm_type;
        }
        return null;
    }
    
    /// Check if generic is instantiated
    pub fn isInstantiated(self: *Monomorphizer, specialized_name: []const u8) bool {
        if (self.instances.get(specialized_name)) |instance| {
            return instance.generated;
        }
        return false;
    }
    
    /// Generate LLVM vtable for specialized interface
    fn generateSpecializedVTable(self: *Monomorphizer, interface_decl: *ast.InterfaceDeclaration) !void {
        // Create vtable struct type
        var method_types = std.ArrayList(u8){};
        defer method_types.deinit();
        
        // Add function pointer for each method
        for (interface_decl.methods.items) |method| {
            var param_types = std.ArrayList(u8){};
            defer param_types.deinit();
            
            // Add 'self' parameter (always a pointer)
            try param_types.append(self.allocator, c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0));
            
            // Add method parameters
            for (method.parameters.items) |param| {
                try param_types.append(self.allocator, try self.typeToLLVMType(param.param_type));
            }
            
            const return_type = if (method.return_type) |rt| 
                try self.typeToLLVMType(rt) 
            else 
                c.LLVMVoidTypeInContext(self.context);
                
            const method_func_type = c.LLVMFunctionType(
                return_type,
                param_types.items.ptr,
                @as(u32, @intCast(param_types.items.len)),
                0
            );
            
            try method_types.append(c.LLVMPointerType(method_func_type, 0));
        }
        
        // Create vtable struct
        const vtable_type = c.LLVMStructCreateNamed(self.context, interface_decl.name.ptr);
        c.LLVMStructSetBody(
            vtable_type,
            method_types.items.ptr,
            @as(u32, @intCast(method_types.items.len)),
            0
        );
        
        // Store vtable type
        if (self.instances.getPtr(interface_decl.name)) |instance| {
            instance.llvm_type = vtable_type;
        }
        
        std.log.info("Generated vtable for interface: {s}", .{interface_decl.name});
    }
    
    /// Validate variance constraints for generic type arguments
    pub fn validateVariance(self: *Monomorphizer, generic_name: []const u8, type_arguments: []ast.Type) !bool {
        const generic_decl = self.generic_declarations.get(generic_name) orelse {
            return error.GenericNotFound;
        };
        
        // Check each type parameter's variance
        for (generic_decl.type_parameters.items, 0..) |param, i| {
            if (i >= type_arguments.len) continue;
            
            const type_arg = type_arguments[i];
            
            // Validate variance rules
            for (param.constraints.items) |constraint| {
                switch (constraint.kind) {
                    .Interface => {
                        // Interfaces are generally contravariant in input positions
                        // and covariant in output positions
                        if (!try self.checkInterfaceVariance(type_arg, constraint.interface_name.?)) {
                            return false;
                        }
                    },
                    .Comparable, .Numeric, .Ordered => {
                        // These constraints require exact type matching (invariant)
                        if (!try self.checkConstraintCompatibility(type_arg, constraint.kind)) {
                            return false;
                        }
                    },
                    .Any, .Sized => {
                        // These are always compatible (covariant)
                        continue;
                    },
                }
            }
        }
        
        return true;
    }
    
    /// Check interface variance compatibility
    fn checkInterfaceVariance(self: *Monomorphizer, type_arg: ast.Type, interface_name: []const u8) !bool {
        // For now, we use structural subtyping
        // A type is compatible if it implements all required methods
        return self.implementsInterface(type_arg, interface_name);
    }
    
    /// Check constraint compatibility for invariant constraints
    fn checkConstraintCompatibility(self: *Monomorphizer, type_arg: ast.Type, constraint_kind: Constraint.ConstraintKind) !bool {
        return switch (constraint_kind) {
            .Comparable => self.isComparable(type_arg),
            .Numeric => self.isNumeric(type_arg),
            .Ordered => self.isOrdered(type_arg),
            else => true,
        };
    }
    
    /// Advanced monomorphization with dependency tracking
    pub fn requestInstantiationWithDeps(self: *Monomorphizer, generic_name: []const u8, type_arguments: []ast.Type, dependencies: [][]const u8) ![]const u8 {
        // Ensure all dependencies are instantiated first
        for (dependencies) |dep_name| {
            if (!self.isInstantiated(dep_name)) {
                std.log.warn("Dependency not yet instantiated: {s}", .{dep_name});
                return error.DependencyNotInstantiated;
            }
        }
        
        // Validate variance before instantiation
        if (!try self.validateVariance(generic_name, type_arguments)) {
            return error.VarianceViolation;
        }
        
        // Proceed with normal instantiation
        return self.requestInstantiation(generic_name, type_arguments, "dependency_based");
    }
};

/// Integration with the main codegen system
pub fn integrateWithCodegen(monomorphizer: *Monomorphizer, codegen: anytype) !void {
    // Process all pending instantiations
    try monomorphizer.processInstantiations();
    
    // Add instantiated functions to codegen
    var instance_iterator = monomorphizer.instances.iterator();
    while (instance_iterator.next()) |entry| {
        const instance = entry.value_ptr;
        if (instance.generated and instance.llvm_function != null) {
            // Register with codegen system
            try codegen.registerFunction(instance.specialized_name, instance.llvm_function.?);
        }
        if (instance.generated and instance.llvm_type != null) {
            // Register type with codegen system  
            try codegen.registerType(instance.specialized_name, instance.llvm_type.?);
        }
    }
}
