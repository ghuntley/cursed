const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Forward declarations

pub const Program = struct {
    statements: ArrayList(Statement),
    imports: ArrayList(ImportStatement),
    package: ?PackageDeclaration,

    pub fn init(allocator: Allocator) Program {
        return Program{
            .statements = .empty,
            .imports = .empty,
            .package = null,
        };
    }

    pub fn deinit(self: *Program, allocator: Allocator) void {
        for (self.statements.items) |*stmt| {
            stmt.deinit(allocator);
        }
        self.statements.deinit(allocator);
        
        for (self.imports.items) |*import| {
            import.deinit(allocator);
        }
        self.imports.deinit(allocator);
        
        if (self.package) |*pkg| {
            pkg.deinit(allocator);
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
            try stmt.print(indent + 2);
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
        self.items.deinit(allocator);
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
        self.parameters.deinit(allocator);
        
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
        self.elements.deinit(allocator);
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
    default_value: ?Expression,
};

pub const TypeParameter = struct {
    name: []const u8,
    constraints: ArrayList(Type),
};

pub const Statement = union(enum) {
    Expression: Expression,
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
        switch (self.*) {
            .Expression => |*expr| expr.deinit(allocator),
            .Let => |*let| let.deinit(allocator),
            .Assignment => |*assign| assign.deinit(allocator),
            .Return => |*ret| ret.deinit(allocator),
            .If => |*if_stmt| if_stmt.deinit(allocator),
            .Function => |*func| func.deinit(allocator),
            .While => |*while_stmt| while_stmt.deinit(allocator),
            // Add more cases as needed
            else => {},
        }
    }

    pub fn print(self: Statement, indent: usize) !void {
        const print_fn = std.debug.print;
        const spaces = "  " ** 10;
        
        switch (self) {
            .Expression => |expr| {
                print_fn("{s}Expression: ", .{spaces[0..indent]});
                try expr.print(0);
                print_fn("\n", .{});
            },
            .Let => |let| {
                print_fn("{s}Let: {s} = ", .{ spaces[0..indent], let.name });
                if (let.initializer) |init| {
                    try init.print(0);
                }
                print_fn("\n", .{});
            },
            .Function => |func| {
                print_fn("{s}Function: {s}\n", .{ spaces[0..indent], func.name });
                for (func.body.items) |stmt| {
                    try stmt.print(indent + 2);
                }
            },
            else => {
                print_fn("{s}Statement: {s}\n", .{ spaces[0..indent], @tagName(self) });
            },
        }
    }
};

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
    Array: ArrayList(Expression),
    Map: ArrayList(MapEntry),
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
        switch (self.*) {
            .Array => |*arr| {
                for (arr.items) |*expr| {
                    expr.deinit(allocator);
                }
                arr.deinit(allocator);
            },
            .Map => |*map| {
                for (map.items) |*entry| {
                    entry.key.deinit(allocator);
                    entry.value.deinit(allocator);
                }
                map.deinit(allocator);
            },
            .Binary => |*bin| bin.deinit(allocator),
            .Call => |*call| call.deinit(allocator),
            else => {},
        }
    }

    pub fn print(self: Expression, indent: usize) !void {
        const print_fn = std.debug.print;
        _ = indent;
        
        switch (self) {
            .Identifier => |id| print_fn("{s}", .{id}),
            .Variable => |var_name| print_fn("{s}", .{var_name}),
            .Integer => |int| print_fn("{}", .{int}),
            .Float => |float| print_fn("{d}", .{float}),
            .String => |str| print_fn("\"{s}\"", .{str}),
            .Boolean => |bool_val| print_fn("{}", .{bool_val}),
            .Character => |char| print_fn("'{c}'", .{char}),
            .Binary => |bin| {
                try bin.left.print(0);
                print_fn(" {s} ", .{bin.operator});
                try bin.right.print(0);
            },
            .Call => |call| {
                try call.function.print(0);
                print_fn("(", .{});
                for (call.arguments.items, 0..) |arg, i| {
                    if (i > 0) print_fn(", ", .{});
                    try arg.print(0);
                }
                print_fn(")", .{});
            },
            else => print_fn("{s}", .{@tagName(self)}),
        }
    }
};

pub const MapEntry = struct {
    key: Expression,
    value: Expression,
};

pub const Literal = union(enum) {
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    Character: u8,
    Null,
};

// Statement structures
pub const LetStatement = struct {
    name: []const u8,
    var_type: ?Type,
    initializer: ?Expression,
    is_mutable: bool,

    pub fn deinit(self: *LetStatement, allocator: Allocator) void {
        if (self.var_type) |*var_type| {
            var_type.deinit(allocator);
        }
        if (self.initializer) |*init| {
            init.deinit(allocator);
        }
    }
};

pub const AssignmentStatement = struct {
    target: Expression,
    value: Expression,
    operator: []const u8,

    pub fn deinit(self: *AssignmentStatement, allocator: Allocator) void {
        self.target.deinit(allocator);
        self.value.deinit(allocator);
    }
};

