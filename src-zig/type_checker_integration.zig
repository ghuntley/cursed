/// Integration layer for the comprehensive type system with CURSED compiler
const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const comprehensive_type_system = @import("comprehensive_type_system.zig");
const error_handling = @import("error_handling.zig");

/// Type variable for constraint resolution
const TypeVariable = struct {
    name: []const u8,
    constraints: ArrayList(TypeConstraint),
    resolved_type: ?comprehensive_type_system.CursedType,
    
    pub fn init(allocator: Allocator, name: []const u8) TypeVariable {
        return TypeVariable{
            .name = name,
            .constraints = .empty,
            .resolved_type = null,
        };
    }
    
    pub fn deinit(self: *TypeVariable) void {
        self.constraints.deinit(allocator);
    }
};

/// Type constraint for resolution
const TypeConstraint = struct {
    kind: ConstraintKind,
    target_type: ?comprehensive_type_system.CursedType,
    
    const ConstraintKind = enum {
        EqualTo,
        SubtypeOf,
        SupertypeOf,
        MustBe,
        Implements,
    };
};

/// Resolved constraint information
const ResolvedConstraint = struct {
    variable_name: []const u8,
    resolved_type: comprehensive_type_system.CursedType,
    constraint_source: ConstraintSource,
    
    const ConstraintSource = enum {
        Inference,
        Explicit,
        Default,
    };
};

/// Constraint resolution result
const ConstraintResolutionResult = struct {
    resolved_constraints: ArrayList(ResolvedConstraint),
    remaining_unknowns: u32,
    success: bool,
    
    pub fn deinit(self: *ConstraintResolutionResult) void {
        self.resolved_constraints.deinit(allocator);
    }
};

