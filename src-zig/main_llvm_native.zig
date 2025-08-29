const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

// LLVM C bindings
const c = @cImport({
    @cDefine("__x86_64__", "1");
    @cDefine("LLVM_HOST_TRIPLE", "\"x86_64-unknown-linux-gnu\"");
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/TargetMachine.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/Transforms/PassManagerBuilder.h");
});

const Token = enum {
    Identifier,
    Number,
    String,
    Keyword,
    Operator,
    Punctuation,
    EOF,
};

const TokenData = struct {
    type: Token,
    value: []const u8,
    line: u32,
    column: u32,
};

const Lexer = struct {
    source: []const u8,
    pos: usize = 0,
    line: u32 = 1,
    column: u32 = 1,
    
    pub fn init(source: []const u8) Lexer {
        return Lexer{ .source = source };
    }
    
    pub fn nextToken(self: *Lexer, allocator: Allocator) !?TokenData {
        while (self.pos < self.source.len) {
            const ch = self.source[self.pos];
            
            // Skip whitespace except newlines
            if (ch == ' ' or ch == '\t' or ch == '\r') {
                self.advance();
                continue;
            }
            
            // Handle newlines
            if (ch == '\n') {
                self.line += 1;
                self.column = 1;
                self.pos += 1;
                continue;
            }
            
            // Skip comments
            if (ch == '#') {
                while (self.pos < self.source.len and self.source[self.pos] != '\n') {
                    self.pos += 1;
                }
                continue;
            }
            
            // Numbers
            if (std.ascii.isDigit(ch)) {
                return self.readNumber();
            }
            
            // Strings
            if (ch == '"') {
                return self.readString(allocator);
            }
            
            // Identifiers and keywords
            if (std.ascii.isAlphabetic(ch) or ch == '_') {
                return self.readIdentifier();
            }
            
            // Single character operators/punctuation
            const token = TokenData{
                .type = switch (ch) {
                    '(' => Token.Punctuation,
                    ')' => Token.Punctuation,
                    '{' => Token.Punctuation,
                    '}' => Token.Punctuation,
                    '=' => Token.Operator,
                    '+' => Token.Operator,
                    '-' => Token.Operator,
                    '*' => Token.Operator,
                    '/' => Token.Operator,
                    '%' => Token.Operator,
                    '<' => Token.Operator,
                    '>' => Token.Operator,
                    '!' => Token.Operator,
                    else => Token.Punctuation,
                },
                .value = self.source[self.pos..self.pos + 1],
                .line = self.line,
                .column = self.column,
            };
            self.advance();
            return token;
        }
        
        return TokenData{
            .type = Token.EOF,
            .value = "",
            .line = self.line,
            .column = self.column,
        };
    }
    
    fn advance(self: *Lexer) void {
        self.pos += 1;
        self.column += 1;
    }
    
    fn readNumber(self: *Lexer) TokenData {
        const start = self.pos;
        while (self.pos < self.source.len and std.ascii.isDigit(self.source[self.pos])) {
            self.advance();
        }
        return TokenData{
            .type = Token.Number,
            .value = self.source[start..self.pos],
            .line = self.line,
            .column = self.column - @as(u32, @intCast(self.pos - start)),
        };
    }
    
    fn readString(self: *Lexer, allocator: Allocator) !TokenData {
        _ = allocator;
        const start = self.pos;
        self.advance(); // Skip opening quote
        while (self.pos < self.source.len and self.source[self.pos] != '"') {
            self.advance();
        }
        if (self.pos < self.source.len) {
            self.advance(); // Skip closing quote
        }
        return TokenData{
            .type = Token.String,
            .value = self.source[start..self.pos],
            .line = self.line,
            .column = self.column - @as(u32, @intCast(self.pos - start)),
        };
    }
    
    fn readIdentifier(self: *Lexer) TokenData {
        const start = self.pos;
        while (self.pos < self.source.len and 
               (std.ascii.isAlphanumeric(self.source[self.pos]) or self.source[self.pos] == '_')) {
            self.advance();
        }
        
        const value = self.source[start..self.pos];
        const token_type = if (isKeyword(value)) Token.Keyword else Token.Identifier;
        
        return TokenData{
            .type = token_type,
            .value = value,
            .line = self.line,
            .column = self.column - @as(u32, @intCast(self.pos - start)),
        };
    }
    
    fn isKeyword(value: []const u8) bool {
        const keywords = [_][]const u8{
            "slay", "sus", "drip", "tea", "lit", "based", "bestie", "ready", 
            "otherwise", "damn", "vibez", "spill", "rn"
        };
        for (keywords) |keyword| {
            if (std.mem.eql(u8, value, keyword)) {
                return true;
            }
        }
        return false;
    }
};

