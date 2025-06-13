#!/bin/bash

# Comprehensive test runner for CURSED Process Management and IPC systems
# 
# This script provides comprehensive testing capabilities for the process management
# and IPC systems with multiple execution modes, coverage analysis, and detailed reporting.

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Default values
VERBOSE=false
QUICK=false
STRESS=false
COVERAGE=false
REPORT_FILE=""
TEST_FILTER=""

# Show usage information
show_help() {
    cat << EOF
CURSED Process Management Test Runner

USAGE:
    $0 [OPTIONS]

OPTIONS:
    --quick              Run only quick integration tests
    --stress             Run stress tests (long duration)
    --all                Run all tests including stress tests
    --test <filter>      Run specific test matching filter
    --coverage           Generate code coverage report
    --report <file>      Generate detailed test report
    --verbose            Enable verbose output
    --help               Show this help message

EXAMPLES:
    $0                                    # Run standard integration tests
    $0 --quick                           # Quick validation tests
    $0 --stress                          # Run stress tests only
    $0 --all                             # Run everything including stress tests
    $0 --test "shared_memory"            # Run shared memory tests only
    $0 --coverage --report coverage.md  # Full coverage analysis with report
    $0 --verbose --all                   # Verbose output for all tests

TEST CATEGORIES:
    Integration Tests:
        - Basic process spawning and lifecycle
        - Process monitoring and resource tracking
        - Process control operations
        - IPC initialization and operations
        - Error handling and edge cases
        - Memory safety validation
        - Performance characteristics

    Stress Tests:
        - Massive process spawning (100+ processes)
        - Concurrent IPC operations (16 threads)
        - Memory pressure scenarios
        - Sustained process load
        - Resource exhaustion recovery
        - Performance degradation analysis
EOF
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose)
            VERBOSE=true
            shift
            ;;
        --quick)
            QUICK=true
            shift
            ;;
        --stress)
            STRESS=true
            shift
            ;;
        --all)
            STRESS=true
            shift
            ;;
        --test)
            TEST_FILTER="$2"
            shift 2
            ;;
        --coverage)
            COVERAGE=true
            shift
            ;;
        --report)
            REPORT_FILE="$2"
            shift 2
            ;;
        --help)
            show_help
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Function to print colored status messages
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Function to run linking fix wrapper
run_with_linking_fix() {
    if [[ -x "./fix_linking.sh" ]]; then
        ./fix_linking.sh "$@"
    else
        "$@"
    fi
}

# Initialize test environment
setup_test_environment() {
    print_status $BLUE "🔧 Setting up test environment..."
    
    # Ensure we're in the project root
    if [[ ! -f "Cargo.toml" ]]; then
        echo "Error: Must be run from project root directory"
        exit 1
    fi
    
    # Check if linking fix is needed and available
    if [[ -x "./fix_linking.sh" ]]; then
        print_status $BLUE "   Using linking fix for Nix environment"
    fi
    
    # Build the project first
    print_status $BLUE "   Building project..."
    if run_with_linking_fix cargo build --quiet; then
        print_status $GREEN "   ✅ Build successful"
    else
        print_status $RED "   ❌ Build failed"
        exit 1
    fi
}

# Run integration tests
run_integration_tests() {
    print_status $BLUE "🧪 Running Process Management Integration Tests..."
    
    local cargo_args=(test --test process_management_integration_test)
    
    if [[ -n "$TEST_FILTER" ]]; then
        cargo_args+=(-- "$TEST_FILTER")
    fi
    
    if [[ "$VERBOSE" == "true" ]]; then
        cargo_args+=(--nocapture)
    fi
    
    if run_with_linking_fix cargo "${cargo_args[@]}"; then
        print_status $GREEN "   ✅ Integration tests passed"
        return 0
    else
        print_status $RED "   ❌ Integration tests failed"
        return 1
    fi
}

