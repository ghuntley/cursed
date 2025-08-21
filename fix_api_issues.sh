#!/bin/bash

# Fix API compatibility issues for full parser integration

echo "🔧 Fixing ArrayList.init() calls..."

# Replace ArrayList.init(allocator) with ArrayList{}
find src-zig/ -name "*.zig" -type f -exec sed -i 's/ArrayList(\([^)]*\))\.init(\([^)]*\))/ArrayList(\1){}/g' {} \;

echo "🔧 Fixing append() calls that need allocator parameter..."

# Fix specific append calls in parser.zig
sed -i 's/try func\.type_parameters\.append(type_param)/try func.type_parameters.append(self.allocator, type_param)/g' src-zig/parser.zig
sed -i 's/try param_types\.append(param_type)/try param_types.append(self.allocator, param_type)/g' src-zig/parser.zig
sed -i 's/try try_body\.append(self\.allocator, try self\.statementToAnyopaque(stmt_ptr))/try try_body.append(self.allocator, try self.statementToAnyopaque(stmt_ptr))/g' src-zig/parser.zig
sed -i 's/try arguments\.append(self\.allocator, arg_ptr)/try arguments.append(self.allocator, arg_ptr)/g' src-zig/parser.zig
sed -i 's/try elements\.append(elem)/try elements.append(self.allocator, elem)/g' src-zig/parser.zig
sed -i 's/try interpolation\.parts\.append(ast\.InterpolationPart{/try interpolation.parts.append(self.allocator, ast.InterpolationPart{/g' src-zig/parser.zig

echo "🔧 Fixing enum literal issues..."

# Fix enum literal variable declarations
sed -i 's/var command_parts = \.empty;/const command_parts = std.ArrayListUnmanaged([]const u8){};/g' src-zig/cross_compilation.zig
sed -i 's/var param_types = \.empty;/var param_types = ArrayList(ast.Type){};/g' src-zig/parser.zig

echo "🔧 Fixing other API issues..."

# Fix unused parameter warnings
sed -i 's/pub fn deinit(self: \*Statement, allocator: Allocator) void {/pub fn deinit(self: *Statement, _: Allocator) void {/g' src-zig/ast_advanced.zig
sed -i 's/pub fn release(self: \*TypedObject, allocator: Allocator) CursedError!void {/pub fn release(self: *TypedObject, _: Allocator) CursedError!void {/g' src-zig/type_system_runtime.zig

# Fix stdout.any() issue
sed -i 's/const writer = stdout\.any();/const writer = stdout.writer();/g' src-zig/lsp_server.zig

# Fix 'export' keyword issue (reserved word)
sed -i 's/for (doc_data\.exports\.items) |export| {/for (doc_data.exports.items) |export_item| {/g' src-zig/advanced_lsp_server.zig
sed -i 's/export\./export_item./g' src-zig/advanced_lsp_server.zig

echo "✅ API fixes applied"
