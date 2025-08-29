const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const testing = std.testing;
const print = std.debug.print;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast.zig");

const Token = lexer.Token;
const TokenKind = lexer.TokenKind;
const Lexer = lexer.Lexer;
const Parser = parser.Parser;
const ParserError = parser.ParserError;
const Program = ast.Program;

/// Error types that can occur during frontend processing
pub const FrontendError = error{
    LexerError,
    ParserError,
    OutOfMemory,
    InvalidInput,
    FileNotFound,
} || ParserError;

/// Configuration options for frontend processing
pub const FrontendOptions = struct {
    /// Enable debug mode for verbose output
    debug: bool = false,
    /// Dump tokens to stdout during processing
    dump_tokens: bool = false,
    /// Dump AST to stdout after parsing
    dump_ast: bool = false,
    /// Maximum number of tokens to process (0 = unlimited)
    max_tokens: usize = 0,
    /// Enable error recovery during parsing
    enable_error_recovery: bool = true,
    
    pub fn init() FrontendOptions {
        return FrontendOptions{};
    }
    
    pub fn withDebug() FrontendOptions {
        return FrontendOptions{
            .debug = true,
            .dump_tokens = true,
            .dump_ast = true,
        };
    }
};

/// Result container for frontend processing
pub const FrontendResult = struct {
    /// Tokenized source code
    tokens: ArrayList(Token),
    /// Parsed AST
    ast: Program,
    /// Source file path
    file_path: []const u8,
    /// Processing succeeded without errors
    success: bool,
    /// Number of errors encountered
    error_count: usize,
    /// Lexer instance (for lifetime management)
    lexer_instance: Lexer,
    /// Parser instance (for lifetime management)
    parser_instance: Parser,
    /// Arena allocator for cleanup
    arena: std.heap.ArenaAllocator,
    
    pub fn init(allocator: Allocator, file_path: []const u8) FrontendResult {
        return FrontendResult{
            .tokens = ArrayList(Token){},
            .ast = undefined, // Will be set during parsing
            .file_path = file_path,
            .success = false,
            .error_count = 0,
            .lexer_instance = undefined,
            .parser_instance = undefined,
            .arena = std.heap.ArenaAllocator.init(allocator),
        };
    }
    
    pub fn deinit(self: *FrontendResult) void {
        // Clean up tokens
        self.tokens.deinit(self.arena.allocator());
        
        // Clean up parser (which includes AST cleanup)
        self.parser_instance.deinit();
        
        // Clean up arena
        self.arena.deinit();
    }
    
    /// Get the total number of tokens
    pub fn getTokenCount(self: *const FrontendResult) usize {
        return self.tokens.items.len;
    }
    
    /// Get the AST if parsing was successful
    pub fn getAst(self: *const FrontendResult) ?Program {
        if (self.success) {
            return self.ast;
        }
        return null;
    }
};

