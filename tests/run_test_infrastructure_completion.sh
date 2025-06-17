#!/bin/bash

# Test Infrastructure Completion Runner
# Runs all implemented functional tests in organized categories

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test categories and their priorities
declare -A TEST_CATEGORIES=(
    ["core_compilation"]="Core Compilation Pipeline"
    ["type_system"]="Type System & Generics"
    ["memory_management"]="Memory Management & GC"
    ["concurrency"]="Concurrency & Goroutines"
    ["stdlib"]="Standard Library"
    ["llvm_codegen"]="LLVM Code Generation"
    ["integration"]="Integration Tests"
)

# Test tracking
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
SKIPPED_TESTS=0

# Logging
LOG_FILE="test_infrastructure_completion.log"
echo "Test Infrastructure Completion Run - $(date)" > "$LOG_FILE"

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1" | tee -a "$LOG_FILE"
}

log_success() {
    echo -e "${GREEN}[PASS]${NC} $1" | tee -a "$LOG_FILE"
}

log_error() {
    echo -e "${RED}[FAIL]${NC} $1" | tee -a "$LOG_FILE"
}

log_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1" | tee -a "$LOG_FILE"
}

# Function to run a test category
run_test_category() {
    local category=$1
    local description=$2
    
    log_info "Running $description tests..."
    
    # Apply linking fix for Nix environment
    if [[ -f "./fix_linking.sh" ]]; then
        export LIBRARY_PATH="/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib:/nix/store/l5g2v1jgfyf3j0jp9iv5b79fi8yrwzpp-zlib-1.3.1/lib:/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib:/nix/store/hd6llsw2dkiazk9d2ywv13cc6alhflly-libxml2-2.13.5/lib:/nix/store/dsqzw96w4sxsp4q9yvkfl2yh701mpwgi-sqlite-3.46.1/lib"
        export RUSTFLAGS="-C linker=gcc -C link-arg=-fuse-ld=bfd"
    fi
    
    # Run category-specific tests
    case $category in
        "core_compilation")
            run_core_compilation_tests
            ;;
        "type_system") 
            run_type_system_tests
            ;;
        "memory_management")
            run_memory_management_tests
            ;;
        "concurrency")
            run_concurrency_tests
            ;;
        "stdlib")
            run_stdlib_tests
            ;;
        "llvm_codegen")
            run_llvm_codegen_tests
            ;;
        "integration")
            run_integration_tests
            ;;
        *)
            log_warning "Unknown test category: $category"
            ;;
    esac
}

# Core compilation pipeline tests
run_core_compilation_tests() {
    local tests=(
        "core_compilation_basic_test"
        "lib_function_integration_test"
        "lexer_parser_integration_test"
        "error_handling_core_test"
        "formatter_basic_test"
    )
    
    for test in "${tests[@]}"; do
        run_single_test "$test"
    done
}

# Type system tests
run_type_system_tests() {
    local tests=(
        "type_system_basic_test"
        "generic_instantiation_test"
        "interface_implementation_basic_test"
        "type_assertion_functional_test"
        "type_conversion_functional_test"
    )
    
    for test in "${tests[@]}"; do
        run_single_test "$test"
    done
}

# Memory management tests
run_memory_management_tests() {
    local tests=(
        "gc_functional_test"
        "memory_allocation_test"
        "circular_reference_test"
        "enhanced_gc_test"
        "production_gc_test"
    )
    
    for test in "${tests[@]}"; do
        run_single_test "$test"
    done
}

# Concurrency tests
run_concurrency_tests() {
    local tests=(
        "goroutine_basic_functional_test"
        "channel_operations_functional_test"
        "sync_primitives_test"
        "goroutine_gc_integration_functional_test"
        "scheduler_basic_test"
    )
    
    for test in "${tests[@]}"; do
        run_single_test "$test"
    done
}

# Standard library tests
run_stdlib_tests() {
    local tests=(
        "math_basic_functional_test"
        "string_operations_functional_test"
        "io_operations_functional_test"
        "collections_functional_test"
        "database_basic_functional_test"
    )
    
    for test in "${tests[@]}"; do
        run_single_test "$test"
    done
}

# LLVM code generation tests
run_llvm_codegen_tests() {
    local tests=(
        "llvm_expression_compilation_test"
        "llvm_control_flow_test"
        "llvm_function_compilation_test"
        "llvm_optimization_test"
        "llvm_ir_generation_test"
    )
    
    for test in "${tests[@]}"; do
        run_single_test "$test"
    done
}

