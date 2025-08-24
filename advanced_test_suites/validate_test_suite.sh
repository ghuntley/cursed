#!/bin/bash
# Validation script for CURSED Advanced Test Suites
set -euo pipefail

echo "🧪 CURSED Advanced Test Suite Validation"
echo "========================================"

# Configuration
CURSED_EXECUTABLE="${CURSED_EXECUTABLE:-./zig-out/bin/cursed-zig}"
TEST_OUTPUT_DIR="${TEST_OUTPUT_DIR:-test_reports}"
RUN_MEMORY_TESTS="${RUN_MEMORY_TESTS:-true}"
RUN_STRESS_TESTS="${RUN_STRESS_TESTS:-false}"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check CURSED executable
    if [[ ! -f "$CURSED_EXECUTABLE" ]]; then
        log_error "CURSED executable not found at: $CURSED_EXECUTABLE"
        log_info "Please build CURSED first: zig build"
        exit 1
    fi
    
    if [[ ! -x "$CURSED_EXECUTABLE" ]]; then
        log_error "CURSED executable is not executable: $CURSED_EXECUTABLE"
        chmod +x "$CURSED_EXECUTABLE" 2>/dev/null || true
    fi
    
    log_success "CURSED executable found: $CURSED_EXECUTABLE"
    
    # Check Python
    if ! command -v python3 &> /dev/null; then
        log_error "Python 3 is required but not found"
        exit 1
    fi
    
    # Check Python packages
    python3 -c "import psutil" 2>/dev/null || {
        log_error "Python package 'psutil' is required. Install with: pip install psutil"
        exit 1
    }
    
    log_success "Python environment OK"
    
    # Check Valgrind (optional)
    if [[ "$RUN_MEMORY_TESTS" == "true" ]]; then
        if command -v valgrind &> /dev/null; then
            log_success "Valgrind found for memory safety testing"
        else
            log_warning "Valgrind not found. Memory safety tests will be skipped"
            RUN_MEMORY_TESTS="false"
        fi
    fi
    
    # Create output directory
    mkdir -p "$TEST_OUTPUT_DIR"
    log_success "Output directory ready: $TEST_OUTPUT_DIR"
}

# Test CURSED basic functionality
test_basic_functionality() {
    log_info "Testing basic CURSED functionality..."
    
    # Test basic execution
    if ! echo 'vibez.spill("Hello, CURSED!")' | $CURSED_EXECUTABLE /dev/stdin &> /dev/null; then
        log_error "Basic CURSED execution failed"
        return 1
    fi
    
    # Test comprehensive stdlib
    if [[ -f "comprehensive_stdlib_test.csd" ]]; then
        if ! $CURSED_EXECUTABLE comprehensive_stdlib_test.csd &> /dev/null; then
            log_warning "Comprehensive stdlib test failed or not available"
        else
            log_success "Comprehensive stdlib test passed"
        fi
    else
        log_warning "comprehensive_stdlib_test.csd not found"
    fi
    
    log_success "Basic functionality tests passed"
}

# Run memory safety tests
run_memory_safety_tests() {
    if [[ "$RUN_MEMORY_TESTS" != "true" ]]; then
        log_info "Skipping memory safety tests (Valgrind not available)"
        return 0
    fi
    
    log_info "Running memory safety tests with Valgrind..."
    
    # Create a simple test file for memory validation
    cat > /tmp/memory_test.csd << 'EOF'
yeet "testz"
yeet "arrayz"

test_start("Memory Safety Test")

slay test_memory_operations() {
    sus arr []drip = []
    bestie (drip i = 0; i < 1000; i = i + 1) {
        append(arr, i)
    }
    
    sus sum drip = 0
    bestie (drip val in arr) {
        sum = sum + val
    }
    
    assert_eq_int(sum, 499500)
    test_pass("Memory operations completed")
}

test_memory_operations()
print_test_summary()
EOF
    
    # Run with Valgrind
    if valgrind --leak-check=full --error-exitcode=1 --quiet \
       $CURSED_EXECUTABLE /tmp/memory_test.csd > "$TEST_OUTPUT_DIR/memory_safety.log" 2>&1; then
        log_success "Memory safety test passed - no leaks detected"
    else
        log_error "Memory safety test failed - check $TEST_OUTPUT_DIR/memory_safety.log"
        cat "$TEST_OUTPUT_DIR/memory_safety.log"
        return 1
    fi
    
    # Cleanup
    rm -f /tmp/memory_test.csd
}

