const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const ast_types = @import("ast_types.zig");

// Fixed AST with proper type safety and no circular dependencies
// Uses forward declarations and proper casting to break cycles

pub const Expression = union(enum) {
    Identifier: []const u8,
    Variable: []const u8,
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    Character: u8,
    Binary: BinaryExpression,
    Call: CallExpression,
    MemberAccess: MemberAccessExpression,
    Literal: ast_types.Literal,
    Unary: UnaryExpression,
    Array: ArrayExpression,
    Map: MapExpression,
    CompositeLiteral: CompositeLiteralExpression,
    ChannelSend: ChannelSendExpression,
    ChannelReceive: ChannelReceiveExpression,
    ChannelCreation: ChannelCreationExpression,
    StructLiteral: StructLiteralExpression,
    Lambda: LambdaExpression,
    Tuple: TupleExpression,
    TupleAccess: TupleAccessExpression,
    ArrayAccess: ArrayAccessExpression,
    SliceAccess: SliceAccessExpression,
    TypeAssertion: TypeAssertionExpression,
    Increment: IncrementExpression,
    Decrement: DecrementExpression,
    Shook: ShookExpression,
    ErrorValue: ErrorValueExpression,
    StructuredError: StructuredErrorExpression,
    Panic: PanicExpression,
    Recover: RecoverExpression,
    TestResult: TestResultExpression,
    TestResultCheck: TestResultCheckExpression,
    RangeFor: RangeForExpression,
    Match: MatchExpression,
    TypeSwitch: TypeSwitchExpression,

    pub fn deinit(self: *Expression, allocator: Allocator) void {
        _ = allocator;
        switch (self.*) {
            .Binary => |*bin| bin.deinit(),
            .Call => |*call| call.deinit(),
            .Array => |*arr| arr.deinit(),
            .Map => |*map| map.deinit(),
            .CompositeLiteral => |*comp| comp.deinit(),
            .StructLiteral => |*struct_lit| struct_lit.deinit(),
            .Lambda => |*lambda| lambda.deinit(),
            
            .TupleAccess => |*access| access.deinit(),
            .ArrayAccess => |*access| access.deinit(),
            .SliceAccess => |*access| access.deinit(),
            .TypeAssertion => |*assert| assert.deinit(),
            .Increment => |*inc| inc.deinit(),
            .Decrement => |*dec| dec.deinit(),
            .Shook => |*shook| shook.deinit(),
            .StructuredError => |*err| err.deinit(),
            .Panic => |*panic| panic.deinit(),
            .TestResultCheck => |*check| check.deinit(),
            .RangeFor => |*range| range.deinit(),
            .Match => |*match| match.deinit(),
            .TypeSwitch => |*switch_expr| switch_expr.deinit(),
            .ChannelSend => |*send| send.deinit(),
            .ChannelReceive => |*recv| recv.deinit(),
            .ChannelCreation => |*create| create.deinit(),
            else => {}, // Simple types don't need cleanup
        }
    }

    pub fn print(self: Expression, indent: usize) !void {
        _ = indent; // Mark unused parameter
        const print_fn = std.debug.print;
        
        switch (self) {
            .Identifier => |id| print_fn("{s}", .{id}),
            .Variable => |var_name| print_fn("{s}", .{var_name}),
            .Integer => |int| print_fn("{}", .{int}),
            .Float => |float| print_fn("{d}", .{float}),
            .String => |str| print_fn("\"{s}\"", .{str}),
            .Boolean => |bool_val| print_fn("{}", .{bool_val}),
            .Character => |char| print_fn("'{c}'", .{char}),
            .Binary => |bin| {
                const left_expr: *Expression = @ptrCast(@alignCast(ast_types.expressionCast(bin.left)));
                const right_expr: *Expression = @ptrCast(@alignCast(ast_types.expressionCast(bin.right)));
                try left_expr.writer().print(0);
                print_fn(" {s} ", .{bin.operator});
                try right_expr.writer().print(0);
            },
            .Call => |call| {
                const func_expr: *Expression = @ptrCast(@alignCast(ast_types.expressionCast(call.function)));
                try func_expr.writer().print(0);
                print_fn("(", .{});
                for (call.arguments.items, 0..) |arg, i| {
                    if (i > 0) print_fn(", ", .{});
                    const arg_expr: *Expression = @ptrCast(@alignCast(ast_types.expressionCast(arg)));
                    try arg_expr.writer().print(0);
                }
                print_fn(")", .{});
            },
            else => print_fn("{s}", .{@tagName(self)}),
        }
    }
};

pub const Program = struct {
    statements: ArrayList(Statement),
    imports: ArrayList(ImportStatement),
    package: ?PackageDeclaration,

    pub fn init() Program {
        return Program{
            .statements = .empty,
            .imports = .empty,
            .package = null,
        };
    }

    pub fn deinit(self: *Program, allocator: Allocator) void {
        _ = allocator;
        for (self.statements.items) |*stmt| {
            stmt.deinit();
        }
        self.statements.deinit(self.allocator);
        
        for (self.imports.items) |*import| {
            import.deinit();
        }
        self.imports.deinit(self.allocator);
        
        if (self.package) |*pkg| {
            pkg.deinit();
        }
    }

    pub fn print(self: Program, indent: usize) !void {
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
            try stmt.writer().print(indent + 2);
        }
    }
};

