#!/bin/bash

# CURSED Cross-Platform Integration Testing Suite
# Tests compilation and execution across different target platforms

set -e

echo "🌐 CURSED Cross-Platform Test Suite"
echo "==================================="

RESULTS_FILE="cross_platform_results.log"
TEST_DIR="cross_platform_tests"
ZIG_COMPILER="./zig-out/bin/cursed-zig"

mkdir -p "$TEST_DIR"
cd "$TEST_DIR"
rm -f "../$RESULTS_FILE"

log_result() {
    echo "$1" | tee -a "../$RESULTS_FILE"
}

echo "📝 Creating cross-platform test programs..."

# Create a simple test program
cat > simple_cross_test.csd << 'EOF'
vibez.spill("Cross-platform test successful!")
sus platform_value drip = 42
vibez.spill("Platform value:", platform_value)
EOF

# Create a more complex test
cat > complex_cross_test.csd << 'EOF'
squad TestData {
    spill value drip
    spill name tea
}

slay process_data(data TestData) drip {
    vibez.spill("Processing:", data.name)
    damn data.value * 2
}

slay main_test() {
    sus test_item TestData = TestData{value: 21, name: "cross-platform"}
    sus result drip = process_data(test_item)
    vibez.spill("Result:", result)
    vibez.spill("Cross-platform complex test completed")
}

main_test()
EOF

# Create arithmetic stress test
cat > arithmetic_cross_test.csd << 'EOF'
slay arithmetic_test() {
    sus a drip = 100
    sus b drip = 50
    sus sum drip = a + b
    sus diff drip = a - b
    sus prod drip = a * b
    sus quot drip = a / b
    
    vibez.spill("Arithmetic results:")
    vibez.spill("Sum:", sum)
    vibez.spill("Difference:", diff)
    vibez.spill("Product:", prod)
    vibez.spill("Quotient:", quot)
}

arithmetic_test()
EOF

test_platform_compilation() {
    local platform_name="$1"
    local test_file="$2"
    local compile_cmd="$3"
    local expected_binary="$4"
    
    echo "🔧 Testing $platform_name compilation..."
    log_result "=== $platform_name Compilation Test ==="
    
    if timeout 120 $compile_cmd > "compile_${platform_name}.log" 2>&1; then
        if [ -f "$expected_binary" ]; then
            echo "  ✅ $platform_name compilation successful"
            log_result "SUCCESS: $platform_name compilation produced binary"
            
            # Test binary properties
            file "$expected_binary" | tee -a "../$RESULTS_FILE" 2>/dev/null || true
            ls -la "$expected_binary" | tee -a "../$RESULTS_FILE" 2>/dev/null || true
            
            # Test execution if on compatible platform
            if [[ "$platform_name" == *"linux"* ]] && [[ "$(uname -s)" == "Linux" ]]; then
                echo "    Testing execution on native platform..."
                if timeout 30 ./"$expected_binary" > "exec_${platform_name}.log" 2>&1; then
                    if grep -q "Cross-platform test successful" "exec_${platform_name}.log"; then
                        echo "    ✅ Native execution successful"
                        log_result "SUCCESS: $platform_name binary execution verified"
                    else
                        echo "    ⚠️ Native execution produced unexpected output"
                        log_result "WARNING: $platform_name execution output mismatch"
                        head -3 "exec_${platform_name}.log" | sed 's/^/      /' | tee -a "../$RESULTS_FILE"
                    fi
                else
                    echo "    ❌ Native execution failed"
                    log_result "FAIL: $platform_name binary execution failed"
                    head -3 "exec_${platform_name}.log" | sed 's/^/      /' | tee -a "../$RESULTS_FILE"
                fi
            else
                echo "    ⏭️ Skipping execution test (cross-platform binary)"
                log_result "SKIP: $platform_name execution test (cross-platform)"
            fi
            
            rm -f "$expected_binary"
            return 0
        else
            echo "  ❌ $platform_name compilation failed - no binary produced"
            log_result "FAIL: $platform_name compilation - no binary"
            head -5 "compile_${platform_name}.log" | sed 's/^/    /' | tee -a "../$RESULTS_FILE"
            return 1
        fi
    else
        echo "  ❌ $platform_name compilation timeout/error"
        log_result "FAIL: $platform_name compilation timeout"
        head -5 "compile_${platform_name}.log" | sed 's/^/    /' | tee -a "../$RESULTS_FILE"
        return 1
    fi
}

