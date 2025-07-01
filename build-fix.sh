#!/bin/bash
# Build script for CURSED compiler that handles libffi linking issues

set -e

echo "🔧 CURSED Compiler Build Script"
echo "==============================="

# Unset RUSTFLAGS to prevent mold linker override
if [ ! -z "${RUSTFLAGS}" ]; then
    echo "⚠️  Unsetting RUSTFLAGS environment variable to prevent linker conflicts"
    unset RUSTFLAGS
fi

# Verify libffi library exists
LIBFFI_PATH="/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib/libffi.so"
if [ ! -f "${LIBFFI_PATH}" ]; then
    echo "❌ LibFFI library not found at expected path: ${LIBFFI_PATH}"
    echo "   Please check your NixOS environment setup"
    exit 1
fi

echo "✅ LibFFI library verified"

# Build the library first
echo "📚 Building library..."
cargo check --lib

# Build main binaries
echo "🏗️  Building main binary..."
cargo build --bin cursed

echo "🏗️  Building REPL..."
cargo build --bin cursed-repl

echo "🏗️  Building test runner..."
cargo build --bin cursed-test

# Verify linking
echo "🔍 Verifying libffi linking..."
if ldd ./target/x86_64-unknown-linux-gnu/debug/cursed | grep -q "libffi.so"; then
    echo "✅ LibFFI successfully linked"
    ldd ./target/x86_64-unknown-linux-gnu/debug/cursed | grep libffi
else
    echo "❌ LibFFI linking verification failed"
    exit 1
fi

# Test basic functionality
echo "🧪 Testing basic functionality..."
if ./target/x86_64-unknown-linux-gnu/debug/cursed --help > /dev/null; then
    echo "✅ Main binary works correctly"
else
    echo "❌ Main binary test failed"
    exit 1
fi

echo ""
echo "🎉 Build completed successfully!"
echo "   Main binary: ./target/x86_64-unknown-linux-gnu/debug/cursed"
echo "   REPL:        ./target/x86_64-unknown-linux-gnu/debug/cursed-repl"
echo "   Test runner: ./target/x86_64-unknown-linux-gnu/debug/cursed-test"
echo ""
echo "💡 To avoid future linking issues, always run:"
echo "   unset RUSTFLAGS && cargo build"
