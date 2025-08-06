#!/bin/bash

# CURSED Final Comprehensive Test Suite
# Uses working binaries and tests all critical functionality

set -e

REPORT_FILE="FINAL_TEST_REPORT.md"
CURSED_BIN="./cursed-unified"

echo "# CURSED Final Test Report - $(date)" > $REPORT_FILE
echo "" >> $REPORT_FILE

# Function to log both to file and stdout
log() {
    echo "$1" | tee -a $REPORT_FILE
}

log "## 1. Binary Verification"
log ""

if [ -f "$CURSED_BIN" ]; then
    log "✅ CURSED binary found: $CURSED_BIN"
    file $CURSED_BIN >> $REPORT_FILE
else
    log "❌ CURSED binary not found"
    exit 1
fi

log ""

log "## 2. Basic Language Features"
log ""

# Test 1: Simple output
cat > test_basic.csd << 'EOF'
vibez.spill("Basic test passed")
EOF

if $CURSED_BIN test_basic.csd > basic_out.log 2>&1; then
    log "✅ Basic output: $(cat basic_out.log)"
else
    log "❌ Basic output failed"
fi

# Test 2: Variables and arithmetic
cat > test_variables.csd << 'EOF'
sus x drip = 10
sus y drip = 20
sus result drip = x + y
vibez.spill("Variables test:")
vibez.spill_drip(result)
EOF

if $CURSED_BIN test_variables.csd > vars_out.log 2>&1; then
    log "✅ Variables and arithmetic: $(cat vars_out.log | tr '\n' ' ')"
else
    log "❌ Variables and arithmetic failed"
fi

# Test 3: Functions
cat > test_functions.csd << 'EOF'
slay add(a drip, b drip) drip {
    damn a + b
}

sus result drip = add(5, 7)
vibez.spill("Function test:")
vibez.spill_drip(result)
EOF

if $CURSED_BIN test_functions.csd > func_out.log 2>&1; then
    log "✅ Functions: $(cat func_out.log | tr '\n' ' ')"
else
    log "❌ Functions failed"
fi

# Test 4: Control flow
cat > test_control.csd << 'EOF'
sus count drip = 0
bestie (count < 3) {
    vibez.spill("Loop iteration")
    count = count + 1
}
vibez.spill("Control flow test done")
EOF

if $CURSED_BIN test_control.csd > control_out.log 2>&1; then
    log "✅ Control flow: $(cat control_out.log | wc -l) lines of output"
else
    log "❌ Control flow failed"
fi

log ""

log "## 3. Standard Library Tests"
log ""

# Test testz framework
cat > test_testz.csd << 'EOF'
yeet "testz"

test_start("testz validation")
assert_true(based)
assert_false(cringe)
assert_eq_int(42, 42)
assert_eq_string("hello", "hello")
print_test_summary()
EOF

if $CURSED_BIN test_testz.csd > testz_out.log 2>&1; then
    log "✅ Testing framework (testz) working"
    log "   Output: $(cat testz_out.log | tr '\n' ' ')"
else
    log "❌ Testing framework failed"
    log "   Error: $(cat testz_out.log)"
fi

# Test vibez module
cat > test_vibez.csd << 'EOF'
yeet "vibez"

vibez.spill("Testing vibez module")
vibez.spill_drip(123)
vibez.spill_lit(based)
vibez.spill_lit(cringe)
EOF

if $CURSED_BIN test_vibez.csd > vibez_out.log 2>&1; then
    log "✅ I/O module (vibez) working"
else
    log "❌ I/O module failed"
fi

log ""

log "## 4. Error Handling"
log ""

# Test syntax error detection
cat > test_error.csd << 'EOF'
this is not valid CURSED syntax!!!
EOF

if $CURSED_BIN test_error.csd > error_out.log 2>&1; then
    log "❌ Error handling failed - should have caught syntax error"
else
    log "✅ Error handling working - caught syntax error"
fi

log ""

log "## 5. Advanced Features"
log ""

# Test structs
cat > test_structs.csd << 'EOF'
squad Point {
    spill x drip
    spill y drip
}

sus p Point = Point{x: 10, y: 20}
vibez.spill("Struct test:")
vibez.spill_drip(p.x)
vibez.spill_drip(p.y)
EOF

if $CURSED_BIN test_structs.csd > struct_out.log 2>&1; then
    log "✅ Structs working"
else
    log "❌ Structs failed - may not be fully implemented"
fi

# Test interfaces
cat > test_interfaces.csd << 'EOF'
collab Drawable {
    slay draw()
}

squad Circle {
    spill radius drip
}

impl Drawable for Circle {
    slay draw() {
        vibez.spill("Drawing circle")
    }
}

sus c Circle = Circle{radius: 5}
c.draw()
EOF

if $CURSED_BIN test_interfaces.csd > interface_out.log 2>&1; then
    log "✅ Interfaces working"
else
    log "❌ Interfaces failed - may not be fully implemented"
fi

log ""

log "## 6. Performance Check"
log ""

# Simple performance test
cat > test_performance.csd << 'EOF'
slay fibonacci(n drip) drip {
    bestie (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

sus start drip = 0  # Placeholder for timing
sus result drip = fibonacci(10)
vibez.spill("Fibonacci(10) = ")
vibez.spill_drip(result)
EOF

start_time=$(date +%s%N)
if $CURSED_BIN test_performance.csd > perf_out.log 2>&1; then
    end_time=$(date +%s%N)
    duration=$(((end_time - start_time) / 1000000))
    log "✅ Performance test completed in ${duration}ms"
    log "   Result: $(cat perf_out.log | tr '\n' ' ')"
else
    log "❌ Performance test failed"
fi

log ""

log "## 7. Cross-Platform Binaries"
log ""

# Check for cross-compiled binaries
if [ -d "zig-out/bin/windows-x64" ]; then
    log "✅ Windows x64 binary available"
else
    log "❌ Windows x64 binary missing"
fi

if [ -d "zig-out/bin/wasm32" ]; then
    log "✅ WebAssembly binary available"
else
    log "❌ WebAssembly binary missing"
fi

log ""

log "## Summary and Recommendations"
log ""

# Count test results
passed=$(grep -c "✅" $REPORT_FILE)
failed=$(grep -c "❌" $REPORT_FILE)

log "### Test Results:"
log "- ✅ Passed: $passed"
log "- ❌ Failed: $failed"
log ""

if [ $failed -eq 0 ]; then
    log "### 🎉 Status: READY FOR RELEASE"
    log ""
    log "All critical tests passed. The CURSED language implementation is ready for:"
    log "- Git tag creation"
    log "- Public release"
    log "- Production use"
elif [ $failed -le 3 ]; then
    log "### ⚠️ Status: MOSTLY READY"
    log ""
    log "Core functionality working. Minor issues to address:"
    log "- Some advanced features may need completion"
    log "- Cross-compilation needs verification"
    log "- Consider beta release tag"
else
    log "### 🔧 Status: NEEDS WORK"
    log ""
    log "Significant issues found. Recommended actions:"
    log "- Fix core functionality issues"
    log "- Complete critical feature implementations"
    log "- Re-run tests before release"
fi

log ""
log "### Cleanup"
log ""
log "Test files created: test_*.csd, *_out.log"
log "Report saved to: $REPORT_FILE"

# Clean up test files
rm -f test_*.csd *_out.log

echo "Final test report completed. See $REPORT_FILE for details."
