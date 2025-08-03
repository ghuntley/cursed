const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Simple AST for parser testing without circular dependencies
pub const Program = struct {
    statements: ArrayList(Statement),
    imports: ArrayList(ImportStatement),
    package: ?PackageDeclaration,

    pub fn init(allocator: Allocator) Program {
        return Program{
            .statements = ArrayList(Statement).init(allocator),
            .imports = ArrayList(ImportStatement).init(allocator),
            .package = null,
        };
    }

    pub fn deinit(self: *Program, allocator: Allocator) void {
        self.statements.deinit();
        self.imports.deinit();
        _ = allocator;
    }

    pub fn print(self: Program, indent: usize) !void {
        const spaces = "  " ** 10;
        std.debug.print("{s}Program:\n", .{spaces[0..indent]});
        
        if (self.package) |pkg| {
            std.debug.print("{s}  Package: {s}\n", .{ spaces[0..indent], pkg.name });
        }
        
        for (self.imports.items) |import| {
            std.debug.print("{s}  Import: {s}\n", .{ spaces[0..indent], import.path });
        }
        
        for (self.statements.items, 0..) |stmt, i| {
            std.debug.print("{s}  Statement {}: {s}\n", .{ spaces[0..indent], i, @tagName(stmt) });
        }
    }
};

pub const ImportStatement = struct {
    path: []const u8,
    alias: ?[]const u8,

    pub fn init(allocator: Allocator, path: []const u8) ImportStatement {
        _ = allocator;
        return ImportStatement{
            .path = path,
            .alias = null,
        };
    }
};

pub const PackageDeclaration = struct {
    name: []const u8,
    version: ?[]const u8,
};

pub const Statement = enum {
    Expression,
    Let,
    Assignment,
    Return,
    If,
    Function,
    While,
    For,
    ForIn,
    Switch,
    PatternSwitch,
    Goroutine,
    Channel,
    Select,
    Struct,
    Interface,
    Implementation,
    TypeAlias,
    Panic,
    Catch,
    Defer,
    Break,
    Continue,
    Increment,
    Decrement,
    ShortDeclaration,
    Yikes,
    Fam,
    Const,
};

pub const Expression = enum {
    Identifier,
    Variable,
    Integer,
    Float,
    String,
    Boolean,
    Character,
    Binary,
    Call,
    MemberAccess,
    Literal,
    Unary,
    Array,
    Map,
    CompositeLiteral,
    ChannelSend,
    ChannelReceive,
    ChannelCreation,
    StructLiteral,
    Lambda,
    Tuple,
    TupleAccess,
    ArrayAccess,
    SliceAccess,
    TypeAssertion,
    Increment,
    Decrement,
    Shook,
    ErrorValue,
    StructuredError,
    Panic,
    Recover,
    TestResult,
    TestResultCheck,
    RangeFor,
    Match,
    TypeSwitch,
    Block,
};

