const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Forward declarations to break circular dependencies
// Use opaque pointers that will be resolved at runtime
pub const ExpressionPtr = *Expression;
pub const StatementPtr = *Statement;

// AST Node base structure for memory management
pub const AstNode = struct {
    tag: AstNodeTag,
    allocator: Allocator,
    
    pub const AstNodeTag = enum {
        Expression,
        Statement,
        Type,
        Program,
    };
    
    pub fn deinit(self: *AstNode) void {
        switch (self.tag) {
            .Expression => {
                const expr: *Expression = @fieldParentPtr("base", self);
                expr.deinitInternal();
            },
            .Statement => {
                const stmt: *Statement = @fieldParentPtr("base", self);
                stmt.deinitInternal();
            },
            else => {},
        }
    }
};

// Expression types - separated into distinct structs to avoid circular dependency
pub const Expression = struct {
    base: AstNode,
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
        
        // Complex types using indices instead of direct pointers
        binary: BinaryExpressionData,
        call: CallExpressionData,
        member_access: MemberAccessData,
        unary: UnaryExpressionData,
        array: ArrayExpressionData,
        literal: Literal,
        
        // Channel operations
        channel_send: ChannelSendData,
        channel_receive: ChannelReceiveData,
        channel_creation: ChannelCreationData,
        
        // Advanced features
        struct_literal: StructLiteralData,
        lambda: LambdaData,
        tuple: TupleData,
        tuple_access: TupleAccessData,
        array_access: ArrayAccessData,
        slice_access: SliceAccessData,
        type_assertion: TypeAssertionData,
        
        // Control flow expressions
        match: MatchData,
        type_switch: TypeSwitchData,
        
        // Error handling
        shook: ShookData,
        error_value: ErrorValueData,
        structured_error: StructuredErrorData,
        panic: PanicData,
        recover: RecoverData,
        
        // Test expressions
        test_result: TestResultData,
        test_result_check: TestResultCheckData,
        
        // Other
        increment: IncrementData,
        decrement: DecrementData,
        range_for: RangeForData,
    };
    
    pub fn init(allocator: Allocator, kind: ExpressionKind) !*Expression {
        const expr = try allocator.create(Expression);
        expr.* = Expression{
            .base = AstNode{
                .tag = .Expression,
                .allocator = allocator,
            },
            .kind = kind,
        };
        return expr;
    }
    
    pub fn deinit(self: *Expression) void {
        self.base.deinit();
    }
    
    pub fn deinitInternal(self: *Expression) void {
        switch (self.kind) {
            .binary => |*binary| {
                binary.left.deinitInternal();
                binary.right.deinitInternal();
            },
            .call => |*call| {
                call.function.deinitInternal();
                for (call.arguments.items) |arg| {
                    arg.deinitInternal();
                }
                call.arguments.deinit();
            },
            .unary => |*unary| {
                unary.operand.deinitInternal();
            },
            .array => |*array| {
                for (array.elements.items) |elem| {
                    elem.deinitInternal();
                }
                array.elements.deinit();
            },
            // Add more cleanup cases as needed
            else => {},
        }
        self.base.allocator.destroy(self);
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

// Expression data structures (no self-references)
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

pub const ChannelSendData = struct {
    channel: *Expression,
    value: *Expression,
};

pub const ChannelReceiveData = struct {
    channel: *Expression,
};

pub const ChannelCreationData = struct {
    element_type: Type,
    buffer_size: ?*Expression,
};

pub const StructLiteralData = struct {
    struct_name: []const u8,
    fields: ArrayList(StructFieldAssignment),
};

pub const LambdaData = struct {
    parameters: ArrayList([]const u8),
    body: *Expression,
};

pub const TupleData = struct {
    elements: ArrayList(*Expression),
};

pub const TupleAccessData = struct {
    tuple: *Expression,
    index: usize,
};

pub const ArrayAccessData = struct {
    array: *Expression,
    index: *Expression,
};

pub const SliceAccessData = struct {
    array: *Expression,
    start: ?*Expression,
    end: ?*Expression,
};

pub const TypeAssertionData = struct {
    value: *Expression,
    target_type: Type,
    is_safe: bool,
};

pub const MatchData = struct {
    expression: *Expression,
    cases: ArrayList(MatchCase),
    default_case: ?*Expression,
};

pub const TypeSwitchData = struct {
    expression: *Expression,
    cases: ArrayList(TypeCase),
    default_case: ?*Expression,
};

pub const ShookData = struct {
    expression: *Expression,
};

pub const ErrorValueData = struct {
    message: []const u8,
};

pub const StructuredErrorData = struct {
    message: *Expression,
    code: ?*Expression,
    details: ?*Expression,
    fields: ArrayList(StructuredErrorField),
};

pub const PanicData = struct {
    message: *Expression,
};

pub const RecoverData = struct {};

pub const TestResultData = struct {
    test_name: []const u8,
    is_passing: bool,
};

pub const TestResultCheckData = struct {
    expression: *Expression,
};

pub const IncrementData = struct {
    variable: *Expression,
};

pub const DecrementData = struct {
    variable: *Expression,
};

pub const RangeForData = struct {
    iterable: *Expression,
};

// Statement types  
pub const Statement = struct {
    base: AstNode,
    kind: StatementKind,
    
    pub const StatementKind = union(enum) {
        expression: *Expression,
        let: LetStatementData,
        assignment: AssignmentStatementData,
        return_stmt: ReturnStatementData,
        if_stmt: IfStatementData,
        function: FunctionStatementData,
        while_stmt: WhileStatementData,
        for_stmt: ForStatementData,
        for_in: ForInStatementData,
        switch_stmt: SwitchStatementData,
        pattern_switch: PatternSwitchStatementData,
        goroutine: GoroutineStatementData,
        channel: ChannelStatementData,
        select_stmt: SelectStatementData,
        struct_stmt: StructStatementData,
        interface: InterfaceStatementData,
        type_alias: TypeAliasStatementData,
        panic_stmt: PanicStatementData,
        catch_stmt: CatchStatementData,
        defer_stmt: DeferStatementData,
        break_stmt: BreakStatementData,
        continue_stmt: ContinueStatementData,
        increment: IncrementStatementData,
        decrement: DecrementStatementData,
        short_declaration: ShortDeclarationStatementData,
        yikes: YikesStatementData,
        fam: FamStatementData,
        const_decl: ConstDeclData,
    };
    
    pub fn init(allocator: Allocator, kind: StatementKind) !*Statement {
        const stmt = try allocator.create(Statement);
        stmt.* = Statement{
            .base = AstNode{
                .tag = .Statement,
                .allocator = allocator,
            },
            .kind = kind,
        };
        return stmt;
    }
    
    pub fn deinit(self: *Statement) void {
        self.base.deinit();
    }
    
    pub fn deinitInternal(self: *Statement) void {
        switch (self.kind) {
            .expression => |expr| {
                expr.deinitInternal();
            },
            .let => |*let| {
                if (let.var_type) |*var_type| {
                    var_type.deinit(self.base.allocator);
                }
                if (let.initializer) |initializer| {
                    initializer.deinitInternal();
                }
            },
            .function => |*func| {
                for (func.body.items) |stmt| {
                    stmt.deinitInternal();
                }
                func.body.deinit();
                func.parameters.deinit();
                func.type_parameters.deinit();
                func.comments.deinit();
            },
            // Add more cleanup cases as needed
            else => {},
        }
        self.base.allocator.destroy(self);
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
            .let => |let| {
                print_fn("{s}Let: {s} = ", .{ spaces[0..indent], let.name });
                if (let.initializer) |initializer| {
                    try initializer.print(0);
                }
                print_fn("\n", .{});
            },
            .function => |func| {
                print_fn("{s}Function: {s}\n", .{ spaces[0..indent], func.name });
                for (func.body.items) |stmt| {
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

pub const ForStatementData = struct {
    init: ?*Statement,
    condition: ?*Expression,
    update: ?*Statement,
    body: ArrayList(*Statement),
};

pub const ForInStatementData = struct {
    variable: []const u8,
    iterable: *Expression,
    body: ArrayList(*Statement),
};

pub const SwitchStatementData = struct {
    expression: *Expression,
    cases: ArrayList(SwitchCase),
    default_case: ?ArrayList(*Statement),
};

pub const PatternSwitchStatementData = struct {
    expression: *Expression,
    patterns: ArrayList(PatternCase),
    default_case: ?ArrayList(*Statement),
};

pub const GoroutineStatementData = struct {
    call: CallExpressionData,
};

pub const ChannelStatementData = struct {
    name: []const u8,
    channel_type: Type,
    buffer_size: ?*Expression,
};

pub const SelectStatementData = struct {
    cases: ArrayList(SelectCase),
    default_case: ?ArrayList(*Statement),
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

pub const TypeAliasStatementData = struct {
    name: []const u8,
    target_type: Type,
    visibility: Visibility,
};

pub const PanicStatementData = struct {
    message: *Expression,
};

pub const CatchStatementData = struct {
    body: ArrayList(*Statement),
    error_variable: ?[]const u8,
    error_type: ?Type,
};

pub const DeferStatementData = struct {
    statement: *Statement,
};

pub const BreakStatementData = struct {};

pub const ContinueStatementData = struct {};

pub const IncrementStatementData = struct {
    variable: *Expression,
};

pub const DecrementStatementData = struct {
    variable: *Expression,
};

pub const ShortDeclarationStatementData = struct {
    names: ArrayList([]const u8),
    values: ArrayList(*Expression),
};

pub const YikesStatementData = struct {
    name: []const u8,
    error_type: ?Type,
    value: ?*Expression,
};

pub const FamStatementData = struct {
    body: ArrayList(*Statement),
    recovery_body: ?ArrayList(*Statement),
    error_variable: ?[]const u8,
};

pub const ConstDeclData = struct {
    name: []const u8,
    const_type: ?Type,
    value: *Expression,
    visibility: Visibility,
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

// Type system - simplified to avoid circular dependencies
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

pub const StructFieldAssignment = struct {
    field_name: []const u8,
    value: *Expression,
};

pub const StructuredErrorField = struct {
    name: []const u8,
    value: *Expression,
};

pub const SwitchCase = struct {
    value: *Expression,
    body: ArrayList(*Statement),
};

pub const PatternCase = struct {
    pattern: Pattern,
    guard: ?*Expression,
    body: ArrayList(*Statement),
};

pub const SelectCase = struct {
    channel_op: ChannelOperation,
    body: ArrayList(*Statement),
};

pub const ChannelOperation = union(enum) {
    Send: struct {
        channel: *Expression,
        value: *Expression,
    },
    Receive: struct {
        channel: *Expression,
        variable: ?[]const u8,
    },
};

pub const MatchCase = struct {
    pattern: Pattern,
    guard: ?*Expression,
    result: *Expression,
};

pub const TypeCase = struct {
    type_pattern: Type,
    variable: ?[]const u8,
    result: *Expression,
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
    return Expression.init(allocator, .{ .binary = BinaryExpressionData{
        .left = left,
        .operator = operator,
        .right = right,
    }});
}

pub fn createCallExpression(allocator: Allocator, function: *Expression, arguments: ArrayList(*Expression)) !*Expression {
    return Expression.init(allocator, .{ .call = CallExpressionData{
        .function = function,
        .arguments = arguments,
    }});
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
        },
        else => try std.testing.expect(false),
    }
}
