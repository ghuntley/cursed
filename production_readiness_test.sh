#!/bin/bash

# CURSED Stdlib Production Readiness Test Script
# Tests native compilation for all stdlib modules

set -e

echo "=== CURSED Stdlib Production Readiness Test ==="
echo "Testing native compilation for all stdlib modules..."

# Create log files
RESULTS_FILE="production_test_results.txt"
PERFORMANCE_FILE="performance_metrics.txt"
ERROR_LOG="compilation_errors.log"

> "$RESULTS_FILE"
> "$PERFORMANCE_FILE"
> "$ERROR_LOG"

# Test counters
TOTAL_MODULES=0
SUCCESSFUL_MODULES=0
FAILED_MODULES=0
COMPILATION_TIMES=()

# Function to test both modes and compare
test_both_modes() {
    local test_file=$1
    local module_name=$2
    
    echo "Testing $module_name..." | tee -a "$RESULTS_FILE"
    
    # Test interpretation mode
    echo "  → Interpretation mode..." | tee -a "$RESULTS_FILE"
    if timeout 30 cargo run --bin cursed "$test_file" > "${module_name}_interpretation.out" 2>"${module_name}_interpretation.err"; then
        echo "    ✅ Interpretation successful" | tee -a "$RESULTS_FILE"
        
        # Test compilation mode
        echo "  → Compilation mode..." | tee -a "$RESULTS_FILE"
        local start_time=$(date +%s.%N)
        
        if timeout 60 cargo run --bin cursed -- compile "$test_file" 2>"${module_name}_compile.err"; then
            local end_time=$(date +%s.%N)
            local compile_time=$(echo "$end_time - $start_time" | bc -l)
            echo "    ✅ Compilation successful (${compile_time}s)" | tee -a "$RESULTS_FILE"
            echo "$module_name: ${compile_time}s" >> "$PERFORMANCE_FILE"
            
            # Execute compiled binary
            local executable=$(basename "$test_file" .csd)
            if [ -f "./$executable" ]; then
                if timeout 30 "./$executable" > "${module_name}_compiled.out" 2>"${module_name}_compiled.err"; then
                    echo "    ✅ Compiled execution successful" | tee -a "$RESULTS_FILE"
                    
                    # Compare outputs
                    if diff -q "${module_name}_interpretation.out" "${module_name}_compiled.out" > /dev/null 2>&1; then
                        echo "    ✅ Output consistency verified" | tee -a "$RESULTS_FILE"
                        SUCCESSFUL_MODULES=$((SUCCESSFUL_MODULES + 1))
                        return 0
                    else
                        echo "    ❌ Output mismatch detected" | tee -a "$RESULTS_FILE"
                        echo "Module: $module_name - Output mismatch" >> "$ERROR_LOG"
                    fi
                else
                    echo "    ❌ Compiled execution failed" | tee -a "$RESULTS_FILE"
                    echo "Module: $module_name - Compiled execution failed" >> "$ERROR_LOG"
                fi
            else
                echo "    ❌ Compiled binary not found" | tee -a "$RESULTS_FILE"
                echo "Module: $module_name - Binary not found" >> "$ERROR_LOG"
            fi
        else
            echo "    ❌ Compilation failed" | tee -a "$RESULTS_FILE"
            echo "Module: $module_name - Compilation failed" >> "$ERROR_LOG"
        fi
    else
        echo "    ❌ Interpretation failed" | tee -a "$RESULTS_FILE"
        echo "Module: $module_name - Interpretation failed" >> "$ERROR_LOG"
    fi
    
    FAILED_MODULES=$((FAILED_MODULES + 1))
    return 1
}

# Test core stdlib modules
echo "=== Testing Core Stdlib Modules ===" | tee -a "$RESULTS_FILE"

