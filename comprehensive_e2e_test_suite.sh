#!/bin/bash

# Comprehensive End-to-End Test Suite for CURSED Compiler
# Combines integration, performance, memory, and cross-platform testing

set -e

echo "🚀 CURSED Comprehensive End-to-End Test Suite"
echo "=============================================="
echo "This test suite validates the entire CURSED compiler pipeline"
echo ""

# Configuration
RESULTS_DIR="e2e_test_results"
MAIN_LOG="$RESULTS_DIR/comprehensive_results.log"
SUMMARY_LOG="$RESULTS_DIR/test_summary.log"

# Create results directory
mkdir -p "$RESULTS_DIR"
rm -f "$MAIN_LOG" "$SUMMARY_LOG"

# Initialize counters
TOTAL_TEST_SUITES=0
PASSED_SUITES=0
FAILED_SUITES=0

log_main() {
    echo "$1" | tee -a "$MAIN_LOG"
}

log_summary() {
    echo "$1" | tee -a "$SUMMARY_LOG"
}

run_test_suite() {
    local suite_name="$1"
    local script_name="$2"
    local description="$3"
    
    TOTAL_TEST_SUITES=$((TOTAL_TEST_SUITES + 1))
    
    echo ""
    echo "📋 Running $suite_name"
    echo "Description: $description"
    echo "Script: $script_name"
    echo "=========================================="
    
    log_main "=== $suite_name ==="
    log_main "Description: $description"
    log_main "Started: $(date)"
    
    if [ ! -f "$script_name" ]; then
        echo "❌ Test script not found: $script_name"
        log_main "ERROR: Test script not found"
        log_summary "FAIL: $suite_name - Script not found"
        FAILED_SUITES=$((FAILED_SUITES + 1))
        return 1
    fi
    
    # Make script executable
    chmod +x "$script_name"
    
    # Run the test suite with timeout
    if timeout 1800 bash "$script_name" > "$RESULTS_DIR/${suite_name,,}_output.log" 2>&1; then
        echo "✅ $suite_name completed successfully"
        log_main "SUCCESS: $suite_name completed"
        log_summary "PASS: $suite_name"
        PASSED_SUITES=$((PASSED_SUITES + 1))
        
        # Extract key results
        if [ -f "${script_name%.sh}_results.log" ]; then
            echo "📊 Key results from $suite_name:"
            tail -10 "${script_name%.sh}_results.log" | sed 's/^/  /'
            cat "${script_name%.sh}_results.log" >> "$MAIN_LOG"
        fi
        
        return 0
    else
        echo "❌ $suite_name failed or timed out"
        log_main "FAIL: $suite_name failed or timed out"
        log_summary "FAIL: $suite_name - Execution failed"
        FAILED_SUITES=$((FAILED_SUITES + 1))
        
        # Show error details
        echo "Error details:"
        tail -20 "$RESULTS_DIR/${suite_name,,}_output.log" | sed 's/^/  /'
        
        return 1
    fi
}