const ASTNode = union(enum) {
    FunctionDef: struct {
        name: []const u8,
        return_type: []const u8,
        body: std.ArrayList(*ASTNode),
    },
    ForLoop: struct {
        init: ?*ASTNode,
        condition: ?*ASTNode,
        update: ?*ASTNode,
        body: std.ArrayList(*ASTNode),
    },
    IfStatement: struct {
        condition: *ASTNode,
        then_branch: std.ArrayList(*ASTNode),
        else_branch: ?std.ArrayList(*ASTNode),
    },
    VarDecl: struct {
        name: []const u8,
        type_name: []const u8,
        value: ?*ASTNode,
    },
    Assignment: struct {
        name: []const u8,
        value: *ASTNode,
    },
    BinaryOp: struct {
        op: []const u8,
        left: *ASTNode,
        right: *ASTNode,
    },
    UnaryOp: struct {
        op: []const u8,
        operand: *ASTNode,
    },
    FunctionCall: struct {
        name: []const u8,
        args: std.ArrayList(*ASTNode),
    },
    Identifier: []const u8,
    Number: i64,
    String: []const u8,
    Return: ?*ASTNode,
    Continue,
    Break,
};

const Parser = struct {
    tokens: std.ArrayList(TokenData),
    pos: usize = 0,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, tokens: std.ArrayList(TokenData)) Parser {
        return Parser{ .allocator = allocator, .tokens = tokens };
    }
    
    pub fn parse(self: *Parser) !std.ArrayList(*ASTNode) {
        var program = std.ArrayList(*ASTNode).init(self.allocator);
        
        while (!self.isAtEnd()) {
            if (try self.parseStatement()) |stmt| {
                try program.append(stmt);
            }
        }
        
        return program;
    }
    
    fn parseStatement(self: *Parser) !?*ASTNode {
        if (self.match("slay")) {
            return self.parseFunctionDef();
        }
        
        if (self.match("sus")) {
            return self.parseVarDecl();
        }
        
        if (self.match("bestie")) {
            return self.parseForLoop();
        }
        
        if (self.match("ready")) {
            return self.parseIfStatement();
        }
        
        if (self.match("damn")) {
            return self.parseReturn();
        }
        
        if (self.match("rn")) {
            const node = try self.allocator.create(ASTNode);
            node.* = ASTNode.Continue;
            return node;
        }
        
        // Expression statement (function call or assignment)
        return self.parseExpressionStatement();
    }
    
    fn parseFunctionDef(self: *Parser) !*ASTNode {
        const name = try self.consume(Token.Identifier, "Expected function name");
        _ = try self.consume(Token.Punctuation, "Expected '('");
        _ = try self.consume(Token.Punctuation, "Expected ')'");
        const return_type = try self.consume(Token.Identifier, "Expected return type");
        _ = try self.consume(Token.Punctuation, "Expected '{'");
        
        var body = std.ArrayList(*ASTNode).init(self.allocator);
        while (!self.check(Token.Punctuation, "}") and !self.isAtEnd()) {
            if (try self.parseStatement()) |s| {
                try body.append(s);
            }
        }
        _ = try self.consume(Token.Punctuation, "Expected '}'");
        
        const node = try self.allocator.create(ASTNode);
        node.* = ASTNode{ .FunctionDef = .{
            .name = name.value,
            .return_type = return_type.value,
            .body = body,
        }};
        return node;
    }
    
    fn parseVarDecl(self: *Parser) !*ASTNode {
        const name = try self.consume(Token.Identifier, "Expected variable name");
        const type_name = try self.consume(Token.Identifier, "Expected type");
        
        var value: ?*ASTNode = null;
        if (self.match("=")) {
            value = try self.parseExpression();
        }
        
        const node = try self.allocator.create(ASTNode);
        node.* = ASTNode{ .VarDecl = .{
            .name = name.value,
            .type_name = type_name.value,
            .value = value,
        }};
        return node;
    }
    
    fn parseForLoop(self: *Parser) !*ASTNode {
        _ = try self.consume(Token.Punctuation, "Expected '('");
        
        // Parse init (sus i drip = 1)
        var init_node: ?*ASTNode = null;
        if (self.match("sus")) {
            init_node = try self.parseVarDecl();
        }
        _ = self.match(";");
        
        // Parse condition (i <= 100)
        var condition: ?*ASTNode = null;
        if (!self.check(Token.Punctuation, ";")) {
            condition = try self.parseExpression();
        }
        _ = self.match(";");
        
        // Parse update (i = i + 1)
        var update: ?*ASTNode = null;
        if (!self.check(Token.Punctuation, ")")) {
            update = try self.parseExpression();
        }
        _ = try self.consume(Token.Punctuation, "Expected ')'");
        _ = try self.consume(Token.Punctuation, "Expected '{'");
        
        var body = std.ArrayList(*ASTNode).init(self.allocator);
        while (!self.check(Token.Punctuation, "}") and !self.isAtEnd()) {
            if (try self.parseStatement()) |s| {
                try body.append(s);
            }
        }
        _ = try self.consume(Token.Punctuation, "Expected '}'");
        
        const node = try self.allocator.create(ASTNode);
        node.* = ASTNode{ .ForLoop = .{
            .init = init_node,
            .condition = condition,
            .update = update,
            .body = body,
        }};
        return node;
    }
    
    fn parseIfStatement(self: *Parser) !*ASTNode {
        _ = try self.consume(Token.Punctuation, "Expected '('");
        const condition = try self.parseExpression();
        _ = try self.consume(Token.Punctuation, "Expected ')'");
        _ = try self.consume(Token.Punctuation, "Expected '{'");
        
        var then_branch = std.ArrayList(*ASTNode).init(self.allocator);
        while (!self.check(Token.Punctuation, "}") and !self.isAtEnd()) {
            if (try self.parseStatement()) |s| {
                try then_branch.append(s);
            }
        }
        _ = try self.consume(Token.Punctuation, "Expected '}'");
        
        var else_branch: ?std.ArrayList(*ASTNode) = null;
        if (self.match("otherwise")) {
            if (self.check(Token.Keyword, "ready")) {
                // else if
                const elif = try self.parseIfStatement();
                else_branch = std.ArrayList(*ASTNode).init(self.allocator);
                try else_branch.?.append(elif);
            } else {
                // else
                _ = try self.consume(Token.Punctuation, "Expected '{'");
                else_branch = std.ArrayList(*ASTNode).init(self.allocator);
                while (!self.check(Token.Punctuation, "}") and !self.isAtEnd()) {
                    if (try self.parseStatement()) |s| {
                        try else_branch.?.append(s);
                    }
                }
                _ = try self.consume(Token.Punctuation, "Expected '}'");
            }
        }
        
        const node = try self.allocator.create(ASTNode);
        node.* = ASTNode{ .IfStatement = .{
            .condition = condition,
            .then_branch = then_branch,
            .else_branch = else_branch,
        }};
        return node;
    }
    
    fn parseReturn(self: *Parser) !*ASTNode {
        var value: ?*ASTNode = null;
        if (!self.isAtEnd() and self.current().type != Token.Punctuation) {
            value = try self.parseExpression();
        }
        
        const node = try self.allocator.create(ASTNode);
        node.* = ASTNode{ .Return = value };
        return node;
    }
    
    fn parseExpressionStatement(self: *Parser) !*ASTNode {
        // Check if it's an assignment (identifier = expression)
        if (self.pos + 1 < self.tokens.items.len and 
            self.current().type == Token.Identifier and
            std.mem.eql(u8, self.tokens.items[self.pos + 1].value, "=")) {
            
            const name = self.advance();
            _ = self.advance(); // consume '='
            const value = try self.parseExpression();
            
            const node = try self.allocator.create(ASTNode);
            node.* = ASTNode{ .Assignment = .{
                .name = name.value,
                .value = value,
            }};
            return node;
        }
        
        return self.parseExpression();
    }
    
    fn parseExpression(self: *Parser) !*ASTNode {
        return self.parseComparison();
    }
    
    fn parseComparison(self: *Parser) !*ASTNode {
        var expr = try self.parseAddition();
        
        while (self.match("==") or self.match("!=") or self.match("<") or 
               self.match(">") or self.match("<=") or self.match(">=")) {
            const op = self.previous().value;
            const right = try self.parseAddition();
            
            const node = try self.allocator.create(ASTNode);
            node.* = ASTNode{ .BinaryOp = .{
                .op = op,
                .left = expr,
                .right = right,
            }};
            expr = node;
        }
        
        return expr;
    }
    
    fn parseAddition(self: *Parser) !*ASTNode {
        var expr = try self.parseMultiplication();
        
        while (self.match("+") or self.match("-")) {
            const op = self.previous().value;
            const right = try self.parseMultiplication();
            
            const node = try self.allocator.create(ASTNode);
            node.* = ASTNode{ .BinaryOp = .{
                .op = op,
                .left = expr,
                .right = right,
            }};
            expr = node;
        }
        
        return expr;
    }
    
    fn parseMultiplication(self: *Parser) !*ASTNode {
        var expr = try self.parseUnary();
        
        while (self.match("*") or self.match("/") or self.match("%")) {
            const op = self.previous().value;
            const right = try self.parseUnary();
            
            const node = try self.allocator.create(ASTNode);
            node.* = ASTNode{ .BinaryOp = .{
                .op = op,
                .left = expr,
                .right = right,
            }};
            expr = node;
        }
        
        return expr;
    }
    
    fn parseUnary(self: *Parser) !*ASTNode {
        if (self.match("-") or self.match("!")) {
            const op = self.previous().value;
            const operand = try self.parseUnary();
            
            const node = try self.allocator.create(ASTNode);
            node.* = ASTNode{ .UnaryOp = .{
                .op = op,
                .operand = operand,
            }};
            return node;
        }
        
        return self.parsePrimary();
    }
    
    fn parsePrimary(self: *Parser) !*ASTNode {
        if (self.current().type == Token.Number) {
            const token = self.advance();
            const value = std.fmt.parseInt(i64, token.value, 10) catch 0;
            
            const node = try self.allocator.create(ASTNode);
            node.* = ASTNode{ .Number = value };
            return node;
        }
        
        if (self.current().type == Token.String) {
            const token = self.advance();
            // Remove quotes
            var value = token.value;
            if (value.len >= 2 and value[0] == '"' and value[value.len - 1] == '"') {
                value = value[1..value.len - 1];
            }
            
            const node = try self.allocator.create(ASTNode);
            node.* = ASTNode{ .String = value };
            return node;
        }
        
        if (self.current().type == Token.Identifier) {
            const name = self.advance();
            
            // Check for function call
            if (self.check(Token.Punctuation, "(") or self.match(".")) {
                if (self.previous().type == Token.Punctuation and std.mem.eql(u8, self.previous().value, ".")) {
                    // Method call like vibez.spill()
                    const method = try self.consume(Token.Identifier, "Expected method name");
                    _ = try self.consume(Token.Punctuation, "Expected '('");
                    
                    var args = std.ArrayList(*ASTNode).init(self.allocator);
                    if (!self.check(Token.Punctuation, ")")) {
                        while (true) {
                            try args.append(try self.parseExpression());
                            if (!self.match(",")) break;
                        }
                    }
                    _ = try self.consume(Token.Punctuation, "Expected ')'");
                    
                    const full_name = try std.fmt.allocPrint(self.allocator, "{s}.{s}", .{name.value, method.value});
                    
                    const node = try self.allocator.create(ASTNode);
                    node.* = ASTNode{ .FunctionCall = .{
                        .name = full_name,
                        .args = args,
                    }};
                    return node;
                } else {
                    _ = try self.consume(Token.Punctuation, "Expected '('");
                    
                    var args = std.ArrayList(*ASTNode).init(self.allocator);
                    if (!self.check(Token.Punctuation, ")")) {
                        while (true) {
                            try args.append(try self.parseExpression());
                            if (!self.match(",")) break;
                        }
                    }
                    _ = try self.consume(Token.Punctuation, "Expected ')'");
                    
                    const node = try self.allocator.create(ASTNode);
                    node.* = ASTNode{ .FunctionCall = .{
                        .name = name.value,
                        .args = args,
                    }};
                    return node;
                }
            }
            
            const node = try self.allocator.create(ASTNode);
            node.* = ASTNode{ .Identifier = name.value };
            return node;
        }
        
        if (self.match("(")) {
            const expr = try self.parseExpression();
            _ = try self.consume(Token.Punctuation, "Expected ')'");
            return expr;
        }
        
        print("Unexpected token: {s}\n", .{self.current().value});
        return error.UnexpectedToken;
    }
    
    fn match(self: *Parser, expected: []const u8) bool {
        if (self.isAtEnd()) return false;
        if (std.mem.eql(u8, self.current().value, expected)) {
            _ = self.advance();
            return true;
        }
        return false;
    }
    
    fn check(self: *Parser, token_type: Token, value: []const u8) bool {
        if (self.isAtEnd()) return false;
        return self.current().type == token_type and std.mem.eql(u8, self.current().value, value);
    }
    
    fn consume(self: *Parser, token_type: Token, message: []const u8) !TokenData {
        if (self.current().type == token_type) {
            return self.advance();
        }
        
        print("Parse error: {s}. Got: {s}\n", .{message, self.current().value});
        return error.ParseError;
    }
    
    fn advance(self: *Parser) TokenData {
        if (!self.isAtEnd()) {
            self.pos += 1;
        }
        return self.previous();
    }
    
    fn isAtEnd(self: *Parser) bool {
        return self.pos >= self.tokens.items.len or self.current().type == Token.EOF;
    }
    
    fn current(self: *Parser) TokenData {
        if (self.pos < self.tokens.items.len) {
            return self.tokens.items[self.pos];
        }
        return TokenData{ .type = Token.EOF, .value = "", .line = 0, .column = 0 };
    }
    
    fn previous(self: *Parser) TokenData {
        return self.tokens.items[self.pos - 1];
    }
};

