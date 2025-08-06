const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Enhanced token types for CURSED language with comprehensive error handling
const TokenType = enum {
    // Literals
    STRING,
    NUMBER,
    IDENTIFIER,
    BOOLEAN,      // based/cringe
    NULL,         // nah
    
    // Keywords
    VIBEZ,        // vibez.spill
    SPILL,        // spill
    DOT,          // .
    
    // Error handling keywords
    YIKES,        // yikes - error creation
    SHOOK,        // shook - error propagation
    FAM,          // fam - panic recovery
    
    // Function keywords
    SLAY,         // slay - function definition
    DAMN,         // damn - return statement
    
    // Variable keywords
    SUS,          // sus - mutable variable
    FACTS,        // facts - immutable variable
    
    // Control flow
    LOWKEY,       // lowkey - if statement
    HIGHKEY,      // highkey - else statement
    BESTIE,       // bestie - for loop
    PERIODT,      // periodt - while loop
    VIBE_CHECK,   // vibe_check - switch statement
    STAN,         // stan - goroutine
    READY,        // ready - select statement
    
    // Operators
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    EQUAL,        // =
    COLON_EQUAL,  // :=
    EQUAL_EQUAL,  // ==
    NOT_EQUAL,    // !=
    LESS,         // <
    GREATER,      // >
    LESS_EQUAL,   // <=
    GREATER_EQUAL,// >=
    LOGICAL_AND,  // &&
    LOGICAL_OR,   // ||
    QUESTION,     // ? (error propagation)
    
    // Delimiters
    LPAREN,       // (
    RPAREN,       // )
    LBRACE,       // {
    RBRACE,       // }
    LBRACKET,     // [
    RBRACKET,     // ]
    COMMA,        // ,
    SEMICOLON,    // ;
    COLON,        // :
    ARROW,        // ->
    
    // Special tokens for error recovery
    EOF,
    NEWLINE,
    UNTERMINATED_STRING,  // For error recovery
    INVALID_NUMBER,       // For error recovery
    UNKNOWN,
    ERROR_TOKEN,          // Generic error token
};

const Token = struct {
    type: TokenType,
    value: []const u8,
    line: usize,
    column: usize,
};

// Enhanced AST node types with error handling support
const ASTNode = union(enum) {
    FunctionCall: struct {
        name: []const u8,
        args: ArrayList(ASTNode),
    },
    StringLiteral: []const u8,
    NumberLiteral: f64,
    Identifier: []const u8,
    
    // Error handling AST nodes
    YikesStatement: struct {
        name: []const u8,
        value: ?*ASTNode,
    },
    ShookExpression: struct {
        expression: *ASTNode,
    },
    FamStatement: struct {
        body: ArrayList(ASTNode),
        recovery: ?ArrayList(ASTNode),
    },
};

