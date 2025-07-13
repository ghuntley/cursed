#!/bin/bash
# Minimal test execution for basic validation
# Target: Complete in under 30 seconds

echo "⚡ Running minimal CURSED test suite..."
start_time=$(date +%s)

# Set aggressive parallel execution
export RUST_TEST_THREADS=$(nproc)
export CARGO_BUILD_JOBS=$(nproc)

echo "📊 Using $RUST_TEST_THREADS test threads and $CARGO_BUILD_JOBS build jobs"

# Run only basic unit tests, exclude expensive modules
echo "🔧 Running basic unit tests only..."
timeout 30 cargo test --lib lexer parser semantic \
    -- --test-threads=$RUST_TEST_THREADS

test_result=$?
end_time=$(date +%s)
duration=$((end_time - start_time))

echo ""
echo "⏱️  Minimal test execution completed in ${duration} seconds"

if [ $test_result -eq 0 ]; then
    echo "✅ Basic tests passed!"
elif [ $test_result -eq 124 ]; then
    echo "⏰ Tests timed out after 30 seconds"
else
    echo "❌ Some tests failed (exit code: $test_result)"
fi

echo ""
echo "🎯 Next steps:"
echo "   - Run 'cargo test parser' for parser tests"
echo "   - Run 'cargo test lexer' for lexer tests" 
echo "   - Run 'cargo test semantic' for semantic tests"
echo "   - Run full suite when all basics pass"

exit $test_result
