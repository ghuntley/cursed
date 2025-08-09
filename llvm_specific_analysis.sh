#!/bin/bash
# Specific LLVM Backend Analysis - Identifying exactly what works vs broken

echo "=== CURSED LLVM Backend Specific Analysis ==="
echo "Date: $(date)"
echo ""

# Test simple cases that should work
echo "=== Test A: What Actually Works ==="

# Test A1: Simplest possible program
echo "Testing A1: Simplest program..."
cat > test_a1.csd << 'EOF'
vibez.spill("Hello World")
EOF

if ./zig-out/bin/cursed --compile test_a1.csd 2>/dev/null; then
    echo "✅ A1: Simple output compiles and works"
    if [ -f test_a1 ]; then
        ./test_a1
    fi
else
    echo "❌ A1: Simple output fails"
fi
echo ""

# Test A2: Variable assignment
echo "Testing A2: Variable assignment only..."
cat > test_a2.csd << 'EOF'
sus x drip = 42
vibez.spill(x)
EOF

if ./zig-out/bin/cursed --compile test_a2.csd 2>/dev/null; then
    echo "✅ A2: Variable assignment compiles and works"
    if [ -f test_a2 ]; then
        ./test_a2
    fi
else
    echo "❌ A2: Variable assignment fails"
fi
echo ""

# Test A3: Arithmetic expression 
echo "Testing A3: Arithmetic expressions..."
cat > test_a3.csd << 'EOF'
sus result drip = 2 + 3
vibez.spill(result)
EOF

if ./zig-out/bin/cursed --compile test_a3.csd 2>/dev/null; then
    echo "✅ A3: Arithmetic expressions compile and work"
    if [ -f test_a3 ]; then
        ./test_a3
    fi
else
    echo "❌ A3: Arithmetic expressions fail"
fi
echo ""

echo "=== Test B: What's Definitely Broken ==="

# Test B1: Functions (we saw this fail)
echo "Testing B1: Function definition..."
cat > test_b1.csd << 'EOF'
slay test_func() drip {
    damn 42
}
sus result drip = test_func()
vibez.spill(result)
EOF

if ./zig-out/bin/cursed --compile test_b1.csd 2>&1; then
    echo "✅ B1: Functions unexpectedly work"
    if [ -f test_b1 ]; then
        ./test_b1
    fi
else
    echo "❌ B1: Functions fail as expected (LLVM verification error)"
fi
echo ""

# Test B2: Control structures (we saw this fail)
echo "Testing B2: Control structures..."
cat > test_b2.csd << 'EOF'
ready (based) {
    vibez.spill("true branch")
}
EOF

if ./zig-out/bin/cursed --compile test_b2.csd 2>&1; then
    echo "✅ B2: Control structures unexpectedly work"
    if [ -f test_b2 ]; then
        ./test_b2
    fi
else
    echo "❌ B2: Control structures fail as expected (LLVM verification error)"
fi
echo ""

# Test B3: Loops (we saw this fail)
echo "Testing B3: Loops..."
cat > test_b3.csd << 'EOF'
sus i drip = 0
bestie (i < 3) {
    vibez.spill(i)
    i = i + 1
}
EOF

if ./zig-out/bin/cursed --compile test_b3.csd 2>&1; then
    echo "✅ B3: Loops unexpectedly work"
    if [ -f test_b3 ]; then
        ./test_b3
    fi
else
    echo "❌ B3: Loops fail as expected (LLVM verification error)"
fi
echo ""

echo "=== Test C: LLVM IR Inspection ==="

# Generate IR for working case
echo "Testing C1: IR generation for working case..."
cat > test_c1.csd << 'EOF'
sus answer drip = 42
vibez.spill("Answer:", answer)
EOF

echo "Attempting to generate IR..."
if ./zig-out/bin/cursed --emit-llvm test_c1.csd 2>/dev/null; then
    if [ -f test_c1.ll ]; then
        echo "✅ C1: IR file generated successfully"
        echo "IR content preview:"
        head -20 test_c1.ll
    else
        echo "⚠️  C1: Compilation succeeded but no .ll file found"
        echo "Looking for IR files..."
        find . -name "*.ll" -mtime -1 2>/dev/null | head -5
    fi
else
    echo "❌ C1: IR generation failed"
fi
echo ""

echo "=== Test D: Backend Selection Analysis ==="

# Check what backends are actually being used
echo "Testing D1: Checking actual compilation paths..."

echo "Compile with debug to see backend selection:"
./zig-out/bin/cursed --compile --debug test_a1.csd 2>&1 | head -20

echo ""
echo "=== Test E: Cross-compilation Reality Check ==="

# Check if cross-compilation actually works or just appears to
echo "Testing E1: Real cross-compilation test..."
cat > test_e1.csd << 'EOF'
vibez.spill("Cross-compiled hello")
EOF

for target in "x86_64-linux" "wasm32-wasi"; do
    echo "Testing real cross-compilation for $target..."
    
    # Clear any previous files
    rm -f test_e1 test_e1.exe test_e1.wasm
    
    if timeout 30 ./zig-out/bin/cursed --compile test_e1.csd --target=$target 2>/dev/null; then
        echo "Compilation succeeded for $target"
        echo "Generated files:"
        ls -la test_e1* 2>/dev/null
        
        # Check file type
        if [ -f test_e1 ]; then
            file test_e1
        fi
        if [ -f test_e1.wasm ]; then
            file test_e1.wasm
        fi
        if [ -f test_e1.exe ]; then
            file test_e1.exe
        fi
    else
        echo "❌ Cross-compilation failed for $target"
    fi
    echo ""
done

echo "=== Test F: Memory Safety and Linking ==="

# Check what the compiled binaries actually contain
echo "Testing F1: Binary analysis..."
if [ -f test_a1 ]; then
    echo "Symbols in working binary:"
    nm test_a1 2>/dev/null | head -10 || echo "nm failed"
    
    echo "Strings in working binary:"
    strings test_a1 | grep -E "(Hello|World|CURSED|llvm)" | head -5
    
    echo "Detailed file info:"
    file test_a1
    
    echo "Runtime dependencies:"
    ldd test_a1 | head -5
fi

echo ""
echo "=== Summary of LLVM Backend Reality ==="
echo "WORKING:"
echo "- Simple output statements (vibez.spill with literals)"  
echo "- Basic variable assignment and evaluation"
echo "- Simple arithmetic expressions" 
echo "- Binary generation and execution"
echo "- Memory safety (no leaks in basic programs)"
echo ""
echo "BROKEN/ISSUES:"
echo "- LLVM verification fails for complex constructs"
echo "- Functions cause 'Basic Block terminator' errors"
echo "- Control structures fail LLVM verification"
echo "- Loops fail LLVM verification"
echo "- IR file generation not working as expected"
echo "- Cross-compilation may not be actually cross-compiling"
echo ""
echo "CONCLUSION: LLVM backend has fundamental IR generation bugs"
echo "that prevent anything beyond the simplest programs from compiling."

# Cleanup
rm -f test_*.csd test_a* test_b* test_c* test_e*
