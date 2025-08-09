#!/bin/bash

# Comprehensive regression test script for CURSED compiler
# This script runs all regression tests and generates detailed reports

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

# Configuration
TIMEOUT_SECONDS=10
VALGRIND_ENABLED=true
VERBOSE=false
PARALLEL_JOBS=4
OUTPUT_DIR="test_coverage"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --no-valgrind)
            VALGRIND_ENABLED=false
            shift
            ;;
        --timeout)
            TIMEOUT_SECONDS="$2"
            shift 2
            ;;
        --jobs|-j)
            PARALLEL_JOBS="$2"
            shift 2
            ;;
        --output-dir|-o)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  --verbose, -v          Enable verbose output"
            echo "  --no-valgrind          Disable valgrind memory checking"
            echo "  --timeout SECONDS      Set timeout per test (default: 10)"
            echo "  --jobs, -j NUM         Number of parallel jobs (default: 4)"
            echo "  --output-dir, -o DIR   Output directory for reports (default: test_coverage)"
            echo "  --help, -h             Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1" >&2
            exit 1
            ;;
    esac
done

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

# Create output directory
mkdir -p "$OUTPUT_DIR"
REPORT_FILE="$OUTPUT_DIR/regression_test_report_$TIMESTAMP.md"

# Initialize report
cat > "$REPORT_FILE" << EOF
# CURSED Compiler Regression Test Report

**Generated:** $(date)
**Configuration:**
- Timeout: ${TIMEOUT_SECONDS}s per test
- Valgrind: $([ "$VALGRIND_ENABLED" = true ] && echo "Enabled" || echo "Disabled")
- Parallel jobs: $PARALLEL_JOBS
- Verbose: $VERBOSE

## Test Results

EOF

# Ensure build is up to date
log_info "Building CURSED compiler..."
if ! zig build; then
    log_error "Build failed!"
    exit 1
fi

# Check if main executable exists
if [ ! -f "zig-out/bin/cursed-zig" ]; then
    log_error "Main executable not found: zig-out/bin/cursed-zig"
    exit 1
fi

# Test categories
declare -A TEST_CATEGORIES=(
    ["parser"]="Parser and AST generation tests"
    ["stdlib"]="Standard library import and function tests" 
    ["memory"]="Memory safety and leak detection tests"
    ["errors"]="Error handling and recovery tests"
    ["roundtrip"]="Round-trip parsing and serialization tests"
)

# Global test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
MEMORY_LEAK_TESTS=0

# Function to run a single test
run_single_test() {
    local test_file="$1"
    local category="$2"
    local test_name=$(basename "$test_file" .csd)
    local start_time=$(date +%s%3N)
    
    local test_output_file="$OUTPUT_DIR/${category}_${test_name}_$TIMESTAMP.log"
    local valgrind_output_file="$OUTPUT_DIR/${category}_${test_name}_valgrind_$TIMESTAMP.log"
    
    # Run basic parser test
    local exit_code=0
    timeout "$TIMEOUT_SECONDS" ./zig-out/bin/cursed-zig "$test_file" > "$test_output_file" 2>&1 || exit_code=$?
    
    local end_time=$(date +%s%3N)
    local duration=$((end_time - start_time))
    
    # Check for memory leaks if valgrind is enabled
    local memory_leaks=0
    if [ "$VALGRIND_ENABLED" = true ]; then
        timeout "$TIMEOUT_SECONDS" valgrind --leak-check=summary --quiet \
            ./zig-out/bin/cursed-zig "$test_file" > /dev/null 2> "$valgrind_output_file" || true
        
        # Parse valgrind output for leaks
        if grep -q "definitely lost" "$valgrind_output_file"; then
            memory_leaks=$(grep "definitely lost" "$valgrind_output_file" | \
                          awk '{print $4}' | sed 's/,//g' || echo "0")
        fi
    fi
    
    # Determine test result
    local status="PASS"
    local error_message=""
    
    if [ $exit_code -eq 124 ]; then
        status="TIMEOUT"
        error_message="Test timed out after ${TIMEOUT_SECONDS}s"
    elif [ $exit_code -ne 0 ]; then
        status="FAIL"
        error_message="Exit code: $exit_code"
    elif [ "$memory_leaks" -gt 0 ]; then
        status="MEMORY_LEAK"
        error_message="Memory leaks detected: ${memory_leaks} bytes"
    fi
    
    # Update counters (thread-safe with file locking)
    (
        flock -x 200
        echo "$status $test_file $duration $memory_leaks $error_message" >> "$OUTPUT_DIR/test_results_raw.txt"
    ) 200>"$OUTPUT_DIR/test_results.lock"
    
    # Log result
    case $status in
        "PASS")
            [ "$VERBOSE" = true ] && log_success "$test_name (${duration}ms)"
            ;;
        "TIMEOUT")
            log_warning "$test_name - $error_message"
            ;;
        "FAIL")
            log_error "$test_name - $error_message"
            [ "$VERBOSE" = true ] && cat "$test_output_file"
            ;;
        "MEMORY_LEAK")
            log_warning "$test_name - $error_message"
            ;;
    esac
}