// Enhanced lexer with error handling keywords
const Lexer = struct {
    input: []const u8,
    position: usize,
    line: usize,
    column: usize,
    allocator: Allocator,

    pub fn init(allocator: Allocator, input: []const u8) Lexer {
        return Lexer{
            .input = input,
            .position = 0,
            .line = 1,
            .column = 1,
            .allocator = allocator,
        };
    }

    pub fn tokenize(self: *Lexer) !ArrayList(Token) {
        var tokens = ArrayList(Token).init(self.allocator);
        
        while (self.position < self.input.len) {
            self.skipWhitespace();
            
            if (self.position >= self.input.len) break;
            
            const start_pos = self.position;
            const start_line = self.line;
            const start_column = self.column;
            
            const ch = self.input[self.position];
            
            switch (ch) {
                '(' => {
                    _ = self.advance();
                    try tokens.append(Token{
                        .type = .LPAREN,
                        .value = self.input[start_pos..self.position],
                        .line = start_line,
                        .column = start_column,
                    });
                },
                ')' => {
                    _ = self.advance();
                    try tokens.append(Token{
                        .type = .RPAREN,
                        .value = self.input[start_pos..self.position],
                        .line = start_line,
                        .column = start_column,
                    });
                },
                '{' => {
                    _ = self.advance();
                    try tokens.append(Token{
                        .type = .LBRACE,
                        .value = self.input[start_pos..self.position],
                        .line = start_line,
                        .column = start_column,
                    });
                },
                '}' => {
                    _ = self.advance();
                    try tokens.append(Token{
                        .type = .RBRACE,
                        .value = self.input[start_pos..self.position],
                        .line = start_line,
                        .column = start_column,
                    });
                },
                ',' => {
                    _ = self.advance();
                    try tokens.append(Token{
                        .type = .COMMA,
                        .value = self.input[start_pos..self.position],
                        .line = start_line,
                        .column = start_column,
                    });
                },
                ';' => {
                    _ = self.advance();
                    try tokens.append(Token{
                        .type = .SEMICOLON,
                        .value = self.input[start_pos..self.position],
                        .line = start_line,
                        .column = start_column,
                    });
                },
                '=' => {
                    _ = self.advance();
                    try tokens.append(Token{
                        .type = .EQUAL,
                        .value = self.input[start_pos..self.position],
                        .line = start_line,
                        .column = start_column,
                    });
                },
                '.' => {
                    _ = self.advance();
                    try tokens.append(Token{
                        .type = .DOT,
                        .value = self.input[start_pos..self.position],
                        .line = start_line,
                        .column = start_column,
                    });
                },
                '"' => {
                    try self.readString(&tokens, start_line, start_column);
                },
                '\n' => {
                    _ = self.advance();
                    self.line += 1;
                    self.column = 1;
                    try tokens.append(Token{
                        .type = .NEWLINE,
                        .value = self.input[start_pos..self.position],
                        .line = start_line,
                        .column = start_column,
                    });
                },
                else => {
                    if (std.ascii.isDigit(ch)) {
                        try self.readNumber(&tokens, start_line, start_column);
                    } else if (std.ascii.isAlphabetic(ch)) {
                        try self.readIdentifier(&tokens, start_line, start_column);
                    } else {
                        // Skip unknown character
                        _ = self.advance();
                    }
                },
            }
        }
        
        try tokens.append(Token{
            .type = .EOF,
            .value = "",
            .line = self.line,
            .column = self.column,
        });
        
        return tokens;
    }

    fn advance(self: *Lexer) void {
        if (self.position < self.input.len) {
            self.position += 1;
            self.column += 1;
        }
    }

    fn skipWhitespace(self: *Lexer) void {
        while (self.position < self.input.len) {
            const ch = self.input[self.position];
            if (ch == ' ' or ch == '\t' or ch == '\r') {
                _ = self.advance();
            } else {
                break;
            }
        }
    }

    fn readString(self: *Lexer, tokens: *ArrayList(Token), line: usize, column: usize) !void {
        const start_pos = self.position;
        self.advance(); // Skip opening quote
        
        while (self.position < self.input.len and self.input[self.position] != '"') {
            // Handle escape sequences
            if (self.input[self.position] == '\\' and self.position + 1 < self.input.len) {
                self.advance(); // Skip escape character
            }
            self.advance();
        }
        
        if (self.position < self.input.len) {
            self.advance(); // Skip closing quote
            try tokens.append(Token{
                .type = .STRING,
                .value = self.input[start_pos..self.position],
                .line = line,
                .column = column,
            });
        } else {
            // Unterminated string - error recovery
            try tokens.append(Token{
                .type = .UNTERMINATED_STRING,
                .value = self.input[start_pos..self.position],
                .line = line,
                .column = column,
            });
        }
    }

    fn readNumber(self: *Lexer, tokens: *ArrayList(Token), line: usize, column: usize) !void {
        const start_pos = self.position;
        var has_dot = false;
        var is_valid = true;
        
        while (self.position < self.input.len) {
            const ch = self.input[self.position];
            if (std.ascii.isDigit(ch)) {
                self.advance();
            } else if (ch == '.' and !has_dot) {
                has_dot = true;
                self.advance();
                // Ensure there's a digit after the dot
                if (self.position >= self.input.len or !std.ascii.isDigit(self.input[self.position])) {
                    is_valid = false;
                    break;
                }
            } else {
                break;
            }
        }
        
        const token_type: TokenType = if (is_valid) .NUMBER else .INVALID_NUMBER;
        
        try tokens.append(Token{
            .type = token_type,
            .value = self.input[start_pos..self.position],
            .line = line,
            .column = column,
        });
    }

    fn readIdentifier(self: *Lexer, tokens: *ArrayList(Token), line: usize, column: usize) !void {
        const start_pos = self.position;
        
        while (self.position < self.input.len and 
               (std.ascii.isAlphanumeric(self.input[self.position]) or self.input[self.position] == '_')) {
            self.advance();
        }
        
        const value = self.input[start_pos..self.position];
        const token_type = self.getKeywordType(value);
        
        try tokens.append(Token{
            .type = token_type,
            .value = value,
            .line = line,
            .column = column,
        });
    }

    fn getKeywordType(self: *Lexer, value: []const u8) TokenType {
        _ = self;
        if (std.mem.eql(u8, value, "vibez")) return .VIBEZ;
        if (std.mem.eql(u8, value, "spill")) return .SPILL;
        if (std.mem.eql(u8, value, "yikes")) return .YIKES;
        if (std.mem.eql(u8, value, "shook")) return .SHOOK;
        if (std.mem.eql(u8, value, "fam")) return .FAM;
        return .IDENTIFIER;
    }
};

// Comprehensive parser error types
const ParseError = error{
    // Basic syntax errors
    ExpectedIdentifier,
    ExpectedLeftBrace,
    ExpectedRightBrace,
    ExpectedLeftParen,
    ExpectedRightParen,
    ExpectedSemicolon,
    UnexpectedToken,
    UnexpectedEof,
    
    // Function-related errors
    NotAFunctionCall,
    InvalidFunctionSignature,
    InvalidReturnType,
    
    // Expression errors
    InvalidExpression,
    UnclosedParentheses,
    InvalidBinaryOperator,
    InvalidUnaryOperator,
    
    // Statement errors
    InvalidStatement,
    InvalidAssignment,
    InvalidVariableDeclaration,
    
    // Error handling errors
    InvalidErrorHandling,
    InvalidYikesStatement,
    InvalidShookExpression,
    InvalidFamBlock,
    
    // Type errors
    InvalidType,
    UnknownType,
    
    // Memory errors
    OutOfMemory,
    
    // Recovery errors
    TooManyErrors,
    MaxRecoveryDepthExceeded,
};

// Error recovery strategy
const RecoveryStrategy = enum {
    SkipToNext,
    InsertToken,
    ReplaceToken,
    Backtrack,
    UseDefault,
    AbortScope,
    SynchronizeToStatement,
    SynchronizeToExpression,
};

// Enhanced error information with recovery context
const ParseErrorInfo = struct {
    error_type: ParseError,
    token: Token,
    message: []const u8,
    suggestions: ArrayList([]const u8),
    recovery_strategy: RecoveryStrategy,
    severity: ErrorSeverity,
    
    const ErrorSeverity = enum {
        Fatal,
        Error,
        Warning,
        Note,
        Help,
    };
};

