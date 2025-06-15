#!/bin/bash

# Comprehensive Process Management and IPC Test Runner for CURSED
# This script runs all process management and IPC tests with proper environment setup

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Test categories
BASIC_TESTS=(
    "process_ipc_integration_comprehensive_test"
    "enhanced_process_management_integration_test"
    "process_management_comprehensive_test"
    "ipc_comprehensive_test"
    "exec_slay_integration_test"
    "exec_vibez_comprehensive_test"
)

ENHANCED_TESTS=(
    "process_management_comprehensive_integration_test"
    "ipc_integration_test"
    "ipc_integration_advanced_test"
    "process_runtime_test"
    "safe_process_management_test"
)

STRESS_TESTS=(
    "process_management_stress_test"
    "ipc_stress_test"
    "process_ipc_performance_test"
)

# Configuration
TIMEOUT_SECONDS=${TIMEOUT_SECONDS:-300}
JOBS=${JOBS:-$(nproc)}
VERBOSE=${VERBOSE:-false}
DRY_RUN=${DRY_RUN:-false}
REPORT_FILE=""
FILTER=""
EXCLUDED_TESTS=""

# Statistics
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0
TESTS_SKIPPED=0
START_TIME=""

# Nix linking fix for compatibility
if [ -f "$PROJECT_ROOT/fix_linking.sh" ]; then
    CARGO_PREFIX="$PROJECT_ROOT/fix_linking.sh"
else
    CARGO_PREFIX=""
fi

# Function to print colored output
print_color() {
    local color=$1
    local message=$2
    echo -e "${color}[PROCESS-IPC-TEST]${NC} ${message}"
}

# Function to print status
print_status() {
    print_color "$CYAN" "$1"
}

# Function to print success
print_success() {
    print_color "$GREEN" "✅ $1"
}

# Function to print warning
print_warning() {
    print_color "$YELLOW" "⚠️  $1"
}

# Function to print error
print_error() {
    print_color "$RED" "❌ $1"
}

# Function to print section header
print_section() {
    echo
    print_color "$PURPLE" "═══════════════════════════════════════════════════════════════"
    print_color "$PURPLE" "  $1"
    print_color "$PURPLE" "═══════════════════════════════════════════════════════════════"
    echo
}

# Function to print usage
print_usage() {
    cat << EOF
Comprehensive Process Management and IPC Test Runner

USAGE:
    $0 [OPTIONS] [COMMAND]

COMMANDS:
    basic               Run basic process and IPC tests
    enhanced            Run enhanced functionality tests
    stress              Run stress and performance tests (ignored by default)
    all                 Run all tests (default)
    integration         Run integration tests only
    unit                Run unit tests only
    
OPTIONS:
    -h, --help          Show this help message
    -v, --verbose       Enable verbose output
    -j, --jobs JOBS     Number of parallel jobs (default: $(nproc))
    -t, --timeout SEC   Test timeout in seconds (default: 300)
    --dry-run           Show what would be run without executing
    --report FILE       Generate test report to file
    --filter PATTERN    Only run tests matching pattern
    --exclude PATTERN   Exclude tests matching pattern
    --ignored           Include ignored tests (stress tests)
    --no-linking-fix    Don't use linking fix (for non-Nix environments)

EXAMPLES:
    $0                          # Run all basic tests
    $0 all                      # Run all tests
    $0 stress --ignored         # Run stress tests
    $0 --filter "ipc"           # Run only IPC-related tests
    $0 --exclude "stress"       # Exclude stress tests
    $0 --report test_report.md  # Generate report

ENVIRONMENT VARIABLES:
    TIMEOUT_SECONDS     Test timeout (default: 300)
    JOBS               Number of parallel jobs
    VERBOSE            Enable verbose output (true/false)
    DRY_RUN            Dry run mode (true/false)

EOF
}

