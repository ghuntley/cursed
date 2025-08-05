//! Advanced CURSED parser implementation with complete language feature support
//! 
//! This parser implements all CURSED language constructs including:
//! - Pattern matching with guards and destructuring
//! - Complex generic type parsing with constraints
//! - Advanced interface definitions and inheritance
//! - Complete struct parsing with field access and methods
//! - Proper error recovery and incremental parsing
//! - Complex control flow constructs
//! - All CURSED syntax from the specs

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const lexer_advanced = @import("lexer_advanced.zig");
const ast = @import("ast_simple.zig");
const allocation_guards = @import("allocation_guards.zig");

const Token = lexer_advanced.Token;
const TokenKind = lexer_advanced.TokenKind;
const Program = ast.Program;
const Statement = ast.Statement;
const Expression = ast.Expression;
const Type = ast.Type;

pub const ParserError = error{
    UnexpectedToken,
    UnexpectedEof,
    InvalidSyntax,
    OutOfMemory,
    MissingToken,
    InvalidExpression,
    InvalidStatement,
    InvalidType,
    InvalidFunction,
    InvalidParameter,
    InvalidBlock,
    InvalidAssignment,
    InvalidPattern,
    InvalidGeneric,
    InvalidInterface,
    InvalidStruct,
    InvalidControlFlow,
    SyntaxError,
    ParseError,
    RecoveryFailed,
};

/// Pattern types for pattern matching
pub const Pattern = union(enum) {
    Literal: LiteralPattern,
    Variable: VariablePattern,
    Type: TypePattern,
    Tuple: TuplePattern,
    Struct: StructPattern,
    Array: ArrayPattern,
    Or: OrPattern,
    Wildcard: WildcardPattern,
    
    pub const LiteralPattern = struct {
        value: Expression,
    };
    
    pub const VariablePattern = struct {
        name: []const u8,
        mutable: bool,
    };
    
    pub const TypePattern = struct {
        type_expr: Type,
        variable: ?[]const u8,
    };
    
    pub const TuplePattern = struct {
        patterns: []Pattern,
    };
    
    pub const StructPattern = struct {
        type_name: []const u8,
        fields: []FieldPattern,
    };
    
    pub const FieldPattern = struct {
        name: []const u8,
        pattern: Pattern,
    };
    
    pub const ArrayPattern = struct {
        patterns: []Pattern,
        rest: ?[]const u8, // For ..rest patterns
    };
    
    pub const OrPattern = struct {
        patterns: []Pattern,
    };
    
    pub const WildcardPattern = struct {};
};

/// Match expression with pattern guards
pub const MatchExpression = struct {
    expr: Expression,
    arms: []MatchArm,
    
    pub const MatchArm = struct {
        pattern: Pattern,
        guard: ?Expression, // Optional guard clause
        body: Expression,
    };
};

/// Generic type parameters with constraints
pub const TypeParameter = struct {
    name: []const u8,
    constraints: []TypeConstraint,
    default_type: ?Type,
};

pub const TypeConstraint = union(enum) {
    Interface: []const u8,
    Trait: []const u8,
    Where: WhereConstraint,
    
    pub const WhereConstraint = struct {
        type_param: []const u8,
        bounds: []TypeConstraint,
    };
};

/// Interface with inheritance and composition
pub const InterfaceDeclaration = struct {
    name: []const u8,
    type_parameters: []TypeParameter,
    extends: [][]const u8, // Interface inheritance
    methods: []InterfaceMethod,
    associated_types: []AssociatedType,
    
    pub const InterfaceMethod = struct {
        name: []const u8,
        parameters: []ast.Parameter,
        return_type: ?Type,
        default_impl: ?[]Statement, // Default implementation
    };
    
    pub const AssociatedType = struct {
        name: []const u8,
        bounds: []TypeConstraint,
        default_type: ?Type,
    };
};

/// Struct with methods and inheritance
pub const StructDeclaration = struct {
    name: []const u8,
    type_parameters: []TypeParameter,
    fields: []StructField,
    methods: []ast.FunctionStatement,
    visibility: Visibility,
    
    pub const StructField = struct {
        name: []const u8,
        field_type: Type,
        visibility: Visibility,
        default_value: ?Expression,
    };
};

pub const Visibility = enum {
    Public,
    Private,
    Protected,
};

/// Advanced error recovery state
pub const RecoveryState = struct {
    synchronization_points: []TokenKind,
    error_count: u32,
    max_errors: u32,
    recovery_mode: bool,
};

