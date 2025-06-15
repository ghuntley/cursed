#!/bin/bash
# Comprehensive LTO Test Runner for CURSED
# 
# Runs all LTO-related tests including unit tests, integration tests,
# performance tests, and generates comprehensive reports.

set -euo pipefail

# Script configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
REPORT_DIR="$PROJECT_ROOT/test_results/lto"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

# Test categories
UNIT_TESTS=("lto_comprehensive_test" "llvm_lto_integration_test")
PERFORMANCE_TESTS=("lto_benchmarks")
ALL_TESTS=("${UNIT_TESTS[@]}" "${PERFORMANCE_TESTS[@]}")

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1" >&2
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1" >&2
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1" >&2
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

# Usage information
show_usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Comprehensive LTO test runner for the CURSED programming language.

OPTIONS:
    --quick         Run only quick unit tests
    --performance   Run only performance/benchmark tests
    --unit          Run only unit tests
    --integration   Run only integration tests
    --all           Run all tests (default)
    --verbose       Enable verbose output
    --report        Generate detailed HTML report
    --coverage      Generate code coverage report
    --timeout SECS  Set test timeout (default: 300 seconds)
    --jobs N        Number of parallel test jobs (default: auto)
    --help          Show this help message

EXAMPLES:
    $0 --quick                  # Quick validation
    $0 --performance --verbose  # Performance testing with details
    $0 --unit --coverage        # Unit tests with coverage
    $0 --all --report           # Full test suite with HTML report

EOF
}

# Parse command line arguments
QUICK_MODE=false
PERFORMANCE_MODE=false
UNIT_MODE=false
INTEGRATION_MODE=false
VERBOSE=false
GENERATE_REPORT=false
GENERATE_COVERAGE=false
TEST_TIMEOUT=300
PARALLEL_JOBS=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --quick)
            QUICK_MODE=true
            shift
            ;;
        --performance)
            PERFORMANCE_MODE=true
            shift
            ;;
        --unit)
            UNIT_MODE=true
            shift
            ;;
        --integration)
            INTEGRATION_MODE=true
            shift
            ;;
        --all)
            # Default behavior, no flag needed
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --report)
            GENERATE_REPORT=true
            shift
            ;;
        --coverage)
            GENERATE_COVERAGE=true
            shift
            ;;
        --timeout)
            TEST_TIMEOUT="$2"
            shift 2
            ;;
        --jobs)
            PARALLEL_JOBS="$2"
            shift 2
            ;;
        --help)
            show_usage
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Determine which tests to run
TESTS_TO_RUN=()

if [[ "$QUICK_MODE" == true ]]; then
    TESTS_TO_RUN=("lto_comprehensive_test")
elif [[ "$PERFORMANCE_MODE" == true ]]; then
    TESTS_TO_RUN=("${PERFORMANCE_TESTS[@]}")
elif [[ "$UNIT_MODE" == true ]]; then
    TESTS_TO_RUN=("lto_comprehensive_test")
elif [[ "$INTEGRATION_MODE" == true ]]; then
    TESTS_TO_RUN=("llvm_lto_integration_test")
else
    # Default: run all tests
    TESTS_TO_RUN=("${ALL_TESTS[@]}")
fi

# Setup test environment
setup_test_environment() {
    log_info "Setting up LTO test environment..."
    
    # Create report directory
    mkdir -p "$REPORT_DIR"
    
    # Create test output directory
    mkdir -p "$PROJECT_ROOT/target/lto_test"
    
    # Source the linking fix if available
    if [[ -f "$PROJECT_ROOT/fix_linking.sh" ]]; then
        log_info "Applying linking fixes for Nix environment..."
        source "$PROJECT_ROOT/fix_linking.sh"
    fi
    
    # Set test-specific environment variables
    export RUST_BACKTRACE=1
    export RUST_LOG=${RUST_LOG:-"cursed=debug,info"}
    
    # Set parallel job count
    if [[ -n "$PARALLEL_JOBS" ]]; then
        export CARGO_BUILD_JOBS="$PARALLEL_JOBS"
    fi
    
    log_success "Test environment setup complete"
}

