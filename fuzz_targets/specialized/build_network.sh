#!/bin/bash
# Build script for network fuzz targets

set -e

echo "Building network fuzz targets..."

CFLAGS="-fsanitize=fuzzer,address -g -O1"
LDFLAGS="-fsanitize=fuzzer,address"

for target in fuzz_network_*.c; do
    if [ -f "$target" ]; then
        echo "Building $target..."
        clang $CFLAGS -o "${target%.c}" "$target" $LDFLAGS
    fi
done

echo "Network fuzz targets built successfully!"
