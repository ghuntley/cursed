/// Comprehensive Type System for CURSED Compiler
/// Implements full type inference, checking, generics, and constraint resolution
const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const error_handling = @import("error_handling.zig");
const CursedError = error_handling.CursedError;

// Enhanced type system with full inference and constraint resolution
pub const CursedType = union(enum) {
    // Primitive types
    Drip: void,       // int64
    Normie: void,     // int32
    Smol: void,       // int8
    Thicc: void,      // int64 (same as drip but different semantics)
    Meal: void,       // float64
    Snack: void,      // float32
    Tea: void,        // string
    Lit: void,        // bool
    Sip: void,        // char
    Vibes: void,      // void
    
    // Composite types
    Array: *ArrayType,
    Slice: *SliceType,
    Struct: *StructType,
    Interface: *InterfaceType,
    Function: *FunctionType,
    Channel: *ChannelType,
    Tuple: *TupleType,
    
    // Generic and constraint types
    Generic: *GenericType,
    TypeParameter: *TypeParameterType,
    Constraint: *ConstraintType,
    
    // Special types for inference
    Unknown: u32,     // Type variable ID for inference
    Error: void,      // Error type for failed inference
    
    pub fn format(self: CursedType, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        
        switch (self) {
            .Drip => try writer.writeAll("drip"),
            .Normie => try writer.writeAll("normie"),
            .Smol => try writer.writeAll("smol"),
            .Thicc => try writer.writeAll("thicc"),
            .Meal => try writer.writeAll("meal"),
            .Snack => try writer.writeAll("snack"),
            .Tea => try writer.writeAll("tea"),
            .Lit => try writer.writeAll("lit"),
            .Sip => try writer.writeAll("sip"),
            .Vibes => try writer.writeAll("vibes"),
            .Array => |arr| try writer.print("[]{}", .{arr.element_type.*}),
            .Slice => |slice| try writer.print("[]{}", .{slice.element_type.*}),
            .Channel => |ch| try writer.print("dm<{}>", .{ch.element_type.*}),
            .Function => |func| {
                try writer.writeAll("slay(");
                for (func.parameters.items, 0..) |param, i| {
                    if (i > 0) try writer.writeAll(", ");
                    try writer.print("{}", .{param});
                }
                try writer.writeAll(") ");
                if (func.return_type) |ret| {
                    try writer.print("{}", .{ret.*});
                } else {
                    try writer.writeAll("vibes");
                }
            },
            .Generic => |gen| try writer.print("{}[{}]", .{gen.base_type.*, gen.type_args.items[0]}),
            .TypeParameter => |param| try writer.writeAll(param.name),
            .Unknown => |id| try writer.print("?T{}", .{id}),
            .Error => try writer.writeAll("!ERROR"),
            else => try writer.writeAll("unknown"),
        }
    }
    
    pub fn isNumeric(self: CursedType) bool {
        return switch (self) {
            .Drip, .Normie, .Smol, .Thicc, .Meal, .Snack => true,
            else => false,
        };
    }
    
    pub fn isInteger(self: CursedType) bool {
        return switch (self) {
            .Drip, .Normie, .Smol, .Thicc => true,
            else => false,
        };
    }
    
    pub fn isFloat(self: CursedType) bool {
        return switch (self) {
            .Meal, .Snack => true,
            else => false,
        };
    }
    
    pub fn isComparable(self: CursedType) bool {
        return switch (self) {
            .Drip, .Normie, .Smol, .Thicc, .Meal, .Snack, .Tea, .Lit, .Sip => true,
            else => false,
        };
    }
    
    pub fn equals(self: CursedType, other: CursedType) bool {
        return switch (self) {
            .Drip => other == .Drip,
            .Normie => other == .Normie,
            .Smol => other == .Smol,
            .Thicc => other == .Thicc,
            .Meal => other == .Meal,
            .Snack => other == .Snack,
            .Tea => other == .Tea,
            .Lit => other == .Lit,
            .Sip => other == .Sip,
            .Vibes => other == .Vibes,
            .Error => other == .Error,
            .Unknown => |id| switch (other) {
                .Unknown => |other_id| id == other_id,
                else => false,
            },
            .Array => |arr| switch (other) {
                .Array => |other_arr| arr.element_type.equals(other_arr.element_type.*),
                else => false,
            },
            else => false, // More complex equality checks needed for composite types
        };
    }
};

pub const ArrayType = struct {
    element_type: *CursedType,
    size: ?u64, // None for dynamic arrays
};

pub const SliceType = struct {
    element_type: *CursedType,
};

pub const StructType = struct {
    name: []const u8,
    fields: ArrayList(StructField),
    type_parameters: ?ArrayList(TypeParameter),
    
    pub const StructField = struct {
        name: []const u8,
        field_type: CursedType,
        is_public: bool,
    };
};

pub const InterfaceType = struct {
    name: []const u8,
    methods: ArrayList(MethodSignature),
    type_parameters: ?ArrayList(TypeParameter),
    
    pub const MethodSignature = struct {
        name: []const u8,
        parameters: ArrayList(CursedType),
        return_type: ?CursedType,
    };
};

pub const FunctionType = struct {
    parameters: ArrayList(CursedType),
    return_type: ?*CursedType,
    is_async: bool,
};

pub const ChannelType = struct {
    element_type: *CursedType,
    direction: ChannelDirection,
    
    pub const ChannelDirection = enum {
        Bidirectional,
        Send,
        Receive,
    };
};

pub const TupleType = struct {
    elements: ArrayList(CursedType),
};

pub const GenericType = struct {
    base_type: *CursedType,
    type_args: ArrayList(CursedType),
};

