#!/bin/bash

# Fix Zig 0.14.0 API compatibility issues

echo "Fixing enhanced_parser.zig..."

# Fix unused variables
sed -i 's/const slay_token = try self.consume(.Slay, "Expected '\''slay'\'' keyword");/_ = try self.consume(.Slay, "Expected '\''slay'\'' keyword");/' src-zig/enhanced_parser.zig

# Fix unused variables in parse statements
sed -i 's/const value = try self.parseAssignment();/_ = try self.parseAssignment();/' src-zig/enhanced_parser.zig

# Fix mutable variables that should be const
sed -i 's/var expr = try self.parseOr();/const expr = try self.parseOr();/' src-zig/enhanced_parser.zig
sed -i 's/var array_type = ast.Type{/const array_type = ast.Type{/' src-zig/enhanced_parser.zig
sed -i 's/var binary_expr = Expression{/const binary_expr = Expression{/g' src-zig/enhanced_parser.zig
sed -i 's/var member_expr = Expression{/const member_expr = Expression{/' src-zig/enhanced_parser.zig
sed -i 's/var index_expr = Expression{/const index_expr = Expression{/' src-zig/enhanced_parser.zig
sed -i 's/var call_expr = Expression{/const call_expr = Expression{/' src-zig/enhanced_parser.zig

# Fix unused captures in parseInt and parseFloat
sed -i 's/std.fmt.parseInt(i64, self.previous().lexeme, 10) catch |err| {/std.fmt.parseInt(i64, self.previous().lexeme, 10) catch {/' src-zig/enhanced_parser.zig
sed -i 's/std.fmt.parseFloat(f64, self.previous().lexeme) catch |err| {/std.fmt.parseFloat(f64, self.previous().lexeme) catch {/' src-zig/enhanced_parser.zig

# Fix unused right variable
sed -i 's/const right = try self.parseUnary();/_ = try self.parseUnary();/' src-zig/enhanced_parser.zig

# Fix unused function parameters by replacing with underscore
sed -i 's/fn isTypeKeyword(self: \*Parser, kind: TokenKind) bool {/fn isTypeKeyword(_: *Parser, kind: TokenKind) bool {/' src-zig/enhanced_parser.zig
sed -i 's/fn parseIfStatement(self: \*Parser) !ast.IfStatement {/fn parseIfStatement(_: *Parser) !ast.IfStatement {/' src-zig/enhanced_parser.zig
sed -i 's/fn parseWhileStatement(self: \*Parser) !ast.WhileStatement {/fn parseWhileStatement(_: *Parser) !ast.WhileStatement {/' src-zig/enhanced_parser.zig
sed -i 's/fn parseForStatement(self: \*Parser) !ast.ForStatement {/fn parseForStatement(_: *Parser) !ast.ForStatement {/' src-zig/enhanced_parser.zig
sed -i 's/fn parseBreakStatement(self: \*Parser) !Statement {/fn parseBreakStatement(_: *Parser) !Statement {/' src-zig/enhanced_parser.zig
sed -i 's/fn parseContinueStatement(self: \*Parser) !Statement {/fn parseContinueStatement(_: *Parser) !Statement {/' src-zig/enhanced_parser.zig
sed -i 's/fn parseStructStatement(self: \*Parser) !ast.StructStatement {/fn parseStructStatement(_: *Parser) !ast.StructStatement {/' src-zig/enhanced_parser.zig
sed -i 's/fn parseInterfaceStatement(self: \*Parser) !ast.InterfaceStatement {/fn parseInterfaceStatement(_: *Parser) !ast.InterfaceStatement {/' src-zig/enhanced_parser.zig
sed -i 's/fn parseImplementationStatement(self: \*Parser) !ast.ImplementationStatement {/fn parseImplementationStatement(_: *Parser) !ast.ImplementationStatement {/' src-zig/enhanced_parser.zig

echo "Zig API compatibility fixes applied!"
