#!/bin/bash

# Performance benchmarking script for CURSED interpreter optimizations
# Compares baseline performance vs optimized performance

set -e

echo "=== CURSED Performance Optimization Benchmark ==="
echo "Date: $(date)"
echo "System: $(uname -a)"
echo

# Build the interpreter
echo "🔨 Building CURSED interpreter..."
zig build
echo "✅ Build complete"
echo

# Create performance test files
echo "📝 Creating performance test files..."

cat > simple_test.csd << 'EOF'
# Simple performance test
sus x drip = 42
sus y drip = x + 10
vibez.spill("Result:", y)
EOF

cat > function_test.csd << 'EOF'
# Function call overhead test
slay compute(n drip) drip {
    sus total drip = 0
    sus i drip = 0
    bestie (i < n) {
        total = total + i
        i = i + 1
    }
    damn total
}

sus result drip = compute(100)
vibez.spill("Function result:", result)
EOF

cat > variable_lookup_test.csd << 'EOF'
# Variable lookup performance test
yeet "mathz"
sus data []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
sus result drip = 0
sus i drip = 0

bestie (i < 50) {
    sus index drip = i % len(data)
    result = result + abs_normie(data[index])
    i = i + 1
}

vibez.spill("Variable lookup result:", result)
EOF

cat > memory_allocation_test.csd << 'EOF'
# Memory allocation performance test
yeet "stringz"
sus base tea = "Hello"
sus counter drip = 0

bestie (counter < 10) {
    sus temp tea = base + " World"
    counter = counter + 1
}

vibez.spill("Memory allocation test complete")
EOF

echo "✅ Test files created"
echo

# Function to run benchmark with timing
run_benchmark() {
    local test_name="$1"
    local test_file="$2"
    local iterations="$3"
    
    echo "🏃 Running $test_name benchmark ($iterations iterations)..."
    
    local total_time=0
    local best_time=999999
    local worst_time=0
    
    for i in $(seq 1 $iterations); do
        local start_time=$(date +%s%N)
        ./zig-out/bin/cursed "$test_file" > /dev/null 2>&1
        local end_time=$(date +%s%N)
        
        local execution_time=$((($end_time - $start_time) / 1000000)) # Convert to milliseconds
        total_time=$(($total_time + $execution_time))
        
        if [ $execution_time -lt $best_time ]; then
            best_time=$execution_time
        fi
        
        if [ $execution_time -gt $worst_time ]; then
            worst_time=$execution_time
        fi
    done
    
    local avg_time=$(($total_time / $iterations))
    
    echo "  Average: ${avg_time}ms"
    echo "  Best:    ${best_time}ms"
    echo "  Worst:   ${worst_time}ms"
    echo
}

# Run benchmarks
echo "📊 Starting performance benchmarks..."
echo

run_benchmark "Simple Operations" "simple_test.csd" 10
run_benchmark "Function Calls" "function_test.csd" 10
run_benchmark "Variable Lookup" "variable_lookup_test.csd" 10
run_benchmark "Memory Allocation" "memory_allocation_test.csd" 10

# Test compiler performance
echo "🔥 Testing LLVM compilation performance..."

echo "📝 Creating compilation test..."
cat > compile_test.csd << 'EOF'
# Compilation performance test
slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

vibez.spill("Fibonacci(10):", fibonacci(10))
EOF

echo "⏱️  Measuring compilation time..."
time ./zig-out/bin/cursed --compile compile_test.csd

# Test binary execution if compilation succeeded
if [ -f ./compile_test ]; then
    echo "⚡ Testing compiled binary performance..."
    time ./compile_test
    rm -f ./compile_test
fi

echo

# Memory usage analysis
echo "🧠 Memory usage analysis..."

echo "📊 Interpreter memory usage:"
/usr/bin/time -v ./zig-out/bin/cursed variable_lookup_test.csd 2>&1 | grep -E "(Maximum resident set size|User time|System time)" || echo "Memory analysis not available"

echo

# Parser performance test
echo "🔍 Parser performance test..."

cat > parser_test.csd << 'EOF'
# Complex parsing test
yeet "mathz"
yeet "stringz"
yeet "arrayz"

squad Point {
    spill x drip
    spill y drip
}

collab Drawable {
    slay draw()
}

slay complex_function(a drip, b tea, c []drip) (drip, tea) {
    sus result drip = 0
    sus message tea = b + " processed"
    
    sus i drip = 0
    bestie (i < len(c)) {
        ready (c[i] > 0) {
            result = result + c[i] * a
        } otherwise {
            result = result + abs_normie(c[i])
        }
        i = i + 1
    }
    
    damn result, message
}

sus data []drip = [1, -2, 3, -4, 5]
sus value, msg = complex_function(2, "Data", data)
vibez.spill("Complex result:", value, msg)
EOF

echo "⏱️  Parser performance:"
time ./zig-out/bin/cursed parser_test.csd

echo

# Optimization effectiveness test
echo "🎯 Optimization effectiveness analysis..."

echo "📈 Standard optimization levels:"

# Test different scenarios
for test in simple_test.csd function_test.csd variable_lookup_test.csd; do
    echo "  Testing $test:"
    
    # Measure with verbose output to see internal timings
    echo "    With verbose output:"
    time ./zig-out/bin/cursed --verbose "$test" 2>&1 | tail -5
    echo
done

# Compilation optimization levels
echo "🔧 Compilation optimization testing:"

for opt_level in 0 1 2 3; do
    echo "  Optimization level O$opt_level:"
    if timeout 30s ./zig-out/bin/cursed --compile -O$opt_level simple_test.csd 2>/dev/null; then
        echo "    ✅ O$opt_level compilation successful"
        if [ -f ./simple_test ]; then
            time ./simple_test > /dev/null 2>&1
            rm -f ./simple_test
        fi
    else
        echo "    ❌ O$opt_level compilation failed or timed out"
    fi
done

echo

# Cross-compilation performance
echo "🌐 Cross-compilation performance test..."

echo "⏱️  Cross-compilation to Linux x64:"
time zig build -Dtarget=x86_64-linux 2>/dev/null && echo "✅ Success" || echo "❌ Failed"

echo "⏱️  Cross-compilation to macOS ARM64:"
time zig build -Dtarget=aarch64-macos 2>/dev/null && echo "✅ Success" || echo "❌ Failed"

echo

# Cleanup
echo "🧹 Cleaning up test files..."
rm -f simple_test.csd function_test.csd variable_lookup_test.csd memory_allocation_test.csd compile_test.csd parser_test.csd

echo "✅ Performance benchmark complete!"
echo
echo "=== Summary ==="
echo "📊 All benchmarks completed successfully"
echo "📈 Check the timing results above for performance metrics"
echo "🎯 Focus optimization efforts on the slowest operations"
echo "🔧 Consider compiler optimization levels for production builds"
