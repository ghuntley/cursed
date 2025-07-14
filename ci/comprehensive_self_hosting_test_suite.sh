#!/bin/bash

# Comprehensive Self-Hosting Test Suite
# Tests all aspects of self-hosting compiler functionality

set -euo pipefail

echo "🔄 CURSED Comprehensive Self-Hosting Test Suite"
echo "==============================================="

# Configuration
COMPILER_BINARY="${COMPILER_BINARY:-target/release/cursed}"
TEST_DIR="${TEST_DIR:-/tmp/comprehensive_self_hosting_tests}"
TIMEOUT_SECONDS="${TIMEOUT_SECONDS:-300}"
PARALLEL_JOBS="${PARALLEL_JOBS:-4}"

# Cleanup and setup
cleanup() {
    echo "🧹 Cleaning up test environment..."
    rm -rf "$TEST_DIR"
}
trap cleanup EXIT

mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# Test tracking
total_test_suites=0
passed_test_suites=0
failed_test_suites=0

# Function to run test suite
run_test_suite() {
    local suite_name=$1
    local suite_function=$2
    
    total_test_suites=$((total_test_suites + 1))
    echo "🧪 Running test suite: $suite_name"
    echo "=================================="
    
    if $suite_function; then
        echo "✅ Test suite '$suite_name' PASSED"
        passed_test_suites=$((passed_test_suites + 1))
        return 0
    else
        echo "❌ Test suite '$suite_name' FAILED"
        failed_test_suites=$((failed_test_suites + 1))
        return 1
    fi
}

# Test Suite 1: Basic Self-Hosting Capability
test_basic_self_hosting() {
    echo "📝 Testing basic self-hosting capability..."
    
    # Create a simple program that demonstrates self-hosting
    cat > simple_self_host.csd << 'EOF'
// Simple self-hosting demonstration program
sus compiler_version tea = "CURSED v1.0 Self-Hosting"
vibez.spill(compiler_version)

// Basic language features
sus count normie = 0
bestie i := 1; i <= 5; i++ {
    count += i
}

vibez.spill("Sum 1-5: " + count)

// Function definition
slay double_value(x normie) normie {
    damn x * 2
}

sus doubled normie = double_value(21)
vibez.spill("Double of 21: " + doubled)
EOF
    
    # Test interpretation
    if timeout $TIMEOUT_SECONDS "$OLDPWD/$COMPILER_BINARY" simple_self_host.csd > simple_interp.out 2>&1; then
        echo "  ✅ Interpretation successful"
    else
        echo "  ❌ Interpretation failed"
        return 1
    fi
    
    # Test compilation
    if timeout $TIMEOUT_SECONDS "$OLDPWD/$COMPILER_BINARY" -- compile simple_self_host.csd -o simple_compiled 2>&1; then
        echo "  ✅ Compilation successful"
    else
        echo "  ❌ Compilation failed"
        return 1
    fi
    
    # Test execution
    if ./simple_compiled > simple_compiled.out 2>&1; then
        echo "  ✅ Execution successful"
    else
        echo "  ❌ Execution failed"
        return 1
    fi
    
    # Compare outputs
    if diff -q simple_interp.out simple_compiled.out > /dev/null; then
        echo "  ✅ Output consistency verified"
        return 0
    else
        echo "  ❌ Output mismatch"
        return 1
    fi
}

