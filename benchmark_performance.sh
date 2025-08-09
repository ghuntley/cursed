#!/bin/bash

# Performance benchmark script for CURSED compiler optimizations
# Compares compilation speed and runtime performance before and after optimizations

set -e

echo "🚀 CURSED Compiler Performance Benchmark"
echo "========================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test files
TEST_FILES=(
    "performance_test.csd"
    "comprehensive_stdlib_test.csd" 
    "comprehensive_test.csd"
    "advanced_features_test.csd"
)

# Compiler executables to test
COMPILERS=(
    "cursed-stable"
    "cursed"
    "cursed-optimized"
)

# Results arrays
declare -A COMPILATION_TIMES
declare -A EXECUTION_TIMES
declare -A BINARY_SIZES

echo -e "${BLUE}Building optimized compiler...${NC}"
if ! zig build -Doptimize-compiler=true -Dparallel=true -Dcache=true -Dllvm-opt=O3; then
    echo -e "${RED}Failed to build optimized compiler${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Build completed successfully${NC}"
echo

# Function to measure compilation time
measure_compilation() {
    local compiler=$1
    local file=$2
    local iterations=${3:-3}
    
    echo -e "${YELLOW}Testing $compiler with $file...${NC}"
    
    local total_time=0
    local success_count=0
    
    for ((i=1; i<=iterations; i++)); do
        echo -n "  Run $i/$iterations: "
        
        # Clear any cached files
        rm -f "${file%.csd}" "${file%.csd}.ll" "${file%.csd}.o" 2>/dev/null || true
        
        # Measure compilation time
        local start_time=$(date +%s%N)
        
        if timeout 60s ./zig-out/bin/$compiler "$file" >/dev/null 2>&1; then
            local end_time=$(date +%s%N)
            local duration=$(((end_time - start_time) / 1000000)) # Convert to milliseconds
            
            echo -e "${GREEN}${duration}ms${NC}"
            total_time=$((total_time + duration))
            success_count=$((success_count + 1))
        else
            echo -e "${RED}FAILED${NC}"
        fi
    done
    
    if [ $success_count -gt 0 ]; then
        local avg_time=$((total_time / success_count))
        COMPILATION_TIMES["$compiler:$file"]=$avg_time
        echo -e "  ${GREEN}Average: ${avg_time}ms${NC}"
    else
        COMPILATION_TIMES["$compiler:$file"]="FAILED"
        echo -e "  ${RED}All runs failed${NC}"
    fi
    
    echo
}

# Function to measure binary size
measure_binary_size() {
    local compiler=$1
    local file=$2
    
    if [ -f "${file%.csd}" ]; then
        local size=$(stat -c%s "${file%.csd}" 2>/dev/null || echo "0")
        BINARY_SIZES["$compiler:$file"]=$size
    else
        BINARY_SIZES["$compiler:$file"]="N/A"
    fi
}

# Function to measure execution time
measure_execution() {
    local compiler=$1
    local file=$2
    local iterations=${3:-3}
    
    local binary="${file%.csd}"
    if [ ! -f "$binary" ]; then
        EXECUTION_TIMES["$compiler:$file"]="N/A"
        return
    fi
    
    local total_time=0
    local success_count=0
    
    for ((i=1; i<=iterations; i++)); do
        local start_time=$(date +%s%N)
        
        if timeout 30s ./"$binary" >/dev/null 2>&1; then
            local end_time=$(date +%s%N)
            local duration=$(((end_time - start_time) / 1000000)) # Convert to milliseconds
            total_time=$((total_time + duration))
            success_count=$((success_count + 1))
        fi
    done
    
    if [ $success_count -gt 0 ]; then
        local avg_time=$((total_time / success_count))
        EXECUTION_TIMES["$compiler:$file"]=$avg_time
    else
        EXECUTION_TIMES["$compiler:$file"]="FAILED"
    fi
}

