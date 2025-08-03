//! Advanced AST definitions for complete CURSED language support
//!
//! This module defines AST nodes for all CURSED language constructs including:
//! - Pattern matching with guards and destructuring
//! - Generic types with constraints and where clauses
//! - Interface inheritance and composition
//! - Advanced struct definitions with methods
//! - Complex control flow constructs
//! - Error handling and defer statements

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const Token = lexer.Token;

/// Program root node
pub const Program = struct {
    package: ?PackageDeclaration,
    imports: ArrayList(ImportStatement),
    statements: ArrayList(Statement),
    
    pub fn init(allocator: Allocator) Program {
        return Program{
            .package = null,
            .imports = ArrayList(ImportStatement).init(allocator),
            .statements = ArrayList(Statement).init(allocator),
        };
    }
    
    pub fn deinit(self: *Program) void {
        self.imports.deinit();
        self.statements.deinit();
    }
};

/// Package declaration
pub const PackageDeclaration = struct {
    name: []const u8,
    version: ?[]const u8,
};

/// Import statement with optional aliasing
pub const ImportStatement = struct {
    path: []const u8,
    alias: ?[]const u8,
    selective: ?[][]const u8, // For selective imports: yeet { func1, func2 } from "module"
};

/// Top-level statement types
pub const Statement = union(enum) {
    Package: PackageDeclaration,
    Import: ImportStatement,
    Function: FunctionStatement,
    Struct: StructStatement,
    Interface: InterfaceStatement,
    Let: LetStatement,
    Const: ConstStatement,
    TypeAlias: TypeAliasStatement,
    Expression: Expression,
    Return: ReturnStatement,
    If: IfStatement,
    While: WhileStatement,
    For: ForStatement,
    Match: MatchStatement,
    Defer: DeferStatement,
    Select: SelectStatement,
    Block: BlockStatement,
    
    pub fn deinit(self: *Statement, allocator: Allocator) void {
        switch (self.*) {
            .Function => |*func| func.deinit(allocator),
            .Struct => |*struct_stmt| struct_stmt.deinit(allocator),
            .Interface => |*interface| interface.deinit(allocator),
            .Expression => |*expr| expr.deinit(allocator),
            else => {},
        }
    }
};

/// Advanced function declaration with generics and constraints
pub const FunctionStatement = struct {
    name: []const u8,
    type_parameters: []TypeParameter,
    parameters: []Parameter,
    return_type: ?Type,
    where_clause: ?WhereClause,
    body: []Statement,
    visibility: Visibility,
    is_async: bool,
    is_extern: bool,
    
    pub fn deinit(self: *FunctionStatement, allocator: Allocator) void {
        allocator.free(self.type_parameters);
        allocator.free(self.parameters);
        if (self.return_type) |*ret_type| ret_type.deinit(allocator);
        allocator.free(self.body);
    }
};

/// Generic type parameter with constraints
pub const TypeParameter = struct {
    name: []const u8,
    constraints: []TypeConstraint,
    default_type: ?Type,
    variance: TypeVariance,
    
    pub const TypeVariance = enum {
        Invariant,
        Covariant,      // +T
        Contravariant,  // -T
    };
};

/// Forward declaration for recursive types
pub const TypeConstraint = union(enum) {
    Interface: InterfaceConstraint,
    Trait: TraitConstraint,
    Where: WhereConstraint,
    Lifetime: LifetimeConstraint,
    Associated: AssociatedTypeConstraint,
    
    pub const InterfaceConstraint = struct {
        interface_name: []const u8,
        type_args: ?[]*Type,
    };
    
    pub const TraitConstraint = struct {
        trait_name: []const u8,
        associated_types: ?[]AssociatedType,
    };
    
    pub const WhereConstraint = struct {
        type_param: []const u8,
        bounds: []*TypeConstraint,
    };
    
    pub const LifetimeConstraint = struct {
        lifetime: []const u8,
        bounds: [][]const u8,
    };
    
    pub const AssociatedTypeConstraint = struct {
        type_name: []const u8,
        constraint: *TypeConstraint,
    };
};

