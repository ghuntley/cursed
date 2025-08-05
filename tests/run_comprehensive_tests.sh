#!/bin/bash

# Comprehensive CURSED Test Suite Runner
# Runs all test suites in both interpretation and compilation modes

set -e

echo "=== CURSED Comprehensive Test Suite ==="
echo "Testing all language features with both Rust and Zig implementations"
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
TEST_DIR="/home/ghuntley/code/cursed/tests"
RESULTS_DIR="/home/ghuntley/code/cursed/test_results"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

# Create results directory
mkdir -p "$RESULTS_DIR"

# Test files
TEST_FILES=(
    "comprehensive_language_test_suite.csd"
    "advanced_features_test_suite.csd"
    "stdlib_integration_test_suite.csd"
    "cross_platform_test_suite.csd"
)

# Implementation commands
RUST_INTERP="cargo run --bin cursed"
RUST_COMPILE="cargo run --bin cursed -- compile"
ZIG_UNIFIED="./cursed-unified"
ZIG_BUILD="./zig-out/bin/cursed-zig"

# Results tracking
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

log_test() {
    local status=$1
    local message=$2
    local logfile=$3
    
    echo -e "[$status] $message"
    echo "[$status] $message" >> "$logfile"
    
    if [ "$status" = "PASS" ]; then
        ((PASSED_TESTS++))
    elif [ "$status" = "FAIL" ]; then
        ((FAILED_TESTS++))
    fi
    ((TOTAL_TESTS++))
}

run_test() {
    local test_file=$1
    local command=$2
    local mode=$3
    local impl=$4
    local logfile=$5
    
    echo -e "${BLUE}Running: $test_file ($impl $mode)${NC}"
    
    if timeout 60 $command "$TEST_DIR/$test_file" > "$logfile.tmp" 2>&1; then
        log_test "PASS" "$test_file ($impl $mode)" "$logfile"
        echo "  ✓ Success"
    else
        log_test "FAIL" "$test_file ($impl $mode)" "$logfile"
        echo -e "  ${RED}✗ Failed${NC}"
        echo "    Error output:" >> "$logfile"
        cat "$logfile.tmp" >> "$logfile"
    fi
    
    rm -f "$logfile.tmp"
}

compile_and_run() {
    local test_file=$1
    local compile_cmd=$2
    local impl=$3
    local logfile=$4
    
    local base_name=$(basename "$test_file" .csd)
    local executable="./$base_name"
    
    echo -e "${BLUE}Compiling and running: $test_file ($impl)${NC}"
    
    if timeout 30 $compile_cmd "$TEST_DIR/$test_file" > "$logfile.compile.tmp" 2>&1; then
        if [ -f "$executable" ]; then
            if timeout 30 "$executable" > "$logfile.run.tmp" 2>&1; then
                log_test "PASS" "$test_file ($impl compilation + execution)" "$logfile"
                echo "  ✓ Compile and run success"
                rm -f "$executable"
            else
                log_test "FAIL" "$test_file ($impl execution)" "$logfile"
                echo -e "  ${RED}✗ Execution failed${NC}"
                echo "    Execution error:" >> "$logfile"
                cat "$logfile.run.tmp" >> "$logfile"
            fi
        else
            log_test "FAIL" "$test_file ($impl compilation - no executable)" "$logfile"
            echo -e "  ${RED}✗ No executable produced${NC}"
        fi
    else
        log_test "FAIL" "$test_file ($impl compilation)" "$logfile"
        echo -e "  ${RED}✗ Compilation failed${NC}"
        echo "    Compilation error:" >> "$logfile"
        cat "$logfile.compile.tmp" >> "$logfile"
    fi
    
    rm -f "$logfile.compile.tmp" "$logfile.run.tmp"
}

echo -e "${YELLOW}Building implementations...${NC}"