pub const ImportStatement = struct {
    path: []const u8,
    alias: ?[]const u8,
    items: ArrayList([]const u8),

    pub fn init(allocator: Allocator, path: []const u8) ImportStatement {
        return ImportStatement{
            .path = path,
            .alias = null,
            .items = .empty,
        };
    }

    pub fn deinit(self: *ImportStatement, allocator: Allocator) void {
        _ = allocator;
                self.items.deinit(self.allocator);
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

pub const Type = union(enum) {
    Basic: ast_types.BasicType,
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
        _ = allocator;
        switch (self.*) {
            .Array => |*arr| arr.deinit(),
            .Map => |*map| map.deinit(),
            .Function => |*func| func.deinit(),
            
            .Channel => |*chan| chan.deinit(),
            .Slice => |*slice| slice.deinit(),
            .Pointer => |*ptr| ptr.deinit(),
            .Interface => |*interface| interface.deinit(),
            .Struct => |*struct_type| struct_type.deinit(),
            .Generic => |*generic| generic.deinit(),
            else => {},
        }
    }
};

pub const ChannelType = struct {
    element_type: ast_types.TypeRef,
    is_send_only: bool,
    is_receive_only: bool,

    pub fn deinit(self: *ChannelType, allocator: Allocator) void {
        _ = allocator;
        ast_types.typeCast(self.element_type).deinit();
        allocator.destroy(ast_types.typeCast(self.element_type));
    }
};

pub const ArrayType = struct {
    element_type: ast_types.TypeRef,
    size: ?usize,

    pub fn deinit(self: *ArrayType, allocator: Allocator) void {
        _ = allocator;
        ast_types.typeCast(self.element_type).deinit();
        allocator.destroy(ast_types.typeCast(self.element_type));
    }
};

pub const SliceType = struct {
    element_type: ast_types.TypeRef,

    pub fn deinit(self: *SliceType, allocator: Allocator) void {
        _ = allocator;
        ast_types.typeCast(self.element_type).deinit();
        allocator.destroy(ast_types.typeCast(self.element_type));
    }
};

pub const MapType = struct {
    key_type: ast_types.TypeRef,
    value_type: ast_types.TypeRef,

    pub fn deinit(self: *MapType, allocator: Allocator) void {
        _ = allocator;
        ast_types.typeCast(self.key_type).deinit();
        ast_types.typeCast(self.value_type).deinit();
        allocator.destroy(ast_types.typeCast(self.key_type));
        allocator.destroy(ast_types.typeCast(self.value_type));
    }
};

pub const PointerType = struct {
    target_type: ast_types.TypeRef,

    pub fn deinit(self: *PointerType, allocator: Allocator) void {
        _ = allocator;
        ast_types.typeCast(self.target_type).deinit();
        allocator.destroy(ast_types.typeCast(self.target_type));
    }
};

pub const FunctionType = struct {
    parameters: ArrayList(Type),
    return_type: ?ast_types.TypeRef,

    pub fn deinit(self: *FunctionType, allocator: Allocator) void {
        _ = allocator;
        for (self.parameters.items) |*param| {
            param.deinit();
        }
        self.parameters.deinit(self.allocator);
        
        if (self.return_type) |ret| {
            ast_types.typeCast(ret).deinit();
            allocator.destroy(ast_types.typeCast(ret));
        }
    }
};

pub const InterfaceType = struct {
    name: []const u8,
    methods: ArrayList(MethodSignature),

    pub fn deinit(self: *InterfaceType, allocator: Allocator) void {
        _ = allocator;
        for (self.methods.items) |*method| {
            method.deinit();
        }
        self.methods.deinit(self.allocator);
    }
};

pub const StructType = struct {
    name: []const u8,
    fields: ArrayList(StructField),

    pub fn deinit(self: *StructType, allocator: Allocator) void {
        _ = allocator;
        for (self.fields.items) |*field| {
            field.deinit();
        }
        self.fields.deinit(self.allocator);
    }
};

pub const GenericType = struct {
    name: []const u8,
    type_arguments: ArrayList(Type),
    constraints: ArrayList(TypeConstraint),

    pub fn deinit(self: *GenericType, allocator: Allocator) void {
        _ = allocator;
        for (self.type_arguments.items) |*type_arg| {
            type_arg.deinit();
        }
        self.type_arguments.deinit(self.allocator);
        
        for (self.constraints.items) |*constraint| {
            constraint.deinit();
        }
        self.constraints.deinit(self.allocator);
    }
};

/// Type constraints for generic parameters
pub const TypeConstraint = union(enum) {
    Interface: []const u8,
    Trait: []const u8,
    Numeric: void,
    Comparable: void,
    Ordered: void,
    Sized: void,
    Any: void,
    
    pub fn deinit(self: *TypeConstraint, allocator: Allocator) void {
        _ = allocator;
        _ = self;
                // No cleanup needed for simple constraints
    }
};

pub const TupleType = struct {
    elements: ArrayList(Type),

    pub fn deinit(self: *TupleType, allocator: Allocator) void {
        _ = allocator;
        for (self.elements.items) |*elem| {
            elem.deinit();
        }
        self.elements.deinit(self.allocator);
    }
};

pub const MethodSignature = struct {
    name: []const u8,
    parameters: ArrayList(Parameter),
    return_type: ?Type,

    pub fn deinit(self: *MethodSignature, allocator: Allocator) void {
        _ = allocator;
        for (self.parameters.items) |*param| {
            param.deinit();
        }
        self.parameters.deinit(self.allocator);
        
        if (self.return_type) |*ret_type| {
            ret_type.deinit();
        }
    }
};

pub const StructField = struct {
    name: []const u8,
    field_type: Type,
    visibility: ast_types.Visibility,

    pub fn deinit(self: *StructField, allocator: Allocator) void {
        _ = allocator;
        self.field_type.deinit(self.allocator);
    }
};

pub const Parameter = struct {
    name: []const u8,
    param_type: Type,
    is_mutable: bool,
    default_value: ?ast_types.ExpressionRef,

    pub fn deinit(self: *Parameter, allocator: Allocator) void {
        _ = allocator;
        self.param_type.deinit(self.allocator);
        if (self.default_value) |default| {
            ast_types.expressionCast(default).deinit();
            allocator.destroy(ast_types.expressionCast(default));
        }
    }
};

pub const TypeParameter = struct {
    name: []const u8,
    constraints: ArrayList(Type),

    pub fn deinit(self: *TypeParameter, allocator: Allocator) void {
        _ = allocator;
        for (self.constraints.items) |*constraint| {
            constraint.deinit();
        }
        self.constraints.deinit(self.allocator);
    }
};

pub const Statement = union(enum) {
    Expression: ast_types.ExpressionRef,
    Let: LetStatement,
    Assignment: AssignmentStatement,
    Return: ReturnStatement,
    If: IfStatement,
    Function: FunctionStatement,
    While: WhileStatement,
    For: ForStatement,
    ForIn: ForInStatement,
    Switch: SwitchStatement,
    PatternSwitch: PatternSwitchStatement,
    Goroutine: GoroutineStatement,
    Channel: ChannelStatement,
    Select: SelectStatement,
    Struct: StructStatement,
    Interface: InterfaceStatement,
    TypeAlias: TypeAliasStatement,
    Panic: PanicStatement,
    Catch: CatchStatement,
    Defer: DeferStatement,
    Break: BreakStatement,
    Continue: ContinueStatement,
    Increment: IncrementStatement,
    Decrement: DecrementStatement,
    ShortDeclaration: ShortDeclarationStatement,
    Yikes: YikesStatement,
    Fam: FamStatement,
    Const: ConstDecl,

    pub fn deinit(self: *Statement, allocator: Allocator) void {
        _ = allocator;
        switch (self.*) {
            .Expression => |expr_ref| {
                ast_types.expressionCast(expr_ref).deinit();
                allocator.destroy(ast_types.expressionCast(expr_ref));
            },
            .Let => |*let| let.deinit(),
            .Assignment => |*assign| assign.deinit(),
            .Return => |*ret| ret.deinit(),
            .If => |*if_stmt| if_stmt.deinit(),
            .Function => |*func| func.deinit(),
            .While => |*while_stmt| while_stmt.deinit(),
            .For => |*for_stmt| for_stmt.deinit(),
            .ForIn => |*for_in| for_in.deinit(),
            .Switch => |*switch_stmt| switch_stmt.deinit(),
            .PatternSwitch => |*pattern_switch| pattern_switch.deinit(),
            .Goroutine => |*goroutine| goroutine.deinit(),
            .Channel => |*channel| channel.deinit(),
            .Select => |*select| select.deinit(),
            .Struct => |*struct_stmt| struct_stmt.deinit(),
            .Interface => |*interface| interface.deinit(),
            .TypeAlias => |*type_alias| type_alias.deinit(),
            .Panic => |*panic| panic.deinit(),
            .Catch => |*catch_stmt| catch_stmt.deinit(),
            .Defer => |*defer_stmt| defer_stmt.deinit(),
            .Increment => |*inc| inc.deinit(),
            .Decrement => |*dec| dec.deinit(),
            .ShortDeclaration => |*short_decl| short_decl.deinit(),
            .Yikes => |*yikes| yikes.deinit(),
            .Fam => |*fam| fam.deinit(),
            .Const => |*const_decl| const_decl.deinit(),
            else => {}, // Simple statements don't need cleanup
        }
    }

    pub fn print(self: Statement, indent: usize) !void {
        const print_fn = std.debug.print;
        const spaces = "  " ** 10;
        
        switch (self) {
            .Expression => |expr_ref| {
                print_fn("{s}Expression: ", .{spaces[0..indent]});
                try ast_types.expressionCast(expr_ref).print(0);
                print_fn("\n", .{});
            },
            .Let => |let| {
                print_fn("{s}Let: {s} = ", .{ spaces[0..indent], let.name });
                if (let.initializer) |init| {
                    try ast_types.expressionCast(init).print(0);
                }
                print_fn("\n", .{});
            },
            .Function => |func| {
                print_fn("{s}Function: {s}\n", .{ spaces[0..indent], func.name });
                for (func.body.items) |stmt| {
                    try stmt.writer().print(indent + 2);
                }
            },
            else => {
                print_fn("{s}Statement: {s}\n", .{ spaces[0..indent], @tagName(self) });
            },
        }
    }
};

// Expression structures using forward references
pub const BinaryExpression = struct {
    left: ast_types.ExpressionRef,
    operator: []const u8,
    right: ast_types.ExpressionRef,

    pub fn deinit(self: *BinaryExpression, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.left).deinit();
        ast_types.expressionCast(self.right).deinit();
        allocator.destroy(ast_types.expressionCast(self.left));
        allocator.destroy(ast_types.expressionCast(self.right));
    }
};