/// Where clause for complex generic constraints
pub const WhereClause = struct {
    constraints: []WhereConstraintItem,
    
    pub const WhereConstraintItem = struct {
        type_expr: *Type,
        bounds: []*TypeConstraint,
    };
};

/// Advanced struct declaration with methods and inheritance
pub const StructStatement = struct {
    name: []const u8,
    type_parameters: []TypeParameter,
    fields: []StructField,
    methods: []FunctionStatement,
    implements: [][]const u8, // Interface implementations
    visibility: Visibility,
    is_packed: bool,
    is_union: bool,
    
    pub fn deinit(self: *StructStatement, allocator: Allocator) void {
        allocator.free(self.type_parameters);
        allocator.free(self.fields);
        allocator.free(self.methods);
        allocator.free(self.implements);
    }
};

/// Struct field with advanced attributes
pub const StructField = struct {
    name: []const u8,
    field_type: Type,
    visibility: Visibility,
    default_value: ?Expression,
    attributes: []FieldAttribute,
    is_static: bool,
    
    pub const FieldAttribute = union(enum) {
        Alignment: u32,
        Offset: u32,
        Tag: []const u8,
        Deprecated: ?[]const u8,
    };
};

/// Advanced interface declaration with inheritance
pub const InterfaceStatement = struct {
    name: []const u8,
    type_parameters: []TypeParameter,
    extends: [][]const u8, // Interface inheritance
    methods: []InterfaceMethod,
    associated_types: []AssociatedType,
    default_implementations: []DefaultImplementation,
    visibility: Visibility,
    
    pub fn deinit(self: *InterfaceStatement, allocator: Allocator) void {
        allocator.free(self.type_parameters);
        allocator.free(self.extends);
        allocator.free(self.methods);
        allocator.free(self.associated_types);
        allocator.free(self.default_implementations);
    }
};

/// Interface method signature
pub const InterfaceMethod = struct {
    name: []const u8,
    type_parameters: ?[]TypeParameter,
    parameters: []Parameter,
    return_type: ?Type,
    is_async: bool,
    is_static: bool,
};

/// Associated type in interface
pub const AssociatedType = struct {
    name: []const u8,
    bounds: []TypeConstraint,
    default_type: ?Type,
};

/// Default implementation in interface
pub const DefaultImplementation = struct {
    method_name: []const u8,
    body: []Statement,
};

/// Function parameter with advanced features
pub const Parameter = struct {
    name: []const u8,
    param_type: Type,
    default_value: ?Expression,
    is_variadic: bool,
    is_mut: bool,
    attributes: []ParameterAttribute,
    
    pub const ParameterAttribute = union(enum) {
        ByRef,
        ByValue,
        NoAlias,
        NonNull,
    };
};

