const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const interpreter = @import("interpreter.zig");
const comprehensive_error = @import("comprehensive_error_runtime.zig");

/// Enhanced Error Handling Integration
/// Integrates comprehensive error runtime with parser, interpreter, and codegen

const ErrorRuntime = comprehensive_error.ErrorRuntime;
const YikesError = comprehensive_error.YikesError;
const ShookResult = comprehensive_error.ShookResult;
const FamBlock = comprehensive_error.FamBlock;
const ErrorType = comprehensive_error.ErrorType;
const ErrorSeverity = comprehensive_error.ErrorSeverity;

/// Enhanced parser with comprehensive error handling
pub const EnhancedParser = struct {
    base_parser: parser.Parser,
    error_runtime: *ErrorRuntime,
    allocator: Allocator,

    pub fn init(allocator: Allocator, tokens: []const lexer.Token) !EnhancedParser {
        var error_runtime = try allocator.create(ErrorRuntime);
        error_runtime.* = ErrorRuntime.init(allocator);

        return EnhancedParser{
            .base_parser = parser.Parser.init(allocator, tokens),
            .error_runtime = error_runtime,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *EnhancedParser) void {
        self.error_runtime.deinit();
        self.allocator.destroy(self.error_runtime);
        self.base_parser.deinit();
    }

    /// Parse with enhanced error handling and recovery
    pub fn parse(self: *EnhancedParser) !ast.Program {
        try self.error_runtime.enterFunction("parse", "parser.zig", 1, 1);
        defer self.error_runtime.exitFunction();

        const result = self.parseWithRecovery();
        return switch (result) {
            .Ok => |program| switch (program) {
                .Pointer => |ptr| @as(*ast.Program, @ptrCast(@alignCast(ptr))).*,
                else => unreachable,
            },
            .Error => |error_obj| {
                // Convert to parser error
                const stderr = std.io.getStdErr().writer();
                try error_obj.format(stderr);
                return switch (error_obj.error_type) {
                    .parse_yikes => parser.ParserError.UnexpectedToken,
                    .memory_yikes => parser.ParserError.OutOfMemory,
                    else => parser.ParserError.UnexpectedToken,
                };
            },
        };
    }

    fn parseWithRecovery(self: *EnhancedParser) ShookResult {
        // Create fam block for error recovery
        var fam_block = FamBlock.init(self.allocator);
        defer fam_block.deinit();

        // Add try handler
        const try_handler = struct {
            fn parse_try(parser_self: *EnhancedParser) fn (*ErrorRuntime) ShookResult {
                return struct {
                    fn inner(runtime: *ErrorRuntime) ShookResult {
                        _ = runtime;
                        const program = parser_self.base_parser.parse() catch |err| {
                            const error_obj = YikesError.init(
                                parser_self.allocator,
                                @errorName(err),
                                .parse_yikes,
                                .error,
                                @intFromError(err)
                            ) catch unreachable;
                            return ShookResult.err(error_obj);
                        };
                        
                        const program_ptr = parser_self.allocator.create(ast.Program) catch unreachable;
                        program_ptr.* = program;
                        return ShookResult.ok(ShookResult.Value{ .Pointer = @ptrCast(program_ptr) });
                    }
                }.inner;
            }
        };

        try fam_block.addTryHandler(try_handler.parse_try(self)(self.error_runtime));

        // Add catch handlers for different error types
        const parse_error_handler = struct {
            fn handle(error_obj: YikesError, runtime: *ErrorRuntime) ShookResult {
                _ = runtime;
                
                // Attempt error recovery based on error type
                switch (error_obj.error_type) {
                    .parse_yikes => {
                        // Try to recover from parse errors
                        std.log.warn("Parse error recovery attempted: {s}", .{error_obj.message});
                        
                        // Create empty program as fallback
                        const allocator = error_obj.allocator;
                        const empty_program = ast.Program{
                            .statements = ArrayList(ast.Statement).init(allocator),
                        };
                        
                        const program_ptr = allocator.create(ast.Program) catch unreachable;
                        program_ptr.* = empty_program;
                        return ShookResult.ok(ShookResult.Value{ .Pointer = @ptrCast(program_ptr) });
                    },
                    else => return ShookResult.err(error_obj),
                }
            }
        }.handle;

        try fam_block.addCatchHandler(.parse_yikes, null, parse_error_handler);

        // Execute with error handling
        return fam_block.execute(self.error_runtime);
    }

    /// Enhanced statement parsing with error context
    pub fn parseStatement(self: *EnhancedParser) !ast.Statement {
        try self.error_runtime.enterFunction("parseStatement", "parser.zig", 100, 1);
        defer self.error_runtime.exitFunction();

        // Check for error handling statements
        if (self.base_parser.check(.Yikes)) {
            return self.parseYikesStatement();
        }
        
        if (self.base_parser.check(.Fam)) {
            return self.parseFamStatement();
        }

        // Delegate to base parser with error wrapping
        return self.base_parser.parseStatement() catch |err| {
            const location = YikesError.SourceLocation{
                .file = "current_file.csd",
                .line = @intCast(self.base_parser.current_token().line),
                .column = @intCast(self.base_parser.current_token().column),
                .function = "parseStatement",
            };

            var error_obj = try self.error_runtime.yikes(
                @errorName(err),
                .parse_yikes,
                .error,
                @intFromError(err),
                location
            );

            try error_obj.addContext("token", @tagName(self.base_parser.current_token().type));
            
            return err;
        };
    }

    fn parseYikesStatement(self: *EnhancedParser) !ast.Statement {
        try self.error_runtime.setLocalVariable("parsing", "yikes_statement");
        
        const yikes_token = self.base_parser.advance();
        _ = yikes_token;

        // Parse yikes message expression
        const message_expr = try self.base_parser.parseExpression();
        
        // Parse optional error code
        var code_expr: ?*ast.Expression = null;
        if (self.base_parser.match(.Comma)) {
            code_expr = try self.base_parser.parseExpression();
        }

        return ast.Statement{
            .Yikes = ast.YikesStatement{
                .message = try self.allocator.create(ast.Expression),
                .code = code_expr,
                .error_type = .runtime_yikes,
                .location = null,
            }
        };
    }

    fn parseFamStatement(self: *EnhancedParser) !ast.Statement {
        try self.error_runtime.setLocalVariable("parsing", "fam_statement");
        
        const fam_token = self.base_parser.advance();
        _ = fam_token;

        // Parse fam { try_body } catch(...) { catch_body } finally { finally_body }
        try self.base_parser.consume(.LeftBrace, "Expected '{' after 'fam'");
        
        var try_body = ArrayList(ast.Statement).init(self.allocator);
        while (!self.base_parser.check(.RightBrace) and !self.base_parser.isAtEnd()) {
            const stmt = try self.parseStatement();
            try try_body.append(stmt);
        }
        
        try self.base_parser.consume(.RightBrace, "Expected '}' after try body");

        // Parse catch blocks
        var catch_blocks = ArrayList(ast.FamStatement.CatchBlock).init(self.allocator);
        while (self.base_parser.match(.Catch) or self.base_parser.check(.Identifier)) {
            if (self.base_parser.previous().type == .Identifier and 
                std.mem.eql(u8, self.base_parser.previous().lexeme, "catch")) {
                
                // Parse catch(error_type) or catch(error_var)
                var error_pattern: ?[]const u8 = null;
                var error_variable: ?[]const u8 = null;
                
                if (self.base_parser.match(.LeftParen)) {
                    const identifier_token = try self.base_parser.consume(.Identifier, "Expected error pattern or variable");
                    error_variable = identifier_token.lexeme;
                    try self.base_parser.consume(.RightParen, "Expected ')' after error variable");
                }

                try self.base_parser.consume(.LeftBrace, "Expected '{' after catch clause");
                
                var catch_body = ArrayList(ast.Statement).init(self.allocator);
                while (!self.base_parser.check(.RightBrace) and !self.base_parser.isAtEnd()) {
                    const stmt = try self.parseStatement();
                    try catch_body.append(stmt);
                }
                
                try self.base_parser.consume(.RightBrace, "Expected '}' after catch body");

                const catch_block = ast.FamStatement.CatchBlock{
                    .error_pattern = error_pattern,
                    .error_variable = error_variable,
                    .body = catch_body,
                };
                
                try catch_blocks.append(catch_block);
            } else {
                break;
            }
        }

        // Parse optional finally block
        var finally_block: ?ArrayList(ast.Statement) = null;
        if (self.base_parser.match(.Finally) or 
           (self.base_parser.check(.Identifier) and std.mem.eql(u8, self.base_parser.peek().lexeme, "finally"))) {
            
            if (self.base_parser.previous().type == .Identifier) {
                _ = self.base_parser.advance(); // consume "finally"
            }
            
            try self.base_parser.consume(.LeftBrace, "Expected '{' after 'finally'");
            
            var finally_body = ArrayList(ast.Statement).init(self.allocator);
            while (!self.base_parser.check(.RightBrace) and !self.base_parser.isAtEnd()) {
                const stmt = try self.parseStatement();
                try finally_body.append(stmt);
            }
            
            try self.base_parser.consume(.RightBrace, "Expected '}' after finally body");
            finally_block = finally_body;
        }

        return ast.Statement{
            .Fam = ast.FamStatement{
                .try_body = try_body,
                .catch_blocks = catch_blocks,
                .finally_block = finally_block,
            }
        };
    }
};

/// Enhanced interpreter with comprehensive error handling
pub const EnhancedInterpreter = struct {
    base_interpreter: interpreter.Interpreter,
    error_runtime: *ErrorRuntime,
    allocator: Allocator,

    pub fn init(allocator: Allocator) !EnhancedInterpreter {
        var error_runtime = try allocator.create(ErrorRuntime);
        error_runtime.* = ErrorRuntime.init(allocator);

        return EnhancedInterpreter{
            .base_interpreter = interpreter.Interpreter.init(allocator),
            .error_runtime = error_runtime,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *EnhancedInterpreter) void {
        self.error_runtime.deinit();
        self.allocator.destroy(self.error_runtime);
        self.base_interpreter.deinit();
    }

    pub fn interpret(self: *EnhancedInterpreter, program: ast.Program) !void {
        try self.error_runtime.enterFunction("interpret", "interpreter.zig", 1, 1);
        defer self.error_runtime.exitFunction();

        for (program.statements.items) |stmt| {
            try self.executeStatement(stmt);
        }
    }

    fn executeStatement(self: *EnhancedInterpreter, stmt: ast.Statement) !void {
        switch (stmt) {
            .Yikes => |yikes| try self.executeYikesStatement(yikes),
            .Fam => |fam| try self.executeFamStatement(fam),
            else => {
                // Delegate to base interpreter with error wrapping
                self.base_interpreter.executeStatement(stmt) catch |err| {
                    const location = YikesError.SourceLocation{
                        .file = "current_file.csd",
                        .line = 1, // TODO: Get actual line from statement
                        .column = 1,
                        .function = "executeStatement",
                    };

                    var error_obj = try self.error_runtime.yikes(
                        @errorName(err),
                        .runtime_yikes,
                        .error,
                        @intFromError(err),
                        location
                    );

                    try error_obj.addContext("statement_type", @tagName(stmt));
                    
                    return err;
                };
            },
        }
    }

    fn executeYikesStatement(self: *EnhancedInterpreter, yikes: ast.YikesStatement) !void {
        try self.error_runtime.setLocalVariable("executing", "yikes_statement");

        // Evaluate message expression
        const message_value = try self.base_interpreter.evaluateExpression(yikes.message.*);
        const message = switch (message_value) {
            .String => |str| str,
            else => "Error message evaluation failed",
        };

        // Evaluate optional code expression
        const code = if (yikes.code) |code_expr| blk: {
            const code_value = try self.base_interpreter.evaluateExpression(code_expr.*);
            break :blk switch (code_value) {
                .Integer => |i| i,
                else => 0,
            };
        } else 0;

        // Create location info
        const location = if (yikes.location) |loc| YikesError.SourceLocation{
            .file = loc.file,
            .line = loc.line,
            .column = loc.column,
            .function = "executeYikesStatement",
        } else null;

        // Create and throw error
        var error_obj = try self.error_runtime.yikes(
            message,
            yikes.error_type,
            .error,
            code,
            location
        );

        // Add execution context
        try error_obj.addContext("execution_mode", "interpreter");
        try error_obj.addContext("statement_type", "yikes");

        // Propagate error
        try self.error_runtime.propagateError(&error_obj);
    }

    fn executeFamStatement(self: *EnhancedInterpreter, fam: ast.FamStatement) !void {
        try self.error_runtime.setLocalVariable("executing", "fam_statement");

        // Create fam block
        var fam_block = FamBlock.init(self.allocator);
        defer fam_block.deinit();

        // Enter fam block in runtime
        try self.error_runtime.famEnter(&fam_block);
        defer self.error_runtime.famExit();

        // Create try handler
        const try_handler = struct {
            fn create(interpreter_ref: *EnhancedInterpreter, try_body: ArrayList(ast.Statement)) fn (*ErrorRuntime) ShookResult {
                return struct {
                    fn execute(runtime: *ErrorRuntime) ShookResult {
                        _ = runtime;
                        
                        for (try_body.items) |stmt| {
                            interpreter_ref.executeStatement(stmt) catch |err| {
                                const error_obj = YikesError.init(
                                    interpreter_ref.allocator,
                                    @errorName(err),
                                    .runtime_yikes,
                                    .error,
                                    @intFromError(err)
                                ) catch unreachable;
                                return ShookResult.err(error_obj);
                            };
                        }
                        
                        return ShookResult.ok(ShookResult.Value{ .Void = {} });
                    }
                }.execute;
            }
        };

        try fam_block.addTryHandler(try_handler.create(self, fam.try_body));

        // Add catch handlers
        for (fam.catch_blocks.items) |catch_block| {
            const catch_handler = struct {
                fn create(
                    interpreter_ref: *EnhancedInterpreter, 
                    catch_body: ArrayList(ast.Statement),
                    error_var: ?[]const u8
                ) fn (YikesError, *ErrorRuntime) ShookResult {
                    return struct {
                        fn handle(error_obj: YikesError, runtime: *ErrorRuntime) ShookResult {
                            _ = runtime;
                            
                            // Set error variable if specified
                            if (error_var) |var_name| {
                                // TODO: Set error variable in interpreter scope
                                _ = var_name;
                            }
                            
                            for (catch_body.items) |stmt| {
                                interpreter_ref.executeStatement(stmt) catch |err| {
                                    const catch_error_obj = YikesError.init(
                                        interpreter_ref.allocator,
                                        @errorName(err),
                                        .runtime_yikes,
                                        .error,
                                        @intFromError(err)
                                    ) catch unreachable;
                                    return ShookResult.err(catch_error_obj);
                                };
                            }
                            
                            return ShookResult.ok(ShookResult.Value{ .Void = {} });
                        }
                    }.handle;
                }
            };

            try fam_block.addCatchHandler(
                null, // Accept any error type for now
                catch_block.error_pattern,
                catch_handler.create(self, catch_block.body, catch_block.error_variable)
            );
        }

        // Set finally handler if present
        if (fam.finally_block) |finally_body| {
            const finally_handler = struct {
                fn create(interpreter_ref: *EnhancedInterpreter, finally_stmts: ArrayList(ast.Statement)) fn (*ErrorRuntime) void {
                    return struct {
                        fn execute(runtime: *ErrorRuntime) void {
                            _ = runtime;
                            
                            for (finally_stmts.items) |stmt| {
                                interpreter_ref.executeStatement(stmt) catch {
                                    // Log but don't propagate errors from finally block
                                    std.log.err("Error in finally block", .{});
                                };
                            }
                        }
                    }.execute;
                }
            };

            fam_block.setFinallyHandler(finally_handler.create(self, finally_body));
        }

        // Execute fam block
        const result = fam_block.execute(self.error_runtime);
        switch (result) {
            .Ok => {},
            .Error => |error_obj| {
                // Unhandled error in fam block
                const stderr = std.io.getStdErr().writer();
                try error_obj.format(stderr);
                return error.RuntimeError;
            },
        }
    }

    /// Enhanced expression evaluation with shook support
    pub fn evaluateExpression(self: *EnhancedInterpreter, expr: ast.Expression) !interpreter.Value {
        switch (expr) {
            .Yikes => |yikes| return self.evaluateYikesExpression(yikes),
            .Shook => |shook| return self.evaluateShookExpression(shook),
            .Fam => |fam| return self.evaluateFamExpression(fam),
            else => {
                return self.base_interpreter.evaluateExpression(expr) catch |err| {
                    var error_obj = try self.error_runtime.yikes(
                        @errorName(err),
                        .runtime_yikes,
                        .error,
                        @intFromError(err),
                        null
                    );

                    try error_obj.addContext("expression_type", @tagName(expr));
                    
                    return err;
                };
            },
        }
    }

    fn evaluateYikesExpression(self: *EnhancedInterpreter, yikes: ast.YikesExpression) !interpreter.Value {
        // Evaluate message
        const message_value = try self.evaluateExpression(yikes.message.*);
        const message = switch (message_value) {
            .String => |str| str,
            else => "Invalid error message",
        };

        // Evaluate optional code
        const code = if (yikes.code) |code_expr| blk: {
            const code_value = try self.evaluateExpression(code_expr.*);
            break :blk switch (code_value) {
                .Integer => |i| i,
                else => 0,
            };
        } else 0;

        // Create error object
        var error_obj = try self.error_runtime.yikes(
            message,
            .runtime_yikes,
            .error,
            code,
            null
        );

        // Return as error value (this should trigger error propagation)
        try self.error_runtime.propagateError(&error_obj);
        
        return interpreter.Value{ .String = "Error created" };
    }

    fn evaluateShookExpression(self: *EnhancedInterpreter, shook: ast.ShookExpression) !interpreter.Value {
        // Evaluate the expression that might contain an error
        const expr_result = self.evaluateExpression(shook.expression.*) catch |err| {
            // Convert interpreter error to ShookResult
            const error_obj = YikesError.init(
                self.allocator,
                @errorName(err),
                .runtime_yikes,
                .error,
                @intFromError(err)
            ) catch unreachable;
            
            const shook_result = ShookResult.err(error_obj);
            return try self.propagateShookResult(shook_result);
        };

        // If no error, return the value
        return expr_result;
    }

    fn evaluateFamExpression(self: *EnhancedInterpreter, fam: ast.FamExpression) !interpreter.Value {
        // FAM expressions are similar to statements but return values
        // This is a simplified implementation
        _ = fam;
        return interpreter.Value{ .Void = {} };
    }

    fn propagateShookResult(self: *EnhancedInterpreter, result: ShookResult) !interpreter.Value {
        const propagated = try self.error_runtime.shook(result, "shook_expression");
        
        return switch (propagated) {
            .Ok => |value| switch (value) {
                .Integer => |i| interpreter.Value{ .Integer = i },
                .Float => |f| interpreter.Value{ .Float = f },
                .String => |s| interpreter.Value{ .String = s },
                .Boolean => |b| interpreter.Value{ .Boolean = b },
                .Void => interpreter.Value{ .Void = {} },
                .Pointer => interpreter.Value{ .Void = {} },
            },
            .Error => |error_obj| {
                // Log error and return appropriate interpreter error
                const stderr = std.io.getStdErr().writer();
                try error_obj.format(stderr);
                return error.RuntimeError;
            },
        };
    }
};

// Test the enhanced error handling integration
test "enhanced error handling integration" {
    const allocator = std.testing.allocator;
    
    // Test enhanced parser
    const tokens = [_]lexer.Token{
        lexer.Token{ .type = .Yikes, .lexeme = "yikes", .line = 1, .column = 1 },
        lexer.Token{ .type = .String, .lexeme = "\"Test error\"", .line = 1, .column = 7 },
        lexer.Token{ .type = .EOF, .lexeme = "", .line = 1, .column = 20 },
    };
    
    var enhanced_parser = try EnhancedParser.init(allocator, &tokens);
    defer enhanced_parser.deinit();
    
    // Test enhanced interpreter
    var enhanced_interpreter = try EnhancedInterpreter.init(allocator);
    defer enhanced_interpreter.deinit();
    
    // Test error runtime functionality
    const location = YikesError.SourceLocation{
        .file = "test.csd",
        .line = 1,
        .column = 1,
        .function = "test",
    };
    
    var error_obj = try enhanced_interpreter.error_runtime.yikes(
        "Integration test error",
        .runtime_yikes,
        .warning,
        42,
        location
    );
    
    try error_obj.addContext("test_type", "integration");
    
    // Test shook propagation
    const shook_result = ShookResult.err(error_obj);
    const propagated = try enhanced_interpreter.error_runtime.shook(shook_result, "test_context");
    
    try std.testing.expect(propagated.isError());
}