pub const CallExpression = struct {
    function: ast_types.ExpressionRef,
    arguments: ArrayList(ast_types.ExpressionRef),

    pub fn deinit(self: *CallExpression, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.function).deinit();
        allocator.destroy(ast_types.expressionCast(self.function));
        
        for (self.arguments.items) |arg| {
            ast_types.expressionCast(arg).deinit();
            allocator.destroy(ast_types.expressionCast(arg));
        }
        self.arguments.deinit(self.allocator);
    }
};

pub const MemberAccessExpression = struct {
    object: ast_types.ExpressionRef,
    property: []const u8,

    pub fn deinit(self: *MemberAccessExpression, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.object).deinit();
        allocator.destroy(ast_types.expressionCast(self.object));
    }
};

pub const UnaryExpression = struct {
    operator: []const u8,
    operand: ast_types.ExpressionRef,

    pub fn deinit(self: *UnaryExpression, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.operand).deinit();
        allocator.destroy(ast_types.expressionCast(self.operand));
    }
};

pub const ArrayExpression = struct {
    elements: ArrayList(ast_types.ExpressionRef),

    pub fn deinit(self: *ArrayExpression, allocator: Allocator) void {
        _ = allocator;
        for (self.elements.items) |elem| {
            ast_types.expressionCast(elem).deinit();
            allocator.destroy(ast_types.expressionCast(elem));
        }
        self.elements.deinit(self.allocator);
    }
};

