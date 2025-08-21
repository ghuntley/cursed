const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Import attribute system for AST node decoration
const attribute_system = @import("attribute_system.zig");
const AttributeList = attribute_system.AttributeList;

/// Enhanced source location for error reporting and DWARF debug information
pub const SourceLocation = struct {
    file: []const u8,
    line: u32,
    column: u32,
    offset: u32,
    
    /// Create a default source location
    pub fn init(file: []const u8, line: u32, column: u32, offset: u32) SourceLocation {
        return SourceLocation{
            .file = file,
            .line = line,
            .column = column,
            .offset = offset,
        };
    }
    
    /// Create unknown location for cases where source info is not available
    pub fn unknown() SourceLocation {
        return SourceLocation{
            .file = "<unknown>",
            .line = 0,
            .column = 0,
            .offset = 0,
        };
    }
};

// Forward declaration for Expression type
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
    MemberAccess: *MemberAccessExpression,
    Literal: Literal,
    Unary: *UnaryExpression,
    Array: *ArrayExpression,
    Map: *MapExpression,
    CompositeLiteral: CompositeLiteralExpression,
    ChannelSend: ChannelSendExpression,
    ChannelReceive: ChannelReceiveExpression,
    ChannelCreation: ChannelCreationExpression,
    StructLiteral: StructLiteralExpression,
    Struct: *StructExpression,
    MethodCall: *MethodCallExpression,
    Lambda: LambdaExpression,
    Tuple: TupleExpression,
    TupleAccess: TupleAccessExpression,
    ArrayAccess: ArrayAccessExpression,
    SliceAccess: SliceAccessExpression,
    TernaryOperator: TernaryExpression,
    TypeAssertion: TypeAssertionExpression,
    Increment: IncrementExpression,
    Decrement: DecrementExpression,
    Yikes: YikesExpression,
    Shook: ShookExpression,
    Fam: FamExpression,
    ErrorValue: ErrorValueExpression,
    StructuredError: StructuredErrorExpression,
    Panic: PanicExpression,
    Recover: RecoverExpression,
    TestResult: TestResultExpression,
    TestResultCheck: TestResultCheckExpression,
    RangeFor: RangeForExpression,
    Match: MatchExpression,
    TypeSwitch: TypeSwitchExpression,
    StringInterpolation: StringInterpolationExpression,
    AwaitExpression: AwaitExpressionType,
    Loop: LoopExpression,
    For: ForExpression,
    While: WhileExpression,
    Block: BlockExpression,
    If: IfExpression,
    FunctionCall: FunctionCallExpression,

    pub fn deinit(self: *Expression, allocator: Allocator) void {
        // Memory cleanup is now handled properly without circular dependencies
        // Using type-erased pointers with runtime casting for safe cleanup
        switch (self.*) {
            .Binary => |bin| {
                const left_expr: *Expression = @ptrCast(@alignCast(bin.left));
                const right_expr: *Expression = @ptrCast(@alignCast(bin.right));
                left_expr.deinit();
                right_expr.deinit();
                allocator.destroy(left_expr);
                allocator.destroy(right_expr);
            },
            .Call => |*call| {
                call.deinit();
            },
            .MemberAccess => |member| {
                allocator.destroy(member);
            },
            .Unary => |unary| {
                allocator.destroy(unary);
            },
            .Array => |array| {
                allocator.destroy(array);
            },
            .Map => |map| {
                allocator.destroy(map);
            },
            .Struct => |struct_expr| {
                allocator.destroy(struct_expr);
            },
            .MethodCall => |method_call| {
                allocator.destroy(method_call);
            },
            .StringInterpolation => |interpolation| {
                for (interpolation.parts.items) |part| {
                    if (part.expression) |expr| {
                        const expr_ptr: *Expression = @ptrCast(@alignCast(expr));
                        expr_ptr.deinit();
                        allocator.destroy(expr_ptr);
                    }
                }
                interpolation.parts.deinit();
            },
            else => {}, // Simple types don't need special cleanup
        }
    }

    pub fn print(self: *anyopaque, indent: usize) !void {
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
            .StringInterpolation => |interpolation| {
                print_fn("StringInterpolation[", .{});
                for (interpolation.parts.items, 0..) |part, i| {
                    if (i > 0) print_fn(", ", .{});
                    if (part.expression != null) {
                        print_fn("${{}}", .{"expr"});
                    } else {
                        print_fn("\"{}\"", .{part.text});
                    }
                }
                print_fn("]", .{});
            },
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

pub const Program = struct {
    statements: ArrayList(*anyopaque),
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
        for (self.statements.items) |stmt| {
            const stmt_ptr: *Statement = @ptrCast(@alignCast(stmt));
            stmt_ptr.deinit();
            allocator.destroy(stmt_ptr);
        }
        self.statements.deinit();
        
        for (self.imports.items) |*import| {
            import.deinit();
        }
        self.imports.deinit();
        
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
            try stmt.print(indent + 2);
        }
    }
};