/// Main frontend pipeline that orchestrates lexing and parsing
pub const Frontend = struct {
    allocator: Allocator,
    options: FrontendOptions,
    
    pub fn init(allocator: Allocator, options: FrontendOptions) Frontend {
        return Frontend{
            .allocator = allocator,
            .options = options,
        };
    }
    
    /// Process source code through the complete frontend pipeline
    pub fn runFrontend(self: *Frontend, source: []const u8, file_path: []const u8) FrontendError!FrontendResult {
        var result = FrontendResult.init(self.allocator, file_path);
        errdefer result.deinit();
        
        if (self.options.debug) {
            print("[DEBUG] Starting frontend pipeline for: {s}\n", .{file_path});
            print("[DEBUG] Source length: {} bytes\n", .{source.len});
        }
        
        // Phase 1: Lexical Analysis
        try self.runLexer(source, &result);
        
        // Phase 2: Syntax Analysis  
        try self.runParser(&result);
        
        result.success = (result.error_count == 0);
        
        if (self.options.debug) {
            print("[DEBUG] Frontend pipeline completed\n");
            print("[DEBUG] Success: {}, Errors: {}, Tokens: {}\n", .{ 
                result.success, 
                result.error_count, 
                result.getTokenCount() 
            });
        }
        
        return result;
    }
    
    /// Run the lexer phase
    fn runLexer(self: *Frontend, source: []const u8, result: *FrontendResult) FrontendError!void {
        if (self.options.debug) {
            print("[DEBUG] Phase 1: Lexical Analysis\n");
        }
        
        // Initialize lexer
        result.lexer_instance = Lexer.init(result.arena.allocator(), source);
        
        // Tokenize source code
        result.tokens = result.lexer_instance.tokenize() catch |err| {
            result.error_count += 1;
            if (self.options.debug) {
                print("[ERROR] Lexer failed: {}\n", .{err});
            }
            return switch (err) {
                error.OutOfMemory => FrontendError.OutOfMemory,
                else => FrontendError.LexerError,
            };
        };
        
        // Check token limit
        if (self.options.max_tokens > 0 and result.tokens.items.len > self.options.max_tokens) {
            if (self.options.debug) {
                print("[WARNING] Token limit exceeded: {} > {}\n", .{ 
                    result.tokens.items.len, 
                    self.options.max_tokens 
                });
            }
            // Truncate tokens to limit
            result.tokens.shrinkRetainingCapacity(self.options.max_tokens);
        }
        
        // Debug output for tokens
        if (self.options.dump_tokens or self.options.debug) {
            try self.dumpTokens(&result.tokens);
        }
        
        if (self.options.debug) {
            print("[DEBUG] Lexer completed: {} tokens generated\n", .{result.tokens.items.len});
        }
    }
    
    /// Run the parser phase
    fn runParser(self: *Frontend, result: *FrontendResult) FrontendError!void {
        if (self.options.debug) {
            print("[DEBUG] Phase 2: Syntax Analysis\n");
        }
        
        // Initialize parser
        result.parser_instance = Parser.initWithFile(
            result.arena.allocator(), 
            result.tokens.items, 
            result.file_path
        );
        
        // Parse tokens into AST
        result.ast = result.parser_instance.parseProgram() catch |err| {
            result.error_count += 1;
            if (self.options.debug) {
                print("[ERROR] Parser failed: {}\n", .{err});
            }
            return switch (err) {
                error.OutOfMemory => FrontendError.OutOfMemory,
                else => FrontendError.ParserError,
            };
        };
        
        // Check if parser had errors (even if parsing completed)
        if (result.parser_instance.had_error) {
            result.error_count += 1;
            if (self.options.debug) {
                print("[WARNING] Parser completed with errors\n");
            }
        }
        
        // Debug output for AST
        if (self.options.dump_ast or self.options.debug) {
            try self.dumpAST(&result.ast, result.arena.allocator());
        }
        
        if (self.options.debug) {
            print("[DEBUG] Parser completed: {} statements in AST\n", .{result.ast.statements.items.len});
        }
    }
    
    /// Dump tokens to stdout for debugging
    fn dumpTokens(self: *Frontend, tokens: *const ArrayList(Token)) !void {
        _ = self;
        
        print("\n=== TOKEN DUMP ===\n");
        print("Total tokens: {}\n\n", .{tokens.items.len});
        
        for (tokens.items, 0..) |token, i| {
            print("{:3}: {}\n", .{ i, token });
        }
        
        print("\n=== END TOKEN DUMP ===\n\n");
    }
    
    /// Dump AST to stdout for debugging
    fn dumpAST(self: *Frontend, program: *const Program, allocator: Allocator) !void {
        _ = allocator; // May be used in the future for complex AST traversal
        
        print("\n=== AST DUMP ===\n");
        print("Program with {} statements:\n\n", .{program.statements.items.len});
        
        for (program.statements.items, 0..) |stmt, i| {
            print("Statement {}: {s}\n", .{ i, @tagName(stmt) });
            try self.dumpStatement(stmt, 1);
        }
        
        print("\n=== END AST DUMP ===\n\n");
    }
    
    /// Helper to recursively dump statement details
    fn dumpStatement(self: *Frontend, stmt: ast.Statement, depth: usize) !void {
        _ = self;
        
        const indent = "  " ** depth;
        
        switch (stmt) {
            .Let => |let_stmt| {
                print("{s}Let: {s} = <expression>\n", .{ indent, let_stmt.name });
            },
            .Function => |func_stmt| {
                print("{s}Function: {s}() -> <return_type>\n", .{ indent, func_stmt.name });
                print("{s}  Parameters: {}\n", .{ indent, func_stmt.parameters.items.len });
                print("{s}  Body statements: {}\n", .{ indent, func_stmt.body.items.len });
            },
            .Expression => {
                print("{s}Expression statement\n", .{indent});
            },
            .Return => {
                print("{s}Return statement\n", .{indent});
            },
            .Block => |block| {
                print("{s}Block with {} statements\n", .{ indent, block.statements.items.len });
            },
            .If => {
                print("{s}If statement\n", .{indent});
            },
            .While => {
                print("{s}While loop\n", .{indent});
            },
            .For => {
                print("{s}For loop\n", .{indent});
            },
            .Struct => |struct_stmt| {
                print("{s}Struct: {s}\n", .{ indent, struct_stmt.name });
            },
            .Interface => |interface_stmt| {
                print("{s}Interface: {s}\n", .{ indent, interface_stmt.name });
            },
            .Implementation => |impl_stmt| {
                print("{s}Implementation for: {s}\n", .{ indent, impl_stmt.target_type });
            },
            .Import => |import_stmt| {
                print("{s}Import: {s}\n", .{ indent, import_stmt.module });
            },
            else => {
                print("{s}Other statement: {s}\n", .{ indent, @tagName(stmt) });
            },
        }
    }
};