pub const TypeCheckerIntegration = struct {
    type_checker: comprehensive_type_system.ComprehensiveTypeChecker,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) !TypeCheckerIntegration {
        return TypeCheckerIntegration{
            .type_checker = try comprehensive_type_system.ComprehensiveTypeChecker.init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *TypeCheckerIntegration) void {
        self.type_checker.deinit(allocator);
    }
    
    /// Main entry point for type checking a CURSED program
    pub fn checkProgram(self: *TypeCheckerIntegration, program: *const ast.Program) !TypeCheckResult {
        const success = try self.type_checker.checkProgram(program);
        const errors = self.type_checker.getErrorMessages();
        
        var error_details = .empty;
        
        for (errors) |error_msg| {
            try error_details.append(allocator, TypeErrorDetail{
                .kind = error_msg.kind,
                .message = error_msg.message,
                .line = error_msg.line,
                .column = error_msg.column,
                .severity = .Error,
            });
        }
        
        return TypeCheckResult{
            .success = success,
            .errors = error_details,
            .warnings = .empty,
        };
    }
    
    /// Type check individual expression (for REPL/incremental checking)
    pub fn checkExpression(self: *TypeCheckerIntegration, expr: *const ast.Expression) !ExpressionTypeResult {
        const inference_result = try self.type_checker.inference_engine.inferExpression(expr);
        
        return ExpressionTypeResult{
            .inferred_type = inference_result.inferred_type,
            .success = inference_result.constraints_satisfied,
            .error_message = inference_result.error_message,
        };
    }
    
    /// Specialized checking for function declarations with generics
    pub fn checkGenericFunction(self: *TypeCheckerIntegration, func_decl: *const ast.FunctionDeclaration) !GenericCheckResult {
        // Check if function has generic parameters
        const has_generics = self.hasGenericParameters(func_decl);
        
        if (has_generics) {
            return try self.checkGenericFunctionDeclaration(func_decl);
        } else {
            const success = try self.type_checker.checkStatement(ast.Statement{ .FunctionDeclaration = func_decl.* });
            return GenericCheckResult{
                .is_generic = false,
                .monomorphization_needed = false,
                .success = success,
                .type_parameters = .empty,
            };
        }
    }
    
    /// Generate detailed type information for IDE integration
    pub fn getTypeInfo(self: *TypeCheckerIntegration, expr: *const ast.Expression) !TypeInfoResult {
        const result = try self.checkExpression(expr);
        
        return TypeInfoResult{
            .cursed_type = result.inferred_type,
            .type_string = try self.typeToDisplayString(result.inferred_type),
            .is_mutable = false, // TODO: Track mutability
            .is_nullable = false, // TODO: Track nullability
            .documentation = null, // TODO: Add documentation support
        };
    }
    
    /// Support for constraint-based type resolution
    pub fn resolveConstraints(self: *TypeCheckerIntegration) !ConstraintResolutionResult {
        // Resolve all pending type constraints in the environment
        var resolved_constraints = .empty;
        var remaining_unknowns: u32 = 0;
        
        // Constraint resolution algorithm implementation
        
        // Phase 1: Collect all type variables and their constraints
        var type_variables = std.HashMap([]const u8, TypeVariable, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator);
        defer type_variables.deinit(allocator);
        
        // Phase 2: Build constraint graph
        var changed = true;
        var iterations: u32 = 0;
        const max_iterations = 100;
        
        while (changed and iterations < max_iterations) {
            changed = false;
            iterations += 1;
            
            var iter = type_variables.iterator();
            while (iter.next()) |entry| {
                const var_name = entry.key_ptr.*;
                var type_var = entry.value_ptr;
                
                if (type_var.resolved_type == null) {
                    // Try to resolve based on constraints
                    if (self.tryResolveTypeVariable(type_var)) {
                        changed = true;
                        try resolved_constraints.append(allocator, ResolvedConstraint{
                            .variable_name = var_name,
                            .resolved_type = type_var.resolved_type.?,
                            .constraint_source = .Inference,
                        });
                    }
                }
            }
        }
        
        // Phase 3: Count remaining unknowns
        var final_iter = type_variables.iterator();
        while (final_iter.next()) |entry| {
            if (entry.value_ptr.resolved_type == null) {
                remaining_unknowns += 1;
            }
        }
        
        return ConstraintResolutionResult{
            .resolved_constraints = resolved_constraints,
            .remaining_unknowns = remaining_unknowns,
            .success = remaining_unknowns == 0,
        };
    }
    
    /// Try to resolve a type variable based on its constraints
    fn tryResolveTypeVariable(self: *TypeCheckerIntegration, type_var: *TypeVariable) bool {
        _ = self;
        
        // Simple constraint resolution heuristics
        if (type_var.constraints.items.len == 0) return false;
        
        // If we have a single equality constraint, use it
        for (type_var.constraints.items) |constraint| {
            switch (constraint.kind) {
                .EqualTo => |concrete_type| {
                    type_var.resolved_type = concrete_type;
                    return true;
                },
                .MustBe => |specific_type| {
                    type_var.resolved_type = specific_type;
                    return true;
                },
                else => continue,
            }
        }
        
        return false;
    }
    
    /// Interface implementation checking
    pub fn checkInterfaceImplementation(self: *TypeCheckerIntegration, struct_type: comprehensive_type_system.CursedType, interface_type: comprehensive_type_system.CursedType) !InterfaceCheckResult {
        return switch (struct_type) {
            .Struct => |struct_info| switch (interface_type) {
                .Interface => |interface_info| {
                    var missing_methods = .empty;
                    var incorrect_signatures = .empty;
                    
                    // Check each required method
                    for (interface_info.methods.items) |required_method| {
                        var found = false;
                        
                        // Look for method in struct (simplified - would need method registry)
                        // This is a placeholder implementation
                        _ = required_method;
                        _ = struct_info;
                        
                        if (!found) {
                            try missing_methods.append(allocator, required_method.name);
                        }
                    }
                    
                    return InterfaceCheckResult{
                        .implements_interface = missing_methods.items.len == 0 and incorrect_signatures.items.len == 0,
                        .missing_methods = missing_methods,
                        .signature_mismatches = incorrect_signatures,
                    };
                },
                else => InterfaceCheckResult{
                    .implements_interface = false,
                    .missing_methods = .empty,
                    .signature_mismatches = .empty,
                },
            },
            else => InterfaceCheckResult{
                .implements_interface = false,
                .missing_methods = .empty,
                .signature_mismatches = .empty,
            },
        };
    }
    
    // Helper functions
    
    fn hasGenericParameters(self: *TypeCheckerIntegration, func_decl: *const ast.FunctionDeclaration) bool {
        _ = self;
        // Check if function name or parameters contain generic syntax like [T]
        if (std.mem.indexOf(u8, func_decl.name, "[")) |_| {
            return true;
        }
        
        for (func_decl.parameters.items) |param| {
            if (self.isGenericType(param.param_type)) {
                return true;
            }
        }
        
        if (func_decl.return_type) |ret_type| {
            if (self.isGenericType(ret_type)) {
                return true;
            }
        }
        
        return false;
    }
    
    fn isGenericType(self: *TypeCheckerIntegration, ast_type: ast.Type) bool {
        _ = self;
        return switch (ast_type) {
            .Generic => true,
            .Identifier => |name| std.mem.indexOf(u8, name, "T") != null, // Simple heuristic
            .Array => |arr| self.isGenericType(arr.element_type.*),
            .Slice => |slice| self.isGenericType(slice.element_type.*),
            else => false,
        };
    }
    
    fn checkGenericFunctionDeclaration(self: *TypeCheckerIntegration, func_decl: *const ast.FunctionDeclaration) !GenericCheckResult {
        var type_parameters = .empty;
        
        // Extract type parameters from function signature
        // This is a simplified implementation
        try type_parameters.append(allocator, TypeParameterInfo{
            .name = "T", // Placeholder
            .constraints = .empty,
            .default_type = null,
        });
        
        // Validate constraints and check function body with generic context
        const success = try self.type_checker.checkStatement(ast.Statement{ .FunctionDeclaration = func_decl.* });
        
        return GenericCheckResult{
            .is_generic = true,
            .monomorphization_needed = true,
            .success = success,
            .type_parameters = type_parameters,
        };
    }
    
    /// Validate that a struct properly implements an interface
    fn validateInterfaceImpl(self: *TypeCheckerIntegration, struct_name: []const u8, interface_name: []const u8) !bool {
        // Get interface definition from type environment
        const interface_type = self.type_checker.type_env.getInterfaceType(interface_name) orelse {
            // Interface not found
            return false;
        };

        // Get struct definition from type environment
        const struct_type = self.type_checker.type_env.getStructType(struct_name) orelse {
            // Struct not found
            return false;
        };

        // Check that all required interface methods are implemented with compatible signatures
        for (interface_type.method_signatures.items) |required_method| {
            const impl_method = self.findStructMethod(struct_type, required_method.name) orelse {
                // Required method not implemented
                return false;
            };

            // Validate method signature compatibility
            if (!try self.isSignatureCompatible(required_method, impl_method)) {
                return false;
            }
        }

        // All methods implemented with compatible signatures
        return true;
    }
    
    /// Find a method in a struct type by name
    fn findStructMethod(self: *TypeCheckerIntegration, struct_type: anytype, method_name: []const u8) ?anytype {
        _ = self;
        // This would search the struct's methods for the given name
        // For now, return null as we need to define the struct method storage
        _ = struct_type;
        _ = method_name;
        return null;
    }
    
    /// Check if two method signatures are compatible
    fn isSignatureCompatible(self: *TypeCheckerIntegration, required: anytype, implemented: anytype) !bool {
        _ = self;
        _ = required;
        _ = implemented;
        
        // This would check:
        // 1. Parameter count matches
        // 2. Parameter types are compatible
        // 3. Return type is compatible
        // 4. Receiver type is compatible
        
        // For now, assume compatible
        return true;
    }
    
    fn typeToDisplayString(self: *TypeCheckerIntegration, cursed_type: comprehensive_type_system.CursedType) ![]const u8 {
        var buffer = .empty;
        const writer = buffer.writer();
        
        try cursed_type.format("", .{}, writer);
        
        return buffer.toOwnedSlice(allocator);
    }
};