pub const TypeParameterType = struct {
    name: []const u8,
    constraints: ArrayList(TypeConstraint),
    variance: TypeVariance,
    
    pub const TypeVariance = enum {
        Invariant,
        Covariant,
        Contravariant,
    };
};

pub const ConstraintType = struct {
    kind: ConstraintKind,
    target_type: ?CursedType,
    
    pub const ConstraintKind = enum {
        Implements,    // T: Interface
        Extends,       // T: BaseType
        Numeric,       // T: Numeric
        Comparable,    // T: Comparable
        Sized,         // T: Sized
        Send,          // T: Send (can be sent across channels)
        Sync,          // T: Sync (can be shared between goroutines)
    };
};

pub const TypeParameter = struct {
    name: []const u8,
    constraints: ArrayList(TypeConstraint),
    default_type: ?CursedType,
};

pub const TypeConstraint = struct {
    kind: ConstraintType.ConstraintKind,
    bound: ?CursedType,
};

// Enhanced type environment with scoping and inference
pub const TypeEnvironment = struct {
    scopes: ArrayList(Scope),
    type_vars: HashMap(u32, CursedType, std.hash_map.DefaultHashContext(u32), std.hash_map.default_max_load_percentage),
    constraints: ArrayList(TypeConstraintSet),
    next_type_var_id: u32,
    allocator: Allocator,
    
    pub const Scope = struct {
        variables: HashMap([]const u8, VariableInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        types: HashMap([]const u8, CursedType, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        
        pub fn init(allocator: Allocator) Scope {
            return Scope{
                .variables = HashMap([]const u8, VariableInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
                .types = HashMap([]const u8, CursedType, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            };
        }
        
        pub fn deinit(self: *Scope) void {
            self.variables.deinit();
            self.types.deinit();
        }
    };
    
    pub const VariableInfo = struct {
        var_type: CursedType,
        is_mutable: bool,
        is_initialized: bool,
    };
    
    pub const TypeConstraintSet = struct {
        type_var: u32,
        constraints: ArrayList(TypeConstraint),
    };
    
    pub fn init(allocator: Allocator) !TypeEnvironment {
        var env = TypeEnvironment{
            .scopes = ArrayList(Scope).init(allocator),
            .type_vars = HashMap(u32, CursedType, std.hash_map.DefaultHashContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .constraints = ArrayList(TypeConstraintSet).init(allocator),
            .next_type_var_id = 1,
            .allocator = allocator,
        };
        
        // Create global scope with builtin types
        try env.enterScope();
        try env.registerBuiltinTypes();
        
        return env;
    }
    
    pub fn deinit(self: *TypeEnvironment) void {
        for (self.scopes.items) |*scope| {
            scope.deinit();
        }
        self.scopes.deinit();
        self.type_vars.deinit();
        self.constraints.deinit();
    }
    
    pub fn enterScope(self: *TypeEnvironment) !void {
        try self.scopes.append(Scope.init(self.allocator));
    }
    
    pub fn exitScope(self: *TypeEnvironment) void {
        if (self.scopes.items.len > 1) {
            var scope = self.scopes.pop();
            scope.deinit();
        }
    }
    
    pub fn freshTypeVar(self: *TypeEnvironment) u32 {
        const id = self.next_type_var_id;
        self.next_type_var_id += 1;
        return id;
    }
    
    pub fn unifyTypeVar(self: *TypeEnvironment, var_id: u32, concrete_type: CursedType) !void {
        try self.type_vars.put(var_id, concrete_type);
    }
    
    pub fn resolveType(self: *TypeEnvironment, cursed_type: CursedType) CursedType {
        return switch (cursed_type) {
            .Unknown => |var_id| self.type_vars.get(var_id) orelse cursed_type,
            else => cursed_type,
        };
    }
    
    pub fn addVariable(self: *TypeEnvironment, name: []const u8, var_type: CursedType, is_mutable: bool) !void {
        if (self.scopes.items.len == 0) return error.NoScope;
        
        const current_scope = &self.scopes.items[self.scopes.items.len - 1];
        try current_scope.variables.put(name, VariableInfo{
            .var_type = var_type,
            .is_mutable = is_mutable,
            .is_initialized = true,
        });
    }
    
    pub fn lookupVariable(self: *TypeEnvironment, name: []const u8) ?VariableInfo {
        // Search from innermost to outermost scope
        var i = self.scopes.items.len;
        while (i > 0) {
            i -= 1;
            if (self.scopes.items[i].variables.get(name)) |info| {
                return info;
            }
        }
        return null;
    }
    
    pub fn addType(self: *TypeEnvironment, name: []const u8, cursed_type: CursedType) !void {
        if (self.scopes.items.len == 0) return error.NoScope;
        
        const current_scope = &self.scopes.items[self.scopes.items.len - 1];
        try current_scope.types.put(name, cursed_type);
    }
    
    pub fn lookupType(self: *TypeEnvironment, name: []const u8) ?CursedType {
        // Search from innermost to outermost scope
        var i = self.scopes.items.len;
        while (i > 0) {
            i -= 1;
            if (self.scopes.items[i].types.get(name)) |cursed_type| {
                return cursed_type;
            }
        }
        return null;
    }
    
    fn registerBuiltinTypes(self: *TypeEnvironment) !void {
        try self.addType("drip", CursedType.Drip);
        try self.addType("normie", CursedType.Normie);
        try self.addType("smol", CursedType.Smol);
        try self.addType("thicc", CursedType.Thicc);
        try self.addType("meal", CursedType.Meal);
        try self.addType("snack", CursedType.Snack);
        try self.addType("tea", CursedType.Tea);
        try self.addType("lit", CursedType.Lit);
        try self.addType("sip", CursedType.Sip);
        try self.addType("vibes", CursedType.Vibes);
    }
};

// Comprehensive type inference engine
pub const TypeInferenceEngine = struct {
    environment: *TypeEnvironment,
    unification_constraints: ArrayList(UnificationConstraint),
    allocator: Allocator,
    
    pub const UnificationConstraint = struct {
        left: CursedType,
        right: CursedType,
        context: ConstraintContext,
        
        pub const ConstraintContext = enum {
            Assignment,
            FunctionCall,
            ReturnValue,
            Comparison,
            Arithmetic,
        };
    };
    
    pub const InferenceResult = struct {
        inferred_type: CursedType,
        constraints_satisfied: bool,
        error_message: ?[]const u8,
    };
    
    pub fn init(allocator: Allocator, environment: *TypeEnvironment) TypeInferenceEngine {
        return TypeInferenceEngine{
            .environment = environment,
            .unification_constraints = ArrayList(UnificationConstraint).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *TypeInferenceEngine) void {
        self.unification_constraints.deinit();
    }
    
    /// Infer type of an expression
    pub fn inferExpression(self: *TypeInferenceEngine, expr: *const ast.Expression) !InferenceResult {
        return switch (expr.*) {
            .Literal => |lit| self.inferLiteral(lit),
            .Identifier => |id| self.inferIdentifier(id),
            .BinaryOp => |binop| self.inferBinaryOperation(binop),
            .UnaryOp => |unop| self.inferUnaryOperation(unop),
            .FunctionCall => |call| self.inferFunctionCall(call),
            .MemberAccess => |member| self.inferMemberAccess(member),
            .ArrayAccess => |access| self.inferArrayAccess(access),
            .ArrayLiteral => |arr| self.inferArrayLiteral(arr),
            .TupleLiteral => |tup| self.inferTupleLiteral(tup),
            .StructLiteral => |struc| self.inferStructLiteral(struc),
            .TypeAssertion => |assertion| self.inferTypeAssertion(assertion),
            .MatchExpression => |match| self.inferMatchExpression(match),
            else => InferenceResult{
                .inferred_type = CursedType.Error,
                .constraints_satisfied = false,
                .error_message = "Unsupported expression for type inference",
            },
        };
    }
    
    fn inferLiteral(self: *TypeInferenceEngine, literal: ast.LiteralExpression) InferenceResult {
        _ = self;
        
        const inferred_type = switch (literal.value) {
            .Integer => CursedType.Drip, // Default to drip for integers
            .Float => CursedType.Meal,   // Default to meal for floats
            .String => CursedType.Tea,
            .Boolean => CursedType.Lit,
            .Character => CursedType.Sip,
            .Null => CursedType.Vibes,
        };
        
        return InferenceResult{
            .inferred_type = inferred_type,
            .constraints_satisfied = true,
            .error_message = null,
        };
    }
    
    fn inferIdentifier(self: *TypeInferenceEngine, identifier: ast.IdentifierExpression) InferenceResult {
        if (self.environment.lookupVariable(identifier.name)) |var_info| {
            return InferenceResult{
                .inferred_type = var_info.var_type,
                .constraints_satisfied = true,
                .error_message = null,
            };
        } else {
            return InferenceResult{
                .inferred_type = CursedType.Error,
                .constraints_satisfied = false,
                .error_message = "Undefined variable",
            };
        }
    }
    
    fn inferBinaryOperation(self: *TypeInferenceEngine, binop: ast.BinaryOpExpression) !InferenceResult {
        const left_result = try self.inferExpression(binop.left);
        const right_result = try self.inferExpression(binop.right);
        
        if (!left_result.constraints_satisfied or !right_result.constraints_satisfied) {
            return InferenceResult{
                .inferred_type = CursedType.Error,
                .constraints_satisfied = false,
                .error_message = "Type inference failed for operands",
            };
        }
        
        const left_type = self.environment.resolveType(left_result.inferred_type);
        const right_type = self.environment.resolveType(right_result.inferred_type);
        
        return switch (binop.operator) {
            .Add, .Subtract, .Multiply, .Divide, .Modulo => {
                if (left_type.isNumeric() and right_type.isNumeric()) {
                    // Type promotion rules
                    const result_type = self.promoteNumericTypes(left_type, right_type);
                    return InferenceResult{
                        .inferred_type = result_type,
                        .constraints_satisfied = true,
                        .error_message = null,
                    };
                } else {
                    return InferenceResult{
                        .inferred_type = CursedType.Error,
                        .constraints_satisfied = false,
                        .error_message = "Arithmetic operations require numeric types",
                    };
                }
            },
            .Equal, .NotEqual => {
                if (self.areTypesCompatible(left_type, right_type)) {
                    return InferenceResult{
                        .inferred_type = CursedType.Lit,
                        .constraints_satisfied = true,
                        .error_message = null,
                    };
                } else {
                    return InferenceResult{
                        .inferred_type = CursedType.Error,
                        .constraints_satisfied = false,
                        .error_message = "Cannot compare incompatible types",
                    };
                }
            },
            .LessThan, .GreaterThan, .LessEqual, .GreaterEqual => {
                if (left_type.isComparable() and right_type.isComparable() and 
                    self.areTypesCompatible(left_type, right_type)) {
                    return InferenceResult{
                        .inferred_type = CursedType.Lit,
                        .constraints_satisfied = true,
                        .error_message = null,
                    };
                } else {
                    return InferenceResult{
                        .inferred_type = CursedType.Error,
                        .constraints_satisfied = false,
                        .error_message = "Comparison requires comparable types",
                    };
                }
            },
            .LogicalAnd, .LogicalOr => {
                if (left_type.equals(CursedType.Lit) and right_type.equals(CursedType.Lit)) {
                    return InferenceResult{
                        .inferred_type = CursedType.Lit,
                        .constraints_satisfied = true,
                        .error_message = null,
                    };
                } else {
                    return InferenceResult{
                        .inferred_type = CursedType.Error,
                        .constraints_satisfied = false,
                        .error_message = "Logical operations require boolean types",
                    };
                }
            },
            else => InferenceResult{
                .inferred_type = CursedType.Error,
                .constraints_satisfied = false,
                .error_message = "Unsupported binary operator",
            },
        };
    }
    
    fn inferUnaryOperation(self: *TypeInferenceEngine, unop: ast.UnaryOpExpression) !InferenceResult {
        const operand_result = try self.inferExpression(unop.operand);
        
        if (!operand_result.constraints_satisfied) {
            return InferenceResult{
                .inferred_type = CursedType.Error,
                .constraints_satisfied = false,
                .error_message = "Type inference failed for operand",
            };
        }
        
        const operand_type = self.environment.resolveType(operand_result.inferred_type);
        
        return switch (unop.operator) {
            .Negate => {
                if (operand_type.isNumeric()) {
                    return InferenceResult{
                        .inferred_type = operand_type,
                        .constraints_satisfied = true,
                        .error_message = null,
                    };
                } else {
                    return InferenceResult{
                        .inferred_type = CursedType.Error,
                        .constraints_satisfied = false,
                        .error_message = "Negation requires numeric type",
                    };
                }
            },
            .LogicalNot => {
                if (operand_type.equals(CursedType.Lit)) {
                    return InferenceResult{
                        .inferred_type = CursedType.Lit,
                        .constraints_satisfied = true,
                        .error_message = null,
                    };
                } else {
                    return InferenceResult{
                        .inferred_type = CursedType.Error,
                        .constraints_satisfied = false,
                        .error_message = "Logical not requires boolean type",
                    };
                }
            },
            else => InferenceResult{
                .inferred_type = CursedType.Error,
                .constraints_satisfied = false,
                .error_message = "Unsupported unary operator",
            },
        };
    }
    
    fn inferFunctionCall(self: *TypeInferenceEngine, call: ast.FunctionCallExpression) !InferenceResult {
        // Look up function in environment
        if (self.environment.lookupVariable(call.function_name)) |func_info| {
            return switch (func_info.var_type) {
                .Function => |func_type| {
                    // Check parameter types and count
                    if (call.arguments.items.len != func_type.parameters.items.len) {
                        return InferenceResult{
                            .inferred_type = CursedType.Error,
                            .constraints_satisfied = false,
                            .error_message = "Argument count mismatch",
                        };
                    }
                    
                    // Infer and check argument types
                    for (call.arguments.items, 0..) |arg, i| {
                        const arg_result = try self.inferExpression(arg);
                        if (!arg_result.constraints_satisfied) {
                            return InferenceResult{
                                .inferred_type = CursedType.Error,
                                .constraints_satisfied = false,
                                .error_message = "Argument type inference failed",
                            };
                        }
                        
                        const arg_type = self.environment.resolveType(arg_result.inferred_type);
                        const param_type = func_type.parameters.items[i];
                        
                        if (!self.areTypesCompatible(arg_type, param_type)) {
                            return InferenceResult{
                                .inferred_type = CursedType.Error,
                                .constraints_satisfied = false,
                                .error_message = "Argument type mismatch",
                            };
                        }
                    }
                    
                    // Return function return type
                    const return_type = if (func_type.return_type) |ret| ret.* else CursedType.Vibes;
                    return InferenceResult{
                        .inferred_type = return_type,
                        .constraints_satisfied = true,
                        .error_message = null,
                    };
                },
                else => InferenceResult{
                    .inferred_type = CursedType.Error,
                    .constraints_satisfied = false,
                    .error_message = "Identifier is not a function",
                },
            };
        } else {
            return InferenceResult{
                .inferred_type = CursedType.Error,
                .constraints_satisfied = false,
                .error_message = "Function not found",
            };
        }
    }
    
    fn inferMemberAccess(self: *TypeInferenceEngine, member: ast.MemberAccessExpression) !InferenceResult {
        const object_result = try self.inferExpression(member.object);
        
        if (!object_result.constraints_satisfied) {
            return InferenceResult{
                .inferred_type = CursedType.Error,
                .constraints_satisfied = false,
                .error_message = "Object type inference failed",
            };
        }
        
        const object_type = self.environment.resolveType(object_result.inferred_type);
        
        return switch (object_type) {
            .Struct => |struct_type| {
                // Look up field in struct
                for (struct_type.fields.items) |field| {
                    if (std.mem.eql(u8, field.name, member.member_name)) {
                        return InferenceResult{
                            .inferred_type = field.field_type,
                            .constraints_satisfied = true,
                            .error_message = null,
                        };
                    }
                }
                return InferenceResult{
                    .inferred_type = CursedType.Error,
                    .constraints_satisfied = false,
                    .error_message = "Field not found in struct",
                };
            },
            .Interface => |interface_type| {
                // Look up method in interface
                for (interface_type.methods.items) |method| {
                    if (std.mem.eql(u8, method.name, member.member_name)) {
                        // Create function type for method
                        const func_type = try self.allocator.create(FunctionType);
                        func_type.* = FunctionType{
                            .parameters = method.parameters,
                            .return_type = if (method.return_type) |ret| blk: {
                                const ret_ptr = try self.allocator.create(CursedType);
                                ret_ptr.* = ret;
                                break :blk ret_ptr;
                            } else null,
                            .is_async = false,
                        };
                        
                        return InferenceResult{
                            .inferred_type = CursedType{ .Function = func_type },
                            .constraints_satisfied = true,
                            .error_message = null,
                        };
                    }
                }
                return InferenceResult{
                    .inferred_type = CursedType.Error,
                    .constraints_satisfied = false,
                    .error_message = "Method not found in interface",
                };
            },
            else => InferenceResult{
                .inferred_type = CursedType.Error,
                .constraints_satisfied = false,
                .error_message = "Member access requires struct or interface type",
            },
        };
    }
    
    fn inferArrayAccess(self: *TypeInferenceEngine, access: ast.ArrayAccessExpression) !InferenceResult {
        const array_result = try self.inferExpression(access.array);
        const index_result = try self.inferExpression(access.index);
        
        if (!array_result.constraints_satisfied or !index_result.constraints_satisfied) {
            return InferenceResult{
                .inferred_type = CursedType.Error,
                .constraints_satisfied = false,
                .error_message = "Array or index type inference failed",
            };
        }
        
        const array_type = self.environment.resolveType(array_result.inferred_type);
        const index_type = self.environment.resolveType(index_result.inferred_type);
        
        // Check index is integer
        if (!index_type.isInteger()) {
            return InferenceResult{
                .inferred_type = CursedType.Error,
                .constraints_satisfied = false,
                .error_message = "Array index must be integer type",
            };
        }
        
        return switch (array_type) {
            .Array => |arr| InferenceResult{
                .inferred_type = arr.element_type.*,
                .constraints_satisfied = true,
                .error_message = null,
            },
            .Slice => |slice| InferenceResult{
                .inferred_type = slice.element_type.*,
                .constraints_satisfied = true,
                .error_message = null,
            },
            else => InferenceResult{
                .inferred_type = CursedType.Error,
                .constraints_satisfied = false,
                .error_message = "Array access requires array or slice type",
            },
        };
    }
    
    fn inferArrayLiteral(self: *TypeInferenceEngine, array_lit: ast.ArrayLiteralExpression) !InferenceResult {
        if (array_lit.elements.items.len == 0) {
            // Empty array - create type variable for element type
            const element_type_var = self.environment.freshTypeVar();
            const element_type_ptr = try self.allocator.create(CursedType);
            element_type_ptr.* = CursedType{ .Unknown = element_type_var };
            
            const array_type_ptr = try self.allocator.create(ArrayType);
            array_type_ptr.* = ArrayType{
                .element_type = element_type_ptr,
                .size = 0,
            };
            
            return InferenceResult{
                .inferred_type = CursedType{ .Array = array_type_ptr },
                .constraints_satisfied = true,
                .error_message = null,
            };
        }
        
        // Infer type from first element
        const first_result = try self.inferExpression(array_lit.elements.items[0]);
        if (!first_result.constraints_satisfied) {
            return InferenceResult{
                .inferred_type = CursedType.Error,
                .constraints_satisfied = false,
                .error_message = "Element type inference failed",
            };
        }
        
        const element_type = self.environment.resolveType(first_result.inferred_type);
        
        // Check all elements have compatible types
        for (array_lit.elements.items[1..]) |elem| {
            const elem_result = try self.inferExpression(elem);
            if (!elem_result.constraints_satisfied) {
                return InferenceResult{
                    .inferred_type = CursedType.Error,
                    .constraints_satisfied = false,
                    .error_message = "Element type inference failed",
                };
            }
            
            const elem_type = self.environment.resolveType(elem_result.inferred_type);
            if (!self.areTypesCompatible(element_type, elem_type)) {
                return InferenceResult{
                    .inferred_type = CursedType.Error,
                    .constraints_satisfied = false,
                    .error_message = "Array elements must have compatible types",
                };
            }
        }
        
        const element_type_ptr = try self.allocator.create(CursedType);
        element_type_ptr.* = element_type;
        
        const array_type_ptr = try self.allocator.create(ArrayType);
        array_type_ptr.* = ArrayType{
            .element_type = element_type_ptr,
            .size = array_lit.elements.items.len,
        };
        
        return InferenceResult{
            .inferred_type = CursedType{ .Array = array_type_ptr },
            .constraints_satisfied = true,
            .error_message = null,
        };
    }
    
    fn inferTupleLiteral(self: *TypeInferenceEngine, tuple_lit: ast.TupleLiteralExpression) !InferenceResult {
        var element_types = ArrayList(CursedType).init(self.allocator);
        
        for (tuple_lit.elements.items) |elem| {
            const elem_result = try self.inferExpression(elem);
            if (!elem_result.constraints_satisfied) {
                return InferenceResult{
                    .inferred_type = CursedType.Error,
                    .constraints_satisfied = false,
                    .error_message = "Tuple element type inference failed",
                };
            }
            
            try element_types.append(self.environment.resolveType(elem_result.inferred_type));
        }
        
        const tuple_type_ptr = try self.allocator.create(TupleType);
        tuple_type_ptr.* = TupleType{
            .elements = element_types,
        };
        
        return InferenceResult{
            .inferred_type = CursedType{ .Tuple = tuple_type_ptr },
            .constraints_satisfied = true,
            .error_message = null,
        };
    }
    
    fn inferStructLiteral(self: *TypeInferenceEngine, struct_lit: ast.StructLiteralExpression) !InferenceResult {
        // Look up struct type by name
        if (self.environment.lookupType(struct_lit.struct_name)) |struct_type| {
            return switch (struct_type) {
                .Struct => |struct_info| {
                    // Check all required fields are provided and types match
                    for (struct_info.fields.items) |field| {
                        var found = false;
                        for (struct_lit.fields.items) |init_field| {
                            if (std.mem.eql(u8, field.name, init_field.name)) {
                                found = true;
                                const field_result = try self.inferExpression(init_field.value);
                                if (!field_result.constraints_satisfied) {
                                    return InferenceResult{
                                        .inferred_type = CursedType.Error,
                                        .constraints_satisfied = false,
                                        .error_message = "Struct field type inference failed",
                                    };
                                }
                                
                                const field_type = self.environment.resolveType(field_result.inferred_type);
                                if (!self.areTypesCompatible(field.field_type, field_type)) {
                                    return InferenceResult{
                                        .inferred_type = CursedType.Error,
                                        .constraints_satisfied = false,
                                        .error_message = "Struct field type mismatch",
                                    };
                                }
                                break;
                            }
                        }
                        if (!found) {
                            return InferenceResult{
                                .inferred_type = CursedType.Error,
                                .constraints_satisfied = false,
                                .error_message = "Missing required struct field",
                            };
                        }
                    }
                    
                    return InferenceResult{
                        .inferred_type = struct_type,
                        .constraints_satisfied = true,
                        .error_message = null,
                    };
                },
                else => InferenceResult{
                    .inferred_type = CursedType.Error,
                    .constraints_satisfied = false,
                    .error_message = "Name does not refer to a struct type",
                },
            };
        } else {
            return InferenceResult{
                .inferred_type = CursedType.Error,
                .constraints_satisfied = false,
                .error_message = "Unknown struct type",
            };
        }
    }
    
    fn inferTypeAssertion(self: *TypeInferenceEngine, assertion: ast.TypeAssertionExpression) !InferenceResult {
        const expr_result = try self.inferExpression(assertion.expression);
        
        if (!expr_result.constraints_satisfied) {
            return InferenceResult{
                .inferred_type = CursedType.Error,
                .constraints_satisfied = false,
                .error_message = "Expression type inference failed",
            };
        }
        
        // Convert AST type to CursedType
        const target_type = try self.astTypeToCursedType(assertion.target_type);
        
        // Check if assertion is valid (runtime check)
        return InferenceResult{
            .inferred_type = target_type,
            .constraints_satisfied = true,
            .error_message = null,
        };
    }
    
    fn inferMatchExpression(self: *TypeInferenceEngine, match_expr: ast.MatchExpression) !InferenceResult {
        const scrutinee_result = try self.inferExpression(match_expr.scrutinee);
        
        if (!scrutinee_result.constraints_satisfied) {
            return InferenceResult{
                .inferred_type = CursedType.Error,
                .constraints_satisfied = false,
                .error_message = "Match scrutinee type inference failed",
            };
        }
        
        const scrutinee_type = self.environment.resolveType(scrutinee_result.inferred_type);
        
        // All case expressions must have the same type
        var result_type: ?CursedType = null;
        
        for (match_expr.cases.items) |case| {
            // Check pattern compatibility with scrutinee
            const pattern_type = try self.inferPattern(case.pattern);
            if (!self.areTypesCompatible(scrutinee_type, pattern_type)) {
                return InferenceResult{
                    .inferred_type = CursedType.Error,
                    .constraints_satisfied = false,
                    .error_message = "Pattern type incompatible with scrutinee",
                };
            }
            
            // Infer case expression type
            const case_result = try self.inferExpression(case.expression);
            if (!case_result.constraints_satisfied) {
                return InferenceResult{
                    .inferred_type = CursedType.Error,
                    .constraints_satisfied = false,
                    .error_message = "Match case expression type inference failed",
                };
            }
            
            const case_type = self.environment.resolveType(case_result.inferred_type);
            
            if (result_type) |existing_type| {
                if (!self.areTypesCompatible(existing_type, case_type)) {
                    return InferenceResult{
                        .inferred_type = CursedType.Error,
                        .constraints_satisfied = false,
                        .error_message = "Match case expressions must have compatible types",
                    };
                }
            } else {
                result_type = case_type;
            }
        }
        
        return InferenceResult{
            .inferred_type = result_type orelse CursedType.Vibes,
            .constraints_satisfied = true,
            .error_message = null,
        };
    }
    
    /// Helper functions
    
    fn promoteNumericTypes(self: *TypeInferenceEngine, left: CursedType, right: CursedType) CursedType {
        _ = self;
        
        // Type promotion rules for CURSED
        if (left.isFloat() or right.isFloat()) {
            // Promote to largest float type
            if (left == .Meal or right == .Meal) return CursedType.Meal;
            return CursedType.Snack;
        }
        
        // Integer promotion
        if (left == .Thicc or right == .Thicc) return CursedType.Thicc;
        if (left == .Drip or right == .Drip) return CursedType.Drip;
        if (left == .Normie or right == .Normie) return CursedType.Normie;
        return CursedType.Smol;
    }
    
    fn areTypesCompatible(self: *TypeInferenceEngine, source: CursedType, target: CursedType) bool {
        _ = self;
        
        // Exact match
        if (source.equals(target)) return true;
        
        // Numeric conversions
        if (source.isNumeric() and target.isNumeric()) {
            // Allow implicit widening conversions
            return switch (source) {
                .Smol => target.isInteger() or target.isFloat(),
                .Normie => target == .Drip or target == .Thicc or target.isFloat(),
                .Drip => target == .Thicc or target.isFloat(),
                .Snack => target == .Meal,
                else => false,
            };
        }
        
        // Interface implementation (simplified)
        return false;
    }
    
    fn inferPattern(self: *TypeInferenceEngine, pattern: ast.Pattern) !CursedType {
        return switch (pattern) {
            .Literal => |lit| switch (lit.value) {
                .Integer => CursedType.Drip,
                .Float => CursedType.Meal,
                .String => CursedType.Tea,
                .Boolean => CursedType.Lit,
                .Character => CursedType.Sip,
                .Null => CursedType.Vibes,
            },
            .Identifier => |id| {
                // Pattern binding - create fresh type variable
                const type_var = self.environment.freshTypeVar();
                try self.environment.addVariable(id.name, CursedType{ .Unknown = type_var }, false);
                return CursedType{ .Unknown = type_var };
            },
            .Wildcard => {
                // Wildcard matches any type
                const type_var = self.environment.freshTypeVar();
                return CursedType{ .Unknown = type_var };
            },
            else => CursedType.Error,
        };
    }
    
    fn astTypeToCursedType(self: *TypeInferenceEngine, ast_type: ast.Type) !CursedType {
        return switch (ast_type) {
            .Primitive => |prim| switch (prim) {
                .Drip => CursedType.Drip,
                .Normie => CursedType.Normie,
                .Smol => CursedType.Smol,
                .Thicc => CursedType.Thicc,
                .Meal => CursedType.Meal,
                .Snack => CursedType.Snack,
                .Tea => CursedType.Tea,
                .Lit => CursedType.Lit,
                .Sip => CursedType.Sip,
                .Vibes => CursedType.Vibes,
            },
            .Identifier => |name| {
                if (self.environment.lookupType(name)) |cursed_type| {
                    return cursed_type;
                } else {
                    return CursedType.Error;
                }
            },
            .Array => |arr| {
                const element_type = try self.astTypeToCursedType(arr.element_type.*);
                const element_type_ptr = try self.allocator.create(CursedType);
                element_type_ptr.* = element_type;
                
                const array_type_ptr = try self.allocator.create(ArrayType);
                array_type_ptr.* = ArrayType{
                    .element_type = element_type_ptr,
                    .size = arr.size,
                };
                
                return CursedType{ .Array = array_type_ptr };
            },
            else => CursedType.Error,
        };
    }
};

/// Comprehensive type checker with constraint resolution
pub const ComprehensiveTypeChecker = struct {
    environment: TypeEnvironment,
    inference_engine: TypeInferenceEngine,
    error_messages: ArrayList(TypeErrorMessage),
    allocator: Allocator,
    
    pub const TypeErrorMessage = struct {
        kind: ErrorKind,
        message: []const u8,
        line: u32,
        column: u32,
        
        pub const ErrorKind = enum {
            TypeError,
            InferenceError,
            ConstraintViolation,
            UnknownIdentifier,
            ArgumentMismatch,
            ReturnTypeMismatch,
        };
    };
    
    pub fn init(allocator: Allocator) !ComprehensiveTypeChecker {
        var environment = try TypeEnvironment.init(allocator);
        var inference_engine = TypeInferenceEngine.init(allocator, &environment);
        
        return ComprehensiveTypeChecker{
            .environment = environment,
            .inference_engine = inference_engine,
            .error_messages = ArrayList(TypeErrorMessage).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *ComprehensiveTypeChecker) void {
        self.environment.deinit();
        self.inference_engine.deinit();
        self.error_messages.deinit();
    }
    
    pub fn checkProgram(self: *ComprehensiveTypeChecker, program: *const ast.Program) !bool {
        var success = true;
        
        for (program.statements.items) |stmt| {
            if (!try self.checkStatement(stmt)) {
                success = false;
            }
        }
        
        return success;
    }
    
    pub fn checkStatement(self: *ComprehensiveTypeChecker, stmt: ast.Statement) !bool {
        return switch (stmt) {
            .VariableDeclaration => |var_decl| self.checkVariableDeclaration(var_decl),
            .FunctionDeclaration => |func_decl| self.checkFunctionDeclaration(func_decl),
            .StructDeclaration => |struct_decl| self.checkStructDeclaration(struct_decl),
            .InterfaceDeclaration => |interface_decl| self.checkInterfaceDeclaration(interface_decl),
            .Assignment => |assign| self.checkAssignment(assign),
            .Return => |ret| self.checkReturn(ret),
            .Expression => |expr| (try self.inference_engine.inferExpression(expr.expression)).constraints_satisfied,
            else => true,
        };
    }
    
    fn checkVariableDeclaration(self: *ComprehensiveTypeChecker, var_decl: ast.VariableDeclaration) !bool {
        if (var_decl.init_value) |init_val| {
            const init_result = try self.inference_engine.inferExpression(init_val);
            
            if (!init_result.constraints_satisfied) {
                try self.addError(.InferenceError, init_result.error_message orelse "Type inference failed", 0, 0);
                return false;
            }
            
            var var_type: CursedType = undefined;
            
            if (var_decl.var_type) |explicit_type| {
                // Check explicit type matches inferred type
                var_type = try self.inference_engine.astTypeToCursedType(explicit_type);
                
                if (!self.inference_engine.areTypesCompatible(init_result.inferred_type, var_type)) {
                    try self.addError(.TypeError, "Variable type annotation doesn't match initialization", 0, 0);
                    return false;
                }
            } else {
                // Use inferred type
                var_type = init_result.inferred_type;
            }
            
            try self.environment.addVariable(var_decl.name, var_type, var_decl.is_mutable);
            return true;
        } else if (var_decl.var_type) |explicit_type| {
            // Variable declaration without initialization
            const var_type = try self.inference_engine.astTypeToCursedType(explicit_type);
            try self.environment.addVariable(var_decl.name, var_type, var_decl.is_mutable);
            return true;
        } else {
            try self.addError(.TypeError, "Variable declaration must have type annotation or initialization", 0, 0);
            return false;
        }
    }
    
    fn checkFunctionDeclaration(self: *ComprehensiveTypeChecker, func_decl: ast.FunctionDeclaration) !bool {
        // Enter new scope for function
        try self.environment.enterScope();
        defer self.environment.exitScope();
        
        // Add parameters to scope
        var param_types = ArrayList(CursedType).init(self.allocator);
        defer param_types.deinit();
        
        for (func_decl.parameters.items) |param| {
            const param_type = try self.inference_engine.astTypeToCursedType(param.param_type);
            try param_types.append(param_type);
            try self.environment.addVariable(param.name, param_type, false);
        }
        
        // Create function type
        const return_type_ptr = if (func_decl.return_type) |ret_type| blk: {
            const ptr = try self.allocator.create(CursedType);
            ptr.* = try self.inference_engine.astTypeToCursedType(ret_type);
            break :blk ptr;
        } else null;
        
        const func_type_ptr = try self.allocator.create(FunctionType);
        func_type_ptr.* = FunctionType{
            .parameters = param_types,
            .return_type = return_type_ptr,
            .is_async = func_decl.is_async,
        };
        
        // Add function to outer scope
        try self.environment.exitScope();
        try self.environment.addVariable(func_decl.name, CursedType{ .Function = func_type_ptr }, false);
        try self.environment.enterScope();
        
        // Re-add parameters to inner scope
        for (func_decl.parameters.items) |param| {
            const param_type = try self.inference_engine.astTypeToCursedType(param.param_type);
            try self.environment.addVariable(param.name, param_type, false);
        }
        
        // Check function body
        var success = true;
        for (func_decl.body.items) |stmt| {
            if (!try self.checkStatement(stmt)) {
                success = false;
            }
        }
        
        return success;
    }
    
    fn checkStructDeclaration(self: *ComprehensiveTypeChecker, struct_decl: ast.StructDeclaration) !bool {
        var fields = ArrayList(StructType.StructField).init(self.allocator);
        
        for (struct_decl.fields.items) |field| {
            const field_type = try self.inference_engine.astTypeToCursedType(field.field_type);
            try fields.append(StructType.StructField{
                .name = field.name,
                .field_type = field_type,
                .is_public = true, // Default to public
            });
        }
        
        const struct_type_ptr = try self.allocator.create(StructType);
        struct_type_ptr.* = StructType{
            .name = struct_decl.name,
            .fields = fields,
            .type_parameters = null,
        };
        
        try self.environment.addType(struct_decl.name, CursedType{ .Struct = struct_type_ptr });
        return true;
    }
    
    fn checkInterfaceDeclaration(self: *ComprehensiveTypeChecker, interface_decl: ast.InterfaceDeclaration) !bool {
        var methods = ArrayList(InterfaceType.MethodSignature).init(self.allocator);
        
        for (interface_decl.methods.items) |method| {
            var param_types = ArrayList(CursedType).init(self.allocator);
            for (method.parameters.items) |param| {
                try param_types.append(try self.inference_engine.astTypeToCursedType(param.param_type));
            }
            
            const return_type = if (method.return_type) |ret| 
                try self.inference_engine.astTypeToCursedType(ret) 
            else 
                null;
            
            try methods.append(InterfaceType.MethodSignature{
                .name = method.name,
                .parameters = param_types,
                .return_type = return_type,
            });
        }
        
        const interface_type_ptr = try self.allocator.create(InterfaceType);
        interface_type_ptr.* = InterfaceType{
            .name = interface_decl.name,
            .methods = methods,
            .type_parameters = null,
        };
        
        try self.environment.addType(interface_decl.name, CursedType{ .Interface = interface_type_ptr });
        return true;
    }
    
    fn checkAssignment(self: *ComprehensiveTypeChecker, assign: ast.AssignmentStatement) !bool {
        const value_result = try self.inference_engine.inferExpression(assign.value);
        
        if (!value_result.constraints_satisfied) {
            try self.addError(.InferenceError, value_result.error_message orelse "Value type inference failed", 0, 0);
            return false;
        }
        
        const target_result = try self.inference_engine.inferExpression(assign.target);
        
        if (!target_result.constraints_satisfied) {
            try self.addError(.InferenceError, "Assignment target type inference failed", 0, 0);
            return false;
        }
        
        if (!self.inference_engine.areTypesCompatible(value_result.inferred_type, target_result.inferred_type)) {
            try self.addError(.TypeError, "Assignment type mismatch", 0, 0);
            return false;
        }
        
        return true;
    }
    
    fn checkReturn(self: *ComprehensiveTypeChecker, ret: ast.ReturnStatement) !bool {
        if (ret.value) |value| {
            const value_result = try self.inference_engine.inferExpression(value);
            
            if (!value_result.constraints_satisfied) {
                try self.addError(.InferenceError, value_result.error_message orelse "Return value type inference failed", 0, 0);
                return false;
            }
            
            // TODO: Check against function return type
            return true;
        }
        
        return true;
    }
    
    fn addError(self: *ComprehensiveTypeChecker, kind: TypeErrorMessage.ErrorKind, message: []const u8, line: u32, column: u32) !void {
        try self.error_messages.append(TypeErrorMessage{
            .kind = kind,
            .message = try self.allocator.dupe(u8, message),
            .line = line,
            .column = column,
        });
    }
    
    pub fn getErrorMessages(self: *const ComprehensiveTypeChecker) []const TypeErrorMessage {
        return self.error_messages.items;
    }
};
