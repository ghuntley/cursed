#!/bin/bash

# Comprehensive Enhanced GC Test Suite Runner
# 
# This script runs all enhanced GC tests with proper linking fix integration
# and provides detailed reporting on test results and performance.

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
VERBOSE=false
QUICK=false
IGNORED=false
COVERAGE=false
REPORT_FILE=""
TEST_FILTER=""

# Test categories
UNIT_TESTS="enhanced_gc_unit_test"
INTEGRATION_TESTS="enhanced_gc_integration_test"
PERFORMANCE_TESTS="enhanced_gc_performance_test"
STRESS_TESTS="enhanced_gc_stress_test"
MEMORY_SAFETY_TESTS="enhanced_gc_memory_safety_test"

ALL_TESTS="$UNIT_TESTS $INTEGRATION_TESTS $PERFORMANCE_TESTS $STRESS_TESTS $MEMORY_SAFETY_TESTS"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --quick|-q)
            QUICK=true
            shift
            ;;
        --ignored|-i)
            IGNORED=true
            shift
            ;;
        --coverage|-c)
            COVERAGE=true
            shift
            ;;
        --report|-r)
            REPORT_FILE="$2"
            shift 2
            ;;
        --filter|-f)
            TEST_FILTER="$2"
            shift 2
            ;;
        --test)
            case $2 in
                unit)
                    ALL_TESTS="$UNIT_TESTS"
                    ;;
                integration)
                    ALL_TESTS="$INTEGRATION_TESTS"
                    ;;
                performance)
                    ALL_TESTS="$PERFORMANCE_TESTS"
                    ;;
                stress)
                    ALL_TESTS="$STRESS_TESTS"
                    ;;
                memory-safety)
                    ALL_TESTS="$MEMORY_SAFETY_TESTS"
                    ;;
                *)
                    echo -e "${RED}Error: Unknown test category '$2'${NC}"
                    echo "Available categories: unit, integration, performance, stress, memory-safety"
                    exit 1
                    ;;
            esac
            shift 2
            ;;
        --help|-h)
            echo "Enhanced GC Test Suite Runner"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --verbose, -v         Enable verbose output"
            echo "  --quick, -q           Run quick tests only (skip long-running tests)"
            echo "  --ignored, -i         Run ignored tests (stress tests, performance tests)"
            echo "  --coverage, -c        Generate coverage report"
            echo "  --report FILE, -r     Write detailed report to FILE"
            echo "  --filter PATTERN, -f  Filter tests by pattern"
            echo "  --test CATEGORY       Run specific test category"
            echo "                        Categories: unit, integration, performance, stress, memory-safety"
            echo "  --help, -h            Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                    # Run all tests"
            echo "  $0 --quick           # Run quick tests only"
            echo "  $0 --test unit       # Run unit tests only"
            echo "  $0 --ignored         # Run stress and performance tests"
            echo "  $0 --coverage        # Generate coverage report"
            exit 0
            ;;
        *)
            echo -e "${RED}Error: Unknown option '$1'${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Function to print status messages
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if linking fix script exists
check_linking_fix() {
    if [[ ! -f "./fix_linking.sh" ]]; then
        print_error "Linking fix script not found: ./fix_linking.sh"
        print_error "Please ensure the linking fix script is available"
        exit 1
    fi
    
    if [[ ! -x "./fix_linking.sh" ]]; then
        print_warning "Making linking fix script executable"
        chmod +x ./fix_linking.sh
    fi
}

# Function to run a single test with proper error handling
run_test() {
    local test_name="$1"
    local test_args="$2"
    local start_time=$(date +%s)
    
    print_status "Running $test_name tests..."
    
    if [[ $VERBOSE == true ]]; then
        echo "Command: ./fix_linking.sh cargo test --test $test_name $test_args"
    fi
    
    local output
    local exit_code
    
    if [[ $VERBOSE == true ]]; then
        ./fix_linking.sh cargo test --test "$test_name" $test_args
        exit_code=$?
    else
        output=$(./fix_linking.sh cargo test --test "$test_name" $test_args 2>&1)
        exit_code=$?
    fi
    
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    if [[ $exit_code -eq 0 ]]; then
        print_success "$test_name tests passed (${duration}s)"
        return 0
    else
        print_error "$test_name tests failed (${duration}s)"
        if [[ $VERBOSE == false ]]; then
            echo "Output:"
            echo "$output"
        fi
        return 1
    fi
}