# Run benchmarks
echo -e "${BLUE}Running compilation benchmarks...${NC}"
echo

for file in "${TEST_FILES[@]}"; do
    if [ ! -f "$file" ]; then
        echo -e "${YELLOW}Warning: $file not found, skipping${NC}"
        continue
    fi
    
    echo -e "${BLUE}Testing with $file${NC}"
    echo "$(printf '=%.0s' {1..50})"
    
    for compiler in "${COMPILERS[@]}"; do
        if [ -f "./zig-out/bin/$compiler" ]; then
            measure_compilation "$compiler" "$file"
            measure_binary_size "$compiler" "$file"
            measure_execution "$compiler" "$file"
        else
            echo -e "${YELLOW}Warning: $compiler not found, skipping${NC}"
        fi
    done
    
    echo
done

# Generate performance report
echo -e "${BLUE}Performance Report${NC}"
echo "==================="
echo

# Calculate speedup factors
echo -e "${GREEN}Compilation Speed Improvements:${NC}"
echo "$(printf '%-20s %-15s %-15s %-15s %s' 'Test File' 'Stable (ms)' 'Standard (ms)' 'Optimized (ms)' 'Speedup')"
echo "$(printf '=%.0s' {1..85})"

for file in "${TEST_FILES[@]}"; do
    if [ ! -f "$file" ]; then continue; fi
    
    stable_time=${COMPILATION_TIMES["cursed-stable:$file"]:-"N/A"}
    standard_time=${COMPILATION_TIMES["cursed:$file"]:-"N/A"}
    optimized_time=${COMPILATION_TIMES["cursed-optimized:$file"]:-"N/A"}
    
    speedup="N/A"
    if [[ "$stable_time" =~ ^[0-9]+$ ]] && [[ "$optimized_time" =~ ^[0-9]+$ ]] && [ "$optimized_time" -gt 0 ]; then
        speedup=$(echo "scale=2; $stable_time / $optimized_time" | bc -l 2>/dev/null || echo "N/A")
        if [ "$speedup" != "N/A" ]; then
            speedup="${speedup}x"
        fi
    fi
    
    printf "%-20s %-15s %-15s %-15s %s\n" \
        "${file%.csd}" \
        "$stable_time" \
        "$standard_time" \
        "$optimized_time" \
        "$speedup"
done

echo
echo -e "${GREEN}Binary Size Comparison:${NC}"
echo "$(printf '%-20s %-15s %-15s %-15s %s' 'Test File' 'Stable (KB)' 'Standard (KB)' 'Optimized (KB)' 'Reduction')"
echo "$(printf '=%.0s' {1..85})"

for file in "${TEST_FILES[@]}"; do
    if [ ! -f "$file" ]; then continue; fi
    
    stable_size=${BINARY_SIZES["cursed-stable:$file"]:-"N/A"}
    standard_size=${BINARY_SIZES["cursed:$file"]:-"N/A"}
    optimized_size=${BINARY_SIZES["cursed-optimized:$file"]:-"N/A"}
    
    # Convert bytes to KB
    if [[ "$stable_size" =~ ^[0-9]+$ ]]; then
        stable_size_kb=$((stable_size / 1024))
    else
        stable_size_kb="N/A"
    fi
    
    if [[ "$standard_size" =~ ^[0-9]+$ ]]; then
        standard_size_kb=$((standard_size / 1024))
    else
        standard_size_kb="N/A"
    fi
    
    if [[ "$optimized_size" =~ ^[0-9]+$ ]]; then
        optimized_size_kb=$((optimized_size / 1024))
    else
        optimized_size_kb="N/A"
    fi
    
    reduction="N/A"
    if [[ "$stable_size" =~ ^[0-9]+$ ]] && [[ "$optimized_size" =~ ^[0-9]+$ ]] && [ "$stable_size" -gt 0 ]; then
        reduction_percent=$(echo "scale=1; (($stable_size - $optimized_size) * 100) / $stable_size" | bc -l 2>/dev/null || echo "N/A")
        if [ "$reduction_percent" != "N/A" ]; then
            reduction="${reduction_percent}%"
        fi
    fi
    
    printf "%-20s %-15s %-15s %-15s %s\n" \
        "${file%.csd}" \
        "$stable_size_kb" \
        "$standard_size_kb" \
        "$optimized_size_kb" \
        "$reduction"