/// Advanced type system
pub const Type = union(enum) {
    Basic: BasicType,
    Identifier: []const u8,
    Generic: GenericType,
    Array: ArrayType,
    Slice: SliceType,
    Pointer: PointerType,
    Reference: ReferenceType,
    Function: FunctionType,
    Tuple: TupleType,
    Channel: ChannelType,
    Interface: InterfaceType,
    Union: UnionType,
    Optional: OptionalType,
    Result: ResultType,
    
    pub const BasicType = enum {
        Normie,    // int
        Tea,       // string
        Lit,       // bool
        Meal,      // float
        Sip,       // byte
        Drip,      // int64
        Smol,      // int32
        Thicc,     // int64
        Txt,       // char
        Vibes,     // any
        Void,      // void
    };
    
    pub const GenericType = struct {
        base: *Type,
        type_args: []*Type,
    };
    
    pub const ArrayType = struct {
        element_type: *Type,
        size: ?Expression,
    };
    
    pub const SliceType = struct {
        element_type: *Type,
    };
    
    pub const PointerType = struct {
        target_type: *Type,
        is_mutable: bool,
        is_nullable: bool,
    };
    
    pub const ReferenceType = struct {
        target_type: *Type,
        lifetime: ?[]const u8,
        is_mutable: bool,
    };
    
    pub const FunctionType = struct {
        parameters: []*Type,
        return_type: ?*Type,
        is_async: bool,
        is_extern: bool,
    };
    
    pub const TupleType = struct {
        elements: []*Type,
    };
    
    pub const ChannelType = struct {
        element_type: *Type,
        buffer_size: ?Expression,
        direction: ChannelDirection,
        
        pub const ChannelDirection = enum {
            BiDirectional,
            SendOnly,
            ReceiveOnly,
        };
    };
    
    pub const InterfaceType = struct {
        name: []const u8,
        type_args: ?[]*Type,
    };
    
    pub const UnionType = struct {
        variants: []*Type,
    };
    
    pub const OptionalType = struct {
        inner_type: *Type,
    };
    
    pub const ResultType = struct {
        ok_type: *Type,
        error_type: *Type,
    };
    
    pub fn deinit(self: *Type, allocator: Allocator) void {
        switch (self.*) {
            .Generic => |*generic| {
                generic.base.deinit(allocator);
                allocator.destroy(generic.base);
                for (generic.type_args) |*arg| {
                    arg.deinit(allocator);
                }
                allocator.free(generic.type_args);
            },
            .Array => |*array| {
                array.element_type.deinit(allocator);
                allocator.destroy(array.element_type);
            },
            .Slice => |*slice| {
                slice.element_type.deinit(allocator);
                allocator.destroy(slice.element_type);
            },
            .Pointer => |*pointer| {
                pointer.target_type.deinit(allocator);
                allocator.destroy(pointer.target_type);
            },
            .Reference => |*reference| {
                reference.target_type.deinit(allocator);
                allocator.destroy(reference.target_type);
            },
            .Function => |*function| {
                for (function.parameters) |*param| {
                    param.deinit(allocator);
                }
                allocator.free(function.parameters);
                if (function.return_type) |ret_type| {
                    ret_type.deinit(allocator);
                    allocator.destroy(ret_type);
                }
            },
            .Tuple => |*tuple| {
                for (tuple.elements) |*element| {
                    element.deinit(allocator);
                }
                allocator.free(tuple.elements);
            },
            .Channel => |*channel| {
                channel.element_type.deinit(allocator);
                allocator.destroy(channel.element_type);
            },
            .Union => |*union_type| {
                for (union_type.variants) |*variant| {
                    variant.deinit(allocator);
                }
                allocator.free(union_type.variants);
            },
            .Optional => |*optional| {
                optional.inner_type.deinit(allocator);
                allocator.destroy(optional.inner_type);
            },
            .Result => |*result| {
                result.ok_type.deinit(allocator);
                result.error_type.deinit(allocator);
                allocator.destroy(result.ok_type);
                allocator.destroy(result.error_type);
            },
            else => {},
        }
    }
};

/// Visibility modifiers
pub const Visibility = enum {
    Public,
    Private,
    Protected,
    Internal,
    Export,
};