pub const FunctionStatement = struct {
    name: []const u8,
    parameters: ArrayList(Parameter),
    return_type: ?Type,
    body: ArrayList(Statement),
    visibility: Visibility,
    is_async: bool,
    type_parameters: ArrayList(TypeParameter),

    pub fn init(allocator: Allocator, name: []const u8) FunctionStatement {
        return FunctionStatement{
            .name = name,
            .parameters = ArrayList(Parameter).init(allocator),
            .return_type = null,
            .body = ArrayList(Statement).init(allocator),
            .visibility = .Private,
            .is_async = false,
            .type_parameters = ArrayList(TypeParameter).init(allocator),
        };
    }

    pub fn deinit(self: *FunctionStatement, allocator: Allocator) void {
        self.parameters.deinit();
        self.body.deinit();
        self.type_parameters.deinit();
        _ = allocator;
    }
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

pub const Type = enum {
    Basic,
    Channel,
    Array,
    Slice,
    Map,
    Pointer,
    Function,
    Interface,
    Struct,
    Generic,
    Tuple,
};

pub const BasicType = enum {
    Normie,
    Tea,
    Txt,
    Sip,
    Smol,
    Mid,
    Thicc,
    Snack,
    Meal,
    Byte,
    Rune,
    Extra,
    Lit,
    Cap,
};

pub const Visibility = enum {
    Public,
    Private,
    Package,
};

pub const LetStatement = struct {
    name: []const u8,
    var_type: ?Type,
    initializer: ?Expression,
    is_mutable: bool,
};

pub const IfStatement = struct {
    condition: Expression,
    then_branch: ArrayList(Statement),
    else_branch: ?ArrayList(Statement),
};

pub const WhileStatement = struct {
    condition: Expression,
    body: ArrayList(Statement),
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

pub const ReturnStatement = struct {
    value: ?Expression,
};

pub const BreakStatement = struct {};

pub const ContinueStatement = struct {};

pub const DeferStatement = struct {
    statement: *Statement,
};

pub const StructStatement = struct {
    name: []const u8,
    fields: ArrayList(StructField),
    visibility: Visibility,
    type_parameters: ArrayList(TypeParameter),
};

pub const StructField = struct {
    name: []const u8,
    field_type: Type,
    visibility: Visibility,
};

pub const InterfaceStatement = struct {
    name: []const u8,
    methods: ArrayList(MethodSignature),
    visibility: Visibility,
    type_parameters: ArrayList(TypeParameter),
};

pub const MethodSignature = struct {
    name: []const u8,
    parameters: ArrayList(Parameter),
    return_type: ?Type,
};

pub const ImplementationStatement = struct {
    implementing_type: []const u8,
    interface_name: []const u8,
    methods: ArrayList(FunctionStatement),
    where_clause: ?[]const u8,
};

pub const TypeAliasStatement = struct {
    name: []const u8,
    target_type: Type,
    visibility: Visibility,
};

pub const GoroutineStatement = struct {
    call: Expression, // Simplified - just store as expression type
};

pub const PatternSwitchStatement = struct {
    expression: Expression,
    patterns: ArrayList(PatternCase),
    default_case: ?ArrayList(Statement),
};

pub const PatternCase = struct {
    pattern: Pattern,
    guard: ?Expression,
    body: ArrayList(Statement),
};

pub const SelectStatement = struct {
    cases: ArrayList(SelectCase),
    default_case: ?ArrayList(Statement),
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

pub const AssignmentStatement = struct {
    target: Expression,
    value: Expression,
    operator: []const u8,
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

pub const Pattern = enum {
    Wildcard,
    Literal,
    Variable,
    Tuple,
    Struct,
    Array,
};

// Stub types for expression components
pub const BinaryExpression = struct {
    left: *Expression,
    operator: []const u8,
    right: *Expression,
};

pub const CallExpression = struct {
    function: *Expression,
    arguments: ArrayList(Expression),
};

pub const MemberAccessExpression = struct {
    object: *Expression,
    property: []const u8,
};

pub const UnaryExpression = struct {
    operator: []const u8,
    operand: *Expression,
};

pub const ArrayExpression = struct {
    elements: ArrayList(*anyopaque),
};

pub const MapExpression = struct {
    entries: ArrayList(MapEntry),
};

pub const MapEntry = struct {
    key: Expression,
    value: Expression,
};

pub const TupleExpression = struct {
    elements: ArrayList(*anyopaque),
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

pub const ChannelCreationExpression = struct {
    element_type: Type,
    buffer_size: ?Expression,
};

pub const ShookExpression = struct {
    expression: *Expression,
};

pub const MatchExpression = struct {
    expression: *Expression,
    cases: ArrayList(MatchCase),
    default_case: ?*Expression,
};

pub const MatchCase = struct {
    pattern: Pattern,
    guard: ?Expression,
    result: Expression,
};

pub const TypeSwitchExpression = struct {
    expression: *Expression,
    cases: ArrayList(TypeCase),
    default_case: ?*Expression,
};

pub const TypeCase = struct {
    type_pattern: Type,
    variable: ?[]const u8,
    result: Expression,
};

pub const BlockExpression = struct {
    statements: ArrayList(Statement),
};

pub const Literal = enum {
    Integer,
    Float,
    String,
    Boolean,
    Character,
    Null,
};

// Array/slice/map type definitions
pub const ArrayType = struct {
    element_type: *Type,
    size: ?usize,
};

pub const SliceType = struct {
    element_type: *Type,
};

pub const MapType = struct {
    key_type: *Type,
    value_type: *Type,
};

pub const ChannelType = struct {
    element_type: *Type,
    is_send_only: bool,
    is_receive_only: bool,
};

pub const FunctionType = struct {
    parameters: ArrayList(Type),
    return_type: ?*Type,
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
};

pub const PointerType = struct {
    target_type: *Type,
};

pub const FieldPattern = struct {
    name: []const u8,
    pattern: Pattern,
};

pub const StructPattern = struct {
    name: []const u8,
    fields: ArrayList(FieldPattern),
};
