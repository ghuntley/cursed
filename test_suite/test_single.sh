#!/bin/bash

CURSED_COMPILER="/home/ghuntley/cursed/zig-out/bin/cursed-compiler"
TEST_FILE="test_programs/edge_cases/edge_case_operator_precedence.💀"
CURSED_ROOT="/home/ghuntley/cursed"

echo "Testing single file: $TEST_FILE"
echo "Working from: $CURSED_ROOT"

# Change to cursed root directory for proper stdlib loading
cd "$CURSED_ROOT"

# Run interpreter mode
echo "Running interpreter..."
if interp_output=$("$CURSED_COMPILER" --interpret "test_suite/$TEST_FILE" 2>&1); then
    interp_exit=0
    echo "Interpreter succeeded with output: '$interp_output'"
else
    interp_exit=$?
    echo "Interpreter failed with exit: $interp_exit, output: '$interp_output'"
fi

# Try to compile
temp_binary="/tmp/cursed_test_single_debug_$$"
echo "Running compiler to: $temp_binary"
if comp_stderr=$("$CURSED_COMPILER" --compile "test_suite/$TEST_FILE" -o "$temp_binary" 2>&1 >/dev/null); then
    echo "Compilation succeeded"
    echo "comp_stderr: '$comp_stderr'"
    
    if [[ -f "$temp_binary" ]]; then
        echo "Binary file exists: $temp_binary"
        if binary_output=$("$temp_binary" 2>/dev/null); then
            echo "Binary execution succeeded with output: '$binary_output'"
            comp_exit=0
        else
            comp_exit=$?
            echo "Binary execution failed with exit: $comp_exit"
        fi
        rm -f "$temp_binary"
    else
        echo "Binary file does NOT exist!"
        comp_exit=1
    fi
else
    comp_exit=$?
    echo "Compilation failed with exit: $comp_exit"
    echo "comp_stderr: '$comp_stderr'"
fi

echo "Final result: interp_exit=$interp_exit, comp_exit=$comp_exit"