# Function to run a single test
run_test() {
    local test_name="$1"
    local test_desc="$2"
    local cargo_args=("${@:3}")
    
    ((TESTS_RUN++))
    
    # Check if test should be excluded
    if [[ -n "$EXCLUDED_TESTS" && "$test_name" =~ $EXCLUDED_TESTS ]]; then
        print_warning "Excluded: $test_desc"
        ((TESTS_SKIPPED++))
        return 0
    fi
    
    # Check filter
    if [[ -n "$FILTER" && ! "$test_name" =~ $FILTER ]]; then
        print_warning "Filtered out: $test_desc"
        ((TESTS_SKIPPED++))
        return 0
    fi
    
    if [ "$DRY_RUN" = "true" ]; then
        print_status "Would run: $test_desc"
        return 0
    fi
    
    print_status "Running: $test_desc"
    
    local start_time=$(date +%s)
    local result=0
    
    # Prepare cargo command
    local cargo_cmd=()
    if [ -n "$CARGO_PREFIX" ]; then
        cargo_cmd+=("$CARGO_PREFIX")
    fi
    cargo_cmd+=("cargo")
    cargo_cmd+=("${cargo_args[@]}")
    
    # Add timeout
    cargo_cmd=(timeout "${TIMEOUT_SECONDS}s" "${cargo_cmd[@]}")
    
    if [ "$VERBOSE" = "true" ]; then
        print_status "Command: ${cargo_cmd[*]}"
        "${cargo_cmd[@]}" || result=$?
    else
        "${cargo_cmd[@]}" >/dev/null 2>&1 || result=$?
    fi
    
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    if [ $result -eq 0 ]; then
        print_success "$test_desc (${duration}s)"
        ((TESTS_PASSED++))
    elif [ $result -eq 124 ]; then
        print_error "$test_desc - TIMEOUT after ${TIMEOUT_SECONDS}s"
        ((TESTS_FAILED++))
    else
        print_error "$test_desc - FAILED (exit code: $result, ${duration}s)"
        ((TESTS_FAILED++))
    fi
    
    return $result
}

# Function to run basic tests
run_basic_tests() {
    print_section "Basic Process and IPC Tests"
    
    local failed=0
    for test_name in "${BASIC_TESTS[@]}"; do
        run_test "$test_name" "Basic Test: $test_name" test --test "$test_name" || ((failed++))
    done
    
    return $failed
}

# Function to run enhanced tests
run_enhanced_tests() {
    print_section "Enhanced Process and IPC Tests"
    
    local failed=0
    for test_name in "${ENHANCED_TESTS[@]}"; do
        run_test "$test_name" "Enhanced Test: $test_name" test --test "$test_name" || ((failed++))
    done
    
    return $failed
}

# Function to run stress tests
run_stress_tests() {
    print_section "Stress and Performance Tests"
    
    local failed=0
    for test_name in "${STRESS_TESTS[@]}"; do
        run_test "$test_name" "Stress Test: $test_name" test --test "$test_name" -- --ignored || ((failed++))
    done
    
    return $failed
}

# Function to run integration tests
run_integration_tests() {
    print_section "Integration Tests"
    
    local failed=0
    
    # Integration-specific tests
    run_test "process_integration" "Process Integration Tests" \
        test --test process_management_comprehensive_integration_test || ((failed++))
    
    run_test "ipc_integration" "IPC Integration Tests" \
        test --test ipc_integration_test || ((failed++))
    
    run_test "comprehensive_integration" "Comprehensive Integration Tests" \
        test --test process_ipc_integration_comprehensive_test || ((failed++))
    
    return $failed
}

# Function to run unit tests
run_unit_tests() {
    print_section "Unit Tests"
    
    local failed=0
    
    # Unit tests within the modules
    run_test "process_unit" "Process Module Unit Tests" \
        test --lib process || ((failed++))
    
    run_test "ipc_unit" "IPC Module Unit Tests" \
        test --lib ipc || ((failed++))
    
    return $failed
}

