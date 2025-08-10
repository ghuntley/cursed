#!/bin/bash
# Build script for memory buffer fuzz targets

set -e

echo "Building memory buffer fuzz targets..."

CFLAGS="-fsanitize=fuzzer,address,undefined -g -O1"
LDFLAGS="-fsanitize=fuzzer,address,undefined"

for target in fuzz_buffer_*.c; do
    if [ -f "$target" ]; then
        echo "Building $target..."
        clang $CFLAGS -o "${target%.c}" "$target" $LDFLAGS
    fi
done

echo "Memory buffer fuzz targets built successfully!"
