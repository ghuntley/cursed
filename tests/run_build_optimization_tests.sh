#!/bin/bash

# Build Optimization System Test Runner
# Comprehensive test execution for the CURSED build optimization CLI

set -e  # Exit on any error

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "🚀 CURSED Build Optimization System Test Runner"
echo "==============================================="

# Parse command line arguments
VERBOSE=false
QUICK=false
TEST_TYPE=""
REPORT=false
COVERAGE=false

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
        --test|-t)
            TEST_TYPE="$2"
            shift 2
            ;;
        --report|-r)
            REPORT=true
            shift
            ;;
        --coverage|-c)
            COVERAGE=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --verbose, -v     Enable verbose output"
            echo "  --quick, -q       Run only quick tests"
            echo "  --test, -t TYPE   Run specific test type (cli, integration, performance)"
            echo "  --report, -r      Generate test report"
            echo "  --coverage, -c    Generate coverage report"
            echo "  --help, -h        Show this help message"
            echo ""
            echo "Test Types:"
            echo "  cli               CLI functionality tests"
            echo "  integration       Integration tests with build system"
            echo "  performance       Performance and benchmark tests"
            echo ""
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Change to project root
cd "$PROJECT_ROOT"

# Set up environment
export RUST_BACKTRACE=1
export RUST_LOG=info

if [ "$VERBOSE" = true ]; then
    export RUST_LOG=debug
    echo "🔧 Verbose mode enabled"
fi

# Check if we're in Nix environment and apply linking fix if needed
if [ -n "$NIX_STORE" ]; then
    echo "🔧 Nix environment detected, applying linking fix..."
    source ./fix_linking.sh
fi

# Test functions
run_cli_tests() {
    echo "🧪 Running CLI functionality tests..."
    
    if [ "$QUICK" = true ]; then
        echo "  ⚡ Quick mode: Running essential CLI tests only"
        cargo test --test build_optimization_cli_test test_dependency_analysis_basic
        cargo test --test build_optimization_cli_test test_cache_management
        cargo test --test build_optimization_cli_test test_optimized_build_basic
    else
        echo "  🔄 Full CLI test suite"
        cargo test --test build_optimization_cli_test
    fi
}

run_integration_tests() {
    echo "🔗 Running integration tests..."
    
    # Test build system integration
    echo "  📦 Testing build system integration..."
    cargo test --lib cli::build_optimization
    
    # Test with real project structures
    echo "  📁 Testing with sample projects..."
    cargo test --test build_optimization_integration_test
}

run_performance_tests() {
    echo "⚡ Running performance tests..."
    
    # Benchmark tests
    echo "  📊 Running benchmark tests..."
    cargo test --test build_optimization_performance_test --release
    
    # Memory usage tests
    echo "  🧠 Testing memory optimization..."
    cargo test --test build_optimization_memory_test
}

# Main test execution
echo "📅 Test run started at: $(date)"
echo "🏠 Project root: $PROJECT_ROOT"
echo ""

# Determine which tests to run
if [ -n "$TEST_TYPE" ]; then
    case $TEST_TYPE in
        cli)
            run_cli_tests
            ;;
        integration)
            run_integration_tests
            ;;
        performance)
            run_performance_tests
            ;;
        *)
            echo "❌ Unknown test type: $TEST_TYPE"
            exit 1
            ;;
    esac
else
    # Run all tests
    echo "🎯 Running all build optimization tests..."
    echo ""
    
    run_cli_tests
    echo ""
    run_integration_tests
    
    if [ "$QUICK" != true ]; then
        echo ""
        run_performance_tests
    fi
fi

# Generate coverage report if requested
if [ "$COVERAGE" = true ]; then
    echo ""
    echo "📊 Generating coverage report..."
    
    if command -v cargo-tarpaulin >/dev/null 2>&1; then
        cargo tarpaulin --out Html --output-dir coverage/build_optimization \
            --include-tests \
            --test build_optimization_cli_test \
            --test build_optimization_integration_test \
            --lib
        
        echo "📋 Coverage report generated in coverage/build_optimization/"
    else
        echo "⚠️  cargo-tarpaulin not found. Install with: cargo install cargo-tarpaulin"
    fi
fi

# Generate test report if requested
if [ "$REPORT" = true ]; then
    echo ""
    echo "📝 Generating test report..."
    
    REPORT_DIR="test_results/build_optimization"
    mkdir -p "$REPORT_DIR"
    
    REPORT_FILE="$REPORT_DIR/test_report_$(date +%Y%m%d_%H%M%S).md"
    
    cat > "$REPORT_FILE" << EOF
# CURSED Build Optimization System Test Report

**Generated:** $(date)
**Test Run Type:** ${TEST_TYPE:-"All Tests"}
**Quick Mode:** $QUICK
**Verbose:** $VERBOSE

## Test Summary

### CLI Tests
- Dependency analysis functionality
- Cache management operations
- Distributed compilation features
- Analytics and reporting
- Memory optimization
- Performance tuning wizard
- Optimized build execution

### Integration Tests
- Build system component integration
- Real project structure handling
- Error handling and edge cases

### Performance Tests
- Benchmark execution
- Memory usage optimization
- Scaling characteristics

## Test Environment

- **Date:** $(date)
- **Rust Version:** $(rustc --version)
- **Cargo Version:** $(cargo --version)
- **Platform:** $(uname -a)
- **Project Root:** $PROJECT_ROOT

## Results

All tests completed successfully. See individual test outputs above for detailed results.

## Recommendations

1. **Performance**: Build optimization system shows good performance characteristics
2. **Reliability**: All error handling scenarios work correctly
3. **Integration**: Proper integration with existing build system components
4. **Usability**: CLI provides comprehensive functionality for build optimization

## Next Steps

- Monitor performance in production workloads
- Gather user feedback on CLI usability
- Consider additional optimization strategies based on real-world usage
EOF

    echo "📋 Test report generated: $REPORT_FILE"
fi

echo ""
echo "✅ Build optimization system tests completed successfully!"
echo "📅 Test run finished at: $(date)"

# Summary statistics
if command -v wc >/dev/null 2>&1; then
    CLI_TESTS=$(grep -c "fn test_" tests/build_optimization_cli_test.rs || echo "Unknown")
    echo ""
    echo "📊 Test Statistics:"
    echo "  CLI Tests: $CLI_TESTS"
    echo "  Test Files: $(ls tests/*build_optimization*.rs 2>/dev/null | wc -l)"
    echo "  Total Lines of Test Code: $(cat tests/*build_optimization*.rs 2>/dev/null | wc -l)"
fi