# Run a single test with proper error handling
run_single_test() {
    local test_name="$1"
    local test_type="$2"
    local log_file="$REPORT_DIR/${test_name}_${TIMESTAMP}.log"
    
    log_info "Running $test_type test: $test_name"
    
    local start_time=$(date +%s)
    local test_cmd=""
    local test_result=0
    
    # Determine test command based on type
    case "$test_type" in
        "unit"|"integration")
            if [[ "$GENERATE_COVERAGE" == true ]]; then
                test_cmd="cargo tarpaulin --test $test_name --out xml --output-dir $REPORT_DIR"
            else
                test_cmd="cargo test --test $test_name"
            fi
            ;;
        "benchmark")
            test_cmd="cargo bench --bench $test_name"
            ;;
        *)
            log_error "Unknown test type: $test_type"
            return 1
            ;;
    esac
    
    # Add verbose flag if requested
    if [[ "$VERBOSE" == true ]]; then
        test_cmd="$test_cmd -- --nocapture"
    fi
    
    # Run the test with timeout
    if timeout "$TEST_TIMEOUT" bash -c "$test_cmd" > "$log_file" 2>&1; then
        test_result=0
    else
        test_result=$?
        if [[ $test_result -eq 124 ]]; then
            log_error "Test $test_name timed out after $TEST_TIMEOUT seconds"
        else
            log_error "Test $test_name failed with exit code $test_result"
        fi
    fi
    
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    # Report results
    if [[ $test_result -eq 0 ]]; then
        log_success "Test $test_name completed successfully in ${duration}s"
    else
        log_error "Test $test_name failed in ${duration}s"
        if [[ "$VERBOSE" == true ]]; then
            log_error "Error output:"
            tail -20 "$log_file" | sed 's/^/  /'
        fi
    fi
    
    # Record results for report generation
    echo "$test_name,$test_type,$test_result,$duration" >> "$REPORT_DIR/test_results.csv"
    
    return $test_result
}

