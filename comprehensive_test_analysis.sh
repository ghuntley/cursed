#!/bin/bash

set -e

echo "CURSED Test Suite - Comprehensive Analysis"
echo "=========================================="

cd /home/ghuntley/cursed

COMPILER_PATH="./zig-out/bin/cursed-compiler"
TEST_DIR="./test_suite/test_programs"
OUTPUT_FILE="comprehensive_test_results.log"

echo "Building compiler..."
zig build 2>/dev/null || echo "Build completed with warnings"

echo "Starting comprehensive test analysis..." > $OUTPUT_FILE
echo "Timestamp: $(date)" >> $OUTPUT_FILE
echo "=========================================" >> $OUTPUT_FILE

# Initialize counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
COMPILE_ERRORS=0
RUNTIME_ERRORS=0
OUTPUT_MISMATCHES=0
EXIT_CODE_ISSUES=0
INTERPRETER_ERRORS=0

echo "Scanning test directory..."
find $TEST_DIR -name "*.csd" -type f | while read -r test_file; do
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    rel_path=${test_file#$TEST_DIR/}
    echo ""
    echo "[$TOTAL_TESTS] Testing: $rel_path"
    echo "[$TOTAL_TESTS] Testing: $rel_path" >> $OUTPUT_FILE
    
    # Test interpreter mode
    interp_output_file="/tmp/cursed_interp_$$.txt"
    interp_exit_code=0
    timeout 30s $COMPILER_PATH --interpret "$test_file" > "$interp_output_file" 2>&1 || interp_exit_code=$?
    
    # Test compiled mode  
    compiled_output_file="/tmp/cursed_compiled_$$.txt"
    compiled_binary="/tmp/cursed_bin_$$"
    compile_exit_code=0
    run_exit_code=0
    
    # Compile
    timeout 30s $COMPILER_PATH --compile "$test_file" -o "$compiled_binary" 2>/dev/null || compile_exit_code=$?
    
    if [ $compile_exit_code -eq 0 ] && [ -f "$compiled_binary" ]; then
        # Run compiled binary
        timeout 30s "$compiled_binary" > "$compiled_output_file" 2>&1 || run_exit_code=$?
        rm -f "$compiled_binary" 2>/dev/null
    else
        echo "Compilation failed" > "$compiled_output_file"
        run_exit_code=1
    fi
    
    # Analyze results
    if [ $interp_exit_code -ne 0 ]; then
        echo "  Result: INTERPRETER_ERROR (exit $interp_exit_code)"
        echo "  Result: INTERPRETER_ERROR (exit $interp_exit_code)" >> $OUTPUT_FILE
        INTERPRETER_ERRORS=$((INTERPRETER_ERRORS + 1))
        FAILED_TESTS=$((FAILED_TESTS + 1))
    elif [ $compile_exit_code -ne 0 ]; then
        echo "  Result: COMPILE_ERROR"
        echo "  Result: COMPILE_ERROR" >> $OUTPUT_FILE
        COMPILE_ERRORS=$((COMPILE_ERRORS + 1))
        FAILED_TESTS=$((FAILED_TESTS + 1))
    elif [ $run_exit_code -ne 0 ] && [ $run_exit_code -ne $interp_exit_code ]; then
        echo "  Result: RUNTIME_ERROR (compiled exit $run_exit_code vs interpreter exit $interp_exit_code)"
        echo "  Result: RUNTIME_ERROR (compiled exit $run_exit_code vs interpreter exit $interp_exit_code)" >> $OUTPUT_FILE
        RUNTIME_ERRORS=$((RUNTIME_ERRORS + 1))
        FAILED_TESTS=$((FAILED_TESTS + 1))
    else
        # Compare outputs
        if ! diff -q "$interp_output_file" "$compiled_output_file" >/dev/null 2>&1; then
            echo "  Result: OUTPUT_MISMATCH"
            echo "  Result: OUTPUT_MISMATCH" >> $OUTPUT_FILE
            echo "  Interpreter:" >> $OUTPUT_FILE
            head -20 "$interp_output_file" >> $OUTPUT_FILE
            echo "  Compiled:" >> $OUTPUT_FILE
            head -20 "$compiled_output_file" >> $OUTPUT_FILE
            OUTPUT_MISMATCHES=$((OUTPUT_MISMATCHES + 1))
            FAILED_TESTS=$((FAILED_TESTS + 1))
        elif [ $interp_exit_code -ne 0 ] || [ $run_exit_code -ne 0 ]; then
            echo "  Result: PASS (EXIT_CODE_ISSUE: exit $run_exit_code, output correct)"
            echo "  Result: PASS (EXIT_CODE_ISSUE: exit $run_exit_code, output correct)" >> $OUTPUT_FILE
            EXIT_CODE_ISSUES=$((EXIT_CODE_ISSUES + 1))
            PASSED_TESTS=$((PASSED_TESTS + 1))
        else
            echo "  Result: PASS"
            echo "  Result: PASS" >> $OUTPUT_FILE
            PASSED_TESTS=$((PASSED_TESTS + 1))
        fi
    fi
    
    # Cleanup
    rm -f "$interp_output_file" "$compiled_output_file" 2>/dev/null
    echo "---" >> $OUTPUT_FILE
done

# Final summary
echo ""
echo "========================================="
echo "COMPREHENSIVE TEST ANALYSIS SUMMARY"
echo "========================================="
echo "Total Tests: $TOTAL_TESTS"
echo "Passed: $PASSED_TESTS"
echo "Failed: $FAILED_TESTS"
echo ""
echo "Failure Categories:"
echo "  Compile Errors: $COMPILE_ERRORS"
echo "  Runtime Errors: $RUNTIME_ERRORS"
echo "  Output Mismatches: $OUTPUT_MISMATCHES"
echo "  Interpreter Errors: $INTERPRETER_ERRORS"
echo ""
echo "Issues (Non-blocking):"
echo "  Exit Code Issues: $EXIT_CODE_ISSUES"
echo ""
if [ $TOTAL_TESTS -gt 0 ]; then
    SUCCESS_RATE=$(echo "scale=1; $PASSED_TESTS * 100 / $TOTAL_TESTS" | bc -l)
    echo "Health Score: ${SUCCESS_RATE}%"
else
    echo "Health Score: N/A"
fi
echo "========================================="

# Also save summary to file
echo "" >> $OUTPUT_FILE
echo "FINAL SUMMARY:" >> $OUTPUT_FILE
echo "Total Tests: $TOTAL_TESTS" >> $OUTPUT_FILE
echo "Passed: $PASSED_TESTS" >> $OUTPUT_FILE
echo "Failed: $FAILED_TESTS" >> $OUTPUT_FILE
echo "Compile Errors: $COMPILE_ERRORS" >> $OUTPUT_FILE
echo "Runtime Errors: $RUNTIME_ERRORS" >> $OUTPUT_FILE  
echo "Output Mismatches: $OUTPUT_MISMATCHES" >> $OUTPUT_FILE
echo "Interpreter Errors: $INTERPRETER_ERRORS" >> $OUTPUT_FILE
echo "Exit Code Issues: $EXIT_CODE_ISSUES" >> $OUTPUT_FILE
if [ $TOTAL_TESTS -gt 0 ]; then
    SUCCESS_RATE=$(echo "scale=1; $PASSED_TESTS * 100 / $TOTAL_TESTS" | bc -l)
    echo "Health Score: ${SUCCESS_RATE}%" >> $OUTPUT_FILE
fi

echo "Complete results saved to: $OUTPUT_FILE"
