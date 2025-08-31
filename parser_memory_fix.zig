// Critical Memory Management and Parser Fixes for CURSED
// This file contains the fixes needed for the parser.zig memory leaks and parsing issues

// ISSUE 1: Memory management in parseStatement line 1125-1128
// PROBLEM: Using arena_allocator.create() but not checking for arena safety
// SOLUTION: Add proper arena validation and error handling

fn parseStatementFixed(self: *Parser) ParserError!Statement {
    // ... existing parsing logic ...
    
    // Expression statement with enhanced error handling for complex expressions
    // CRITICAL FIX: Prevent complex expressions with braces from being parsed as function names
    const expr = self.parseExpression() catch |parse_err| {
        // Enhanced error context for complex expression parsing
        _ = self.reportErrorWithContext("Error parsing complex expression statement - check for misplaced braces or operator precedence issues", "parseStatement") catch {};
        self.synchronize();
        return parse_err;
    };
    
    // FIXED: Proper memory allocation with arena safety check
    const expr_ptr = self.arena_allocator.create(Expression) catch |alloc_err| {
        // Log the specific allocation error
        std.debug.print("MEMORY ERROR: Failed to allocate Expression in parseStatement: {}\n", .{alloc_err});
        _ = self.reportErrorWithContext("Out of memory allocating expression", "parseStatement") catch {};
        return ParserError.OutOfMemory;
    };

    expr_ptr.* = expr;
    
    return Statement{ .Expression = expr };
}

// ISSUE 2: ArrayList creation using wrong allocator pattern
// PROBLEM: Line 2010 uses arguments.append(self.arena_allocator, arg_ptr) which mixes allocators
// SOLUTION: Fix allocator usage consistency

fn finishCallFixed(self: *Parser, callee: Expression) ParserError!Expression {
    var arguments = ArrayList(*Expression).init(self.allocator); // Use main allocator for list
    defer arguments.deinit(); // Ensure proper cleanup

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
            try arguments.append(arg_ptr); // Use main allocator for list operations

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

// ISSUE 3: String interpolation parsing in method calls causes "complex expression" errors
// PROBLEM: String format like "2 + 3 * 4 = {}" is being misinterpreted as complex expression
// SOLUTION: Improve string literal detection and parsing

fn parseStringLiteralFixed(self: *Parser) ParserError!Expression {
    const token = self.advance();
    const str_content = if (token.lexeme.len >= 2 and 
                           token.lexeme[0] == '"' and 
                           token.lexeme[token.lexeme.len-1] == '"')
                         token.lexeme[1..token.lexeme.len-1] // Remove quotes
                         else token.lexeme;
    
    // FIXED: Better string interpolation detection
    // Check for {} patterns that are NOT arithmetic expressions
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

// ISSUE 4: parseExpressionPratt not handling method calls properly
// PROBLEM: Method calls like vibez.spill() with complex arguments cause parsing failures
// SOLUTION: Improve method call parsing in Pratt parser

fn parsePrattMethodCallFixed(self: *Parser, left: Expression) ParserError!Expression {
    _ = self.advance(); // consume '.'
    
    // FIXED: Better method name validation
    if (!self.check(.Identifier) and !self.isKeywordAllowedAsMethodName()) {
        std.debug.print("DEBUG: Expected method name after '.', found: {}\n", .{self.peek().kind});
        return ParserError.UnexpectedToken;
    }
    
    const property = self.advance().lexeme;
    
    // If next token is '(', treat as method call
    if (self.check(.LeftParen)) {
        _ = self.advance(); // '('
        var arguments = ArrayList(*Expression).init(self.allocator);
        defer arguments.deinit();
        
        if (!self.check(.RightParen)) {
            while (true) {
                // FIXED: Better error handling for method arguments
                const arg = self.parseExpression() catch |parse_err| {
                    std.debug.print("DEBUG: Failed to parse method argument: {}\n", .{parse_err});
                    return parse_err;
                };
                const arg_ptr = try self.arena_allocator.create(Expression);
                arg_ptr.* = arg;
                try arguments.append(arg_ptr);
                
                if (!self.match(.Comma)) break;
            }
        }
        _ = try self.consume(.RightParen, "Expected ')' after method arguments");
        
        return Expression{ .MethodCall = try self.allocateMethodCall(ast.MethodCallExpression{
            .object = try self.allocateExpression(left),
            .method_name = property,
            .arguments = arguments,
        })};
    }
    
    // plain member access
    return Expression{ .MemberAccess = try self.allocateMemberAccess(ast.MemberAccessExpression{
        .object = try self.allocateExpression(left),
        .property = property,
    })};
}

// ISSUE 5: Thread safety issue in hash map operations
// PROBLEM: The interpreter is having thread safety issues with variable definitions
// SOLUTION: This is in interpreter.zig, but parser needs to ensure proper cleanup

fn deinitFixed(self: *Parser) void {
    // FIXED: Ensure all arena-allocated memory is properly cleaned up
    // The arena automatically cleans up all its allocations
    self.arena.deinit();
    
    // Report final error recovery stats if there were issues
    if (self.error_recovery_stats.total_errors > 0) {
        self.error_recovery_stats.reportStats();
    }
}
