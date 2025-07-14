#!/bin/bash

# Local Self-Hosting Testing Script
# Run this script to test self-hosting validation locally

set -euo pipefail

echo "🧪 Local Self-Hosting Testing"
echo "============================="

# Build compiler first
echo "🏗️  Building compiler..."
cargo build --release --bin cursed

# Run fast tests for quick validation
echo "🚀 Running fast test suite..."
if [ -f "run_fast_tests_final.sh" ]; then
    bash run_fast_tests_final.sh
else
    echo "⚠️  Fast test suite not found, running cargo test..."
    cargo test --lib
fi

# Test individual CI components
echo "🔧 Testing CI components individually..."

echo "  1. Bootstrap validation..."
if timeout 120 bash ci/bootstrap_validation_tests.sh; then
    echo "    ✅ Bootstrap validation passed"
else
    echo "    ❌ Bootstrap validation failed"
fi

echo "  2. Performance regression detection..."
if timeout 120 bash ci/performance_regression_detection.sh; then
    echo "    ✅ Performance regression detection passed"
else
    echo "    ❌ Performance regression detection failed"
fi

echo "  3. Self-hosting validation..."
if timeout 300 bash ci/self_hosting_validation.sh; then
    echo "    ✅ Self-hosting validation passed"
else
    echo "    ❌ Self-hosting validation failed"
fi

echo "  4. Comprehensive self-hosting test suite..."
if timeout 300 bash ci/comprehensive_self_hosting_test_suite.sh; then
    echo "    ✅ Comprehensive self-hosting test suite passed"
else
    echo "    ❌ Comprehensive self-hosting test suite failed"
fi

echo ""
echo "🎉 Local self-hosting testing completed!"
echo "📊 Check generated reports for detailed results"
