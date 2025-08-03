const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Simplified AST to break circular dependencies without complex memory management
pub const Expression = struct {
    allocator: Allocator,
    kind: ExpressionKind,
    
    pub const ExpressionKind = union(enum) {
        // Primitive types
        identifier: []const u8,
        variable: []const u8,
        integer: i64,
        float: f64,
        string: []const u8,
        boolean: bool,
        character: u8,
        
        // Complex types using heap-allocated pointers
        binary: *BinaryExpressionData,
        call: *CallExpressionData,
        member_access: *MemberAccessData,
        unary: *UnaryExpressionData,
        array: *ArrayExpressionData,
        literal: Literal,
        
        // Other expression types can be added here
    };
    
    pub fn init(allocator: Allocator, kind: ExpressionKind) !*Expression {
        const expr = try allocator.create(Expression);
        expr.* = Expression{
            .allocator = allocator,
            .kind = kind,
        };
        return expr;
    }
    
    pub fn deinit(self: *Expression) void {
        switch (self.kind) {
            .binary => |binary_data| {
                binary_data.left.deinit();
                binary_data.right.deinit();
                self.allocator.destroy(binary_data);
            },
            .call => |call_data| {
                call_data.function.deinit();
                for (call_data.arguments.items) |arg| {
                    arg.deinit();
                }
                call_data.arguments.deinit();
                self.allocator.destroy(call_data);
            },
            .unary => |unary_data| {
                unary_data.operand.deinit();
                self.allocator.destroy(unary_data);
            },
            .array => |array_data| {
                for (array_data.elements.items) |elem| {
                    elem.deinit();
                }
                array_data.elements.deinit();
                self.allocator.destroy(array_data);
            },
            .member_access => |member_data| {
                member_data.object.deinit();
                self.allocator.destroy(member_data);
            },
            // Primitive types don't need cleanup
            else => {},
        }
        self.allocator.destroy(self);
    }
    
    pub fn print(self: *const Expression, indent: usize) !void {
        const print_fn = std.debug.print;
        const spaces = "  " ** 10;
        
        switch (self.kind) {
            .identifier => |id| print_fn("{s}{s}", .{ spaces[0..indent], id }),
            .variable => |var_name| print_fn("{s}{s}", .{ spaces[0..indent], var_name }),
            .integer => |int| print_fn("{s}{}", .{ spaces[0..indent], int }),
            .float => |float| print_fn("{s}{d}", .{ spaces[0..indent], float }),
            .string => |str| print_fn("{s}\"{s}\"", .{ spaces[0..indent], str }),
            .boolean => |bool_val| print_fn("{s}{}", .{ spaces[0..indent], bool_val }),
            .character => |char| print_fn("{s}'{c}'", .{ spaces[0..indent], char }),
            .binary => |binary| {
                try binary.left.print(indent);
                print_fn(" {s} ", .{binary.operator});
                try binary.right.print(indent);
            },
            .call => |call| {
                try call.function.print(indent);
                print_fn("(", .{});
                for (call.arguments.items, 0..) |arg, i| {
                    if (i > 0) print_fn(", ", .{});
                    try arg.print(0);
                }
                print_fn(")", .{});
            },
            else => print_fn("{s}{s}", .{ spaces[0..indent], @tagName(self.kind) }),
        }
    }
};

// Expression data structures
pub const BinaryExpressionData = struct {
    left: *Expression,
    operator: []const u8,
    right: *Expression,
};

pub const CallExpressionData = struct {
    function: *Expression,
    arguments: ArrayList(*Expression),
};

pub const MemberAccessData = struct {
    object: *Expression,
    property: []const u8,
};

pub const UnaryExpressionData = struct {
    operator: []const u8,
    operand: *Expression,
};

pub const ArrayExpressionData = struct {
    elements: ArrayList(*Expression),
};

