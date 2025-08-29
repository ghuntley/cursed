const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Simple, clean AST without circular dependencies
// Uses index-based references instead of pointers to break cycles

pub const NodeIndex = u32;
pub const INVALID_NODE: NodeIndex = std.math.maxInt(NodeIndex);

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

pub const Literal = union(enum) {
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    Character: u8,
    Null,
};

// Expression types - use indices instead of pointers
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
    Literal: Literal,
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

    pub fn deinit(self: *Expression, allocator: Allocator, ast: *AST) void {
        switch (self.*) {
            .Binary => |*bin| bin.deinit(allocator, ast),
            .Call => |*call| call.deinit(allocator, ast),
            .Array => |*arr| arr.deinit(allocator, ast),
            .Map => |*map| map.deinit(allocator, ast),
            .CompositeLiteral => |*comp| comp.deinit(allocator, ast),
            .StructLiteral => |*struct_lit| struct_lit.deinit(allocator, ast),
            .Lambda => |*lambda| lambda.deinit(allocator, ast),
             ast),
            .TupleAccess => |*access| access.deinit(allocator, ast),
            .ArrayAccess => |*access| access.deinit(allocator, ast),
            .SliceAccess => |*access| access.deinit(allocator, ast),
            .TypeAssertion => |*assert| assert.deinit(allocator, ast),
            .Increment => |*inc| inc.deinit(allocator, ast),
            .Decrement => |*dec| dec.deinit(allocator, ast),
            .Shook => |*shook| shook.deinit(allocator, ast),
            .StructuredError => |*err| err.deinit(allocator, ast),
            .Panic => |*panic| panic.deinit(allocator, ast),
            .TestResultCheck => |*check| check.deinit(allocator, ast),
            .RangeFor => |*range| range.deinit(allocator, ast),
            .Match => |*match| match.deinit(allocator, ast),
            .TypeSwitch => |*switch_expr| switch_expr.deinit(allocator, ast),
            .ChannelSend => |*send| send.deinit(allocator, ast),
            .ChannelReceive => |*recv| recv.deinit(allocator, ast),
            .ChannelCreation => |*create| create.deinit(allocator, ast),
            else => {}, // Simple types don't need cleanup
        }
    }

    pub fn print(self: Expression, indent: usize, ast: *AST) !void {
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
                if (bin.left != INVALID_NODE) {
                    try ast.expressions.items[bin.left].print(0, ast);
                }
                print_fn(" {s} ", .{bin.operator});
                if (bin.right != INVALID_NODE) {
                    try ast.expressions.items[bin.right].print(0, ast);
                }
            },
            .Call => |call| {
                if (call.function != INVALID_NODE) {
                    try ast.expressions.items[call.function].print(0, ast);
                }
                print_fn("(", .{});
                for (call.arguments.items, 0..) |arg, i| {
                    if (i > 0) print_fn(", ", .{});
                    if (arg != INVALID_NODE) {
                        try ast.expressions.items[arg].print(0, ast);
                    }
                }
                print_fn(")", .{});
            },
            else => print_fn("{s}", .{@tagName(self)}),
        }
    }
};