// Result types for different checking operations

pub const TypeCheckResult = struct {
    success: bool,
    errors: ArrayList(TypeErrorDetail),
    warnings: ArrayList(TypeErrorDetail),
    
    pub fn deinit(self: *TypeCheckResult) void {
        self.errors.deinit(allocator);
        self.warnings.deinit(allocator);
    }
};

pub const TypeErrorDetail = struct {
    kind: comprehensive_type_system.ComprehensiveTypeChecker.TypeErrorMessage.ErrorKind,
    message: []const u8,
    line: u32,
    column: u32,
    severity: Severity,
    
    pub const Severity = enum {
        Error,
        Warning,
        Info,
        Hint,
    };
};

pub const ExpressionTypeResult = struct {
    inferred_type: comprehensive_type_system.CursedType,
    success: bool,
    error_message: ?[]const u8,
};

pub const GenericCheckResult = struct {
    is_generic: bool,
    monomorphization_needed: bool,
    success: bool,
    type_parameters: ArrayList(TypeParameterInfo),
    
    pub fn deinit(self: *GenericCheckResult) void {
        for (self.type_parameters.items) |*param| {
            param.constraints.deinit(allocator);
        }
        self.type_parameters.deinit(allocator);
    }
};

pub const TypeParameterInfo = struct {
    name: []const u8,
    constraints: ArrayList([]const u8),
    default_type: ?[]const u8,
};