/// Expression types with advanced pattern matching
pub const Expression = union(enum) {
    Literal: LiteralExpression,
    Identifier: []const u8,
    Binary: BinaryExpression,
    Unary: UnaryExpression,
    Call: CallExpression,
    MemberAccess: MemberAccessExpression,
    ArrayAccess: ArrayAccessExpression,
    Assignment: AssignmentExpression,
    Match: MatchExpression,
    Lambda: LambdaExpression,
    Tuple: TupleExpression,
    Array: ArrayExpression,
    Struct: StructExpression,
    Cast: CastExpression,
    TypeAssertion: TypeAssertionExpression,
    Channel: ChannelExpression,
    Async: AsyncExpression,
    Await: AwaitExpression,
    
    pub fn deinit(self: *Expression, allocator: Allocator) void {
        switch (self.*) {
            .Binary => |*binary| {
                binary.left.deinit(allocator);
                binary.right.deinit(allocator);
                allocator.destroy(binary.left);
                allocator.destroy(binary.right);
            },
            .Unary => |*unary| {
                unary.operand.deinit(allocator);
                allocator.destroy(unary.operand);
            },
            .Call => |*call| {
                call.callee.deinit(allocator);
                allocator.destroy(call.callee);
                for (call.arguments) |*arg| {
                    arg.deinit(allocator);
                }
                allocator.free(call.arguments);
            },
            .Match => |*match| {
                match.expr.deinit(allocator);
                allocator.destroy(match.expr);
                for (match.arms) |*arm| {
                    arm.pattern.deinit(allocator);
                    if (arm.guard) |*guard| {
                        guard.deinit(allocator);
                        allocator.destroy(guard);
                    }
                    arm.body.deinit(allocator);
                }
                allocator.free(match.arms);
            },
            .Array => |*array| {
                for (array.elements) |*element| {
                    element.deinit(allocator);
                }
                allocator.free(array.elements);
            },
            .Tuple => |*tuple| {
                for (tuple.elements) |*element| {
                    element.deinit(allocator);
                }
                allocator.free(tuple.elements);
            },
            else => {},
        }
    }
};

/// Literal values
pub const LiteralExpression = struct {
    value: LiteralValue,
    
    pub const LiteralValue = union(enum) {
        Integer: i64,
        Float: f64,
        String: []const u8,
        Character: u8,
        Boolean: bool,
        Null,
    };
};

/// Binary operation
pub const BinaryExpression = struct {
    left: *Expression,
    operator: Token,
    right: *Expression,
};

/// Unary operation
pub const UnaryExpression = struct {
    operator: Token,
    operand: *Expression,
};

/// Function call with type arguments
pub const CallExpression = struct {
    callee: *Expression,
    type_arguments: ?[]Type,
    arguments: []Expression,
    is_async: bool,
};

/// Member access (dot notation)
pub const MemberAccessExpression = struct {
    object: *Expression,
    member: []const u8,
    is_safe: bool, // Optional chaining
};

/// Array/slice indexing
pub const ArrayAccessExpression = struct {
    array: *Expression,
    index: *Expression,
};

/// Assignment operations
pub const AssignmentExpression = struct {
    target: *Expression,
    operator: AssignmentOperator,
    value: *Expression,
    
    pub const AssignmentOperator = enum {
        Assign,      // =
        AddAssign,   // +=
        SubAssign,   // -=
        MulAssign,   // *=
        DivAssign,   // /=
        ModAssign,   // %=
        AndAssign,   // &=
        OrAssign,    // |=
        XorAssign,   // ^=
        ShlAssign,   // <<=
        ShrAssign,   // >>=
    };
};

/// Advanced pattern matching
pub const MatchExpression = struct {
    expr: *Expression,
    arms: []MatchArm,
    
    pub const MatchArm = struct {
        pattern: Pattern,
        guard: ?*Expression,
        body: Expression,
    };
};

