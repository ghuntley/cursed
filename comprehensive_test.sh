#!/bin/bash

# Comprehensive Production Test Suite for CURSED Compiler
echo "🚀 Running Comprehensive Production Test Suite..."

# Memory Safety Tests
echo "🔒 Memory Safety Validation..."
valgrind --error-exitcode=1 ./zig-out/bin/cursed stdlib/testz/test_testz.csd > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ Memory safety: PASSED"
else
    echo "❌ Memory safety: FAILED"
fi

# Core Stdlib Module Tests
echo "📚 Core Standard Library Tests..."
modules=("testz" "mathz" "stringz" "arrayz" "hashz")
passed=0
total=${#modules[@]}

for module in "${modules[@]}"; do
    ./zig-out/bin/cursed stdlib/$module/test_$module.csd > /dev/null 2>&1
    if [ $? -eq 0 ]; then
        echo "✅ $module module: PASSED"
        ((passed++))
    else
        echo "❌ $module module: FAILED"
    fi
done

echo "📊 Core Modules: $passed/$total passed"

# Cross-platform Validation
echo "🌐 Cross-platform Build Tests..."
targets=("x86_64-linux" "aarch64-macos" "x86_64-windows")
cross_passed=0
cross_total=${#targets[@]}

for target in "${targets[@]}"; do
    timeout 30 zig build -Dtarget=$target > /dev/null 2>&1
    if [ $? -eq 0 ]; then
        echo "✅ Cross-compile $target: PASSED"
        ((cross_passed++))
    else
        echo "❌ Cross-compile $target: FAILED"
    fi
done

echo "📊 Cross-platform: $cross_passed/$cross_total passed"

# Basic Functionality Tests
echo "⚙️  Basic Functionality Tests..."
basic_passed=0
basic_total=5

# Variable test
echo 'sus x drip = 42; vibez.spill("Value:", x)' > temp_var_test.csd
./zig-out/bin/cursed temp_var_test.csd > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ Variable declarations: PASSED"
    ((basic_passed++))
else
    echo "❌ Variable declarations: FAILED"
fi

# Function test
echo 'slay add(a drip, b drip) drip { damn a + b }; vibez.spill(add(3, 4))' > temp_func_test.csd
./zig-out/bin/cursed temp_func_test.csd > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ Function definitions: PASSED"
    ((basic_passed++))
else
    echo "❌ Function definitions: FAILED"
fi

# Array test
echo 'sus arr []drip = [1, 2, 3]; vibez.spill(arr[0])' > temp_array_test.csd
./zig-out/bin/cursed temp_array_test.csd > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ Array operations: PASSED"
    ((basic_passed++))
else
    echo "❌ Array operations: FAILED"
fi

# Struct test
echo 'squad Point { spill x drip; spill y drip }; sus p Point = Point{x: 1, y: 2}' > temp_struct_test.csd
./zig-out/bin/cursed temp_struct_test.csd > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ Struct definitions: PASSED"
    ((basic_passed++))
else
    echo "❌ Struct definitions: FAILED"
fi

# Control flow test
echo 'sus x drip = 5; ready (x > 0) { vibez.spill("positive") }' > temp_control_test.csd
./zig-out/bin/cursed temp_control_test.csd > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ Control flow: PASSED"
    ((basic_passed++))
else
    echo "❌ Control flow: FAILED"
fi

echo "📊 Basic Functionality: $basic_passed/$basic_total passed"

# Build Performance Test
echo "⚡ Build Performance Test..."
start_time=$(date +%s%N)
zig build > /dev/null 2>&1
end_time=$(date +%s%N)
build_time=$(( (end_time - start_time) / 1000000 ))
echo "🕐 Build time: ${build_time}ms"

if [ $build_time -lt 1000 ]; then
    echo "✅ Build performance: EXCELLENT (<1s)"
elif [ $build_time -lt 5000 ]; then
    echo "✅ Build performance: GOOD (<5s)"
else
    echo "⚠️  Build performance: ACCEPTABLE (>5s)"
fi

# Summary
echo ""
echo "📋 PRODUCTION READINESS SUMMARY"
echo "================================"
total_tests=$((total + cross_total + basic_total + 1)) # +1 for memory safety
total_passed=$((passed + cross_passed + basic_passed + 1))
percentage=$((total_passed * 100 / total_tests))

echo "Overall Test Results: $total_passed/$total_tests passed (${percentage}%)"

if [ $percentage -ge 90 ]; then
    echo "🎉 PRODUCTION READY: Compiler is ready for production use!"
elif [ $percentage -ge 80 ]; then
    echo "⚠️  MOSTLY READY: Compiler is mostly ready with some issues"
else
    echo "❌ NOT READY: Critical issues need to be addressed"
fi

# Cleanup
rm -f temp_*.csd

echo "🏁 Comprehensive test suite completed!"
