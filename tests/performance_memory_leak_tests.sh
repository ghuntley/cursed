#!/bin/bash
# Performance and Memory Leak Detection Tests
# Tests compiler performance, memory usage, and leak detection

set -e

echo "🚀 Performance and Memory Leak Detection Tests"
echo "=============================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Test results
PERF_TESTS=0
PERF_PASSED=0
PERF_FAILED=0

# Helper function for performance testing
run_perf_test() {
    local test_name="$1"
    local test_file="$2"
    local max_time_seconds="$3"
    local max_memory_mb="$4"
    
    PERF_TESTS=$((PERF_TESTS + 1))
    echo -e "${BLUE}🔍 Performance Test: $test_name${NC}"
    
    # Build the test program
    cargo build --release > /dev/null 2>&1 || {
        echo -e "${RED}❌ Build failed${NC}"
        PERF_FAILED=$((PERF_FAILED + 1))
        return 1
    }
    
    # Test compilation time
    echo "  ⏱️  Testing compilation performance..."
    local start_time=$(date +%s.%N)
    
    if timeout $((max_time_seconds * 2)) cargo run --release --bin cursed -- compile "$test_file" > /tmp/perf_compile.log 2>&1; then
        local end_time=$(date +%s.%N)
        local compile_time=$(echo "$end_time - $start_time" | bc -l)
        local compile_time_int=$(echo "$compile_time / 1" | bc)
        
        if [[ $compile_time_int -gt $max_time_seconds ]]; then
            echo -e "  ${RED}❌ Compilation too slow: ${compile_time}s > ${max_time_seconds}s${NC}"
            PERF_FAILED=$((PERF_FAILED + 1))
            return 1
        fi
        echo -e "  ${GREEN}✅ Compilation time: ${compile_time}s${NC}"
    else
        echo -e "  ${RED}❌ Compilation failed or timed out${NC}"
        cat /tmp/perf_compile.log | head -n 5
        PERF_FAILED=$((PERF_FAILED + 1))
        return 1
    fi
    
    # Test memory usage during execution
    local executable_name=$(basename "$test_file" .csd)
    if [[ -f "./$executable_name" ]]; then
        echo "  💾 Testing memory usage..."
        
        # Use time command to measure memory
        if timeout 30 /usr/bin/time -v "./$executable_name" > /tmp/perf_exec.log 2> /tmp/perf_memory.log; then
            local max_memory_kb=$(grep "Maximum resident set size" /tmp/perf_memory.log | awk '{print $6}')
            local max_memory_mb_actual=$((max_memory_kb / 1024))
            
            if [[ $max_memory_mb_actual -gt $max_memory_mb ]]; then
                echo -e "  ${RED}❌ Memory usage too high: ${max_memory_mb_actual}MB > ${max_memory_mb}MB${NC}"
                PERF_FAILED=$((PERF_FAILED + 1))
                return 1
            fi
            echo -e "  ${GREEN}✅ Memory usage: ${max_memory_mb_actual}MB${NC}"
        else
            echo -e "  ${RED}❌ Execution failed or timed out${NC}"
            cat /tmp/perf_exec.log | head -n 5
            PERF_FAILED=$((PERF_FAILED + 1))
            return 1
        fi
        
        rm -f "./$executable_name"
    else
        echo -e "  ${RED}❌ Executable not generated${NC}"
        PERF_FAILED=$((PERF_FAILED + 1))
        return 1
    fi
    
    PERF_PASSED=$((PERF_PASSED + 1))
    echo -e "${GREEN}✅ $test_name: PASSED${NC}"
    echo
}

# Check for required tools
echo "🔧 Checking for required tools..."
if ! command -v bc &> /dev/null; then
    echo -e "${RED}❌ bc calculator not found. Install with: sudo apt install bc${NC}"
    exit 1
fi

if ! command -v time &> /dev/null; then
    echo -e "${RED}❌ time command not found. Install with: sudo apt install time${NC}"
    exit 1
fi

if command -v valgrind &> /dev/null; then
    VALGRIND_AVAILABLE=true
    echo -e "${GREEN}✅ Valgrind available for memory leak detection${NC}"
else
    VALGRIND_AVAILABLE=false
    echo -e "${YELLOW}⚠️  Valgrind not available. Install with: sudo apt install valgrind${NC}"
fi

echo

# Create test directory
mkdir -p /tmp/cursed_perf_tests
cd /tmp/cursed_perf_tests

echo "📋 Creating performance test programs..."
echo "======================================="

