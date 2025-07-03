#!/bin/bash
# CURSED Compiler Build Script - LIBFFI LINKER FIX
# This script permanently resolves the libffi linker issue by overriding mold linker with gcc

set -e

echo "🔧 Building CURSED compiler with libffi linker fix..."

# Source the environment file to get the proper RUSTFLAGS
source .envrc

# Build the CURSED compiler
cargo build --release

echo "✅ CURSED compiler built successfully!"
echo "🎯 Libffi linker issue permanently resolved"
echo "📦 Binary available at: target/release/cursed"
