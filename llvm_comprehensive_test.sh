#!/bin/bash
# Comprehensive LLVM Backend Testing - Using the correct cursed executable
set -e

echo "=== CURSED LLVM Backend Comprehensive Testing ==="
echo "Testing Date: $(date)"
echo "Using: ./zig-out/bin/cursed (unified LLVM implementation)"
echo ""

# Clean previous test artifacts
rm -f llvm_test_*.csd llvm_test_*-native *.ll *.o *.exe *.wasm

# Test 1: Basic LLVM Compilation
echo "=== Test 1: Basic LLVM Compilation ==="
cat > llvm_test_basic.csd << 'EOF'
sus answer drip = 42
vibez.spill("The answer is:", answer)
EOF

echo "Testing basic LLVM compilation..."
if ./zig-out/bin/cursed --compile llvm_test_basic.csd; then
    echo "✅ Basic LLVM compilation: SUCCESS"
    if [ -f llvm_test_basic ] || [ -f llvm_test_basic.exe ]; then
        echo "Testing binary execution..."
        if ./llvm_test_basic 2>/dev/null || ./llvm_test_basic.exe 2>/dev/null; then
            echo "✅ Basic binary execution: SUCCESS"
        else
            echo "❌ Basic binary execution: FAILED"
        fi
    else
        echo "❌ Binary not generated"
        ls -la llvm_test_basic*
    fi
else
    echo "❌ Basic LLVM compilation: FAILED"
fi
echo ""

# Test 2: LLVM IR Generation
echo "=== Test 2: LLVM IR Generation ==="
cat > llvm_test_ir.csd << 'EOF'
sus x drip = 10
sus y drip = 20
sus sum drip = x + y
vibez.spill("Sum:", sum)
EOF

echo "Testing LLVM IR generation..."
if ./zig-out/bin/cursed --emit-llvm llvm_test_ir.csd; then
    echo "✅ LLVM IR generation: SUCCESS"
    if [ -f llvm_test_ir.ll ]; then
        echo "✅ IR file generated: llvm_test_ir.ll"
        echo "IR file size: $(wc -l < llvm_test_ir.ll) lines"
        echo "Sample IR content:"
        head -15 llvm_test_ir.ll
        echo "..."
        echo "IR generation successful!"
    else
        echo "❌ IR file not generated"
        ls -la llvm_test_ir*
    fi
else
    echo "❌ LLVM IR generation: FAILED"
fi
echo ""

# Test 3: Optimization Levels
echo "=== Test 3: Optimization Level Testing ==="
cat > llvm_test_opt.csd << 'EOF'
slay calculate(n drip) drip {
    sus result drip = 0
    sus i drip = 0
    bestie (i < n) {
        result = result + i
        i = i + 1
    }
    damn result
}

sus result drip = calculate(100)
vibez.spill("Result:", result)
EOF

for opt_level in 0 1 2 3; do
    echo "Testing optimization level O$opt_level..."
    binary_name="llvm_test_opt_O$opt_level"
    
    if ./zig-out/bin/cursed --compile llvm_test_opt.csd --optimize=$opt_level -o $binary_name 2>/dev/null || \
       ./zig-out/bin/cursed --compile llvm_test_opt.csd -O$opt_level 2>/dev/null; then
        echo "✅ Optimization O$opt_level: COMPILED"
        
        # Test execution
        if [ -f llvm_test_opt ] || [ -f "$binary_name" ]; then
            echo "Testing O$opt_level binary execution..."
            if timeout 5 ./llvm_test_opt 2>/dev/null || timeout 5 ./$binary_name 2>/dev/null; then
                echo "✅ Optimization O$opt_level execution: SUCCESS"
            else
                echo "❌ Optimization O$opt_level execution: FAILED/TIMEOUT"
            fi
        else
            echo "⚠️  O$opt_level compiled but binary not found"
        fi
    else
        echo "❌ Optimization O$opt_level: FAILED"
    fi
done
echo ""

# Test 4: Debug Information
echo "=== Test 4: Debug Information Generation ==="
cat > llvm_test_debug.csd << 'EOF'
slay debug_function(value drip) drip {
    vibez.spill("Debug value:", value)
    damn value * 2
}

