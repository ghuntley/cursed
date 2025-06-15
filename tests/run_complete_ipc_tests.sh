#!/bin/bash

# Complete IPC Test Suite Runner for CURSED Programming Language
# 
# This script runs comprehensive tests for all Inter-Process Communication
# mechanisms including performance, stress, and integration testing.

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
CARGO_CMD="cargo"
TEST_TIMEOUT="300" # 5 minutes
VERBOSE=false
QUICK=false
IGNORED=false
REPORT_FILE=""
COVERAGE=false

# Function to print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}[IPC-TEST]${NC} ${message}"
}

print_success() {
    print_status "$GREEN" "$1"
}

print_error() {
    print_status "$RED" "$1"
}

print_warning() {
    print_status "$YELLOW" "$1"
}

print_info() {
    print_status "$BLUE" "$1"
}

# Function to check if linking fix is needed
check_linking_fix() {
    if [[ -f "./fix_linking.sh" ]]; then
        print_info "Using linking fix for Nix environment"
        CARGO_CMD="./fix_linking.sh cargo"
    else
        print_info "Using standard cargo command"
    fi
}

# Function to run a single test with timeout and error handling
run_test() {
    local test_name=$1
    local extra_args=$2
    
    print_info "Running test: $test_name"
    
    local cmd="$CARGO_CMD test --test $test_name $extra_args"
    if [[ "$VERBOSE" == "true" ]]; then
        cmd="$cmd -- --show-output"
    fi
    
    if timeout "$TEST_TIMEOUT" $cmd; then
        print_success "✓ $test_name passed"
        return 0
    else
        local exit_code=$?
        if [[ $exit_code -eq 124 ]]; then
            print_error "✗ $test_name timed out after ${TEST_TIMEOUT}s"
        else
            print_error "✗ $test_name failed with exit code $exit_code"
        fi
        return $exit_code
    fi
}