done

echo
echo -e "${GREEN}Summary:${NC}"

# Calculate overall metrics
total_tests=0
successful_optimizations=0
total_speedup=0

for file in "${TEST_FILES[@]}"; do
    if [ ! -f "$file" ]; then continue; fi
    
    stable_time=${COMPILATION_TIMES["cursed-stable:$file"]:-""}
    optimized_time=${COMPILATION_TIMES["cursed-optimized:$file"]:-""}
    
    if [[ "$stable_time" =~ ^[0-9]+$ ]] && [[ "$optimized_time" =~ ^[0-9]+$ ]] && [ "$optimized_time" -gt 0 ]; then
        speedup=$(echo "scale=2; $stable_time / $optimized_time" | bc -l 2>/dev/null || echo "0")
        if [ "$speedup" != "0" ]; then
            total_speedup=$(echo "$total_speedup + $speedup" | bc -l)
            successful_optimizations=$((successful_optimizations + 1))
        fi
    fi
    total_tests=$((total_tests + 1))
done

if [ $successful_optimizations -gt 0 ]; then
    avg_speedup=$(echo "scale=2; $total_speedup / $successful_optimizations" | bc -l)
    echo -e "• Average speedup: ${GREEN}${avg_speedup}x${NC}"
    echo -e "• Successful optimizations: ${GREEN}$successful_optimizations/$total_tests${NC}"
    
    if (( $(echo "$avg_speedup > 2.0" | bc -l) )); then
        echo -e "• Performance grade: ${GREEN}EXCELLENT${NC} (>2x speedup)"
    elif (( $(echo "$avg_speedup > 1.5" | bc -l) )); then
        echo -e "• Performance grade: ${GREEN}GOOD${NC} (>1.5x speedup)"
    elif (( $(echo "$avg_speedup > 1.2" | bc -l) )); then
        echo -e "• Performance grade: ${YELLOW}FAIR${NC} (>1.2x speedup)"
    else
        echo -e "• Performance grade: ${RED}POOR${NC} (<1.2x speedup)"
    fi
else
    echo -e "• ${RED}No successful optimizations measured${NC}"
fi

echo
echo -e "${BLUE}Benchmark completed!${NC}"

# Performance recommendations
echo
echo -e "${BLUE}Recommendations:${NC}"

if [ $successful_optimizations -eq 0 ]; then
    echo -e "• ${YELLOW}Enable LLVM optimizations with: zig build -Dllvm-opt=O3${NC}"
    echo -e "• ${YELLOW}Enable parallel compilation with: zig build -Dparallel=true${NC}"
    echo -e "• ${YELLOW}Enable caching with: zig build -Dcache=true${NC}"
elif (( $(echo "$avg_speedup < 2.0" | bc -l) )); then
    echo -e "• ${YELLOW}Consider using -Dfast-build=false for better runtime performance${NC}"
    echo -e "• ${YELLOW}Try higher LLVM optimization levels (O3, Ofast)${NC}"
    echo -e "• ${YELLOW}Enable profile-guided optimization for production builds${NC}"
else
    echo -e "• ${GREEN}Excellent performance! Consider these advanced optimizations:${NC}"
    echo -e "  - Link-time optimization (LTO)"
    echo -e "  - Profile-guided optimization (PGO)"
    echo -e "  - Target-specific optimizations"
fi

echo
echo -e "${GREEN}Performance benchmark completed successfully!${NC}"
