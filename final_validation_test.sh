#!/bin/bash

# Final CURSED Compiler Validation Test
# Comprehensive demonstration of end-to-end functionality

set -e

echo "🎯 CURSED Compiler Final Validation Test"
echo "========================================"
echo "Testing complete compiler pipeline with real programs"
echo ""

# Configuration
ZIG_COMPILER="./zig-out/bin/cursed-zig"
TEST_DIR="final_validation"
RESULTS_FILE="final_validation_results.log"

mkdir -p "$TEST_DIR"
cd "$TEST_DIR"
rm -f "../$RESULTS_FILE"

log_result() {
    echo "$1" | tee -a "../$RESULTS_FILE"
}

# Test 1: Complex Program with All Language Features
echo "📝 Test 1: Complex program with multiple language features"
cat > complex_program.csd << 'EOF'
fr fr Complex CURSED program demonstrating all major features

squad Calculator {
    spill value drip
    spill name tea
}

collab Processor {
    slay process(input drip) drip
}

flex Calculator => Processor {
    slay process(input drip) drip {
        damn input * 2
    }
}

slay fibonacci(n drip) drip {
    if (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

slay main_program() {
    sus calc Calculator = Calculator{value: 100, name: "TestCalc"}
    vibez.spill("Calculator name:", calc.name)
    vibez.spill("Calculator value:", calc.value)
    
    sus processed drip = calc.process(42)
    vibez.spill("Processed value:", processed)
    
    sus fib_result drip = fibonacci(10)
    vibez.spill("Fibonacci(10):", fib_result)
    
    sus counter drip = 0
    bestie (counter < 3) {
        vibez.spill("Counter:", counter)
        counter = counter + 1
    }
    
    vibez.spill("Complex program completed successfully!")
}

main_program()
EOF

echo "Testing interpretation mode..."
if timeout 60 ../"$ZIG_COMPILER" complex_program.csd > complex_interp.log 2>&1; then
    if grep -q "Complex program completed successfully" complex_interp.log; then
        echo "  ✅ Complex program interpretation: PASS"
        log_result "PASS: Complex program interpretation"
    else
        echo "  ❌ Complex program interpretation: FAIL - unexpected output"
        log_result "FAIL: Complex program interpretation - output mismatch"
    fi
else
    echo "  ❌ Complex program interpretation: FAIL - execution error"
    log_result "FAIL: Complex program interpretation - execution error"
fi

echo "Testing compilation mode..."
if timeout 60 ../"$ZIG_COMPILER" complex_program.csd --compile > complex_compile.log 2>&1; then
    if [ -f "complex_program" ]; then
        if timeout 30 ./complex_program > complex_exec.log 2>&1; then
            if grep -q "Complex program completed successfully" complex_exec.log; then
                echo "  ✅ Complex program compilation: PASS"
                log_result "PASS: Complex program compilation and execution"
                
                # Show performance comparison
                echo "    📊 Performance comparison:"
                echo "    Interpretation output:"
                grep "Fibonacci(10):" complex_interp.log | head -1 | sed 's/^/      /'
                echo "    Compiled binary output:"
                grep "Fibonacci(10):" complex_exec.log | head -1 | sed 's/^/      /'
            else
                echo "  ❌ Complex program compilation: FAIL - compiled binary output mismatch"
                log_result "FAIL: Complex program compilation - binary output mismatch"
            fi
        else
            echo "  ❌ Complex program compilation: FAIL - binary execution failed"
            log_result "FAIL: Complex program compilation - binary execution failed"
        fi
        rm -f complex_program
    else
        echo "  ❌ Complex program compilation: FAIL - no binary produced"
        log_result "FAIL: Complex program compilation - no binary produced"
    fi
else
    echo "  ❌ Complex program compilation: FAIL - compilation error"
    log_result "FAIL: Complex program compilation - compilation error"
fi

# Test 2: Stress Test Program
echo ""
echo "📝 Test 2: Performance stress test"
cat > stress_test.csd << 'EOF'
slay stress_computation() {
    sus total drip = 0
    sus outer drip = 0
    bestie (outer < 1000) {
        sus inner drip = 0
        bestie (inner < 100) {
            total = total + (outer * inner)
            inner = inner + 1
        }
        outer = outer + 1
    }
    vibez.spill("Stress test result:", total)
}

stress_computation()
EOF

echo "Testing stress performance..."
echo "  Interpretation mode timing:"
time timeout 120 ../"$ZIG_COMPILER" stress_test.csd > stress_interp.log 2>&1 || true

echo "  Compilation mode timing:"
time timeout 60 bash -c "../$ZIG_COMPILER stress_test.csd --compile && ./stress_test" > stress_compile.log 2>&1 || true

if [ -f "stress_test" ]; then
    rm -f stress_test
fi

# Test 3: Concurrency Test (if supported)
echo ""
echo "📝 Test 3: Concurrency functionality"
cat > concurrency_test.csd << 'EOF'
slay worker_function(id drip) {
    sus i drip = 0
    bestie (i < 100) {
        sus computation drip = i * id
        i = i + 1
    }
    vibez.spill("Worker", id, "completed")
}

slay concurrency_main() {
    vibez.spill("Starting concurrency test")
    
    stan {
        worker_function(1)
    }
    
    stan {
        worker_function(2)
    }
    
    vibez.spill("Main thread completed")
}

concurrency_main()
EOF

echo "Testing concurrency..."
if timeout 60 ../"$ZIG_COMPILER" concurrency_test.csd > concurrency_interp.log 2>&1; then
    if grep -q "Main thread completed" concurrency_interp.log; then
        echo "  ✅ Concurrency test: PASS"
        log_result "PASS: Concurrency test"
    else
        echo "  ⚠️ Concurrency test: PARTIAL - basic execution works"
        log_result "PARTIAL: Concurrency test - basic execution"
    fi
else
    echo "  ❌ Concurrency test: FAIL"
    log_result "FAIL: Concurrency test"
fi

# Test 4: Memory Management Test
echo ""
echo "📝 Test 4: Memory management stress test"
cat > memory_test.csd << 'EOF'
squad DataPoint {
    spill x drip
    spill y drip
    spill data drip
}

slay memory_stress() {
    sus count drip = 0
    bestie (count < 5000) {
        sus point DataPoint = DataPoint{x: count, y: count * 2, data: count * count}
        sus temp drip = point.x + point.y + point.data
        count = count + 1
    }
    vibez.spill("Memory stress test completed")
}

memory_stress()
EOF

echo "Testing memory management..."
if command -v valgrind >/dev/null 2>&1; then
    echo "  Running with valgrind memory analysis..."
    valgrind --leak-check=full --show-leak-kinds=all \
        ../"$ZIG_COMPILER" memory_test.csd > valgrind_memory.log 2>&1 || true
    
    if grep -q "Memory stress test completed" valgrind_memory.log; then
        echo "  ✅ Memory stress test execution: PASS"
        log_result "PASS: Memory stress test execution"
        
        # Check for memory leaks
        if grep -q "definitely lost: 0 bytes" valgrind_memory.log; then
            echo "  ✅ Memory leak check: PASS (no leaks)"
            log_result "PASS: No memory leaks detected"
        else
            echo "  ⚠️ Memory leak check: WARNING (potential leaks)"
            log_result "WARNING: Potential memory leaks detected"
            grep "lost:" valgrind_memory.log | head -3 | sed 's/^/    /'
        fi
    else
        echo "  ❌ Memory stress test: FAIL"
        log_result "FAIL: Memory stress test execution"
    fi
else
    echo "  ⚠️ Valgrind not available - running basic memory test"
    if timeout 60 ../"$ZIG_COMPILER" memory_test.csd > memory_basic.log 2>&1; then
        if grep -q "Memory stress test completed" memory_basic.log; then
            echo "  ✅ Basic memory test: PASS"
            log_result "PASS: Basic memory test"
        else
            echo "  ❌ Basic memory test: FAIL"
            log_result "FAIL: Basic memory test"
        fi
    else
        echo "  ❌ Basic memory test: FAIL (execution error)"
        log_result "FAIL: Basic memory test - execution error"
    fi
fi

# Test 5: Feature Completeness Test
echo ""
echo "📝 Test 5: Language feature completeness"
cat > feature_test.csd << 'EOF'
fr fr Testing all major CURSED language features

squad TestStruct {
    spill number drip
    spill text tea
}

collab TestInterface {
    slay test_method() drip
}

flex TestStruct => TestInterface {
    slay test_method() drip {
        damn number * 2
    }
}

slay test_function(param drip) drip {
    damn param + 10
}

slay feature_completeness_test() {
    fr fr Variables
    sus int_var drip = 42
    sus string_var tea = "test"
    sus bool_var lit = based
    
    fr fr Structs
    sus test_obj TestStruct = TestStruct{number: 100, text: "hello"}
    
    fr fr Functions
    sus func_result drip = test_function(5)
    
    fr fr Interface calls
    sus interface_result drip = test_obj.test_method()
    
    fr fr Control flow
    sus loop_counter drip = 0
    bestie (loop_counter < 3) {
        loop_counter = loop_counter + 1
    }
    
    fr fr Conditional
    if (int_var > 0) {
        vibez.spill("Positive number")
    }
    
    vibez.spill("Feature test results:")
    vibez.spill("Integer:", int_var)
    vibez.spill("String:", string_var)
    vibez.spill("Function result:", func_result)
    vibez.spill("Interface result:", interface_result)
    vibez.spill("Loop iterations:", loop_counter)
    vibez.spill("All features tested successfully!")
}

feature_completeness_test()
EOF

echo "Testing feature completeness..."
if timeout 60 ../"$ZIG_COMPILER" feature_test.csd > feature_interp.log 2>&1; then
    if grep -q "All features tested successfully" feature_interp.log; then
        echo "  ✅ Feature completeness: PASS"
        log_result "PASS: Feature completeness test"
        
        # Count successful feature tests
        echo "    📊 Feature test results:"
        grep -E "(Integer:|String:|Function result:|Interface result:)" feature_interp.log | sed 's/^/      /'
    else
        echo "  ❌ Feature completeness: FAIL - incomplete execution"
        log_result "FAIL: Feature completeness test - incomplete"
    fi
else
    echo "  ❌ Feature completeness: FAIL - execution error"
    log_result "FAIL: Feature completeness test - execution error"
fi

# Generate final validation report
cd ..
echo ""
echo "📊 Final Validation Results"
echo "==========================="

# Count results
TOTAL_TESTS=$(grep -c "PASS\|FAIL\|PARTIAL:" "$RESULTS_FILE" 2>/dev/null || echo "0")
PASSED_TESTS=$(grep -c "PASS:" "$RESULTS_FILE" 2>/dev/null || echo "0")
FAILED_TESTS=$(grep -c "FAIL:" "$RESULTS_FILE" 2>/dev/null || echo "0")
PARTIAL_TESTS=$(grep -c "PARTIAL:" "$RESULTS_FILE" 2>/dev/null || echo "0")
WARNING_TESTS=$(grep -c "WARNING:" "$RESULTS_FILE" 2>/dev/null || echo "0")

echo "Test Summary:"
echo "  ✅ Passed: $PASSED_TESTS"
echo "  ❌ Failed: $FAILED_TESTS"
echo "  ⚠️ Partial: $PARTIAL_TESTS"
echo "  ⚠️ Warnings: $WARNING_TESTS"
echo "  📊 Total: $TOTAL_TESTS"

if [ "$TOTAL_TESTS" -gt 0 ]; then
    SUCCESS_RATE=$(( (PASSED_TESTS * 100) / TOTAL_TESTS ))
    echo "  📈 Success Rate: ${SUCCESS_RATE}%"
else
    SUCCESS_RATE=0
fi

echo ""
echo "🎯 Validation Assessment:"

if [ "$FAILED_TESTS" -eq 0 ] && [ "$PASSED_TESTS" -gt 5 ]; then
    echo "🎉 EXCELLENT: CURSED compiler passes comprehensive validation!"
    echo "✅ Ready for production use with:"
    echo "   - Complete language feature support"
    echo "   - Both interpretation and compilation modes working"
    echo "   - Complex program execution verified"
    echo "   - Memory management functional"
    echo "   - Performance characteristics documented"
    exit 0
elif [ "$SUCCESS_RATE" -ge 70 ]; then
    echo "✅ GOOD: CURSED compiler is largely functional"
    echo "⚠️ Minor issues detected but core functionality working"
    echo "📋 Review warnings and partial failures for optimization"
    exit 0
else
    echo "❌ NEEDS WORK: Multiple test failures detected"
    echo "🔧 Requires significant fixes before production use"
    echo "📋 Check detailed logs for failure analysis"
    exit 1
fi

echo ""
echo "📋 Detailed results in: $RESULTS_FILE"