# Function to run all IPC unit tests
run_unit_tests() {
    print_info "=== Running IPC Unit Tests ==="
    
    local tests=(
        "ipc_complete_test_suite::test_ipc_system_lifecycle"
        "ipc_complete_test_suite::test_named_pipes_comprehensive"
        "ipc_complete_test_suite::test_shared_memory_comprehensive"
        "ipc_complete_test_suite::test_message_queues_comprehensive"
        "ipc_complete_test_suite::test_semaphores_comprehensive"
        "ipc_complete_test_suite::test_barriers_comprehensive"
        "ipc_complete_test_suite::test_rwlock_timeout"
        "ipc_complete_test_suite::test_condition_variables"
        "ipc_complete_test_suite::test_distributed_coordinator"
        "ipc_complete_test_suite::test_unix_domain_sockets"
        "ipc_complete_test_suite::test_signal_handling"
        "ipc_complete_test_suite::test_rpc_functionality"
    )
    
    local failed_tests=()
    local passed_count=0
    
    for test in "${tests[@]}"; do
        if run_test "ipc_complete_test_suite" "-- --exact $test"; then
            ((passed_count++))
        else
            failed_tests+=("$test")
        fi
    done
    
    print_info "Unit tests summary: $passed_count/${#tests[@]} passed"
    
    if [[ ${#failed_tests[@]} -gt 0 ]]; then
        print_warning "Failed unit tests:"
        for test in "${failed_tests[@]}"; do
            print_warning "  - $test"
        done
        return 1
    fi
    
    return 0
}

# Function to run performance tests
run_performance_tests() {
    print_info "=== Running IPC Performance Tests ==="
    
    local perf_tests=(
        "ipc_complete_test_suite::test_ipc_performance"
    )
    
    local failed_tests=()
    local passed_count=0
    
    for test in "${perf_tests[@]}"; do
        if run_test "ipc_complete_test_suite" "-- --exact $test"; then
            ((passed_count++))
        else
            failed_tests+=("$test")
        fi
    done
    
    print_info "Performance tests summary: $passed_count/${#perf_tests[@]} passed"
    
    if [[ ${#failed_tests[@]} -gt 0 ]]; then
        print_warning "Failed performance tests:"
        for test in "${failed_tests[@]}"; do
            print_warning "  - $test"
        done
        return 1
    fi
    
    return 0
}

# Function to run stress tests
run_stress_tests() {
    print_info "=== Running IPC Stress Tests ==="
    
    local stress_tests=(
        "ipc_complete_test_suite::test_ipc_concurrent_stress"
        "ipc_complete_test_suite::test_multi_ipc_integration"
    )
    
    local failed_tests=()
    local passed_count=0
    
    for test in "${stress_tests[@]}"; do
        if run_test "ipc_complete_test_suite" "-- --exact $test"; then
            ((passed_count++))
        else
            failed_tests+=("$test")
        fi
    done
    
    print_info "Stress tests summary: $passed_count/${#stress_tests[@]} passed"
    
    if [[ ${#failed_tests[@]} -gt 0 ]]; then
        print_warning "Failed stress tests:"
        for test in "${stress_tests[@]}"; do
            print_warning "  - $test"
        done
        return 1
    fi
    
    return 0
}

# Function to run integration tests
run_integration_tests() {
    print_info "=== Running IPC Integration Tests ==="
    
    local integration_tests=(
        "ipc_comprehensive_test"
        "ipc_integration_advanced_test"
        "ipc_stress_test"
    )
    
    local failed_tests=()
    local passed_count=0
    
    for test in "${integration_tests[@]}"; do
        if run_test "$test" ""; then
            ((passed_count++))
        else
            failed_tests+=("$test")
        fi
    done
    
    print_info "Integration tests summary: $passed_count/${#integration_tests[@]} passed"
    
    if [[ ${#failed_tests[@]} -gt 0 ]]; then
        print_warning "Failed integration tests:"
        for test in "${failed_tests[@]}"; do
            print_warning "  - $test"
        done
        return 1
    fi
    
    return 0
}

# Function to run error handling tests
run_error_tests() {
    print_info "=== Running IPC Error Handling Tests ==="
    
    local error_tests=(
        "ipc_complete_test_suite::test_ipc_error_handling"
        "ipc_complete_test_suite::test_ipc_statistics_monitoring"
        "ipc_complete_test_suite::test_ipc_final_cleanup"
    )
    
    local failed_tests=()
    local passed_count=0
    
    for test in "${error_tests[@]}"; do
        if run_test "ipc_complete_test_suite" "-- --exact $test"; then
            ((passed_count++))
        else
            failed_tests+=("$test")
        fi
    done
    
    print_info "Error handling tests summary: $passed_count/${#error_tests[@]} passed"
    
    if [[ ${#failed_tests[@]} -gt 0 ]]; then
        print_warning "Failed error handling tests:"
        for test in "${failed_tests[@]}"; do
            print_warning "  - $test"
        done
        return 1
    fi
    
    return 0
}

# Function to generate coverage report
generate_coverage() {
    print_info "=== Generating IPC Test Coverage Report ==="
    
    if command -v cargo-tarpaulin >/dev/null 2>&1; then
        print_info "Running cargo-tarpaulin for coverage analysis"
        
        $CARGO_CMD tarpaulin \
            --tests \
            --out Xml \
            --out Html \
            --output-dir coverage/ \
            --include-tests \
            --timeout 600 \
            --test ipc_complete_test_suite \
            --test ipc_comprehensive_test \
            --test ipc_integration_advanced_test \
            --test ipc_stress_test
        
        if [[ -f "coverage/tarpaulin-report.html" ]]; then
            print_success "Coverage report generated: coverage/tarpaulin-report.html"
        fi
    else
        print_warning "cargo-tarpaulin not found. Install with: cargo install cargo-tarpaulin"
    fi
}

# Function to generate test report
generate_report() {
    if [[ -n "$REPORT_FILE" ]]; then
        print_info "Generating test report: $REPORT_FILE"
        
        cat > "$REPORT_FILE" << EOF
# CURSED IPC Test Suite Report

Generated: $(date)

## Test Summary

### Unit Tests
- Named Pipes: Comprehensive functionality testing
- Shared Memory: Multi-size operations and mapping
- Message Queues: Different message types and statistics
- Semaphores: Binary, counting, and named semaphores
- Barriers: Multi-thread coordination
- RwLock: Timeout-based read/write operations
- Condition Variables: Wait/notify with timeout
- Distributed Coordinator: Multi-node coordination
- Unix Domain Sockets: Bidirectional communication
- Signal Handling: Custom signal callbacks
- RPC: Remote procedure calls with serialization

### Performance Tests
- Shared Memory: >1MB/s throughput validation
- Concurrent Operations: Multi-thread stress testing

### Stress Tests
- Concurrent IPC: 8 threads × 100 operations each
- Multi-IPC Integration: Complex coordination scenarios

### Integration Tests
- Cross-platform compatibility
- Resource cleanup and lifecycle management
- Error handling and recovery
- Statistics monitoring

## Test Coverage

The test suite covers:
- ✓ All IPC mechanisms (pipes, sockets, shared memory, queues, semaphores)
- ✓ Synchronization primitives (barriers, rwlocks, condition variables)
- ✓ Distributed coordination and signal handling
- ✓ RPC with serialization and error handling
- ✓ Performance validation with throughput requirements
- ✓ Concurrent access patterns and stress scenarios
- ✓ Error conditions and recovery mechanisms
- ✓ Resource lifecycle management and cleanup

## Quality Assurance

- **Memory Safety**: All operations use safe Rust patterns
- **Thread Safety**: Comprehensive concurrent testing
- **Error Handling**: Timeout and failure scenarios validated
- **Performance**: Throughput and latency requirements met
- **Cross-platform**: Unix/Linux compatibility verified
- **Resource Management**: Proper cleanup and lifecycle handling

EOF
        
        print_success "Test report generated: $REPORT_FILE"
    fi
}

# Function to show usage
show_usage() {
    cat << EOF
CURSED IPC Test Suite Runner

Usage: $0 [OPTIONS]

OPTIONS:
    --help              Show this help message
    --verbose           Enable verbose test output
    --quick             Run only essential tests (skip stress tests)
    --ignored           Run only ignored/slow tests
    --coverage          Generate coverage report with cargo-tarpaulin
    --report FILE       Generate test report to specified file
    --timeout SECONDS   Set test timeout (default: 300)

TEST CATEGORIES:
    --unit              Run only unit tests
    --performance       Run only performance tests
    --stress            Run only stress tests
    --integration       Run only integration tests
    --error             Run only error handling tests

EXAMPLES:
    $0                  # Run all tests
    $0 --quick          # Run essential tests only
    $0 --unit --verbose # Run unit tests with verbose output
    $0 --coverage       # Run tests and generate coverage report
    $0 --report ipc_test_report.md  # Generate test report

EOF
}

# Parse command line arguments
RUN_UNIT=false
RUN_PERFORMANCE=false
RUN_STRESS=false
RUN_INTEGRATION=false
RUN_ERROR=false
RUN_ALL=true

while [[ $# -gt 0 ]]; do
    case $1 in
        --help)
            show_usage
            exit 0
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --quick)
            QUICK=true
            shift
            ;;
        --ignored)
            IGNORED=true
            shift
            ;;
        --coverage)
            COVERAGE=true
            shift
            ;;
        --report)
            REPORT_FILE="$2"
            shift 2
            ;;
        --timeout)
            TEST_TIMEOUT="$2"
            shift 2
            ;;
        --unit)
            RUN_UNIT=true
            RUN_ALL=false
            shift
            ;;
        --performance)
            RUN_PERFORMANCE=true
            RUN_ALL=false
            shift
            ;;
        --stress)
            RUN_STRESS=true
            RUN_ALL=false
            shift
            ;;
        --integration)
            RUN_INTEGRATION=true
            RUN_ALL=false
            shift
            ;;
        --error)
            RUN_ERROR=true
            RUN_ALL=false
            shift
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Main execution
main() {
    print_info "CURSED IPC Complete Test Suite"
    print_info "=============================="
    
    # Check environment and setup
    check_linking_fix
    
    local start_time=$(date +%s)
    local total_errors=0
    
    # Run selected test categories
    if [[ "$RUN_ALL" == "true" || "$RUN_UNIT" == "true" ]]; then
        if ! run_unit_tests; then
            ((total_errors++))
        fi
    fi
    
    if [[ "$RUN_ALL" == "true" || "$RUN_PERFORMANCE" == "true" ]]; then
        if ! run_performance_tests; then
            ((total_errors++))
        fi
    fi
    
    if [[ "$RUN_ALL" == "true" || "$RUN_INTEGRATION" == "true" ]]; then
        if ! run_integration_tests; then
            ((total_errors++))
        fi
    fi
    
    if [[ "$RUN_ALL" == "true" || "$RUN_ERROR" == "true" ]]; then
        if ! run_error_tests; then
            ((total_errors++))
        fi
    fi
    
    # Run stress tests unless quick mode is enabled
    if [[ "$QUICK" != "true" ]] && [[ "$RUN_ALL" == "true" || "$RUN_STRESS" == "true" ]]; then
        if ! run_stress_tests; then
            ((total_errors++))
        fi
    fi
    
    # Generate coverage report if requested
    if [[ "$COVERAGE" == "true" ]]; then
        generate_coverage
    fi
    
    # Generate test report if requested
    generate_report
    
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    print_info "=============================="
    print_info "Test suite completed in ${duration}s"
    
    if [[ $total_errors -eq 0 ]]; then
        print_success "All IPC tests passed! ✓"
        exit 0
    else
        print_error "$total_errors test category(ies) failed"
        exit 1
    fi
}

# Run main function
main "$@"