# Run stress tests
run_stress_tests() {
    print_status $BLUE "🔥 Running Process Management Stress Tests..."
    print_status $YELLOW "   Warning: These tests may take several minutes and consume significant resources"
    
    local cargo_args=(test --test process_management_stress_test -- --ignored)
    
    if [[ -n "$TEST_FILTER" ]]; then
        cargo_args+=("$TEST_FILTER")
    fi
    
    if [[ "$VERBOSE" == "true" ]]; then
        cargo_args+=(--nocapture)
    fi
    
    if run_with_linking_fix cargo "${cargo_args[@]}"; then
        print_status $GREEN "   ✅ Stress tests passed"
        return 0
    else
        print_status $RED "   ❌ Stress tests failed"
        return 1
    fi
}

# Generate coverage report
generate_coverage() {
    print_status $BLUE "📊 Generating code coverage report..."
    
    # Check if cargo-tarpaulin is installed
    if ! command -v cargo-tarpaulin &> /dev/null; then
        print_status $YELLOW "   Installing cargo-tarpaulin..."
        cargo install cargo-tarpaulin || {
            print_status $RED "   ❌ Failed to install cargo-tarpaulin"
            return 1
        }
    fi
    
    local coverage_args=(
        tarpaulin
        --tests
        --out Html
        --output-dir target/tarpaulin
        --include-tests
        --exclude-files "tests/*"
        --ignore-panics
    )
    
    # Include specific test files
    coverage_args+=(--test process_management_integration_test)
    
    if [[ "$STRESS" == "true" ]]; then
        coverage_args+=(--test process_management_stress_test)
        coverage_args+=(-- --ignored)
    fi
    
    if run_with_linking_fix cargo "${coverage_args[@]}"; then
        print_status $GREEN "   ✅ Coverage report generated: target/tarpaulin/tarpaulin-report.html"
        return 0
    else
        print_status $RED "   ❌ Coverage generation failed"
        return 1
    fi
}

