#!/bin/bash
# Fast test execution script for CURSED
# Optimized for quick feedback loop (target: under 2 minutes)

echo "🚀 Running fast CURSED test suite..."
start_time=$(date +%s)

# Set parallel test execution
export RUST_TEST_THREADS=$(nproc)
export CARGO_BUILD_JOBS=$(nproc)

echo "📊 Using $RUST_TEST_THREADS test threads and $CARGO_BUILD_JOBS build jobs"

# Run only library and binary tests (skip integration tests)
echo "🔧 Running library and binary tests..."
timeout 60 cargo test --lib --bins -- --test-threads=$RUST_TEST_THREADS

test_result=$?
end_time=$(date +%s)
duration=$((end_time - start_time))

echo ""
echo "⏱️  Test execution completed in ${duration} seconds"

if [ $test_result -eq 0 ]; then
    echo "✅ All fast tests passed!"
    if [ $duration -lt 120 ]; then
        echo "🎯 Target achieved: Tests completed under 2 minutes"
    else
        echo "⚠️  Tests took longer than 2 minutes target"
    fi
else
    echo "❌ Some tests failed (exit code: $test_result)"
fi

exit $test_result
