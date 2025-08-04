#!/bin/bash

# Fix log statements in all tools
for file in src-zig/tools/simple_*.zig; do
    if [ -f "$file" ]; then
        echo "Fixing log statements in $file"
        # Replace std.log.err("message"); with std.log.err("message", .{});
        sed -i 's/std\.log\.err("\([^"]*\)");/std.log.err("\1", .{});/g' "$file"
        sed -i 's/std\.log\.info("\([^"]*\)");/std.log.info("\1", .{});/g' "$file"
        sed -i 's/std\.log\.warn("\([^"]*\)");/std.log.warn("\1", .{});/g' "$file"
    fi
done

echo "Log statements fixed"