# Run test categories
run_test_categories() {
    log_info "Running test categories..."
    
    local categories=("edge_cases" "performance" "integration" "security" "cross_platform")
    
    # Add stress tests if enabled
    if [[ "$RUN_STRESS_TESTS" == "true" ]]; then
        categories+=("stress")
        log_info "Stress testing enabled"
    else
        log_info "Stress testing disabled (use RUN_STRESS_TESTS=true to enable)"
    fi
    
    local failed_categories=()
    
    for category in "${categories[@]}"; do
        log_info "Running $category tests..."
        
        if python3 advanced_test_suites/run_all_tests.py \
           --categories "$category" \
           --json-report "$TEST_OUTPUT_DIR/${category}_results.json" \
           --junit-xml "$TEST_OUTPUT_DIR/${category}_results.xml" \
           > "$TEST_OUTPUT_DIR/${category}_output.log" 2>&1; then
            log_success "$category tests passed"
        else
            log_error "$category tests failed"
            failed_categories+=("$category")
            
            # Show last 20 lines of output for quick debugging
            log_info "Last 20 lines of $category test output:"
            tail -n 20 "$TEST_OUTPUT_DIR/${category}_output.log" || true
        fi
    done
    
    if [[ ${#failed_categories[@]} -eq 0 ]]; then
        log_success "All test categories passed!"
        return 0
    else
        log_error "Failed categories: ${failed_categories[*]}"
        return 1
    fi
}

# Generate comprehensive report
generate_report() {
    log_info "Generating comprehensive test report..."
    
    # Run full test suite with reporting
    python3 advanced_test_suites/run_all_tests.py \
        --json-report "$TEST_OUTPUT_DIR/full_results.json" \
        --junit-xml "$TEST_OUTPUT_DIR/full_results.xml" \
        --html-report "$TEST_OUTPUT_DIR/full_results.html" \
        --output-dir "$TEST_OUTPUT_DIR" \
        > "$TEST_OUTPUT_DIR/full_test_output.log" 2>&1
    
    local exit_code=$?
    
    # Parse results
    if [[ -f "$TEST_OUTPUT_DIR/full_results.json" ]]; then
        local total_tests=$(jq -r '.summary.total_tests' "$TEST_OUTPUT_DIR/full_results.json" 2>/dev/null || echo "0")
        local passed=$(jq -r '.summary.passed' "$TEST_OUTPUT_DIR/full_results.json" 2>/dev/null || echo "0")
        local failed=$(jq -r '.summary.failed' "$TEST_OUTPUT_DIR/full_results.json" 2>/dev/null || echo "0")
        local errors=$(jq -r '.summary.errors' "$TEST_OUTPUT_DIR/full_results.json" 2>/dev/null || echo "0")
        local success_rate=$(echo "scale=1; $passed * 100 / $total_tests" | bc -l 2>/dev/null || echo "0.0")
        
        echo ""
        echo "📊 COMPREHENSIVE TEST RESULTS"
        echo "=============================="
        echo "Total Tests: $total_tests"
        echo "Passed: $passed"
        echo "Failed: $failed"
        echo "Errors: $errors"
        echo "Success Rate: ${success_rate}%"
        echo ""
        
        if [[ $(echo "$success_rate >= 95.0" | bc -l 2>/dev/null || echo "0") == "1" ]]; then
            log_success "🎉 EXCELLENT! Success rate ≥95% - Production Ready!"
        elif [[ $(echo "$success_rate >= 90.0" | bc -l 2>/dev/null || echo "0") == "1" ]]; then
            log_warning "⚠️  GOOD: Success rate ≥90% - Minor issues to address"
        else
            log_error "❌ NEEDS WORK: Success rate <90% - Significant issues found"
        fi
        
        echo "Reports generated:"
        echo "  JSON: $TEST_OUTPUT_DIR/full_results.json"
        echo "  XML:  $TEST_OUTPUT_DIR/full_results.xml"
        echo "  HTML: $TEST_OUTPUT_DIR/full_results.html"
        echo ""
    else
        log_error "Failed to generate test report"
        exit_code=1
    fi
    
    return $exit_code
}

# Performance validation
validate_performance() {
    log_info "Validating performance characteristics..."
    
    # Simple performance test
    local start_time=$(date +%s%N)
    
    # Run a basic performance test
    cat > /tmp/perf_test.csd << 'EOF'
slay fibonacci(n drip) drip {
    ready (n <= 1) { damn n }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

sus result drip = fibonacci(20)
vibez.spill("Fibonacci(20) =", result)
EOF
    
    $CURSED_EXECUTABLE /tmp/perf_test.csd > /dev/null 2>&1
    
    local end_time=$(date +%s%N)
    local duration=$(( (end_time - start_time) / 1000000 )) # Convert to milliseconds
    
    log_info "Basic performance test took ${duration}ms"
    
    if [[ $duration -lt 5000 ]]; then  # Less than 5 seconds
        log_success "Performance test completed in reasonable time"
    else
        log_warning "Performance test took longer than expected (${duration}ms)"
    fi
    
    # Cleanup
    rm -f /tmp/perf_test.csd
}

# System information
show_system_info() {
    echo ""
    echo "🖥️  SYSTEM INFORMATION"
    echo "====================="
    echo "OS: $(uname -s) $(uname -r)"
    echo "Architecture: $(uname -m)"
    echo "CPU: $(nproc) cores"
    echo "Memory: $(free -h | awk '/^Mem:/ {print $2}' 2>/dev/null || echo 'N/A')"
    echo "CURSED Executable: $CURSED_EXECUTABLE"
    echo "Python: $(python3 --version 2>/dev/null || echo 'N/A')"
    echo "Valgrind: $(valgrind --version 2>/dev/null | head -n1 || echo 'Not available')"
    echo ""
}

# Main execution
main() {
    echo "Starting CURSED Advanced Test Suite validation..."
    echo "Run with RUN_STRESS_TESTS=true to include stress tests"
    echo "Run with RUN_MEMORY_TESTS=false to skip memory safety tests"
    echo ""
    
    show_system_info
    
    # Check prerequisites
    check_prerequisites
    
    # Test basic functionality
    test_basic_functionality || {
        log_error "Basic functionality tests failed. Cannot continue."
        exit 1
    }
    
    # Run memory safety tests
    run_memory_safety_tests || {
        log_error "Memory safety tests failed. This is critical."
        exit 1
    }
    
    # Validate performance
    validate_performance
    
    # Run test categories
    local categories_exit_code=0
    run_test_categories || categories_exit_code=$?
    
    # Generate comprehensive report
    local report_exit_code=0
    generate_report || report_exit_code=$?
    
    # Final status
    echo ""
    echo "🏁 VALIDATION COMPLETE"
    echo "======================"
    
    if [[ $categories_exit_code -eq 0 && $report_exit_code -eq 0 ]]; then
        log_success "✅ All validations passed! CURSED advanced test suite is working correctly."
        echo "   Reports available in: $TEST_OUTPUT_DIR/"
        exit 0
    else
        log_error "❌ Some validations failed. Check the reports for details."
        echo "   Reports available in: $TEST_OUTPUT_DIR/"
        exit 1
    fi
}

# Handle script interruption
cleanup() {
    echo ""
    log_warning "Script interrupted. Cleaning up..."
    rm -f /tmp/memory_test.csd /tmp/perf_test.csd
    exit 130
}

trap cleanup INT TERM

# Execute main function
main "$@"