check_prerequisites() {
    echo "🔍 Checking prerequisites..."
    
    local missing_deps=()
    
    # Check for essential tools
    if ! command -v zig >/dev/null 2>&1; then
        missing_deps+=("zig")
    fi
    
    if ! command -v cargo >/dev/null 2>&1; then
        missing_deps+=("cargo")
    fi
    
    if ! command -v gcc >/dev/null 2>&1; then
        missing_deps+=("gcc")
    fi
    
    # Check for optional tools
    local optional_tools=("valgrind" "clang" "node")
    local missing_optional=()
    
    for tool in "${optional_tools[@]}"; do
        if ! command -v "$tool" >/dev/null 2>&1; then
            missing_optional+=("$tool")
        fi
    done
    
    if [ ${#missing_deps[@]} -gt 0 ]; then
        echo "❌ Missing required dependencies: ${missing_deps[*]}"
        log_main "ERROR: Missing required dependencies: ${missing_deps[*]}"
        exit 1
    fi
    
    if [ ${#missing_optional[@]} -gt 0 ]; then
        echo "⚠️ Missing optional tools (some tests will be skipped): ${missing_optional[*]}"
        log_main "WARNING: Missing optional tools: ${missing_optional[*]}"
    fi
    
    echo "✅ Prerequisites check completed"
    log_main "Prerequisites check: PASS"
}

build_compilers() {
    echo "🏗️ Building CURSED compilers..."
    
    log_main "=== Compiler Build Phase ==="
    
    # Build Zig compiler
    echo "Building Zig compiler..."
    if timeout 300 zig build > "$RESULTS_DIR/zig_build.log" 2>&1; then
        echo "✅ Zig compiler build successful"
        log_main "SUCCESS: Zig compiler build"
    else
        echo "⚠️ Zig compiler build failed (will try with available binaries)"
        log_main "WARNING: Zig compiler build failed"
        cat "$RESULTS_DIR/zig_build.log" | tail -10 | sed 's/^/  /'
    fi
    
    # Build Rust compiler  
    echo "Building Rust compiler..."
    if timeout 600 cargo build > "$RESULTS_DIR/rust_build.log" 2>&1; then
        echo "✅ Rust compiler build successful"
        log_main "SUCCESS: Rust compiler build"
    else
        echo "⚠️ Rust compiler build failed"
        log_main "WARNING: Rust compiler build failed"
        cat "$RESULTS_DIR/rust_build.log" | tail -10 | sed 's/^/  /'
    fi
    
    # Check if we have working compilers
    if [ -f "./zig-out/bin/cursed-zig" ] || [ -f "./target/debug/cursed" ]; then
        echo "✅ At least one compiler available"
        log_main "SUCCESS: Compilers available for testing"
    else
        echo "❌ No working compilers available"
        log_main "ERROR: No working compilers"
        exit 1
    fi
}

run_quick_smoke_test() {
    echo "💨 Running quick smoke test..."
    
    log_main "=== Quick Smoke Test ==="
    
    # Create simple test
    echo 'vibez.spill("Smoke test successful!")' > smoke_test.csd
    
    # Test Zig compiler if available
    if [ -f "./zig-out/bin/cursed-zig" ]; then
        if timeout 30 ./zig-out/bin/cursed-zig smoke_test.csd > "$RESULTS_DIR/smoke_zig.log" 2>&1; then
            if grep -q "Smoke test successful" "$RESULTS_DIR/smoke_zig.log"; then
                echo "✅ Zig compiler smoke test passed"
                log_main "SUCCESS: Zig compiler smoke test"
            else
                echo "❌ Zig compiler smoke test output mismatch"
                log_main "FAIL: Zig compiler smoke test"
                exit 1
            fi
        else
            echo "❌ Zig compiler smoke test failed"
            log_main "FAIL: Zig compiler smoke test execution"
            exit 1
        fi
    fi
    
    # Test Rust compiler if available
    if [ -f "./target/debug/cursed" ]; then
        if timeout 30 ./target/debug/cursed smoke_test.csd > "$RESULTS_DIR/smoke_rust.log" 2>&1; then
            if grep -q "Smoke test successful" "$RESULTS_DIR/smoke_rust.log"; then
                echo "✅ Rust compiler smoke test passed"
                log_main "SUCCESS: Rust compiler smoke test"
            else
                echo "❌ Rust compiler smoke test output mismatch"
                log_main "FAIL: Rust compiler smoke test"
                exit 1
            fi
        else
            echo "❌ Rust compiler smoke test failed"
            log_main "FAIL: Rust compiler smoke test execution"
            exit 1
        fi
    fi
    
    rm -f smoke_test.csd
    echo "✅ Smoke test completed successfully"
}

generate_final_report() {
    echo ""
    echo "📊 Generating Final Test Report"
    echo "==============================="
    
    local report_file="$RESULTS_DIR/final_test_report.md"
    
    cat > "$report_file" << EOF
# CURSED Compiler End-to-End Test Report

**Date:** $(date)
**System:** $(uname -a)
**Test Duration:** $(( $(date +%s) - START_TIME )) seconds

## Executive Summary

- **Total Test Suites:** $TOTAL_TEST_SUITES
- **Passed Suites:** $PASSED_SUITES  
- **Failed Suites:** $FAILED_SUITES
- **Success Rate:** $(( (PASSED_SUITES * 100) / TOTAL_TEST_SUITES ))%

## Test Results Summary

EOF
    
    # Add test results
    cat "$SUMMARY_LOG" >> "$report_file"
    
    cat >> "$report_file" << EOF

## Available Artifacts

- \`comprehensive_results.log\` - Detailed test execution log
- \`integration_test_results.log\` - Integration test details  
- \`performance_results.log\` - Performance benchmark results
- \`cross_platform_results.log\` - Cross-platform test results
- Individual test suite output logs in \`e2e_test_results/\`

## Recommendations

EOF
    
    if [ $FAILED_SUITES -eq 0 ]; then
        cat >> "$report_file" << EOF
✅ **All test suites passed successfully!**

The CURSED compiler is ready for production use with:
- Functional interpretation and compilation modes
- Cross-platform support verified
- Performance characteristics documented
- Memory safety validated

EOF
    else
        cat >> "$report_file" << EOF
❌ **Some test suites failed.**

Priority actions needed:
1. Review failed test suite logs for root causes
2. Fix identified issues before production deployment
3. Re-run comprehensive testing after fixes

Critical Issues:
EOF
        grep "FAIL:" "$SUMMARY_LOG" | sed 's/^/- /' >> "$report_file"
    fi
    
    echo "📋 Final report generated: $report_file"
    
    # Display summary
    echo ""
    echo "🎯 Final Results Summary:"
    echo "   Total Test Suites: $TOTAL_TEST_SUITES"
    echo "   Passed: $PASSED_SUITES"
    echo "   Failed: $FAILED_SUITES"
    echo "   Success Rate: $(( (PASSED_SUITES * 100) / TOTAL_TEST_SUITES ))%"
    echo ""
    
    if [ $FAILED_SUITES -eq 0 ]; then
        echo "🎉 All comprehensive tests passed!"
        echo "✅ CURSED compiler is fully validated for production use"
        return 0
    else
        echo "❌ Some test suites failed - review required"
        echo "📋 Check detailed logs in $RESULTS_DIR/"
        return 1
    fi
}

# Record start time
START_TIME=$(date +%s)

log_main "CURSED Comprehensive End-to-End Test Suite"
log_main "Date: $(date)"
log_main "System: $(uname -a)"
log_main "=========================================="

# Run test phases
check_prerequisites
build_compilers
run_quick_smoke_test

echo ""
echo "🧪 Starting Comprehensive Test Execution"
echo "========================================"

# Run all test suites
run_test_suite "Integration_Tests" "integration_test_suite.sh" "Core language features and end-to-end functionality"

run_test_suite "Performance_Tests" "performance_test_suite.sh" "Performance benchmarks and memory leak detection"

run_test_suite "Cross_Platform_Tests" "cross_platform_test_suite.sh" "Cross-platform compilation and execution"

# Run existing test suites if they exist
if [ -f "run_fast_tests_final.sh" ]; then
    run_test_suite "Fast_Unit_Tests" "run_fast_tests_final.sh" "Fast unit test execution"
fi

if [ -f "bootstrap_validation.sh" ]; then
    run_test_suite "Bootstrap_Tests" "bootstrap_validation.sh" "Self-hosting and bootstrap validation"
fi

# Generate final report and exit with appropriate code
if generate_final_report; then
    exit 0
else
    exit 1
fi