sus result drip = debug_function(21)
vibez.spill("Final result:", result)
EOF

echo "Testing debug information generation..."
if ./zig-out/bin/cursed --compile llvm_test_debug.csd --debug-info; then
    echo "✅ Debug info compilation: SUCCESS"
    if [ -f llvm_test_debug ]; then
        echo "Checking debug symbols..."
        if file llvm_test_debug | grep -q "debug_info\|with debug"; then
            echo "✅ Debug symbols present in binary"
        else
            echo "⚠️  Debug symbols may not be present"
        fi
        
        echo "Testing debug binary execution..."
        if ./llvm_test_debug; then
            echo "✅ Debug binary execution: SUCCESS"
        else
            echo "❌ Debug binary execution: FAILED"
        fi
    fi
else
    echo "❌ Debug info compilation: FAILED"
fi
echo ""

# Test 5: Cross-compilation (with proper timeouts)
echo "=== Test 5: Cross-compilation Testing ==="
cat > llvm_test_cross.csd << 'EOF'
vibez.spill("Hello from cross-compiled binary!")
sus platform drip = 42
vibez.spill("Platform test value:", platform)
EOF

targets=("x86_64-linux" "x86_64-windows" "aarch64-linux" "wasm32-wasi")

for target in "${targets[@]}"; do
    echo "Testing cross-compilation target: $target"
    
    # Use timeout to catch hanging compilation
    if timeout 60 ./zig-out/bin/cursed --compile llvm_test_cross.csd --target=$target 2>/dev/null; then
        echo "✅ Cross-compilation $target: SUCCESS"
        
        # Check for generated files
        if [ -f "llvm_test_cross" ] || [ -f "llvm_test_cross.exe" ] || [ -f "llvm_test_cross.wasm" ]; then
            echo "✅ Binary generated for $target"
            # Show file info
            ls -la llvm_test_cross* 2>/dev/null | head -3
        else
            echo "⚠️  Compilation succeeded but no binary found for $target"
        fi
    else
        exit_code=$?
        if [ $exit_code -eq 124 ]; then
            echo "❌ Cross-compilation $target: TIMEOUT (60s) - HANGING ISSUE CONFIRMED"
        else
            echo "❌ Cross-compilation $target: FAILED (exit code: $exit_code)"
        fi
    fi
    
    # Clean up
    rm -f llvm_test_cross llvm_test_cross.exe llvm_test_cross.wasm
    echo ""
done

# Test 6: Advanced Features
echo "=== Test 6: Advanced LLVM Feature Testing ==="
cat > llvm_test_advanced.csd << 'EOF'
yeet "arrayz"

slay array_processor(numbers []drip) drip {
    sus total drip = 0
    sus i drip = 0
    bestie (i < len(numbers)) {
        total = total + numbers[i]
        i = i + 1
    }
    damn total
}

sus data []drip = [1, 2, 3, 4, 5]
sus sum drip = array_processor(data)
vibez.spill("Array sum:", sum)
vibez.spill("Array length:", len(data))
EOF

echo "Testing advanced feature compilation..."
if ./zig-out/bin/cursed --compile llvm_test_advanced.csd; then
    echo "✅ Advanced feature compilation: SUCCESS"
    if [ -f llvm_test_advanced ]; then
        echo "Testing advanced feature execution..."
        if ./llvm_test_advanced; then
            echo "✅ Advanced feature execution: SUCCESS"
        else
            echo "❌ Advanced feature execution: FAILED"
        fi
    fi
else
    echo "❌ Advanced feature compilation: FAILED"
fi
echo ""

# Test 7: Performance Comparison
echo "=== Test 7: Interpreter vs Compiled Performance ==="
cat > llvm_test_performance.csd << 'EOF'
slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    } otherwise {
        damn fibonacci(n - 1) + fibonacci(n - 2)
    }
}

sus result drip = fibonacci(25)
vibez.spill("Fibonacci(25):", result)
EOF