pub const MapExpression = struct {
    entries: ArrayList(MapEntry),

    pub fn deinit(self: *MapExpression, allocator: Allocator) void {
        _ = allocator;
        for (self.entries.items) |*entry| {
            entry.deinit();
        }
        self.entries.deinit(self.allocator);
    }
};

pub const MapEntry = struct {
    key: ast_types.ExpressionRef,
    value: ast_types.ExpressionRef,

    pub fn deinit(self: *MapEntry, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.key).deinit();
        ast_types.expressionCast(self.value).deinit();
        allocator.destroy(ast_types.expressionCast(self.key));
        allocator.destroy(ast_types.expressionCast(self.value));
    }
};

pub const CompositeLiteralExpression = struct {
    type_name: []const u8,
    elements: ArrayList(ast_types.ExpressionRef),

    pub fn deinit(self: *CompositeLiteralExpression, allocator: Allocator) void {
        _ = allocator;
        for (self.elements.items) |elem| {
            ast_types.expressionCast(elem).deinit();
            allocator.destroy(ast_types.expressionCast(elem));
        }
        self.elements.deinit(self.allocator);
    }
};

pub const ChannelSendExpression = struct {
    channel: ast_types.ExpressionRef,
    value: ast_types.ExpressionRef,

    pub fn deinit(self: *ChannelSendExpression, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.channel).deinit();
        ast_types.expressionCast(self.value).deinit();
        allocator.destroy(ast_types.expressionCast(self.channel));
        allocator.destroy(ast_types.expressionCast(self.value));
    }
};

pub const ChannelReceiveExpression = struct {
    channel: ast_types.ExpressionRef,

    pub fn deinit(self: *ChannelReceiveExpression, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.channel).deinit();
        allocator.destroy(ast_types.expressionCast(self.channel));
    }
};

pub const ChannelCreationExpression = struct {
    element_type: Type,
    buffer_size: ?ast_types.ExpressionRef,

    pub fn deinit(self: *ChannelCreationExpression, allocator: Allocator) void {
        _ = allocator;
        self.element_type.deinit(self.allocator);
        if (self.buffer_size) |buffer| {
            ast_types.expressionCast(buffer).deinit();
            allocator.destroy(ast_types.expressionCast(buffer));
        }
    }
};

pub const StructLiteralExpression = struct {
    struct_name: []const u8,
    fields: ArrayList(StructFieldAssignment),

    pub fn deinit(self: *StructLiteralExpression, allocator: Allocator) void {
        _ = allocator;
        for (self.fields.items) |*field| {
            field.deinit();
        }
        self.fields.deinit(self.allocator);
    }
};

pub const StructFieldAssignment = struct {
    field_name: []const u8,
    value: ast_types.ExpressionRef,

    pub fn deinit(self: *StructFieldAssignment, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.value).deinit();
        allocator.destroy(ast_types.expressionCast(self.value));
    }
};

pub const LambdaExpression = struct {
    parameters: ArrayList([]const u8),
    body: ast_types.ExpressionRef,

    pub fn deinit(self: *LambdaExpression, allocator: Allocator) void {
        _ = allocator;
        self.parameters.deinit(self.allocator);
        ast_types.expressionCast(self.body).deinit();
        allocator.destroy(ast_types.expressionCast(self.body));
    }
};

pub const TupleExpression = struct {
    elements: ArrayList(ast_types.ExpressionRef),

    pub fn deinit(self: *TupleExpression, allocator: Allocator) void {
        _ = allocator;
        for (self.elements.items) |elem| {
            ast_types.expressionCast(elem).deinit();
            allocator.destroy(ast_types.expressionCast(elem));
        }
        self.elements.deinit(self.allocator);
    }
};

pub const TupleAccessExpression = struct {
    tuple: ast_types.ExpressionRef,
    index: usize,

    pub fn deinit(self: *TupleAccessExpression, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.tuple).deinit();
        allocator.destroy(ast_types.expressionCast(self.tuple));
    }
};

pub const ArrayAccessExpression = struct {
    array: ast_types.ExpressionRef,
    index: ast_types.ExpressionRef,

    pub fn deinit(self: *ArrayAccessExpression, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.array).deinit();
        ast_types.expressionCast(self.index).deinit();
        allocator.destroy(ast_types.expressionCast(self.array));
        allocator.destroy(ast_types.expressionCast(self.index));
    }
};

pub const SliceAccessExpression = struct {
    array: ast_types.ExpressionRef,
    start: ?ast_types.ExpressionRef,
    end: ?ast_types.ExpressionRef,

    pub fn deinit(self: *SliceAccessExpression, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.array).deinit();
        allocator.destroy(ast_types.expressionCast(self.array));
        
        if (self.start) |start| {
            ast_types.expressionCast(start).deinit();
            allocator.destroy(ast_types.expressionCast(start));
        }
        
        if (self.end) |end| {
            ast_types.expressionCast(end).deinit();
            allocator.destroy(ast_types.expressionCast(end));
        }
    }
};

pub const TypeAssertionExpression = struct {
    value: ast_types.ExpressionRef,
    target_type: Type,
    is_safe: bool,

    pub fn deinit(self: *TypeAssertionExpression, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.value).deinit();
        allocator.destroy(ast_types.expressionCast(self.value));
        self.target_type.deinit(self.allocator);
    }
};