# Function to run tests in a category
run_test_category() {
    local category="$1"
    local description="${TEST_CATEGORIES[$category]}"
    
    log_info "Running $category tests: $description"
    
    local test_dir="tests/regression/$category"
    if [ ! -d "$test_dir" ]; then
        log_warning "Test directory not found: $test_dir"
        return
    fi
    
    # Find all .csd files in the category
    local test_files=()
    while IFS= read -r -d '' file; do
        test_files+=("$file")
    done < <(find "$test_dir" -name "*.csd" -print0 | sort -z)
    
    if [ ${#test_files[@]} -eq 0 ]; then
        log_warning "No test files found in $test_dir"
        return
    fi
    
    # Run tests in parallel
    export -f run_single_test log_info log_success log_warning log_error
    export TIMEOUT_SECONDS VALGRIND_ENABLED VERBOSE OUTPUT_DIR TIMESTAMP
    export RED GREEN YELLOW BLUE NC
    
    printf "%s\n" "${test_files[@]}" | \
        xargs -I {} -P "$PARALLEL_JOBS" bash -c "run_single_test '{}' '$category'"
}

# Main test execution
log_info "Starting comprehensive regression tests..."

# Initialize results file
echo "" > "$OUTPUT_DIR/test_results_raw.txt"

# Run tests for each category
for category in "${!TEST_CATEGORIES[@]}"; do
    run_test_category "$category"
done

# Process results
log_info "Processing test results..."

if [ -f "$OUTPUT_DIR/test_results_raw.txt" ]; then
    while read -r status test_file duration memory_leaks error_message; do
        [ -z "$status" ] && continue
        
        TOTAL_TESTS=$((TOTAL_TESTS + 1))
        
        case $status in
            "PASS")
                PASSED_TESTS=$((PASSED_TESTS + 1))
                ;;
            "MEMORY_LEAK")
                MEMORY_LEAK_TESTS=$((MEMORY_LEAK_TESTS + 1))
                ;;
            *)
                FAILED_TESTS=$((FAILED_TESTS + 1))
                ;;
        esac
    done < "$OUTPUT_DIR/test_results_raw.txt"
fi

# Generate detailed report
cat >> "$REPORT_FILE" << EOF
### Summary

- **Total Tests:** $TOTAL_TESTS
- **Passed:** $PASSED_TESTS ($(( TOTAL_TESTS > 0 ? PASSED_TESTS * 100 / TOTAL_TESTS : 0 ))%)
- **Failed:** $FAILED_TESTS ($(( TOTAL_TESTS > 0 ? FAILED_TESTS * 100 / TOTAL_TESTS : 0 ))%)
- **Memory Leaks:** $MEMORY_LEAK_TESTS

### Detailed Results

| Test | Category | Status | Duration (ms) | Memory Leaks | Error |
|------|----------|--------|---------------|--------------|-------|
EOF

if [ -f "$OUTPUT_DIR/test_results_raw.txt" ]; then
    while read -r status test_file duration memory_leaks error_message; do
        [ -z "$status" ] && continue
        
        local test_name=$(basename "$test_file" .csd)
        local category=$(basename "$(dirname "$test_file")")
        local status_emoji
        
        case $status in
            "PASS") status_emoji="✅" ;;
            "MEMORY_LEAK") status_emoji="🚨" ;;
            *) status_emoji="❌" ;;
        esac
        
        echo "| $test_name | $category | $status_emoji $status | $duration | $memory_leaks | $error_message |" >> "$REPORT_FILE"
    done < "$OUTPUT_DIR/test_results_raw.txt"
fi

# Print final summary
echo ""
log_info "=== REGRESSION TEST SUMMARY ==="
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

log_info "Detailed report: $REPORT_FILE"

# Cleanup
rm -f "$OUTPUT_DIR/test_results.lock"

# Exit with appropriate code
if [ $FAILED_TESTS -gt 0 ] || [ $MEMORY_LEAK_TESTS -gt 0 ]; then
    log_error "Some tests failed or had memory leaks"
    exit 1
else
    log_success "All tests passed!"
    exit 0
fi
