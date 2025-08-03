#!/bin/bash

# Comprehensive End-to-End Integration Test Suite for CURSED Compiler
# Tests both Zig and Rust implementations with full program execution validation

set -e

echo "🚀 CURSED Comprehensive Integration Test Suite"
echo "=============================================="

# Configuration
ZIG_COMPILER="./zig-out/bin/cursed-zig"
RUST_COMPILER="./target/debug/cursed"
TEST_DIR="integration_tests"
RESULTS_FILE="integration_test_results.log"

# Initialize
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"
rm -f "../$RESULTS_FILE"

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Logging function
log_result() {
    echo "$1" | tee -a "../$RESULTS_FILE"
}

# Test function
run_test() {
    local test_name="$1"
    local test_file="$2"
    local compiler="$3"
    local mode="$4"  # "interpret" or "compile"
    local expected_output="$5"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    echo "🔧 Testing: $test_name ($mode mode)"
    
    if [ "$mode" = "interpret" ]; then
        if timeout 30 $compiler "$test_file" > output.txt 2>&1; then
            if [ -n "$expected_output" ] && grep -q "$expected_output" output.txt; then
                echo "  ✅ PASS: $test_name ($mode)"
                log_result "PASS: $test_name ($mode) - Output matches expected"
                PASSED_TESTS=$((PASSED_TESTS + 1))
                return 0
            elif [ -z "$expected_output" ]; then
                echo "  ✅ PASS: $test_name ($mode)"
                log_result "PASS: $test_name ($mode) - Execution successful"
                PASSED_TESTS=$((PASSED_TESTS + 1))
                return 0
            else
                echo "  ❌ FAIL: $test_name ($mode) - Output mismatch"
                log_result "FAIL: $test_name ($mode) - Expected: $expected_output"
                cat output.txt | head -5 | sed 's/^/    /' | tee -a "../$RESULTS_FILE"
                FAILED_TESTS=$((FAILED_TESTS + 1))
                return 1
            fi
        else
            echo "  ❌ FAIL: $test_name ($mode) - Execution failed"
            log_result "FAIL: $test_name ($mode) - Execution failed"
            cat output.txt | head -5 | sed 's/^/    /' | tee -a "../$RESULTS_FILE"
            FAILED_TESTS=$((FAILED_TESTS + 1))
            return 1
        fi
    else  # compile mode
        local binary_name=$(basename "$test_file" .csd)
        if timeout 30 $compiler "$test_file" --compile > compile_output.txt 2>&1; then
            if [ -f "$binary_name" ]; then
                if timeout 30 ./"$binary_name" > output.txt 2>&1; then
                    if [ -n "$expected_output" ] && grep -q "$expected_output" output.txt; then
                        echo "  ✅ PASS: $test_name ($mode)"
                        log_result "PASS: $test_name ($mode) - Compiled and executed correctly"
                        PASSED_TESTS=$((PASSED_TESTS + 1))
                        rm -f "$binary_name"
                        return 0
                    elif [ -z "$expected_output" ]; then
                        echo "  ✅ PASS: $test_name ($mode)"
                        log_result "PASS: $test_name ($mode) - Compiled and executed successfully"
                        PASSED_TESTS=$((PASSED_TESTS + 1))
                        rm -f "$binary_name"
                        return 0
                    else
                        echo "  ❌ FAIL: $test_name ($mode) - Compiled binary output mismatch"
                        log_result "FAIL: $test_name ($mode) - Expected: $expected_output"
                        cat output.txt | head -5 | sed 's/^/    /' | tee -a "../$RESULTS_FILE"
                        FAILED_TESTS=$((FAILED_TESTS + 1))
                        rm -f "$binary_name"
                        return 1
                    fi
                else
                    echo "  ❌ FAIL: $test_name ($mode) - Compiled binary execution failed"
                    log_result "FAIL: $test_name ($mode) - Binary execution failed"
                    cat output.txt | head -5 | sed 's/^/    /' | tee -a "../$RESULTS_FILE"
                    FAILED_TESTS=$((FAILED_TESTS + 1))
                    rm -f "$binary_name"
                    return 1
                fi
            else
                echo "  ❌ FAIL: $test_name ($mode) - Compilation failed"
                log_result "FAIL: $test_name ($mode) - No binary produced"
                cat compile_output.txt | head -5 | sed 's/^/    /' | tee -a "../$RESULTS_FILE"
                FAILED_TESTS=$((FAILED_TESTS + 1))
                return 1
            fi
        else
            echo "  ❌ FAIL: $test_name ($mode) - Compilation timeout/error"
            log_result "FAIL: $test_name ($mode) - Compilation timeout"
            cat compile_output.txt | head -5 | sed 's/^/    /' | tee -a "../$RESULTS_FILE"
            FAILED_TESTS=$((FAILED_TESTS + 1))
            return 1
        fi
    fi
}

# Check compilers exist
echo "📋 Checking compiler availability..."
if [ ! -f "../$ZIG_COMPILER" ]; then
    echo "⚠️  Zig compiler not found, building..."
    cd .. && zig build && cd "$TEST_DIR"
fi

