const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

/// CURSED AST with Full Concurrency Support
/// 
/// This AST definition includes all CURSED language constructs with
/// complete concurrency feature support:
/// - Goroutines (stan keyword)
/// - Channels (dm<T> type)
/// - Select statements (ready keyword)
/// - Channel operations (send/receive)
/// - Goroutine spawning and management

pub const Node = struct {
    start_pos: u32,
    end_pos: u32,
};

pub const Identifier = struct {
    node: Node,
    value: []const u8,
};

pub const Program = struct {
    statements: ArrayList(Statement),
    
    pub fn init() Program {
        return Program{
            .statements = .empty,
        };
    }
    
    pub fn deinit(self: *Program) void {
        self.statements.deinit(self.allocator);
    }
};

pub const Statement = union(enum) {
    let_statement: *LetStatement,
    return_statement: *ReturnStatement,
    expression_statement: *ExpressionStatement,
    block_statement: *BlockStatement,
    function_statement: *FunctionStatement,
    goroutine_statement: *GoroutineStatement,
    select_statement: *SelectStatement,
    defer_statement: *DeferStatement,
    if_statement: *IfStatement,
    while_statement: *WhileStatement,
    for_statement: *ForStatement,
};

pub const Expression = union(enum) {
    identifier: *Identifier,
    integer_literal: *IntegerLiteral,
    string_literal: *StringLiteral,
    boolean_literal: *BooleanLiteral,
    float_literal: *FloatLiteral,
    function_literal: *FunctionLiteral,
    call_expression: *CallExpression,
    binary_expression: *BinaryExpression,
    unary_expression: *UnaryExpression,
    index_expression: *IndexExpression,
    
    // Concurrency expressions
    channel_literal: *ChannelLiteral,
    goroutine_spawn: *GoroutineSpawn,
    channel_send: *ChannelSend,
    channel_receive: *ChannelReceive,
    select_expression: *SelectExpression,
    
    // Advanced expressions
    struct_literal: *StructLiteral,
    array_literal: *ArrayLiteral,
    map_literal: *MapLiteral,
    match_expression: *MatchExpression,
    type_assertion: *TypeAssertion,
};

pub const Type = union(enum) {
    identifier: *Identifier,
    array_type: *ArrayType,
    map_type: *MapType,
    function_type: *FunctionType,
    channel_type: *ChannelType,
    interface_type: *InterfaceType,
    struct_type: *StructType,
    generic_type: *GenericType,
    
    // Basic types
    normie,  // int
    tea,     // string
    lit,     // bool
    meal,    // float
    smol,    // small int
    thicc,   // large int
};

// Basic statement types
pub const LetStatement = struct {
    node: Node,
    name: *Identifier,
    value: Expression,
    type_annotation: ?Type,
    is_mutable: bool,
};

pub const ReturnStatement = struct {
    node: Node,
    return_value: ?Expression,
};

pub const ExpressionStatement = struct {
    node: Node,
    expression: Expression,
};

pub const BlockStatement = struct {
    node: Node,
    statements: ArrayList(Statement),
    
    pub fn init() BlockStatement {
        return BlockStatement{
            .node = Node{ .start_pos = 0, .end_pos = 0 },
            .statements = .empty,
        };
    }
    
    pub fn deinit(self: *BlockStatement) void {
        self.statements.deinit(self.allocator);
    }
};

pub const FunctionStatement = struct {
    node: Node,
    name: *Identifier,
    parameters: ArrayList(*Parameter),
    return_type: ?Type,
    body: *BlockStatement,
    is_generic: bool,
    generic_params: ?ArrayList(*Identifier),
};

// Concurrency-specific statements
pub const GoroutineStatement = struct {
    node: Node,
    function: Expression,
    arguments: ArrayList(Expression),
};

pub const SelectStatement = struct {
    node: Node,
    cases: ArrayList(*SelectCase),
    default_case: ?*BlockStatement,
    timeout: ?Expression,
};

pub const SelectCase = struct {
    node: Node,
    operation: SelectOperation,
    body: *BlockStatement,
};

pub const SelectOperation = union(enum) {
    send: *ChannelSend,
    receive: *ChannelReceive,
    default: void,
};

pub const DeferStatement = struct {
    node: Node,
    call: *CallExpression,
};

// Control flow statements
pub const IfStatement = struct {
    node: Node,
    condition: Expression,
    consequence: *BlockStatement,
    alternative: ?*BlockStatement,
};