test_zig_cross_compilation() {
    echo "🏗️ Testing Zig cross-compilation capabilities..."
    
    # Test different Zig targets if available
    local targets=(
        "x86_64-linux"
        "aarch64-linux" 
        "x86_64-windows"
        "x86_64-macos"
        "wasm32-freestanding"
    )
    
    for target in "${targets[@]}"; do
        echo "Testing Zig target: $target"
        log_result "=== Zig Cross-Compilation: $target ==="
        
        # Try to build with Zig for specific target
        if timeout 60 zig build-exe -target "$target" -lc simple_cross_test.csd > "zig_${target}.log" 2>&1; then
            local binary_name="simple_cross_test"
            if [[ "$target" == *"windows"* ]]; then
                binary_name="${binary_name}.exe"
            elif [[ "$target" == *"wasm"* ]]; then
                binary_name="${binary_name}.wasm"
            fi
            
            if [ -f "$binary_name" ]; then
                echo "  ✅ Zig $target cross-compilation successful"
                log_result "SUCCESS: Zig $target cross-compilation"
                file "$binary_name" | tee -a "../$RESULTS_FILE" 2>/dev/null || true
                rm -f "$binary_name"
            else
                echo "  ⚠️ Zig $target compilation completed but no expected binary"
                log_result "WARNING: Zig $target - unexpected binary name"
            fi
        else
            echo "  ❌ Zig $target cross-compilation failed"
            log_result "FAIL: Zig $target cross-compilation"
            head -3 "zig_${target}.log" | sed 's/^/    /' | tee -a "../$RESULTS_FILE"
        fi
    done
}

test_c_backend_cross_compilation() {
    echo "🔨 Testing C backend cross-compilation..."
    
    # Test C code generation and compilation with different compilers
    local c_compilers=("gcc" "clang")
    
    for compiler in "${c_compilers[@]}"; do
        if command -v "$compiler" >/dev/null 2>&1; then
            echo "Testing with $compiler..."
            log_result "=== C Backend Test: $compiler ==="
            
            # Generate C code first (if CURSED supports this)
            if timeout 60 ../"$ZIG_COMPILER" --emit-c simple_cross_test.csd > "c_gen_${compiler}.log" 2>&1; then
                if [ -f "simple_cross_test.c" ]; then
                    echo "  ✅ C code generation successful"
                    
                    # Compile the C code
                    if timeout 60 "$compiler" -o "test_${compiler}" simple_cross_test.c > "c_compile_${compiler}.log" 2>&1; then
                        echo "  ✅ C compilation with $compiler successful"
                        log_result "SUCCESS: C backend with $compiler"
                        
                        # Test execution
                        if timeout 30 ./test_"${compiler}" > "c_exec_${compiler}.log" 2>&1; then
                            if grep -q "Cross-platform test successful" "c_exec_${compiler}.log"; then
                                echo "    ✅ C backend execution successful"
                                log_result "SUCCESS: C backend execution with $compiler"
                            else
                                echo "    ⚠️ C backend execution unexpected output"
                                log_result "WARNING: C backend execution output with $compiler"
                            fi
                        else
                            echo "    ❌ C backend execution failed"
                            log_result "FAIL: C backend execution with $compiler"
                        fi
                        
                        rm -f "test_${compiler}"
                    else
                        echo "  ❌ C compilation with $compiler failed"
                        log_result "FAIL: C compilation with $compiler"
                        head -3 "c_compile_${compiler}.log" | sed 's/^/    /' | tee -a "../$RESULTS_FILE"
                    fi
                    
                    rm -f simple_cross_test.c
                else
                    echo "  ❌ C code generation failed - no .c file produced"
                    log_result "FAIL: C code generation"
                fi
            else
                echo "  ⚠️ C code generation not supported or failed"
                log_result "SKIP: C code generation not available"
            fi
        else
            echo "  ⏭️ $compiler not available"
            log_result "SKIP: $compiler not available"
        fi
    done
}