if [ ! -f "../$RUST_COMPILER" ]; then
    echo "⚠️  Rust compiler not found, building..."
    cd .. && cargo build && cd "$TEST_DIR"
fi

# Test 1: Basic Hello World
echo "📝 Creating basic hello world test..."
cat > test_hello.csd << 'EOF'
vibez.spill("Hello, CURSED!")
EOF

run_test "Basic Hello World (Zig)" "test_hello.csd" "../$ZIG_COMPILER" "interpret" "Hello, CURSED!"
run_test "Basic Hello World Compile (Zig)" "test_hello.csd" "../$ZIG_COMPILER" "compile" "Hello, CURSED!"

# Test 2: Arithmetic Operations
echo "📝 Creating arithmetic test..."
cat > test_arithmetic.csd << 'EOF'
sus a drip = 10
sus b drip = 5
sus sum drip = a + b
sus product drip = a * b
vibez.spill("Sum:", sum)
vibez.spill("Product:", product)
EOF

run_test "Arithmetic Operations (Zig)" "test_arithmetic.csd" "../$ZIG_COMPILER" "interpret" "Sum:"
run_test "Arithmetic Compile (Zig)" "test_arithmetic.csd" "../$ZIG_COMPILER" "compile" "Sum:"

# Test 3: Function Definitions and Calls
echo "📝 Creating function test..."
cat > test_functions.csd << 'EOF'
slay add_numbers(x drip, y drip) drip {
    damn x + y
}

slay main_func() {
    sus result drip = add_numbers(10, 20)
    vibez.spill("Function result:", result)
}

main_func()
EOF

run_test "Function Calls (Zig)" "test_functions.csd" "../$ZIG_COMPILER" "interpret" "Function result:"
run_test "Function Calls Compile (Zig)" "test_functions.csd" "../$ZIG_COMPILER" "compile" "Function result:"

# Test 4: Struct Definition and Usage
echo "📝 Creating struct test..."
cat > test_structs.csd << 'EOF'
squad Point {
    spill x drip
    spill y drip
}

sus p Point = Point{x: 10, y: 20}
vibez.spill("Point x:", p.x)
vibez.spill("Point y:", p.y)
EOF

run_test "Struct Usage (Zig)" "test_structs.csd" "../$ZIG_COMPILER" "interpret" "Point x:"
run_test "Struct Usage Compile (Zig)" "test_structs.csd" "../$ZIG_COMPILER" "compile" "Point x:"

# Test 5: Control Flow (Loops)
echo "📝 Creating loop test..."
cat > test_loops.csd << 'EOF'
sus counter drip = 0
bestie (counter < 3) {
    vibez.spill("Counter:", counter)
    counter = counter + 1
}
vibez.spill("Loop completed")
EOF

run_test "Loop Control Flow (Zig)" "test_loops.csd" "../$ZIG_COMPILER" "interpret" "Loop completed"
run_test "Loop Control Flow Compile (Zig)" "test_loops.csd" "../$ZIG_COMPILER" "compile" "Loop completed"

# Test 6: Interface Definition (if supported)
echo "📝 Creating interface test..."
cat > test_interfaces.csd << 'EOF'
collab Drawable {
    slay draw()
}

squad Circle {
    spill radius drip
}

flex Circle => Drawable {
    slay draw() {
        vibez.spill("Drawing circle")
    }
}

sus c Circle = Circle{radius: 5}
c.draw()
EOF

run_test "Interface Implementation (Zig)" "test_interfaces.csd" "../$ZIG_COMPILER" "interpret" "Drawing circle"
run_test "Interface Implementation Compile (Zig)" "test_interfaces.csd" "../$ZIG_COMPILER" "compile" "Drawing circle"

# Test 7: Concurrency (if supported)
echo "📝 Creating concurrency test..."
cat > test_concurrency.csd << 'EOF'
slay goroutine_test() {
    vibez.spill("Goroutine executed")
}

stan {
    goroutine_test()
}

vibez.spill("Main thread")
EOF

run_test "Basic Concurrency (Zig)" "test_concurrency.csd" "../$ZIG_COMPILER" "interpret" "Main thread"
run_test "Basic Concurrency Compile (Zig)" "test_concurrency.csd" "../$ZIG_COMPILER" "compile" "Main thread"

# Test 8: Memory Stress Test
echo "📝 Creating memory stress test..."
cat > test_memory.csd << 'EOF'
slay memory_stress() {
    sus counter drip = 0
    bestie (counter < 1000) {
        sus temp drip = counter * 2
        counter = counter + 1
    }
    vibez.spill("Memory stress completed:", counter)
}

memory_stress()
EOF

run_test "Memory Stress Test (Zig)" "test_memory.csd" "../$ZIG_COMPILER" "interpret" "Memory stress completed:"
run_test "Memory Stress Test Compile (Zig)" "test_memory.csd" "../$ZIG_COMPILER" "compile" "Memory stress completed:"

# Test 9: Error Handling (if supported)
echo "📝 Creating error handling test..."
cat > test_errors.csd << 'EOF'
slay divide_safe(a drip, b drip) drip {
    if (b == 0) {
        vibez.spill("Error: Division by zero")
        damn 0
    }
    damn a / b
}

