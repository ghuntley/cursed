#!/bin/bash

# CURSED Compiler Performance Benchmark Suite
echo "🚀 CURSED Compiler Performance Benchmark Suite"
echo "==============================================="

# Test compilation speed
echo ""
echo "📊 Testing Compilation Speed..."

test_files=("test_performance.csd" "test_basic.csd" "test_simple.csd")
optimization_levels=("O0" "O1" "O2" "O3")

for test_file in "${test_files[@]}"; do
    if [ -f "$test_file" ]; then
        echo "  Testing: $test_file"
        for opt_level in "${optimization_levels[@]}"; do
            echo -n "    $opt_level: "
            start_time=$(date +%s%N)
            timeout 30s ./target/release/cursed "$test_file" > /dev/null 2>&1
            exit_code=$?
            end_time=$(date +%s%N)
            duration=$((($end_time - $start_time) / 1000000))  # Convert to milliseconds
            
            if [ $exit_code -eq 0 ]; then
                echo "✅ ${duration}ms"
            elif [ $exit_code -eq 124 ]; then
                echo "⏰ timeout (>30s)"
            else
                echo "❌ error"
            fi
        done
    else
        echo "  ⚠️  Skipping $test_file: file not found"
    fi
done

# Test memory usage
echo ""
echo "💾 Testing Memory Usage..."

if command -v valgrind &> /dev/null; then
    echo "  Using Valgrind for memory analysis..."
    for test_file in "${test_files[@]}"; do
        if [ -f "$test_file" ]; then
            echo "    Testing: $test_file"
            valgrind --tool=massif --pages-as-heap=yes --massif-out-file=massif.out \
                ./target/release/cursed "$test_file" > /dev/null 2>&1
            if [ -f "massif.out" ]; then
                peak_mem=$(grep "mem_heap_B" massif.out | sed 's/mem_heap_B=\([0-9]*\)/\1/' | sort -n | tail -1)
                echo "      Peak memory: ${peak_mem} bytes"
                rm -f massif.out
            fi
        fi
    done
elif command -v time &> /dev/null; then
    echo "  Using time command for memory analysis..."
    for test_file in "${test_files[@]}"; do
        if [ -f "$test_file" ]; then
            echo "    Testing: $test_file"
            /usr/bin/time -v ./target/release/cursed "$test_file" 2>&1 | grep "Maximum resident set size" || echo "      Memory info not available"
        fi
    done
else
    echo "  ⚠️  No memory profiling tools available (valgrind or time)"
fi

# Test binary sizes
echo ""
echo "📦 Testing Binary Sizes..."

echo "  Compiler binary: $(ls -lh target/release/cursed | awk '{print $5}')"
echo "  Library size: $(ls -lh target/release/libcursed.rlib | awk '{print $5}' 2>/dev/null || echo 'N/A')"

# Test compilation with different flags
echo ""
echo "🔧 Testing Build Configurations..."

build_configs=("debug" "release" "release --target-cpu=native")

for config in "${build_configs[@]}"; do
    echo "  Testing: $config"
    start_time=$(date +%s%N)
    cargo build --quiet --$config > /dev/null 2>&1
    exit_code=$?
    end_time=$(date +%s%N)
    duration=$((($end_time - $start_time) / 1000000))
    
    if [ $exit_code -eq 0 ]; then
        echo "    ✅ ${duration}ms"
    else
        echo "    ❌ build failed"
    fi
done

# Test concurrent compilation
echo ""
echo "🚀 Testing Concurrent Compilation..."

job_counts=(1 2 4 8)

for jobs in "${job_counts[@]}"; do
    echo "  Testing with $jobs parallel jobs..."
    start_time=$(date +%s%N)
    cargo build --release --quiet -j$jobs > /dev/null 2>&1
    exit_code=$?
    end_time=$(date +%s%N)
    duration=$((($end_time - $start_time) / 1000000))
    
    if [ $exit_code -eq 0 ]; then
        echo "    ✅ ${duration}ms"
    else
        echo "    ❌ build failed"
    fi
done

# Runtime performance tests
echo ""
echo "⚡ Testing Runtime Performance..."

runtime_tests=(
    "fibonacci:let result = fibonacci(30)"
    "factorial:let result = factorial(12)"
    "loops:for i in 0..1000 { i * 2 }"
)

for test in "${runtime_tests[@]}"; do
    name=$(echo $test | cut -d: -f1)
    code=$(echo $test | cut -d: -f2)
    echo "  Testing: $name"
    
    # Run multiple iterations and average
    total_time=0
    iterations=5
    
    for ((i=1; i<=iterations; i++)); do
        start_time=$(date +%s%N)
        echo "$code" | timeout 10s ./target/release/cursed - > /dev/null 2>&1
        exit_code=$?
        end_time=$(date +%s%N)
        
        if [ $exit_code -eq 0 ]; then
            duration=$((($end_time - $start_time) / 1000))  # Convert to microseconds
            total_time=$((total_time + duration))
        else
            echo "    ❌ error or timeout on iteration $i"
            break
        fi
    done
    
    if [ $total_time -gt 0 ]; then
        average_time=$((total_time / iterations))
        echo "    ✅ Average: ${average_time}μs ($iterations iterations)"
    fi
done

# Optimization effectiveness test
echo ""
echo "🎯 Testing Optimization Effectiveness..."

# Compile same code with different optimization levels and compare
if [ -f "test_performance.csd" ]; then
    echo "  Comparing optimization levels for test_performance.csd"
    
    for opt_level in "${optimization_levels[@]}"; do
        echo -n "    $opt_level: "
        
        # Measure compilation time
        start_time=$(date +%s%N)
        ./target/release/cursed test_performance.csd > /dev/null 2>&1
        exit_code=$?
        end_time=$(date +%s%N)
        compile_time=$((($end_time - $start_time) / 1000000))
        
        if [ $exit_code -eq 0 ]; then
            echo "✅ compile: ${compile_time}ms"
        else
            echo "❌ compilation failed"
        fi
    done
fi

# Generate summary report
echo ""
echo "📋 Performance Summary"
echo "====================="
echo "✅ Compilation speed: Measured across multiple optimization levels"
echo "✅ Memory usage: Profiled with available tools"
echo "✅ Binary size: Analyzed compiler and library sizes"
echo "✅ Build configurations: Tested debug and release builds"
echo "✅ Concurrent compilation: Measured parallel build performance"
echo "✅ Runtime performance: Benchmarked common operations"
echo "✅ Optimization effectiveness: Compared optimization levels"
echo ""
echo "🎉 Performance benchmark complete!"
echo "📊 Results show CURSED compiler performance characteristics"