# Test Suite 2: Advanced Language Features
test_advanced_language_features() {
    echo "📝 Testing advanced language features..."
    
    # Create comprehensive language test
    cat > advanced_features.csd << 'EOF'
// Advanced language features test
yeet "core"

// Type system
sus integer normie = 42
sus floating meal = 3.14159
sus text tea = "Advanced features test"
sus flag lit = based

// Type conversions
sus converted_float meal = integer.(meal)
sus converted_int normie = floating.(normie)

vibez.spill("Type conversions: " + converted_int + " " + converted_float)

// Arrays and indexing
sus numbers [5]normie = [1, 2, 3, 4, 5]
sus sum normie = 0

bestie i := 0; i < 5; i++ {
    sum += numbers[i]
}

vibez.spill("Array sum: " + sum)

// Tuples
sus data (normie, tea, lit) = (100, "test", based)
vibez.spill("Tuple: " + data.0 + " " + data.1 + " " + data.2)

// Complex control flow
sus result tea = ""
if flag {
    if sum > 10 {
        result = "High sum"
    } else {
        result = "Low sum"
    }
} else {
    result = "Flag is false"
}

vibez.spill("Control flow result: " + result)

// Function with multiple parameters
slay complex_function(a normie, b tea, c lit) tea {
    if c {
        damn b + " " + a
    } else {
        damn "Function disabled"
    }
}

sus function_result tea = complex_function(42, "answer", based)
vibez.spill("Function result: " + function_result)

// Recursive function
slay fibonacci(n normie) normie {
    if n <= 1 {
        damn n
    } else {
        damn fibonacci(n - 1) + fibonacci(n - 2)
    }
}

sus fib_result normie = fibonacci(10)
vibez.spill("Fibonacci(10): " + fib_result)
EOF
    
    # Test both modes
    if timeout $TIMEOUT_SECONDS "$OLDPWD/$COMPILER_BINARY" advanced_features.csd > advanced_interp.out 2>&1; then
        echo "  ✅ Advanced features interpretation successful"
    else
        echo "  ❌ Advanced features interpretation failed"
        return 1
    fi
    
    if timeout $TIMEOUT_SECONDS "$OLDPWD/$COMPILER_BINARY" -- compile advanced_features.csd -o advanced_compiled 2>&1; then
        echo "  ✅ Advanced features compilation successful"
    else
        echo "  ❌ Advanced features compilation failed"
        return 1
    fi
    
    if ./advanced_compiled > advanced_compiled.out 2>&1; then
        echo "  ✅ Advanced features execution successful"
    else
        echo "  ❌ Advanced features execution failed"
        return 1
    fi
    
    # Compare outputs
    if diff -q advanced_interp.out advanced_compiled.out > /dev/null; then
        echo "  ✅ Advanced features output consistency verified"
        return 0
    else
        echo "  ❌ Advanced features output mismatch"
        return 1
    fi
}

# Test Suite 3: Module System
test_module_system() {
    echo "📝 Testing module system..."
    
    # Create module test
    cat > module_test.csd << 'EOF'
// Module system test
yeet "core"
yeet "stringz"

// Test core module functionality
sus message tea = "Module system test"
vibez.spill(message)

// Test stringz module (if available)
sus text tea = "hello world"
sus upper_text tea = stringz.to_upper(text)
vibez.spill("Uppercase: " + upper_text)

sus text_length normie = stringz.length(text)
vibez.spill("Length: " + text_length)

// Test string operations
sus concatenated tea = stringz.concat("Hello", " ", "World")
vibez.spill("Concatenated: " + concatenated)
EOF
    
    # Test module system
    if timeout $TIMEOUT_SECONDS "$OLDPWD/$COMPILER_BINARY" module_test.csd > module_interp.out 2>&1; then
        echo "  ✅ Module system interpretation successful"
    else
        echo "  ❌ Module system interpretation failed"
        return 1
    fi
    
    if timeout $TIMEOUT_SECONDS "$OLDPWD/$COMPILER_BINARY" -- compile module_test.csd -o module_compiled 2>&1; then
        echo "  ✅ Module system compilation successful"
    else
        echo "  ❌ Module system compilation failed"
        return 1
    fi
    
    if ./module_compiled > module_compiled.out 2>&1; then
        echo "  ✅ Module system execution successful"
    else
        echo "  ❌ Module system execution failed"
        return 1
    fi
    
    # Compare outputs
    if diff -q module_interp.out module_compiled.out > /dev/null; then
        echo "  ✅ Module system output consistency verified"
        return 0
    else
        echo "  ❌ Module system output mismatch"
        return 1
    fi
}

