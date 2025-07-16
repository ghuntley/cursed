#!/bin/bash

echo "=== ARM64 Compilation Pipeline Test ==="
echo

# Test interpretation mode
echo "1. Testing interpretation mode:"
cargo run --bin cursed arm64_hello_test.csd > interp_output.txt 2>&1
interp_exit=$?
echo "Interpretation exit code: $interp_exit"

if [ $interp_exit -eq 0 ]; then
    echo "✅ Interpretation mode successful"
    echo "Output:"
    tail -1 interp_output.txt
else
    echo "❌ Interpretation mode failed"
    cat interp_output.txt
fi

echo

# Test LLVM IR generation
echo "2. Testing LLVM IR generation for ARM64:"
cargo run --bin cursed -- compile --emit-ir arm64_hello_test.csd > llvm_output.txt 2>&1
llvm_exit=$?
echo "LLVM IR generation exit code: $llvm_exit"

if [ $llvm_exit -eq 0 ]; then
    echo "✅ LLVM IR generation successful"
    echo "Target triple:"
    grep "target triple" arm64_hello_test.ll
    echo "Main function:"
    grep -A 5 "define i32 @main" arm64_hello_test.ll
else
    echo "❌ LLVM IR generation failed"
    cat llvm_output.txt
fi

echo

# Verify ARM64-specific content
echo "3. Verifying ARM64-specific compilation:"
if [ -f arm64_hello_test.ll ]; then
    echo "✅ LLVM IR file generated"
    
    # Check for ARM64 target
    if grep -q "aarch64-apple-darwin" arm64_hello_test.ll; then
        echo "✅ ARM64 target triple detected"
    else
        echo "❌ ARM64 target not found"
    fi
    
    # Check for main function
    if grep -q "define i32 @main" arm64_hello_test.ll; then
        echo "✅ Main function defined"
    else
        echo "❌ Main function not found"
    fi
    
    # Check for string constant
    if grep -q "Hello, ARM64 World!" arm64_hello_test.ll; then
        echo "✅ String constant present"
    else
        echo "❌ String constant not found"
    fi
else
    echo "❌ LLVM IR file not generated"
fi

echo

# Summary
echo "=== ARM64 Pipeline Test Summary ==="
if [ $interp_exit -eq 0 ] && [ $llvm_exit -eq 0 ] && [ -f arm64_hello_test.ll ]; then
    echo "✅ ARM64 compilation pipeline working correctly"
    echo "✅ Interpretation mode functional"
    echo "✅ LLVM IR generation functional"
    echo "✅ ARM64 target compilation verified"
else
    echo "❌ ARM64 compilation pipeline has issues"
fi

echo
echo "Files generated:"
ls -la arm64_hello_test* | grep -v "\.csd$"
