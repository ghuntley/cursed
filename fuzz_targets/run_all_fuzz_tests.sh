#!/bin/bash
# CURSED Automated Fuzz Testing Harness

set -e

echo "🚀 Starting CURSED fuzz testing..."

# Compile and run libFuzzer targets
echo "📦 Building libFuzzer targets..."
for fuzz_target in fuzz_*.c; do
    if [ -f "$fuzz_target" ]; then
        echo "Building $fuzz_target..."
        clang -fsanitize=fuzzer,address -g "$fuzz_target" -o "${fuzz_target%.c}"
        echo "Running $fuzz_target for 60 seconds..."
        timeout 60 "./${fuzz_target%.c}" || true
    fi
done

# Run cargo-fuzz targets
if [ -d "cargo_fuzz" ]; then
    echo "🦀 Running Rust fuzz targets..."
    cd cargo_fuzz
    cargo install cargo-fuzz
    for fuzz_target in src/fuzz_*.rs; do
        if [ -f "$fuzz_target" ]; then
            target_name=$(basename "$fuzz_target" .rs)
            echo "Running $target_name for 60 seconds..."
            timeout 60 cargo fuzz run "$target_name" || true
        fi
    done
    cd ..
fi

# Run Zig fuzz targets
echo "⚡ Building Zig fuzz targets..."
for fuzz_target in fuzz_*.zig; do
    if [ -f "$fuzz_target" ]; then
        echo "Building $fuzz_target..."
        zig build-exe -fsanitize-c -lc "$fuzz_target"
        echo "Running basic test for $fuzz_target..."
        "./${fuzz_target%.zig}" || true
    fi
done

echo "✅ Fuzz testing complete!"
