#!/bin/bash

# CURSED Compilation Test Suite
# Tests only compilation to native executables
# Usage: ./test_compilation_only.sh [--verbose]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="${SCRIPT_DIR}/.."
CURSED_COMPILER="${PROJECT_ROOT}/zig-out/bin/cursed-compiler"
TEST_DIR="${SCRIPT_DIR}/test_programs"

# Colors for output
RED='\033[31m'
GREEN='\033[32m'
YELLOW='\033[33m'
CYAN='\033[36m'
BOLD='\033[1m'
RESET='\033[0m'

# Options
VERBOSE=0
if [[ "${1:-}" == "--verbose" || "${1:-}" == "-v" ]]; then
    VERBOSE=1
fi

# Counters
TOTAL=0
PASSED=0
FAILED=0
COMPILE_ERRORS=0

echo -e "${BOLD}CURSED Compilation Test Suite${RESET}"
echo "=============================="

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

# Find all .csd test files and process them
mapfile -t test_files < <(find "$TEST_DIR" -name "*.csd" | sort)

for test_file in "${test_files[@]}"; do
    TOTAL=$((TOTAL + 1))
    test_name=$(basename "$test_file" .csd)
    
    echo -e "${BOLD}Testing: $test_name${RESET}"
    
    # Try to compile - run from project root for proper stdlib resolution
    # Compiler generates binary with input file name (ignoring -o flag)
    expected_binary="${PROJECT_ROOT}/${test_name}" 
    if (cd "$PROJECT_ROOT" && comp_output=$("$CURSED_COMPILER" --compile "${test_file#${PROJECT_ROOT}/}" 2>&1)); then
        # Run compiled binary
        if compiled_output=$("$expected_binary" 2>&1); then
            comp_exit=0
            echo -e "  Result: ${GREEN}COMPILE SUCCESS${RESET}"
            PASSED=$((PASSED + 1))
            
            if [[ $VERBOSE -eq 1 ]]; then
                show_output "Compiled" "$compiled_output" "$comp_exit"
            fi
        else
            comp_exit=$?
            echo -e "  Result: ${YELLOW}RUNTIME ERROR${RESET}"
            FAILED=$((FAILED + 1))
            show_output "Compiled (runtime error)" "$compiled_output" "$comp_exit"
        fi
        rm -f "$expected_binary" 2>/dev/null || true
    else
        comp_exit=$?
        echo -e "  Result: ${RED}COMPILE ERROR${RESET}"
        COMPILE_ERRORS=$((COMPILE_ERRORS + 1))
        show_output "Compilation Error" "$comp_output" "$comp_exit"
    fi
    echo
done

# Final summary
echo -e "${BOLD}Test Summary:${RESET}"
echo "=============="
echo "  Total: $TOTAL"
echo -e "  ${GREEN}Passed: $PASSED${RESET}"
echo -e "  ${YELLOW}Failed: $FAILED${RESET}"
echo -e "  ${RED}Compile Errors: $COMPILE_ERRORS${RESET}"

if [[ $COMPILE_ERRORS -eq 0 && $FAILED -eq 0 ]]; then
    echo -e "${GREEN}All tests passed!${RESET}"
    exit 0
elif [[ $COMPILE_ERRORS -gt 0 ]]; then
    echo -e "${RED}Some tests failed to compile${RESET}"
    exit 1
else
    echo -e "${YELLOW}Some tests had runtime errors${RESET}"
    exit 1
fi
