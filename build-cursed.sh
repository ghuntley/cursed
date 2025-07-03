#!/bin/bash
# CURSED Compiler Build Script
# Permanent solution for libffi linker issues

set -e

echo "🔧 Setting up CURSED build environment..."

# Set the proper RUSTFLAGS for linking with all required libraries
export RUSTFLAGS="-L /nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib -L /nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib -L /nix/store/0z4hrksbdrwv9xb8ycjk3rq9ppmw0350-libxml2-2.13.5/lib -C link-arg=-Wl,-rpath,/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib -C link-arg=-Wl,-rpath,/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib -C link-arg=-Wl,-rpath,/nix/store/0z4hrksbdrwv9xb8ycjk3rq9ppmw0350-libxml2-2.13.5/lib -C linker=gcc"

echo "📦 Building CURSED compiler with fixed linker settings..."
cargo build --release

echo "✅ CURSED compiler built successfully!"
echo "🚀 Running test program..."
./target/x86_64-unknown-linux-gnu/release/cursed test_hello_cursed.csd

echo "🎉 CURSED compiler is working correctly!"