// Enhanced parser with comprehensive error recovery and PGO support
const Parser = struct {
    tokens: []const Token,
    current: usize,
    allocator: Allocator,
    
    // Error recovery state
    errors: ArrayList(ParseErrorInfo),
    had_error: bool,
    panic_mode: bool,
    max_errors: usize,
    recovery_depth: usize,
    max_recovery_depth: usize,
    
    // Parsing context for better error messages
    in_function: bool,
    in_loop: bool,
    in_match: bool,
    scope_depth: usize,
    
    // Profile-guided optimization data
    pgo_data: ?*ProfileGuidedOptimizer,
    
    // Recovery points for backtracking
    recovery_points: ArrayList(RecoveryPoint),
    
    const RecoveryPoint = struct {
        position: usize,
        scope_depth: usize,
        error_count: usize,
    };

    pub fn init(allocator: Allocator, tokens: []const Token) Parser {
        return Parser{
            .tokens = tokens,
            .current = 0,
            .allocator = allocator,
            .errors = ArrayList(ParseErrorInfo).init(allocator),
            .had_error = false,
            .panic_mode = false,
            .max_errors = 100,
            .recovery_depth = 0,
            .max_recovery_depth = 10,
            .in_function = false,
            .in_loop = false,
            .in_match = false,
            .scope_depth = 0,
            .pgo_data = null,
            .recovery_points = ArrayList(RecoveryPoint).init(allocator),
        };
    }
    
    pub fn deinit(self: *Parser) void {
        for (self.errors.items) |error_info| {
            self.allocator.free(error_info.message);
            error_info.suggestions.deinit();
        }
        self.errors.deinit();
        self.recovery_points.deinit();
    }
    
    pub fn setPGOData(self: *Parser, pgo_data: *ProfileGuidedOptimizer) void {
        self.pgo_data = pgo_data;
    }

    /// Parse with comprehensive error recovery
    pub fn parse(self: *Parser) !ArrayList(ASTNode) {
        return self.parseWithRecovery();
    }
    
    pub fn parseWithRecovery(self: *Parser) !ArrayList(ASTNode) {
        var statements = ArrayList(ASTNode).init(self.allocator);
        errdefer {
            // Clean up any allocated AST nodes on error
            for (statements.items) |stmt| {
                self.freeASTNode(stmt);
            }
            statements.deinit();
        }
        
        while (!self.isAtEnd()) {
            // Skip newlines
            if (self.peek().type == .NEWLINE) {
                _ = self.advance();
                continue;
            }
            
            // Create recovery point before parsing statement
            try self.createRecoveryPoint();
            
            // Parse statement with recovery
            if (self.parseStatementWithRecovery()) |stmt| {
                try statements.append(stmt);
                self.removeLastRecoveryPoint();
            } else |err| {
                // Handle error with recovery strategies
                if (!try self.handleParseError(err)) {
                    // Too many errors or fatal error, abort
                    return err;
                }
                // Continue parsing after recovery
            }
            
            // Check if we've exceeded max errors
            if (self.errors.items.len >= self.max_errors) {
                self.reportError("Too many errors, aborting parse");
                return ParseError.TooManyErrors;
            }
        }
        
        // Report errors if any
        if (self.errors.items.len > 0) {
            self.reportAllErrors();
        }
        
        return statements;
    }
    
    /// Enhanced error recovery methods
    fn createRecoveryPoint(self: *Parser) !void {
        const point = RecoveryPoint{
            .position = self.current,
            .scope_depth = self.scope_depth,
            .error_count = self.errors.items.len,
        };
        try self.recovery_points.append(point);
    }
    
    fn removeLastRecoveryPoint(self: *Parser) void {
        if (self.recovery_points.items.len > 0) {
            _ = self.recovery_points.pop();
        }
    }
    
    fn recoverToPoint(self: *Parser, strategy: RecoveryStrategy) bool {
        switch (strategy) {
            .Backtrack => {
                if (self.recovery_points.items.len > 0) {
                    const point = self.recovery_points.pop();
                    self.current = point.position;
                    self.scope_depth = point.scope_depth;
                    // Remove errors that occurred after this point
                    while (self.errors.items.len > point.error_count) {
                        const error_info = self.errors.pop();
                        self.allocator.free(error_info.message);
                        error_info.suggestions.deinit();
                    }
                    return true;
                }
            },
            .SynchronizeToStatement => {
                self.synchronizeToStatement();
                return true;
            },
            .SynchronizeToExpression => {
                self.synchronizeToExpression();
                return true;
            },
            .SkipToNext => {
                _ = self.advance();
                return true;
            },
            else => {
                return false;
            },
        }
        return false;
    }
    
    fn synchronizeToStatement(self: *Parser) void {
        self.panic_mode = false;
        
        while (!self.isAtEnd()) {
            if (self.previous().type == .SEMICOLON) return;
            
            switch (self.peek().type) {
                .SLAY, .SUS, .FACTS, .LOWKEY, .BESTIE, .PERIODT,
                .VIBE_CHECK, .STAN, .READY, .YIKES, .FAM => return,
                else => {},
            }
            
            _ = self.advance();
        }
    }
    
    fn synchronizeToExpression(self: *Parser) void {
        while (!self.isAtEnd()) {
            switch (self.peek().type) {
                .SEMICOLON, .COMMA, .RPAREN, .RBRACE, .RBRACKET => return,
                else => {},
            }
            _ = self.advance();
        }
    }
    
    fn handleParseError(self: *Parser, err: ParseError) !bool {
        self.recovery_depth += 1;
        defer self.recovery_depth -= 1;
        
        if (self.recovery_depth > self.max_recovery_depth) {
            return false; // Max recovery depth exceeded
        }
        
        // Create error info
        const current_token = if (self.current < self.tokens.len) 
            self.tokens[self.current] else 
            Token{ .type = .EOF, .value = "", .line = 0, .column = 0 };
            
        const message = try self.createErrorMessage(err, current_token);
        const suggestions = try self.generateSuggestions(err, current_token);
        const strategy = self.selectRecoveryStrategy(err, current_token);
        
        const error_info = ParseErrorInfo{
            .error_type = err,
            .token = current_token,
            .message = message,
            .suggestions = suggestions,
            .recovery_strategy = strategy,
            .severity = self.getErrorSeverity(err),
        };
        
        try self.errors.append(error_info);
        
        // Apply recovery strategy
        return self.recoverToPoint(strategy);
    }
    
    fn createErrorMessage(self: *Parser, err: ParseError, token: Token) ![]u8 {
        const base_message = switch (err) {
            ParseError.ExpectedIdentifier => "Expected identifier",
            ParseError.ExpectedLeftBrace => "Expected '{'",
            ParseError.ExpectedRightBrace => "Expected '}'",
            ParseError.ExpectedLeftParen => "Expected '('",
            ParseError.ExpectedRightParen => "Expected ')'",
            ParseError.UnexpectedToken => "Unexpected token",
            ParseError.UnexpectedEof => "Unexpected end of file",
            ParseError.InvalidErrorHandling => "Invalid error handling syntax",
            ParseError.InvalidYikesStatement => "Invalid 'yikes' statement",
            ParseError.InvalidShookExpression => "Invalid 'shook' expression",
            ParseError.InvalidFamBlock => "Invalid 'fam' block",
            else => "Parse error",
        };
        
        return std.fmt.allocPrint(self.allocator, "{s} at line {}, column {}, got '{s}'", 
            .{ base_message, token.line, token.column, token.value });
    }
    
    fn generateSuggestions(self: *Parser, err: ParseError, token: Token) !ArrayList([]const u8) {
        var suggestions = ArrayList([]const u8).init(self.allocator);
        
        switch (err) {
            ParseError.ExpectedIdentifier => {
                try suggestions.append(try self.allocator.dupe(u8, "Try using a valid identifier name"));
                if (self.isKeyword(token.value)) {
                    try suggestions.append(try std.fmt.allocPrint(self.allocator, 
                        "'{s}' is a keyword, use a different name", .{token.value}));
                }
            },
            ParseError.ExpectedLeftBrace => {
                try suggestions.append(try self.allocator.dupe(u8, "Add '{' to start a block"));
            },
            ParseError.ExpectedRightBrace => {
                try suggestions.append(try self.allocator.dupe(u8, "Add '}' to close the block"));
            },
            ParseError.InvalidYikesStatement => {
                try suggestions.append(try self.allocator.dupe(u8, "Use: yikes error_name = \"error message\""));
            },
            ParseError.InvalidShookExpression => {
                try suggestions.append(try self.allocator.dupe(u8, "Use: shook expression"));
            },
            ParseError.InvalidFamBlock => {
                try suggestions.append(try self.allocator.dupe(u8, "Use: fam { ... }"));
            },
            else => {},
        }
        
        return suggestions;
    }
    
    fn selectRecoveryStrategy(self: *Parser, err: ParseError, token: Token) RecoveryStrategy {
        _ = token;
        
        return switch (err) {
            ParseError.ExpectedLeftBrace, ParseError.ExpectedRightBrace => .SynchronizeToStatement,
            ParseError.ExpectedLeftParen, ParseError.ExpectedRightParen => .SynchronizeToExpression,
            ParseError.UnexpectedToken => .SkipToNext,
            ParseError.UnexpectedEof => .UseDefault,
            ParseError.InvalidErrorHandling => .SynchronizeToStatement,
            else => .SynchronizeToStatement,
        };
    }
    
    fn getErrorSeverity(self: *Parser, err: ParseError) ParseErrorInfo.ErrorSeverity {
        _ = self;
        
        return switch (err) {
            ParseError.UnexpectedEof, ParseError.TooManyErrors => .Fatal,
            ParseError.ExpectedIdentifier, ParseError.ExpectedLeftBrace, 
            ParseError.UnexpectedToken => .Error,
            ParseError.InvalidErrorHandling => .Warning,
            else => .Error,
        };
    }
    
    fn isKeyword(self: *Parser, value: []const u8) bool {
        _ = self;
        
        const keywords = [_][]const u8{
            "vibez", "spill", "yikes", "shook", "fam", "slay", "damn",
            "sus", "facts", "lowkey", "highkey", "bestie", "periodt",
            "vibe_check", "stan", "ready", "based", "cringe", "nah"
        };
        
        for (keywords) |keyword| {
            if (std.mem.eql(u8, value, keyword)) {
                return true;
            }
        }
        
        return false;
    }
    
    fn reportAllErrors(self: *Parser) void {
        print("🚨 Parse completed with {} errors:\n", .{self.errors.items.len});
        
        for (self.errors.items, 0..) |error_info, i| {
            const severity_icon = switch (error_info.severity) {
                .Fatal => "💀",
                .Error => "❌", 
                .Warning => "⚠️",
                .Note => "ℹ️",
                .Help => "💡",
            };
            
            print("{s} Error {}: {s}\n", .{ severity_icon, i + 1, error_info.message });
            
            if (error_info.suggestions.items.len > 0) {
                print("   Suggestions:\n");
                for (error_info.suggestions.items) |suggestion| {
                    print("   - {s}\n", .{suggestion});
                }
            }
        }
    }
    
    fn parseStatementWithRecovery(self: *Parser) !ASTNode {
        return self.parseStatement();
    }

    fn parseStatement(self: *Parser) !ASTNode {
        const current_token = self.peek();
        
        switch (current_token.type) {
            .YIKES => return try self.parseYikesStatement(),
            .FAM => return try self.parseFamStatement(),
            .VIBEZ => return try self.parseExpression(),
            else => return try self.parseExpression(),
        }
    }

    fn parseYikesStatement(self: *Parser) !ASTNode {
        _ = self.advance(); // consume 'yikes'
        
        if (self.peek().type != .IDENTIFIER) {
            return error.ExpectedIdentifier;
        }
        
        const name = self.advance().value;
        var value: ?*ASTNode = null;
        
        if (self.peek().type == .EQUAL) {
            _ = self.advance(); // consume '='
            const value_node = try self.allocator.create(ASTNode);
            value_node.* = try self.parseExpression();
            value = value_node;
        }
        
        return ASTNode{ .YikesStatement = .{
            .name = name,
            .value = value,
        }};
    }

    fn parseFamStatement(self: *Parser) !ASTNode {
        _ = self.advance(); // consume 'fam'
        
        if (self.peek().type != .LBRACE) {
            return error.ExpectedLeftBrace;
        }
        _ = self.advance(); // consume '{'
        
        var body = ArrayList(ASTNode).init(self.allocator);
        
        while (self.peek().type != .RBRACE and !self.isAtEnd()) {
            if (self.peek().type == .NEWLINE) {
                _ = self.advance();
                continue;
            }
            const stmt = try self.parseStatement();
            try body.append(stmt);
        }
        
        if (self.peek().type == .RBRACE) {
            _ = self.advance(); // consume '}'
        }
        
        return ASTNode{ .FamStatement = .{
            .body = body,
            .recovery = null, // TODO: Add recovery parsing
        }};
    }

    fn parseExpression(self: *Parser) !ASTNode {
        const current_token = self.peek();
        
        switch (current_token.type) {
            .VIBEZ => {
                return try self.parseFunctionCall();
            },
            .STRING => {
                const token = self.advance();
                // Remove quotes from string value
                const str_value = if (token.value.len >= 2 and 
                                     token.value[0] == '"' and 
                                     token.value[token.value.len-1] == '"')
                                  token.value[1..token.value.len-1]
                                  else token.value;
                return ASTNode{ .StringLiteral = str_value };
            },
            .NUMBER => {
                const token = self.advance();
                const num = std.fmt.parseFloat(f64, token.value) catch 0.0;
                return ASTNode{ .NumberLiteral = num };
            },
            .IDENTIFIER => {
                const token = self.advance();
                return ASTNode{ .Identifier = token.value };
            },
            .SHOOK => {
                _ = self.advance(); // consume 'shook'
                const expr_node = try self.allocator.create(ASTNode);
                expr_node.* = try self.parseExpression();
                return ASTNode{ .ShookExpression = .{ .expression = expr_node }};
            },
            else => {
                return error.UnexpectedToken;
            },
        }
    }

    fn parseFunctionCall(self: *Parser) !ASTNode {
        const start = self.current;
        var name_parts = ArrayList([]const u8).init(self.allocator);
        defer name_parts.deinit();
        
        // Parse vibez.spill pattern
        if (self.peek().type == .VIBEZ) {
            try name_parts.append(self.advance().value);
            
            if (self.peek().type == .DOT) {
                _ = self.advance(); // consume '.'
                if (self.peek().type == .SPILL or self.peek().type == .IDENTIFIER) {
                    try name_parts.append(self.advance().value);
                }
            }
        }
        
        const name = try name_parts.toOwnedSlice();
        defer self.allocator.free(name);
        
        if (self.peek().type != .LPAREN) {
            // Not a function call, backtrack
            self.current = start;
            return error.NotAFunctionCall;
        }
        
        _ = self.advance(); // consume '('
        
        var args = ArrayList(ASTNode).init(self.allocator);
        
        while (self.peek().type != .RPAREN and !self.isAtEnd()) {
            if (args.items.len > 0) {
                if (self.peek().type == .COMMA) {
                    _ = self.advance(); // consume ','
                } else {
                    break;
                }
            }
            
            const arg = try self.parseExpression();
            try args.append(arg);
        }
        
        if (self.peek().type == .RPAREN) {
            _ = self.advance(); // consume ')'
        }
        
        const full_name = try std.mem.join(self.allocator, ".", name);
        
        return ASTNode{ .FunctionCall = .{
            .name = full_name,
            .args = args,
        }};
    }

    fn peek(self: *Parser) Token {
        if (self.current >= self.tokens.len) {
            return Token{ .type = .EOF, .value = "", .line = 0, .column = 0 };
        }
        return self.tokens[self.current];
    }

    fn advance(self: *Parser) Token {
        if (!self.isAtEnd()) {
            self.current += 1;
        }
        return self.previous();
    }

    fn previous(self: *Parser) Token {
        if (self.current == 0) {
            return Token{ .type = .EOF, .value = "", .line = 0, .column = 0 };
        }
        return self.tokens[self.current - 1];
    }

    fn isAtEnd(self: *Parser) bool {
        return self.peek().type == .EOF;
    }
    
    /// Free memory allocated for an AST node
    fn freeASTNode(self: *Parser, node: ASTNode) void {
        switch (node) {
            .FunctionCall => |call| {
                self.allocator.free(call.name);
                for (call.args.items) |arg| {
                    self.freeASTNode(arg);
                }
                call.args.deinit();
            },
            .YikesStatement => |yikes| {
                if (yikes.value) |value| {
                    self.freeASTNode(value.*);
                    self.allocator.destroy(value);
                }
            },
            .ShookExpression => |shook| {
                self.freeASTNode(shook.expression.*);
                self.allocator.destroy(shook.expression);
            },
            .FamStatement => |fam| {
                for (fam.body.items) |stmt| {
                    self.freeASTNode(stmt);
                }
                fam.body.deinit();
                if (fam.recovery) |recovery| {
                    for (recovery.items) |stmt| {
                        self.freeASTNode(stmt);
                    }
                    recovery.deinit();
                }
            },
            else => {
                // Other node types don't need explicit cleanup
            },
        }
    }
};