pub const IncrementExpression = struct {
    variable: ast_types.ExpressionRef,

    pub fn deinit(self: *IncrementExpression, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.variable).deinit();
        allocator.destroy(ast_types.expressionCast(self.variable));
    }
};

pub const DecrementExpression = struct {
    variable: ast_types.ExpressionRef,

    pub fn deinit(self: *DecrementExpression, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.variable).deinit();
        allocator.destroy(ast_types.expressionCast(self.variable));
    }
};

pub const ShookExpression = struct {
    expression: ast_types.ExpressionRef,

    pub fn deinit(self: *ShookExpression, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.expression).deinit();
        allocator.destroy(ast_types.expressionCast(self.expression));
    }
};

pub const ErrorValueExpression = struct {
    message: []const u8,

    pub fn deinit(self: *ErrorValueExpression, allocator: Allocator) void {
        _ = allocator;
        _ = self;
            }
};

pub const StructuredErrorExpression = struct {
    message: ast_types.ExpressionRef,
    code: ?ast_types.ExpressionRef,
    details: ?ast_types.ExpressionRef,
    fields: ArrayList(StructuredErrorField),

    pub fn deinit(self: *StructuredErrorExpression, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.message).deinit();
        allocator.destroy(ast_types.expressionCast(self.message));
        
        if (self.code) |code| {
            ast_types.expressionCast(code).deinit();
            allocator.destroy(ast_types.expressionCast(code));
        }
        
        if (self.details) |details| {
            ast_types.expressionCast(details).deinit();
            allocator.destroy(ast_types.expressionCast(details));
        }
        
        for (self.fields.items) |*field| {
            field.deinit();
        }
        self.fields.deinit(self.allocator);
    }
};

pub const StructuredErrorField = struct {
    name: []const u8,
    value: ast_types.ExpressionRef,

    pub fn deinit(self: *StructuredErrorField, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.value).deinit();
        allocator.destroy(ast_types.expressionCast(self.value));
    }
};

pub const PanicExpression = struct {
    message: ast_types.ExpressionRef,

    pub fn deinit(self: *PanicExpression, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.message).deinit();
        allocator.destroy(ast_types.expressionCast(self.message));
    }
};

pub const RecoverExpression = struct {
    pub fn deinit(self: *RecoverExpression, allocator: Allocator) void {
        _ = allocator;
        _ = self;
            }
};

pub const TestResultExpression = struct {
    test_name: []const u8,
    is_passing: bool,

    pub fn deinit(self: *TestResultExpression, allocator: Allocator) void {
        _ = allocator;
        _ = self;
            }
};

pub const TestResultCheckExpression = struct {
    expression: ast_types.ExpressionRef,

    pub fn deinit(self: *TestResultCheckExpression, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.expression).deinit();
        allocator.destroy(ast_types.expressionCast(self.expression));
    }
};

// Additional declarations for generics support
pub const InterfaceDeclaration = struct {
    name: []const u8,
    methods: ArrayList(InterfaceMethod),
    type_parameters: ArrayList(TypeParameter),

    pub fn deinit(self: *InterfaceDeclaration, allocator: Allocator) void {
        _ = allocator;
        for (self.methods.items) |*method| {
            method.deinit();
        }
        self.methods.deinit(self.allocator);
        for (self.type_parameters.items) |*param| {
            param.deinit();
        }
        self.type_parameters.deinit(self.allocator);
    }
};

pub const InterfaceMethod = struct {
    name: []const u8,
    parameters: ArrayList(Parameter),
    return_type: ?Type,

    pub fn deinit(self: *InterfaceMethod, allocator: Allocator) void {
        _ = allocator;
        for (self.parameters.items) |*param| {
            param.param_type.deinit();
        }
        self.parameters.deinit(self.allocator);
        if (self.return_type) |*ret_type| {
            ret_type.deinit();
        }
    }
};

pub const RangeForExpression = struct {
    iterable: ast_types.ExpressionRef,

    pub fn deinit(self: *RangeForExpression, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.iterable).deinit();
        allocator.destroy(ast_types.expressionCast(self.iterable));
    }
};

pub const MatchExpression = struct {
    expression: ast_types.ExpressionRef,
    cases: ArrayList(MatchCase),
    default_case: ?ast_types.ExpressionRef,

    pub fn deinit(self: *MatchExpression, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.expression).deinit();
        allocator.destroy(ast_types.expressionCast(self.expression));
        
        for (self.cases.items) |*case| {
            case.deinit();
        }
        self.cases.deinit(self.allocator);
        
        if (self.default_case) |default| {
            ast_types.expressionCast(default).deinit();
            allocator.destroy(ast_types.expressionCast(default));
        }
    }
};

pub const TypeSwitchExpression = struct {
    expression: ast_types.ExpressionRef,
    cases: ArrayList(TypeCase),
    default_case: ?ast_types.ExpressionRef,

    pub fn deinit(self: *TypeSwitchExpression, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.expression).deinit();
        allocator.destroy(ast_types.expressionCast(self.expression));
        
        for (self.cases.items) |*case| {
            case.deinit();
        }
        self.cases.deinit(self.allocator);
        
        if (self.default_case) |default| {
            ast_types.expressionCast(default).deinit();
            allocator.destroy(ast_types.expressionCast(default));
        }
    }
};

// Statement structures
pub const LetStatement = struct {
    name: []const u8,
    var_type: ?Type,
    initializer: ?ast_types.ExpressionRef,
    is_mutable: bool,

    pub fn deinit(self: *LetStatement, allocator: Allocator) void {
        _ = allocator;
        if (self.var_type) |*var_type| {
            var_type.deinit();
        }
        if (self.initializer) |init| {
            ast_types.expressionCast(init).deinit();
            allocator.destroy(ast_types.expressionCast(init));
        }
    }
};

