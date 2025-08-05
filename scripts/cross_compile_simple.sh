#!/usr/bin/env bash
# Simple cross-compilation test to isolate the issue

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

targets=("x86_64-linux" "aarch64-linux" "x86_64-windows" "wasm32-wasi")

print_status "Testing cross-compilation for ${#targets[@]} targets..."

for target in "${targets[@]}"; do
    print_status "Building target: $target"
    rm -rf zig-out
    
    if zig build -Dtarget="$target" -Doptimize=ReleaseFast >/dev/null 2>&1; then
        print_success "✅ $target successful"
    else
        echo "❌ $target failed"
    fi
done

print_success "All targets processed"