/// Pattern types for matching
pub const Pattern = union(enum) {
    Literal: LiteralPattern,
    Variable: VariablePattern,
    Wildcard: WildcardPattern,
    Tuple: TuplePattern,
    Struct: StructPattern,
    Array: ArrayPattern,
    Slice: SlicePattern,
    Or: OrPattern,
    Range: RangePattern,
    Type: TypePattern,
    Guard: GuardPattern,
    
    pub const LiteralPattern = struct {
        value: LiteralExpression.LiteralValue,
    };
    
    pub const VariablePattern = struct {
        name: []const u8,
        is_mutable: bool,
        type_annotation: ?Type,
    };
    
    pub const WildcardPattern = struct {};
    
    pub const TuplePattern = struct {
        patterns: []Pattern,
    };
    
    pub const StructPattern = struct {
        type_name: []const u8,
        fields: []FieldPattern,
        is_exhaustive: bool,
        
        pub const FieldPattern = struct {
            name: []const u8,
            pattern: Pattern,
        };
    };
    
    pub const ArrayPattern = struct {
        patterns: []Pattern,
        rest: ?RestPattern,
        
        pub const RestPattern = struct {
            name: ?[]const u8,
            pattern: ?Pattern,
        };
    };
    
    pub const SlicePattern = struct {
        patterns: []Pattern,
        rest: ?ArrayPattern.RestPattern,
    };
    
    pub const OrPattern = struct {
        patterns: []Pattern,
    };
    
    pub const RangePattern = struct {
        start: *Expression,
        end: *Expression,
        is_inclusive: bool,
    };
    
    pub const TypePattern = struct {
        type_expr: Type,
        variable: ?[]const u8,
    };
    
    pub const GuardPattern = struct {
        pattern: *Pattern,
        condition: *Expression,
    };
    
    pub fn deinit(self: *Pattern, allocator: Allocator) void {
        switch (self.*) {
            .Tuple => |*tuple| {
                for (tuple.patterns) |*pattern| {
                    pattern.deinit(allocator);
                }
                allocator.free(tuple.patterns);
            },
            .Struct => |*struct_pattern| {
                for (struct_pattern.fields) |*field| {
                    field.pattern.deinit(allocator);
                }
                allocator.free(struct_pattern.fields);
            },
            .Array => |*array| {
                for (array.patterns) |*pattern| {
                    pattern.deinit(allocator);
                }
                allocator.free(array.patterns);
            },
            .Or => |*or_pattern| {
                for (or_pattern.patterns) |*pattern| {
                    pattern.deinit(allocator);
                }
                allocator.free(or_pattern.patterns);
            },
            .Range => |*range| {
                range.start.deinit(allocator);
                range.end.deinit(allocator);
                allocator.destroy(range.start);
                allocator.destroy(range.end);
            },
            .Guard => |*guard| {
                guard.pattern.deinit(allocator);
                guard.condition.deinit(allocator);
                allocator.destroy(guard.pattern);
                allocator.destroy(guard.condition);
            },
            else => {},
        }
    }
};

/// Lambda/closure expression
pub const LambdaExpression = struct {
    parameters: []Parameter,
    return_type: ?Type,
    body: LambdaBody,
    captures: []Capture,
    
    pub const LambdaBody = union(enum) {
        Expression: *Expression,
        Block: []Statement,
    };
    
    pub const Capture = struct {
        name: []const u8,
        mode: CaptureMode,
        
        pub const CaptureMode = enum {
            ByValue,
            ByReference,
            ByMutableReference,
        };
    };
};

/// Tuple expression
pub const TupleExpression = struct {
    elements: []Expression,
};

/// Array literal
pub const ArrayExpression = struct {
    elements: []Expression,
    element_type: ?Type,
};

/// Struct literal
pub const StructExpression = struct {
    type_name: []const u8,
    fields: []FieldInitializer,
    
    pub const FieldInitializer = struct {
        name: []const u8,
        value: Expression,
    };
};

/// Type casting
pub const CastExpression = struct {
    expression: *Expression,
    target_type: Type,
    is_safe: bool,
};

/// Type assertion
pub const TypeAssertionExpression = struct {
    expression: *Expression,
    asserted_type: Type,
};

/// Channel operations
pub const ChannelExpression = struct {
    operation: ChannelOperation,
    
    pub const ChannelOperation = union(enum) {
        Send: SendOperation,
        Receive: ReceiveOperation,
        Make: MakeOperation,
        Close: CloseOperation,
        
        pub const SendOperation = struct {
            channel: *Expression,
            value: *Expression,
        };
        
        pub const ReceiveOperation = struct {
            channel: *Expression,
        };
        
        pub const MakeOperation = struct {
            element_type: Type,
            buffer_size: ?*Expression,
        };
        
        pub const CloseOperation = struct {
            channel: *Expression,
        };
    };
};

/// Async expression
pub const AsyncExpression = struct {
    expression: *Expression,
};

/// Await expression
pub const AwaitExpression = struct {
    expression: *Expression,
};

/// Control flow statements
pub const IfStatement = struct {
    condition: Expression,
    then_branch: []Statement,
    else_branch: ?[]Statement,
};