// Statement types
pub const Statement = struct {
    allocator: Allocator,
    kind: StatementKind,
    
    pub const StatementKind = union(enum) {
        expression: *Expression,
        let: *LetStatementData,
        assignment: *AssignmentStatementData,
        return_stmt: *ReturnStatementData,
        if_stmt: *IfStatementData,
        function: *FunctionStatementData,
        while_stmt: *WhileStatementData,
        struct_stmt: *StructStatementData,
        interface: *InterfaceStatementData,
    };
    
    pub fn init(allocator: Allocator, kind: StatementKind) !*Statement {
        const stmt = try allocator.create(Statement);
        stmt.* = Statement{
            .allocator = allocator,
            .kind = kind,
        };
        return stmt;
    }
    
    pub fn deinit(self: *Statement) void {
        switch (self.kind) {
            .expression => |expr| {
                expr.deinit();
            },
            .let => |let_data| {
                if (let_data.var_type) |*var_type| {
                    var_type.deinit(self.allocator);
                }
                if (let_data.initializer) |initializer| {
                    initializer.deinit();
                }
                self.allocator.destroy(let_data);
            },
            .function => |func_data| {
                for (func_data.body.items) |stmt| {
                    stmt.deinit();
                }
                func_data.body.deinit();
                func_data.parameters.deinit();
                func_data.type_parameters.deinit();
                func_data.comments.deinit();
                self.allocator.destroy(func_data);
            },
            else => {
                // TODO: Add cleanup for other statement types
            },
        }
        self.allocator.destroy(self);
    }
    
    pub fn print(self: *const Statement, indent: usize) !void {
        const print_fn = std.debug.print;
        const spaces = "  " ** 10;
        
        switch (self.kind) {
            .expression => |expr| {
                print_fn("{s}Expression: ", .{spaces[0..indent]});
                try expr.print(0);
                print_fn("\n", .{});
            },
            .let => |let_data| {
                print_fn("{s}Let: {s} = ", .{ spaces[0..indent], let_data.name });
                if (let_data.initializer) |initializer| {
                    try initializer.print(0);
                }
                print_fn("\n", .{});
            },
            .function => |func_data| {
                print_fn("{s}Function: {s}\n", .{ spaces[0..indent], func_data.name });
                for (func_data.body.items) |stmt| {
                    try stmt.print(indent + 2);
                }
            },
            else => {
                print_fn("{s}Statement: {s}\n", .{ spaces[0..indent], @tagName(self.kind) });
            },
        }
    }
};

// Statement data structures
pub const LetStatementData = struct {
    name: []const u8,
    var_type: ?Type,
    initializer: ?*Expression,
    is_mutable: bool,
};

pub const AssignmentStatementData = struct {
    target: *Expression,
    value: *Expression,
    operator: []const u8,
};

pub const ReturnStatementData = struct {
    value: ?*Expression,
};

pub const IfStatementData = struct {
    condition: *Expression,
    then_branch: ArrayList(*Statement),
    else_branch: ?ArrayList(*Statement),
};

pub const FunctionStatementData = struct {
    name: []const u8,
    parameters: ArrayList(Parameter),
    return_type: ?Type,
    body: ArrayList(*Statement),
    visibility: Visibility,
    is_async: bool,
    type_parameters: ArrayList(TypeParameter),
    comments: ArrayList(Comment),
};

pub const WhileStatementData = struct {
    condition: *Expression,
    body: ArrayList(*Statement),
};

pub const StructStatementData = struct {
    name: []const u8,
    fields: ArrayList(StructField),
    visibility: Visibility,
    type_parameters: ArrayList(TypeParameter),
};

pub const InterfaceStatementData = struct {
    name: []const u8,
    methods: ArrayList(MethodSignature),
    visibility: Visibility,
    type_parameters: ArrayList(TypeParameter),
};

