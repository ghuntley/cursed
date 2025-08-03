const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Simple token types for CURSED language
const TokenType = enum {
    // Literals
    STRING,
    NUMBER,
    IDENTIFIER,
    
    // Keywords
    VIBEZ,    // vibez.spill
    SPILL,    // spill
    DOT,      // .
    
    // Operators
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    
    // Delimiters
    LPAREN,   // (
    RPAREN,   // )
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

// Simple AST node types
const ASTNode = union(enum) {
    FunctionCall: struct {
        name: []const u8,
        args: ArrayList(ASTNode),
    },
    StringLiteral: []const u8,
    NumberLiteral: f64,
    Program: ArrayList(ASTNode),
};

// Simple lexer
const SimpleLexer = struct {
    source: []const u8,
    position: usize,
    line: usize,
    column: usize,
    allocator: Allocator,
    
    const Self = @This();
    
    fn init(allocator: Allocator, source: []const u8) Self {
        return Self{
            .source = source,
            .position = 0,
            .line = 1,
            .column = 1,
            .allocator = allocator,
        };
    }
    
    fn isAtEnd(self: *Self) bool {
        return self.position >= self.source.len;
    }
    
    fn advance(self: *Self) u8 {
        if (self.isAtEnd()) return 0;
        const char = self.source[self.position];
        self.position += 1;
        if (char == '\n') {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        return char;
    }
    
    fn peek(self: *Self) u8 {
        if (self.isAtEnd()) return 0;
        return self.source[self.position];
    }
    
    fn skipWhitespace(self: *Self) void {
        while (!self.isAtEnd()) {
            const char = self.peek();
            if (char == ' ' or char == '\t' or char == '\r') {
                _ = self.advance();
            } else {
                break;
            }
        }
    }
    
    fn tokenizeString(self: *Self) !Token {
        const start = self.position;
        _ = self.advance(); // Skip opening quote
        
        while (!self.isAtEnd() and self.peek() != '"') {
            _ = self.advance();
        }
        
        if (self.isAtEnd()) {
            return error.UnterminatedString;
        }
        
        const value = self.source[start + 1..self.position];
        _ = self.advance(); // Skip closing quote
        
        return Token{
            .type = .STRING,
            .value = value,
            .line = self.line,
            .column = self.column,
        };
    }
    
    fn tokenizeNumber(self: *Self) Token {
        const start = self.position;
        
        while (!self.isAtEnd() and (std.ascii.isDigit(self.peek()) or self.peek() == '.')) {
            _ = self.advance();
        }
        
        return Token{
            .type = .NUMBER,
            .value = self.source[start..self.position],
            .line = self.line,
            .column = self.column,
        };
    }
    
    fn tokenizeIdentifier(self: *Self) Token {
        const start = self.position;
        
        while (!self.isAtEnd() and (std.ascii.isAlphabetic(self.peek()) or std.ascii.isDigit(self.peek()) or self.peek() == '_')) {
            _ = self.advance();
        }
        
        const value = self.source[start..self.position];
        var token_type = TokenType.IDENTIFIER;
        if (std.mem.eql(u8, value, "vibez")) {
            token_type = .VIBEZ;
        } else if (std.mem.eql(u8, value, "spill")) {
            token_type = .SPILL;
        }
        
        return Token{
            .type = token_type,
            .value = value,
            .line = self.line,
            .column = self.column,
        };
    }
    
    fn nextToken(self: *Self) !Token {
        self.skipWhitespace();
        
        if (self.isAtEnd()) {
            return Token{
                .type = .EOF,
                .value = "",
                .line = self.line,
                .column = self.column,
            };
        }
        
        const char = self.peek();
        
        return switch (char) {
            '"' => try self.tokenizeString(),
            '(' => {
                _ = self.advance();
                return Token{ .type = .LPAREN, .value = "(", .line = self.line, .column = self.column };
            },
            ')' => {
                _ = self.advance();
                return Token{ .type = .RPAREN, .value = ")", .line = self.line, .column = self.column };
            },
            ',' => {
                _ = self.advance();
                return Token{ .type = .COMMA, .value = ",", .line = self.line, .column = self.column };
            },
            ';' => {
                _ = self.advance();
                return Token{ .type = .SEMICOLON, .value = ";", .line = self.line, .column = self.column };
            },
            '.' => {
                _ = self.advance();
                return Token{ .type = .DOT, .value = ".", .line = self.line, .column = self.column };
            },
            '+' => {
                _ = self.advance();
                return Token{ .type = .PLUS, .value = "+", .line = self.line, .column = self.column };
            },
            '-' => {
                _ = self.advance();
                return Token{ .type = .MINUS, .value = "-", .line = self.line, .column = self.column };
            },
            '*' => {
                _ = self.advance();
                return Token{ .type = .MULTIPLY, .value = "*", .line = self.line, .column = self.column };
            },
            '/' => {
                _ = self.advance();
                return Token{ .type = .DIVIDE, .value = "/", .line = self.line, .column = self.column };
            },
            '\n' => {
                _ = self.advance();
                return Token{ .type = .NEWLINE, .value = "\n", .line = self.line, .column = self.column };
            },
            else => {
                if (std.ascii.isDigit(char)) {
                    return self.tokenizeNumber();
                } else if (std.ascii.isAlphabetic(char) or char == '_') {
                    return self.tokenizeIdentifier();
                } else {
                    _ = self.advance();
                    return Token{ .type = .UNKNOWN, .value = self.source[self.position-1..self.position], .line = self.line, .column = self.column };
                }
            },
        };
    }
    
    fn tokenize(self: *Self) !ArrayList(Token) {
        var tokens = ArrayList(Token).init(self.allocator);
        
        while (true) {
            const token = try self.nextToken();
            if (token.type == .EOF) break;
            if (token.type != .NEWLINE) { // Skip newlines
                try tokens.append(token);
            }
        }
        
        return tokens;
    }
};

// Simple parser
const SimpleParser = struct {
    tokens: []const Token,
    position: usize,
    allocator: Allocator,
    
    const Self = @This();
    
    fn init(allocator: Allocator, tokens: []const Token) Self {
        return Self{
            .tokens = tokens,
            .position = 0,
            .allocator = allocator,
        };
    }
    
    fn isAtEnd(self: *Self) bool {
        return self.position >= self.tokens.len or self.tokens[self.position].type == .EOF;
    }
    
    fn advance(self: *Self) Token {
        if (self.isAtEnd()) {
            return Token{ .type = .EOF, .value = "", .line = 0, .column = 0 };
        }
        const token = self.tokens[self.position];
        self.position += 1;
        return token;
    }
    
    fn peek(self: *Self) Token {
        if (self.isAtEnd()) {
            return Token{ .type = .EOF, .value = "", .line = 0, .column = 0 };
        }
        return self.tokens[self.position];
    }
    
    fn parseExpression(self: *Self) error{OutOfMemory, UnexpectedToken}!ASTNode {
        const token = self.peek();
        
        switch (token.type) {
            .STRING => {
                _ = self.advance();
                return ASTNode{ .StringLiteral = token.value };
            },
            .NUMBER => {
                _ = self.advance();
                const value = std.fmt.parseFloat(f64, token.value) catch 0.0;
                return ASTNode{ .NumberLiteral = value };
            },
            .VIBEZ => {
                return try self.parseFunctionCall();
            },
            else => {
                return error.UnexpectedToken;
            },
        }
    }
    
    fn parseFunctionCall(self: *Self) !ASTNode {
        var name_parts = ArrayList(u8).init(self.allocator);
        defer name_parts.deinit();
        
        // Parse "vibez.spill"
        const vibez_token = self.advance();
        try name_parts.appendSlice(vibez_token.value);
        
        if (self.peek().type == .DOT) {
            _ = self.advance(); // consume dot
            try name_parts.append('.');
            
            const spill_token = self.advance();
            try name_parts.appendSlice(spill_token.value);
        }
        
        const name = try name_parts.toOwnedSlice();
        
        // Parse arguments
        var args = ArrayList(ASTNode).init(self.allocator);
        
        if (self.peek().type == .LPAREN) {
            _ = self.advance(); // consume (
            
            while (!self.isAtEnd() and self.peek().type != .RPAREN) {
                const arg = try self.parseExpression();
                try args.append(arg);
                
                if (self.peek().type == .COMMA) {
                    _ = self.advance(); // consume comma
                }
            }
            
            if (self.peek().type == .RPAREN) {
                _ = self.advance(); // consume )
            }
        }
        
        return ASTNode{ .FunctionCall = .{ .name = name, .args = args } };
    }
    
    fn parseStatement(self: *Self) !ASTNode {
        return try self.parseExpression();
    }
    
    fn parse(self: *Self) !ASTNode {
        var statements = ArrayList(ASTNode).init(self.allocator);
        
        while (!self.isAtEnd()) {
            const stmt = try self.parseStatement();
            try statements.append(stmt);
            
            // Skip optional semicolon
            if (self.peek().type == .SEMICOLON) {
                _ = self.advance();
            }
        }
        
        return ASTNode{ .Program = statements };
    }
};

// Simple interpreter
const SimpleInterpreter = struct {
    allocator: Allocator,
    
    const Self = @This();
    
    fn init(allocator: Allocator) Self {
        return Self{ .allocator = allocator };
    }
    
    fn executeNode(self: *Self, node: ASTNode) !void {
        switch (node) {
            .Program => |statements| {
                for (statements.items) |stmt| {
                    try self.executeNode(stmt);
                }
            },
            .FunctionCall => |call| {
                if (std.mem.eql(u8, call.name, "vibez.spill")) {
                    for (call.args.items) |arg| {
                        switch (arg) {
                            .StringLiteral => |str| print("{s}", .{str}),
                            .NumberLiteral => |num| print("{d}", .{num}),
                            else => print("(expression)", .{}),
                        }
                    }
                    print("\n", .{});
                }
            },
            .StringLiteral => |str| {
                print("{s}", .{str});
            },
            .NumberLiteral => |num| {
                print("{d}", .{num});
            },
        }
    }
};

// Simple compiler to executable
const SimpleCompiler = struct {
    allocator: Allocator,
    
    const Self = @This();
    
    fn init(allocator: Allocator) Self {
        return Self{ .allocator = allocator };
    }
    
    fn compile(self: *Self, ast: ASTNode, output_path: []const u8) !void {
        // Generate a simple C program
        var c_code = ArrayList(u8).init(self.allocator);
        defer c_code.deinit();
        
        try c_code.appendSlice("#include <stdio.h>\n\nint main() {\n");
        
        try self.generateC(ast, &c_code);
        
        try c_code.appendSlice("    return 0;\n}\n");
        
        // Write C file
        const c_file_path = try std.fmt.allocPrint(self.allocator, "{s}.c", .{output_path});
        defer self.allocator.free(c_file_path);
        
        const c_file = try std.fs.cwd().createFile(c_file_path, .{});
        defer c_file.close();
        
        try c_file.writeAll(c_code.items);
        
        // Compile with gcc
        const compile_result = try std.process.Child.run(.{
            .allocator = self.allocator,
            .argv = &[_][]const u8{ "gcc", "-o", output_path, c_file_path },
        });
        defer self.allocator.free(compile_result.stdout);
        defer self.allocator.free(compile_result.stderr);
        
        if (compile_result.term.Exited != 0) {
            print("Compilation failed: {s}\n", .{compile_result.stderr});
            return error.CompilationFailed;
        }
        
        // Clean up C file
        std.fs.cwd().deleteFile(c_file_path) catch {};
    }
    
    fn generateC(self: *Self, node: ASTNode, code: *ArrayList(u8)) !void {
        switch (node) {
            .Program => |statements| {
                for (statements.items) |stmt| {
                    try self.generateC(stmt, code);
                }
            },
            .FunctionCall => |call| {
                if (std.mem.eql(u8, call.name, "vibez.spill")) {
                    try code.appendSlice("    printf(");
                    for (call.args.items, 0..) |arg, i| {
                        if (i > 0) try code.appendSlice(", ");
                        switch (arg) {
                            .StringLiteral => |str| {
                                try code.appendSlice("\"");
                                try code.appendSlice(str);
                                try code.appendSlice("\\n\"");
                            },
                            .NumberLiteral => |num| {
                                const num_str = try std.fmt.allocPrint(self.allocator, "\"{d}\\n\"", .{num});
                                defer self.allocator.free(num_str);
                                try code.appendSlice(num_str);
                            },
                            else => try code.appendSlice("\"(expression)\\n\""),
                        }
                    }
                    try code.appendSlice(");\n");
                }
            },
            else => {},
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
        printUsage();
        return;
    }

    if (std.mem.eql(u8, args[1], "--version")) {
        print("CURSED Minimal Working Compiler v1.0.0\n", .{});
        print("Supports: vibez.spill() function calls with string literals\n", .{});
        return;
    }

    const filename = args[1];
    
    // Parse command line options
    var compile_mode = false;
    var debug_mode = false;
    
    for (args[2..]) |arg| {
        if (std.mem.eql(u8, arg, "--compile")) {
            compile_mode = true;
        } else if (std.mem.eql(u8, arg, "--debug")) {
            debug_mode = true;
        }
    }

    // Read source file
    const file = std.fs.cwd().openFile(filename, .{}) catch |err| {
        print("Error: Could not open file '{s}': {}\n", .{ filename, err });
        return;
    };
    defer file.close();

    const source = file.readToEndAlloc(allocator, 1024 * 1024) catch |err| {
        print("Error: Could not read file '{s}': {}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);

    // Tokenize
    var lexer = SimpleLexer.init(allocator, source);
    const tokens = lexer.tokenize() catch |err| {
        print("Lexer error: {}\n", .{err});
        return;
    };
    defer tokens.deinit();

    if (debug_mode) {
        print("=== TOKENS ===\n", .{});
        for (tokens.items) |token| {
            print("{}: '{s}'\n", .{ token.type, token.value });
        }
        print("\n", .{});
    }

    // Parse
    var parser = SimpleParser.init(allocator, tokens.items);
    const ast = parser.parse() catch |err| {
        print("Parser error: {}\n", .{err});
        return;
    };

    if (debug_mode) {
        print("=== AST ===\n", .{});
        print("Parsed AST successfully\n", .{});
        print("\n", .{});
    }

    if (compile_mode) {
        // Compile to native executable
        const output_name = try getOutputName(allocator, filename);
        defer allocator.free(output_name);
        
        print("🔧 Compiling '{s}' to native executable '{s}'\n", .{ filename, output_name });
        
        var compiler = SimpleCompiler.init(allocator);
        compiler.compile(ast, output_name) catch |err| {
            print("Compilation error: {}\n", .{err});
            return;
        };
        
        print("✅ Generated native executable: {s}\n", .{output_name});
        print("🚀 Test it: ./{s}\n", .{output_name});
    } else {
        // Interpret directly
        print("🚀 Interpreting CURSED program...\n", .{});
        
        var interpreter = SimpleInterpreter.init(allocator);
        interpreter.executeNode(ast) catch |err| {
            print("Interpreter error: {}\n", .{err});
            return;
        };
        
        print("✅ Program execution completed\n", .{});
    }
}

fn printUsage() void {
    print("CURSED Minimal Working Compiler v1.0.0\n", .{});
    print("A functional compiler that actually works!\n", .{});
    print("\nUsage: cursed-zig <file.csd> [OPTIONS]\n", .{});
    print("       cursed-zig --version\n", .{});
    print("\nOptions:\n", .{});
    print("  --compile     Generate native executable (uses GCC)\n", .{});
    print("  --debug       Show debug output (tokens, AST)\n", .{});
    print("\nSupported CURSED features:\n", .{});
    print("  • vibez.spill(\"text\") - Print text to console\n", .{});
    print("  • String literals with double quotes\n", .{});
    print("  • Basic arithmetic (planned)\n", .{});
    print("\nExample CURSED program:\n", .{});
    print("  vibez.spill(\"Hello from CURSED!\")\n", .{});
}

fn getOutputName(allocator: Allocator, filename: []const u8) ![]u8 {
    if (std.mem.endsWith(u8, filename, ".csd")) {
        return try allocator.dupe(u8, filename[0..filename.len - 4]);
    }
    return try std.fmt.allocPrint(allocator, "{s}_out", .{filename});
}