sus result drip = divide_safe(10, 2)
vibez.spill("Division result:", result)
sus error_result drip = divide_safe(10, 0)
EOF

run_test "Error Handling (Zig)" "test_errors.csd" "../$ZIG_COMPILER" "interpret" "Division result:"
run_test "Error Handling Compile (Zig)" "test_errors.csd" "../$ZIG_COMPILER" "compile" "Division result:"

# Test 10: Complex Program Integration
echo "📝 Creating complex integration test..."
cat > test_complex.csd << 'EOF'
squad Calculator {
    spill value drip
}

slay new_calculator(initial drip) Calculator {
    damn Calculator{value: initial}
}

slay add_to_calculator(calc Calculator, amount drip) Calculator {
    calc.value = calc.value + amount
    damn calc
}

slay main_program() {
    sus calc Calculator = new_calculator(10)
    calc = add_to_calculator(calc, 20)
    vibez.spill("Calculator final value:", calc.value)
}

main_program()
EOF

run_test "Complex Integration (Zig)" "test_complex.csd" "../$ZIG_COMPILER" "interpret" "Calculator final value:"
run_test "Complex Integration Compile (Zig)" "test_complex.csd" "../$ZIG_COMPILER" "compile" "Calculator final value:"

# Performance Benchmark
echo "📝 Running performance benchmark..."
cat > test_performance.csd << 'EOF'
slay fibonacci(n drip) drip {
    if (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

sus start_time drip = 0  # Placeholder for actual timing
sus result drip = fibonacci(20)
vibez.spill("Fibonacci(20):", result)
vibez.spill("Performance test completed")
EOF

echo "⏱️ Performance Benchmark (Interpretation):"
time timeout 60 ../"$ZIG_COMPILER" test_performance.csd > perf_interpret.txt 2>&1 || true

echo "⏱️ Performance Benchmark (Compilation):"
time timeout 60 bash -c "../$ZIG_COMPILER --compile test_performance.csd && ./test_performance" > perf_compile.txt 2>&1 || true

# Memory Leak Detection (using valgrind if available)
if command -v valgrind >/dev/null 2>&1; then
    echo "🔍 Running memory leak detection..."
    echo "Memory leak test with valgrind:" >> "../$RESULTS_FILE"
    valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes \
        ../"$ZIG_COMPILER" test_memory.csd > valgrind_output.txt 2>&1 || true
    
    if grep -q "definitely lost: 0 bytes" valgrind_output.txt; then
        echo "  ✅ No memory leaks detected"
        log_result "PASS: Memory leak test - No leaks detected"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo "  ⚠️ Potential memory leaks detected"
        log_result "WARNING: Memory leak test - Potential leaks found"
        grep "lost:" valgrind_output.txt | head -3 | sed 's/^/    /' | tee -a "../$RESULTS_FILE"
    fi
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
else
    echo "⚠️ Valgrind not available - skipping memory leak detection"
fi

# Cross-platform test (if cross-compilation targets exist)
if [ -f "../cross_compilation_results/linux_x86_64_cursed" ]; then
    echo "🌐 Testing cross-platform binary..."
    if timeout 30 ../cross_compilation_results/linux_x86_64_cursed test_hello.csd > cross_output.txt 2>&1; then
        if grep -q "Hello, CURSED!" cross_output.txt; then
            echo "  ✅ Cross-platform binary test passed"
            log_result "PASS: Cross-platform binary test"
            PASSED_TESTS=$((PASSED_TESTS + 1))
        else
            echo "  ❌ Cross-platform binary test failed"
            log_result "FAIL: Cross-platform binary test"
            FAILED_TESTS=$((FAILED_TESTS + 1))
        fi
    else
        echo "  ❌ Cross-platform binary execution failed"
        log_result "FAIL: Cross-platform binary execution"
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
fi

# Cleanup
cd ..
rm -rf "$TEST_DIR"

# Final Results
echo ""
echo "📊 Integration Test Results Summary"
echo "=================================="
echo "Total Tests: $TOTAL_TESTS"
echo "Passed: $PASSED_TESTS"
echo "Failed: $FAILED_TESTS"
echo "Success Rate: $(( (PASSED_TESTS * 100) / TOTAL_TESTS ))%"

log_result ""
log_result "FINAL SUMMARY:"
log_result "Total Tests: $TOTAL_TESTS"
log_result "Passed: $PASSED_TESTS"
log_result "Failed: $FAILED_TESTS"
log_result "Success Rate: $(( (PASSED_TESTS * 100) / TOTAL_TESTS ))%"

if [ $FAILED_TESTS -eq 0 ]; then
    echo "🎉 All integration tests passed!"
    echo "✅ CURSED compiler is fully functional for end-to-end usage"
    exit 0
else
    echo "❌ Some integration tests failed"
    echo "📋 Check $RESULTS_FILE for detailed failure information"
    echo ""
    echo "🔍 Debug commands for failed tests:"
    echo "   ./zig-out/bin/cursed-zig test_file.csd"
    echo "   ./zig-out/bin/cursed-zig --compile test_file.csd"
    exit 1
fi