pub const WhileStatement = struct {
    node: Node,
    condition: Expression,
    body: *BlockStatement,
};

pub const ForStatement = struct {
    node: Node,
    init: ?Statement,
    condition: ?Expression,
    update: ?Expression,
    body: *BlockStatement,
};

// Basic expression types
pub const IntegerLiteral = struct {
    node: Node,
    value: i64,
};

pub const StringLiteral = struct {
    node: Node,
    value: []const u8,
};

pub const BooleanLiteral = struct {
    node: Node,
    value: bool,
};

pub const FloatLiteral = struct {
    node: Node,
    value: f64,
};

pub const FunctionLiteral = struct {
    node: Node,
    parameters: ArrayList(*Parameter),
    body: *BlockStatement,
    return_type: ?Type,
    is_generic: bool,
    generic_params: ?ArrayList(*Identifier),
    
    pub fn init() FunctionLiteral {
        return FunctionLiteral{
            .node = Node{ .start_pos = 0, .end_pos = 0 },
            .parameters = .empty,
            .body = undefined,
            .return_type = null,
            .is_generic = false,
            .generic_params = null,
        };
    }
    
    pub fn deinit(self: *FunctionLiteral) void {
        self.parameters.deinit(self.allocator);
        if (self.generic_params) |*params| {
            params.deinit();
        }
    }
};

pub const Parameter = struct {
    node: Node,
    name: *Identifier,
    type_annotation: Type,
    default_value: ?Expression,
};

pub const CallExpression = struct {
    node: Node,
    function: *Expression,
    arguments: ArrayList(Expression),
    
    pub fn init() CallExpression {
        return CallExpression{
            .node = Node{ .start_pos = 0, .end_pos = 0 },
            .function = undefined,
            .arguments = .empty,
        };
    }
    
    pub fn deinit(self: *CallExpression) void {
        self.arguments.deinit(self.allocator);
    }
};

pub const BinaryExpression = struct {
    node: Node,
    left: *Expression,
    operator: BinaryOperator,
    right: *Expression,
};

pub const BinaryOperator = enum {
    add,      // +
    subtract, // -
    multiply, // *
    divide,   // /
    modulo,   // %
    equal,    // ==
    not_equal, // !=
    less_than, // <
    greater_than, // >
    less_equal,   // <=
    greater_equal, // >=
    logical_and,  // &&
    logical_or,   // ||
    bitwise_and,  // &
    bitwise_or,   // |
    bitwise_xor,  // ^
    left_shift,   // <<
    right_shift,  // >>
    assign,       // =
    add_assign,   // +=
    sub_assign,   // -=
    mul_assign,   // *=
    div_assign,   // /=
};

pub const UnaryExpression = struct {
    node: Node,
    operator: UnaryOperator,
    operand: *Expression,
};

pub const UnaryOperator = enum {
    negate,    // -
    logical_not, // !
    bitwise_not, // ~
    address_of,  // &
    dereference, // *
};

pub const IndexExpression = struct {
    node: Node,
    left: *Expression,
    index: *Expression,
};

// Concurrency-specific expressions
pub const ChannelLiteral = struct {
    node: Node,
    element_type: Type,
    capacity: ?*Expression,
    is_send_only: bool,
    is_receive_only: bool,
};

pub const GoroutineSpawn = struct {
    node: Node,
    function: *Expression,
    arguments: ArrayList(Expression),
    
    pub fn init() GoroutineSpawn {
        return GoroutineSpawn{
            .node = Node{ .start_pos = 0, .end_pos = 0 },
            .function = undefined,
            .arguments = .empty,
        };
    }
    
    pub fn deinit(self: *GoroutineSpawn) void {
        self.arguments.deinit(self.allocator);
    }
};

pub const ChannelSend = struct {
    node: Node,
    channel: *Expression,
    value: *Expression,
};

pub const ChannelReceive = struct {
    node: Node,
    channel: *Expression,
    target: ?*Identifier, // Optional variable to assign result
};

pub const SelectExpression = struct {
    node: Node,
    cases: ArrayList(*SelectCase),
    default_case: ?*BlockStatement,
    timeout: ?Expression,
    
    pub fn init() SelectExpression {
        return SelectExpression{
            .node = Node{ .start_pos = 0, .end_pos = 0 },
            .cases = .empty,
            .default_case = null,
            .timeout = null,
        };
    }
    
    pub fn deinit(self: *SelectExpression) void {
        self.cases.deinit(self.allocator);
    }
};