# Run all selected tests
run_tests() {
    log_info "Running LTO tests..."
    
    # Initialize results file
    echo "test_name,test_type,result,duration" > "$REPORT_DIR/test_results.csv"
    
    local total_tests=${#TESTS_TO_RUN[@]}
    local passed_tests=0
    local failed_tests=0
    local test_number=0
    
    for test_name in "${TESTS_TO_RUN[@]}"; do
        test_number=$((test_number + 1))
        log_info "Running test $test_number/$total_tests: $test_name"
        
        # Determine test type
        local test_type="unit"
        if [[ "$test_name" == *"integration"* ]]; then
            test_type="integration"
        elif [[ "$test_name" == *"benchmark"* ]]; then
            test_type="benchmark"
        fi
        
        if run_single_test "$test_name" "$test_type"; then
            passed_tests=$((passed_tests + 1))
        else
            failed_tests=$((failed_tests + 1))
        fi
        
        echo # Add spacing between tests
    done
    
    # Summary
    log_info "Test execution completed"
    log_info "Total tests: $total_tests"
    log_success "Passed: $passed_tests"
    if [[ $failed_tests -gt 0 ]]; then
        log_error "Failed: $failed_tests"
    else
        log_success "Failed: $failed_tests"
    fi
    
    return $failed_tests
}

# Generate comprehensive test report
generate_test_report() {
    if [[ "$GENERATE_REPORT" != true ]]; then
        return 0
    fi
    
    log_info "Generating comprehensive test report..."
    
    local report_file="$REPORT_DIR/lto_test_report_${TIMESTAMP}.html"
    
    cat > "$report_file" << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CURSED LTO Test Report</title>
    <style>
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; margin: 40px; background-color: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; background: white; padding: 30px; border-radius: 10px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #2c3e50; border-bottom: 3px solid #3498db; padding-bottom: 10px; }
        h2 { color: #34495e; margin-top: 30px; }
        .summary { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin: 20px 0; }
        .stat-card { background: #ecf0f1; padding: 20px; border-radius: 8px; text-align: center; }
        .stat-number { font-size: 2em; font-weight: bold; color: #2c3e50; }
        .stat-label { color: #7f8c8d; font-size: 0.9em; }
        .passed { color: #27ae60; }
        .failed { color: #e74c3c; }
        .warning { color: #f39c12; }
        table { width: 100%; border-collapse: collapse; margin: 20px 0; }
        th, td { padding: 12px; text-align: left; border-bottom: 1px solid #ddd; }
        th { background-color: #3498db; color: white; }
        tr:nth-child(even) { background-color: #f2f2f2; }
        .test-passed { background-color: #d5f4e6; }
        .test-failed { background-color: #ffeaea; }
        .footer { margin-top: 40px; padding-top: 20px; border-top: 1px solid #ddd; text-align: center; color: #7f8c8d; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🔧 CURSED LTO Test Report</h1>
        <p><strong>Generated:</strong> $(date)</p>
        <p><strong>Test Suite:</strong> Link-Time Optimization (LTO) System</p>
EOF
    
    # Add test summary
    if [[ -f "$REPORT_DIR/test_results.csv" ]]; then
        local total_tests=$(tail -n +2 "$REPORT_DIR/test_results.csv" | wc -l)
        local passed_tests=$(tail -n +2 "$REPORT_DIR/test_results.csv" | awk -F',' '$3==0' | wc -l)
        local failed_tests=$(tail -n +2 "$REPORT_DIR/test_results.csv" | awk -F',' '$3!=0' | wc -l)
        local total_duration=$(tail -n +2 "$REPORT_DIR/test_results.csv" | awk -F',' '{sum+=$4} END {print sum}')
        
        cat >> "$report_file" << EOF
        
        <div class="summary">
            <div class="stat-card">
                <div class="stat-number">$total_tests</div>
                <div class="stat-label">Total Tests</div>
            </div>
            <div class="stat-card">
                <div class="stat-number passed">$passed_tests</div>
                <div class="stat-label">Passed</div>
            </div>
            <div class="stat-card">
                <div class="stat-number failed">$failed_tests</div>
                <div class="stat-label">Failed</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">${total_duration}s</div>
                <div class="stat-label">Total Duration</div>
            </div>
        </div>
        
        <h2>📊 Test Results</h2>
        <table>
            <thead>
                <tr>
                    <th>Test Name</th>
                    <th>Type</th>
                    <th>Result</th>
                    <th>Duration</th>
                </tr>
            </thead>
            <tbody>
EOF
        
        # Add test results
        tail -n +2 "$REPORT_DIR/test_results.csv" | while IFS=',' read -r test_name test_type result duration; do
            local row_class=""
            local result_text=""
            if [[ "$result" == "0" ]]; then
                row_class="test-passed"
                result_text="✅ PASSED"
            else
                row_class="test-failed"
                result_text="❌ FAILED"
            fi
            
            cat >> "$report_file" << EOF
                <tr class="$row_class">
                    <td>$test_name</td>
                    <td>$test_type</td>
                    <td>$result_text</td>
                    <td>${duration}s</td>
                </tr>
EOF
        done
    fi
    
    cat >> "$report_file" << 'EOF'
            </tbody>
        </table>
        
        <h2>🎯 LTO System Features Tested</h2>
        <ul>
            <li><strong>Cross-Module Optimization:</strong> Inter-procedural analysis and optimization</li>
            <li><strong>Dead Code Elimination:</strong> Whole-program dead code detection and removal</li>
            <li><strong>Function Inlining:</strong> Cross-module function inlining optimization</li>
            <li><strong>Global Variable Optimization:</strong> Global variable merging and optimization</li>
            <li><strong>Constant Propagation:</strong> Cross-module constant propagation</li>
            <li><strong>Devirtualization:</strong> Virtual function call optimization</li>
            <li><strong>Thin LTO:</strong> Fast parallel link-time optimization</li>
            <li><strong>Full LTO:</strong> Maximum whole-program optimization</li>
            <li><strong>Build Integration:</strong> Seamless integration with build system</li>
            <li><strong>Performance Analysis:</strong> Optimization effectiveness measurement</li>
        </ul>
        
        <h2>📈 Performance Characteristics</h2>
        <p>The LTO system has been tested for:</p>
        <ul>
            <li>Scalability with increasing number of modules</li>
            <li>Optimization effectiveness across different code patterns</li>
            <li>Memory usage during optimization</li>
            <li>Parallel processing efficiency</li>
            <li>Cache effectiveness for incremental builds</li>
        </ul>
        
        <div class="footer">
            <p>CURSED Programming Language - LTO Test Suite</p>
            <p>Report generated by automated testing framework</p>
        </div>
    </div>
</body>
</html>
EOF
    
    log_success "Test report generated: $report_file"
}

# Generate code coverage report
generate_coverage_report() {
    if [[ "$GENERATE_COVERAGE" != true ]]; then
        return 0
    fi
    
    log_info "Generating code coverage report..."
    
    # Check if tarpaulin is installed
    if ! command -v cargo-tarpaulin &> /dev/null; then
        log_warning "cargo-tarpaulin not found, skipping coverage report"
        log_info "Install with: cargo install cargo-tarpaulin"
        return 0
    fi
    
    # Generate coverage for LTO modules
    local coverage_cmd="cargo tarpaulin"
    coverage_cmd="$coverage_cmd --out html --output-dir $REPORT_DIR"
    coverage_cmd="$coverage_cmd --include-tests"
    coverage_cmd="$coverage_cmd --packages cursed"
    coverage_cmd="$coverage_cmd --exclude-files 'tests/*' 'benches/*'"
    
    if timeout $((TEST_TIMEOUT * 2)) bash -c "$coverage_cmd" > "$REPORT_DIR/coverage.log" 2>&1; then
        log_success "Coverage report generated in $REPORT_DIR"
    else
        log_error "Failed to generate coverage report"
        return 1
    fi
}

# Cleanup function
cleanup() {
    log_info "Cleaning up test artifacts..."
    
    # Clean up temporary test files
    rm -rf "$PROJECT_ROOT/target/lto_test" 2>/dev/null || true
    
    # Compress old log files
    if [[ -d "$REPORT_DIR" ]]; then
        find "$REPORT_DIR" -name "*.log" -mtime +7 -exec gzip {} \; 2>/dev/null || true
    fi
}

# Main execution
main() {
    log_info "Starting CURSED LTO comprehensive test suite..."
    
    # Change to project root
    cd "$PROJECT_ROOT"
    
    # Setup environment
    setup_test_environment
    
    # Run tests
    local test_exit_code=0
    if ! run_tests; then
        test_exit_code=1
    fi
    
    # Generate reports
    generate_test_report
    generate_coverage_report
    
    # Cleanup
    cleanup
    
    # Final summary
    echo
    log_info "═══════════════════════════════════════"
    log_info "LTO Test Suite Execution Complete"
    log_info "═══════════════════════════════════════"
    
    if [[ $test_exit_code -eq 0 ]]; then
        log_success "All tests passed successfully! 🎉"
        log_info "Reports available in: $REPORT_DIR"
    else
        log_error "Some tests failed. Check the reports for details."
        log_info "Reports available in: $REPORT_DIR"
    fi
    
    exit $test_exit_code
}

# Run main function
main "$@"