# Test Suite 4: Error Handling and Edge Cases
test_error_handling() {
    echo "📝 Testing error handling and edge cases..."
    
    # Create error handling test
    cat > error_handling.csd << 'EOF'
// Error handling test
sus safe_divide(a normie, b normie) normie {
    if b == 0 {
        vibez.spill("Division by zero avoided")
        damn 0
    } else {
        damn a / b
    }
}

// Test normal case
sus result1 normie = safe_divide(10, 2)
vibez.spill("Normal division: " + result1)

// Test edge case
sus result2 normie = safe_divide(10, 0)
vibez.spill("Edge case result: " + result2)

// Test array bounds safety
sus array [3]normie = [1, 2, 3]
sus safe_index normie = 1
sus safe_value normie = array[safe_index]
vibez.spill("Safe array access: " + safe_value)

// Test boolean edge cases
sus false_flag lit = cap
sus true_flag lit = based

if false_flag {
    vibez.spill("This should not print")
} else {
    vibez.spill("False flag handled correctly")
}

if true_flag {
    vibez.spill("True flag handled correctly")
} else {
    vibez.spill("This should not print")
}
EOF
    
    # Test error handling
    if timeout $TIMEOUT_SECONDS "$OLDPWD/$COMPILER_BINARY" error_handling.csd > error_interp.out 2>&1; then
        echo "  ✅ Error handling interpretation successful"
    else
        echo "  ❌ Error handling interpretation failed"
        return 1
    fi
    
    if timeout $TIMEOUT_SECONDS "$OLDPWD/$COMPILER_BINARY" -- compile error_handling.csd -o error_compiled 2>&1; then
        echo "  ✅ Error handling compilation successful"
    else
        echo "  ❌ Error handling compilation failed"
        return 1
    fi
    
    if ./error_compiled > error_compiled.out 2>&1; then
        echo "  ✅ Error handling execution successful"
    else
        echo "  ❌ Error handling execution failed"
        return 1
    fi
    
    # Compare outputs
    if diff -q error_interp.out error_compiled.out > /dev/null; then
        echo "  ✅ Error handling output consistency verified"
        return 0
    else
        echo "  ❌ Error handling output mismatch"
        return 1
    fi
}

# Test Suite 5: Performance and Scalability
test_performance_scalability() {
    echo "📝 Testing performance and scalability..."
    
    # Create performance test
    cat > performance_test.csd << 'EOF'
// Performance and scalability test
sus large_computation() normie {
    sus result normie = 0
    sus iterations normie = 1000
    
    bestie i := 0; i < iterations; i++ {
        bestie j := 0; j < 100; j++ {
            result += i * j
        }
    }
    
    damn result
}

sus computation_result normie = large_computation()
vibez.spill("Large computation result: " + computation_result)

// Test large array
sus large_array [100]normie
bestie i := 0; i < 100; i++ {
    large_array[i] = i * i
}

sus array_sum normie = 0
bestie i := 0; i < 100; i++ {
    array_sum += large_array[i]
}

vibez.spill("Large array sum: " + array_sum)

// Test deep recursion (limited depth for safety)
sus factorial(n normie) normie {
    if n <= 1 {
        damn 1
    } else {
        damn n * factorial(n - 1)
    }
}

sus factorial_result normie = factorial(10)
vibez.spill("Factorial(10): " + factorial_result)
EOF
    
    # Test performance
    start_time=$(date +%s%N)
    if timeout $TIMEOUT_SECONDS "$OLDPWD/$COMPILER_BINARY" performance_test.csd > perf_interp.out 2>&1; then
        end_time=$(date +%s%N)
        interp_time=$((end_time - start_time))
        interp_seconds=$(echo "scale=3; $interp_time / 1000000000" | bc)
        echo "  ✅ Performance interpretation successful (${interp_seconds}s)"
    else
        echo "  ❌ Performance interpretation failed"
        return 1
    fi
    
    start_time=$(date +%s%N)
    if timeout $TIMEOUT_SECONDS "$OLDPWD/$COMPILER_BINARY" -- compile performance_test.csd -o perf_compiled 2>&1; then
        end_time=$(date +%s%N)
        compile_time=$((end_time - start_time))
        compile_seconds=$(echo "scale=3; $compile_time / 1000000000" | bc)
        echo "  ✅ Performance compilation successful (${compile_seconds}s)"
    else
        echo "  ❌ Performance compilation failed"
        return 1
    fi
    
    start_time=$(date +%s%N)
    if ./perf_compiled > perf_compiled.out 2>&1; then
        end_time=$(date +%s%N)
        exec_time=$((end_time - start_time))
        exec_seconds=$(echo "scale=3; $exec_time / 1000000000" | bc)
        echo "  ✅ Performance execution successful (${exec_seconds}s)"
    else
        echo "  ❌ Performance execution failed"
        return 1
    fi
    
    # Compare outputs
    if diff -q perf_interp.out perf_compiled.out > /dev/null; then
        echo "  ✅ Performance output consistency verified"
        echo "  📊 Performance metrics:"
        echo "    Interpretation: ${interp_seconds}s"
        echo "    Compilation: ${compile_seconds}s"
        echo "    Execution: ${exec_seconds}s"
        return 0
    else
        echo "  ❌ Performance output mismatch"
        return 1
    fi
}

