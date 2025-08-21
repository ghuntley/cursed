#!/bin/bash
# CURSED v1.0.x Regression Test Suite
# Comprehensive testing for patch release validation

set -euo pipefail

# Configuration
CURSED_BIN="${CURSED_BIN:-./zig-out/bin/cursed-zig}"
TEST_DIR="${TEST_DIR:-./test_suite}"
TEMP_DIR="/tmp/cursed-regression-$$"
LOG_FILE="regression-test-results-$(date +%Y%m%d-%H%M%S).log"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
SKIPPED_TESTS=0

# Initialize
mkdir -p "$TEMP_DIR"
exec > >(tee -a "$LOG_FILE")
exec 2>&1

echo "==================================================="
echo "CURSED v1.0.x Regression Test Suite"
echo "Started: $(date)"
echo "CURSED Binary: $CURSED_BIN"
echo "Test Directory: $TEST_DIR"
echo "==================================================="

# Verify CURSED installation
verify_installation() {
    echo -e "${BLUE}[INFO]${NC} Verifying CURSED installation..."
    
    if [[ ! -x "$CURSED_BIN" ]]; then
        echo -e "${RED}[ERROR]${NC} CURSED binary not found or not executable: $CURSED_BIN"
        exit 1
    fi
    
    local version_output
    if version_output=$("$CURSED_BIN" --version 2>&1); then
        echo -e "${GREEN}[PASS]${NC} CURSED version: $version_output"
    else
        echo -e "${RED}[FAIL]${NC} Failed to get CURSED version"
        exit 1
    fi
}

# Run a single test
run_test() {
    local test_file="$1"
    local test_name=$(basename "$test_file" .csd)
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    echo -e "${BLUE}[TEST]${NC} Running $test_name..."
    
    # Check if test should be skipped
    if grep -q "// SKIP" "$test_file"; then
        echo -e "${YELLOW}[SKIP]${NC} Test marked as skipped: $test_name"
        SKIPPED_TESTS=$((SKIPPED_TESTS + 1))
        return 0
    fi
    
    local output_file="$TEMP_DIR/$test_name.out"
    local error_file="$TEMP_DIR/$test_name.err"
    local expected_file="${test_file%.csd}.expected"
    
    # Run the test
    if timeout 30s "$CURSED_BIN" "$test_file" > "$output_file" 2> "$error_file"; then
        local test_passed=true
        
        # Check expected output if available
        if [[ -f "$expected_file" ]]; then
            if ! diff -q "$output_file" "$expected_file" > /dev/null; then
                echo -e "${RED}[FAIL]${NC} Output mismatch for $test_name"
                echo "Expected:"
                cat "$expected_file" | head -20
                echo "Actual:"
                cat "$output_file" | head -20
                test_passed=false
            fi
        fi
        
        # Check for memory errors in stderr
        if grep -q "ERROR\|LEAK\|CRASH" "$error_file"; then
            echo -e "${RED}[FAIL]${NC} Memory or runtime error in $test_name"
            cat "$error_file"
            test_passed=false
        fi
        
        if $test_passed; then
            echo -e "${GREEN}[PASS]${NC} $test_name"
            PASSED_TESTS=$((PASSED_TESTS + 1))
        else
            FAILED_TESTS=$((FAILED_TESTS + 1))
        fi
    else
        echo -e "${RED}[FAIL]${NC} Execution failed for $test_name"
        if [[ -s "$error_file" ]]; then
            echo "Error output:"
            cat "$error_file"
        fi
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
}