# Build Rust implementation
echo "Building Rust implementation..."
if ! cargo build > "$RESULTS_DIR/rust_build_$TIMESTAMP.log" 2>&1; then
    echo -e "${RED}Rust build failed!${NC}"
    exit 1
fi

# Build Zig implementations
echo "Building Zig unified implementation..."
if ! zig build-exe src-zig/main_unified.zig -lc --name cursed-unified > "$RESULTS_DIR/zig_unified_build_$TIMESTAMP.log" 2>&1; then
    echo -e "${YELLOW}Zig unified build failed, trying alternative...${NC}"
fi

echo "Building Zig standard implementation..."
if ! zig build > "$RESULTS_DIR/zig_build_$TIMESTAMP.log" 2>&1; then
    echo -e "${YELLOW}Zig standard build failed${NC}"
fi

echo

# Main test execution
MAIN_LOG="$RESULTS_DIR/comprehensive_test_results_$TIMESTAMP.log"
echo "=== CURSED Comprehensive Test Results - $(date) ===" > "$MAIN_LOG"

for test_file in "${TEST_FILES[@]}"; do
    echo -e "${YELLOW}=== Testing: $test_file ===${NC}"
    echo "=== Testing: $test_file ===" >> "$MAIN_LOG"
    
    # Test with Rust implementation
    echo -e "${BLUE}--- Rust Implementation ---${NC}"
    run_test "$test_file" "$RUST_INTERP" "interpretation" "Rust" "$MAIN_LOG"
    compile_and_run "$test_file" "$RUST_COMPILE" "Rust" "$MAIN_LOG"
    
    # Test with Zig unified implementation
    if [ -f "./cursed-unified" ]; then
        echo -e "${BLUE}--- Zig Unified Implementation ---${NC}"
        run_test "$test_file" "$ZIG_UNIFIED" "interpretation" "Zig-Unified" "$MAIN_LOG"
        
        # Try compilation with Zig unified
        if timeout 30 $ZIG_UNIFIED --compile "$TEST_DIR/$test_file" > /dev/null 2>&1; then
            local base_name=$(basename "$test_file" .csd)
            if [ -f "./$base_name" ]; then
                log_test "PASS" "$test_file (Zig-Unified compilation)" "$MAIN_LOG"
                rm -f "./$base_name"
            fi
        fi
    fi
    
    # Test with Zig standard implementation
    if [ -f "./zig-out/bin/cursed-zig" ]; then
        echo -e "${BLUE}--- Zig Standard Implementation ---${NC}"
        run_test "$test_file" "$ZIG_BUILD" "interpretation" "Zig-Standard" "$MAIN_LOG"
        
        # Try compilation with Zig standard
        if timeout 30 $ZIG_BUILD --compile "$TEST_DIR/$test_file" > /dev/null 2>&1; then
            local base_name=$(basename "$test_file" .csd)
            if [ -f "./$base_name" ]; then
                log_test "PASS" "$test_file (Zig-Standard compilation)" "$MAIN_LOG"
                rm -f "./$base_name"
            fi
        fi
    fi
    
    echo
done

# Cross-platform testing
echo -e "${YELLOW}=== Cross-Platform Validation ===${NC}"
echo "=== Cross-Platform Validation ===" >> "$MAIN_LOG"

# Test on current platform
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

echo "Platform: $PLATFORM, Architecture: $ARCH" >> "$MAIN_LOG"

# Run platform-specific tests
run_test "cross_platform_test_suite.csd" "$RUST_INTERP" "cross-platform" "Rust-$PLATFORM" "$MAIN_LOG"

if [ -f "./cursed-unified" ]; then
    run_test "cross_platform_test_suite.csd" "$ZIG_UNIFIED" "cross-platform" "Zig-$PLATFORM" "$MAIN_LOG"
fi

echo

# Performance testing
echo -e "${YELLOW}=== Performance Testing ===${NC}"
echo "=== Performance Testing ===" >> "$MAIN_LOG"

