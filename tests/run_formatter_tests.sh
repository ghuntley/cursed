#!/bin/bash
# Comprehensive test runner for CURSED formatter tests

set -e

echo "🧪 Running CURSED Formatter Test Suite"
echo "======================================"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to run a test with timing
run_test() {
    local test_name="$1"
    local test_file="$2"
    
    echo -n "Running $test_name tests... "
    start_time=$(date +%s)
    
    if cargo test --test "$test_file" 2>/dev/null; then
        end_time=$(date +%s)
        duration=$((end_time - start_time))
        echo -e "${GREEN}✓ PASSED${NC} (${duration}s)"
        return 0
    else
        echo -e "${RED}✗ FAILED${NC}"
        return 1
    fi
}

# Function to run a test with verbose output
run_test_verbose() {
    local test_name="$1"
    local test_file="$2"
    
    echo "Running $test_name tests with verbose output:"
    echo "----------------------------------------------"
    
    if cargo test --test "$test_file" -- --nocapture; then
        echo -e "${GREEN}✓ $test_name tests PASSED${NC}"
        return 0
    else
        echo -e "${RED}✗ $test_name tests FAILED${NC}"
        return 1
    fi
}

# Parse command line arguments
VERBOSE=false
SPECIFIC_TEST=""
GENERATE_REPORT=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -t|--test)
            SPECIFIC_TEST="$2"
            shift 2
            ;;
        -r|--report)
            GENERATE_REPORT=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  -v, --verbose     Run tests with verbose output"
            echo "  -t, --test NAME   Run specific test suite"
            echo "  -r, --report      Generate test coverage report"
            echo "  -h, --help        Show this help message"
            echo ""
            echo "Available test suites:"
            echo "  unit              Formatter unit tests"
            echo "  integration       Formatter integration tests"
            echo "  cli               CLI tool tests"
            echo "  golden            Golden file tests"
            echo "  config            Configuration tests"
            echo "  all               All formatter tests"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Test definitions
declare -A TESTS=(
    ["unit"]="formatter_unit_test"
    ["integration"]="formatter_integration_test"
    ["cli"]="formatter_cli_test"
    ["golden"]="formatter_golden_test"
    ["config"]="formatter_config_test"
)

# Track test results
PASSED_TESTS=0
FAILED_TESTS=0
FAILED_TEST_NAMES=()

# Function to run all tests
run_all_tests() {
    echo "Running all formatter tests..."
    echo ""
    
    for test_name in "${!TESTS[@]}"; do
        test_file="${TESTS[$test_name]}"
        
        if [ "$VERBOSE" = true ]; then
            if run_test_verbose "$test_name" "$test_file"; then
                ((PASSED_TESTS++))
            else
                ((FAILED_TESTS++))
                FAILED_TEST_NAMES+=("$test_name")
            fi
        else
            if run_test "$test_name" "$test_file"; then
                ((PASSED_TESTS++))
            else
                ((FAILED_TESTS++))
                FAILED_TEST_NAMES+=("$test_name")
            fi
        fi
        echo ""
    done
}

# Function to run specific test
run_specific_test() {
    local test_name="$1"
    
    if [[ ! -v "TESTS[$test_name]" ]]; then
        echo -e "${RED}Error: Unknown test suite '$test_name'${NC}"
        echo "Available test suites: ${!TESTS[*]}"
        exit 1
    fi
    
    local test_file="${TESTS[$test_name]}"
    
    if [ "$VERBOSE" = true ]; then
        if run_test_verbose "$test_name" "$test_file"; then
            ((PASSED_TESTS++))
        else
            ((FAILED_TESTS++))
            FAILED_TEST_NAMES+=("$test_name")
        fi
    else
        if run_test "$test_name" "$test_file"; then
            ((PASSED_TESTS++))
        else
            ((FAILED_TESTS++))
            FAILED_TEST_NAMES+=("$test_name")
        fi
    fi
}

# Main execution
if [ -n "$SPECIFIC_TEST" ]; then
    if [ "$SPECIFIC_TEST" = "all" ]; then
        run_all_tests
    else
        run_specific_test "$SPECIFIC_TEST"
    fi
else
    run_all_tests
fi

# Generate test coverage report if requested
if [ "$GENERATE_REPORT" = true ]; then
    echo ""
    echo "Generating test coverage report..."
    
    # Install cargo-tarpaulin if not available
    if ! command -v cargo-tarpaulin &> /dev/null; then
        echo "Installing cargo-tarpaulin for coverage reporting..."
        cargo install cargo-tarpaulin
    fi
    
    # Run coverage for formatter tests
    cargo tarpaulin --out Html --output-dir target/coverage \
        --test formatter_unit_test \
        --test formatter_integration_test \
        --test formatter_cli_test \
        --test formatter_golden_test \
        --test formatter_config_test
    
    echo "Coverage report generated in target/coverage/tarpaulin-report.html"
fi

# Print summary
echo ""
echo "======================================"
echo "📊 Test Summary"
echo "======================================"
echo "Total test suites: $((PASSED_TESTS + FAILED_TESTS))"
echo -e "Passed: ${GREEN}$PASSED_TESTS${NC}"
echo -e "Failed: ${RED}$FAILED_TESTS${NC}"

if [ $FAILED_TESTS -gt 0 ]; then
    echo ""
    echo -e "${RED}Failed test suites:${NC}"
    for test_name in "${FAILED_TEST_NAMES[@]}"; do
        echo "  - $test_name"
    done
    echo ""
    echo -e "${YELLOW}To run a specific failed test with verbose output:${NC}"
    echo "  $0 --verbose --test <test_name>"
    exit 1
else
    echo ""
    echo -e "${GREEN}🎉 All formatter tests passed!${NC}"
    exit 0
fi