# Function to run tests with coverage
run_with_coverage() {
    print_status "Running tests with coverage analysis..."
    
    # Check if cargo-tarpaulin is installed
    if ! command -v cargo-tarpaulin &> /dev/null; then
        print_warning "cargo-tarpaulin not found, installing..."
        cargo install cargo-tarpaulin
    fi
    
    local coverage_args="--all-features --workspace --timeout 300"
    if [[ $IGNORED == true ]]; then
        coverage_args="$coverage_args --ignored"
    fi
    
    ./fix_linking.sh cargo tarpaulin $coverage_args --out Html --output-dir coverage/
    
    if [[ $? -eq 0 ]]; then
        print_success "Coverage report generated in coverage/"
    else
        print_error "Coverage analysis failed"
        return 1
    fi
}

# Function to generate test report
generate_report() {
    local report_file="$1"
    local test_results="$2"
    
    print_status "Generating test report: $report_file"
    
    cat > "$report_file" << EOF
# Enhanced GC Test Suite Report

Generated: $(date)

## Test Configuration
- Verbose: $VERBOSE
- Quick mode: $QUICK
- Include ignored tests: $IGNORED
- Coverage analysis: $COVERAGE
- Test filter: ${TEST_FILTER:-"none"}

## Test Results
$test_results

## Test Categories Executed
EOF

    for test in $ALL_TESTS; do
        echo "- $test" >> "$report_file"
    done

    cat >> "$report_file" << EOF

## Performance Notes
- All tests use the linking fix infrastructure for Nix environment compatibility
- Stress tests and performance tests require the --ignored flag
- Memory safety tests validate critical GC guarantees

## Next Steps
- Review any failed tests in detail
- Run ignored tests for comprehensive validation
- Generate coverage report for code coverage analysis
EOF

    print_success "Test report written to $report_file"
}

# Main execution
main() {
    print_status "Starting Enhanced GC Test Suite"
    print_status "Configuration: verbose=$VERBOSE, quick=$QUICK, ignored=$IGNORED, coverage=$COVERAGE"
    
    # Check prerequisites
    check_linking_fix
    
    # Build test arguments
    local test_args=""
    if [[ $VERBOSE == true ]]; then
        test_args="$test_args --nocapture"
    fi
    
    if [[ $IGNORED == true ]]; then
        test_args="$test_args -- --ignored"
    elif [[ $QUICK == true ]]; then
        test_args="$test_args"  # Quick mode skips ignored tests by default
    fi
    
    if [[ -n $TEST_FILTER ]]; then
        test_args="$test_args $TEST_FILTER"
    fi
    
    # Run coverage analysis if requested
    if [[ $COVERAGE == true ]]; then
        run_with_coverage
        return $?
    fi
    
    # Track test results
    local passed_tests=0
    local failed_tests=0
    local test_results=""
    
    # Run each test category
    for test in $ALL_TESTS; do
        if run_test "$test" "$test_args"; then
            ((passed_tests++))
            test_results="$test_results\n✓ $test: PASSED"
        else
            ((failed_tests++))
            test_results="$test_results\n✗ $test: FAILED"
        fi
        echo ""  # Add spacing between tests
    done
    
    # Generate summary
    local total_tests=$((passed_tests + failed_tests))
    print_status "Test Summary:"
    echo "  Total tests: $total_tests"
    echo "  Passed: $passed_tests"
    echo "  Failed: $failed_tests"
    
    # Generate report if requested
    if [[ -n $REPORT_FILE ]]; then
        generate_report "$REPORT_FILE" "$test_results"
    fi
    
    # Final result
    if [[ $failed_tests -eq 0 ]]; then
        print_success "All enhanced GC tests passed!"
        return 0
    else
        print_error "$failed_tests test(s) failed"
        return 1
    fi
}

# Run main function
main "$@"