/// Convenience function for simple frontend processing
pub fn processCursedSource(allocator: Allocator, source: []const u8, file_path: []const u8) FrontendError!FrontendResult {
    var frontend = Frontend.init(allocator, FrontendOptions.init());
    return frontend.runFrontend(source, file_path);
}

/// Convenience function for frontend processing with debug output
pub fn processCursedSourceDebug(allocator: Allocator, source: []const u8, file_path: []const u8) FrontendError!FrontendResult {
    var frontend = Frontend.init(allocator, FrontendOptions.withDebug());
    return frontend.runFrontend(source, file_path);
}

// ===== TESTS =====

test "Frontend initialization" {
    const allocator = testing.allocator;
    const options = FrontendOptions.init();
    const frontend = Frontend.init(allocator, options);
    
    try testing.expect(frontend.allocator == allocator);
    try testing.expect(frontend.options.debug == false);
}

test "Frontend with debug options" {
    const allocator = testing.allocator;
    const options = FrontendOptions.withDebug();
    const frontend = Frontend.init(allocator, options);
    
    try testing.expect(frontend.options.debug == true);
    try testing.expect(frontend.options.dump_tokens == true);
    try testing.expect(frontend.options.dump_ast == true);
}

test "FrontendResult initialization" {
    const allocator = testing.allocator;
    var result = FrontendResult.init(allocator, "test.csd");
    defer result.deinit();
    
    try testing.expect(result.success == false);
    try testing.expect(result.error_count == 0);
    try testing.expect(std.mem.eql(u8, result.file_path, "test.csd"));
}

test "Simple CURSED source processing" {
    const allocator = testing.allocator;
    const source = 
        \\sus x drip = 42
        \\vibez.spill(x)
    ;
    
    var result = try processCursedSource(allocator, source, "test.csd");
    defer result.deinit();
    
    try testing.expect(result.getTokenCount() > 0);
    try testing.expect(result.ast.statements.items.len > 0);
}

test "Empty source handling" {
    const allocator = testing.allocator;
    const source = "";
    
    var result = try processCursedSource(allocator, source, "empty.csd");
    defer result.deinit();
    
    // Should still produce at least EOF token
    try testing.expect(result.getTokenCount() >= 1);
}

test "Invalid syntax error handling" {
    const allocator = testing.allocator;
    const source = "sus x drip = @ invalid syntax here";
    
    var result = processCursedSource(allocator, source, "invalid.csd") catch |err| {
        // Should fail with parser error
        try testing.expect(err == FrontendError.ParserError or err == FrontendError.LexerError);
        return;
    };
    defer result.deinit();
    
    // If we get here, parsing succeeded but may have errors
    try testing.expect(result.error_count > 0 or !result.success);
}

test "Token limit enforcement" {
    const allocator = testing.allocator;
    const source = 
        \\sus a drip = 1
        \\sus b drip = 2  
        \\sus c drip = 3
    ;
    
    var options = FrontendOptions.init();
    options.max_tokens = 5; // Very small limit
    
    var frontend = Frontend.init(allocator, options);
    var result = try frontend.runFrontend(source, "limited.csd");
    defer result.deinit();
    
    try testing.expect(result.getTokenCount() <= 5);
}

test "Debug mode output" {
    const allocator = testing.allocator;
    const source = "sus x drip = 42";
    
    var result = try processCursedSourceDebug(allocator, source, "debug.csd");
    defer result.deinit();
    
    // Debug mode should still work and produce valid results
    try testing.expect(result.getTokenCount() > 0);
    try testing.expect(result.ast.statements.items.len > 0);
}

test "Complex CURSED program processing" {
    const allocator = testing.allocator;
    const source = 
        \\slay fibonacci(n drip) drip {
        \\    ready (n <= 1) {
        \\        damn n
        \\    }
        \\    damn fibonacci(n - 1) + fibonacci(n - 2)
        \\}
        \\
        \\sus result drip = fibonacci(10)
        \\vibez.spill(result)
    ;
    
    var result = try processCursedSource(allocator, source, "fibonacci.csd");
    defer result.deinit();
    
    try testing.expect(result.success == true or result.error_count == 0);
    try testing.expect(result.getTokenCount() > 20); // Should have many tokens
    try testing.expect(result.ast.statements.items.len >= 2); // Function + variable
}

test "Memory management" {
    const allocator = testing.allocator;
    const source = "sus x drip = 42";
    
    // Process multiple sources to test cleanup
    for (0..10) |i| {
        var filename_buf: [32]u8 = undefined;
        const filename = try std.fmt.bufPrint(filename_buf[0..], "test{}.csd", .{i});
        
        var result = try processCursedSource(allocator, source, filename);
        defer result.deinit();
        
        try testing.expect(result.getTokenCount() > 0);
    }
    
    // If we reach here without memory leaks, cleanup worked correctly
}
