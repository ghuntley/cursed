#!/bin/bash

# CURSED Interpreter vs Compiler Parity Test Suite - Fixed Version
# Updated to work with fixed interpreter and compiler
# Usage: ./run_tests_fixed.sh [--verbose] [--continue-on-fail]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CURSED_ROOT="${SCRIPT_DIR}/.."
CURSED_COMPILER="${CURSED_ROOT}/zig-out/bin/cursed-compiler"
TEST_DIR="${SCRIPT_DIR}/test_programs"

# Colors for output
RED='\033[31m'
GREEN='\033[32m'
YELLOW='\033[33m'
CYAN='\033[36m'
BLUE='\033[34m'
BOLD='\033[1m'
RESET='\033[0m'

# Options
VERBOSE=0
CONTINUE_ON_FAIL=0
for arg in "$@"; do
    case $arg in
        --verbose|-v)
            VERBOSE=1
            ;;
        --continue-on-fail|-c)
            CONTINUE_ON_FAIL=1
            ;;
    esac
done

# Counters
TOTAL=0
PASSED=0
FAILED=0
COMPILE_ERRORS=0
INTERPRETER_ERRORS=0

echo -e "${BOLD}CURSED Test Suite - Fixed Version${RESET}"
echo "===================================="
echo "Compiler: $CURSED_COMPILER"
echo "Test directory: $TEST_DIR"
echo "Working from: $CURSED_ROOT (for stdlib loading)"
echo

# Function to display output with proper formatting
show_output() {
    local title="$1"
    local output="$2"
    local exit_code="$3"
    
    echo -e "  ${CYAN}$title (exit: $exit_code):${RESET}"
    if [[ -n "$output" ]]; then
        echo "$output" | sed 's/^/    /'
    else
        echo "    (no output)"
    fi
    echo
}

# Function to display output as hex for debugging differences
show_hex_output() {
    local title="$1"
    local output="$2"
    local exit_code="$3"
    
    echo -e "  ${CYAN}$title HEX (exit: $exit_code):${RESET}"
    if [[ -n "$output" ]]; then
        echo -n "$output" | xxd | sed 's/^/    /'
    else
        echo "    (no output)"
    fi
    echo
}

# Check if compiler exists
if [[ ! -x "$CURSED_COMPILER" ]]; then
    echo -e "${RED}Error: Compiler not found or not executable: $CURSED_COMPILER${RESET}"
    echo "Run 'zig build' first to build the compiler"
    exit 1
fi

# Create results directory
mkdir -p "${SCRIPT_DIR}/results"

# Find all .csd test files and process them
echo "Scanning for test files..."
mapfile -t test_files < <(find "$TEST_DIR" -name "*.csd" | sort)
echo "Found ${#test_files[@]} test files"
echo

