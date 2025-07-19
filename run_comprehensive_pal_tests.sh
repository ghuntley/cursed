#!/bin/bash
# run_comprehensive_pal_tests.sh - Complete PAL testing pipeline

set -e  # Exit on any error

echo "🚀 CURSED PAL Comprehensive Testing Pipeline"
echo "============================================"

# Configuration
TIMESTAMP=$(date '+%Y%m%d_%H%M%S')
RESULTS_DIR="pal_test_results_${TIMESTAMP}"
LOG_FILE="${RESULTS_DIR}/pal_tests.log"

# Create results directory
mkdir -p "$RESULTS_DIR"

# Function to log and display
log_and_display() {
    echo "$1" | tee -a "$LOG_FILE"
}

# Start logging
log_and_display "🔧 Starting PAL comprehensive test suite at $(date)"
log_and_display "📂 Results will be saved to: $RESULTS_DIR"

# Phase 0: Validation
log_and_display ""
log_and_display "📋 Phase 0: Integration Validation"
log_and_display "=================================="

if ! ./validate_pal_integration.sh 2>&1 | tee -a "$LOG_FILE"; then
    log_and_display "❌ PAL integration validation failed"
    exit 1
fi

# Phase 1: Build and basic checks
log_and_display ""
log_and_display "🔨 Phase 1: Build System Validation"
log_and_display "==================================="

log_and_display "Building CURSED compiler..."
if cargo build 2>&1 | tee -a "$LOG_FILE"; then
    log_and_display "✅ Build successful"
else
    log_and_display "❌ Build failed"
    exit 1
fi

# Phase 2: Cross-compilation tests
log_and_display ""
log_and_display "🌐 Phase 2: Cross-Platform Compilation"
log_and_display "======================================"

if ./test_cross_compilation.sh 2>&1 | tee -a "$LOG_FILE"; then
    log_and_display "✅ Cross-compilation tests completed"
else
    log_and_display "⚠️  Some cross-compilation targets failed"
fi

# Phase 3: Feature detection tests
log_and_display ""
log_and_display "🔍 Phase 3: Feature Detection Tests"
log_and_display "===================================="

if cargo run --bin cursed test_feature_detection.csd 2>&1 | tee -a "$LOG_FILE"; then
    log_and_display "✅ Feature detection test passed"
else
    log_and_display "⚠️  Feature detection test had issues"
fi

# Phase 4: Memory management tests
log_and_display ""
log_and_display "🧠 Phase 4: Memory Management Tests"
log_and_display "===================================="

MEMORY_TESTS=(
    "test_memory_pal.csd"
    "test_memory_alignment.csd"
    "test_large_pages.csd"
)

for test in "${MEMORY_TESTS[@]}"; do
    log_and_display "Running $test..."
    if cargo run --bin cursed "$test" 2>&1 | tee -a "$LOG_FILE"; then
        log_and_display "✅ $test passed"
    else
        log_and_display "❌ $test failed"
    fi
done

# Phase 5: Scheduler tests
log_and_display ""
log_and_display "⚡ Phase 5: Scheduler Tests"
log_and_display "=========================="

SCHEDULER_TESTS=(
    "test_scheduler_pal.csd"
)

# Platform-specific scheduler tests
if [[ $(uname -m) == "arm64" && $(uname -s) == "Darwin" ]]; then
    SCHEDULER_TESTS+=("test_apple_silicon_cores.csd")
    log_and_display "🍎 Adding Apple Silicon specific tests"
fi

if [[ $(uname -s) == "Linux" ]]; then
    SCHEDULER_TESTS+=("test_numa_scheduling.csd")
    log_and_display "🐧 Adding Linux NUMA tests"
fi

for test in "${SCHEDULER_TESTS[@]}"; do
    log_and_display "Running $test..."
    if cargo run --bin cursed "$test" 2>&1 | tee -a "$LOG_FILE"; then
        log_and_display "✅ $test passed"
    else
        log_and_display "❌ $test failed"
    fi
done

# Phase 6: Hardware feature tests
log_and_display ""
log_and_display "🔧 Phase 6: Hardware Feature Tests"
log_and_display "=================================="

HARDWARE_TESTS=(
    "test_simd_features.csd"
    "test_crypto_acceleration.csd"
)

for test in "${HARDWARE_TESTS[@]}"; do
    log_and_display "Running $test..."
    if cargo run --bin cursed "$test" 2>&1 | tee -a "$LOG_FILE"; then
        log_and_display "✅ $test passed"
    else
        log_and_display "❌ $test failed"
    fi