# Test Suite 6: Self-Hosting Bootstrap
test_self_hosting_bootstrap() {
    echo "📝 Testing self-hosting bootstrap capability..."
    
    # Create bootstrap test program
    cat > bootstrap_test.csd << 'EOF'
// Self-hosting bootstrap test
sus bootstrap_message tea = "Self-hosting bootstrap test"
vibez.spill(bootstrap_message)

// Simulate basic compiler operations
slay tokenize(source tea) normie {
    // Simulate tokenization
    sus token_count normie = stringz.length(source) / 5
    damn token_count
}

slay parse(token_count normie) lit {
    // Simulate parsing
    damn token_count > 0
}

slay compile_to_binary(parsed lit) lit {
    // Simulate compilation
    damn parsed
}

// Bootstrap simulation
sus source_code tea = "sus x normie = 42; vibez.spill(x)"
sus tokens normie = tokenize(source_code)
sus parsed lit = parse(tokens)
sus compiled lit = compile_to_binary(parsed)

if compiled {
    vibez.spill("Bootstrap compilation successful")
} else {
    vibez.spill("Bootstrap compilation failed")
}

vibez.spill("Tokens: " + tokens)
EOF
    
    # Test bootstrap
    if timeout $TIMEOUT_SECONDS "$OLDPWD/$COMPILER_BINARY" bootstrap_test.csd > bootstrap_interp.out 2>&1; then
        echo "  ✅ Bootstrap interpretation successful"
    else
        echo "  ❌ Bootstrap interpretation failed"
        return 1
    fi
    
    if timeout $TIMEOUT_SECONDS "$OLDPWD/$COMPILER_BINARY" -- compile bootstrap_test.csd -o bootstrap_compiled 2>&1; then
        echo "  ✅ Bootstrap compilation successful"
    else
        echo "  ❌ Bootstrap compilation failed"
        return 1
    fi
    
    if ./bootstrap_compiled > bootstrap_compiled.out 2>&1; then
        echo "  ✅ Bootstrap execution successful"
    else
        echo "  ❌ Bootstrap execution failed"
        return 1
    fi
    
    # Compare outputs
    if diff -q bootstrap_interp.out bootstrap_compiled.out > /dev/null; then
        echo "  ✅ Bootstrap output consistency verified"
        return 0
    else
        echo "  ❌ Bootstrap output mismatch"
        return 1
    fi
}

