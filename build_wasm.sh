#!/bin/bash

# Build script for WASM compilation
# Tests WebAssembly compatibility with proper feature flags

set -e

echo "Building CURSED for WebAssembly..."

# Ensure wasm32 target is installed
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi

echo "Building for wasm32-unknown-unknown..."
cargo build --target wasm32-unknown-unknown --no-default-features --features crypto-rustcrypto,wasm

echo "Building for wasm32-wasi..."  
cargo build --target wasm32-wasi --no-default-features --features crypto-rustcrypto,wasm

echo "WASM builds completed successfully!"

# Check file sizes
echo -e "\nBuild output sizes:"
ls -lh target/wasm32-unknown-unknown/debug/cursed.wasm 2>/dev/null || echo "wasm32-unknown-unknown binary not found"
ls -lh target/wasm32-wasi/debug/cursed.wasm 2>/dev/null || echo "wasm32-wasi binary not found"

echo -e "\nWASM compatibility test completed!"
