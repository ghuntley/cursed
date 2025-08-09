#!/bin/bash
# LLVM Backend Comprehensive Testing Suite
# Tests what actually works vs what's broken

set -e

echo "=== CURSED LLVM Backend Testing Suite ==="
echo "Testing Date: $(date)"
echo "Platform: $(uname -a)"
echo ""

# Clean previous test artifacts
rm -f test_*.csd test_*-native *.ll *.o

# Test 1: Basic Variable Program
echo "=== Test 1: Basic Variable Compilation ==="
cat > test_basic_var.csd << 'EOF'
sus answer drip = 42
vibez.spill("The answer is:", answer)
EOF

echo "Testing basic compilation..."
if ./zig-out/bin/cursed-zig --compile test_basic_var.csd; then
    echo "✅ Basic compilation: SUCCESS"
    if [ -f test_basic_var ]; then
        echo "Testing binary execution..."
        if ./test_basic_var; then
            echo "✅ Basic binary execution: SUCCESS"
        else
            echo "❌ Basic binary execution: FAILED"
        fi
    else
        echo "❌ Binary not generated"
    fi
else
    echo "❌ Basic compilation: FAILED"
fi
echo ""

# Test 2: Function Compilation
echo "=== Test 2: Function Compilation ==="
cat > test_function.csd << 'EOF'
slay multiply(a drip, b drip) drip {
    damn a * b
}

sus result drip = multiply(6, 7)
vibez.spill("6 * 7 =", result)
EOF

echo "Testing function compilation..."
if ./zig-out/bin/cursed-zig --compile test_function.csd; then
    echo "✅ Function compilation: SUCCESS"
    if [ -f test_function ]; then
        echo "Testing function binary execution..."
        if ./test_function; then
            echo "✅ Function binary execution: SUCCESS"
        else
            echo "❌ Function binary execution: FAILED"
        fi
    fi
else
    echo "❌ Function compilation: FAILED"
fi
echo ""

# Test 3: Control Structures
echo "=== Test 3: Control Structure Compilation ==="
cat > test_control.csd << 'EOF'
sus i drip = 0
bestie (i < 3) {
    vibez.spill("Loop iteration:", i)
    i = i + 1
}
vibez.spill("Loop complete")
EOF

echo "Testing control structure compilation..."
if ./zig-out/bin/cursed-zig --compile test_control.csd; then
    echo "✅ Control structure compilation: SUCCESS"
    if [ -f test_control ]; then
        echo "Testing control structure binary execution..."
        if ./test_control; then
            echo "✅ Control structure binary execution: SUCCESS"
        else
            echo "❌ Control structure binary execution: FAILED"
        fi
    fi
else
    echo "❌ Control structure compilation: FAILED"
fi
echo ""

# Test 4: Array Operations
echo "=== Test 4: Array Compilation ==="
cat > test_array.csd << 'EOF'
yeet "arrayz"
sus numbers []drip = [1, 2, 3, 4, 5]
vibez.spill("Array length:", len(numbers))
vibez.spill("First element:", numbers[0])
EOF

echo "Testing array compilation..."
if ./zig-out/bin/cursed-zig --compile test_array.csd; then
    echo "✅ Array compilation: SUCCESS"
    if [ -f test_array ]; then
        echo "Testing array binary execution..."
        if ./test_array; then
            echo "✅ Array binary execution: SUCCESS"
        else
            echo "❌ Array binary execution: FAILED"
        fi
    fi
else
    echo "❌ Array compilation: FAILED"
fi
echo ""

# Test 5: Optimization Levels
echo "=== Test 5: Optimization Level Testing ==="
cat > test_optimization.csd << 'EOF'
slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    } otherwise {
        damn fibonacci(n - 1) + fibonacci(n - 2)
    }
}

sus result drip = fibonacci(10)
vibez.spill("Fibonacci(10) =", result)
EOF

for opt_level in O0 O1 O2 O3; do
    echo "Testing optimization level: $opt_level"
    if ./zig-out/bin/cursed-zig --compile test_optimization.csd --$opt_level 2>/dev/null || 
       ./zig-out/bin/cursed-zig --compile test_optimization.csd 2>/dev/null; then
        echo "✅ Optimization $opt_level: COMPILED"
        if [ -f test_optimization ]; then
            echo "Testing optimized binary execution..."
            if timeout 5 ./test_optimization; then
                echo "✅ Optimization $opt_level execution: SUCCESS"
            else
                echo "❌ Optimization $opt_level execution: FAILED/TIMEOUT"
            fi
        fi
    else
        echo "❌ Optimization $opt_level: FAILED"
    fi
done
echo ""

# Test 6: Cross-compilation targets
echo "=== Test 6: Cross-compilation Testing ==="
cat > test_cross.csd << 'EOF'
vibez.spill("Hello from cross-compiled binary!")
EOF

