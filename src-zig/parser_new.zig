const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const ast = @import("ast_new.zig");

const Token = lexer.Token;
const TokenKind = lexer.TokenKind;
const Program = ast.Program;
const Statement = ast.Statement;
const Expression = ast.Expression;
const FunctionStatementData = ast.FunctionStatementData;
const LetStatementData = ast.LetStatementData;
const Type = ast.Type;
const Parameter = ast.Parameter;
const StructField = ast.StructField;
const MethodSignature = ast.MethodSignature;
const TypeParameter = ast.TypeParameter;

pub const ParserError = error{
    UnexpectedToken,
    UnexpectedEof,
    InvalidSyntax,
    OutOfMemory,
};

pub const Parser = struct {
    tokens: []const Token,
    current: usize,
    allocator: Allocator,

    pub fn init(allocator: Allocator, tokens: []const Token) Parser {
        return Parser{
            .tokens = tokens,
            .current = 0,
            .allocator = allocator,
        };
    }

    pub fn parseProgram(self: *Parser) ParserError!Program {
        var program = Program.init(self.allocator);
        
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
                try program.imports.append(allocator, import_stmt);
                continue;
            }

            // Parse regular statements
            const stmt = try self.parseStatement();
            try program.statements.append(allocator, stmt);
        }

        return program;
    }

    fn parsePackageDeclaration(self: *Parser) ParserError!ast.PackageDeclaration {
        _ = try self.consume(.Vibe, "Expected 'vibe'");
        
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.previous().lexeme;
        _ = self.advance();
        
        return ast.PackageDeclaration{
            .name = name,
            .version = null,
        };
    }

    fn parseImportStatement(self: *Parser) ParserError!ast.ImportStatement {
        _ = try self.consume(.Yeet, "Expected 'yeet'");
        
        if (!self.check(.StringLiteral)) {
            return ParserError.UnexpectedToken;
        }
        
        const path_token = self.advance();
        const path = path_token.lexeme[1..path_token.lexeme.len-1]; // Remove quotes
        
        const import_stmt = ast.ImportStatement.init(self.allocator, path);
        
        // Parse optional alias: yeet "module" as alias
        var alias: ?[]const u8 = null;
        if (self.match(.As)) {
            const alias_token = self.advance();
            if (alias_token.type != .Identifier) {
                return ParserError.ExpectedIdentifier;
            }
            alias = alias_token.lexeme;
        }
        
        // Parse optional specific imports: yeet "module" { func1, func2 }
        var specific_imports: ?[][]const u8 = null;
        if (self.match(.LeftBrace)) {
            var imports = std.ArrayList([]const u8){};
            while (!self.check(.RightBrace) and !self.isAtEnd()) {
                const import_token = self.advance();
                if (import_token.type != .Identifier) {
                    return ParserError.ExpectedIdentifier;
                }
                try imports.append(allocator, import_token.lexeme);
                if (!self.match(.Comma)) break;
            }
            if (!self.match(.RightBrace)) {
                return ParserError.ExpectedRightBrace;
            }
            specific_imports = try imports.toOwnedSlice();
        }
        
        return import_stmt;
    }

    fn parseStatement(self: *Parser) ParserError!*Statement {
        // Handle different statement types
        if (self.check(.Sus)) {
            return try self.parseLetStatement();
        } else if (self.check(.Slay)) {
            return try self.parseFunctionStatement();
        } else if (self.check(.If)) {
            return try self.parseIfStatement();
        } else if (self.check(.While)) {
            return try self.parseWhileStatement();
        } else if (self.check(.For)) {
            return try self.parseForStatement();
        } else if (self.check(.Yolo)) {
            return try self.parseReturnStatement();
        } else if (self.check(.Squad)) {
            return try self.parseStructStatement();
        } else if (self.check(.Collab)) {
            return try self.parseInterfaceStatement();
        } else {
            // Default to expression statement
            return try self.parseExpressionStatement();
        }
    }

    fn parseLetStatement(self: *Parser) ParserError!*Statement {
        _ = try self.consume(.Sus, "Expected 'sus'");
        
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // Parse optional type annotation
        var var_type: ?Type = null;
        if (self.check(.Identifier)) {
            var_type = try self.parseType();
        }
        
        // Parse optional initializer
        var initializer: ?*Expression = null;
        if (self.match(.Equal)) {
            initializer = try self.parseExpression();
        }
        
        const let_data = LetStatementData{
            .name = name,
            .var_type = var_type,
            .initializer = initializer,
            .is_mutable = true, // CURSED variables are mutable by default
        };
        
        return Statement.init(self.allocator, .{ .let = let_data });
    }

    fn parseFunctionStatement(self: *Parser) ParserError!*Statement {
        _ = try self.consume(.Slay, "Expected 'slay'");
        
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // Parse parameters
        _ = try self.consume(.LeftParen, "Expected '(' after function name");
        var parameters = std.ArrayList(u8){};
        
        if (!self.check(.RightParen)) {
            // Parse parameter list
            const param = try self.parseParameter();
            try parameters.append(allocator, param);
            
            while (self.match(.Comma)) {
                const next_param = try self.parseParameter();
                try parameters.append(allocator, next_param);
            }
        }
        
        _ = try self.consume(.RightParen, "Expected ')' after parameters");
        
        // Parse optional return type
        var return_type: ?Type = null;
        if (self.check(.Identifier)) {
            return_type = try self.parseType();
        }
        
        // Parse function body
        _ = try self.consume(.LeftBrace, "Expected '{' before function body");
        var body = std.ArrayList(u8){};
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.check(.Newline) or self.check(.Semicolon)) {
                _ = self.advance();
                continue;
            }
            const stmt = try self.parseStatement();
            try body.append(allocator, stmt);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}' after function body");
        
        const func_data = FunctionStatementData{
            .name = name,
            .parameters = parameters,
            .return_type = return_type,
            .body = body,
            .visibility = .Private,
            .is_async = false,
            .type_parameters = .empty,
            .comments = .empty,
        };
        
        return Statement.init(self.allocator, .{ .function = func_data });
    }

    fn parseIfStatement(self: *Parser) ParserError!*Statement {
        _ = try self.consume(.If, "Expected 'if'");
        
        const condition = try self.parseExpression();
        
        _ = try self.consume(.LeftBrace, "Expected '{' after if condition");
        var then_branch = std.ArrayList(u8){};
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.check(.Newline) or self.check(.Semicolon)) {
                _ = self.advance();
                continue;
            }
            const stmt = try self.parseStatement();
            try then_branch.append(allocator, stmt);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}' after if body");
        
        // Parse optional else branch
        var else_branch: ?ArrayList(*Statement) = null;
        if (self.match(.Else)) {
            _ = try self.consume(.LeftBrace, "Expected '{' after else");
            var else_stmts = std.ArrayList(u8){};
            
            while (!self.check(.RightBrace) and !self.isAtEnd()) {
                if (self.check(.Newline) or self.check(.Semicolon)) {
                    _ = self.advance();
                    continue;
                }
                const stmt = try self.parseStatement();
                try else_stmts.append(allocator, stmt);
            }
            
            _ = try self.consume(.RightBrace, "Expected '}' after else body");
            else_branch = else_stmts;
        }
        
        const if_data = ast.IfStatementData{
            .condition = condition,
            .then_branch = then_branch,
            .else_branch = else_branch,
        };
        
        return Statement.init(self.allocator, .{ .if_stmt = if_data });
    }

    fn parseWhileStatement(self: *Parser) ParserError!*Statement {
        _ = try self.consume(.While, "Expected 'while'");
        
        const condition = try self.parseExpression();
        
        _ = try self.consume(.LeftBrace, "Expected '{' after while condition");
        var body = std.ArrayList(u8){};
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.check(.Newline) or self.check(.Semicolon)) {
                _ = self.advance();
                continue;
            }
            const stmt = try self.parseStatement();
            try body.append(allocator, stmt);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}' after while body");
        
        const while_data = ast.WhileStatementData{
            .condition = condition,
            .body = body,
        };
        
        return Statement.init(self.allocator, .{ .while_stmt = while_data });
    }

    fn parseForStatement(self: *Parser) ParserError!*Statement {
        _ = try self.consume(.For, "Expected 'for'");
        _ = try self.consume(.LeftParen, "Expected '(' after 'for'");
        
        // Parse init statement (optional)
        var init: ?*Statement = null;
        if (!self.check(.Semicolon)) {
            init = try self.parseStatement();
        }
        _ = try self.consume(.Semicolon, "Expected ';' after for init");
        
        // Parse condition (optional)
        var condition: ?*Expression = null;
        if (!self.check(.Semicolon)) {
            condition = try self.parseExpression();
        }
        _ = try self.consume(.Semicolon, "Expected ';' after for condition");
        
        // Parse update statement (optional)
        var update: ?*Statement = null;
        if (!self.check(.RightParen)) {
            update = try self.parseStatement();
        }
        _ = try self.consume(.RightParen, "Expected ')' after for clauses");
        
        // Parse body
        _ = try self.consume(.LeftBrace, "Expected '{' before for body");
        var body = std.ArrayList(u8){};
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.check(.Newline) or self.check(.Semicolon)) {
                _ = self.advance();
                continue;
            }
            const stmt = try self.parseStatement();
            try body.append(allocator, stmt);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}' after for body");
        
        const for_data = ast.ForStatementData{
            .init = init,
            .condition = condition,
            .update = update,
            .body = body,
        };
        
        return Statement.init(self.allocator, .{ .for_stmt = for_data });
    }

    fn parseReturnStatement(self: *Parser) ParserError!*Statement {
        _ = try self.consume(.Yolo, "Expected 'yolo'");
        
        var value: ?*Expression = null;
        if (!self.check(.Newline) and !self.check(.Semicolon) and !self.check(.RightBrace)) {
            value = try self.parseExpression();
        }
        
        const return_data = ast.ReturnStatementData{
            .value = value,
        };
        
        return Statement.init(self.allocator, .{ .return_stmt = return_data });
    }

    fn parseStructStatement(self: *Parser) ParserError!*Statement {
        _ = try self.consume(.Squad, "Expected 'squad'");
        
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        _ = try self.consume(.LeftBrace, "Expected '{' after struct name");
        var fields = std.ArrayList(u8){};
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.check(.Newline) or self.check(.Semicolon)) {
                _ = self.advance();
                continue;
            }
            
            // Parse visibility
            var visibility = ast.Visibility.Private;
            if (self.match(.Spill)) {
                visibility = ast.Visibility.Public;
            }
            
            // Parse field name
            if (!self.check(.Identifier)) {
                return ParserError.UnexpectedToken;
            }
            const field_name = self.advance().lexeme;
            
            // Parse field type
            if (!self.check(.Identifier)) {
                return ParserError.UnexpectedToken;
            }
            // Parse full type
            const field_type = try self.parseType();
            
            const field = StructField{
                .name = field_name,
                .field_type = field_type,
                .visibility = visibility,
            };
            
            try fields.append(allocator, field);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}' after struct body");
        
        const struct_data = ast.StructStatementData{
            .name = name,
            .fields = fields,
            .visibility = .Private,
            .type_parameters = .empty,
        };
        
        return Statement.init(self.allocator, .{ .struct_stmt = struct_data });
    }

    fn parseInterfaceStatement(self: *Parser) ParserError!*Statement {
        _ = try self.consume(.Collab, "Expected 'collab'");
        
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        _ = try self.consume(.LeftBrace, "Expected '{' after interface name");
        var methods = std.ArrayList(u8){};
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.check(.Newline) or self.check(.Semicolon)) {
                _ = self.advance();
                continue;
            }
            
            // Parse method signature
            _ = try self.consume(.Slay, "Expected 'slay' for method");
            
            if (!self.check(.Identifier)) {
                return ParserError.UnexpectedToken;
            }
            const method_name = self.advance().lexeme;
            
            // Parse parameters
            _ = try self.consume(.LeftParen, "Expected '(' after method name");
            var parameters = std.ArrayList(u8){};
            
            if (!self.check(.RightParen)) {
                const param = try self.parseParameter();
                try parameters.append(allocator, param);
                
                while (self.match(.Comma)) {
                    const next_param = try self.parseParameter();
                    try parameters.append(allocator, next_param);
                }
            }
            
            _ = try self.consume(.RightParen, "Expected ')' after parameters");
            
            // Parse optional return type
            var return_type: ?Type = null;
            if (self.check(.Identifier)) {
                // Parse full return type
                return_type = try self.parseType();
            }
            
            const method = MethodSignature{
                .name = method_name,
                .parameters = parameters,
                .return_type = return_type,
            };
            
            try methods.append(allocator, method);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}' after interface body");
        
        const interface_data = ast.InterfaceStatementData{
            .name = name,
            .methods = methods,
            .visibility = .Private,
            .type_parameters = .empty,
        };
        
        return Statement.init(self.allocator, .{ .interface = interface_data });
    }

    fn parseExpressionStatement(self: *Parser) ParserError!*Statement {
        const expr = try self.parseExpression();
        return Statement.init(self.allocator, .{ .expression = expr });
    }

    fn parseExpression(self: *Parser) ParserError!*Expression {
        return try self.parseLogicalOr();
    }

    fn parseLogicalOr(self: *Parser) ParserError!*Expression {
        var expr = try self.parseLogicalAnd();

        while (self.match(.PipePipe)) {
            const operator = self.previous().lexeme;
            const right = try self.parseLogicalAnd();
            expr = try ast.createBinaryExpression(self.allocator, expr, operator, right);
        }

        return expr;
    }

    fn parseLogicalAnd(self: *Parser) ParserError!*Expression {
        var expr = try self.parseEquality();

        while (self.match(.AmpAmp)) {
            const operator = self.previous().lexeme;
            const right = try self.parseEquality();
            expr = try ast.createBinaryExpression(self.allocator, expr, operator, right);
        }

        return expr;
    }

    fn parseEquality(self: *Parser) ParserError!*Expression {
        var expr = try self.parseComparison();

        while (self.match(.EqualEqual) or self.match(.BangEqual)) {
            const operator = self.previous().lexeme;
            const right = try self.parseComparison();
            expr = try ast.createBinaryExpression(self.allocator, expr, operator, right);
        }

        return expr;
    }

    fn parseComparison(self: *Parser) ParserError!*Expression {
        var expr = try self.parseTerm();

        while (self.match(.Greater) or self.match(.GreaterEqual) or 
              self.match(.Less) or self.match(.LessEqual)) {
            const operator = self.previous().lexeme;
            const right = try self.parseTerm();
            expr = try ast.createBinaryExpression(self.allocator, expr, operator, right);
        }

        return expr;
    }

    fn parseTerm(self: *Parser) ParserError!*Expression {
        var expr = try self.parseFactor();

        while (self.match(.Minus) or self.match(.Plus)) {
            const operator = self.previous().lexeme;
            const right = try self.parseFactor();
            expr = try ast.createBinaryExpression(self.allocator, expr, operator, right);
        }

        return expr;
    }

    fn parseFactor(self: *Parser) ParserError!*Expression {
        var expr = try self.parseUnary();

        while (self.match(.Slash) or self.match(.Star)) {
            const operator = self.previous().lexeme;
            const right = try self.parseUnary();
            expr = try ast.createBinaryExpression(self.allocator, expr, operator, right);
        }

        return expr;
    }

    fn parseUnary(self: *Parser) ParserError!*Expression {
        if (self.match(.Bang) or self.match(.Minus)) {
            const operator = self.previous().lexeme;
            const right = try self.parseUnary();
            
            const unary_data = ast.UnaryExpressionData{
                .operator = operator,
                .operand = right,
            };
            
            return Expression.init(self.allocator, .{ .unary = unary_data });
        }

        return try self.parseCall();
    }

    fn parseCall(self: *Parser) ParserError!*Expression {
        var expr = try self.parsePrimary();

        while (true) {
            if (self.match(.LeftParen)) {
                expr = try self.finishCall(expr);
            } else if (self.match(.Dot)) {
                if (!self.check(.Identifier)) {
                    return ParserError.UnexpectedToken;
                }
                const name = self.advance().lexeme;
                
                const member_data = ast.MemberAccessData{
                    .object = expr,
                    .property = name,
                };
                
                expr = try Expression.init(self.allocator, .{ .member_access = member_data });
            } else {
                break;
            }
        }

        return expr;
    }

    fn finishCall(self: *Parser, callee: *Expression) ParserError!*Expression {
        var arguments = std.ArrayList(u8){};

        if (!self.check(.RightParen)) {
            const arg = try self.parseExpression();
            try arguments.append(allocator, arg);
            
            while (self.match(.Comma)) {
                const next_arg = try self.parseExpression();
                try arguments.append(allocator, next_arg);
            }
        }

        _ = try self.consume(.RightParen, "Expected ')' after arguments");

        const call_data = ast.CallExpressionData{
            .function = callee,
            .arguments = arguments,
        };

        return Expression.init(self.allocator, .{ .call = call_data });
    }

    fn parsePrimary(self: *Parser) ParserError!*Expression {
        if (self.match(.Based)) {
            return ast.createBooleanExpression(self.allocator, true);
        }

        if (self.match(.Lies)) {
            return ast.createBooleanExpression(self.allocator, false);
        }

        if (self.match(.Cap)) {
            return Expression.init(self.allocator, .{ .literal = ast.Literal.Null });
        }

        if (self.match(.Number)) {
            const value = self.previous().lexeme;
            // Try to parse as integer first
            if (std.fmt.parseInt(i64, value, 10)) |int_val| {
                return ast.createIntegerExpression(self.allocator, int_val);
            } else |_| {
                // Parse as float
                if (std.fmt.parseFloat(f64, value)) |float_val| {
                    return ast.createFloatExpression(self.allocator, float_val);
                } else |_| {
                    return ParserError.InvalidSyntax;
                }
            }
        }

        if (self.match(.StringLiteral)) {
            const value = self.previous().lexeme;
            // Remove quotes
            const string_value = value[1..value.len-1];
            return ast.createStringExpression(self.allocator, string_value);
        }

        if (self.match(.Identifier)) {
            const name = self.previous().lexeme;
            return ast.createIdentifierExpression(self.allocator, name);
        }

        if (self.match(.LeftParen)) {
            const expr = try self.parseExpression();
            _ = try self.consume(.RightParen, "Expected ')' after expression");
            return expr;
        }

        return ParserError.UnexpectedToken;
    }

    fn parseParameter(self: *Parser) ParserError!Parameter {
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // Parse parameter type
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        // Parse full type
        const param_type = try self.parseType();
        
        return Parameter{
            .name = name,
            .param_type = param_type,
            .is_mutable = false,
            .default_value = null,
        };
    }

    // Utility methods
    fn match(self: *Parser, token_type: TokenKind) bool {
        if (self.check(token_type)) {
            _ = self.advance();
            return true;
        }
        return false;
    }

    fn check(self: *Parser, token_type: TokenKind) bool {
        if (self.isAtEnd()) return false;
        return self.peek().kind == token_type;
    }

    fn advance(self: *Parser) Token {
        if (!self.isAtEnd()) self.current += 1;
        return self.previous();
    }

    fn isAtEnd(self: *Parser) bool {
        return self.peek().kind == .Eof;
    }

    // Recovery parsing methods
    fn synchronize(self: *Parser) void {
        _ = self.advance();
        
        while (!self.isAtEnd()) {
            if (self.previous().kind == .Semicolon) return;
            
            switch (self.peek().kind) {
                .Slay, .Sus, .Facts, .Squad, .Collab, .Vibe, .Yeet => return,
                else => {},
            }
            
            _ = self.advance();
        }
    }

    fn recoverToNext(self: *Parser, target_tokens: []const TokenKind) void {
        while (!self.isAtEnd()) {
            for (target_tokens) |target| {
                if (self.check(target)) return;
            }
            _ = self.advance();
        }
    }

    fn skipToStatementEnd(self: *Parser) void {
        while (!self.isAtEnd() and !self.check(.Semicolon) and !self.check(.Newline)) {
            _ = self.advance();
        }
        if (self.check(.Semicolon) or self.check(.Newline)) {
            _ = self.advance();
        }
    }

    fn peek(self: *Parser) Token {
        return self.tokens[self.current];
    }

    fn previous(self: *Parser) Token {
        return self.tokens[self.current - 1];
    }

    fn consume(self: *Parser, token_type: TokenKind, message: []const u8) ParserError!Token {
        if (self.check(token_type)) {
            return self.advance();
        }
        
        // Enhanced error reporting with location
        const current_token = if (self.current < self.tokens.len) self.tokens[self.current] else Token.init(.EOF, "", 0, 0);
        std.debug.print("Parse error at line {}, column {}: {s}. Expected {:?}, got {:?}\n", 
            .{ current_token.line, current_token.column, message, token_type, current_token.kind });
        return ParserError.UnexpectedToken;
    }

    // Complete type parsing implementation
    fn parseType(self: *Parser) ParserError!Type {
        return try self.parseComplexType();
    }
    
    fn parseComplexType(self: *Parser) ParserError!Type {
        // Parse primary type first
        var base_type = try self.parsePrimaryType();
        
        // Handle composite types (arrays, slices, channels, etc.)
        while (true) {
            if (self.match(.LeftBracket)) {
                // Array or slice type
                if (self.check(.RightBracket)) {
                    // Slice type []T
                    _ = self.advance(); // consume ']'
                    const slice_type = SliceType{
                        .element_type = self.allocator.create(Type) catch return ParserError.OutOfMemory,
                    };
                    slice_type.element_type.* = base_type;
                    base_type = Type{ .Slice = slice_type };
                } else {
                    // Array type [N]T - parse size expression
                    const size_expr = try self.parseExpression();
                    _ = try self.consume(.RightBracket, "Expected ']'");
                    
                    const array_type = ArrayType{
                        .element_type = self.allocator.create(Type) catch return ParserError.OutOfMemory,
                        .size = self.allocator.create(Expression) catch return ParserError.OutOfMemory,
                    };
                    array_type.element_type.* = base_type;
                    array_type.size.?.* = size_expr;
                    base_type = Type{ .Array = array_type };
                }
            } else if (self.match(.Asterisk)) {
                // Pointer type *T or *mut T
                var is_mutable = false;
                if (self.match(.Mut) or self.check(.Sus)) {
                    is_mutable = true;
                    if (self.check(.Sus)) _ = self.advance();
                }
                
                const pointer_type = PointerType{
                    .target_type = self.allocator.create(Type) catch return ParserError.OutOfMemory,
                    .is_mutable = is_mutable,
                };
                pointer_type.target_type.* = base_type;
                base_type = Type{ .Pointer = pointer_type };
            } else {
                break;
            }
        }
        
        return base_type;
    }
    
    fn parsePrimaryType(self: *Parser) ParserError!Type {
        const current_token = self.peek();
        
        // Parse CURSED basic types
        switch (current_token.kind) {
            .Normie => {
                _ = self.advance();
                return Type{ .Basic = .Normie };
            },
            .Tea => {
                _ = self.advance();
                return Type{ .Basic = .Tea };
            },
            .Txt => {
                _ = self.advance();
                return Type{ .Basic = .Txt };
            },
            .Lit => {
                _ = self.advance();
                return Type{ .Basic = .Lit };
            },
            .Sip => {
                _ = self.advance();
                return Type{ .Basic = .Sip };
            },
            .Smol => {
                _ = self.advance();
                return Type{ .Basic = .Smol };
            },
            .Mid => {
                _ = self.advance();
                return Type{ .Basic = .Mid };
            },
            .Thicc => {
                _ = self.advance();
                return Type{ .Basic = .Thicc };
            },
            .Snack => {
                _ = self.advance();
                return Type{ .Basic = .Snack };
            },
            .Meal => {
                _ = self.advance();
                return Type{ .Basic = .Meal };
            },
            .Byte => {
                _ = self.advance();
                return Type{ .Basic = .Byte };
            },
            .Rune => {
                _ = self.advance();
                return Type{ .Basic = .Rune };
            },
            .Extra => {
                _ = self.advance();
                return Type{ .Basic = .Extra };
            },
            .Identifier => {
                return try self.parseIdentifierType();
            },
            .LeftParen => {
                return try self.parseTupleOrFunctionType();
            },
            .Map => {
                return try self.parseMapType();
            },
            .Dm => {
                return try self.parseChannelType();
            },
            else => {
                return ParserError.UnexpectedToken;
            }
        }
    }
    
    fn parseIdentifierType(self: *Parser) ParserError!Type {
        const name = self.advance().lexeme;
        
        // Check for generic type arguments with both [T] and <T> syntax
        if (self.match(.LeftAngle) or self.match(.LeftBracket)) {
        const is_square_bracket = self.previous().token_type == .LeftBracket;
        
        // Generic type like Vec<T>, Map<K, V> or Box[T]
        var type_arguments = std.ArrayList(u8){};
        
        const closing_token = if (is_square_bracket) TokenType.RightBracket else TokenType.RightAngle;
        
        if (!self.check(closing_token)) {
        while (true) {
            const type_arg = try self.parseType();
                try type_arguments.append(allocator, type_arg);
                
                if (!self.match(.Comma)) break;
            }
        }
        
        _ = try self.consume(closing_token, if (is_square_bracket) "Expected ']' after type arguments" else "Expected '>' after type arguments");
        
        const generic_type = GenericType{
            .name = name,
            .type_arguments = type_arguments,
                .constraints = .empty,
        };
        
        return Type{ .Generic = generic_type };
    }
        
        // Simple custom type
        return Type{ .Custom = name };
    }
    
    fn parseTupleOrFunctionType(self: *Parser) ParserError!Type {
        _ = self.advance(); // consume '('
        
        var elements = std.ArrayList(u8){};
        
        if (!self.check(.RightParen)) {
            while (true) {
                const element_type = try self.parseType();
                try elements.append(allocator, element_type);
                
                if (!self.match(.Comma)) break;
            }
        }
        
        _ = try self.consume(.RightParen, "Expected ')'");
        
        // Check if this is a function type (has -> after)
        if (self.match(.Arrow)) {
            // Function type (T1, T2) -> ReturnType
            const return_type = self.allocator.create(Type) catch return ParserError.OutOfMemory;
            return_type.* = try self.parseType();
            
            const function_type = FunctionType{
                .parameters = elements,
                .return_type = return_type,
                .is_variadic = false,
            };
            
            return Type{ .Function = function_type };
        }
        
        // Tuple type (T1, T2, T3)
        const tuple_type = TupleType{ .elements = elements };
        return Type{ .Tuple = tuple_type };
    }
    
    fn parseMapType(self: *Parser) ParserError!Type {
        _ = self.advance(); // consume 'map'
        _ = try self.consume(.LeftBracket, "Expected '[' after 'map'");
        
        const key_type = self.allocator.create(Type) catch return ParserError.OutOfMemory;
        key_type.* = try self.parseType();
        
        _ = try self.consume(.RightBracket, "Expected ']' after key type");
        
        const value_type = self.allocator.create(Type) catch return ParserError.OutOfMemory;
        value_type.* = try self.parseType();
        
        const map_type = MapType{
            .key_type = key_type,
            .value_type = value_type,
        };
        
        return Type{ .Map = map_type };
    }
    
    fn parseChannelType(self: *Parser) ParserError!Type {
        _ = self.advance(); // consume 'dm'
        _ = try self.consume(.LeftAngle, "Expected '<' after 'dm'");
        
        const element_type = self.allocator.create(Type) catch return ParserError.OutOfMemory;
        element_type.* = try self.parseType();
        
        _ = try self.consume(.RightAngle, "Expected '>' after element type");
        
        const channel_type = ChannelType{
            .element_type = element_type,
            .is_send_only = false,
            .is_receive_only = false,
        };
        
        return Type{ .Channel = channel_type };
    }
};

