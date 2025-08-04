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

pub const Parser = struct {
    tokens: []const Token,
    current: usize,
    allocator: Allocator,
    had_error: bool,
    in_function: bool,
    in_loop: bool,
    scope_depth: usize,

    pub fn init(allocator: Allocator, tokens: []const Token) Parser {
        return Parser{
            .tokens = tokens,
            .current = 0,
            .allocator = allocator,
            .had_error = false,
            .in_function = false,
            .in_loop = false,
            .scope_depth = 0,
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
        
        if (!self.check(.StringLiteral) and !self.check(.String)) {
            return ParserError.UnexpectedToken;
        }
        
        const path_token = self.advance();
        const path = if (path_token.lexeme.len >= 2 and 
                        path_token.lexeme[0] == '"' and 
                        path_token.lexeme[path_token.lexeme.len-1] == '"')
                     path_token.lexeme[1..path_token.lexeme.len-1] // Remove quotes
                     else path_token.lexeme;
        
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
        try self.consume(.Slay, "Expected 'slay'");
        
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        var func = FunctionStatement.init(self.allocator, name);
        
        // Parse generic type parameters <T, U>
        if (self.match(.Less) or self.match(.LeftAngle)) {
            while (!self.check(.Greater) and !self.check(.RightAngle) and !self.isAtEnd()) {
                if (self.check(.Identifier)) {
                    const param_name = self.advance().lexeme;
                    var type_param = ast.TypeParameter{
                        .name = param_name,
                        .constraints = ArrayList(ast.Type).init(self.allocator),
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
            
            const element_type_ptr = try self.allocator.create(ast.Type);
            element_type_ptr.* = try self.parseType();
            
            return ast.Type{ .Array = ast.ArrayType{
                .element_type = element_type_ptr,
                .size = size,
            }};
        }

        // Map types map[key_type]value_type
        if (self.matchIdentifier("map")) {
            try self.consume(.LeftBracket, "Expected '[' after 'map'");
            
            const key_type_ptr = try self.allocator.create(ast.Type);
            key_type_ptr.* = try self.parseType();
            
            try self.consume(.RightBracket, "Expected ']'");
            
            const value_type_ptr = try self.allocator.create(ast.Type);
            value_type_ptr.* = try self.parseType();
            
            return ast.Type{ .Map = ast.MapType{
                .key_type = key_type_ptr,
                .value_type = value_type_ptr,
            }};
        }

        // Channel types dm<element_type>
        if (self.check(.Dm) or self.matchIdentifier("dm")) {
            _ = self.advance();
            if (self.match(.Less) or self.match(.LeftAngle)) {
                const element_type_ptr = try self.allocator.create(ast.Type);
                element_type_ptr.* = try self.parseType();
                
                try self.consume(.Greater, "Expected '>' after channel element type");
                
                return ast.Type{ .Channel = ast.ChannelType{
                    .element_type = element_type_ptr,
                    .is_send_only = false,
                    .is_receive_only = false,
                }};
            }
        }

        // Function types (param_types) -> return_type
        if (self.check(.LeftParen)) {
            // Look ahead to see if this is a function type
            if (self.isFunctionType()) {
                _ = self.advance(); // consume '('
                
                var param_types = ArrayList(ast.Type).init(self.allocator);
                
                if (!self.check(.RightParen)) {
                    while (true) {
                        const param_type = try self.parseType();
                        try param_types.append(param_type);
                        
                        if (!self.match(.Comma)) break;
                    }
                }
                
                try self.consume(.RightParen, "Expected ')'");
                
                var return_type: ?*ast.Type = null;
                if (self.match(.Arrow)) {
                    return_type = try self.allocator.create(ast.Type);
                    return_type.?.* = try self.parseType();
                }
                
                return ast.Type{ .Function = ast.FunctionType{
                    .parameters = param_types,
                    .return_type = return_type,
                }};
            }
        }

        // Tuple types (type1, type2, ...)
        if (self.check(.LeftParen)) {
            _ = self.advance();
            
            var elements = ArrayList(ast.Type).init(self.allocator);
            
            if (!self.check(.RightParen)) {
                while (true) {
                    const elem_type = try self.parseType();
                    try elements.append(elem_type);
                    
                    if (!self.match(.Comma)) break;
                }
            }
            
            try self.consume(.RightParen, "Expected ')'");
            
            // Single element in parens is just grouped, not a tuple
            if (elements.items.len == 1) {
                const single_type = elements.items[0];
                elements.deinit();
                return single_type;
            }
            
            return ast.Type{ .Tuple = ast.TupleType{ .elements = elements }};
        }

        // Generic types with parameters Name<T, U>
        if (self.check(.Identifier)) {
            const type_name = self.advance().lexeme;
            
            if (self.match(.Less) or self.match(.LeftAngle)) {
                // Generic type instantiation
                var type_args = ArrayList(ast.Type).init(self.allocator);
                
                while (!self.check(.Greater) and !self.check(.RightAngle) and !self.isAtEnd()) {
                    const arg_type = try self.parseType();
                    try type_args.append(arg_type);
                    
                    if (!self.match(.Comma)) break;
                }
                
                if (!self.match(.Greater) and !self.match(.RightAngle)) {
                    return ParserError.MissingToken;
                }
                
                return ast.Type{ .Generic = ast.GenericType{
                    .name = type_name,
                    .type_arguments = type_args,
                    .constraints = ArrayList(ast.TypeConstraint).init(self.allocator),
                }};
            }
            
            // Check if it's a basic type name like normie, tea, etc.
            if (std.mem.eql(u8, type_name, "normie")) return ast.Type{ .Basic = .Normie };
            if (std.mem.eql(u8, type_name, "tea")) return ast.Type{ .Basic = .Tea };
            if (std.mem.eql(u8, type_name, "txt")) return ast.Type{ .Basic = .Txt };
            if (std.mem.eql(u8, type_name, "sip")) return ast.Type{ .Basic = .Sip };
            if (std.mem.eql(u8, type_name, "smol")) return ast.Type{ .Basic = .Smol };
            if (std.mem.eql(u8, type_name, "mid")) return ast.Type{ .Basic = .Mid };
            if (std.mem.eql(u8, type_name, "thicc")) return ast.Type{ .Basic = .Thicc };
            if (std.mem.eql(u8, type_name, "snack")) return ast.Type{ .Basic = .Snack };
            if (std.mem.eql(u8, type_name, "meal")) return ast.Type{ .Basic = .Meal };
            if (std.mem.eql(u8, type_name, "byte")) return ast.Type{ .Basic = .Byte };
            if (std.mem.eql(u8, type_name, "rune")) return ast.Type{ .Basic = .Rune };
            if (std.mem.eql(u8, type_name, "extra")) return ast.Type{ .Basic = .Extra };
            if (std.mem.eql(u8, type_name, "lit")) return ast.Type{ .Basic = .Lit };
            if (std.mem.eql(u8, type_name, "cap")) return ast.Type{ .Basic = .Cap };
            
            // Custom struct or interface type
            return ast.Type{ .Struct = ast.StructType{
                .name = type_name,
                .fields = ArrayList(ast.StructField).init(self.allocator),
            }};
        }

        // Basic types using keywords
        if (self.match(.Normie)) return ast.Type{ .Basic = .Normie };
        if (self.match(.Tea)) return ast.Type{ .Basic = .Tea };
        if (self.match(.Txt)) return ast.Type{ .Basic = .Txt };
        if (self.match(.Sip)) return ast.Type{ .Basic = .Sip };
        if (self.match(.Smol)) return ast.Type{ .Basic = .Smol };
        if (self.match(.Mid)) return ast.Type{ .Basic = .Mid };
        if (self.match(.Thicc)) return ast.Type{ .Basic = .Thicc };
        if (self.match(.Snack)) return ast.Type{ .Basic = .Snack };
        if (self.match(.Meal)) return ast.Type{ .Basic = .Meal };
        if (self.match(.Byte)) return ast.Type{ .Basic = .Byte };
        if (self.match(.Rune)) return ast.Type{ .Basic = .Rune };
        if (self.match(.Extra)) return ast.Type{ .Basic = .Extra };
        if (self.match(.Lit)) return ast.Type{ .Basic = .Lit };
        if (self.match(.Cap)) return ast.Type{ .Basic = .Cap };
        
        return ParserError.InvalidType;
    }

    fn parseExpression(self: *Parser) ParserError!Expression {
        return self.parseAssignment();
    }

    fn parseAssignment(self: *Parser) ParserError!Expression {
        const expr = try self.parseOr();

        if (self.match(.Equal) or self.match(.PlusEqual) or self.match(.MinusEqual) or 
           self.match(.StarEqual) or self.match(.SlashEqual) or self.match(.PercentEqual)) {
            const operator = self.previous().lexeme;
            const value = try self.parseAssignment();
            
            // Convert to assignment expression (not statement)
            return Expression{ .Binary = ast.BinaryExpression{
                .left = try self.allocateExpression(expr),
                .operator = operator,
                .right = try self.allocateExpression(value),
            }};
        }

        return expr;
    }

    fn parseOr(self: *Parser) ParserError!Expression {
        var expr = try self.parseAnd();

        while (self.match(.PipePipe) or self.match(.Pipe)) {
            const operator = self.previous().lexeme;
            const right = try self.parseAnd();
            
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = try self.allocateExpression(expr),
                .operator = operator,
                .right = try self.allocateExpression(right),
            }};
        }

        return expr;
    }

    fn parseAnd(self: *Parser) ParserError!Expression {
        var expr = try self.parseEquality();

        while (self.match(.AmpAmp) or self.match(.Amp)) {
            const operator = self.previous().lexeme;
            const right = try self.parseEquality();
            
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = try self.allocateExpression(expr),
                .operator = operator,
                .right = try self.allocateExpression(right),
            }};
        }

        return expr;
    }

    fn parseEquality(self: *Parser) ParserError!Expression {
        var expr = try self.parseComparison();

        while (self.match(.BangEqual) or self.match(.EqualEqual)) {
            const operator = self.previous().lexeme;
            const right = try self.parseComparison();
            
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = try self.allocateExpression(expr),
                .operator = operator,
                .right = try self.allocateExpression(right),
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
            
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = try self.allocateExpression(expr),
                .operator = operator,
                .right = try self.allocateExpression(right),
            }};
        }

        return expr;
    }

    fn parseTerm(self: *Parser) ParserError!Expression {
        var expr = try self.parseFactor();

        while (self.match(.Minus) or self.match(.Plus)) {
            const operator = self.previous().lexeme;
            const right = try self.parseFactor();
            
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = try self.allocateExpression(expr),
                .operator = operator,
                .right = try self.allocateExpression(right),
            }};
        }

        return expr;
    }

    fn parseFactor(self: *Parser) ParserError!Expression {
        var expr = try self.parseUnary();

        while (self.match(.Slash) or self.match(.Star) or self.match(.Percent)) {
            const operator = self.previous().lexeme;
            const right = try self.parseUnary();
            
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = try self.allocateExpression(expr),
                .operator = operator,
                .right = try self.allocateExpression(right),
            }};
        }

        return expr;
    }

    fn parseUnary(self: *Parser) ParserError!Expression {
        if (self.match(.Bang) or self.match(.Minus) or self.match(.Plus)) {
            const operator = self.previous().lexeme;
            const right = try self.parseUnary();
            
            return Expression{ .Unary = try self.allocateUnaryExpression(ast.UnaryExpression{
                .operator = operator,
                .operand = try self.allocateExpression(right),
            })};
        }

        // Handle shook error propagation operator
        if (self.match(.Shook)) {
            const wrapped_expr = try self.allocateExpression(try self.parseUnary());
            
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
                
                expr = Expression{ .MemberAccess = try self.allocateMemberAccess(ast.MemberAccessExpression{
                    .object = try self.allocateExpression(expr),
                    .property = property,
                })};
            } else if (self.match(.LeftBracket)) {
                // Array/slice access expr[index] or expr[start:end]
                const index = try self.parseExpression();
                
                if (self.match(.Colon)) {
                    // Slice access expr[start:end]
                    const end = if (!self.check(.RightBracket)) try self.parseExpression() else null;
                    try self.consume(.RightBracket, "Expected ']'");
                    
                    expr = Expression{ .SliceAccess = ast.SliceAccessExpression{
                        .array = try self.allocateExpression(expr),
                        .start = try self.allocateExpression(index),
                        .end = if (end) |e| try self.allocateExpression(e) else null,
                    }};
                } else {
                    // Array access expr[index]
                    try self.consume(.RightBracket, "Expected ']'");
                    
                    expr = Expression{ .ArrayAccess = ast.ArrayAccessExpression{
                        .array = try self.allocateExpression(expr),
                        .index = try self.allocateExpression(index),
                    }};
                }
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

        return Expression{ .Call = ast.CallExpression{
            .function = try self.allocateExpression(callee),
            .arguments = arguments,
        }};
    }

    fn parsePrimary(self: *Parser) ParserError!Expression {
        // Boolean literals
        if (self.match(.Based) or self.match(.Truth)) {
            return Expression{ .Boolean = true };
        }
        
        // SPEC CONFORMANCE: Only accept canonical boolean and nil literals
        if (self.match(.Cringe)) {
            return Expression{ .Boolean = false };
        }
        
        if (self.match(.Nah)) {
            return Expression{ .Literal = ast.Literal{ .Nil = {} } };
        }
        
        // SPEC CONFORMANCE: Reject deprecated forms
        if (self.match(.Lies) or self.match(.Cap) or self.match(.Truth)) {
            return ParserError.InvalidSyntax; // Deprecated: use canonical forms
        }
        
        // Numbers
        if (self.check(.Number) or self.check(.Integer)) {
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
        if (self.check(.StringLiteral) or self.check(.String)) {
            const token = self.advance();
            const str_content = if (token.lexeme.len >= 2 and 
                                   token.lexeme[0] == '"' and 
                                   token.lexeme[token.lexeme.len-1] == '"')
                                 token.lexeme[1..token.lexeme.len-1] // Remove quotes
                                 else token.lexeme;
            return Expression{ .String = str_content };
        }
        
        // Characters
        if (self.check(.Character)) {
            const token = self.advance();
            const char_content = if (token.lexeme.len >= 2 and 
                                    token.lexeme[0] == '\'' and 
                                    token.lexeme[token.lexeme.len-1] == '\'')
                                   token.lexeme[1..token.lexeme.len-1] // Remove quotes
                                   else token.lexeme;
            if (char_content.len == 1) {
                return Expression{ .Character = char_content[0] };
            }
            return ParserError.InvalidSyntax;
        }

        // Array literals [1, 2, 3]
        if (self.match(.LeftBracket)) {
            var elements = ArrayList(Expression).init(self.allocator);
            
            if (!self.check(.RightBracket)) {
                while (true) {
                    const elem = try self.parseExpression();
                    try elements.append(elem);
                    
                    if (!self.match(.Comma)) break;
                }
            }
            
            try self.consume(.RightBracket, "Expected ']'");
            
            return Expression{ .Array = try self.allocateArrayExpression(ast.ArrayExpression{
                .elements = try self.convertExpressionsToPointers(elements),
            })};
        }

        // Tuple literals (1, 2, 3)
        if (self.match(.LeftParen)) {
            // Look ahead to see if this is a tuple or just grouped expression
            if (self.check(.RightParen)) {
                // Empty tuple ()
                _ = self.advance();
                return Expression{ .Tuple = ast.TupleExpression{
                    .elements = ArrayList(*anyopaque).init(self.allocator),
                }};
            }
            
            var elements = ArrayList(Expression).init(self.allocator);
            var has_comma = false;
            
            while (true) {
                const elem = try self.parseExpression();
                try elements.append(elem);
                
                if (self.match(.Comma)) {
                    has_comma = true;
                    if (self.check(.RightParen)) break; // Trailing comma
                } else {
                    break;
                }
            }
            
            try self.consume(.RightParen, "Expected ')'");
            
            // Single element without comma is just grouped expression
            if (elements.items.len == 1 and !has_comma) {
                const single_expr = elements.items[0];
                elements.deinit();
                return single_expr;
            }
            
            // Multiple elements or single with comma is a tuple
            return Expression{ .Tuple = ast.TupleExpression{
                .elements = try self.convertExpressionsToPointers(elements),
            }};
        }

        // Map literals {key: value, ...}
        if (self.match(.LeftBrace)) {
            var entries = ArrayList(ast.MapEntry).init(self.allocator);
            
            if (!self.check(.RightBrace)) {
                while (true) {
                    const key = try self.parseExpression();
                    try self.consume(.Colon, "Expected ':' after map key");
                    const value = try self.parseExpression();
                    
                    try entries.append(ast.MapEntry{
                        .key = key,
                        .value = value,
                    });
                    
                    if (!self.match(.Comma)) break;
                }
            }
            
            try self.consume(.RightBrace, "Expected '}'");
            
            return Expression{ .Map = try self.allocateMapExpression(ast.MapExpression{
                .entries = entries,
            })};
        }

        // Match expression
        if (self.check(.Match)) {
            return try self.parseMatchExpression();
        }

        // Lambda expressions |params| -> body
        if (self.match(.Pipe)) {
            var params = ArrayList([]const u8).init(self.allocator);
            
            if (!self.check(.Pipe)) {
                while (true) {
                    if (!self.check(.Identifier)) {
                        return ParserError.UnexpectedToken;
                    }
                    
                    try params.append(self.advance().lexeme);
                    
                    if (!self.match(.Comma)) break;
                }
            }
            
            try self.consume(.Pipe, "Expected '|' after lambda parameters");
            try self.consume(.Arrow, "Expected '->' after lambda parameters");
            
            const body = try self.allocateExpression(try self.parseExpression());
            
            return Expression{ .Lambda = ast.LambdaExpression{
                .parameters = params,
                .body = body,
            }};
        }

        // Channel creation make_channel<T>()
        if (self.matchIdentifier("make_channel")) {
            if (self.match(.Less) or self.match(.LeftAngle)) {
                const element_type = try self.parseType();
                try self.consume(.Greater, "Expected '>' after channel element type");
                
                try self.consume(.LeftParen, "Expected '(' after make_channel<T>");
                
                var buffer_size: ?Expression = null;
                if (!self.check(.RightParen)) {
                    buffer_size = try self.parseExpression();
                }
                
                try self.consume(.RightParen, "Expected ')' after make_channel");
                
                return Expression{ .ChannelCreation = ast.ChannelCreationExpression{
                    .element_type = element_type,
                    .buffer_size = buffer_size,
                }};
            }
        }

        // Identifiers
        if (self.check(.Identifier)) {
            const name = self.advance().lexeme;
            
            // Check for struct literal Name{field: value, ...}
            if (self.check(.LeftBrace)) {
                return try self.parseStructLiteral(name);
            }
            
            return Expression{ .Identifier = name };
        }
        
        return ParserError.UnexpectedToken;
    }

    fn parseStructLiteral(self: *Parser, struct_name: []const u8) ParserError!Expression {
        try self.consume(.LeftBrace, "Expected '{'");
        
        var fields = ArrayList(ast.StructFieldAssignment).init(self.allocator);
        
        if (!self.check(.RightBrace)) {
            while (true) {
                if (!self.check(.Identifier)) {
                    return ParserError.UnexpectedToken;
                }
                
                const field_name = self.advance().lexeme;
                try self.consume(.Colon, "Expected ':' after field name");
                const value = try self.parseExpression();
                
                try fields.append(ast.StructFieldAssignment{
                    .field_name = field_name,
                    .value = value,
                });
                
                if (!self.match(.Comma)) break;
            }
        }
        
        try self.consume(.RightBrace, "Expected '}'");
        
        return Expression{ .StructLiteral = ast.StructLiteralExpression{
            .struct_name = struct_name,
            .fields = fields,
        }};
    }

    fn parseMatchExpression(self: *Parser) ParserError!Expression {
        try self.consume(.Match, "Expected 'match'");
        
        const value = try self.allocateExpression(try self.parseExpression());
        
        try self.consume(.LeftBrace, "Expected '{' after match value");
        
        var cases = ArrayList(ast.MatchCase).init(self.allocator);
        var default_case: ?*Expression = null;
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            // Check for default case
            if (self.match(.Basic) or self.matchIdentifier("_")) {
                try self.consume(.Arrow, "Expected '->' after default pattern");
                default_case = try self.allocateExpression(try self.parseExpression());
                _ = self.match(.Comma);
                continue;
            }
            
            // Parse pattern
            const pattern = try self.parsePattern();
            
            // Optional guard
            var guard: ?Expression = null;
            if (self.matchIdentifier("if")) {
                guard = try self.parseExpression();
            }
            
            try self.consume(.Arrow, "Expected '->' after pattern");
            const result = try self.parseExpression();
            
            try cases.append(ast.MatchCase{
                .pattern = pattern,
                .guard = guard,
                .result = result,
            });
            
            _ = self.match(.Comma);
        }
        
        try self.consume(.RightBrace, "Expected '}' after match cases");
        
        return Expression{ .Match = ast.MatchExpression{
            .expression = value,
            .cases = cases,
            .default_case = default_case,
        }};
    }

    fn parsePattern(self: *Parser) ParserError!ast.Pattern {
        // Wildcard pattern _
        if (self.matchIdentifier("_")) {
            return ast.Pattern.Wildcard;
        }

        // Literal patterns
        if (self.check(.Number) or self.check(.Integer)) {
            const token = self.advance();
            const value = std.fmt.parseInt(i64, token.lexeme, 10) catch {
                return ParserError.InvalidSyntax;
            };
            return ast.Pattern{ .Literal = ast.Literal{ .Integer = value }};
        }

        if (self.check(.StringLiteral) or self.check(.String)) {
            const token = self.advance();
            const str_content = if (token.lexeme.len >= 2) 
                               token.lexeme[1..token.lexeme.len-1] 
                               else token.lexeme;
            return ast.Pattern{ .Literal = ast.Literal{ .String = str_content }};
        }

        // SPEC CONFORMANCE: Only accept canonical boolean and nil literals in patterns
        if (self.match(.Based)) {
            return ast.Pattern{ .Literal = ast.Literal{ .Boolean = true }};
        }

        if (self.match(.Cringe)) {
            return ast.Pattern{ .Literal = ast.Literal{ .Boolean = false }};
        }
        
        if (self.match(.Nah)) {
            return ast.Pattern{ .Literal = ast.Literal{ .Nil = {} }};
        }
        
        // SPEC CONFORMANCE: Reject deprecated pattern forms
        if (self.match(.Truth) or self.match(.Lies) or self.match(.Cap)) {
            return ParserError.InvalidSyntax; // Use canonical forms instead
        }

        // Variable pattern
        if (self.check(.Identifier)) {
            const name = self.advance().lexeme;
            return ast.Pattern{ .Variable = name };
        }

        // Tuple pattern (pat1, pat2, ...)
        if (self.match(.LeftParen)) {
            var patterns = ArrayList(ast.Pattern).init(self.allocator);
            
            if (!self.check(.RightParen)) {
                while (true) {
                    const pattern = try self.parsePattern();
                    try patterns.append(pattern);
                    
                    if (!self.match(.Comma)) break;
                }
            }
            
            try self.consume(.RightParen, "Expected ')'");
            return ast.Pattern{ .Tuple = patterns };
        }

        // Struct pattern StructName{field: pattern, ...}
        if (self.check(.Identifier)) {
            const struct_name = self.advance().lexeme;
            
            if (self.match(.LeftBrace)) {
                var fields = ArrayList(ast.FieldPattern).init(self.allocator);
                
                if (!self.check(.RightBrace)) {
                    while (true) {
                        if (!self.check(.Identifier)) {
                            return ParserError.UnexpectedToken;
                        }
                        
                        const field_name = self.advance().lexeme;
                        try self.consume(.Colon, "Expected ':' after field name");
                        const pattern = try self.parsePattern();
                        
                        try fields.append(ast.FieldPattern{
                            .name = field_name,
                            .pattern = pattern,
                        });
                        
                        if (!self.match(.Comma)) break;
                    }
                }
                
                try self.consume(.RightBrace, "Expected '}'");
                
                return ast.Pattern{ .Struct = ast.StructPattern{
                    .name = struct_name,
                    .fields = fields,
                }};
            }
            
            return ast.Pattern{ .Variable = struct_name };
        }

        return ParserError.InvalidPattern;
    }

    // Helper methods for parsing statements
    fn parseReturnStatement(self: *Parser) ParserError!Statement {
        // SPEC CONFORMANCE: Only accept canonical "damn" return keyword
        if (!self.matchIdentifier("damn")) {
            return ParserError.UnexpectedToken;
        }
        
        var return_stmt = ast.ReturnStatement{ .value = null };
        
        // Parse optional return value
        if (!self.check(.Semicolon) and !self.check(.Newline) and !self.isAtEnd() and !self.check(.RightBrace)) {
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
            if (self.match(.Newline)) continue;
            
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
                    if (self.match(.Newline)) continue;
                    
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
        self.in_loop = true;
        defer { self.in_loop = false; }
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
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
        
        // Check for range-for loop (bestie var := flex ...)
        if (self.isRangeForLoop()) {
            return try self.parseRangeForStatement();
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
            self.in_loop = true;
            defer { self.in_loop = false; }
            
            while (!self.check(.RightBrace) and !self.isAtEnd()) {
                if (self.match(.Newline)) continue;
                
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
        self.in_loop = true;
        defer { self.in_loop = false; }
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
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
        self.in_loop = true;
        defer { self.in_loop = false; }
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = try self.parseStatement();
            try body.append(stmt);
        }
        
        try self.consume(.RightBrace, "Expected '}'");
        
        // Create ForIn statement
        return Statement{ .ForIn = ast.ForInStatement{
            .variable = if (variables.items.len > 0) variables.items[0] else "_",
            .iterable = iterable,
            .body = body,
        }};
    }

    // Continuing with more parsing methods...
    fn parseStructStatement(self: *Parser) ParserError!Statement {
        _ = self.advance(); // consume squad/struct
        
        // Parse struct name
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // Parse generic type parameters <T, U>
        var type_parameters = ArrayList(ast.TypeParameter).init(self.allocator);
        if (self.match(.Less) or self.match(.LeftAngle)) {
            while (!self.check(.Greater) and !self.check(.RightAngle) and !self.isAtEnd()) {
                if (self.check(.Identifier)) {
                    const param_name = self.advance().lexeme;
                    var param = ast.TypeParameter{
                        .name = param_name,
                        .constraints = ArrayList(ast.Type).init(self.allocator),
                    };
                    
                    // Parse constraints (T: Interface1 + Interface2)
                    if (self.match(.Colon)) {
                        while (!self.check(.Comma) and !self.check(.Greater) and !self.check(.RightAngle)) {
                            const constraint = try self.parseType();
                            try param.constraints.append(constraint);
                            if (!self.match(.Plus)) break;
                        }
                    }
                    
                    try type_parameters.append(param);
                }
                
                if (!self.match(.Comma)) break;
            }
            
            if (!self.match(.Greater) and !self.match(.RightAngle)) {
                return ParserError.MissingToken;
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
            
            // Parse visibility modifier
            var visibility = ast.Visibility.Private;
            if (self.match(.Spill)) {
                visibility = .Public;
            } else if (self.match(.Priv)) {
                visibility = .Private;
            } else if (self.match(.Crew)) {
                visibility = .Package;
            }
            
            // Parse field
            const field = try self.parseStructField(visibility);
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

    fn parseStructField(self: *Parser, visibility: ast.Visibility) ParserError!ast.StructField {
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
            .visibility = visibility,
        };
    }

    fn parseInterfaceStatement(self: *Parser) ParserError!Statement {
        try self.consume(.Collab, "Expected 'collab'");
        
        // Parse interface name
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // Parse generic type parameters
        var type_parameters = ArrayList(ast.TypeParameter).init(self.allocator);
        if (self.match(.Less) or self.match(.LeftAngle)) {
            while (!self.check(.Greater) and !self.check(.RightAngle) and !self.isAtEnd()) {
                if (self.check(.Identifier)) {
                    const param_name = self.advance().lexeme;
                    const param = ast.TypeParameter{
                        .name = param_name,
                        .constraints = ArrayList(ast.Type).init(self.allocator),
                    };
                    try type_parameters.append(param);
                }
                
                if (!self.match(.Comma)) break;
            }
            
            if (!self.match(.Greater) and !self.match(.RightAngle)) {
                return ParserError.MissingToken;
            }
        }
        
        // Parse interface inheritance (extends)
        var extends = ArrayList([]const u8).init(self.allocator);
        if (self.match(.Extends)) {
            while (true) {
                if (!self.check(.Identifier)) {
                    return ParserError.UnexpectedToken;
                }
                const parent_interface = self.advance().lexeme;
                try extends.append(parent_interface);
                
                if (!self.match(.Comma)) break;
            }
        }
        
        // Parse interface composition (with)
        var compositions = ArrayList([]const u8).init(self.allocator);
        if (self.match(.With)) {
            while (true) {
                if (!self.check(.Identifier)) {
                    return ParserError.UnexpectedToken;
                }
                const composed_interface = self.advance().lexeme;
                try compositions.append(composed_interface);
                
                if (!self.match(.Comma)) break;
            }
        }
        
        // Expect '{'
        try self.consume(.LeftBrace, "Expected '{' after interface name");
        
        // Parse methods
        var methods = ArrayList(ast.MethodSignature).init(self.allocator);
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            // Skip newlines
            if (self.match(.Newline)) {
                continue;
            }
            
            // Parse method signature
            const method = try self.parseMethodSignature();
            try methods.append(method);
            
            // Optional semicolon
            _ = self.match(.Semicolon);
        }
        
        try self.consume(.RightBrace, "Expected '}' after interface methods");
        
        return Statement{ .Interface = ast.InterfaceStatement{
            .name = name,
            .methods = methods,
            .visibility = .Public,
            .type_parameters = type_parameters,
            .extends = extends,
            .compositions = compositions,
        }};
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
        try self.consume(.LeftParen, "Expected '('");
        
        var parameters = ArrayList(Parameter).init(self.allocator);
        
        if (!self.check(.RightParen)) {
            while (true) {
                const param = try self.parseParameter();
                try parameters.append(param);
                
                if (!self.match(.Comma)) break;
            }
        }
        
        try self.consume(.RightParen, "Expected ')'");
        
        // Parse return type (optional)
        var return_type: ?ast.Type = null;
        if (!self.check(.Semicolon) and !self.check(.Newline) and !self.check(.RightBrace)) {
            return_type = try self.parseType();
        }
        
        return ast.MethodSignature{
            .name = name,
            .parameters = parameters,
            .return_type = return_type,
        };
    }

    // More advanced parsing methods continue...
    fn parseImplementationStatement(self: *Parser) ParserError!Statement {
        // Handle "impl TypeName for InterfaceName" syntax
        try self.consume(.Impl, "Expected 'impl'");
        
        // Parse implementing type
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        const implementing_type = self.advance().lexeme;
        
        // Expect 'for'
        if (!self.match(.ForImpl)) {
            return ParserError.UnexpectedToken;
        }
        
        // Parse interface name
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        const interface_name = self.advance().lexeme;
        
        // Parse implementation body
        try self.consume(.LeftBrace, "Expected '{'");
        
        var methods = ArrayList(ast.FunctionStatement).init(self.allocator);
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const method = try self.parseFunctionStatement();
            try methods.append(method);
        }
        
        try self.consume(.RightBrace, "Expected '}'");
        
        return Statement{ .Implementation = ast.ImplementationStatement{
            .implementing_type = implementing_type,
            .interface_name = interface_name,
            .methods = methods,
            .where_clause = null,
        }};
    }

    fn parseTypeAliasStatement(self: *Parser) ParserError!Statement {
        // Handle "be like" or just "BeLike" token
        if (self.check(.BeLike)) {
            _ = self.advance();
        } else if (self.matchIdentifier("be")) {
            try self.consumeIdentifier("like", "Expected 'like' after 'be'");
        } else {
            return ParserError.UnexpectedToken;
        }
        
        // Parse alias name
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        const name = self.advance().lexeme;
        
        // Expect '='
        try self.consume(.Equal, "Expected '=' after type alias name");
        
        // Parse target type
        const target_type = try self.parseType();
        
        return Statement{ .TypeAlias = ast.TypeAliasStatement{
            .name = name,
            .target_type = target_type,
            .visibility = .Public,
        }};
    }

    fn parseGoroutineStatement(self: *Parser) ParserError!Statement {
        try self.consume(.Stan, "Expected 'stan'");
        
        // Parse block or expression
        if (self.check(.LeftBrace)) {
            // Block form: stan { ... }
            try self.consume(.LeftBrace, "Expected '{'");
            
            var body = ArrayList(Statement).init(self.allocator);
            while (!self.check(.RightBrace) and !self.isAtEnd()) {
                if (self.match(.Newline)) continue;
                
                const stmt = try self.parseStatement();
                try body.append(stmt);
            }
            
            try self.consume(.RightBrace, "Expected '}'");
            
            return Statement{ .Goroutine = ast.GoroutineStatement{
                .call = Expression{ .Block = ast.BlockExpression{ .statements = body } },
            }};
        } else {
            // Expression form: stan functionCall()
            const call_expr = try self.parseExpression();
            return Statement{ .Goroutine = ast.GoroutineStatement{
                .call = call_expr,
            }};
        }
    }

    fn parseVibeCheckStatement(self: *Parser) ParserError!Statement {
        try self.consume(.VibeCheck, "Expected 'vibe check'");
        
        const expression = try self.parseExpression();
        
        try self.consume(.LeftBrace, "Expected '{'");
        
        var patterns = ArrayList(ast.PatternCase).init(self.allocator);
        var default_case: ?ArrayList(Statement) = null;
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            // Check for default case
            if (self.match(.Basic)) {
                try self.consume(.Colon, "Expected ':' after 'basic'");
                
                var default_stmts = ArrayList(Statement).init(self.allocator);
                while (!self.check(.Mood) and !self.check(.Basic) and !self.check(.RightBrace) and !self.isAtEnd()) {
                    if (self.match(.Newline)) continue;
                    
                    const stmt = try self.parseStatement();
                    try default_stmts.append(stmt);
                }
                
                default_case = default_stmts;
                continue;
            }
            
            // Parse case
            if (self.match(.Mood)) {
                const pattern = try self.parsePattern();
                
                var guard: ?Expression = null;
                if (self.matchIdentifier("if")) {
                    guard = try self.parseExpression();
                }
                
                try self.consume(.Colon, "Expected ':' after case pattern");
                
                var case_body = ArrayList(Statement).init(self.allocator);
                while (!self.check(.Mood) and !self.check(.Basic) and !self.check(.RightBrace) and !self.isAtEnd()) {
                    if (self.match(.Newline)) continue;
                    
                    const stmt = try self.parseStatement();
                    try case_body.append(stmt);
                }
                
                try patterns.append(ast.PatternCase{
                    .pattern = pattern,
                    .guard = guard,
                    .body = case_body,
                });
            }
        }
        
        try self.consume(.RightBrace, "Expected '}'");
        
        return Statement{ .PatternSwitch = ast.PatternSwitchStatement{
            .expression = expression,
            .patterns = patterns,
            .default_case = default_case,
        }};
    }

    fn parseSelectStatement(self: *Parser) ParserError!Statement {
        if (self.check(.Select)) {
            _ = self.advance();
        } else if (self.check(.Ready)) {
            _ = self.advance();
        } else {
            return ParserError.UnexpectedToken;
        }
        
        try self.consume(.LeftBrace, "Expected '{'");
        
        var cases = ArrayList(ast.SelectCase).init(self.allocator);
        var default_case: ?ArrayList(Statement) = null;
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            // Check for default case
            if (self.match(.Basic)) {
                try self.consume(.Colon, "Expected ':' after 'basic'");
                
                var default_stmts = ArrayList(Statement).init(self.allocator);
                while (!self.check(.Mood) and !self.check(.Basic) and !self.check(.RightBrace) and !self.isAtEnd()) {
                    if (self.match(.Newline)) continue;
                    
                    const stmt = try self.parseStatement();
                    try default_stmts.append(stmt);
                }
                
                default_case = default_stmts;
                continue;
            }
            
            // Parse case
            if (self.match(.Mood)) {
                const channel_op = try self.parseChannelOperation();
                
                try self.consume(.Colon, "Expected ':' after channel operation");
                
                var case_body = ArrayList(Statement).init(self.allocator);
                while (!self.check(.Mood) and !self.check(.Basic) and !self.check(.RightBrace) and !self.isAtEnd()) {
                    if (self.match(.Newline)) continue;
                    
                    const stmt = try self.parseStatement();
                    try case_body.append(stmt);
                }
                
                try cases.append(ast.SelectCase{
                    .channel_op = channel_op,
                    .body = case_body,
                });
            }
        }
        
        try self.consume(.RightBrace, "Expected '}'");
        
        return Statement{ .Select = ast.SelectStatement{
            .cases = cases,
            .default_case = default_case,
        }};
    }

    fn parseChannelOperation(self: *Parser) ParserError!ast.ChannelOperation {
        // Parse channel expression
        const channel = try self.parseExpression();
        
        if (self.match(.LeftArrow)) {
            // Send operation: channel <- value
            const value = try self.parseExpression();
            return ast.ChannelOperation{ .Send = .{
                .channel = channel,
                .value = value,
            }};
        } else {
            // Receive operation: variable := <-channel or just <-channel
            var variable: ?[]const u8 = null;
            
            // Check if this is an assignment
            if (self.match(.ColonEqual)) {
                // Get variable name from the channel expression
                switch (channel) {
                    .Identifier => |name| {
                        variable = name;
                    },
                    else => {
                        return ParserError.InvalidSyntax;
                    }
                }
                
                // Expect <- and then the actual channel
                try self.consume(.LeftArrow, "Expected '<-'");
                const actual_channel = try self.parseExpression();
                
                return ast.ChannelOperation{ .Receive = .{
                    .channel = actual_channel,
                    .variable = variable,
                }};
            } else {
                // Just receiving: <-channel
                return ast.ChannelOperation{ .Receive = .{
                    .channel = channel,
                    .variable = null,
                }};
            }
        }
    }

    fn parseDeferStatement(self: *Parser) ParserError!Statement {
        try self.consume(.Later, "Expected 'later'");
        
        const stmt_ptr = try self.allocator.create(Statement);
        stmt_ptr.* = try self.parseStatement();
        
        return Statement{ .Defer = ast.DeferStatement{ .statement = stmt_ptr } };
    }

    fn parseYikesStatement(self: *Parser) ParserError!ast.YikesStatement {
        try self.consume(.Yikes, "Expected 'yikes'");
        
        // Parse error name/identifier
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        var error_type: ?ast.Type = null;
        var value: ?Expression = null;
        
        // Optional error type annotation (yikes MyError tea)
        if (self.checkType()) {
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
            if (self.match(.Newline)) continue;
            
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
                if (self.match(.Newline)) continue;
                
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

    fn parseConstDeclaration(self: *Parser) ParserError!ast.ConstDecl {
        try self.consume(.Facts, "Expected 'facts'");
        
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        var const_type: ?ast.Type = null;
        if (self.match(.Colon)) {
            const_type = try self.parseType();
        }
        
        try self.consume(.Equal, "Expected '=' after constant name");
        
        const value = try self.parseExpression();
        
        return ast.ConstDecl{
            .name = name,
            .const_type = const_type,
            .value = value,
            .visibility = .Public,
        };
    }

    fn parseShortDeclaration(self: *Parser) ParserError!Statement {
        var names = ArrayList([]const u8).init(self.allocator);
        
        // Parse variable names (can be tuple destructuring)
        if (self.match(.LeftParen)) {
            // Tuple destructuring: (a, b, c) := (1, 2, 3)
            while (!self.check(.RightParen) and !self.isAtEnd()) {
                if (!self.check(.Identifier)) {
                    return ParserError.UnexpectedToken;
                }
                
                try names.append(self.advance().lexeme);
                
                if (!self.match(.Comma)) break;
            }
            
            try self.consume(.RightParen, "Expected ')'");
        } else {
            // Single variable or comma-separated: a, b := 1, 2
            if (!self.check(.Identifier)) {
                return ParserError.UnexpectedToken;
            }
            
            try names.append(self.advance().lexeme);
            
            while (self.match(.Comma)) {
                if (!self.check(.Identifier)) {
                    return ParserError.UnexpectedToken;
                }
                
                try names.append(self.advance().lexeme);
            }
        }
        
        try self.consume(.ColonEqual, "Expected ':=' in short declaration");
        
        // Parse values
        var values = ArrayList(Expression).init(self.allocator);
        
        if (self.match(.LeftParen)) {
            // Tuple values: (1, 2, 3)
            while (!self.check(.RightParen) and !self.isAtEnd()) {
                const value = try self.parseExpression();
                try values.append(value);
                
                if (!self.match(.Comma)) break;
            }
            
            try self.consume(.RightParen, "Expected ')'");
        } else {
            // Single value or comma-separated: 1, 2
            const value = try self.parseExpression();
            try values.append(value);
            
            while (self.match(.Comma)) {
                const next_value = try self.parseExpression();
                try values.append(next_value);
            }
        }
        
        return Statement{ .ShortDeclaration = ast.ShortDeclarationStatement{
            .names = names,
            .values = values,
        }};
    }

    fn parseAssignmentStatement(self: *Parser) ParserError!Statement {
        const target = try self.parseExpression();
        
        if (!self.match(.Equal) and !self.match(.PlusEqual) and !self.match(.MinusEqual) and
           !self.match(.StarEqual) and !self.match(.SlashEqual) and !self.match(.PercentEqual)) {
            return ParserError.UnexpectedToken;
        }
        
        const operator = self.previous().lexeme;
        const value = try self.parseExpression();
        
        return Statement{ .Assignment = ast.AssignmentStatement{
            .target = target,
            .value = value,
            .operator = operator,
        }};
    }

    // Helper utility methods
    fn match(self: *Parser, kind: TokenKind) bool {
        if (self.check(kind)) {
            _ = self.advance();
            return true;
        }
        return false;
    }

    fn matchIdentifier(self: *Parser, identifier: []const u8) bool {
        if (self.check(.Identifier)) {
            const token = self.peek();
            if (std.mem.eql(u8, token.lexeme, identifier)) {
                _ = self.advance();
                return true;
            }
        }
        return false;
    }

    fn checkIdentifier(self: *Parser, identifier: []const u8) bool {
        if (self.check(.Identifier)) {
            const token = self.peek();
            return std.mem.eql(u8, token.lexeme, identifier);
        }
        return false;
    }

    fn check(self: *Parser, kind: TokenKind) bool {
        if (self.isAtEnd()) return false;
        return self.peek().kind == kind;
    }

    fn checkType(self: *Parser) bool {
        return self.check(.Normie) or self.check(.Tea) or self.check(.Txt) or
               self.check(.Sip) or self.check(.Smol) or self.check(.Mid) or
               self.check(.Thicc) or self.check(.Snack) or self.check(.Meal) or
               self.check(.Byte) or self.check(.Rune) or self.check(.Extra) or
               self.check(.Lit) or self.check(.Cap) or self.check(.Identifier) or
               self.check(.LeftBracket) or self.check(.Dm) or 
               self.checkIdentifier("map") or self.check(.LeftParen);
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

    fn peekNext(self: *Parser) Token {
        if (self.current + 1 >= self.tokens.len) {
            return Token.init(.Eof, "", 0, 0);
        }
        return self.tokens[self.current + 1];
    }

    fn previous(self: *Parser) Token {
        if (self.current == 0) return self.tokens[0];
        return self.tokens[self.current - 1];
    }

    fn consume(self: *Parser, kind: TokenKind, message: []const u8) ParserError!Token {
        if (self.check(kind)) return self.advance();
        
        std.debug.print("Parser error: {s}. Expected {}, got {}\n", .{ message, kind, self.peek().kind });
        self.had_error = true;
        return ParserError.UnexpectedToken;
    }

    fn consumeIdentifier(self: *Parser, identifier: []const u8, message: []const u8) ParserError!Token {
        if (self.matchIdentifier(identifier)) return self.previous();
        
        std.debug.print("Parser error: {s}. Expected '{s}', got {s}\n", .{ message, identifier, self.peek().lexeme });
        self.had_error = true;
        return ParserError.UnexpectedToken;
    }

    fn synchronize(self: *Parser) void {
        _ = self.advance();

        while (!self.isAtEnd()) {
            if (self.previous().kind == .Semicolon) return;

            switch (self.peek().kind) {
                .Slay, .Sus, .Facts, .Lowkey, .Bestie, .Periodt, .Squad, .Collab => return,
                else => {},
            }

            _ = self.advance();
        }
    }

    // Helper predicates
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

    fn isRangeForLoop(self: *Parser) bool {
        // Look for pattern: identifier [, identifier] := flex
        var pos = self.current;
        
        if (pos >= self.tokens.len or self.tokens[pos].kind != .Identifier) {
            return false;
        }
        pos += 1;
        
        // Optional second identifier with comma
        if (pos < self.tokens.len and self.tokens[pos].kind == .Comma) {
            pos += 1;
            if (pos >= self.tokens.len or self.tokens[pos].kind != .Identifier) {
                return false;
            }
            pos += 1;
        }
        
        // Check for := flex
        if (pos < self.tokens.len and 
            (self.tokens[pos].kind == .ColonEqual or self.tokens[pos].kind == .Equal)) {
            pos += 1;
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

    // Memory allocation helpers
    fn allocateExpression(self: *Parser, expr: Expression) ParserError!*Expression {
        const ptr = self.allocator.create(Expression) catch return ParserError.OutOfMemory;
        ptr.* = expr;
        return ptr;
    }

    fn allocateUnaryExpression(self: *Parser, unary: ast.UnaryExpression) ParserError!*ast.UnaryExpression {
        const ptr = self.allocator.create(ast.UnaryExpression) catch return ParserError.OutOfMemory;
        ptr.* = unary;
        return ptr;
    }

    fn allocateMemberAccess(self: *Parser, member_access: ast.MemberAccessExpression) ParserError!*ast.MemberAccessExpression {
        const ptr = self.allocator.create(ast.MemberAccessExpression) catch return ParserError.OutOfMemory;
        ptr.* = member_access;
        return ptr;
    }

    fn allocateArrayExpression(self: *Parser, array_expr: ast.ArrayExpression) ParserError!*ast.ArrayExpression {
        const ptr = self.allocator.create(ast.ArrayExpression) catch return ParserError.OutOfMemory;
        ptr.* = array_expr;
        return ptr;
    }

    fn allocateMapExpression(self: *Parser, map_expr: ast.MapExpression) ParserError!*ast.MapExpression {
        const ptr = self.allocator.create(ast.MapExpression) catch return ParserError.OutOfMemory;
        ptr.* = map_expr;
        return ptr;
    }

    fn convertExpressionsToPointers(self: *Parser, expressions: ArrayList(Expression)) ParserError!ArrayList(*anyopaque) {
        var pointers = ArrayList(*anyopaque).init(self.allocator);
        
        for (expressions.items) |expr| {
            const ptr = try self.allocateExpression(expr);
            try pointers.append(@ptrCast(ptr));
        }
        
        expressions.deinit();
        return pointers;
    }

    // Advanced parser features
    fn parseGenericType(self: *Parser, base_name: []const u8) ParserError!ast.Type {
        // Parse generic type like Vec<T>, Map<K,V>
        if (!self.match(.Less) and !self.match(.LeftAngle)) {
            return ast.Type{ .Custom = base_name };
        }
        
        var type_arguments = ArrayList(ast.Type).init(self.allocator);
        
        while (!self.check(.Greater) and !self.check(.RightAngle) and !self.isAtEnd()) {
            const type_arg = try self.parseType();
            try type_arguments.append(type_arg);
            
            if (!self.match(.Comma)) break;
        }
        
        if (!self.match(.Greater) and !self.match(.RightAngle)) {
            return ParserError.MissingToken;
        }
        
        return ast.Type{ .Generic = ast.GenericType{
            .name = base_name,
            .type_arguments = type_arguments,
            .constraints = ArrayList(ast.TypeConstraint).init(self.allocator),
        }};
    }
    
    fn parseTypeConstraint(self: *Parser) ParserError!ast.TypeConstraint {
        // Parse constraints like T: Drawable, T = String, T <: Number
        if (self.match(.Colon)) {
            // Interface constraint: T: InterfaceName
            if (!self.check(.Identifier)) {
                return ParserError.UnexpectedToken;
            }
            const interface_name = self.advance().lexeme;
            return ast.TypeConstraint{ .Interface = interface_name };
        }
        
        if (self.match(.Equal)) {
            // Equality constraint: T = ConcreteType
            const concrete_type = try self.parseType();
            return ast.TypeConstraint{ .Equality = concrete_type };
        }
        
        if (self.check(.Less) and self.peekNext().kind == .Colon) {
            // Subtype constraint: T <: SuperType
            _ = self.advance(); // consume '<'
            _ = self.advance(); // consume ':'
            const super_type = try self.parseType();
            return ast.TypeConstraint{ .Subtype = super_type };
        }
        
        if (self.check(.Greater) and self.peekNext().kind == .Colon) {
            // Supertype constraint: T >: SubType
            _ = self.advance(); // consume '>'
            _ = self.advance(); // consume ':'
            const sub_type = try self.parseType();
            return ast.TypeConstraint{ .Supertype = sub_type };
        }
        
        if (self.matchIdentifier("where")) {
            // Where clause constraint: where T.method() > 0
            // For now, just consume the rest as a string
            var where_clause = ArrayList(u8).init(self.allocator);
            while (!self.check(.Comma) and !self.check(.Greater) and !self.check(.RightAngle) and !self.isAtEnd()) {
                const token = self.advance();
                try where_clause.appendSlice(token.lexeme);
                try where_clause.append(' ');
            }
            return ast.TypeConstraint{ .WhereClause = where_clause.items };
        }
        
        return ParserError.InvalidSyntax;
    }
    
    fn parseComplexType(self: *Parser) ParserError!ast.Type {
        // Parse union types: Type1 | Type2 | Type3
        var base_type = try self.parseBasicType();
        
        if (self.match(.Pipe)) {
            var union_types = ArrayList(ast.Type).init(self.allocator);
            try union_types.append(base_type);
            
            while (true) {
                const union_member = try self.parseBasicType();
                try union_types.append(union_member);
                
                if (!self.match(.Pipe)) break;
            }
            
            // For now, represent union as a generic type
            return ast.Type{ .Generic = ast.GenericType{
                .name = "Union",
                .type_arguments = union_types,
                .constraints = ArrayList(ast.TypeConstraint).init(self.allocator),
            }};
        }
        
        return base_type;
    }
    
    fn parseBasicType(self: *Parser) ParserError!ast.Type {
        // Parse a single basic type (before union/intersection operators)
        if (self.check(.Identifier)) {
            const type_name = self.advance().lexeme;
            
            // Check for generic arguments
            if (self.check(.Less) or self.check(.LeftAngle)) {
                return try self.parseGenericType(type_name);
            }
            
            return ast.Type{ .Custom = type_name };
        }
        
        // Fall back to existing parseType logic
        return try self.parseType();
    }
    
    fn parseAdvancedFunctionSignature(self: *Parser) ParserError!FunctionStatement {
        try self.consume(.Slay, "Expected 'slay'");
        
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        var func = FunctionStatement.init(self.allocator, name);
        
        // Parse generic type parameters with enhanced constraints
        if (self.match(.Less) or self.match(.LeftAngle)) {
            while (!self.check(.Greater) and !self.check(.RightAngle) and !self.isAtEnd()) {
                if (self.check(.Identifier)) {
                    const param_name = self.advance().lexeme;
                    var type_param = ast.TypeParameter{
                        .name = param_name,
                        .constraints = ArrayList(ast.TypeConstraint).init(self.allocator),
                        .default_type = null,
                        .variance = .Invariant,
                    };
                    
                    // Parse variance modifiers
                    if (self.matchIdentifier("out")) {
                        type_param.variance = .Covariant;
                    } else if (self.matchIdentifier("in")) {
                        type_param.variance = .Contravariant;
                    }
                    
                    // Parse constraints (T: SomeInterface + AnotherInterface)
                    if (self.match(.Colon)) {
                        while (!self.check(.Comma) and !self.check(.Greater) and !self.check(.RightAngle)) {
                            const constraint = try self.parseTypeConstraint();
                            try type_param.constraints.append(constraint);
                            if (!self.match(.Plus)) break;
                        }
                    }
                    
                    // Parse default type
                    if (self.match(.Equal)) {
                        type_param.default_type = try self.parseType();
                    }
                    
                    try func.type_parameters.append(type_param);
                }
                
                if (!self.match(.Comma)) break;
            }
            
            if (!self.match(.Greater) and !self.match(.RightAngle)) {
                return ParserError.MissingToken;
            }
        }
        
        // Parse parameters with complex types
        try self.consume(.LeftParen, "Expected '(' after function name");
        
        if (!self.check(.RightParen)) {
            while (true) {
                const param = try self.parseAdvancedParameter();
                try func.parameters.append(param);
                
                if (!self.match(.Comma)) break;
            }
        }
        
        try self.consume(.RightParen, "Expected ')' after parameters");
        
        // Parse return type (can be complex)
        if (!self.check(.LeftBrace)) {
            func.return_type = try self.parseComplexType();
        }
        
        // Parse function body
        try self.consume(.LeftBrace, "Expected '{'");
        
        self.in_function = true;
        defer { self.in_function = false; }
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = try self.parseStatement();
            try func.body.append(stmt);
        }
        
        try self.consume(.RightBrace, "Expected '}'");
        
        return func;
    }
    
    fn parseAdvancedParameter(self: *Parser) ParserError!Parameter {
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // Parse parameter type (can be complex)
        const param_type = try self.parseComplexType();
        
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
};

// Tests
test "parser basic program" {
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

test "parser CURSED function" {
    const allocator = std.testing.allocator;
    
    // Create tokens for "slay test() lit { damn based }"
    const tokens = [_]Token{
        Token.init(.Slay, "slay", 1, 1),
        Token.init(.Identifier, "test", 1, 6),
        Token.init(.LeftParen, "(", 1, 10),
        Token.init(.RightParen, ")", 1, 11),
        Token.init(.Lit, "lit", 1, 13),
        Token.init(.LeftBrace, "{", 1, 17),
        Token.init(.Identifier, "damn", 1, 19),
        Token.init(.Based, "based", 1, 24),
        Token.init(.RightBrace, "}", 1, 30),
        Token.init(.Eof, "", 1, 31),
    };
    
    var parser = Parser.init(allocator, &tokens);
    const program = try parser.parseProgram();
    defer program.deinit(allocator);
    
    try std.testing.expect(program.statements.items.len == 1);
    
    switch (program.statements.items[0]) {
        .Function => |func| {
            try std.testing.expect(std.mem.eql(u8, func.name, "test"));
            try std.testing.expect(func.return_type != null);
            try std.testing.expect(func.body.items.len == 1);
        },
        else => try std.testing.expect(false),
    }
}
