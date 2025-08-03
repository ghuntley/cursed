#!/bin/bash

# Fix all the warnings in ast.zig by removing pointless discards
sed -i 's/pub fn print(self: Expression, indent: usize, ast: \*AST)/pub fn print(self: Expression, _: usize, ast: *AST)/' src-zig/ast.zig
sed -i '/^\s*_ = allocator;$/d' src-zig/ast.zig
sed -i '/^\s*_ = ast;$/d' src-zig/ast.zig  
sed -i '/^\s*_ = self;$/d' src-zig/ast.zig

echo "Fixed ast.zig warnings"