// Program structure  
pub const Program = struct {
    statements: ArrayList(*Statement),
    imports: ArrayList(ImportStatement),
    package: ?PackageDeclaration,
    allocator: Allocator,

    pub fn init(allocator: Allocator) Program {
        return Program{
            .statements = ArrayList(*Statement).init(allocator),
            .imports = ArrayList(ImportStatement).init(allocator),
            .package = null,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Program) void {
        for (self.statements.items) |stmt| {
            stmt.deinit();
        }
        self.statements.deinit();
        
        for (self.imports.items) |*import| {
            import.deinit(self.allocator);
        }
        self.imports.deinit();
        
        if (self.package) |*pkg| {
            pkg.deinit(self.allocator);
        }
    }

    pub fn print(self: *const Program, indent: usize) !void {
        const print_fn = std.debug.print;
        const spaces = "  " ** 10;
        
        print_fn("{s}Program:\n", .{spaces[0..indent]});
        
        if (self.package) |pkg| {
            print_fn("{s}  Package: {s}\n", .{ spaces[0..indent], pkg.name });
        }
        
        for (self.imports.items) |import| {
            print_fn("{s}  Import: {s}\n", .{ spaces[0..indent], import.path });
        }
        
        for (self.statements.items) |stmt| {
            try stmt.print(indent + 2);
        }
    }
};

// Supporting types
pub const ImportStatement = struct {
    path: []const u8,
    alias: ?[]const u8,
    items: ArrayList([]const u8),

    pub fn init(allocator: Allocator, path: []const u8) ImportStatement {
        return ImportStatement{
            .path = path,
            .alias = null,
            .items = ArrayList([]const u8).init(allocator),
        };
    }

    pub fn deinit(self: *ImportStatement, allocator: Allocator) void {
        _ = allocator;
        self.items.deinit();
    }
};

pub const PackageDeclaration = struct {
    name: []const u8,
    version: ?[]const u8,

    pub fn deinit(self: *PackageDeclaration, allocator: Allocator) void {
        _ = allocator;
        _ = self;
    }
};

pub const Visibility = enum {
    Public,    // spill
    Private,   // priv  
    Package,   // crew
};

pub const Comment = struct {
    text: []const u8,
    is_doc_comment: bool,
    line: usize,
    column: usize,
};

// Type system
pub const Type = union(enum) {
    Basic: BasicType,
    Channel: ChannelType,
    Array: ArrayType,
    Slice: SliceType,
    Map: MapType,
    Pointer: PointerType,
    Function: FunctionType,
    Interface: InterfaceType,
    Struct: StructType,
    Generic: GenericType,
    Tuple: TupleType,

    pub fn deinit(self: *Type, allocator: Allocator) void {
        switch (self.*) {
            .Array => |*arr| arr.deinit(allocator),
            .Map => |*map| map.deinit(allocator),
            .Function => |*func| func.deinit(allocator),
            .Tuple => |*tuple| tuple.deinit(allocator),
            else => {},
        }
    }
};

pub const BasicType = enum {
    Normie,    // i32
    Tea,       // string  
    Txt,       // string alias
    Sip,       // char
    Smol,      // i8
    Mid,       // i16
    Thicc,     // i64
    Snack,     // f32
    Meal,      // f64
    Byte,      // u8
    Rune,      // i32 alias
    Extra,     // complex
    Lit,       // bool
    Cap,       // null/nil
};

pub const ChannelType = struct {
    element_type: *Type,
    is_send_only: bool,
    is_receive_only: bool,
};

pub const ArrayType = struct {
    element_type: *Type,
    size: ?usize,

    pub fn deinit(self: *ArrayType, allocator: Allocator) void {
        self.element_type.deinit(allocator);
        allocator.destroy(self.element_type);
    }
};

pub const SliceType = struct {
    element_type: *Type,
};

pub const MapType = struct {
    key_type: *Type,
    value_type: *Type,

    pub fn deinit(self: *MapType, allocator: Allocator) void {
        self.key_type.deinit(allocator);
        self.value_type.deinit(allocator);
        allocator.destroy(self.key_type);
        allocator.destroy(self.value_type);
    }
};

pub const PointerType = struct {
    target_type: *Type,
};

pub const FunctionType = struct {
    parameters: ArrayList(Type),
    return_type: ?*Type,

    pub fn deinit(self: *FunctionType, allocator: Allocator) void {
        for (self.parameters.items) |*param| {
            param.deinit(allocator);
        }
        self.parameters.deinit();
        
        if (self.return_type) |ret| {
            ret.deinit(allocator);
            allocator.destroy(ret);
        }
    }
};

pub const InterfaceType = struct {
    name: []const u8,
    methods: ArrayList(MethodSignature),
};

pub const StructType = struct {
    name: []const u8,
    fields: ArrayList(StructField),
};

pub const GenericType = struct {
    name: []const u8,
    constraints: ArrayList(Type),
};

pub const TupleType = struct {
    elements: ArrayList(Type),

    pub fn deinit(self: *TupleType, allocator: Allocator) void {
        for (self.elements.items) |*elem| {
            elem.deinit(allocator);
        }
        self.elements.deinit();
    }
};

pub const MethodSignature = struct {
    name: []const u8,
    parameters: ArrayList(Parameter),
    return_type: ?Type,
};

pub const StructField = struct {
    name: []const u8,
    field_type: Type,
    visibility: Visibility,
};

pub const Parameter = struct {
    name: []const u8,
    param_type: Type,
    is_mutable: bool,
    default_value: ?*Expression,
};

pub const TypeParameter = struct {
    name: []const u8,
    constraints: ArrayList(Type),
};

// Supporting structures for expressions
pub const Literal = union(enum) {
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    Character: u8,
    Null,
};

// Helper functions for AST creation
pub fn createIntegerExpression(allocator: Allocator, value: i64) !*Expression {
    return Expression.init(allocator, .{ .integer = value });
}

pub fn createFloatExpression(allocator: Allocator, value: f64) !*Expression {
    return Expression.init(allocator, .{ .float = value });
}

pub fn createStringExpression(allocator: Allocator, value: []const u8) !*Expression {
    return Expression.init(allocator, .{ .string = value });
}

pub fn createBooleanExpression(allocator: Allocator, value: bool) !*Expression {
    return Expression.init(allocator, .{ .boolean = value });
}

pub fn createIdentifierExpression(allocator: Allocator, name: []const u8) !*Expression {
    return Expression.init(allocator, .{ .identifier = name });
}

pub fn createBinaryExpression(allocator: Allocator, left: *Expression, operator: []const u8, right: *Expression) !*Expression {
    const binary_data = try allocator.create(BinaryExpressionData);
    binary_data.* = BinaryExpressionData{
        .left = left,
        .operator = operator,
        .right = right,
    };
    
    return Expression.init(allocator, .{ .binary = binary_data });
}

pub fn createCallExpression(allocator: Allocator, function: *Expression, arguments: ArrayList(*Expression)) !*Expression {
    const call_data = try allocator.create(CallExpressionData);
    call_data.* = CallExpressionData{
        .function = function,
        .arguments = arguments,
    };
    
    return Expression.init(allocator, .{ .call = call_data });
}

// Test functions
test "ast creation without circular dependency" {
    const allocator = std.testing.allocator;
    
    var program = Program.init(allocator);
    defer program.deinit();
    
    try std.testing.expect(program.statements.items.len == 0);
}

test "expression creation and cleanup" {
    const allocator = std.testing.allocator;
    
    const expr = try createIntegerExpression(allocator, 42);
    defer expr.deinit();
    
    switch (expr.kind) {
        .integer => |val| try std.testing.expect(val == 42),
        else => try std.testing.expect(false),
    }
}

test "binary expression creation" {
    const allocator = std.testing.allocator;
    
    const left = try createIntegerExpression(allocator, 1);
    const right = try createIntegerExpression(allocator, 2);
    const binary = try createBinaryExpression(allocator, left, "+", right);
    defer binary.deinit();
    
    switch (binary.kind) {
        .binary => |bin| {
            try std.testing.expect(std.mem.eql(u8, bin.operator, "+"));
            switch (bin.left.kind) {
                .integer => |val| try std.testing.expect(val == 1),
                else => try std.testing.expect(false),
            }
            switch (bin.right.kind) {
                .integer => |val| try std.testing.expect(val == 2),
                else => try std.testing.expect(false),
            }
        },
        else => try std.testing.expect(false),
    }
}

test "complex nested expressions" {
    const allocator = std.testing.allocator;
    
    // Create (1 + 2) * 3
    const left1 = try createIntegerExpression(allocator, 1);
    const right1 = try createIntegerExpression(allocator, 2);
    const add_expr = try createBinaryExpression(allocator, left1, "+", right1);
    
    const right2 = try createIntegerExpression(allocator, 3);
    const mul_expr = try createBinaryExpression(allocator, add_expr, "*", right2);
    defer mul_expr.deinit();
    
    // Verify structure
    switch (mul_expr.kind) {
        .binary => |bin| {
            try std.testing.expect(std.mem.eql(u8, bin.operator, "*"));
            switch (bin.left.kind) {
                .binary => |left_bin| try std.testing.expect(std.mem.eql(u8, left_bin.operator, "+")),
                else => try std.testing.expect(false),
            }
            switch (bin.right.kind) {
                .integer => |val| try std.testing.expect(val == 3),
                else => try std.testing.expect(false),
            }
        },
        else => try std.testing.expect(false),
    }
}