# Memory safety tests
run_memory_tests() {
    echo -e "${BLUE}[INFO]${NC} Running memory safety tests..."
    
    local memory_test_files=(
        "comprehensive_memory_test.csd"
        "array_bounds_test.csd" 
        "string_safety_test.csd"
        "goroutine_memory_test.csd"
    )
    
    for test_file in "${memory_test_files[@]}"; do
        local full_path="$TEST_DIR/$test_file"
        if [[ -f "$full_path" ]]; then
            # Run with valgrind if available
            if command -v valgrind >/dev/null 2>&1; then
                echo -e "${BLUE}[INFO]${NC} Running $test_file with Valgrind..."
                local valgrind_out="$TEMP_DIR/$(basename "$test_file" .csd).valgrind"
                
                if valgrind --error-exitcode=1 --leak-check=full \
                   --show-leak-kinds=all --track-origins=yes \
                   "$CURSED_BIN" "$full_path" 2> "$valgrind_out"; then
                    echo -e "${GREEN}[PASS]${NC} Memory safety: $test_file"
                    PASSED_TESTS=$((PASSED_TESTS + 1))
                else
                    echo -e "${RED}[FAIL]${NC} Memory issues in $test_file"
                    cat "$valgrind_out"
                    FAILED_TESTS=$((FAILED_TESTS + 1))
                fi
                TOTAL_TESTS=$((TOTAL_TESTS + 1))
            else
                run_test "$full_path"
            fi
        fi
    done
}

# Performance regression tests
run_performance_tests() {
    echo -e "${BLUE}[INFO]${NC} Running performance regression tests..."
    
    local perf_test_file="$TEST_DIR/performance_benchmark.csd"
    if [[ -f "$perf_test_file" ]]; then
        echo -e "${BLUE}[INFO]${NC} Running performance benchmark..."
        
        local benchmark_out="$TEMP_DIR/benchmark.out"
        local benchmark_baseline="$TEST_DIR/performance_baseline.txt"
        
        # Run benchmark 3 times and take average
        local total_time=0
        for i in {1..3}; do
            local start_time=$(date +%s.%N)
            "$CURSED_BIN" "$perf_test_file" > /dev/null 2>&1
            local end_time=$(date +%s.%N)
            local run_time=$(echo "$end_time - $start_time" | bc)
            total_time=$(echo "$total_time + $run_time" | bc)
        done
        
        local avg_time=$(echo "scale=3; $total_time / 3" | bc)
        echo "Average execution time: ${avg_time}s" > "$benchmark_out"
        
        # Compare with baseline if available
        if [[ -f "$benchmark_baseline" ]]; then
            local baseline_time=$(cat "$benchmark_baseline")
            local threshold=$(echo "$baseline_time * 1.5" | bc) # 50% regression threshold
            
            if (( $(echo "$avg_time > $threshold" | bc -l) )); then
                echo -e "${RED}[FAIL]${NC} Performance regression detected"
                echo "Baseline: ${baseline_time}s, Current: ${avg_time}s"
                FAILED_TESTS=$((FAILED_TESTS + 1))
            else
                echo -e "${GREEN}[PASS]${NC} Performance within acceptable range"
                PASSED_TESTS=$((PASSED_TESTS + 1))
            fi
        else
            echo -e "${YELLOW}[INFO]${NC} No baseline available, recording current performance: ${avg_time}s"
            echo "$avg_time" > "$benchmark_baseline"
            PASSED_TESTS=$((PASSED_TESTS + 1))
        fi
        
        TOTAL_TESTS=$((TOTAL_TESTS + 1))
    fi
}

# Cross-compilation tests
run_cross_compilation_tests() {
    echo -e "${BLUE}[INFO]${NC} Running cross-compilation tests..."
    
    local simple_test="$TEST_DIR/basic_test.csd"
    if [[ ! -f "$simple_test" ]]; then
        echo -e "${YELLOW}[SKIP]${NC} No basic test file found for cross-compilation"
        return
    fi
    
    local targets=(
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-macos"
        "aarch64-macos"
        "x86_64-windows"
    )
    
    for target in "${targets[@]}"; do
        echo -e "${BLUE}[INFO]${NC} Testing cross-compilation for $target..."
        
        local output_binary="$TEMP_DIR/test-$target"
        if timeout 60s "$CURSED_BIN" --compile --target "$target" \
           --output "$output_binary" "$simple_test" 2>/dev/null; then
            echo -e "${GREEN}[PASS]${NC} Cross-compilation: $target"
            PASSED_TESTS=$((PASSED_TESTS + 1))
        else
            echo -e "${RED}[FAIL]${NC} Cross-compilation failed: $target"
            FAILED_TESTS=$((FAILED_TESTS + 1))
        fi
        TOTAL_TESTS=$((TOTAL_TESTS + 1))
    done
}