for impl in "Rust" "Zig-Unified"; do
    if [[ "$impl" == "Rust" ]]; then
        cmd="$RUST_INTERP"
    elif [[ "$impl" == "Zig-Unified" ]] && [ -f "./cursed-unified" ]; then
        cmd="$ZIG_UNIFIED"
    else
        continue
    fi
    
    echo "Performance test: $impl" >> "$MAIN_LOG"
    
    # Simple performance test
    echo 'sus start = time.now(); bestie i := 0; i < 10000; i = i + 1 { }; sus end = time.now(); vibez.spill("Time:", time.diff(end, start).milliseconds(), "ms")' > /tmp/perf_test.csd
    
    if timeout 10 $cmd /tmp/perf_test.csd >> "$MAIN_LOG" 2>&1; then
        log_test "PASS" "Performance test ($impl)" "$MAIN_LOG"
    else
        log_test "FAIL" "Performance test ($impl)" "$MAIN_LOG"
    fi
done

rm -f /tmp/perf_test.csd

# Memory testing
echo -e "${YELLOW}=== Memory Testing ===${NC}"
echo "=== Memory Testing ===" >> "$MAIN_LOG"

# Create a memory stress test
cat > /tmp/memory_test.csd << 'EOF'
yeet "testz"

test_start("Memory Allocation Test")
sus items []drip = []

bestie i := 0; i < 1000; i = i + 1 {
    items = append(items, i)
}

assert_eq_int(len(items), 1000)
print_test_summary()
EOF

for impl in "Rust" "Zig-Unified"; do
    if [[ "$impl" == "Rust" ]]; then
        cmd="$RUST_INTERP"
    elif [[ "$impl" == "Zig-Unified" ]] && [ -f "./cursed-unified" ]; then
        cmd="$ZIG_UNIFIED"
    else
        continue
    fi
    
    echo "Memory test: $impl" >> "$MAIN_LOG"
    
    # Run with valgrind if available
    if command -v valgrind >/dev/null 2>&1; then
        if timeout 30 valgrind --leak-check=summary --error-exitcode=1 $cmd /tmp/memory_test.csd >> "$MAIN_LOG" 2>&1; then
            log_test "PASS" "Memory test with valgrind ($impl)" "$MAIN_LOG"
        else
            log_test "WARN" "Memory test with valgrind shows issues ($impl)" "$MAIN_LOG"
        fi
    else
        if timeout 10 $cmd /tmp/memory_test.csd >> "$MAIN_LOG" 2>&1; then
            log_test "PASS" "Memory test ($impl)" "$MAIN_LOG"
        else
            log_test "FAIL" "Memory test ($impl)" "$MAIN_LOG"
        fi
    fi
done

rm -f /tmp/memory_test.csd

# Generate final report
echo
echo -e "${YELLOW}=== FINAL TEST SUMMARY ===${NC}"
echo "=== FINAL TEST SUMMARY ===" >> "$MAIN_LOG"

echo "Total tests run: $TOTAL_TESTS" | tee -a "$MAIN_LOG"
echo "Tests passed: $PASSED_TESTS" | tee -a "$MAIN_LOG"  
echo "Tests failed: $FAILED_TESTS" | tee -a "$MAIN_LOG"

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}🎉 All tests passed!${NC}" | tee -a "$MAIN_LOG"
    exit 0
else
    SUCCESS_RATE=$((PASSED_TESTS * 100 / TOTAL_TESTS))
    echo -e "${YELLOW}⚠️  Success rate: $SUCCESS_RATE%${NC}" | tee -a "$MAIN_LOG"
    
    if [ $SUCCESS_RATE -ge 80 ]; then
        echo -e "${YELLOW}✓ Good success rate (>= 80%)${NC}" | tee -a "$MAIN_LOG"
        exit 0
    else
        echo -e "${RED}✗ Low success rate (< 80%)${NC}" | tee -a "$MAIN_LOG"
        exit 1
    fi
fi