// Enhanced interpreter with error handling execution
const Interpreter = struct {
    allocator: Allocator,
    errors: ArrayList([]const u8),

    pub fn init(allocator: Allocator) Interpreter {
        return Interpreter{
            .allocator = allocator,
            .errors = ArrayList([]const u8).init(allocator),
        };
    }

    pub fn deinit(self: *Interpreter) void {
        for (self.errors.items) |error_msg| {
            self.allocator.free(error_msg);
        }
        self.errors.deinit();
    }

    pub fn execute(self: *Interpreter, ast: ArrayList(ASTNode)) !void {
        for (ast.items) |node| {
            try self.executeNode(node);
        }
    }

    fn executeNode(self: *Interpreter, node: ASTNode) !void {
        switch (node) {
            .FunctionCall => |call| {
                if (std.mem.eql(u8, call.name, "vibez.spill")) {
                    try self.executeVibezSpill(call.args);
                } else {
                    print("Unknown function: {s}\n", .{call.name});
                }
            },
            .YikesStatement => |yikes| {
                try self.executeYikesStatement(yikes);
            },
            .FamStatement => |fam| {
                try self.executeFamStatement(fam);
            },
            .ShookExpression => |shook| {
                try self.executeShookExpression(shook);
            },
            else => {
                print("Unsupported statement type\n", .{});
            },
        }
    }

    fn executeVibezSpill(self: *Interpreter, args: ArrayList(ASTNode)) !void {
        _ = self;
        for (args.items, 0..) |arg, i| {
            if (i > 0) print(" ", .{});
            try printNode(arg);
        }
        print("\n", .{});
    }

    fn executeYikesStatement(self: *Interpreter, yikes: anytype) !void {
        print("Error created: {s}", .{yikes.name});
        if (yikes.value) |value| {
            print(" = ");
            try printNode(value.*);
        }
        print("\n", .{});
        
        // Store error for potential propagation
        const error_msg = try std.fmt.allocPrint(self.allocator, "Error: {s}", .{yikes.name});
        try self.errors.append(error_msg);
    }

    fn executeFamStatement(self: *Interpreter, fam: anytype) !void {
        print("Executing fam block with error recovery\n", .{});
        
        // Execute body statements with error catching
        for (fam.body.items) |stmt| {
            self.executeNode(stmt) catch |err| {
                print("Caught error in fam block: {any}\n", .{err});
                if (fam.recovery) |recovery| {
                    print("Executing recovery block\n", .{});
                    for (recovery.items) |recovery_stmt| {
                        try self.executeNode(recovery_stmt);
                    }
                }
                return; // Exit after handling error
            };
        }
        
        print("Fam block completed successfully\n", .{});
    }

    fn executeShookExpression(self: *Interpreter, shook: anytype) !void {
        print("Executing shook expression with error propagation\n", .{});
        
        // Try to execute the wrapped expression
        self.executeNode(shook.expression.*) catch |err| {
            print("Error propagated by shook: {any}\n", .{err});
            return err; // Propagate the error up
        };
    }

    fn printNode(node: ASTNode) !void {
        switch (node) {
            .StringLiteral => |str| print("{s}", .{str}),
            .NumberLiteral => |num| print("{d}", .{num}),
            .Identifier => |id| print("{s}", .{id}),
            else => print("(complex expression)", .{}),
        }
    }
};