pub const ImportItem = struct {
    name: []const u8,
    alias: ?[]const u8,
};

pub const ImportStatement = struct {
    // Core import information
    path: []const u8,
    alias: ?[]const u8,
    
    // Multiple imports support: yeet "mod1", "mod2", "mod3"
    multiple_paths: ArrayList([]const u8),
    
    // Selective imports support: yeet { func1, func2 } from "module"
    selective_items: ArrayList(ImportItem),
    is_selective: bool,
    
    // Version specification support: yeet "module@^1.0.0"
    version: ?[]const u8,

    pub fn init(_: Allocator, path: []const u8) ImportStatement {
                return ImportStatement{
            .path = path,
            .alias = null,
            .multiple_paths = .empty,
            .selective_items = .empty,
            .is_selective = false,
            .version = null,
        };
    }

    pub fn deinit(self: *ImportStatement, _: Allocator) void {
        self.multiple_paths.deinit();
        self.selective_items.deinit();
    }
    
    pub fn addMultiplePath(self: *ImportStatement, _: Allocator, path: []const u8) !void {
        try self.multiple_paths.append(path);
    }
    
    pub fn addSelectiveItem(self: *ImportStatement, _: Allocator, name: []const u8, item_alias: ?[]const u8) !void {
        try self.selective_items.append(ImportItem{ .name = name, .alias = item_alias });
        self.is_selective = true;
    }
};

