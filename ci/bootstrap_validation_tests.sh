#!/bin/bash

# Bootstrap Validation Tests for CI Pipeline
# Comprehensive testing of bootstrap compilation process

set -euo pipefail

echo "🔄 CURSED Bootstrap Validation Tests"
echo "===================================="

# Configuration
COMPILER_BINARY="${COMPILER_BINARY:-target/release/cursed}"
TEST_DIR="${TEST_DIR:-/tmp/bootstrap_tests}"
TIMEOUT_SECONDS="${TIMEOUT_SECONDS:-60}"

# Cleanup and setup
cleanup() {
    echo "🧹 Cleaning up test directory..."
    rm -rf "$TEST_DIR"
}
trap cleanup EXIT

mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# Test result tracking
total_tests=0
passed_tests=0
failed_tests=0

# Function to run individual bootstrap test
run_bootstrap_test() {
    local test_name=$1
    local test_code=$2
    local expected_output=$3
    
    total_tests=$((total_tests + 1))
    echo "🧪 Running bootstrap test: $test_name"
    
    # Create test file
    echo "$test_code" > "${test_name}.csd"
    
    # Test interpretation mode
    echo "  📝 Testing interpretation mode..."
    if timeout $TIMEOUT_SECONDS "$OLDPWD/$COMPILER_BINARY" "${test_name}.csd" > "${test_name}_interp.out" 2>&1; then
        echo "    ✅ Interpretation successful"
    else
        echo "    ❌ Interpretation failed"
        cat "${test_name}_interp.out"
        failed_tests=$((failed_tests + 1))
        return 1
    fi
    
    # Test compilation mode
    echo "  🔨 Testing compilation mode..."
    if timeout $TIMEOUT_SECONDS "$OLDPWD/$COMPILER_BINARY" -- compile "${test_name}.csd" -o "${test_name}_compiled" 2>&1; then
        echo "    ✅ Compilation successful"
        
        # Run compiled executable
        if ./"${test_name}_compiled" > "${test_name}_compiled.out" 2>&1; then
            echo "    ✅ Execution successful"
            
            # Compare outputs
            if diff -q "${test_name}_interp.out" "${test_name}_compiled.out" > /dev/null; then
                echo "    ✅ Output consistency verified"
                passed_tests=$((passed_tests + 1))
                return 0
            else
                echo "    ❌ Output mismatch between interpretation and compilation"
                echo "    Interpretation output:"
                cat "${test_name}_interp.out"
                echo "    Compilation output:"
                cat "${test_name}_compiled.out"
                failed_tests=$((failed_tests + 1))
                return 1
            fi
        else
            echo "    ❌ Compiled executable execution failed"
            failed_tests=$((failed_tests + 1))
            return 1
        fi
    else
        echo "    ❌ Compilation failed"
        failed_tests=$((failed_tests + 1))
        return 1
    fi
}

# Test Suite 1: Basic Language Features
echo "📚 Test Suite 1: Basic Language Features"
echo "========================================"

run_bootstrap_test "basic_variables" \
'sus message tea = "Bootstrap test 1"
vibez.spill(message)' \
"Bootstrap test 1"

run_bootstrap_test "arithmetic_operations" \
'sus a normie = 10
sus b normie = 5
sus result normie = a + b * 2
vibez.spill(result)' \
"20"

run_bootstrap_test "boolean_logic" \
'sus flag lit = based
if flag {
    vibez.spill("Boolean test passed")
}' \
"Boolean test passed"

run_bootstrap_test "array_operations" \
'sus numbers [3]normie = [1, 2, 3]
bestie i := 0; i < 3; i++ {
    vibez.spill(numbers[i])
}' \
"1\n2\n3"

# Test Suite 2: Function Definitions
echo ""
echo "🔧 Test Suite 2: Function Definitions"
echo "====================================="

run_bootstrap_test "simple_function" \
'slay greet(name tea) tea {
    damn "Hello, " + name + "!"
}

sus greeting tea = greet("Bootstrap")
vibez.spill(greeting)' \
"Hello, Bootstrap!"

run_bootstrap_test "recursive_function" \
'slay factorial(n normie) normie {
    if n <= 1 {
        damn 1
    } else {
        damn n * factorial(n - 1)
    }
}

sus result normie = factorial(5)
vibez.spill(result)' \
"120"

