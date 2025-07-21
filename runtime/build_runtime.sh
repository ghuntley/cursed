#!/bin/bash

# CURSED Runtime Library Build Script
# This script builds the CURSED runtime library for various targets

set -euo pipefail

# Default values
TARGET=""
PROFILE="debug"
VERBOSE=false
CLEAN=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --target)
            TARGET="$2"
            shift 2
            ;;
        --release)
            PROFILE="release"
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --clean)
            CLEAN=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo "  --target TARGET    Build for specific target"
            echo "  --release          Build in release mode"
            echo "  --verbose          Verbose output"
            echo "  --clean            Clean before building"
            echo "  -h, --help         Show this help"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Set default target if not provided
if [[ -z "$TARGET" ]]; then
    TARGET=$(rustc -vV | grep host | cut -d' ' -f2)
fi

echo "Building CURSED runtime for target: $TARGET"
echo "Profile: $PROFILE"

# Clean if requested
if [[ "$CLEAN" == "true" ]]; then
    echo "Cleaning..."
    cargo clean
fi

# Build arguments
BUILD_ARGS=("--lib")
if [[ "$TARGET" != "" ]]; then
    BUILD_ARGS+=("--target" "$TARGET")
fi
if [[ "$PROFILE" == "release" ]]; then
    BUILD_ARGS+=("--release")
fi
if [[ "$VERBOSE" == "true" ]]; then
    BUILD_ARGS+=("--verbose")
fi

# Build the runtime
echo "Building runtime with: cargo build ${BUILD_ARGS[*]}"
cargo build "${BUILD_ARGS[@]}"

echo "Runtime build completed successfully for $TARGET"
