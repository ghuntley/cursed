#!/bin/bash
# Build script for parser fuzz targets

set -e

echo "Building parser fuzz targets..."

# Compile with sanitizers
CFLAGS="-fsanitize=fuzzer,address,undefined -g -O1 -fno-omit-frame-pointer"
LDFLAGS="-fsanitize=fuzzer,address,undefined"

# Build each parser target
for target in fuzz_parser_*.c; do
    if [ -f "$target" ]; then
        echo "Building $target..."
        clang $CFLAGS -o "${target%.c}" "$target" $LDFLAGS
    fi
done

# Build Zig targets
for target in fuzz_parser_*.zig; do
    if [ -f "$target" ]; then
        echo "Building $target..."
        zig build-exe -fsanitize-c -lc "$target"
    fi
done

echo "Parser fuzz targets built successfully!"
