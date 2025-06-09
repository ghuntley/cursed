#!/bin/bash
# Bootstrap Test Runner Script
#
# This script runs the comprehensive bootstrap testing pipeline
# and generates detailed reports.

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TEST_OUTPUT_DIR="$PROJECT_ROOT/test_results/bootstrap"
VERBOSE=${VERBOSE:-false}
QUICK_MODE=${QUICK_MODE:-false}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')] $1${NC}"
}

log_success() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')] ✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}[$(date +'%Y-%m-%d %H:%M:%S')] ⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] ❌ $1${NC}"
}

# Usage information
usage() {
    cat << EOF
Bootstrap Test Runner

Usage: $0 [OPTIONS]

OPTIONS:
    -h, --help          Show this help message
    -v, --verbose       Enable verbose output
    -q, --quick         Run quick test suite (subset of tests)
    --clean             Clean test outputs before running
    --report-only       Generate reports from existing test results
    --category CATEGORY Run only specific test category
                        (minimal_subset, stage2_compiler, self_compilation, 
                         performance, regression, ci_integration, memory_usage)

ENVIRONMENT VARIABLES:
    VERBOSE             Set to 'true' to enable verbose output
    QUICK_MODE          Set to 'true' to run quick test suite
    TEST_TIMEOUT        Test timeout in seconds (default: 300)
    KEEP_OUTPUTS        Set to 'true' to keep test outputs

EXAMPLES:
    $0                              # Run full test suite
    $0 --quick                      # Run quick test suite
    $0 --verbose                    # Run with verbose output
    $0 --category minimal_subset    # Run only minimal subset tests
    $0 --clean                      # Clean and run full suite

EOF
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                usage
                exit 0
                ;;
            -v|--verbose)
                VERBOSE=true
                shift
                ;;
            -q|--quick)
                QUICK_MODE=true
                shift
                ;;
            --clean)
                CLEAN_OUTPUTS=true
                shift
                ;;
            --report-only)
                REPORT_ONLY=true
                shift
                ;;
            --category)
                if [[ -n $2 ]]; then
                    TEST_CATEGORY="$2"
                    shift 2
                else
                    log_error "Category argument required"
                    exit 1
                fi
                ;;
            *)
                log_error "Unknown option: $1"
                usage
                exit 1
                ;;
        esac
    done
}