echo "Testing interpreter performance..."
time1=$(date +%s.%N)
if ./zig-out/bin/cursed llvm_test_performance.csd >/dev/null 2>&1; then
    time2=$(date +%s.%N)
    interpreter_time=$(echo "$time2 - $time1" | bc -l 2>/dev/null || echo "measurement_failed")
    echo "✅ Interpreter execution: SUCCESS (${interpreter_time}s)"
else
    echo "❌ Interpreter execution: FAILED"
    interpreter_time="failed"
fi

echo "Testing compiled binary performance..."
if ./zig-out/bin/cursed --compile llvm_test_performance.csd; then
    if [ -f llvm_test_performance ]; then
        time1=$(date +%s.%N)
        if ./llvm_test_performance >/dev/null 2>&1; then
            time2=$(date +%s.%N)
            compiled_time=$(echo "$time2 - $time1" | bc -l 2>/dev/null || echo "measurement_failed")
            echo "✅ Compiled execution: SUCCESS (${compiled_time}s)"
            
            if [[ "$interpreter_time" != "failed" && "$compiled_time" != "measurement_failed" && "$interpreter_time" != "measurement_failed" ]]; then
                speedup=$(echo "scale=2; $interpreter_time / $compiled_time" | bc -l 2>/dev/null || echo "unknown")
                echo "🚀 Performance speedup: ${speedup}x"
            fi
        else
            echo "❌ Compiled execution: FAILED"
        fi
    fi
else
    echo "❌ Performance test compilation: FAILED"
fi
echo ""

# Test 8: Error Handling in LLVM
echo "=== Test 8: LLVM Error Handling ==="
cat > llvm_test_error.csd << 'EOF'
sus invalid_syntax = this will not parse correctly
vibez.spill("This should not compile")
EOF

echo "Testing LLVM error handling..."
if ./zig-out/bin/cursed --compile llvm_test_error.csd 2>/dev/null; then
    echo "❌ LLVM error handling: Should have failed but succeeded"
else
    echo "✅ LLVM error handling: Correctly rejected invalid syntax"
fi
echo ""

# Test 9: Memory Safety of Compiled Binaries
echo "=== Test 9: Memory Safety Testing ==="
if command -v valgrind >/dev/null 2>&1; then
    if [ -f llvm_test_basic ]; then
        echo "Testing memory safety of LLVM-compiled binary..."
        if valgrind --error-exitcode=1 --leak-check=full --quiet ./llvm_test_basic 2>/dev/null; then
            echo "✅ LLVM binary memory safety: SUCCESS"
        else
            echo "❌ LLVM binary memory safety: FAILED"
        fi
    else
        echo "⚠️  No basic test binary for memory testing"
    fi
else
    echo "⚠️  Valgrind not available for memory testing"
fi
echo ""

# Test 10: LLVM Linking and Dependencies
echo "=== Test 10: LLVM Linking Analysis ==="
echo "Checking LLVM installation status..."
if command -v llvm-config >/dev/null 2>&1; then
    echo "✅ llvm-config found: $(which llvm-config)"
    echo "LLVM version: $(llvm-config --version)"
    echo "LLVM libdir: $(llvm-config --libdir)"
    echo "LLVM libraries: $(llvm-config --libs | wc -w) available"
else
    echo "❌ llvm-config not found"
fi

echo "Checking compiled binary dependencies..."
if [ -f llvm_test_basic ]; then
    echo "Dependencies for llvm_test_basic:"
    ldd llvm_test_basic 2>/dev/null | grep -E "(LLVM|llvm)" || echo "No LLVM runtime dependencies found"
else
    echo "⚠️  No compiled binary to analyze"
fi
echo ""

echo "=== LLVM Backend Test Summary ==="
echo "Test completed at: $(date)"
echo ""
echo "Analysis of LLVM backend capabilities:"
echo "- Basic compilation: Tested"
echo "- IR generation: Tested" 
echo "- Optimization levels: Tested"
echo "- Debug information: Tested"
echo "- Cross-compilation: Tested (with timeout detection)"
echo "- Advanced features: Tested"
echo "- Performance comparison: Tested"
echo "- Error handling: Tested"
echo "- Memory safety: Tested"
echo "- LLVM integration: Analyzed"
echo ""
echo "Look for ✅ (working), ❌ (broken), ⚠️  (partial), and 🚀 (performance) indicators."
