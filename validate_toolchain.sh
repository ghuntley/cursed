#!/bin/bash

# CURSED Toolchain Validation Script
# Comprehensive validation of build environment and toolchain setup

set -e

echo "🔍 CURSED Toolchain Validation"
echo "=============================="

# Load environment if available
source ~/.cursed_env 2>/dev/null || true

VALIDATION_PASSED=0
VALIDATION_TOTAL=0

validate_command() {
    local cmd="$1"
    local description="$2"
    ((VALIDATION_TOTAL++))
    
    echo -n "Checking $description... "
    if command -v "$cmd" >/dev/null 2>&1; then
        echo "✅ PASS"
        ((VALIDATION_PASSED++))
        return 0
    else
        echo "❌ FAIL"
        return 1
    fi
}

validate_file() {
    local file="$1"
    local description="$2"
    ((VALIDATION_TOTAL++))
    
    echo -n "Checking $description... "
    if [ -f "$file" ]; then
        echo "✅ PASS"
        ((VALIDATION_PASSED++))
        return 0
    else
        echo "❌ FAIL"
        return 1
    fi
}

validate_env_var() {
    local var="$1"
    local description="$2"
    ((VALIDATION_TOTAL++))
    
    echo -n "Checking $description... "
    if [ -n "${!var}" ]; then
        echo "✅ PASS (${!var})"
        ((VALIDATION_PASSED++))
        return 0
    else
        echo "❌ FAIL"
        return 1
    fi
}

echo ""
echo "📋 Core Toolchain Validation"
echo "----------------------------"
validate_command "zig" "Zig compiler"
validate_command "gcc" "GCC compiler"
validate_command "clang-18" "Clang 18 compiler"
validate_command "llvm-config-18" "LLVM config"
validate_command "llvm-ar" "LLVM archiver"

echo ""
echo "📋 Cross-compilation Toolchain Validation"
echo "------------------------------------------"
validate_command "aarch64-linux-gnu-gcc" "ARM64 cross-compiler"
validate_command "x86_64-w64-mingw32-gcc" "Windows cross-compiler"

echo ""
echo "📋 LLVM Library Validation"
echo "---------------------------"
validate_file "/usr/lib/llvm-18/lib/libLLVM.so" "LLVM shared library"
validate_file "/usr/include/llvm-18" "LLVM headers"

echo ""
echo "📋 Environment Variables Validation"
echo "------------------------------------"
validate_env_var "LLVM_SYS_181_PREFIX" "LLVM prefix"
validate_env_var "LLVM_CONFIG_PATH" "LLVM config path"
validate_env_var "CC" "C compiler"
validate_env_var "CXX" "C++ compiler"

echo ""
echo "🔨 Build System Validation"
echo "---------------------------"
echo -n "Testing CURSED build... "
((VALIDATION_TOTAL++))
if zig build >/dev/null 2>&1; then
    echo "✅ PASS"
    ((VALIDATION_PASSED++))
else
    echo "❌ FAIL"
fi

echo -n "Testing CURSED execution... "
((VALIDATION_TOTAL++))
if echo 'vibez.spill("Validation test")' > test_validation.csd && \
   ./zig-out/bin/cursed test_validation.csd >/dev/null 2>&1; then
    echo "✅ PASS"
    ((VALIDATION_PASSED++))
    rm -f test_validation.csd
else
    echo "❌ FAIL"
    rm -f test_validation.csd
fi

echo -n "Testing cross-compilation (Linux ARM64)... "
((VALIDATION_TOTAL++))
if zig build -Dtarget=aarch64-linux >/dev/null 2>&1; then
    echo "✅ PASS"
    ((VALIDATION_PASSED++))
else
    echo "❌ FAIL"
fi

echo ""
echo "📊 Validation Summary"
echo "====================="
echo "Passed: $VALIDATION_PASSED / $VALIDATION_TOTAL"

if [ "$VALIDATION_PASSED" -eq "$VALIDATION_TOTAL" ]; then
    echo "🎉 All validations passed! Toolchain is ready."
    exit 0
elif [ "$VALIDATION_PASSED" -ge $((VALIDATION_TOTAL * 80 / 100)) ]; then
    echo "⚠️  Most validations passed. Minor issues detected."
    exit 0
else
    echo "❌ Multiple validation failures. Please run setup_build_environment.sh"
    exit 1
fi