// Note: Expression creation functions are now in ast_new.zig

// Test function
test "parser basic functionality" {
    const allocator = std.testing.allocator;
    
    // Test parsing a simple expression
    var tokens = [_]Token{
        Token{ .kind = .Number, .lexeme = "42", .line = 1, .column = 1 },
        Token{ .kind = .Plus, .lexeme = "+", .line = 1, .column = 3 },
        Token{ .kind = .Number, .lexeme = "10", .line = 1, .column = 4 },
        Token{ .kind = .Eof, .lexeme = "", .line = 1, .column = 6 },
    };
    
    var parser = Parser.init(allocator, &tokens);
    const expr = try parser.parseExpression();
    defer expr.deinit();
    
    // Verify it's a binary expression
    switch (expr.kind) {
        .binary => |binary| {
            try std.testing.expect(std.mem.eql(u8, binary.operator, "+"));
        },
        else => try std.testing.expect(false),
    }
}

test "parser program parsing" {
    const allocator = std.testing.allocator;
    
    // Test parsing a simple program
    var tokens = [_]Token{
        Token{ .kind = .Sus, .lexeme = "sus", .line = 1, .column = 1 },
        Token{ .kind = .Identifier, .lexeme = "x", .line = 1, .column = 5 },
        Token{ .kind = .Identifier, .lexeme = "normie", .line = 1, .column = 7 },
        Token{ .kind = .Equal, .lexeme = "=", .line = 1, .column = 14 },
        Token{ .kind = .Number, .lexeme = "42", .line = 1, .column = 16 },
        Token{ .kind = .Eof, .lexeme = "", .line = 1, .column = 18 },
    };
    
    var parser = Parser.init(allocator, &tokens);
    var program = try parser.parseProgram();
    defer program.deinit();
    
    try std.testing.expect(program.statements.items.len == 1);
}
