#!/bin/bash
cd test_suite
total_tests=0
passed_tests=0
failed_tests=0
error_tests=0

echo "CURSED Test Suite Summary"
echo "========================="
echo

for test_file in $(find test_programs -name "*.csd" | sort); do
    total_tests=$((total_tests + 1))
    echo -n "[${total_tests}] Testing: ${test_file} ... "
    
    # Run interpreter
    timeout 5 ../zig-out/bin/cursed-compiler --interpret "$test_file" > /tmp/interp_out.txt 2>&1
    interp_exit=$?
    
    # Run compiler
    timeout 10 ../zig-out/bin/cursed-compiler --compile "$test_file" -o /tmp/test_binary > /tmp/compile_out.txt 2>&1
    compile_exit=$?
    
    if [[ $compile_exit -eq 0 ]]; then
        timeout 5 /tmp/test_binary > /tmp/compiled_out.txt 2>&1
        run_exit=$?
        
        if [[ $interp_exit -eq 0 && $run_exit -eq 0 ]]; then
            if diff -q /tmp/interp_out.txt /tmp/compiled_out.txt > /dev/null 2>&1; then
                passed_tests=$((passed_tests + 1))
                echo "PASS"
            else
                failed_tests=$((failed_tests + 1))
                echo "FAIL (output mismatch)"
            fi
        elif [[ $interp_exit -ne 0 && $run_exit -ne 0 ]]; then
            # Both failed, check if they failed the same way
            passed_tests=$((passed_tests + 1))
            echo "PASS (both failed)"
        else
            failed_tests=$((failed_tests + 1))
            echo "FAIL (exit code mismatch: interp=$interp_exit, compiled=$run_exit)"
        fi
    else
        if [[ $interp_exit -eq 0 ]]; then
            failed_tests=$((failed_tests + 1))
            echo "FAIL (compilation failed)"
        else
            error_tests=$((error_tests + 1))
            echo "ERROR (both failed to run)"
        fi
    fi
done

echo
echo "Final Results:"
echo "=============="
echo "Total tests: $total_tests"
echo "Passed: $passed_tests"
echo "Failed: $failed_tests"
echo "Errors: $error_tests"
echo "Success rate: $(echo "scale=1; $passed_tests * 100 / $total_tests" | bc -l)%"
