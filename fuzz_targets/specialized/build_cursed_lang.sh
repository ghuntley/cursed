#!/bin/bash
# Build script for CURSED language fuzz targets

set -e

echo "Building CURSED language fuzz targets..."

# First ensure CURSED compiler is built
if [ ! -f "../zig-out/bin/cursed-zig" ]; then
    echo "Building CURSED compiler first..."
    cd ..
    zig build
    cd fuzz_targets
fi

# Build C targets
CFLAGS="-fsanitize=fuzzer,address -g -O1"
LDFLAGS="-fsanitize=fuzzer,address"

for target in fuzz_cursed_*.c; do
    if [ -f "$target" ]; then
        echo "Building $target..."
        clang $CFLAGS -o "${target%.c}" "$target" $LDFLAGS -I../src-zig
    fi
done

# Build Zig targets
for target in fuzz_cursed_*.zig; do
    if [ -f "$target" ]; then
        echo "Building $target..."
        zig build-exe -fsanitize-c -lc "$target" --main-pkg-path .. -I../src-zig
    fi
done

echo "CURSED language fuzz targets built successfully!"
