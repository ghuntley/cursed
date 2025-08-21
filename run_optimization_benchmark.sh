#!/bin/bash

# CURSED Compiler Optimization Benchmark Script
# Tests different optimization levels and measures performance

echo "🚀 CURSED Compiler Optimization Benchmark"
echo "========================================="

# Ensure we have a clean build
echo "🔨 Building CURSED compiler..."
zig build || exit 1

CURSED_BIN="./zig-out/bin/cursed-zig"
TEST_FILE="optimization_test.csd"

if [[ ! -f "$CURSED_BIN" ]]; then
    echo "❌ CURSED compiler not found at $CURSED_BIN"
    exit 1
fi

if [[ ! -f "$TEST_FILE" ]]; then
    echo "❌ Test file not found at $TEST_FILE"
    exit 1
fi

# Create results directory
mkdir -p optimization_benchmark_results
cd optimization_benchmark_results

echo ""
echo "📊 Testing Different Optimization Levels"
echo "========================================"

# Test each optimization level
for OPT_LEVEL in O0 O1 O2 O3 Oz Os; do
    echo ""
    echo "🔧 Testing optimization level: $OPT_LEVEL"
    echo "-----------------------------------"
    
    # Measure compilation time
    echo "⏱️  Measuring compilation time..."
    START_TIME=$(date +%s.%N)
    
    # Compile with optimization level
    if [[ "$OPT_LEVEL" == "O0" ]]; then
        ../$CURSED_BIN "../$TEST_FILE" > "${OPT_LEVEL}_compile.log" 2>&1
    else
        ../$CURSED_BIN --optimize="$OPT_LEVEL" "../$TEST_FILE" > "${OPT_LEVEL}_compile.log" 2>&1
    fi
    
    COMPILE_EXIT_CODE=$?
    END_TIME=$(date +%s.%N)
    COMPILE_TIME=$(echo "$END_TIME - $START_TIME" | bc -l)
    
    if [[ $COMPILE_EXIT_CODE -eq 0 ]]; then
        echo "  ✅ Compilation successful in ${COMPILE_TIME}s"
        
        # Measure execution time (interpretation mode)
        echo "🏃 Measuring execution time..."
        START_TIME=$(date +%s.%N)
        
        ../$CURSED_BIN "../$TEST_FILE" > "${OPT_LEVEL}_execution.log" 2>&1
        EXEC_EXIT_CODE=$?
        
        END_TIME=$(date +%s.%N)
        EXEC_TIME=$(echo "$END_TIME - $START_TIME" | bc -l)
        
        if [[ $EXEC_EXIT_CODE -eq 0 ]]; then
            echo "  ✅ Execution successful in ${EXEC_TIME}s"
        else
            echo "  ❌ Execution failed"
            echo "  📋 Check ${OPT_LEVEL}_execution.log for details"
        fi
        
        # Record results
        echo "${OPT_LEVEL},${COMPILE_TIME},${EXEC_TIME}" >> benchmark_results.csv
        
    else
        echo "  ❌ Compilation failed"
        echo "  📋 Check ${OPT_LEVEL}_compile.log for details"
        echo "${OPT_LEVEL},FAILED,FAILED" >> benchmark_results.csv
    fi
done

echo ""
echo "📈 Benchmark Results Summary"
echo "=========================="

# Create CSV header if results exist
if [[ -f benchmark_results.csv ]]; then
    echo "Optimization_Level,Compile_Time(s),Execution_Time(s)" > results_summary.csv
    cat benchmark_results.csv >> results_summary.csv
    
    echo "📊 Results saved to results_summary.csv:"
    echo ""
    column -t -s ',' results_summary.csv
else
    echo "❌ No benchmark results generated"
fi

echo ""
echo "🔬 Optimization Analysis"
echo "======================"

# Analyze optimization effectiveness
if [[ -f benchmark_results.csv ]]; then
    echo "📋 Compilation Time Analysis:"
    echo "  O0 (no optimization) should be fastest to compile"
    echo "  O3 (aggressive optimization) should be slowest to compile"
    echo ""
    
    echo "🚀 Execution Time Analysis:"  
    echo "  O0 (no optimization) should be slowest to execute"
    echo "  O2/O3 (optimized) should be fastest to execute"
    echo ""
    
    # Calculate speedup if we have valid data
    O0_EXEC=$(grep "^O0," benchmark_results.csv | cut -d',' -f3)
    O2_EXEC=$(grep "^O2," benchmark_results.csv | cut -d',' -f3)
    O3_EXEC=$(grep "^O3," benchmark_results.csv | cut -d',' -f3)
    
    if [[ "$O0_EXEC" != "FAILED" && "$O2_EXEC" != "FAILED" && "$O3_EXEC" != "FAILED" ]]; then
        O2_SPEEDUP=$(echo "scale=2; $O0_EXEC / $O2_EXEC" | bc -l 2>/dev/null || echo "N/A")
        O3_SPEEDUP=$(echo "scale=2; $O0_EXEC / $O3_EXEC" | bc -l 2>/dev/null || echo "N/A")
        
        echo "⚡ Performance Improvements:"
        echo "  O2 speedup over O0: ${O2_SPEEDUP}x"
        echo "  O3 speedup over O0: ${O3_SPEEDUP}x"
    else
        echo "⚠️  Cannot calculate speedup - some tests failed"
    fi
fi

echo ""
echo "💡 Optimization Recommendations"
echo "=============================="
echo "  O0: Use for development/debugging (fast compilation)"
echo "  O1: Use for development with some optimization"  
echo "  O2: Use for production builds (good balance)"
echo "  O3: Use for performance-critical applications"
echo "  Oz: Use for size-constrained environments"
echo "  Os: Use when both size and speed matter"

echo ""
echo "✅ Optimization benchmark completed!"
echo "📁 Results saved in optimization_benchmark_results/"

cd ..
