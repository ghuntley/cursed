#!/bin/bash

# Simple regression test script for CURSED compiler
# This script tests the basic functionality without requiring complex builds

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_ROOT"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*"
}

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
MEMORY_LEAK_TESTS=0

# Check if we have a working cursed binary
CURSED_BINARY=""
if [ -f "zig-out/bin/cursed-zig" ]; then
    CURSED_BINARY="./zig-out/bin/cursed-zig"
elif [ -f "zig-out/bin/cursed" ]; then
    CURSED_BINARY="./zig-out/bin/cursed"
elif [ -f "zig-out/bin/cursed-minimal" ]; then
    CURSED_BINARY="./zig-out/bin/cursed-minimal"
else
    log_error "No CURSED binary found. Try running 'zig build' first."
    exit 1
fi

log_info "Using CURSED binary: $CURSED_BINARY"

# Function to run a single test
run_test() {
    test_file="$1"
    category="$2"
    test_name=$(basename "$test_file" .csd)
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    # Determine expected behavior based on category
    should_succeed=true
    if [[ "$category" == "errors" ]]; then
        should_succeed=false
    fi
    
    # Run the test with timeout
    exit_code=0
    output_file="/tmp/cursed_test_${test_name}_$$.log"
    
    timeout 10 "$CURSED_BINARY" "$test_file" > "$output_file" 2>&1 || exit_code=$?
    
    # Check result
    test_passed=false
    message=""
    
    if [ $exit_code -eq 124 ]; then
        message="TIMEOUT"
    elif [ $exit_code -eq 0 ] && [ "$should_succeed" = true ]; then
        test_passed=true
        message="PASS"
    elif [ $exit_code -ne 0 ] && [ "$should_succeed" = false ]; then
        test_passed=true
        message="PASS (expected error)"
    elif [ $exit_code -eq 0 ] && [ "$should_succeed" = false ]; then
        message="FAIL (should have errored)"
    else
        message="FAIL (exit code: $exit_code)"
    fi
    
    # Update counters
    if [ "$test_passed" = true ]; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
        log_success "$category/$test_name - $message"
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
        log_error "$category/$test_name - $message"
        
        # Show error details for failures
        if [ -f "$output_file" ] && [ -s "$output_file" ]; then
            echo "  Error output:"
            head -3 "$output_file" | sed 's/^/    /'
        fi
    fi
    
    # Check for memory leaks if valgrind is available
    if command -v valgrind >/dev/null 2>&1; then
        valgrind_output="/tmp/cursed_valgrind_${test_name}_$$.log"
        timeout 10 valgrind --leak-check=summary --quiet \
            "$CURSED_BINARY" "$test_file" >/dev/null 2>"$valgrind_output" || true
        
        if grep -q "definitely lost" "$valgrind_output" 2>/dev/null; then
            MEMORY_LEAK_TESTS=$((MEMORY_LEAK_TESTS + 1))
            log_warning "$category/$test_name - Memory leaks detected"
        fi
        
        rm -f "$valgrind_output"
    fi
    
    rm -f "$output_file"
}

# Main test execution
log_info "=== CURSED REGRESSION TEST SUITE ==="
log_info "Testing with binary: $CURSED_BINARY"

# Test categories
declare -A TEST_CATEGORIES=(
    ["parser"]="Parser and AST generation tests"
    ["stdlib"]="Standard library import and function tests" 
    ["memory"]="Memory safety and leak detection tests"
    ["errors"]="Error handling and recovery tests"
    ["roundtrip"]="Round-trip parsing and serialization tests"
)

# Run tests for each category
for category in "${!TEST_CATEGORIES[@]}"; do
    test_dir="tests/regression/$category"
    
    if [ ! -d "$test_dir" ]; then
        log_warning "Test directory not found: $test_dir"
        continue
    fi
    
    log_info "Running $category tests: ${TEST_CATEGORIES[$category]}"
    
    # Find and run all .csd files
    test_count=0
    while IFS= read -r -d '' test_file; do
        run_test "$test_file" "$category"
        test_count=$((test_count + 1))
    done < <(find "$test_dir" -name "*.csd" -print0 | sort -z)
    
    if [ $test_count -eq 0 ]; then
        log_warning "No test files found in $test_dir"
    else
        log_info "Completed $test_count tests in $category category"
    fi
done

# Generate simple report
echo ""
log_info "=== TEST SUMMARY ==="
log_info "Total Tests: $TOTAL_TESTS"
if [ $PASSED_TESTS -gt 0 ]; then
    log_success "Passed: $PASSED_TESTS ($(( TOTAL_TESTS > 0 ? PASSED_TESTS * 100 / TOTAL_TESTS : 0 ))%)"
fi
if [ $FAILED_TESTS -gt 0 ]; then
    log_error "Failed: $FAILED_TESTS ($(( TOTAL_TESTS > 0 ? FAILED_TESTS * 100 / TOTAL_TESTS : 0 ))%)"
fi
if [ $MEMORY_LEAK_TESTS -gt 0 ]; then
    log_warning "Memory Leaks: $MEMORY_LEAK_TESTS"
fi
log_info "==================="

# Create a simple summary report
REPORT_FILE="test_coverage/regression_test_summary_$(date +%Y%m%d_%H%M%S).md"
mkdir -p test_coverage

cat > "$REPORT_FILE" << EOF
# CURSED Compiler Regression Test Summary

**Generated:** $(date)
**Binary Used:** $CURSED_BINARY

## Results

- **Total Tests:** $TOTAL_TESTS
- **Passed:** $PASSED_TESTS ($(( TOTAL_TESTS > 0 ? PASSED_TESTS * 100 / TOTAL_TESTS : 0 ))%)
- **Failed:** $FAILED_TESTS ($(( TOTAL_TESTS > 0 ? FAILED_TESTS * 100 / TOTAL_TESTS : 0 ))%)
- **Memory Leaks:** $MEMORY_LEAK_TESTS

## Test Categories

EOF

for category in "${!TEST_CATEGORIES[@]}"; do
    echo "- **$category**: ${TEST_CATEGORIES[$category]}" >> "$REPORT_FILE"
done

log_info "Report saved to: $REPORT_FILE"

# Exit with appropriate code
if [ $FAILED_TESTS -gt 0 ]; then
    log_error "Some tests failed"
    exit 1
else
    log_success "All tests passed!"
    exit 0
fi