// Profile-Guided Optimizer for advanced optimization
const ProfileGuidedOptimizer = struct {
    allocator: Allocator,
    profile_data: ProfileData,
    optimization_level: OptimizationLevel,
    
    const OptimizationLevel = enum {
        Speed,
        Size,
        Balanced,
        Custom,
    };
    
    const ProfileData = struct {
        function_counts: std.HashMap([]const u8, u64, std.hash_map.StringContext, 80),
        basic_block_counts: std.HashMap([]const u8, u64, std.hash_map.StringContext, 80),
        total_execution_time: u64, // nanoseconds
        hot_functions: ArrayList([]const u8),
        cold_functions: ArrayList([]const u8),
        
        pub fn init(allocator: Allocator) ProfileData {
            return ProfileData{
                .function_counts = std.HashMap([]const u8, u64, std.hash_map.StringContext, 80).init(allocator),
                .basic_block_counts = std.HashMap([]const u8, u64, std.hash_map.StringContext, 80).init(allocator),
                .total_execution_time = 0,
                .hot_functions = ArrayList([]const u8).init(allocator),
                .cold_functions = ArrayList([]const u8).init(allocator),
            };
        }
        
        pub fn deinit(self: *ProfileData) void {
            self.function_counts.deinit();
            self.basic_block_counts.deinit();
            self.hot_functions.deinit();
            self.cold_functions.deinit();
        }
    };
    
    pub fn init(allocator: Allocator) ProfileGuidedOptimizer {
        return ProfileGuidedOptimizer{
            .allocator = allocator,
            .profile_data = ProfileData.init(allocator),
            .optimization_level = .Balanced,
        };
    }
    
    pub fn deinit(self: *ProfileGuidedOptimizer) void {
        self.profile_data.deinit();
    }
    
    pub fn setOptimizationLevel(self: *ProfileGuidedOptimizer, level: OptimizationLevel) void {
        self.optimization_level = level;
    }
    
    pub fn recordFunctionCall(self: *ProfileGuidedOptimizer, function_name: []const u8) !void {
        const result = try self.profile_data.function_counts.getOrPut(function_name);
        if (!result.found_existing) {
            result.value_ptr.* = 0;
        }
        result.value_ptr.* += 1;
    }
    
    pub fn recordBasicBlockExecution(self: *ProfileGuidedOptimizer, block_name: []const u8) !void {
        const result = try self.profile_data.basic_block_counts.getOrPut(block_name);
        if (!result.found_existing) {
            result.value_ptr.* = 0;
        }
        result.value_ptr.* += 1;
    }
    
    pub fn analyzeHotColdFunctions(self: *ProfileGuidedOptimizer, hot_threshold: u64) !void {
        var iterator = self.profile_data.function_counts.iterator();
        
        while (iterator.next()) |entry| {
            if (entry.value_ptr.* >= hot_threshold) {
                try self.profile_data.hot_functions.append(entry.key_ptr.*);
            } else {
                try self.profile_data.cold_functions.append(entry.key_ptr.*);
            }
        }
    }
    
    pub fn shouldInlineFunction(self: *ProfileGuidedOptimizer, function_name: []const u8) bool {
        if (self.profile_data.function_counts.get(function_name)) |count| {
            return switch (self.optimization_level) {
                .Speed => count > 1000,
                .Size => count > 10000,
                .Balanced => count > 5000,
                .Custom => count > 2500,
            };
        }
        return false;
    }
    
    pub fn getOptimizationRecommendations(self: *ProfileGuidedOptimizer) ![]const u8 {
        var recommendations = ArrayList(u8).init(self.allocator);
        
        try recommendations.appendSlice("PGO Optimization Recommendations:\n");
        
        if (self.profile_data.hot_functions.items.len > 0) {
            try recommendations.appendSlice("Hot functions (consider inlining):\n");
            for (self.profile_data.hot_functions.items) |func_name| {
                try recommendations.appendSlice("  - ");
                try recommendations.appendSlice(func_name);
                try recommendations.appendSlice("\n");
            }
        }
        
        if (self.profile_data.cold_functions.items.len > 0) {
            try recommendations.appendSlice("Cold functions (consider outlining):\n");
            for (self.profile_data.cold_functions.items) |func_name| {
                try recommendations.appendSlice("  - ");
                try recommendations.appendSlice(func_name);
                try recommendations.appendSlice("\n");
            }
        }
        
        return recommendations.toOwnedSlice();
    }
    
    pub fn exportProfileData(self: *ProfileGuidedOptimizer, file_path: []const u8) !void {
        const file = try std.fs.cwd().createFile(file_path, .{});
        defer file.close();
        
        var buffered_writer = std.io.bufferedWriter(file.writer());
        const writer = buffered_writer.writer();
        
        try writer.print("# CURSED Profile Data\n");
        try writer.print("total_execution_time: {}\n", .{self.profile_data.total_execution_time});
        try writer.print("\n[function_counts]\n");
        
        var iterator = self.profile_data.function_counts.iterator();
        while (iterator.next()) |entry| {
            try writer.print("{s}: {}\n", .{ entry.key_ptr.*, entry.value_ptr.* });
        }
        
        try writer.print("\n[basic_block_counts]\n");
        var bb_iterator = self.profile_data.basic_block_counts.iterator();
        while (bb_iterator.next()) |entry| {
            try writer.print("{s}: {}\n", .{ entry.key_ptr.*, entry.value_ptr.* });
        }
        
        try buffered_writer.flush();
    }
    
    pub fn importProfileData(self: *ProfileGuidedOptimizer, file_path: []const u8) !void {
        const file_content = try std.fs.cwd().readFileAlloc(self.allocator, file_path, 1024 * 1024);
        defer self.allocator.free(file_content);
        
        var lines = std.mem.split(u8, file_content, "\n");
        var current_section: []const u8 = "";
        
        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t\r");
            if (trimmed.len == 0 or trimmed[0] == '#') continue;
            
            if (std.mem.startsWith(u8, trimmed, "[") and std.mem.endsWith(u8, trimmed, "]")) {
                current_section = trimmed[1..trimmed.len-1];
                continue;
            }
            
            if (std.mem.indexOf(u8, trimmed, ":")) |colon_pos| {
                const key = std.mem.trim(u8, trimmed[0..colon_pos], " \t");
                const value_str = std.mem.trim(u8, trimmed[colon_pos+1..], " \t");
                
                if (std.mem.eql(u8, current_section, "function_counts")) {
                    const count = std.fmt.parseInt(u64, value_str, 10) catch continue;
                    try self.profile_data.function_counts.put(try self.allocator.dupe(u8, key), count);
                } else if (std.mem.eql(u8, current_section, "basic_block_counts")) {
                    const count = std.fmt.parseInt(u64, value_str, 10) catch continue;
                    try self.profile_data.basic_block_counts.put(try self.allocator.dupe(u8, key), count);
                } else if (std.mem.eql(u8, key, "total_execution_time")) {
                    self.profile_data.total_execution_time = std.fmt.parseInt(u64, value_str, 10) catch 0;
                }
            }
        }
    }
};

