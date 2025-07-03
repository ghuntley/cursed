#!/bin/bash

# CURSED Compiler Build Script with LibFFI Fix
# This script ensures the correct environment variables are set for building
# the CURSED compiler with proper libffi linking.

# Find the correct nix store paths dynamically
LIBFFI_PATH=$(find /nix/store -maxdepth 1 -name "*libffi*" -type d | head -1)
NCURSES_PATH=$(find /nix/store -maxdepth 1 -name "*ncurses*" -type d | grep -v "man" | grep -v "dev" | head -1)
LIBXML2_PATH=$(find /nix/store -maxdepth 1 -name "*libxml2*" -type d | head -1)
LLVM_PATH=$(dirname $(find /nix/store -name "llc" -type f 2>/dev/null | grep -v kernel | grep -v fhsenv | head -1))

# Add LLVM tools to PATH for compilation
if [ -n "$LLVM_PATH" ]; then
    export PATH="$LLVM_PATH:$PATH"
    echo "🔧 LLVM tools path: $LLVM_PATH"
fi

# Set the correct RUSTFLAGS to avoid libffi linking issues
export RUSTFLAGS="-L ${NCURSES_PATH}/lib -L ${LIBFFI_PATH}/lib -L ${LIBXML2_PATH}/lib -C link-arg=-Wl,-rpath,${NCURSES_PATH}/lib -C link-arg=-Wl,-rpath,${LIBFFI_PATH}/lib -C link-arg=-Wl,-rpath,${LIBXML2_PATH}/lib -C linker=gcc"

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
    echo "  export PATH=\"$LLVM_PATH:\$PATH\" && ./target/x86_64-unknown-linux-gnu/debug/cursed --compile test.csd -o output"
    echo "  export PATH=\"$LLVM_PATH:\$PATH\" && ./target/x86_64-unknown-linux-gnu/debug/cursed --compile test_hello_cursed.csd"
else
    echo ""
    echo "❌ Build failed with exit code: $BUILD_EXIT_CODE"
fi

exit $BUILD_EXIT_CODE