const LLVMCodeGen = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    variables: std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    pub fn init(allocator: Allocator, module_name: []const u8) !*LLVMCodeGen {
        // Initialize LLVM
        c.LLVMInitializeCore(c.LLVMGetGlobalPassRegistry());
        c.LLVMInitializeNativeTarget();
        c.LLVMInitializeNativeAsmPrinter();
        
        const context = c.LLVMContextCreate();
        const module_name_z = try std.fmt.allocPrintZ(allocator, "{s}", .{module_name});
        defer allocator.free(module_name_z);
        const module = c.LLVMModuleCreateWithNameInContext(module_name_z.ptr, context);
        const builder = c.LLVMCreateBuilderInContext(context);
        
        const codegen = try allocator.create(LLVMCodeGen);
        codegen.* = LLVMCodeGen{
            .allocator = allocator,
            .context = context,
            .module = module,
            .builder = builder,
            .variables = std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
        
        // Set target triple
        const triple = c.LLVMGetDefaultTargetTriple();
        c.LLVMSetTarget(module, triple);
        c.LLVMDisposeMessage(triple);
        
        return codegen;
    }
    
    pub fn deinit(self: *LLVMCodeGen) void {
        self.variables.deinit();
        c.LLVMDisposeBuilder(self.builder);
        c.LLVMDisposeModule(self.module);
        c.LLVMContextDispose(self.context);
        self.allocator.destroy(self);
    }
    
    pub fn generateProgram(self: *LLVMCodeGen, ast: std.ArrayList(*ASTNode)) !void {
        for (ast.items) |node| {
            try self.generateNode(node);
        }
    }
    
    fn generateNode(self: *LLVMCodeGen, node: *ASTNode) !void {
        switch (node.*) {
            .FunctionDef => |func| {
                try self.generateFunction(func);
            },
            else => {
                // Other nodes are handled within function context
            },
        }
    }
    
    fn generateFunction(self: *LLVMCodeGen, func: anytype) !void {
        // Create function type
        const return_type = if (std.mem.eql(u8, func.return_type, "drip"))
            c.LLVMInt32TypeInContext(self.context)
        else
            c.LLVMVoidTypeInContext(self.context);
        
        const func_type = c.LLVMFunctionType(return_type, null, 0, 0);
        const func_name_z = try std.fmt.allocPrintZ(self.allocator, "{s}", .{func.name});
        defer self.allocator.free(func_name_z);
        const llvm_func = c.LLVMAddFunction(self.module, func_name_z.ptr, func_type);
        
        // Create entry block
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, llvm_func, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Generate function body
        for (func.body.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Add default return if needed
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            if (std.mem.eql(u8, func.return_type, "drip")) {
                const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
                _ = c.LLVMBuildRet(self.builder, zero);
            } else {
                _ = c.LLVMBuildRetVoid(self.builder);
            }
        }
    }
    
    fn generateStatement(self: *LLVMCodeGen, node: *ASTNode) !void {
        switch (node.*) {
            .VarDecl => |var_decl| {
                const llvm_type = if (std.mem.eql(u8, var_decl.type_name, "drip"))
                    c.LLVMInt32TypeInContext(self.context)
                else if (std.mem.eql(u8, var_decl.type_name, "tea"))
                    c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)
                else
                    c.LLVMInt32TypeInContext(self.context);
                
                const var_name_z = try std.fmt.allocPrintZ(self.allocator, "{s}", .{var_decl.name});
                defer self.allocator.free(var_name_z);
                const alloca = c.LLVMBuildAlloca(self.builder, llvm_type, var_name_z.ptr);
                
                if (var_decl.value) |value| {
                    const llvm_value = try self.generateExpression(value);
                    _ = c.LLVMBuildStore(self.builder, llvm_value, alloca);
                }
                
                const key = try self.allocator.dupe(u8, var_decl.name);
                try self.variables.put(key, alloca);
            },
            
            .Assignment => |assign| {
                const alloca = self.variables.get(assign.name) orelse return error.UndefinedVariable;
                const value = try self.generateExpression(assign.value);
                _ = c.LLVMBuildStore(self.builder, value, alloca);
            },
            
            .ForLoop => |for_loop| {
                const function = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
                const loop_cond = c.LLVMAppendBasicBlockInContext(self.context, function, "loop_cond");
                const loop_body = c.LLVMAppendBasicBlockInContext(self.context, function, "loop_body");
                const loop_end = c.LLVMAppendBasicBlockInContext(self.context, function, "loop_end");
                
                // Generate initialization
                if (for_loop.init) |init_stmt| {
                    try self.generateStatement(init_stmt);
                }
                
                // Branch to condition check
                _ = c.LLVMBuildBr(self.builder, loop_cond);
                
                // Generate condition check
                c.LLVMPositionBuilderAtEnd(self.builder, loop_cond);
                if (for_loop.condition) |condition| {
                    const cond_value = try self.generateExpression(condition);
                    _ = c.LLVMBuildCondBr(self.builder, cond_value, loop_body, loop_end);
                } else {
                    _ = c.LLVMBuildBr(self.builder, loop_body);
                }
                
                // Generate loop body
                c.LLVMPositionBuilderAtEnd(self.builder, loop_body);
                for (for_loop.body.items) |stmt| {
                    try self.generateStatement(stmt);
                    // Check for early termination
                    if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) != null) {
                        break;
                    }
                }
                
                // Generate update
                if (for_loop.update) |update| {
                    try self.generateStatement(update);
                }
                
                // Branch back to condition (if not already terminated)
                if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
                    _ = c.LLVMBuildBr(self.builder, loop_cond);
                }
                
                // Continue with end block
                c.LLVMPositionBuilderAtEnd(self.builder, loop_end);
            },
            
            .IfStatement => |if_stmt| {
                const function = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
                const then_block = c.LLVMAppendBasicBlockInContext(self.context, function, "if_then");
                const else_block = c.LLVMAppendBasicBlockInContext(self.context, function, "if_else");
                const merge_block = c.LLVMAppendBasicBlockInContext(self.context, function, "if_merge");
                
                // Generate condition
                const cond_value = try self.generateExpression(if_stmt.condition);
                _ = c.LLVMBuildCondBr(self.builder, cond_value, then_block, else_block);
                
                // Generate then branch
                c.LLVMPositionBuilderAtEnd(self.builder, then_block);
                for (if_stmt.then_branch.items) |stmt| {
                    try self.generateStatement(stmt);
                    if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) != null) {
                        break;
                    }
                }
                if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
                    _ = c.LLVMBuildBr(self.builder, merge_block);
                }
                
                // Generate else branch
                c.LLVMPositionBuilderAtEnd(self.builder, else_block);
                if (if_stmt.else_branch) |else_stmts| {
                    for (else_stmts.items) |stmt| {
                        try self.generateStatement(stmt);
                        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) != null) {
                            break;
                        }
                    }
                }
                if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
                    _ = c.LLVMBuildBr(self.builder, merge_block);
                }
                
                // Continue with merge block
                c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
            },
            
            .FunctionCall => {
                _ = try self.generateExpression(node);
            },
            
            .Return => |ret| {
                if (ret) |value| {
                    const return_value = try self.generateExpression(value);
                    _ = c.LLVMBuildRet(self.builder, return_value);
                } else {
                    _ = c.LLVMBuildRetVoid(self.builder);
                }
            },
            
            .Continue => {
                // TODO: Implement continue (need to track loop blocks)
            },
            
            else => {
                // Expression statement
                _ = try self.generateExpression(node);
            },
        }
    }
    
    fn generateExpression(self: *LLVMCodeGen, node: *ASTNode) !c.LLVMValueRef {
        switch (node.*) {
            .Number => |num| {
                return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), @intCast(num), 0);
            },
            
            .String => |str| {
                const str_z = try std.fmt.allocPrintZ(self.allocator, "{s}", .{str});
                defer self.allocator.free(str_z);
                return c.LLVMBuildGlobalStringPtr(self.builder, str_z.ptr, "str");
            },
            
            .Identifier => |name| {
                const alloca = self.variables.get(name) orelse return error.UndefinedVariable;
                const name_z = try std.fmt.allocPrintZ(self.allocator, "{s}_load", .{name});
                defer self.allocator.free(name_z);
                return c.LLVMBuildLoad2(self.builder, c.LLVMInt32TypeInContext(self.context), alloca, name_z.ptr);
            },
            
            .BinaryOp => |binop| {
                const left = try self.generateExpression(binop.left);
                const right = try self.generateExpression(binop.right);
                
                if (std.mem.eql(u8, binop.op, "+")) {
                    return c.LLVMBuildAdd(self.builder, left, right, "add");
                } else if (std.mem.eql(u8, binop.op, "-")) {
                    return c.LLVMBuildSub(self.builder, left, right, "sub");
                } else if (std.mem.eql(u8, binop.op, "*")) {
                    return c.LLVMBuildMul(self.builder, left, right, "mul");
                } else if (std.mem.eql(u8, binop.op, "/")) {
                    return c.LLVMBuildSDiv(self.builder, left, right, "div");
                } else if (std.mem.eql(u8, binop.op, "%")) {
                    return c.LLVMBuildSRem(self.builder, left, right, "mod");
                } else if (std.mem.eql(u8, binop.op, "==")) {
                    return c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, left, right, "eq");
                } else if (std.mem.eql(u8, binop.op, "!=")) {
                    return c.LLVMBuildICmp(self.builder, c.LLVMIntNE, left, right, "ne");
                } else if (std.mem.eql(u8, binop.op, "<")) {
                    return c.LLVMBuildICmp(self.builder, c.LLVMIntSLT, left, right, "lt");
                } else if (std.mem.eql(u8, binop.op, "<=")) {
                    return c.LLVMBuildICmp(self.builder, c.LLVMIntSLE, left, right, "le");
                } else if (std.mem.eql(u8, binop.op, ">")) {
                    return c.LLVMBuildICmp(self.builder, c.LLVMIntSGT, left, right, "gt");
                } else if (std.mem.eql(u8, binop.op, ">=")) {
                    return c.LLVMBuildICmp(self.builder, c.LLVMIntSGE, left, right, "ge");
                }
                
                return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
            },
            
            .UnaryOp => |unop| {
                const operand = try self.generateExpression(unop.operand);
                if (std.mem.eql(u8, unop.op, "-")) {
                    return c.LLVMBuildNeg(self.builder, operand, "neg");
                }
                return operand;
            },
            
            .FunctionCall => |call| {
                if (std.mem.eql(u8, call.name, "vibez.spill")) {
                    // Handle print function
                    return try self.generatePrintCall(call.args);
                }
                return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
            },
            
            else => {
                return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
            },
        }
    }
    
    fn generatePrintCall(self: *LLVMCodeGen, args: std.ArrayList(*ASTNode)) !c.LLVMValueRef {
        // Declare printf if not exists
        var printf_func = c.LLVMGetNamedFunction(self.module, "printf");
        if (printf_func == null) {
            const printf_type = c.LLVMFunctionType(
                c.LLVMInt32TypeInContext(self.context),
                @ptrCast(&c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)),
                1,
                1 // varargs
            );
            printf_func = c.LLVMAddFunction(self.module, "printf", printf_type);
        }
        
        if (args.items.len > 0) {
            const arg = try self.generateExpression(args.items[0]);
            
            // Check if it's a string literal or integer
            const arg_type = c.LLVMTypeOf(arg);
            if (c.LLVMGetTypeKind(arg_type) == c.LLVMPointerTypeKind) {
                // String - add newline
                const format_str = c.LLVMBuildGlobalStringPtr(self.builder, "%s\n", "fmt_str");
                return c.LLVMBuildCall2(
                    self.builder,
                    c.LLVMGlobalGetValueType(printf_func.?),
                    printf_func.?,
                    @ptrCast(&[_]c.LLVMValueRef{ format_str, arg }),
                    2,
                    ""
                );
            } else {
                // Integer
                const format_str = c.LLVMBuildGlobalStringPtr(self.builder, "%d\n", "fmt_int");
                return c.LLVMBuildCall2(
                    self.builder,
                    c.LLVMGlobalGetValueType(printf_func.?),
                    printf_func.?,
                    @ptrCast(&[_]c.LLVMValueRef{ format_str, arg }),
                    2,
                    ""
                );
            }
        }
        
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }
    
    pub fn writeIRToFile(self: *LLVMCodeGen, filename: []const u8) !void {
        const filename_z = try std.fmt.allocPrintZ(self.allocator, "{s}", .{filename});
        defer self.allocator.free(filename_z);
        
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMPrintModuleToFile(self.module, filename_z.ptr, &error_msg) != 0) {
            defer c.LLVMDisposeMessage(error_msg);
            print("Failed to write IR: {s}\n", .{error_msg});
            return error.LLVMIRWriteFailed;
        }
        
        print("✅ LLVM IR written to: {s}\n", .{filename});
    }
    
    pub fn compileToNative(self: *LLVMCodeGen, output_file: []const u8) !void {
        // First write IR to temporary file
        const ir_file = "temp.ll";
        try self.writeIRToFile(ir_file);
        
        // Compile with clang
        const result = std.ChildProcess.exec(.{
            .allocator = self.allocator,
            .argv = &[_][]const u8{ "clang", "-O2", ir_file, "-o", output_file },
        }) catch |err| {
            print("❌ Error executing clang: {any}\n", .{err});
            return;
        };
        defer self.allocator.free(result.stdout);
        defer self.allocator.free(result.stderr);
        
        if (result.term.Exited != 0) {
            print("❌ Clang compilation failed:\n{s}\n", .{result.stderr});
            return;
        }
        
        print("✅ Native executable: {s}\n", .{output_file});
        
        // Clean up IR file
        std.fs.cwd().deleteFile(ir_file) catch {};
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        print("Usage: cursed-compiler [--compile] <source.csd> [output]\n", .{});
        return;
    }
    
    const compile_flag = std.mem.eql(u8, args[1], "--compile");
    const source_file = if (compile_flag) args[2] else args[1];
    const output_file = if (compile_flag and args.len > 3) args[3] else "output";
    
    // Read source file
    const source = std.fs.cwd().readFileAlloc(allocator, source_file, 1024 * 1024) catch |err| {
        print("❌ Error reading file: {any}\n", .{err});
        return;
    };
    defer allocator.free(source);
    
    print("🚀 Compiling CURSED source: {s}\n", .{source_file});
    
    // Tokenize
    var lexer = Lexer.init(source);
    var tokens = std.ArrayList(TokenData).init(allocator);
    defer tokens.deinit();
    
    while (true) {
        const token = try lexer.nextToken(allocator);
        if (token == null) break;
        try tokens.append(token.?);
        if (token.?.type == Token.EOF) break;
    }
    
    print("✅ Tokenized {} tokens\n", .{tokens.items.len});
    
    // Parse
    var parser = Parser.init(allocator, tokens);
    const ast = try parser.parse();
    defer {
        for (ast.items) |node| {
            // TODO: Free AST nodes properly
            _ = node;
        }
        ast.deinit();
    }
    
    print("✅ Parsed {} AST nodes\n", .{ast.items.len});
    
    if (compile_flag) {
        // Generate LLVM IR and compile to native
        var codegen = try LLVMCodeGen.init(allocator, "cursed_program");
        defer codegen.deinit();
        
        try codegen.generateProgram(ast);
        
        print("🔨 Compiling to native executable...\n");
        try codegen.compileToNative(output_file);
        
        print("🎉 Compilation complete! Run with: ./{s}\n", .{output_file});
    } else {
        print("✅ Syntax check passed\n");
    }
}
