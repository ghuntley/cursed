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

pub const StructFieldInfo = struct {
    name: []const u8,
    field_type: TypeExpression,
    offset: usize,
    is_required: bool,
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
            .parameters = .empty,
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
        // Make a copy of the element type to avoid double free
        const element_copy = TypeExpression.init(allocator, element_type.kind, element_type.name);
        try arr_type.parameters.append(element_copy);
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

    pub fn isInteger(self: *const TypeExpression) bool {
        if (self.name) |name| {
            return std.mem.eql(u8, name, "drip") or 
                   std.mem.eql(u8, name, "normie") or
                   std.mem.eql(u8, name, "thicc") or
                   std.mem.eql(u8, name, "smol") or
                   std.mem.eql(u8, name, "mid");
        }
        return false;
    }

    pub fn isIterable(self: *const TypeExpression) bool {
        if (self.name) |name| {
            return std.mem.eql(u8, name, "tea") or // strings are iterable
                   std.mem.startsWith(u8, name, "[]"); // arrays are iterable
        }
        return false;
    }

    pub fn getElementType(self: *const TypeExpression) ?*TypeExpression {
        if (self.name) |name| {
            if (std.mem.eql(u8, name, "tea")) {
                // String elements are characters
                return TypeExpression.named(std.heap.page_allocator, "sip");
            } else if (std.mem.startsWith(u8, name, "[]")) {
                // Extract array element type
                const element_type_name = name[2..];
                return TypeExpression.named(std.heap.page_allocator, element_type_name);
            }
        }
        return null;
    }

    pub fn isCompatibleWith(self: *const TypeExpression, other: *const TypeExpression) bool {
        return self.equals(other) or self.canCoerceTo(other);
    }

    pub fn canCoerceTo(self: *const TypeExpression, other: *const TypeExpression) bool {
        if (self.name) |self_name| {
            if (other.name) |other_name| {
                // CURSED type coercion rules
                if (std.mem.eql(u8, self_name, "normie") and std.mem.eql(u8, other_name, "drip")) return true;
                if (std.mem.eql(u8, self_name, "smol") and std.mem.eql(u8, other_name, "drip")) return true;
                if (std.mem.eql(u8, self_name, "drip") and std.mem.eql(u8, other_name, "thicc")) return true;
                if (std.mem.eql(u8, self_name, "snack") and std.mem.eql(u8, other_name, "meal")) return true;
                if (std.mem.eql(u8, self_name, "sip") and std.mem.eql(u8, other_name, "tea")) return true;
            }
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
            .parameters = ArrayList(TypeExpression){},
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

pub const StructFieldDefinition = struct {
    name: []const u8,
    field_type: TypeExpression,
    visibility: ast.Visibility,
    default_value: ?ast.Expression,
    is_mutable: bool,

    pub fn init(allocator: Allocator, name: []const u8, field_type: TypeExpression, visibility: ast.Visibility) StructFieldDefinition {
        return StructFieldDefinition{
            .name = allocator.dupe(u8, name) catch name,
            .field_type = field_type,
            .visibility = visibility,
            .default_value = null,
            .is_mutable = true,
        };
    }

    pub fn deinit(self: *StructFieldDefinition, allocator: Allocator) void {
        allocator.free(self.name);
        self.field_type.deinit();
    }
};

pub const TypeDefinition = struct {
    name: []const u8,
    kind: TypeKind,
    methods: ArrayList(MethodSignature),
    fields: ArrayList(StructFieldDefinition), // Added field storage
    is_builtin: bool,
    allocator: Allocator,

    pub fn init(allocator: Allocator, name: []const u8, kind: TypeKind) TypeDefinition {
        return TypeDefinition{
            .name = allocator.dupe(u8, name) catch name,
            .kind = kind,
            .methods = .empty,
            .fields = .empty,
            .is_builtin = false,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *TypeDefinition) void {
        self.allocator.free(self.name);
        for (self.methods.items) |*method| {
            method.deinit(self.allocator);
        }
        self.methods.deinit(self.allocator);
        for (self.fields.items) |*field| {
            field.deinit(self.allocator);
        }
        self.fields.deinit(self.allocator);
    }

    pub fn addMethod(self: *TypeDefinition, method: MethodSignature) !void {
        try self.methods.append(self.allocator, method);
    }

    pub fn getMethod(self: *const TypeDefinition, name: []const u8) ?*const MethodSignature {
        for (self.methods.items) |*method| {
            if (std.mem.eql(u8, method.name, name)) {
                return method;
            }
        }
        return null;
    }

    pub fn addField(self: *TypeDefinition, field: StructFieldDefinition) !void {
        try self.fields.append(self.allocator, field);
    }

    pub fn getField(self: *const TypeDefinition, name: []const u8) ?*const StructFieldDefinition {
        for (self.fields.items) |*field| {
            if (std.mem.eql(u8, field.name, name)) {
                return field;
            }
        }
        return null;
    }

    pub fn hasField(self: *const TypeDefinition, name: []const u8) bool {
        return self.getField(name) != null;
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
        UnknownStructType,
        IncompatibleFieldType,
        MissingStructField,
        UnknownStructField,
        VarianceViolation,
        CyclicTypeReference,
        InferenceConvergenceFailed,
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
    current_function_return_type: ?*TypeExpression = null,

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
        try spill_method.parameters.append(allocator, TypeExpression.named(allocator, "tea"));
        spill_method.return_type = TypeExpression.named(allocator, "cap");
        try vibez_type.addMethod(spill_method);
        
        try environment.addTypeDefinition(vibez_type);

        var scopes = ArrayList(HashMap([]const u8, VariableInfo, TypeEnvironment.StringContext, std.hash_map.default_max_load_percentage)){};
        try scopes.append(allocator, HashMap([]const u8, VariableInfo, TypeEnvironment.StringContext, std.hash_map.default_max_load_percentage).init(allocator));

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
        try self.scopes.append(self.allocator, HashMap([]const u8, VariableInfo, TypeEnvironment.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator));
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
            .Basic => |basic| switch (basic) {
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
                .Txt => TypeExpression.named(self.allocator, "txt"),
                .Byte => TypeExpression.named(self.allocator, "byte"),
                .Rune => TypeExpression.named(self.allocator, "rune"),
                .Extra => TypeExpression.named(self.allocator, "extra"),
                .Cap => TypeExpression.named(self.allocator, "cap"),
            },
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
                var element_types = .empty;
                defer element_types.deinit();
                
                for (tuple_info.elements.items) |*elem_type| {
                    try element_types.append(self.allocator, try self.astTypeToTypeExpression(elem_type));
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
            .Float => TypeExpression.named(self.allocator, "snack"),
            .String => TypeExpression.named(self.allocator, "tea"),
            .Boolean => TypeExpression.named(self.allocator, "lit"),
            .Character => TypeExpression.named(self.allocator, "sip"),
            .Identifier => |name| {
                if (self.getVariable(name)) |var_info| {
                    return var_info.type_expr;
                } else if (self.environment.hasType(name)) {
                    return TypeExpression.named(self.allocator, name);
                } else {
                    return error.UnknownIdentifier;
                }
            },
            .Variable => |name| {
                if (self.getVariable(name)) |var_info| {
                    return var_info.type_expr;
                } else {
                    return error.UnknownVariable;
                }
            },
            .MemberAccess => |member_access| {
                return self.checkMemberAccess(member_access);
            },
            .Call => |call_expr| {
                return self.checkCall(&call_expr);
            },
            .Binary => |binary| {
                return self.checkBinaryOperation(&binary);
            },
            .Unary => |unary| {
                return self.checkUnaryOperation(unary);
            },
            .Array => |array_expr| {
                return self.checkArrayExpression(array_expr);
            },
            .Tuple => |tuple_expr| {
                return self.checkTupleExpression(&tuple_expr);
            },
            .StructLiteral => |struct_literal| {
                return self.checkStructLiteral(&struct_literal);
            },
            .ArrayAccess => |array_access| {
                return self.checkArrayAccess(&array_access);
            },
            .MethodCall => |method_call| {
                return self.checkMethodCall(method_call);
            },
            .TypeAssertion => |type_assertion| {
                return self.checkTypeAssertion(&type_assertion);
            },
            .Lambda => |lambda| {
                return self.checkLambdaExpression(&lambda);
            },
            else => TypeExpression.named(self.allocator, "unknown"),
        };
    }

    fn checkMemberAccess(self: *TypeChecker, member_access: *const ast.MemberAccessExpression) anyerror!TypeExpression {
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

    fn checkCall(self: *TypeChecker, call_expr: anytype) !TypeExpression {
        // Check if this is a method call
        if (call_expr.function.* == .MemberAccess) {
            const member_access = call_expr.function.MemberAccess;
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
                            const arg_type = try self.checkExpression(arg.*);
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
        
        if (std.mem.eql(u8, binary.operator, "+") or 
            std.mem.eql(u8, binary.operator, "-") or 
            std.mem.eql(u8, binary.operator, "*") or 
            std.mem.eql(u8, binary.operator, "/")) {
            if (left_type.isNumeric() and right_type.isNumeric()) {
                return left_type; // Return left type for now
            } else {
                return error.TypeMismatch;
            }
        } else if (std.mem.eql(u8, binary.operator, "==") or 
                   std.mem.eql(u8, binary.operator, "!=") or 
                   std.mem.eql(u8, binary.operator, "<") or 
                   std.mem.eql(u8, binary.operator, "<=") or 
                   std.mem.eql(u8, binary.operator, ">") or 
                   std.mem.eql(u8, binary.operator, ">=")) {
            if (self.typesCompatible(&left_type, &right_type)) {
                return TypeExpression.named(self.allocator, "lit");
            } else {
                return error.TypeMismatch;
            }
        } else if (std.mem.eql(u8, binary.operator, "&&") or 
                   std.mem.eql(u8, binary.operator, "||")) {
            if (left_type.isBoolean() and right_type.isBoolean()) {
                return TypeExpression.named(self.allocator, "lit");
            } else {
                return error.TypeMismatch;
            }
        } else {
            return error.UnsupportedOperation;
        }
    }

    fn checkUnaryOperation(self: *TypeChecker, unary: *const ast.UnaryExpression) !TypeExpression {
        const operand_type = try self.checkExpression(unary.operand);
        
        if (std.mem.eql(u8, unary.operator, "!")) {
            if (operand_type.isBoolean()) {
                return TypeExpression.named(self.allocator, "lit");
            } else {
                return error.TypeMismatch;
            }
        } else if (std.mem.eql(u8, unary.operator, "-") or std.mem.eql(u8, unary.operator, "+")) {
            if (operand_type.isNumeric()) {
                return operand_type;
            } else {
                return error.TypeMismatch;
            }
        } else if (std.mem.eql(u8, unary.operator, "&")) {
            return TypeExpression.pointer(self.allocator, operand_type);
        } else if (std.mem.eql(u8, unary.operator, "*")) {
            if (operand_type.kind == .Pointer and operand_type.parameters.items.len > 0) {
                return operand_type.parameters.items[0];
            } else {
                return error.TypeMismatch;
            }
        } else {
            return error.UnsupportedOperation;
        }
    }

    // Additional expression checking helpers
    fn checkArrayExpression(self: *TypeChecker, array_expr: *const ast.ArrayExpression) !TypeExpression {
        if (array_expr.elements.items.len == 0) {
            return TypeExpression.array(self.allocator, TypeExpression.named(self.allocator, "unknown"));
        }
        
        const first_type = try self.checkExpression(array_expr.elements.items[0]);
        
        // Check all elements have compatible types
        for (array_expr.elements.items[1..]) |element| {
            const element_type = try self.checkExpression(element);
            if (!self.typesCompatible(&first_type, &element_type)) {
                return error.ArrayElementTypeMismatch;
            }
        }
        
        return TypeExpression.array(self.allocator, first_type);
    }

    fn checkTupleExpression(self: *TypeChecker, tuple_expr: *const ast.TupleExpression) !TypeExpression {
        var element_types = .empty;
        defer element_types.deinit();
        
        for (tuple_expr.elements.items) |element| {
            try element_types.append(self.allocator, try self.checkExpression(element));
        }
        
        return TypeExpression.tuple(self.allocator, element_types.items);
    }

    fn checkStructLiteral(self: *TypeChecker, struct_literal: *const ast.StructLiteralExpression) !TypeExpression {
        // Check if the struct type exists
        if (!self.environment.hasType(struct_literal.type_name)) {
            return error.UnknownStructType;
        }
        
        // Enhanced struct field validation with fail-fast errors
        const struct_def = self.environment.getStructDefinition(struct_literal.type_name) orelse {
            return self.createTypeError(.UnknownStructType, "Struct type '{s}' not found", .{struct_literal.type_name});
        };
        
        // Validate all required fields are present
        for (struct_def.fields) |field| {
            var field_found = false;
            for (struct_literal.fields) |literal_field| {
                if (std.mem.eql(u8, field.name, literal_field.name)) {
                    // Check field type compatibility
                    const field_type = try self.checkExpression(literal_field.value);
                    const expected_type = try self.resolveTypeExpression(field.field_type);
                    
                    if (!self.typesAreCompatible(field_type, expected_type)) {
                        return self.createTypeError(.IncompatibleFieldType, 
                            "Field '{s}' expects type '{s}' but got '{s}'", 
                            .{ field.name, expected_type.toString(), field_type.toString() });
                    }
                    field_found = true;
                    break;
                }
            }
            
            if (!field_found) {
                return self.createTypeError(.MissingStructField, 
                    "Required field '{s}' missing in struct literal", .{field.name});
            }
        }
        
        // Check for unknown fields
        for (struct_literal.fields) |literal_field| {
            var field_exists = false;
            for (struct_def.fields) |field| {
                if (std.mem.eql(u8, field.name, literal_field.name)) {
                    field_exists = true;
                    break;
                }
            }
            
            if (!field_exists) {
                return self.createTypeError(.UnknownStructField, 
                    "Unknown field '{s}' in struct '{s}'", .{ literal_field.name, struct_literal.type_name });
            }
        }
        
        return TypeExpression.named(self.allocator, struct_literal.type_name);
    }

    fn checkArrayAccess(self: *TypeChecker, array_access: *const ast.ArrayAccessExpression) !TypeExpression {
        const array_type = try self.checkExpression(array_access.array);
        const index_type = try self.checkExpression(array_access.index);
        
        // Index must be numeric
        if (!index_type.isNumeric()) {
            return error.NonNumericArrayIndex;
        }
        
        // Array type must be an array
        if (array_type.kind != .Array or array_type.parameters.items.len == 0) {
            return error.IndexingNonArray;
        }
        
        return array_type.parameters.items[0];
    }

    fn checkMethodCall(self: *TypeChecker, method_call: *const ast.MethodCallExpression) !TypeExpression {
        const receiver_type = try self.checkExpression(method_call.receiver);
        
        if (receiver_type.name) |type_name| {
            if (self.environment.getType(type_name)) |type_def| {
                if (type_def.getMethod(method_call.method_name)) |method| {
                    // Check argument types
                    if (method_call.arguments.items.len != method.parameters.items.len) {
                        return error.ArgumentCountMismatch;
                    }
                    
                    for (method_call.arguments.items, 0..) |*arg, i| {
                        const arg_type = try self.checkExpression(arg.*);
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
        
        return error.UnknownMethod;
    }

    fn checkTypeAssertion(self: *TypeChecker, type_assertion: *const ast.TypeAssertionExpression) !TypeExpression {
        const expr_type = try self.checkExpression(type_assertion.expression);
        const target_type = try self.astTypeToTypeExpression(&type_assertion.target_type);
        
        // Check if the type assertion is valid
        if (!self.typesCompatible(&expr_type, &target_type)) {
            return error.InvalidTypeAssertion;
        }
        
        return target_type;
    }

    fn checkLambdaExpression(self: *TypeChecker, lambda: *const ast.LambdaExpression) !TypeExpression {
        // Create function scope
        try self.enterScope();
        defer self.exitScope();
        
        var param_types = .empty;
        defer param_types.deinit();
        
        // Add parameters to scope
        for (lambda.parameters.items) |*param| {
            const param_type = if (param.type_annotation) |type_annotation|
                try self.astTypeToTypeExpression(type_annotation)
            else
                TypeExpression.named(self.allocator, "unknown");
            
            try param_types.append(self.allocator, param_type);
            try self.addVariable(param.name, param_type, false);
        }
        
        // Check body expression
        const return_type = try self.checkExpression(lambda.body);
        
        // Create function type
        var func_type = TypeExpression.init(self.allocator, .Function, "function");
        try func_type.parameters.appendSlice(param_types.items);
        func_type.return_type = try self.allocator.create(TypeExpression);
        func_type.return_type.?.* = return_type;
        
        return func_type;
    }

    // Check statement type correctness
    pub fn checkStatement(self: *TypeChecker, stmt: *const ast.Statement) !void {
        switch (stmt.*) {
            .Let => |var_decl| {
                const var_type: TypeExpression = if (var_decl.type_annotation) |type_annotation|
                    try self.astTypeToTypeExpression(&type_annotation)
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
                _ = try self.checkExpression(@ptrCast(@alignCast(expr)));
            },
            .Function => |func_decl| {
                _ = try self.checkFunctionDeclaration(func_decl);
            },
            .Struct => |struct_decl| {
                _ = try self.checkStructDeclaration(struct_decl);
            },
            .Interface => |interface_decl| {
                _ = try self.checkInterfaceDeclaration(interface_decl);
            },
            .If => |if_stmt| {
                try self.checkIfStatement(if_stmt);
            },
            .While => |while_stmt| {
                try self.checkWhileStatement(while_stmt);
            },
            .Return => |return_stmt| {
                try self.checkReturnStatement(return_stmt);
            },
            .Assignment => |assignment| {
                try self.checkAssignment(assignment);
            },
            .For => |for_stmt| {
                try self.checkForStatement(for_stmt);
            },
            .ForIn => |forin_stmt| {
                try self.checkForInStatement(forin_stmt);
            },
            .Switch => |switch_stmt| {
                try self.checkSwitchStatement(switch_stmt);
            },
            .PatternSwitch => |pattern_switch| {
                try self.checkPatternSwitchStatement(pattern_switch);
            },
            .Goroutine => |goroutine| {
                try self.checkGoroutineStatement(goroutine);
            },
            .Stan => |stan| {
                try self.checkStanStatement(stan);
            },
            .Channel => |channel| {
                try self.checkChannelStatement(channel);
            },
            .Select => |select_stmt| {
                try self.checkSelectStatement(select_stmt);
            },
            .Struct => |struct_stmt| {
                try self.checkStructStatement(struct_stmt);
            },
            .Interface => |interface_stmt| {
                try self.checkInterfaceStatement(interface_stmt);
            },
            .Implementation => |impl_stmt| {
                try self.checkImplementationStatement(impl_stmt);
            },
            .TypeAlias => |alias| {
                try self.checkTypeAliasStatement(alias);
            },
            .Panic => |panic| {
                try self.checkPanicStatement(panic);
            },
            .Catch => |catch_stmt| {
                try self.checkCatchStatement(catch_stmt);
            },
            .Defer => |defer_stmt| {
                try self.checkDeferStatement(defer_stmt);
            },
            .Break => |break_stmt| {
                try self.checkBreakStatement(break_stmt);
            },
            .Continue => |continue_stmt| {
                try self.checkContinueStatement(continue_stmt);
            },
            // Note: Block statements handled at higher level
            else => {
                // Handle unknown statement types gracefully
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

    // Statement checking methods
    fn checkFunctionDeclaration(self: *TypeChecker, func_decl: *const ast.FunctionDeclaration) !TypeExpression {
        try self.enterScope();
        defer self.exitScope();
        
        var param_types = .empty;
        defer param_types.deinit();
        
        // Add parameters to scope and collect their types
        for (func_decl.parameters.items) |*param| {
            const param_type = if (param.type_annotation) |type_annotation|
                try self.astTypeToTypeExpression(type_annotation)
            else
                TypeExpression.named(self.allocator, "unknown");
            
            try param_types.append(self.allocator, param_type);
            try self.addVariable(param.name, param_type, false);
        }
        
        // Determine return type
        const return_type = if (func_decl.return_type) |return_type_annotation|
            try self.astTypeToTypeExpression(return_type_annotation)
        else
            TypeExpression.named(self.allocator, "cap");
        
        // Check function body if present
        if (func_decl.body) |body| {
            for (body.items) |statement| {
                try self.checkStatement(statement);
            }
        }
        
        // Create function type
        var func_type = TypeExpression.init(self.allocator, .Function, func_decl.name);
        try func_type.parameters.appendSlice(param_types.items);
        func_type.return_type = try self.allocator.create(TypeExpression);
        func_type.return_type.?.* = return_type;
        
        // Add function to current scope
        try self.addVariable(func_decl.name, func_type, false);
        
        return func_type;
    }

    fn checkStructDeclaration(self: *TypeChecker, struct_decl: *const ast.StructDeclaration) !TypeExpression {
        // Create struct field validation table
        var field_map = std.HashMap([]const u8, TypeExpression, std.hash_map.StringContext, 80).init(self.allocator);
        defer field_map.deinit(self.allocator);
        
        // Validate field types and check for duplicates
        for (struct_decl.fields.items) |*field| {
            // Check for duplicate field names
            if (field_map.contains(field.name)) {
                return error.DuplicateFieldName;
            }
            
            if (field.field_type) |field_type| {
                const field_type_expr = try self.astTypeToTypeExpression(field_type);
                
                // Validate field type is resolvable and not self-referential at depth 0
                try self.validateFieldType(field_type_expr, struct_decl.name, 0);
                
                // Store validated field type
                try field_map.put(field.name, field_type_expr);
            } else {
                return error.MissingFieldType;
            }
        }
        
        // Create struct type definition with validated fields
        var struct_type_def = TypeDefinition.init(self.allocator, struct_decl.name, .Struct);
        
        // Store field information in struct definition  
        var fields_iter = field_map.iterator();
        while (fields_iter.next()) |entry| {
            const field_def = StructFieldDefinition{
                .name = try self.allocator.dupe(u8, entry.key_ptr.*),
                .field_type = entry.value_ptr.*,
                .is_required = true,
            };
            try struct_type_def.addField(field_def);
        }
        
        // Add struct type to environment
        try self.environment.addTypeDefinition(struct_type_def);
        
        return TypeExpression.named(self.allocator, struct_decl.name);
    }

    /// Validate that a field type is valid and prevent infinite recursion
    fn validateFieldType(self: *TypeChecker, field_type: TypeExpression, struct_name: []const u8, depth: u8) !void {
        if (depth > 10) {
            return error.RecursiveTypeDefinition;
        }
        
        switch (field_type.kind) {
            .Named => {
                if (field_type.name) |type_name| {
                    // Check for direct self-reference at depth 0
                    if (depth == 0 and std.mem.eql(u8, type_name, struct_name)) {
                        return error.SelfReferencingField;
                    }
                    
                    // Validate that the named type exists
                    if (self.environment.getType(type_name) == null) {
                        // Allow forward references by checking if it's a known primitive
                        if (!self.isPrimitiveType(type_name)) {
                            return error.UndefinedType;
                        }
                    }
                }
            },
            .Array => {
                // Validate array element type
                if (field_type.parameters.items.len > 0) {
                    try self.validateFieldType(field_type.parameters.items[0], struct_name, depth + 1);
                }
            },
            .Pointer => {
                // Allow pointer to self-type (breaks recursion)
                if (field_type.parameters.items.len > 0) {
                    try self.validateFieldType(field_type.parameters.items[0], struct_name, depth + 1);
                }
            },
            .Struct => {
                // Nested struct validation
                if (field_type.name) |type_name| {
                    if (std.mem.eql(u8, type_name, struct_name)) {
                        return error.RecursiveStructNesting;
                    }
                }
            },
            .Primitive => {
                // Primitives are always valid
            },
            else => {
                // Other types require specific validation
                return error.UnsupportedFieldType;
            }
        }
    }

    /// Check if a type name represents a primitive type
    fn isPrimitiveType(self: *TypeChecker, type_name: []const u8) bool {
        _ = self;
        const primitives = [_][]const u8{ "drip", "tea", "lit", "sus", "i32", "f64", "bool", "string" };
        for (primitives) |primitive| {
            if (std.mem.eql(u8, type_name, primitive)) {
                return true;
            }
        }
        return false;
    }

    fn checkInterfaceDeclaration(self: *TypeChecker, interface_decl: *const ast.InterfaceDeclaration) !TypeExpression {
        // Validate method signatures
        for (interface_decl.methods.items) |*method| {
            // Check parameter types
            for (method.parameters.items) |*param| {
                if (param.type_annotation) |param_type| {
                    _ = try self.astTypeToTypeExpression(param_type);
                }
            }
            
            // Check return type
            if (method.return_type) |return_type| {
                _ = try self.astTypeToTypeExpression(return_type);
            }
        }
        
        // Create interface type definition
        const interface_type_def = TypeDefinition.init(self.allocator, interface_decl.name, .Interface);
        
        // Add method signatures to interface definition
        for (interface_decl.methods.items) |method| {
            var param_types = std.ArrayList(*TypeExpression){};
            for (method.parameters.items) |param| {
                const param_type = try self.resolveType(param.type_annotation);
                try param_types.append(self.allocator, param_type);
            }
            
            const return_type = if (method.return_type) |ret_type|
                try self.resolveType(ret_type)
            else
                TypeExpression.named(self.allocator, "cap");
                
            // Store method signature information
            param_types.deinit();
            return_type.deinit();
        }
        
        // Add interface type to environment
        try self.environment.addTypeDefinition(interface_type_def);
        
        return TypeExpression.named(self.allocator, interface_decl.name);
    }

    fn checkIfStatement(self: *TypeChecker, if_stmt: *const ast.IfStatement) !void {
        // Check condition
        const condition_type = try self.checkExpression(if_stmt.condition);
        if (!condition_type.isBoolean()) {
            return error.NonBooleanCondition;
        }
        
        // Check then branch
        try self.enterScope();
        defer self.exitScope();
        for (if_stmt.then_branch.items) |statement| {
            try self.checkStatement(statement);
        }
        
        // Check else branch if present
        if (if_stmt.else_branch) |else_branch| {
            try self.enterScope();
            defer self.exitScope();
            for (else_branch.items) |statement| {
                try self.checkStatement(statement);
            }
        }
    }

    fn checkWhileStatement(self: *TypeChecker, while_stmt: *const ast.WhileStatement) !void {
        // Check condition
        const condition_type = try self.checkExpression(while_stmt.condition);
        if (!condition_type.isBoolean()) {
            return error.NonBooleanCondition;
        }
        
        // Check body
        try self.enterScope();
        defer self.exitScope();
        for (while_stmt.body.items) |statement| {
            try self.checkStatement(statement);
        }
    }

    fn checkReturnStatement(self: *TypeChecker, return_stmt: *const ast.ReturnStatement) !void {
        const return_type = if (return_stmt.value) |value|
            try self.checkExpression(value)
        else
            TypeExpression.named(self.allocator, "cap");
        
        // Check against current function's return type if we have context
        if (self.current_function_return_type) |expected_type| {
            if (!self.typesCompatible(&return_type, expected_type)) {
                return error.ReturnTypeMismatch;
            }
        }
    }

    // Missing statement type checking methods
    fn checkForStatement(self: *TypeChecker, for_stmt: *const ast.ForStatement) !void {
        // Check initializer
        if (for_stmt.init) |init_stmt| {
            try self.checkStatement(init_stmt);
        }
        
        // Check condition
        if (for_stmt.condition) |condition| {
            const condition_type = try self.checkExpression(condition);
            if (!condition_type.isBoolean()) {
                return error.NonBooleanCondition;
            }
        }
        
        // Check increment
        if (for_stmt.increment) |increment| {
            _ = try self.checkExpression(increment);
        }
        
        // Check body
        try self.enterScope();
        defer self.exitScope();
        for (for_stmt.body.items) |statement| {
            try self.checkStatement(statement);
        }
    }

    fn checkForInStatement(self: *TypeChecker, forin_stmt: *const ast.ForInStatement) !void {
        // Check iterable expression
        const iterable_type = try self.checkExpression(forin_stmt.iterable);
        
        // Verify it's actually iterable (array, string, etc.)
        if (!iterable_type.isIterable()) {
            return error.NotIterable;
        }
        
        // Check body with iterator variable in scope
        try self.enterScope();
        defer self.exitScope();
        
        // Add iterator variable to scope
        const element_type = iterable_type.getElementType() orelse TypeExpression.named(self.allocator, "cap");
        const var_info = VariableInfo.init(self.allocator, forin_stmt.variable, element_type, false);
        try self.environment.addVariable(var_info);
        
        for (forin_stmt.body.items) |statement| {
            try self.checkStatement(statement);
        }
    }

    fn checkSwitchStatement(self: *TypeChecker, switch_stmt: *const ast.SwitchStatement) !void {
        const discriminant_type = try self.checkExpression(switch_stmt.discriminant);
        
        for (switch_stmt.cases.items) |case| {
            for (case.values.items) |value| {
                const case_type = try self.checkExpression(value);
                if (!self.typesCompatible(&discriminant_type, &case_type)) {
                    return error.SwitchCaseTypeMismatch;
                }
            }
            
            try self.enterScope();
            defer self.exitScope();
            for (case.body.items) |statement| {
                try self.checkStatement(statement);
            }
        }
        
        if (switch_stmt.default_case) |default| {
            try self.enterScope();
            defer self.exitScope();
            for (default.items) |statement| {
                try self.checkStatement(statement);
            }
        }
    }

    fn checkPatternSwitchStatement(self: *TypeChecker, pattern_switch: *const ast.PatternSwitchStatement) !void {
        const discriminant_type = try self.checkExpression(pattern_switch.discriminant);
        
        for (pattern_switch.cases.items) |case| {
            // Check pattern compatibility
            try self.enterScope();
            defer self.exitScope();
            
            // Add pattern variables to scope
            // TODO: Extract variables from pattern and add to scope
            
            for (case.body.items) |statement| {
                try self.checkStatement(statement);
            }
        }
        
        _ = discriminant_type;
    }

    fn checkGoroutineStatement(self: *TypeChecker, goroutine: *const ast.GoroutineStatement) !void {
        // Check goroutine body
        try self.enterScope();
        defer self.exitScope();
        for (goroutine.body.items) |statement| {
            try self.checkStatement(statement);
        }
    }

    fn checkStanStatement(self: *TypeChecker, stan: *const ast.StanStatement) !void {
        // Stan statements are compile-time assertions
        const condition_type = try self.checkExpression(stan.condition);
        if (!condition_type.isBoolean()) {
            return error.NonBooleanCondition;
        }
    }

    fn checkChannelStatement(self: *TypeChecker, channel: *const ast.ChannelStatement) !void {
        // Check channel type if specified
        if (channel.channel_type) |ch_type| {
            _ = try self.resolveType(ch_type);
        }
        
        // Check buffer size if specified
        if (channel.buffer_size) |size| {
            const size_type = try self.checkExpression(size);
            if (!size_type.isInteger()) {
                return error.InvalidBufferSize;
            }
        }
    }

    fn checkSelectStatement(self: *TypeChecker, select_stmt: *const ast.SelectStatement) !void {
        for (select_stmt.cases.items) |case| {
            try self.enterScope();
            defer self.exitScope();
            
            // Check channel operation
            if (case.channel_op) |op| {
                _ = try self.checkExpression(op);
            }
            
            for (case.body.items) |statement| {
                try self.checkStatement(statement);
            }
        }
        
        if (select_stmt.default_case) |default| {
            try self.enterScope();
            defer self.exitScope();
            for (default.items) |statement| {
                try self.checkStatement(statement);
            }
        }
    }

    fn checkStructStatement(self: *TypeChecker, struct_stmt: *const ast.StructStatement) !void {
        // Check field types
        for (struct_stmt.fields.items) |field| {
            _ = try self.resolveType(field.type_annotation);
        }
        
        // Register struct type
        const struct_type_def = TypeDefinition.init(self.allocator, struct_stmt.name, .Struct);
        try self.environment.addTypeDefinition(struct_type_def);
    }

    fn checkInterfaceStatement(self: *TypeChecker, interface_stmt: *const ast.InterfaceStatement) !void {
        // Check method signatures
        for (interface_stmt.methods.items) |method| {
            for (method.parameters.items) |param| {
                _ = try self.resolveType(param.type_annotation);
            }
            
            if (method.return_type) |ret_type| {
                _ = try self.resolveType(ret_type);
            }
        }
        
        // Register interface type
        const interface_type_def = TypeDefinition.init(self.allocator, interface_stmt.name, .Interface);
        try self.environment.addTypeDefinition(interface_type_def);
    }

    fn checkImplementationStatement(self: *TypeChecker, impl_stmt: *const ast.ImplementationStatement) !void {
        // Verify interface exists
        const interface_type = try self.environment.getTypeDefinition(impl_stmt.interface_name) orelse {
            return error.UndefinedInterface;
        };
        
        // Verify struct/type exists
        const impl_type = try self.environment.getTypeDefinition(impl_stmt.type_name) orelse {
            return error.UndefinedType;
        };
        
        // Check method implementations
        for (impl_stmt.methods.items) |method| {
            try self.checkFunctionDeclaration(method);
        }
        
        _ = interface_type;
        _ = impl_type;
    }

    fn checkTypeAliasStatement(self: *TypeChecker, alias: *const ast.TypeAliasStatement) !void {
        _ = try self.resolveType(alias.target_type);
        
        // Register type alias
        const alias_type_def = TypeDefinition.init(self.allocator, alias.name, .Alias);
        try self.environment.addTypeDefinition(alias_type_def);
    }

    fn checkPanicStatement(self: *TypeChecker, panic: *const ast.PanicStatement) !void {
        // Check panic message
        const message_type = try self.checkExpression(panic.message);
        if (!message_type.isString()) {
            return error.InvalidPanicMessage;
        }
    }

    fn checkCatchStatement(self: *TypeChecker, catch_stmt: *const ast.CatchStatement) !void {
        // Check body
        try self.enterScope();
        defer self.exitScope();
        
        // Add error variable to scope if specified
        if (catch_stmt.error_var) |error_var| {
            const error_type = TypeExpression.named(self.allocator, "CursedError");
            const var_info = VariableInfo.init(self.allocator, error_var, error_type, false);
            try self.environment.addVariable(var_info);
        }
        
        for (catch_stmt.body.items) |statement| {
            try self.checkStatement(statement);
        }
    }

    fn checkDeferStatement(self: *TypeChecker, defer_stmt: *const ast.DeferStatement) !void {
        // Check deferred expression
        _ = try self.checkExpression(defer_stmt.expression);
    }

    fn checkBreakStatement(self: *TypeChecker, break_stmt: *const ast.BreakStatement) !void {
        _ = self;
        _ = break_stmt; // Break statements don't need type checking
    }

    fn checkContinueStatement(self: *TypeChecker, continue_stmt: *const ast.ContinueStatement) !void {
        _ = self;
        _ = continue_stmt; // Continue statements don't need type checking
    }

    /// Create a typed error with formatted message
    fn createTypeError(self: *TypeChecker, kind: TypeCheckError.TypeErrorKind, comptime fmt: []const u8, args: anytype) !TypeExpression {
        const message = try std.fmt.allocPrint(self.allocator, fmt, args);
        _ = TypeCheckError.init(self.allocator, kind, message);
        self.allocator.free(message);
        return error.TypeCheckingFailed;
    }
    
    /// Check type compatibility for struct fields
    fn typesAreCompatible(self: *TypeChecker, type1: TypeExpression, type2: TypeExpression) bool {
        _ = self;
        // Enhanced type compatibility checking
        return type1.isCompatibleWith(type2);
    }
    
    /// Resolve type expression to concrete type
    fn resolveTypeExpression(self: *TypeChecker, type_expr: ast.Type) !TypeExpression {
        switch (type_expr) {
            .Basic => |basic| {
                switch (basic) {
                    .Drip => return TypeExpression.primitive(.Integer),
                    .Tea => return TypeExpression.primitive(.String),
                    .Lit => return TypeExpression.primitive(.Boolean),
                    else => return TypeExpression.primitive(.Void),
                }
            },
            .Primitive => |prim| return TypeExpression.primitive(prim),
            .Custom => |name| return TypeExpression.named(self.allocator, name),
            else => return TypeExpression.primitive(.Void),
        }
    }

    fn checkAssignment(self: *TypeChecker, assignment: *const ast.AssignmentStatement) !void {
        const lhs_type = try self.checkExpression(assignment.target);
        const rhs_type = try self.checkExpression(assignment.value);
        
        if (!self.typesCompatible(&lhs_type, &rhs_type)) {
            return error.AssignmentTypeMismatch;
        }
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
            for (body.items) |statement| {
                try self.checkStatement(statement);
            }
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
    var var_type = TypeExpression.named(allocator, "drip");
    defer var_type.deinit();
    try checker.addVariable("x", var_type, false);
    
    const retrieved = checker.getVariable("x");
    try std.testing.expect(retrieved != null);
    try std.testing.expect(std.mem.eql(u8, retrieved.?.name, "x"));
}

test "type compatibility" {
    const allocator = std.testing.allocator;
    var checker = try TypeChecker.init(allocator);
    defer checker.deinit();
    
    var drip_type = TypeExpression.named(allocator, "drip");
    defer drip_type.deinit();
    var normie_type = TypeExpression.named(allocator, "normie");
    defer normie_type.deinit();
    var tea_type = TypeExpression.named(allocator, "tea");
    defer tea_type.deinit();
    
    // Test numeric type compatibility
    try std.testing.expect(checker.typesCompatible(&drip_type, &normie_type));
    try std.testing.expect(!checker.typesCompatible(&drip_type, &tea_type));
}
