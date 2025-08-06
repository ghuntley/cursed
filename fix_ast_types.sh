#!/bin/bash

# Fix AST type casting issues in parser.zig

echo "Fixing AST expression and statement casting issues..."

# Fix remaining expression assignments that need to be casted to *anyopaque
sed -i 's/\.condition = condition/const condition_ptr = try self.allocator.create(Expression); condition_ptr.* = condition; .condition = @ptrCast(condition_ptr)/g' src-zig/parser.zig

# Fix remaining ignored token returns
sed -i 's/self\.advance();/_ = self.advance();/g' src-zig/parser.zig

# Fix BinaryExpression assignments in expressions
sed -i 's/return Expression{ \.Binary = binary_expr }/const binary_ptr = try self.allocator.create(ast.BinaryExpression); binary_ptr.* = binary_expr; return Expression{ .Binary = .{ .left = @ptrCast(binary_ptr.left), .operator = binary_ptr.operator, .right = @ptrCast(binary_ptr.right) } };/g' src-zig/parser.zig

echo "AST type fixes applied"