# Standard library tests
run_stdlib_tests() {
    echo -e "${BLUE}[INFO]${NC} Running standard library tests..."
    
    local stdlib_test="comprehensive_stdlib_test.csd"
    if [[ -f "$stdlib_test" ]]; then
        run_test "$stdlib_test"
    fi
    
    # Test individual modules
    local modules=(
        "vibez" "mathz" "stringz" "arrayz" "filez"
        "networkz" "timez" "testz" "concurrenz"
    )
    
    for module in "${modules[@]}"; do
        local module_test="$TEST_DIR/${module}_test.csd"
        if [[ -f "$module_test" ]]; then
            run_test "$module_test"
        fi
    done
}

# Concurrency tests
run_concurrency_tests() {
    echo -e "${BLUE}[INFO]${NC} Running concurrency tests..."
    
    local concurrency_tests=(
        "goroutine_basic_test.csd"
        "channel_operations_test.csd"
        "concurrent_stress_test.csd"
        "race_condition_test.csd"
    )
    
    for test in "${concurrency_tests[@]}"; do
        local test_path="$TEST_DIR/$test"
        if [[ -f "$test_path" ]]; then
            run_test "$test_path"
        fi
    done
}

# Error handling tests
run_error_tests() {
    echo -e "${BLUE}[INFO]${NC} Running error handling tests..."
    
    local error_tests=(
        "error_propagation_test.csd"
        "panic_recovery_test.csd"
        "resource_cleanup_test.csd"
    )
    
    for test in "${error_tests[@]}"; do
        local test_path="$TEST_DIR/$test"
        if [[ -f "$test_path" ]]; then
            run_test "$test_path"
        fi
    done
}

# Syntax and parsing tests
run_syntax_tests() {
    echo -e "${BLUE}[INFO]${NC} Running syntax and parsing tests..."
    
    # Find all .csd files in test directory
    while IFS= read -r -d '' test_file; do
        # Skip specific test categories handled elsewhere
        local basename=$(basename "$test_file")
        if [[ "$basename" =~ ^(memory|performance|cross|stdlib|concurrency|error)_ ]]; then
            continue
        fi
        
        run_test "$test_file"
    done < <(find "$TEST_DIR" -name "*.csd" -type f -print0 2>/dev/null | head -50)
}

# Report results
generate_report() {
    echo ""
    echo "==================================================="
    echo "Regression Test Results"
    echo "==================================================="
    echo "Total Tests: $TOTAL_TESTS"
    echo "Passed: $PASSED_TESTS"
    echo "Failed: $FAILED_TESTS"  
    echo "Skipped: $SKIPPED_TESTS"
    echo ""
    
    if [[ $FAILED_TESTS -eq 0 ]]; then
        echo -e "${GREEN}✅ All tests passed!${NC}"
        echo "Success rate: 100%"
    else
        local success_rate=$((PASSED_TESTS * 100 / (PASSED_TESTS + FAILED_TESTS)))
        echo -e "${RED}❌ $FAILED_TESTS tests failed${NC}"
        echo "Success rate: $success_rate%"
    fi
    
    echo ""
    echo "Completed: $(date)"
    echo "Log file: $LOG_FILE"
    echo "==================================================="
}

# Cleanup
cleanup() {
    echo -e "${BLUE}[INFO]${NC} Cleaning up temporary files..."
    rm -rf "$TEMP_DIR"
}

# Main execution
main() {
    verify_installation
    
    # Create test directory structure if needed
    mkdir -p "$TEST_DIR"
    
    # Run test suites
    run_syntax_tests
    run_stdlib_tests
    run_memory_tests
    run_concurrency_tests
    run_error_tests
    run_performance_tests
    run_cross_compilation_tests
    
    generate_report
    cleanup
    
    # Exit with error if any tests failed
    if [[ $FAILED_TESTS -gt 0 ]]; then
        exit 1
    fi
}

# Handle interrupts
trap cleanup EXIT
trap 'echo -e "${RED}[INTERRUPTED]${NC} Test suite interrupted"; exit 1' INT TERM

# Run main function
main "$@"
