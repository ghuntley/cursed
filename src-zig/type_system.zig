const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const error_handling = @import("error_handling.zig");
const CursedError = error_handling.CursedError;

// Core type system structures
pub const TypeKind = enum {
    Primitive,
    Struct,
    Interface,
    Function,
    Array,
    Tuple,
    Map,
    Pointer,
    Generic,
    Named,

    pub fn toString(self: TypeKind) []const u8 {
        return switch (self) {
            .Primitive => "Primitive",
            .Struct => "Struct",
            .Interface => "Interface",
            .Function => "Function",
            .Array => "Array",
            .Tuple => "Tuple",
            .Map => "Map",
            .Pointer => "Pointer",
            .Generic => "Generic",
            .Named => "Named",
        };
    }
};

pub const TypeExpression = struct {
    kind: TypeKind,
    name: ?[]const u8,
    parameters: ArrayList(TypeExpression),
    return_type: ?*TypeExpression,
    allocator: Allocator,

    pub fn init(allocator: Allocator, kind: TypeKind, name: ?[]const u8) TypeExpression {
        return TypeExpression{
            .kind = kind,
            .name = if (name) |n| allocator.dupe(u8, n) catch n else null,
            .parameters = ArrayList(TypeExpression).init(allocator),
            .return_type = null,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *TypeExpression) void {
        for (self.parameters.items) |*param| {
            param.deinit();
        }
        self.parameters.deinit();
        if (self.name) |name| {
            self.allocator.free(name);
        }
        if (self.return_type) |ret_type| {
            ret_type.deinit();
            self.allocator.destroy(ret_type);
        }
    }

    pub fn named(allocator: Allocator, name: []const u8) TypeExpression {
        return TypeExpression.init(allocator, .Named, name);
    }

    pub fn primitive(allocator: Allocator, name: []const u8) TypeExpression {
        return TypeExpression.init(allocator, .Primitive, name);
    }

    pub fn array(allocator: Allocator, element_type: TypeExpression) !TypeExpression {
        var arr_type = TypeExpression.init(allocator, .Array, "Array");
        try arr_type.parameters.append(element_type);
        return arr_type;
    }

    pub fn tuple(allocator: Allocator, element_types: []const TypeExpression) !TypeExpression {
        var tuple_type = TypeExpression.init(allocator, .Tuple, "Tuple");
        for (element_types) |element| {
            try tuple_type.parameters.append(element);
        }
        return tuple_type;
    }

    pub fn map(allocator: Allocator, key_type: TypeExpression, value_type: TypeExpression) !TypeExpression {
        var map_type = TypeExpression.init(allocator, .Map, "Map");
        try map_type.parameters.append(key_type);
        try map_type.parameters.append(value_type);
        return map_type;
    }

    pub fn pointer(allocator: Allocator, pointee_type: TypeExpression) !TypeExpression {
        var ptr_type = TypeExpression.init(allocator, .Pointer, "Pointer");
        try ptr_type.parameters.append(pointee_type);
        return ptr_type;
    }

    pub fn equals(self: *const TypeExpression, other: *const TypeExpression) bool {
        if (self.kind != other.kind) return false;
        
        if (self.name) |self_name| {
            if (other.name) |other_name| {
                if (!std.mem.eql(u8, self_name, other_name)) return false;
            } else {
                return false;
            }
        } else if (other.name != null) {
            return false;
        }

        if (self.parameters.items.len != other.parameters.items.len) return false;
        
        for (self.parameters.items, 0..) |*param, i| {
            if (!param.equals(&other.parameters.items[i])) return false;
        }

        return true;
    }

    pub fn isNumeric(self: *const TypeExpression) bool {
        if (self.name) |name| {
            return std.mem.eql(u8, name, "drip") or
                   std.mem.eql(u8, name, "normie") or
                   std.mem.eql(u8, name, "thicc") or
                   std.mem.eql(u8, name, "smol") or
                   std.mem.eql(u8, name, "mid") or
                   std.mem.eql(u8, name, "snack") or
                   std.mem.eql(u8, name, "meal");
        }
        return false;
    }

    pub fn isBoolean(self: *const TypeExpression) bool {
        if (self.name) |name| {
            return std.mem.eql(u8, name, "lit");
        }
        return false;
    }

    pub fn isString(self: *const TypeExpression) bool {
        if (self.name) |name| {
            return std.mem.eql(u8, name, "tea");
        }
        return false;
    }
};

pub const MethodSignature = struct {
    name: []const u8,
    parameters: ArrayList(TypeExpression),
    return_type: ?TypeExpression,
    allocator: Allocator,

    pub fn init(allocator: Allocator, name: []const u8) MethodSignature {
        return MethodSignature{
            .name = allocator.dupe(u8, name) catch name,
            .parameters = ArrayList(TypeExpression).init(allocator),
            .return_type = null,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *MethodSignature) void {
        self.allocator.free(self.name);
        for (self.parameters.items) |*param| {
            param.deinit();
        }
        self.parameters.deinit();
        if (self.return_type) |*ret_type| {
            ret_type.deinit();
        }
    }
};

pub const TypeDefinition = struct {
    name: []const u8,
    kind: TypeKind,
    methods: ArrayList(MethodSignature),
    is_builtin: bool,
    allocator: Allocator,

    pub fn init(allocator: Allocator, name: []const u8, kind: TypeKind) TypeDefinition {
        return TypeDefinition{
            .name = allocator.dupe(u8, name) catch name,
            .kind = kind,
            .methods = ArrayList(MethodSignature).init(allocator),
            .is_builtin = false,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *TypeDefinition) void {
        self.allocator.free(self.name);
        for (self.methods.items) |*method| {
            method.deinit();
        }
        self.methods.deinit();
    }

    pub fn addMethod(self: *TypeDefinition, method: MethodSignature) !void {
        try self.methods.append(method);
    }

    pub fn getMethod(self: *const TypeDefinition, name: []const u8) ?*const MethodSignature {
        for (self.methods.items) |*method| {
            if (std.mem.eql(u8, method.name, name)) {
                return method;
            }
        }
        return null;
    }
};

// Type environment for symbol table management
pub const TypeEnvironment = struct {
    type_definitions: HashMap([]const u8, TypeDefinition, StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,

    const StringContext = struct {
        pub fn hash(self: @This(), s: []const u8) u64 {
            _ = self;
            return std.hash_map.hashString(s);
        }
        pub fn eql(self: @This(), a: []const u8, b: []const u8) bool {
            _ = self;
            return std.mem.eql(u8, a, b);
        }
    };

    pub fn init(allocator: Allocator) TypeEnvironment {
        return TypeEnvironment{
            .type_definitions = HashMap([]const u8, TypeDefinition, StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *TypeEnvironment) void {
        var iter = self.type_definitions.iterator();
        while (iter.next()) |entry| {
            var type_def = entry.value_ptr;
            type_def.deinit();
        }
        self.type_definitions.deinit();
    }

    pub fn addBuiltinType(self: *TypeEnvironment, name: []const u8, kind: TypeKind) !void {
        var type_def = TypeDefinition.init(self.allocator, name, kind);
        type_def.is_builtin = true;
        try self.type_definitions.put(name, type_def);
    }

    pub fn addTypeDefinition(self: *TypeEnvironment, type_def: TypeDefinition) !void {
        try self.type_definitions.put(type_def.name, type_def);
    }

    pub fn getType(self: *const TypeEnvironment, name: []const u8) ?*const TypeDefinition {
        return self.type_definitions.getPtr(name);
    }

    pub fn hasType(self: *const TypeEnvironment, name: []const u8) bool {
        return self.type_definitions.contains(name);
    }
};

// Variable information for scoping
pub const VariableInfo = struct {
    name: []const u8,
    type_expr: TypeExpression,
    is_mutable: bool,
    is_initialized: bool,
    allocator: Allocator,

    pub fn init(allocator: Allocator, name: []const u8, type_expr: TypeExpression, is_mutable: bool) VariableInfo {
        return VariableInfo{
            .name = allocator.dupe(u8, name) catch name,
            .type_expr = type_expr,
            .is_mutable = is_mutable,
            .is_initialized = false,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *VariableInfo) void {
        self.allocator.free(self.name);
        // Note: type_expr is owned by caller, don't deinit here
    }
};

// Type checking error types
pub const TypeCheckError = struct {
    message: []const u8,
    kind: TypeErrorKind,
    location: ?SourceLocation,
    allocator: Allocator,

    pub const TypeErrorKind = enum {
        UnknownType,
        TypeMismatch,
        UnknownVariable,
        UnknownMethod,
        InvalidOperation,
        ArgumentCountMismatch,
        InvalidAccess,
    };

    pub const SourceLocation = struct {
        line: usize,
        column: usize,
        file: []const u8,
    };

    pub fn init(allocator: Allocator, kind: TypeErrorKind, message: []const u8) TypeCheckError {
        return TypeCheckError{
            .message = allocator.dupe(u8, message) catch message,
            .kind = kind,
            .location = null,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *TypeCheckError) void {
        self.allocator.free(self.message);
    }
};

// Main type checker
pub const TypeChecker = struct {
    environment: TypeEnvironment,
    scopes: ArrayList(HashMap([]const u8, VariableInfo, TypeEnvironment.StringContext, std.hash_map.default_max_load_percentage)),
    allocator: Allocator,

    pub fn init(allocator: Allocator) !TypeChecker {
        var environment = TypeEnvironment.init(allocator);
        
        // Add CURSED built-in types
        try environment.addBuiltinType("lit", .Primitive);      // boolean
        try environment.addBuiltinType("drip", .Primitive);     // integer  
        try environment.addBuiltinType("normie", .Primitive);   // integer
        try environment.addBuiltinType("thicc", .Primitive);    // large integer
        try environment.addBuiltinType("smol", .Primitive);     // small integer
        try environment.addBuiltinType("mid", .Primitive);      // medium integer
        try environment.addBuiltinType("tea", .Primitive);      // string
        try environment.addBuiltinType("sip", .Primitive);      // character
        try environment.addBuiltinType("snack", .Primitive);    // float
        try environment.addBuiltinType("meal", .Primitive);     // double
        try environment.addBuiltinType("byte", .Primitive);     // byte
        try environment.addBuiltinType("rune", .Primitive);     // unicode char
        try environment.addBuiltinType("cap", .Primitive);      // void/unit

        // Add vibez built-in object with spill method
        var vibez_type = TypeDefinition.init(allocator, "vibez", .Struct);
        vibez_type.is_builtin = true;
        
        var spill_method = MethodSignature.init(allocator, "spill");
        try spill_method.parameters.append(TypeExpression.named(allocator, "tea"));
        spill_method.return_type = TypeExpression.named(allocator, "cap");
        try vibez_type.addMethod(spill_method);
        
        try environment.addTypeDefinition(vibez_type);

        var scopes = ArrayList(HashMap([]const u8, VariableInfo, TypeEnvironment.StringContext, std.hash_map.default_max_load_percentage)).init(allocator);
        try scopes.append(HashMap([]const u8, VariableInfo, TypeEnvironment.StringContext, std.hash_map.default_max_load_percentage).init(allocator));

        return TypeChecker{
            .environment = environment,
            .scopes = scopes,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *TypeChecker) void {
        self.environment.deinit();
        for (self.scopes.items) |*scope| {
            var iter = scope.iterator();
            while (iter.next()) |entry| {
                var var_info = entry.value_ptr;
                var_info.deinit();
            }
            scope.deinit();
        }
        self.scopes.deinit();
    }

    pub fn enterScope(self: *TypeChecker) !void {
        try self.scopes.append(HashMap([]const u8, VariableInfo, TypeEnvironment.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator));
    }

    pub fn exitScope(self: *TypeChecker) void {
        if (self.scopes.items.len > 1) {
            var scope = self.scopes.pop();
            var iter = scope.iterator();
            while (iter.next()) |entry| {
                var var_info = entry.value_ptr;
                var_info.deinit();
            }
            scope.deinit();
        }
    }

    pub fn addVariable(self: *TypeChecker, name: []const u8, type_expr: TypeExpression, is_mutable: bool) !void {
        if (self.scopes.items.len == 0) return;
        
        var current_scope = &self.scopes.items[self.scopes.items.len - 1];
        const var_info = VariableInfo.init(self.allocator, name, type_expr, is_mutable);
        try current_scope.put(name, var_info);
    }

    pub fn getVariable(self: *const TypeChecker, name: []const u8) ?*const VariableInfo {
        // Search from most recent scope to oldest
        var i = self.scopes.items.len;
        while (i > 0) {
            i -= 1;
            const scope = &self.scopes.items[i];
            if (scope.getPtr(name)) |var_info| {
                return var_info;
            }
        }
        return null;
    }

    // Convert AST types to TypeExpression
    pub fn astTypeToTypeExpression(self: *TypeChecker, ast_type: *const ast.Type) !TypeExpression {
        return switch (ast_type.*) {
            .Lit => TypeExpression.named(self.allocator, "lit"),
            .Drip => TypeExpression.named(self.allocator, "drip"),
            .Normie => TypeExpression.named(self.allocator, "normie"),
            .Thicc => TypeExpression.named(self.allocator, "thicc"),
            .Smol => TypeExpression.named(self.allocator, "smol"),
            .Mid => TypeExpression.named(self.allocator, "mid"),
            .Tea => TypeExpression.named(self.allocator, "tea"),
            .Sip => TypeExpression.named(self.allocator, "sip"),
            .Snack => TypeExpression.named(self.allocator, "snack"),
            .Meal => TypeExpression.named(self.allocator, "meal"),
            .Byte => TypeExpression.named(self.allocator, "byte"),
            .Rune => TypeExpression.named(self.allocator, "rune"),
            .Cap => TypeExpression.named(self.allocator, "cap"),
            .Custom => |name| TypeExpression.named(self.allocator, name),
            .Array => |array_info| {
                const element_type = try self.astTypeToTypeExpression(array_info.element_type);
                return TypeExpression.array(self.allocator, element_type);
            },
            .Slice => |slice_info| {
                const element_type = try self.astTypeToTypeExpression(slice_info.element_type);
                return TypeExpression.array(self.allocator, element_type);
            },
            .Tuple => |tuple_info| {
                var element_types = ArrayList(TypeExpression).init(self.allocator);
                defer element_types.deinit();
                
                for (tuple_info.element_types.items) |*elem_type| {
                    try element_types.append(try self.astTypeToTypeExpression(elem_type));
                }
                
                return TypeExpression.tuple(self.allocator, element_types.items);
            },
            else => TypeExpression.named(self.allocator, "unknown"),
        };
    }

    // Main expression type checking
    pub fn checkExpression(self: *TypeChecker, expr: *const ast.Expression) !TypeExpression {
        return switch (expr.*) {
            .Integer => TypeExpression.named(self.allocator, "drip"),
            .String => TypeExpression.named(self.allocator, "tea"),
            .Boolean => TypeExpression.named(self.allocator, "lit"),
            .Identifier => |name| {
                if (self.getVariable(name)) |var_info| {
                    return var_info.type_expr;
                } else if (self.environment.hasType(name)) {
                    return TypeExpression.named(self.allocator, name);
                } else {
                    return error.UnknownIdentifier;
                }
            },
            .MemberAccess => |member_access| {
                return self.checkMemberAccess(member_access);
            },
            .Call => |call_expr| {
                return self.checkCall(call_expr);
            },
            .Binary => |binary| {
                return self.checkBinaryOperation(binary);
            },
            .Unary => |unary| {
                return self.checkUnaryOperation(unary);
            },
            .Tuple => |tuple_expr| {
                var element_types = ArrayList(TypeExpression).init(self.allocator);
                defer element_types.deinit();
                
                for (tuple_expr.elements.items) |*element| {
                    try element_types.append(try self.checkExpression(element));
                }
                
                return TypeExpression.tuple(self.allocator, element_types.items);
            },
            else => TypeExpression.named(self.allocator, "unknown"),
        };
    }

    fn checkMemberAccess(self: *TypeChecker, member_access: *const ast.MemberAccess) !TypeExpression {
        const object_type = try self.checkExpression(member_access.object);
        
        if (object_type.name) |type_name| {
            if (self.environment.getType(type_name)) |type_def| {
                if (type_def.getMethod(member_access.property)) |method| {
                    if (method.return_type) |ret_type| {
                        return ret_type;
                    } else {
                        return TypeExpression.named(self.allocator, "cap");
                    }
                }
            }
        }
        
        return error.UnknownProperty;
    }

    fn checkCall(self: *TypeChecker, call_expr: *const ast.CallExpression) !TypeExpression {
        // Check if this is a method call
        if (call_expr.function.* == .MemberAccess) {
            const member_access = &call_expr.function.MemberAccess;
            const object_type = try self.checkExpression(member_access.object);
            
            if (object_type.name) |type_name| {
                if (self.environment.getType(type_name)) |type_def| {
                    if (type_def.getMethod(member_access.property)) |method| {
                        // Check argument count
                        if (call_expr.arguments.items.len != method.parameters.items.len) {
                            return error.ArgumentCountMismatch;
                        }
                        
                        // Check argument types
                        for (call_expr.arguments.items, 0..) |*arg, i| {
                            const arg_type = try self.checkExpression(arg);
                            const expected_type = &method.parameters.items[i];
                            if (!self.typesCompatible(&arg_type, expected_type)) {
                                return error.TypeMismatch;
                            }
                        }
                        
                        if (method.return_type) |ret_type| {
                            return ret_type;
                        } else {
                            return TypeExpression.named(self.allocator, "cap");
                        }
                    }
                }
            }
        }
        
        return error.UnknownFunction;
    }

    fn checkBinaryOperation(self: *TypeChecker, binary: *const ast.BinaryExpression) !TypeExpression {
        const left_type = try self.checkExpression(binary.left);
        const right_type = try self.checkExpression(binary.right);
        
        switch (binary.operator) {
            .Add, .Subtract, .Multiply, .Divide => {
                if (left_type.isNumeric() and right_type.isNumeric()) {
                    return left_type; // Return left type for now
                } else {
                    return error.TypeMismatch;
                }
            },
            .Equal, .NotEqual, .Less, .Greater, .LessEqual, .GreaterEqual => {
                if (self.typesCompatible(&left_type, &right_type)) {
                    return TypeExpression.named(self.allocator, "lit");
                } else {
                    return error.TypeMismatch;
                }
            },
            .And, .Or => {
                if (left_type.isBoolean() and right_type.isBoolean()) {
                    return TypeExpression.named(self.allocator, "lit");
                } else {
                    return error.TypeMismatch;
                }
            },
        }
    }

    fn checkUnaryOperation(self: *TypeChecker, unary: *const ast.UnaryExpression) !TypeExpression {
        const operand_type = try self.checkExpression(unary.operand);
        
        switch (unary.operator) {
            .Not => {
                if (operand_type.isBoolean()) {
                    return TypeExpression.named(self.allocator, "lit");
                } else {
                    return error.TypeMismatch;
                }
            },
            .Minus, .Plus => {
                if (operand_type.isNumeric()) {
                    return operand_type;
                } else {
                    return error.TypeMismatch;
                }
            },
            .AddressOf => {
                return TypeExpression.pointer(self.allocator, operand_type);
            },
            .Dereference => {
                if (operand_type.kind == .Pointer and operand_type.parameters.items.len > 0) {
                    return operand_type.parameters.items[0];
                } else {
                    return error.TypeMismatch;
                }
            },
        }
    }

    // Check statement type correctness
    pub fn checkStatement(self: *TypeChecker, stmt: *const ast.Statement) !void {
        switch (stmt.*) {
            .Let => |var_decl| {
                const var_type: TypeExpression = if (var_decl.type_annotation) |type_annotation|
                    try self.astTypeToTypeExpression(type_annotation)
                else if (var_decl.initializer) |init_expr|
                    try self.checkExpression(init_expr)
                else
                    return error.VariableNeedsTypeOrInitializer;
                
                // If both type annotation and initializer exist, check compatibility
                if (var_decl.type_annotation != null and var_decl.initializer != null) {
                    const init_type = try self.checkExpression(var_decl.initializer.?);
                    if (!self.typesCompatible(&var_type, &init_type)) {
                        return error.TypeMismatch;
                    }
                }
                
                try self.addVariable(var_decl.name, var_type, var_decl.is_mutable);
            },
            .Expression => |expr| {
                _ = try self.checkExpression(expr);
            },
            // Note: Block statements handled at higher level
            else => {
                // TODO: Implement other statement types
            },
        }
    }

    // Type compatibility checking with CURSED type coercion rules
    pub fn typesCompatible(self: *const TypeChecker, t1: *const TypeExpression, t2: *const TypeExpression) bool {
        _ = self;
        
        if (t1.equals(t2)) return true;
        
        if (t1.name) |t1_name| {
            if (t2.name) |t2_name| {
                // CURSED type coercion rules
                return switch (std.hash_map.hashString(t1_name)) {
                    std.hash_map.hashString("drip") => std.mem.eql(u8, t2_name, "normie") or std.mem.eql(u8, t2_name, "thicc"),
                    std.hash_map.hashString("normie") => std.mem.eql(u8, t2_name, "drip") or std.mem.eql(u8, t2_name, "thicc"),
                    std.hash_map.hashString("thicc") => std.mem.eql(u8, t2_name, "drip") or std.mem.eql(u8, t2_name, "normie"),
                    std.hash_map.hashString("smol") => std.mem.eql(u8, t2_name, "mid") or std.mem.eql(u8, t2_name, "normie"),
                    std.hash_map.hashString("mid") => std.mem.eql(u8, t2_name, "smol") or std.mem.eql(u8, t2_name, "normie"),
                    std.hash_map.hashString("snack") => std.mem.eql(u8, t2_name, "meal"),
                    std.hash_map.hashString("meal") => std.mem.eql(u8, t2_name, "snack"),
                    else => false,
                };
            }
        }
        
        return false;
    }

    // Check function signatures
    pub fn checkFunctionSignature(self: *TypeChecker, func_decl: *const ast.FunctionDeclaration) !TypeExpression {
        try self.enterScope();
        defer self.exitScope();
        
        // Add parameters to scope
        for (func_decl.parameters.items) |*param| {
            if (param.type_annotation) |type_annotation| {
                const param_type = try self.astTypeToTypeExpression(type_annotation);
                try self.addVariable(param.name, param_type, false);
            }
        }
        
        // Check function body if present
        if (func_decl.body) |body| {
            try self.checkStatement(body);
        }
        
        // Return function type
        if (func_decl.return_type) |return_type| {
            return self.astTypeToTypeExpression(return_type);
        } else {
            return TypeExpression.named(self.allocator, "cap");
        }
    }
};

// Public interface functions
pub fn createTypeChecker(allocator: Allocator) !*TypeChecker {
    const checker = try allocator.create(TypeChecker);
    checker.* = try TypeChecker.init(allocator);
    return checker;
}

pub fn destroyTypeChecker(allocator: Allocator, checker: *TypeChecker) void {
    checker.deinit();
    allocator.destroy(checker);
}

// Type checking integration functions
pub fn checkProgram(checker: *TypeChecker, program: *const ast.Program) !void {
    for (program.statements.items) |stmt_ptr| {
        const statement: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
        try checker.checkStatement(statement);
    }
}

// Test helper functions
test "type checker initialization" {
    const allocator = std.testing.allocator;
    var checker = try TypeChecker.init(allocator);
    defer checker.deinit();
    
    // Test built-in types exist
    try std.testing.expect(checker.environment.hasType("lit"));
    try std.testing.expect(checker.environment.hasType("drip"));
    try std.testing.expect(checker.environment.hasType("tea"));
    try std.testing.expect(checker.environment.hasType("vibez"));
}

test "variable type checking" {
    const allocator = std.testing.allocator;
    var checker = try TypeChecker.init(allocator);
    defer checker.deinit();
    
    // Test variable addition and lookup
    const var_type = TypeExpression.named(allocator, "drip");
    try checker.addVariable("x", var_type, false);
    
    const retrieved = checker.getVariable("x");
    try std.testing.expect(retrieved != null);
    try std.testing.expect(std.mem.eql(u8, retrieved.?.name, "x"));
}

test "type compatibility" {
    const allocator = std.testing.allocator;
    var checker = try TypeChecker.init(allocator);
    defer checker.deinit();
    
    const drip_type = TypeExpression.named(allocator, "drip");
    const normie_type = TypeExpression.named(allocator, "normie");
    const tea_type = TypeExpression.named(allocator, "tea");
    
    // Test numeric type compatibility
    try std.testing.expect(checker.typesCompatible(&drip_type, &normie_type));
    try std.testing.expect(!checker.typesCompatible(&drip_type, &tea_type));
}