# Performance Test 1: Simple computation
cat > simple_computation.csd << 'EOF'
fr fr Simple computation performance test
slay fibonacci(n drip) drip {
    lowkey n <= 1 {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

sus result drip = fibonacci(20)
vibez.spill("Fibonacci(20):", result)
EOF

# Performance Test 2: Memory allocation intensive
cat > memory_intensive.csd << 'EOF'
fr fr Memory intensive test
sus big_array []drip = []
bestie i := 0; i < 10000; i = i + 1 {
    big_array.push(i * 2)
}

sus sum drip = 0
bestie i := 0; i < big_array.len(); i = i + 1 {
    sum = sum + big_array[i]
}

vibez.spill("Sum of 10000 elements:", sum)
EOF

# Performance Test 3: String processing
cat > string_processing.csd << 'EOF'
fr fr String processing performance test
sus text tea = "Hello, CURSED! "
sus repeated tea = ""

bestie i := 0; i < 1000; i = i + 1 {
    repeated = repeated + text
}

vibez.spill("String length:", repeated.len())
EOF

# Performance Test 4: Function call overhead
cat > function_calls.csd << 'EOF'
fr fr Function call overhead test
slay simple_add(a drip, b drip) drip {
    damn a + b
}

sus total drip = 0
bestie i := 0; i < 100000; i = i + 1 {
    total = simple_add(total, 1)
}

vibez.spill("Function call result:", total)
EOF

# Performance Test 5: Concurrent operations
cat > concurrent_perf.csd << 'EOF'
fr fr Concurrent performance test
sus ch = make_channel<drip>()
sus num_goroutines drip = 10

fr fr Launch goroutines
bestie i := 0; i < num_goroutines; i = i + 1 {
    stan {
        bestie j := 0; j < 100; j = j + 1 {
            dm_send(ch, i * 100 + j)
        }
    }
}

fr fr Collect results
sus results []drip = []
bestie i := 0; i < num_goroutines * 100; i = i + 1 {
    sus value drip = dm_recv(ch)
    results.push(value)
}

vibez.spill("Concurrent operations completed:", results.len())
EOF

echo "🚀 Running Performance Tests..."
echo "==============================="

# Set working directory back to CURSED project
cd /home/ghuntley/code/cursed

# Run performance tests with different thresholds
run_perf_test "Simple Computation" "/tmp/cursed_perf_tests/simple_computation.csd" 5 50
run_perf_test "Memory Intensive" "/tmp/cursed_perf_tests/memory_intensive.csd" 10 100
run_perf_test "String Processing" "/tmp/cursed_perf_tests/string_processing.csd" 15 200
run_perf_test "Function Calls" "/tmp/cursed_perf_tests/function_calls.csd" 8 75
run_perf_test "Concurrent Operations" "/tmp/cursed_perf_tests/concurrent_perf.csd" 20 150

# Memory leak detection with Valgrind (if available)
if [[ "$VALGRIND_AVAILABLE" == true ]]; then
    echo -e "${BLUE}🔍 Running Memory Leak Detection Tests...${NC}"
    echo "========================================"
    
    # Create a simple test for memory leak detection
    cat > /tmp/cursed_perf_tests/leak_test.csd << 'EOF'
fr fr Memory leak detection test
sus array []drip = []
bestie i := 0; i < 1000; i = i + 1 {
    array.push(i)
}
vibez.spill("Leak test completed")
EOF
    
    # Compile the test
    if cargo run --release --bin cursed -- compile "/tmp/cursed_perf_tests/leak_test.csd" > /dev/null 2>&1; then
        echo "  🔬 Running Valgrind memory leak analysis..."
        
        if valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes \
           --error-exitcode=1 ./leak_test > /tmp/valgrind_output.log 2>&1; then
            echo -e "  ${GREEN}✅ No memory leaks detected${NC}"
        else
            echo -e "  ${YELLOW}⚠️  Potential memory issues detected${NC}"
            echo "  📄 Valgrind summary:"
            grep -E "(definitely lost|indirectly lost|possibly lost)" /tmp/valgrind_output.log || echo "    No leaks summary found"
        fi
        
        rm -f ./leak_test
    else
        echo -e "  ${RED}❌ Failed to compile leak test${NC}"
    fi
fi

echo "📊 Performance Test Results"
echo "==========================="
echo -e "Total performance tests: ${BLUE}$PERF_TESTS${NC}"
echo -e "Passed: ${GREEN}$PERF_PASSED${NC}"
echo -e "Failed: ${RED}$PERF_FAILED${NC}"

if [[ $PERF_FAILED -eq 0 ]]; then
    echo -e "${GREEN}🎉 All performance tests passed!${NC}"
    echo -e "${GREEN}✅ CURSED compiler performance is acceptable${NC}"
    exit 0
else
    echo -e "${RED}❌ Some performance tests failed. Optimization needed.${NC}"
    exit 1
fi

# Cleanup
rm -rf /tmp/cursed_perf_tests
rm -f /tmp/perf_*.log /tmp/valgrind_output.log