pub const AssignmentStatement = struct {
    target: ast_types.ExpressionRef,
    value: ast_types.ExpressionRef,
    operator: []const u8,

    pub fn deinit(self: *AssignmentStatement, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.target).deinit();
        ast_types.expressionCast(self.value).deinit();
        allocator.destroy(ast_types.expressionCast(self.target));
        allocator.destroy(ast_types.expressionCast(self.value));
    }
};

pub const ReturnStatement = struct {
    value: ?ast_types.ExpressionRef,

    pub fn deinit(self: *ReturnStatement, allocator: Allocator) void {
        _ = allocator;
        if (self.value) |val| {
            ast_types.expressionCast(val).deinit();
            allocator.destroy(ast_types.expressionCast(val));
        }
    }
};

pub const IfStatement = struct {
    condition: ast_types.ExpressionRef,
    then_branch: ArrayList(Statement),
    else_branch: ?ArrayList(Statement),

    pub fn deinit(self: *IfStatement, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.condition).deinit();
        allocator.destroy(ast_types.expressionCast(self.condition));
        
        for (self.then_branch.items) |*stmt| {
            stmt.deinit();
        }
        self.then_branch.deinit(self.allocator);
        
        if (self.else_branch) |*else_br| {
            for (else_br.items) |*stmt| {
                stmt.deinit();
            }
            else_br.deinit();
        }
    }
};

pub const FunctionStatement = struct {
    name: []const u8,
    parameters: ArrayList(Parameter),
    return_type: ?Type,
    body: ArrayList(Statement),
    visibility: ast_types.Visibility,
    is_async: bool,
    type_parameters: ArrayList(TypeParameter),
    comments: ArrayList(ast_types.Comment),

    pub fn init(allocator: Allocator, name: []const u8) FunctionStatement {
        return FunctionStatement{
            .name = name,
            .parameters = .empty,
            .return_type = null,
            .body = .empty,
            .visibility = .Private,
            .is_async = false,
            .type_parameters = .empty,
            .comments = .empty,
        };
    }

    pub fn deinit(self: *FunctionStatement, allocator: Allocator) void {
        _ = allocator;
        for (self.parameters.items) |*param| {
            param.deinit();
        }
        self.parameters.deinit(self.allocator);
        
        if (self.return_type) |*ret_type| {
            ret_type.deinit();
        }
        
        for (self.body.items) |*stmt| {
            stmt.deinit();
        }
        self.body.deinit(self.allocator);
        
        for (self.type_parameters.items) |*type_param| {
            type_param.deinit();
        }
        self.type_parameters.deinit(self.allocator);
        
        self.comments.deinit(self.allocator);
    }
};

pub const WhileStatement = struct {
    condition: ast_types.ExpressionRef,
    body: ArrayList(Statement),

    pub fn deinit(self: *WhileStatement, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.condition).deinit();
        allocator.destroy(ast_types.expressionCast(self.condition));
        
        for (self.body.items) |*stmt| {
            stmt.deinit();
        }
        self.body.deinit(self.allocator);
    }
};

pub const ForStatement = struct {
    init: ?ast_types.StatementRef,
    condition: ?ast_types.ExpressionRef,
    update: ?ast_types.StatementRef,
    body: ArrayList(Statement),

    pub fn deinit(self: *ForStatement, allocator: Allocator) void {
        _ = allocator;
        if (self.init) |init| {
            ast_types.statementCast(init).deinit();
            allocator.destroy(ast_types.statementCast(init));
        }
        
        if (self.condition) |cond| {
            ast_types.expressionCast(cond).deinit();
            allocator.destroy(ast_types.expressionCast(cond));
        }
        
        if (self.update) |update| {
            ast_types.statementCast(update).deinit();
            allocator.destroy(ast_types.statementCast(update));
        }
        
        for (self.body.items) |*stmt| {
            stmt.deinit();
        }
        self.body.deinit(self.allocator);
    }
};

pub const ForInStatement = struct {
    variable: []const u8,
    iterable: ast_types.ExpressionRef,
    body: ArrayList(Statement),

    pub fn deinit(self: *ForInStatement, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.iterable).deinit();
        allocator.destroy(ast_types.expressionCast(self.iterable));
        
        for (self.body.items) |*stmt| {
            stmt.deinit();
        }
        self.body.deinit(self.allocator);
    }
};

pub const SwitchStatement = struct {
    expression: ast_types.ExpressionRef,
    cases: ArrayList(SwitchCase),
    default_case: ?ArrayList(Statement),

    pub fn deinit(self: *SwitchStatement, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.expression).deinit();
        allocator.destroy(ast_types.expressionCast(self.expression));
        
        for (self.cases.items) |*case| {
            case.deinit();
        }
        self.cases.deinit(self.allocator);
        
        if (self.default_case) |*default| {
            for (default.items) |*stmt| {
                stmt.deinit();
            }
            default.deinit();
        }
    }
};

pub const PatternSwitchStatement = struct {
    expression: ast_types.ExpressionRef,
    patterns: ArrayList(PatternCase),
    default_case: ?ArrayList(Statement),

    pub fn deinit(self: *PatternSwitchStatement, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.expression).deinit();
        allocator.destroy(ast_types.expressionCast(self.expression));
        
        for (self.patterns.items) |*pattern| {
            pattern.deinit();
        }
        self.patterns.deinit(self.allocator);
        
        if (self.default_case) |*default| {
            for (default.items) |*stmt| {
                stmt.deinit();
            }
            default.deinit();
        }
    }
};

pub const GoroutineStatement = struct {
    call: CallExpression,

    pub fn deinit(self: *GoroutineStatement, allocator: Allocator) void {
        _ = allocator;
        self.call.deinit(self.allocator);
    }
};