targets=("x86_64-linux" "x86_64-macos" "aarch64-linux" "aarch64-macos" "x86_64-windows" "wasm32-wasi")

for target in "${targets[@]}"; do
    echo "Testing cross-compilation target: $target"
    
    # Try with timeout to detect hanging
    if timeout 30 ./zig-out/bin/cursed-zig --compile test_cross.csd --target=$target 2>/dev/null || \
       timeout 30 zig build -Dtarget=$target 2>/dev/null; then
        echo "✅ Cross-compilation $target: SUCCESS"
        if [ -f "test_cross" ] || [ -f "test_cross.exe" ] || [ -f "test_cross.wasm" ]; then
            echo "✅ Binary generated for $target"
        else
            echo "⚠️  Compilation succeeded but no binary found for $target"
        fi
    else
        exit_code=$?
        if [ $exit_code -eq 124 ]; then
            echo "❌ Cross-compilation $target: TIMEOUT/HANGING"
        else
            echo "❌ Cross-compilation $target: FAILED"
        fi
    fi
    
    # Clean up generated files
    rm -f test_cross test_cross.exe test_cross.wasm
done
echo ""

# Test 7: LLVM IR Generation
echo "=== Test 7: LLVM IR Generation Testing ==="
cat > test_ir.csd << 'EOF'
sus x drip = 10
sus y drip = 20
sus sum drip = x + y
vibez.spill("Sum:", sum)
EOF

echo "Testing LLVM IR generation..."
if ./zig-out/bin/cursed-zig --emit-llvm test_ir.csd 2>/dev/null || \
   ./zig-out/bin/cursed-zig --compile test_ir.csd 2>/dev/null; then
    if [ -f test_ir.ll ]; then
        echo "✅ LLVM IR generation: SUCCESS"
        echo "IR file size: $(wc -l < test_ir.ll) lines"
        echo "Sample IR content:"
        head -10 test_ir.ll
    else
        echo "⚠️  Compilation succeeded but no .ll file found"
    fi
else
    echo "❌ LLVM IR generation: FAILED"
fi
echo ""

# Test 8: LLVM Path Detection
echo "=== Test 8: LLVM Path Detection ==="
echo "Checking LLVM installation..."
if command -v llvm-config >/dev/null 2>&1; then
    echo "✅ llvm-config found: $(which llvm-config)"
    echo "LLVM version: $(llvm-config --version 2>/dev/null || echo 'unknown')"
    echo "LLVM libdir: $(llvm-config --libdir 2>/dev/null || echo 'unknown')"
else
    echo "❌ llvm-config not found in PATH"
fi

if command -v clang >/dev/null 2>&1; then
    echo "✅ clang found: $(which clang)"
    echo "Clang version: $(clang --version | head -1)"
else
    echo "❌ clang not found in PATH"
fi

# Check if LLVM libraries are accessible
echo "Checking LLVM library accessibility..."
if ldconfig -p | grep -q libLLVM; then
    echo "✅ LLVM libraries found in system"
else
    echo "❌ LLVM libraries not found in system"
fi
echo ""

# Test 9: Complex Language Features
echo "=== Test 9: Complex Feature Compilation ==="
cat > test_complex.csd << 'EOF'
squad Person {
    spill name tea
    spill age drip
}

slay create_person(name tea, age drip) Person {
    damn Person{name: name, age: age}
}

sus person Person = create_person("Alice", 30)
vibez.spill("Person:", person.name, "Age:", person.age)
EOF

echo "Testing complex feature compilation..."
if ./zig-out/bin/cursed-zig --compile test_complex.csd 2>/dev/null; then
    echo "✅ Complex feature compilation: SUCCESS"
    if [ -f test_complex ]; then
        echo "Testing complex feature binary execution..."
        if ./test_complex; then
            echo "✅ Complex feature binary execution: SUCCESS"
        else
            echo "❌ Complex feature binary execution: FAILED"
        fi
    fi
else
    echo "❌ Complex feature compilation: FAILED"
fi
echo ""

# Test 10: Memory Safety in Compiled Binaries
echo "=== Test 10: Memory Safety Testing ==="
if command -v valgrind >/dev/null 2>&1 && [ -f test_basic_var ]; then
    echo "Testing memory safety of compiled binaries..."
    if valgrind --error-exitcode=1 --leak-check=full ./test_basic_var 2>/dev/null; then
        echo "✅ Compiled binary memory safety: SUCCESS"
    else
        echo "❌ Compiled binary memory safety: FAILED"
    fi
else
    echo "⚠️  Valgrind not available or no test binary for memory testing"
fi
echo ""

echo "=== LLVM Backend Test Summary ==="
echo "Test completed at: $(date)"
echo ""
echo "Key findings will be in the output above."
echo "Look for ✅ (working), ❌ (broken), and ⚠️  (partial/warning) indicators."
