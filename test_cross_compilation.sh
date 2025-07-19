#!/bin/bash
# test_cross_compilation.sh

echo "🔀 Testing cross-platform compilation..."

TARGETS=(
    "aarch64-apple-darwin"
    "aarch64-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "x86_64-unknown-linux-gnu"
    "x86_64-pc-windows-msvc"
    "wasm32-unknown-unknown"
    "wasm32-wasi"
)

for target in "${TARGETS[@]}"; do
    echo "Checking target: $target"
    if rustup target list --installed | grep -q "$target"; then
        if cargo check --target "$target" --quiet; then
            echo "✅ $target: Compilation successful"
        else
            echo "❌ $target: Compilation failed"
        fi
    else
        echo "⚠️  $target: Target not installed"
        echo "   Install with: rustup target add $target"
    fi
done