pub const PackageDeclaration = struct {
    name: []const u8,
    version: ?[]const u8,

    pub fn deinit(self: *PackageDeclaration, _: Allocator) void {
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
    Custom: []const u8, // For user-defined types and identifiers
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

    pub fn deinit(self: *Type, _: Allocator) void {
        switch (self.*) {
            .Array => |*arr| arr.deinit(),
            .Map => |*map| map.deinit(),
            .Function => |*func| func.deinit(),
            .Tuple => |*tuple| tuple.deinit(),
            else => {},
        }
    }
};

pub const BasicType = enum {
    Normie,    // i32
    Drip,      // f32/f64 (legacy float type)
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
    _owned: bool = true, // Track ownership to prevent double-free

    pub fn deinit(self: *ArrayType, _: Allocator) void {
        // TEMPORARY FIX: Skip cleanup entirely to prevent double-free
        // The module loader manages the lifetime of these type pointers
        // This causes a small memory leak but prevents crashes
        _ = self;
                // TODO: Implement proper reference counting system
    }
};

pub const SliceType = struct {
    element_type: *Type,
};

pub const MapType = struct {
    key_type: *Type,
    value_type: *Type,
    _key_owned: bool = true,    // Track key ownership to prevent double-free
    _value_owned: bool = true,  // Track value ownership to prevent double-free

    pub fn deinit(self: *MapType, _: Allocator) void {
        // TEMPORARY FIX: Skip cleanup entirely to prevent double-free
        // The module loader manages the lifetime of these type pointers
        _ = self;
                // TODO: Implement proper reference counting system
    }
};

pub const PointerType = struct {
    target_type: *Type,
};

pub const FunctionType = struct {
    parameters: ArrayList(Type),
    return_type: ?*Type,
    _return_owned: bool = true, // Track return type ownership

    pub fn deinit(self: *FunctionType, _: Allocator) void {
        // Clean up parameters (these are value types, safe to cleanup)
        for (self.parameters.items) |*param| {
            param.deinit();
        }
        self.parameters.deinit();
        
        // TEMPORARY FIX: Skip return type cleanup to prevent double-free
        _ = self._return_owned;
        // TODO: Implement proper reference counting for return_type
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
    constraints: ArrayList(TypeConstraint),
    type_arguments: ArrayList(Type),
};

pub const TupleType = struct {
    elements: ArrayList(Type),

    pub fn deinit(self: *TupleType, _: Allocator) void {
        for (self.elements.items) |*elem| {
            elem.deinit();
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
    default_value: ?*anyopaque,
};

pub const TypeParameter = struct {
    name: []const u8,
    constraints: ArrayList(Type),
};

pub const Statement = union(enum) {
    Expression: *anyopaque,
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
    Stan: StanStatement,
    Channel: ChannelStatement,
    Select: SelectStatement,
    Struct: StructStatement,
    Interface: InterfaceStatement,
    Implementation: ImplementationStatement,
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
    Block: BlockStatement,

    pub fn deinit(self: *Statement, allocator: Allocator) void {
        switch (self.*) {
            .Expression => |expr| {
                const expr_ptr: *Expression = @ptrCast(@alignCast(expr));
                expr_ptr.deinit();
                allocator.destroy(expr_ptr);
            },
            .Let => |*let| let.deinit(),
            .Assignment => |*assign| assign.deinit(),
            .Return => |*ret| ret.deinit(),
            .If => |*if_stmt| if_stmt.deinit(),
            .Function => |*func| func.deinit(),
            .While => |*while_stmt| while_stmt.deinit(),
            .Interface => |*interface_stmt| interface_stmt.deinit(),
            .Implementation => |*impl_stmt| impl_stmt.deinit(),
            .Stan => |*stan| stan.deinit(),
            .Block => |*block| block.deinit(),
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

pub const MapEntry = struct {
    key: *Expression,
    value: *Expression,
};

pub const Literal = union(enum) {
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    Character: u8,
    Null,
    Nil,
};

// Statement structures
pub const LetStatement = struct {
    name: []const u8,
    var_type: ?Type,
    type_annotation: ?Type,
    initializer: ?*Expression,
    is_mutable: bool,
    location: SourceLocation = SourceLocation.unknown(), // Debug location for DWARF

    pub fn deinit(self: *LetStatement, allocator: Allocator) void {
        // TEMPORARY FIX: Skip type cleanup to prevent double-free
        // The module loader manages type lifetimes
        _ = self.var_type;
        _ = self.type_annotation;
        
        // Only cleanup the initializer (expression) which is usually safe
        if (self.initializer) |init| {
            init.deinit();
            allocator.destroy(init);
        }
        // TODO: Implement proper reference counting for types
    }
};

pub const AssignmentStatement = struct {
    target: *anyopaque,
    value: *anyopaque,
    operator: []const u8,

    pub fn deinit(self: *AssignmentStatement, allocator: Allocator) void {
        const target_expr: *Expression = @ptrCast(@alignCast(self.target));
        target_expr.deinit();
        allocator.destroy(target_expr);
        
        const value_expr: *Expression = @ptrCast(@alignCast(self.value));
        value_expr.deinit();
        allocator.destroy(value_expr);
    }
};

pub const ReturnStatement = struct {
    value: ?*anyopaque,

    pub fn deinit(self: *ReturnStatement, allocator: Allocator) void {
        if (self.value) |val| {
            const expr_ptr: *Expression = @ptrCast(@alignCast(val));
            expr_ptr.deinit();
            allocator.destroy(expr_ptr);
        }
    }
};

pub const IfStatement = struct {
    condition: *anyopaque,
    then_branch: ArrayList(*anyopaque),
    else_branch: ?ArrayList(*anyopaque),

    pub fn deinit(self: *IfStatement, allocator: Allocator) void {
        const condition_expr: *Expression = @ptrCast(@alignCast(self.condition));
        condition_expr.deinit();
        allocator.destroy(condition_expr);
        // Note: Individual statement cleanup handled by parent scope
        self.then_branch.deinit();
        
        if (self.else_branch) |*else_br| {
            // Note: Individual statement cleanup handled by parent scope  
            else_br.deinit();
        }
    }
};

pub const FunctionStatement = struct {
    name: []const u8,
    parameters: ArrayList(Parameter),
    return_type: ?Type,
    body: ArrayList(*Statement),
    visibility: Visibility,
    is_async: bool,
    type_parameters: ArrayList(TypeParameter),
    comments: ArrayList(Comment),
    location: SourceLocation = SourceLocation.unknown(), // Debug location for DWARF
    attributes: ?AttributeList = null, // Attribute decorations for code generation

    pub fn init(_: Allocator, name: []const u8) FunctionStatement {
                return FunctionStatement{
            .name = name,
            .parameters = .empty,
            .return_type = null,
            .body = .empty,
            .visibility = .Private,
            .is_async = false,
            .type_parameters = .empty,
            .comments = .empty,
            .location = SourceLocation.unknown(),
            .attributes = null,
        };
    }

    pub fn deinit(self: *FunctionStatement, allocator: Allocator) void {
        for (self.parameters.items) |*param| {
            param.param_type.deinit();
            if (param.default_value) |default| {
                const default_expr: *Expression = @ptrCast(@alignCast(default));
                default_expr.deinit();
            }
        }
        self.parameters.deinit();
        
        if (self.return_type) |*ret_type| {
            ret_type.deinit();
        }
        
        for (self.body.items) |stmt| {
            stmt.deinit();
            allocator.destroy(stmt);
        }
        self.body.deinit();
        
        for (self.type_parameters.items) |*type_param| {
            for (type_param.constraints.items) |*constraint| {
                constraint.deinit();
            }
            type_param.constraints.deinit();
        }
        self.type_parameters.deinit();
        
        self.comments.deinit();
        
        if (self.attributes) |*attrs| {
            attrs.deinit();
        }
    }
};

pub const WhileStatement = struct {
    condition: *Expression,
    body: ArrayList(*Statement),

    pub fn deinit(self: *WhileStatement, allocator: Allocator) void {
        self.condition.deinit();
        allocator.destroy(self.condition);
        for (self.body.items) |stmt| {
            stmt.deinit();
            allocator.destroy(stmt);
        }
        self.body.deinit();
    }
};

pub const ForStatement = struct {
    init: ?*Statement,
    condition: ?*Expression,
    update: ?*Statement,
    body: ArrayList(*Statement),
};

pub const ForInStatement = struct {
    variable: []const u8,
    iterable: *Expression,
    body: ArrayList(*Statement),
};

pub const SwitchStatement = struct {
    expression: *anyopaque,
    cases: ArrayList(SwitchCase),
    default_case: ?ArrayList(*anyopaque),
};

pub const PatternSwitchStatement = struct {
    expression: *Expression,
    patterns: ArrayList(PatternCase),
    default_case: ?ArrayList(*Statement),
};

pub const GoroutineStatement = struct {
    call: CallExpression,
};

pub const StanStatement = struct {
    body: ArrayList(*anyopaque),
    
    pub fn deinit(self: *StanStatement, _: Allocator) void {
        for (self.body.items) |stmt| {
            const stmt_ptr: *Statement = @ptrCast(@alignCast(stmt));
            stmt_ptr.deinit();
        }
        self.body.deinit();
    }
};

pub const ChannelStatement = struct {
    name: []const u8,
    channel_type: Type,
    buffer_size: ?*anyopaque,
};

pub const SelectStatement = struct {
    cases: ArrayList(SelectCase),
    default_case: ?ArrayList(*Statement),
};

pub const StructStatement = struct {
    name: []const u8,
    fields: ArrayList(StructField),
    methods: ArrayList(FunctionStatement),
    visibility: Visibility,
    type_parameters: ArrayList(TypeParameter),
    attributes: ?AttributeList = null, // Attribute decorations for memory layout and optimization
    
    pub fn deinit(self: *StructStatement, _: Allocator) void {
        for (self.fields.items) |*field| {
            field.field_type.deinit();
        }
        self.fields.deinit();
        for (self.methods.items) |*method| {
            method.deinit();
        }
        self.methods.deinit();
        for (self.type_parameters.items) |*type_param| {
            type_param.constraints.deinit();
        }
        self.type_parameters.deinit();
        
        if (self.attributes) |*attrs| {
            attrs.deinit();
        }
    }
};

pub const InterfaceStatement = struct {
    name: []const u8,
    methods: ArrayList(MethodSignature),
    visibility: Visibility,
    type_parameters: ArrayList(TypeParameter),
    extends: ArrayList([]const u8), // Interface inheritance  
    compositions: ArrayList([]const u8), // Interface composition with "with"
    
    pub fn deinit(self: *InterfaceStatement, _: Allocator) void {
                self.methods.deinit();
        self.type_parameters.deinit();
        self.extends.deinit();
        self.compositions.deinit();
    }
};

pub const ImplementationStatement = struct {
    implementing_type: []const u8,
    interface_name: []const u8,
    methods: ArrayList(FunctionStatement),
    where_clause: ?[]const u8,
    
    pub fn deinit(self: *ImplementationStatement, _: Allocator) void {
        for (self.methods.items) |*method| {
            method.deinit();
        }
        self.methods.deinit();
    }
};

pub const TypeAliasStatement = struct {
    name: []const u8,
    target_type: Type,
    visibility: Visibility,
};

pub const PanicStatement = struct {
    message: *anyopaque,
};

pub const CatchStatement = struct {
    body: ArrayList(*anyopaque),
    error_variable: ?[]const u8,
    error_type: ?Type,
};

pub const DeferStatement = struct {
    statement: *anyopaque,
};

pub const BreakStatement = struct {};

pub const ContinueStatement = struct {};

pub const IncrementStatement = struct {
    variable: *anyopaque,
};

pub const DecrementStatement = struct {
    variable: *anyopaque,
};

pub const ShortDeclarationStatement = struct {
    names: ArrayList([]const u8),
    values: ArrayList(*Expression),
};

pub const YikesStatement = struct {
    message: *Expression,  // Error message expression
    error_type: ?[]const u8,  // Optional error type
    location: ?SourceLocation,
};

pub const FamStatement = struct {
    try_body: ArrayList(Statement),  // Code to try
    catch_blocks: ArrayList(CatchBlock),  // Catch handlers
    finally_block: ?ArrayList(Statement),  // Finally block
    
    pub const CatchBlock = struct {
        error_variable: ?[]const u8,  // Variable to bind error to
        error_type: ?[]const u8,  // Type of error to catch (optional)
        body: ArrayList(Statement),  // Handler code
    };
};

pub const ConstDecl = struct {
    name: []const u8,
    const_type: ?Type,
    value: *anyopaque,
    visibility: Visibility,
};

/// Block statement for grouping statements
pub const BlockStatement = struct {
    statements: ArrayList(*anyopaque),
    
    pub fn init() BlockStatement {
        return BlockStatement{
            .statements = .empty,
        };
    }
    
    pub fn deinit(self: *BlockStatement, allocator: Allocator) void {
        for (self.statements.items) |stmt| {
            const stmt_ptr: *Statement = @ptrCast(@alignCast(stmt));
            stmt_ptr.deinit();
            allocator.destroy(stmt_ptr);
        }
        self.statements.deinit();
    }
};

// Expression structures
pub const BinaryExpression = struct {
    left: *Expression,
    operator: []const u8,
    right: *Expression,

    pub fn deinit(self: *BinaryExpression, allocator: Allocator) void {
        self.left.deinit();
        self.right.deinit();
        allocator.destroy(self.left);
        allocator.destroy(self.right);
    }
};

pub const CallExpression = struct {
    function: *Expression,
    arguments: ArrayList(*Expression),

    pub fn deinit(self: *CallExpression, allocator: Allocator) void {
        self.function.deinit();
        allocator.destroy(self.function);
        
        for (self.arguments.items) |arg| {
            arg.deinit();
            allocator.destroy(arg);
        }
        self.arguments.deinit();
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
    elements: ArrayList(*anyopaque),
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
    buffer_size: ?*Expression,
};

pub const StructLiteralExpression = struct {
    struct_name: []const u8,
    fields: ArrayList(StructFieldAssignment),
};

pub const StructFieldAssignment = struct {
    field_name: []const u8,
    value: *Expression,
};

pub const FieldInitializer = struct {
    field_name: []const u8,
    value: *Expression,
};

pub const StructExpression = struct {
    struct_name: []const u8,
    fields: ArrayList(FieldInitializer),
    
    pub fn deinit(self: *StructExpression, allocator: Allocator) void {
        for (self.fields.items) |*field| {
            field.value.deinit();
            allocator.destroy(field.value);
        }
        self.fields.deinit();
    }
};

pub const MethodCallExpression = struct {
    object: *Expression,
    method_name: []const u8,
    arguments: ArrayList(*Expression),
    
    pub fn deinit(self: *MethodCallExpression, allocator: Allocator) void {
        self.object.deinit();
        allocator.destroy(self.object);
        
        for (self.arguments.items) |arg| {
            arg.deinit();
            allocator.destroy(arg);
        }
        self.arguments.deinit();
    }
};

pub const LambdaExpression = struct {
    parameters: ArrayList([]const u8),
    body: *Expression,
};

pub const ArrayExpression = struct {
    elements: ArrayList(*Expression),
};

pub const MapExpression = struct {
    entries: ArrayList(MapEntry),
};

pub const TupleExpression = struct {
    elements: ArrayList(*Expression),
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

pub const TernaryExpression = struct {
    condition: *Expression,
    true_expr: *Expression,
    false_expr: *Expression,
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

/// YIKES - Error creation expression
pub const YikesExpression = struct {
    message: *Expression,
    code: ?*Expression,
    source_location: ?SourceLocation,
};

/// SHOOK - Error propagation expression  
pub const ShookExpression = struct {
    expression: *Expression,
    catch_handler: ?*Expression, // Optional immediate catch
};

/// FAM - Panic recovery block
pub const FamExpression = struct {
    try_body: ArrayList(*anyopaque), // Points to Statement
    catch_handler: ?CatchHandler,
    finally_handler: ?FinallyHandler,
    
    pub const CatchHandler = struct {
        error_variable: []const u8,
        handler_body: ArrayList(*anyopaque), // Points to Statement
    };
    
    pub const FinallyHandler = struct {
        finally_body: ArrayList(*anyopaque), // Points to Statement
    };
};

pub const ErrorValueExpression = struct {
    message: []const u8,
    code: i64,
};

pub const StructuredErrorExpression = struct {
    message: *Expression,
    code: ?*Expression,
    details: ?*Expression,
    fields: ArrayList(StructuredErrorField),
};

pub const StructuredErrorField = struct {
    name: []const u8,
    value: *anyopaque,
};

pub const PanicExpression = struct {
    message: *Expression,
};

pub const RecoverExpression = struct {
    context: ?*anyopaque, // Optional recovery context
};

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
    value: *anyopaque,
    body: ArrayList(*anyopaque),
};

pub const PatternCase = struct {
    pattern: Pattern,
    guard: ?*Expression,
    body: ArrayList(*Statement),
};

pub const SelectCase = struct {
    channel_op: ChannelOperation,
    body: ArrayList(*anyopaque),
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
    guard: ?*anyopaque,
    result: *anyopaque,
};

pub const TypeCase = struct {
    type_pattern: Type,
    variable: ?[]const u8,
    result: *anyopaque,
};

pub const Pattern = union(enum) {
    Wildcard,
    Literal: Literal,
    Variable: []const u8,
    Tuple: ArrayList(Pattern),
    Struct: StructPattern,
    Array: ArrayList(Pattern),
    Range: RangePattern,
    Guard: GuardPattern,
    Or: OrPattern,
    Enum: EnumPattern,
};

pub const StructPattern = struct {
    name: []const u8,
    fields: ArrayList(FieldPattern),
};

pub const FieldPattern = struct {
    name: []const u8,
    pattern: Pattern,
};

/// Range pattern for pattern matching (1..10, 'a'..'z')
pub const RangePattern = struct {
    start: *Expression,
    end: *Expression,
    is_inclusive: bool = true,
};

/// Guard pattern with condition (x when x > 0)
pub const GuardPattern = struct {
    pattern: *Pattern,
    guard: *Expression,
};

/// Or pattern (multiple alternatives: x | y | z)
pub const OrPattern = struct {
    patterns: ArrayList(Pattern),
};

/// Enum variant pattern matching
pub const EnumPattern = struct {
    enum_name: []const u8,
    variant_name: []const u8,
    patterns: ArrayList(Pattern), // For associated data
};

// Type constraint definitions for generics
pub const TypeConstraint = union(enum) {
    Interface: []const u8,
    Equality: Type,
    Subtype: Type,
    Supertype: Type,
    WhereClause: []WhereClauseData,
    
    pub const WhereClauseData = struct {
        type_parameter: []const u8,
        constraints: ArrayList(TypeConstraint),
    };
};

// String interpolation support
pub const StringInterpolationExpression = struct {
    parts: ArrayList(InterpolationPart),
    
    pub fn init() StringInterpolationExpression {
        return StringInterpolationExpression{
            .parts = .empty,
        };
    }
    
    pub fn deinit(self: *StringInterpolationExpression, allocator: Allocator) void {
        for (self.parts.items) |part| {
            if (part.expression) |expr| {
                const expr_ptr: *Expression = @ptrCast(@alignCast(expr));
                expr_ptr.deinit();
                allocator.destroy(expr_ptr);
            }
        }
        self.parts.deinit();
    }
};

pub const InterpolationPart = struct {
    text: []const u8,        // Literal text part
    expression: ?*anyopaque, // Expression to evaluate (null for literal parts)
    format_spec: ?[]const u8, // Optional format specification
};

// Async/await expression types
pub const AwaitExpressionType = struct {
    expression: *Expression,
    timeout: ?*Expression,
    cancellation_token: ?*Expression,
    
    pub fn init(expression: *Expression) AwaitExpressionType {
        return AwaitExpressionType{
            .expression = expression,
            .timeout = null,
            .cancellation_token = null,
        };
    }
};

pub const AsyncFunction = struct {
    name: []const u8,
    parameters: []Parameter,
    return_type: ?*Type,
    body: *Expression,
    is_generator: bool,
    yield_type: ?*Type,
    
    pub fn init(name: []const u8, parameters: []Parameter, body: *Expression) AsyncFunction {
        return AsyncFunction{
            .name = name,
            .parameters = parameters,
            .return_type = null,
            .body = body,
            .is_generator = false,
            .yield_type = null,
        };
    }
};

// Control flow expression types for proper async transformation
pub const LoopExpression = struct {
    body: *Expression,
    
    pub fn init(body: *Expression) LoopExpression {
        return LoopExpression{ .body = body };
    }
};

pub const ForExpression = struct {
    variable: []const u8,
    iterable: *Expression,
    body: *Expression,
    
    pub fn init(variable: []const u8, iterable: *Expression, body: *Expression) ForExpression {
        return ForExpression{
            .variable = variable,
            .iterable = iterable,
            .body = body,
        };
    }
};

pub const WhileExpression = struct {
    condition: *Expression,
    body: *Expression,
    
    pub fn init(condition: *Expression, body: *Expression) WhileExpression {
        return WhileExpression{
            .condition = condition,
            .body = body,
        };
    }
};

pub const BlockExpression = struct {
    statements: []*Expression,
    
    pub fn init(statements: []*Expression) BlockExpression {
        return BlockExpression{ .statements = statements };
    }
};

pub const IfExpression = struct {
    condition: *Expression,
    then_branch: *Expression,
    else_branch: ?*Expression,
    
    pub fn init(condition: *Expression, then_branch: *Expression) IfExpression {
        return IfExpression{
            .condition = condition,
            .then_branch = then_branch,
            .else_branch = null,
        };
    }
};

pub const FunctionCallExpression = struct {
    function: *Expression,
    arguments: []*Expression,
    
    pub fn init(function: *Expression, arguments: []*Expression) FunctionCallExpression {
        return FunctionCallExpression{
            .function = function,
            .arguments = arguments,
        };
    }
};

// AST type alias for tools
pub const AST = Program;

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