# Integration tests
run_integration_tests() {
    local tests=(
        "end_to_end_compilation_test"
        "package_integration_test"
        "optimization_integration_test"
        "performance_integration_test"
        "cross_module_integration_test"
    )
    
    for test in "${tests[@]}"; do
        run_single_test "$test"
    done
}

# Function to run a single test
run_single_test() {
    local test_name=$1
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    log_info "Running test: $test_name"
    
    # Check if test file exists
    if [[ -f "tests/${test_name}.rs" ]]; then
        # Run the test
        if cargo test --test "$test_name" >> "$LOG_FILE" 2>&1; then
            log_success "$test_name"
            PASSED_TESTS=$((PASSED_TESTS + 1))
        else
            log_error "$test_name"
            FAILED_TESTS=$((FAILED_TESTS + 1))
        fi
    else
        log_warning "$test_name - File not found, skipping"
        SKIPPED_TESTS=$((SKIPPED_TESTS + 1))
    fi
}

# Function to run existing working tests
run_existing_working_tests() {
    log_info "Running existing working tests..."
    
    # Tests we know are working
    local working_tests=(
        "math_basic_test"
        "math_logarithmic_test"
        "enhanced_gc_unit_test"
        "enhanced_gc_integration_test" 
        "goroutine_runtime_basic_test"
        "type_assertion_integration_test"
        "formatter_config_test"
        "formatter_unit_test"
        "console_io_test"
    )
    
    for test in "${working_tests[@]}"; do
        run_single_test "$test"
    done
}

# Function to generate test report
generate_test_report() {
    local report_file="test_infrastructure_completion_report.md"
    
    cat > "$report_file" << EOF
# Test Infrastructure Completion Report

**Generated:** $(date)

## Summary

- **Total Tests:** $TOTAL_TESTS
- **Passed:** $PASSED_TESTS
- **Failed:** $FAILED_TESTS  
- **Skipped:** $SKIPPED_TESTS
- **Success Rate:** $(echo "scale=2; $PASSED_TESTS * 100 / $TOTAL_TESTS" | bc -l)%

## Test Categories Executed

EOF

    for category in "${!TEST_CATEGORIES[@]}"; do
        echo "- **${TEST_CATEGORIES[$category]}**" >> "$report_file"
    done
    
    cat >> "$report_file" << EOF

## Detailed Results

See \`$LOG_FILE\` for detailed test execution logs.

## Next Steps

1. Implement failing tests with real functionality
2. Create missing test files  
3. Expand test coverage for critical areas
4. Integrate with CI/CD pipeline

## Implementation Status

### Completed
- Test infrastructure framework
- Test categorization system
- Automated test runner
- Progress tracking and reporting

### In Progress  
- Converting TODO stubs to functional tests
- Building comprehensive test coverage
- Performance and stress testing

### Planned
- Full test suite completion
- CI/CD integration
- Performance benchmarking
- Regression testing
EOF

    log_info "Test report generated: $report_file"
}

# Main execution
main() {
    log_info "Starting Test Infrastructure Completion..."
    log_info "This will validate converted functional tests"
    
    # Check for linking fix
    if [[ ! -f "./fix_linking.sh" ]]; then
        log_warning "fix_linking.sh not found - some tests may fail in Nix environment"
    fi
    
    # Run existing working tests first
    run_existing_working_tests
    
    # Run tests by category
    for category in "${!TEST_CATEGORIES[@]}"; do
        run_test_category "$category" "${TEST_CATEGORIES[$category]}"
    done
    
    # Generate final report
    generate_test_report
    
    log_info "Test Infrastructure Completion finished"
    log_info "Results: $PASSED_TESTS passed, $FAILED_TESTS failed, $SKIPPED_TESTS skipped out of $TOTAL_TESTS total"
    
    # Exit with appropriate code
    if [[ $FAILED_TESTS -eq 0 ]]; then
        exit 0
    else
        exit 1
    fi
}

# Parse command line arguments
case "${1:-}" in
    "--help" | "-h")
        echo "Usage: $0 [category]"
        echo "Categories: ${!TEST_CATEGORIES[*]}"
        exit 0
        ;;
    "")
        main
        ;;
    *)
        if [[ -n "${TEST_CATEGORIES[$1]}" ]]; then
            run_test_category "$1" "${TEST_CATEGORIES[$1]}"
        else
            log_error "Unknown category: $1"
            echo "Available categories: ${!TEST_CATEGORIES[*]}"
            exit 1
        fi
        ;;
esac
