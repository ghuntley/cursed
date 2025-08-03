#!/bin/bash

# Fix all print statements in main_unified_concurrency.zig

sed -i 's/print("\([^"]*\)")/print("\1", .{})/g' /home/ghuntley/code/cursed/src-zig/main_unified_concurrency.zig

echo "Fixed all print statements"
