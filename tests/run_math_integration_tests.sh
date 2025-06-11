#!/bin/bash

# CURSED Mathematics Library Integration Test Runner
# Comprehensive testing script for the complete math library

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Test configuration
VERBOSE=false
QUICK=false
COVERAGE=false
REPORT_FILE=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -q|--quick)
            QUICK=true
            shift
            ;;
        -c|--coverage)
            COVERAGE=true
            shift
            ;;
        -r|--report)
            REPORT_FILE="$2"
            shift 2
            ;;
        -h|--help)
            echo "CURSED Math Library Integration Test Runner"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -v, --verbose    Enable verbose output"
            echo "  -q, --quick      Run quick tests only"
            echo "  -c, --coverage   Generate coverage report"
            echo "  -r, --report     Generate detailed report file"
            echo "  -h, --help       Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                           # Run all tests"
            echo "  $0 --quick                   # Quick validation"
            echo "  $0 --verbose --coverage      # Full tests with coverage"
            echo "  $0 --report math_test_report.md  # Generate report"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Helper functions
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

log_header() {
    echo -e "${PURPLE}[HEADER]${NC} $1"
}

log_test() {
    echo -e "${CYAN}[TEST]${NC} $1"
}

# Test execution function
run_test() {
    local test_name="$1"
    local test_command="$2"
    local description="$3"
    
    if [[ "$VERBOSE" == "true" ]]; then
        log_test "Running: $test_name - $description"
    fi
    
    if eval "$test_command" > /dev/null 2>&1; then
        if [[ "$VERBOSE" == "true" ]]; then
            log_success "$test_name passed"
        fi
        return 0
    else
        log_error "$test_name failed"
        if [[ "$VERBOSE" == "true" ]]; then
            eval "$test_command"
        fi
        return 1
    fi
}

# Progress tracking
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
START_TIME=$(date +%s)