# Find all test files
for test_file in stdlib/*/test_*.csd; do
    if [ -f "$test_file" ]; then
        TOTAL_MODULES=$((TOTAL_MODULES + 1))
        module_name=$(basename "$(dirname "$test_file")")
        test_both_modes "$test_file" "$module_name"
        echo "" | tee -a "$RESULTS_FILE"
    fi
done

# Test complex multi-module programs
echo "=== Testing Complex Multi-Module Programs ===" | tee -a "$RESULTS_FILE"

# Create complex test program
cat > complex_stdlib_test.csd << 'EOF'
yeet "testz"
yeet "math"
yeet "string"
yeet "crypto"
yeet "json"
yeet "csv"

test_start("Complex multi-module test")

# Test math operations
sus result normie = math.add(10, 20)
assert_eq_int(result, 30)

# Test string operations  
sus text tea = "Hello, World!"
sus length normie = string.length(text)
assert_eq_int(length, 13)

# Test JSON parsing
sus json_data tea = '{"name": "test", "value": 42}'
sus parsed_data tea = json.parse(json_data)
assert_eq_string(parsed_data, "test")

print_test_summary()
EOF

if [ -f "complex_stdlib_test.csd" ]; then
    TOTAL_MODULES=$((TOTAL_MODULES + 1))
    test_both_modes "complex_stdlib_test.csd" "complex_multi_module"
fi

# Test self-hosting capability
echo "=== Testing Self-Hosting Capability ===" | tee -a "$RESULTS_FILE"

cat > self_hosting_production_test.csd << 'EOF'
vibez.spill("Self-hosting production test")
sus x normie = 42
sus y drip = 3.14
vibez.spill("Integer:", x)
vibez.spill("Float:", y)
vibez.spill("Self-hosting test completed successfully")
EOF

if [ -f "self_hosting_production_test.csd" ]; then
    TOTAL_MODULES=$((TOTAL_MODULES + 1))
    test_both_modes "self_hosting_production_test.csd" "self_hosting_production"
fi

# Generate performance report
echo "=== Performance Analysis ===" | tee -a "$RESULTS_FILE"
if [ -s "$PERFORMANCE_FILE" ]; then
    echo "Compilation times:" | tee -a "$RESULTS_FILE"
    sort -k2 -n "$PERFORMANCE_FILE" | tee -a "$RESULTS_FILE"
    
    # Calculate average compilation time
    avg_time=$(awk '{sum += $2; count++} END {print sum/count}' "$PERFORMANCE_FILE")
    echo "Average compilation time: ${avg_time}s" | tee -a "$RESULTS_FILE"
fi

# Generate final report
echo "=== Production Readiness Report ===" | tee -a "$RESULTS_FILE"
echo "Total modules tested: $TOTAL_MODULES" | tee -a "$RESULTS_FILE"
echo "Successful modules: $SUCCESSFUL_MODULES" | tee -a "$RESULTS_FILE"
echo "Failed modules: $FAILED_MODULES" | tee -a "$RESULTS_FILE"

if [ $TOTAL_MODULES -gt 0 ]; then
    success_rate=$(echo "scale=2; $SUCCESSFUL_MODULES * 100 / $TOTAL_MODULES" | bc -l)
    echo "Success rate: ${success_rate}%" | tee -a "$RESULTS_FILE"
    
    if [ "${success_rate%.*}" -ge 95 ]; then
        echo "✅ PRODUCTION READY: High success rate (≥95%)" | tee -a "$RESULTS_FILE"
    elif [ "${success_rate%.*}" -ge 80 ]; then
        echo "⚠️  NEEDS ATTENTION: Moderate success rate (80-94%)" | tee -a "$RESULTS_FILE"
    else
        echo "❌ NOT PRODUCTION READY: Low success rate (<80%)" | tee -a "$RESULTS_FILE"
    fi
fi

# Show errors if any
if [ -s "$ERROR_LOG" ]; then
    echo "=== Compilation Errors ===" | tee -a "$RESULTS_FILE"
    cat "$ERROR_LOG" | tee -a "$RESULTS_FILE"
fi

echo "=== Test Complete ===" | tee -a "$RESULTS_FILE"
echo "Results saved to: $RESULTS_FILE"
echo "Performance metrics: $PERFORMANCE_FILE"
echo "Error log: $ERROR_LOG"

# Cleanup temporary files
rm -f *_interpretation.out *_interpretation.err *_compiled.out *_compiled.err *_compile.err
find . -name "test_*" -type f -executable -delete
find . -name "complex_*" -type f -executable -delete
find . -name "self_hosting_*" -type f -executable -delete
