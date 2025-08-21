#!/bin/bash

# Fix specific lines in ast.zig for unused allocator parameters
echo "Fixing ast.zig unused allocator parameters..."

sed -i '281s/allocator: Allocator/_: Allocator/' src-zig/ast.zig
sed -i '313s/allocator: Allocator/_: Allocator/' src-zig/ast.zig  
sed -i '353s/allocator: Allocator/_: Allocator/' src-zig/ast.zig
sed -i '372s/allocator: Allocator/_: Allocator/' src-zig/ast.zig
sed -i '389s/allocator: Allocator/_: Allocator/' src-zig/ast.zig
sed -i '421s/allocator: Allocator/_: Allocator/' src-zig/ast.zig
sed -i '734s/allocator: Allocator/_: Allocator/' src-zig/ast.zig
sed -i '762s/allocator: Allocator/_: Allocator/' src-zig/ast.zig
sed -i '790s/allocator: Allocator/_: Allocator/' src-zig/ast.zig
sed -i '804s/allocator: Allocator/_: Allocator/' src-zig/ast.zig

echo "Done!"