# Generate detailed test report
generate_report() {
    local report_file="$1"
    print_status $BLUE "📝 Generating detailed test report: $report_file"
    
    cat > "$report_file" << EOF
# CURSED Process Management Test Report

Generated: $(date)

## Test Suite Overview

The CURSED Process Management and IPC system test suite provides comprehensive validation
of process spawning, lifecycle management, inter-process communication, and system
integration capabilities.

### Test Categories

#### Integration Tests
- **Basic Process Operations**: Spawning, execution, and lifecycle management
- **Process Monitoring**: Resource usage tracking and system information
- **Process Control**: Priority management, signal handling, and termination
- **IPC Operations**: Shared memory, message queues, pipes, and semaphores
- **Error Handling**: Edge cases and failure scenarios
- **Memory Safety**: Resource cleanup and bounds checking
- **Performance**: Baseline performance characteristics

#### Stress Tests
- **Massive Process Spawning**: 100+ concurrent processes
- **Concurrent IPC Operations**: 16 threads × 50 operations each
- **Memory Pressure**: Large shared memory allocations
- **Sustained Load**: 30-second continuous process spawning
- **Resource Exhaustion**: File descriptor and process limits
- **Performance Degradation**: Load scaling analysis

## Test Execution Results

EOF

    # Add current timestamp and system info
    echo "### System Information" >> "$report_file"
    echo "" >> "$report_file"
    echo "- **OS**: $(uname -s) $(uname -r)" >> "$report_file"
    echo "- **Architecture**: $(uname -m)" >> "$report_file"
    echo "- **CPU Cores**: $(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 'Unknown')" >> "$report_file"
    echo "- **Memory**: $(free -h 2>/dev/null | grep '^Mem:' | awk '{print $2}' || echo 'Unknown')" >> "$report_file"
    echo "- **Rust Version**: $(rustc --version)" >> "$report_file"
    echo "" >> "$report_file"
    
    # Add test results summary
    echo "### Test Summary" >> "$report_file"
    echo "" >> "$report_file"
    
    if [[ "$QUICK" == "true" ]]; then
        echo "- **Mode**: Quick integration tests only" >> "$report_file"
    elif [[ "$STRESS" == "true" ]]; then
        echo "- **Mode**: Full test suite including stress tests" >> "$report_file"
    else
        echo "- **Mode**: Standard integration tests" >> "$report_file"
    fi
    
    echo "- **Filter**: ${TEST_FILTER:-None}" >> "$report_file"
    echo "- **Coverage**: ${COVERAGE:-false}" >> "$report_file"
    echo "" >> "$report_file"
    
    # Add detailed test descriptions
    cat >> "$report_file" << EOF
### Test Descriptions

#### Integration Tests

1. **test_basic_process_spawning**
   - Validates basic process creation and execution
   - Tests command-line argument passing
   - Verifies output capture and parsing
   - **Expected**: Process executes successfully with correct output

2. **test_process_lifecycle_management**
   - Tests long-running process spawning and monitoring
   - Validates process state tracking (running/terminated)
   - Tests process termination and cleanup
   - **Expected**: Full lifecycle management without resource leaks

3. **test_process_monitoring_and_resources**
   - Validates process information retrieval (PID, name, etc.)
   - Tests resource usage tracking (memory, CPU, file descriptors)
   - Verifies system information functions
   - **Expected**: Accurate resource monitoring and system info

4. **test_process_control_operations**
   - Tests process existence checking
   - Validates priority management operations
   - Tests command existence and path resolution
   - **Expected**: Proper process control with appropriate permissions

5. **test_process_timeout_handling**
   - Validates timeout mechanisms for long-running processes
   - Tests proper process termination on timeout
   - Verifies error reporting for timeout scenarios
   - **Expected**: Reliable timeout handling without resource leaks

6. **test_process_io_redirection**
   - Tests stdin/stdout/stderr redirection
   - Validates pipe-based communication
   - Tests output capture and parsing
   - **Expected**: Proper I/O redirection and data integrity

7. **test_process_environment_handling**
   - Tests environment variable management
   - Validates custom environment setup
   - Tests environment inheritance and isolation
   - **Expected**: Correct environment variable handling

8. **test_ipc_initialization**
   - Validates IPC subsystem startup and shutdown
   - Tests resource initialization and cleanup
   - Verifies statistics collection
   - **Expected**: Clean IPC subsystem lifecycle

9. **test_shared_memory_operations**
   - Tests shared memory creation and management
   - Validates data writing and reading operations
   - Tests memory view creation and access
   - **Expected**: Reliable shared memory operations

10. **test_named_pipe_communication**
    - Tests named pipe creation and communication
    - Validates bidirectional data transfer
    - Tests pipe cleanup and resource management
    - **Expected**: Proper pipe communication without data loss

11. **test_message_queue_operations**
    - Tests message queue creation and operations
    - Validates message sending and receiving
    - Tests priority-based message handling
    - **Expected**: Reliable message queue operations

12. **test_semaphore_synchronization**
    - Tests semaphore creation and operations
    - Validates acquire/release mechanisms
    - Tests value tracking and limits
    - **Expected**: Proper synchronization primitives

13. **test_concurrent_process_operations**
    - Tests multiple concurrent process spawning
    - Validates thread safety of process operations
    - Tests resource contention handling
    - **Expected**: High success rate under concurrent load

14. **test_process_signal_handling** (Unix only)
    - Tests signal registration and handling
    - Validates signal delivery and processing
    - Tests signal-based inter-process communication
    - **Expected**: Reliable signal handling mechanisms

15. **test_error_handling_edge_cases**
    - Tests error scenarios and edge cases
    - Validates proper error reporting and types
    - Tests recovery from failure conditions
    - **Expected**: Graceful error handling and recovery

16. **test_performance_characteristics**
    - Measures baseline performance metrics
    - Tests operation timing and throughput
    - Validates performance consistency
    - **Expected**: Acceptable performance within defined limits

17. **test_memory_safety_validation**
    - Tests memory bounds checking
    - Validates resource cleanup and leak prevention
    - Tests handling of invalid operations
    - **Expected**: Memory safety without crashes or leaks

#### Stress Tests (--stress or --all mode)

1. **test_massive_process_spawning**
   - Spawns 100+ processes in batches
   - Tests system limits and resource management
   - Validates high-volume process operations
   - **Expected**: ≥85% success rate under load

2. **test_concurrent_ipc_operations**
   - 16 threads × 50 operations each (800 total operations)
   - Tests all IPC mechanisms concurrently
   - Validates thread safety and performance
   - **Expected**: ≥90% success rate for all IPC types

3. **test_memory_pressure_scenarios**
   - Creates 50+ shared memory regions (1MB each)
   - Tests memory allocation limits
   - Validates graceful degradation under pressure
   - **Expected**: Successful creation of ≥20 regions

4. **test_sustained_process_load**
   - 30-second continuous process spawning
   - Tests system stability under sustained load
   - Validates resource cleanup and management
   - **Expected**: ≥80% process completion rate

5. **test_resource_exhaustion_recovery**
   - Tests file descriptor and process limits
   - Validates recovery after resource release
   - Tests graceful handling of exhaustion
   - **Expected**: Successful recovery and continued operation

6. **test_performance_degradation_analysis**
   - Tests performance at load levels 1, 5, 10, 20, 50
   - Measures operation timing and success rates
   - Validates performance scaling characteristics
   - **Expected**: <10x performance degradation, ≥90% success rates

### Success Criteria

- **Integration Tests**: All tests must pass for basic functionality
- **Stress Tests**: Success rates must meet defined thresholds
- **Performance**: Operations must complete within acceptable time limits
- **Memory Safety**: No memory leaks or crashes under normal and stress conditions
- **Cross-Platform**: Tests must pass on Unix and Windows (where applicable)

### Coverage Goals

- **Function Coverage**: ≥90% of process management functions
- **Line Coverage**: ≥85% of critical code paths
- **Branch Coverage**: ≥80% of conditional logic
- **Error Path Coverage**: ≥75% of error handling paths

EOF

    print_status $GREEN "   ✅ Test report generated: $report_file"
}

