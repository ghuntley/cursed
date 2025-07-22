#!/bin/bash

# Test cross-compilation fixes for CURSED
# Tests major cross-compilation targets to verify fixes

set -e

echo "🚀 Testing cross-compilation fixes for CURSED"
echo "=============================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

targets=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu" 
    "x86_64-pc-windows-gnu"
    "wasm32-unknown-unknown"
    "x86_64-apple-darwin"
)

results=()

for target in "${targets[@]}"; do
    echo ""
    echo -e "${YELLOW}Testing ${target}...${NC}"
    
    if timeout 300 cargo check --target "$target" >/dev/null 2>&1; then
        echo -e "${GREEN}✅ ${target} - SUCCESS${NC}"
        results+=("SUCCESS")
    else
        echo -e "${RED}❌ ${target} - FAILED${NC}"
        results+=("FAILED")
    fi
done

echo ""
echo "🏁 Cross-compilation Test Results:"
echo "=================================="

success_count=0
total_count=${#targets[@]}

for i in "${!targets[@]}"; do
    target="${targets[$i]}"
    result="${results[$i]}"
    
    if [ "$result" = "SUCCESS" ]; then
        echo -e "${GREEN}✅ ${target}${NC}"
        ((success_count++))
    else
        echo -e "${RED}❌ ${target}${NC}"
    fi
done

echo ""
echo "📊 Summary: ${success_count}/${total_count} targets working"

if [ $success_count -ge 3 ]; then
    echo -e "${GREEN}🎉 Cross-compilation fixes successful! ${success_count}/5 targets working (goal: 3+)${NC}"
    exit 0
else
    echo -e "${YELLOW}⚠️  More fixes needed. Only ${success_count}/5 targets working (goal: 3+)${NC}"
    exit 1
fi
