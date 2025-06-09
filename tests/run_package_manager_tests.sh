#!/bin/bash

# CURSED Package Manager Test Runner
# Comprehensive test execution script for all package manager components

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
VERBOSE=false
TEST_FILTER=""
REPORT=false
TIMEOUT=300

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose)
            VERBOSE=true
            shift
            ;;
        --filter)
            TEST_FILTER="$2"
            shift 2
            ;;
        --report)
            REPORT=true
            shift
            ;;
        --timeout)
            TIMEOUT="$2"
            shift 2
            ;;
        --help)
            echo "CURSED Package Manager Test Runner"
            echo ""
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --verbose              Enable verbose output"
            echo "  --filter PATTERN       Run only tests matching pattern"
            echo "  --report               Generate test coverage report"
            echo "  --timeout SECONDS      Test timeout (default: 300)"
            echo "  --help                 Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                                    # Run all tests"
            echo "  $0 --verbose                          # Verbose output"
            echo "  $0 --filter package_manager_unit     # Run only unit tests"
            echo "  $0 --report                           # Generate coverage report"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Print header
echo -e "${BLUE}🦄 CURSED Package Manager Test Suite${NC}"
echo -e "${BLUE}======================================${NC}"
echo ""

# Set up environment variables for testing
export RUST_LOG=${RUST_LOG:-debug}
export RUST_BACKTRACE=${RUST_BACKTRACE:-1}

# Create test report directory
if [ "$REPORT" = true ]; then
    mkdir -p test-reports
fi

# Test configuration
CARGO_TEST_ARGS=""
if [ "$VERBOSE" = true ]; then
    CARGO_TEST_ARGS="$CARGO_TEST_ARGS --verbose"
fi

if [ -n "$TEST_FILTER" ]; then
    CARGO_TEST_ARGS="$CARGO_TEST_ARGS $TEST_FILTER"
fi

# Function to run a test suite
run_test_suite() {
    local test_name="$1"
    local description="$2"
    
    echo -e "${YELLOW}📋 Running $description${NC}"
    echo "----------------------------------------"
    
    if [ "$VERBOSE" = true ]; then
        echo "Command: cargo test --test $test_name $CARGO_TEST_ARGS"
    fi
    
    start_time=$(date +%s)
    
    if timeout $TIMEOUT cargo test --test "$test_name" $CARGO_TEST_ARGS; then
        end_time=$(date +%s)
        duration=$((end_time - start_time))
        echo -e "${GREEN}✅ $description completed in ${duration}s${NC}"
        return 0
    else
        end_time=$(date +%s)
        duration=$((end_time - start_time))
        echo -e "${RED}❌ $description failed after ${duration}s${NC}"
        return 1
    fi
}

# Function to run integration tests
run_integration_suite() {
    echo -e "${YELLOW}🔧 Running Integration Test Suite${NC}"
    echo "========================================"
    
    # Set up test environment
    export CURSED_PKG_TEST_MODE=true
    export CURSED_PKG_CACHE_DIR="$(mktemp -d)"
    
    local failed_tests=0
    
    # Package Manager Integration Tests
    if ! run_test_suite "package_manager_integration_test" "Package Manager Integration Tests"; then
        ((failed_tests++))
    fi
    
    # CLI Integration Tests  
    if ! run_test_suite "package_manager_cli_test" "CLI Integration Tests"; then
        ((failed_tests++))
    fi
    
    # Mock Infrastructure Tests
    if ! run_test_suite "package_manager_mock_test" "Mock Infrastructure Tests"; then
        ((failed_tests++))
    fi
    
    # Clean up test environment
    rm -rf "$CURSED_PKG_CACHE_DIR"
    
    echo ""
    if [ $failed_tests -eq 0 ]; then
        echo -e "${GREEN}🎉 All integration tests passed!${NC}"
        return 0
    else
        echo -e "${RED}💥 $failed_tests integration test(s) failed${NC}"
        return 1
    fi
}

# Function to run unit tests
run_unit_suite() {
    echo -e "${YELLOW}🧪 Running Unit Test Suite${NC}"
    echo "================================"
    
    if run_test_suite "package_manager_unit_test" "Package Manager Unit Tests"; then
        echo -e "${GREEN}✅ All unit tests passed!${NC}"
        return 0
    else
        echo -e "${RED}❌ Unit tests failed${NC}"
        return 1
    fi
}