pub const ReturnStatement = struct {
    value: ?Expression,

    pub fn deinit(self: *ReturnStatement, allocator: Allocator) void {
        if (self.value) |*val| {
            val.deinit(allocator);
        }
    }
};

pub const IfStatement = struct {
    condition: Expression,
    then_branch: ArrayList(Statement),
    else_branch: ?ArrayList(Statement),

    pub fn deinit(self: *IfStatement, allocator: Allocator) void {
        self.condition.deinit(allocator);
        for (self.then_branch.items) |*stmt| {
            stmt.deinit(allocator);
        }
        self.then_branch.deinit(allocator);
        
        if (self.else_branch) |*else_br| {
            for (else_br.items) |*stmt| {
                stmt.deinit(allocator);
            }
            else_br.deinit(allocator);
        }
    }
};

pub const FunctionStatement = struct {
    name: []const u8,
    parameters: ArrayList(Parameter),
    return_type: ?Type,
    body: ArrayList(Statement),
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

    pub fn deinit(self: *FunctionStatement, allocator: Allocator) void {
        for (self.parameters.items) |*param| {
            param.param_type.deinit(allocator);
            if (param.default_value) |*default| {
                default.deinit(allocator);
            }
        }
        self.parameters.deinit(allocator);
        
        if (self.return_type) |*ret_type| {
            ret_type.deinit(allocator);
        }
        
        for (self.body.items) |*stmt| {
            stmt.deinit(allocator);
        }
        self.body.deinit(allocator);
        
        for (self.type_parameters.items) |*type_param| {
            for (type_param.constraints.items) |*constraint| {
                constraint.deinit(allocator);
            }
            type_param.constraints.deinit(allocator);
        }
        self.type_parameters.deinit(allocator);
        
        self.comments.deinit(allocator);
    }
};

pub const WhileStatement = struct {
    condition: Expression,
    body: ArrayList(Statement),

    pub fn deinit(self: *WhileStatement, allocator: Allocator) void {
        self.condition.deinit(allocator);
        for (self.body.items) |*stmt| {
            stmt.deinit(allocator);
        }
        self.body.deinit(allocator);
    }
};

pub const ForStatement = struct {
    init: ?Statement,
    condition: ?Expression,
    update: ?Statement,
    body: ArrayList(Statement),
};

pub const ForInStatement = struct {
    variable: []const u8,
    iterable: Expression,
    body: ArrayList(Statement),
};

pub const SwitchStatement = struct {
    expression: Expression,
    cases: ArrayList(SwitchCase),
    default_case: ?ArrayList(Statement),
};

pub const PatternSwitchStatement = struct {
    expression: Expression,
    patterns: ArrayList(PatternCase),
    default_case: ?ArrayList(Statement),
};

pub const GoroutineStatement = struct {
    call: CallExpression,
};

pub const ChannelStatement = struct {
    name: []const u8,
    channel_type: Type,
    buffer_size: ?Expression,
};

pub const SelectStatement = struct {
    cases: ArrayList(SelectCase),
    default_case: ?ArrayList(Statement),
};

pub const StructStatement = struct {
    name: []const u8,
    fields: ArrayList(StructField),
    visibility: Visibility,
    type_parameters: ArrayList(TypeParameter),
};

pub const InterfaceStatement = struct {
    name: []const u8,
    methods: ArrayList(MethodSignature),
    visibility: Visibility,
    type_parameters: ArrayList(TypeParameter),
};

pub const TypeAliasStatement = struct {
    name: []const u8,
    target_type: Type,
    visibility: Visibility,
};

pub const PanicStatement = struct {
    message: Expression,
};

pub const CatchStatement = struct {
    body: ArrayList(Statement),
    error_variable: ?[]const u8,
    error_type: ?Type,
};

pub const DeferStatement = struct {
    statement: *Statement,
};

pub const BreakStatement = struct {};

pub const ContinueStatement = struct {};

pub const IncrementStatement = struct {
    variable: Expression,
};

pub const DecrementStatement = struct {
    variable: Expression,
};

pub const ShortDeclarationStatement = struct {
    names: ArrayList([]const u8),
    values: ArrayList(Expression),
};

pub const YikesStatement = struct {
    name: []const u8,
    error_type: ?Type,
    value: ?Expression,
};

pub const FamStatement = struct {
    body: ArrayList(Statement),
    recovery_body: ?ArrayList(Statement),
    error_variable: ?[]const u8,
};

pub const ConstDecl = struct {
    name: []const u8,
    const_type: ?Type,
    value: Expression,
    visibility: Visibility,
};

// Expression structures
pub const BinaryExpression = struct {
    left: *Expression,
    operator: []const u8,
    right: *Expression,

    pub fn deinit(self: *BinaryExpression, allocator: Allocator) void {
        self.left.deinit(allocator);
        self.right.deinit(allocator);
        allocator.destroy(self.left);
        allocator.destroy(self.right);
    }
};