pub const WhileStatement = struct {
    condition: Expression,
    body: []Statement,
    label: ?[]const u8,
};

pub const ForStatement = struct {
    init: ?Statement,
    condition: ?Expression,
    update: ?Statement,
    body: []Statement,
    label: ?[]const u8,
    range_info: ?RangeInfo,
    
    pub const RangeInfo = struct {
        iterator: []const u8,
        iterable: Expression,
        index_var: ?[]const u8,
    };
};

pub const MatchStatement = struct {
    expression: Expression,
    arms: []MatchExpression.MatchArm,
};

pub const DeferStatement = struct {
    expression: Expression,
};

/// Select statement for channel operations
pub const SelectStatement = struct {
    cases: []SelectCase,
    
    pub const SelectCase = struct {
        channel_op: ?Statement,
        statements: []Statement,
        is_default: bool,
    };
};

pub const BlockStatement = struct {
    statements: []Statement,
};

/// Variable declaration
pub const LetStatement = struct {
    name: []const u8,
    var_type: Type,
    value: Expression,
    is_mutable: bool,
    is_static: bool,
};

/// Constant declaration
pub const ConstStatement = struct {
    name: []const u8,
    const_type: ?Type,
    value: Expression,
    visibility: Visibility,
};

/// Type alias declaration
pub const TypeAliasStatement = struct {
    name: []const u8,
    type_parameters: ?[]TypeParameter,
    target_type: Type,
    visibility: Visibility,
};

/// Return statement
pub const ReturnStatement = struct {
    value: ?Expression,
};

/// Error handling
pub const ErrorExpression = struct {
    kind: ErrorKind,
    
    pub const ErrorKind = union(enum) {
        Try: TryExpression,
        Catch: CatchExpression,
        Throw: ThrowExpression,
        
        pub const TryExpression = struct {
            expression: *Expression,
            catch_clauses: []CatchClause,
            finally_clause: ?[]Statement,
        };
        
        pub const CatchExpression = struct {
            pattern: Pattern,
            body: []Statement,
        };
        
        pub const ThrowExpression = struct {
            error_value: *Expression,
        };
        
        pub const CatchClause = struct {
            error_pattern: Pattern,
            body: []Statement,
        };
    };
};

/// Async/await support
pub const AsyncStatement = struct {
    body: Expression,
};

pub const AwaitStatement = struct {
    expression: Expression,
};

/// Module and namespace support
pub const ModuleStatement = struct {
    name: []const u8,
    body: []Statement,
    visibility: Visibility,
};

pub const UseStatement = struct {
    path: []const u8,
    imports: ?[]ImportItem,
    
    pub const ImportItem = struct {
        name: []const u8,
        alias: ?[]const u8,
    };
};

/// Attribute support
pub const Attribute = struct {
    name: []const u8,
    arguments: ?[]AttributeArgument,
    
    pub const AttributeArgument = union(enum) {
        Literal: LiteralExpression.LiteralValue,
        Expression: Expression,
        NamedArgument: NamedAttributeArgument,
        
        pub const NamedAttributeArgument = struct {
            name: []const u8,
            value: AttributeArgument,
        };
    };
};

/// Documentation comments
pub const Documentation = struct {
    content: []const u8,
    kind: DocumentationKind,
    
    pub const DocumentationKind = enum {
        LineComment,
        BlockComment,
        DocComment,
    };
};

/// Source location information
pub const SourceLocation = struct {
    line: u32,
    column: u32,
    file: ?[]const u8,
    span: ?SourceSpan,
    
    pub const SourceSpan = struct {
        start: u32,
        end: u32,
    };
};

/// Node trait for all AST nodes
pub const ASTNode = struct {
    location: ?SourceLocation,
    attributes: ?[]Attribute,
    documentation: ?Documentation,
    
    pub fn hasAttribute(self: *const ASTNode, name: []const u8) bool {
        if (self.attributes) |attrs| {
            for (attrs) |attr| {
                if (std.mem.eql(u8, attr.name, name)) {
                    return true;
                }
            }
        }
        return false;
    }
};