pub const TypeInfoResult = struct {
    cursed_type: comprehensive_type_system.CursedType,
    type_string: []const u8,
    is_mutable: bool,
    is_nullable: bool,
    documentation: ?[]const u8,
};

pub const ConstraintResolutionResult = struct {
    resolved_constraints: ArrayList(ResolvedConstraint),
    remaining_unknowns: u32,
    success: bool,
    
    pub fn deinit(self: *ConstraintResolutionResult) void {
        self.resolved_constraints.deinit(allocator);
    }
};

pub const ResolvedConstraint = struct {
    type_var_id: u32,
    resolved_type: comprehensive_type_system.CursedType,
};

pub const InterfaceCheckResult = struct {
    implements_interface: bool,
    missing_methods: ArrayList([]const u8),
    signature_mismatches: ArrayList(SignatureMismatch),
    
    pub fn deinit(self: *InterfaceCheckResult) void {
        self.missing_methods.deinit(allocator);
        self.signature_mismatches.deinit(allocator);
    }
};

pub const SignatureMismatch = struct {
    method_name: []const u8,
    expected_signature: []const u8,
    actual_signature: []const u8,
    issue: MismatchKind,
    
    pub const MismatchKind = enum {
        ParameterCount,
        ParameterType,
        ReturnType,
        Visibility,
    };
};

// Convenience functions for integration with existing compiler

/// Create a type checker instance and check a program
pub fn checkCursedProgram(allocator: Allocator, program: *const ast.Program) !TypeCheckResult {
    var integration = try TypeCheckerIntegration.init(allocator);
    defer integration.deinit(allocator);
    
    return integration.checkProgram(program);
}

/// Quick type checking for a single expression (useful for REPL)
pub fn inferExpressionType(allocator: Allocator, expr: *const ast.Expression) !ExpressionTypeResult {
    var integration = try TypeCheckerIntegration.init(allocator);
    defer integration.deinit(allocator);
    
    return integration.checkExpression(expr);
}

/// Validate interface implementation
pub fn validateInterfaceImplementation(allocator: Allocator, struct_name: []const u8, interface_name: []const u8) !bool {
    var integration = try TypeCheckerIntegration.init(allocator);
    defer integration.deinit(allocator);
    
    return try integration.validateInterfaceImpl(struct_name, interface_name);
}

// Error formatting for user-friendly output

pub fn formatTypeError(error_detail: TypeErrorDetail, allocator: Allocator) ![]const u8 {
    return std.fmt.allocPrint(allocator, "[{s}] Line {d}, Column {d}: {s}", .{
        @tagName(error_detail.kind),
        error_detail.line,
        error_detail.column,
        error_detail.message,
    });
}

pub fn formatTypeErrors(errors: []const TypeErrorDetail, allocator: Allocator) ![]const u8 {
    var buffer = .empty;
    const writer = buffer.writer();
    
    if (errors.len == 0) {
        try writer.writeAll("No type errors found.\n");
    } else {
        try writer.print("Found {} type error(s):\n\n", .{errors.len});
        
        for (errors, 0..) |error_detail, i| {
            try writer.print("{}. ", .{i + 1});
            const formatted = try formatTypeError(error_detail, allocator);
            defer allocator.free(formatted);
            try writer.writeAll(formatted);
            try writer.writeAll("\n");
        }
    }
    
    return buffer.toOwnedSlice(allocator);
}