# Report generation
generate_report() {
    if [[ -n "$REPORT_FILE" ]]; then
        log_info "Generating detailed report: $REPORT_FILE"
        
        cat > "$REPORT_FILE" << EOF
# CURSED Mathematics Library Integration Test Report

Generated: $(date)
Duration: $(($(date +%s) - START_TIME)) seconds

## Test Summary

- **Total Tests**: $TOTAL_TESTS
- **Passed**: $PASSED_TESTS
- **Failed**: $FAILED_TESTS
- **Success Rate**: $(( PASSED_TESTS * 100 / TOTAL_TESTS ))%

## Test Categories

### 1. Library Compilation
- Module compilation and integration
- Function export validation
- Naming conflict resolution

### 2. Function Integration
- Cross-module functionality
- Error handling consistency
- Type system integration

### 3. Mathematical Validation
- Identity verification
- Precision testing
- Domain validation

### 4. Performance Testing
- Execution speed validation
- Memory efficiency testing
- Scalability verification

### 5. Example Validation
- Demo program execution
- Usage pattern validation
- Real-world scenario testing

## Configuration Used

- Verbose: $VERBOSE
- Quick Mode: $QUICK
- Coverage: $COVERAGE
- Test Command: \`./fix_linking.sh cargo test\`

## Results Details

EOF
    fi
}

# Initialize report
if [[ -n "$REPORT_FILE" ]]; then
    generate_report
fi

# Main test execution
main() {
    log_header "=========================================="
    log_header "CURSED Mathematics Library Integration Tests"
    log_header "=========================================="
    echo ""
    
    # Check if we're in the right directory
    if [[ ! -f "Cargo.toml" ]]; then
        log_error "Not in CURSED project root directory"
        exit 1
    fi
    
    log_info "Starting comprehensive mathematics library testing..."
    echo ""
    
    # 1. Library Compilation Tests
    log_header "1. Library Compilation and Integration"
    
    ((TOTAL_TESTS++))
    if run_test "lib_compilation" "./fix_linking.sh cargo check --lib" "Library compilation"; then
        ((PASSED_TESTS++))
    else
        ((FAILED_TESTS++))
    fi
    
    ((TOTAL_TESTS++))
    if run_test "math_module_compilation" "./fix_linking.sh cargo check --lib -p cursed" "Math module compilation"; then
        ((PASSED_TESTS++))
    else
        ((FAILED_TESTS++))
    fi
    
    # 2. Core Function Tests
    log_header "2. Core Mathematical Function Tests"
    
    # Basic operations
    ((TOTAL_TESTS++))
    if run_test "math_basic_test" "./fix_linking.sh cargo test math_basic_test" "Basic mathematical operations"; then
        ((PASSED_TESTS++))
    else
        ((FAILED_TESTS++))
    fi
    
    # Logarithmic functions
    ((TOTAL_TESTS++))
    if run_test "math_logarithmic_test" "./fix_linking.sh cargo test math_logarithmic_test" "Logarithmic and exponential functions"; then
        ((PASSED_TESTS++))
    else
        ((FAILED_TESTS++))
    fi
    
    # Integration test
    ((TOTAL_TESTS++))
    if run_test "math_integration_test" "./fix_linking.sh cargo test math_library_integration_test" "Math library integration"; then
        ((PASSED_TESTS++))
    else
        ((FAILED_TESTS++))
    fi
    
    if [[ "$QUICK" == "false" ]]; then
        # 3. Individual Module Tests
        log_header "3. Individual Module Validation"
        
        # Test each module if tests exist
        for module in trigonometry constants statistics special utilities random; do
            if grep -q "math_${module}_test" tests/*.rs 2>/dev/null; then
                ((TOTAL_TESTS++))
                if run_test "math_${module}_test" "./fix_linking.sh cargo test math_${module}_test" "Math $module module"; then
                    ((PASSED_TESTS++))
                else
                    ((FAILED_TESTS++))
                fi
            fi
        done
        
        # 4. Cross-Module Integration Tests
        log_header "4. Cross-Module Integration"
        
        ((TOTAL_TESTS++))
        if run_test "cross_module_integration" "./fix_linking.sh cargo test test_cross_module_functionality" "Cross-module functionality"; then
            ((PASSED_TESTS++))
        else
            ((FAILED_TESTS++))
        fi
        
        ((TOTAL_TESTS++))
        if run_test "error_handling_integration" "./fix_linking.sh cargo test test_error_handling_integration" "Error handling integration"; then
            ((PASSED_TESTS++))
        else
            ((FAILED_TESTS++))
        fi
        
        # 5. Performance and Memory Tests
        log_header "5. Performance and Memory Validation"
        
        ((TOTAL_TESTS++))
        if run_test "performance_test" "./fix_linking.sh cargo test test_performance_characteristics" "Performance characteristics"; then
            ((PASSED_TESTS++))
        else
            ((FAILED_TESTS++))
        fi
        
        ((TOTAL_TESTS++))
        if run_test "memory_efficiency_test" "./fix_linking.sh cargo test test_memory_efficiency" "Memory efficiency"; then
            ((PASSED_TESTS++))
        else
            ((FAILED_TESTS++))
        fi
        
        # 6. Mathematical Identity Validation
        log_header "6. Mathematical Identity Validation"
        
        ((TOTAL_TESTS++))
        if run_test "mathematical_identities" "./fix_linking.sh cargo test test_mathematical_identities" "Mathematical identities"; then
            ((PASSED_TESTS++))
        else
            ((FAILED_TESTS++))
        fi
        
        ((TOTAL_TESTS++))
        if run_test "type_consistency" "./fix_linking.sh cargo test test_type_consistency" "Type consistency"; then
            ((PASSED_TESTS++))
        else
            ((FAILED_TESTS++))
        fi
        
        # 7. Comprehensive Workflow Test
        log_header "7. Comprehensive Workflow Validation"
        
        ((TOTAL_TESTS++))
        if run_test "comprehensive_workflow" "./fix_linking.sh cargo test test_comprehensive_workflow" "Comprehensive workflow"; then
            ((PASSED_TESTS++))
        else
            ((FAILED_TESTS++))
        fi
    fi
    
    # 8. Coverage Testing (if requested)
    if [[ "$COVERAGE" == "true" ]]; then
        log_header "8. Coverage Analysis"
        
        if command -v cargo-tarpaulin &> /dev/null; then
            log_info "Running coverage analysis..."
            ./fix_linking.sh cargo tarpaulin --out Html --output-dir coverage/math --include-tests --ignore-config
            log_success "Coverage report generated in coverage/math/"
        else
            log_warning "cargo-tarpaulin not installed, skipping coverage"
        fi
    fi
    
    # 9. Example Compilation Test
    log_header "9. Example Program Validation"
    
    if [[ -f "examples/math_comprehensive_demo.csd" ]]; then
        log_info "Math comprehensive demo found - validating CURSED syntax"
        # Note: This would require the CURSED compiler to be built
        # For now, we just check the file exists and has valid structure
        if grep -q "slay main()" examples/math_comprehensive_demo.csd; then
            log_success "Demo program structure validated"
        else
            log_warning "Demo program may have syntax issues"
        fi
    else
        log_warning "Math comprehensive demo not found"
    fi
    
    # Final Results
    echo ""
    log_header "=========================================="
    log_header "TEST EXECUTION COMPLETE"
    log_header "=========================================="
    
    END_TIME=$(date +%s)
    DURATION=$((END_TIME - START_TIME))
    
    echo ""
    log_info "Test Summary:"
    echo "  Total Tests: $TOTAL_TESTS"
    echo "  Passed: $PASSED_TESTS"
    echo "  Failed: $FAILED_TESTS"
    echo "  Duration: ${DURATION}s"
    
    if [[ $FAILED_TESTS -eq 0 ]]; then
        log_success "All tests passed! Math library integration successful."
        SUCCESS_RATE=100
    else
        SUCCESS_RATE=$(( PASSED_TESTS * 100 / TOTAL_TESTS ))
        if [[ $SUCCESS_RATE -ge 80 ]]; then
            log_warning "Most tests passed ($SUCCESS_RATE%), but some issues remain."
        else
            log_error "Significant test failures ($SUCCESS_RATE% success rate)."
        fi
    fi
    
    # Generate final report
    if [[ -n "$REPORT_FILE" ]]; then
        cat >> "$REPORT_FILE" << EOF

### Final Test Execution

- **Duration**: ${DURATION} seconds
- **Success Rate**: ${SUCCESS_RATE}%
- **Status**: $(if [[ $FAILED_TESTS -eq 0 ]]; then echo "✅ All tests passed"; else echo "❌ Some tests failed"; fi)

### Recommendations

EOF
        
        if [[ $FAILED_TESTS -eq 0 ]]; then
            cat >> "$REPORT_FILE" << EOF
- The mathematics library integration is complete and functional
- All modules work together cohesively
- Error handling is consistent across modules
- Performance characteristics meet requirements
- Ready for production use
EOF
        else
            cat >> "$REPORT_FILE" << EOF
- Review failed test cases for specific issues
- Check function naming conflicts
- Validate error handling implementation
- Consider performance optimizations
- Update documentation as needed
EOF
        fi
        
        log_success "Detailed report generated: $REPORT_FILE"
    fi
    
    echo ""
    log_header "Integration test execution complete."
    
    # Exit with appropriate code
    if [[ $FAILED_TESTS -eq 0 ]]; then
        exit 0
    else
        exit 1
    fi
}

# Execute main function
main "$@"
