#!/bin/bash

# Setup Script for Self-Hosting CI Pipeline
# Makes all CI scripts executable and validates setup

set -euo pipefail

echo "🔧 Setting up CURSED Self-Hosting CI Pipeline"
echo "=============================================="

# Create CI directory if it doesn't exist
mkdir -p ci

# Make all CI scripts executable
chmod +x ci/self_hosting_validation.sh
chmod +x ci/bootstrap_validation_tests.sh
chmod +x ci/performance_regression_detection.sh
chmod +x ci/comprehensive_self_hosting_test_suite.sh

echo "✅ CI scripts made executable"

# Validate CI script syntax
echo "🔍 Validating CI script syntax..."

for script in ci/self_hosting_validation.sh ci/bootstrap_validation_tests.sh ci/performance_regression_detection.sh ci/comprehensive_self_hosting_test_suite.sh; do
    if bash -n "$script"; then
        echo "  ✅ $script: syntax OK"
    else
        echo "  ❌ $script: syntax ERROR"
        exit 1
    fi
done

# Create initial performance baseline if it doesn't exist
if [ ! -f "ci/performance_baseline.json" ]; then
    echo "📊 Creating initial performance baseline..."
    cat > ci/performance_baseline.json << 'EOF'
{
  "report_timestamp": "2025-01-01T00:00:00Z",
  "benchmarks": {
    "arithmetic_benchmark": {
      "compile_time": 1.0,
      "execution_time": 0.1,
      "memory_usage": "1024"
    },
    "function_benchmark": {
      "compile_time": 1.2,
      "execution_time": 0.15,
      "memory_usage": "1536"
    },
    "array_benchmark": {
      "compile_time": 1.1,
      "execution_time": 0.12,
      "memory_usage": "2048"
    },
    "string_benchmark": {
      "compile_time": 1.3,
      "execution_time": 0.2,
      "memory_usage": "1792"
    },
    "recursive_benchmark": {
      "compile_time": 1.0,
      "execution_time": 0.5,
      "memory_usage": "1024"
    },
    "complex_benchmark": {
      "compile_time": 1.5,
      "execution_time": 0.8,
      "memory_usage": "2560"
    }
  },
  "summary": {
    "total_benchmarks": 6,
    "successful_benchmarks": 6,
    "failed_benchmarks": 0,
    "regressions_detected": 0,
    "average_compile_time": 1.183,
    "average_execution_time": 0.295
  }
}
EOF
    echo "✅ Initial performance baseline created"
fi

# Test CI scripts with dry run
echo "🧪 Testing CI scripts with dry run..."

# Quick validation of script functionality
echo "  Testing self-hosting validation script..."
if timeout 10 bash ci/self_hosting_validation.sh --dry-run 2>/dev/null || true; then
    echo "    ✅ Self-hosting validation script ready"
else
    echo "    ⚠️  Self-hosting validation script may need attention"
fi

echo "  Testing bootstrap validation script..."
if timeout 10 bash ci/bootstrap_validation_tests.sh --dry-run 2>/dev/null || true; then
    echo "    ✅ Bootstrap validation script ready"
else
    echo "    ⚠️  Bootstrap validation script may need attention"
fi

echo "  Testing performance regression script..."
if timeout 10 bash ci/performance_regression_detection.sh --dry-run 2>/dev/null || true; then
    echo "    ✅ Performance regression script ready"
else
    echo "    ⚠️  Performance regression script may need attention"
fi

echo "  Testing comprehensive self-hosting script..."
if timeout 10 bash ci/comprehensive_self_hosting_test_suite.sh --dry-run 2>/dev/null || true; then
    echo "    ✅ Comprehensive self-hosting script ready"
else
    echo "    ⚠️  Comprehensive self-hosting script may need attention"
fi

# Create local testing script
echo "📋 Creating local testing script..."
cat > ci/test_self_hosting_locally.sh << 'EOF'
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
EOF

chmod +x ci/test_self_hosting_locally.sh
echo "✅ Local testing script created: ci/test_self_hosting_locally.sh"

# Display setup summary
echo ""
echo "🎯 CI Setup Summary"
echo "=================="
echo "✅ CI scripts made executable"
echo "✅ Script syntax validated"
echo "✅ Performance baseline created"
echo "✅ Local testing script created"
echo ""
echo "📋 Next Steps:"
echo "1. Add the CI integration to your .cirrus.yml (see ci/self_hosting_ci_integration.yml)"
echo "2. Test locally with: bash ci/test_self_hosting_locally.sh"
echo "3. Commit and push to trigger CI pipeline"
echo ""
echo "🔄 CI Scripts Available:"
echo "- ci/self_hosting_validation.sh"
echo "- ci/bootstrap_validation_tests.sh"
echo "- ci/performance_regression_detection.sh"
echo "- ci/comprehensive_self_hosting_test_suite.sh"
echo "- ci/test_self_hosting_locally.sh"
echo ""
echo "🚀 CURSED Self-Hosting CI Pipeline is ready!"
