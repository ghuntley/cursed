const std = @import("std");
const builtin = @import("builtin");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const ast = @import("ast.zig");
const crash_handler = @import("crash_handler.zig");
const safe_operations = @import("safe_operations.zig");
const hygienic_macro_system = @import("hygienic_macro_system.zig");

const Token = lexer.Token;
const TokenKind = lexer.TokenKind;
const Program = ast.Program;
const Statement = ast.Statement;
const Expression = ast.Expression;
const FunctionStatement = ast.FunctionStatement;
const LetStatement = ast.LetStatement;
const Type = ast.Type;
const Parameter = ast.Parameter;

// Pratt Parser Types
pub const Prec = enum {
    None,
    Assignment,  // =
    Or,          // ||
    And,         // &&
    Equality,    // == !=
    Comparison,  // > >= < <=
    Term,        // + -
    Factor,      // * / %
    Unary,       // ! - * &
    Access,      // .
    Call,        // () []
    Primary,

    pub fn lessThan(self: Prec, other: Prec) bool {
        return @intFromEnum(self) < @intFromEnum(other);
    }
};

pub const PrefixParseFn = ?*const fn(*Parser) ParserError!Expression;
pub const InfixParseFn = ?*const fn(*Parser, Expression) ParserError!Expression;

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
    InfiniteLoop,
    ParseTimeout,
};

pub const ErrorRecoveryStats = struct {
    total_errors: usize = 0,
    semicolon_recoveries: usize = 0,
    statement_recoveries: usize = 0,
    expression_recoveries: usize = 0,
    delimiter_recoveries: usize = 0,
    tokens_skipped: usize = 0,
    
    pub fn init() ErrorRecoveryStats {
        return ErrorRecoveryStats{};
    }
    
    pub fn reportStats(self: *const ErrorRecoveryStats) void {
        _ = self;
        // Error recovery statistics disabled for clean test output
    }
};