# Main execution function
main() {
    print_status $BLUE "🚀 Starting CURSED Process Management Test Suite"
    
    setup_test_environment
    
    local overall_success=true
    
    # Run integration tests (unless only stress tests requested)
    if [[ "$STRESS" != "true" ]] || [[ -n "$TEST_FILTER" ]]; then
        if ! run_integration_tests; then
            overall_success=false
        fi
    fi
    
    # Run stress tests if requested
    if [[ "$STRESS" == "true" ]]; then
        if ! run_stress_tests; then
            overall_success=false
        fi
    fi
    
    # Generate coverage report if requested
    if [[ "$COVERAGE" == "true" ]]; then
        if ! generate_coverage; then
            print_status $YELLOW "   ⚠️  Coverage generation failed, but continuing..."
        fi
    fi
    
    # Generate detailed report if requested
    if [[ -n "$REPORT_FILE" ]]; then
        generate_report "$REPORT_FILE"
    fi
    
    # Print final results
    echo ""
    if [[ "$overall_success" == "true" ]]; then
        print_status $GREEN "🎉 All Process Management tests completed successfully!"
    else
        print_status $RED "💥 Some Process Management tests failed!"
        exit 1
    fi
    
    # Print helpful information
    echo ""
    print_status $BLUE "📋 Additional Information:"
    echo "   • Run with --help for usage options"
    echo "   • Use --stress to run comprehensive stress tests"
    echo "   • Use --coverage to generate detailed coverage reports"
    echo "   • Use --report <file> to generate detailed test documentation"
    echo ""
}

# Run the main function
main "$@"
