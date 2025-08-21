const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const ArenaAllocator = std.heap.ArenaAllocator;

const lexer = @import("memory_safe_lexer.zig");
const ast = @import("ast.zig");
const error_reporting = @import("memory_safe_error_reporting.zig");

const Token = lexer.Token;
const TokenKind = lexer.TokenKind;
const Program = ast.Program;
const Statement = ast.Statement;
const Expression = ast.Expression;
const FunctionStatement = ast.FunctionStatement;
const LetStatement = ast.LetStatement;
const Type = ast.Type;
const Parameter = ast.Parameter;
const ErrorReporter = error_reporting.ErrorReporter;

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
    SyntaxError,
};

/// Memory-safe parser using arena allocator
pub const Parser = struct {
    tokens: []const Token,
    current: usize,
    arena: ArenaAllocator,
    backing_allocator: Allocator,
    had_error: bool,
    in_function: bool,
    in_loop: bool,
    scope_depth: usize,
    file_path: []const u8,
    error_reporter: ?*ErrorReporter,

    pub fn init(backing_allocator: Allocator, tokens: []const Token) Parser {
        return Parser{
            .tokens = tokens,
            .current = 0,
            .arena = ArenaAllocator.init(backing_allocator),
            .backing_allocator = backing_allocator,
            .had_error = false,
            .in_function = false,
            .in_loop = false,
            .scope_depth = 0,
            .file_path = "unknown",
            .error_reporter = null,
        };
    }

    pub fn initWithFile(backing_allocator: Allocator, tokens: []const Token, file_path: []const u8) Parser {
        return Parser{
            .tokens = tokens,
            .current = 0,
            .arena = ArenaAllocator.init(backing_allocator),
            .backing_allocator = backing_allocator,
            .had_error = false,
            .in_function = false,
            .in_loop = false,
            .scope_depth = 0,
            .file_path = file_path,
            .error_reporter = null,
        };
    }
    
    pub fn setErrorReporter(self: *Parser, reporter: *ErrorReporter) void {
        self.error_reporter = reporter;
    }

    pub fn deinit(self: *Parser) void {
        // Arena automatically cleans up all AST nodes and temporary allocations
        self.arena.deinit();
    }

    pub fn parseProgram(self: *Parser) ParserError!Program {
        const arena_allocator = self.arena.allocator();
        var program = Program.init(arena_allocator);
        
        // Use errdefer for automatic cleanup on error
        errdefer {
            // Arena cleanup handles all allocated memory
            program.statements.deinit();
            program.imports.deinit();
        }
        
        while (!self.isAtEnd()) {
            // Skip newlines and semicolons
            if (self.check(.Newline) or self.check(.Semicolon)) {
                _ = self.advance();
                continue;
            }

            // Parse package declaration
            if (self.check(.Vibe)) {
                program.package = try self.parsePackageDeclaration();
                continue;
            }

            // Parse import statement
            if (self.check(.Yeet)) {
                const import_stmt = try self.parseImportStatement();
                try program.imports.append(import_stmt);
                continue;
            }

            // Parse regular statements
            const stmt = try self.parseStatement();
            try program.statements.append(stmt);
        }

        return program;
    }

    // Source location tracking methods
    fn getCurrentSourceLocation(self: *Parser) ?ast.SourceLocation {
        if (self.current < self.tokens.len) {
            const token = self.tokens[self.current];
            return ast.SourceLocation{
                .file = self.file_path,
                .line = @intCast(token.line),
                .column = @intCast(token.column),
            };
        }
        return null;
    }

    fn getSourceLocationForToken(self: *Parser, token: Token) ast.SourceLocation {
        return ast.SourceLocation{
            .file = self.file_path,
            .line = @intCast(token.line),
            .column = @intCast(token.column),
        };
    }

    // Enhanced error reporting with memory safety
    fn reportError(self: *Parser, message: []const u8) ParserError {
        const location = self.getCurrentSourceLocation();
        
        if (self.error_reporter) |reporter| {
            if (location) |loc| {
                const error_location = error_reporting.SourceLocation.init(
                    loc.file,
                    loc.line,
                    loc.column,
                    0 // char_offset not available here
                );
                reporter.reportError(.E104_InvalidSyntax, message, error_location) catch {};
            }
        } else {
            if (location) |loc| {
                std.debug.print("Error at {}:{}:{} - {s}\n", .{ loc.file, loc.line, loc.column, message });
            } else {
                std.debug.print("Error: {s}\n", .{message});
            }
        }
        
        self.had_error = true;
        return ParserError.SyntaxError;
    }

    fn reportErrorAtToken(self: *Parser, token: Token, message: []const u8) ParserError {
        const location = self.getSourceLocationForToken(token);
        
        if (self.error_reporter) |reporter| {
            const error_location = error_reporting.SourceLocation.init(
                location.file,
                location.line,
                location.column,
                0 // char_offset not available here
            );
            reporter.reportError(.E104_InvalidSyntax, message, error_location) catch {};
        } else {
            std.debug.print("Error at {}:{}:{} - {s}\n", .{ location.file, location.line, location.column, message });
        }
        
        self.had_error = true;
        return ParserError.SyntaxError;
    }

    // Recovery parsing
    fn synchronize(self: *Parser) void {
        self.advance();
        
        while (!self.isAtEnd()) {
            if (self.previous().kind == .Semicolon) return;
            
            switch (self.peek().kind) {
                .Slay, .Sus, .Facts, .Squad, .Collab, .Vibe, .Yeet => return,
                else => {},
            }
            
            self.advance();
        }
    }

    fn recoverToNext(self: *Parser, target_tokens: []const TokenKind) void {
        while (!self.isAtEnd()) {
            for (target_tokens) |target| {
                if (self.check(target)) return;
            }
            self.advance();
        }
    }

    fn parsePackageDeclaration(self: *Parser) ParserError!ast.PackageDeclaration {
        _ = try self.consume(.Vibe, "Expected 'vibe'");
        
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        return ast.PackageDeclaration{
            .name = name,
            .version = null,
        };
    }

    fn parseImportStatement(self: *Parser) ParserError!ast.ImportStatement {
        const arena_allocator = self.arena.allocator();
        
        _ = try self.consume(.Yeet, "Expected 'yeet'");
        
        if (!self.check(.StringLiteral) and !self.check(.String)) {
            return ParserError.UnexpectedToken;
        }
        
        const path_token = self.advance();
        const path = if (path_token.lexeme.len >= 2 and 
                        path_token.lexeme[0] == '"' and 
                        path_token.lexeme[path_token.lexeme.len-1] == '"')
                     path_token.lexeme[1..path_token.lexeme.len-1] // Remove quotes
                     else path_token.lexeme;
        
        var import_stmt = ast.ImportStatement.init(arena_allocator, path);
        
        // Handle alias (as name)
        if (self.match(.As)) {
            if (self.check(.Identifier)) {
                import_stmt.alias = self.advance().lexeme;
            }
        }
        
        return import_stmt;
    }

    fn parseStatement(self: *Parser) ParserError!Statement {
        // Function declaration (slay)
        if (self.check(.Slay)) {
            return Statement{ .Function = try self.parseFunctionStatement() };
        }
        
        // Variable declaration (sus/facts)
        if (self.check(.Sus) or self.check(.Facts)) {
            return Statement{ .Let = try self.parseLetStatement() };
        }
        
        // Return statement (damn only - canonical spec)
        if (self.matchIdentifier("damn")) {
            return try self.parseReturnStatement();
        }
        
        // If statement (lowkey)
        if (self.check(.Lowkey)) {
            return Statement{ .If = try self.parseIfStatement() };
        }
        
        // While statement (periodt/flex)
        if (self.check(.Periodt) or self.check(.Flex)) {
            return Statement{ .While = try self.parseWhileStatement() };
        }
        
        // For statement (bestie)
        if (self.check(.Bestie)) {
            return try self.parseForStatement();
        }
        
        // Break/continue
        if (self.check(.Ghosted)) {
            _ = self.advance();
            return Statement{ .Break = ast.BreakStatement{} };
        }
        
        if (self.check(.Simp)) {
            _ = self.advance();
            return Statement{ .Continue = ast.ContinueStatement{} };
        }
        
        // Defer statement (later)
        if (self.check(.Later)) {
            return try self.parseDeferStatement();
        }
        
        // Struct declaration (squad)
        if (self.check(.Squad) or self.check(.Struct)) {
            return try self.parseStructStatement();
        }
        
        // Interface declaration (collab)
        if (self.check(.Collab)) {
            return try self.parseInterfaceStatement();
        }

        // Implementation statement (impl)
        if (self.check(.Impl)) {
            return try self.parseImplementationStatement();
        }
        
        // Type alias (be like)
        if (self.check(.BeLike) or self.checkIdentifier("be")) {
            return try self.parseTypeAliasStatement();
        }

        // Goroutine statement (stan)
        if (self.check(.Stan)) {
            return try self.parseGoroutineStatement();
        }

        // Match expression (match)
        if (self.check(.Match)) {
            const match_expr = try self.parseMatchExpression();
            return Statement{ .Expression = match_expr };
        }

        // Vibe check (switch)
        if (self.check(.VibeCheck)) {
            return try self.parseVibeCheckStatement();
        }

        // Select statement
        if (self.check(.Select) or self.check(.Ready)) {
            return try self.parseSelectStatement();
        }
        
        // Error handling statements
        if (self.check(.Yikes)) {
            return Statement{ .Yikes = try self.parseYikesStatement() };
        }
        
        if (self.check(.Fam)) {
            return Statement{ .Fam = try self.parseFamStatement() };
        }

        // Constants (facts at top level)
        if (self.check(.Facts) and self.isTopLevel()) {
            return Statement{ .Const = try self.parseConstDeclaration() };
        }
        
        // Short variable declaration (x := value or (a, b) := (1, 2))
        if (self.isShortDeclaration()) {
            return try self.parseShortDeclaration();
        }

        // Assignment statement
        if (self.isAssignment()) {
            return try self.parseAssignmentStatement();
        }
        
        // Expression statement
        const expr = try self.parseExpression();
        return Statement{ .Expression = expr };
    }

    fn parseFunctionStatement(self: *Parser) ParserError!FunctionStatement {
        const arena_allocator = self.arena.allocator();
        
        try self.consume(.Slay, "Expected 'slay'");
        
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        var func = FunctionStatement.init(arena_allocator, name);
        
        // Parse generic type parameters <T, U>
        if (self.match(.Less) or self.match(.LeftAngle)) {
            while (!self.check(.Greater) and !self.check(.RightAngle) and !self.isAtEnd()) {
                if (self.check(.Identifier)) {
                    const param_name = self.advance().lexeme;
                    var type_param = ast.TypeParameter{
                        .name = param_name,
                        .constraints = .empty,
                    };
                    
                    // Parse constraints (T: SomeInterface)
                    if (self.match(.Colon)) {
                        while (!self.check(.Comma) and !self.check(.Greater) and !self.check(.RightAngle)) {
                            const constraint = try self.parseType();
                            try type_param.constraints.append(constraint);
                            if (!self.match(.Plus)) break;
                        }
                    }
                    
                    try func.type_parameters.append(type_param);
                }
                
                if (!self.match(.Comma)) break;
            }
            
            if (!self.match(.Greater) and !self.match(.RightAngle)) {
                return ParserError.MissingToken;
            }
        }
        
        // Parse parameters
        try self.consume(.LeftParen, "Expected '(' after function name");
        
        if (!self.check(.RightParen)) {
            while (true) {
                const param = try self.parseParameter();
                try func.parameters.append(param);
                
                if (!self.match(.Comma)) break;
            }
        }
        
        try self.consume(.RightParen, "Expected ')' after parameters");
        
        // Parse return type
        if (!self.check(.LeftBrace)) {
            func.return_type = try self.parseType();
        }
        
        // Parse body
        try self.consume(.LeftBrace, "Expected '{'");
        
        self.in_function = true;
        defer { self.in_function = false; }
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            // Skip newlines
            if (self.match(.Newline)) continue;
            
            const stmt = try self.parseStatement();
            try func.body.append(stmt);
        }
        
        try self.consume(.RightBrace, "Expected '}'");
        
        return func;
    }

    fn parseLetStatement(self: *Parser) ParserError!LetStatement {
        const is_mutable = self.match(.Sus); // sus = mutable, facts = immutable
        if (!is_mutable) {
            _ = self.match(.Facts);
        }
        
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        var let_stmt = LetStatement{
            .name = name,
            .var_type = null,
            .initializer = null,
            .is_mutable = is_mutable,
        };
        
        // Parse type annotation (sus x tea = "hello" or sus x: tea)
        if (self.match(.Colon)) {
            let_stmt.var_type = try self.parseType();
        } else if (self.checkType()) {
            let_stmt.var_type = try self.parseType();
        }
        
        // Parse initializer
        if (self.match(.Equal) or self.match(.ColonEqual)) {
            let_stmt.initializer = try self.parseExpression();
        }
        
        return let_stmt;
    }

    fn parseParameter(self: *Parser) ParserError!Parameter {
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // Parse type (required for parameters in CURSED)
        const param_type = try self.parseType();
        
        var param = Parameter{
            .name = name,
            .param_type = param_type,
            .is_mutable = false,
            .default_value = null,
        };

        // Parse default value
        if (self.match(.Equal)) {
            param.default_value = try self.parseExpression();
        }
        
        return param;
    }

    fn parseType(self: *Parser) ParserError!ast.Type {
        const arena_allocator = self.arena.allocator();
        
        // Array/slice types []element_type or [size]element_type
        if (self.match(.LeftBracket)) {
            // Check for size or empty for slice
            var size: ?usize = null;
            if (!self.check(.RightBracket)) {
                if (self.check(.Number) or self.check(.Integer)) {
                    const size_token = self.advance();
                    size = std.fmt.parseInt(usize, size_token.lexeme, 10) catch null;
                }
            }
            
            try self.consume(.RightBracket, "Expected ']'");
            
            const element_type_ptr = try arena_allocator.create(ast.Type);
            element_type_ptr.* = try self.parseType();
            
            return ast.Type{ .Array = ast.ArrayType{
                .element_type = element_type_ptr,
                .size = size,
            }};
        }

        // Map types map[key_type]value_type
        if (self.matchIdentifier("map")) {
            try self.consume(.LeftBracket, "Expected '[' after 'map'");
            
            const key_type_ptr = try arena_allocator.create(ast.Type);
            key_type_ptr.* = try self.parseType();
            
            try self.consume(.RightBracket, "Expected ']'");
            
            const value_type_ptr = try arena_allocator.create(ast.Type);
            value_type_ptr.* = try self.parseType();
            
            return ast.Type{ .Map = ast.MapType{
                .key_type = key_type_ptr,
                .value_type = value_type_ptr,
            }};
        }

        // Function types (arg1_type, arg2_type) -> return_type
        if (self.check(.LeftParen) and self.isFunctionType()) {
            _ = self.advance(); // consume '('
            
            var param_types = .empty;
            
            if (!self.check(.RightParen)) {
                while (true) {
                    const param_type = try self.parseType();
                    try param_types.append(param_type);
                    
                    if (!self.match(.Comma)) break;
                }
            }
            
            try self.consume(.RightParen, "Expected ')'");
            try self.consume(.Arrow, "Expected '->' for function type");
            
            const return_type_ptr = try arena_allocator.create(ast.Type);
            return_type_ptr.* = try self.parseType();
            
            return ast.Type{ .Function = ast.FunctionType{
                .parameter_types = param_types,
                .return_type = return_type_ptr,
            }};
        }

        // Tuple types (type1, type2, type3)
        if (self.match(.LeftParen)) {
            var element_types = .empty;
            
            if (!self.check(.RightParen)) {
                while (true) {
                    const element_type = try self.parseType();
                    try element_types.append(element_type);
                    
                    if (!self.match(.Comma)) break;
                }
            }
            
            try self.consume(.RightParen, "Expected ')'");
            
            return ast.Type{ .Tuple = ast.TupleType{
                .element_types = element_types,
            }};
        }

        // Basic types
        if (self.check(.Normie)) {
            _ = self.advance();
            return ast.Type{ .Basic = .Integer };
        }

        if (self.check(.Tea) or self.check(.Txt)) {
            _ = self.advance();
            return ast.Type{ .Basic = .String };
        }

        if (self.check(.Lit)) {
            _ = self.advance();
            return ast.Type{ .Basic = .Boolean };
        }

        if (self.check(.Meal)) {
            _ = self.advance();
            return ast.Type{ .Basic = .Float };
        }

        if (self.check(.Smol)) {
            _ = self.advance();
            return ast.Type{ .Basic = .I8 };
        }

        if (self.check(.Mid)) {
            _ = self.advance();
            return ast.Type{ .Basic = .I16 };
        }

        if (self.check(.Thicc)) {
            _ = self.advance();
            return ast.Type{ .Basic = .I64 };
        }

        if (self.check(.Snack)) {
            _ = self.advance();
            return ast.Type{ .Basic = .F32 };
        }

        if (self.check(.Byte)) {
            _ = self.advance();
            return ast.Type{ .Basic = .U8 };
        }

        if (self.check(.Sip)) {
            _ = self.advance();
            return ast.Type{ .Basic = .Char };
        }

        if (self.check(.Rune)) {
            _ = self.advance();
            return ast.Type{ .Basic = .Rune };
        }

        if (self.check(.Dm)) {
            _ = self.advance();
            // Channel type - expect generic parameter
            try self.consume(.Less, "Expected '<' after 'dm'");
            const element_type_ptr = try arena_allocator.create(ast.Type);
            element_type_ptr.* = try self.parseType();
            try self.consume(.Greater, "Expected '>' after channel element type");
            
            return ast.Type{ .Channel = ast.ChannelType{
                .element_type = element_type_ptr,
            }};
        }

        // Custom types (struct names, type aliases, etc.)
        if (self.check(.Identifier)) {
            const type_name = self.advance().lexeme;
            return ast.Type{ .Custom = type_name };
        }

        return ParserError.InvalidType;
    }

    // Memory allocation helpers using arena
    fn allocateExpression(self: *Parser, expr: Expression) ParserError!*Expression {
        const arena_allocator = self.arena.allocator();
        const ptr = arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
        ptr.* = expr;
        return ptr;
    }

    fn allocateType(self: *Parser, type_val: ast.Type) ParserError!*ast.Type {
        const arena_allocator = self.arena.allocator();
        const ptr = arena_allocator.create(ast.Type) catch return ParserError.OutOfMemory;
        ptr.* = type_val;
        return ptr;
    }

    // Stub implementations for missing methods
    fn parseExpression(self: *Parser) ParserError!Expression {
        // Simplified expression parsing for now
        if (self.check(.Number)) {
            const token = self.advance();
            return Expression{ .Literal = ast.LiteralExpression{
                .value = ast.LiteralValue{ .Integer = std.fmt.parseInt(i64, token.lexeme, 10) catch 0 },
            }};
        }
        
        if (self.check(.StringLiteral)) {
            const token = self.advance();
            return Expression{ .Literal = ast.LiteralExpression{
                .value = ast.LiteralValue{ .String = token.lexeme },
            }};
        }
        
        if (self.check(.Based)) {
            _ = self.advance();
            return Expression{ .Literal = ast.LiteralExpression{
                .value = ast.LiteralValue{ .Boolean = true },
            }};
        }
        
        if (self.check(.Cringe)) {
            _ = self.advance();
            return Expression{ .Literal = ast.LiteralExpression{
                .value = ast.LiteralValue{ .Boolean = false },
            }};
        }
        
        if (self.check(.Identifier)) {
            const token = self.advance();
            return Expression{ .Identifier = ast.IdentifierExpression{
                .name = token.lexeme,
            }};
        }
        
        return ParserError.InvalidExpression;
    }

    // Stubs for other parsing methods - implement as needed
    fn parseReturnStatement(self: *Parser) ParserError!Statement {
        _ = self;
        return ParserError.InvalidStatement;
    }

    fn parseIfStatement(self: *Parser) ParserError!ast.IfStatement {
        _ = self;
        return error.InvalidStatement;
    }

    fn parseWhileStatement(self: *Parser) ParserError!ast.WhileStatement {
        _ = self;
        return error.InvalidStatement;
    }

    fn parseForStatement(self: *Parser) ParserError!Statement {
        _ = self;
        return ParserError.InvalidStatement;
    }

    fn parseDeferStatement(self: *Parser) ParserError!Statement {
        _ = self;
        return ParserError.InvalidStatement;
    }

    fn parseStructStatement(self: *Parser) ParserError!Statement {
        _ = self;
        return ParserError.InvalidStatement;
    }

    fn parseInterfaceStatement(self: *Parser) ParserError!Statement {
        _ = self;
        return ParserError.InvalidStatement;
    }

    fn parseImplementationStatement(self: *Parser) ParserError!Statement {
        _ = self;
        return ParserError.InvalidStatement;
    }

    fn parseTypeAliasStatement(self: *Parser) ParserError!Statement {
        _ = self;
        return ParserError.InvalidStatement;
    }

    fn parseGoroutineStatement(self: *Parser) ParserError!Statement {
        _ = self;
        return ParserError.InvalidStatement;
    }

    fn parseMatchExpression(self: *Parser) ParserError!Expression {
        _ = self;
        return ParserError.InvalidExpression;
    }

    fn parseVibeCheckStatement(self: *Parser) ParserError!Statement {
        _ = self;
        return ParserError.InvalidStatement;
    }

    fn parseSelectStatement(self: *Parser) ParserError!Statement {
        _ = self;
        return ParserError.InvalidStatement;
    }

    fn parseYikesStatement(self: *Parser) ParserError!ast.YikesStatement {
        _ = self;
        return error.InvalidStatement;
    }

    fn parseFamStatement(self: *Parser) ParserError!ast.FamStatement {
        _ = self;
        return error.InvalidStatement;
    }

    fn parseConstDeclaration(self: *Parser) ParserError!ast.ConstDeclaration {
        _ = self;
        return error.InvalidStatement;
    }

    fn parseShortDeclaration(self: *Parser) ParserError!Statement {
        _ = self;
        return ParserError.InvalidStatement;
    }

    fn parseAssignmentStatement(self: *Parser) ParserError!Statement {
        _ = self;
        return ParserError.InvalidStatement;
    }

    // Helper methods
    fn isAtEnd(self: *Parser) bool {
        return self.current >= self.tokens.len or self.peek().kind == .Eof;
    }

    fn peek(self: *Parser) Token {
        if (self.current >= self.tokens.len) {
            return Token.init(.Eof, "", 0, 0);
        }
        return self.tokens[self.current];
    }

    fn previous(self: *Parser) Token {
        if (self.current == 0) {
            return Token.init(.Eof, "", 0, 0);
        }
        return self.tokens[self.current - 1];
    }

    fn advance(self: *Parser) Token {
        if (!self.isAtEnd()) self.current += 1;
        return self.previous();
    }

    fn check(self: *Parser, kind: TokenKind) bool {
        if (self.isAtEnd()) return false;
        return self.peek().kind == kind;
    }

    fn match(self: *Parser, kind: TokenKind) bool {
        if (self.check(kind)) {
            _ = self.advance();
            return true;
        }
        return false;
    }

    fn matchIdentifier(self: *Parser, text: []const u8) bool {
        if (self.check(.Identifier) and std.mem.eql(u8, self.peek().lexeme, text)) {
            _ = self.advance();
            return true;
        }
        return false;
    }

    fn checkIdentifier(self: *Parser, text: []const u8) bool {
        return self.check(.Identifier) and std.mem.eql(u8, self.peek().lexeme, text);
    }

    fn checkType(self: *Parser) bool {
        return self.check(.Normie) or self.check(.Tea) or self.check(.Lit) or 
               self.check(.Meal) or self.check(.Smol) or self.check(.Mid) or 
               self.check(.Thicc) or self.check(.Snack) or self.check(.Byte) or 
               self.check(.Sip) or self.check(.Rune) or self.check(.Dm);
    }

    fn consume(self: *Parser, kind: TokenKind, message: []const u8) ParserError!Token {
        if (self.check(kind)) return self.advance();
        
        return self.reportError(message);
    }

    fn isTopLevel(self: *Parser) bool {
        return self.scope_depth == 0;
    }

    fn isShortDeclaration(self: *Parser) bool {
        // Look ahead for := pattern
        var pos = self.current;
        
        // Handle tuple pattern (a, b) := ...
        if (pos < self.tokens.len and self.tokens[pos].kind == .LeftParen) {
            pos += 1;
            // Skip through identifiers and commas
            while (pos < self.tokens.len) {
                if (self.tokens[pos].kind == .RightParen) {
                    pos += 1;
                    break;
                }
                if (self.tokens[pos].kind != .Identifier and self.tokens[pos].kind != .Comma) {
                    return false;
                }
                pos += 1;
            }
        }
        
        // Handle single or multiple identifiers: a, b := ...
        while (pos < self.tokens.len and self.tokens[pos].kind == .Identifier) {
            pos += 1;
            if (pos < self.tokens.len and self.tokens[pos].kind == .Comma) {
                pos += 1;
            } else {
                break;
            }
        }
        
        return pos < self.tokens.len and self.tokens[pos].kind == .ColonEqual;
    }

    fn isAssignment(self: *Parser) bool {
        // Simple assignment detection
        var pos = self.current;
        
        // Skip over primary expression tokens
        while (pos < self.tokens.len) {
            const token_kind = self.tokens[pos].kind;
            if (token_kind == .Equal or token_kind == .PlusEqual or 
               token_kind == .MinusEqual or token_kind == .StarEqual or
               token_kind == .SlashEqual or token_kind == .PercentEqual) {
                return true;
            }
            if (token_kind == .Semicolon or token_kind == .Newline or 
               token_kind == .LeftBrace or token_kind == .RightBrace) {
                return false;
            }
            pos += 1;
        }
        
        return false;
    }

    fn isFunctionType(self: *Parser) bool {
        // Look ahead to see if this looks like a function type
        var pos = self.current + 1; // Skip the '('
        var paren_depth: usize = 1;
        
        while (pos < self.tokens.len and paren_depth > 0) {
            switch (self.tokens[pos].kind) {
                .LeftParen => paren_depth += 1,
                .RightParen => {
                    paren_depth -= 1;
                    if (paren_depth == 0) {
                        // Check if next token is '->' indicating function type
                        if (pos + 1 < self.tokens.len and self.tokens[pos + 1].kind == .Arrow) {
                            return true;
                        }
                    }
                },
                .Eof => break,
                else => {},
            }
            pos += 1;
        }
        
        return false;
    }
};

// Tests
test "memory safe parser basic program" {
    const allocator = std.testing.allocator;
    
    // Create tokens for "slay main_character() { }"
    const tokens = [_]Token{
        Token.init(.Slay, "slay", 1, 1),
        Token.init(.Identifier, "main_character", 1, 6),
        Token.init(.LeftParen, "(", 1, 20),
        Token.init(.RightParen, ")", 1, 21),
        Token.init(.LeftBrace, "{", 1, 23),
        Token.init(.RightBrace, "}", 1, 25),
        Token.init(.Eof, "", 1, 26),
    };
    
    var parser = Parser.init(allocator, &tokens);
    defer parser.deinit();
    
    const program = try parser.parseProgram();
    
    try std.testing.expect(program.statements.items.len == 1);
    
    switch (program.statements.items[0]) {
        .Function => |func| {
            try std.testing.expect(std.mem.eql(u8, func.name, "main_character"));
        },
        else => try std.testing.expect(false),
    }
}