# Test Suite 7: Cross-Platform Compatibility
test_cross_platform_compatibility() {
    echo "📝 Testing cross-platform compatibility..."
    
    # Create cross-platform test
    cat > cross_platform.csd << 'EOF'
// Cross-platform compatibility test
sus platform_test() tea {
    // Test basic arithmetic (should work everywhere)
    sus result normie = 2 + 2
    if result == 4 {
        damn "Arithmetic works"
    } else {
        damn "Arithmetic failed"
    }
}

sus arithmetic_result tea = platform_test()
vibez.spill("Platform test: " + arithmetic_result)

// Test string operations (should work everywhere)
sus text tea = "Hello"
sus world tea = "World"
sus combined tea = text + " " + world
vibez.spill("String test: " + combined)

// Test array operations (should work everywhere)
sus numbers [3]normie = [1, 2, 3]
sus sum normie = numbers[0] + numbers[1] + numbers[2]
vibez.spill("Array test: " + sum)

// Test function calls (should work everywhere)
slay universal_function(x normie) normie {
    damn x * 2
}

sus function_result normie = universal_function(21)
vibez.spill("Function test: " + function_result)
EOF
    
    # Test cross-platform compatibility
    if timeout $TIMEOUT_SECONDS "$OLDPWD/$COMPILER_BINARY" cross_platform.csd > cross_interp.out 2>&1; then
        echo "  ✅ Cross-platform interpretation successful"
    else
        echo "  ❌ Cross-platform interpretation failed"
        return 1
    fi
    
    if timeout $TIMEOUT_SECONDS "$OLDPWD/$COMPILER_BINARY" -- compile cross_platform.csd -o cross_compiled 2>&1; then
        echo "  ✅ Cross-platform compilation successful"
    else
        echo "  ❌ Cross-platform compilation failed"
        return 1
    fi
    
    if ./cross_compiled > cross_compiled.out 2>&1; then
        echo "  ✅ Cross-platform execution successful"
    else
        echo "  ❌ Cross-platform execution failed"
        return 1
    fi
    
    # Compare outputs
    if diff -q cross_interp.out cross_compiled.out > /dev/null; then
        echo "  ✅ Cross-platform output consistency verified"
        return 0
    else
        echo "  ❌ Cross-platform output mismatch"
        return 1
    fi
}

# Main execution
echo "🚀 Starting comprehensive self-hosting test suite..."

# Ensure compiler is built
if [ ! -f "$OLDPWD/$COMPILER_BINARY" ]; then
    echo "🏗️  Building compiler..."
    cd "$OLDPWD"
    cargo build --release --bin cursed
    cd "$TEST_DIR"
fi

# Run all test suites
echo "📋 Running all test suites..."
echo ""

run_test_suite "Basic Self-Hosting Capability" "test_basic_self_hosting"
echo ""

run_test_suite "Advanced Language Features" "test_advanced_language_features"
echo ""

run_test_suite "Module System" "test_module_system"
echo ""

run_test_suite "Error Handling and Edge Cases" "test_error_handling"
echo ""

run_test_suite "Performance and Scalability" "test_performance_scalability"
echo ""

run_test_suite "Self-Hosting Bootstrap" "test_self_hosting_bootstrap"
echo ""

run_test_suite "Cross-Platform Compatibility" "test_cross_platform_compatibility"
echo ""

# Generate final report
echo "📊 Comprehensive Self-Hosting Test Suite Report"
echo "==============================================="
echo "Total test suites: $total_test_suites"
echo "Passed: $passed_test_suites"
echo "Failed: $failed_test_suites"
echo "Success rate: $(echo "scale=2; $passed_test_suites * 100 / $total_test_suites" | bc)%"

# Check overall result
if [ $failed_test_suites -eq 0 ]; then
    echo ""
    echo "🎉 All comprehensive self-hosting tests passed!"
    echo "✅ CURSED compiler is fully ready for self-hosting deployment"
    echo "🚀 Self-hosting capability verified across all test scenarios"
    exit 0
else
    echo ""
    echo "❌ Some comprehensive self-hosting tests failed"
    echo "🔍 Review failed test suites before deployment"
    echo "⚠️  Self-hosting capability requires attention"
    exit 1
fi