for test_file in "${test_files[@]}"; do
    TOTAL=$((TOTAL + 1))
    test_name=$(basename "$test_file" .csd)
    relative_path="${test_file#$TEST_DIR/}"
    
    echo -e "${BOLD}[$TOTAL/${#test_files[@]}] Testing: $relative_path${RESET}"
    
    # Change to cursed root directory for proper stdlib loading
    cd "$CURSED_ROOT"
    
    # Run interpreter mode (from cursed root for stdlib access)
    # Program output goes to stderr, memory leaks also go to stderr
    # We need to separate program output from memory leak errors
    interp_output=""
    interp_stderr=""
    interp_exit=1
    if combined_output=$("$CURSED_COMPILER" --interpret "$test_file" 2>&1); then
        # Extract only the program output lines (filter out memory errors and debug info)  
        # Step 1: Split on error(gpa) and take only the first part (program output)
        interp_output=$(echo "$combined_output" | sed '/^error(gpa):/,$d' | grep -v -E '^/[^:]*:[0-9]+:[0-9]+:' | grep -v '🔧\|✅\|🔍\|🚀\|🎉\|🧹' || echo "")
        interp_exit=0
    else
        interp_exit=$?
        combined_output=$("$CURSED_COMPILER" --interpret "$test_file" 2>&1 || true)
        interp_output=$(echo "$combined_output" | sed '/^error(gpa):/,$d' | grep -v -E '^/[^:]*:[0-9]+:[0-9]+:' | grep -v '🔧\|✅\|🔍\|🚀\|🎉\|🧹' || echo "")
    fi
    
    # Try to compile (also from cursed root)
    temp_binary="/tmp/cursed_test_${test_name}_$$"
    comp_output=""
    comp_exit=1
    compiled_output=""
    
    if comp_stderr=$("$CURSED_COMPILER" --compile "$test_file" -o "$temp_binary" 2>&1 >/dev/null); then
        comp_exit=0
        # Run compiled binary (capture stdout since compiled programs output to stdout)
        if binary_output=$("$temp_binary" 2>/dev/null); then
            compiled_output="$binary_output"
        else
            binary_exit=$?
            compiled_output="Runtime error (exit $binary_exit)"
            comp_exit=$binary_exit
        fi
        # Clean up binary
        rm -f "$temp_binary"
    else
        comp_exit=$?
        compiled_output="Compilation failed"
    fi
    
    # Change back to test suite directory
    cd "$SCRIPT_DIR"
    
    # Always show outputs if verbose
    show_details=0
    if [[ $VERBOSE -eq 1 ]]; then
        show_details=1
    fi
    
    # Determine test result
    test_failed=0
    result_color=""
    result_text=""
    
    # Compare results
    if [[ $interp_exit -eq 0 && $comp_exit -eq 0 ]]; then
        # Both succeeded - compare outputs after normalization
        # Normalize outputs: trim whitespace, remove trailing newlines, normalize line endings
        interp_normalized=$(echo "$interp_output" | sed 's/[[:space:]]*$//' | tr -d '\r')
        compiled_normalized=$(echo "$compiled_output" | sed 's/[[:space:]]*$//' | tr -d '\r')
        
        if [[ "$interp_normalized" == "$compiled_normalized" ]]; then
            result_color="$GREEN"
            result_text="PASS"
            PASSED=$((PASSED + 1))
        else
            result_color="$YELLOW"
            result_text="FAIL (different outputs)"
            FAILED=$((FAILED + 1))
            test_failed=1
            show_details=1
            
            # In verbose mode, show byte-by-byte differences for debugging
            if [[ $VERBOSE -eq 1 ]]; then
                echo -e "    ${CYAN}Debug: Normalized outputs differ${RESET}"
                echo -e "    ${CYAN}Interpreter normalized: '${interp_normalized}'${RESET}" 
                echo -e "    ${CYAN}Compiled normalized: '${compiled_normalized}'${RESET}"
                echo -e "    ${CYAN}Interpreter raw bytes:${RESET}"
                echo -n "$interp_output" | xxd | sed 's/^/      /'
                echo -e "    ${CYAN}Compiled raw bytes:${RESET}"
                echo -n "$compiled_output" | xxd | sed 's/^/      /'
            fi
        fi
    elif [[ $interp_exit -ne 0 && $comp_exit -ne 0 ]]; then
        # Both failed - this could be expected for error tests
        # Normalize error messages the same way
        interp_normalized=$(echo "$interp_output" | sed 's/[[:space:]]*$//' | tr -d '\r')
        compiled_normalized=$(echo "$compiled_output" | sed 's/[[:space:]]*$//' | tr -d '\r')
        
        result_color="$BLUE"
        result_text="CONSISTENT FAILURE (both failed)"
        PASSED=$((PASSED + 1))
        if [[ "$interp_normalized" != "$compiled_normalized" ]]; then
            result_color="$YELLOW"
            result_text="FAIL (different error messages)"
            FAILED=$((FAILED + 1))
            PASSED=$((PASSED - 1))
            test_failed=1
            show_details=1
        fi
    else
        # One succeeded, one failed - this is definitely a problem
        if [[ $interp_exit -ne 0 ]]; then
            result_color="$RED"
            result_text="INTERPRETER ERROR (compiled worked)"
            INTERPRETER_ERRORS=$((INTERPRETER_ERRORS + 1))
        else
            result_color="$RED"
            result_text="COMPILE ERROR (interpreted worked)"
            COMPILE_ERRORS=$((COMPILE_ERRORS + 1))
        fi
        test_failed=1
        show_details=1
    fi
    
    echo -e "  Result: ${result_color}$result_text${RESET}"
    
    # Show detailed output if needed
    if [[ $show_details -eq 1 ]]; then
        # Show normal output
        show_output "Interpreter" "$interp_output" "$interp_exit"
        show_output "Compiled" "$compiled_output" "$comp_exit"
        
        # For failed tests, always show hex comparison to identify differences
        if [[ $test_failed -eq 1 && $interp_exit -eq 0 && $comp_exit -eq 0 ]]; then
            echo -e "  ${YELLOW}HEX COMPARISON (to identify differences):${RESET}"
            show_hex_output "Interpreter" "$interp_output" "$interp_exit"
            show_hex_output "Compiled" "$compiled_output" "$comp_exit"
        fi
        
        if [[ -n "$comp_output" && "$comp_output" != "$compiled_output" ]]; then
            show_output "Compilation" "$comp_output" "$comp_exit"
        fi
    fi
    
    # Exit on first failure unless continue flag is set
    if [[ $test_failed -eq 1 && $CONTINUE_ON_FAIL -eq 0 ]]; then
        echo -e "${RED}Test failed. Stopping execution.${RESET}"
        echo ""
        echo -e "${BOLD}Results at failure:${RESET}"
        echo "  Passed: $PASSED"
        echo "  Failed: $FAILED"
        echo "  Interpreter Errors: $INTERPRETER_ERRORS"
        echo "  Compile Errors: $COMPILE_ERRORS"
        echo "  Total Processed: $TOTAL"
        echo "  Remaining: $((${#test_files[@]} - TOTAL))"
        exit 1
    fi
    
    echo "---"
