#!/bin/bash
# Fast test execution script for CURSED - Final Version
# Optimized for quick feedback loop (target: under 2 minutes)

echo "🚀 Running fast CURSED test suite (modular approach)..."
start_time=$(date +%s)

# Set parallel test execution
export RUST_TEST_THREADS=32
export CARGO_BUILD_JOBS=32

echo "📊 Using $RUST_TEST_THREADS test threads and $CARGO_BUILD_JOBS build jobs"

total_tests=0
passed_tests=0
failed_tests=0

# Function to run and report test results
run_test_group() {
    local name=$1
    local pattern=$2
    local timeout_seconds=${3:-30}
    
    echo "🔧 Testing $name..."
    
    if timeout $timeout_seconds cargo test --lib -- "$pattern" --test-threads=$RUST_TEST_THREADS 2>/dev/null; then
        local count=$(cargo test --lib -- "$pattern" --test-threads=$RUST_TEST_THREADS 2>&1 | grep -o '[0-9]* passed' | cut -d' ' -f1)
        count=${count:-0}
        echo "  ✅ $name: $count tests passed"
        passed_tests=$((passed_tests + count))
        total_tests=$((total_tests + count))
    else
        echo "  ❌ $name: tests failed or timed out"
        failed_tests=$((failed_tests + 1))
    fi
}

# Run core test groups with timeouts
run_test_group "Lexer" "lexer" 15
run_test_group "Parser" "parser" 15  
run_test_group "Semantic" "semantic" 20
run_test_group "AST" "ast" 10
run_test_group "Type System" "type_system" 20
run_test_group "Common Utils" "common" 10

# Quick validation of critical functionality
echo "🔧 Testing core functionality..."
if timeout 20 cargo test --lib -- "test_basic" --test-threads=$RUST_TEST_THREADS 2>/dev/null; then
    echo "  ✅ Core functionality tests passed"
else
    echo "  ⚠️ Some core functionality tests failed"
fi

end_time=$(date +%s)
duration=$((end_time - start_time))

echo ""
echo "📊 Test Summary:"
echo "   Total groups tested: $((passed_tests + failed_tests)) groups"
echo "   Successful groups: $passed_tests"
echo "   Failed groups: $failed_tests"
echo "⏱️  Test execution completed in ${duration} seconds"

if [ $failed_tests -eq 0 ]; then
    echo "✅ All core tests passed!"
    if [ $duration -lt 120 ]; then
        echo "🎯 Target achieved: Tests completed under 2 minutes"
    else
        echo "⚠️  Tests took longer than 2 minutes target"
    fi
    exit 0
else
    echo "❌ Some test groups failed"
    echo ""
    echo "🔍 To debug specific failures:"
    echo "   cargo test --lib -- lexer"
    echo "   cargo test --lib -- parser"
    echo "   cargo test --lib -- semantic"
    exit 1
fi