pub const ChannelStatement = struct {
    name: []const u8,
    channel_type: Type,
    buffer_size: ?ast_types.ExpressionRef,

    pub fn deinit(self: *ChannelStatement, allocator: Allocator) void {
        _ = allocator;
        self.channel_type.deinit(self.allocator);
        if (self.buffer_size) |buffer| {
            ast_types.expressionCast(buffer).deinit();
            allocator.destroy(ast_types.expressionCast(buffer));
        }
    }
};

pub const SelectStatement = struct {
    cases: ArrayList(SelectCase),
    default_case: ?ArrayList(Statement),

    pub fn deinit(self: *SelectStatement, allocator: Allocator) void {
        _ = allocator;
        for (self.cases.items) |*case| {
            case.deinit();
        }
        self.cases.deinit(self.allocator);
        
        if (self.default_case) |*default| {
            for (default.items) |*stmt| {
                stmt.deinit();
            }
            default.deinit();
        }
    }
};

pub const StructStatement = struct {
    name: []const u8,
    fields: ArrayList(StructField),
    visibility: ast_types.Visibility,
    type_parameters: ArrayList(TypeParameter),

    pub fn deinit(self: *StructStatement, allocator: Allocator) void {
        _ = allocator;
        for (self.fields.items) |*field| {
            field.deinit();
        }
        self.fields.deinit(self.allocator);
        
        for (self.type_parameters.items) |*type_param| {
            type_param.deinit();
        }
        self.type_parameters.deinit(self.allocator);
    }
};

pub const InterfaceStatement = struct {
    name: []const u8,
    methods: ArrayList(MethodSignature),
    visibility: ast_types.Visibility,
    type_parameters: ArrayList(TypeParameter),

    pub fn deinit(self: *InterfaceStatement, allocator: Allocator) void {
        _ = allocator;
        for (self.methods.items) |*method| {
            method.deinit();
        }
        self.methods.deinit(self.allocator);
        
        for (self.type_parameters.items) |*type_param| {
            type_param.deinit();
        }
        self.type_parameters.deinit(self.allocator);
    }
};

pub const TypeAliasStatement = struct {
    name: []const u8,
    target_type: Type,
    visibility: ast_types.Visibility,

    pub fn deinit(self: *TypeAliasStatement, allocator: Allocator) void {
        _ = allocator;
        self.target_type.deinit(self.allocator);
    }
};

pub const PanicStatement = struct {
    message: ast_types.ExpressionRef,

    pub fn deinit(self: *PanicStatement, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.message).deinit();
        allocator.destroy(ast_types.expressionCast(self.message));
    }
};

pub const CatchStatement = struct {
    body: ArrayList(Statement),
    error_variable: ?[]const u8,
    error_type: ?Type,

    pub fn deinit(self: *CatchStatement, allocator: Allocator) void {
        _ = allocator;
        for (self.body.items) |*stmt| {
            stmt.deinit();
        }
        self.body.deinit(self.allocator);
        
        if (self.error_type) |*err_type| {
            err_type.deinit();
        }
    }
};

pub const DeferStatement = struct {
    statement: ast_types.StatementRef,

    pub fn deinit(self: *DeferStatement, allocator: Allocator) void {
        _ = allocator;
        ast_types.statementCast(self.statement).deinit();
        allocator.destroy(ast_types.statementCast(self.statement));
    }
};

pub const BreakStatement = struct {
    pub fn deinit(self: *BreakStatement, allocator: Allocator) void {
        _ = allocator;
        _ = self;
            }
};

pub const ContinueStatement = struct {
    pub fn deinit(self: *ContinueStatement, allocator: Allocator) void {
        _ = allocator;
        _ = self;
            }
};

pub const IncrementStatement = struct {
    variable: ast_types.ExpressionRef,

    pub fn deinit(self: *IncrementStatement, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.variable).deinit();
        allocator.destroy(ast_types.expressionCast(self.variable));
    }
};

pub const DecrementStatement = struct {
    variable: ast_types.ExpressionRef,

    pub fn deinit(self: *DecrementStatement, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.variable).deinit();
        allocator.destroy(ast_types.expressionCast(self.variable));
    }
};

pub const ShortDeclarationStatement = struct {
    names: ArrayList([]const u8),
    values: ArrayList(ast_types.ExpressionRef),

    pub fn deinit(self: *ShortDeclarationStatement, allocator: Allocator) void {
        _ = allocator;
        self.names.deinit(self.allocator);
        
        for (self.values.items) |val| {
            ast_types.expressionCast(val).deinit();
            allocator.destroy(ast_types.expressionCast(val));
        }
        self.values.deinit(self.allocator);
    }
};

pub const YikesStatement = struct {
    name: []const u8,
    error_type: ?Type,
    value: ?ast_types.ExpressionRef,

    pub fn deinit(self: *YikesStatement, allocator: Allocator) void {
        _ = allocator;
        if (self.error_type) |*err_type| {
            err_type.deinit();
        }
        
        if (self.value) |val| {
            ast_types.expressionCast(val).deinit();
            allocator.destroy(ast_types.expressionCast(val));
        }
    }
};

pub const FamStatement = struct {
    body: ArrayList(Statement),
    recovery_body: ?ArrayList(Statement),
    error_variable: ?[]const u8,

    pub fn deinit(self: *FamStatement, allocator: Allocator) void {
        _ = allocator;
        for (self.body.items) |*stmt| {
            stmt.deinit();
        }
        self.body.deinit(self.allocator);
        
        if (self.recovery_body) |*recovery| {
            for (recovery.items) |*stmt| {
                stmt.deinit();
            }
            recovery.deinit();
        }
    }
};

