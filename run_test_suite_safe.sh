#!/bin/bash
# Safe Test Suite Runner with Hang Prevention
# P1-HIGH: Ensures test suite completes without hanging

set -e

echo "🔧 Building CURSED compiler with hang prevention fixes..."
timeout 30s zig build || {
    echo "❌ Build timeout after 30 seconds"
    exit 1
}

echo "✅ Build completed successfully"

echo "🧪 Running hang prevention validation test..."
timeout 15s ./zig-out/bin/cursed test_hang_prevention.csd || {
    echo "❌ Hang prevention test timeout after 15 seconds"
    exit 1
}

echo "🧪 Running basic functionality tests..."
timeout 10s ./zig-out/bin/cursed basic_test.csd || {
    echo "❌ Basic test timeout after 10 seconds"
    exit 1
}

echo "🧪 Running stdlib tests with timeout safety..."
timeout 30s ./zig-out/bin/cursed comprehensive_stdlib_test.csd || {
    echo "❌ Stdlib test timeout after 30 seconds"
    exit 1
}

echo "🧪 Running GC stress test with bounds..."
timeout 20s ./zig-out/bin/cursed basic_memory_test.csd || {
    echo "❌ GC stress test timeout after 20 seconds"  
    exit 1
}

echo "🧪 Running concurrency tests with timeout safety..."
timeout 25s ./zig-out/bin/cursed basic_concurrency_test.csd || {
    echo "❌ Concurrency test timeout after 25 seconds"
    exit 1
}

echo "🧪 Running unit tests with bounded execution..."
timeout 45s zig build test || {
    echo "❌ Unit test timeout after 45 seconds"
    exit 1
}

echo "🎉 All tests completed successfully without hanging!"
echo "⏱️  Total execution time: $(date)"
echo "✅ Test suite is now CI/CD safe with reliable execution"
