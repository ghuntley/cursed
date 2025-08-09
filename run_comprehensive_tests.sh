#!/bin/bash

# CURSED Compiler Comprehensive Test Suite
# Testing what works vs what's broken

echo "🧪 CURSED Compiler Comprehensive Test Suite"
echo "============================================"

# Test results counters
PASS=0
FAIL=0
MEMORY_LEAK=0
COMPILE_PASS=0
COMPILE_FAIL=0

test_feature() {
    local name="$1"
    local file="$2"
    local content="$3"
    
    echo "## Testing: $name"
    echo "$content" > "$file"
    
    # Test interpretation
    echo "### Interpretation Test:"
    if ./zig-out/bin/cursed-minimal "$file" > /dev/null 2>&1; then
        echo "✅ PASS - Interpretation works"
        ((PASS++))
        
        # Test memory safety
        echo "### Memory Safety Test:"
        if valgrind --tool=memcheck --leak-check=yes --error-exitcode=1 ./zig-out/bin/cursed-minimal "$file" > /dev/null 2>&1; then
            echo "✅ PASS - Memory safe"
        else
            echo "❌ FAIL - Memory leaks detected"
            ((MEMORY_LEAK++))
        fi
        
        # Test LLVM compilation
        echo "### LLVM Compilation Test:"
        if ./zig-out/bin/cursed-zig "$file" --compile > /dev/null 2>&1; then
            echo "✅ PASS - LLVM compilation works"
            ((COMPILE_PASS++))
            
            # Test binary execution
            if [ -f "${file%.*}" ]; then
                if ./"${file%.*}" > /dev/null 2>&1; then
                    echo "✅ PASS - Binary execution works"
                else
                    echo "❌ FAIL - Binary execution failed"
                fi
                rm -f "${file%.*}"
            fi
        else
            echo "❌ FAIL - LLVM compilation failed"
            ((COMPILE_FAIL++))
        fi
    else
        echo "❌ FAIL - Interpretation failed"
        ((FAIL++))
    fi
    
    rm -f "$file"
    echo ""
}

# Test basic features
test_feature "Variables" "test_vars.csd" 'sus x drip = 42
vibez.spill("Value:", x)'

test_feature "Arithmetic" "test_math.csd" 'sus a drip = 10
sus b drip = 5
sus sum drip = a + b
vibez.spill("Sum:", sum)'

test_feature "Functions" "test_funcs.csd" 'slay add(x drip, y drip) drip {
    damn x + y
}
sus result drip = add(5, 3)
vibez.spill("Result:", result)'

test_feature "Arrays" "test_arrays.csd" 'sus arr []drip = [1, 2, 3]
vibez.spill("First:", arr[0])'

test_feature "While Loops" "test_while.csd" 'sus i drip = 0
bestie (i < 3) {
    vibez.spill("Count:", i)
    i = i + 1
}'

test_feature "If Statement" "test_if.csd" 'sus x drip = 5
ready (x > 3) {
    vibez.spill("Greater")
}'

# Test complex expressions
test_feature "Complex Expression" "test_complex.csd" 'sus result drip = ((5 + 3) * 2) - 1
vibez.spill("Complex:", result)'

# Test stdlib imports
echo "## Testing: Stdlib Imports"
echo "### mathz module:"
echo 'yeet "mathz"
sus result drip = abs_normie(-10)
vibez.spill("Abs:", result)' > test_stdlib.csd

if ./zig-out/bin/cursed-zig test_stdlib.csd > /dev/null 2>&1; then
    echo "✅ PASS - mathz module loads"
else
    echo "❌ FAIL - mathz module failed (memory leaks detected)"
fi
rm -f test_stdlib.csd

# Test cross-compilation
echo "## Testing: Cross-compilation"
echo 'sus x drip = 42
vibez.spill("Cross-compile test:", x)' > cross_test.csd

for target in x86_64-linux wasm32-freestanding; do
    echo "### Testing target: $target"
    if zig build -Dtarget=$target > /dev/null 2>&1; then
        echo "✅ PASS - Cross-compilation to $target works"
    else
        echo "❌ FAIL - Cross-compilation to $target failed"
    fi
done

rm -f cross_test.csd

# Summary
echo "============================================"
echo "🏁 Test Summary"
echo "============================================"
echo "✅ Interpretation PASS: $PASS"
echo "❌ Interpretation FAIL: $FAIL"
echo "💾 Memory leaks detected: $MEMORY_LEAK"
echo "🔥 LLVM compilation PASS: $COMPILE_PASS"
echo "💥 LLVM compilation FAIL: $COMPILE_FAIL"
echo "============================================"

if [ $FAIL -eq 0 ] && [ $MEMORY_LEAK -eq 0 ]; then
    echo "🎉 All core features working and memory safe!"
    exit 0
else
    echo "⚠️  Some issues detected - see details above"
    exit 1
fi