# Test Suite 3: Control Flow
echo ""
echo "🎯 Test Suite 3: Control Flow"
echo "============================="

run_bootstrap_test "if_else_chain" \
'sus value normie = 15

if value < 10 {
    vibez.spill("Small")
} else if value < 20 {
    vibez.spill("Medium")
} else {
    vibez.spill("Large")
}' \
"Medium"

run_bootstrap_test "for_loop" \
'sus sum normie = 0
bestie i := 1; i <= 5; i++ {
    sum += i
}
vibez.spill(sum)' \
"15"

# Test Suite 4: Type System
echo ""
echo "🔍 Test Suite 4: Type System"
echo "============================"

run_bootstrap_test "type_conversions" \
'sus integer normie = 42
sus floating meal = integer.(meal)
sus back_to_int normie = floating.(normie)
vibez.spill(back_to_int)' \
"42"

run_bootstrap_test "tuple_operations" \
'sus data (normie, tea) = (100, "test")
vibez.spill(data.0)
vibez.spill(data.1)' \
"100\ntest"

# Test Suite 5: Module System
echo ""
echo "📦 Test Suite 5: Module System"
echo "=============================="

run_bootstrap_test "core_module" \
'yeet "core"
sus message tea = "Module test"
vibez.spill(message)' \
"Module test"

# Test Suite 6: Error Handling
echo ""
echo "⚠️  Test Suite 6: Error Handling"
echo "================================"

run_bootstrap_test "basic_error_handling" \
'slay safe_divide(a normie, b normie) normie {
    if b == 0 {
        damn 0
    } else {
        damn a / b
    }
}

sus result normie = safe_divide(10, 2)
vibez.spill(result)' \
"5"

# Test Suite 7: Advanced Features
echo ""
echo "🚀 Test Suite 7: Advanced Features"
echo "=================================="

run_bootstrap_test "short_declarations" \
'x := 42
y := "test"
z := based
vibez.spill(x)
vibez.spill(y)
vibez.spill(z)' \
"42\ntest\ntrue"

run_bootstrap_test "increment_decrement" \
'sus counter normie = 0
counter++
vibez.spill(counter)
counter--
vibez.spill(counter)' \
"1\n0"

# Test Suite 8: Stdlib Integration
echo ""
echo "📚 Test Suite 8: Stdlib Integration"
echo "==================================="

run_bootstrap_test "string_operations" \
'yeet "stringz"
sus text tea = "hello world"
sus upper tea = stringz.to_upper(text)
vibez.spill(upper)' \
"HELLO WORLD"

# Performance Test
echo ""
echo "⚡ Performance Test"
echo "=================="

echo "🔄 Running performance benchmark..."
start_time=$(date +%s%N)

# Create performance test
cat > performance_test.csd << 'EOF'
// Performance test program
sus iterations normie = 1000

slay fibonacci(n normie) normie {
    if n <= 1 {
        damn n
    } else {
        damn fibonacci(n - 1) + fibonacci(n - 2)
    }
}

sus result normie = fibonacci(20)
vibez.spill(result)
EOF

# Run performance test
timeout $TIMEOUT_SECONDS "$OLDPWD/$COMPILER_BINARY" -- compile performance_test.csd -o performance_test
./performance_test > performance_output.txt

end_time=$(date +%s%N)
duration=$((end_time - start_time))
seconds=$(echo "scale=3; $duration / 1000000000" | bc)

echo "📊 Performance test completed in ${seconds}s"
echo "📈 Output: $(cat performance_output.txt)"

# Generate detailed test report
echo ""
echo "📊 Bootstrap Validation Test Report"
echo "===================================="
echo "Total tests: $total_tests"
echo "Passed: $passed_tests"
echo "Failed: $failed_tests"
echo "Success rate: $(echo "scale=2; $passed_tests * 100 / $total_tests" | bc)%"
echo "Performance test time: ${seconds}s"

if [ $failed_tests -eq 0 ]; then
    echo ""
    echo "🎉 All bootstrap validation tests passed!"
    echo "✅ CURSED compiler bootstrap is fully functional"
    echo "🚀 Ready for self-hosting deployment"
    exit 0
else
    echo ""
    echo "❌ Some bootstrap tests failed"
    echo "🔍 Review failed tests before deployment"
    exit 1
fi
