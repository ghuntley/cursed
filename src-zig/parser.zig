const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const ast = @import("ast_simple.zig");

const Token = lexer.Token;
const TokenKind = lexer.TokenKind;
const Program = ast.Program;
const Statement = ast.Statement;
const Expression = ast.Expression;
const FunctionStatement = ast.FunctionStatement;
const LetStatement = ast.LetStatement;
const Type = ast.Type;
const Parameter = ast.Parameter;

// Add missing needed types
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
                try program.imports.append(import_stmt);
                continue;
            }

            // Parse regular statements
            const stmt = try self.parseStatement();
            try program.statements.append(stmt);
        }

        return program;
    }

    fn parsePackageDeclaration(self: *Parser) ParserError!ast.PackageDeclaration {
        try self.consume(.Vibe, "Expected 'vibe'");
        
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
        try self.consume(.Yeet, "Expected 'yeet'");
        
        if (!self.check(.StringLiteral)) {
            return ParserError.UnexpectedToken;
        }
        
        const path_token = self.advance();
        const path = path_token.lexeme[1..path_token.lexeme.len-1]; // Remove quotes
        
        var import_stmt = ast.ImportStatement.init(self.allocator, path);
        
        // Handle alias (as name)
        if (self.match(.As)) {
            if (self.check(.Identifier)) {
                import_stmt.alias = self.advance().lexeme;
            }
        }
        
        return import_stmt;
    }

    fn parseStatement(self: *Parser) ParserError!Statement {
        // Function declaration
        if (self.check(.Slay)) {
            return Statement{ .Function = try self.parseFunctionStatement() };
        }
        
        // Variable declaration (sus/facts)
        if (self.check(.Sus) or self.check(.Facts)) {
            return Statement{ .Let = try self.parseLetStatement() };
        }
        
        // Return statement (yolo)
        if (self.check(.Yolo)) {
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
        
        // Error handling statements
        if (self.check(.Yikes)) {
            return Statement{ .Yikes = try self.parseYikesStatement() };
        }
        
        if (self.check(.Fam)) {
            return Statement{ .Fam = try self.parseFamStatement() };
        }
        
        // Expression statement
        const expr = try self.parseExpression();
        return Statement{ .Expression = expr };
    }

    fn parseFunctionStatement(self: *Parser) ParserError!FunctionStatement {
        try self.consume(.Slay, "Expected 'slay'");
        
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        var func = FunctionStatement.init(self.allocator, name);
        
        // Parse parameters
        try self.consume(.LeftParen, "Expected '('");
        
        if (!self.check(.RightParen)) {
            while (true) {
                const param = try self.parseParameter();
                try func.parameters.append(param);
                
                if (!self.match(.Comma)) break;
            }
        }
        
        try self.consume(.RightParen, "Expected ')'");
        
        // Parse return type
        if (self.match(.Arrow)) {
            func.return_type = try self.parseType();
        }
        
        // Parse body
        try self.consume(.LeftBrace, "Expected '{'");
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
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
        
        // Parse type annotation
        if (self.match(.Colon)) {
            let_stmt.var_type = try self.parseType();
        }
        
        // Parse initializer
        if (self.match(.Equal) or self.match(.ColonEqual)) {
            let_stmt.initializer = try self.parseExpression();
        }
        
        return let_stmt;
    }

    fn parseReturnStatement(self: *Parser) ParserError!Statement {
        try self.consume(.Yolo, "Expected 'yolo'");
        
        var return_stmt = ast.ReturnStatement{ .value = null };
        
        // Parse optional return value
        if (!self.check(.Semicolon) and !self.check(.Newline) and !self.isAtEnd()) {
            return_stmt.value = try self.parseExpression();
        }
        
        return Statement{ .Return = return_stmt };
    }

    fn parseIfStatement(self: *Parser) ParserError!ast.IfStatement {
        try self.consume(.Lowkey, "Expected 'lowkey'");
        
        const condition = try self.parseExpression();
        
        try self.consume(.LeftBrace, "Expected '{'");
        
        var then_branch = ArrayList(Statement).init(self.allocator);
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            const stmt = try self.parseStatement();
            try then_branch.append(stmt);
        }
        
        try self.consume(.RightBrace, "Expected '}'");
        
        var else_branch: ?ArrayList(Statement) = null;
        
        // Parse else clause (highkey)
        if (self.match(.Highkey)) {
            var else_stmts = ArrayList(Statement).init(self.allocator);
            
            if (self.check(.Lowkey)) {
                // else if
                const elif_stmt = try self.parseIfStatement();
                try else_stmts.append(Statement{ .If = elif_stmt });
            } else {
                // else block
                try self.consume(.LeftBrace, "Expected '{'");
                
                while (!self.check(.RightBrace) and !self.isAtEnd()) {
                    const stmt = try self.parseStatement();
                    try else_stmts.append(stmt);
                }
                
                try self.consume(.RightBrace, "Expected '}'");
            }
            
            else_branch = else_stmts;
        }
        
        return ast.IfStatement{
            .condition = condition,
            .then_branch = then_branch,
            .else_branch = else_branch,
        };
    }

    fn parseWhileStatement(self: *Parser) ParserError!ast.WhileStatement {
        _ = self.advance(); // consume periodt/flex
        
        const condition = try self.parseExpression();
        
        try self.consume(.LeftBrace, "Expected '{'");
        
        var body = ArrayList(Statement).init(self.allocator);
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            const stmt = try self.parseStatement();
            try body.append(stmt);
        }
        
        try self.consume(.RightBrace, "Expected '}'");
        
        return ast.WhileStatement{
            .condition = condition,
            .body = body,
        };
    }

    fn parseForStatement(self: *Parser) ParserError!Statement {
        try self.consume(.Bestie, "Expected 'bestie'");
        
        // Check if this is a range-for loop (bestie var := flex ...)
        if (self.check(.Identifier)) {
            // Look ahead to check for range-for pattern
            if (self.isRangeForLoop()) {
                return try self.parseRangeForStatement();
            }
        }
        
        // Check if it's a while-style for loop (no semicolons)
        if (!self.hasSemicolonsBeforeBrace()) {
            // While-style for loop: bestie condition { ... }
            var condition: ?Expression = null;
            
            if (!self.check(.LeftBrace)) {
                condition = try self.parseExpression();
            }
            
            try self.consume(.LeftBrace, "Expected '{'");
            
            var body = ArrayList(Statement).init(self.allocator);
            while (!self.check(.RightBrace) and !self.isAtEnd()) {
                const stmt = try self.parseStatement();
                try body.append(stmt);
            }
            
            try self.consume(.RightBrace, "Expected '}'");
            
            return Statement{ .For = ast.ForStatement{
                .init = null,
                .condition = condition,
                .update = null,
                .body = body,
            }};
        }
        
        // C-style for loop: bestie init; condition; update { ... }
        
        // Parse init statement (optional)
        var init_stmt: ?Statement = null;
        if (!self.check(.Semicolon)) {
            init_stmt = try self.parseStatement();
        }
        try self.consume(.Semicolon, "Expected ';' after for loop init");
        
        // Parse condition (optional)
        var condition: ?Expression = null;
        if (!self.check(.Semicolon)) {
            condition = try self.parseExpression();
        }
        try self.consume(.Semicolon, "Expected ';' after for loop condition");
        
        // Parse update statement (optional)
        var update: ?Statement = null;
        if (!self.check(.LeftBrace)) {
            update = try self.parseStatement();
        }
        
        // Parse body
        try self.consume(.LeftBrace, "Expected '{'");
        
        var body = ArrayList(Statement).init(self.allocator);
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            const stmt = try self.parseStatement();
            try body.append(stmt);
        }
        
        try self.consume(.RightBrace, "Expected '}'");
        
        return Statement{ .For = ast.ForStatement{
            .init = init_stmt,
            .condition = condition,
            .update = update,
            .body = body,
        }};
    }

    fn parseDeferStatement(self: *Parser) ParserError!Statement {
        try self.consume(.Later, "Expected 'later'");
        
        const stmt_ptr = try self.allocator.create(Statement);
        stmt_ptr.* = try self.parseStatement();
        
        return Statement{ .Defer = ast.DeferStatement{ .statement = stmt_ptr } };
    }

    fn parseStructStatement(self: *Parser) ParserError!Statement {
        _ = self.advance(); // consume squad/struct
        
        // Parse struct name
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // Parse generic type parameters <T, U>
        var type_parameters = ArrayList(ast.TypeParameter).init(self.allocator);
        if (self.match(.Less)) {
            // Parse first type parameter
            if (!self.check(.Greater)) {
                if (self.check(.Identifier)) {
                    const param_name = self.advance().lexeme;
                    const param = ast.TypeParameter{
                        .name = param_name,
                        .constraints = ArrayList(Type).init(self.allocator),
                    };
                    try type_parameters.append(param);
                }
                
                // Parse additional type parameters
                while (self.match(.Comma)) {
                    if (self.check(.Identifier)) {
                        const param_name = self.advance().lexeme;
                        const param = ast.TypeParameter{
                            .name = param_name,
                            .constraints = ArrayList(Type).init(self.allocator),
                        };
                        try type_parameters.append(param);
                    }
                }
            }
            
            if (!self.match(.Greater)) {
                return ParserError.UnexpectedToken;
            }
        }
        
        // Expect '{'
        try self.consume(.LeftBrace, "Expected '{' after struct name");
        
        // Parse fields
        var fields = ArrayList(ast.StructField).init(self.allocator);
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            // Skip newlines
            if (self.match(.Newline)) {
                continue;
            }
            
            // Parse field
            const field = try self.parseStructField();
            try fields.append(field);
            
            // Optional comma
            _ = self.match(.Comma);
        }
        
        try self.consume(.RightBrace, "Expected '}' after struct fields");
        
        return Statement{ .Struct = ast.StructStatement{
            .name = name,
            .fields = fields,
            .visibility = .Public,
            .type_parameters = type_parameters,
        }};
    }

    fn parseInterfaceStatement(self: *Parser) ParserError!Statement {
        try self.consume(.Collab, "Expected 'collab'");
        
        // Parse interface name
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // Parse generic type parameters <T, U>
        var type_parameters = ArrayList(ast.TypeParameter).init(self.allocator);
        if (self.match(.Less)) {
            // Parse first type parameter
            if (!self.check(.Greater)) {
                if (self.check(.Identifier)) {
                    const param_name = self.advance().lexeme;
                    const param = ast.TypeParameter{
                        .name = param_name,
                        .constraints = ArrayList(Type).init(self.allocator),
                    };
                    try type_parameters.append(param);
                }
                
                // Parse additional type parameters
                while (self.match(.Comma)) {
                    if (self.check(.Identifier)) {
                        const param_name = self.advance().lexeme;
                        const param = ast.TypeParameter{
                            .name = param_name,
                            .constraints = ArrayList(Type).init(self.allocator),
                        };
                        try type_parameters.append(param);
                    }
                }
            }
            
            if (!self.match(.Greater)) {
                return ParserError.UnexpectedToken;
            }
        }
        
        // Expect opening brace
        try self.consume(.LeftBrace, "Expected '{' after interface name");
        
        // Parse method signatures
        var methods = ArrayList(ast.MethodSignature).init(self.allocator);
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            // Skip newlines
            if (self.match(.Newline)) {
                continue;
            }
            
            // Parse method signature
            const method = try self.parseMethodSignature();
            try methods.append(method);
        }
        
        try self.consume(.RightBrace, "Expected '}' after interface methods");
        
        return Statement{ .Interface = ast.InterfaceStatement{
            .name = name,
            .methods = methods,
            .visibility = .Public,
            .type_parameters = type_parameters,
        }};
    }

    fn parseExpression(self: *Parser) ParserError!Expression {
        return self.parseAssignment();
    }

    fn parseAssignment(self: *Parser) ParserError!Expression {
        const expr = try self.parseOr();
        
        if (self.match(.Equal) or self.match(.PlusEqual) or 
           self.match(.MinusEqual) or self.match(.StarEqual) or
           self.match(.SlashEqual) or self.match(.PercentEqual)) {
            
            const operator = self.previous().lexeme;
            const value = try self.parseAssignment();
            
            // For now, return the value (assignment statements handled elsewhere)
            _ = operator;
            return value;
        }
        
        return expr;
    }

    fn parseOr(self: *Parser) ParserError!Expression {
        var expr = try self.parseAnd();
        
        while (self.match(.PipePipe)) {
            const operator = self.previous().lexeme;
            const right = try self.parseAnd();
            
            const left_ptr = try self.allocator.create(Expression);
            const right_ptr = try self.allocator.create(Expression);
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

    fn parseAnd(self: *Parser) ParserError!Expression {
        var expr = try self.parseEquality();
        
        while (self.match(.AmpAmp)) {
            const operator = self.previous().lexeme;
            const right = try self.parseEquality();
            
            const left_ptr = try self.allocator.create(Expression);
            const right_ptr = try self.allocator.create(Expression);
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

    fn parseEquality(self: *Parser) ParserError!Expression {
        var expr = try self.parseComparison();
        
        while (self.match(.BangEqual) or self.match(.EqualEqual)) {
            const operator = self.previous().lexeme;
            const right = try self.parseComparison();
            
            const left_ptr = try self.allocator.create(Expression);
            const right_ptr = try self.allocator.create(Expression);
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

    fn parseComparison(self: *Parser) ParserError!Expression {
        var expr = try self.parseTerm();
        
        while (self.match(.Greater) or self.match(.GreaterEqual) or 
              self.match(.Less) or self.match(.LessEqual)) {
            const operator = self.previous().lexeme;
            const right = try self.parseTerm();
            
            const left_ptr = try self.allocator.create(Expression);
            const right_ptr = try self.allocator.create(Expression);
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

    fn parseTerm(self: *Parser) ParserError!Expression {
        var expr = try self.parseFactor();
        
        while (self.match(.Minus) or self.match(.Plus)) {
            const operator = self.previous().lexeme;
            const right = try self.parseFactor();
            
            const left_ptr = try self.allocator.create(Expression);
            const right_ptr = try self.allocator.create(Expression);
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

    fn parseFactor(self: *Parser) ParserError!Expression {
        var expr = try self.parseUnary();
        
        while (self.match(.Slash) or self.match(.Star) or self.match(.Percent)) {
            const operator = self.previous().lexeme;
            const right = try self.parseUnary();
            
            const left_ptr = try self.allocator.create(Expression);
            const right_ptr = try self.allocator.create(Expression);
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

    fn parseUnary(self: *Parser) ParserError!Expression {
        if (self.match(.Bang) or self.match(.Minus)) {
            const operator = self.previous().lexeme;
            const right = try self.parseUnary();
            
            const right_ptr = try self.allocator.create(Expression);
            right_ptr.* = right;
            
            return Expression{ .Unary = ast.UnaryExpression{
                .operator = operator,
                .operand = right_ptr,
            }};
        }
        
        // Handle shook error propagation operator
        if (self.match(.Shook)) {
            const wrapped_expr = try self.allocator.create(Expression);
            wrapped_expr.* = try self.parseUnary();
            
            return Expression{ .Shook = ast.ShookExpression{
                .expression = wrapped_expr,
            }};
        }
        
        return self.parseCall();
    }

    fn parseCall(self: *Parser) ParserError!Expression {
        var expr = try self.parsePrimary();
        
        while (true) {
            if (self.match(.LeftParen)) {
                expr = try self.finishCall(expr);
            } else if (self.match(.Dot)) {
                if (!self.check(.Identifier)) {
                    return ParserError.UnexpectedToken;
                }
                const property = self.advance().lexeme;
                
                const object_ptr = try self.allocator.create(Expression);
                object_ptr.* = expr;
                
                expr = Expression{ .MemberAccess = ast.MemberAccessExpression{
                    .object = object_ptr,
                    .property = property,
                }};
            } else {
                break;
            }
        }
        
        return expr;
    }

    fn finishCall(self: *Parser, callee: Expression) ParserError!Expression {
        var arguments = ArrayList(Expression).init(self.allocator);
        
        if (!self.check(.RightParen)) {
            while (true) {
                const arg = try self.parseExpression();
                try arguments.append(arg);
                
                if (!self.match(.Comma)) break;
            }
        }
        
        try self.consume(.RightParen, "Expected ')' after arguments");
        
        const callee_ptr = try self.allocator.create(Expression);
        callee_ptr.* = callee;
        
        return Expression{ .Call = ast.CallExpression{
            .function = callee_ptr,
            .arguments = arguments,
        }};
    }

    fn parsePrimary(self: *Parser) ParserError!Expression {
        // Based (true)
        if (self.match(.Based) or self.match(.Truth)) {
            return Expression{ .Boolean = true };
        }
        
        // Lies/cringe (false)
        if (self.match(.Lies) or self.match(.Cap)) {
            return Expression{ .Boolean = false };
        }
        
        // Numbers
        if (self.check(.Number)) {
            const token = self.advance();
            if (std.mem.indexOf(u8, token.lexeme, ".")) |_| {
                // Float
                const value = std.fmt.parseFloat(f64, token.lexeme) catch {
                    return ParserError.InvalidSyntax;
                };
                return Expression{ .Float = value };
            } else {
                // Integer
                const value = std.fmt.parseInt(i64, token.lexeme, 10) catch {
                    return ParserError.InvalidSyntax;
                };
                return Expression{ .Integer = value };
            }
        }
        
        // Strings
        if (self.check(.StringLiteral)) {
            const token = self.advance();
            const str_content = token.lexeme[1..token.lexeme.len-1]; // Remove quotes
            return Expression{ .String = str_content };
        }
        
        // Characters
        if (self.check(.Character)) {
            const token = self.advance();
            const char_content = token.lexeme[1..token.lexeme.len-1]; // Remove quotes
            if (char_content.len == 1) {
                return Expression{ .Character = char_content[0] };
            }
            return ParserError.InvalidSyntax;
        }
        
        // Identifiers
        if (self.check(.Identifier)) {
            const name = self.advance().lexeme;
            return Expression{ .Identifier = name };
        }
        
        // Parenthesized expressions
        if (self.match(.LeftParen)) {
            const expr = try self.parseExpression();
            try self.consume(.RightParen, "Expected ')' after expression");
            return expr;
        }
        
        return ParserError.UnexpectedToken;
    }

    fn parseParameter(self: *Parser) ParserError!Parameter {
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        try self.consume(.Colon, "Expected ':' after parameter name");
        
        const param_type = try self.parseType();
        
        return Parameter{
            .name = name,
            .param_type = param_type,
            .is_mutable = false,
            .default_value = null,
        };
    }

    fn parseType(self: *Parser) ParserError!Type {
        // Basic types
        if (self.match(.Normie)) return Type{ .Basic = .Normie };
        if (self.match(.Tea)) return Type{ .Basic = .Tea };
        if (self.match(.Txt)) return Type{ .Basic = .Txt };
        if (self.match(.Sip)) return Type{ .Basic = .Sip };
        if (self.match(.Smol)) return Type{ .Basic = .Smol };
        if (self.match(.Mid)) return Type{ .Basic = .Mid };
        if (self.match(.Thicc)) return Type{ .Basic = .Thicc };
        if (self.match(.Snack)) return Type{ .Basic = .Snack };
        if (self.match(.Meal)) return Type{ .Basic = .Meal };
        if (self.match(.Byte)) return Type{ .Basic = .Byte };
        if (self.match(.Rune)) return Type{ .Basic = .Rune };
        if (self.match(.Extra)) return Type{ .Basic = .Extra };
        if (self.match(.Lit)) return Type{ .Basic = .Lit };
        if (self.match(.Cap)) return Type{ .Basic = .Cap };
        
        return ParserError.UnexpectedToken;
    }

    // Utility methods
    fn match(self: *Parser, kind: TokenKind) bool {
        if (self.check(kind)) {
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
        return self.peek().kind == .Eof;
    }

    fn peek(self: *Parser) Token {
        if (self.current >= self.tokens.len) {
            return Token.init(.Eof, "", 0, 0);
        }
        return self.tokens[self.current];
    }

    fn previous(self: *Parser) Token {
        if (self.current == 0) return self.tokens[0];
        return self.tokens[self.current - 1];
    }

    fn consume(self: *Parser, kind: TokenKind, message: []const u8) ParserError!Token {
        if (self.check(kind)) return self.advance();
        
        std.debug.print("Parser error: {s}\n", .{message});
        return ParserError.UnexpectedToken;
    }

    // Helper functions for parsing
    fn parseStructField(self: *Parser) ParserError!ast.StructField {
        // Parse field name
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // Parse field type
        const field_type = try self.parseType();
        
        return ast.StructField{
            .name = name,
            .field_type = field_type,
            .visibility = .Public,
        };
    }

    fn parseMethodSignature(self: *Parser) ParserError!ast.MethodSignature {
        // Expect 'slay' keyword
        try self.consume(.Slay, "Expected 'slay' keyword for method");
        
        // Parse method name
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // Parse parameters
        const parameters = try self.parseParameters();
        
        // Parse return type (optional)
        var return_type: ?Type = null;
        if (self.check(.Normie) or self.check(.Tea) or self.check(.Lit) or
            self.check(.Sip) or self.check(.Smol) or self.check(.Mid) or
            self.check(.Thicc) or self.check(.Snack) or self.check(.Meal) or
            self.check(.Byte) or self.check(.Rune) or self.check(.Extra) or
            self.check(.Identifier)) {
            return_type = try self.parseType();
        }
        
        return ast.MethodSignature{
            .name = name,
            .parameters = parameters,
            .return_type = return_type,
        };
    }

    fn isRangeForLoop(self: *Parser) bool {
        // Simple heuristic: look for := or = followed by flex
        var pos = self.current + 1;
        
        // Skip comma and identifier if present (for multi-variable assignment)
        if (pos < self.tokens.len and self.tokens[pos].kind == .Comma) {
            pos += 1;
            if (pos < self.tokens.len and self.tokens[pos].kind == .Identifier) {
                pos += 1;
            }
        }
        
        // Look for := or =
        if (pos < self.tokens.len and 
            (self.tokens[pos].kind == .ColonEqual or self.tokens[pos].kind == .Equal)) {
            pos += 1;
            // Look for flex
            if (pos < self.tokens.len and self.tokens[pos].kind == .Flex) {
                return true;
            }
        }
        
        return false;
    }

    fn hasSemicolonsBeforeBrace(self: *Parser) bool {
        var pos = self.current;
        var semicolon_count: usize = 0;
        
        while (pos < self.tokens.len) {
            const token = self.tokens[pos];
            if (token.kind == .LeftBrace) {
                break;
            }
            if (token.kind == .Semicolon) {
                semicolon_count += 1;
            }
            if (token.kind == .Eof) {
                break;
            }
            pos += 1;
        }
        
        return semicolon_count > 0;
    }

    fn parseRangeForStatement(self: *Parser) ParserError!Statement {
        // Parse variable(s) for range-for loop
        var variables = ArrayList([]const u8).init(self.allocator);
        
        // Parse first variable
        if (self.check(.Identifier)) {
            try variables.append(self.advance().lexeme);
        }
        
        // Parse second variable if comma present
        if (self.match(.Comma)) {
            if (self.check(.Identifier)) {
                try variables.append(self.advance().lexeme);
            }
        }
        
        // Expect := or =
        if (!self.match(.ColonEqual) and !self.match(.Equal)) {
            return ParserError.UnexpectedToken;
        }
        
        // Expect 'flex'
        try self.consume(.Flex, "Expected 'flex' in range-for loop");
        
        // Parse iterable expression
        const iterable = try self.parseExpression();
        
        // Parse body
        try self.consume(.LeftBrace, "Expected '{'");
        
        var body = ArrayList(Statement).init(self.allocator);
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            const stmt = try self.parseStatement();
            try body.append(stmt);
        }
        
        try self.consume(.RightBrace, "Expected '}'");
        
        // For now, convert to a simplified ForStatement
        // In a full implementation, you'd want a dedicated ForInStatement
        return Statement{ .For = ast.ForStatement{
            .init = null,
            .condition = iterable,
            .update = null,
            .body = body,
        }};
    }

    // CURSED Error Handling System Implementation
    fn parseYikesStatement(self: *Parser) ParserError!ast.YikesStatement {
        try self.consume(.Yikes, "Expected 'yikes'");
        
        // Parse error name/identifier
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        var error_type: ?Type = null;
        var value: ?Expression = null;
        
        // Optional error type annotation (yikes MyError tea)
        if (self.check(.Identifier)) {
            error_type = try self.parseType();
        }
        
        // Optional error value assignment (yikes MyError = "something went wrong")
        if (self.match(.Equal)) {
            value = try self.parseExpression();
        }
        
        return ast.YikesStatement{
            .name = name,
            .error_type = error_type,
            .value = value,
        };
    }

    fn parseFamStatement(self: *Parser) ParserError!ast.FamStatement {
        try self.consume(.Fam, "Expected 'fam'");
        
        // Parse main body block
        try self.consume(.LeftBrace, "Expected '{'");
        
        var body = ArrayList(Statement).init(self.allocator);
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            const stmt = try self.parseStatement();
            try body.append(stmt);
        }
        
        try self.consume(.RightBrace, "Expected '}'");
        
        var recovery_body: ?ArrayList(Statement) = null;
        var error_variable: ?[]const u8 = null;
        
        // Optional recovery block (fam { ... } catch(err) { ... })
        if (self.match(.Catch)) {
            // Parse error variable in catch block
            if (self.match(.LeftParen)) {
                if (self.check(.Identifier)) {
                    error_variable = self.advance().lexeme;
                }
                try self.consume(.RightParen, "Expected ')'");
            }
            
            // Parse recovery body
            try self.consume(.LeftBrace, "Expected '{'");
            
            var recovery = ArrayList(Statement).init(self.allocator);
            while (!self.check(.RightBrace) and !self.isAtEnd()) {
                const stmt = try self.parseStatement();
                try recovery.append(stmt);
            }
            
            try self.consume(.RightBrace, "Expected '}'");
            recovery_body = recovery;
        }
        
        return ast.FamStatement{
            .body = body,
            .recovery_body = recovery_body,
            .error_variable = error_variable,
        };
    }

    fn parseShookExpression(self: *Parser) ParserError!Expression {
        try self.consume(.Shook, "Expected 'shook'");
        
        // Parse the wrapped expression that might fail
        const wrapped_expr = try self.allocator.create(Expression);
        wrapped_expr.* = try self.parseUnaryExpression();
        
        return Expression{ .Shook = ast.ShookExpression{
            .expression = wrapped_expr,
        }};
    }
};

test "parser basic program" {
    const allocator = std.testing.allocator;
    
    // Create tokens for "slay main_character() { }"
    const tokens = [_]Token{
        Token.init(.Slay, "slay", 1, 1),
        Token.init(.MainCharacter, "main_character", 1, 6),
        Token.init(.LeftParen, "(", 1, 20),
        Token.init(.RightParen, ")", 1, 21),
        Token.init(.LeftBrace, "{", 1, 23),
        Token.init(.RightBrace, "}", 1, 25),
        Token.init(.Eof, "", 1, 26),
    };
    
    var parser = Parser.init(allocator, &tokens);
    const program = try parser.parseProgram();
    defer program.deinit(allocator);
    
    try std.testing.expect(program.statements.items.len == 1);
    
    switch (program.statements.items[0]) {
        .Function => |func| {
            try std.testing.expect(std.mem.eql(u8, func.name, "main_character"));
        },
        else => try std.testing.expect(false),
    }
}

test "parser expressions" {
    const allocator = std.testing.allocator;
    
    // Create tokens for "42 + 24"
    const tokens = [_]Token{
        Token.init(.Number, "42", 1, 1),
        Token.init(.Plus, "+", 1, 4),
        Token.init(.Number, "24", 1, 6),
        Token.init(.Eof, "", 1, 8),
    };
    
    var parser = Parser.init(allocator, &tokens);
    const program = try parser.parseProgram();
    defer program.deinit(allocator);
    
    try std.testing.expect(program.statements.items.len == 1);
    
    switch (program.statements.items[0]) {
        .Expression => |expr| {
            switch (expr) {
                .Binary => |bin| {
                    try std.testing.expect(std.mem.eql(u8, bin.operator, "+"));
                },
                else => try std.testing.expect(false),
            }
        },
        else => try std.testing.expect(false),
    }
}
