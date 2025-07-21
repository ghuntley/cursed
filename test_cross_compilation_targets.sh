#!/bin/bash

# CURSED Cross-Compilation Target Status Report Script
# ===================================================

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Create results directory
mkdir -p cross_compilation_results

echo -e "${BLUE}🎯 CURSED Cross-Compilation Infrastructure Test${NC}"
echo -e "${BLUE}===============================================${NC}"
echo

# Target platforms
declare -A TARGETS=(
    ["x86_64-apple-darwin"]="macOS Intel (x86_64)"
    ["x86_64-unknown-linux-gnu"]="Linux x86_64"
    ["aarch64-unknown-linux-gnu"]="Linux ARM64"
    ["x86_64-pc-windows-gnu"]="Windows x86_64"
    ["wasm32-unknown-unknown"]="WebAssembly"
)

# Test results
declare -A RESULTS=()
declare -A ERRORS=()
declare -A DURATION=()

# Function to test individual target
test_target() {
    local target=$1
    local description=$2
    local log_file="cross_compilation_results/${target}.log"
    
    echo -n -e "Testing ${YELLOW}${target}${NC} (${description})... "
    
    local start_time=$(date +%s)
    
    # Attempt cargo check for the target
    if timeout 120s cargo check --target "$target" --quiet > "$log_file" 2>&1; then
        local end_time=$(date +%s)
        DURATION[$target]=$((end_time - start_time))
        RESULTS[$target]="SUCCESS"
        echo -e "${GREEN}✅ PASSED${NC} (${DURATION[$target]}s)"
    else
        local end_time=$(date +%s)
        DURATION[$target]=$((end_time - start_time))
        RESULTS[$target]="FAILED"
        # Extract key error messages
        ERRORS[$target]=$(tail -20 "$log_file" | grep -E "(error|Error|ERROR)" | head -3 | tr '\n' '; ')
        echo -e "${RED}❌ FAILED${NC} (${DURATION[$target]}s)"
    fi
}

# Test each target
for target in "${!TARGETS[@]}"; do
    test_target "$target" "${TARGETS[$target]}"
done

echo
echo -e "${BLUE}📊 Cross-Compilation Results Summary${NC}"
echo -e "${BLUE}====================================${NC}"

# Count successes and failures
success_count=0
failure_count=0

for target in "${!TARGETS[@]}"; do
    if [[ "${RESULTS[$target]}" == "SUCCESS" ]]; then
        ((success_count++))
    else
        ((failure_count++))
    fi
done

echo "Total targets tested: ${#TARGETS[@]}"
echo -e "Successful builds: ${GREEN}${success_count}${NC}"
echo -e "Failed builds: ${RED}${failure_count}${NC}"
echo

# Detailed results
echo -e "${BLUE}📋 Detailed Results${NC}"
echo -e "${BLUE}==================${NC}"

for target in "${!TARGETS[@]}"; do
    local status="${RESULTS[$target]}"
    local duration="${DURATION[$target]}"
    local description="${TARGETS[$target]}"
    
    if [[ "$status" == "SUCCESS" ]]; then
        echo -e "${GREEN}✅ ${target}${NC}"
        echo -e "   Platform: ${description}"
        echo -e "   Status: Compilation successful"
        echo -e "   Duration: ${duration}s"
    else
        echo -e "${RED}❌ ${target}${NC}"
        echo -e "   Platform: ${description}"
        echo -e "   Status: Compilation failed"
        echo -e "   Duration: ${duration}s"
        echo -e "   Key errors: ${ERRORS[$target]}"
    fi
    echo
done

# Generate recommendations
echo -e "${BLUE}🔧 Recommendations${NC}"
echo -e "${BLUE}=================${NC}"

if [[ $failure_count -gt 0 ]]; then
    echo "Issues found with cross-compilation targets:"
    echo
    
    for target in "${!TARGETS[@]}"; do
        if [[ "${RESULTS[$target]}" == "FAILED" ]]; then
            case "$target" in
                "x86_64-apple-darwin")
                    echo -e "${YELLOW}• macOS Intel:${NC} NixOS clang wrapper incompatibility"
                    echo "  Fix: Update .cargo/config.toml with proper macOS linker configuration"
                    ;;
                "aarch64-unknown-linux-gnu")
                    echo -e "${YELLOW}• Linux ARM64:${NC} Cross-compilation timeout or missing toolchain"
                    echo "  Fix: Install aarch64-unknown-linux-gnu-gcc and configure properly"
                    ;;
                "x86_64-pc-windows-gnu")
                    echo -e "${YELLOW}• Windows x64:${NC} MinGW compilation errors"
                    echo "  Fix: Resolve Windows-specific dependency conflicts"
                    ;;
                "wasm32-unknown-unknown")
                    echo -e "${YELLOW}• WebAssembly:${NC} Feature flag and dependency issues"
                    echo "  Fix: Enable proper WASM features and resolve tokio/mio conflicts"
                    ;;
            esac
            echo
        fi
    done
fi

# Success rate calculation
success_rate=$((success_count * 100 / ${#TARGETS[@]}))
echo -e "Overall success rate: ${success_rate}%"

if [[ $success_rate -ge 80 ]]; then
    echo -e "${GREEN}✅ Cross-compilation infrastructure is mostly stable${NC}"
elif [[ $success_rate -ge 40 ]]; then
    echo -e "${YELLOW}⚠️  Cross-compilation needs improvements${NC}"
else
    echo -e "${RED}❌ Cross-compilation infrastructure needs major fixes${NC}"
fi

echo
echo -e "${BLUE}📁 Log files saved to: cross_compilation_results/${NC}"
echo "Check individual target logs for detailed error information."

# Save summary to file
{
    echo "CURSED Cross-Compilation Test Results"
    echo "====================================="
    echo "Test Date: $(date)"
    echo "Success Rate: ${success_rate}% (${success_count}/${#TARGETS[@]})"
    echo
    echo "Results:"
    for target in "${!TARGETS[@]}"; do
        echo "  $target: ${RESULTS[$target]} (${DURATION[$target]}s)"
    done
} > cross_compilation_results/summary.txt

echo "Summary saved to: cross_compilation_results/summary.txt"