pub const Statement = union(enum) {
    Expression: NodeIndex,
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

    pub fn deinit(self: *Statement, allocator: Allocator, ast: *AST) void {
        switch (self.*) {
            .Let => |*let| let.deinit(allocator, ast),
            .Assignment => |*assign| assign.deinit(allocator, ast),
            .Return => |*ret| ret.deinit(allocator, ast),
            .If => |*if_stmt| if_stmt.deinit(allocator, ast),
            .Function => |*func| func.deinit(allocator, ast),
            .While => |*while_stmt| while_stmt.deinit(allocator, ast),
            .For => |*for_stmt| for_stmt.deinit(allocator, ast),
            .ForIn => |*for_in| for_in.deinit(allocator, ast),
            .Switch => |*switch_stmt| switch_stmt.deinit(allocator, ast),
            .PatternSwitch => |*pattern_switch| pattern_switch.deinit(allocator, ast),
            .Goroutine => |*goroutine| goroutine.deinit(allocator, ast),
            .Channel => |*channel| channel.deinit(allocator, ast),
            .Select => |*select| select.deinit(allocator, ast),
            .Struct => |*struct_stmt| struct_stmt.deinit(allocator, ast),
            .Interface => |*interface| interface.deinit(allocator, ast),
            .TypeAlias => |*type_alias| type_alias.deinit(allocator, ast),
            .Panic => |*panic| panic.deinit(allocator, ast),
            .Catch => |*catch_stmt| catch_stmt.deinit(allocator, ast),
            .Defer => |*defer_stmt| defer_stmt.deinit(allocator, ast),
            .Increment => |*inc| inc.deinit(allocator, ast),
            .Decrement => |*dec| dec.deinit(allocator, ast),
            .ShortDeclaration => |*short_decl| short_decl.deinit(allocator, ast),
            .Yikes => |*yikes| yikes.deinit(allocator, ast),
            .Fam => |*fam| fam.deinit(allocator, ast),
            .Const => |*const_decl| const_decl.deinit(allocator, ast),
            else => {}, // Simple statements don't need cleanup
        }
    }

    pub fn print(self: Statement, indent: usize, ast: *AST) !void {
        const print_fn = std.debug.print;
        const spaces = "  " ** 10;
        
        switch (self) {
            .Expression => |expr_idx| {
                print_fn("{s}Expression: ", .{spaces[0..indent]});
                if (expr_idx != INVALID_NODE) {
                    try ast.expressions.items[expr_idx].print(0, ast);
                }
                print_fn("\n", .{});
            },
            .Let => |let| {
                print_fn("{s}Let: {s} = ", .{ spaces[0..indent], let.name });
                if (let.initializer != INVALID_NODE) {
                    try ast.expressions.items[let.initializer].print(0, ast);
                }
                print_fn("\n", .{});
            },
            .Function => |func| {
                print_fn("{s}Function: {s}\n", .{ spaces[0..indent], func.name });
                for (func.body.items) |stmt_idx| {
                    if (stmt_idx != INVALID_NODE) {
                        try ast.statements.items[stmt_idx].print(indent + 2, ast);
                    }
                }
            },
            else => {
                print_fn("{s}Statement: {s}\n", .{ spaces[0..indent], @tagName(self) });
            },
        }
    }
};

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

    pub fn deinit(self: *Type, allocator: Allocator, ast: *AST) void {
        switch (self.*) {
            .Array => |*arr| arr.deinit(allocator, ast),
            .Map => |*map| map.deinit(allocator, ast),
            .Function => |*func| func.deinit(allocator, ast),
             ast),
            .Channel => |*chan| chan.deinit(allocator, ast),
            .Slice => |*slice| slice.deinit(allocator, ast),
            .Pointer => |*ptr| ptr.deinit(allocator, ast),
            .Interface => |*interface| interface.deinit(allocator, ast),
            .Struct => |*struct_type| struct_type.deinit(allocator, ast),
            .Generic => |*generic| generic.deinit(allocator, ast),
            else => {},
        }
    }
};