// Incremental parser for handling large files and real-time parsing
const IncrementalParser = struct {
    allocator: Allocator,
    cached_tokens: std.HashMap(usize, ArrayList(Token), std.hash_map.AutoContext(usize), 80),
    cached_asts: std.HashMap(usize, ArrayList(ASTNode), std.hash_map.AutoContext(usize), 80),
    file_fingerprints: std.HashMap([]const u8, u64, std.hash_map.StringContext, 80),
    
    pub fn init(allocator: Allocator) IncrementalParser {
        return IncrementalParser{
            .allocator = allocator,
            .cached_tokens = std.HashMap(usize, ArrayList(Token), std.hash_map.AutoContext(usize), 80).init(allocator),
            .cached_asts = std.HashMap(usize, ArrayList(ASTNode), std.hash_map.AutoContext(usize), 80).init(allocator),
            .file_fingerprints = std.HashMap([]const u8, u64, std.hash_map.StringContext, 80).init(allocator),
        };
    }
    
    pub fn deinit(self: *IncrementalParser) void {
        self.cached_tokens.deinit();
        self.cached_asts.deinit();
        self.file_fingerprints.deinit();
    }
    
    pub fn parseIncremental(self: *IncrementalParser, file_path: []const u8, content: []const u8) !ArrayList(ASTNode) {
        const fingerprint = self.calculateFingerprint(content);
        
        // Check if file has changed
        if (self.file_fingerprints.get(file_path)) |cached_fingerprint| {
            if (cached_fingerprint == fingerprint) {
                // File unchanged, return cached AST
                const cache_key = std.hash_map.hashString(file_path);
                if (self.cached_asts.get(cache_key)) |cached_ast| {
                    var result = ArrayList(ASTNode).init(self.allocator);
                    try result.appendSlice(cached_ast.items);
                    return result;
                }
            }
        }
        
        // File changed or not cached, parse and cache
        var lexer = Lexer.init(self.allocator, content);
        const tokens = try lexer.tokenize();
        defer tokens.deinit();
        
        var parser = Parser.init(self.allocator, tokens.items);
        defer parser.deinit();
        const ast = try parser.parseWithRecovery();
        
        // Cache results
        const cache_key = std.hash_map.hashString(file_path);
        try self.cached_asts.put(cache_key, ast);
        try self.file_fingerprints.put(try self.allocator.dupe(u8, file_path), fingerprint);
        
        var result = ArrayList(ASTNode).init(self.allocator);
        try result.appendSlice(ast.items);
        return result;
    }
    
    fn calculateFingerprint(self: *IncrementalParser, content: []const u8) u64 {
        _ = self;
        return std.hash_map.hashString(content);
    }
    
    pub fn invalidateCache(self: *IncrementalParser, file_path: []const u8) void {
        const cache_key = std.hash_map.hashString(file_path);
        _ = self.cached_tokens.remove(cache_key);
        _ = self.cached_asts.remove(cache_key);
        _ = self.file_fingerprints.remove(file_path);
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        print("Usage: cursed-zig <filename.csd>\n", .{});
        print("Enhanced CURSED compiler with error handling support\n", .{});
        print("Supports: yikes (error creation), shook (error propagation), fam (panic recovery)\n", .{});
        return;
    }

    const filename = args[1];
    
    // Read source file
    const file_content = std.fs.cwd().readFileAlloc(allocator, filename, 1024 * 1024) catch |err| {
        print("❌ Error reading file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(file_content);

    print("🚀 Enhanced CURSED Compiler with Error Handling\n", .{});
    print("📁 Processing {s} ({} bytes)\n", .{ filename, file_content.len });

    // Lexical analysis
    var lexer = Lexer.init(allocator, file_content);
    const tokens = try lexer.tokenize();
    defer tokens.deinit();

    print("🔍 Lexed {} tokens\n", .{tokens.items.len});

    // Check for error handling features
    var has_error_handling = false;
    for (tokens.items) |token| {
        if (token.type == .YIKES or token.type == .SHOOK or token.type == .FAM) {
            has_error_handling = true;
            break;
        }
    }

    if (has_error_handling) {
        print("✨ Error handling features detected (yikes/shook/fam)\n", .{});
    }

    // Parse AST
    var parser = Parser.init(allocator, tokens.items);
    const ast = parser.parse() catch |err| {
        print("❌ Parser error: {any}\n", .{err});
        return;
    };
    defer ast.deinit();

    print("🌳 Generated AST with {} statements\n", .{ast.items.len});

    // Execute with enhanced interpreter
    var interpreter = Interpreter.init(allocator);
    defer interpreter.deinit();

    print("🚀 Interpreting CURSED program with error handling...\n", .{});
    interpreter.execute(ast) catch |err| {
        print("❌ Execution error: {any}\n", .{err});
        return;
    };

    print("✅ Program execution completed\n", .{});
    
    // Report error statistics
    if (interpreter.errors.items.len > 0) {
        print("📊 Error handling statistics:\n", .{});
        print("   Total errors created: {}\n", .{interpreter.errors.items.len});
        for (interpreter.errors.items) |error_msg| {
            print("   - {s}\n", .{error_msg});
        }
    }
}
