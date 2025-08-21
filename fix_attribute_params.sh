#!/bin/bash

# Fix attribute_system.zig unused allocator parameters  
echo "Fixing attribute_system.zig unused parameters..."

sed -i '102s/allocator: Allocator/_: Allocator/' src-zig/attribute_system.zig
sed -i '111s/allocator: Allocator/_: Allocator/' src-zig/attribute_system.zig
sed -i '119s/allocator: Allocator/_: Allocator/' src-zig/attribute_system.zig
sed -i '186s/allocator: Allocator/_: Allocator/' src-zig/attribute_system.zig
sed -i '224s/allocator: Allocator/_: Allocator/' src-zig/attribute_system.zig

echo "Done!"