pub const ConstDecl = struct {
    name: []const u8,
    const_type: ?Type,
    value: ast_types.ExpressionRef,
    visibility: ast_types.Visibility,

    pub fn deinit(self: *ConstDecl, allocator: Allocator) void {
        _ = allocator;
        if (self.const_type) |*const_type| {
            const_type.deinit();
        }
        
        ast_types.expressionCast(self.value).deinit();
        allocator.destroy(ast_types.expressionCast(self.value));
    }
};

// Support structures
pub const SwitchCase = struct {
    value: ast_types.ExpressionRef,
    body: ArrayList(Statement),

    pub fn deinit(self: *SwitchCase, allocator: Allocator) void {
        _ = allocator;
        ast_types.expressionCast(self.value).deinit();
        allocator.destroy(ast_types.expressionCast(self.value));
        
        for (self.body.items) |*stmt| {
            stmt.deinit();
        }
        self.body.deinit(self.allocator);
    }
};

pub const PatternCase = struct {
    pattern: Pattern,
    guard: ?ast_types.ExpressionRef,
    body: ArrayList(Statement),

    pub fn deinit(self: *PatternCase, allocator: Allocator) void {
        _ = allocator;
        self.pattern.deinit(self.allocator);
        
        if (self.guard) |guard| {
            ast_types.expressionCast(guard).deinit();
            allocator.destroy(ast_types.expressionCast(guard));
        }
        
        for (self.body.items) |*stmt| {
            stmt.deinit();
        }
        self.body.deinit(self.allocator);
    }
};

pub const SelectCase = struct {
    channel_op: ChannelOperation,
    body: ArrayList(Statement),

    pub fn deinit(self: *SelectCase, allocator: Allocator) void {
        _ = allocator;
        self.channel_op.deinit(self.allocator);
        
        for (self.body.items) |*stmt| {
            stmt.deinit();
        }
        self.body.deinit(self.allocator);
    }
};

pub const ChannelOperation = union(enum) {
    Send: struct {
        channel: ast_types.ExpressionRef,
        value: ast_types.ExpressionRef,
    },
    Receive: struct {
        channel: ast_types.ExpressionRef,
        variable: ?[]const u8,
    },

    pub fn deinit(self: *ChannelOperation, allocator: Allocator) void {
        _ = allocator;
        switch (self.*) {
            .Send => |*send| {
                ast_types.expressionCast(send.channel).deinit();
                ast_types.expressionCast(send.value).deinit();
                allocator.destroy(ast_types.expressionCast(send.channel));
                allocator.destroy(ast_types.expressionCast(send.value));
            },
            .Receive => |*recv| {
                ast_types.expressionCast(recv.channel).deinit();
                allocator.destroy(ast_types.expressionCast(recv.channel));
            },
        }
    }
};

pub const MatchCase = struct {
    pattern: Pattern,
    guard: ?ast_types.ExpressionRef,
    result: ast_types.ExpressionRef,

    pub fn deinit(self: *MatchCase, allocator: Allocator) void {
        _ = allocator;
        self.pattern.deinit(self.allocator);
        
        if (self.guard) |guard| {
            ast_types.expressionCast(guard).deinit();
            allocator.destroy(ast_types.expressionCast(guard));
        }
        
        ast_types.expressionCast(self.result).deinit();
        allocator.destroy(ast_types.expressionCast(self.result));
    }
};

pub const TypeCase = struct {
    type_pattern: Type,
    variable: ?[]const u8,
    result: ast_types.ExpressionRef,

    pub fn deinit(self: *TypeCase, allocator: Allocator) void {
        _ = allocator;
        self.type_pattern.deinit(self.allocator);
        ast_types.expressionCast(self.result).deinit();
        allocator.destroy(ast_types.expressionCast(self.result));
    }
};

pub const Pattern = union(enum) {
    Wildcard,
    Literal: ast_types.Literal,
    Variable: []const u8,
    Tuple: ArrayList(Pattern),
    Struct: StructPattern,
    Array: ArrayList(Pattern),

    pub fn deinit(self: *Pattern, allocator: Allocator) void {
        _ = allocator;
        switch (self.*) {
            }
                tuple.deinit();
            },
            .Struct => |*struct_pattern| {
                struct_pattern.deinit();
            },
            .Array => |*array| {
                for (array.items) |*pattern| {
                    pattern.deinit();
                }
                array.deinit();
            },
            else => {},
        }
    }
};

pub const StructPattern = struct {
    name: []const u8,
    fields: ArrayList(FieldPattern),

    pub fn deinit(self: *StructPattern, allocator: Allocator) void {
        _ = allocator;
        for (self.fields.items) |*field| {
            field.deinit();
        }
        self.fields.deinit(self.allocator);
    }
};

pub const FieldPattern = struct {
    name: []const u8,
    pattern: Pattern,

    pub fn deinit(self: *FieldPattern, allocator: Allocator) void {
        _ = allocator;
        self.pattern.deinit(self.allocator);
    }
};

// Tests
test "ast creation" {
    const allocator = std.testing.allocator;
    
    var program = Program.init(allocator);
    defer program.deinit();
    
    try std.testing.expect(program.statements.items.len == 0);
}

test "function statement" {
    const allocator = std.testing.allocator;
    
    var func = FunctionStatement.init(allocator, "test_function");
    defer func.deinit();
    
    try std.testing.expect(std.mem.eql(u8, func.name, "test_function"));
    try std.testing.expect(func.parameters.items.len == 0);
}

test "expression cleanup" {
    const allocator = std.testing.allocator;
    
    // Test that we can create and cleanup expressions without circular dependency issues
    var expr = Expression{ .Integer = 42 };
    defer expr.deinit();
    
    try std.testing.expect(expr == .Integer);
}
