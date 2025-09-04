#!/bin/bash

# Quick test run that continues on failures and gives summary
cd "$(dirname "$0")/.."

passed=0
failed=0
compile_errors=0
interpreter_errors=0
exit_code_warnings=0

for file in test_suite/test_programs/*/*.csd; do
    if [[ "$file" == *".skip" ]]; then
        continue
    fi
    
    name=$(basename "$file")
    echo -n "Testing $name... "
    
    # Run interpreter
    interp_output=$(./zig-out/bin/cursed-compiler --interpret "$file" 2>&1)
    interp_exit=$?
    
    # Run compiler
    compiled_output=$(./zig-out/bin/cursed-compiler --compile "$file" -o temp_test_binary 2>&1 && ./temp_test_binary 2>&1)
    comp_exit=$?
    
    # Clean up
    rm -f temp_test_binary
    
    # Compare results
    if [[ $interp_exit -eq 0 && $comp_exit -eq 0 ]]; then
        # Both succeeded - check if outputs match
        if [[ "$interp_output" == "$compiled_output" ]]; then
            echo "PASS"
            ((passed++))
        else
            echo "FAIL (output mismatch)"
            ((failed++))
        fi
    elif [[ $interp_exit -ne 0 && $comp_exit -ne 0 ]]; then
        echo "PASS (both failed consistently)"
        ((passed++))
    elif [[ $interp_exit -ne 0 ]]; then
        echo "INTERPRETER ERROR"
        ((interpreter_errors++))
    else
        echo "COMPILE ERROR"
        ((compile_errors++))
    fi
done

echo ""
echo "Summary:"
echo "  Passed: $passed"
echo "  Failed: $failed" 
echo "  Compile Errors: $compile_errors"
echo "  Interpreter Errors: $interpreter_errors"
echo "  Total: $((passed + failed + compile_errors + interpreter_errors))"
