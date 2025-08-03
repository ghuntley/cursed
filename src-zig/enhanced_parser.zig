const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const error_reporting = @import("enhanced_error_reporting.zig");
const enhanced_lexer = @import("enhanced_lexer.zig");

const ErrorReporter = error_reporting.ErrorReporter;
const ErrorCode = error_reporting.ErrorCode;
const SourceLocation = error_reporting.SourceLocation;
const Token = enhanced_lexer.Token;
const TokenKind = enhanced_lexer.TokenKind;

const ast = @import("ast_simple.zig");
const Program = ast.Program;
const Statement = ast.Statement;
const Expression = ast.Expression;

/// Enhanced parser with comprehensive error reporting and recovery
pub const Parser = struct {
    tokens: []const Token,
    current: usize,
    allocator: Allocator,
    error_reporter: *ErrorReporter,
    in_function: bool,
    in_loop: bool,
    scope_depth: usize,
    panic_mode: bool,  // For error recovery
    
    pub fn init(allocator: Allocator, tokens: []const Token, error_reporter: *ErrorReporter) Parser {
        return Parser{
            .tokens = tokens,
            .current = 0,
            .allocator = allocator,
            .error_reporter = error_reporter,
            .in_function = false,
            .in_loop = false,
            .scope_depth = 0,
            .panic_mode = false,
        };
    }
    
    pub fn parseProgram(self: *Parser) !Program {
        var program = Program.init(self.allocator);
        
        while (!self.isAtEnd()) {
            // Skip error tokens and newlines for recovery
            if (self.check(.Error) or self.check(.Newline)) {
                _ = self.advance();
                continue;
            }
            
            // Parse package declaration
            if (self.check(.Vibe)) {
                if (self.parsePackageDeclaration()) |package_decl| {
                    program.package = package_decl;
                } else |err| {
                    try self.handleParseError(err, "Failed to parse package declaration");
                    self.synchronize();
                }
                continue;
            }
            
            // Parse import statement
            if (self.check(.Yeet)) {
                if (self.parseImportStatement()) |import_stmt| {
                    try program.imports.append(import_stmt);
                } else |err| {
                    try self.handleParseError(err, "Failed to parse import statement");
                    self.synchronize();
                }
                continue;
            }
            
            // Parse regular statements
            if (self.parseStatement()) |stmt| {
                try program.statements.append(stmt);
            } else |err| {
                try self.handleParseError(err, "Failed to parse statement");
                self.synchronize();
            }
        }
        
        return program;
    }
    
    fn parsePackageDeclaration(self: *Parser) !ast.PackageDeclaration {
        _ = try self.consume(.Vibe, "Expected 'vibe' keyword");
        
        if (!self.check(.Identifier)) {
            try self.reportError(.E102_ExpectedToken, "Expected package name after 'vibe'", self.peek().location);
            return error.ParseError;
        }
        
        const name_token = self.advance();
        
        return ast.PackageDeclaration{
            .name = name_token.lexeme,
            .version = null,
        };
    }
    
    fn parseImportStatement(self: *Parser) !ast.ImportStatement {
        _ = try self.consume(.Yeet, "Expected 'yeet' keyword");
        
        if (!self.check(.StringLiteral)) {
            try self.reportError(.E102_ExpectedToken, "Expected string literal for import path", self.peek().location);
            try self.reportSuggestion("Import syntax: yeet \"module_name\"", null);
            return error.ParseError;
        }
        
        const path_token = self.advance();
        const path = if (path_token.lexeme.len >= 2 and 
                        path_token.lexeme[0] == '"' and 
                        path_token.lexeme[path_token.lexeme.len-1] == '"')
                     path_token.lexeme[1..path_token.lexeme.len-1] // Remove quotes
                     else path_token.lexeme;
        
        var import_stmt = ast.ImportStatement.init(self.allocator, path);
        
        // Handle optional alias
        if (self.match(.As)) {
            if (self.check(.Identifier)) {
                import_stmt.alias = self.advance().lexeme;
            } else {
                try self.reportError(.E102_ExpectedToken, "Expected identifier after 'as'", self.peek().location);
                try self.reportSuggestion("Alias syntax: yeet \"module\" as alias_name", null);
            }
        }
        
        return import_stmt;
    }
    
    fn parseStatement(self: *Parser) !Statement {
        // Function declaration (slay)
        if (self.check(.Slay)) {
            return Statement{ .Function = try self.parseFunctionStatement() };
        }
        
        // Variable declaration (sus/facts)
        if (self.check(.Sus) or self.check(.Facts)) {
            return Statement{ .Let = try self.parseLetStatement() };
        }
        
        // Return statement (damn)
        if (self.matchIdentifier("damn")) {
            return try self.parseReturnStatement();
        }
        
        // If statement (lowkey)
        if (self.check(.Lowkey)) {
            return Statement{ .If = try self.parseIfStatement() };
        }
        
        // While loop (periodt)
        if (self.check(.Periodt)) {
            return Statement{ .While = try self.parseWhileStatement() };
        }
        
        // For loop (bestie)
        if (self.check(.Bestie)) {
            return Statement{ .For = try self.parseForStatement() };
        }
        
        // Break statement (ghosted)
        if (self.check(.Ghosted)) {
            return try self.parseBreakStatement();
        }
        
        // Continue statement (simp)
        if (self.check(.Simp)) {
            return try self.parseContinueStatement();
        }
        
        // Struct declaration (squad)
        if (self.check(.Squad)) {
            return Statement{ .Struct = try self.parseStructStatement() };
        }
        
        // Interface declaration (collab)
        if (self.check(.Collab)) {
            return Statement{ .Interface = try self.parseInterfaceStatement() };
        }
        
        // Implementation (flex)
        if (self.check(.Flex)) {
            return Statement{ .Implementation = try self.parseImplementationStatement() };
        }
        
        // Expression statement (assignment, function calls, etc.)
        const expr = try self.parseExpression();
        return Statement{ .Expression = expr };
    }
    
    fn parseFunctionStatement(self: *Parser) !ast.FunctionStatement {
        _ = try self.consume(.Slay, "Expected 'slay' keyword");
        
        if (!self.check(.Identifier)) {
            try self.reportError(.E102_ExpectedToken, "Expected function name after 'slay'", self.peek().location);
            try self.reportSuggestion("Function syntax: slay functionName(params) returnType { ... }", null);
            return error.ParseError;
        }
        
        const name_token = self.advance();
        const old_in_function = self.in_function;
        self.in_function = true;
        defer self.in_function = old_in_function;
        
        // Parse parameters
        _ = try self.consume(.LeftParen, "Expected '(' after function name");
        
        var parameters = ArrayList(ast.Parameter).init(self.allocator);
        
        if (!self.check(.RightParen)) {
            while (true) {
                if (self.parseParameter()) |param| {
                    try parameters.append(param);
                } else |err| {
                    try self.handleParseError(err, "Failed to parse function parameter");
                    // Try to recover by skipping to next comma or closing paren
                    while (!self.check(.Comma) and !self.check(.RightParen) and !self.isAtEnd()) {
                        _ = self.advance();
                    }
                }
                
                if (!self.match(.Comma)) break;
                
                if (self.check(.RightParen)) {
                    try self.reportWarning(.E104_InvalidSyntax, "Trailing comma in parameter list", self.previous().location);
                    break;
                }
            }
        }
        
        _ = try self.consume(.RightParen, "Expected ')' after parameters");
        
        // Parse return type (optional)
        var return_type: ?ast.Type = null;
        if (self.match(.Arrow) or self.match(.Identifier)) {
            // Handle both -> Type and just Type syntax
            if (self.previous().kind == .Arrow) {
                if (!self.check(.Identifier)) {
                    try self.reportError(.E102_ExpectedToken, "Expected return type after '->'", self.peek().location);
                    try self.reportSuggestion("Return type syntax: slay func() -> normie { ... }", null);
                    return error.ParseError;
                }
            }
            
            return_type = try self.parseType();
        }
        
        // Parse function body
        _ = try self.consume(.LeftBrace, "Expected '{' before function body");
        
        var body = ArrayList(Statement).init(self.allocator);
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.parseStatement()) |stmt| {
                try body.append(stmt);
            } else |err| {
                try self.handleParseError(err, "Failed to parse statement in function body");
                self.synchronize();
            }
        }
        
        _ = try self.consume(.RightBrace, "Expected '}' after function body");
        
        return ast.FunctionStatement{
            .name = name_token.lexeme,
            .parameters = try parameters.toOwnedSlice(),
            .return_type = return_type,
            .body = try body.toOwnedSlice(),
            .allocator = self.allocator,
        };
    }
    
    fn parseParameter(self: *Parser) !ast.Parameter {
        if (!self.check(.Identifier)) {
            try self.reportError(.E102_ExpectedToken, "Expected parameter name", self.peek().location);
            try self.reportSuggestion("Parameter syntax: name type", null);
            return error.ParseError;
        }
        
        const name_token = self.advance();
        
        if (!self.check(.Identifier)) {
            try self.reportError(.E102_ExpectedToken, "Expected parameter type", self.peek().location);
            try self.reportSuggestion("Available types: normie, tea, lit, meal, smol, thicc", null);
            return error.ParseError;
        }
        
        const param_type = try self.parseType();
        
        return ast.Parameter{
            .name = name_token.lexeme,
            .param_type = param_type,
        };
    }
    
    fn parseType(self: *Parser) !ast.Type {
        if (!self.check(.Identifier) and !self.isTypeKeyword(self.peek().kind)) {
            try self.reportError(.E107_InvalidType, "Expected type name", self.peek().location);
            try self.reportSuggestion("CURSED types: normie (i32), tea (string), lit (bool), meal (f64), smol (i8), thicc (i64)", null);
            return error.ParseError;
        }
        
        const type_token = self.advance();
        
        // Handle array types [Type] or []Type
        if (self.match(.LeftBracket)) {
            if (self.match(.RightBracket)) {
                // Dynamic array []Type
                return ast.Type{
                    .Array = .{
                        .element_type = try self.allocator.create(ast.Type),
                        .size = null,
                    }
                };
            } else {
                // Fixed array [size]Type - parse size expression
                const size_expr = try self.parseExpression();
                _ = try self.consume(.RightBracket, "Expected ']' after array size");
                
                const array_type = ast.Type{
                    .Array = .{
                        .element_type = try self.allocator.create(ast.Type),
                        .size = size_expr,
                    }
                };
                array_type.Array.element_type.* = ast.Type{ .Basic = type_token.lexeme };
                return array_type;
            }
        }
        
        // Handle generic types Type<T>
        if (self.match(.Less)) {
            var type_params = ArrayList(ast.Type).init(self.allocator);
            
            while (true) {
                const param_type = try self.parseType();
                try type_params.append(param_type);
                
                if (!self.match(.Comma)) break;
            }
            
            _ = try self.consume(.Greater, "Expected '>' after generic type parameters");
            
            return ast.Type{
                .Generic = .{
                    .name = type_token.lexeme,
                    .type_params = try type_params.toOwnedSlice(),
                }
            };
        }
        
        // Basic type
        return ast.Type{ .Basic = type_token.lexeme };
    }
    
    fn parseLetStatement(self: *Parser) !ast.LetStatement {
        const keyword_token = self.advance(); // sus or facts
        const is_mutable = keyword_token.kind == .Sus;
        
        if (!self.check(.Identifier)) {
            try self.reportError(.E102_ExpectedToken, "Expected variable name", self.peek().location);
            const suggestion = if (is_mutable) "sus variableName type = value" else "facts constantName type = value";
            try self.reportSuggestion(suggestion, null);
            return error.ParseError;
        }
        
        const name_token = self.advance();
        
        // Parse optional type annotation
        var var_type: ?ast.Type = null;
        if (self.check(.Identifier) and !self.check(.Assign) and !self.check(.ColonAssign)) {
            var_type = try self.parseType();
        }
        
        // Parse assignment
        var initializer: ?Expression = null;
        if (self.match(.Assign) or self.match(.ColonAssign)) {
            initializer = try self.parseExpression();
        } else if (!is_mutable) {
            // Constants must be initialized
            try self.reportError(.E206_InvalidAssignment, "Constants declared with 'facts' must be initialized", name_token.location);
            try self.reportSuggestion("facts constantName type = value", null);
            return error.ParseError;
        }
        
        return ast.LetStatement{
            .name = name_token.lexeme,
            .var_type = var_type,
            .initializer = initializer,
            .is_mutable = is_mutable,
        };
    }
    
    fn parseReturnStatement(self: *Parser) !Statement {
        const damn_token = self.previous(); // 'damn' was already consumed
        
        if (!self.in_function) {
            try self.reportError(.E207_UnreachableCode, "Return statement outside of function", damn_token.location);
            try self.reportSuggestion("Return statements can only be used inside functions", null);
        }
        
        var value: ?Expression = null;
        if (!self.check(.Newline) and !self.check(.Semicolon) and !self.check(.RightBrace) and !self.isAtEnd()) {
            value = try self.parseExpression();
        }
        
        return Statement{ .Return = .{ .value = value } };
    }
    
    fn parseExpression(self: *Parser) !Expression {
        return self.parseAssignment();
    }
    
    fn parseAssignment(self: *Parser) !Expression {
        const expr = try self.parseOr();
        
        if (self.match(.Assign) or self.match(.PlusAssign) or self.match(.MinusAssign) or 
           self.match(.StarAssign) or self.match(.SlashAssign)) {
            const operator = self.previous();
            _ = try self.parseAssignment();
            
            return Expression{
                .Assignment = .{
                    .left = try self.allocator.create(Expression),
                    .operator = operator.lexeme,
                    .right = try self.allocator.create(Expression),
                }
            };
        }
        
        return expr;
    }
    
    fn parseOr(self: *Parser) !Expression {
        var expr = try self.parseAnd();
        
        while (self.match(.Or)) {
            const operator = self.previous();
            const right = try self.parseAnd();
            
            const binary_expr = Expression{
                .Binary = .{
                    .left = try self.allocator.create(Expression),
                    .operator = operator.lexeme,
                    .right = try self.allocator.create(Expression),
                }
            };
            binary_expr.Binary.left.* = expr;
            binary_expr.Binary.right.* = right;
            expr = binary_expr;
        }
        
        return expr;
    }
    
    fn parseAnd(self: *Parser) !Expression {
        var expr = try self.parseEquality();
        
        while (self.match(.And)) {
            const operator = self.previous();
            const right = try self.parseEquality();
            
            const binary_expr = Expression{
                .Binary = .{
                    .left = try self.allocator.create(Expression),
                    .operator = operator.lexeme,
                    .right = try self.allocator.create(Expression),
                }
            };
            binary_expr.Binary.left.* = expr;
            binary_expr.Binary.right.* = right;
            expr = binary_expr;
        }
        
        return expr;
    }
    
    fn parseEquality(self: *Parser) !Expression {
        var expr = try self.parseComparison();
        
        while (self.match(.NotEqual) or self.match(.Equal)) {
            const operator = self.previous();
            const right = try self.parseComparison();
            
            const binary_expr = Expression{
                .Binary = .{
                    .left = try self.allocator.create(Expression),
                    .operator = operator.lexeme,
                    .right = try self.allocator.create(Expression),
                }
            };
            binary_expr.Binary.left.* = expr;
            binary_expr.Binary.right.* = right;
            expr = binary_expr;
        }
        
        return expr;
    }
    
    fn parseComparison(self: *Parser) !Expression {
        var expr = try self.parseTerm();
        
        while (self.match(.Greater) or self.match(.GreaterEqual) or 
              self.match(.Less) or self.match(.LessEqual)) {
            const operator = self.previous();
            const right = try self.parseTerm();
            
            const binary_expr = Expression{
                .Binary = .{
                    .left = try self.allocator.create(Expression),
                    .operator = operator.lexeme,
                    .right = try self.allocator.create(Expression),
                }
            };
            binary_expr.Binary.left.* = expr;
            binary_expr.Binary.right.* = right;
            expr = binary_expr;
        }
        
        return expr;
    }
    
    fn parseTerm(self: *Parser) !Expression {
        var expr = try self.parseFactor();
        
        while (self.match(.Minus) or self.match(.Plus)) {
            const operator = self.previous();
            const right = try self.parseFactor();
            
            const binary_expr = Expression{
                .Binary = .{
                    .left = try self.allocator.create(Expression),
                    .operator = operator.lexeme,
                    .right = try self.allocator.create(Expression),
                }
            };
            binary_expr.Binary.left.* = expr;
            binary_expr.Binary.right.* = right;
            expr = binary_expr;
        }
        
        return expr;
    }
    
    fn parseFactor(self: *Parser) !Expression {
        var expr = try self.parseUnary();
        
        while (self.match(.Slash) or self.match(.Star) or self.match(.Percent)) {
            const operator = self.previous();
            const right = try self.parseUnary();
            
            const binary_expr = Expression{
                .Binary = .{
                    .left = try self.allocator.create(Expression),
                    .operator = operator.lexeme,
                    .right = try self.allocator.create(Expression),
                }
            };
            binary_expr.Binary.left.* = expr;
            binary_expr.Binary.right.* = right;
            expr = binary_expr;
        }
        
        return expr;
    }
    
    fn parseUnary(self: *Parser) !Expression {
        if (self.match(.Not) or self.match(.Minus)) {
            const operator = self.previous();
            _ = try self.parseUnary();
            
            return Expression{
                .Unary = .{
                    .operator = operator.lexeme,
                    .operand = try self.allocator.create(Expression),
                }
            };
        }
        
        return self.parseCall();
    }
    
    fn parseCall(self: *Parser) !Expression {
        var expr = try self.parsePrimary();
        
        while (true) {
            if (self.match(.LeftParen)) {
                expr = try self.finishCall(expr);
            } else if (self.match(.Dot)) {
                if (!self.check(.Identifier)) {
                    try self.reportError(.E102_ExpectedToken, "Expected property name after '.'", self.peek().location);
                    return error.ParseError;
                }
                const name = self.advance();
                
                const member_expr = Expression{
                    .MemberAccess = .{
                        .object = try self.allocator.create(Expression),
                        .property = name.lexeme,
                    }
                };
                member_expr.MemberAccess.object.* = expr;
                expr = member_expr;
            } else if (self.match(.LeftBracket)) {
                const index = try self.parseExpression();
                _ = try self.consume(.RightBracket, "Expected ']' after array index");
                
                const index_expr = Expression{
                    .Index = .{
                        .object = try self.allocator.create(Expression),
                        .index = try self.allocator.create(Expression),
                    }
                };
                index_expr.Index.object.* = expr;
                index_expr.Index.index.* = index;
                expr = index_expr;
            } else {
                break;
            }
        }
        
        return expr;
    }
    
    fn finishCall(self: *Parser, callee: Expression) !Expression {
        var arguments = ArrayList(Expression).init(self.allocator);
        
        if (!self.check(.RightParen)) {
            while (true) {
                const arg = try self.parseExpression();
                try arguments.append(arg);
                
                if (!self.match(.Comma)) break;
                
                if (self.check(.RightParen)) {
                    try self.reportWarning(.E104_InvalidSyntax, "Trailing comma in argument list", self.previous().location);
                    break;
                }
            }
        }
        
        _ = try self.consume(.RightParen, "Expected ')' after arguments");
        
        const call_expr = Expression{
            .Call = .{
                .callee = try self.allocator.create(Expression),
                .arguments = try arguments.toOwnedSlice(),
            }
        };
        call_expr.Call.callee.* = callee;
        
        return call_expr;
    }
    
    fn parsePrimary(self: *Parser) !Expression {
        if (self.match(.Based)) {
            return Expression{ .Boolean = true };
        }
        
        if (self.match(.Cringe)) {
            return Expression{ .Boolean = false };
        }
        
        if (self.match(.Integer)) {
            const value = std.fmt.parseInt(i64, self.previous().lexeme, 10) catch {
                try self.reportError(.E003_InvalidNumber, "Invalid integer format", self.previous().location);
                return error.ParseError;
            };
            return Expression{ .Integer = value };
        }
        
        if (self.match(.Float)) {
            const value = std.fmt.parseFloat(f64, self.previous().lexeme) catch {
                try self.reportError(.E003_InvalidNumber, "Invalid float format", self.previous().location);
                return error.ParseError;
            };
            return Expression{ .Float = value };
        }
        
        if (self.match(.StringLiteral)) {
            var lexeme = self.previous().lexeme;
            // Remove quotes if present
            if (lexeme.len >= 2 and lexeme[0] == '"' and lexeme[lexeme.len-1] == '"') {
                lexeme = lexeme[1..lexeme.len-1];
            }
            return Expression{ .String = lexeme };
        }
        
        if (self.match(.Character)) {
            var lexeme = self.previous().lexeme;
            // Remove quotes if present
            if (lexeme.len >= 2 and lexeme[0] == '\'' and lexeme[lexeme.len-1] == '\'') {
                lexeme = lexeme[1..lexeme.len-1];
            }
            return Expression{ .Character = if (lexeme.len > 0) lexeme[0] else 0 };
        }
        
        if (self.match(.Identifier)) {
            return Expression{ .Identifier = self.previous().lexeme };
        }
        
        if (self.match(.LeftParen)) {
            const expr = try self.parseExpression();
            _ = try self.consume(.RightParen, "Expected ')' after expression");
            return expr;
        }
        
        // Error: unexpected token
        try self.reportError(.E101_UnexpectedToken, "Unexpected token in expression", self.peek().location);
        try self.reportSuggestion("Expected: number, string, identifier, or '('", null);
        return error.ParseError;
    }
    
    // Helper methods for error handling and recovery
    fn handleParseError(self: *Parser, err: anyerror, context: []const u8) !void {
        if (self.panic_mode) return; // Already in panic mode
        
        self.panic_mode = true;
        
        const location = if (self.current > 0) self.tokens[self.current - 1].location else 
                        if (self.tokens.len > 0) self.tokens[0].location else
                        SourceLocation.init(self.error_reporter.allocator.dupe(u8, "unknown") catch "unknown", 1, 1, 0);
        
        const error_msg = try std.fmt.allocPrint(self.allocator, "{s}: {}", .{ context, err });
        defer self.allocator.free(error_msg);
        
        const error_code = switch (err) {
            error.UnexpectedToken => ErrorCode.E101_UnexpectedToken,
            error.UnexpectedEof => ErrorCode.E103_UnexpectedEOF,
            error.ParseError => ErrorCode.E104_InvalidSyntax,
            error.OutOfMemory => ErrorCode.E305_OutOfMemory,
            else => ErrorCode.E104_InvalidSyntax,
        };
        
        try self.error_reporter.reportError(error_code, error_msg, location);
    }
    
    fn synchronize(self: *Parser) void {
        self.panic_mode = false;
        
        _ = self.advance();
        
        while (!self.isAtEnd()) {
            if (self.previous().kind == .Semicolon or self.previous().kind == .Newline) return;
            
            switch (self.peek().kind) {
                .Slay, .Sus, .Facts, .Lowkey, .Squad, .Collab, .Flex, .Vibe, .Yeet => return,
                else => {},
            }
            
            _ = self.advance();
        }
    }
    
    fn reportError(self: *Parser, code: ErrorCode, message: []const u8, location: SourceLocation) !void {
        try self.error_reporter.reportError(code, message, location);
    }
    
    fn reportWarning(self: *Parser, code: ErrorCode, message: []const u8, location: SourceLocation) !void {
        try self.error_reporter.reportWarning(code, message, location);
    }
    
    fn reportSuggestion(self: *Parser, message: []const u8, replacement: ?[]const u8) !void {
        // Add suggestion to the last diagnostic message
        if (self.error_reporter.diagnostics.items.len > 0) {
            var last_diagnostic = &self.error_reporter.diagnostics.items[self.error_reporter.diagnostics.items.len - 1];
            const suggestion = if (replacement) |repl| 
                error_reporting.Suggestion.initWithReplacement(message, repl)
            else 
                error_reporting.Suggestion.init(message);
            
            try last_diagnostic.addSuggestion(suggestion);
        }
    }
    
    // Token matching and consumption
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
    
    fn check(self: *Parser, kind: TokenKind) bool {
        if (self.isAtEnd()) return false;
        return self.peek().kind == kind;
    }
    
    fn advance(self: *Parser) Token {
        if (!self.isAtEnd()) self.current += 1;
        return self.previous();
    }
    
    fn isAtEnd(self: *Parser) bool {
        return self.current >= self.tokens.len or self.peek().kind == .EOF;
    }
    
    fn peek(self: *Parser) Token {
        if (self.current >= self.tokens.len) {
            // Return a dummy EOF token
            return Token.init(.EOF, "", SourceLocation.init("", 0, 0, 0));
        }
        return self.tokens[self.current];
    }
    
    fn previous(self: *Parser) Token {
        if (self.current == 0 or self.current > self.tokens.len) {
            return Token.init(.EOF, "", SourceLocation.init("", 0, 0, 0));
        }
        return self.tokens[self.current - 1];
    }
    
    fn consume(self: *Parser, kind: TokenKind, message: []const u8) !Token {
        if (self.check(kind)) return self.advance();
        
        const error_msg = try std.fmt.allocPrint(self.allocator, "{s}. Got: {s}", .{ message, self.peek().kind.toString() });
        defer self.allocator.free(error_msg);
        
        try self.reportError(.E102_ExpectedToken, error_msg, self.peek().location);
        return error.ParseError;
    }
    
    fn isTypeKeyword(_: *Parser, kind: TokenKind) bool {
        return switch (kind) {
            .Normie, .Tea, .Lit, .Meal, .Smol, .Thicc => true,
            else => false,
        };
    }
    
    // Stub implementations for missing parse methods
    fn parseIfStatement(_: *Parser) !ast.IfStatement {
        // Implementation would go here
        return error.ParseError;
    }
    
    fn parseWhileStatement(_: *Parser) !ast.WhileStatement {
        // Implementation would go here
        return error.ParseError;
    }
    
    fn parseForStatement(_: *Parser) !ast.ForStatement {
        // Implementation would go here
        return error.ParseError;
    }
    
    fn parseBreakStatement(_: *Parser) !Statement {
        // Implementation would go here
        return error.ParseError;
    }
    
    fn parseContinueStatement(_: *Parser) !Statement {
        // Implementation would go here
        return error.ParseError;
    }
    
    fn parseStructStatement(_: *Parser) !ast.StructStatement {
        // Implementation would go here
        return error.ParseError;
    }
    
    fn parseInterfaceStatement(_: *Parser) !ast.InterfaceStatement {
        // Implementation would go here
        return error.ParseError;
    }
    
    fn parseImplementationStatement(_: *Parser) !ast.ImplementationStatement {
        // Implementation would go here
        return error.ParseError;
    }
};

// Testing
test "enhanced parser with error reporting" {
    const allocator = std.testing.allocator;
    
    var error_reporter = ErrorReporter.init(allocator, 10);
    defer error_reporter.deinit();
    
    // Test valid parsing
    const source = "slay main() normie { sus x normie = 42; damn x; }";
    var lexer = try enhanced_lexer.Lexer.init(allocator, source, "test.csd", &error_reporter);
    defer lexer.deinit();
    
    const tokens = try lexer.tokenize();
    defer allocator.free(tokens);
    
    var parser = Parser.init(allocator, tokens, &error_reporter);
    
    const program = try parser.parseProgram();
    defer program.deinit();
    
    try std.testing.expect(program.statements.len > 0);
    try std.testing.expect(!error_reporter.hasErrors());
}
