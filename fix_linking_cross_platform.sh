#!/bin/bash
# Cross-platform linking fix for CURSED codebase
# This script handles linker issues on both Linux and macOS

# Detect operating system
OS="$(uname)"

case $OS in
    Linux)
        echo "🐧 Detected Linux - applying Nix environment linking fixes"
        
        # Set up proper library paths for the BFD linker to find required libraries (Linux/Nix specific)
        export LIBRARY_PATH="/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib:/nix/store/l5g2v1jgfyf3j0jp9iv5b79fi8yrwzpp-zlib-1.3.1/lib:/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib:/nix/store/hd6llsw2dkiazk9d2ywv13cc6alhflly-libxml2-2.13.5/lib:/nix/store/dsqzw96w4sxsp4q9yvkfl2yh701mpwgi-sqlite-3.46.1/lib"
        
        # Override RUSTFLAGS to force BFD linker instead of mold
        export RUSTFLAGS="-C linker=gcc -C link-arg=-fuse-ld=bfd"
        
        echo "   LIBRARY_PATH: $LIBRARY_PATH"
        echo "   RUSTFLAGS: $RUSTFLAGS"
        ;;
        
    Darwin)
        echo "🍎 Detected macOS - applying macOS-specific linking configuration"
        
        # macOS-specific configurations
        if command -v brew >/dev/null 2>&1; then
            # Use Homebrew paths if available
            BREW_PREFIX=$(brew --prefix)
            export LIBRARY_PATH="$BREW_PREFIX/lib:${LIBRARY_PATH:-}"
            export CPATH="$BREW_PREFIX/include:${CPATH:-}"
            echo "   Using Homebrew prefix: $BREW_PREFIX"
        fi
        
        # Set up LLVM paths (common macOS locations)
        if [ -d "/opt/homebrew/opt/llvm/lib" ]; then
            export LIBRARY_PATH="/opt/homebrew/opt/llvm/lib:${LIBRARY_PATH:-}"
            export CPATH="/opt/homebrew/opt/llvm/include:${CPATH:-}"
            echo "   Added Homebrew LLVM paths"
        elif [ -d "/usr/local/opt/llvm/lib" ]; then
            export LIBRARY_PATH="/usr/local/opt/llvm/lib:${LIBRARY_PATH:-}"
            export CPATH="/usr/local/opt/llvm/include:${CPATH:-}"
            echo "   Added Intel Mac LLVM paths"
        fi
        
        # macOS specific Rust flags
        export RUSTFLAGS="-C link-arg=-Wl,-rpath,@loader_path ${RUSTFLAGS:-}"
        
        echo "   LIBRARY_PATH: ${LIBRARY_PATH:-}"
        echo "   CPATH: ${CPATH:-}"
        echo "   RUSTFLAGS: $RUSTFLAGS"
        ;;
        
    *)
        echo "⚠️  Unknown operating system: $OS"
        echo "   Proceeding without platform-specific linking fixes"
        ;;
esac

echo ""

# Execute the command passed as arguments
if [ $# -eq 0 ]; then
    echo "Usage: $0 <command> [args...]"
    echo "Examples:"
    echo "  $0 cargo build"
    echo "  $0 cargo test"
    echo "  $0 cargo test --test simple_core_test"
    exit 1
fi

exec "$@"