// Core AST container that holds all nodes
pub const AST = struct {
    expressions: ArrayList(Expression),
    statements: ArrayList(Statement),
    types: ArrayList(Type),
    allocator: Allocator,

    pub fn init() AST {
        return AST{
            .expressions = .empty,
            .statements = .empty,
            .types = .empty,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *AST) void {
        // Clean up all expressions
        for (self.expressions.items) |*expr| {
            expr.deinit(self.allocator, self);
        }
        self.expressions.deinit(self.allocator);

        // Clean up all statements
        for (self.statements.items) |*stmt| {
            stmt.deinit(self.allocator, self);
        }
        self.statements.deinit(self.allocator);

        // Clean up all types
        for (self.types.items) |*type_val| {
            type_val.deinit(self.allocator, self);
        }
        self.types.deinit(self.allocator);
    }

    pub fn addExpression(self: *AST, expr: Expression) !NodeIndex {
        const index = @as(NodeIndex, @intCast(self.expressions.items.len));
        try self.expressions.append(self.allocator, expr);
        return index;
    }

    pub fn addStatement(self: *AST, stmt: Statement) !NodeIndex {
        const index = @as(NodeIndex, @intCast(self.statements.items.len));
        try self.statements.append(allocator, stmt);
        return index;
    }

    pub fn addType(self: *AST, type_val: Type) !NodeIndex {
        const index = @as(NodeIndex, @intCast(self.types.items.len));
        try self.types.append(allocator, type_val);
        return index;
    }

    pub fn getExpression(self: *AST, index: NodeIndex) ?*Expression {
        if (index == INVALID_NODE or index >= self.expressions.items.len) return null;
        return &self.expressions.items[index];
    }

    pub fn getStatement(self: *AST, index: NodeIndex) ?*Statement {
        if (index == INVALID_NODE or index >= self.statements.items.len) return null;
        return &self.statements.items[index];
    }

    pub fn getType(self: *AST, index: NodeIndex) ?*Type {
        if (index == INVALID_NODE or index >= self.types.items.len) return null;
        return &self.types.items[index];
    }
};

pub const Program = struct {
    statements: ArrayList(NodeIndex),
    imports: ArrayList(ImportStatement),
    package: ?PackageDeclaration,

    pub fn init() Program {
        return Program{
            .statements = .empty,
            .imports = .empty,
            .package = null,
        };
    }

    pub fn deinit(self: *Program, allocator: Allocator, ast: *AST) void {
        _ = ast;
        self.statements.deinit(self.allocator);
        
        for (self.imports.items) |*import| {
            import.deinit();
        }
        self.imports.deinit(self.allocator);
        
        if (self.package) |*pkg| {
            pkg.deinit();
        }
    }

    pub fn print(self: Program, indent: usize, ast: *AST) !void {
        const print_fn = std.debug.print;
        const spaces = "  " ** 10;
        
        print_fn("{s}Program:\n", .{spaces[0..indent]});
        
        if (self.package) |pkg| {
            print_fn("{s}  Package: {s}\n", .{ spaces[0..indent], pkg.name });
        }
        
        for (self.imports.items) |import| {
            print_fn("{s}  Import: {s}\n", .{ spaces[0..indent], import.path });
        }
        
        for (self.statements.items) |stmt_idx| {
            if (stmt_idx != INVALID_NODE) {
                try ast.statements.items[stmt_idx].print(indent + 2, ast);
            }
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

// Expression structures using indices
pub const BinaryExpression = struct {
    left: NodeIndex,
    operator: []const u8,
    right: NodeIndex,

    pub fn deinit(self: *BinaryExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
        // Expressions will be cleaned up by the AST container
    }
};

pub const CallExpression = struct {
    function: NodeIndex,
    arguments: ArrayList(NodeIndex),

    pub fn deinit(self: *CallExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        self.arguments.deinit(self.allocator);
    }
};

pub const MemberAccessExpression = struct {
    object: NodeIndex,
    property: []const u8,

    pub fn deinit(self: *MemberAccessExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const UnaryExpression = struct {
    operator: []const u8,
    operand: NodeIndex,

    pub fn deinit(self: *UnaryExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const ArrayExpression = struct {
    elements: ArrayList(NodeIndex),

    pub fn deinit(self: *ArrayExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        self.elements.deinit(self.allocator);
    }
};

pub const MapExpression = struct {
    entries: ArrayList(MapEntry),

    pub fn deinit(self: *MapExpression, allocator: Allocator, ast: *AST) void {
        for (self.entries.items) |*entry| {
            entry.deinit(allocator, ast);
        }
        self.entries.deinit(self.allocator);
    }
};

pub const MapEntry = struct {
    key: NodeIndex,
    value: NodeIndex,

    pub fn deinit(self: *MapEntry, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

// Type structures
pub const ChannelType = struct {
    element_type: NodeIndex,
    is_send_only: bool,
    is_receive_only: bool,

    pub fn deinit(self: *ChannelType, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const ArrayType = struct {
    element_type: *Type,
    size: ?*Expression,

    pub fn deinit(self: *ArrayType, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const SliceType = struct {
    element_type: *Type,

    pub fn deinit(self: *SliceType, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const MapType = struct {
    key_type: NodeIndex,
    value_type: NodeIndex,

    pub fn deinit(self: *MapType, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const PointerType = struct {
    target_type: *Type,
    is_mutable: bool,

    pub fn deinit(self: *PointerType, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const FunctionType = struct {
    parameters: ArrayList(Type),
    return_type: ?NodeIndex,

    pub fn deinit(self: *FunctionType, allocator: Allocator, ast: *AST) void {
        for (self.parameters.items) |*param| {
            param.deinit(allocator, ast);
        }
        self.parameters.deinit(self.allocator);
    }
};

pub const InterfaceType = struct {
    name: []const u8,
    methods: ArrayList(MethodSignature),

    pub fn deinit(self: *InterfaceType, allocator: Allocator, ast: *AST) void {
        for (self.methods.items) |*method| {
            method.deinit(allocator, ast);
        }
        self.methods.deinit(self.allocator);
    }
};

pub const StructType = struct {
    name: []const u8,
    fields: ArrayList(StructField),

    pub fn deinit(self: *StructType, allocator: Allocator, ast: *AST) void {
        for (self.fields.items) |*field| {
            field.deinit(allocator, ast);
        }
        self.fields.deinit(self.allocator);
    }
};

pub const GenericType = struct {
    name: []const u8,
    constraints: ArrayList(Type),

    pub fn deinit(self: *GenericType, allocator: Allocator, ast: *AST) void {
        for (self.constraints.items) |*constraint| {
            constraint.deinit(allocator, ast);
        }
        self.constraints.deinit(self.allocator);
    }
};

pub const TupleType = struct {
    elements: ArrayList(Type),

    pub fn deinit(self: *TupleType, allocator: Allocator, ast: *AST) void {
        for (self.elements.items) |*elem| {
            elem.deinit(allocator, ast);
        }
        self.elements.deinit(self.allocator);
    }
};

pub const MethodSignature = struct {
    name: []const u8,
    parameters: ArrayList(Parameter),
    return_type: ?Type,

    pub fn deinit(self: *MethodSignature, allocator: Allocator, ast: *AST) void {
        for (self.parameters.items) |*param| {
            param.deinit(allocator, ast);
        }
        self.parameters.deinit(self.allocator);
        
        if (self.return_type) |*ret_type| {
            ret_type.deinit(allocator, ast);
        }
    }
};

pub const StructField = struct {
    name: []const u8,
    field_type: Type,
    visibility: Visibility,

    pub fn deinit(self: *StructField, allocator: Allocator, ast: *AST) void {
        self.field_type.deinit(allocator, ast);
    }
};

pub const Parameter = struct {
    name: []const u8,
    param_type: Type,
    is_mutable: bool,
    default_value: ?NodeIndex,

    pub fn deinit(self: *Parameter, allocator: Allocator, ast: *AST) void {
        self.param_type.deinit(allocator, ast);
                _ = self;
    }
};

pub const TypeParameter = struct {
    name: []const u8,
    constraints: ArrayList(Type),

    pub fn deinit(self: *TypeParameter, allocator: Allocator, ast: *AST) void {
        for (self.constraints.items) |*constraint| {
            constraint.deinit(allocator, ast);
        }
        self.constraints.deinit(self.allocator);
    }
};

// Statement structures
pub const LetStatement = struct {
    name: []const u8,
    var_type: ?Type,
    initializer: NodeIndex,
    is_mutable: bool,

    pub fn deinit(self: *LetStatement, allocator: Allocator, ast: *AST) void {
        if (self.var_type) |*var_type| {
            var_type.deinit(allocator, ast);
        }
        _ = self;
    }
};

pub const AssignmentStatement = struct {
    target: NodeIndex,
    value: NodeIndex,
    operator: []const u8,

    pub fn deinit(self: *AssignmentStatement, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const ReturnStatement = struct {
    value: ?NodeIndex,

    pub fn deinit(self: *ReturnStatement, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const IfStatement = struct {
    condition: NodeIndex,
    then_branch: ArrayList(NodeIndex),
    else_branch: ?ArrayList(NodeIndex),

    pub fn deinit(self: *IfStatement, allocator: Allocator, ast: *AST) void {
                _ = ast;
        self.then_branch.deinit(self.allocator);
        
        if (self.else_branch) |*else_br| {
            else_br.deinit();
        }
    }
};

pub const FunctionStatement = struct {
    name: []const u8,
    parameters: ArrayList(Parameter),
    return_type: ?Type,
    body: ArrayList(NodeIndex),
    visibility: Visibility,
    is_async: bool,
    type_parameters: ArrayList(TypeParameter),
    comments: ArrayList(Comment),

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

    pub fn deinit(self: *FunctionStatement, allocator: Allocator, ast: *AST) void {
        for (self.parameters.items) |*param| {
            param.deinit(allocator, ast);
        }
        self.parameters.deinit(self.allocator);
        
        if (self.return_type) |*ret_type| {
            ret_type.deinit(allocator, ast);
        }
        
        self.body.deinit(self.allocator);
        
        for (self.type_parameters.items) |*type_param| {
            type_param.deinit(allocator, ast);
        }
        self.type_parameters.deinit(self.allocator);
        
        self.comments.deinit(self.allocator);
    }
};

pub const WhileStatement = struct {
    condition: NodeIndex,
    body: ArrayList(NodeIndex),

    pub fn deinit(self: *WhileStatement, allocator: Allocator, ast: *AST) void {
                _ = ast;
        self.body.deinit(self.allocator);
    }
};

// Add stub implementations for remaining expression types to make it compile
pub const CompositeLiteralExpression = struct {
    type_name: []const u8,
    elements: ArrayList(NodeIndex),

    pub fn deinit(self: *CompositeLiteralExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        self.elements.deinit(self.allocator);
    }
};

pub const ChannelSendExpression = struct {
    channel: NodeIndex,
    value: NodeIndex,

    pub fn deinit(self: *ChannelSendExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const ChannelReceiveExpression = struct {
    channel: NodeIndex,

    pub fn deinit(self: *ChannelReceiveExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const ChannelCreationExpression = struct {
    element_type: Type,
    buffer_size: ?NodeIndex,

    pub fn deinit(self: *ChannelCreationExpression, allocator: Allocator, ast: *AST) void {
        self.element_type.deinit(allocator, ast);
        _ = self;
    }
};

pub const StructLiteralExpression = struct {
    struct_name: []const u8,
    fields: ArrayList(StructFieldAssignment),

    pub fn deinit(self: *StructLiteralExpression, allocator: Allocator, ast: *AST) void {
        for (self.fields.items) |*field| {
            field.deinit(allocator, ast);
        }
        self.fields.deinit(self.allocator);
    }
};

pub const StructFieldAssignment = struct {
    field_name: []const u8,
    value: NodeIndex,

    pub fn deinit(self: *StructFieldAssignment, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const LambdaExpression = struct {
    parameters: ArrayList([]const u8),
    body: NodeIndex,

    pub fn deinit(self: *LambdaExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        self.parameters.deinit(self.allocator);
    }
};

pub const TupleExpression = struct {
    elements: ArrayList(NodeIndex),

    pub fn deinit(self: *TupleExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        self.elements.deinit(self.allocator);
    }
};

pub const TupleAccessExpression = struct {
    tuple: NodeIndex,
    index: usize,

    pub fn deinit(self: *TupleAccessExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const ArrayAccessExpression = struct {
    array: NodeIndex,
    index: NodeIndex,

    pub fn deinit(self: *ArrayAccessExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const SliceAccessExpression = struct {
    array: NodeIndex,
    start: ?NodeIndex,
    end: ?NodeIndex,

    pub fn deinit(self: *SliceAccessExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const TypeAssertionExpression = struct {
    value: NodeIndex,
    target_type: Type,
    is_safe: bool,

    pub fn deinit(self: *TypeAssertionExpression, allocator: Allocator, ast: *AST) void {
        self.target_type.deinit(allocator, ast);
        _ = self;
    }
};

pub const IncrementExpression = struct {
    variable: NodeIndex,

    pub fn deinit(self: *IncrementExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const DecrementExpression = struct {
    variable: NodeIndex,

    pub fn deinit(self: *DecrementExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const ShookExpression = struct {
    expression: NodeIndex,

    pub fn deinit(self: *ShookExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const ErrorValueExpression = struct {
    message: []const u8,

    pub fn deinit(self: *ErrorValueExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const StructuredErrorExpression = struct {
    message: NodeIndex,
    code: ?NodeIndex,
    details: ?NodeIndex,
    fields: ArrayList(StructuredErrorField),

    pub fn deinit(self: *StructuredErrorExpression, allocator: Allocator, ast: *AST) void {
        for (self.fields.items) |*field| {
            field.deinit(allocator, ast);
        }
        self.fields.deinit(self.allocator);
        _ = self;
    }
};

pub const StructuredErrorField = struct {
    name: []const u8,
    value: NodeIndex,

    pub fn deinit(self: *StructuredErrorField, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const PanicExpression = struct {
    message: NodeIndex,

    pub fn deinit(self: *PanicExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const RecoverExpression = struct {
    pub fn deinit(self: *RecoverExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const TestResultExpression = struct {
    test_name: []const u8,
    is_passing: bool,

    pub fn deinit(self: *TestResultExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const TestResultCheckExpression = struct {
    expression: NodeIndex,

    pub fn deinit(self: *TestResultCheckExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const RangeForExpression = struct {
    iterable: NodeIndex,

    pub fn deinit(self: *RangeForExpression, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const MatchExpression = struct {
    expression: NodeIndex,
    cases: ArrayList(MatchCase),
    default_case: ?NodeIndex,

    pub fn deinit(self: *MatchExpression, allocator: Allocator, ast: *AST) void {
        for (self.cases.items) |*case| {
            case.deinit(allocator, ast);
        }
        self.cases.deinit(self.allocator);
        _ = self;
    }
};

pub const TypeSwitchExpression = struct {
    expression: NodeIndex,
    cases: ArrayList(TypeCase),
    default_case: ?NodeIndex,

    pub fn deinit(self: *TypeSwitchExpression, allocator: Allocator, ast: *AST) void {
        for (self.cases.items) |*case| {
            case.deinit(allocator, ast);
        }
        self.cases.deinit(self.allocator);
        _ = self;
    }
};

// Add remaining statement types as stubs
pub const ForStatement = struct {
    init: ?NodeIndex,
    condition: ?NodeIndex,
    update: ?NodeIndex,
    body: ArrayList(NodeIndex),

    pub fn deinit(self: *ForStatement, allocator: Allocator, ast: *AST) void {
                _ = ast;
        self.body.deinit(self.allocator);
    }
};

pub const ForInStatement = struct {
    variable: []const u8,
    iterable: NodeIndex,
    body: ArrayList(NodeIndex),

    pub fn deinit(self: *ForInStatement, allocator: Allocator, ast: *AST) void {
                _ = ast;
        self.body.deinit(self.allocator);
    }
};

pub const SwitchStatement = struct {
    expression: NodeIndex,
    cases: ArrayList(SwitchCase),
    default_case: ?ArrayList(NodeIndex),

    pub fn deinit(self: *SwitchStatement, allocator: Allocator, ast: *AST) void {
        for (self.cases.items) |*case| {
            case.deinit(allocator, ast);
        }
        self.cases.deinit(self.allocator);
        
        if (self.default_case) |*default| {
            default.deinit();
        }
    }
};

pub const PatternSwitchStatement = struct {
    expression: NodeIndex,
    patterns: ArrayList(PatternCase),
    default_case: ?ArrayList(NodeIndex),

    pub fn deinit(self: *PatternSwitchStatement, allocator: Allocator, ast: *AST) void {
        for (self.patterns.items) |*pattern| {
            pattern.deinit(allocator, ast);
        }
        self.patterns.deinit(self.allocator);
        
        if (self.default_case) |*default| {
            default.deinit();
        }
    }
};

pub const GoroutineStatement = struct {
    call: CallExpression,

    pub fn deinit(self: *GoroutineStatement, allocator: Allocator, ast: *AST) void {
        self.call.deinit(allocator, ast);
    }
};

pub const ChannelStatement = struct {
    name: []const u8,
    channel_type: Type,
    buffer_size: ?NodeIndex,

    pub fn deinit(self: *ChannelStatement, allocator: Allocator, ast: *AST) void {
        self.channel_type.deinit(allocator, ast);
        _ = self;
    }
};

pub const SelectStatement = struct {
    cases: ArrayList(SelectCase),
    default_case: ?ArrayList(NodeIndex),

    pub fn deinit(self: *SelectStatement, allocator: Allocator, ast: *AST) void {
        for (self.cases.items) |*case| {
            case.deinit(allocator, ast);
        }
        self.cases.deinit(self.allocator);
        
        if (self.default_case) |*default| {
            default.deinit();
        }
    }
};

pub const StructStatement = struct {
    name: []const u8,
    fields: ArrayList(StructField),
    visibility: Visibility,
    type_parameters: ArrayList(TypeParameter),

    pub fn deinit(self: *StructStatement, allocator: Allocator, ast: *AST) void {
        for (self.fields.items) |*field| {
            field.deinit(allocator, ast);
        }
        self.fields.deinit(self.allocator);
        
        for (self.type_parameters.items) |*type_param| {
            type_param.deinit(allocator, ast);
        }
        self.type_parameters.deinit(self.allocator);
    }
};

pub const InterfaceStatement = struct {
    name: []const u8,
    methods: ArrayList(MethodSignature),
    visibility: Visibility,
    type_parameters: ArrayList(TypeParameter),

    pub fn deinit(self: *InterfaceStatement, allocator: Allocator, ast: *AST) void {
        for (self.methods.items) |*method| {
            method.deinit(allocator, ast);
        }
        self.methods.deinit(self.allocator);
        
        for (self.type_parameters.items) |*type_param| {
            type_param.deinit(allocator, ast);
        }
        self.type_parameters.deinit(self.allocator);
    }
};

pub const TypeAliasStatement = struct {
    name: []const u8,
    target_type: Type,
    visibility: Visibility,

    pub fn deinit(self: *TypeAliasStatement, allocator: Allocator, ast: *AST) void {
        self.target_type.deinit(allocator, ast);
    }
};

pub const PanicStatement = struct {
    message: NodeIndex,

    pub fn deinit(self: *PanicStatement, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const CatchStatement = struct {
    body: ArrayList(NodeIndex),
    error_variable: ?[]const u8,
    error_type: ?Type,

    pub fn deinit(self: *CatchStatement, allocator: Allocator, ast: *AST) void {
        self.body.deinit(self.allocator);
        
        if (self.error_type) |*err_type| {
            err_type.deinit(allocator, ast);
        }
    }
};

pub const DeferStatement = struct {
    statement: NodeIndex,

    pub fn deinit(self: *DeferStatement, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const BreakStatement = struct {
    pub fn deinit(self: *BreakStatement, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const ContinueStatement = struct {
    pub fn deinit(self: *ContinueStatement, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const IncrementStatement = struct {
    variable: NodeIndex,

    pub fn deinit(self: *IncrementStatement, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const DecrementStatement = struct {
    variable: NodeIndex,

    pub fn deinit(self: *DecrementStatement, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const ShortDeclarationStatement = struct {
    names: ArrayList([]const u8),
    values: ArrayList(NodeIndex),

    pub fn deinit(self: *ShortDeclarationStatement, allocator: Allocator, ast: *AST) void {
                _ = ast;
        self.names.deinit(self.allocator);
        self.values.deinit(self.allocator);
    }
};

pub const YikesStatement = struct {
    name: []const u8,
    error_type: ?Type,
    value: ?NodeIndex,

    pub fn deinit(self: *YikesStatement, allocator: Allocator, ast: *AST) void {
        if (self.error_type) |*err_type| {
            err_type.deinit(allocator, ast);
        }
        _ = self;
    }
};

pub const FamStatement = struct {
    body: ArrayList(NodeIndex),
    recovery_body: ?ArrayList(NodeIndex),
    error_variable: ?[]const u8,

    pub fn deinit(self: *FamStatement, allocator: Allocator, ast: *AST) void {
                _ = ast;
        self.body.deinit(self.allocator);
        
        if (self.recovery_body) |*recovery| {
            recovery.deinit();
        }
    }
};

pub const ConstDecl = struct {
    name: []const u8,
    const_type: ?Type,
    value: NodeIndex,
    visibility: Visibility,

    pub fn deinit(self: *ConstDecl, allocator: Allocator, ast: *AST) void {
        if (self.const_type) |*const_type| {
            const_type.deinit(allocator, ast);
        }
        _ = self;
    }
};

// Support structures
pub const SwitchCase = struct {
    value: NodeIndex,
    body: ArrayList(NodeIndex),

    pub fn deinit(self: *SwitchCase, allocator: Allocator, ast: *AST) void {
                _ = ast;
        self.body.deinit(self.allocator);
    }
};

pub const PatternCase = struct {
    pattern: Pattern,
    guard: ?NodeIndex,
    body: ArrayList(NodeIndex),

    pub fn deinit(self: *PatternCase, allocator: Allocator, ast: *AST) void {
        self.pattern.deinit(allocator, ast);
        self.body.deinit(self.allocator);
        _ = self;
    }
};

pub const SelectCase = struct {
    channel_op: ChannelOperation,
    body: ArrayList(NodeIndex),

    pub fn deinit(self: *SelectCase, allocator: Allocator, ast: *AST) void {
        self.channel_op.deinit(allocator, ast);
        self.body.deinit(self.allocator);
    }
};

pub const ChannelOperation = union(enum) {
    Send: struct {
        channel: NodeIndex,
        value: NodeIndex,
    },
    Receive: struct {
        channel: NodeIndex,
        variable: ?[]const u8,
    },

    pub fn deinit(self: *ChannelOperation, allocator: Allocator, ast: *AST) void {
                _ = ast;
        _ = self;
    }
};

pub const MatchCase = struct {
    pattern: Pattern,
    guard: ?NodeIndex,
    result: NodeIndex,

    pub fn deinit(self: *MatchCase, allocator: Allocator, ast: *AST) void {
        self.pattern.deinit(allocator, ast);
        _ = self;
    }
};

pub const TypeCase = struct {
    type_pattern: Type,
    variable: ?[]const u8,
    result: NodeIndex,

    pub fn deinit(self: *TypeCase, allocator: Allocator, ast: *AST) void {
        self.type_pattern.deinit(allocator, ast);
        _ = self;
    }
};

pub const Pattern = union(enum) {
    Wildcard,
    Literal: Literal,
    Variable: []const u8,
    Tuple: ArrayList(Pattern),
    Struct: StructPattern,
    Array: ArrayList(Pattern),

    pub fn deinit(self: *Pattern, allocator: Allocator, ast: *AST) void {
        switch (self.*) {
             ast);
                }
                tuple.deinit();
            },
            .Struct => |*struct_pattern| {
                struct_pattern.deinit(allocator, ast);
            },
            .Array => |*array| {
                for (array.items) |*pattern| {
                    pattern.deinit(allocator, ast);
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

    pub fn deinit(self: *StructPattern, allocator: Allocator, ast: *AST) void {
        for (self.fields.items) |*field| {
            field.deinit(allocator, ast);
        }
        self.fields.deinit(self.allocator);
    }
};

pub const FieldPattern = struct {
    name: []const u8,
    pattern: Pattern,

    pub fn deinit(self: *FieldPattern, allocator: Allocator, ast: *AST) void {
        self.pattern.deinit(allocator, ast);
    }
};

// Tests
test "ast creation" {
    const allocator = std.testing.allocator;
    
    var ast = AST.init(allocator);
    defer ast.deinit();
    
    var program = Program.init(allocator);
    defer program.deinit(allocator, &ast);
    
    try std.testing.expect(program.statements.items.len == 0);
}

test "function statement" {
    const allocator = std.testing.allocator;
    
    var ast = AST.init(allocator);
    defer ast.deinit();
    
    var func = FunctionStatement.init(allocator, "test_function");
    defer func.deinit(allocator, &ast);
    
    try std.testing.expect(std.mem.eql(u8, func.name, "test_function"));
    try std.testing.expect(func.parameters.items.len == 0);
}

test "expression cleanup" {
    const allocator = std.testing.allocator;
    
    var ast = AST.init(allocator);
    defer ast.deinit();
    
    // Test that we can create and cleanup expressions without circular dependency issues
    const expr_idx = try ast.addExpression(Expression{ .Integer = 42 });
    
    try std.testing.expect(ast.expressions.items[expr_idx] == .Integer);
    try std.testing.expect(ast.expressions.items[expr_idx].Integer == 42);
}

test "binary expression with indices" {
    const allocator = std.testing.allocator;
    
    var ast = AST.init(allocator);
    defer ast.deinit();
    
    // Create left and right expressions
    const left_idx = try ast.addExpression(Expression{ .Integer = 10 });
    const right_idx = try ast.addExpression(Expression{ .Integer = 20 });
    
    // Create binary expression
    const binary = BinaryExpression{
        .left = left_idx,
        .operator = "+",
        .right = right_idx,
    };
    const binary_idx = try ast.addExpression(Expression{ .Binary = binary });
    
    // Verify the structure
    const binary_expr = ast.getExpression(binary_idx).?;
    try std.testing.expect(binary_expr.* == .Binary);
    try std.testing.expect(binary_expr.Binary.left == left_idx);
    try std.testing.expect(binary_expr.Binary.right == right_idx);
    try std.testing.expect(std.mem.eql(u8, binary_expr.Binary.operator, "+"));
}