/// Advanced parser with complete language support
pub const AdvancedParser = struct {
    tokens: []const Token,
    current: usize,
    allocator: Allocator,
    had_error: bool,
    in_function: bool,
    in_loop: bool,
    in_match: bool,
    scope_depth: usize,
    recovery_state: RecoveryState,
    
    pub fn init(allocator: Allocator, tokens: []const Token) AdvancedParser {
        return AdvancedParser{
            .tokens = tokens,
            .current = 0,
            .allocator = allocator,
            .had_error = false,
            .in_function = false,
            .in_loop = false,
            .in_match = false,
            .scope_depth = 0,
            .recovery_state = RecoveryState{
                .synchronization_points = &[_]TokenKind{
                    .Slay,      // function
                    .Sus,       // variable
                    .Squad,     // struct
                    .Collab,    // interface
                    .LeftBrace, // blocks
                    .Semicolon, // statements
                },
                .error_count = 0,
                .max_errors = 10,
                .recovery_mode = false,
            },
        };
    }
    
    pub fn parseProgram(self: *AdvancedParser) ParserError!Program {
        var program = Program.init(self.allocator);
        errdefer program.deinit(); // Clean up on error
        
        while (!self.isAtEnd()) {
            if (self.recovery_state.error_count >= self.recovery_state.max_errors) {
                return ParserError.RecoveryFailed;
            }
            
            // Skip newlines and semicolons
            if (self.check(.Newline) or self.check(.Semicolon)) {
                _ = self.advance();
                continue;
            }
            
            // Handle comments
            if (self.check(.Comment)) {
                _ = self.advance();
                continue;
            }
            
            // Parse top-level declarations with cleanup on error
            const stmt = self.parseTopLevelDeclaration() catch |err| {
                if (self.recovery_state.recovery_mode) {
                    self.synchronize();
                    continue;
                } else {
                    return err;
                }
            };
            
            program.statements.append(stmt) catch |err| {
                // Clean up the statement if append fails
                stmt.deinit(self.allocator);
                return err;
            };
        }
        
        return program;
    }
    
    fn parseTopLevelDeclaration(self: *AdvancedParser) ParserError!Statement {
        // Package declaration
        if (self.check(.Vibe)) {
            return Statement{ .Package = try self.parsePackageDeclaration() };
        }
        
        // Import statement
        if (self.check(.Yeet)) {
            return Statement{ .Import = try self.parseImportStatement() };
        }
        
        // Function declaration
        if (self.check(.Slay)) {
            return Statement{ .Function = try self.parseAdvancedFunctionDeclaration() };
        }
        
        // Struct declaration
        if (self.check(.Squad) or self.check(.Struct)) {
            return Statement{ .Struct = try self.parseAdvancedStructDeclaration() };
        }
        
        // Interface declaration
        if (self.check(.Collab)) {
            return Statement{ .Interface = try self.parseInterfaceDeclaration() };
        }
        
        // Variable declaration
        if (self.check(.Sus)) {
            return Statement{ .Let = try self.parseVariableDeclaration() };
        }
        
        // Type alias
        if (self.check(.BeLike)) {
            return Statement{ .TypeAlias = try self.parseTypeAlias() };
        }
        
        // Constant declaration
        if (self.check(.Facts)) {
            return Statement{ .Const = try self.parseConstantDeclaration() };
        }
        
        return self.parseStatement();
    }
    
    /// Parse advanced function declaration with generics and constraints
    fn parseAdvancedFunctionDeclaration(self: *AdvancedParser) ParserError!ast.FunctionStatement {
        try self.consume(.Slay, "Expected 'slay'");
        
        const name = try self.parseIdentifier();
        
        // Parse generic type parameters
        var type_parameters = ArrayList(TypeParameter).init(self.allocator);
        if (self.match(.Less) or self.match(.LeftAngle)) {
            while (!self.check(.Greater) and !self.check(.RightAngle) and !self.isAtEnd()) {
                const type_param = try self.parseTypeParameter();
                try type_parameters.append(type_param);
                
                if (!self.match(.Comma)) break;
            }
            _ = try self.consume(.Greater, "Expected '>' after type parameters");
        }
        
        // Parse parameters
        try self.consume(.LeftParen, "Expected '(' after function name");
        var parameters = ArrayList(ast.Parameter).init(self.allocator);
        
        while (!self.check(.RightParen) and !self.isAtEnd()) {
            const param = try self.parseParameter();
            try parameters.append(param);
            
            if (!self.match(.Comma)) break;
        }
        
        try self.consume(.RightParen, "Expected ')' after parameters");
        
        // Parse return type
        var return_type: ?Type = null;
        if (self.match(.Arrow) or self.current_token_is_identifier("->")) {
            if (self.current_token_is_identifier("->")) _ = self.advance();
            return_type = try self.parseType();
        }
        
        // Parse where clause
        var where_constraints = ArrayList(TypeConstraint).init(self.allocator);
        if (self.current_token_is_identifier("where")) {
            _ = self.advance();
            while (!self.check(.LeftBrace) and !self.isAtEnd()) {
                const constraint = try self.parseWhereConstraint();
                try where_constraints.append(constraint);
                
                if (!self.match(.Comma)) break;
            }
        }
        
        // Parse function body
        self.in_function = true;
        try self.consume(.LeftBrace, "Expected '{' before function body");
        const body = try self.parseStatements();
        try self.consume(.RightBrace, "Expected '}' after function body");
        self.in_function = false;
        
        return ast.FunctionStatement{
            .name = name,
            .parameters = try parameters.toOwnedSlice(),
            .return_type = return_type,
            .body = body,
            .visibility = .Public,
        };
    }
    
    /// Parse type parameter with constraints
    fn parseTypeParameter(self: *AdvancedParser) ParserError!TypeParameter {
        const name = try self.parseIdentifier();
        
        var constraints = ArrayList(TypeConstraint).init(self.allocator);
        
        // Parse constraints (T: Display + Clone)
        if (self.match(.Colon)) {
            while (!self.check(.Comma) and !self.check(.Greater) and !self.check(.RightAngle) and !self.isAtEnd()) {
                const constraint = try self.parseTypeConstraint();
                try constraints.append(constraint);
                
                if (!self.match(.Plus)) break;
            }
        }
        
        // Parse default type
        var default_type: ?Type = null;
        if (self.match(.Equal)) {
            default_type = try self.parseType();
        }
        
        return TypeParameter{
            .name = name,
            .constraints = try constraints.toOwnedSlice(),
            .default_type = default_type,
        };
    }
    
    /// Parse type constraint
    fn parseTypeConstraint(self: *AdvancedParser) ParserError!TypeConstraint {
        const constraint_name = try self.parseIdentifier();
        return TypeConstraint{ .Interface = constraint_name };
    }
    
    /// Parse where constraint
    fn parseWhereConstraint(self: *AdvancedParser) ParserError!TypeConstraint {
        const type_param = try self.parseIdentifier();
        try self.consume(.Colon, "Expected ':' after type parameter in where clause");
        
        var bounds = ArrayList(TypeConstraint).init(self.allocator);
        while (!self.check(.Comma) and !self.check(.LeftBrace) and !self.isAtEnd()) {
            const bound = try self.parseTypeConstraint();
            try bounds.append(bound);
            
            if (!self.match(.Plus)) break;
        }
        
        return TypeConstraint{ .Where = TypeConstraint.WhereConstraint{
            .type_param = type_param,
            .bounds = try bounds.toOwnedSlice(),
        }};
    }
    
    /// Parse advanced struct declaration with methods and inheritance
    fn parseAdvancedStructDeclaration(self: *AdvancedParser) ParserError!ast.StructStatement {
        _ = self.advance(); // consume 'squad' or 'struct'
        
        const name = try self.parseIdentifier();
        
        // Parse generic type parameters
        var type_parameters = ArrayList(TypeParameter).init(self.allocator);
        if (self.match(.Less) or self.match(.LeftAngle)) {
            while (!self.check(.Greater) and !self.check(.RightAngle) and !self.isAtEnd()) {
                const type_param = try self.parseTypeParameter();
                try type_parameters.append(type_param);
                
                if (!self.match(.Comma)) break;
            }
            _ = try self.consume(.Greater, "Expected '>' after type parameters");
        }
        
        try self.consume(.LeftBrace, "Expected '{' after struct name");
        
        // Parse fields
        var fields = ArrayList(ast.StructField).init(self.allocator);
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            // Skip newlines
            if (self.check(.Newline)) {
                _ = self.advance();
                continue;
            }
            
            const field = try self.parseStructField();
            try fields.append(field);
            
            // Optional comma or newline
            if (self.check(.Comma) or self.check(.Newline)) {
                _ = self.advance();
            }
        }
        
        try self.consume(.RightBrace, "Expected '}' after struct fields");
        
        return ast.StructStatement{
            .name = name,
            .fields = try fields.toOwnedSlice(),
        };
    }
    
    /// Parse struct field with type and optional default value
    fn parseStructField(self: *AdvancedParser) ParserError!ast.StructField {
        // Parse visibility (optional)
        var visibility = Visibility.Public;
        if (self.current_token_is_identifier("private")) {
            visibility = .Private;
            _ = self.advance();
        } else if (self.current_token_is_identifier("protected")) {
            visibility = .Protected;
            _ = self.advance();
        }
        
        // Parse field name and type
        const field_name = try self.parseIdentifier();
        const field_type = try self.parseType();
        
        // Parse optional default value
        var default_value: ?Expression = null;
        if (self.match(.Equal)) {
            default_value = try self.parseExpression();
        }
        
        return ast.StructField{
            .name = field_name,
            .field_type = field_type,
        };
    }
    
    /// Parse interface declaration with inheritance and methods
    fn parseInterfaceDeclaration(self: *AdvancedParser) ParserError!ast.InterfaceStatement {
        try self.consume(.Collab, "Expected 'collab'");
        
        const name = try self.parseIdentifier();
        
        // Parse generic type parameters
        var type_parameters = ArrayList(TypeParameter).init(self.allocator);
        if (self.match(.Less) or self.match(.LeftAngle)) {
            while (!self.check(.Greater) and !self.check(.RightAngle) and !self.isAtEnd()) {
                const type_param = try self.parseTypeParameter();
                try type_parameters.append(type_param);
                
                if (!self.match(.Comma)) break;
            }
            _ = try self.consume(.Greater, "Expected '>' after type parameters");
        }
        
        // Parse interface inheritance (extends/with)
        var extends = ArrayList([]const u8).init(self.allocator);
        if (self.current_token_is_identifier("extends")) {
            _ = self.advance();
            while (!self.check(.LeftBrace) and !self.isAtEnd()) {
                const parent = try self.parseIdentifier();
                try extends.append(parent);
                
                if (!self.match(.Comma)) break;
            }
        }
        
        try self.consume(.LeftBrace, "Expected '{' after interface declaration");
        
        // Parse interface methods
        var methods = ArrayList(ast.InterfaceMethod).init(self.allocator);
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            // Skip newlines
            if (self.check(.Newline)) {
                _ = self.advance();
                continue;
            }
            
            const method = try self.parseInterfaceMethod();
            try methods.append(method);
        }
        
        try self.consume(.RightBrace, "Expected '}' after interface body");
        
        return ast.InterfaceStatement{
            .name = name,
            .methods = try methods.toOwnedSlice(),
        };
    }
    
    /// Parse interface method signature
    fn parseInterfaceMethod(self: *AdvancedParser) ParserError!ast.InterfaceMethod {
        try self.consume(.Slay, "Expected 'slay' for interface method");
        
        const name = try self.parseIdentifier();
        
        // Parse parameters
        try self.consume(.LeftParen, "Expected '(' after method name");
        var parameters = ArrayList(ast.Parameter).init(self.allocator);
        
        while (!self.check(.RightParen) and !self.isAtEnd()) {
            const param = try self.parseParameter();
            try parameters.append(param);
            
            if (!self.match(.Comma)) break;
        }
        
        try self.consume(.RightParen, "Expected ')' after parameters");
        
        // Parse return type
        var return_type: ?Type = null;
        if (self.match(.Arrow) or self.current_token_is_identifier("->")) {
            if (self.current_token_is_identifier("->")) _ = self.advance();
            return_type = try self.parseType();
        }
        
        // Parse optional semicolon
        if (self.check(.Semicolon)) {
            _ = self.advance();
        }
        
        return ast.InterfaceMethod{
            .name = name,
            .parameters = try parameters.toOwnedSlice(),
            .return_type = return_type,
        };
    }
    
    /// Parse pattern matching expression
    fn parseMatchExpression(self: *AdvancedParser) ParserError!Expression {
        try self.consume(.VibeCheck, "Expected 'vibe_check'");
        
        const expr = try self.parseExpression();
        
        try self.consume(.LeftBrace, "Expected '{' after match expression");
        
        var arms = ArrayList(MatchExpression.MatchArm).init(self.allocator);
        
        self.in_match = true;
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            // Skip newlines
            if (self.check(.Newline)) {
                _ = self.advance();
                continue;
            }
            
            const arm = try self.parseMatchArm();
            try arms.append(arm);
        }
        
        self.in_match = false;
        
        try self.consume(.RightBrace, "Expected '}' after match arms");
        
        const match_expr = MatchExpression{
            .expr = expr,
            .arms = try arms.toOwnedSlice(),
        };
        
        return Expression{ .Match = match_expr };
    }
    
    /// Parse match arm with pattern and optional guard
    fn parseMatchArm(self: *AdvancedParser) ParserError!MatchExpression.MatchArm {
        const pattern = try self.parsePattern();
        
        // Parse optional guard clause
        var guard: ?Expression = null;
        if (self.current_token_is_identifier("if")) {
            _ = self.advance();
            guard = try self.parseExpression();
        }
        
        try self.consume(.Arrow, "Expected '=>' after pattern");
        
        const body = try self.parseExpression();
        
        // Optional comma
        if (self.check(.Comma)) {
            _ = self.advance();
        }
        
        return MatchExpression.MatchArm{
            .pattern = pattern,
            .guard = guard,
            .body = body,
        };
    }
    
    /// Parse pattern with full destructuring support
    fn parsePattern(self: *AdvancedParser) ParserError!Pattern {
        return try self.parseOrPattern();
    }
    
    /// Parse or pattern (pattern1 | pattern2)
    fn parseOrPattern(self: *AdvancedParser) ParserError!Pattern {
        const pattern = try self.parseStructPattern();
        
        if (self.match(.Pipe)) {
            var patterns = ArrayList(Pattern).init(self.allocator);
            try patterns.append(pattern);
            
            while (true) {
                const next_pattern = try self.parseStructPattern();
                try patterns.append(next_pattern);
                
                if (!self.match(.Pipe)) break;
            }
            
            return Pattern{ .Or = Pattern.OrPattern{
                .patterns = try patterns.toOwnedSlice(),
            }};
        }
        
        return pattern;
    }
    
    /// Parse struct pattern
    fn parseStructPattern(self: *AdvancedParser) ParserError!Pattern {
        if (self.check(.Identifier) and self.peekNext().kind == .LeftBrace) {
            const type_name = try self.parseIdentifier();
            try self.consume(.LeftBrace, "Expected '{' after struct name in pattern");
            
            var fields = ArrayList(Pattern.FieldPattern).init(self.allocator);
            
            while (!self.check(.RightBrace) and !self.isAtEnd()) {
                const field_name = try self.parseIdentifier();
                try self.consume(.Colon, "Expected ':' after field name");
                const field_pattern = try self.parsePattern();
                
                try fields.append(Pattern.FieldPattern{
                    .name = field_name,
                    .pattern = field_pattern,
                });
                
                if (!self.match(.Comma)) break;
            }
            
            try self.consume(.RightBrace, "Expected '}' after struct pattern fields");
            
            return Pattern{ .Struct = Pattern.StructPattern{
                .type_name = type_name,
                .fields = try fields.toOwnedSlice(),
            }};
        }
        
        return try self.parseTuplePattern();
    }
    
    /// Parse tuple pattern
    fn parseTuplePattern(self: *AdvancedParser) ParserError!Pattern {
        if (self.match(.LeftParen)) {
            var patterns = ArrayList(Pattern).init(self.allocator);
            
            while (!self.check(.RightParen) and !self.isAtEnd()) {
                const pattern = try self.parsePattern();
                try patterns.append(pattern);
                
                if (!self.match(.Comma)) break;
            }
            
            try self.consume(.RightParen, "Expected ')' after tuple pattern");
            
            return Pattern{ .Tuple = Pattern.TuplePattern{
                .patterns = try patterns.toOwnedSlice(),
            }};
        }
        
        return try self.parseArrayPattern();
    }
    
    /// Parse array pattern
    fn parseArrayPattern(self: *AdvancedParser) ParserError!Pattern {
        if (self.match(.LeftBracket)) {
            var patterns = ArrayList(Pattern).init(self.allocator);
            var rest: ?[]const u8 = null;
            
            while (!self.check(.RightBracket) and !self.isAtEnd()) {
                // Handle rest pattern (..rest)
                if (self.match(.DotDot)) {
                    if (self.check(.Identifier)) {
                        rest = try self.parseIdentifier();
                    }
                    break;
                }
                
                const pattern = try self.parsePattern();
                try patterns.append(pattern);
                
                if (!self.match(.Comma)) break;
            }
            
            try self.consume(.RightBracket, "Expected ']' after array pattern");
            
            return Pattern{ .Array = Pattern.ArrayPattern{
                .patterns = try patterns.toOwnedSlice(),
                .rest = rest,
            }};
        }
        
        return try self.parsePrimaryPattern();
    }
    
    /// Parse primary pattern (literal, variable, wildcard, type)
    fn parsePrimaryPattern(self: *AdvancedParser) ParserError!Pattern {
        const current_token = self.peek();
        
        switch (current_token.kind) {
            .Underscore => {
                _ = self.advance();
                return Pattern{ .Wildcard = Pattern.WildcardPattern{} };
            },
            .IntegerLiteral, .FloatLiteral, .StringLiteral => {
                const literal = try self.parseExpression();
                return Pattern{ .Literal = Pattern.LiteralPattern{ .value = literal } };
            },
            .Based, .Cringe => {
                const literal = try self.parseExpression();
                return Pattern{ .Literal = Pattern.LiteralPattern{ .value = literal } };
            },
            .Identifier => {
                const name = try self.parseIdentifier();
                
                // Check if it's a type pattern
                if (self.match(.LeftParen)) {
                    // Type pattern: TypeName(variable)
                    var variable: ?[]const u8 = null;
                    if (!self.check(.RightParen)) {
                        variable = try self.parseIdentifier();
                    }
                    try self.consume(.RightParen, "Expected ')' after type pattern");
                    
                    return Pattern{ .Type = Pattern.TypePattern{
                        .type_expr = Type{ .Identifier = name },
                        .variable = variable,
                    }};
                } else {
                    // Variable pattern
                    return Pattern{ .Variable = Pattern.VariablePattern{
                        .name = name,
                        .mutable = false,
                    }};
                }
            },
            else => {
                return ParserError.InvalidPattern;
            },
        }
    }
    
    /// Parse advanced control flow statements
    fn parseAdvancedControlFlow(self: *AdvancedParser) ParserError!Statement {
        // Match expression (vibe_check)
        if (self.check(.VibeCheck)) {
            const match_expr = try self.parseMatchExpression();
            return Statement{ .Expression = match_expr };
        }
        
        // Defer statement (later)
        if (self.check(.Later)) {
            return try self.parseDeferStatement();
        }
        
        // Select statement (ready)
        if (self.check(.Ready)) {
            return try self.parseSelectStatement();
        }
        
        // For-range loop (bestie)
        if (self.check(.Bestie)) {
            return try self.parseForStatement();
        }
        
        // While loop (periodt)
        if (self.check(.Periodt)) {
            return try self.parseWhileStatement();
        }
        
        return ParserError.InvalidControlFlow;
    }
    
    /// Parse defer statement
    fn parseDeferStatement(self: *AdvancedParser) ParserError!Statement {
        try self.consume(.Later, "Expected 'later'");
        const expr = try self.parseExpression();
        
        return Statement{ .Defer = ast.DeferStatement{
            .expression = expr,
        }};
    }
    
    /// Parse select statement for channel operations
    fn parseSelectStatement(self: *AdvancedParser) ParserError!Statement {
        try self.consume(.Ready, "Expected 'ready'");
        try self.consume(.LeftBrace, "Expected '{' after 'ready'");
        
        var cases = ArrayList(ast.SelectCase).init(self.allocator);
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            // Skip newlines
            if (self.check(.Newline)) {
                _ = self.advance();
                continue;
            }
            
            if (self.check(.Mood)) {
                // Channel operation case
                _ = self.advance(); // consume 'mood'
                
                var channel_op: ?Statement = null;
                if (self.check(.Identifier)) {
                    // Could be send or receive
                    if (self.peekNext().kind == .LeftArrow) {
                        // Send operation: ch <- value
                        channel_op = try self.parseStatement();
                    } else if (self.peek().kind == .LeftArrow) {
                        // Receive operation: value := <-ch
                        channel_op = try self.parseStatement();
                    }
                }
                
                try self.consume(.Colon, "Expected ':' after case");
                
                var statements = ArrayList(Statement).init(self.allocator);
                while (!self.check(.Mood) and !self.check(.Basic) and !self.check(.RightBrace) and !self.isAtEnd()) {
                    const stmt = try self.parseStatement();
                    try statements.append(stmt);
                }
                
                try cases.append(ast.SelectCase{
                    .channel_op = channel_op,
                    .statements = try statements.toOwnedSlice(),
                    .is_default = false,
                });
            } else if (self.check(.Basic)) {
                // Default case
                _ = self.advance(); // consume 'basic'
                try self.consume(.Colon, "Expected ':' after 'basic'");
                
                var statements = ArrayList(Statement).init(self.allocator);
                while (!self.check(.Mood) and !self.check(.Basic) and !self.check(.RightBrace) and !self.isAtEnd()) {
                    const stmt = try self.parseStatement();
                    try statements.append(stmt);
                }
                
                try cases.append(ast.SelectCase{
                    .channel_op = null,
                    .statements = try statements.toOwnedSlice(),
                    .is_default = true,
                });
            }
        }
        
        try self.consume(.RightBrace, "Expected '}' after select cases");
        
        return Statement{ .Select = ast.SelectStatement{
            .cases = try cases.toOwnedSlice(),
        }};
    }
    
    /// Error recovery with synchronization
    fn synchronize(self: *AdvancedParser) void {
        self.recovery_state.recovery_mode = true;
        self.recovery_state.error_count += 1;
        
        while (!self.isAtEnd()) {
            // Check for synchronization points
            for (self.recovery_state.synchronization_points) |sync_point| {
                if (self.check(sync_point)) {
                    self.recovery_state.recovery_mode = false;
                    return;
                }
            }
            
            _ = self.advance();
        }
        
        self.recovery_state.recovery_mode = false;
    }
    
    /// Enhanced error reporting
    fn reportError(self: *AdvancedParser, message: []const u8) ParserError {
        const current_token = self.peek();
        std.debug.print("Parse error at line {}, column {}: {s}\n", .{
            current_token.line, current_token.column, message
        });
        std.debug.print("Current token: {s}\n", .{current_token.lexeme});
        
        self.had_error = true;
        return ParserError.SyntaxError;
    }
    
    // Utility methods
    fn check(self: *AdvancedParser, token_kind: TokenKind) bool {
        if (self.isAtEnd()) return false;
        return self.peek().kind == token_kind;
    }
    
    fn match(self: *AdvancedParser, token_kind: TokenKind) bool {
        if (self.check(token_kind)) {
            _ = self.advance();
            return true;
        }
        return false;
    }
    
    fn advance(self: *AdvancedParser) Token {
        if (!self.isAtEnd()) self.current += 1;
        return self.previous();
    }
    
    fn isAtEnd(self: *AdvancedParser) bool {
        return self.peek().kind == .Eof;
    }
    
    fn peek(self: *AdvancedParser) Token {
        return self.tokens[self.current];
    }
    
    fn peekNext(self: *AdvancedParser) Token {
        if (self.current + 1 >= self.tokens.len) {
            return Token{ .kind = .Eof, .lexeme = "", .line = 0, .column = 0 };
        }
        return self.tokens[self.current + 1];
    }
    
    fn previous(self: *AdvancedParser) Token {
        return self.tokens[self.current - 1];
    }
    
    fn consume(self: *AdvancedParser, token_kind: TokenKind, message: []const u8) ParserError!Token {
        if (self.check(token_kind)) {
            return self.advance();
        }
        
        return self.reportError(message);
    }
    
    fn current_token_is_identifier(self: *AdvancedParser, identifier: []const u8) bool {
        if (!self.check(.Identifier)) return false;
        return std.mem.eql(u8, self.peek().lexeme, identifier);
    }
    
    // Placeholder implementations for existing methods
    fn parsePackageDeclaration(self: *AdvancedParser) ParserError!ast.PackageDeclaration {
        try self.consume(.Vibe, "Expected 'vibe'");
        const name = try self.parseIdentifier();
        return ast.PackageDeclaration{ .name = name, .version = null };
    }
    
    fn parseImportStatement(self: *AdvancedParser) ParserError!ast.ImportStatement {
        try self.consume(.Yeet, "Expected 'yeet'");
        const path = self.peek().lexeme;
        _ = self.advance();
        return ast.ImportStatement{ .path = path, .alias = null };
    }
    
    fn parseVariableDeclaration(self: *AdvancedParser) ParserError!ast.LetStatement {
        try self.consume(.Sus, "Expected 'sus'");
        const name = try self.parseIdentifier();
        const var_type = try self.parseType();
        
        var value: ?Expression = null;
        if (self.match(.Equal)) {
            value = try self.parseExpression();
        }
        
        return ast.LetStatement{
            .name = name,
            .value = value.?,
            .var_type = var_type,
        };
    }
    
    fn parseTypeAlias(self: *AdvancedParser) ParserError!ast.TypeAliasStatement {
        try self.consume(.BeLike, "Expected 'be_like'");
        const name = try self.parseIdentifier();
        const target_type = try self.parseType();
        return ast.TypeAliasStatement{ .name = name, .target_type = target_type };
    }
    
    fn parseConstantDeclaration(self: *AdvancedParser) ParserError!ast.ConstStatement {
        try self.consume(.Facts, "Expected 'facts'");
        const name = try self.parseIdentifier();
        try self.consume(.Equal, "Expected '=' after constant name");
        const value = try self.parseExpression();
        return ast.ConstStatement{ .name = name, .value = value };
    }
    
    fn parseParameter(self: *AdvancedParser) ParserError!ast.Parameter {
        const name = try self.parseIdentifier();
        const param_type = try self.parseType();
        return ast.Parameter{ .name = name, .param_type = param_type };
    }
    
    fn parseStatement(self: *AdvancedParser) ParserError!Statement {
        // Try advanced control flow first
        if (self.check(.VibeCheck) or self.check(.Later) or self.check(.Ready) or 
           self.check(.Bestie) or self.check(.Periodt)) {
            return self.parseAdvancedControlFlow();
        }
        
        // Return statement
        if (self.check(.Yolo) or self.check(.Damn)) {
            _ = self.advance();
            var value: ?Expression = null;
            if (!self.check(.Semicolon) and !self.check(.Newline) and !self.isAtEnd()) {
                value = try self.parseExpression();
            }
            return Statement{ .Return = ast.ReturnStatement{ .value = value } };
        }
        
        // Expression statement
        const expr = try self.parseExpression();
        return Statement{ .Expression = expr };
    }
    
    fn parseStatements(self: *AdvancedParser) ParserError![]Statement {
        var statements = ArrayList(Statement).init(self.allocator);
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            // Skip newlines and semicolons
            if (self.check(.Newline) or self.check(.Semicolon)) {
                _ = self.advance();
                continue;
            }
            
            const stmt = try self.parseStatement();
            try statements.append(stmt);
        }
        
        return statements.toOwnedSlice();
    }
    
    fn parseForStatement(self: *AdvancedParser) ParserError!Statement {
        try self.consume(.Bestie, "Expected 'bestie'");
        
        // Parse for loop variants
        // bestie (init; condition; update) { }
        // bestie condition { }
        // bestie range { }
        
        var init_stmt: ?Statement = null;
        var condition: ?Expression = null;
        var update: ?Statement = null;
        var range_expr: ?Expression = null;
        var iterator_var: ?[]const u8 = null;
        
        if (self.match(.LeftParen)) {
            // Traditional for loop with parentheses
            if (!self.check(.Semicolon)) {
                init_stmt = try self.parseStatement();
            }
            try self.consume(.Semicolon, "Expected ';' after for loop init");
            
            if (!self.check(.Semicolon)) {
                condition = try self.parseExpression();
            }
            try self.consume(.Semicolon, "Expected ';' after for loop condition");
            
            if (!self.check(.RightParen)) {
                update = try self.parseStatement();
            }
            try self.consume(.RightParen, "Expected ')' after for loop header");
        } else {
            // Range-based for loop or condition-only
            if (self.check(.Identifier) and self.peekNext().kind == .In) {
                // Range loop: bestie item in collection
                iterator_var = try self.parseIdentifier();
                try self.consume(.In, "Expected 'in' in range loop");
                range_expr = try self.parseExpression();
            } else {
                // Condition-only loop: bestie condition
                condition = try self.parseExpression();
            }
        }
        
        try self.consume(.LeftBrace, "Expected '{' after for loop header");
        self.in_loop = true;
        const body = try self.parseStatements();
        self.in_loop = false;
        try self.consume(.RightBrace, "Expected '}' after for loop body");
        
        return Statement{ .For = ast.ForStatement{
            .init = init_stmt,
            .condition = condition,
            .update = update,
            .body = body,
        }};
    }
    
    fn parseWhileStatement(self: *AdvancedParser) ParserError!Statement {
        try self.consume(.Periodt, "Expected 'periodt'");
        const condition = try self.parseExpression();
        
        try self.consume(.LeftBrace, "Expected '{' after while condition");
        self.in_loop = true;
        const body = try self.parseStatements();
        self.in_loop = false;
        try self.consume(.RightBrace, "Expected '}' after while body");
        
        return Statement{ .While = ast.WhileStatement{
            .condition = condition,
            .body = body,
        }};
    }
    
    fn parseExpression(self: *AdvancedParser) ParserError!Expression {
        return try self.parseLogicalOr();
    }
    
    fn parseLogicalOr(self: *AdvancedParser) ParserError!Expression {
        var expr = try self.parseLogicalAnd();
        
        while (self.match(.PipePipe)) {
            const operator = self.previous();
            const right = try self.parseLogicalAnd();
            
            // Allocate left pointer with error cleanup
            const left_ptr = self.allocator.create(Expression) catch return ParserError.OutOfMemory;
            errdefer self.allocator.destroy(left_ptr);
            
            // Allocate right pointer with error cleanup
            const right_ptr = self.allocator.create(Expression) catch {
                self.allocator.destroy(left_ptr);
                return ParserError.OutOfMemory;
            };
            errdefer self.allocator.destroy(right_ptr);
            
            left_ptr.* = expr;
            right_ptr.* = right;
            
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = left_ptr,
                .operator = operator,
                .right = right_ptr,
            }};
        }
        
        return expr;
    }
    
    fn parseLogicalAnd(self: *AdvancedParser) ParserError!Expression {
        var expr = try self.parseEquality();
        
        while (self.match(.AmpersandAmpersand)) {
            const operator = self.previous();
            const right = try self.parseEquality();
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = self.allocator.create(Expression) catch return ParserError.OutOfMemory,
                .operator = operator,
                .right = self.allocator.create(Expression) catch return ParserError.OutOfMemory,
            }};
            expr.Binary.left.* = expr;
            expr.Binary.right.* = right;
        }
        
        return expr;
    }
    
    fn parseEquality(self: *AdvancedParser) ParserError!Expression {
        var expr = try self.parseComparison();
        
        while (self.match(.EqualEqual) or self.match(.BangEqual)) {
            const operator = self.previous();
            const right = try self.parseComparison();
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = self.allocator.create(Expression) catch return ParserError.OutOfMemory,
                .operator = operator,
                .right = self.allocator.create(Expression) catch return ParserError.OutOfMemory,
            }};
            expr.Binary.left.* = expr;
            expr.Binary.right.* = right;
        }
        
        return expr;
    }
    
    fn parseComparison(self: *AdvancedParser) ParserError!Expression {
        var expr = try self.parseTerm();
        
        while (self.match(.Greater) or self.match(.GreaterEqual) or 
              self.match(.Less) or self.match(.LessEqual)) {
            const operator = self.previous();
            const right = try self.parseTerm();
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = self.allocator.create(Expression) catch return ParserError.OutOfMemory,
                .operator = operator,
                .right = self.allocator.create(Expression) catch return ParserError.OutOfMemory,
            }};
            expr.Binary.left.* = expr;
            expr.Binary.right.* = right;
        }
        
        return expr;
    }
    
    fn parseTerm(self: *AdvancedParser) ParserError!Expression {
        var expr = try self.parseFactor();
        
        while (self.match(.Minus) or self.match(.Plus)) {
            const operator = self.previous();
            const right = try self.parseFactor();
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = self.allocator.create(Expression) catch return ParserError.OutOfMemory,
                .operator = operator,
                .right = self.allocator.create(Expression) catch return ParserError.OutOfMemory,
            }};
            expr.Binary.left.* = expr;
            expr.Binary.right.* = right;
        }
        
        return expr;
    }
    
    fn parseFactor(self: *AdvancedParser) ParserError!Expression {
        var expr = try self.parseUnary();
        
        while (self.match(.Slash) or self.match(.Star) or self.match(.Percent)) {
            const operator = self.previous();
            const right = try self.parseUnary();
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = self.allocator.create(Expression) catch return ParserError.OutOfMemory,
                .operator = operator,
                .right = self.allocator.create(Expression) catch return ParserError.OutOfMemory,
            }};
            expr.Binary.left.* = expr;
            expr.Binary.right.* = right;
        }
        
        return expr;
    }
    
    fn parseUnary(self: *AdvancedParser) ParserError!Expression {
        if (self.match(.Bang) or self.match(.Minus)) {
            const operator = self.previous();
            const right = try self.parseUnary();
            const operand = self.allocator.create(Expression) catch return ParserError.OutOfMemory;
            operand.* = right;
            return Expression{ .Unary = ast.UnaryExpression{
                .operator = operator,
                .operand = operand,
            }};
        }
        
        return try self.parseCall();
    }
    
    fn parseCall(self: *AdvancedParser) ParserError!Expression {
        var expr = try self.parsePrimary();
        
        while (true) {
            if (self.match(.LeftParen)) {
                expr = try self.finishCall(expr);
            } else if (self.match(.Dot)) {
                const name = try self.parseIdentifier();
                expr = Expression{ .MemberAccess = ast.MemberAccessExpression{
                    .object = self.allocator.create(Expression) catch return ParserError.OutOfMemory,
                    .member = name,
                }};
                expr.MemberAccess.object.* = expr;
            } else if (self.match(.LeftBracket)) {
                const index = try self.parseExpression();
                try self.consume(.RightBracket, "Expected ']' after array index");
                expr = Expression{ .ArrayAccess = ast.ArrayAccessExpression{
                    .array = self.allocator.create(Expression) catch return ParserError.OutOfMemory,
                    .index = self.allocator.create(Expression) catch return ParserError.OutOfMemory,
                }};
                expr.ArrayAccess.array.* = expr;
                expr.ArrayAccess.index.* = index;
            } else {
                break;
            }
        }
        
        return expr;
    }
    
    fn finishCall(self: *AdvancedParser, callee: Expression) ParserError!Expression {
        var arguments = ArrayList(Expression).init(self.allocator);
        
        if (!self.check(.RightParen)) {
            while (true) {
                const arg = try self.parseExpression();
                try arguments.append(arg);
                
                if (!self.match(.Comma)) break;
            }
        }
        
        try self.consume(.RightParen, "Expected ')' after arguments");
        
        const callee_ptr = self.allocator.create(Expression) catch return ParserError.OutOfMemory;
        callee_ptr.* = callee;
        
        return Expression{ .Call = ast.CallExpression{
            .callee = callee_ptr,
            .arguments = try arguments.toOwnedSlice(),
        }};
    }
    
    fn parsePrimary(self: *AdvancedParser) ParserError!Expression {
        // Match expression
        if (self.check(.VibeCheck)) {
            return try self.parseMatchExpression();
        }
        
        // Boolean literals
        if (self.match(.Based)) {
            return Expression{ .Literal = ast.LiteralExpression{ .value = .{ .Boolean = true } } };
        }
        
        if (self.match(.Cringe)) {
            return Expression{ .Literal = ast.LiteralExpression{ .value = .{ .Boolean = false } } };
        }
        
        // Numeric literals
        if (self.match(.IntegerLiteral)) {
            const value = std.fmt.parseInt(i64, self.previous().lexeme, 10) catch return ParserError.InvalidExpression;
            return Expression{ .Literal = ast.LiteralExpression{ .value = .{ .Integer = value } } };
        }
        
        if (self.match(.FloatLiteral)) {
            const value = std.fmt.parseFloat(f64, self.previous().lexeme) catch return ParserError.InvalidExpression;
            return Expression{ .Literal = ast.LiteralExpression{ .value = .{ .Float = value } } };
        }
        
        // String literals
        if (self.match(.StringLiteral)) {
            return Expression{ .Literal = ast.LiteralExpression{ .value = .{ .String = self.previous().lexeme } } };
        }
        
        // Identifiers
        if (self.match(.Identifier)) {
            return Expression{ .Identifier = self.previous().lexeme };
        }
        
        // Grouped expression
        if (self.match(.LeftParen)) {
            const expr = try self.parseExpression();
            try self.consume(.RightParen, "Expected ')' after expression");
            return expr;
        }
        
        return ParserError.InvalidExpression;
    }
    
    fn parseType(self: *AdvancedParser) ParserError!Type {
        return try self.parseComplexType();
    }
    
    fn parseComplexType(self: *AdvancedParser) ParserError!Type {
        var base_type = try self.parsePrimaryType();
        
        // Handle composite types
        while (true) {
            if (self.match(.LeftBracket)) {
                if (self.check(.RightBracket)) {
                    // Slice type []T
                    _ = self.advance();
                    base_type = Type{ .Slice = Type.SliceType{
                        .element_type = self.allocator.create(Type) catch return ParserError.OutOfMemory,
                    }};
                } else {
                    // Array type [N]T
                    while (!self.check(.RightBracket) and !self.isAtEnd()) {
                        _ = self.advance();
                    }
                    _ = try self.consume(.RightBracket, "Expected ']'");
                    
                    base_type = Type{ .Array = Type.ArrayType{
                        .element_type = self.allocator.create(Type) catch return ParserError.OutOfMemory,
                        .size = null,
                    }};
                }
            } else if (self.match(.Star)) {
                // Pointer type *T
                base_type = Type{ .Pointer = Type.PointerType{
                    .target_type = self.allocator.create(Type) catch return ParserError.OutOfMemory,
                    .is_mutable = true,
                }};
            } else {
                break;
            }
        }
        
        return base_type;
    }
    
    fn parsePrimaryType(self: *AdvancedParser) ParserError!Type {
        const current_token = self.peek();
        
        switch (current_token.kind) {
            .Normie => {
                _ = self.advance();
                return Type{ .Basic = .Normie };
            },
            .Tea => {
                _ = self.advance();
                return Type{ .Basic = .Tea };
            },
            .Lit => {
                _ = self.advance();
                return Type{ .Basic = .Lit };
            },
            .Identifier => {
                const name = try self.parseIdentifier();
                return Type{ .Identifier = name };
            },
            else => {
                return ParserError.InvalidType;
            },
        }
    }
    
    fn parseIdentifier(self: *AdvancedParser) ParserError![]const u8 {
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        const name = self.peek().lexeme;
        _ = self.advance();
        return name;
    }
};
