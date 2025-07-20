#!/bin/bash

# Test cross-compilation for all fucking targets

set -e

echo "===== Cross-compilation test for ALL targets ====="

# Clean first
echo "Cleaning..."
cargo clean

# Test 1: Native macOS ARM64 (should work)
echo "===== Testing native macOS ARM64 ====="
cargo build --target aarch64-apple-darwin --release --verbose 2>&1 | head -20

# Test 2: macOS x86_64
echo "===== Testing macOS x86_64 ====="  
cargo build --target x86_64-apple-darwin --release

# Test 3: WebAssembly (should work)
echo "===== Testing WebAssembly ====="
cargo build --target wasm32-unknown-unknown --release

# Test 4: Linux x86_64 (cross-compilation)
echo "===== Testing Linux x86_64 ====="
cargo build --target x86_64-unknown-linux-gnu --release

# Test 5: Linux ARM64 (cross-compilation)
echo "===== Testing Linux ARM64 ====="
cargo build --target aarch64-unknown-linux-gnu --release

# Test 6: Windows x86_64 (cross-compilation)
echo "===== Testing Windows x86_64 ====="
cargo build --target x86_64-pc-windows-gnu --release

# Test 7: Windows i686 (cross-compilation)
echo "===== Testing Windows i686 ====="
cargo build --target i686-pc-windows-gnu --release

echo "===== All cross-compilation tests completed! ====="

# List all the binaries we built
echo ""
echo "Built binaries:"
find target -name "cursed*" -executable -type f | sort