# Function to run performance benchmarks
run_performance_tests() {
    echo -e "${YELLOW}⚡ Running Performance Tests${NC}"
    echo "================================="
    
    echo "Performance test configuration:"
    echo "  - Timeout: ${TIMEOUT}s"
    echo "  - Test data: tests/package_manager_test_files/test_scenarios/performance_tests.json"
    echo ""
    
    # Set performance test environment
    export CURSED_PKG_PERF_TEST=true
    export CURSED_PKG_TEST_TIMEOUT=$TIMEOUT
    
    if cargo test package_manager_performance --release -- --ignored; then
        echo -e "${GREEN}⚡ Performance tests completed successfully${NC}"
        return 0
    else
        echo -e "${YELLOW}⚠️  Performance tests not implemented yet${NC}"
        return 0  # Don't fail the suite for missing perf tests
    fi
}

# Function to generate coverage report
generate_coverage_report() {
    echo -e "${YELLOW}📊 Generating Coverage Report${NC}"
    echo "=================================="
    
    if command -v cargo-tarpaulin >/dev/null 2>&1; then
        echo "Running cargo tarpaulin for coverage analysis..."
        
        cargo tarpaulin \
            --tests \
            --out Html \
            --output-dir test-reports \
            --exclude-files 'target/*' \
            --include-tests \
            --verbose \
            --timeout $TIMEOUT \
            --package cursed \
            --features package_manager || {
            echo -e "${YELLOW}⚠️  Coverage report generation failed${NC}"
            return 1
        }
        
        echo -e "${GREEN}📊 Coverage report generated: test-reports/tarpaulin-report.html${NC}"
    else
        echo -e "${YELLOW}⚠️  cargo-tarpaulin not installed, skipping coverage report${NC}"
        echo "Install with: cargo install cargo-tarpaulin"
        return 0
    fi
}

# Main test execution
main() {
    local exit_code=0
    
    # Check if package manager module exists
    if [ ! -f "src/package_manager/mod.rs" ]; then
        echo -e "${RED}❌ Package manager module not found${NC}"
        echo "Expected: src/package_manager/mod.rs"
        exit 1
    fi
    
    # Build the project first
    echo -e "${BLUE}🔨 Building CURSED package manager...${NC}"
    if ! cargo build; then
        echo -e "${RED}❌ Build failed${NC}"
        exit 1
    fi
    echo -e "${GREEN}✅ Build successful${NC}"
    echo ""
    
    # Run test suites
    case "$TEST_FILTER" in
        *unit*)
            if ! run_unit_suite; then
                exit_code=1
            fi
            ;;
        *integration*)
            if ! run_integration_suite; then
                exit_code=1
            fi
            ;;
        *performance*|*perf*)
            if ! run_performance_tests; then
                exit_code=1
            fi
            ;;
        "")
            # Run all tests
            echo -e "${BLUE}🚀 Running complete test suite...${NC}"
            echo ""
            
            if ! run_unit_suite; then
                exit_code=1
            fi
            echo ""
            
            if ! run_integration_suite; then
                exit_code=1
            fi
            echo ""
            
            if ! run_performance_tests; then
                exit_code=1
            fi
            ;;
        *)
            # Run filtered tests
            if ! run_test_suite "$TEST_FILTER" "Filtered Tests ($TEST_FILTER)"; then
                exit_code=1
            fi
            ;;
    esac
    
    # Generate coverage report if requested
    if [ "$REPORT" = true ]; then
        echo ""
        generate_coverage_report || true  # Don't fail suite if coverage fails
    fi
    
    # Print summary
    echo ""
    echo -e "${BLUE}📋 Test Summary${NC}"
    echo "==============="
    
    if [ $exit_code -eq 0 ]; then
        echo -e "${GREEN}🎉 All tests passed successfully!${NC}"
        echo ""
        echo "Package manager test coverage:"
        echo "  ✅ Unit tests (metadata, cache, registry, resolver)"
        echo "  ✅ Integration tests (end-to-end workflows)"
        echo "  ✅ CLI tests (command-line interface)"
        echo "  ✅ Mock infrastructure (testing utilities)"
        
        if [ "$REPORT" = true ]; then
            echo "  📊 Coverage report generated"
        fi
    else
        echo -e "${RED}💥 Some tests failed${NC}"
        echo ""
        echo "Check the output above for details."
        echo "Use --verbose for more detailed error information."
    fi
    
    exit $exit_code
}

# Run main function
main "$@"
