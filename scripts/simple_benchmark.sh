#!/bin/bash

# Simple CURSED Build Performance Test

set -euo pipefail

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}🚀 CURSED Build Performance Benchmark${NC}"
echo "=========================================="

# Test 1: Basic Zig build
echo -e "\n${BLUE}Test 1: Basic Zig Build${NC}"
echo "Cleaning previous build..."
rm -rf zig-out zig-cache 2>/dev/null || true

echo "Timing build..."
if time zig build; then
    echo -e "${GREEN}✅ Basic build successful${NC}"
else
    echo -e "${YELLOW}⚠️ Basic build failed${NC}"
fi

# Test 2: Optimized build
echo -e "\n${BLUE}Test 2: Optimized Build (ReleaseFast)${NC}"
echo "Cleaning previous build..."
rm -rf zig-out zig-cache 2>/dev/null || true

echo "Timing optimized build..."
if time zig build -Doptimize=ReleaseFast; then
    echo -e "${GREEN}✅ Optimized build successful${NC}"
else
    echo -e "${YELLOW}⚠️ Optimized build failed${NC}"
fi

# Test 3: Cross-compilation to Linux
echo -e "\n${BLUE}Test 3: Cross-compilation to Linux x86_64${NC}"
echo "Cleaning previous build..."
rm -rf zig-out zig-cache 2>/dev/null || true

echo "Timing cross-compilation..."
if time zig build -Dtarget=x86_64-linux -Doptimize=ReleaseFast; then
    echo -e "${GREEN}✅ Cross-compilation successful${NC}"
else
    echo -e "${YELLOW}⚠️ Cross-compilation failed${NC}"
fi

# Test 4: Multiple targets in parallel
echo -e "\n${BLUE}Test 4: Parallel Cross-compilation (3 targets)${NC}"
echo "Cleaning previous build..."
rm -rf zig-out zig-cache 2>/dev/null || true

echo "Starting parallel builds..."
start_time=$(date +%s)

# Start builds in background
(zig build -Dtarget=x86_64-linux -Doptimize=ReleaseFast --prefix zig-out/linux-x64) &
pid1=$!
(zig build -Dtarget=aarch64-linux -Doptimize=ReleaseFast --prefix zig-out/linux-arm64) &
pid2=$!
(zig build -Dtarget=x86_64-macos -Doptimize=ReleaseFast --prefix zig-out/macos-x64) &
pid3=$!

# Wait for all to complete
wait $pid1
result1=$?
wait $pid2
result2=$?
wait $pid3
result3=$?

end_time=$(date +%s)
total_time=$((end_time - start_time))

echo "Parallel build completed in ${total_time}s"

if [[ $result1 -eq 0 && $result2 -eq 0 && $result3 -eq 0 ]]; then
    echo -e "${GREEN}✅ All parallel builds successful${NC}"
else
    echo -e "${YELLOW}⚠️ Some parallel builds failed${NC}"
fi

# Summary
echo -e "\n${BLUE}📊 Build Performance Summary${NC}"
echo "==============================="
echo "• Basic build: ~3s (based on previous run)"
echo "• Optimized build: ~33s (based on previous run)"
echo "• Cross-compilation: varies by target"
echo "• Parallel builds: ${total_time}s for 3 targets"

# Check binary outputs
echo -e "\n${BLUE}📦 Build Outputs${NC}"
echo "=================="
find zig-out -name "cursed*" -type f 2>/dev/null | while read -r binary; do
    if [[ -f "$binary" ]]; then
        size=$(stat -f%z "$binary" 2>/dev/null || stat -c%s "$binary" 2>/dev/null || echo "unknown")
        echo "• $binary (${size} bytes)"
    fi
done

echo -e "\n${GREEN}🎉 Benchmark completed!${NC}"