// Advanced expression types
pub const StructLiteral = struct {
    node: Node,
    name: *Identifier,
    fields: ArrayList(*FieldValue),
    
    pub fn init() StructLiteral {
        return StructLiteral{
            .node = Node{ .start_pos = 0, .end_pos = 0 },
            .name = undefined,
            .fields = .empty,
        };
    }
    
    pub fn deinit(self: *StructLiteral) void {
        self.fields.deinit(self.allocator);
    }
};

pub const FieldValue = struct {
    node: Node,
    name: *Identifier,
    value: Expression,
};

pub const ArrayLiteral = struct {
    node: Node,
    elements: ArrayList(Expression),
    element_type: ?Type,
    
    pub fn init() ArrayLiteral {
        return ArrayLiteral{
            .node = Node{ .start_pos = 0, .end_pos = 0 },
            .elements = .empty,
            .element_type = null,
        };
    }
    
    pub fn deinit(self: *ArrayLiteral) void {
        self.elements.deinit(self.allocator);
    }
};

pub const MapLiteral = struct {
    node: Node,
    pairs: ArrayList(*MapPair),
    key_type: ?Type,
    value_type: ?Type,
    
    pub fn init() MapLiteral {
        return MapLiteral{
            .node = Node{ .start_pos = 0, .end_pos = 0 },
            .pairs = .empty,
            .key_type = null,
            .value_type = null,
        };
    }
    
    pub fn deinit(self: *MapLiteral) void {
        self.pairs.deinit(self.allocator);
    }
};

pub const MapPair = struct {
    node: Node,
    key: Expression,
    value: Expression,
};

pub const MatchExpression = struct {
    node: Node,
    value: *Expression,
    arms: ArrayList(*MatchArm),
    
    pub fn init() MatchExpression {
        return MatchExpression{
            .node = Node{ .start_pos = 0, .end_pos = 0 },
            .value = undefined,
            .arms = .empty,
        };
    }
    
    pub fn deinit(self: *MatchExpression) void {
        self.arms.deinit(self.allocator);
    }
};

pub const MatchArm = struct {
    node: Node,
    pattern: Pattern,
    guard: ?Expression,
    body: Expression,
};

pub const Pattern = union(enum) {
    identifier: *Identifier,
    literal: Expression,
    wildcard: void,
    struct_pattern: *StructPattern,
    array_pattern: *ArrayPattern,
    or_pattern: *OrPattern,
};

pub const StructPattern = struct {
    node: Node,
    name: *Identifier,
    fields: ArrayList(*FieldPattern),
};

pub const FieldPattern = struct {
    node: Node,
    name: *Identifier,
    pattern: Pattern,
};

pub const ArrayPattern = struct {
    node: Node,
    elements: ArrayList(Pattern),
};

pub const OrPattern = struct {
    node: Node,
    patterns: ArrayList(Pattern),
};

pub const TypeAssertion = struct {
    node: Node,
    expression: *Expression,
    target_type: Type,
};

// Type definitions
pub const ArrayType = struct {
    node: Node,
    element_type: *Type,
    size: ?*Expression,
};

pub const MapType = struct {
    node: Node,
    key_type: *Type,
    value_type: *Type,
};

pub const FunctionType = struct {
    node: Node,
    parameters: ArrayList(Type),
    return_type: *Type,
    
    pub fn init() FunctionType {
        return FunctionType{
            .node = Node{ .start_pos = 0, .end_pos = 0 },
            .parameters = .empty,
            .return_type = undefined,
        };
    }
    
    pub fn deinit(self: *FunctionType) void {
        self.parameters.deinit(self.allocator);
    }
};

pub const ChannelType = struct {
    node: Node,
    element_type: *Type,
    is_send_only: bool,
    is_receive_only: bool,
    capacity: ?*Expression,
};

pub const InterfaceType = struct {
    node: Node,
    name: *Identifier,
    methods: ArrayList(*MethodSignature),
    
    pub fn init() InterfaceType {
        return InterfaceType{
            .node = Node{ .start_pos = 0, .end_pos = 0 },
            .name = undefined,
            .methods = .empty,
        };
    }
    
    pub fn deinit(self: *InterfaceType) void {
        self.methods.deinit(self.allocator);
    }
};

