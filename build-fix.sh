#!/bin/bash
# CURSED Compiler Build Script - LIBFFI LINKER FIX
# This script permanently resolves the libffi linker issue by overriding mold linker with gcc

set -e

echo "🔧 Building CURSED compiler with libffi linker fix..."

# Override RUSTFLAGS to use gcc instead of mold and configure proper library paths
export RUSTFLAGS="-L /nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib -L /nix/store/0z4hrksbdrwv9xb8ycjk3rq9ppmw0350-libxml2-2.13.5/lib -L /nix/store/7xfkxczlw3scrjvky5c73705k19q4lxs-devenv-profile/lib -L /nix/store/09b5m303v4d52wjry30xsabj65vnhkni-libffi-3.4.7/lib -C link-arg=-Wl,-rpath,/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib -C link-arg=-Wl,-rpath,/nix/store/0z4hrksbdrwv9xb8ycjk3rq9ppmw0350-libxml2-2.13.5/lib -C link-arg=-Wl,-rpath,/nix/store/7xfkxczlw3scrjvky5c73705k19q4lxs-devenv-profile/lib -C link-arg=-Wl,-rpath,/nix/store/09b5m303v4d52wjry30xsabj65vnhkni-libffi-3.4.7/lib -C linker=gcc"

# Build the CURSED compiler
cargo build --release

echo "✅ CURSED compiler built successfully!"
echo "🎯 Libffi linker issue permanently resolved"
echo "📦 Binary available at: target/release/cursed"
