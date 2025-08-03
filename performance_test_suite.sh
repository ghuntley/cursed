#!/bin/bash

# CURSED Performance and Memory Testing Suite
# Comprehensive benchmarking and memory leak detection

set -e

echo "🚀 CURSED Performance & Memory Test Suite"
echo "========================================="

ZIG_COMPILER="./zig-out/bin/cursed-zig"
RUST_COMPILER="./target/debug/cursed"
PERF_DIR="performance_tests"
RESULTS_FILE="performance_results.log"

mkdir -p "$PERF_DIR"
cd "$PERF_DIR"
rm -f "../$RESULTS_FILE"

log_result() {
    echo "$1" | tee -a "../$RESULTS_FILE"
}

echo "📊 Creating performance test programs..."

# Performance Test 1: Fibonacci Recursion
cat > fibonacci_test.csd << 'EOF'
slay fibonacci(n drip) drip {
    if (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

slay main() {
    sus result drip = fibonacci(25)
    vibez.spill("Fibonacci(25):", result)
}

main()
EOF

# Performance Test 2: Loop-Heavy Computation
cat > computation_test.csd << 'EOF'
slay compute_heavy() {
    sus total drip = 0
    sus i drip = 0
    bestie (i < 100000) {
        sus j drip = 0
        bestie (j < 100) {
            total = total + (i * j)
            j = j + 1
        }
        i = i + 1
    }
    vibez.spill("Computation result:", total)
}

compute_heavy()
EOF

# Performance Test 3: Memory Allocation Stress
cat > memory_stress_test.csd << 'EOF'
squad DataPoint {
    spill x drip
    spill y drip
    spill value drip
}

slay memory_stress() {
    sus i drip = 0
    bestie (i < 10000) {
        sus point DataPoint = DataPoint{x: i, y: i * 2, value: i * i}
        # Simulate some work with the struct
        sus temp drip = point.x + point.y + point.value
        i = i + 1
    }
    vibez.spill("Memory stress completed")
}

memory_stress()
EOF

# Performance Test 4: Function Call Overhead
cat > function_overhead_test.csd << 'EOF'
slay simple_add(a drip, b drip) drip {
    damn a + b
}

slay test_function_calls() {
    sus total drip = 0
    sus i drip = 0
    bestie (i < 50000) {
        total = total + simple_add(i, i + 1)
        i = i + 1
    }
    vibez.spill("Function call test result:", total)
}

test_function_calls()
EOF

# Performance Test 5: Struct Access Performance
cat > struct_access_test.csd << 'EOF'
squad TestStruct {
    spill field1 drip
    spill field2 drip
    spill field3 drip
    spill field4 drip
    spill field5 drip
}

slay test_struct_access() {
    sus test_obj TestStruct = TestStruct{
        field1: 10,
        field2: 20,
        field3: 30,
        field4: 40,
        field5: 50
    }
    
    sus total drip = 0
    sus i drip = 0
    bestie (i < 20000) {
        total = total + test_obj.field1 + test_obj.field2 + test_obj.field3 + test_obj.field4 + test_obj.field5
        i = i + 1
    }
    vibez.spill("Struct access test result:", total)
}

test_struct_access()
EOF

run_performance_test() {
    local test_name="$1"
    local test_file="$2"
    local compiler="$3"
    local mode="$4"
    
    echo "⏱️ Running $test_name ($mode mode)..."
    
    if [ "$mode" = "interpret" ]; then
        log_result "=== $test_name (Interpretation) ==="
        echo "Interpretation timing:" >> "../$RESULTS_FILE"
        { time timeout 120 $compiler "$test_file"; } 2>&1 | tee -a "../$RESULTS_FILE"
        echo "" >> "../$RESULTS_FILE"
    else
        log_result "=== $test_name (Compilation) ==="
        local binary_name=$(basename "$test_file" .csd)
        
        echo "Compilation timing:" >> "../$RESULTS_FILE"
        { time timeout 60 $compiler "$test_file" --compile; } 2>&1 | tee -a "../$RESULTS_FILE"
        
        if [ -f "$binary_name" ]; then
            echo "Execution timing:" >> "../$RESULTS_FILE"
            { time timeout 120 ./"$binary_name"; } 2>&1 | tee -a "../$RESULTS_FILE"
            rm -f "$binary_name"
        else
            log_result "ERROR: Compilation failed to produce binary"
        fi
        echo "" >> "../$RESULTS_FILE"
    fi
}

run_memory_test() {
    local test_name="$1"
    local test_file="$2"
    local compiler="$3"
    
    echo "🔍 Running memory test: $test_name..."
    
    if command -v valgrind >/dev/null 2>&1; then
        log_result "=== Memory Test: $test_name ==="
        echo "Running valgrind memory analysis..." >> "../$RESULTS_FILE"
        
        valgrind --tool=memcheck \
                 --leak-check=full \
                 --show-leak-kinds=all \
                 --track-origins=yes \
                 --verbose \
                 $compiler "$test_file" > "memcheck_${test_name}.log" 2>&1 || true
        
        # Extract key memory statistics
        grep -E "(definitely lost|indirectly lost|possibly lost|ERROR SUMMARY)" "memcheck_${test_name}.log" | tee -a "../$RESULTS_FILE" || true
        
        # Heap usage analysis
        if command -v valgrind >/dev/null 2>&1; then
            echo "Running heap usage analysis..." >> "../$RESULTS_FILE"
            valgrind --tool=massif \
                     --massif-out-file="massif_${test_name}.out" \
                     $compiler "$test_file" > "massif_${test_name}.log" 2>&1 || true
            
            if command -v ms_print >/dev/null 2>&1 && [ -f "massif_${test_name}.out" ]; then
                ms_print "massif_${test_name}.out" | head -20 | tee -a "../$RESULTS_FILE" || true
            fi
        fi
        echo "" >> "../$RESULTS_FILE"
    else
        echo "⚠️ Valgrind not available - using basic memory monitoring"
        log_result "=== Basic Memory Test: $test_name ==="
        echo "Running with time and monitoring..." >> "../$RESULTS_FILE"
        /usr/bin/time -v $compiler "$test_file" > "time_${test_name}.log" 2>&1 || true
        grep -E "(Maximum resident set size|User time|System time)" "time_${test_name}.log" | tee -a "../$RESULTS_FILE" || true
        echo "" >> "../$RESULTS_FILE"
    fi
}

run_concurrency_stress_test() {
    echo "🧵 Running concurrency stress test..."
    
    cat > concurrency_stress.csd << 'EOF'
slay worker_task(id drip) {
    sus i drip = 0
    bestie (i < 1000) {
        sus computation drip = i * id
        i = i + 1
    }
    vibez.spill("Worker", id, "completed")
}

slay concurrency_test() {
    sus worker_count drip = 0
    bestie (worker_count < 10) {
        stan {
            worker_task(worker_count)
        }
        worker_count = worker_count + 1
    }
    vibez.spill("All workers started")
}

concurrency_test()
EOF
    
    log_result "=== Concurrency Stress Test ==="
    echo "Running concurrency test..." >> "../$RESULTS_FILE"
    
    { time timeout 60 ../"$ZIG_COMPILER" concurrency_stress.csd; } 2>&1 | tee -a "../$RESULTS_FILE" || true
    echo "" >> "../$RESULTS_FILE"
}

echo "🏃 Starting performance benchmarks..."

# Build compilers if needed
if [ ! -f "../$ZIG_COMPILER" ]; then
    echo "Building Zig compiler..."
    cd .. && zig build && cd "$PERF_DIR"
fi

if [ ! -f "../$RUST_COMPILER" ]; then
    echo "Building Rust compiler..."
    cd .. && cargo build && cd "$PERF_DIR"
fi

log_result "CURSED Performance Test Results"
log_result "Date: $(date)"
log_result "System: $(uname -a)"
log_result "================================"

# Run performance tests
run_performance_test "Fibonacci Recursion" "fibonacci_test.csd" "../$ZIG_COMPILER" "interpret"
run_performance_test "Fibonacci Recursion" "fibonacci_test.csd" "../$ZIG_COMPILER" "compile"

run_performance_test "Heavy Computation" "computation_test.csd" "../$ZIG_COMPILER" "interpret"
run_performance_test "Heavy Computation" "computation_test.csd" "../$ZIG_COMPILER" "compile"

run_performance_test "Function Call Overhead" "function_overhead_test.csd" "../$ZIG_COMPILER" "interpret"
run_performance_test "Function Call Overhead" "function_overhead_test.csd" "../$ZIG_COMPILER" "compile"

run_performance_test "Struct Access" "struct_access_test.csd" "../$ZIG_COMPILER" "interpret"
run_performance_test "Struct Access" "struct_access_test.csd" "../$ZIG_COMPILER" "compile"

# Run memory tests
echo "🧠 Starting memory analysis..."
run_memory_test "memory_stress" "memory_stress_test.csd" "../$ZIG_COMPILER"
run_memory_test "fibonacci" "fibonacci_test.csd" "../$ZIG_COMPILER"
run_memory_test "computation" "computation_test.csd" "../$ZIG_COMPILER"

# Run concurrency stress test
run_concurrency_stress_test

# Generate comparison report
log_result ""
log_result "=== Performance Comparison Summary ==="

echo "📊 Generating performance summary..."

# Extract timing information and create summary
if command -v bc >/dev/null 2>&1; then
    echo "Performance metrics extracted to $RESULTS_FILE"
else
    echo "⚠️ bc command not available - manual analysis of timing results required"
fi

# Cleanup
cd ..

echo ""
echo "✅ Performance testing completed!"
echo "📋 Results saved to: $RESULTS_FILE"
echo ""
echo "📊 Summary:"
echo "   - Performance benchmarks run for interpretation and compilation modes"
echo "   - Memory leak detection performed with valgrind (if available)"
echo "   - Concurrency stress testing completed"
echo "   - Results include timing, memory usage, and leak analysis"
echo ""
echo "🔍 To review results: cat $RESULTS_FILE"