pub const MethodSignature = struct {
    node: Node,
    name: *Identifier,
    parameters: ArrayList(Type),
    return_type: ?Type,
};

pub const StructType = struct {
    node: Node,
    name: *Identifier,
    fields: ArrayList(*FieldDefinition),
    is_generic: bool,
    generic_params: ?ArrayList(*Identifier),
    
    pub fn init() StructType {
        return StructType{
            .node = Node{ .start_pos = 0, .end_pos = 0 },
            .name = undefined,
            .fields = .empty,
            .is_generic = false,
            .generic_params = null,
        };
    }
    
    pub fn deinit(self: *StructType) void {
        self.fields.deinit(self.allocator);
        if (self.generic_params) |*params| {
            params.deinit();
        }
    }
};

pub const FieldDefinition = struct {
    node: Node,
    name: *Identifier,
    field_type: Type,
    is_public: bool,
    default_value: ?Expression,
};

pub const GenericType = struct {
    node: Node,
    base_type: *Type,
    type_arguments: ArrayList(Type),
    
    pub fn init() GenericType {
        return GenericType{
            .node = Node{ .start_pos = 0, .end_pos = 0 },
            .base_type = undefined,
            .type_arguments = .empty,
        };
    }
    
    pub fn deinit(self: *GenericType) void {
        self.type_arguments.deinit(self.allocator);
    }
};

// Utility functions for AST manipulation
pub fn createIdentifier(allocator: Allocator, name: []const u8) !*Identifier {
    const identifier = try allocator.create(Identifier);
    identifier.* = Identifier{
        .node = Node{ .start_pos = 0, .end_pos = 0 },
        .value = name,
    };
    return identifier;
}

pub fn createIntegerLiteral(allocator: Allocator, value: i64) !*IntegerLiteral {
    const literal = try allocator.create(IntegerLiteral);
    literal.* = IntegerLiteral{
        .node = Node{ .start_pos = 0, .end_pos = 0 },
        .value = value,
    };
    return literal;
}

pub fn createChannelLiteral(allocator: Allocator, element_type: Type, capacity: ?*Expression) !*ChannelLiteral {
    const channel = try allocator.create(ChannelLiteral);
    channel.* = ChannelLiteral{
        .node = Node{ .start_pos = 0, .end_pos = 0 },
        .element_type = element_type,
        .capacity = capacity,
        .is_send_only = false,
        .is_receive_only = false,
    };
    return channel;
}

pub fn createGoroutineSpawn(allocator: Allocator, function: *Expression) !*GoroutineSpawn {
    const spawn = try allocator.create(GoroutineSpawn);
    spawn.* = GoroutineSpawn.init(allocator);
    spawn.function = function;
    return spawn;
}

pub fn createSelectExpression(allocator: Allocator) !*SelectExpression {
        _ = allocator;
    const select_expr = try allocator.create(SelectExpression);
    select_expr.* = SelectExpression.init(allocator);
    return select_expr;
}

// Tests
test "AST creation and basic operations" {
    const allocator = std.testing.allocator;
    
    // Test program creation
    var program = Program.init(allocator);
    defer program.deinit();
    
    // Test identifier creation
    const ident = try createIdentifier(allocator, "test_var");
    defer allocator.destroy(ident);
    
    try std.testing.expectEqualStrings("test_var", ident.value);
}

test "concurrency AST nodes" {
    const allocator = std.testing.allocator;
    
    // Test channel literal creation
    const channel = try createChannelLiteral(allocator, Type.normie, null);
    defer allocator.destroy(channel);
    
    try std.testing.expect(channel.element_type == Type.normie);
    try std.testing.expect(channel.capacity == null);
    
    // Test goroutine spawn creation
    const function_expr = try allocator.create(Expression);
    defer allocator.destroy(function_expr);
    function_expr.* = Expression{ .identifier = try createIdentifier(allocator, "test_func") };
    defer allocator.destroy(function_expr.identifier);
    
    const spawn = try createGoroutineSpawn(allocator, function_expr);
    defer {
        spawn.deinit();
        allocator.destroy(spawn);
    }
    
    try std.testing.expect(spawn.function == function_expr);
    
    // Test select expression creation
    const select_expr = try createSelectExpression(allocator);
    defer {
        select_expr.deinit();
        allocator.destroy(select_expr);
    }
    
    try std.testing.expect(select_expr.cases.items.len == 0);
}