done

# Generate final report
echo ""
echo -e "${BOLD}Final Results:${RESET}"
echo "=============="
echo "  Total Tests: $TOTAL"
echo "  Passed: $PASSED"
echo "  Failed: $FAILED"
echo "  Interpreter Errors: $INTERPRETER_ERRORS"
echo "  Compile Errors: $COMPILE_ERRORS"
echo ""

# Calculate percentages
if [[ $TOTAL -gt 0 ]]; then
    pass_rate=$((PASSED * 100 / TOTAL))
    echo "  Pass Rate: ${pass_rate}%"
fi

# Write summary report
report_file="${SCRIPT_DIR}/results/test_suite_report_$(date +%Y%m%d_%H%M%S).md"
cat > "$report_file" << EOF
# CURSED Test Suite Report

Generated: $(date)

## Summary

- **Total Tests**: $TOTAL
- **Passed**: $PASSED
- **Failed**: $FAILED
- **Interpreter Errors**: $INTERPRETER_ERRORS
- **Compile Errors**: $COMPILE_ERRORS
- **Pass Rate**: ${pass_rate}%

## Test Categories

EOF

# Count tests by directory
echo "## Tests by Category" >> "$report_file"
echo "" >> "$report_file"
for category in $(find "$TEST_DIR" -type d -mindepth 1 | sed "s|$TEST_DIR/||" | sort); do
    count=$(find "$TEST_DIR/$category" -name "*.csd" | wc -l)
    if [[ $count -gt 0 ]]; then
        echo "- **$category**: $count tests" >> "$report_file"
    fi
done

echo "" >> "$report_file"
echo "Report saved to: $report_file"

# Final exit code
if [[ $PASSED -eq $TOTAL ]]; then
    echo -e "${GREEN}All tests passed!${RESET}"
    exit 0
else
    echo -e "${RED}Some tests failed. See details above.${RESET}"
    exit 1
fi