pub const Parser = struct {
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
    error_recovery_stats: ErrorRecoveryStats,
    macro_system: ?*hygienic_macro_system.HygienicMacroSystem,
    use_pratt: bool = true,
    // Infinite loop detection
    loop_position_counter: std.AutoHashMap(usize, usize),
    parse_start_time: i64,
    const MAX_LOOP_ATTEMPTS: usize = 10;
    const PARSE_TIMEOUT_MS: i64 = 30000; // 30 seconds

    pub fn init(allocator: Allocator, tokens: []const Token) Parser {
        var arena = std.heap.ArenaAllocator.init(allocator);
        return Parser{
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
            .error_recovery_stats = ErrorRecoveryStats.init(),
            .macro_system = null,
            .use_pratt = true,
            .loop_position_counter = std.AutoHashMap(usize, usize).init(allocator),
            .parse_start_time = std.time.milliTimestamp(),
        };
    }

    pub fn deinit(self: *Parser) void {
        // FIXED: Ensure all arena-allocated memory is properly cleaned up
        // The arena automatically cleans up all its allocations
        self.arena.deinit();
        
        // Clean up loop detection hashmap
        self.loop_position_counter.deinit();
        
        // Report final error recovery stats if there were issues
        if (self.error_recovery_stats.total_errors > 0) {
            self.error_recovery_stats.reportStats();
        }
    }

    /// Check for infinite loops and timeouts
    fn checkInfiniteLoop(self: *Parser) ParserError!void {
        // Check for timeout
        const current_time = std.time.milliTimestamp();
        if (current_time - self.parse_start_time > Parser.PARSE_TIMEOUT_MS) {
            std.debug.print("ERROR: Parser timeout after {} ms\n", .{current_time - self.parse_start_time});
            return ParserError.ParseTimeout;
        }
        
        // Check for infinite loop at current position
        const result = self.loop_position_counter.getOrPut(self.current) catch {
            return ParserError.OutOfMemory;
        };
        
        if (result.found_existing) {
            result.value_ptr.* += 1;
            if (result.value_ptr.* > Parser.MAX_LOOP_ATTEMPTS) {
                std.debug.print("ERROR: Infinite loop detected at position {} (attempted {} times)\n", .{ self.current, result.value_ptr.* });
                std.debug.print("Current token: {any}\n", .{if (self.current < self.tokens.len) self.tokens[self.current].kind else .Eof});
                return ParserError.InfiniteLoop;
            }
        } else {
            result.value_ptr.* = 1;
        }
    }
    
    /// Reset infinite loop counter when parser position advances
    fn resetLoopCounter(self: *Parser) void {
        // Clear old position counters periodically to avoid memory issues
        if (self.loop_position_counter.count() > 100) {
            self.loop_position_counter.clearAndFree();
        }
    }

    /// Check if the current token is a keyword that can be used as a method name
    fn isKeywordAllowedAsMethodName(self: *Parser) bool {
        return switch (self.peek().kind) {
            .Tea, .Drip, .Lit, .Cap, .Normie, .Smol, .Mid, .Thicc, 
            .Snack, .Meal, .Byte, .Rune, .Extra, .Txt, .Sip => true,
            else => false,
        };
    }

    // Helper to handle >> vs > ambiguity in generic types
    fn matchGenericClosing(self: *Parser, depth: u32) bool {
        if (depth == 1) {
            return self.match(.Greater) or self.match(.RightAngle) or self.match(.RightShift);
        } else {
            return self.match(.Greater) or self.match(.RightAngle);
        }
    }

    pub fn initWithFile(allocator: Allocator, tokens: []const Token, file_path: []const u8) Parser {
        var arena = std.heap.ArenaAllocator.init(allocator);
        return Parser{
            .tokens = tokens,
            .current = 0,
            .allocator = allocator,
            .arena = arena,
            .arena_allocator = arena.allocator(),
            .had_error = false,
            .in_function = false,
            .in_loop = false,
            .scope_depth = 0,
            .file_path = file_path,
            .telemetry = null,
            .error_recovery_stats = ErrorRecoveryStats.init(),
            .macro_system = null,
            .use_pratt = true,
            .loop_position_counter = std.AutoHashMap(usize, usize).init(allocator),
            .parse_start_time = std.time.milliTimestamp(),
        };
    }
    
    pub fn initWithTelemetry(allocator: Allocator, tokens: []const Token, file_path: []const u8, telemetry: *crash_handler.CrashTelemetry) Parser {
        var arena = std.heap.ArenaAllocator.init(allocator);
        return Parser{
            .tokens = tokens,
            .current = 0,
            .allocator = allocator,
            .arena = arena,
            .arena_allocator = arena.allocator(),
            .had_error = false,
            .in_function = false,
            .in_loop = false,
            .scope_depth = 0,
            .file_path = file_path,
            .telemetry = telemetry,
            .error_recovery_stats = ErrorRecoveryStats.init(),
            .macro_system = null,
            .use_pratt = true,
            .loop_position_counter = std.AutoHashMap(usize, usize).init(allocator),
            .parse_start_time = std.time.milliTimestamp(),
        };
    }

    // Safe type conversion helpers to replace unsafe @ptrCast
    fn statementToAnyopaque(self: *Parser, stmt_ptr: *Statement) !*anyopaque {
        // Runtime type validation - ensure the pointer is properly aligned
        const alignment = @alignOf(Statement);
        const addr = @intFromPtr(stmt_ptr);
        if (addr % alignment != 0) {
            if (self.telemetry) |telemetry| {
                var context = crash_handler.CrashContext.init(
                    self.allocator,
                    .Fatal,
                    "Statement pointer is not properly aligned",
                    @src().file,
                    @src().line,
                    0,
                    @src().fn_name
                ) catch return error.AlignmentError;
                defer context.deinit(self.allocator);
                
                telemetry.recordCrash(context) catch {};
            }
            return error.AlignmentError;
        }
        
        return @ptrCast(stmt_ptr);
    }
    
    fn expressionToAnyopaque(self: *Parser, expr_ptr: *Expression) !*anyopaque {
        // Runtime type validation - ensure the pointer is properly aligned
        const alignment = @alignOf(Expression);
        const addr = @intFromPtr(expr_ptr);
        if (addr % alignment != 0) {
            if (self.telemetry) |telemetry| {
                var context = crash_handler.CrashContext.init(
                    self.allocator,
                    .Fatal,
                    "Expression pointer is not properly aligned",
                    @src().file,
                    @src().line,
                    0,
                    @src().fn_name
                ) catch return error.AlignmentError;
                defer context.deinit(self.allocator);
                
                telemetry.recordCrash(context) catch {};
            }
            return error.AlignmentError;
        }
        
        return @ptrCast(expr_ptr);
    }
    
    fn anyopaqueToStatement(_: *Parser, ptr: *anyopaque) ?*Statement {
        // Check alignment
        const alignment = @alignOf(Statement);
        const addr = @intFromPtr(ptr);
        if (addr % alignment != 0) {
            return null;
        }
        
        return @ptrCast(@alignCast(ptr));
    }
    
    fn anyopaqueToExpression(_: *Parser, ptr: *anyopaque) ?*Expression {
        // Check alignment
        const alignment = @alignOf(Expression);
        const addr = @intFromPtr(ptr);
        if (addr % alignment != 0) {
            return null;
        }
        
        return @ptrCast(@alignCast(ptr));
    }

    // Pratt Parser Implementation Functions

    /// Get precedence for a given token kind
    fn getPrecedence(self: *Parser, token_kind: TokenKind) Prec {
        _ = self;
        return switch (token_kind) {
            .Equal, .PlusEqual, .MinusEqual, .StarEqual, .SlashEqual, .PercentEqual => .Assignment,
            .PipePipe => .Or,
            .AmpAmp => .And,
            .EqualEqual, .BangEqual => .Equality,
            .Greater, .GreaterEqual, .Less, .LessEqual => .Comparison,
            .Plus, .Minus => .Term,
            .Star, .Slash, .Percent => .Factor,
            .PlusPlus, .MinusMinus => .Unary,
            .Bang, .At => .Unary, // Added .At for ඞ unary operator
            .LeftParen, .LeftBracket, .LeftBrace => .Call,
            .Dot => .Access,
            else => .None,
        };
    }

    /// Get prefix parsing function for a given token kind
    fn getPrefixFunction(self: *Parser, token_kind: TokenKind) PrefixParseFn {
        _ = self;
        return switch (token_kind) {
            .Identifier => parsePrattIdentifier,
            .Number, .Integer => parsePrattNumber,
            .StringLiteral, .String => parsePrattString,
            .Based, .Cringe, .Truth => parsePrattBoolean,
            .Nah => parsePrattNil,
            .LeftParen => parsePrattGrouping,
            .LeftBracket => parsePrattArrayOrComposite,
            .LeftBrace => parsePrattMapOrComposite,
            .Bang, .Minus, .Star, .Amp, .At => parsePrattUnary, // Added .At for ඞ address-of operator
            .PlusPlus, .MinusMinus => parsePrattPrefixIncrement,
            .Normie, .Tea, .Txt, .Sip, .Smol, .Mid, .Thicc, .Snack, .Meal, .Byte, .Rune, .Extra, .Lit, .Cap, .Yikes => parsePrattTypeForComposite,
            else => null,
        };
    }

    /// Get infix parsing function for a given token kind  
    fn getInfixFunction(self: *Parser, token_kind: TokenKind) InfixParseFn {
        _ = self;
        return switch (token_kind) {
            .Plus, .Minus, .Star, .Slash, .Percent => parsePrattBinary,
            .EqualEqual, .BangEqual => parsePrattBinary,
            .Greater, .GreaterEqual, .Less, .LessEqual => parsePrattBinary,
            .AmpAmp, .PipePipe => parsePrattBinary,
            .Equal, .PlusEqual, .MinusEqual, .StarEqual, .SlashEqual, .PercentEqual => parsePrattAssignment,
            .LeftParen => parsePrattCall,
            .Dot => parsePrattMemberAccess,
            .LeftBracket => parsePrattArrayAccess,
            .LeftBrace => parsePrattStructLiteral,
            .PlusPlus, .MinusMinus => parsePrattPostfixIncrement,
            else => null,
        };
    }

    // Prefix parsing functions

    fn parsePrattIdentifier(self: *Parser) ParserError!Expression {
        const name = self.advance().lexeme;
        
        // CRITICAL: Check for composite literal syntax: Type[expr]{...}
        // Only check for specific types that can be composite literals
        if (self.isBasicTypeName(name) and self.check(.LeftBracket)) {
            // Look ahead to see if this is Type[expr]{...} pattern
            const saved_current = self.current;
            
            // Parse the bracket part temporarily to check for { afterwards
            if (self.match(.LeftBracket)) {
                // Skip the bracket expression
                var bracket_depth: usize = 1;
                while (bracket_depth > 0 and !self.isAtEnd()) {
                    if (self.check(.LeftBracket)) {
                        bracket_depth += 1;
                    } else if (self.check(.RightBracket)) {
                        bracket_depth -= 1;
                    }
                    self.current += 1;
                }
                
                // Check if followed by {
                if (self.check(.LeftBrace)) {
                    // This is a composite literal! Parse it properly
                    self.current = saved_current; // Reset to before the bracket
                    return self.parseCompositeLiteralFromType(name) catch |err| {
                        std.debug.print("DEBUG: Failed to parse composite literal for type '{s}': {any}\n", .{ name, err });
                        return err;
                    };
                }
                
                // Not a composite literal, reset and continue as identifier
                self.current = saved_current;
            }
        }
        
        return Expression{ .Identifier = name };
    }

    fn parsePrattTypeForComposite(self: *Parser) ParserError!Expression {
        const type_token = self.advance();
        const type_name = type_token.lexeme;
        
        // Check for composite literal syntax: Type[expr]{...}
        if (self.check(.LeftBracket)) {
            // Look ahead to see if this is Type[expr]{...} pattern
            const saved_current = self.current;
            
            // Parse the bracket part temporarily to check for { afterwards
            if (self.match(.LeftBracket)) {
                // Skip the bracket expression
                var bracket_depth: usize = 1;
                while (bracket_depth > 0 and !self.isAtEnd()) {
                    if (self.check(.LeftBracket)) {
                        bracket_depth += 1;
                    } else if (self.check(.RightBracket)) {
                        bracket_depth -= 1;
                    }
                    self.current += 1;
                }
                
                // Check if followed by {
                if (self.check(.LeftBrace)) {
                    // This is a composite literal! Parse it properly
                    self.current = saved_current; // Reset to before the bracket
                    return self.parseCompositeLiteralFromType(type_name);
                }
                
                // Not a composite literal, reset and continue as identifier
                self.current = saved_current;
            }
        }
        
        // If not a composite literal, treat as a regular identifier for now
        return Expression{ .Identifier = type_name };
    }

    fn parsePrattNumber(self: *Parser) ParserError!Expression {
        const token = self.advance();
        if (std.mem.indexOf(u8, token.lexeme, ".")) |_| {
            const value = std.fmt.parseFloat(f64, token.lexeme) catch {
                return ParserError.InvalidSyntax;
            };
            return Expression{ .Float = value };
        } else {
            const value = std.fmt.parseInt(i64, token.lexeme, 10) catch {
                return ParserError.InvalidSyntax;
            };
            return Expression{ .Integer = value };
        }
    }

    fn parsePrattString(self: *Parser) ParserError!Expression {
        const token = self.advance();
        // Strip quotes properly like other string parsing functions
        const str_content = if (token.lexeme.len >= 2 and 
                               token.lexeme[0] == '"' and 
                               token.lexeme[token.lexeme.len-1] == '"')
                              token.lexeme[1..token.lexeme.len-1] // Remove quotes
                              else token.lexeme;
        return Expression{ .String = str_content }; // Store WITHOUT quotes
    }

    fn parsePrattBoolean(self: *Parser) ParserError!Expression {
        const token = self.advance();
        return Expression{ .Boolean = switch (token.kind) {
            .Based, .Truth => true,
            .Cringe => false,
            else => unreachable,
        }};
    }

    fn parsePrattNil(self: *Parser) ParserError!Expression {
        _ = self.advance();
        return Expression{ .Literal = ast.Literal{ .Nil = {} } };
    }

    fn parsePrattGrouping(self: *Parser) ParserError!Expression {
        _ = self.advance(); // consume '('
        const expr = try self.parseExpression();
        _ = try self.consume(.RightParen, "Expected ')' after expression");
        return expr;
    }

    fn parsePrattUnary(self: *Parser) ParserError!Expression {
        const operator = self.advance().lexeme;
        const right = try self.parseExpressionPrattPrec(.Unary);
        return Expression{ .Unary = try self.allocateUnaryExpression(ast.UnaryExpression{
            .operator = operator,
            .operand = try self.allocateExpression(right),
        })};
    }

    fn parsePrattArrayOrComposite(self: *Parser) ParserError!Expression {
        _ = self.advance(); // consume '['
        
        // CRITICAL: Add bounds check to prevent segfaults
        if (self.isAtEnd()) {
            return ParserError.UnexpectedEof;
        }
        
        // NEW SYNTAX: Only parse array literals [1, 2, 3] here
        // Composite literals now use syntax: Type[value]{...} (parsed elsewhere)
        
        // Check if this is an empty array []
        if (self.check(.RightBracket)) {
            _ = self.advance(); // consume ']'
            
            return Expression{ .Array = try self.allocateArrayExpression(ast.ArrayExpression{
                .elements = .{},
            })};
        }
        
        // Parse as array literal [expr1, expr2, ...]
        return self.parseArrayLiteralElements() catch |elem_err| {
            // CRITICAL: Proper error handling
            return if (elem_err == ParserError.UnexpectedEof) 
                ParserError.UnexpectedEof 
            else 
                ParserError.InvalidExpression;
        };
    }

    fn parsePrattMapOrComposite(self: *Parser) ParserError!Expression {
        _ = self.advance(); // consume '{'
        
        // Empty braces {}
        if (self.check(.RightBrace)) {
            _ = self.advance(); // consume '}'
            // For now, treat empty {} as empty map
            return Expression{ .Map = try self.allocateMapExpression(ast.MapExpression{
                .entries = .{},
            })};
        }
        
        // Parse as map literal {key: value, ...}
        var entries = ArrayList(ast.MapEntry){};
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            const key = try self.parseExpression();
            _ = try self.consume(.Colon, "Expected ':' after map key");
            const value = try self.parseExpression();
            
            const key_ptr = try self.arena_allocator.create(Expression);
            key_ptr.* = key;
            const value_ptr = try self.arena_allocator.create(Expression);
            value_ptr.* = value;
            
            try entries.append(self.allocator, ast.MapEntry{
                .key = key_ptr,
                .value = value_ptr,
            });
            
            if (!self.match(.Comma)) break;
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
        return Expression{ .Map = try self.allocateMapExpression(ast.MapExpression{
            .entries = entries,
        })};
    }

    // Infix parsing functions

    fn parsePrattBinary(self: *Parser, left: Expression) ParserError!Expression {
        const operator_token = self.advance();
        const precedence = self.getPrecedence(operator_token.kind);
        const right = try self.parseExpressionPrattPrec(precedence);
        
        return Expression{ .Binary = ast.BinaryExpression{
            .left = try self.allocateExpression(left),
            .operator = operator_token.lexeme,
            .right = try self.allocateExpression(right),
        }};
    }

    fn parsePrattAssignment(self: *Parser, left: Expression) ParserError!Expression {
        const operator_token = self.advance();
        const value = try self.parseExpressionPrattPrec(.Assignment);
        
        return Expression{ .Binary = ast.BinaryExpression{
            .left = try self.allocateExpression(left),
            .operator = operator_token.lexeme,
            .right = try self.allocateExpression(value),
        }};
    }

    fn parsePrattCall(self: *Parser, left: Expression) ParserError!Expression {
        _ = self.advance(); // consume '('
        var arguments = std.ArrayList(*Expression){ .items = &.{}, .capacity = 0 };
        // CRITICAL FIX: Do NOT defer arguments.deinit - arena will handle cleanup

        if (!self.check(.RightParen)) {
            while (true) {
                const arg = self.parseExpression() catch |arg_err| {
                    _ = self.reportErrorWithContext("Error parsing function call argument", "parsePrattCall") catch {};
                    return arg_err;
                };
                
                const arg_ptr = self.arena_allocator.create(Expression) catch |alloc_err| {
                    std.debug.print("MEMORY ERROR: Failed to allocate argument expression: {any}\n", .{alloc_err});
                    return ParserError.OutOfMemory;
                };
                
                arg_ptr.* = arg;
                arguments.append(self.arena_allocator, arg_ptr) catch |append_err| {
                    std.debug.print("MEMORY ERROR: Failed to append argument to list: {any}\n", .{append_err});
                    return ParserError.OutOfMemory;
                };

                if (!self.match(.Comma)) break;
            }
        }

        _ = self.consume(.RightParen, "Expected ')' after arguments") catch |paren_err| {
            _ = self.reportErrorWithContext("Missing closing parenthesis for function call", "parsePrattCall") catch {};
            return paren_err;
        };
        
        // CRITICAL MEMORY SAFETY FIX: Use the arguments list directly, no need to copy
        // Transfer ownership to the function call expression

        // CRITICAL MEMORY SAFETY FIX: Safe function pointer allocation
        const function_ptr = self.allocateExpression(left) catch |alloc_err| {
            std.debug.print("MEMORY ERROR: Failed to allocate function expression: {any}\n", .{alloc_err});
            return ParserError.OutOfMemory;
        };
        
        return Expression{ .Call = .{
            .function = function_ptr,
            .arguments = arguments,
        }};
    }

    fn parsePrattMemberAccess(self: *Parser, left: Expression) ParserError!Expression {
        
        _ = self.advance(); // consume '.'
        // FIXED: Better method name validation with debugging
        if (!self.check(.Identifier) and !self.isKeywordAllowedAsMethodName()) {
            std.debug.print("DEBUG: Expected method name after '.', found: {any}\n", .{self.peek().kind});
            return ParserError.UnexpectedToken;
        }
        const property = self.advance().lexeme;
        
        // If next token is '(', treat as method call
        if (self.check(.LeftParen)) {
            _ = self.advance(); // '('
            var arguments = std.ArrayList(*Expression){ .items = &.{}, .capacity = 0 };
            defer arguments.deinit(self.arena_allocator);
            
            if (!self.check(.RightParen)) {
                while (true) {
                    // FIXED: Better error handling for method arguments
                    const arg = self.parseExpression() catch |parse_err| {
                        // std.debug.print("DEBUG: Failed to parse method argument: {any}\n", .{parse_err});
                        return parse_err;
                    };
                    const arg_ptr = self.arena_allocator.create(Expression) catch |alloc_err| {
                        std.debug.print("MEMORY ERROR: Failed to allocate method argument expression: {any}\n", .{alloc_err});
                        return ParserError.OutOfMemory;
                    };
                    
                    arg_ptr.* = arg;
                    arguments.append(self.arena_allocator, arg_ptr) catch |append_err| {
                        std.debug.print("MEMORY ERROR: Failed to append method argument to list: {any}\n", .{append_err});
                        return ParserError.OutOfMemory;
                    };
                    
                    if (!self.match(.Comma)) break;
                }
            }
            _ = try self.consume(.RightParen, "Expected ')' after arguments");
            
            // FIXED: Use arena allocator for arguments_copy to prevent memory leaks
            var arguments_copy = std.ArrayList(*Expression){ .items = &.{}, .capacity = 0 };
            for (arguments.items) |arg| {
                try arguments_copy.append(self.arena_allocator, arg);
            }
            
            return Expression{ .MethodCall = try self.allocateMethodCall(ast.MethodCallExpression{
                .object = try self.allocateExpression(left),
                .method_name = property,
                .arguments = arguments_copy,
            })};
        }
        
        // plain member access
        return Expression{ .MemberAccess = try self.allocateMemberAccess(ast.MemberAccessExpression{
            .object = try self.allocateExpression(left),
            .property = property,
        })};
    }

    fn parsePrattArrayAccess(self: *Parser, left: Expression) ParserError!Expression {
        _ = self.advance(); // consume '['
        
        // CRITICAL MEMORY SAFETY FIX: Validate index expression parsing
        const index = self.parseExpression() catch |index_err| {
            _ = self.reportErrorWithContext("Error parsing array index expression", "parsePrattArrayAccess") catch {};
            return index_err;
        };
        
        _ = self.consume(.RightBracket, "Expected ']'") catch |bracket_err| {
            _ = self.reportErrorWithContext("Missing closing bracket for array access", "parsePrattArrayAccess") catch {};
            return bracket_err;
        };
        
        // CRITICAL MEMORY SAFETY FIX: Safe memory allocation with error handling
        const array_ptr = self.allocateExpression(left) catch |alloc_err| {
            std.debug.print("MEMORY ERROR: Failed to allocate array expression: {any}\n", .{alloc_err});
            return ParserError.OutOfMemory;
        };
        
        const index_ptr = self.allocateExpression(index) catch |alloc_err| {
            std.debug.print("MEMORY ERROR: Failed to allocate index expression: {any}\n", .{alloc_err});
            return ParserError.OutOfMemory;
        };
        
        return Expression{ .ArrayAccess = ast.ArrayAccessExpression{
            .array = array_ptr,
            .index = index_ptr,
        }};
    }

    // Prefix increment/decrement: ++x, --x
    fn parsePrattPrefixIncrement(self: *Parser) ParserError!Expression {
        const operator_token = self.advance(); // consume ++ or --
        const operator = operator_token.lexeme;
        const operand = try self.parseExpressionPrattPrec(.Unary);
        
        return Expression{ .Unary = try self.allocateUnaryExpression(ast.UnaryExpression{
            .operator = operator,
            .operand = try self.allocateExpression(operand),
        })};
    }

    // Postfix increment/decrement: x++, x--
    fn parsePrattPostfixIncrement(self: *Parser, left: Expression) ParserError!Expression {
        const operator_token = self.advance(); // consume ++ or --
        const operator = operator_token.lexeme;
        
        return Expression{ .Unary = try self.allocateUnaryExpression(ast.UnaryExpression{
            .operator = operator,
            .operand = try self.allocateExpression(left),
        })};
    }

    fn parsePrattStructLiteral(self: *Parser, left: Expression) ParserError!Expression {
        // left should be an identifier representing the struct name
        const struct_name = switch (left) {
            .Identifier => |name| name,
            else => return ParserError.UnexpectedToken,
        };

        _ = self.advance(); // consume '{'

        var fields = ArrayList(ast.FieldInitializer){};

        if (!self.check(.RightBrace)) {
            while (true) {
                if (!self.check(.Identifier)) {
                    return ParserError.UnexpectedToken;
                }

                const field_name = self.advance().lexeme;
                _ = try self.consume(.Colon, "Expected ':' after field name");
                const value = try self.parseExpression();

                const value_ptr = try self.arena_allocator.create(Expression);
                value_ptr.* = value;

                try fields.append(self.allocator, ast.FieldInitializer{
                    .field_name = field_name,
                    .value = value_ptr,
                });

                if (!self.match(.Comma)) break;
            }
        }

        _ = try self.consume(.RightBrace, "Expected '}'");

        // Return the new StructExpression type
        return Expression{ .Struct = try self.allocateStructExpression(ast.StructExpression{
            .struct_name = struct_name,
            .fields = fields,
        })};
    }

    pub fn parseProgram(self: *Parser) ParserError!Program {
        var program = Program.init(self.arena_allocator);
        errdefer {
            // Skip program.deinit() - arena cleanup will handle all allocations
            // No need to deinit arena here - it will be handled by parser.deinit()
        }
        
        var statement_count: usize = 0;
        const max_statements = 10000; // Prevent infinite loops/excessive memory usage
        
        while (!self.isAtEnd() and statement_count < max_statements) {
            statement_count += 1;
            
            // Skip newlines, semicolons, and comments
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
                    _ = self.reportErrorWithContext("Error parsing package declaration", "parseProgram") catch {};
                    self.synchronize();
                    if (err == ParserError.OutOfMemory) return err;
                }
                continue;
            }

            // Parse import statement with error recovery
            if (self.check(.Yeet)) {
                if (self.parseImportStatement()) |import_stmt| {
                    // Add import as a regular statement
                    const stmt = Statement{ .Import = import_stmt };
                    const stmt_ptr = self.arena_allocator.create(Statement) catch {
                        _ = self.reportErrorWithContext("Out of memory allocating import statement", "parseProgram") catch {};
                        return ParserError.OutOfMemory;
                    };
                    stmt_ptr.* = stmt;
                    
                    program.statements.append(self.arena_allocator, stmt_ptr) catch {
                        _ = self.reportErrorWithContext("Out of memory adding import statement", "parseProgram") catch {};
                        return ParserError.OutOfMemory;
                    };
                } else |err| {
                    _ = self.reportErrorWithContext("Error parsing import statement", "parseProgram") catch {};
                    self.synchronize();
                    if (err == ParserError.OutOfMemory) return err;
                }
                continue;
            }

            // Parse regular statements with enhanced error handling and recovery
            // std.debug.print("DEBUG: About to parse statement at token: {any}\n", .{if (self.current < self.tokens.len) self.tokens[self.current].kind else .Eof});
            if (self.parseStatement()) |stmt| {
                // std.debug.print("DEBUG: Successfully parsed statement: {any}\n", .{stmt});
                // CRITICAL MEMORY SAFETY FIX: Add bounds checking and validation
                const stmt_ptr = self.arena_allocator.create(Statement) catch |alloc_err| {
                    std.debug.print("MEMORY ERROR: Failed to allocate statement in parseProgram: {any}\n", .{alloc_err});
                    _ = self.reportErrorWithContext("Out of memory allocating statement", "parseProgram") catch {};
                    return ParserError.OutOfMemory;
                };

                stmt_ptr.* = stmt;
                
                // Consume optional semicolon after statement
                if (self.check(.Semicolon)) {
                    _ = self.advance();
                }
                
                program.statements.append(self.arena_allocator, stmt_ptr) catch {
                    _ = self.reportErrorWithContext("Out of memory adding statement to program", "parseProgram") catch {};
                    return ParserError.OutOfMemory;
                };
            } else |err| {
                const error_token = if (self.current < self.tokens.len) self.tokens[self.current] else self.tokens[self.tokens.len - 1];
                // std.debug.print("DEBUG: Parse statement error: {any} at token: {any}\n", .{err, error_token.kind});
                _ = self.reportErrorAtToken(error_token, "Failed to parse statement") catch {};
                
                // Use enhanced error recovery strategy
                self.recoverFromStatementError();
                
                if (err == ParserError.OutOfMemory) return err;
                // Continue parsing after error recovery
            }
        }
        
        if (statement_count >= max_statements) {
            _ = self.reportErrorWithContext("Maximum number of statements exceeded", "parseProgram") catch {};
            return ParserError.InvalidSyntax;
        }

        // Report error recovery statistics if any errors occurred
        if (self.error_recovery_stats.total_errors > 0) {
            self.error_recovery_stats.reportStats();
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
                .offset = @intCast(token.offset),
            };
        }
        return null;
    }

    fn getSourceLocationForToken(self: *Parser, token: Token) ast.SourceLocation {
        return ast.SourceLocation{
            .file = self.file_path,
            .line = @intCast(token.line),
            .column = @intCast(token.column),
            .offset = @intCast(token.offset),
        };
    }

    // Enhanced error reporting with recovery context
    fn reportError(self: *Parser, message: []const u8) ParserError {
        return self.reportErrorWithContext(message, "parser");
    }
    
    fn reportErrorWithContext(self: *Parser, message: []const u8, context: []const u8) ParserError {
        _ = message;
        _ = context;
        // Error reporting disabled for clean test output
        self.had_error = true;
        return ParserError.SyntaxError;
    }

    fn reportErrorAtToken(self: *Parser, token: Token, message: []const u8) ParserError {
        _ = token;
        _ = message;
        // Error reporting disabled for clean test output
        self.had_error = true;
        return ParserError.SyntaxError;
    }

    // Enhanced recovery parsing with multiple strategies
    fn synchronize(self: *Parser) void {
        _ = self.reportErrorWithContext("Synchronizing parser after error", "synchronize") catch {};
        self.syncToSemicolon();
    }

    /// Sync to semicolon algorithm - the primary error recovery mechanism
    /// Scans forward until finding a semicolon or statement-starting token
    fn syncToSemicolon(self: *Parser) void {
        var tokens_skipped: usize = 0;
        const max_skip = 50; // Prevent infinite loops
        
        self.error_recovery_stats.semicolon_recoveries += 1;
        
        // Skip the current erroneous token
        if (!self.isAtEnd()) {
            _ = self.advance();
            tokens_skipped += 1;
        }
        
        while (!self.isAtEnd() and tokens_skipped < max_skip) {
            const current_token = self.peek();
            
            // Stop at semicolon - this is our sync point
            if (current_token.kind == .Semicolon) {
                _ = self.advance(); // consume the semicolon
                self.error_recovery_stats.tokens_skipped += tokens_skipped;
                if (builtin.mode == .Debug) {
                    // Recovered at semicolon
                }
                return;
            }
            
            // Stop at newline (statement separator in many cases)
            if (current_token.kind == .Newline) {
                _ = self.advance(); // consume the newline
                self.error_recovery_stats.tokens_skipped += tokens_skipped;
                if (builtin.mode == .Debug) {
                    // Recovered at newline
                }
                return;
            }
            
            // Stop at statement-starting keywords
            switch (current_token.kind) {
                .Slay, .Sus, .Facts, .Squad, .Collab, .Vibe, .Yeet, .Ready, .Lowkey, 
                .Periodt, .Flex, .Bestie, .Ghosted, .Simp, .Later, .Impl, .BeLike, 
                .Stan, .Match, .VibeCheck => {
                    // Don't consume these - let the next parsing cycle handle them
                    self.error_recovery_stats.tokens_skipped += tokens_skipped;
                    if (builtin.mode == .Debug) {
                        // Recovered at statement keyword
                    }
                    return;
                },
                else => {}
        }
            
            // Stop at block delimiters that might indicate recovery points
            switch (current_token.kind) {
                .RightBrace, .RightParen, .RightBracket => {
                    // Don't consume these - they might be needed for proper parsing
                    self.error_recovery_stats.tokens_skipped += tokens_skipped;
                    if (builtin.mode == .Debug) {
                        // Recovered at delimiter
                    }
                    return;
                },
                else => {}
        }
            
            _ = self.advance();
            tokens_skipped += 1;
        }
        
        self.error_recovery_stats.tokens_skipped += tokens_skipped;
        if (tokens_skipped >= max_skip) {
            _ = self.reportErrorWithContext("Maximum tokens skipped during error recovery", "syncToSemicolon") catch {};
        }
    }

    /// Sync to specific token - for targeted recovery
    fn syncToToken(self: *Parser, target: TokenKind) void {
        var tokens_skipped: usize = 0;
        const max_skip = 30;
        
        while (!self.isAtEnd() and tokens_skipped < max_skip) {
            if (self.check(target)) {
                return; // Found target, don't consume it
            }
            
            _ = self.advance();
            tokens_skipped += 1;
        }
    }

    /// Sync to any of multiple target tokens
    fn syncToAnyToken(self: *Parser, targets: []const TokenKind) void {
        var tokens_skipped: usize = 0;
        const max_skip = 30;
        
        while (!self.isAtEnd() and tokens_skipped < max_skip) {
            for (targets) |target| {
                if (self.check(target)) {
                    return; // Found one of the targets
                }
            }
            
            _ = self.advance();
            tokens_skipped += 1;
        }
    }

    /// Sync to matching delimiter (e.g., find closing brace for opening brace)
    fn syncToMatchingDelimiter(self: *Parser, open: TokenKind, close: TokenKind) void {
        var depth: i32 = 1; // Start with 1 since we're already inside
        var tokens_skipped: usize = 0;
        const max_skip = 100;
        
        while (!self.isAtEnd() and depth > 0 and tokens_skipped < max_skip) {
            const current = self.peek();
            
            if (current.kind == open) {
                depth += 1;
            } else if (current.kind == close) {
                depth -= 1;
            }
            
            _ = self.advance();
            tokens_skipped += 1;
        }
    }

    /// Enhanced recovery that tries multiple strategies
    fn recoverToNext(self: *Parser, target_tokens: []const TokenKind) void {
        // First try to sync to semicolon
        const start_pos = self.current;
        self.syncToSemicolon();
        
        // If we didn't make progress, try syncing to target tokens
        if (self.current == start_pos) {
            self.syncToAnyToken(target_tokens);
        }
    }

    /// Recovery for expression parsing - tries to find expression boundaries
    fn recoverFromExpressionError(self: *Parser) void {
        const expr_recovery_tokens = [_]TokenKind{
            .Semicolon, .Newline, .Comma, .RightParen, .RightBrace, .RightBracket,
            .Plus, .Minus, .Star, .Slash, .Equal, .EqualEqual, .BangEqual,
            .Less, .LessEqual, .Greater, .GreaterEqual
        };
        
        self.syncToAnyToken(&expr_recovery_tokens);
    }

    /// Recovery for statement parsing - tries to find statement boundaries
    fn recoverFromStatementError(self: *Parser) void {
        self.error_recovery_stats.total_errors += 1;
        self.error_recovery_stats.statement_recoveries += 1;
        
        // First try semicolon sync
        self.syncToSemicolon();
        
        // If no semicolon found, try to find next statement
        if (self.isAtEnd()) return;
        
        const stmt_start_tokens = [_]TokenKind{
            .Slay, .Sus, .Facts, .Squad, .Collab, .Ready, .Lowkey,
            .Periodt, .Flex, .Bestie, .Later, .Impl, .Stan, .Match
        };
        
        // Attempting additional statement recovery
        self.syncToAnyToken(&stmt_start_tokens);
    }

    fn parsePackageDeclaration(self: *Parser) ParserError!ast.PackageDeclaration {
        _ = try self.consume(.Vibe, "Expected 'vibe'");
        
        // Accept either Identifier or MainCharacter for package names
        if (!self.check(.Identifier) and !self.check(.MainCharacter)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.peek().lexeme;
        _ = self.advance();
        
        return ast.PackageDeclaration{
            .name = name,
            .version = null,
        };
    }

    fn parseImportStatement(self: *Parser) ParserError!ast.ImportStatement {
        _ = try self.consume(.Yeet, "Expected 'yeet'");
        
        // Check if this is a selective import: yeet { items } from "module"
        if (self.match(.LeftBrace)) {
            return self.parseSelectiveImport();
        }
        
        // Parse regular import(s)
        return self.parseRegularImport();
    }

    fn parseSelectiveImport(self: *Parser) ParserError!ast.ImportStatement {
        // Parse: yeet { item1, item2 as alias, item3 } from "module"
        var import_stmt = ast.ImportStatement.init(self.allocator, "");
        
        // Parse selective items
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (!self.check(.Identifier)) {
                _ = self.reportErrorWithContext("Expected identifier in selective import", "parseSelectiveImport") catch {};
                return ParserError.UnexpectedToken;
            }
            
            const item_name = self.advance().lexeme;
            var item_alias: ?[]const u8 = null;
            
            // Check for item-level alias: item as alias
            if (self.match(.As)) {
                if (!self.check(.Identifier)) {
                    _ = self.reportErrorWithContext("Expected identifier after 'as' in selective import", "parseSelectiveImport") catch {};
                    return ParserError.UnexpectedToken;
                }
                item_alias = self.advance().lexeme;
            }
            
            try import_stmt.addSelectiveItem(self.allocator, item_name, item_alias);
            
            // Handle comma separator or end of list
            if (self.match(.Comma)) {
                continue;
            } else if (self.check(.RightBrace)) {
                break;
            } else {
                _ = self.reportErrorWithContext("Expected ',' or '}' in selective import", "parseSelectiveImport") catch {};
                return ParserError.UnexpectedToken;
            }
        }
        
        _ = try self.consume(.RightBrace, "Expected '}' after selective import items");
        _ = try self.consume(.From, "Expected 'from' after selective import items");
        
        if (!self.check(.StringLiteral) and !self.check(.String)) {
            _ = self.reportErrorWithContext("Expected string literal after 'from'", "parseSelectiveImport") catch {};
            return ParserError.UnexpectedToken;
        }
        
        const path_token = self.advance();
        const path = self.extractStringLiteral(path_token.lexeme);
        import_stmt.path = path;
        
        return import_stmt;
    }

    fn parseRegularImport(self: *Parser) ParserError!ast.ImportStatement {
        // Parse regular imports: single, multiple, or aliased
        if (!self.check(.StringLiteral) and !self.check(.String)) {
            _ = self.reportErrorWithContext("Expected string literal in import statement", "parseRegularImport") catch {};
            return ParserError.UnexpectedToken;
        }
        
        const first_path_token = self.advance();
        var first_path = self.extractStringLiteral(first_path_token.lexeme);
        
        // Check for version specification: "module@^1.0.0"
        var version: ?[]const u8 = null;
        if (std.mem.indexOf(u8, first_path, "@")) |at_index| {
            version = first_path[at_index + 1..];
            first_path = first_path[0..at_index];
        }
        
        var import_stmt = ast.ImportStatement.init(self.allocator, first_path);
        import_stmt.version = version;
        
        // Check for multiple imports: "mod1", "mod2", "mod3"
        while (self.match(.Comma)) {
            if (!self.check(.StringLiteral) and !self.check(.String)) {
                _ = self.reportErrorWithContext("Expected string literal after comma in import statement", "parseRegularImport") catch {};
                return ParserError.UnexpectedToken;
            }
            
            const path_token = self.advance();
            var path = self.extractStringLiteral(path_token.lexeme);
            
            // Handle version in multiple imports
            if (std.mem.indexOf(u8, path, "@")) |at_index| {
                path = path[0..at_index]; // Strip version for multiple imports (not currently supported)
            }
            
            try import_stmt.addMultiplePath(self.allocator, path);
        }
        
        // Handle alias: "module" as alias (only for single imports)
        if (import_stmt.multiple_paths.items.len == 0 and self.match(.As)) {
            if (!self.check(.Identifier)) {
                _ = self.reportErrorWithContext("Expected identifier after 'as' in import statement", "parseRegularImport") catch {};
                return ParserError.UnexpectedToken;
            }
            import_stmt.alias = self.advance().lexeme;
        }
        
        return import_stmt;
    }
    
    fn extractStringLiteral(self: *Parser, lexeme: []const u8) []const u8 {
        _ = self; // Mark parameter as used
        if (lexeme.len >= 2 and lexeme[0] == '"' and lexeme[lexeme.len-1] == '"') {
            return lexeme[1..lexeme.len-1]; // Remove quotes
        }
        return lexeme;
    }
    
    /// Create an empty expression for empty statements (standalone semicolons)
    fn createEmptyExpression(self: *Parser) Expression {
        _ = self; // Suppress unused parameter warning
        return Expression{ .Literal = ast.Literal{ .Nil = {} } };
    }

    fn parseStatement(self: *Parser) ParserError!Statement {
        // Check for infinite loop
        try self.checkInfiniteLoop();
        
        // Skip comments at statement level
        while (self.check(.LineComment) or self.check(.BlockComment) or self.check(.Comment)) {
            _ = self.advance();
        }
        
        // DEBUG: parseStatement entry logging removed for production
        
        // Handle empty statements (standalone semicolons)
        if (self.check(.Semicolon)) {
            _ = self.advance();
            self.resetLoopCounter(); // Reset after advancing
            return Statement{ .Expression = self.createEmptyExpression() };
        }
        
        // CRITICAL FIX: Block statement parsing - handle standalone braces
        // This prevents complex expressions from being misinterpreted
        if (self.check(.LeftBrace)) {
            return try self.parseBlockStatement();
        }
        
        // Function declaration (slay) with enhanced error recovery
        if (self.check(.Slay)) {
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                const error_token = if (self.current < self.tokens.len) self.tokens[self.current] else self.tokens[self.tokens.len - 1];
                _ = self.reportErrorAtToken(error_token, "Error parsing function statement") catch {};

                // Try to recover to the end of the function
                self.syncToMatchingDelimiter(.LeftBrace, .RightBrace);
                self.recoverFromStatementError();
                return parse_err;
            }};
        }
        
        // Variable declaration (sus/facts) with lookahead to distinguish from function calls
        if (self.check(.Sus)) {
            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
                const error_token = if (self.current < self.tokens.len) self.tokens[self.current] else self.tokens[self.tokens.len - 1];
                _ = self.reportErrorAtToken(error_token, "Error parsing variable declaration") catch {};
                
                // Sync to semicolon for variable declarations
                self.syncToSemicolon();
                return parse_err;
            }};
        }
        
        // facts can be either variable declaration or function call - lookahead to decide
        if (self.check(.Facts)) {
            // Lookahead to check if this is facts(args) or facts variable = value
            if (self.current + 1 < self.tokens.len and self.tokens[self.current + 1].kind == .LeftParen) {
                // This is facts(...) function call - parse as expression statement
                const expr = try self.parseExpression();
                const expr_ptr = try self.arena_allocator.create(Expression);
                expr_ptr.* = expr;
                return Statement{ .Expression = expr };
            } else {
                // This is facts variable = value - parse as variable declaration
                return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
                    const error_token = if (self.current < self.tokens.len) self.tokens[self.current] else self.tokens[self.tokens.len - 1];
                    _ = self.reportErrorAtToken(error_token, "Error parsing variable declaration") catch {};
                    
                    // Sync to semicolon for variable declarations
                    self.syncToSemicolon();
                    return parse_err;
                }};
            }
        }
        
        // Return statement (return/yolo/damn - multiple supported forms)
        if (self.match(.Return) or self.match(.Yolo) or self.match(.Damn) or self.matchIdentifier("return")) {
            return try self.parseReturnStatement();
        }
        
        // If statement (lowkey/ready)
        if (self.check(.Lowkey) or self.check(.Ready)) {
            return Statement{ .If = try self.parseIfStatement() };
        }
        
        // While statement (periodt/flex) or For statement (bestie)
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
            // std.debug.print("DEBUG: Parsing struct statement\n", .{});
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
            return try self.parseStanStatement();
        }

        // Match expression (match)
        if (self.check(.Match)) {
            const match_expr = try self.parseMatchExpression();
            const expr_ptr = try self.arena_allocator.create(Expression);

            expr_ptr.* = match_expr;
            return Statement{ .Expression = match_expr };
        }

        // Vibe check (switch)
        if (self.check(.VibeCheck)) {
            return try self.parseVibeCheckStatement();
        }

        // Select statement
        if (self.check(.Select)) {
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
            // DEBUG: Short declaration detection removed
            return try self.parseShortDeclaration();
        }

        // Assignment statement
        if (self.isAssignment()) {
            // DEBUG: Assignment detection removed
            return try self.parseAssignmentStatement();
        }
        
        // Expression statement with enhanced error handling for complex expressions
        // CRITICAL FIX: Prevent complex expressions with braces from being parsed as function names
        const expr = self.parseExpression() catch |parse_err| {
            // Enhanced error context for complex expression parsing
            _ = self.reportErrorWithContext("Error parsing complex expression statement - check for misplaced braces or operator precedence issues", "parseStatement") catch {};
            self.synchronize();
            return parse_err;
        };
        
        // CRITICAL MEMORY SAFETY FIX: Do NOT allocate expr_ptr here - Expression is used directly
        // The arena_allocator handles memory management, but we shouldn't double-allocate
        
        // DEBUG: Successful parsing - logging removed
        return Statement{ .Expression = expr };
    }

    // CRITICAL FIX: Parse standalone block statements
    // This handles cases where braces follow expressions but should be separate statements
    fn parseBlockStatement(self: *Parser) ParserError!Statement {
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        var statements = std.ArrayList(*Statement){ .items = &.{}, .capacity = 0 };
        // CRITICAL FIX: Do NOT defer statements.deinit - the ArrayList will be owned by BlockStatement
        
        // Parse statements within the block
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            // Skip newlines and comments
            if (self.match(.Newline) or self.match(.LineComment) or self.match(.BlockComment)) {
                continue;
            }
            
            const stmt = try self.parseStatement();
            const stmt_ptr = try self.arena_allocator.create(Statement);

            stmt_ptr.* = stmt;
            
            try statements.append(self.allocator, stmt_ptr);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
        // Transfer ownership of the ArrayList to the BlockStatement
        return Statement{ .Block = ast.BlockStatement{
            .statements = statements,
        }};
    }

    fn parseFunctionStatement(self: *Parser) ParserError!FunctionStatement {
        _ = try self.consume(.Slay, "Expected 'slay'");
        if (!self.check(.Identifier) and !self.check(.MainCharacter)) {
            _ = self.reportErrorWithContext("Expected function name after 'slay'", "parseFunctionStatement") catch {};
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // Validate function name length
        if (name.len == 0 or name.len > 255) {
            _ = self.reportErrorWithContext("Invalid function name length", "parseFunctionStatement") catch {};
            return ParserError.InvalidFunction;
        }
        
        var func = FunctionStatement.init(self.arena_allocator, name);
        
        // Parse generic type parameters <T, U>
        if (self.match(.Less) or self.match(.LeftAngle)) {
            while (!self.check(.Greater) and !self.check(.RightAngle) and !self.check(.RightShift) and !self.isAtEnd()) {
                if (self.check(.Identifier)) {
                    const param_name = self.advance().lexeme;
                    var type_param = ast.TypeParameter{
                        .name = param_name,
                        .constraints = .empty,
                    };
                    
                    // Parse constraints (T: SomeInterface)
                    if (self.match(.Colon)) {
                        while (!self.check(.Comma) and !self.check(.Greater) and !self.check(.RightAngle) and !self.check(.RightShift)) {
                            const constraint = try self.parseType();
                            try type_param.constraints.append(self.allocator, constraint);
                            if (!self.match(.Plus)) break;
                        }
                    }
                    
                    try func.type_parameters.append(self.allocator, type_param);
                }
                
                if (!self.match(.Comma)) break;
            }
            
            // Handle both > and >> for closing generic parameters
            if (!self.match(.Greater) and !self.match(.RightAngle) and !self.match(.RightShift)) {
                return ParserError.MissingToken;
            }
        }
        
        // Parse parameters
        _ = try self.consume(.LeftParen, "Expected '(' after function name");
        
        if (!self.check(.RightParen)) {
            while (true) {
                const param = try self.parseParameter();
                try func.parameters.append(self.arena_allocator, param);
                
                if (!self.match(.Comma)) break;
            }
        }
        
        _ = try self.consume(.RightParen, "Expected ')' after parameters");
        
        // Parse return type
        if (!self.check(.LeftBrace)) {
            func.return_type = try self.parseType();
        }
        
        // Parse body
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        self.in_function = true;
        defer { self.in_function = false; }
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            // Skip newlines and comments
            while (self.match(.Newline) or self.match(.LineComment) or self.match(.BlockComment) or self.match(.Comment)) {
                // continue
            }
            
            // Break if we hit the closing brace
            if (self.check(.RightBrace)) break;
            
            // DEBUG: Function body statement parsing - logging removed
            // std.debug.print("DEBUG: Parsing function body statement at token: {any}\n", .{if (self.current < self.tokens.len) self.tokens[self.current].kind else .Eof});
            const stmt = try self.parseStatement();
            // std.debug.print("DEBUG: Successfully parsed function body statement\n", .{});
            const stmt_ptr = try self.arena_allocator.create(Statement); 

            stmt_ptr.* = stmt;
            
            // Consume optional semicolon or newline after statement in function body
            while (self.match(.Semicolon) or self.match(.Newline)) {
                // continue consuming
            }
            
            try func.body.append(self.arena_allocator, stmt_ptr);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
        return func;
    }

    fn parseLetStatement(self: *Parser) ParserError!LetStatement {
        const is_mutable = self.match(.Sus); // sus = mutable, facts = immutable
        if (!is_mutable) {
            _ = self.match(.Facts);
        }
        
        // CURSED syntax: sus <identifier> <type> = <expr>
        // First parse the identifier (variable name), then optional type
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // Parse optional type annotation after identifier
        var var_type: ?Type = null;
        if (self.checkBasicType() or self.check(.Identifier) or self.check(.Slay) or self.check(.At)) {
            var_type = try self.parseType();
        }
        
        var let_stmt = LetStatement{
            .name = name,
            .var_type = var_type,
            .type_annotation = var_type,
            .initializer = null,
            .is_mutable = is_mutable,
        };
        
        // Parse initializer
        if (self.match(.Equal) or self.match(.ColonEqual)) {
            const init_expr = try self.parseExpression();
            const init_ptr = try self.arena_allocator.create(Expression);
    
            init_ptr.* = init_expr;
            let_stmt.initializer = init_ptr;
        }
        
        return let_stmt;
    }

    fn parseParameter(self: *Parser) ParserError!Parameter {
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // CRITICAL MEMORY SAFETY FIX: Validate name is not null/empty
        if (name.len == 0) {
            _ = self.reportErrorWithContext("Empty parameter name", "parseParameter") catch {};
            return ParserError.UnexpectedToken;
        }
        
        // Skip 'drip' modifier if present (invalid syntax from old tests)
        if (self.check(.Identifier) and std.mem.eql(u8, self.peek().lexeme, "drip")) {
            _ = self.advance(); // consume 'drip'
        }
        
        // Parse type (optional for parameters in CURSED, defaults to 'auto')
        var param_type: Type = Type{ .Basic = ast.BasicType.Auto };
        if (self.checkType()) {
            param_type = self.parseType() catch blk: {
                _ = self.reportErrorWithContext("Error parsing parameter type", "parseParameter") catch {};
                // Use auto type as fallback for recovery
                break :blk Type{ .Basic = ast.BasicType.Auto };
            };
        }
        
        var param = Parameter{
            .name = name,
            .param_type = param_type,
            .is_mutable = false,
            .default_value = null,
        };

        // Parse default value with memory safety
        if (self.match(.Equal)) {
            const default_expr = self.parseExpression() catch |expr_err| {
                _ = self.reportErrorWithContext("Error parsing parameter default value", "parseParameter") catch {};
                return expr_err;
            }; 
            
            const default_ptr = self.arena_allocator.create(Expression) catch |alloc_err| {
                std.debug.print("MEMORY ERROR: Failed to allocate default value expression: {any}\n", .{alloc_err});
                return ParserError.OutOfMemory;
            }; 

            default_ptr.* = default_expr; 
            param.default_value = self.expressionToAnyopaque(default_ptr) catch |conv_err| {
                _ = self.reportErrorWithContext("Error converting default value", "parseParameter") catch {};
                return conv_err;
            };
        }
        
        return param;
    }

    fn isBasicTypeName(self: *Parser, name: []const u8) bool {
        _ = self; // unused parameter
        return std.mem.eql(u8, name, "normie") or
               std.mem.eql(u8, name, "tea") or
               std.mem.eql(u8, name, "txt") or
               std.mem.eql(u8, name, "sip") or
               std.mem.eql(u8, name, "smol") or
               std.mem.eql(u8, name, "mid") or
               std.mem.eql(u8, name, "thicc") or
               std.mem.eql(u8, name, "snack") or
               std.mem.eql(u8, name, "meal") or
               std.mem.eql(u8, name, "byte") or
               std.mem.eql(u8, name, "rune") or
               std.mem.eql(u8, name, "extra") or
               std.mem.eql(u8, name, "lit") or
               std.mem.eql(u8, name, "cap") or
               std.mem.eql(u8, name, "yikes");
    }

    fn parseType(self: *Parser) ParserError!ast.Type {
        // CRITICAL SYNTAX CHANGE: NEW POSTFIX ARRAY SYNTAX
        // Parse base type first, then handle postfix array/slice syntax: type[expr]
        
        // Start by parsing the base type (normie, tea, identifier, etc.)
        const base_type = if (self.checkBasicType()) 
            try self.parseBasicType()
        else if (self.check(.Identifier))
            blk: {
                const type_name = self.advance().lexeme;
                
                // Check for generic arguments
                if (self.check(.Less) or self.check(.LeftAngle)) {
                    break :blk try self.parseGenericType(type_name);
                }
                
                // Check if it's a known basic type name
                if (std.mem.eql(u8, type_name, "normie")) {
                    break :blk ast.Type{ .Basic = .Normie };
                } else if (std.mem.eql(u8, type_name, "tea")) {
                    break :blk ast.Type{ .Basic = .Tea };
                } else if (std.mem.eql(u8, type_name, "txt")) {
                    break :blk ast.Type{ .Basic = .Txt };
                } else if (std.mem.eql(u8, type_name, "sip")) {
                    break :blk ast.Type{ .Basic = .Sip };
                } else if (std.mem.eql(u8, type_name, "smol")) {
                    break :blk ast.Type{ .Basic = .Smol };
                } else if (std.mem.eql(u8, type_name, "mid")) {
                    break :blk ast.Type{ .Basic = .Mid };
                } else if (std.mem.eql(u8, type_name, "thicc")) {
                    break :blk ast.Type{ .Basic = .Thicc };
                } else if (std.mem.eql(u8, type_name, "snack")) {
                    break :blk ast.Type{ .Basic = .Snack };
                } else if (std.mem.eql(u8, type_name, "meal")) {
                    break :blk ast.Type{ .Basic = .Meal };
                } else if (std.mem.eql(u8, type_name, "byte")) {
                    break :blk ast.Type{ .Basic = .Byte };
                } else if (std.mem.eql(u8, type_name, "rune")) {
                    break :blk ast.Type{ .Basic = .Rune };
                } else if (std.mem.eql(u8, type_name, "extra")) {
                    break :blk ast.Type{ .Basic = .Extra };
                } else if (std.mem.eql(u8, type_name, "lit")) {
                    break :blk ast.Type{ .Basic = .Lit };
                } else if (std.mem.eql(u8, type_name, "cap")) {
                    break :blk ast.Type{ .Basic = .Cap };
                } else if (std.mem.eql(u8, type_name, "yikes")) {
                    break :blk ast.Type{ .Basic = .Yikes };
                } else {
                    // Custom struct or interface type
                    break :blk ast.Type{ .Struct = ast.StructType{
                        .name = type_name,
                        .fields = .empty,
                    }};
                }
            }
        else if (self.check(.Slay))
            // Function type: slay(params) return_type
            try self.parseFunctionType()
        else if (self.check(.LeftBracket))
            // ERROR: Old syntax []type is no longer supported
            return self.reportOldArraySyntaxError()
        else if (self.check(.At))
            // Pointer type: ඞT
            try self.parsePointerType()
        else
            return ParserError.InvalidType;
        
        // Now handle postfix array/slice syntax: type[expr][expr]...
        var result_type = base_type;
        while (self.match(.LeftBracket)) {
            if (self.check(.RightBracket)) {
                // Empty brackets type[] means slice (no size)
                _ = self.advance(); // consume ']'
                
                const element_type_ptr = try self.arena_allocator.create(ast.Type);
                element_type_ptr.* = result_type;
                
                result_type = ast.Type{ .Array = ast.ArrayType{
                    .ref_counted = ast.RefCounted.init(self.allocator),
                    .element_type = ast.RefPtr(ast.Type).init(element_type_ptr),
                    .size = null, // slice - no fixed size
                }};
            } else {
                // Parse the size/value expression
                if (self.check(.Number) or self.check(.Integer)) {
                    // Numeric size: type[5]
                    const size_token = self.advance();
                    const size = std.fmt.parseInt(usize, size_token.lexeme, 10) catch {
                        return ParserError.InvalidSyntax;
                    };
                    
                    _ = try self.consume(.RightBracket, "Expected ']' after array size");
                    
                    const element_type_ptr = try self.arena_allocator.create(ast.Type);
                    element_type_ptr.* = result_type;
                    
                    result_type = ast.Type{ .Array = ast.ArrayType{
                        .ref_counted = ast.RefCounted.init(self.allocator),
                        .element_type = ast.RefPtr(ast.Type).init(element_type_ptr),
                        .size = size,
                    }};
                } else if (self.check(.Identifier) and std.mem.eql(u8, self.peek().lexeme, "value")) {
                    // Special case: type[value] means slice
                    _ = self.advance(); // consume 'value'
                    _ = try self.consume(.RightBracket, "Expected ']' after 'value'");
                    
                    const element_type_ptr = try self.arena_allocator.create(ast.Type);
                    element_type_ptr.* = result_type;
                    
                    result_type = ast.Type{ .Array = ast.ArrayType{
                        .ref_counted = ast.RefCounted.init(self.allocator),
                        .element_type = ast.RefPtr(ast.Type).init(element_type_ptr),
                        .size = null, // slice - no fixed size
                    }};
                } else {
                    // Complex expression - for now just treat as slice
                    // In full implementation, we'd parse and evaluate the expression
                    _ = try self.parseExpression(); // consume the expression
                    _ = try self.consume(.RightBracket, "Expected ']' after array expression");
                    
                    const element_type_ptr = try self.arena_allocator.create(ast.Type);
                    element_type_ptr.* = result_type;
                    
                    result_type = ast.Type{ .Array = ast.ArrayType{
                        .ref_counted = ast.RefCounted.init(self.allocator),
                        .element_type = ast.RefPtr(ast.Type).init(element_type_ptr),
                        .size = null, // treat as slice for now
                    }};
                }
            }
        }
        
        // Handle generics for certain types after array processing
        if (self.check(.Less) or self.check(.LeftAngle)) {
            switch (result_type) {
                .Basic => |basic| {
                    switch (basic) {
                        .Yikes => {
                            // Simple parsing for yikes<T> - just consume tokens until >
                            _ = self.advance(); // consume < or LeftAngle
                            
                            // Parse the type argument (should be drip in most cases)
                            const inner_type = try self.parseType();
                            
                            // Consume the closing >
                            if (!self.match(.Greater) and !self.match(.RightAngle)) {
                                return ParserError.InvalidType;
                            }
                            
                            // For now, just return a generic type with yikes as base
                            return ast.Type{ .Generic = ast.GenericType{
                                .name = "yikes",
                                .type_arguments = blk: {
                                    var args = std.ArrayList(ast.Type){};
                                    try args.append(self.allocator, inner_type);
                                    break :blk args;
                                },
                                .constraints = std.ArrayList(ast.TypeConstraint){},
                            }};
                        },
                        else => {
                            // Other basic types don't support generics
                        }
                    }
                },
                else => {}
            }
        }
        
        // Handle error-returning function syntax: type yikes
        if (self.match(.Yikes)) {
            // Return the base type since the error capability is implicit
            return result_type;
        }
        
        return result_type;
    }
    
    /// Report helpful error message when old array syntax is used
    fn reportOldArraySyntaxError(self: *Parser) ParserError {
        _ = self.reportErrorWithContext(
            "Old array syntax '[]type' is no longer supported. Use new syntax 'type[value]' for slices or 'type[size]' for arrays.",
            "parseType"
        ) catch {};
        return ParserError.InvalidSyntax;
    }
    
    /// Check if identifier is a known CURSED type name
    fn isKnownTypeName(self: *Parser, name: []const u8) bool {
        _ = self; // suppress unused parameter
        return std.mem.eql(u8, name, "normie") or
               std.mem.eql(u8, name, "tea") or
               std.mem.eql(u8, name, "txt") or
               std.mem.eql(u8, name, "sip") or
               std.mem.eql(u8, name, "smol") or
               std.mem.eql(u8, name, "mid") or
               std.mem.eql(u8, name, "thicc") or
               std.mem.eql(u8, name, "snack") or
               std.mem.eql(u8, name, "meal") or
               std.mem.eql(u8, name, "byte") or
               std.mem.eql(u8, name, "rune") or
               std.mem.eql(u8, name, "extra") or
               std.mem.eql(u8, name, "lit") or
               std.mem.eql(u8, name, "cap") or
               std.mem.eql(u8, name, "yikes") or
               std.mem.eql(u8, name, "drip");
    }
    
    /// Get BasicType from type name string
    fn getBasicTypeFromName(self: *Parser, name: []const u8) ast.Type {
        _ = self; // suppress unused parameter
        if (std.mem.eql(u8, name, "normie")) {
            return ast.Type{ .Basic = .Normie };
        } else if (std.mem.eql(u8, name, "drip")) {
            return ast.Type{ .Basic = .Drip };
        } else if (std.mem.eql(u8, name, "tea")) {
            return ast.Type{ .Basic = .Tea };
        } else if (std.mem.eql(u8, name, "txt")) {
            return ast.Type{ .Basic = .Txt };
        } else if (std.mem.eql(u8, name, "sip")) {
            return ast.Type{ .Basic = .Sip };
        } else if (std.mem.eql(u8, name, "smol")) {
            return ast.Type{ .Basic = .Smol };
        } else if (std.mem.eql(u8, name, "mid")) {
            return ast.Type{ .Basic = .Mid };
        } else if (std.mem.eql(u8, name, "thicc")) {
            return ast.Type{ .Basic = .Thicc };
        } else if (std.mem.eql(u8, name, "snack")) {
            return ast.Type{ .Basic = .Snack };
        } else if (std.mem.eql(u8, name, "meal")) {
            return ast.Type{ .Basic = .Meal };
        } else if (std.mem.eql(u8, name, "byte")) {
            return ast.Type{ .Basic = .Byte };
        } else if (std.mem.eql(u8, name, "rune")) {
            return ast.Type{ .Basic = .Rune };
        } else if (std.mem.eql(u8, name, "extra")) {
            return ast.Type{ .Basic = .Extra };
        } else if (std.mem.eql(u8, name, "lit")) {
            return ast.Type{ .Basic = .Lit };
        } else if (std.mem.eql(u8, name, "cap")) {
            return ast.Type{ .Basic = .Cap };
        } else if (std.mem.eql(u8, name, "yikes")) {
            return ast.Type{ .Basic = .Yikes };
        } else {
            // Default to custom type if not recognized
            return ast.Type{ .Custom = name };
        }
    }
    
    /// Parse function types: slay(params) return_type
    fn parseFunctionType(self: *Parser) ParserError!ast.Type {
        _ = try self.consume(.Slay, "Expected 'slay'");
        _ = try self.consume(.LeftParen, "Expected '(' after 'slay'");
        
        var param_types = ArrayList(ast.Type){};
        
        // Parse parameter types
        while (!self.check(.RightParen) and !self.isAtEnd()) {
            const param_type = try self.parseType();
            try param_types.append(self.allocator, param_type);
            
            if (!self.match(.Comma)) break;
        }
        
        _ = try self.consume(.RightParen, "Expected ')' after function parameters");
        
        // Parse return type (optional)
        var return_type: ?*ast.Type = null;
        if (!self.check(.Newline) and !self.check(.Semicolon) and !self.isAtEnd() and !self.check(.RightBrace)) {
            return_type = try self.arena_allocator.create(ast.Type);
            return_type.?.* = try self.parseType();
        }
        
        var func_return_type: ?ast.RefPtr(ast.Type) = null;
        if (return_type) |rt| {
            func_return_type = ast.RefPtr(ast.Type).init(rt);
        }
        
        return ast.Type{ .Function = ast.FunctionType{
            .parameters = param_types,
            .return_type = func_return_type,
            .ref_counted = ast.RefCounted.init(self.allocator),
        }};
    }

    fn parsePointerType(self: *Parser) ParserError!ast.Type {
        _ = try self.consume(.At, "Expected 'ඞ' for pointer type");
        
        // Parse the target type
        const target_type_ptr = try self.arena_allocator.create(ast.Type);
        target_type_ptr.* = try self.parseType();
        
        return ast.Type{ .Pointer = ast.PointerType{
            .ref_counted = ast.RefCounted.init(self.allocator),
            .target_type = ast.RefPtr(ast.Type).init(target_type_ptr),
        }};
    }

    pub fn parseExpression(self: *Parser) ParserError!Expression {
        if (self.use_pratt) {
            return self.parseExpressionPratt();
        } else {
            return self.parseAssignment();
        }
    }

    /// Pratt parser implementation with precedence-based expression parsing
    pub fn parseExpressionPratt(self: *Parser) ParserError!Expression {
        return self.parseExpressionPrattPrec(.None);
    }

    /// Core Pratt parser algorithm - parses expressions based on precedence
    pub fn parseExpressionPrattPrec(self: *Parser, precedence: Prec) ParserError!Expression {
        const current_token = self.peek();
        const prefix_fn = self.getPrefixFunction(current_token.kind);
        if (prefix_fn == null) {
            return ParserError.UnexpectedToken;
        }

        var left = try prefix_fn.?(self);

        while (precedence.lessThan(self.getPrecedence(self.peek().kind))) {
            const infix_fn = self.getInfixFunction(self.peek().kind);
            if (infix_fn == null) {
                break;
            }
            left = try infix_fn.?(self, left);
        }

        return left;
    }

    // Helper methods for array and composite literal parsing
    
    fn parseArrayLiteralElements(self: *Parser) ParserError!Expression {
        var elements = ArrayList(Expression){};
        
        while (!self.check(.RightBracket) and !self.isAtEnd()) {
            const elem = try self.parseExpression();
            try elements.append(self.arena_allocator, elem);
            
            if (!self.match(.Comma)) break;
        }
        
        _ = try self.consume(.RightBracket, "Expected ']'");
        
        return Expression{ .Array = try self.allocateArrayExpression(ast.ArrayExpression{
            .elements = try self.convertExpressionsToPointers(&elements),
        })};
    }

    fn parseCompositeLiteral(self: *Parser) ParserError!Expression {
        // NEW SYNTAX: This function is no longer used with []Type{} syntax
        // Instead, composite literals use Type[value]{} syntax
        // This function is kept for backward compatibility and error handling
        return self.reportOldArraySyntaxError();
    }

    fn parseCompositeLiteralWithType(self: *Parser, element_type: ast.Type) ParserError!Expression {
        // CRITICAL: Type is used for semantic context but not in current AST structure
        _ = element_type;
        // CRITICAL: Safe consumption with error handling
        _ = self.consume(.LeftBrace, "Expected '{' after type in composite literal") catch |err| {
            // Provide context for debugging
            std.debug.print("DEBUG: parseCompositeLiteralWithType - failed to consume LeftBrace at position {d}, token: {any}\n", .{ self.current, if (self.current < self.tokens.len) self.tokens[self.current].kind else .Eof });
            return err;
        };
        
        // Parse elements in composite literal with memory safety
        var elements = ArrayList(Expression){};
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            const elem = self.parseExpression() catch |expr_err| {
                // CRITICAL: Clean error handling for expression parsing
                std.debug.print("DEBUG: parseCompositeLiteralWithType - failed to parse element at position {d}\n", .{self.current});
                return expr_err;
            };
            
            elements.append(self.arena_allocator, elem) catch |append_err| {
                std.debug.print("MEMORY ERROR: Failed to append element to composite literal: {any}\n", .{append_err});
                return ParserError.OutOfMemory;
            };
            
            if (!self.match(.Comma)) break;
        }
        
        // CRITICAL: Safe consumption with bounds check
        if (self.isAtEnd()) {
            std.debug.print("DEBUG: parseCompositeLiteralWithType - unexpected EOF, expected RightBrace\n", .{});
            return ParserError.UnexpectedEof;
        }
        
        _ = self.consume(.RightBrace, "Expected '}' after composite literal elements") catch |err| {
            std.debug.print("DEBUG: parseCompositeLiteralWithType - failed to consume RightBrace at position {d}\n", .{self.current});
            return err;
        };
        
        // Return as array expression (type info is implicit from context)
        return Expression{ .Array = try self.allocateArrayExpression(ast.ArrayExpression{
            .elements = self.convertExpressionsToPointers(&elements) catch |conv_err| {
                std.debug.print("MEMORY ERROR: Failed to convert expressions to pointers: {any}\n", .{conv_err});
                return ParserError.OutOfMemory;
            },
        })};
    }

    fn parseCompositeLiteralFromType(self: *Parser, type_name: []const u8) ParserError!Expression {
        _ = type_name; // Type info is not used in current AST structure but kept for semantic context
        
        // Parse Type[expr]{...} syntax
        // We already consumed the type identifier, now parse [expr]
        _ = try self.consume(.LeftBracket, "Expected '[' after type in composite literal");
        
        // Parse the array expression (size or 'value')
        var array_size: ?usize = null;
        if (self.check(.Number) or self.check(.Integer)) {
            const size_token = self.advance();
            array_size = std.fmt.parseInt(usize, size_token.lexeme, 10) catch null;
        } else if (self.check(.Identifier) and std.mem.eql(u8, self.peek().lexeme, "value")) {
            _ = self.advance(); // consume 'value'
            // array_size stays null for slice
        } else {
            // For now, treat complex expressions as slices
            _ = try self.parseExpression();
            // array_size stays null
        }
        
        _ = try self.consume(.RightBracket, "Expected ']' after array size");
        
        // Now parse the composite literal body {...}
        _ = try self.consume(.LeftBrace, "Expected '{' after array type");
        
        // Parse elements in composite literal
        var elements = ArrayList(Expression){};
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            const elem = try self.parseExpression();
            try elements.append(self.arena_allocator, elem);
            
            if (!self.match(.Comma)) break;
        }
        
        _ = try self.consume(.RightBrace, "Expected '}' after composite literal elements");
        
        // Return as array expression - type info is contextual
        return Expression{ .Array = try self.allocateArrayExpression(ast.ArrayExpression{
            .elements = try self.convertExpressionsToPointers(&elements),
        })};
    }

    fn parseAssignment(self: *Parser) ParserError!Expression {
        const expr = try self.parseOr();

        // CRITICAL FIX: Enhanced assignment operator precedence handling
        // This prevents expressions like "i + 1 { ... }" from being misinterpreted
        if (self.match(.Equal) or self.match(.PlusEqual) or self.match(.MinusEqual) or 
           self.match(.StarEqual) or self.match(.SlashEqual) or self.match(.PercentEqual)) {
            const operator = self.previous().lexeme;
            
            // CRITICAL: Ensure we don't parse assignment within complex expression contexts
            // that could be confused with function calls
            const value = try self.parseAssignment();
            
            // Convert to assignment expression (not statement)
            return Expression{ .Binary = .{
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
        var expr = try self.parseStringConcatenation();

        while (self.match(.Minus) or self.match(.Plus)) {
            const operator = self.previous().lexeme;
            const right = try self.parseStringConcatenation();
            
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = try self.allocateExpression(expr),
                .operator = operator,
                .right = try self.allocateExpression(right),
            }};
        }

        return expr;
    }

    fn parseStringConcatenation(self: *Parser) ParserError!Expression {
        var expr = try self.parseFactor();

        while (self.match(.PlusPlus) or (self.check(.Plus) and self.isStringExpression(expr))) {
            const operator = if (self.previous().kind == .PlusPlus) "++" else "+";
            const right = try self.parseFactor();
            
            expr = Expression{ .Binary = ast.BinaryExpression{
                .left = try self.allocateExpression(expr),
                .operator = operator,
                .right = try self.allocateExpression(right),
            }};
        }

        return expr;
    }

    fn isStringExpression(_: *Parser, expr: Expression) bool {
        switch (expr) {
            .String => return true,
            // Only treat identifiers as strings if we have clear evidence
            // Identifiers alone should not trigger string concatenation to avoid conflicts with arithmetic
            .Call => return true, // Could return a string
            .MemberAccess => return true, // Could be a string property
            else => return false,
        }
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
        if (self.match(.Bang) or self.match(.Minus) or self.match(.Star) or self.match(.Amp) or self.match(.At)) {
            const operator = self.previous().lexeme;
            const right = try self.parseUnary();
            
            return Expression{ .Unary = try self.allocateUnaryExpression(ast.UnaryExpression{
                .operator = operator,
                .operand = try self.allocateExpression(right),
            })};
        }

        // Handle yikes error creation
        if (self.match(.Yikes)) {
            const message_expr = try self.allocateExpression(try self.parseUnary());
            var code_expr: ?*Expression = null;
            
            // Optional error code
            if (self.match(.Comma)) {
                code_expr = try self.allocateExpression(try self.parseUnary());
            }
            
            return Expression{ .Yikes = ast.YikesExpression{
                .message = message_expr,
                .code = code_expr,
                .source_location = self.getCurrentSourceLocation(),
            }};
        }

        // Handle shook error propagation operator
        if (self.match(.Shook)) {
            const wrapped_expr = try self.allocateExpression(try self.parseUnary());
            var catch_handler: ?*Expression = null;
            
            // Optional immediate catch handler
            if (self.check(.LeftBrace)) {
                catch_handler = try self.allocateExpression(try self.parseExpression());
            }
            
            return Expression{ .Shook = ast.ShookExpression{
                .expression = wrapped_expr,
                .catch_handler = catch_handler,
            }};
        }
        
        // Handle fam panic recovery blocks
        if (self.match(.Fam)) {
            return try self.parseFamBlock();
        }

        return self.parseCall();
    }

    fn parseFamBlock(self: *Parser) ParserError!Expression {
        // Parse fam { try_body } catch(error_var) { catch_body } finally { finally_body }
        _ = try self.consume(.LeftBrace, "Expected '{' after 'fam'");
        
        var try_body = ArrayList(*Statement){};
        defer try_body.deinit(self.allocator);
        
        // Parse try body statements
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = try self.parseStatement();
            const stmt_ptr = try self.arena_allocator.create(Statement);

            stmt_ptr.* = stmt;
            try try_body.append(self.allocator, stmt_ptr);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}' after try body");
        
        var catch_handler: ?ast.FamExpression.CatchHandler = null;
        var finally_handler: ?ast.FamExpression.FinallyHandler = null;
        
        // Optional sus (catch) handler  
        if (self.match(.Sus)) {
            const error_variable = if (self.check(.Identifier))
                self.advance().lexeme
            else
                "error";
                
        _ = try self.consume(.LeftBrace, "Expected '{' for sus catch body");
            
            var catch_body = ArrayList(*Statement){};
            
            while (!self.check(.RightBrace) and !self.isAtEnd()) {
                if (self.match(.Newline)) continue;
                
                const stmt = try self.parseStatement();
                const stmt_ptr = try self.arena_allocator.create(Statement);

                stmt_ptr.* = stmt;
                try catch_body.append(self.allocator, stmt_ptr);
            }
            
        _ = try self.consume(.RightBrace, "Expected '}' after sus catch body");
            
            catch_handler = ast.FamExpression.CatchHandler{
                .error_variable = error_variable,
                .handler_body = catch_body,
            };
        }
        
        // Optional finally handler
        if (self.matchKeyword("finally")) {
        _ = try self.consume(.LeftBrace, "Expected '{' for finally body");
            
            var finally_body = ArrayList(*Statement){};
            
            while (!self.check(.RightBrace) and !self.isAtEnd()) {
                if (self.match(.Newline)) continue;
                
                const stmt = try self.parseStatement();
                const stmt_ptr = try self.arena_allocator.create(Statement);

                stmt_ptr.* = stmt;
                try finally_body.append(self.allocator, stmt_ptr);
            }
            
        _ = try self.consume(.RightBrace, "Expected '}' after finally body");
            
            finally_handler = ast.FamExpression.FinallyHandler{
                .finally_body = finally_body,
            };
        }
        
        return Expression{ .Fam = ast.FamExpression{
            .try_body = try_body,
            .catch_handler = catch_handler,
            .finally_handler = finally_handler,
        }};
    }

    fn parseCall(self: *Parser) ParserError!Expression {
        var expr = try self.parsePrimary();

        while (true) {
            if (self.match(.LeftParen)) {
                expr = try self.finishCall(expr);
            } else if (self.match(.Dot)) {
                // Allow identifiers and keywords that can be used as method names
                if (!self.check(.Identifier) and !self.isKeywordAllowedAsMethodName()) {
                    return ParserError.UnexpectedToken;
                }
                const property = self.advance().lexeme;
                
                // Check if this is a method call (identifier followed by parentheses)
                if (self.check(.LeftParen)) {
                    _ = self.advance(); // consume '('
                    
                    var arguments = ArrayList(*Expression){};
                    defer arguments.deinit(self.allocator);
                    
                    if (!self.check(.RightParen)) {
                        while (true) {
                            const arg = try self.parseExpression();
                            const arg_ptr = try self.arena_allocator.create(Expression);
                            arg_ptr.* = arg;
                            try arguments.append(self.allocator, arg_ptr);

                            if (!self.match(.Comma)) break;
                        }
                    }

                    _ = try self.consume(.RightParen, "Expected ')' after method arguments");

                    // CRITICAL FIX: Clone the arguments to prevent use-after-free
                    var arguments_copy = ArrayList(*Expression){};
                    for (arguments.items) |arg| {
                        try arguments_copy.append(self.allocator, arg);
                    }

                    expr = Expression{ .MethodCall = try self.allocateMethodCall(ast.MethodCallExpression{
                        .object = try self.allocateExpression(expr),
                        .method_name = property,
                        .arguments = arguments_copy,
                    })};
                } else {
                    // Regular member access
                    expr = Expression{ .MemberAccess = try self.allocateMemberAccess(ast.MemberAccessExpression{
                        .object = try self.allocateExpression(expr),
                        .property = property,
                    })};
                }
            } else if (self.match(.LeftBracket)) {
                // Array/slice access expr[index] or expr[start:end]
                const index = try self.parseExpression();
                
                if (self.match(.Colon)) {
                    // Slice access expr[start:end]
                    const end = if (!self.check(.RightBracket)) try self.parseExpression() else null;
        _ = try self.consume(.RightBracket, "Expected ']'");
                    
                    expr = Expression{ .SliceAccess = ast.SliceAccessExpression{
                        .array = try self.allocateExpression(expr),
                        .start = try self.allocateExpression(index),
                        .end = if (end) |e| try self.allocateExpression(e) else null,
                    }};
                } else {
                    // Array access expr[index]
        _ = try self.consume(.RightBracket, "Expected ']'");
                    
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
        var arguments = ArrayList(*Expression){};
        defer arguments.deinit(self.allocator);

        if (!self.check(.RightParen)) {
            while (true) {
                // Skip comments in argument lists
                while (self.check(.LineComment) or self.check(.BlockComment) or self.check(.Comment)) {
                    _ = self.advance();
                }
                
                if (self.check(.RightParen)) break;
                
                const arg = try self.parseExpression();
                const arg_ptr = try self.arena_allocator.create(Expression); // Arena for expressions
                arg_ptr.* = arg;
                try arguments.append(self.allocator, arg_ptr); // Use main allocator for list operations

                if (!self.match(.Comma)) break;
                
                // Skip comments after comma
                while (self.check(.LineComment) or self.check(.BlockComment) or self.check(.Comment)) {
                    _ = self.advance();
                }
            }
        }

        _ = try self.consume(.RightParen, "Expected ')' after arguments");
        
        // CRITICAL FIX: Clone the arguments to prevent use-after-free
        var arguments_copy = ArrayList(*Expression){};
        for (arguments.items) |arg| {
            try arguments_copy.append(self.allocator, arg);
        }

        return Expression{ .Call = .{
            .function = try self.allocateExpression(callee),
            .arguments = arguments_copy,
        }};
    }

    fn parsePrimary(self: *Parser) ParserError!Expression {
        // CRITICAL FIX: Enhanced primary expression parsing to prevent misinterpretation
        // of complex expressions as function names
        
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
            
            // FIXED: Better string interpolation detection
            // Check for {} patterns that are format placeholders (NOT arithmetic expressions)
            if (std.mem.indexOf(u8, str_content, "{}")) |_| {
                // This is a format string with placeholders, not interpolation
                return Expression{ .String = str_content };
            }
            
            // Check for string interpolation patterns like ${variable}
            if (std.mem.indexOf(u8, str_content, "${")) |_| {
                return try self.parseStringInterpolation(str_content);
            }
            
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
            var elements = ArrayList(Expression){};
            defer elements.deinit(self.allocator);
            
            if (!self.check(.RightBracket)) {
                while (true) {
                    const elem = try self.parseExpression();
                    try elements.append(self.allocator, elem);
                    
                    if (!self.match(.Comma)) break;
                }
            }
            
        _ = try self.consume(.RightBracket, "Expected ']'");
            
            return Expression{ .Array = try self.allocateArrayExpression(ast.ArrayExpression{
                .elements = try self.convertExpressionsToPointers(&elements),
            })};
        }

        // Grouped expressions and tuples (expr) or (1, 2, 3)
        if (self.match(.LeftParen)) {
            // Look ahead to see if this is a tuple or just grouped expression
            if (self.check(.RightParen)) {
                // Empty tuple ()
                _ = self.advance();
                return Expression{ .Tuple = ast.TupleExpression{
                    .elements = .empty,
                }};
            }
            
            var elements = ArrayList(Expression){};
            defer elements.deinit(self.allocator);
            var has_comma = false;
            
            while (true) {
                // Parse expression with full precedence
                const elem = try self.parseExpression();
                try elements.append(self.allocator, elem);
                
                if (self.match(.Comma)) {
                    has_comma = true;
                    if (self.check(.RightParen)) break; // Trailing comma
                } else {
                    break;
                }
            }
            
        _ = try self.consume(.RightParen, "Expected ')'");
            
            // Single element without comma is just grouped expression
            if (elements.items.len == 1 and !has_comma) {
                const single_expr = elements.items[0];
                return single_expr;
            }
            
            // Multiple elements or single with comma is a tuple
            return Expression{ .Tuple = ast.TupleExpression{
                .elements = try self.convertExpressionsToPointers(&elements),
            }};
        }

        // Map literals {key: value, ...}
        if (self.match(.LeftBrace)) {
            var entries = ArrayList(ast.MapEntry){};
            
            if (!self.check(.RightBrace)) {
                while (true) {
                    const key = try self.parseExpression();
        _ = try self.consume(.Colon, "Expected ':' after map key");
                    const value = try self.parseExpression();
                    
                    const key_ptr = try self.arena_allocator.create(Expression);
                    key_ptr.* = key;
                    const value_ptr = try self.arena_allocator.create(Expression);
                    value_ptr.* = value;
                    
                    try entries.append(self.allocator, ast.MapEntry{
                        .key = key_ptr,
                        .value = value_ptr,
                    });
                    
                    if (!self.match(.Comma)) break;
                }
            }
            
        _ = try self.consume(.RightBrace, "Expected '}'");
            
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
            var params = ArrayList([]const u8){};
            
            if (!self.check(.Pipe)) {
                while (true) {
                    if (!self.check(.Identifier)) {
                        return ParserError.UnexpectedToken;
                    }
                    
                    try params.append(self.allocator, self.advance().lexeme);
                    
                    if (!self.match(.Comma)) break;
                }
            }
            
        _ = try self.consume(.Pipe, "Expected '|' after lambda parameters");
        _ = try self.consume(.Arrow, "Expected '->' after lambda parameters");
            
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
        _ = try self.consume(.Greater, "Expected '>' after channel element type");
                
        _ = try self.consume(.LeftParen, "Expected '(' after make_channel<T>");
                
                var buffer_size: ?*Expression = null;
                if (!self.check(.RightParen)) {
                    const buffer_expr = try self.parseExpression();
                    const buffer_ptr = try self.arena_allocator.create(Expression);
                    buffer_ptr.* = buffer_expr;
                    buffer_size = buffer_ptr;
                }
                
        _ = try self.consume(.RightParen, "Expected ')' after make_channel");
                
                return Expression{ .ChannelCreation = ast.ChannelCreationExpression{
                    .element_type = element_type,
                    .buffer_size = buffer_size,
                }};
            }
        }

        // Channel creation dm[type](capacity)
        if (self.matchIdentifier("dm")) {
            if (self.match(.LeftBracket)) {
                const element_type = try self.parseType();
                _ = try self.consume(.RightBracket, "Expected ']' after channel element type");
                _ = try self.consume(.LeftParen, "Expected '(' after dm[T]");
                
                var buffer_size: ?*Expression = null;
                if (!self.check(.RightParen)) {
                    const buffer_expr = try self.parseExpression();
                    const buffer_ptr = try self.arena_allocator.create(Expression);
                    buffer_ptr.* = buffer_expr;
                    buffer_size = buffer_ptr;
                }
                
                _ = try self.consume(.RightParen, "Expected ')' after dm[T]");
                
                return Expression{ .ChannelCreation = ast.ChannelCreationExpression{
                    .element_type = element_type,
                    .buffer_size = buffer_size,
                }};
            }
        }

        // CRITICAL FIX: Enhanced identifier parsing with brace disambiguation
        // This prevents expressions like "i + 1 { ... }" from being interpreted as function names
        if (self.check(.Identifier) or self.check(.Facts)) {
            const name = self.advance().lexeme;
            
            // NEW SYNTAX: Check for composite literal Type[value]{...}
            if (self.check(.LeftBracket)) {
                // Parse array/slice type suffix: Type[value] or Type[5]
                var type_with_array = if (self.isKnownTypeName(name))
                    self.getBasicTypeFromName(name)
                else
                    ast.Type{ .Custom = name };
                    
                // Parse array/slice suffix(es)
                while (self.match(.LeftBracket)) {
                    if (self.check(.RightBracket)) {
                        // Empty brackets Type[] means slice (no size)
                        _ = self.advance(); // consume ']'
                        
                        const element_type_ptr = try self.arena_allocator.create(ast.Type);
                        element_type_ptr.* = type_with_array;
                        
                        type_with_array = ast.Type{ .Array = ast.ArrayType{
                            .ref_counted = ast.RefCounted.init(self.allocator),
                            .element_type = ast.RefPtr(ast.Type).init(element_type_ptr),
                            .size = null, // slice - no fixed size
                        }};
                    } else {
                        // Parse the size/value expression
                        if (self.check(.Number) or self.check(.Integer)) {
                            // Numeric size: Type[5]
                            const size_token = self.advance();
                            const size = std.fmt.parseInt(usize, size_token.lexeme, 10) catch {
                                return ParserError.InvalidSyntax;
                            };
                            
                            _ = try self.consume(.RightBracket, "Expected ']' after array size");
                            
                            const element_type_ptr = try self.arena_allocator.create(ast.Type);
                            element_type_ptr.* = type_with_array;
                            
                            type_with_array = ast.Type{ .Array = ast.ArrayType{
                                .ref_counted = ast.RefCounted.init(self.allocator),
                                .element_type = ast.RefPtr(ast.Type).init(element_type_ptr),
                                .size = size,
                            }};
                        } else if (self.check(.Identifier) and std.mem.eql(u8, self.peek().lexeme, "value")) {
                            // Special case: Type[value] means slice
                            _ = self.advance(); // consume 'value'
                            _ = try self.consume(.RightBracket, "Expected ']' after 'value'");
                            
                            const element_type_ptr = try self.arena_allocator.create(ast.Type);
                            element_type_ptr.* = type_with_array;
                            
                            type_with_array = ast.Type{ .Array = ast.ArrayType{
                                .ref_counted = ast.RefCounted.init(self.allocator),
                                .element_type = ast.RefPtr(ast.Type).init(element_type_ptr),
                                .size = null, // slice - no fixed size
                            }};
                        } else {
                            // Complex expression - consume and treat as slice
                            _ = try self.parseExpression(); // consume the expression
                            _ = try self.consume(.RightBracket, "Expected ']' after array expression");
                            
                            const element_type_ptr = try self.arena_allocator.create(ast.Type);
                            element_type_ptr.* = type_with_array;
                            
                            type_with_array = ast.Type{ .Array = ast.ArrayType{
                                .ref_counted = ast.RefCounted.init(self.allocator),
                                .element_type = ast.RefPtr(ast.Type).init(element_type_ptr),
                                .size = null, // treat as slice
                            }};
                        }
                    }
                }
                
                // Check for composite literal: Type[value]{...}
                if (self.check(.LeftBrace)) {
                    return try self.parseCompositeLiteralWithType(type_with_array);
                } else {
                    // This was just a type expression, return identifier
                    // (The type parsing was speculative)
                    return Expression{ .Identifier = name };
                }
            }
            
            // CRITICAL: Check for struct literal Name{field: value, ...} with proper brace handling
            // This distinguishes between struct literals and erroneous complex expressions
            if (self.check(.LeftBrace)) {
                std.debug.print("DEBUG: Found brace after identifier '{s}'\n", .{name});
                // Additional validation: ensure this is actually a struct literal context
                // and not a misplaced brace from a complex expression
                if (self.isValidStructLiteralContext()) {
                    std.debug.print("DEBUG: Valid struct literal context detected\n", .{});
                    return try self.parseStructLiteral(name);
                } else {
                    std.debug.print("DEBUG: Invalid struct literal context, treating as identifier\n", .{});
                    // This might be part of a complex expression that was incorrectly parsed
                    // Return the identifier and let the caller handle the brace
                    // The brace will be handled by parseBlockStatement in parseStatement
                    std.debug.print("DEBUG: Returning identifier due to invalid struct context: '{s}'\n", .{name});
                    return Expression{ .Identifier = name };
                }
            }
            
            // std.debug.print("DEBUG: Returning final identifier: '{s}'\n", .{name});
            return Expression{ .Identifier = name };
        }
        
        // std.debug.print("DEBUG: parsePrimary unexpected token: {any}\n", .{self.peek().kind});
        return ParserError.UnexpectedToken;
    }

    fn parseStructLiteral(self: *Parser, struct_name: []const u8) ParserError!Expression {
        std.debug.print("DEBUG: Parsing struct literal for '{s}'\n", .{struct_name});
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        // Support both StructFieldAssignment (legacy) and FieldInitializer (new)
        var fields = std.ArrayList(ast.FieldInitializer){};
        
        if (!self.check(.RightBrace)) {
            while (true) {
                if (!self.check(.Identifier)) {
                    std.debug.print("DEBUG: Expected identifier for field name, got: {any}\n", .{self.peek().kind});
                    return ParserError.UnexpectedToken;
                }
                
                const field_name = self.advance().lexeme;
                std.debug.print("DEBUG: Parsing field '{s}'\n", .{field_name});
        _ = try self.consume(.Colon, "Expected ':' after field name");
                const value = try self.parseExpression();
                
                const value_ptr = try self.arena_allocator.create(Expression);
                value_ptr.* = value;
                
                try fields.append(self.allocator, ast.FieldInitializer{
                    .field_name = field_name,
                    .value = value_ptr,
                });
                
                if (!self.match(.Comma)) break;
            }
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
        // Return the new StructExpression type
        return Expression{ .Struct = try self.allocateStructExpression(ast.StructExpression{
            .struct_name = struct_name,
            .fields = fields,
        })};
    }

    fn parseMatchExpression(self: *Parser) ParserError!Expression {
        _ = try self.consume(.Match, "Expected 'match'");
        
        const value = try self.allocateExpression(try self.parseExpression());
        
        _ = try self.consume(.LeftBrace, "Expected '{' after match value");
        
        var cases = ArrayList(ast.MatchCase){};
        var default_case: ?*Expression = null;
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            // Check for default case
            if (self.match(.Basic) or self.matchIdentifier("_")) {
        _ = try self.consume(.Arrow, "Expected '->' after default pattern");
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
            
        _ = try self.consume(.Arrow, "Expected '->' after pattern");
            const result = try self.parseExpression();
            
            var guard_ptr: ?*anyopaque = null;
            if (guard) |g| {
                const g_ptr = try self.arena_allocator.create(Expression);
                g_ptr.* = g;
                guard_ptr = try self.expressionToAnyopaque(g_ptr);
            }
            
            const result_ptr = try self.arena_allocator.create(Expression);
            result_ptr.* = result;
            
            try cases.append(self.allocator, ast.MatchCase{
                .pattern = pattern,
                .guard = guard_ptr,
                .result = try self.expressionToAnyopaque(result_ptr),
            });
            
            _ = self.match(.Comma);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}' after match cases");
        
        return Expression{ .Match = ast.MatchExpression{
            .expression = value,
            .cases = cases,
            .default_case = default_case,
        }};
    }

    fn parsePattern(self: *Parser) ParserError!ast.Pattern {
        return self.parsePatternOr();
    }

    /// Parse OR patterns (pattern1 | pattern2 | ...)
    fn parsePatternOr(self: *Parser) ParserError!ast.Pattern {
        var patterns = ArrayList(ast.Pattern){};
        
        const first_pattern = try self.parsePatternRange();
        try patterns.append(self.allocator, first_pattern);
        
        while (self.match(.Pipe)) {
            const pattern = try self.parsePatternRange();
            try patterns.append(self.allocator, pattern);
        }
        
        if (patterns.items.len == 1) {
            const single_pattern = patterns.items[0];
            patterns.deinit(self.allocator);
            return single_pattern;
        }
        
        return ast.Pattern{ .Array = patterns }; // Reuse Array for OR patterns temporarily
    }

    /// Parse range patterns (0..10, 'a'..'z')
    fn parsePatternRange(self: *Parser) ParserError!ast.Pattern {
        const start_pattern = try self.parsePatternPrimary();
        
        // Check for range operator (..)
        if (self.check(.DotDot)) {
            _ = self.advance(); // consume '..'
            
            // Check for inclusive range (...) 
            const is_inclusive = !self.check(.Dot);
            if (!is_inclusive) {
                _ = self.advance(); // consume the extra '.' for exclusive range
            }
            
            const end_pattern = try self.parsePatternPrimary();
            
            // For now, convert patterns to expressions (simplified)
            // In a full implementation, you'd handle pattern-to-expression conversion properly
            const start_expr = try self.arena_allocator.create(Expression);
            const end_expr = try self.arena_allocator.create(Expression);
            
            // Convert literal patterns to expressions
            switch (start_pattern) {
                .Literal => |lit| {
                    switch (lit) {
                        .Integer => |val| start_expr.* = Expression{ .Literal = ast.Literal{ .Integer = val }},
                        .String => |val| start_expr.* = Expression{ .Literal = ast.Literal{ .String = val }},
                        .Boolean => |val| start_expr.* = Expression{ .Literal = ast.Literal{ .Boolean = val }},
                        else => return ParserError.InvalidPattern,
                    }
                },
                else => return ParserError.InvalidPattern,
            }
            
            switch (end_pattern) {
                .Literal => |lit| {
                    switch (lit) {
                        .Integer => |val| end_expr.* = Expression{ .Literal = ast.Literal{ .Integer = val }},
                        .String => |val| end_expr.* = Expression{ .Literal = ast.Literal{ .String = val }},
                        .Boolean => |val| end_expr.* = Expression{ .Literal = ast.Literal{ .Boolean = val }},
                        else => return ParserError.InvalidPattern,
                    }
                },
                else => return ParserError.InvalidPattern,
            }
            
            // Note: This is a simplified range pattern representation
            // In the full implementation, you'd use the advanced AST
            return start_pattern; // Fallback for now
        }
        
        return start_pattern;
    }

    /// Parse primary patterns with guards
    fn parsePatternPrimary(self: *Parser) ParserError!ast.Pattern {
        const base_pattern = try self.parsePatternBase();
        
        // Check for guard condition (when/if)
        if (self.matchIdentifier("when") or self.matchIdentifier("if")) {
            const guard_condition = try self.parseExpression();
            
            // Note: This is simplified - in full implementation use advanced AST Guard pattern
            // For now, return the base pattern
            _ = guard_condition; // Suppress unused variable warning
            return base_pattern;
        }
        
        return base_pattern;
    }

    /// Parse base patterns (literals, variables, tuples, etc.)
    fn parsePatternBase(self: *Parser) ParserError!ast.Pattern {
        // Wildcard pattern _
        if (self.matchIdentifier("_")) {
            return ast.Pattern.Wildcard;
        }

        // Literal patterns with range detection
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

        // Array pattern [pat1, pat2, ...]
        if (self.match(.LeftBracket)) {
            var patterns = ArrayList(ast.Pattern){};
            
            if (!self.check(.RightBracket)) {
                while (true) {
                    const pattern = try self.parsePattern();
                    try patterns.append(self.allocator, pattern);
                    
                    if (!self.match(.Comma)) break;
                }
            }
            
            _ = try self.consume(.RightBracket, "Expected ']'");
            return ast.Pattern{ .Array = patterns };
        }

        // Tuple pattern (pat1, pat2, ...)
        if (self.match(.LeftParen)) {
            var patterns = ArrayList(ast.Pattern){};
            
            if (!self.check(.RightParen)) {
                while (true) {
                    const pattern = try self.parsePattern();
                    try patterns.append(self.allocator, pattern);
                    
                    if (!self.match(.Comma)) break;
                }
            }
            
            _ = try self.consume(.RightParen, "Expected ')'");
            return ast.Pattern{ .Tuple = patterns };
        }

        // Struct pattern or Variable pattern
        if (self.check(.Identifier)) {
            const name = self.advance().lexeme;
            
            // Struct pattern StructName{field: pattern, ...}
            if (self.match(.LeftBrace)) {
                var fields = ArrayList(ast.FieldPattern){};
                
                if (!self.check(.RightBrace)) {
                    while (true) {
                        if (!self.check(.Identifier)) {
                            return ParserError.UnexpectedToken;
                        }
                        
                        const field_name = self.advance().lexeme;
                        _ = try self.consume(.Colon, "Expected ':' after field name");
                        const pattern = try self.parsePattern();
                        
                        try fields.append(self.allocator, ast.FieldPattern{
                            .name = field_name,
                            .pattern = pattern,
                        });
                        
                        if (!self.match(.Comma)) break;
                    }
                }
                
                _ = try self.consume(.RightBrace, "Expected '}'");
                
                return ast.Pattern{ .Struct = ast.StructPattern{
                    .name = name,
                    .fields = fields,
                }};
            }
            
            // Simple variable pattern
            return ast.Pattern{ .Variable = name };
        }

        return ParserError.InvalidPattern;
    }

    // Helper methods for parsing statements
    fn parseReturnStatement(self: *Parser) ParserError!Statement {
        // SPEC CONFORMANCE: Return token (return/yolo/damn) has already been consumed by caller
        var return_stmt = ast.ReturnStatement{ .value = null };
        
        // Parse optional return value
        if (!self.check(.Semicolon) and !self.check(.Newline) and !self.isAtEnd() and !self.check(.RightBrace)) {
            const value_expr = try self.parseExpression();
            const value_ptr = try self.arena_allocator.create(Expression);

            value_ptr.* = value_expr;
            return_stmt.value = try self.expressionToAnyopaque(value_ptr);
        }
        
        return Statement{ .Return = return_stmt };
    }

    fn parseIfStatement(self: *Parser) ParserError!ast.IfStatement {
        // Handle both 'ready' and 'lowkey' keywords
        if (self.check(.Ready)) {
            _ = self.advance(); // consume 'ready'
        } else {
            _ = try self.consume(.Lowkey, "Expected 'lowkey' or 'ready'");
        }
        
        // CURSED syntax allows condition without parentheses: ready condition { or lowkey condition {
        var has_parens = false;
        if (self.match(.LeftParen)) {
            has_parens = true;
        }
        
        const condition = try self.parseExpression();
        
        if (has_parens) {
            _ = try self.consume(.RightParen, "Expected ')' after condition");
        }
        
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        var then_branch = ArrayList(*Statement){};
        // Use arena allocator consistently for ArrayList management
        errdefer then_branch.deinit(self.arena_allocator);
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = try self.parseStatement();
            const stmt_ptr = try self.arena_allocator.create(Statement); 

            stmt_ptr.* = stmt; 
            try then_branch.append(self.arena_allocator, stmt_ptr);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
        var else_branch: ?ArrayList(*Statement) = null;
        
        // Parse else clause (highkey/otherwise)
        if (self.match(.Highkey) or self.match(.Otherwise)) {
            var else_stmts = ArrayList(*Statement){};
            // Use arena allocator consistently and add error cleanup
            errdefer else_stmts.deinit(self.arena_allocator);
            
            if (self.check(.Lowkey) or self.check(.Ready)) {
                // else if
                const elif_stmt = try self.parseIfStatement();
                const if_stmt = Statement{ .If = elif_stmt };
                const if_stmt_ptr = try self.arena_allocator.create(Statement);
        
                if_stmt_ptr.* = if_stmt;
                try else_stmts.append(self.arena_allocator, if_stmt_ptr);
            } else {
                // else block
        _ = try self.consume(.LeftBrace, "Expected '{'");
                
                while (!self.check(.RightBrace) and !self.isAtEnd()) {
                    if (self.match(.Newline)) continue;
                    
                    const stmt = try self.parseStatement();
                    const stmt_ptr = try self.arena_allocator.create(Statement); 
    
                    stmt_ptr.* = stmt; 
                    try else_stmts.append(self.arena_allocator, stmt_ptr);
                }
                
        _ = try self.consume(.RightBrace, "Expected '}'");
            }
            
            else_branch = else_stmts;
        }
        
        const condition_ptr = try self.arena_allocator.create(Expression);

        condition_ptr.* = condition;
        
        return ast.IfStatement{
            .condition = try self.expressionToAnyopaque(condition_ptr),
            .then_branch = then_branch,
            .else_branch = else_branch,
        };
    }

    fn parseWhileStatement(self: *Parser) ParserError!ast.WhileStatement {
        _ = self.advance(); // consume periodt/flex/bestie
        
        // CURSED syntax allows condition without parentheses: bestie condition {
        var has_parens = false;
        if (self.match(.LeftParen)) {
            has_parens = true;
        }
        
        const condition = try self.parseExpression();
        
        if (has_parens) {
            _ = try self.consume(.RightParen, "Expected ')' after condition");
        }
        
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        var body = ArrayList(*Statement){};
        self.in_loop = true;
        defer { self.in_loop = false; }
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = try self.parseStatement();
            const stmt_ptr = try self.arena_allocator.create(Statement); 
            stmt_ptr.* = stmt; 
            try body.append(self.allocator, stmt_ptr);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
        const condition_ptr = try self.arena_allocator.create(Expression);
        condition_ptr.* = condition;
        
        return ast.WhileStatement{
            .condition = condition_ptr,
            .body = body,
        };
    }

    fn parseForStatement(self: *Parser) ParserError!Statement {
        _ = try self.consume(.Bestie, "Expected 'bestie'");
        
        // Debug logging removed for test compatibility
        
        // Check for infinite loop
        try self.checkInfiniteLoop();
        
        // Check for range-for loop (bestie var := flex ...)
        if (self.isRangeForLoop()) {
            // Range for loop detected
            return try self.parseRangeForStatement();
        }
        
        // Check if it's a while-style for loop (no semicolons)
        if (!self.hasSemicolonsBeforeBrace()) {
            // While-style for loop detected
            // While-style for loop: bestie condition { ... }
            var condition: ?Expression = null;
            
            if (!self.check(.LeftBrace)) {
                condition = self.parseExpression() catch {
                    // If expression parsing fails, provide better error recovery
                    // Failed to parse condition
                    return ParserError.InvalidSyntax;
                };
            }
            
        _ = try self.consume(.LeftBrace, "Expected '{'");
            
            var body = ArrayList(*Statement){};
            self.in_loop = true;
            defer { self.in_loop = false; }
            
            while (!self.check(.RightBrace) and !self.isAtEnd()) {
                if (self.match(.Newline)) continue;
                
                const stmt = try self.parseStatement();
                const stmt_ptr = try self.arena_allocator.create(Statement); 
                stmt_ptr.* = stmt; 
                try body.append(self.allocator, stmt_ptr);
            }
            
        _ = try self.consume(.RightBrace, "Expected '}'");
            
            var condition_ptr: ?*Expression = null;
            if (condition) |cond| {
                const cond_ptr = try self.arena_allocator.create(Expression);
                cond_ptr.* = cond;
                condition_ptr = cond_ptr;
            }
            
            return Statement{ .For = ast.ForStatement{
                .init = null,
                .condition = condition_ptr,
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
        _ = try self.consume(.Semicolon, "Expected ';' after for loop init");
        
        // Parse condition (optional)
        var condition: ?Expression = null;
        if (!self.check(.Semicolon)) {
            condition = try self.parseExpression();
        }
        _ = try self.consume(.Semicolon, "Expected ';' after for loop condition");
        
        // Parse update statement (optional)
        var update: ?Statement = null;
        if (!self.check(.LeftBrace)) {
            update = try self.parseStatement();
        }
        
        // Parse body
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        var body = ArrayList(*Statement){};
        self.in_loop = true;
        defer { self.in_loop = false; }
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = try self.parseStatement();
            const stmt_ptr = try self.arena_allocator.create(Statement);
            stmt_ptr.* = stmt;
            try body.append(self.allocator, stmt_ptr);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
        return Statement{ .For = ast.ForStatement{
            .init = if (init_stmt) |stmt| blk: {
                const stmt_ptr = try self.arena_allocator.create(Statement);
                stmt_ptr.* = stmt;
                break :blk stmt_ptr;
            } else null,
            .condition = if (condition) |cond| blk: {
                const cond_ptr = try self.arena_allocator.create(Expression);
                cond_ptr.* = cond;
                break :blk cond_ptr;
            } else null,
            .update = if (update) |stmt| blk: {
                const stmt_ptr = try self.arena_allocator.create(Statement);
                stmt_ptr.* = stmt;
                break :blk stmt_ptr;
            } else null,
            .body = body,
        }};
    }

    fn parseRangeForStatement(self: *Parser) ParserError!Statement {
        // Parse variable(s) for range-for loop
        var variables = ArrayList([]const u8){};
        
        // Parse first variable
        if (self.check(.Identifier)) {
            try variables.append(self.allocator, self.advance().lexeme);
        }
        
        // Parse second variable if comma present
        if (self.match(.Comma)) {
            if (self.check(.Identifier)) {
                try variables.append(self.allocator, self.advance().lexeme);
            }
        }
        
        // Expect := or =
        if (!self.match(.ColonEqual) and !self.match(.Equal)) {
            return ParserError.UnexpectedToken;
        }
        
        // Expect 'flex'
        _ = try self.consume(.Flex, "Expected 'flex' in range-for loop");
        
        // Parse iterable expression
        const iterable = try self.parseExpression();
        const iterable_ptr = try self.arena_allocator.create(Expression);
        iterable_ptr.* = iterable;
        
        // Parse body
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        var body = ArrayList(*Statement){};
        self.in_loop = true;
        defer { self.in_loop = false; }
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = try self.parseStatement();
            const stmt_ptr = try self.arena_allocator.create(Statement);
            stmt_ptr.* = stmt;
            try body.append(self.allocator, stmt_ptr);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
        // Create ForIn statement
        return Statement{ .ForIn = ast.ForInStatement{
            .variable = if (variables.items.len > 0) variables.items[0] else "_",
            .iterable = iterable_ptr,
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
        var type_parameters = ArrayList(ast.TypeParameter){};
        if (self.match(.Less) or self.match(.LeftAngle)) {
            while (!self.check(.Greater) and !self.check(.RightAngle) and !self.isAtEnd()) {
                if (self.check(.Identifier)) {
                    const param_name = self.advance().lexeme;
                    var param = ast.TypeParameter{
                        .name = param_name,
                        .constraints = ArrayList(ast.Type){}
        };
                    
                    // Parse constraints (T: Interface1 + Interface2)
                    if (self.match(.Colon)) {
                        while (!self.check(.Comma) and !self.check(.Greater) and !self.check(.RightAngle)) {
                            const constraint = try self.parseType();
                            try param.constraints.append(self.allocator, constraint);
                            if (!self.match(.Plus)) break;
                        }
                    }
                    
                    try type_parameters.append(self.allocator, param);
                }
                
                if (!self.match(.Comma)) break;
            }
            
            if (!self.match(.Greater) and !self.match(.RightAngle)) {
                return ParserError.MissingToken;
            }
        }
        
        // Expect '{'
        _ = try self.consume(.LeftBrace, "Expected '{' after struct name");
        
        // Parse fields and methods
        var fields = ArrayList(ast.StructField){};
        var methods = ArrayList(ast.FunctionStatement){};
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            // Skip newlines
            if (self.match(.Newline)) {
                continue;
            }
            
            // Check for method definition (slay keyword)
            if (self.check(.Slay)) {
                const method = try self.parseStructMethod();
                try methods.append(self.allocator, method);
                continue;
            }
            
            // Parse visibility modifier for fields
            var visibility = ast.Visibility.Private;
            // Removed .Spill keyword - not used as visibility modifier
            if (self.match(.Priv)) {
                visibility = .Private;
            } else if (self.match(.Crew)) {
                visibility = .Package;
            }
            
            // Parse field
            const field = try self.parseStructField(visibility);
            try fields.append(self.allocator, field);
            
            // Optional comma
            _ = self.match(.Comma);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}' after struct fields");
        
        return Statement{ .Struct = ast.StructStatement{
            .name = name,
            .fields = fields,
            .methods = methods,
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

    fn parseStructMethod(self: *Parser) ParserError!ast.FunctionStatement {
        // Parse as a regular function, but it's a method
        _ = try self.consume(.Slay, "Expected 'slay' keyword");
        
        // Parse method name
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // Parse parameters
        _ = try self.consume(.LeftParen, "Expected '(' after method name");
        
        var parameters = ArrayList(ast.Parameter){};
        
        if (!self.check(.RightParen)) {
            while (true) {
                const param = try self.parseParameter();
                try parameters.append(self.allocator, param);
                
                if (!self.match(.Comma)) break;
            }
        }
        
        _ = try self.consume(.RightParen, "Expected ')' after parameters");
        
        // Parse return type (optional)
        var return_type: ?ast.Type = null;
        if (!self.check(.LeftBrace)) {
            return_type = try self.parseType();
        }
        
        // Parse method body
        _ = try self.consume(.LeftBrace, "Expected '{' before method body");
        
        var body = ArrayList(*ast.Statement){};
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) {
                continue;
            }
            
            const stmt = try self.parseStatement();
            const stmt_ptr = try self.arena_allocator.create(Statement);
            stmt_ptr.* = stmt;
            try body.append(self.allocator, stmt_ptr);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}' after method body");
        
        return ast.FunctionStatement{
            .name = name,
            .parameters = parameters,
            .return_type = return_type,
            .body = body,
            .visibility = .Public,
            .is_async = false,
            .type_parameters = .empty,
            .comments = .empty,
        };
    }

    fn parseInterfaceStatement(self: *Parser) ParserError!Statement {
        _ = try self.consume(.Collab, "Expected 'collab'");
        
        // Parse interface name
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // Parse generic type parameters
        var type_parameters = ArrayList(ast.TypeParameter){};
        if (self.match(.Less) or self.match(.LeftAngle)) {
            while (!self.check(.Greater) and !self.check(.RightAngle) and !self.isAtEnd()) {
                if (self.check(.Identifier)) {
                    const param_name = self.advance().lexeme;
                    const param = ast.TypeParameter{
                        .name = param_name,
                        .constraints = ArrayList(ast.Type){}
        };
                    try type_parameters.append(self.allocator, param);
                }
                
                if (!self.match(.Comma)) break;
            }
            
            if (!self.match(.Greater) and !self.match(.RightAngle)) {
                return ParserError.MissingToken;
            }
        }
        
        // Parse interface inheritance (extends)
        var extends = std.ArrayList([]const u8){};
        if (self.match(.Extends)) {
            while (true) {
                if (!self.check(.Identifier)) {
                    return ParserError.UnexpectedToken;
                }
                const parent_interface = self.advance().lexeme;
                try extends.append(self.allocator, parent_interface);
                
                if (!self.match(.Comma)) break;
            }
        }
        
        // Parse interface composition (with)
        var compositions = std.ArrayList([]const u8){};
        if (self.match(.With)) {
            while (true) {
                if (!self.check(.Identifier)) {
                    return ParserError.UnexpectedToken;
                }
                const composed_interface = self.advance().lexeme;
                try compositions.append(self.allocator, composed_interface);
                
                if (!self.match(.Comma)) break;
            }
        }
        
        // Expect '{'
        _ = try self.consume(.LeftBrace, "Expected '{' after interface name");
        
        // Parse methods
        var methods = ArrayList(ast.MethodSignature){};
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            // Skip newlines
            if (self.match(.Newline)) {
                continue;
            }
            
            // Parse method signature
            const method = try self.parseMethodSignature();
            try methods.append(self.allocator, method);
            
            // Optional semicolon
            _ = self.match(.Semicolon);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}' after interface methods");
        
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
        _ = try self.consume(.Slay, "Expected 'slay' keyword for method");
        
        // Parse method name
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // Parse parameters
        _ = try self.consume(.LeftParen, "Expected '('");
        
        var parameters = ArrayList(ast.Parameter){};
        
        if (!self.check(.RightParen)) {
            while (true) {
                const param = try self.parseParameter();
                try parameters.append(self.allocator, param);
                
                if (!self.match(.Comma)) break;
            }
        }
        
        _ = try self.consume(.RightParen, "Expected ')'");
        
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
        _ = try self.consume(.Impl, "Expected 'impl'");
        
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
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        var methods = ArrayList(ast.FunctionStatement){};
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const method = try self.parseFunctionStatement();
            try methods.append(self.allocator, method);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
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
            _ = try self.consumeIdentifier("like", "Expected 'like' after 'be'");
        } else {
            return ParserError.UnexpectedToken;
        }
        
        // Parse alias name
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        const name = self.advance().lexeme;
        
        // Expect '='
        _ = try self.consume(.Equal, "Expected '=' after type alias name");
        
        // Parse target type
        const target_type = try self.parseType();
        
        return Statement{ .TypeAlias = ast.TypeAliasStatement{
            .name = name,
            .target_type = target_type,
            .visibility = .Public,
        }};
    }

    fn parseStanStatement(self: *Parser) ParserError!Statement {
        _ = try self.consume(.Stan, "Expected 'stan'");
        
        // Parse block: stan { ... }
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        var body = ArrayList(*Statement){};
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = try self.parseStatement();
            const stmt_ptr = try self.arena_allocator.create(Statement); 
            stmt_ptr.* = stmt; 
            try body.append(self.allocator, stmt_ptr);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
        return Statement{ .Stan = ast.StanStatement{
            .body = body,
        }};
    }

    fn parseGoroutineStatement(self: *Parser) ParserError!Statement {
        _ = try self.consume(.Stan, "Expected 'stan'");
        
        // Parse block or expression
        if (self.check(.LeftBrace)) {
            // Block form: stan { ... }
        _ = try self.consume(.LeftBrace, "Expected '{'");
            
            var body = ArrayList(*Statement){};
            while (!self.check(.RightBrace) and !self.isAtEnd()) {
                if (self.match(.Newline)) continue;
                
                const stmt = try self.parseStatement();
                const stmt_ptr = try self.arena_allocator.create(Statement); 
                stmt_ptr.* = stmt; 
                try body.append(self.allocator, stmt_ptr);
            }
            
        _ = try self.consume(.RightBrace, "Expected '}'");
            
            return Statement{ .Goroutine = ast.GoroutineStatement{
                .call = Expression{ .Block = ast.BlockExpression{ .statements = body } }
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
        _ = try self.consume(.VibeCheck, "Expected 'vibe check'");
        
        const expression = try self.parseExpression();
        const expression_ptr = try self.arena_allocator.create(Expression);
        expression_ptr.* = expression;
        
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        var patterns = ArrayList(ast.PatternCase){};
        var default_case: ?ArrayList(*Statement) = null;
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            // Check for default case
            if (self.match(.Basic)) {
        _ = try self.consume(.Colon, "Expected ':' after 'basic'");
                
                var default_stmts = std.ArrayList(*Statement){};
                while (!self.check(.Mood) and !self.check(.Basic) and !self.check(.RightBrace) and !self.isAtEnd()) {
                    if (self.match(.Newline)) continue;
                    
                    const stmt = try self.parseStatement();
                    const stmt_ptr = try self.arena_allocator.create(Statement); 
                    stmt_ptr.* = stmt; 
                    try default_stmts.append(self.allocator, stmt_ptr);
                }
                
                default_case = default_stmts;
                continue;
            }
            
            // Parse case
            if (self.match(.Mood)) {
                const pattern = try self.parsePattern();
                
                var guard: ?*Expression = null;
                if (self.matchIdentifier("if")) {
                    const guard_expr = try self.parseExpression();
                    const guard_ptr = try self.arena_allocator.create(Expression);
                    guard_ptr.* = guard_expr;
                    guard = guard_ptr;
                }
                
        _ = try self.consume(.Colon, "Expected ':' after case pattern");
                
                var case_body = ArrayList(*ast.Statement){};
                while (!self.check(.Mood) and !self.check(.Basic) and !self.check(.RightBrace) and !self.isAtEnd()) {
                    if (self.match(.Newline)) continue;
                    
                    const stmt = try self.parseStatement();
                    const stmt_ptr = try self.arena_allocator.create(Statement); stmt_ptr.* = stmt; try case_body.append(self.allocator, stmt_ptr);
                }
                
                try patterns.append(self.allocator, ast.PatternCase{
                    .pattern = pattern,
                    .guard = guard,
                    .body = case_body,
                });
            }
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
        return Statement{ .PatternSwitch = ast.PatternSwitchStatement{
            .expression = expression_ptr,
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
        
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        var cases = ArrayList(ast.SelectCase){};
        var default_case: ?ArrayList(*Statement) = null;
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            // Check for default case
            if (self.match(.Basic)) {
        _ = try self.consume(.Colon, "Expected ':' after 'basic'");
                
                var default_stmts = std.ArrayList(*Statement){};
                while (!self.check(.Mood) and !self.check(.Basic) and !self.check(.RightBrace) and !self.isAtEnd()) {
                    if (self.match(.Newline)) continue;
                    
                    const stmt = try self.parseStatement();
                    const stmt_ptr = try self.arena_allocator.create(Statement); 
                    stmt_ptr.* = stmt; 
                    try default_stmts.append(self.allocator, stmt_ptr);
                }
                
                default_case = default_stmts;
                continue;
            }
            
            // Parse case
            if (self.match(.Mood)) {
                const channel_op = try self.parseChannelOperation();
                
        _ = try self.consume(.Colon, "Expected ':' after channel operation");
                
                var case_body = ArrayList(*Statement){};
                while (!self.check(.Mood) and !self.check(.Basic) and !self.check(.RightBrace) and !self.isAtEnd()) {
                    if (self.match(.Newline)) continue;
                    
                    const stmt = try self.parseStatement();
                    const stmt_ptr = try self.arena_allocator.create(Statement); 
                    stmt_ptr.* = stmt; 
                    try case_body.append(self.allocator, stmt_ptr);
                }
                
                try cases.append(self.allocator, ast.SelectCase{
                    .channel_op = channel_op,
                    .body = case_body,
                });
            }
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
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
            const channel_ptr = try self.arena_allocator.create(Expression);
            channel_ptr.* = channel;
            const value_ptr = try self.arena_allocator.create(Expression);
            value_ptr.* = value;
            return ast.ChannelOperation{ .Send = .{
                .channel = channel_ptr,
                .value = value_ptr,
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
        _ = try self.consume(.LeftArrow, "Expected '<-'");
                const actual_channel = try self.parseExpression();
                const actual_channel_ptr = try self.arena_allocator.create(Expression);
                actual_channel_ptr.* = actual_channel;
                
                return ast.ChannelOperation{ .Receive = .{
                    .channel = actual_channel_ptr,
                    .variable = variable,
                }};
            } else {
                // Just receiving: <-channel
                const channel_ptr = try self.arena_allocator.create(Expression);
                channel_ptr.* = channel;
                return ast.ChannelOperation{ .Receive = .{
                    .channel = channel_ptr,
                    .variable = null,
                }};
            }
        }
    }

    fn parseDeferStatement(self: *Parser) ParserError!Statement {
        _ = try self.consume(.Later, "Expected 'later'");
        
        const stmt_ptr = try self.arena_allocator.create(Statement);
        stmt_ptr.* = try self.parseStatement();
        
        return Statement{ .Defer = ast.DeferStatement{ .statement = stmt_ptr } };
    }

    fn parseYikesStatement(self: *Parser) ParserError!ast.YikesStatement {
        _ = try self.consume(.Yikes, "Expected 'yikes'");
        
        // Parse error message expression
        const message_expr = try self.parseExpression();
        const message_ptr = try self.arena_allocator.create(Expression);
        message_ptr.* = message_expr;
        
        // Optional error type annotation (yikes "message" as RuntimeError)
        var error_type: ?[]const u8 = null;
        if (self.match(.As)) {
            if (!self.check(.Identifier)) {
                return ParserError.UnexpectedToken;
            }
            error_type = self.advance().lexeme;
        }
        
        return ast.YikesStatement{
            .message = message_ptr,
            .error_type = error_type,
            .location = self.getCurrentSourceLocation(),
        };
    }

    fn parseFamStatement(self: *Parser) ParserError!ast.FamStatement {
        _ = try self.consume(.Fam, "Expected 'fam'");
        
        // Parse try body block
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        var try_body = ArrayList(ast.Statement){};
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = try self.parseStatement();
            try try_body.append(self.allocator, stmt);
        }
        _ = try self.consume(.RightBrace, "Expected '}'");
        
        // Parse catch blocks
        var catch_blocks = std.ArrayList(ast.FamStatement.CatchBlock){};
        while (self.match(.Shook)) {
            var error_variable: ?[]const u8 = null;
            var error_type: ?[]const u8 = null;
            
            // Parse error variable (shook error)
            if (self.check(.Identifier)) {
                error_variable = self.advance().lexeme;
            }
            
            // Optional type constraint (shook error: RuntimeError)
            if (self.match(.Colon)) {
                if (!self.check(.Identifier)) {
                    return ParserError.UnexpectedToken;
                }
                error_type = self.advance().lexeme;
            }
            
            // Parse catch body
            _ = try self.consume(.LeftBrace, "Expected '{'");
            var catch_body = ArrayList(ast.Statement){};
            while (!self.check(.RightBrace) and !self.isAtEnd()) {
                if (self.match(.Newline)) continue;
                
                const stmt = try self.parseStatement();
                try catch_body.append(self.allocator, stmt);
            }
            _ = try self.consume(.RightBrace, "Expected '}'");
            
            try catch_blocks.append(self.allocator, ast.FamStatement.CatchBlock{
                .error_variable = error_variable,
                .error_type = error_type,
                .body = catch_body,
            });
        }
        
        // Optional finally block (not supported by basic lexer yet)
        const finally_block: ?ArrayList(Statement) = null;
        // READY: Can be enabled when Finally token is added to lexer.zig
        // if (self.match(.Finally)) {
        //     _ = try self.consume(.LeftBrace, "Expected '{'");
        //     finally_block = .empty;
        //     while (!self.check(.RightBrace) and !self.isAtEnd()) {
        //         if (self.match(.Newline)) continue;
        //         
        //         const stmt = try self.parseStatement();
        //         try finally_block.?.append(allocator, stmt);
        //     }
        //     _ = try self.consume(.RightBrace, "Expected '}'");
        // }
        
        return ast.FamStatement{
            .try_body = try_body,
            .catch_blocks = catch_blocks,
            .finally_block = finally_block,
        };
    }

    fn parseConstDeclaration(self: *Parser) ParserError!ast.ConstDecl {
        _ = try self.consume(.Facts, "Expected 'facts'");
        
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        var const_type: ?ast.Type = null;
        if (self.match(.Colon)) {
            const_type = try self.parseType();
        }
        
        _ = try self.consume(.Equal, "Expected '=' after constant name");
        
        const value = try self.parseExpression();
        const value_ptr = try self.arena_allocator.create(Expression);
        value_ptr.* = value;
        
        return ast.ConstDecl{
            .name = name,
            .const_type = const_type,
            .value = @ptrCast(value_ptr),
            .visibility = .Public,
        };
    }

    fn parseShortDeclaration(self: *Parser) ParserError!Statement {
        var names = ArrayList([]const u8){};
        
        // Parse variable names (can be tuple destructuring)
        if (self.match(.LeftParen)) {
            // Tuple destructuring: (a, b, c) := (1, 2, 3)
            while (!self.check(.RightParen) and !self.isAtEnd()) {
                if (!self.check(.Identifier)) {
                    return ParserError.UnexpectedToken;
                }
                
                try names.append(self.allocator, self.advance().lexeme);
                
                if (!self.match(.Comma)) break;
            }
            
        _ = try self.consume(.RightParen, "Expected ')'");
        } else {
            // Single variable or comma-separated: a, b := 1, 2
            if (!self.check(.Identifier)) {
                return ParserError.UnexpectedToken;
            }
            
            try names.append(self.allocator, self.advance().lexeme);
            
            while (self.match(.Comma)) {
                if (!self.check(.Identifier)) {
                    return ParserError.UnexpectedToken;
                }
                
                try names.append(self.allocator, self.advance().lexeme);
            }
        }
        
        _ = try self.consume(.ColonEqual, "Expected ':=' in short declaration");
        
        // Parse values
        var values = std.ArrayList(*Expression){};
        
        if (self.match(.LeftParen)) {
            // Tuple values: (1, 2, 3)
            while (!self.check(.RightParen) and !self.isAtEnd()) {
                const value = try self.parseExpression();
                const value_ptr = try self.arena_allocator.create(Expression);
                value_ptr.* = value;
                try values.append(self.allocator, value_ptr);
                
                if (!self.match(.Comma)) break;
            }
            
        _ = try self.consume(.RightParen, "Expected ')'");
        } else {
            // Single value or comma-separated: 1, 2
            const value = try self.parseExpression();
            const value_ptr = try self.arena_allocator.create(Expression);
            value_ptr.* = value;
            try values.append(self.allocator, value_ptr);
            
            while (self.match(.Comma)) {
                const next_value = try self.parseExpression();
                const next_value_ptr = try self.arena_allocator.create(Expression);
                next_value_ptr.* = next_value;
                try values.append(self.allocator, next_value_ptr);
            }
        }
        
        return Statement{ .ShortDeclaration = ast.ShortDeclarationStatement{
            .names = names,
            .values = values,
        }};
    }

    fn parseAssignmentStatement(self: *Parser) ParserError!Statement {
        // Parse only the left-hand side (target) without consuming assignment operator
        const target = try self.parsePrimary(); // Use parsePrimary instead of parseExpression
        const target_ptr = try self.arena_allocator.create(Expression);

        target_ptr.* = target;
        
        // Now check for assignment operator
        if (self.match(.Equal) or self.match(.PlusEqual) or self.match(.MinusEqual) or
           self.match(.StarEqual) or self.match(.SlashEqual) or self.match(.PercentEqual)) {
            const operator = self.previous().lexeme;
            
            // Validate that the target is actually assignable
            if (!self.isValidAssignmentTarget(target)) {
                _ = self.reportErrorWithContext("Invalid assignment target - complex expressions cannot be assigned to", "parseAssignmentStatement") catch {};
                return ParserError.InvalidAssignment;
            }
            
            const value = try self.parseExpression();
            const value_ptr = try self.arena_allocator.create(Expression);

            value_ptr.* = value;
            
            return Statement{ .Assignment = ast.AssignmentStatement{
                .target = @ptrCast(target_ptr),
                .value = @ptrCast(value_ptr),
                .operator = operator,
            }};
        } else {
            // If no assignment operator found, this is an expression statement
            return Statement{ .Expression = target };
        }
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

    fn matchKeyword(self: *Parser, keyword: []const u8) bool {
        // Try matching as identifier first (for keywords like "finally")
        if (self.matchIdentifier(keyword)) {
            return true;
        }
        // Also check if it's a token type (for reserved keywords)
        if (self.check(.Identifier)) {
            const token = self.peek();
            if (std.mem.eql(u8, token.lexeme, keyword)) {
                _ = self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(self: *Parser, kind: TokenKind) bool {
        if (self.isAtEnd()) return false;
        return self.peek().kind == kind;
    }

    fn checkType(self: *Parser) bool {
        return self.check(.Normie) or self.check(.Drip) or self.check(.Tea) or self.check(.Txt) or
               self.check(.Sip) or self.check(.Smol) or self.check(.Mid) or
               self.check(.Thicc) or self.check(.Snack) or self.check(.Meal) or
               self.check(.Byte) or self.check(.Rune) or self.check(.Extra) or
               self.check(.Lit) or self.check(.Cap) or self.check(.Identifier) or
               self.check(.LeftBracket) or self.check(.Dm) or 
               self.checkIdentifier("map") or self.check(.LeftParen);
    }

    fn checkBasicType(self: *Parser) bool {
        return self.check(.Normie) or self.check(.Drip) or self.check(.Tea) or self.check(.Txt) or
               self.check(.Sip) or self.check(.Smol) or self.check(.Mid) or
               self.check(.Thicc) or self.check(.Snack) or self.check(.Meal) or
               self.check(.Byte) or self.check(.Rune) or self.check(.Extra) or
               self.check(.Lit) or self.check(.Cap) or self.check(.Yikes) or self.check(.Identifier);
               // Removed .At - pointer types should be handled separately in parseType
    }

    fn advance(self: *Parser) Token {
        if (!self.isAtEnd()) {
            self.current += 1;
            // Reset loop counter when position actually advances
            if (self.current % 10 == 0) { // Periodic cleanup
                self.resetLoopCounter();
            }
        }
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
        
        // Enhanced error reporting with graceful handling
        const current_token = self.peek();
        var buffer: [256]u8 = undefined;
        const error_msg = std.fmt.bufPrint(buffer[0..], "{s}. Expected {any}, got {any}", .{ message, kind, current_token.kind }) catch message;
        
        _ = self.reportErrorWithContext(error_msg, "consume") catch {};
        return ParserError.UnexpectedToken;
    }

    fn consumeIdentifier(self: *Parser, identifier: []const u8, message: []const u8) ParserError!Token {
        if (self.matchIdentifier(identifier)) return self.previous();
        
        std.debug.print("Parser error: {s}. Expected '{s}', got {s}\n", .{ message, identifier, self.peek().lexeme });
        self.had_error = true;
        return ParserError.UnexpectedToken;
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
        // Enhanced assignment detection for simple cases like "x = 10"
        var pos = self.current;
        
        // Must start with an identifier for assignment
        if (pos >= self.tokens.len or self.tokens[pos].kind != .Identifier) {
            return false;
        }
        pos += 1;
        
        // Look for assignment operator immediately after identifier
        if (pos >= self.tokens.len) {
            return false;
        }
        
        const token_kind = self.tokens[pos].kind;
        if (token_kind == .Equal or token_kind == .PlusEqual or 
           token_kind == .MinusEqual or token_kind == .StarEqual or
           token_kind == .SlashEqual or token_kind == .PercentEqual) {
            return true;
        }
        
        // FIXED: Only check for simple member access assignment like obj.prop = value
        // Don't scan too far ahead as it can find unrelated assignments in later statements
        const initial_pos = pos;
        const max_lookahead = 5;  // Limit lookahead to prevent false positives
        
        while (pos < self.tokens.len and (pos - initial_pos) < max_lookahead) {
            const current_kind = self.tokens[pos].kind;
            if (current_kind == .Equal or current_kind == .PlusEqual or 
               current_kind == .MinusEqual or current_kind == .StarEqual or
               current_kind == .SlashEqual or current_kind == .PercentEqual) {
                return true;
            }
            // Stop at statement terminators or function calls (which indicate this is not an assignment)
            if (current_kind == .Semicolon or current_kind == .Newline or 
               current_kind == .LeftBrace or current_kind == .RightBrace or
               current_kind == .LeftParen) {  // LeftParen indicates a function call, not assignment
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

    // CRITICAL FIX: Helper function to validate struct literal context
    // This prevents misidentification of complex expressions as struct literals
    fn isValidStructLiteralContext(self: *Parser) bool {
        // DEBUG: Checking if valid struct literal context
        // Look ahead to see if the brace contains field assignments (field: value)
        var pos = self.current + 1; // Skip the '{'
        var brace_depth: usize = 1;
        var found_colon_assignment = false;
        
        while (pos < self.tokens.len and brace_depth > 0) {
            switch (self.tokens[pos].kind) {
                .LeftBrace => brace_depth += 1,
                .RightBrace => brace_depth -= 1,
                .Colon => {
                    // Check if this colon is at the right depth for field assignment
                    if (brace_depth == 1 and pos > self.current + 1) {
                        // Previous token should be an identifier (field name)
                        if (self.tokens[pos - 1].kind == .Identifier) {
                            found_colon_assignment = true;
                        }
                    }
                },
                .Eof => break,
                else => {}
        }
            pos += 1;
        }
        
        // If we found field:value patterns, this is likely a struct literal
        // If not, it might be a misplaced brace from complex expression parsing
        return found_colon_assignment;
    }

    // CRITICAL FIX: Validate assignment targets to prevent complex expressions being treated as assignable
    fn isValidAssignmentTarget(_: *Parser, target: Expression) bool {
        switch (target) {
            .Identifier => return true,
            .MemberAccess => return true,
            .ArrayAccess => return true,
            .SliceAccess => return true,
            // Binary expressions like "i + 1" are NOT valid assignment targets
            .Binary => return false,
            // Function calls are NOT valid assignment targets
            .Call => return false,
            // Literals are NOT valid assignment targets
            .Integer, .Float, .String, .Boolean, .Character => return false,
            else => return false,
        }
    }

    fn hasSemicolonsBeforeBrace(self: *Parser) bool {
        var pos = self.current;
        var semicolon_count: usize = 0;
        const start_pos = pos;
        var loop_protection: usize = 0;
        const MAX_LOOKAHEAD: usize = 1000; // Limit lookahead to prevent infinite loops
        
        // Check for semicolons before brace
        
        while (pos < self.tokens.len and loop_protection < MAX_LOOKAHEAD) {
            const token = self.tokens[pos];
            // Check token type
            
            // Break conditions
            if (token.kind == .LeftBrace) {
                break;
            }
            if (token.kind == .Eof) {
                break;
            }
            // Also break on unexpected control flow tokens that indicate we've gone too far
            if (token.kind == .RightBrace or token.kind == .RightParen) {
                // Hit unexpected delimiter, stopping scan
                break;
            }
            
            if (token.kind == .Semicolon) {
                semicolon_count += 1;
            }
            
            pos += 1;
            loop_protection += 1;
            
            // Additional safety: if we've scanned too far without finding a brace, assume no semicolons
            if (pos - start_pos > 50) {
                if (builtin.mode == .Debug) {
                    std.debug.print("DEBUG: hasSemicolonsBeforeBrace scanned too far ({}), assuming no C-style for loop\n", .{pos - start_pos});
                }
                return false;
            }
        }
        
        // If we hit loop protection limit, return false to avoid infinite loops
        if (loop_protection >= MAX_LOOKAHEAD) {
            // Hit lookahead limit, assuming no C-style for loop
            return false;
        }
        
        // Found semicolons after scanning tokens
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
                else => {}
        }
            pos += 1;
        }
        
        return false;
    }

    // Memory allocation helpers
    fn allocateExpression(self: *Parser, expr: Expression) ParserError!*Expression {
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
        ptr.* = expr;
        return ptr;
    }

    fn allocateUnaryExpression(self: *Parser, unary: ast.UnaryExpression) ParserError!*ast.UnaryExpression {
        const ptr = self.arena_allocator.create(ast.UnaryExpression) catch return ParserError.OutOfMemory;
        ptr.* = unary;
        return ptr;
    }

    // Arena-based allocation helpers (automatic cleanup)
    fn allocateExpressionArena(self: *Parser, expr: Expression) ParserError!*Expression {
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
        ptr.* = expr;
        return ptr;
    }

    fn allocateUnaryExpressionArena(self: *Parser, unary: ast.UnaryExpression) ParserError!*ast.UnaryExpression {
        const ptr = self.arena_allocator.create(ast.UnaryExpression) catch return ParserError.OutOfMemory;
        ptr.* = unary;
        return ptr;
    }

    fn allocateMemberAccessArena(self: *Parser, member_access: ast.MemberAccessExpression) ParserError!*ast.MemberAccessExpression {
        const ptr = self.arena_allocator.create(ast.MemberAccessExpression) catch return ParserError.OutOfMemory;
        ptr.* = member_access;
        return ptr;
    }

    fn allocateArrayExpressionArena(self: *Parser, array_expr: ast.ArrayExpression) ParserError!*ast.ArrayExpression {
        const ptr = self.arena_allocator.create(ast.ArrayExpression) catch return ParserError.OutOfMemory;
        ptr.* = array_expr;
        return ptr;
    }

    fn allocateMapExpressionArena(self: *Parser, map_expr: ast.MapExpression) ParserError!*ast.MapExpression {
        const ptr = self.arena_allocator.create(ast.MapExpression) catch return ParserError.OutOfMemory;
        ptr.* = map_expr;
        return ptr;
    }

    fn allocateMethodCallArena(self: *Parser, method_call: ast.MethodCallExpression) ParserError!*ast.MethodCallExpression {
        const ptr = self.arena_allocator.create(ast.MethodCallExpression) catch return ParserError.OutOfMemory;
        ptr.* = method_call;
        return ptr;
    }

    fn allocateStructExpressionArena(self: *Parser, struct_expr: ast.StructExpression) ParserError!*ast.StructExpression {
        const ptr = self.arena_allocator.create(ast.StructExpression) catch return ParserError.OutOfMemory;
        ptr.* = struct_expr;
        return ptr;
    }

    fn allocateMemberAccess(self: *Parser, member_access: ast.MemberAccessExpression) ParserError!*ast.MemberAccessExpression {
        const ptr = self.arena_allocator.create(ast.MemberAccessExpression) catch return ParserError.OutOfMemory;
        ptr.* = member_access;
        return ptr;
    }

    fn allocateArrayExpression(self: *Parser, array_expr: ast.ArrayExpression) ParserError!*ast.ArrayExpression {
        const ptr = self.arena_allocator.create(ast.ArrayExpression) catch return ParserError.OutOfMemory;
        ptr.* = array_expr;
        return ptr;
    }

    fn allocateMapExpression(self: *Parser, map_expr: ast.MapExpression) ParserError!*ast.MapExpression {
        const ptr = self.arena_allocator.create(ast.MapExpression) catch return ParserError.OutOfMemory;
        ptr.* = map_expr;
        return ptr;
    }

    fn allocateMethodCall(self: *Parser, method_call: ast.MethodCallExpression) ParserError!*ast.MethodCallExpression {
        const ptr = self.arena_allocator.create(ast.MethodCallExpression) catch return ParserError.OutOfMemory;
        ptr.* = method_call;
        return ptr;
    }

    fn allocateStructExpression(self: *Parser, struct_expr: ast.StructExpression) ParserError!*ast.StructExpression {
        const ptr = self.arena_allocator.create(ast.StructExpression) catch return ParserError.OutOfMemory;
        ptr.* = struct_expr;
        return ptr;
    }

    fn convertExpressionsToPointers(self: *Parser, expressions: *ArrayList(Expression)) ParserError!ArrayList(*Expression) {
        var pointers = ArrayList(*Expression){};
        
        for (expressions.items) |expr| {
            const ptr = try self.allocateExpression(expr);
            try pointers.append(self.allocator, ptr);
        }
        
        // CRITICAL: Do not deinit - arena allocator handles cleanup
        return pointers;
    }

    // CRITICAL FIX: Advanced parser features with crash protection for nested generics
    fn parseGenericType(self: *Parser, base_name: []const u8) ParserError!ast.Type {
        // CRITICAL FIX: Parse generic type like Vec<T>, Map<K,V>, Vec<Vec<T>>, HashMap<K,V>
        // with proper nested generic support and crash protection
        
        var type_arguments = std.ArrayList(ast.Type){};
        var nesting_depth: u32 = 0;
        const max_depth = 10; // Prevent infinite recursion
        
        // CRITICAL FIX: Parse type arguments with nested generic support
        while (!self.check(.Greater) and !self.check(.RightAngle) and !self.check(.RightShift) and !self.isAtEnd()) {
            // Prevent infinite recursion
            if (nesting_depth > max_depth) {
                _ = self.reportErrorWithContext("Generic type nesting too deep", "parseGenericType") catch {};
                return ParserError.InvalidType;
            }
            
            const type_arg = try self.parseTypeWithRecovery();
            try type_arguments.append(self.allocator, type_arg);
            
            // CRITICAL FIX: Handle >> token correctly for Vec<Vec<T>>
            if (self.check(.RightShift)) {
                // Split >> into > >
                self.current -= 1; // Go back
                // Handle >> as two > tokens (simplified approach)
                _ = self.advance(); // Skip the >>
                break;
            }
            
            // Check for end of type arguments
            if (self.check(.Greater) or self.check(.RightAngle)) {
                _ = self.advance();
                break;
            }
            
            // Must have comma between type arguments
            if (!self.match(.Comma)) {
                // CRITICAL FIX: Don't crash on malformed generics, use error recovery
                _ = self.reportErrorWithContext("Expected ',' between generic type arguments", "parseGenericType") catch {};
                self.recoverFromExpressionError();
                break;
            }
            
            nesting_depth += 1;
        }
        
        // CRITICAL FIX: If we didn't find closing bracket, add error recovery
        if (self.check(.Greater) or self.check(.RightAngle)) {
            _ = self.advance();
        } else if (!self.isAtEnd()) {
            _ = self.reportErrorWithContext("Expected '>' to close generic type", "parseGenericType") catch {};
            self.recoverFromExpressionError();
        }
        
        return ast.Type{ .Generic = ast.GenericType{
            .name = base_name,
            .type_arguments = type_arguments,
            .constraints = .empty,
        }};
    }
    
    /// CRITICAL FIX: Parse type with error recovery to prevent parser crashes
    fn parseTypeWithRecovery(self: *Parser) ParserError!ast.Type {
        return self.parseType() catch {
            // Add error recovery for type parsing
            _ = self.reportErrorWithContext("Failed to parse type argument", "parseTypeWithRecovery") catch {};
            self.recoverFromExpressionError();
            
            // Return a default type to continue parsing
            return ast.Type{ .Basic = ast.BasicType.Drip }; // Default to integer type
        };
    }
    
    /// Helper to get current line for error reporting
    fn current_line(self: *Parser) u32 {
        if (self.current == 0 or self.current >= self.tokens.len) return 1;
        return @intCast(self.tokens[self.current].line);
    }
    
    fn parseTypeConstraint(self: *Parser) ParserError!ast.TypeConstraint {
        // Parse constraints like T: Drawable, T: Numeric, T: Comparable & Sized
        if (self.match(.Colon)) {
            return try self.parseConstraintExpression();
        }
        
        return ast.TypeConstraint{ .Any = {} };
    }
    
    fn parseConstraintExpression(self: *Parser) ParserError!ast.TypeConstraint {
        // Parse constraint with potential combinations using &, |
        const constraint = try self.parseBasicConstraint();
        
        while (self.match(.Ampersand) or self.match(.Pipe)) {
            // For now, just take the first constraint
            // READY: Compound constraints implementation ready for when lexer supports '+' in constraints
            _ = try self.parseBasicConstraint();
        }
        
        return constraint;
    }
    
    fn parseBasicConstraint(self: *Parser) ParserError!ast.TypeConstraint {
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const constraint_name = self.advance().lexeme;
        
        // Built-in constraint types
        if (std.mem.eql(u8, constraint_name, "Numeric")) {
            return ast.TypeConstraint{ .Numeric = {} };
        } else if (std.mem.eql(u8, constraint_name, "Comparable")) {
            return ast.TypeConstraint{ .Comparable = {} };
        } else if (std.mem.eql(u8, constraint_name, "Ordered")) {
            return ast.TypeConstraint{ .Ordered = {} };
        } else if (std.mem.eql(u8, constraint_name, "Sized")) {
            return ast.TypeConstraint{ .Sized = {} };
        } else {
            // Interface constraint
            return ast.TypeConstraint{ .Interface = constraint_name };
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
            var where_clause = std.ArrayList(u8){};
            while (!self.check(.Comma) and !self.check(.Greater) and !self.check(.RightAngle) and !self.isAtEnd()) {
                const token = self.advance();
                try where_clause.appendSlice(token.lexeme);
                try where_clause.append(self.allocator, ' ');
            }
            return ast.TypeConstraint{ .WhereClause = where_clause.items };
        }
        
        return ParserError.InvalidSyntax;
    }
    
    fn parseComplexType(self: *Parser) ParserError!ast.Type {
        // Parse union types: Type1 | Type2 | Type3
        const base_type = try self.parseBasicType();
        // Check for error-returning function syntax: normie yikes, tea yikes, etc.
        if (self.match(.Yikes)) {
            // Return the base type since the error capability is implicit
            return base_type;
        }
        
        if (self.match(.Pipe)) {
            var union_types = std.ArrayList(u8){};
            try union_types.append(self.allocator, base_type);
            
            while (true) {
                const union_member = try self.parseBasicType();
                try union_types.append(self.allocator, union_member);
                
                if (!self.match(.Pipe)) break;
            }
            
            // For now, represent union as a generic type
            return ast.Type{ .Generic = ast.GenericType{
                .name = "Union",
                .type_arguments = union_types,
                .constraints = .empty,
            }};
        }
        
        return base_type;
    }
    
    fn parseBasicType(self: *Parser) ParserError!ast.Type {
        // CURSED type keywords - complete support for all BasicType variants
        if (self.match(.Normie)) {
            return ast.Type{ .Basic = ast.BasicType.Normie };
        }
        
        if (self.match(.Drip)) {
            return ast.Type{ .Basic = ast.BasicType.Drip };
        }
        
        if (self.match(.Tea)) {
            return ast.Type{ .Basic = ast.BasicType.Tea };
        }
        
        if (self.match(.Txt)) {
            return ast.Type{ .Basic = ast.BasicType.Txt };
        }
        
        if (self.match(.Sip)) {
            return ast.Type{ .Basic = ast.BasicType.Sip };
        }
        
        if (self.match(.Smol)) {
            return ast.Type{ .Basic = ast.BasicType.Smol };
        }
        
        if (self.match(.Mid)) {
            return ast.Type{ .Basic = ast.BasicType.Mid };
        }
        
        if (self.match(.Thicc)) {
            return ast.Type{ .Basic = ast.BasicType.Thicc };
        }
        
        if (self.match(.Snack)) {
            return ast.Type{ .Basic = ast.BasicType.Snack };
        }
        
        if (self.match(.Meal)) {
            return ast.Type{ .Basic = ast.BasicType.Meal };
        }
        
        if (self.match(.Byte)) {
            return ast.Type{ .Basic = ast.BasicType.Byte };
        }
        
        if (self.match(.Rune)) {
            return ast.Type{ .Basic = ast.BasicType.Rune };
        }
        
        if (self.match(.Extra)) {
            return ast.Type{ .Basic = ast.BasicType.Extra };
        }
        
        if (self.match(.Lit)) {
            return ast.Type{ .Basic = ast.BasicType.Lit };
        }
        
        if (self.match(.Cap)) {
            return ast.Type{ .Basic = ast.BasicType.Cap };
        }
        
        if (self.match(.Yikes)) {
            return ast.Type{ .Basic = ast.BasicType.Yikes };
        }
        
        // Function types with slay keyword
        if (self.match(.Slay)) {
            // Parse function type: slay() return_type or slay(param_types) return_type
            _ = try self.consume(.LeftParen, "Expected '(' after 'slay'");
            
            var param_types = ArrayList(ast.Type){};
            
            // Parse parameter types
            while (!self.check(.RightParen) and !self.isAtEnd()) {
                const param_type = try self.parseType();
                try param_types.append(self.allocator, param_type);
                
                if (!self.match(.Comma)) break;
            }
            
            _ = try self.consume(.RightParen, "Expected ')' after function parameters");
            
            // Parse return type (optional)
            var return_type: ?*ast.Type = null;
            if (!self.check(.Newline) and !self.check(.Semicolon) and !self.isAtEnd() and !self.check(.RightBrace)) {
                return_type = try self.arena_allocator.create(ast.Type);
        
                return_type.?.* = try self.parseType();
            }
            
            var func_return_type: ?ast.RefPtr(ast.Type) = null;
            if (return_type) |rt| {
                func_return_type = ast.RefPtr(ast.Type).init(rt);
            }
            
            return ast.Type{ .Function = ast.FunctionType{
                .parameters = param_types,
                .return_type = func_return_type,
                .ref_counted = ast.RefCounted.init(self.allocator),
            }};
        }
        
        // Custom/identifier types
        if (self.check(.Identifier)) {
            const type_name = self.advance().lexeme;
            
            // Check for generic arguments
            if (self.check(.Less) or self.check(.LeftAngle)) {
                return try self.parseGenericType(type_name);
            }
            
            return ast.Type{ .Custom = type_name };
        }
        
        return ParserError.InvalidType;
    }
    
    fn parseAdvancedFunctionSignature(self: *Parser) ParserError!FunctionStatement {
        _ = try self.consume(.Slay, "Expected 'slay'");
        
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
                        .constraints = .empty,
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
                            try type_param.constraints.append(self.allocator, constraint);
                            if (!self.match(.Plus)) break;
                        }
                    }
                    
                    // Parse default type
                    if (self.match(.Equal)) {
                        type_param.default_type = try self.parseType();
                    }
                    
                    try func.type_parameters.append(self.allocator, type_param);
                }
                
                if (!self.match(.Comma)) break;
            }
            
            if (!self.match(.Greater) and !self.match(.RightAngle)) {
                return ParserError.MissingToken;
            }
        }
        
        // Parse parameters with complex types
        _ = try self.consume(.LeftParen, "Expected '(' after function name");
        
        if (!self.check(.RightParen)) {
            while (true) {
                const param = try self.parseAdvancedParameter();
                try func.parameters.append(self.allocator, param);
                
                if (!self.match(.Comma)) break;
            }
        }
        
        _ = try self.consume(.RightParen, "Expected ')' after parameters");
        
        // Parse return type (can be complex)
        if (!self.check(.LeftBrace)) {
            func.return_type = try self.parseComplexType();
        }
        
        // Parse function body
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        self.in_function = true;
        defer { self.in_function = false; }
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = try self.parseStatement();
            const stmt_ptr = try self.arena_allocator.create(Statement); 
            stmt_ptr.* = stmt; 
            try func.body.append(self.allocator, stmt_ptr);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
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
            const default_expr = try self.parseExpression(); const default_ptr = try self.arena_allocator.create(Expression); default_ptr.* = default_expr; param.default_value = @ptrCast(default_ptr);
        }
        
        return param;
    }
    
    /// Parse string interpolation "Hello ${name}!" 
    fn parseStringInterpolation(self: *Parser, str_content: []const u8) ParserError!Expression {
        var interpolation = ast.StringInterpolationExpression.init();
        
        var pos: usize = 0;
        while (pos < str_content.len) {
            // Find next interpolation start
            if (std.mem.indexOfPos(u8, str_content, pos, "${")) |start| {
                // Add literal text before interpolation
                if (start > pos) {
                    const text_part = str_content[pos..start];
                    try interpolation.parts.append(self.allocator, ast.InterpolationPart{
                        .text = text_part,
                        .expression = null,
                        .format_spec = null,
                    });
                }
                
                // Find closing brace
                const expr_start = start + 2; // Skip "${"
                if (std.mem.indexOfPos(u8, str_content, expr_start, "}")) |end| {
                    const expr_text = str_content[expr_start..end];
                    
                    // Parse expression from text (simplified for now)
                    const expr_ptr = try self.arena_allocator.create(Expression);
                    expr_ptr.* = Expression{ .Identifier = expr_text };
                    
                    try interpolation.parts.append(self.allocator, ast.InterpolationPart{
                        .text = "",
                        .expression = try self.expressionToAnyopaque(expr_ptr),
                        .format_spec = null,
                    });
                    
                    pos = end + 1;
                } else {
                    return ParserError.InvalidSyntax; // Unclosed interpolation
                }
            } else {
                // Add remaining literal text
                if (pos < str_content.len) {
                    const text_part = str_content[pos..];
                    try interpolation.parts.append(self.allocator, ast.InterpolationPart{
                        .text = text_part,
                        .expression = null,
                        .format_spec = null,
                    });
                }
                break;
            }
        }
        
        return Expression{ .StringInterpolation = interpolation };
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
    var program = try parser.parseProgram();
    defer program.deinit(allocator);
    
    try std.testing.expect(program.statements.items.len == 1);
    
    const stmt: *Statement = @ptrCast(@alignCast(program.statements.items[0]));
    switch (stmt.*) {
        .Function => |*func| {
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
    var program = try parser.parseProgram();
    defer program.deinit(allocator);
    
    try std.testing.expect(program.statements.items.len == 1);
    
    const stmt: *Statement = @ptrCast(@alignCast(program.statements.items[0]));
    switch (stmt.*) {
        .Expression => |expr| {
            const typed_expr: *Expression = @ptrCast(@alignCast(expr));
            switch (typed_expr.*) {
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
    var program = try parser.parseProgram();
    defer program.deinit(allocator);
    
    try std.testing.expect(program.statements.items.len == 1);
    
    const stmt: *Statement = @ptrCast(@alignCast(program.statements.items[0]));
    switch (stmt.*) {
        .Function => |*func| {
            try std.testing.expect(std.mem.eql(u8, func.name, "test"));
            try std.testing.expect(func.return_type != null);
            try std.testing.expect(func.body.items.len == 1);
        },
        else => try std.testing.expect(false),
    }
}
