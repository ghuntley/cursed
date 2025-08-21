const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const ast = @import("ast.zig");
const crash_handler = @import("crash_handler.zig");
const safe_operations = @import("safe_operations.zig");

const Token = lexer.Token;
const TokenKind = lexer.TokenKind;
const Program = ast.Program;
const Statement = ast.Statement;
const Expression = ast.Expression;
const FunctionStatement = ast.FunctionStatement;
const LetStatement = ast.LetStatement;
const Type = ast.Type;
const Parameter = ast.Parameter;

/// Enhanced parser with comprehensive error handling
/// Replaces panic/abort calls with graceful error recovery
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
    AlignmentError,
    CorruptedAst,
    TooManyErrors,
    RecursionLimit,
    InvalidInput,
};

/// Detailed error information with source context
pub const ParseError = struct {
    kind: ParserError,
    message: []const u8,
    location: ?ast.SourceLocation,
    token: ?Token,
    context: []const u8,
    
    pub fn init(kind: ParserError, message: []const u8, location: ?ast.SourceLocation, token: ?Token, context: []const u8) ParseError {
        return ParseError{
            .kind = kind,
            .message = message,
            .location = location,
            .token = token,
            .context = context,
        };
    }
    
    pub fn print(self: ParseError, writer: anytype) !void {
        if (self.location) |loc| {
            try writer.print("Error at {}:{}:{} - {s}", .{ loc.file, loc.line, loc.column, self.message });
        } else {
            try writer.print("Error: {s}", .{self.message});
        }
        
        if (self.token) |token| {
            try writer.print(" (token: {s})", .{token.lexeme});
        }
        
        if (self.context.len > 0) {
            try writer.print(" (context: {s})", .{self.context});
        }
        
        try writer.print("\n");
    }
};

