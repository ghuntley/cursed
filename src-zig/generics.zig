/// Complete monomorphization system for CURSED generics with [T] syntax
const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Types.h");
    @cInclude("llvm-c/Target.h");
});

const ast = @import("ast_fixed.zig");
const type_system = @import("type_system_runtime.zig");

/// Generic type parameter with constraints
pub const TypeParameter = struct {
    name: []const u8,
    constraints: ArrayList(Constraint),
    
    pub fn init(allocator: Allocator, name: []const u8) TypeParameter {
        return TypeParameter{
            .name = name,
            .constraints = ArrayList(Constraint).init(allocator),
        };
    }
    
    pub fn deinit(self: *TypeParameter, allocator: Allocator) void {
        for (self.constraints.items) |*constraint| {
            constraint.deinit(allocator);
        }
        self.constraints.deinit();
    }
};

/// Type constraints for generic parameters
pub const Constraint = struct {
    kind: ConstraintKind,
    interface_name: ?[]const u8 = null,
    
    pub const ConstraintKind = enum {
        Any,           // No constraints - T
        Comparable,    // T: Comparable - can use ==, !=
        Numeric,       // T: Numeric - supports +, -, *, /
        Ordered,       // T: Ordered - supports <, >, <=, >=
        Interface,     // T: SomeInterface - implements interface
        Sized,         // T: Sized - has known size at compile time
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
        Function: *ast.FunctionDeclaration,
        Struct: *ast.StructDeclaration,  
        Interface: *ast.InterfaceDeclaration,
    };
    
    pub fn init(allocator: Allocator, name: []const u8, kind: DeclarationKind) GenericDeclaration {
        return GenericDeclaration{
            .name = name,
            .type_parameters = ArrayList(TypeParameter).init(allocator),
            .kind = kind,
            .ast_node = undefined, // Set by caller
        };
    }
    
    pub fn deinit(self: *GenericDeclaration, allocator: Allocator) void {
        for (self.type_parameters.items) |*param| {
            param.deinit(allocator);
        }
        self.type_parameters.deinit();
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
            .substitutions = ArrayList(TypeSubstitution).init(allocator),
            .specialized_name = specialized_name,
        };
    }
    
    pub fn deinit(self: *MonomorphizedInstance, allocator: Allocator) void {
        self.substitutions.deinit();
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
    
    pub const InstantiationRequest = struct {
        generic_name: []const u8,
        type_arguments: ArrayList(ast.Type),
        usage_location: []const u8, // For error reporting
        
        pub fn init(allocator: Allocator, generic_name: []const u8, usage_location: []const u8) InstantiationRequest {
            return InstantiationRequest{
                .generic_name = generic_name,
                .type_arguments = ArrayList(ast.Type).init(allocator),
                .usage_location = usage_location,
            };
        }
        
        pub fn deinit(self: *InstantiationRequest, allocator: Allocator) void {
            self.type_arguments.deinit();
            _ = allocator;
        }
    };
    
    pub fn init(allocator: Allocator, context: c.LLVMContextRef, module: c.LLVMModuleRef) Monomorphizer {
        return Monomorphizer{
            .allocator = allocator,
            .context = context,
            .module = module,
            .generic_declarations = HashMap([]const u8, GenericDeclaration, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .instances = HashMap([]const u8, MonomorphizedInstance, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .work_queue = ArrayList(InstantiationRequest).init(allocator),
        };
    }
    
    pub fn deinit(self: *Monomorphizer) void {
        var decl_iterator = self.generic_declarations.iterator();
        while (decl_iterator.next()) |entry| {
            entry.value_ptr.deinit(self.allocator);
        }
        self.generic_declarations.deinit();
        
        var instance_iterator = self.instances.iterator();
        while (instance_iterator.next()) |entry| {
            entry.value_ptr.deinit(self.allocator);
        }
        self.instances.deinit();
        
        for (self.work_queue.items) |*request| {
            request.deinit(self.allocator);
        }
        self.work_queue.deinit();
    }
    
    /// Register a generic declaration
    pub fn registerGeneric(self: *Monomorphizer, declaration: GenericDeclaration) !void {
        const owned_name = try self.allocator.dupe(u8, declaration.name);
        try self.generic_declarations.put(owned_name, declaration);
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
            try request.type_arguments.append(type_arg);
        }
        try self.work_queue.append(request);
        
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
            try instance.substitutions.append(TypeSubstitution.init(param_name, type_arg));
        }
        
        try self.instances.put(try self.allocator.dupe(u8, specialized_name), instance);
        return specialized_name;
    }
    
    /// Process all pending instantiations
    pub fn processInstantiations(self: *Monomorphizer) !void {
        while (self.work_queue.items.len > 0) {
            const request = self.work_queue.pop();
            defer request.deinit(self.allocator);
            
            try self.instantiateGeneric(request);
        }
    }
    
    /// Generate specialized name for generic instantiation
    fn generateSpecializedName(self: *Monomorphizer, generic_name: []const u8, type_arguments: []ast.Type) ![]const u8 {
        var name_builder = ArrayList(u8).init(self.allocator);
        defer name_builder.deinit();
        
        try name_builder.appendSlice(generic_name);
        
        for (type_arguments) |type_arg| {
            try name_builder.appendSlice("_");
            const type_name = try self.typeToString(type_arg);
            defer self.allocator.free(type_name);
            try name_builder.appendSlice(type_name);
        }
        
        return name_builder.toOwnedSlice();
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
    
    /// Validate type constraints
    fn validateConstraints(self: *Monomorphizer, generic_decl: GenericDeclaration, type_arguments: []ast.Type) !void {
        for (generic_decl.type_parameters.items, 0..) |param, i| {
            const type_arg = type_arguments[i];
            
            for (param.constraints.items) |constraint| {
                const valid = switch (constraint.kind) {
                    .Any => true,
                    .Comparable => try self.isComparable(type_arg),
                    .Numeric => try self.isNumeric(type_arg),
                    .Ordered => try self.isOrdered(type_arg),
                    .Interface => try self.implementsInterface(type_arg, constraint.interface_name.?),
                    .Sized => try self.isSized(type_arg),
                };
                
                if (!valid) {
                    std.log.err("Type constraint violation: {s} does not satisfy constraint {}", 
                        .{try self.typeToString(type_arg), constraint.kind});
                    return error.ConstraintViolation;
                }
            }
        }
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
        _ = self;
        _ = type_arg;
        _ = interface_name;
        // TODO: Implement interface checking
        return true;
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
        var substitution_map = HashMap([]const u8, ast.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator);
        defer substitution_map.deinit();
        
        for (generic_decl.type_parameters.items, 0..) |param, i| {
            try substitution_map.put(param.name, type_arguments[i]);
        }
        
        // Create specialized function declaration
        var specialized_func = ast.FunctionDeclaration{
            .name = try self.generateSpecializedName(generic_decl.name, type_arguments),
            .parameters = ArrayList(ast.Parameter).init(self.allocator),
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
        var substitution_map = HashMap([]const u8, ast.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator);
        defer substitution_map.deinit();
        
        for (generic_decl.type_parameters.items, 0..) |param, i| {
            try substitution_map.put(param.name, type_arguments[i]);
        }
        
        // Create specialized struct declaration
        var specialized_struct = ast.StructDeclaration{
            .name = try self.generateSpecializedName(generic_decl.name, type_arguments),
            .fields = ArrayList(ast.StructField).init(self.allocator),
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
        _ = self;
        _ = generic_decl;
        _ = type_arguments;
        // TODO: Implement generic interface instantiation
        std.log.warn("Generic interface instantiation not yet implemented", .{});
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
        var new_statements = ArrayList(ast.Statement).init(self.allocator);
        
        for (original_statements) |stmt| {
            const new_stmt = try self.substituteStatement(stmt, substitutions);
            try new_statements.append(new_stmt);
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
        var param_types = ArrayList(c.LLVMTypeRef).init(self.allocator);
        defer param_types.deinit();
        
        for (func_decl.parameters.items) |param| {
            try param_types.append(try self.typeToLLVMType(param.param_type));
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
        
        // Store in instance
        if (self.instances.getPtr(func_decl.name)) |instance| {
            instance.llvm_function = llvm_function;
        }
        
        std.log.info("Generated specialized function: {s}", .{func_decl.name});
    }
    
    /// Generate LLVM struct type for specialized generic struct
    fn generateSpecializedLLVMStruct(self: *Monomorphizer, struct_decl: *ast.StructDeclaration) !void {
        // Create LLVM struct type
        var field_types = ArrayList(c.LLVMTypeRef).init(self.allocator);
        defer field_types.deinit();
        
        for (struct_decl.fields.items) |field| {
            try field_types.append(try self.typeToLLVMType(field.field_type));
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
