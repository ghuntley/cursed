#!/bin/bash
# Fix linking issues in CURSED codebase by setting proper environment variables
# This script overrides mold linker issues in the Nix environment

# Set up proper library paths for the BFD linker to find required libraries
export LIBRARY_PATH="/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib:/nix/store/l5g2v1jgfyf3j0jp9iv5b79fi8yrwzpp-zlib-1.3.1/lib:/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib:/nix/store/hd6llsw2dkiazk9d2ywv13cc6alhflly-libxml2-2.13.5/lib"

# Override RUSTFLAGS to force BFD linker instead of mold
export RUSTFLAGS="-C linker=gcc -C link-arg=-fuse-ld=bfd"

# Print environment info
echo "🔧 Fixed linking environment:"
echo "   LIBRARY_PATH: $LIBRARY_PATH"
echo "   RUSTFLAGS: $RUSTFLAGS"
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