done

# Phase 7: WebAssembly tests (if available)
if command -v wasmtime &> /dev/null; then
    log_and_display ""
    log_and_display "🕸️ Phase 7: WebAssembly Tests"
    log_and_display "============================="
    
    log_and_display "Building for WebAssembly..."
    if cargo build --target wasm32-wasi 2>&1 | tee -a "$LOG_FILE"; then
        log_and_display "✅ WASM build successful"
        
        WASM_TESTS=(
            "test_wasm_memory.csd"
            "test_wasm_scheduling.csd"
        )
        
        for test in "${WASM_TESTS[@]}"; do
            log_and_display "Running WASM $test..."
            if wasmtime target/wasm32-wasi/debug/cursed.wasm "$test" 2>&1 | tee -a "$LOG_FILE"; then
                log_and_display "✅ WASM $test passed"
            else
                log_and_display "❌ WASM $test failed"
            fi
        done
    else
        log_and_display "❌ WASM build failed"
    fi
else
    log_and_display ""
    log_and_display "⚠️  WebAssembly tests skipped (wasmtime not available)"
fi

# Phase 8: Stress tests
log_and_display ""
log_and_display "💪 Phase 8: Stress Tests"
log_and_display "========================"

STRESS_TESTS=(
    "test_memory_stress.csd"
    "test_scheduler_stress.csd"
)

for test in "${STRESS_TESTS[@]}"; do
    log_and_display "Running $test..."
    if timeout 60 cargo run --bin cursed "$test" 2>&1 | tee -a "$LOG_FILE"; then
        log_and_display "✅ $test passed"
    else
        log_and_display "❌ $test failed or timed out"
    fi
done

# Phase 9: Performance benchmarks
log_and_display ""
log_and_display "📊 Phase 9: Performance Benchmarks"
log_and_display "==================================="

log_and_display "Running performance benchmark..."
if cargo run --bin cursed benchmark_pal_performance.csd 2>&1 | tee -a "$LOG_FILE"; then
    log_and_display "✅ Performance benchmark completed"
else
    log_and_display "❌ Performance benchmark failed"
fi

# Phase 10: Results analysis
log_and_display ""
log_and_display "📈 Phase 10: Results Analysis"
log_and_display "============================="

log_and_display "Analyzing test results..."
if python3 analyze_pal_results.py "$LOG_FILE" > "${RESULTS_DIR}/analysis_report.txt" 2>&1; then
    log_and_display "✅ Results analysis completed"
    cat "${RESULTS_DIR}/analysis_report.txt" | tee -a "$LOG_FILE"
else
    log_and_display "⚠️  Results analysis had issues"
fi

# Generate test summary
log_and_display ""
log_and_display "📋 Test Summary"
log_and_display "==============="

TOTAL_TESTS=$(grep -c "Running.*\.csd" "$LOG_FILE" || echo "0")
PASSED_TESTS=$(grep -c "✅.*passed" "$LOG_FILE" || echo "0")
FAILED_TESTS=$(grep -c "❌.*failed" "$LOG_FILE" || echo "0")

log_and_display "Total tests run: $TOTAL_TESTS"
log_and_display "Tests passed: $PASSED_TESTS"
log_and_display "Tests failed: $FAILED_TESTS"

if [ "$FAILED_TESTS" -eq 0 ]; then
    log_and_display "🎉 All tests passed!"
    EXIT_CODE=0
elif [ "$PASSED_TESTS" -gt "$FAILED_TESTS" ]; then
    log_and_display "⚠️  Some tests failed, but majority passed"
    EXIT_CODE=1
else
    log_and_display "❌ Significant test failures detected"
    EXIT_CODE=2
fi

# Create platform benchmark if requested
if [ "$1" == "--create-benchmark" ]; then
    log_and_display ""
    log_and_display "📊 Creating platform benchmark..."
    ./create_platform_benchmark.sh 2>&1 | tee -a "$LOG_FILE"
fi

# Final summary
log_and_display ""
log_and_display "🏁 Testing Complete"
log_and_display "==================="
log_and_display "Results directory: $RESULTS_DIR"
log_and_display "Full log: $LOG_FILE"
log_and_display "Analysis report: ${RESULTS_DIR}/analysis_report.txt"
log_and_display "Completed at: $(date)"

exit $EXIT_CODE