/// Enhanced parser with graceful error handling
pub const EnhancedParser = struct {
    tokens: []const Token,
    current: usize,
    allocator: Allocator,
    arena: std.heap.ArenaAllocator,
    arena_allocator: Allocator,
    had_error: bool,
    in_function: bool,
    in_loop: bool,
    scope_depth: usize,
    file_path: []const u8,
    telemetry: ?*crash_handler.CrashTelemetry,
    
    // Error handling state
    errors: ArrayList(ParseError),
    max_errors: usize,
    recursion_depth: usize,
    max_recursion: usize,
    panic_mode: bool,
    
    pub fn init(allocator: Allocator, tokens: []const Token) EnhancedParser {
        var arena = std.heap.ArenaAllocator.init(allocator);
        return EnhancedParser{
            .tokens = tokens,
            .current = 0,
            .allocator = allocator,
            .arena = arena,
            .arena_allocator = arena.allocator(),
            .had_error = false,
            .in_function = false,
            .in_loop = false,
            .scope_depth = 0,
            .file_path = "unknown",
            .telemetry = null,
            .errors = .empty,
            .max_errors = 10,
            .recursion_depth = 0,
            .max_recursion = 100,
            .panic_mode = false,
        };
    }

    pub fn deinit(self: *EnhancedParser) void {
        self.errors.deinit(allocator);
        self.arena.deinit(allocator);
    }

    pub fn initWithFile(allocator: Allocator, tokens: []const Token, file_path: []const u8) EnhancedParser {
        var parser = init(allocator, tokens);
        parser.file_path = file_path;
        return parser;
    }
    
    pub fn initWithTelemetry(allocator: Allocator, tokens: []const Token, file_path: []const u8, telemetry: *crash_handler.CrashTelemetry) EnhancedParser {
        var parser = initWithFile(allocator, tokens, file_path);
        parser.telemetry = telemetry;
        return parser;
    }

    /// Safe type conversion with validation
    fn safePtrCast(self: *EnhancedParser, comptime T: type, ptr: anytype) ParserError!*T {
        const alignment = @alignOf(T);
        const addr = @intFromPtr(ptr);
        if (addr % alignment != 0) {
            try self.reportError("Invalid pointer alignment", "safePtrCast");
            return ParserError.AlignmentError;
        }
        return @ptrCast(@alignCast(ptr));
    }

    /// Enhanced error reporting with context
    fn reportError(self: *EnhancedParser, message: []const u8, context: []const u8) ParserError {
        return self.reportErrorAtLocation(message, context, self.getCurrentSourceLocation(), self.peek());
    }
    
    fn reportErrorAtLocation(self: *EnhancedParser, message: []const u8, context: []const u8, location: ?ast.SourceLocation, token: Token) ParserError {
        // Validate inputs
        if (message.len == 0 or message.len > 1024) {
            std.debug.print("Internal error: Invalid error message length: {}\n", .{message.len});
            self.had_error = true;
            return ParserError.InvalidInput;
        }
        
        // Check if we've hit the error limit
        if (self.errors.items.len >= self.max_errors) {
            std.debug.print("Too many parse errors, aborting...\n");
            return ParserError.TooManyErrors;
        }
        
        const error_info = ParseError.init(ParserError.SyntaxError, message, location, token, context);
        
        // Try to add error to list (gracefully handle OOM)
        self.errors.append(allocator, error_info) catch {
            // If we can't allocate for error tracking, at least print it
            std.debug.print("Error: {s} (failed to track error due to OOM)\n", .{message});
        };
        
        // Print error immediately for user feedback
        error_info.print(std.debug) catch {};
        
        self.had_error = true;
        return ParserError.SyntaxError;
    }

    /// Get current source location with bounds checking
    fn getCurrentSourceLocation(self: *EnhancedParser) ?ast.SourceLocation {
        if (self.current < self.tokens.len) {
            const token = self.tokens[self.current];
            return ast.SourceLocation{
                .file = self.file_path,
                .line = @as(u32, @intCast(@min(token.line, std.math.maxInt(u32)))),
                .column = @as(u32, @intCast(@min(token.column, std.math.maxInt(u32)))),
                .offset = @as(u32, @intCast(@min(token.offset, std.math.maxInt(u32)))),
            };
        }
        return null;
    }

    /// Enhanced synchronization for error recovery
    fn synchronize(self: *EnhancedParser) void {
        self.panic_mode = false;
        
        _ = self.advance();
        
        // Synchronize to statement boundaries
        while (!self.isAtEnd()) {
            if (self.previous().kind == .Semicolon) return;
            
            switch (self.peek().kind) {
                .Slay, .Sus, .Facts, .Squad, .Collab, .Vibe, .Yeet, .Ready, .Lowkey => return,
                else => {},
            }
            
            _ = self.advance();
        }
    }

    /// Recovery to specific token types
    fn recoverToNext(self: *EnhancedParser, target_tokens: []const TokenKind) void {
        self.panic_mode = false;
        
        while (!self.isAtEnd()) {
            for (target_tokens) |target| {
                if (self.check(target)) return;
            }
            _ = self.advance();
        }
    }

    /// Enhanced bounds checking for recursion
    fn checkRecursionLimit(self: *EnhancedParser) ParserError!void {
        if (self.recursion_depth >= self.max_recursion) {
            try self.reportError("Maximum recursion depth exceeded", "recursion_check");
            return ParserError.RecursionLimit;
        }
    }

    /// Safe program parsing with comprehensive error handling
    pub fn parseProgram(self: *EnhancedParser) ParserError!Program {
        var program = Program.init(self.allocator);
        errdefer program.deinit(allocator);
        
        while (!self.isAtEnd()) {
            // Skip newlines, semicolons, and comments gracefully
            if (self.check(.Newline) or self.check(.Semicolon) or 
               self.check(.LineComment) or self.check(.BlockComment) or self.check(.Comment)) {
                _ = self.advance();
                continue;
            }

            // Parse package declaration with error recovery
            if (self.check(.Vibe)) {
                if (self.parsePackageDeclaration()) |pkg| {
                    program.package = pkg;
                } else |err| {
                    self.synchronize();
                    if (err == ParserError.TooManyErrors) return err;
                }
                continue;
            }

            // Parse import statement with error recovery
            if (self.check(.Yeet)) {
                if (self.parseImportStatement()) |import_stmt| {
                    program.imports.append(allocator, import_stmt) catch |err| {
                        try self.reportError("Failed to add import to program", "parseProgram");
                        if (err == error.OutOfMemory) return ParserError.OutOfMemory;
                    };
                } else |err| {
                    self.synchronize();
                    if (err == ParserError.TooManyErrors) return err;
                }
                continue;
            }

            // Parse regular statements with enhanced error handling
            if (self.parseStatement()) |stmt| {
                const stmt_ptr = self.allocator.create(Statement) catch |err| {
                    try self.reportError("Out of memory allocating statement", "parseProgram");
                    return ParserError.OutOfMemory;
                };
                
                stmt_ptr.* = stmt;
                
                // Safe pointer conversion
                const anyopaque_ptr = self.safePtrCast(anyopaque, stmt_ptr) catch |err| {
                    self.allocator.destroy(stmt_ptr);
                    try self.reportError("Failed to convert statement pointer", "parseProgram");
                    return err;
                };
                
                program.statements.append(self.allocator, anyopaque_ptr) catch |err| {
                    self.allocator.destroy(stmt_ptr);
                    try self.reportError("Failed to add statement to program", "parseProgram");
                    if (err == error.OutOfMemory) return ParserError.OutOfMemory;
                };
            } else |err| {
                self.synchronize();
                if (err == ParserError.TooManyErrors) return err;
            }
        }

        return program;
    }

    /// Enhanced statement parsing with error recovery
    fn parseStatement(self: *EnhancedParser) ParserError!Statement {
        try self.checkRecursionLimit();
        self.recursion_depth += 1;
        defer self.recursion_depth -= 1;
        
        // Skip comments at statement level
        while (self.check(.LineComment) or self.check(.BlockComment) or self.check(.Comment)) {
            _ = self.advance();
        }
        
        // Function declaration (slay)
        if (self.check(.Slay)) {
            return Statement{ .Function = self.parseFunctionStatement() catch |err| {
                self.synchronize();
                return err;
            }};
        }
        
        // Variable declaration (sus/facts)
        if (self.check(.Sus) or self.check(.Facts)) {
            return Statement{ .Let = self.parseLetStatement() catch |err| {
                self.synchronize();
                return err;
            }};
        }
        
        // Return statement (damn only - canonical spec)
        if (self.matchIdentifier("damn")) {
            return self.parseReturnStatement() catch |err| {
                self.synchronize();
                return err;
            };
        }
        
        // If statement (lowkey/ready)
        if (self.check(.Lowkey) or self.check(.Ready)) {
            return Statement{ .If = self.parseIfStatement() catch |err| {
                self.synchronize();
                return err;
            }};
        }
        
        // While statement (periodt/flex/bestie)
        if (self.check(.Periodt) or self.check(.Flex) or self.check(.Bestie)) {
            return Statement{ .While = self.parseWhileStatement() catch |err| {
                self.synchronize();
                return err;
            }};
        }
        
        // Break/continue with validation
        if (self.check(.Ghosted)) {
            if (!self.in_loop) {
                try self.reportError("'ghosted' statement outside of loop", "parseStatement");
                return ParserError.InvalidStatement;
            }
            _ = self.advance();
            return Statement{ .Break = ast.BreakStatement{} };
        }
        
        if (self.check(.Simp)) {
            if (!self.in_loop) {
                try self.reportError("'simp' statement outside of loop", "parseStatement");
                return ParserError.InvalidStatement;
            }
            _ = self.advance();
            return Statement{ .Continue = ast.ContinueStatement{} };
        }
        
        // Expression statement as fallback with error handling
        const expr = self.parseExpression() catch |err| {
            self.synchronize();
            return err;
        };
        
        const expr_ptr = self.allocator.create(Expression) catch {
            try self.reportError("Out of memory allocating expression", "parseStatement");
            return ParserError.OutOfMemory;
        };
        errdefer self.allocator.destroy(expr_ptr);
        
        expr_ptr.* = expr;
        
        const anyopaque_ptr = self.safePtrCast(anyopaque, expr_ptr) catch |err| {
            return err;
        };
        
        return Statement{ .Expression = anyopaque_ptr };
    }

    /// Enhanced function statement parsing
    fn parseFunctionStatement(self: *EnhancedParser) ParserError!FunctionStatement {
        try self.checkRecursionLimit();
        
        _ = try self.consume(.Slay, "Expected 'slay'");
        
        if (!self.check(.Identifier) and !self.check(.Spill)) {
            try self.reportError("Expected function name after 'slay'", "parseFunctionStatement");
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        var func = FunctionStatement.init(self.allocator, name);
        errdefer func.deinit(allocator);
        
        // Parse parameters with error recovery
        _ = try self.consume(.LeftParen, "Expected '(' after function name");
        
        if (!self.check(.RightParen)) {
            while (true) {
                const param = self.parseParameter() catch |err| {
                    // Try to recover by skipping to next comma or closing paren
                    self.recoverToNext(&[_]TokenKind{.Comma, .RightParen});
                    if (self.check(.RightParen)) break;
                    if (self.check(.Comma)) {
                        _ = self.advance(); // consume comma and continue
                        continue;
                    }
                    return err;
                };
                
                func.parameters.append(allocator, param) catch {
                    try self.reportError("Out of memory adding parameter", "parseFunctionStatement");
                    return ParserError.OutOfMemory;
                };
                
                if (!self.match(.Comma)) break;
            }
        }
        
        _ = try self.consume(.RightParen, "Expected ')' after parameters");
        
        // Parse return type (optional)
        if (!self.check(.LeftBrace)) {
            func.return_type = self.parseType() catch |err| {
                try self.reportError("Invalid return type", "parseFunctionStatement");
                return err;
            };
        }
        
        // Parse function body
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        self.in_function = true;
        defer { self.in_function = false; }
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = self.parseStatement() catch |err| {
                self.synchronize();
                if (err == ParserError.TooManyErrors) return err;
                continue; // Skip this statement and try the next one
            };
            
            const stmt_ptr = self.allocator.create(Statement) catch {
                try self.reportError("Out of memory allocating statement in function body", "parseFunctionStatement");
                return ParserError.OutOfMemory;
            };
            stmt_ptr.* = stmt; 
            
            func.body.append(self.allocator, stmt_ptr) catch {
                self.allocator.destroy(stmt_ptr);
                try self.reportError("Out of memory adding statement to function body", "parseFunctionStatement");
                return ParserError.OutOfMemory;
            };
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
        return func;
    }

    /// Enhanced parameter parsing with validation
    fn parseParameter(self: *EnhancedParser) ParserError!Parameter {
        if (!self.check(.Identifier)) {
            try self.reportError("Expected parameter name", "parseParameter");
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // Parse parameter type with validation
        const param_type = self.parseType() catch |err| {
            try self.reportError("Invalid parameter type", "parseParameter");
            return err;
        };
        
        return Parameter{
            .name = name,
            .param_type = param_type,
            .is_mutable = false,
            .default_value = null,
        };
    }

    /// Enhanced let statement parsing
    fn parseLetStatement(self: *EnhancedParser) ParserError!LetStatement {
        const is_const = self.match(.Facts);
        if (!is_const) {
            _ = try self.consume(.Sus, "Expected 'sus' or 'facts'");
        }
        
        if (!self.check(.Identifier)) {
            try self.reportError("Expected variable name", "parseLetStatement");
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // Parse type annotation (optional)
        var var_type: ?ast.Type = null;
        if (self.match(.Colon)) {
            var_type = self.parseType() catch |err| {
                try self.reportError("Invalid type annotation", "parseLetStatement");
                return err;
            };
        }
        
        // Parse initializer
        _ = try self.consume(.Equal, "Expected '=' after variable name");
        
        const value = self.parseExpression() catch |err| {
            try self.reportError("Invalid variable initializer", "parseLetStatement");
            return err;
        };
        
        const value_ptr = self.allocator.create(Expression) catch {
            try self.reportError("Out of memory allocating expression", "parseLetStatement");
            return ParserError.OutOfMemory;
        };
        value_ptr.* = value;
        
        const anyopaque_ptr = self.safePtrCast(anyopaque, value_ptr) catch |err| {
            self.allocator.destroy(value_ptr);
            return err;
        };
        
        return LetStatement{
            .name = name,
            .var_type = var_type,
            .value = anyopaque_ptr,
            .is_mutable = !is_const,
        };
    }

    /// Enhanced expression parsing with proper error recovery
    fn parseExpression(self: *EnhancedParser) ParserError!Expression {
        try self.checkRecursionLimit();
        self.recursion_depth += 1;
        defer self.recursion_depth -= 1;
        
        return self.parseAssignment();
    }

    fn parseAssignment(self: *EnhancedParser) ParserError!Expression {
        var expr = try self.parseOr();

        if (self.match(.Equal) or self.match(.PlusEqual) or self.match(.MinusEqual) or
           self.match(.StarEqual) or self.match(.SlashEqual) or self.match(.PercentEqual)) {
            const operator = self.previous().lexeme;
            const value = try self.parseAssignment();
            
            const left_ptr = self.allocator.create(Expression) catch {
                try self.reportError("Out of memory in assignment", "parseAssignment");
                return ParserError.OutOfMemory;
            };
            left_ptr.* = expr;
            
            const right_ptr = self.allocator.create(Expression) catch {
                self.allocator.destroy(left_ptr);
                try self.reportError("Out of memory in assignment", "parseAssignment");
                return ParserError.OutOfMemory;
            };
            right_ptr.* = value;
            
            return Expression{ .Assignment = ast.AssignmentExpression{
                .left = left_ptr,
                .operator = operator,
                .right = right_ptr,
            }};
        }

        return expr;
    }

    fn parseOr(self: *EnhancedParser) ParserError!Expression {
        var expr = try self.parseAnd();

        while (self.match(.Or) or self.match(.PipePipe)) {
            const operator = self.previous().lexeme;
            const right = try self.parseAnd();
            
            const left_ptr = self.allocator.create(Expression) catch {
                try self.reportError("Out of memory in logical or", "parseOr");
                return ParserError.OutOfMemory;
            };
            left_ptr.* = expr;
            
            const right_ptr = self.allocator.create(Expression) catch {
                self.allocator.destroy(left_ptr);
                try self.reportError("Out of memory in logical or", "parseOr");
                return ParserError.OutOfMemory;
            };
            right_ptr.* = right;
            
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = left_ptr,
                .operator = operator,
                .right = right_ptr,
            }};
        }

        return expr;
    }

    fn parseAnd(self: *EnhancedParser) ParserError!Expression {
        var expr = try self.parseEquality();

        while (self.match(.And) or self.match(.AmpersandAmpersand)) {
            const operator = self.previous().lexeme;
            const right = try self.parseEquality();
            
            const left_ptr = self.allocator.create(Expression) catch {
                try self.reportError("Out of memory in logical and", "parseAnd");
                return ParserError.OutOfMemory;
            };
            left_ptr.* = expr;
            
            const right_ptr = self.allocator.create(Expression) catch {
                self.allocator.destroy(left_ptr);
                try self.reportError("Out of memory in logical and", "parseAnd");
                return ParserError.OutOfMemory;
            };
            right_ptr.* = right;
            
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = left_ptr,
                .operator = operator,
                .right = right_ptr,
            }};
        }

        return expr;
    }

    fn parseEquality(self: *EnhancedParser) ParserError!Expression {
        var expr = try self.parseComparison();

        while (self.match(.BangEqual) or self.match(.EqualEqual)) {
            const operator = self.previous().lexeme;
            const right = try self.parseComparison();
            
            const left_ptr = self.allocator.create(Expression) catch {
                try self.reportError("Out of memory in equality", "parseEquality");
                return ParserError.OutOfMemory;
            };
            left_ptr.* = expr;
            
            const right_ptr = self.allocator.create(Expression) catch {
                self.allocator.destroy(left_ptr);
                try self.reportError("Out of memory in equality", "parseEquality");
                return ParserError.OutOfMemory;
            };
            right_ptr.* = right;
            
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = left_ptr,
                .operator = operator,
                .right = right_ptr,
            }};
        }

        return expr;
    }

    fn parseComparison(self: *EnhancedParser) ParserError!Expression {
        var expr = try self.parseTerm();

        while (self.match(.Greater) or self.match(.GreaterEqual) or
               self.match(.Less) or self.match(.LessEqual)) {
            const operator = self.previous().lexeme;
            const right = try self.parseTerm();
            
            const left_ptr = self.allocator.create(Expression) catch {
                try self.reportError("Out of memory in comparison", "parseComparison");
                return ParserError.OutOfMemory;
            };
            left_ptr.* = expr;
            
            const right_ptr = self.allocator.create(Expression) catch {
                self.allocator.destroy(left_ptr);
                try self.reportError("Out of memory in comparison", "parseComparison");
                return ParserError.OutOfMemory;
            };
            right_ptr.* = right;
            
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = left_ptr,
                .operator = operator,
                .right = right_ptr,
            }};
        }

        return expr;
    }

    fn parseTerm(self: *EnhancedParser) ParserError!Expression {
        var expr = try self.parseFactor();

        while (self.match(.Minus) or self.match(.Plus)) {
            const operator = self.previous().lexeme;
            const right = try self.parseFactor();
            
            const left_ptr = self.allocator.create(Expression) catch {
                try self.reportError("Out of memory in term", "parseTerm");
                return ParserError.OutOfMemory;
            };
            left_ptr.* = expr;
            
            const right_ptr = self.allocator.create(Expression) catch {
                self.allocator.destroy(left_ptr);
                try self.reportError("Out of memory in term", "parseTerm");
                return ParserError.OutOfMemory;
            };
            right_ptr.* = right;
            
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = left_ptr,
                .operator = operator,
                .right = right_ptr,
            }};
        }

        return expr;
    }

    fn parseFactor(self: *EnhancedParser) ParserError!Expression {
        var expr = try self.parseUnary();

        while (self.match(.Slash) or self.match(.Star) or self.match(.Percent)) {
            const operator = self.previous().lexeme;
            const right = try self.parseUnary();
            
            const left_ptr = self.allocator.create(Expression) catch {
                try self.reportError("Out of memory in factor", "parseFactor");
                return ParserError.OutOfMemory;
            };
            left_ptr.* = expr;
            
            const right_ptr = self.allocator.create(Expression) catch {
                self.allocator.destroy(left_ptr);
                try self.reportError("Out of memory in factor", "parseFactor");
                return ParserError.OutOfMemory;
            };
            right_ptr.* = right;
            
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = left_ptr,
                .operator = operator,
                .right = right_ptr,
            }};
        }

        return expr;
    }

    fn parseUnary(self: *EnhancedParser) ParserError!Expression {
        if (self.match(.Bang) or self.match(.Minus) or self.match(.Plus)) {
            const operator = self.previous().lexeme;
            const right = try self.parseUnary();
            
            const operand_ptr = self.allocator.create(Expression) catch {
                try self.reportError("Out of memory in unary", "parseUnary");
                return ParserError.OutOfMemory;
            };
            operand_ptr.* = right;
            
            const unary_ptr = self.allocator.create(ast.UnaryExpression) catch {
                self.allocator.destroy(operand_ptr);
                try self.reportError("Out of memory in unary", "parseUnary");
                return ParserError.OutOfMemory;
            };
            unary_ptr.* = ast.UnaryExpression{
                .operator = operator,
                .operand = operand_ptr,
            };
            
            return Expression{ .Unary = unary_ptr };
        }

        return self.parseCall();
    }

    fn parseCall(self: *EnhancedParser) ParserError!Expression {
        var expr = try self.parsePrimary();

        while (true) {
            if (self.match(.LeftParen)) {
                expr = try self.finishCall(expr);
            } else if (self.match(.Dot)) {
                if (!self.check(.Identifier)) {
                    try self.reportError("Expected property name after '.'", "parseCall");
                    return ParserError.UnexpectedToken;
                }
                const property = self.advance().lexeme;
                
                const object_ptr = self.allocator.create(Expression) catch {
                    try self.reportError("Out of memory in member access", "parseCall");
                    return ParserError.OutOfMemory;
                };
                object_ptr.* = expr;
                
                const member_access_ptr = self.allocator.create(ast.MemberAccessExpression) catch {
                    self.allocator.destroy(object_ptr);
                    try self.reportError("Out of memory in member access", "parseCall");
                    return ParserError.OutOfMemory;
                };
                member_access_ptr.* = ast.MemberAccessExpression{
                    .object = object_ptr,
                    .property = property,
                };
                
                expr = Expression{ .MemberAccess = member_access_ptr };
            } else {
                break;
            }
        }

        return expr;
    }

    fn finishCall(self: *EnhancedParser, callee: Expression) ParserError!Expression {
        var arguments = .empty;
        errdefer {
            for (arguments.items) |arg| {
                self.allocator.destroy(arg);
            }
            arguments.deinit(allocator);
        }

        if (!self.check(.RightParen)) {
            while (true) {
                const arg = try self.parseExpression();
                const arg_ptr = self.allocator.create(Expression) catch {
                    try self.reportError("Out of memory in function call", "finishCall");
                    return ParserError.OutOfMemory;
                };
                arg_ptr.* = arg;
                
                arguments.append(self.allocator, arg_ptr) catch {
                    self.allocator.destroy(arg_ptr);
                    try self.reportError("Out of memory in function call", "finishCall");
                    return ParserError.OutOfMemory;
                };

                if (!self.match(.Comma)) break;
            }
        }

        _ = try self.consume(.RightParen, "Expected ')' after arguments");

        const function_ptr = self.allocator.create(Expression) catch {
            try self.reportError("Out of memory in function call", "finishCall");
            return ParserError.OutOfMemory;
        };
        function_ptr.* = callee;

        return Expression{ .Call = .{
            .function = function_ptr,
            .arguments = arguments,
        }};
    }

    fn parsePrimary(self: *EnhancedParser) ParserError!Expression {
        // Boolean literals with validation
        if (self.match(.Based) or self.match(.Truth)) {
            return Expression{ .Boolean = true };
        }
        
        if (self.match(.Cringe)) {
            return Expression{ .Boolean = false };
        }
        
        if (self.match(.Nah)) {
            return Expression{ .Literal = ast.Literal{ .Nil = {} } };
        }
        
        // Numbers with validation
        if (self.check(.Number) or self.check(.Integer)) {
            const token = self.advance();
            if (std.mem.indexOf(u8, token.lexeme, ".")) |_| {
                // Float
                const value = std.fmt.parseFloat(f64, token.lexeme) catch {
                    try self.reportError("Invalid float literal", "parsePrimary");
                    return ParserError.InvalidSyntax;
                };
                return Expression{ .Float = value };
            } else {
                // Integer
                const value = std.fmt.parseInt(i64, token.lexeme, 10) catch {
                    try self.reportError("Invalid integer literal", "parsePrimary");
                    return ParserError.InvalidSyntax;
                };
                return Expression{ .Integer = value };
            }
        }
        
        // Strings with validation
        if (self.check(.StringLiteral) or self.check(.String)) {
            const token = self.advance();
            const str_content = if (token.lexeme.len >= 2 and 
                                   token.lexeme[0] == '"' and 
                                   token.lexeme[token.lexeme.len-1] == '"')
                                 token.lexeme[1..token.lexeme.len-1] // Remove quotes
                                 else token.lexeme;
            
            return Expression{ .String = str_content };
        }
        
        // Identifiers
        if (self.check(.Identifier)) {
            const name = self.advance().lexeme;
            return Expression{ .Identifier = name };
        }
        
        // Grouped expressions
        if (self.match(.LeftParen)) {
            const expr = try self.parseExpression();
            _ = try self.consume(.RightParen, "Expected ')' after expression");
            return expr;
        }
        
        try self.reportError("Unexpected token in expression", "parsePrimary");
        return ParserError.UnexpectedToken;
    }

    /// Enhanced type parsing with validation
    fn parseType(self: *EnhancedParser) ParserError!ast.Type {
        // Basic types
        if (self.match(.Drip)) {
            return ast.Type{ .Basic = ast.BasicType.Drip };
        }
        
        if (self.match(.Tea)) {
            return ast.Type{ .Basic = ast.BasicType.Tea };
        }
        
        if (self.match(.Lit)) {
            return ast.Type{ .Basic = ast.BasicType.Lit };
        }
        
        // Custom types
        if (self.check(.Identifier)) {
            const type_name = self.advance().lexeme;
            return ast.Type{ .Custom = type_name };
        }
        
        try self.reportError("Expected type", "parseType");
        return ParserError.InvalidType;
    }

    /// Additional parsing methods with error handling
    fn parsePackageDeclaration(self: *EnhancedParser) ParserError!ast.PackageDeclaration {
        _ = try self.consume(.Vibe, "Expected 'vibe'");
        
        if (!self.check(.Identifier)) {
            try self.reportError("Expected package name", "parsePackageDeclaration");
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        return ast.PackageDeclaration{
            .name = name,
            .version = null,
        };
    }

    fn parseImportStatement(self: *EnhancedParser) ParserError!ast.ImportStatement {
        _ = try self.consume(.Yeet, "Expected 'yeet'");
        
        if (!self.check(.StringLiteral) and !self.check(.String)) {
            try self.reportError("Expected import path", "parseImportStatement");
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

    fn parseReturnStatement(self: *EnhancedParser) ParserError!Statement {
        var return_stmt = ast.ReturnStatement{ .value = null };
        
        // Parse optional return value
        if (!self.check(.Semicolon) and !self.check(.Newline) and !self.isAtEnd() and !self.check(.RightBrace)) {
            const value_expr = try self.parseExpression();
            const value_ptr = self.allocator.create(Expression) catch {
                try self.reportError("Out of memory in return statement", "parseReturnStatement");
                return ParserError.OutOfMemory;
            };
            value_ptr.* = value_expr;
            
            const anyopaque_ptr = self.safePtrCast(anyopaque, value_ptr) catch |err| {
                self.allocator.destroy(value_ptr);
                return err;
            };
            
            return_stmt.value = anyopaque_ptr;
        }
        
        return Statement{ .Return = return_stmt };
    }

    fn parseIfStatement(self: *EnhancedParser) ParserError!ast.IfStatement {
        // Handle both 'ready' and 'lowkey' keywords
        if (self.check(.Ready)) {
            _ = self.advance(); // consume 'ready'
        } else {
            _ = try self.consume(.Lowkey, "Expected 'lowkey' or 'ready'");
        }
        
        _ = try self.consume(.LeftParen, "Expected '(' after if keyword");
        const condition = try self.parseExpression();
        _ = try self.consume(.RightParen, "Expected ')' after condition");
        
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        var then_branch = .empty;
        errdefer {
            for (then_branch.items) |stmt_ptr| {
                const stmt: *Statement = self.safePtrCast(Statement, stmt_ptr) catch continue;
                self.allocator.destroy(stmt);
            }
            then_branch.deinit(allocator);
        }
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = self.parseStatement() catch |err| {
                self.synchronize();
                if (err == ParserError.TooManyErrors) return err;
                continue;
            };
            
            const stmt_ptr = self.allocator.create(Statement) catch {
                try self.reportError("Out of memory in if statement", "parseIfStatement");
                return ParserError.OutOfMemory;
            };
            stmt_ptr.* = stmt;
            
            const anyopaque_ptr = self.safePtrCast(anyopaque, stmt_ptr) catch |err| {
                self.allocator.destroy(stmt_ptr);
                return err;
            };
            
            then_branch.append(self.allocator, anyopaque_ptr) catch {
                self.allocator.destroy(stmt_ptr);
                try self.reportError("Out of memory in if statement", "parseIfStatement");
                return ParserError.OutOfMemory;
            };
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
        const condition_ptr = self.allocator.create(Expression) catch {
            try self.reportError("Out of memory in if statement", "parseIfStatement");
            return ParserError.OutOfMemory;
        };
        condition_ptr.* = condition;
        
        const condition_anyopaque = self.safePtrCast(anyopaque, condition_ptr) catch |err| {
            self.allocator.destroy(condition_ptr);
            return err;
        };
        
        return ast.IfStatement{
            .condition = condition_anyopaque,
            .then_branch = then_branch,
            .else_branch = null,
        };
    }

    fn parseWhileStatement(self: *EnhancedParser) ParserError!ast.WhileStatement {
        _ = self.advance(); // consume periodt/flex/bestie
        
        _ = try self.consume(.LeftParen, "Expected '(' after while keyword");
        const condition = try self.parseExpression();
        _ = try self.consume(.RightParen, "Expected ')' after condition");
        
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        var body = .empty;
        errdefer {
            for (body.items) |stmt| {
                self.allocator.destroy(stmt);
            }
            body.deinit(allocator);
        }
        
        self.in_loop = true;
        defer { self.in_loop = false; }
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = self.parseStatement() catch |err| {
                self.synchronize();
                if (err == ParserError.TooManyErrors) return err;
                continue;
            };
            
            const stmt_ptr = self.allocator.create(Statement) catch {
                try self.reportError("Out of memory in while statement", "parseWhileStatement");
                return ParserError.OutOfMemory;
            };
            stmt_ptr.* = stmt;
            
            body.append(self.allocator, stmt_ptr) catch {
                self.allocator.destroy(stmt_ptr);
                try self.reportError("Out of memory in while statement", "parseWhileStatement");
                return ParserError.OutOfMemory;
            };
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
        const condition_ptr = self.allocator.create(Expression) catch {
            try self.reportError("Out of memory in while statement", "parseWhileStatement");
            return ParserError.OutOfMemory;
        };
        condition_ptr.* = condition;
        
        return ast.WhileStatement{
            .condition = condition_ptr,
            .body = body,
        };
    }

    /// Helper utility methods with bounds checking
    fn match(self: *EnhancedParser, kind: TokenKind) bool {
        if (self.check(kind)) {
            _ = self.advance();
            return true;
        }
        return false;
    }

    fn matchIdentifier(self: *EnhancedParser, identifier: []const u8) bool {
        if (self.check(.Identifier)) {
            const token = self.peek();
            if (std.mem.eql(u8, token.lexeme, identifier)) {
                _ = self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(self: *EnhancedParser, kind: TokenKind) bool {
        if (self.isAtEnd()) return false;
        return self.peek().kind == kind;
    }

    fn advance(self: *EnhancedParser) Token {
        if (!self.isAtEnd()) self.current += 1;
        return self.previous();
    }

    fn isAtEnd(self: *EnhancedParser) bool {
        return self.current >= self.tokens.len or self.peek().kind == .Eof;
    }

    fn peek(self: *EnhancedParser) Token {
        if (self.current >= self.tokens.len) {
            return Token.init(.Eof, "", 0, 0);
        }
        return self.tokens[self.current];
    }

    fn previous(self: *EnhancedParser) Token {
        if (self.current == 0) return self.tokens[0];
        return self.tokens[self.current - 1];
    }

    fn consume(self: *EnhancedParser, kind: TokenKind, message: []const u8) ParserError!Token {
        if (self.check(kind)) return self.advance();
        
        try self.reportError(message, "consume");
        return ParserError.UnexpectedToken;
    }

    /// Get all collected errors
    pub fn getErrors(self: *EnhancedParser) []const ParseError {
        return self.errors.items;
    }

    /// Print all errors
    pub fn printErrors(self: *EnhancedParser, writer: anytype) !void {
        for (self.errors.items) |error_info| {
            try error_info.print(writer);
        }
    }

    /// Check if parsing had errors
    pub fn hasErrors(self: *EnhancedParser) bool {
        return self.had_error or self.errors.items.len > 0;
    }
};

// Test the enhanced parser
test "enhanced parser graceful error handling" {
    const allocator = std.testing.allocator;
    
    // Test with malformed tokens
    const tokens = [_]Token{
        Token.init(.Slay, "slay", 1, 1),
        Token.init(.Number, "invalid", 1, 6), // Invalid number
        Token.init(.Plus, "+", 1, 13),
        Token.init(.Eof, "", 1, 14),
    };
    
    var parser = EnhancedParser.init(allocator, &tokens);
    defer parser.deinit(allocator);
    
    // Should not crash, but should report errors
    const result = parser.parseProgram();
    
    // Check that errors were collected
    try std.testing.expect(parser.hasErrors());
    
    if (result) |program| {
        defer program.deinit(allocator);
        // Parser should continue despite errors
    } else |err| {
        // Error should be handled gracefully
        try std.testing.expect(err != error.OutOfMemory);
    }
}
