const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Enhanced token types for CURSED language with error handling
const TokenType = enum {
    // Literals
    STRING,
    NUMBER,
    IDENTIFIER,
    
    // Keywords
    VIBEZ,    // vibez.spill
    SPILL,    // spill
    DOT,      // .
    
    // Error handling keywords
    YIKES,    // yikes - error creation
    SHOOK,    // shook - error propagation
    FAM,      // fam - panic recovery
    
    // Operators
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    EQUAL,    // =
    
    // Delimiters
    LPAREN,   // (
    RPAREN,   // )
    LBRACE,   // {
    RBRACE,   // }
    COMMA,    // ,
    SEMICOLON, // ;
    
    // Special
    EOF,
    NEWLINE,
    UNKNOWN,
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
            self.advance();
        }
        
        if (self.position < self.input.len) {
            self.advance(); // Skip closing quote
        }
        
        try tokens.append(Token{
            .type = .STRING,
            .value = self.input[start_pos..self.position],
            .line = line,
            .column = column,
        });
    }

    fn readNumber(self: *Lexer, tokens: *ArrayList(Token), line: usize, column: usize) !void {
        const start_pos = self.position;
        
        while (self.position < self.input.len and 
               (std.ascii.isDigit(self.input[self.position]) or self.input[self.position] == '.')) {
            self.advance();
        }
        
        try tokens.append(Token{
            .type = .NUMBER,
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

// Parser error types
const ParseError = error{
    ExpectedIdentifier,
    ExpectedLeftBrace,
    UnexpectedToken,
    NotAFunctionCall,
    OutOfMemory,
};

// Enhanced parser with error handling support
const Parser = struct {
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

    pub fn parse(self: *Parser) !ArrayList(ASTNode) {
        var statements = ArrayList(ASTNode).init(self.allocator);
        
        while (!self.isAtEnd()) {
            // Skip newlines
            if (self.peek().type == .NEWLINE) {
                _ = self.advance();
                continue;
            }
            
            const stmt = try self.parseStatement();
            try statements.append(stmt);
        }
        
        return statements;
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