pub const CallExpression = struct {
    function: *Expression,
    arguments: ArrayList(Expression),

    pub fn deinit(self: *CallExpression, allocator: Allocator) void {
        self.function.deinit(allocator);
        allocator.destroy(self.function);
        
        for (self.arguments.items) |*arg| {
            arg.deinit(allocator);
        }
        self.arguments.deinit(allocator);
    }
};

pub const MemberAccessExpression = struct {
    object: *Expression,
    property: []const u8,
};

pub const UnaryExpression = struct {
    operator: []const u8,
    operand: *Expression,
};

pub const CompositeLiteralExpression = struct {
    type_name: []const u8,
    elements: ArrayList(Expression),
};

pub const ChannelSendExpression = struct {
    channel: *Expression,
    value: *Expression,
};

pub const ChannelReceiveExpression = struct {
    channel: *Expression,
};

pub const ChannelCreationExpression = struct {
    element_type: Type,
    buffer_size: ?Expression,
};

pub const StructLiteralExpression = struct {
    struct_name: []const u8,
    fields: ArrayList(StructFieldAssignment),
};

pub const StructFieldAssignment = struct {
    field_name: []const u8,
    value: Expression,
};

pub const LambdaExpression = struct {
    parameters: ArrayList([]const u8),
    body: *Expression,
};

pub const TupleExpression = struct {
    elements: ArrayList(Expression),
};

pub const TupleAccessExpression = struct {
    tuple: *Expression,
    index: usize,
};

pub const ArrayAccessExpression = struct {
    array: *Expression,
    index: *Expression,
};

pub const SliceAccessExpression = struct {
    array: *Expression,
    start: ?*Expression,
    end: ?*Expression,
};

pub const TypeAssertionExpression = struct {
    value: *Expression,
    target_type: Type,
    is_safe: bool,
};

pub const IncrementExpression = struct {
    variable: *Expression,
};

pub const DecrementExpression = struct {
    variable: *Expression,
};

pub const ShookExpression = struct {
    expression: *Expression,
};

pub const ErrorValueExpression = struct {
    message: []const u8,
};

pub const StructuredErrorExpression = struct {
    message: *Expression,
    code: ?*Expression,
    details: ?*Expression,
    fields: ArrayList(StructuredErrorField),
};

pub const StructuredErrorField = struct {
    name: []const u8,
    value: Expression,
};

pub const PanicExpression = struct {
    message: *Expression,
};

pub const RecoverExpression = struct {};

pub const TestResultExpression = struct {
    test_name: []const u8,
    is_passing: bool,
};

pub const TestResultCheckExpression = struct {
    expression: *Expression,
};

pub const RangeForExpression = struct {
    iterable: *Expression,
};

pub const MatchExpression = struct {
    expression: *Expression,
    cases: ArrayList(MatchCase),
    default_case: ?*Expression,
};

pub const TypeSwitchExpression = struct {
    expression: *Expression,
    cases: ArrayList(TypeCase),
    default_case: ?*Expression,
};

// Support structures
pub const SwitchCase = struct {
    value: Expression,
    body: ArrayList(Statement),
};

pub const PatternCase = struct {
    pattern: Pattern,
    guard: ?Expression,
    body: ArrayList(Statement),
};

pub const SelectCase = struct {
    channel_op: ChannelOperation,
    body: ArrayList(Statement),
};

pub const ChannelOperation = union(enum) {
    Send: struct {
        channel: Expression,
        value: Expression,
    },
    Receive: struct {
        channel: Expression,
        variable: ?[]const u8,
    },
};

pub const MatchCase = struct {
    pattern: Pattern,
    guard: ?Expression,
    result: Expression,
};

pub const TypeCase = struct {
    type_pattern: Type,
    variable: ?[]const u8,
    result: Expression,
};

pub const Pattern = union(enum) {
    Wildcard,
    Literal: Literal,
    Variable: []const u8,
    Tuple: ArrayList(Pattern),
    Struct: StructPattern,
    Array: ArrayList(Pattern),
};

pub const StructPattern = struct {
    name: []const u8,
    fields: ArrayList(FieldPattern),
};

pub const FieldPattern = struct {
    name: []const u8,
    pattern: Pattern,
};

test "ast creation" {
    const allocator = std.testing.allocator;
    
    var program = Program.init(allocator);
    defer program.deinit(allocator);
    
    try std.testing.expect(program.statements.items.len == 0);
}

test "function statement" {
    const allocator = std.testing.allocator;
    
    var func = FunctionStatement.init(allocator, "test_function");
    defer func.deinit(allocator);
    
    try std.testing.expect(std.mem.eql(u8, func.name, "test_function"));
    try std.testing.expect(func.parameters.items.len == 0);
}
