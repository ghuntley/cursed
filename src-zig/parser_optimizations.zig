const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

// Parser optimization module for CURSED compiler
// Focuses on faster tokenization and improved AST generation

// Fast token pool to reduce allocations during tokenization
const TokenPool = struct {
    tokens: ArrayList(Token),
    token_cache: [1024]Token,
    cache_index: u32,
    allocator: Allocator,
    
    const Token = struct {
        kind: TokenKind,
        lexeme: []const u8,
        line: u32,
        column: u32,
    };
    
    const TokenKind = enum {
        // Keywords
        sus, slay, damn, ready, otherwise, bestie, squad, collab, yeet, vibez, stan, shook, fam,
        
        // Types
        drip, tea, lit, normie,
        
        // Literals
        integer, float, string, boolean, identifier,
        
        // Operators
        plus, minus, multiply, divide, modulo, assign, equals, not_equals, less_than, greater_than, less_equal, greater_equal,
        
        // Punctuation
        left_paren, right_paren, left_brace, right_brace, left_bracket, right_bracket,
        semicolon, comma, dot, arrow, pipe,
        
        // Special
        newline, eof, invalid,
    };
    
    const Self = @This();
    
    pub fn init(allocator: Allocator) Self {
        return Self{
            .tokens = ArrayList(Token).init(allocator),
            .token_cache = undefined,
            .cache_index = 0,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.tokens.deinit();
    }
    
    pub fn getToken(self: *Self) *Token {
        if (self.cache_index < 1024) {
            const token = &self.token_cache[self.cache_index];
            self.cache_index += 1;
            return token;
        }
        
        // Fallback to heap allocation if cache is full
        self.tokens.append(Token{
            .kind = .invalid,
            .lexeme = "",
            .line = 0,
            .column = 0,
        }) catch unreachable;
        
        return &self.tokens.items[self.tokens.items.len - 1];
    }
    
    pub fn reset(self: *Self) void {
        self.cache_index = 0;
        self.tokens.clearRetainingCapacity();
    }
    
    pub fn getAllTokens(self: *Self) []Token {
        if (self.cache_index > 0) {
            return self.token_cache[0..self.cache_index];
        }
        return self.tokens.items;
    }
};

// Optimized tokenizer with faster character classification
const FastTokenizer = struct {
    source: []const u8,
    position: u32,
    line: u32,
    column: u32,
    token_pool: TokenPool,
    keyword_map: HashMap([]const u8, TokenPool.TokenKind, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator, source: []const u8) !Self {
        var keyword_map = HashMap([]const u8, TokenPool.TokenKind, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
        
        // Pre-populate keyword map for O(1) lookup
        try keyword_map.put("sus", .sus);
        try keyword_map.put("slay", .slay);
        try keyword_map.put("damn", .damn);
        try keyword_map.put("ready", .ready);
        try keyword_map.put("otherwise", .otherwise);
        try keyword_map.put("bestie", .bestie);
        try keyword_map.put("squad", .squad);
        try keyword_map.put("collab", .collab);
        try keyword_map.put("yeet", .yeet);
        try keyword_map.put("vibez", .vibez);
        try keyword_map.put("stan", .stan);
        try keyword_map.put("shook", .shook);
        try keyword_map.put("fam", .fam);
        try keyword_map.put("drip", .drip);
        try keyword_map.put("tea", .tea);
        try keyword_map.put("lit", .lit);
        try keyword_map.put("normie", .normie);
        try keyword_map.put("based", .boolean);
        try keyword_map.put("cringe", .boolean);
        
        return Self{
            .source = source,
            .position = 0,
            .line = 1,
            .column = 1,
            .token_pool = TokenPool.init(allocator),
            .keyword_map = keyword_map,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.token_pool.deinit();
        self.keyword_map.deinit();
    }
    
    // Fast character classification using lookup tables
    const char_types = blk: {
        var types = [_]u8{0} ** 256;
        
        // 1 = letter, 2 = digit, 4 = whitespace, 8 = operator, 16 = punctuation
        for ('a'..'z' + 1) |c| types[c] |= 1;
        for ('A'..'Z' + 1) |c| types[c] |= 1;
        types['_'] |= 1;
        
        for ('0'..'9' + 1) |c| types[c] |= 2;
        
        types[' '] |= 4;
        types['\t'] |= 4;
        types['\r'] |= 4;
        
        const operators = "+-*/%=<>!&|";
        for (operators) |c| types[c] |= 8;
        
        const punctuation = "(){}[];,.";
        for (punctuation) |c| types[c] |= 16;
        
        break :blk types;
    };
    
    inline fn isLetter(c: u8) bool {
        return char_types[c] & 1 != 0;
    }
    
    inline fn isDigit(c: u8) bool {
        return char_types[c] & 2 != 0;
    }
    
    inline fn isWhitespace(c: u8) bool {
        return char_types[c] & 4 != 0;
    }
    
    inline fn isOperator(c: u8) bool {
        return char_types[c] & 8 != 0;
    }
    
    inline fn isPunctuation(c: u8) bool {
        return char_types[c] & 16 != 0;
    }
    
    pub fn tokenize(self: *Self) ![]TokenPool.Token {
        self.token_pool.reset();
        
        while (self.position < self.source.len) {
            const start_pos = self.position;
            const start_line = self.line;
            const start_column = self.column;
            
            const c = self.source[self.position];
            
            // Fast whitespace skipping
            if (isWhitespace(c)) {
                self.skipWhitespace();
                continue;
            }
            
            // Fast newline handling
            if (c == '\n') {
                self.newToken(.newline, self.source[start_pos..self.position + 1], start_line, start_column);
                self.advance();
                self.line += 1;
                self.column = 1;
                continue;
            }
            
            // Fast comment skipping
            if (c == '#') {
                self.skipComment();
                continue;
            }
            
            // Fast string literal tokenization
            if (c == '"') {
                try self.tokenizeString(start_line, start_column);
                continue;
            }
            
            // Fast number tokenization
            if (isDigit(c)) {
                self.tokenizeNumber(start_line, start_column);
                continue;
            }
            
            // Fast identifier/keyword tokenization
            if (isLetter(c)) {
                self.tokenizeIdentifierOrKeyword(start_line, start_column);
                continue;
            }
            
            // Fast operator tokenization
            if (isOperator(c)) {
                self.tokenizeOperator(start_line, start_column);
                continue;
            }
            
            // Fast punctuation tokenization
            if (isPunctuation(c)) {
                self.tokenizePunctuation(start_line, start_column);
                continue;
            }
            
            // Unknown character
            self.newToken(.invalid, self.source[start_pos..self.position + 1], start_line, start_column);
            self.advance();
        }
        
        // Add EOF token
        self.newToken(.eof, "", self.line, self.column);
        
        return self.token_pool.getAllTokens();
    }
    
    inline fn advance(self: *Self) void {
        self.position += 1;
        self.column += 1;
    }
    
    inline fn peek(self: *Self, offset: u32) u8 {
        const pos = self.position + offset;
        if (pos >= self.source.len) return 0;
        return self.source[pos];
    }
    
    fn skipWhitespace(self: *Self) void {
        while (self.position < self.source.len and isWhitespace(self.source[self.position])) {
            self.advance();
        }
    }
    
    fn skipComment(self: *Self) void {
        while (self.position < self.source.len and self.source[self.position] != '\n') {
            self.advance();
        }
    }
    
    fn tokenizeString(self: *Self, start_line: u32, start_column: u32) !void {
        const start_pos = self.position;
        self.advance(); // Skip opening quote
        
        while (self.position < self.source.len and self.source[self.position] != '"') {
            if (self.source[self.position] == '\\') {
                self.advance(); // Skip escape character
                if (self.position < self.source.len) {
                    self.advance(); // Skip escaped character
                }
            } else {
                self.advance();
            }
        }
        
        if (self.position < self.source.len) {
            self.advance(); // Skip closing quote
        }
        
        self.newToken(.string, self.source[start_pos..self.position], start_line, start_column);
    }
    
    fn tokenizeNumber(self: *Self, start_line: u32, start_column: u32) void {
        const start_pos = self.position;
        var has_dot = false;
        
        while (self.position < self.source.len) {
            const c = self.source[self.position];
            if (isDigit(c)) {
                self.advance();
            } else if (c == '.' and !has_dot) {
                has_dot = true;
                self.advance();
            } else {
                break;
            }
        }
        
        const token_kind: TokenPool.TokenKind = if (has_dot) .float else .integer;
        self.newToken(token_kind, self.source[start_pos..self.position], start_line, start_column);
    }
    
    fn tokenizeIdentifierOrKeyword(self: *Self, start_line: u32, start_column: u32) void {
        const start_pos = self.position;
        
        while (self.position < self.source.len) {
            const c = self.source[self.position];
            if (isLetter(c) or isDigit(c)) {
                self.advance();
            } else {
                break;
            }
        }
        
        const lexeme = self.source[start_pos..self.position];
        const token_kind = self.keyword_map.get(lexeme) orelse .identifier;
        self.newToken(token_kind, lexeme, start_line, start_column);
    }
    
    fn tokenizeOperator(self: *Self, start_line: u32, start_column: u32) void {
        const start_pos = self.position;
        const c = self.source[self.position];
        
        var token_kind: TokenPool.TokenKind = .invalid;
        
        switch (c) {
            '+' => { token_kind = .plus; self.advance(); },
            '-' => { 
                if (self.peek(1) == '>') {
                    token_kind = .arrow;
                    self.advance();
                    self.advance();
                } else {
                    token_kind = .minus;
                    self.advance();
                }
            },
            '*' => { token_kind = .multiply; self.advance(); },
            '/' => { token_kind = .divide; self.advance(); },
            '%' => { token_kind = .modulo; self.advance(); },
            '=' => {
                if (self.peek(1) == '=') {
                    token_kind = .equals;
                    self.advance();
                    self.advance();
                } else {
                    token_kind = .assign;
                    self.advance();
                }
            },
            '!' => {
                if (self.peek(1) == '=') {
                    token_kind = .not_equals;
                    self.advance();
                    self.advance();
                } else {
                    self.advance();
                }
            },
            '<' => {
                if (self.peek(1) == '=') {
                    token_kind = .less_equal;
                    self.advance();
                    self.advance();
                } else {
                    token_kind = .less_than;
                    self.advance();
                }
            },
            '>' => {
                if (self.peek(1) == '=') {
                    token_kind = .greater_equal;
                    self.advance();
                    self.advance();
                } else {
                    token_kind = .greater_than;
                    self.advance();
                }
            },
            '|' => { token_kind = .pipe; self.advance(); },
            else => { self.advance(); },
        }
        
        self.newToken(token_kind, self.source[start_pos..self.position], start_line, start_column);
    }
    
    fn tokenizePunctuation(self: *Self, start_line: u32, start_column: u32) void {
        const start_pos = self.position;
        const c = self.source[self.position];
        
        const token_kind: TokenPool.TokenKind = switch (c) {
            '(' => .left_paren,
            ')' => .right_paren,
            '{' => .left_brace,
            '}' => .right_brace,
            '[' => .left_bracket,
            ']' => .right_bracket,
            ';' => .semicolon,
            ',' => .comma,
            '.' => .dot,
            else => .invalid,
        };
        
        self.advance();
        self.newToken(token_kind, self.source[start_pos..self.position], start_line, start_column);
    }
    
    fn newToken(self: *Self, kind: TokenPool.TokenKind, lexeme: []const u8, line: u32, column: u32) void {
        const token = self.token_pool.getToken();
        token.kind = kind;
        token.lexeme = lexeme;
        token.line = line;
        token.column = column;
    }
};

// AST node pool for efficient memory management
const ASTNodePool = struct {
    nodes: ArrayList(ASTNode),
    node_cache: [2048]ASTNode,
    cache_index: u32,
    allocator: Allocator,
    
    const ASTNode = struct {
        kind: NodeKind,
        value: []const u8,
        children: [8]?*ASTNode, // Fixed size array for common cases
        child_count: u8,
        line: u32,
        column: u32,
    };
    
    const NodeKind = enum {
        program, function_def, variable_decl, assignment, if_statement, while_loop,
        expression, binary_op, unary_op, function_call, identifier, literal,
        block, return_statement, struct_def, interface_def, import_statement,
    };
    
    const Self = @This();
    
    pub fn init(allocator: Allocator) Self {
        return Self{
            .nodes = ArrayList(ASTNode).init(allocator),
            .node_cache = undefined,
            .cache_index = 0,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.nodes.deinit();
    }
    
    pub fn getNode(self: *Self) *ASTNode {
        if (self.cache_index < 2048) {
            const node = &self.node_cache[self.cache_index];
            self.cache_index += 1;
            node.children = [_]?*ASTNode{null} ** 8;
            node.child_count = 0;
            return node;
        }
        
        // Fallback to heap allocation
        self.nodes.append(ASTNode{
            .kind = .expression,
            .value = "",
            .children = [_]?*ASTNode{null} ** 8,
            .child_count = 0,
            .line = 0,
            .column = 0,
        }) catch unreachable;
        
        return &self.nodes.items[self.nodes.items.len - 1];
    }
    
    pub fn addChild(parent: *ASTNode, child: *ASTNode) void {
        if (parent.child_count < 8) {
            parent.children[parent.child_count] = child;
            parent.child_count += 1;
        }
    }
    
    pub fn reset(self: *Self) void {
        self.cache_index = 0;
        self.nodes.clearRetainingCapacity();
    }
};

// Optimized recursive descent parser
pub const FastParser = struct {
    tokens: []TokenPool.Token,
    position: u32,
    ast_pool: ASTNodePool,
    allocator: Allocator,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator, tokens: []TokenPool.Token) Self {
        return Self{
            .tokens = tokens,
            .position = 0,
            .ast_pool = ASTNodePool.init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.ast_pool.deinit();
    }
    
    pub fn parse(self: *Self) !*ASTNodePool.ASTNode {
        self.ast_pool.reset();
        return self.parseProgram();
    }
    
    inline fn currentToken(self: *Self) TokenPool.Token {
        if (self.position >= self.tokens.len) {
            return TokenPool.Token{
                .kind = .eof,
                .lexeme = "",
                .line = 0,
                .column = 0,
            };
        }
        return self.tokens[self.position];
    }
    
    inline fn advance(self: *Self) void {
        if (self.position < self.tokens.len) {
            self.position += 1;
        }
    }
    
    inline fn match(self: *Self, kind: TokenPool.TokenKind) bool {
        if (self.currentToken().kind == kind) {
            self.advance();
            return true;
        }
        return false;
    }
    
    fn parseProgram(self: *Self) !*ASTNodePool.ASTNode {
        const program = self.ast_pool.getNode();
        program.kind = .program;
        program.value = "program";
        
        while (self.currentToken().kind != .eof) {
            if (const statement = try self.parseStatement()) |stmt| {
                ASTNodePool.addChild(program, stmt);
            }
        }
        
        return program;
    }
    
    fn parseStatement(self: *Self) !?*ASTNodePool.ASTNode {
        const token = self.currentToken();
        
        return switch (token.kind) {
            .sus => self.parseVariableDeclaration(),
            .slay => self.parseFunctionDefinition(),
            .ready => self.parseIfStatement(),
            .bestie => self.parseWhileLoop(),
            .damn => self.parseReturnStatement(),
            .squad => self.parseStructDefinition(),
            .collab => self.parseInterfaceDefinition(),
            .yeet => self.parseImportStatement(),
            .identifier => self.parseAssignmentOrExpression(),
            .newline => blk: {
                self.advance();
                break :blk null;
            },
            else => self.parseExpression(),
        };
    }
    
    fn parseVariableDeclaration(self: *Self) !*ASTNodePool.ASTNode {
        const node = self.ast_pool.getNode();
        node.kind = .variable_decl;
        
        self.advance(); // Skip 'sus'
        
        if (self.currentToken().kind == .identifier) {
            const name_node = self.ast_pool.getNode();
            name_node.kind = .identifier;
            name_node.value = self.currentToken().lexeme;
            ASTNodePool.addChild(node, name_node);
            self.advance();
        }
        
        // Parse type
        if (self.currentToken().kind == .drip or self.currentToken().kind == .tea or self.currentToken().kind == .lit) {
            const type_node = self.ast_pool.getNode();
            type_node.kind = .identifier;
            type_node.value = self.currentToken().lexeme;
            ASTNodePool.addChild(node, type_node);
            self.advance();
        }
        
        // Parse initializer
        if (self.match(.assign)) {
            if (const expr = try self.parseExpression()) |init_expr| {
                ASTNodePool.addChild(node, init_expr);
            }
        }
        
        return node;
    }
    
    fn parseFunctionDefinition(self: *Self) !*ASTNodePool.ASTNode {
        const node = self.ast_pool.getNode();
        node.kind = .function_def;
        
        self.advance(); // Skip 'slay'
        
        // Parse function name
        if (self.currentToken().kind == .identifier) {
            const name_node = self.ast_pool.getNode();
            name_node.kind = .identifier;
            name_node.value = self.currentToken().lexeme;
            ASTNodePool.addChild(node, name_node);
            self.advance();
        }
        
        // Parse parameters
        if (self.match(.left_paren)) {
            while (!self.match(.right_paren) and self.currentToken().kind != .eof) {
                if (const param = try self.parseStatement()) |param_node| {
                    ASTNodePool.addChild(node, param_node);
                }
                if (!self.match(.comma)) break;
            }
        }
        
        // Parse body
        if (self.match(.left_brace)) {
            const body = self.ast_pool.getNode();
            body.kind = .block;
            
            while (!self.match(.right_brace) and self.currentToken().kind != .eof) {
                if (const stmt = try self.parseStatement()) |stmt_node| {
                    ASTNodePool.addChild(body, stmt_node);
                }
            }
            
            ASTNodePool.addChild(node, body);
        }
        
        return node;
    }
    
    fn parseIfStatement(self: *Self) !*ASTNodePool.ASTNode {
        const node = self.ast_pool.getNode();
        node.kind = .if_statement;
        
        self.advance(); // Skip 'ready'
        
        // Parse condition
        if (self.match(.left_paren)) {
            if (const condition = try self.parseExpression()) |cond| {
                ASTNodePool.addChild(node, cond);
            }
            _ = self.match(.right_paren);
        }
        
        // Parse then block
        if (const then_stmt = try self.parseStatement()) |then_node| {
            ASTNodePool.addChild(node, then_node);
        }
        
        // Parse else block
        if (self.match(.otherwise)) {
            if (const else_stmt = try self.parseStatement()) |else_node| {
                ASTNodePool.addChild(node, else_node);
            }
        }
        
        return node;
    }
    
    fn parseWhileLoop(self: *Self) !*ASTNodePool.ASTNode {
        const node = self.ast_pool.getNode();
        node.kind = .while_loop;
        
        self.advance(); // Skip 'bestie'
        
        // Parse condition
        if (self.match(.left_paren)) {
            if (const condition = try self.parseExpression()) |cond| {
                ASTNodePool.addChild(node, cond);
            }
            _ = self.match(.right_paren);
        }
        
        // Parse body
        if (const body = try self.parseStatement()) |body_node| {
            ASTNodePool.addChild(node, body_node);
        }
        
        return node;
    }
    
    fn parseReturnStatement(self: *Self) !*ASTNodePool.ASTNode {
        const node = self.ast_pool.getNode();
        node.kind = .return_statement;
        
        self.advance(); // Skip 'damn'
        
        if (const expr = try self.parseExpression()) |return_expr| {
            ASTNodePool.addChild(node, return_expr);
        }
        
        return node;
    }
    
    fn parseStructDefinition(self: *Self) !*ASTNodePool.ASTNode {
        const node = self.ast_pool.getNode();
        node.kind = .struct_def;
        
        self.advance(); // Skip 'squad'
        
        // Parse struct name
        if (self.currentToken().kind == .identifier) {
            const name_node = self.ast_pool.getNode();
            name_node.kind = .identifier;
            name_node.value = self.currentToken().lexeme;
            ASTNodePool.addChild(node, name_node);
            self.advance();
        }
        
        // Parse fields
        if (self.match(.left_brace)) {
            while (!self.match(.right_brace) and self.currentToken().kind != .eof) {
                if (const field = try self.parseStatement()) |field_node| {
                    ASTNodePool.addChild(node, field_node);
                }
            }
        }
        
        return node;
    }
    
    fn parseInterfaceDefinition(self: *Self) !*ASTNodePool.ASTNode {
        const node = self.ast_pool.getNode();
        node.kind = .interface_def;
        
        self.advance(); // Skip 'collab'
        
        // Parse interface name
        if (self.currentToken().kind == .identifier) {
            const name_node = self.ast_pool.getNode();
            name_node.kind = .identifier;
            name_node.value = self.currentToken().lexeme;
            ASTNodePool.addChild(node, name_node);
            self.advance();
        }
        
        // Parse methods
        if (self.match(.left_brace)) {
            while (!self.match(.right_brace) and self.currentToken().kind != .eof) {
                if (const method = try self.parseStatement()) |method_node| {
                    ASTNodePool.addChild(node, method_node);
                }
            }
        }
        
        return node;
    }
    
    fn parseImportStatement(self: *Self) !*ASTNodePool.ASTNode {
        const node = self.ast_pool.getNode();
        node.kind = .import_statement;
        
        self.advance(); // Skip 'yeet'
        
        if (self.currentToken().kind == .string) {
            const module_node = self.ast_pool.getNode();
            module_node.kind = .literal;
            module_node.value = self.currentToken().lexeme;
            ASTNodePool.addChild(node, module_node);
            self.advance();
        }
        
        return node;
    }
    
    fn parseAssignmentOrExpression(self: *Self) !*ASTNodePool.ASTNode {
        const expr = try self.parseExpression();
        
        if (self.match(.assign)) {
            const assign_node = self.ast_pool.getNode();
            assign_node.kind = .assignment;
            ASTNodePool.addChild(assign_node, expr.?);
            
            if (const value = try self.parseExpression()) |value_expr| {
                ASTNodePool.addChild(assign_node, value_expr);
            }
            
            return assign_node;
        }
        
        return expr.?;
    }
    
    fn parseExpression(self: *Self) !?*ASTNodePool.ASTNode {
        return self.parseEquality();
    }
    
    fn parseEquality(self: *Self) !?*ASTNodePool.ASTNode {
        var expr = try self.parseComparison();
        
        while (self.currentToken().kind == .equals or self.currentToken().kind == .not_equals) {
            const op = self.currentToken();
            self.advance();
            
            const right = try self.parseComparison();
            
            const binary_op = self.ast_pool.getNode();
            binary_op.kind = .binary_op;
            binary_op.value = op.lexeme;
            ASTNodePool.addChild(binary_op, expr.?);
            ASTNodePool.addChild(binary_op, right.?);
            
            expr = binary_op;
        }
        
        return expr;
    }
    
    fn parseComparison(self: *Self) !?*ASTNodePool.ASTNode {
        var expr = try self.parseTerm();
        
        while (self.currentToken().kind == .less_than or self.currentToken().kind == .greater_than or 
               self.currentToken().kind == .less_equal or self.currentToken().kind == .greater_equal) {
            const op = self.currentToken();
            self.advance();
            
            const right = try self.parseTerm();
            
            const binary_op = self.ast_pool.getNode();
            binary_op.kind = .binary_op;
            binary_op.value = op.lexeme;
            ASTNodePool.addChild(binary_op, expr.?);
            ASTNodePool.addChild(binary_op, right.?);
            
            expr = binary_op;
        }
        
        return expr;
    }
    
    fn parseTerm(self: *Self) !?*ASTNodePool.ASTNode {
        var expr = try self.parseFactor();
        
        while (self.currentToken().kind == .plus or self.currentToken().kind == .minus) {
            const op = self.currentToken();
            self.advance();
            
            const right = try self.parseFactor();
            
            const binary_op = self.ast_pool.getNode();
            binary_op.kind = .binary_op;
            binary_op.value = op.lexeme;
            ASTNodePool.addChild(binary_op, expr.?);
            ASTNodePool.addChild(binary_op, right.?);
            
            expr = binary_op;
        }
        
        return expr;
    }
    
    fn parseFactor(self: *Self) !?*ASTNodePool.ASTNode {
        var expr = try self.parseUnary();
        
        while (self.currentToken().kind == .multiply or self.currentToken().kind == .divide or self.currentToken().kind == .modulo) {
            const op = self.currentToken();
            self.advance();
            
            const right = try self.parseUnary();
            
            const binary_op = self.ast_pool.getNode();
            binary_op.kind = .binary_op;
            binary_op.value = op.lexeme;
            ASTNodePool.addChild(binary_op, expr.?);
            ASTNodePool.addChild(binary_op, right.?);
            
            expr = binary_op;
        }
        
        return expr;
    }
    
    fn parseUnary(self: *Self) !?*ASTNodePool.ASTNode {
        if (self.currentToken().kind == .minus) {
            const op = self.currentToken();
            self.advance();
            
            const expr = try self.parseUnary();
            
            const unary_op = self.ast_pool.getNode();
            unary_op.kind = .unary_op;
            unary_op.value = op.lexeme;
            ASTNodePool.addChild(unary_op, expr.?);
            
            return unary_op;
        }
        
        return self.parsePrimary();
    }
    
    fn parsePrimary(self: *Self) !?*ASTNodePool.ASTNode {
        const token = self.currentToken();
        
        switch (token.kind) {
            .integer, .float, .string, .boolean => {
                const literal = self.ast_pool.getNode();
                literal.kind = .literal;
                literal.value = token.lexeme;
                literal.line = token.line;
                literal.column = token.column;
                self.advance();
                return literal;
            },
            .identifier => {
                const id = self.ast_pool.getNode();
                id.kind = .identifier;
                id.value = token.lexeme;
                id.line = token.line;
                id.column = token.column;
                self.advance();
                
                // Check for function call
                if (self.match(.left_paren)) {
                    const call = self.ast_pool.getNode();
                    call.kind = .function_call;
                    ASTNodePool.addChild(call, id);
                    
                    while (!self.match(.right_paren) and self.currentToken().kind != .eof) {
                        if (const arg = try self.parseExpression()) |arg_expr| {
                            ASTNodePool.addChild(call, arg_expr);
                        }
                        if (!self.match(.comma)) break;
                    }
                    
                    return call;
                }
                
                return id;
            },
            .left_paren => {
                self.advance(); // Skip '('
                const expr = try self.parseExpression();
                _ = self.match(.right_paren); // Skip ')'
                return expr;
            },
            else => return null,
        }
    }
};

// Performance benchmarking for parser
pub const ParserBenchmark = struct {
    tokenization_time_ns: u64,
    parsing_time_ns: u64,
    memory_used_bytes: usize,
    nodes_created: u32,
    tokens_created: u32,
    
    const Self = @This();
    
    pub fn init() Self {
        return Self{
            .tokenization_time_ns = 0,
            .parsing_time_ns = 0,
            .memory_used_bytes = 0,
            .nodes_created = 0,
            .tokens_created = 0,
        };
    }
    
    pub fn print(self: *Self) void {
        std.debug.print("=== Parser Performance Report ===\n");
        std.debug.print("Tokenization: {d:.2}ms\n", .{@as(f64, @floatFromInt(self.tokenization_time_ns)) / 1_000_000.0});
        std.debug.print("Parsing: {d:.2}ms\n", .{@as(f64, @floatFromInt(self.parsing_time_ns)) / 1_000_000.0});
        std.debug.print("Total: {d:.2}ms\n", .{@as(f64, @floatFromInt(self.tokenization_time_ns + self.parsing_time_ns)) / 1_000_000.0});
        std.debug.print("Memory used: {} bytes\n", .{self.memory_used_bytes});
        std.debug.print("Tokens created: {}\n", .{self.tokens_created});
        std.debug.print("AST nodes created: {}\n", .{self.nodes_created});
        
        if (self.tokens_created > 0) {
            const tokens_per_ms = @as(f64, @floatFromInt(self.tokens_created)) / (@as(f64, @floatFromInt(self.tokenization_time_ns)) / 1_000_000.0);
            std.debug.print("Tokenization rate: {d:.0} tokens/ms\n", .{tokens_per_ms});
        }
        
        if (self.nodes_created > 0) {
            const nodes_per_ms = @as(f64, @floatFromInt(self.nodes_created)) / (@as(f64, @floatFromInt(self.parsing_time_ns)) / 1_000_000.0);
            std.debug.print("Parsing rate: {d:.0} nodes/ms\n", .{nodes_per_ms});
        }
    }
};
