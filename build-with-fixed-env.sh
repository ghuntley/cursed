#!/bin/bash

# CURSED Compiler Build Script with LibFFI Fix
# This script ensures the correct environment variables are set for building
# the CURSED compiler with proper libffi linking.

# Set the correct RUSTFLAGS to avoid libffi linking issues
export RUSTFLAGS="-L /nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib -L /nix/store/0z4hrkxczlw3scrjvky5c73705k19q4lxs-devenv-profile/lib -L /nix/store/09b5m303v4d52wjry30xsabj65vnhkni-libffi-3.4.7/lib -L /nix/store/0z4hrksbdrwv9xb8ycjk3rq9ppmw0350-libxml2-2.13.5/lib -C link-arg=-Wl,-rpath,/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib -C link-arg=-Wl,-rpath,/nix/store/7xfkxczlw3scrjvky5c73705k19q4lxs-devenv-profile/lib -C link-arg=-Wl,-rpath,/nix/store/09b5m303v4d52wjry30xsabj65vnhkni-libffi-3.4.7/lib -C link-arg=-Wl,-rpath,/nix/store/0z4hrksbdrwv9xb8ycjk3rq9ppmw0350-libxml2-2.13.5/lib -C linker=gcc"

echo "🔧 Building CURSED compiler with fixed linker environment..."
echo "📦 RUSTFLAGS: $RUSTFLAGS"
echo ""

# Run the cargo build command
cargo build "$@"

BUILD_EXIT_CODE=$?

if [ $BUILD_EXIT_CODE -eq 0 ]; then
    echo ""
    echo "✅ Build successful! CURSED compiler is ready."
    echo "📍 Binary location: target/x86_64-unknown-linux-gnu/debug/cursed"
    echo ""
    echo "🚀 Usage examples:"
    echo "  ./target/x86_64-unknown-linux-gnu/debug/cursed test_hello_cursed.csd"
    echo "  ./target/x86_64-unknown-linux-gnu/debug/cursed --compile test.csd -o output"
else
    echo ""
    echo "❌ Build failed with exit code: $BUILD_EXIT_CODE"
fi

exit $BUILD_EXIT_CODE
