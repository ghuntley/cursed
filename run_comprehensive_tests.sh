#!/bin/bash

# CURSED Comprehensive Test Suite
# This script runs all available tests and generates a comprehensive report

set -e

REPORT_FILE="COMPREHENSIVE_TEST_REPORT.md"
LOG_FILE="test_execution.log"

echo "# CURSED Comprehensive Test Report" > $REPORT_FILE
echo "" >> $REPORT_FILE
echo "Generated: $(date)" >> $REPORT_FILE
echo "" >> $REPORT_FILE

# Function to log both to file and stdout
log() {
    echo "$1" | tee -a $LOG_FILE
    echo "$1" >> $REPORT_FILE
}

log "## 1. Core Build System Test"
log ""

# Test basic build
if zig build-exe src-zig/main_unified.zig -lc --name cursed-unified-test > build_test.log 2>&1; then
    log "✅ Core build system working"
else
    log "❌ Core build system failed"
    log "Error details:"
    cat build_test.log >> $REPORT_FILE
fi

log ""

log "## 2. CLI Interface Tests"
log ""

# Test main executables
for exe in zig-out/bin/cursed zig-out/bin/cursed-zig; do
    if [ -f "$exe" ]; then
        log "Testing $exe:"
        
        # Test help command
        if $exe --help > cli_test.log 2>&1; then
            log "✅ Help command works"
        else
            log "❌ Help command failed"
        fi
        
        # Test version command
        if $exe --version > cli_test.log 2>&1; then
            log "✅ Version command works"
        else
            log "❌ Version command failed"
        fi
        
        log ""
    else
        log "❌ $exe not found"
    fi
done

log "## 3. Basic Functionality Tests"
log ""

# Create a simple test program
cat > basic_test_comprehensive.csd << 'EOF'
vibez.spill("Hello CURSED!")
sus x drip = 42
vibez.spill("The answer is:")
vibez.spill_drip(x)
EOF

# Test interpretation
if ./zig-out/bin/cursed basic_test_comprehensive.csd > basic_output.log 2>&1; then
    log "✅ Basic interpretation works"
    log "Output:"
    cat basic_output.log >> $REPORT_FILE
else
    log "❌ Basic interpretation failed"
    log "Error:"
    cat basic_output.log >> $REPORT_FILE
fi

log ""

log "## 4. Standard Library Tests"
log ""

# Test testz framework
cat > testz_test_comprehensive.csd << 'EOF'
yeet "testz"

test_start("basic test")
assert_true(based)
assert_false(cringe)
assert_eq_int(42, 42)
print_test_summary()
EOF

if ./zig-out/bin/cursed testz_test_comprehensive.csd > testz_output.log 2>&1; then
    log "✅ Testing framework works"
    log "Output:"
    cat testz_output.log >> $REPORT_FILE
else
    log "❌ Testing framework failed"
    log "Error:"
    cat testz_output.log >> $REPORT_FILE
fi

log ""

log "## 5. Cross-Compilation Tests"
log ""

# Test cross-compilation (if build.zig supports it)
platforms=("x86_64-linux" "aarch64-linux" "x86_64-macos" "aarch64-macos" "x86_64-windows")

for platform in "${platforms[@]}"; do
    if zig build -Dtarget=$platform > cross_$platform.log 2>&1; then
        log "✅ Cross-compilation for $platform successful"
    else
        log "❌ Cross-compilation for $platform failed"
    fi
done

log ""

log "## 6. Performance Tests"
log ""

# Create performance test
cat > performance_test_comprehensive.csd << 'EOF'
slay fibonacci(n drip) drip {
    bestie (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

sus result drip = fibonacci(20)
vibez.spill("Fibonacci(20) =")
vibez.spill_drip(result)
EOF

# Time the execution
if time ./zig-out/bin/cursed performance_test_comprehensive.csd > perf_output.log 2>&1; then
    log "✅ Performance test completed"
    log "Output:"
    cat perf_output.log >> $REPORT_FILE
else
    log "❌ Performance test failed"
    log "Error:"
    cat perf_output.log >> $REPORT_FILE
fi

log ""

log "## 7. Error Handling Tests"
log ""

# Test syntax error handling
cat > error_test_comprehensive.csd << 'EOF'
this is invalid syntax
EOF

if ./zig-out/bin/cursed error_test_comprehensive.csd > error_output.log 2>&1; then
    log "❌ Error handling failed - should have caught syntax error"
else
    log "✅ Error handling works - caught syntax error"
    log "Error message:"
    cat error_output.log >> $REPORT_FILE
fi

log ""

log "## 8. Memory and Resource Tests"
log ""

# Test memory usage with a larger program
cat > memory_test_comprehensive.csd << 'EOF'
slay test_arrays() {
    sus arr normie = [1, 2, 3, 4, 5]
    sus count drip = 0
    bestie (count < 5) {
        vibez.spill_drip(arr[count])
        count = count + 1
    }
}

test_arrays()
vibez.spill("Memory test completed")
EOF

if valgrind --tool=memcheck --leak-check=yes ./zig-out/bin/cursed memory_test_comprehensive.csd > memory_output.log 2>&1; then
    log "✅ Memory test with valgrind completed"
    grep -E "(definitely lost|possibly lost|ERROR SUMMARY)" memory_output.log >> $REPORT_FILE
else
    log "⚠️ Valgrind not available or test failed"
fi

log ""

log "## Test Summary"
log ""

# Count successes and failures
successes=$(grep -c "✅" $REPORT_FILE || echo 0)
failures=$(grep -c "❌" $REPORT_FILE || echo 0)
warnings=$(grep -c "⚠️" $REPORT_FILE || echo 0)

log "Results:"
log "- ✅ Successful tests: $successes"
log "- ❌ Failed tests: $failures"  
log "- ⚠️ Warnings: $warnings"
log ""

if [ $failures -eq 0 ]; then
    log "🎉 All critical tests passed! Ready for release."
else
    log "🔧 Some tests failed. Review needed before release."
fi

log ""
log "## Recommendations"
log ""

if [ $failures -gt 0 ]; then
    log "- Fix failed test cases before tagging release"
    log "- Review error logs for detailed failure information"
fi

log "- Consider running extended test suite on target platforms"
log "- Validate performance benchmarks on production hardware"
log "- Complete any remaining stdlib module implementations"

echo "Test report saved to: $REPORT_FILE"
echo "Test logs saved to: $LOG_FILE"