# Setup test environment
setup_environment() {
    log "Setting up bootstrap test environment"
    
    # Create output directories
    mkdir -p "$TEST_OUTPUT_DIR"
    mkdir -p "$TEST_OUTPUT_DIR/logs"
    mkdir -p "$TEST_OUTPUT_DIR/reports"
    mkdir -p "$TEST_OUTPUT_DIR/artifacts"
    
    # Clean outputs if requested
    if [[ "$CLEAN_OUTPUTS" == "true" ]]; then
        log "Cleaning previous test outputs"
        rm -rf "$TEST_OUTPUT_DIR"/*
        mkdir -p "$TEST_OUTPUT_DIR"/{logs,reports,artifacts}
    fi
    
    # Check prerequisites
    check_prerequisites
}

# Check prerequisites
check_prerequisites() {
    log "Checking prerequisites"
    
    # Check if cargo is available
    if ! command -v cargo &> /dev/null; then
        log_error "cargo not found - please install Rust"
        exit 1
    fi
    
    # Check if the project builds
    if [[ "$QUICK_MODE" != "true" ]]; then
        log "Building project"
        cd "$PROJECT_ROOT"
        if ! cargo build --release > "$TEST_OUTPUT_DIR/logs/build.log" 2>&1; then
            log_error "Project build failed - check $TEST_OUTPUT_DIR/logs/build.log"
            exit 1
        fi
        log_success "Project build successful"
    fi
    
    # Check if cursed binary exists
    CURSED_BINARY="$PROJECT_ROOT/target/release/cursed"
    if [[ ! -f "$CURSED_BINARY" ]] && [[ "$QUICK_MODE" != "true" ]]; then
        CURSED_BINARY="$PROJECT_ROOT/target/debug/cursed"
        if [[ ! -f "$CURSED_BINARY" ]]; then
            log_warning "CURSED binary not found - some tests may fail"
        fi
    fi
    
    log_success "Prerequisites check completed"
}

# Run test category
run_test_category() {
    local category=$1
    local test_name=$2
    
    log "Running $category tests"
    
    local log_file="$TEST_OUTPUT_DIR/logs/${category}.log"
    local cargo_args="--test $test_name"
    
    if [[ "$VERBOSE" == "true" ]]; then
        cargo_args="$cargo_args -- --nocapture"
    fi
    
    # Set environment variables for tests
    export BOOTSTRAP_TEST_OUTPUT_DIR="$TEST_OUTPUT_DIR"
    export BOOTSTRAP_CURSED_BINARY="$CURSED_BINARY"
    export RUST_LOG=${RUST_LOG:-info}
    
    cd "$PROJECT_ROOT"
    
    if cargo test $cargo_args > "$log_file" 2>&1; then
        log_success "$category tests passed"
        return 0
    else
        log_error "$category tests failed - check $log_file"
        return 1
    fi
}

# Run minimal subset tests
run_minimal_subset_tests() {
    run_test_category "minimal_subset" "bootstrap::minimal_subset"
}

# Run stage 2 compiler tests
run_stage2_compiler_tests() {
    run_test_category "stage2_compiler" "bootstrap::stage2_compiler"
}

# Run self-compilation tests
run_self_compilation_tests() {
    run_test_category "self_compilation" "bootstrap::self_compilation"
}

# Run performance benchmarks
run_performance_tests() {
    run_test_category "performance" "bootstrap::performance_benchmarks"
}

# Run regression tests
run_regression_tests() {
    run_test_category "regression" "bootstrap::regression_tests"
}

# Run CI integration tests
run_ci_integration_tests() {
    run_test_category "ci_integration" "bootstrap::ci_integration"
}

# Run memory usage tests
run_memory_usage_tests() {
    run_test_category "memory_usage" "bootstrap::memory_usage"
}

# Run comprehensive test suite
run_comprehensive_tests() {
    run_test_category "comprehensive" "bootstrap_comprehensive_test"
}

# Run all tests
run_all_tests() {
    log "Starting comprehensive bootstrap test suite"
    
    local failed_categories=()
    local start_time=$(date +%s)
    
    # Define test categories
    local categories=(
        "minimal_subset:run_minimal_subset_tests"
        "stage2_compiler:run_stage2_compiler_tests"
        "self_compilation:run_self_compilation_tests"
        "performance:run_performance_tests"
        "regression:run_regression_tests"
        "ci_integration:run_ci_integration_tests"
        "memory_usage:run_memory_usage_tests"
        "comprehensive:run_comprehensive_tests"
    )
    
    # Filter categories if specific category requested
    if [[ -n "$TEST_CATEGORY" ]]; then
        local filtered_categories=()
        for category in "${categories[@]}"; do
            if [[ "$category" == "$TEST_CATEGORY:"* ]]; then
                filtered_categories+=("$category")
            fi
        done
        categories=("${filtered_categories[@]}")
        
        if [[ ${#categories[@]} -eq 0 ]]; then
            log_error "Unknown test category: $TEST_CATEGORY"
            exit 1
        fi
    fi
    
    # Run each category
    for category_func in "${categories[@]}"; do
        local category_name="${category_func%:*}"
        local func_name="${category_func#*:}"
        
        log "Running $category_name test category"
        
        if [[ "$QUICK_MODE" == "true" ]] && [[ "$category_name" == "performance" || "$category_name" == "memory_usage" ]]; then
            log_warning "Skipping $category_name tests in quick mode"
            continue
        fi
        
        if $func_name; then
            log_success "$category_name tests completed successfully"
        else
            log_error "$category_name tests failed"
            failed_categories+=("$category_name")
        fi
    done
    
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    # Report results
    log "Test execution completed in ${duration}s"
    
    if [[ ${#failed_categories[@]} -eq 0 ]]; then
        log_success "All bootstrap test categories passed! 🎉"
        return 0
    else
        log_error "Failed test categories: ${failed_categories[*]}"
        return 1
    fi
}

# Generate test report
generate_report() {
    log "Generating test report"
    
    local report_file="$TEST_OUTPUT_DIR/reports/bootstrap_test_report.md"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    
    cat > "$report_file" << EOF
# Bootstrap Test Report

**Generated:** $timestamp  
**Test Suite:** CURSED Bootstrap Compiler  
**Mode:** $([ "$QUICK_MODE" == "true" ] && echo "Quick" || echo "Comprehensive")

## Summary

EOF
    
    # Add test results summary
    local total_logs=$(find "$TEST_OUTPUT_DIR/logs" -name "*.log" | wc -l)
    local failed_logs=$(grep -l "FAILED\|failed\|error" "$TEST_OUTPUT_DIR/logs"/*.log 2>/dev/null | wc -l || echo 0)
    local passed_logs=$((total_logs - failed_logs))
    
    cat >> "$report_file" << EOF
- **Total Test Categories:** $total_logs
- **Passed:** $passed_logs
- **Failed:** $failed_logs
- **Success Rate:** $(( passed_logs * 100 / total_logs ))%

## Test Categories

EOF
    
    # Add details for each category
    for log_file in "$TEST_OUTPUT_DIR/logs"/*.log; do
        if [[ -f "$log_file" ]]; then
            local category=$(basename "$log_file" .log)
            local status="✅ PASSED"
            
            if grep -q "FAILED\|failed\|error" "$log_file"; then
                status="❌ FAILED"
            fi
            
            cat >> "$report_file" << EOF
### $category

**Status:** $status

\`\`\`
$(tail -20 "$log_file")
\`\`\`

EOF
        fi
    done
    
    # Add artifacts section
    cat >> "$report_file" << EOF
## Artifacts

- **Log Files:** \`test_results/bootstrap/logs/\`
- **Test Reports:** \`test_results/bootstrap/reports/\`
- **Test Artifacts:** \`test_results/bootstrap/artifacts/\`

## Environment

- **OS:** $(uname -s)
- **Architecture:** $(uname -m)
- **Rust Version:** $(rustc --version)
- **Cargo Version:** $(cargo --version)

EOF
    
    log_success "Test report generated: $report_file"
}

# Cleanup function
cleanup() {
    if [[ "$KEEP_OUTPUTS" != "true" ]]; then
        log "Cleaning up temporary test files"
        # Add cleanup logic here if needed
    fi
}

# Main execution
main() {
    trap cleanup EXIT
    
    log "🚀 Bootstrap Test Runner Starting"
    log "Project: CURSED Bootstrap Compiler"
    log "Mode: $([ "$QUICK_MODE" == "true" ] && echo "Quick" || echo "Comprehensive")"
    
    parse_args "$@"
    
    if [[ "$REPORT_ONLY" == "true" ]]; then
        generate_report
        exit 0
    fi
    
    setup_environment
    
    if run_all_tests; then
        generate_report
        log_success "🎉 Bootstrap test suite completed successfully!"
        exit 0
    else
        generate_report
        log_error "💥 Bootstrap test suite failed - check reports for details"
        exit 1
    fi
}

# Run main function if script is executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
