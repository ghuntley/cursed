#!/bin/bash
# LLVM Environment Setup Script for CURSED Development

set -e

echo "🔧 Setting up LLVM environment for CURSED development..."

# Detect LLVM-18 installation
if command -v llvm-config-18 >/dev/null 2>&1; then
    LLVM_VERSION=$(llvm-config-18 --version)
    LLVM_LIBDIR=$(llvm-config-18 --libdir)
    LLVM_INCLUDEDIR=$(llvm-config-18 --includedir)
    
    echo "✅ Found LLVM-18: version $LLVM_VERSION"
    echo "✅ Library directory: $LLVM_LIBDIR"
    echo "✅ Include directory: $LLVM_INCLUDEDIR"
    
    # Export environment variables
    export LLVM_SYS_180_PREFIX="$LLVM_INCLUDEDIR/.."
    export LLVM_CONFIG_PATH="$(which llvm-config-18)"
    export PKG_CONFIG_PATH="$LLVM_LIBDIR/pkgconfig:$PKG_CONFIG_PATH"
    export LD_LIBRARY_PATH="$LLVM_LIBDIR:/lib/x86_64-linux-gnu:/usr/lib/x86_64-linux-gnu:$LD_LIBRARY_PATH"
    
    # Verify paths exist
    if [ -d "$LLVM_LIBDIR" ]; then
        echo "✅ LLVM library directory exists and is accessible"
    else
        echo "❌ LLVM library directory not accessible: $LLVM_LIBDIR"
        exit 1
    fi
    
    if [ -d "$LLVM_INCLUDEDIR" ]; then
        echo "✅ LLVM include directory exists and is accessible"
    else
        echo "❌ LLVM include directory not accessible: $LLVM_INCLUDEDIR"
        exit 1
    fi
    
    # Check for key LLVM libraries
    if [ -f "$LLVM_LIBDIR/libLLVM-18.so" ] || [ -f "$LLVM_LIBDIR/libLLVM.so.1" ]; then
        echo "✅ LLVM shared library found"
    else
        echo "⚠️ LLVM shared library not found, checking static libraries..."
        if ls "$LLVM_LIBDIR"/libLLVM*.a >/dev/null 2>&1; then
            echo "✅ LLVM static libraries found"
        else
            echo "❌ No LLVM libraries found in $LLVM_LIBDIR"
            exit 1
        fi
    fi
    
    echo ""
    echo "🎯 LLVM environment ready for CURSED development"
    echo "📝 Use: source setup_llvm_env.sh"
    echo "🏗️  Then: zig build"
    echo ""
    
else
    echo "❌ llvm-config-18 not found. Please install LLVM-18:"
    echo "   Ubuntu/Debian: sudo apt install llvm-18 llvm-18-dev"
    echo "   macOS: brew install llvm@18"
    echo ""
    exit 1
fi

# Check CPU architecture detection
ARCH=$(uname -m)
echo "🖥️  System architecture: $ARCH"

# Warn about CPU detection issues
if [ "$ARCH" = "x86_64" ]; then
    echo "✅ x86_64 architecture detected - CPU override handling enabled"
else
    echo "ℹ️  Non-x86_64 architecture - ensure proper target configuration"
fi

echo ""
echo "🚀 Environment setup complete! You can now run:"
echo "   zig build"
echo ""