# Function to check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check if we're in the right directory
    if [ ! -f "$PROJECT_ROOT/Cargo.toml" ]; then
        print_error "Not in CURSED project root directory"
        exit 1
    fi
    
    # Check if cargo is available
    if ! command -v cargo >/dev/null 2>&1; then
        print_error "cargo not found in PATH"
        exit 1
    fi
    
    # Check if linking fix is available in Nix environment
    if [ -n "$CARGO_PREFIX" ] && [ ! -f "$CARGO_PREFIX" ]; then
        print_warning "Linking fix not found: $CARGO_PREFIX"
        CARGO_PREFIX=""
    fi
    
    # Check for required test files
    local missing_tests=()
    for test_name in "${BASIC_TESTS[@]}"; do
        if [ ! -f "$PROJECT_ROOT/tests/${test_name}.rs" ]; then
            missing_tests+=("$test_name")
        fi
    done
    
    if [ ${#missing_tests[@]} -gt 0 ]; then
        print_warning "Some test files are missing: ${missing_tests[*]}"
    fi
    
    print_success "Prerequisites check completed"
}

# Function to generate report
generate_report() {
    local report_file="$1"
    local end_time=$(date +%s)
    local total_duration=$((end_time - $(date -d "$START_TIME" +%s)))
    
    cat > "$report_file" << EOF
# Comprehensive Process Management and IPC Test Report

**Generated:** $(date)
**Duration:** ${total_duration}s
**Command:** $0 $*

## Summary

- **Tests Run:** $TESTS_RUN
- **Tests Passed:** $TESTS_PASSED
- **Tests Failed:** $TESTS_FAILED
- **Tests Skipped:** $TESTS_SKIPPED
- **Success Rate:** $(( TESTS_RUN > 0 ? (TESTS_PASSED * 100) / TESTS_RUN : 0 ))%

## Test Categories

### Basic Tests
Process management and IPC fundamental functionality tests.

### Enhanced Tests
Advanced features including integration with enhanced ExecSlay and ExecVibez.

### Stress Tests
Performance and reliability tests under high load.

### Integration Tests
End-to-end functionality validation.

### Unit Tests
Individual module and component tests.

## Environment

- **Project Root:** $PROJECT_ROOT
- **Cargo Prefix:** ${CARGO_PREFIX:-"None"}
- **Jobs:** $JOBS
- **Timeout:** ${TIMEOUT_SECONDS}s
- **Filter:** ${FILTER:-"None"}
- **Excluded:** ${EXCLUDED_TESTS:-"None"}

## Results

EOF

    if [ $TESTS_FAILED -eq 0 ]; then
        echo "✅ **All tests passed successfully!**" >> "$report_file"
    else
        echo "❌ **$TESTS_FAILED test(s) failed.**" >> "$report_file"
    fi
    
    echo "" >> "$report_file"
    echo "## Recommendations" >> "$report_file"
    echo "" >> "$report_file"
    
    if [ $TESTS_FAILED -gt 0 ]; then
        echo "- Review failed tests and fix underlying issues" >> "$report_file"
        echo "- Run tests with --verbose for detailed output" >> "$report_file"
        echo "- Check system resources and environment setup" >> "$report_file"
    else
        echo "- Consider running stress tests with --ignored flag" >> "$report_file"
        echo "- Process and IPC system is ready for production use" >> "$report_file"
    fi
    
    print_success "Report generated: $report_file"
}

# Function to print final summary
print_summary() {
    local end_time=$(date +%s)
    local total_duration=$((end_time - $(date -d "$START_TIME" +%s)))
    
    echo
    print_section "Test Summary"
    
    echo "Tests Run:    $TESTS_RUN"
    echo "Tests Passed: $TESTS_PASSED"
    echo "Tests Failed: $TESTS_FAILED"
    echo "Tests Skipped: $TESTS_SKIPPED"
    echo "Duration:     ${total_duration}s"
    
    if [ $TESTS_RUN -gt 0 ]; then
        local success_rate=$(( (TESTS_PASSED * 100) / TESTS_RUN ))
        echo "Success Rate: ${success_rate}%"
    fi
    
    echo
    
    if [ $TESTS_FAILED -eq 0 ]; then
        print_success "All tests completed successfully! 🎉"
        print_success "The CURSED Process Management and IPC system is fully functional."
    else
        print_error "$TESTS_FAILED test(s) failed"
        print_error "Please review the failed tests and fix the underlying issues."
    fi
}

# Main execution
main() {
    START_TIME=$(date)
    local command="basic"
    local include_ignored=false
    local use_linking_fix=true
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                print_usage
                exit 0
                ;;
            -v|--verbose)
                VERBOSE=true
                shift
                ;;
            -j|--jobs)
                JOBS="$2"
                shift 2
                ;;
            -t|--timeout)
                TIMEOUT_SECONDS="$2"
                shift 2
                ;;
            --dry-run)
                DRY_RUN=true
                shift
                ;;
            --report)
                REPORT_FILE="$2"
                shift 2
                ;;
            --filter)
                FILTER="$2"
                shift 2
                ;;
            --exclude)
                EXCLUDED_TESTS="$2"
                shift 2
                ;;
            --ignored)
                include_ignored=true
                shift
                ;;
            --no-linking-fix)
                use_linking_fix=false
                shift
                ;;
            basic|enhanced|stress|all|integration|unit)
                command="$1"
                shift
                ;;
            *)
                print_error "Unknown option: $1"
                print_usage
                exit 1
                ;;
        esac
    done
    
    # Disable linking fix if requested
    if [ "$use_linking_fix" = "false" ]; then
        CARGO_PREFIX=""
    fi
    
    # Print configuration
    print_section "Comprehensive Process Management and IPC Test Runner"
    echo "Command:        $command"
    echo "Verbose:        $VERBOSE"
    echo "Jobs:           $JOBS"
    echo "Timeout:        ${TIMEOUT_SECONDS}s"
    echo "Dry Run:        $DRY_RUN"
    echo "Report File:    ${REPORT_FILE:-"None"}"
    echo "Filter:         ${FILTER:-"None"}"
    echo "Excluded:       ${EXCLUDED_TESTS:-"None"}"
    echo "Include Ignored: $include_ignored"
    echo "Linking Fix:    ${CARGO_PREFIX:-"Disabled"}"
    
    # Check prerequisites
    check_prerequisites
    
    # Change to project root
    cd "$PROJECT_ROOT"
    
    # Run tests based on command
    local total_failed=0
    
    case $command in
        basic)
            run_basic_tests || ((total_failed++))
            ;;
        enhanced)
            run_enhanced_tests || ((total_failed++))
            ;;
        stress)
            if [ "$include_ignored" = "true" ]; then
                run_stress_tests || ((total_failed++))
            else
                print_warning "Stress tests require --ignored flag"
            fi
            ;;
        integration)
            run_integration_tests || ((total_failed++))
            ;;
        unit)
            run_unit_tests || ((total_failed++))
            ;;
        all)
            run_basic_tests || ((total_failed++))
            run_enhanced_tests || ((total_failed++))
            run_integration_tests || ((total_failed++))
            run_unit_tests || ((total_failed++))
            
            if [ "$include_ignored" = "true" ]; then
                run_stress_tests || ((total_failed++))
            fi
            ;;
        *)
            print_error "Unknown command: $command"
            exit 1
            ;;
    esac
    
    # Generate report if requested
    if [ -n "$REPORT_FILE" ]; then
        generate_report "$REPORT_FILE"
    fi
    
    # Print summary
    print_summary
    
    # Exit with appropriate code
    if [ $TESTS_FAILED -eq 0 ]; then
        exit 0
    else
        exit 1
    fi
}

# Handle script interruption
trap 'print_error "Test run interrupted"; exit 130' INT TERM

# Run main function
main "$@"
