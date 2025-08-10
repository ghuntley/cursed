#!/bin/bash
# Build script for file I/O fuzz targets

set -e

echo "Building file I/O fuzz targets..."

CFLAGS="-fsanitize=fuzzer,address -g -O1"
LDFLAGS="-fsanitize=fuzzer,address"

for target in fuzz_file_*.c; do
    if [ -f "$target" ]; then
        echo "Building $target..."
        clang $CFLAGS -o "${target%.c}" "$target" $LDFLAGS
    fi
done

echo "File I/O fuzz targets built successfully!"