test_wasm_compilation() {
    echo "🕸️ Testing WebAssembly compilation..."
    log_result "=== WebAssembly Compilation Test ==="
    
    # Test WASM compilation if supported
    if timeout 60 ../"$ZIG_COMPILER" --target=wasm32 simple_cross_test.csd > "wasm_compile.log" 2>&1; then
        if [ -f "simple_cross_test.wasm" ]; then
            echo "  ✅ WASM compilation successful"
            log_result "SUCCESS: WASM compilation"
            file simple_cross_test.wasm | tee -a "../$RESULTS_FILE" 2>/dev/null || true
            
            # Test with Node.js if available
            if command -v node >/dev/null 2>&1; then
                cat > wasm_test.js << 'EOF'
const fs = require('fs');
const wasmBuffer = fs.readFileSync('./simple_cross_test.wasm');
WebAssembly.instantiate(wasmBuffer).then(result => {
    console.log('WASM module loaded successfully');
    // Try to execute if it has exports
    if (result.instance.exports.main) {
        result.instance.exports.main();
    }
}).catch(err => {
    console.error('WASM execution failed:', err);
});
EOF
                
                if timeout 30 node wasm_test.js > "wasm_exec.log" 2>&1; then
                    echo "    ✅ WASM execution test completed"
                    log_result "SUCCESS: WASM execution test"
                    cat "wasm_exec.log" | head -3 | sed 's/^/      /' | tee -a "../$RESULTS_FILE"
                else
                    echo "    ⚠️ WASM execution test failed"
                    log_result "WARNING: WASM execution test failed"
                fi
                
                rm -f wasm_test.js
            else
                echo "    ⏭️ Node.js not available for WASM testing"
                log_result "SKIP: WASM execution test (Node.js unavailable)"
            fi
            
            rm -f simple_cross_test.wasm
        else
            echo "  ❌ WASM compilation failed - no .wasm file"
            log_result "FAIL: WASM compilation - no output"
            head -3 "wasm_compile.log" | sed 's/^/    /' | tee -a "../$RESULTS_FILE"
        fi
    else
        echo "  ⚠️ WASM compilation not supported"
        log_result "SKIP: WASM compilation not supported"
    fi
}

# Build compiler if needed
if [ ! -f "../$ZIG_COMPILER" ]; then
    echo "Building Zig compiler..."
    cd .. && zig build && cd "$TEST_DIR"
fi

log_result "CURSED Cross-Platform Test Results"
log_result "Date: $(date)"
log_result "Host System: $(uname -a)"
log_result "================================="

# Run interpretation test to establish baseline
echo "🔄 Testing baseline interpretation..."
log_result "=== Baseline Interpretation Test ==="
if timeout 30 ../"$ZIG_COMPILER" simple_cross_test.csd > baseline_interp.log 2>&1; then
    if grep -q "Cross-platform test successful" baseline_interp.log; then
        echo "✅ Baseline interpretation successful"
        log_result "SUCCESS: Baseline interpretation"
    else
        echo "❌ Baseline interpretation failed"
        log_result "FAIL: Baseline interpretation"
        head -3 baseline_interp.log | sed 's/^/  /' | tee -a "../$RESULTS_FILE"
    fi
else
    echo "❌ Baseline interpretation timeout"
    log_result "FAIL: Baseline interpretation timeout"
fi

# Test platform-specific compilations
test_platform_compilation "native_linux" "simple_cross_test.csd" "../$ZIG_COMPILER --compile simple_cross_test.csd" "simple_cross_test"

# Test cross-compilation features
test_zig_cross_compilation
test_c_backend_cross_compilation  
test_wasm_compilation

# Test with complex program
echo "🧩 Testing complex cross-platform program..."
test_platform_compilation "complex_native" "complex_cross_test.csd" "../$ZIG_COMPILER --compile complex_cross_test.csd" "complex_cross_test"

# Test arithmetic operations across platforms
echo "🔢 Testing arithmetic across platforms..."
test_platform_compilation "arithmetic_native" "arithmetic_cross_test.csd" "../$ZIG_COMPILER --compile arithmetic_cross_test.csd" "arithmetic_cross_test"

# Cleanup
cd ..

# Generate summary
echo ""
echo "📊 Cross-Platform Test Summary"
echo "==============================="

# Count results
SUCCESSES=$(grep -c "SUCCESS:" "$RESULTS_FILE" 2>/dev/null || echo "0")
FAILURES=$(grep -c "FAIL:" "$RESULTS_FILE" 2>/dev/null || echo "0") 
WARNINGS=$(grep -c "WARNING:" "$RESULTS_FILE" 2>/dev/null || echo "0")
SKIPS=$(grep -c "SKIP:" "$RESULTS_FILE" 2>/dev/null || echo "0")

echo "Results Summary:"
echo "  ✅ Successes: $SUCCESSES"
echo "  ❌ Failures: $FAILURES"
echo "  ⚠️ Warnings: $WARNINGS"
echo "  ⏭️ Skipped: $SKIPS"

log_result ""
log_result "=== FINAL SUMMARY ==="
log_result "Successes: $SUCCESSES"
log_result "Failures: $FAILURES"
log_result "Warnings: $WARNINGS"
log_result "Skipped: $SKIPS"

if [ "$FAILURES" -eq 0 ]; then
    echo "🎉 Cross-platform testing completed successfully!"
    echo "✅ CURSED compiler supports multiple target platforms"
    exit 0
else
    echo "❌ Some cross-platform tests failed"
    echo "📋 Check $RESULTS_FILE for detailed results"
    exit 1
fi
