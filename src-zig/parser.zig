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
        std.debug.print("\n=== Error Recovery Statistics ===\n", .{});
        std.debug.print("Total errors encountered: {}\n", .{self.total_errors});
        std.debug.print("Semicolon recoveries: {}\n", .{self.semicolon_recoveries});
        std.debug.print("Statement recoveries: {}\n", .{self.statement_recoveries});
        std.debug.print("Expression recoveries: {}\n", .{self.expression_recoveries});
        std.debug.print("Delimiter recoveries: {}\n", .{self.delimiter_recoveries});
        std.debug.print("Total tokens skipped: {}\n", .{self.tokens_skipped});
        std.debug.print("================================\n", .{});
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
        };
    }

    pub fn deinit(self: *Parser) void {
        self.arena.deinit();
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

    pub fn parseProgram(self: *Parser) ParserError!Program {
        var program = Program.init(self.allocator);
        errdefer program.deinit(self.allocator);
        
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
                    program.imports.append(import_stmt) catch {
                        _ = self.reportErrorWithContext("Out of memory adding import", "parseProgram") catch {};
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
            if (self.parseStatement()) |stmt| {
                const stmt_ptr = self.allocator.create(Statement) catch {
                    _ = self.reportErrorWithContext("Out of memory allocating statement", "parseProgram") catch {};
                    return ParserError.OutOfMemory;
                };
                errdefer self.allocator.destroy(stmt_ptr);
                stmt_ptr.* = stmt;
                
                const anyopaque_ptr = self.statementToAnyopaque(stmt_ptr) catch |err| {
                    _ = self.reportErrorWithContext("Error converting statement to anyopaque", "parseProgram") catch {};
                    return err;
                };
                
                program.statements.append(anyopaque_ptr) catch {
                    _ = self.reportErrorWithContext("Out of memory adding statement to program", "parseProgram") catch {};
                    return ParserError.OutOfMemory;
                };
            } else |err| {
                const error_token = if (self.current < self.tokens.len) self.tokens[self.current] else self.tokens[self.tokens.len - 1];
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
        // Validate message before printing
        if (message.len == 0 or message.len > 1024) {
            std.debug.print("Error: Invalid error message (length: {})\n", .{message.len});
            self.had_error = true;
            return ParserError.SyntaxError;
        }
        
        const location = self.getCurrentSourceLocation();
        if (location) |loc| {
            // Bounds check for safe formatting
            if (loc.line < 65536 and loc.column < 65536) {
                std.debug.print("Error at {s}:{}:{} - {s} (context: {s})\n", .{ loc.file, loc.line, loc.column, message, context });
            } else {
                std.debug.print("Error in {s} - {s} (context: {s})\n", .{ loc.file, message, context });
            }
        } else {
            std.debug.print("Error: {s} (context: {s})\n", .{ message, context });
        }
        self.had_error = true;
        return ParserError.SyntaxError;
    }

    fn reportErrorAtToken(self: *Parser, token: Token, message: []const u8) ParserError {
        const location = self.getSourceLocationForToken(token);
        std.debug.print("Error at {s}:{}:{} - {s}\n", .{ location.file, location.line, location.column, message });
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
                std.debug.print("INFO: Recovered at semicolon after skipping {} tokens\n", .{tokens_skipped});
                return;
            }
            
            // Stop at newline (statement separator in many cases)
            if (current_token.kind == .Newline) {
                _ = self.advance(); // consume the newline
                self.error_recovery_stats.tokens_skipped += tokens_skipped;
                std.debug.print("INFO: Recovered at newline after skipping {} tokens\n", .{tokens_skipped});
                return;
            }
            
            // Stop at statement-starting keywords
            switch (current_token.kind) {
                .Slay, .Sus, .Facts, .Squad, .Collab, .Vibe, .Yeet, .Ready, .Lowkey, 
                .Periodt, .Flex, .Bestie, .Ghosted, .Simp, .Later, .Impl, .BeLike, 
                .Stan, .Match, .VibeCheck => {
                    // Don't consume these - let the next parsing cycle handle them
                    self.error_recovery_stats.tokens_skipped += tokens_skipped;
                    std.debug.print("INFO: Recovered at statement keyword '{s}' after skipping {} tokens\n", .{@tagName(current_token.kind), tokens_skipped});
                    return;
                },
                else => {},
            }
            
            // Stop at block delimiters that might indicate recovery points
            switch (current_token.kind) {
                .RightBrace, .RightParen, .RightBracket => {
                    // Don't consume these - they might be needed for proper parsing
                    self.error_recovery_stats.tokens_skipped += tokens_skipped;
                    std.debug.print("INFO: Recovered at delimiter '{s}' after skipping {} tokens\n", .{@tagName(current_token.kind), tokens_skipped});
                    return;
                },
                else => {},
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
        
        std.debug.print("INFO: Attempting additional statement recovery\n", .{});
        self.syncToAnyToken(&stmt_start_tokens);
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
        
        // Handle first import path
        if (!self.check(.StringLiteral) and !self.check(.String)) {
            return ParserError.UnexpectedToken;
        }
        
        const first_path_token = self.advance();
        const first_path = if (first_path_token.lexeme.len >= 2 and 
                              first_path_token.lexeme[0] == '"' and 
                              first_path_token.lexeme[first_path_token.lexeme.len-1] == '"')
                           first_path_token.lexeme[1..first_path_token.lexeme.len-1] // Remove quotes
                           else first_path_token.lexeme;
        
        var import_stmt = ast.ImportStatement.init(self.allocator, first_path);
        
        // Handle comma-separated additional imports
        while (self.match(.Comma)) {
            if (!self.check(.StringLiteral) and !self.check(.String)) {
                _ = self.reportErrorWithContext("Expected string literal after comma in import statement", "parseImportStatement") catch {};
                return ParserError.UnexpectedToken;
            }
            
            const path_token = self.advance();
            const path = if (path_token.lexeme.len >= 2 and 
                            path_token.lexeme[0] == '"' and 
                            path_token.lexeme[path_token.lexeme.len-1] == '"')
                         path_token.lexeme[1..path_token.lexeme.len-1] // Remove quotes
                         else path_token.lexeme;
            
            import_stmt.items.append(path) catch {
                _ = self.reportErrorWithContext("Out of memory adding import item", "parseImportStatement") catch {};
                return ParserError.OutOfMemory;
            };
        }
        
        // Handle alias (as name) - only for simple single imports
        if (import_stmt.items.items.len == 0 and self.match(.As)) {
            if (self.check(.Identifier)) {
                import_stmt.alias = self.advance().lexeme;
            } else {
                _ = self.reportErrorWithContext("Expected identifier after 'as' in import statement", "parseImportStatement") catch {};
                return ParserError.UnexpectedToken;
            }
        }
        
        return import_stmt;
    }

    fn parseStatement(self: *Parser) ParserError!Statement {
        // Skip comments at statement level
        while (self.check(.LineComment) or self.check(.BlockComment) or self.check(.Comment)) {
            _ = self.advance();
        }
        
        // Function declaration (slay) with enhanced error recovery
        if (self.check(.Slay)) {
            return Statement{ .Function = self.parseFunctionStatement() catch |err| {
                const error_token = if (self.current < self.tokens.len) self.tokens[self.current] else self.tokens[self.tokens.len - 1];
                _ = self.reportErrorAtToken(error_token, "Error parsing function statement") catch {};
                
                // Try to recover to the end of the function
                self.syncToMatchingDelimiter(.LeftBrace, .RightBrace);
                self.recoverFromStatementError();
                return err;
            }};
        }
        
        // Variable declaration (sus/facts) with enhanced error recovery
        if (self.check(.Sus) or self.check(.Facts)) {
            return Statement{ .Let = self.parseLetStatement() catch |err| {
                const error_token = if (self.current < self.tokens.len) self.tokens[self.current] else self.tokens[self.tokens.len - 1];
                _ = self.reportErrorAtToken(error_token, "Error parsing variable declaration") catch {};
                
                // Sync to semicolon for variable declarations
                self.syncToSemicolon();
                return err;
            }};
        }
        
        // Return statement (damn only - canonical spec)
        if (self.matchIdentifier("damn")) {
            return try self.parseReturnStatement();
        }
        
        // If statement (lowkey/ready)
        if (self.check(.Lowkey) or self.check(.Ready)) {
            return Statement{ .If = try self.parseIfStatement() };
        }
        
        // While statement (periodt/flex/bestie)
        if (self.check(.Periodt) or self.check(.Flex) or self.check(.Bestie)) {
            return Statement{ .While = try self.parseWhileStatement() };
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
            std.debug.print("DEBUG: Parsing struct statement\n", .{});
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
            const expr_ptr = try self.allocator.create(Expression);
            errdefer self.allocator.destroy(expr_ptr);
            expr_ptr.* = match_expr;
            return Statement{ .Expression = try self.expressionToAnyopaque(expr_ptr) };
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
        
        // Expression statement with error handling
        const expr = self.parseExpression() catch |err| {
            _ = self.reportErrorWithContext("Error parsing expression statement", "parseStatement") catch {};
            self.synchronize();
            return err;
        };
        
        const expr_ptr = self.allocator.create(Expression) catch {
            _ = self.reportErrorWithContext("Out of memory allocating expression", "parseStatement") catch {};
            return ParserError.OutOfMemory;
        };
        errdefer self.allocator.destroy(expr_ptr);
        expr_ptr.* = expr;
        
        return Statement{ .Expression = self.expressionToAnyopaque(expr_ptr) catch |err| {
            _ = self.reportErrorWithContext("Error converting expression to anyopaque", "parseStatement") catch {};
            return err;
        }};
    }

    fn parseFunctionStatement(self: *Parser) ParserError!FunctionStatement {
        _ = try self.consume(.Slay, "Expected 'slay'");
        
        if (!self.check(.Identifier)) {
            _ = self.reportErrorWithContext("Expected function name after 'slay'", "parseFunctionStatement") catch {};
            return ParserError.UnexpectedToken;
        }
        
        const name = self.advance().lexeme;
        
        // Validate function name length
        if (name.len == 0 or name.len > 255) {
            _ = self.reportErrorWithContext("Invalid function name length", "parseFunctionStatement") catch {};
            return ParserError.InvalidFunction;
        }
        
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
        _ = try self.consume(.LeftParen, "Expected '(' after function name");
        
        if (!self.check(.RightParen)) {
            while (true) {
                const param = try self.parseParameter();
                try func.parameters.append(param);
                
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
            // Skip newlines
            if (self.match(.Newline)) continue;
            
            const stmt = try self.parseStatement();
            const stmt_ptr = try self.allocator.create(Statement); 
            errdefer self.allocator.destroy(stmt_ptr);
            stmt_ptr.* = stmt; 
            try func.body.append(stmt_ptr);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
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
            .type_annotation = null,
            .initializer = null,
            .is_mutable = is_mutable,
        };
        
        // Parse type annotation (sus x tea = "hello" or sus x: tea)
        if (self.match(.Colon)) {
            let_stmt.var_type = try self.parseType();
            let_stmt.type_annotation = let_stmt.var_type;
        } else if (self.checkType()) {
            let_stmt.var_type = try self.parseType();
            let_stmt.type_annotation = let_stmt.var_type;
        }
        
        // Parse initializer
        if (self.match(.Equal) or self.match(.ColonEqual)) {
            const init_expr = try self.parseExpression();
            const init_ptr = try self.allocator.create(Expression);
            errdefer self.allocator.destroy(init_ptr);
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
            const default_expr = try self.parseExpression(); 
            const default_ptr = try self.allocator.create(Expression); 
            errdefer self.allocator.destroy(default_ptr);
            default_ptr.* = default_expr; 
            param.default_value = try self.expressionToAnyopaque(default_ptr);
        }
        
        return param;
    }

    fn parseType(self: *Parser) ParserError!ast.Type {
        // Check for array types first []element_type or [size]element_type
        if (self.match(.LeftBracket)) {
            // Check for size or empty for slice
            var size: ?usize = null;
            if (!self.check(.RightBracket)) {
                if (self.check(.Number) or self.check(.Integer)) {
                    const size_token = self.advance();
                    size = std.fmt.parseInt(usize, size_token.lexeme, 10) catch null;
                }
            }
            
        _ = try self.consume(.RightBracket, "Expected ']'");
            
            const element_type_ptr = try self.allocator.create(ast.Type);
            errdefer self.allocator.destroy(element_type_ptr);
            element_type_ptr.* = try self.parseType();
            
            return ast.Type{ .Array = ast.ArrayType{
                .element_type = element_type_ptr,
                .size = size,
            }};
        }
        
        // Check for basic types (drip, tea, lit, etc.)
        if (self.checkBasicType()) {
            const base_type = try self.parseBasicType();
            
            // Check for array suffix []
            if (self.match(.LeftBracket)) {
                // Check for size or empty for slice
                var size: ?usize = null;
                if (!self.check(.RightBracket)) {
                    if (self.check(.Number) or self.check(.Integer)) {
                        const size_token = self.advance();
                        size = std.fmt.parseInt(usize, size_token.lexeme, 10) catch null;
                    }
                }
                
        _ = try self.consume(.RightBracket, "Expected ']'");
                
                const element_type_ptr = try self.allocator.create(ast.Type);
                errdefer self.allocator.destroy(element_type_ptr);
                element_type_ptr.* = base_type;
                
                return ast.Type{ .Array = ast.ArrayType{
                    .element_type = element_type_ptr,
                    .size = size,
                }};
            }
            
            return base_type;
        }

        // This block becomes redundant since we moved it to the top
        if (false and self.match(.LeftBracket)) {
            // Check for size or empty for slice
            var size: ?usize = null;
            if (!self.check(.RightBracket)) {
                if (self.check(.Number) or self.check(.Integer)) {
                    const size_token = self.advance();
                    size = std.fmt.parseInt(usize, size_token.lexeme, 10) catch null;
                }
            }
            
        _ = try self.consume(.RightBracket, "Expected ']'");
            
            const element_type_ptr = try self.allocator.create(ast.Type);
            errdefer self.allocator.destroy(element_type_ptr);
            element_type_ptr.* = try self.parseType();
            
            return ast.Type{ .Array = ast.ArrayType{
                .element_type = element_type_ptr,
                .size = size,
            }};
        }

        // Map types map[key_type]value_type
        if (self.matchIdentifier("map")) {
        _ = try self.consume(.LeftBracket, "Expected '[' after 'map'");
            
            const key_type_ptr = try self.allocator.create(ast.Type);
            errdefer self.allocator.destroy(key_type_ptr);
            key_type_ptr.* = try self.parseType();
            
        _ = try self.consume(.RightBracket, "Expected ']'");
            
            const value_type_ptr = try self.allocator.create(ast.Type);
            errdefer self.allocator.destroy(value_type_ptr);
            value_type_ptr.* = try self.parseType();
            
            return ast.Type{ .Map = ast.MapType{
                .key_type = key_type_ptr,
                .value_type = value_type_ptr,
            }};
        }

        // Channel types dm<element_type> or dm[element_type]
        if (self.check(.Dm) or self.matchIdentifier("dm")) {
            _ = self.advance();
            if (self.match(.Less) or self.match(.LeftAngle) or self.match(.LeftBracket)) {
                const element_type_ptr = try self.allocator.create(ast.Type);
                errdefer self.allocator.destroy(element_type_ptr);
                element_type_ptr.* = try self.parseType();
                
                // Match the corresponding closing bracket
                if (self.check(.Greater) or self.check(.RightAngle)) {
                    _ = try self.consume(.Greater, "Expected '>' after channel element type");
                } else if (self.check(.RightBracket)) {
                    _ = try self.consume(.RightBracket, "Expected ']' after channel element type");
                } else {
                    return ParserError.UnexpectedToken;
                }
                
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
                
        _ = try self.consume(.RightParen, "Expected ')'");
                
                var return_type: ?*ast.Type = null;
                if (self.match(.Arrow)) {
                    return_type = try self.allocator.create(ast.Type);
                    errdefer self.allocator.destroy(return_type.?);
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
            
        _ = try self.consume(.RightParen, "Expected ')'");
            
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

        // Function types with slay keyword
        if (self.match(.Slay)) {
            // Parse function type: slay() return_type or slay(param_types) return_type
            _ = try self.consume(.LeftParen, "Expected '(' after 'slay'");
            
            var param_types = ArrayList(ast.Type).init(self.allocator);
            
            // Parse parameter types
            while (!self.check(.RightParen) and !self.isAtEnd()) {
                const param_type = try self.parseType();
                try param_types.append(param_type);
                
                if (!self.match(.Comma)) break;
            }
            
            _ = try self.consume(.RightParen, "Expected ')' after function parameters");
            
            // Parse return type (optional)
            var return_type: ?*ast.Type = null;
            if (!self.check(.Newline) and !self.check(.Semicolon) and !self.isAtEnd()) {
                return_type = try self.allocator.create(ast.Type);
                errdefer self.allocator.destroy(return_type.?);
                return_type.?.* = try self.parseType();
            }
            
            return ast.Type{ .Function = ast.FunctionType{
                .parameters = param_types,
                .return_type = return_type,
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
            .Identifier => return true, // Could be a string variable
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
        if (self.match(.Bang) or self.match(.Minus) or self.match(.Plus)) {
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
        
        var try_body = ArrayList(*anyopaque).init(self.allocator);
        
        // Parse try body statements
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = try self.parseStatement();
            const stmt_ptr = try self.allocator.create(Statement);
            errdefer self.allocator.destroy(stmt_ptr);
            stmt_ptr.* = stmt;
            try try_body.append(try self.statementToAnyopaque(stmt_ptr));
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
            
            var catch_body = ArrayList(*anyopaque).init(self.allocator);
            
            while (!self.check(.RightBrace) and !self.isAtEnd()) {
                if (self.match(.Newline)) continue;
                
                const stmt = try self.parseStatement();
                const stmt_ptr = try self.allocator.create(Statement);
                errdefer self.allocator.destroy(stmt_ptr);
                stmt_ptr.* = stmt;
                try catch_body.append(try self.statementToAnyopaque(stmt_ptr));
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
            
            var finally_body = ArrayList(*anyopaque).init(self.allocator);
            
            while (!self.check(.RightBrace) and !self.isAtEnd()) {
                if (self.match(.Newline)) continue;
                
                const stmt = try self.parseStatement();
                const stmt_ptr = try self.allocator.create(Statement);
                errdefer self.allocator.destroy(stmt_ptr);
                stmt_ptr.* = stmt;
                try finally_body.append(try self.statementToAnyopaque(stmt_ptr));
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
                if (!self.check(.Identifier)) {
                    return ParserError.UnexpectedToken;
                }
                const property = self.advance().lexeme;
                
                // Check if this is a method call (identifier followed by parentheses)
                if (self.check(.LeftParen)) {
                    _ = self.advance(); // consume '('
                    
                    var arguments = ArrayList(*Expression).init(self.allocator);
                    
                    if (!self.check(.RightParen)) {
                        while (true) {
                            const arg = try self.parseExpression();
                            const arg_ptr = try self.allocator.create(Expression);
                            arg_ptr.* = arg;
                            try arguments.append(arg_ptr);

                            if (!self.match(.Comma)) break;
                        }
                    }

                    _ = try self.consume(.RightParen, "Expected ')' after method arguments");

                    expr = Expression{ .MethodCall = try self.allocateMethodCall(ast.MethodCallExpression{
                        .object = try self.allocateExpression(expr),
                        .method_name = property,
                        .arguments = arguments,
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
        var arguments = ArrayList(*Expression).init(self.allocator);

        if (!self.check(.RightParen)) {
            while (true) {
                // Skip comments in argument lists
                while (self.check(.LineComment) or self.check(.BlockComment) or self.check(.Comment)) {
                    _ = self.advance();
                }
                
                if (self.check(.RightParen)) break;
                
                const arg = try self.parseExpression();
                const arg_ptr = try self.allocator.create(Expression);
                arg_ptr.* = arg;
                try arguments.append(arg_ptr);

                if (!self.match(.Comma)) break;
                
                // Skip comments after comma
                while (self.check(.LineComment) or self.check(.BlockComment) or self.check(.Comment)) {
                    _ = self.advance();
                }
            }
        }

        _ = try self.consume(.RightParen, "Expected ')' after arguments");

        return Expression{ .Call = .{
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
            
            // Check for string interpolation patterns
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
            var elements = ArrayList(Expression).init(self.allocator);
            
            if (!self.check(.RightBracket)) {
                while (true) {
                    const elem = try self.parseExpression();
                    try elements.append(elem);
                    
                    if (!self.match(.Comma)) break;
                }
            }
            
        _ = try self.consume(.RightBracket, "Expected ']'");
            
            return Expression{ .Array = try self.allocateArrayExpression(ast.ArrayExpression{
                .elements = try self.convertExpressionsToPointers(elements),
            })};
        }

        // Grouped expressions and tuples (expr) or (1, 2, 3)
        if (self.match(.LeftParen)) {
            // Look ahead to see if this is a tuple or just grouped expression
            if (self.check(.RightParen)) {
                // Empty tuple ()
                _ = self.advance();
                return Expression{ .Tuple = ast.TupleExpression{
                    .elements = ArrayList(*Expression).init(self.allocator),
                }};
            }
            
            var elements = ArrayList(Expression).init(self.allocator);
            var has_comma = false;
            
            while (true) {
                // Parse expression with full precedence
                const elem = try self.parseExpression();
                try elements.append(elem);
                
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
        _ = try self.consume(.Colon, "Expected ':' after map key");
                    const value = try self.parseExpression();
                    
                    const key_ptr = try self.allocator.create(Expression);
                    key_ptr.* = key;
                    const value_ptr = try self.allocator.create(Expression);
                    value_ptr.* = value;
                    
                    try entries.append(ast.MapEntry{
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
                    const buffer_ptr = try self.allocator.create(Expression);
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
                    const buffer_ptr = try self.allocator.create(Expression);
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
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        // Support both StructFieldAssignment (legacy) and FieldInitializer (new)
        var fields = ArrayList(ast.FieldInitializer).init(self.allocator);
        
        if (!self.check(.RightBrace)) {
            while (true) {
                if (!self.check(.Identifier)) {
                    return ParserError.UnexpectedToken;
                }
                
                const field_name = self.advance().lexeme;
        _ = try self.consume(.Colon, "Expected ':' after field name");
                const value = try self.parseExpression();
                
                const value_ptr = try self.allocator.create(Expression);
                value_ptr.* = value;
                
                try fields.append(ast.FieldInitializer{
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
        
        var cases = ArrayList(ast.MatchCase).init(self.allocator);
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
                const g_ptr = try self.allocator.create(Expression);
                g_ptr.* = g;
                guard_ptr = try self.expressionToAnyopaque(g_ptr);
            }
            
            const result_ptr = try self.allocator.create(Expression);
            result_ptr.* = result;
            
            try cases.append(ast.MatchCase{
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
        var patterns = ArrayList(ast.Pattern).init(self.allocator);
        
        const first_pattern = try self.parsePatternRange();
        try patterns.append(first_pattern);
        
        while (self.match(.Pipe)) {
            const pattern = try self.parsePatternRange();
            try patterns.append(pattern);
        }
        
        if (patterns.items.len == 1) {
            const single_pattern = patterns.items[0];
            patterns.deinit();
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
            const start_expr = try self.allocator.create(Expression);
            const end_expr = try self.allocator.create(Expression);
            
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
            var patterns = ArrayList(ast.Pattern).init(self.allocator);
            
            if (!self.check(.RightBracket)) {
                while (true) {
                    const pattern = try self.parsePattern();
                    try patterns.append(pattern);
                    
                    if (!self.match(.Comma)) break;
                }
            }
            
            _ = try self.consume(.RightBracket, "Expected ']'");
            return ast.Pattern{ .Array = patterns };
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
            
            _ = try self.consume(.RightParen, "Expected ')'");
            return ast.Pattern{ .Tuple = patterns };
        }

        // Struct pattern or Variable pattern
        if (self.check(.Identifier)) {
            const name = self.advance().lexeme;
            
            // Struct pattern StructName{field: pattern, ...}
            if (self.match(.LeftBrace)) {
                var fields = ArrayList(ast.FieldPattern).init(self.allocator);
                
                if (!self.check(.RightBrace)) {
                    while (true) {
                        if (!self.check(.Identifier)) {
                            return ParserError.UnexpectedToken;
                        }
                        
                        const field_name = self.advance().lexeme;
                        _ = try self.consume(.Colon, "Expected ':' after field name");
                        const pattern = try self.parsePattern();
                        
                        try fields.append(ast.FieldPattern{
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
        // SPEC CONFORMANCE: "damn" token has already been consumed by caller
        var return_stmt = ast.ReturnStatement{ .value = null };
        
        // Parse optional return value
        if (!self.check(.Semicolon) and !self.check(.Newline) and !self.isAtEnd() and !self.check(.RightBrace)) {
            const value_expr = try self.parseExpression();
            const value_ptr = try self.allocator.create(Expression);
            errdefer self.allocator.destroy(value_ptr);
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
        
        _ = try self.consume(.LeftParen, "Expected '(' after if keyword");
        const condition = try self.parseExpression();
        _ = try self.consume(.RightParen, "Expected ')' after condition");
        
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        var then_branch = ArrayList(*anyopaque).init(self.allocator);
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = try self.parseStatement();
            const stmt_ptr = try self.allocator.create(Statement); 
            errdefer self.allocator.destroy(stmt_ptr);
            stmt_ptr.* = stmt; 
            try then_branch.append(try self.statementToAnyopaque(stmt_ptr));
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
        var else_branch: ?ArrayList(*anyopaque) = null;
        
        // Parse else clause (highkey/otherwise)
        if (self.match(.Highkey) or self.match(.Otherwise)) {
            var else_stmts = ArrayList(*anyopaque).init(self.allocator);
            
            if (self.check(.Lowkey) or self.check(.Ready)) {
                // else if
                const elif_stmt = try self.parseIfStatement();
                const if_stmt = Statement{ .If = elif_stmt };
                const if_stmt_ptr = try self.allocator.create(Statement);
                errdefer self.allocator.destroy(if_stmt_ptr);
                if_stmt_ptr.* = if_stmt;
                try else_stmts.append(try self.statementToAnyopaque(if_stmt_ptr));
            } else {
                // else block
        _ = try self.consume(.LeftBrace, "Expected '{'");
                
                while (!self.check(.RightBrace) and !self.isAtEnd()) {
                    if (self.match(.Newline)) continue;
                    
                    const stmt = try self.parseStatement();
                    const stmt_ptr = try self.allocator.create(Statement); 
                    errdefer self.allocator.destroy(stmt_ptr);
                    stmt_ptr.* = stmt; 
                    try else_stmts.append(try self.statementToAnyopaque(stmt_ptr));
                }
                
        _ = try self.consume(.RightBrace, "Expected '}'");
            }
            
            else_branch = else_stmts;
        }
        
        const condition_ptr = try self.allocator.create(Expression);
        errdefer self.allocator.destroy(condition_ptr);
        condition_ptr.* = condition;
        
        return ast.IfStatement{
            .condition = try self.expressionToAnyopaque(condition_ptr),
            .then_branch = then_branch,
            .else_branch = else_branch,
        };
    }

    fn parseWhileStatement(self: *Parser) ParserError!ast.WhileStatement {
        _ = self.advance(); // consume periodt/flex/bestie
        
        _ = try self.consume(.LeftParen, "Expected '(' after while keyword");
        const condition = try self.parseExpression();
        _ = try self.consume(.RightParen, "Expected ')' after condition");
        
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        var body = ArrayList(*Statement).init(self.allocator);
        self.in_loop = true;
        defer { self.in_loop = false; }
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = try self.parseStatement();
            const stmt_ptr = try self.allocator.create(Statement); stmt_ptr.* = stmt; try body.append(stmt_ptr);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
        const condition_ptr = try self.allocator.create(Expression);
        condition_ptr.* = condition;
        
        return ast.WhileStatement{
            .condition = condition_ptr,
            .body = body,
        };
    }

    fn parseForStatement(self: *Parser) ParserError!Statement {
        _ = try self.consume(.Bestie, "Expected 'bestie'");
        
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
            
        _ = try self.consume(.LeftBrace, "Expected '{'");
            
            var body = ArrayList(*Statement).init(self.allocator);
            self.in_loop = true;
            defer { self.in_loop = false; }
            
            while (!self.check(.RightBrace) and !self.isAtEnd()) {
                if (self.match(.Newline)) continue;
                
                const stmt = try self.parseStatement();
                const stmt_ptr = try self.allocator.create(Statement); stmt_ptr.* = stmt; try body.append(stmt_ptr);
            }
            
        _ = try self.consume(.RightBrace, "Expected '}'");
            
            var condition_ptr: ?*Expression = null;
            if (condition) |cond| {
                const cond_ptr = try self.allocator.create(Expression);
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
        
        var body = ArrayList(*Statement).init(self.allocator);
        self.in_loop = true;
        defer { self.in_loop = false; }
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = try self.parseStatement();
            const stmt_ptr = try self.allocator.create(Statement); stmt_ptr.* = stmt; try body.append(stmt_ptr);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}'");
        
        return Statement{ .For = ast.ForStatement{
            .init = if (init_stmt) |stmt| blk: {
                const stmt_ptr = try self.allocator.create(Statement);
                stmt_ptr.* = stmt;
                break :blk stmt_ptr;
            } else null,
            .condition = if (condition) |cond| blk: {
                const cond_ptr = try self.allocator.create(Expression);
                cond_ptr.* = cond;
                break :blk cond_ptr;
            } else null,
            .update = if (update) |stmt| blk: {
                const stmt_ptr = try self.allocator.create(Statement);
                stmt_ptr.* = stmt;
                break :blk stmt_ptr;
            } else null,
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
        _ = try self.consume(.Flex, "Expected 'flex' in range-for loop");
        
        // Parse iterable expression
        const iterable = try self.parseExpression();
        const iterable_ptr = try self.allocator.create(Expression);
        iterable_ptr.* = iterable;
        
        // Parse body
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        var body = ArrayList(*Statement).init(self.allocator);
        self.in_loop = true;
        defer { self.in_loop = false; }
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = try self.parseStatement();
            const stmt_ptr = try self.allocator.create(Statement); stmt_ptr.* = stmt; try body.append(stmt_ptr);
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
        _ = try self.consume(.LeftBrace, "Expected '{' after struct name");
        
        // Parse fields and methods
        var fields = ArrayList(ast.StructField).init(self.allocator);
        var methods = ArrayList(ast.FunctionStatement).init(self.allocator);
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            // Skip newlines
            if (self.match(.Newline)) {
                continue;
            }
            
            // Check for method definition (slay keyword)
            if (self.check(.Slay)) {
                const method = try self.parseStructMethod();
                try methods.append(method);
                continue;
            }
            
            // Parse visibility modifier for fields
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
        
        var parameters = ArrayList(ast.Parameter).init(self.allocator);
        
        if (!self.check(.RightParen)) {
            while (true) {
                const param = try self.parseParameter();
                try parameters.append(param);
                
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
        
        var body = ArrayList(*Statement).init(self.allocator);
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) {
                continue;
            }
            
            const stmt = try self.parseStatement();
            const stmt_ptr = try self.allocator.create(Statement);
            stmt_ptr.* = stmt;
            try body.append(stmt_ptr);
        }
        
        _ = try self.consume(.RightBrace, "Expected '}' after method body");
        
        return ast.FunctionStatement{
            .name = name,
            .parameters = parameters,
            .return_type = return_type,
            .body = body,
            .visibility = .Public,
            .is_async = false,
            .type_parameters = ArrayList(ast.TypeParameter).init(self.allocator),
            .comments = ArrayList(ast.Comment).init(self.allocator),
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
        _ = try self.consume(.LeftBrace, "Expected '{' after interface name");
        
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
        
        var parameters = ArrayList(Parameter).init(self.allocator);
        
        if (!self.check(.RightParen)) {
            while (true) {
                const param = try self.parseParameter();
                try parameters.append(param);
                
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
        
        var methods = ArrayList(ast.FunctionStatement).init(self.allocator);
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const method = try self.parseFunctionStatement();
            try methods.append(method);
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
        
        var body = ArrayList(*anyopaque).init(self.allocator);
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = try self.parseStatement();
            const stmt_ptr = try self.allocator.create(Statement); 
            stmt_ptr.* = stmt; 
            try body.append(try self.statementToAnyopaque(stmt_ptr));
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
            
            var body = ArrayList(*anyopaque).init(self.allocator);
            while (!self.check(.RightBrace) and !self.isAtEnd()) {
                if (self.match(.Newline)) continue;
                
                const stmt = try self.parseStatement();
                const stmt_ptr = try self.allocator.create(Statement); 
                stmt_ptr.* = stmt; 
                try body.append(try self.statementToAnyopaque(stmt_ptr));
            }
            
        _ = try self.consume(.RightBrace, "Expected '}'");
            
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
        _ = try self.consume(.VibeCheck, "Expected 'vibe check'");
        
        const expression = try self.parseExpression();
        const expression_ptr = try self.allocator.create(Expression);
        expression_ptr.* = expression;
        
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        var patterns = ArrayList(ast.PatternCase).init(self.allocator);
        var default_case: ?ArrayList(*Statement) = null;
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            // Check for default case
            if (self.match(.Basic)) {
        _ = try self.consume(.Colon, "Expected ':' after 'basic'");
                
                var default_stmts = ArrayList(*Statement).init(self.allocator);
                while (!self.check(.Mood) and !self.check(.Basic) and !self.check(.RightBrace) and !self.isAtEnd()) {
                    if (self.match(.Newline)) continue;
                    
                    const stmt = try self.parseStatement();
                    const stmt_ptr = try self.allocator.create(Statement); stmt_ptr.* = stmt; try default_stmts.append(stmt_ptr);
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
                    const guard_ptr = try self.allocator.create(Expression);
                    guard_ptr.* = guard_expr;
                    guard = guard_ptr;
                }
                
        _ = try self.consume(.Colon, "Expected ':' after case pattern");
                
                var case_body = ArrayList(*Statement).init(self.allocator);
                while (!self.check(.Mood) and !self.check(.Basic) and !self.check(.RightBrace) and !self.isAtEnd()) {
                    if (self.match(.Newline)) continue;
                    
                    const stmt = try self.parseStatement();
                    const stmt_ptr = try self.allocator.create(Statement); stmt_ptr.* = stmt; try case_body.append(stmt_ptr);
                }
                
                try patterns.append(ast.PatternCase{
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
        
        var cases = ArrayList(ast.SelectCase).init(self.allocator);
        var default_case: ?ArrayList(*Statement) = null;
        
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            // Check for default case
            if (self.match(.Basic)) {
        _ = try self.consume(.Colon, "Expected ':' after 'basic'");
                
                var default_stmts = ArrayList(*Statement).init(self.allocator);
                while (!self.check(.Mood) and !self.check(.Basic) and !self.check(.RightBrace) and !self.isAtEnd()) {
                    if (self.match(.Newline)) continue;
                    
                    const stmt = try self.parseStatement();
                    const stmt_ptr = try self.allocator.create(Statement); stmt_ptr.* = stmt; try default_stmts.append(stmt_ptr);
                }
                
                default_case = default_stmts;
                continue;
            }
            
            // Parse case
            if (self.match(.Mood)) {
                const channel_op = try self.parseChannelOperation();
                
        _ = try self.consume(.Colon, "Expected ':' after channel operation");
                
                var case_body = ArrayList(*anyopaque).init(self.allocator);
                while (!self.check(.Mood) and !self.check(.Basic) and !self.check(.RightBrace) and !self.isAtEnd()) {
                    if (self.match(.Newline)) continue;
                    
                    const stmt = try self.parseStatement();
                    const stmt_ptr = try self.allocator.create(Statement); 
                    stmt_ptr.* = stmt; 
                    try case_body.append(try self.statementToAnyopaque(stmt_ptr));
                }
                
                try cases.append(ast.SelectCase{
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
            const channel_ptr = try self.allocator.create(Expression);
            channel_ptr.* = channel;
            const value_ptr = try self.allocator.create(Expression);
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
                const actual_channel_ptr = try self.allocator.create(Expression);
                actual_channel_ptr.* = actual_channel;
                
                return ast.ChannelOperation{ .Receive = .{
                    .channel = actual_channel_ptr,
                    .variable = variable,
                }};
            } else {
                // Just receiving: <-channel
                const channel_ptr = try self.allocator.create(Expression);
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
        
        const stmt_ptr = try self.allocator.create(Statement);
        stmt_ptr.* = try self.parseStatement();
        
        return Statement{ .Defer = ast.DeferStatement{ .statement = stmt_ptr } };
    }

    fn parseYikesStatement(self: *Parser) ParserError!ast.YikesStatement {
        _ = try self.consume(.Yikes, "Expected 'yikes'");
        
        // Parse error message expression
        const message_expr = try self.parseExpression();
        const message_ptr = try self.allocator.create(Expression);
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
            .location = null,  // TODO: Set from current token location
        };
    }

    fn parseFamStatement(self: *Parser) ParserError!ast.FamStatement {
        _ = try self.consume(.Fam, "Expected 'fam'");
        
        // Parse try body block
        _ = try self.consume(.LeftBrace, "Expected '{'");
        
        var try_body = ArrayList(Statement).init(self.allocator);
        while (!self.check(.RightBrace) and !self.isAtEnd()) {
            if (self.match(.Newline)) continue;
            
            const stmt = try self.parseStatement();
            try try_body.append(stmt);
        }
        _ = try self.consume(.RightBrace, "Expected '}'");
        
        // Parse catch blocks
        var catch_blocks = ArrayList(ast.FamStatement.CatchBlock).init(self.allocator);
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
            var catch_body = ArrayList(Statement).init(self.allocator);
            while (!self.check(.RightBrace) and !self.isAtEnd()) {
                if (self.match(.Newline)) continue;
                
                const stmt = try self.parseStatement();
                try catch_body.append(stmt);
            }
            _ = try self.consume(.RightBrace, "Expected '}'");
            
            try catch_blocks.append(ast.FamStatement.CatchBlock{
                .error_variable = error_variable,
                .error_type = error_type,
                .body = catch_body,
            });
        }
        
        // Optional finally block (not supported by basic lexer yet)
        const finally_block: ?ArrayList(Statement) = null;
        // TODO: Enable when Finally token is added to lexer.zig
        // if (self.match(.Finally)) {
        //     _ = try self.consume(.LeftBrace, "Expected '{'");
        //     finally_block = ArrayList(Statement).init(self.allocator);
        //     while (!self.check(.RightBrace) and !self.isAtEnd()) {
        //         if (self.match(.Newline)) continue;
        //         
        //         const stmt = try self.parseStatement();
        //         try finally_block.?.append(stmt);
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
        const value_ptr = try self.allocator.create(Expression);
        value_ptr.* = value;
        
        return ast.ConstDecl{
            .name = name,
            .const_type = const_type,
            .value = @ptrCast(value_ptr),
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
            
        _ = try self.consume(.RightParen, "Expected ')'");
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
        
        _ = try self.consume(.ColonEqual, "Expected ':=' in short declaration");
        
        // Parse values
        var values = ArrayList(*Expression).init(self.allocator);
        
        if (self.match(.LeftParen)) {
            // Tuple values: (1, 2, 3)
            while (!self.check(.RightParen) and !self.isAtEnd()) {
                const value = try self.parseExpression();
                const value_ptr = try self.allocator.create(Expression);
                value_ptr.* = value;
                try values.append(value_ptr);
                
                if (!self.match(.Comma)) break;
            }
            
        _ = try self.consume(.RightParen, "Expected ')'");
        } else {
            // Single value or comma-separated: 1, 2
            const value = try self.parseExpression();
            const value_ptr = try self.allocator.create(Expression);
            value_ptr.* = value;
            try values.append(value_ptr);
            
            while (self.match(.Comma)) {
                const next_value = try self.parseExpression();
                const next_value_ptr = try self.allocator.create(Expression);
                next_value_ptr.* = next_value;
                try values.append(next_value_ptr);
            }
        }
        
        return Statement{ .ShortDeclaration = ast.ShortDeclarationStatement{
            .names = names,
            .values = values,
        }};
    }

    fn parseAssignmentStatement(self: *Parser) ParserError!Statement {
        const target = try self.parseExpression();
        const target_ptr = try self.allocator.create(Expression);
        errdefer self.allocator.destroy(target_ptr);
        target_ptr.* = target;
        
        // Check for assignment operators
        if (self.match(.Equal) or self.match(.PlusEqual) or self.match(.MinusEqual) or
           self.match(.StarEqual) or self.match(.SlashEqual) or self.match(.PercentEqual)) {
            const operator = self.previous().lexeme;
            const value = try self.parseExpression();
            const value_ptr = try self.allocator.create(Expression);
            errdefer self.allocator.destroy(value_ptr);
            value_ptr.* = value;
            
            return Statement{ .Assignment = ast.AssignmentStatement{
                .target = @ptrCast(target_ptr),
                .value = @ptrCast(value_ptr),
                .operator = operator,
            }};
        } else {
            // If no assignment operator found, this is an expression statement
            return Statement{ .Expression = try self.expressionToAnyopaque(target_ptr) };
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
               self.check(.Lit) or self.check(.Cap) or self.check(.Identifier);
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

    fn allocateMethodCall(self: *Parser, method_call: ast.MethodCallExpression) ParserError!*ast.MethodCallExpression {
        const ptr = self.allocator.create(ast.MethodCallExpression) catch return ParserError.OutOfMemory;
        ptr.* = method_call;
        return ptr;
    }

    fn allocateStructExpression(self: *Parser, struct_expr: ast.StructExpression) ParserError!*ast.StructExpression {
        const ptr = self.allocator.create(ast.StructExpression) catch return ParserError.OutOfMemory;
        ptr.* = struct_expr;
        return ptr;
    }

    fn convertExpressionsToPointers(self: *Parser, expressions: ArrayList(Expression)) ParserError!ArrayList(*Expression) {
        var pointers = ArrayList(*Expression).init(self.allocator);
        
        for (expressions.items) |expr| {
            const ptr = try self.allocateExpression(expr);
            try pointers.append(ptr);
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
            // TODO: Implement compound constraints
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
        const base_type = try self.parseBasicType();
        
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
        
        // Function types with slay keyword
        if (self.match(.Slay)) {
            // Parse function type: slay() return_type or slay(param_types) return_type
            _ = try self.consume(.LeftParen, "Expected '(' after 'slay'");
            
            var param_types = ArrayList(ast.Type).init(self.allocator);
            
            // Parse parameter types
            while (!self.check(.RightParen) and !self.isAtEnd()) {
                const param_type = try self.parseType();
                try param_types.append(param_type);
                
                if (!self.match(.Comma)) break;
            }
            
            _ = try self.consume(.RightParen, "Expected ')' after function parameters");
            
            // Parse return type (optional)
            var return_type: ?*ast.Type = null;
            if (!self.check(.Newline) and !self.check(.Semicolon) and !self.isAtEnd() and !self.check(.RightBrace)) {
                return_type = try self.allocator.create(ast.Type);
                errdefer self.allocator.destroy(return_type.?);
                return_type.?.* = try self.parseType();
            }
            
            return ast.Type{ .Function = ast.FunctionType{
                .parameters = param_types,
                .return_type = return_type,
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
        _ = try self.consume(.LeftParen, "Expected '(' after function name");
        
        if (!self.check(.RightParen)) {
            while (true) {
                const param = try self.parseAdvancedParameter();
                try func.parameters.append(param);
                
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
            const stmt_ptr = try self.allocator.create(Statement); 
            stmt_ptr.* = stmt; 
            try func.body.append(stmt_ptr);
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
            const default_expr = try self.parseExpression(); const default_ptr = try self.allocator.create(Expression); default_ptr.* = default_expr; param.default_value = @ptrCast(default_ptr);
        }
        
        return param;
    }
    
    /// Parse string interpolation "Hello ${name}!" 
    fn parseStringInterpolation(self: *Parser, str_content: []const u8) ParserError!Expression {
        var interpolation = ast.StringInterpolationExpression.init(self.allocator);
        
        var pos: usize = 0;
        while (pos < str_content.len) {
            // Find next interpolation start
            if (std.mem.indexOfPos(u8, str_content, pos, "${")) |start| {
                // Add literal text before interpolation
                if (start > pos) {
                    const text_part = str_content[pos..start];
                    try interpolation.parts.append(ast.InterpolationPart{
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
                    const expr_ptr = try self.allocator.create(Expression);
                    expr_ptr.* = Expression{ .Identifier = expr_text };
                    
                    try interpolation.parts.append(ast.InterpolationPart{
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
                    try interpolation.parts.append(ast.InterpolationPart{
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
